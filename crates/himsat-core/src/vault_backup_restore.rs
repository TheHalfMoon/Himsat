//! B505 portable-backup fresh-device restore material verification.
//!
//! Provider authentication, payload reconstruction, inner-format verification,
//! and SQLCipher integrity complete before any protector creation, VRK storage,
//! protected genesis decision, local canonical publication, or anchor mutation.

use crate::vault::{ProtectedFreshnessState, ProtectorError, SecretProtector, VaultId};
use crate::vault_backup_provider::PortableBackupProvider;
use crate::vault_backup_sqlcipher::{
    BackupSqlCipherVerificationError, SqlCipherVerifiedBackupSet, reverify_staged_sqlcipher_backup,
    verify_staged_sqlcipher_backup,
};
use crate::vault_backup_verify::{BackupSemanticError, verify_backup_semantics_before_sqlcipher};
use crate::vault_blob::{
    BoundedBlobContext, BoundedBlobError, bounded_blob_nonce, decrypt_bounded_blob,
};
use crate::vault_freshness::{
    AuthenticatedFreshnessManifest, FreshnessDecision, FreshnessError, authenticate_for_open,
};
use crate::vault_keys::{KeyDerivationContext, KeyPurpose, OwnedKeyMaterial};
use crate::vault_lease::VaultLease;
use crate::vault_manifest::{ManifestAuthMetadata, ManifestObject};
use crate::vault_nonce::{
    FreshBoundedBlobError, NonceLifecycleError, NoncePurpose, NonceReservation,
    NonceReservationLedger,
};
use crate::vault_restore::{
    FreshDeviceNewestnessRiskAccepted, FreshDeviceRestoreGenesis, RestoreError,
    prepare_fresh_device_restore_genesis,
};
use crate::vault_sqlcipher::{
    SqlCipherGenerationEndpoint, SqlCipherGenerationMigrationError, open_sqlcipher_database,
    stage_sqlcipher_generation,
};
use sha2::{Digest as _, Sha256 as RestoreSha256};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use zeroize::Zeroizing;

#[derive(Debug)]
pub enum FreshDeviceBackupMaterialError<E> {
    StagingCreateFailed,
    StagingSyncFailed,
    Semantic(BackupSemanticError<E>),
    SqlCipher(BackupSqlCipherVerificationError),
    CorruptOrTampered,
}

impl<E: fmt::Display> fmt::Display for FreshDeviceBackupMaterialError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StagingCreateFailed => {
                f.write_str("fresh-device restore staging creation failed")
            }
            Self::StagingSyncFailed => f.write_str("fresh-device restore staging sync failed"),
            Self::Semantic(error) => {
                write!(f, "portable backup semantic verification failed: {error}")
            }
            Self::SqlCipher(error) => {
                write!(f, "portable backup SQLCipher verification failed: {error}")
            }
            Self::CorruptOrTampered => {
                f.write_str("fresh-device restore material is corrupt or tampered")
            }
        }
    }
}

impl<E: Error + 'static> Error for FreshDeviceBackupMaterialError<E> {}

pub struct FreshDeviceVerifiedBackup {
    verified_backup: SqlCipherVerifiedBackupSet,
}

impl FreshDeviceVerifiedBackup {
    #[must_use]
    pub fn verified_backup(&self) -> &SqlCipherVerifiedBackupSet {
        &self.verified_backup
    }

    #[must_use]
    pub fn into_verified_backup(self) -> SqlCipherVerifiedBackupSet {
        self.verified_backup
    }
}

#[derive(Debug)]
pub enum FreshDeviceGenesisGateError {
    Protector(ProtectorError),
    Restore(RestoreError),
    RecoveredKeyMismatch,
    ProtectedStateChanged,
}

impl fmt::Display for FreshDeviceGenesisGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protector(error) => write!(f, "fresh-device protector operation failed: {error}"),
            Self::Restore(error) => write!(f, "fresh-device genesis preparation failed: {error}"),
            Self::RecoveredKeyMismatch => {
                f.write_str("fresh-device protected VRK does not match recovered VRK")
            }
            Self::ProtectedStateChanged => {
                f.write_str("fresh-device protected state changed during genesis preparation")
            }
        }
    }
}

impl Error for FreshDeviceGenesisGateError {}

pub struct FreshDevicePreparedGenesis {
    verified_backup: FreshDeviceVerifiedBackup,
    genesis: FreshDeviceRestoreGenesis,
}

impl FreshDevicePreparedGenesis {
    #[must_use]
    pub fn verified_backup(&self) -> &FreshDeviceVerifiedBackup {
        &self.verified_backup
    }

    #[must_use]
    pub const fn genesis(&self) -> &FreshDeviceRestoreGenesis {
        &self.genesis
    }

    #[must_use]
    pub fn into_parts(self) -> (FreshDeviceVerifiedBackup, FreshDeviceRestoreGenesis) {
        (self.verified_backup, self.genesis)
    }
}

pub(crate) fn key_material_equal(left: &OwnedKeyMaterial, right: &OwnedKeyMaterial) -> bool {
    left.with_bytes(|left_bytes| {
        right.with_bytes(|right_bytes| {
            left_bytes
                .iter()
                .zip(right_bytes.iter())
                .fold(0_u8, |difference, (left, right)| {
                    difference | (left ^ right)
                })
                == 0
        })
    })
}

/// Consumes completely authenticated B505Q material and prepares only the
/// canonical B502 fresh-device genesis decision. It does not publish local
/// storage or install the protected freshness anchor.
///
/// Existing protected state is never overwritten. A present anchor rejects
/// fresh-device genesis. An explicit `UNINITIALIZED` record must unlock the
/// exact recovered generation and match the recovered VRK. Only a typed
/// `ItemMissing` result may create the recovered VRK record, which is then
/// reread and required to expose explicit `UNINITIALIZED` freshness.
pub fn prepare_verified_fresh_device_genesis<P>(
    protector: &mut P,
    verified_backup: FreshDeviceVerifiedBackup,
    risk_acceptance: FreshDeviceNewestnessRiskAccepted,
) -> Result<FreshDevicePreparedGenesis, FreshDeviceGenesisGateError>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
{
    let genesis = verified_backup
        .verified_backup()
        .pre_sqlcipher()
        .with_structured_store_verification_key(|recovered_vrk, vault_id, generation| {
            let protected_state = match protector.read_freshness_anchor(vault_id) {
                Ok(ProtectedFreshnessState::Present(_)) => {
                    return Err(FreshDeviceGenesisGateError::Restore(
                        RestoreError::AnchorAlreadyInitialized,
                    ));
                }
                Ok(ProtectedFreshnessState::Uninitialized) => {
                    let existing = protector
                        .unlock_vrk(vault_id, generation)
                        .map_err(FreshDeviceGenesisGateError::Protector)?;
                    if !key_material_equal(recovered_vrk, &existing) {
                        return Err(FreshDeviceGenesisGateError::RecoveredKeyMismatch);
                    }
                    ProtectedFreshnessState::Uninitialized
                }
                Err(ProtectorError::ItemMissing) => {
                    protector
                        .protect_or_store_vrk(vault_id, generation, recovered_vrk)
                        .map_err(FreshDeviceGenesisGateError::Protector)?;
                    let stored = protector
                        .unlock_vrk(vault_id, generation)
                        .map_err(FreshDeviceGenesisGateError::Protector)?;
                    if !key_material_equal(recovered_vrk, &stored) {
                        return Err(FreshDeviceGenesisGateError::RecoveredKeyMismatch);
                    }
                    match protector
                        .read_freshness_anchor(vault_id)
                        .map_err(FreshDeviceGenesisGateError::Protector)?
                    {
                        ProtectedFreshnessState::Uninitialized => {
                            ProtectedFreshnessState::Uninitialized
                        }
                        ProtectedFreshnessState::Present(_) => {
                            return Err(FreshDeviceGenesisGateError::ProtectedStateChanged);
                        }
                    }
                }
                Err(error) => return Err(FreshDeviceGenesisGateError::Protector(error)),
            };

            prepare_fresh_device_restore_genesis(
                recovered_vrk,
                vault_id,
                protected_state,
                verified_backup
                    .verified_backup()
                    .pre_sqlcipher()
                    .source_manifest_envelope(),
                risk_acceptance,
            )
            .map_err(FreshDeviceGenesisGateError::Restore)
        })?;

    Ok(FreshDevicePreparedGenesis {
        verified_backup,
        genesis,
    })
}

/// Caller-owned coordination proof for the B505 existing-device restore gate.
///
/// The caller must keep ordinary vault writes excluded while this gate runs.
/// Later mutation/staging grains must acquire or reassert their own live
/// exclusion; this read-only gate does not claim quiescence survives its return.
pub trait ExistingDeviceRestoreGateBackend {
    type Error;

    fn assert_normal_writes_quiesced(&mut self) -> Result<(), Self::Error>;

    fn verify_current_inventory(
        &mut self,
        current_vrk: &OwnedKeyMaterial,
        manifest: &crate::vault_manifest::ManifestPlaintext,
    ) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExistingDeviceRestoreRoute {
    SameGeneration,
    CrossGeneration,
}

#[derive(Debug)]
pub enum ExistingDeviceRestoreGateError<E> {
    Backend(E),
    Protector(ProtectorError),
    Freshness(FreshnessError),
    RestoreStateNotStable,
    SourceVaultMismatch,
    SourceNotOlder,
    RestoreGenerationAhead,
    KeyGenerationIdentityMismatch,
}

impl<E: fmt::Display> fmt::Display for ExistingDeviceRestoreGateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(error) => {
                write!(f, "existing-device restore gate backend failed: {error}")
            }
            Self::Protector(error) => {
                write!(f, "existing-device restore protector failed: {error}")
            }
            Self::Freshness(error) => {
                write!(f, "existing-device restore freshness failed: {error}")
            }
            Self::RestoreStateNotStable => {
                f.write_str("existing-device restore current state is not stable")
            }
            Self::SourceVaultMismatch => {
                f.write_str("existing-device restore source vault does not match current vault")
            }
            Self::SourceNotOlder => {
                f.write_str("existing-device restore source is not older than the trusted anchor")
            }
            Self::RestoreGenerationAhead => {
                f.write_str("existing-device restore source generation is ahead of current state")
            }
            Self::KeyGenerationIdentityMismatch => {
                f.write_str("existing-device restore same generation has different key material")
            }
        }
    }
}

impl<E: Error + 'static> Error for ExistingDeviceRestoreGateError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend(error) => Some(error),
            Self::Protector(error) => Some(error),
            Self::Freshness(error) => Some(error),
            _ => None,
        }
    }
}

/// Read-only, authenticated decision produced before any existing-device
/// restore staging or canonical mutation.
pub struct ExistingDeviceRestoreGate {
    verified_backup: FreshDeviceVerifiedBackup,
    authenticated_current: AuthenticatedFreshnessManifest,
    current_vrk: OwnedKeyMaterial,
    current_identity: crate::vault::VaultLeaseIdentity,
    trusted_anchor: crate::vault::FreshnessAnchor,
    route: ExistingDeviceRestoreRoute,
}

impl ExistingDeviceRestoreGate {
    #[must_use]
    pub const fn route(&self) -> ExistingDeviceRestoreRoute {
        self.route
    }

    #[must_use]
    pub const fn current_identity(&self) -> crate::vault::VaultLeaseIdentity {
        self.current_identity
    }

    #[must_use]
    pub const fn trusted_anchor(&self) -> crate::vault::FreshnessAnchor {
        self.trusted_anchor
    }

    #[must_use]
    pub const fn current_manifest(&self) -> &AuthenticatedFreshnessManifest {
        &self.authenticated_current
    }

    #[must_use]
    pub fn verified_backup(&self) -> &FreshDeviceVerifiedBackup {
        &self.verified_backup
    }

    pub fn with_current_vrk<T>(&self, operation: impl FnOnce(&OwnedKeyMaterial) -> T) -> T {
        operation(&self.current_vrk)
    }
}

fn current_manifest_is_stable(
    manifest: &crate::vault_manifest::ManifestPlaintext,
    identity: crate::vault::VaultLeaseIdentity,
) -> bool {
    let current_generation = identity.key_generation();
    manifest.vault_id() == identity.vault_id()
        && manifest.active_key_generation() == current_generation
        && manifest.rotation_phase() == crate::vault_manifest::RotationPhase::None
        && manifest.rotation_target_generation().is_none()
        && manifest.generations().iter().all(|entry| {
            if entry.generation() == current_generation {
                entry.state() == crate::vault_manifest::GenerationState::Active
            } else {
                entry.generation() < current_generation
                    && entry.state() == crate::vault_manifest::GenerationState::Retained
            }
        })
        && manifest
            .objects()
            .iter()
            .all(|object| object.key_generation() == current_generation)
}

/// Authenticates and freezes the B505 existing-device restore decision without
/// creating target objects, re-encrypting data, publishing a manifest, or
/// advancing protected freshness.
///
/// This gate accepts only a stable current manifest exactly equal to the
/// protected anchor. Interrupted publication, active/retained rotation state,
/// generation-table drift, inventory-generation drift, and protector drift all
/// fail closed. A same-generation source must also prove recovered/current VRK
/// identity; an older source generation is routed to the later cross-generation
/// rebase path, while a generation ahead of current state is rejected.
pub fn prepare_existing_device_restore_gate<B, P>(
    backend: &mut B,
    protector: &mut P,
    current_identity: crate::vault::VaultLeaseIdentity,
    current_manifest_envelope: &[u8],
    verified_backup: FreshDeviceVerifiedBackup,
) -> Result<ExistingDeviceRestoreGate, ExistingDeviceRestoreGateError<B::Error>>
where
    B: ExistingDeviceRestoreGateBackend,
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
{
    backend
        .assert_normal_writes_quiesced()
        .map_err(ExistingDeviceRestoreGateError::Backend)?;

    let vault_id = current_identity.vault_id();
    let current_generation = current_identity.key_generation();
    let trusted_anchor = match protector.read_freshness_anchor(vault_id) {
        Ok(ProtectedFreshnessState::Present(anchor)) if anchor.vault_id() == vault_id => anchor,
        Ok(ProtectedFreshnessState::Present(_)) | Ok(ProtectedFreshnessState::Uninitialized) => {
            return Err(ExistingDeviceRestoreGateError::RestoreStateNotStable);
        }
        Err(error) => return Err(ExistingDeviceRestoreGateError::Protector(error)),
    };

    let current_vrk = protector
        .unlock_vrk(vault_id, current_generation)
        .map_err(ExistingDeviceRestoreGateError::Protector)?;
    let authenticated_current =
        authenticate_for_open(&current_vrk, trusted_anchor, current_manifest_envelope)
            .map_err(ExistingDeviceRestoreGateError::Freshness)?;
    if authenticated_current.decision() != FreshnessDecision::Current
        || !current_manifest_is_stable(authenticated_current.manifest(), current_identity)
    {
        return Err(ExistingDeviceRestoreGateError::RestoreStateNotStable);
    }
    backend
        .verify_current_inventory(&current_vrk, authenticated_current.manifest())
        .map_err(ExistingDeviceRestoreGateError::Backend)?;

    let source = verified_backup.verified_backup().pre_sqlcipher();
    if source.vault_id() != vault_id {
        return Err(ExistingDeviceRestoreGateError::SourceVaultMismatch);
    }
    if source.source_freshness_epoch() >= trusted_anchor.highest_epoch() {
        return Err(ExistingDeviceRestoreGateError::SourceNotOlder);
    }
    if source.key_generation() > current_generation {
        return Err(ExistingDeviceRestoreGateError::RestoreGenerationAhead);
    }

    let route = if source.key_generation() == current_generation {
        let same_key = source.with_structured_store_verification_key(|source_vrk, _, _| {
            key_material_equal(source_vrk, &current_vrk)
        });
        if !same_key {
            return Err(ExistingDeviceRestoreGateError::KeyGenerationIdentityMismatch);
        }
        ExistingDeviceRestoreRoute::SameGeneration
    } else {
        ExistingDeviceRestoreRoute::CrossGeneration
    };

    backend
        .assert_normal_writes_quiesced()
        .map_err(ExistingDeviceRestoreGateError::Backend)?;
    match protector.read_freshness_anchor(vault_id) {
        Ok(ProtectedFreshnessState::Present(anchor)) if anchor == trusted_anchor => {}
        Ok(_) => return Err(ExistingDeviceRestoreGateError::RestoreStateNotStable),
        Err(error) => return Err(ExistingDeviceRestoreGateError::Protector(error)),
    }
    let reread_current_vrk = protector
        .unlock_vrk(vault_id, current_generation)
        .map_err(ExistingDeviceRestoreGateError::Protector)?;
    if !key_material_equal(&current_vrk, &reread_current_vrk) {
        return Err(ExistingDeviceRestoreGateError::RestoreStateNotStable);
    }

    Ok(ExistingDeviceRestoreGate {
        verified_backup,
        authenticated_current,
        current_vrk,
        current_identity,
        trusted_anchor,
        route,
    })
}

/// One authenticated retained B202 encryption instance used only to reconcile a
/// same-generation restore nonce collision.
#[derive(Clone)]
pub struct ExistingDeviceRetainedBlob {
    logical_id: [u8; 16],
    envelope: Vec<u8>,
}

impl ExistingDeviceRetainedBlob {
    #[must_use]
    pub fn new(logical_id: [u8; 16], envelope: Vec<u8>) -> Self {
        Self {
            logical_id,
            envelope,
        }
    }

    #[must_use]
    pub const fn logical_id(&self) -> [u8; 16] {
        self.logical_id
    }

    #[must_use]
    pub fn envelope(&self) -> &[u8] {
        &self.envelope
    }
}

/// B505 target-staging storage boundary. Implementations hold write exclusion,
/// use create-if-absent durable staging, and return only authenticated retained
/// B202 evidence from `retained_blob_for_reservation`.
pub trait ExistingDeviceRestoreTargetBackend: ExistingDeviceRestoreGateBackend {
    fn target_storage_available(&mut self, storage_id: [u8; 16]) -> Result<bool, Self::Error>;

    fn sqlcipher_target_path(&mut self, storage_id: [u8; 16]) -> Result<PathBuf, Self::Error>;

    fn stage_blob(&mut self, storage_id: [u8; 16], envelope: &[u8]) -> Result<(), Self::Error>;

    fn read_staged_blob(&mut self, storage_id: [u8; 16]) -> Result<Vec<u8>, Self::Error>;

    fn retained_blob_for_reservation(
        &mut self,
        reservation: NonceReservation,
    ) -> Result<Option<ExistingDeviceRetainedBlob>, Self::Error>;
}

#[derive(Debug)]
pub enum ExistingDeviceRestoreStagingError<E> {
    Backend(E),
    Protector(ProtectorError),
    SourceSqlCipher(BackupSqlCipherVerificationError),
    SqlCipher(SqlCipherGenerationMigrationError),
    Blob(BoundedBlobError),
    FreshBlob(FreshBoundedBlobError),
    Nonce(NonceLifecycleError),
    RandomnessUnavailable,
    TargetStorageIdExhausted,
    TargetPath,
    Copy,
    Durability,
    SourceChanged,
    TargetIdentityMismatch,
    NonceReservationConflict,
    StagedBlobMismatch,
    ProtectedStateChanged,
}

impl<E: fmt::Display> fmt::Display for ExistingDeviceRestoreStagingError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(error) => {
                write!(f, "existing-device restore staging backend failed: {error}")
            }
            Self::Protector(error) => write!(
                f,
                "existing-device restore staging protector failed: {error}"
            ),
            Self::SourceSqlCipher(error) => write!(
                f,
                "existing-device restore source SQLCipher verification failed: {error}"
            ),
            Self::SqlCipher(error) => write!(
                f,
                "existing-device restore SQLCipher staging failed: {error}"
            ),
            Self::Blob(error) => write!(
                f,
                "existing-device restore blob verification failed: {error}"
            ),
            Self::FreshBlob(error) => write!(
                f,
                "existing-device restore blob re-encryption failed: {error}"
            ),
            Self::Nonce(error) => write!(
                f,
                "existing-device restore nonce reconciliation failed: {error}"
            ),
            Self::RandomnessUnavailable => {
                f.write_str("existing-device restore target-id randomness is unavailable")
            }
            Self::TargetStorageIdExhausted => {
                f.write_str("existing-device restore could not allocate a fresh target storage id")
            }
            Self::TargetPath => f.write_str("existing-device restore target path is invalid"),
            Self::Copy => f.write_str("existing-device restore SQLCipher exact copy failed"),
            Self::Durability => {
                f.write_str("existing-device restore staged SQLCipher durability failed")
            }
            Self::SourceChanged => {
                f.write_str("existing-device restore source SQLCipher changed during staging")
            }
            Self::TargetIdentityMismatch => {
                f.write_str("existing-device restore staged SQLCipher identity mismatch")
            }
            Self::NonceReservationConflict => f.write_str(
                "existing-device restore copied blob nonce conflicts with retained history",
            ),
            Self::StagedBlobMismatch => {
                f.write_str("existing-device restore staged blob reread mismatch")
            }
            Self::ProtectedStateChanged => f.write_str(
                "existing-device restore protected current state changed during staging",
            ),
        }
    }
}

impl<E: Error + 'static> Error for ExistingDeviceRestoreStagingError<E> {}

/// Verified non-canonical target inventory produced before the B505 staging
/// manifest and B502 publication units. The current canonical object set and
/// protected freshness state remain unchanged.
pub struct ExistingDeviceStagedRestore {
    gate: ExistingDeviceRestoreGate,
    staged_objects: Vec<ManifestObject>,
}

impl ExistingDeviceStagedRestore {
    #[must_use]
    pub fn gate(&self) -> &ExistingDeviceRestoreGate {
        &self.gate
    }

    #[must_use]
    pub fn staged_objects(&self) -> &[ManifestObject] {
        &self.staged_objects
    }

    #[must_use]
    pub fn into_parts(self) -> (ExistingDeviceRestoreGate, Vec<ManifestObject>) {
        (self.gate, self.staged_objects)
    }
}

const TARGET_STORAGE_ID_ATTEMPTS: usize = 32;

fn generate_target_storage_id<B>(
    backend: &mut B,
    forbidden: &mut HashSet<[u8; 16]>,
) -> Result<[u8; 16], ExistingDeviceRestoreStagingError<B::Error>>
where
    B: ExistingDeviceRestoreTargetBackend,
{
    for _ in 0..TARGET_STORAGE_ID_ATTEMPTS {
        let mut candidate = [0_u8; 16];
        getrandom::fill(&mut candidate)
            .map_err(|_| ExistingDeviceRestoreStagingError::RandomnessUnavailable)?;
        if candidate == [0_u8; 16] || forbidden.contains(&candidate) {
            continue;
        }
        if !backend
            .target_storage_available(candidate)
            .map_err(ExistingDeviceRestoreStagingError::Backend)?
        {
            continue;
        }
        forbidden.insert(candidate);
        return Ok(candidate);
    }
    Err(ExistingDeviceRestoreStagingError::TargetStorageIdExhausted)
}

fn sqlcipher_file_identity(path: &Path) -> io::Result<(u64, [u8; 32])> {
    let mut file = File::open(path)?;
    let mut hasher = RestoreSha256::new();
    let mut total = 0_u64;
    let mut buffer = vec![0_u8; 1024 * 1024];
    loop {
        let read = std::io::Read::read(&mut file, &mut buffer)?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(u64::try_from(read).map_err(io::Error::other)?)
            .ok_or_else(|| io::Error::other("SQLCipher file length overflow"))?;
        hasher.update(&buffer[..read]);
    }
    Ok((total, hasher.finalize().into()))
}

fn copy_same_generation_sqlcipher<E>(
    source: &Path,
    target: &Path,
    expected: (u64, [u8; 32]),
    current_vrk: &OwnedKeyMaterial,
    current_identity: crate::vault::VaultLeaseIdentity,
) -> Result<(u64, [u8; 32]), ExistingDeviceRestoreStagingError<E>> {
    if target.exists() {
        return Err(ExistingDeviceRestoreStagingError::TargetIdentityMismatch);
    }
    let source_before = sqlcipher_file_identity(source)
        .map_err(|_| ExistingDeviceRestoreStagingError::TargetPath)?;
    if source_before != expected {
        return Err(ExistingDeviceRestoreStagingError::SourceChanged);
    }
    let mut source_file =
        File::open(source).map_err(|_| ExistingDeviceRestoreStagingError::Copy)?;
    let mut target_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(target)
        .map_err(|_| ExistingDeviceRestoreStagingError::Copy)?;
    let copied = io::copy(&mut source_file, &mut target_file)
        .map_err(|_| ExistingDeviceRestoreStagingError::Copy)?;
    if copied != expected.0 {
        return Err(ExistingDeviceRestoreStagingError::TargetIdentityMismatch);
    }
    target_file
        .sync_all()
        .map_err(|_| ExistingDeviceRestoreStagingError::Durability)?;
    drop(target_file);

    let source_after = sqlcipher_file_identity(source)
        .map_err(|_| ExistingDeviceRestoreStagingError::TargetPath)?;
    if source_after != source_before {
        return Err(ExistingDeviceRestoreStagingError::SourceChanged);
    }
    let target_identity = sqlcipher_file_identity(target)
        .map_err(|_| ExistingDeviceRestoreStagingError::TargetPath)?;
    if target_identity != expected {
        return Err(ExistingDeviceRestoreStagingError::TargetIdentityMismatch);
    }

    let lease = VaultLease::new(current_identity);
    let context = KeyDerivationContext::new(
        current_identity.vault_id(),
        current_identity.key_generation(),
        KeyPurpose::StructuredStore,
    );
    let handle = open_sqlcipher_database(target, lease.keyed_handle_lease(), context, current_vrk)
        .map_err(|_| ExistingDeviceRestoreStagingError::TargetIdentityMismatch)?;
    handle
        .verify_integrity()
        .map_err(|_| ExistingDeviceRestoreStagingError::TargetIdentityMismatch)?;
    drop(handle);
    Ok(target_identity)
}

fn require_exact_retained_blob<B>(
    backend: &mut B,
    reservation: NonceReservation,
    logical_id: [u8; 16],
    source_envelope: &[u8],
) -> Result<(), ExistingDeviceRestoreStagingError<B::Error>>
where
    B: ExistingDeviceRestoreTargetBackend,
{
    let Some(retained) = backend
        .retained_blob_for_reservation(reservation)
        .map_err(ExistingDeviceRestoreStagingError::Backend)?
    else {
        return Err(ExistingDeviceRestoreStagingError::NonceReservationConflict);
    };
    if retained.logical_id() != logical_id || retained.envelope() != source_envelope {
        return Err(ExistingDeviceRestoreStagingError::NonceReservationConflict);
    }
    Ok(())
}

fn verify_protected_staging_state<B, P>(
    backend: &mut B,
    protector: &mut P,
    gate: &ExistingDeviceRestoreGate,
) -> Result<(), ExistingDeviceRestoreStagingError<B::Error>>
where
    B: ExistingDeviceRestoreTargetBackend,
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
{
    backend
        .assert_normal_writes_quiesced()
        .map_err(ExistingDeviceRestoreStagingError::Backend)?;
    let identity = gate.current_identity();
    match protector
        .read_freshness_anchor(identity.vault_id())
        .map_err(ExistingDeviceRestoreStagingError::Protector)?
    {
        ProtectedFreshnessState::Present(anchor) if anchor == gate.trusted_anchor() => {}
        _ => return Err(ExistingDeviceRestoreStagingError::ProtectedStateChanged),
    }
    let reread = protector
        .unlock_vrk(identity.vault_id(), identity.key_generation())
        .map_err(ExistingDeviceRestoreStagingError::Protector)?;
    let key_matches = gate.with_current_vrk(|current| key_material_equal(current, &reread));
    if !key_matches {
        return Err(ExistingDeviceRestoreStagingError::ProtectedStateChanged);
    }
    gate.with_current_vrk(|current| {
        backend
            .verify_current_inventory(current, gate.current_manifest().manifest())
            .map_err(ExistingDeviceRestoreStagingError::Backend)
    })
}

/// Stages a verified non-canonical restore inventory under current generation `H`.
/// `G == H` preserves ciphertext and reconciles B202 nonces; `G < H` re-encrypts
/// through qualified SQLCipher/B202 paths. This function never publishes or anchors.
pub fn stage_existing_device_restore_targets<B, P>(
    backend: &mut B,
    protector: &mut P,
    structured_store_source: &Path,
    nonce_ledger: &mut NonceReservationLedger,
    gate: ExistingDeviceRestoreGate,
) -> Result<ExistingDeviceStagedRestore, ExistingDeviceRestoreStagingError<B::Error>>
where
    B: ExistingDeviceRestoreTargetBackend,
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
{
    verify_protected_staging_state(backend, protector, &gate)?;
    let identity = gate.current_identity();
    if nonce_ledger.vault_id() != identity.vault_id() {
        return Err(ExistingDeviceRestoreStagingError::Nonce(
            NonceLifecycleError::VaultMismatch,
        ));
    }
    let verified = gate.verified_backup().verified_backup();
    reverify_staged_sqlcipher_backup(structured_store_source, verified)
        .map_err(ExistingDeviceRestoreStagingError::SourceSqlCipher)?;
    let source = verified.pre_sqlcipher();

    let mut forbidden_storage_ids: HashSet<[u8; 16]> = gate
        .current_manifest()
        .manifest()
        .objects()
        .iter()
        .map(ManifestObject::storage_id)
        .collect();
    forbidden_storage_ids.insert(source.structured_store_source_storage_id());
    forbidden_storage_ids.extend(
        source
            .generic_artifacts()
            .iter()
            .map(|blob| blob.source_storage_id()),
    );

    let structured_target_id = generate_target_storage_id(backend, &mut forbidden_storage_ids)?;
    let structured_target_path = backend
        .sqlcipher_target_path(structured_target_id)
        .map_err(ExistingDeviceRestoreStagingError::Backend)?;
    let current_generation = identity.key_generation();
    let expected_source_identity = (
        source.structured_store_exact_length(),
        source.structured_store_exact_sha256(),
    );

    let structured_target_identity = match gate.route() {
        ExistingDeviceRestoreRoute::SameGeneration => gate.with_current_vrk(|current_vrk| {
            copy_same_generation_sqlcipher(
                structured_store_source,
                &structured_target_path,
                expected_source_identity,
                current_vrk,
                identity,
            )
        })?,
        ExistingDeviceRestoreRoute::CrossGeneration => source
            .with_structured_store_verification_key(|source_vrk, vault_id, source_generation| {
                gate.with_current_vrk(|current_vrk| {
                    let source_lease = VaultLease::new(crate::vault::VaultLeaseIdentity::new(
                        vault_id,
                        source_generation,
                    ));
                    let target_lease = VaultLease::new(identity);
                    let source_endpoint = SqlCipherGenerationEndpoint::new(
                        structured_store_source,
                        source_lease.keyed_handle_lease(),
                        KeyDerivationContext::new(
                            vault_id,
                            source_generation,
                            KeyPurpose::StructuredStore,
                        ),
                        source_vrk,
                    );
                    let target_endpoint = SqlCipherGenerationEndpoint::new(
                        &structured_target_path,
                        target_lease.keyed_handle_lease(),
                        KeyDerivationContext::new(
                            vault_id,
                            current_generation,
                            KeyPurpose::StructuredStore,
                        ),
                        current_vrk,
                    );
                    drop(
                        stage_sqlcipher_generation(source_endpoint, target_endpoint)
                            .map_err(ExistingDeviceRestoreStagingError::SqlCipher)?,
                    );
                    sqlcipher_file_identity(&structured_target_path)
                        .map_err(|_| ExistingDeviceRestoreStagingError::TargetPath)
                })
            })?,
    };
    reverify_staged_sqlcipher_backup(structured_store_source, verified)
        .map_err(ExistingDeviceRestoreStagingError::SourceSqlCipher)?;

    let mut staged_objects = Vec::new();
    staged_objects
        .try_reserve_exact(source.generic_artifacts().len().saturating_add(1))
        .map_err(|_| ExistingDeviceRestoreStagingError::TargetPath)?;
    staged_objects.push(ManifestObject::new(
        [0_u8; 16],
        structured_target_id,
        current_generation,
        structured_target_identity.0,
        structured_target_identity.1,
        ManifestAuthMetadata::StructuredStore,
    ));

    for blob in source.generic_artifacts() {
        let target_storage_id = generate_target_storage_id(backend, &mut forbidden_storage_ids)?;
        let target_context =
            BoundedBlobContext::new(identity.vault_id(), blob.logical_id(), current_generation);

        let (target_reservation, target_envelope) = match gate.route() {
            ExistingDeviceRestoreRoute::SameGeneration => {
                let nonce = bounded_blob_nonce(blob.envelope())
                    .map_err(ExistingDeviceRestoreStagingError::Blob)?;
                let reservation = NonceReservation::new(
                    identity.vault_id(),
                    NoncePurpose::BoundedBlob,
                    current_generation,
                    nonce,
                );
                if nonce_ledger.contains(reservation) {
                    require_exact_retained_blob(
                        backend,
                        reservation,
                        blob.logical_id(),
                        blob.envelope(),
                    )?;
                } else {
                    nonce_ledger
                        .record_authenticated_canonical_reservation(reservation)
                        .map_err(ExistingDeviceRestoreStagingError::Nonce)?;
                }
                (reservation, blob.envelope().to_vec())
            }
            ExistingDeviceRestoreRoute::CrossGeneration => source
                .with_structured_store_verification_key(|source_vrk, _, source_generation| {
                    gate.with_current_vrk(|current_vrk| {
                        let source_context = BoundedBlobContext::new(
                            identity.vault_id(),
                            blob.logical_id(),
                            source_generation,
                        );
                        let plaintext = Zeroizing::new(
                            decrypt_bounded_blob(source_vrk, source_context, blob.envelope())
                                .map_err(ExistingDeviceRestoreStagingError::Blob)?,
                        );
                        let fresh = nonce_ledger
                            .encrypt_fresh_bounded_blob(current_vrk, target_context, &plaintext)
                            .map_err(ExistingDeviceRestoreStagingError::FreshBlob)?;
                        Ok::<_, ExistingDeviceRestoreStagingError<B::Error>>(fresh.into_parts())
                    })
                })?,
        };

        backend
            .stage_blob(target_storage_id, &target_envelope)
            .map_err(ExistingDeviceRestoreStagingError::Backend)?;
        let reread = backend
            .read_staged_blob(target_storage_id)
            .map_err(ExistingDeviceRestoreStagingError::Backend)?;
        if reread != target_envelope {
            return Err(ExistingDeviceRestoreStagingError::StagedBlobMismatch);
        }
        gate.with_current_vrk(|current_vrk| {
            decrypt_bounded_blob(current_vrk, target_context, &reread)
                .map(Zeroizing::new)
                .map_err(ExistingDeviceRestoreStagingError::Blob)
        })?;
        staged_objects.push(ManifestObject::new(
            blob.logical_id(),
            target_storage_id,
            current_generation,
            u64::try_from(target_envelope.len())
                .map_err(|_| ExistingDeviceRestoreStagingError::TargetIdentityMismatch)?,
            RestoreSha256::digest(&target_envelope).into(),
            ManifestAuthMetadata::GenericArtifactBlob {
                nonce: target_reservation.nonce(),
            },
        ));
    }

    staged_objects.sort_by_key(|object| (object.kind(), object.logical_id()));
    verify_protected_staging_state(backend, protector, &gate)?;
    Ok(ExistingDeviceStagedRestore {
        gate,
        staged_objects,
    })
}

fn remove_database_candidate(path: &Path) {
    let _ = std::fs::remove_file(path);
    for suffix in ["-wal", "-shm"] {
        let _ = std::fs::remove_file(PathBuf::from(format!("{}{}", path.display(), suffix)));
    }
}

pub fn verify_fresh_device_backup_restore_material<P: PortableBackupProvider>(
    provider: &mut P,
    descriptor_bytes: &[u8],
    passphrase: &str,
    expected_vault_id: VaultId,
    structured_store_staging_path: &Path,
) -> Result<FreshDeviceVerifiedBackup, FreshDeviceBackupMaterialError<P::Error>> {
    let mut staging_owned = false;
    let result = (|| {
        let mut staging = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(structured_store_staging_path)
            .map_err(|_| FreshDeviceBackupMaterialError::StagingCreateFailed)?;
        staging_owned = true;

        let pre_sqlcipher = verify_backup_semantics_before_sqlcipher(
            provider,
            descriptor_bytes,
            passphrase,
            &mut staging,
        )
        .map_err(FreshDeviceBackupMaterialError::Semantic)?;
        staging
            .sync_all()
            .map_err(|_| FreshDeviceBackupMaterialError::StagingSyncFailed)?;
        drop(staging);

        let verified = verify_staged_sqlcipher_backup(structured_store_staging_path, pre_sqlcipher)
            .map_err(FreshDeviceBackupMaterialError::SqlCipher)?;
        if verified.pre_sqlcipher().vault_id() != expected_vault_id {
            return Err(FreshDeviceBackupMaterialError::CorruptOrTampered);
        }
        Ok(FreshDeviceVerifiedBackup {
            verified_backup: verified,
        })
    })();

    if result.is_err() && staging_owned {
        remove_database_candidate(structured_store_staging_path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{
        AccessScope, FreshnessAnchor, FreshnessEpoch, HardwareBacking, KeyGeneration, ManifestHash,
        UserPresencePolicy, VaultLeaseIdentity,
    };
    use crate::vault_backup::descriptor_provider_key;
    use crate::vault_backup_creation::create_publish_and_verify_portable_backup;
    use crate::vault_backup_packaging::BackupSourceFile;
    use crate::vault_backup_restore_publication::{
        FreshDevicePublicationOperationError, FreshDeviceRestoreBackend,
        publish_prepared_fresh_device_restore,
    };
    use crate::vault_backup_verify::PreSqlCipherVerifiedBackupSet;
    use crate::vault_keys::{KeyDerivationContext, KeyPurpose, OwnedKeyMaterial};
    use crate::vault_lease::VaultLease;
    use crate::vault_manifest::{
        GenerationState, ManifestAuthMetadata, ManifestContext, ManifestGeneration, ManifestObject,
        ManifestPlaintext, RotationPhase, encrypt_fresh_manifest, manifest_hash,
    };
    use crate::vault_migration::CopyVerifyPublishStage;
    use crate::vault_nonce::NonceReservationLedger;
    use crate::vault_recovery::{RecoveryContext, encrypt_recovery_envelope};
    use crate::vault_sqlcipher::BackupSnapshotQuiescenceGuard;
    use rusqlite::Connection;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::error::Error as StdError;
    use std::fmt::Write as _;
    use std::io::{Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    const PASSPHRASE: &str = "correct horse battery staple";
    const RESTORE_BLOB_LOGICAL_ID: [u8; 16] = [0x61; 16];
    const RESTORE_BLOB_STORAGE_ID: [u8; 16] = [0x62; 16];
    const RESTORE_BLOB_PLAINTEXT: &[u8] = b"HIMSAT_B505U_RESTORE_BLOB";
    static NEXT_TEMP: AtomicU64 = AtomicU64::new(1);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ProviderError {
        Missing,
        Exists,
    }

    impl fmt::Display for ProviderError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Self::Missing => "missing",
                Self::Exists => "exists",
            })
        }
    }
    impl StdError for ProviderError {}

    #[derive(Default)]
    struct MemoryProvider {
        objects: HashMap<String, Vec<u8>>,
    }

    impl PortableBackupProvider for MemoryProvider {
        type Error = ProviderError;

        fn put_if_absent(&mut self, key: &str, bytes: &[u8]) -> Result<(), Self::Error> {
            if self.objects.contains_key(key) {
                return Err(ProviderError::Exists);
            }
            self.objects.insert(key.to_owned(), bytes.to_vec());
            Ok(())
        }

        fn read(&mut self, key: &str) -> Result<Vec<u8>, Self::Error> {
            self.objects.get(key).cloned().ok_or(ProviderError::Missing)
        }
    }

    #[derive(Default)]
    struct Guard;
    impl BackupSnapshotQuiescenceGuard for Guard {
        type Error = ();
        fn assert_normal_writes_quiesced(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct TestProtector {
        record: Option<([u8; 32], ProtectedFreshnessState)>,
        protect_calls: usize,
        unlock_calls: usize,
        genesis_calls: usize,
        replacement_key_on_genesis: Option<[u8; 32]>,
    }

    impl TestProtector {
        fn missing() -> Self {
            Self {
                record: None,
                protect_calls: 0,
                unlock_calls: 0,
                genesis_calls: 0,
                replacement_key_on_genesis: None,
            }
        }

        fn uninitialized(key: [u8; 32]) -> Self {
            Self {
                record: Some((key, ProtectedFreshnessState::Uninitialized)),
                protect_calls: 0,
                unlock_calls: 0,
                genesis_calls: 0,
                replacement_key_on_genesis: None,
            }
        }

        fn present(key: [u8; 32], anchor: FreshnessAnchor) -> Self {
            Self {
                record: Some((key, ProtectedFreshnessState::Present(anchor))),
                protect_calls: 0,
                unlock_calls: 0,
                genesis_calls: 0,
                replacement_key_on_genesis: None,
            }
        }
    }

    impl SecretProtector for TestProtector {
        type VaultRootKey = OwnedKeyMaterial;

        fn create_protector(
            &mut self,
            _requested_scope: AccessScope,
            _user_presence_policy: UserPresencePolicy,
        ) -> Result<(), ProtectorError> {
            Ok(())
        }

        fn protect_or_store_vrk(
            &mut self,
            _vault_id: VaultId,
            _key_generation: KeyGeneration,
            vrk: &Self::VaultRootKey,
        ) -> Result<(), ProtectorError> {
            self.protect_calls += 1;
            if self.record.is_some() {
                return Err(ProtectorError::CorruptOrTampered);
            }
            let bytes = vrk.with_bytes(|bytes| *bytes);
            self.record = Some((bytes, ProtectedFreshnessState::Uninitialized));
            Ok(())
        }

        fn unlock_vrk(
            &mut self,
            _vault_id: VaultId,
            _key_generation: KeyGeneration,
        ) -> Result<Self::VaultRootKey, ProtectorError> {
            self.unlock_calls += 1;
            let (key, _) = self.record.ok_or(ProtectorError::ItemMissing)?;
            Ok(OwnedKeyMaterial::from_bytes(key))
        }

        fn read_freshness_anchor(
            &self,
            _vault_id: VaultId,
        ) -> Result<ProtectedFreshnessState, ProtectorError> {
            self.record
                .as_ref()
                .map(|(_, state)| *state)
                .ok_or(ProtectorError::ItemMissing)
        }

        fn install_genesis_freshness_anchor(
            &mut self,
            _vault_id: VaultId,
            expected_state: ProtectedFreshnessState,
            new_anchor: FreshnessAnchor,
        ) -> Result<(), ProtectorError> {
            self.genesis_calls += 1;
            let replacement_key = self.replacement_key_on_genesis;
            let (key, state) = self.record.as_mut().ok_or(ProtectorError::ItemMissing)?;
            if *state != expected_state || expected_state != ProtectedFreshnessState::Uninitialized
            {
                return Err(ProtectorError::CorruptOrTampered);
            }
            if let Some(replacement_key) = replacement_key {
                *key = replacement_key;
            }
            *state = ProtectedFreshnessState::Present(new_anchor);
            Ok(())
        }

        fn advance_freshness_anchor(
            &mut self,
            _vault_id: VaultId,
            _expected_old: FreshnessAnchor,
            _new_anchor: FreshnessAnchor,
        ) -> Result<(), ProtectorError> {
            Err(ProtectorError::UnsupportedPolicy)
        }

        fn replace_protector(&mut self, _vault_id: VaultId) -> Result<(), ProtectorError> {
            Err(ProtectorError::UnsupportedPolicy)
        }

        fn remove_protector(&mut self, _vault_id: VaultId) -> Result<(), ProtectorError> {
            self.record = None;
            Ok(())
        }

        fn actual_access_scope(&self) -> AccessScope {
            AccessScope::SameUserAccount
        }

        fn requires_user_presence(&self) -> bool {
            false
        }

        fn hardware_backed_state(&self) -> HardwareBacking {
            HardwareBacking::Unknown
        }
    }

    fn vault_id() -> VaultId {
        VaultId::from_bytes([0x42; 16])
    }
    fn generation() -> KeyGeneration {
        KeyGeneration::new(7).unwrap()
    }
    fn vrk() -> OwnedKeyMaterial {
        OwnedKeyMaterial::from_bytes([0x31; 32])
    }
    fn unused_path(label: &str) -> PathBuf {
        let id = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("himsat-b505q-{label}-{}-{id}", std::process::id()))
    }
    fn raw_key_pragma(vrk: &OwnedKeyMaterial) -> String {
        let context =
            KeyDerivationContext::new(vault_id(), generation(), KeyPurpose::StructuredStore);
        context.derive_purpose_key(vrk).with_bytes(|bytes| {
            let mut hex = String::with_capacity(bytes.len() * 2);
            for byte in bytes {
                write!(&mut hex, "{byte:02x}").unwrap();
            }
            format!("PRAGMA key = \"x'{hex}'\";")
        })
    }
    fn file_identity(path: &Path) -> (u64, [u8; 32]) {
        let bytes = std::fs::read(path).unwrap();
        (bytes.len() as u64, Sha256::digest(bytes).into())
    }

    fn materialized_hash_for_test(path: &Path) -> [u8; 32] {
        Sha256::digest(std::fs::read(path).unwrap()).into()
    }

    struct Fixture {
        provider: MemoryProvider,
        descriptor: Vec<u8>,
        source_db: PathBuf,
        source_blob: PathBuf,
        open_source: Connection,
    }

    fn fixture() -> Fixture {
        let key = vrk();
        let source_db = unused_path("source.sqlite3");
        let open_source = Connection::open(&source_db).unwrap();
        open_source.execute_batch(&raw_key_pragma(&key)).unwrap();
        let mode = open_source
            .query_row("PRAGMA journal_mode = WAL;", [], |row| {
                row.get::<_, String>(0)
            })
            .unwrap();
        assert_eq!(mode, "wal");
        open_source
            .execute_batch(
                "PRAGMA wal_autocheckpoint = 0; CREATE TABLE restore_probe (value INTEGER NOT NULL); BEGIN IMMEDIATE; INSERT INTO restore_probe VALUES (73); COMMIT;",
            )
            .unwrap();
        let (db_len, db_sha) = file_identity(&source_db);
        let mut ledger = NonceReservationLedger::new(vault_id());
        let blob_context =
            BoundedBlobContext::new(vault_id(), RESTORE_BLOB_LOGICAL_ID, generation());
        let blob = ledger
            .encrypt_fresh_bounded_blob(&key, blob_context, RESTORE_BLOB_PLAINTEXT)
            .unwrap();
        let blob_nonce = blob.reservation().nonce();
        let blob_envelope = blob.into_parts().1;
        let source_blob = unused_path("source-blob.bin");
        std::fs::write(&source_blob, &blob_envelope).unwrap();
        let epoch = FreshnessEpoch::new(9).unwrap();
        let manifest_plaintext = ManifestPlaintext::new(
            vault_id(),
            epoch,
            ManifestHash::from_bytes([0x81; 32]),
            generation(),
            (RotationPhase::None, None),
            vec![ManifestGeneration::new(
                generation(),
                GenerationState::Active,
            )],
            vec![
                ManifestObject::new(
                    RESTORE_BLOB_LOGICAL_ID,
                    RESTORE_BLOB_STORAGE_ID,
                    generation(),
                    blob_envelope.len() as u64,
                    Sha256::digest(&blob_envelope).into(),
                    ManifestAuthMetadata::GenericArtifactBlob { nonce: blob_nonce },
                ),
                ManifestObject::new(
                    [0; 16],
                    [0x54; 16],
                    generation(),
                    db_len,
                    db_sha,
                    ManifestAuthMetadata::StructuredStore,
                ),
            ],
        )
        .unwrap();
        let (_, manifest) = encrypt_fresh_manifest(
            &mut ledger,
            &key,
            ManifestContext::new(vault_id(), generation(), epoch),
            &manifest_plaintext,
        )
        .unwrap();
        let anchor = FreshnessAnchor::new(vault_id(), epoch, manifest_hash(&manifest));
        let recovery = encrypt_recovery_envelope(
            &key,
            RecoveryContext::new(vault_id(), generation()),
            PASSPHRASE,
        )
        .unwrap();
        let snapshot = unused_path("creation-snapshot.sqlite3");
        let package = unused_path("creation-package");
        let verify = unused_path("creation-verify.sqlite3");
        let lease = VaultLease::new(VaultLeaseIdentity::new(vault_id(), generation()));
        let mut provider = MemoryProvider::default();
        let generic_sources = [BackupSourceFile::new(
            RESTORE_BLOB_STORAGE_ID,
            source_blob.clone(),
        )];
        let accepted = create_publish_and_verify_portable_backup(
            &mut Guard,
            &mut provider,
            anchor,
            lease.keyed_handle_lease(),
            &key,
            &recovery,
            PASSPHRASE,
            &manifest,
            &source_db,
            &generic_sources,
            &snapshot,
            &package,
            &verify,
        )
        .unwrap();
        let descriptor = provider
            .objects
            .get(&descriptor_provider_key(accepted.set_id()))
            .unwrap()
            .clone();
        Fixture {
            provider,
            descriptor,
            source_db,
            source_blob,
            open_source,
        }
    }

    fn cleanup(fixture: Fixture) {
        drop(fixture.open_source);
        remove_database_candidate(&fixture.source_db);
        let _ = std::fs::remove_file(&fixture.source_blob);
    }

    fn verified_material(
        fixture: &mut Fixture,
        label: &str,
    ) -> (FreshDeviceVerifiedBackup, PathBuf) {
        let staging = unused_path(label);
        let material = verify_fresh_device_backup_restore_material(
            &mut fixture.provider,
            &fixture.descriptor,
            PASSPHRASE,
            vault_id(),
            &staging,
        )
        .unwrap();
        (material, staging)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum PublicationBackendStep {
        Stage,
        VerifyStaged,
        Publish,
        Reopen,
        VerifyReopened,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum PublicationBackendError {
        Injected(PublicationBackendStep),
        InvalidState,
    }

    impl fmt::Display for PublicationBackendError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Injected(step) => write!(f, "injected publication failure at {step:?}"),
                Self::InvalidState => f.write_str("publication backend state is invalid"),
            }
        }
    }

    impl StdError for PublicationBackendError {}

    struct TestPublicationBackend {
        fail_at: Option<PublicationBackendStep>,
        calls: Vec<PublicationBackendStep>,
        staged_sqlcipher: Option<Vec<u8>>,
        staged_blobs: Vec<([u8; 16], [u8; 16], Vec<u8>)>,
        published_manifest: Option<Vec<u8>>,
        tamper_published_read: bool,
        reopened: bool,
    }

    impl TestPublicationBackend {
        fn new() -> Self {
            Self {
                fail_at: None,
                calls: Vec::new(),
                staged_sqlcipher: None,
                staged_blobs: Vec::new(),
                published_manifest: None,
                tamper_published_read: false,
                reopened: false,
            }
        }

        fn record(&mut self, step: PublicationBackendStep) -> Result<(), PublicationBackendError> {
            self.calls.push(step);
            if self.fail_at == Some(step) {
                return Err(PublicationBackendError::Injected(step));
            }
            Ok(())
        }
    }

    impl FreshDeviceRestoreBackend for TestPublicationBackend {
        type Error = PublicationBackendError;

        fn stage_verified_backup(
            &mut self,
            structured_store_source: &Path,
            verified: &PreSqlCipherVerifiedBackupSet,
        ) -> Result<(), Self::Error> {
            self.record(PublicationBackendStep::Stage)?;
            self.staged_sqlcipher = Some(
                std::fs::read(structured_store_source)
                    .map_err(|_| PublicationBackendError::InvalidState)?,
            );
            self.staged_blobs = verified
                .generic_artifacts()
                .iter()
                .map(|blob| {
                    (
                        blob.logical_id(),
                        blob.source_storage_id(),
                        blob.envelope().to_vec(),
                    )
                })
                .collect();
            Ok(())
        }

        fn verify_staged_backup(
            &mut self,
            _vrk: &OwnedKeyMaterial,
            verified: &PreSqlCipherVerifiedBackupSet,
        ) -> Result<(), Self::Error> {
            self.record(PublicationBackendStep::VerifyStaged)?;
            let bytes = self
                .staged_sqlcipher
                .as_ref()
                .ok_or(PublicationBackendError::InvalidState)?;
            let length =
                u64::try_from(bytes.len()).map_err(|_| PublicationBackendError::InvalidState)?;
            let sha256: [u8; 32] = Sha256::digest(bytes).into();
            if length != verified.structured_store_exact_length()
                || sha256 != verified.structured_store_exact_sha256()
                || self.staged_blobs.len() != verified.generic_artifacts().len()
            {
                return Err(PublicationBackendError::InvalidState);
            }
            for (actual, expected) in self
                .staged_blobs
                .iter()
                .zip(verified.generic_artifacts().iter())
            {
                if actual.0 != expected.logical_id()
                    || actual.1 != expected.source_storage_id()
                    || actual.2.as_slice() != expected.envelope()
                {
                    return Err(PublicationBackendError::InvalidState);
                }
            }
            Ok(())
        }

        fn publish_manifest(&mut self, envelope: &[u8]) -> Result<(), Self::Error> {
            self.record(PublicationBackendStep::Publish)?;
            self.published_manifest = Some(envelope.to_vec());
            Ok(())
        }

        fn read_published_manifest(&self) -> Result<Vec<u8>, Self::Error> {
            let mut manifest = self
                .published_manifest
                .clone()
                .ok_or(PublicationBackendError::InvalidState)?;
            if self.tamper_published_read {
                let last = manifest
                    .last_mut()
                    .ok_or(PublicationBackendError::InvalidState)?;
                *last ^= 1;
            }
            Ok(manifest)
        }

        fn reopen_published(
            &mut self,
            _vrk: &OwnedKeyMaterial,
            manifest: &ManifestPlaintext,
        ) -> Result<(), Self::Error> {
            self.record(PublicationBackendStep::Reopen)?;
            if self.published_manifest.is_none() || manifest.vault_id() != vault_id() {
                return Err(PublicationBackendError::InvalidState);
            }
            self.reopened = true;
            Ok(())
        }

        fn verify_reopened(
            &mut self,
            _vrk: &OwnedKeyMaterial,
            manifest: &ManifestPlaintext,
        ) -> Result<(), Self::Error> {
            self.record(PublicationBackendStep::VerifyReopened)?;
            if !self.reopened || manifest.vault_id() != vault_id() {
                return Err(PublicationBackendError::InvalidState);
            }
            Ok(())
        }
    }

    fn prepared_material(
        fixture: &mut Fixture,
        label: &str,
    ) -> (FreshDevicePreparedGenesis, PathBuf, TestProtector) {
        let (material, staging) = verified_material(fixture, label);
        let mut protector = TestProtector::uninitialized([0x31; 32]);
        let prepared = prepare_verified_fresh_device_genesis(
            &mut protector,
            material,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        )
        .unwrap();
        (prepared, staging, protector)
    }

    #[test]
    fn authenticated_backup_yields_verified_fresh_device_material() {
        let mut fixture = fixture();
        let staging = unused_path("restore.sqlite3");
        let material = verify_fresh_device_backup_restore_material(
            &mut fixture.provider,
            &fixture.descriptor,
            PASSPHRASE,
            vault_id(),
            &staging,
        )
        .unwrap();
        assert_eq!(
            material.verified_backup().pre_sqlcipher().vault_id(),
            vault_id()
        );
        assert!(staging.is_file());
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn provider_authentication_failure_removes_unaccepted_staging() {
        let mut fixture = fixture();
        let mut descriptor = fixture.descriptor.clone();
        let last = descriptor.len() - 1;
        descriptor[last] ^= 1;
        let staging = unused_path("tampered.sqlite3");
        let result = verify_fresh_device_backup_restore_material(
            &mut fixture.provider,
            &descriptor,
            PASSPHRASE,
            vault_id(),
            &staging,
        );
        assert!(matches!(
            result,
            Err(FreshDeviceBackupMaterialError::Semantic(_))
        ));
        assert!(!staging.exists());
        cleanup(fixture);
    }

    #[test]
    fn wrong_expected_vault_is_rejected_after_complete_backup_verification() {
        let mut fixture = fixture();
        let staging = unused_path("wrong-vault.sqlite3");
        let result = verify_fresh_device_backup_restore_material(
            &mut fixture.provider,
            &fixture.descriptor,
            PASSPHRASE,
            VaultId::from_bytes([0x99; 16]),
            &staging,
        );
        assert!(matches!(
            result,
            Err(FreshDeviceBackupMaterialError::CorruptOrTampered)
        ));
        assert!(!staging.exists());
        cleanup(fixture);
    }

    #[test]
    fn existing_uninitialized_matching_vrk_prepares_genesis_without_replacement() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "genesis-existing.sqlite3");
        let mut protector = TestProtector::uninitialized([0x31; 32]);
        let prepared = prepare_verified_fresh_device_genesis(
            &mut protector,
            material,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        )
        .unwrap();
        assert_eq!(protector.protect_calls, 0);
        assert_eq!(protector.unlock_calls, 1);
        assert!(!prepared.genesis().global_newestness_proven());
        assert_eq!(prepared.genesis().manifest().vault_id(), vault_id());
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_uninitialized_mismatched_vrk_fails_without_replacement() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "genesis-mismatch.sqlite3");
        let mut protector = TestProtector::uninitialized([0x99; 32]);
        let result = prepare_verified_fresh_device_genesis(
            &mut protector,
            material,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        );
        assert!(matches!(
            result,
            Err(FreshDeviceGenesisGateError::RecoveredKeyMismatch)
        ));
        assert_eq!(protector.protect_calls, 0);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn present_anchor_rejects_fresh_device_genesis_without_vrk_mutation() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "genesis-present.sqlite3");
        let anchor = FreshnessAnchor::new(
            vault_id(),
            FreshnessEpoch::new(12).unwrap(),
            ManifestHash::from_bytes([0x55; 32]),
        );
        let mut protector = TestProtector::present([0x31; 32], anchor);
        let result = prepare_verified_fresh_device_genesis(
            &mut protector,
            material,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        );
        assert!(matches!(
            result,
            Err(FreshDeviceGenesisGateError::Restore(
                RestoreError::AnchorAlreadyInitialized
            ))
        ));
        assert_eq!(protector.protect_calls, 0);
        assert_eq!(protector.unlock_calls, 0);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn item_missing_is_the_only_path_that_stores_recovered_vrk() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "genesis-missing.sqlite3");
        let mut protector = TestProtector::missing();
        let prepared = prepare_verified_fresh_device_genesis(
            &mut protector,
            material,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        )
        .unwrap();
        assert_eq!(protector.protect_calls, 1);
        assert_eq!(protector.unlock_calls, 1);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Uninitialized
        );
        assert!(!prepared.genesis().global_newestness_proven());
        remove_database_candidate(&staging);
        cleanup(fixture);
    }
    #[test]
    fn prepared_backup_is_published_anchored_and_reopened_in_b307_order() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-success.sqlite3");
        let accepted_anchor = prepared.genesis().accepted_anchor();
        let mut backend = TestPublicationBackend::new();

        let complete =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap();

        assert_eq!(complete.accepted_anchor(), accepted_anchor);
        assert!(!complete.global_newestness_proven());
        assert_eq!(protector.genesis_calls, 1);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Present(accepted_anchor)
        );
        assert_eq!(
            backend.calls,
            vec![
                PublicationBackendStep::Stage,
                PublicationBackendStep::VerifyStaged,
                PublicationBackendStep::Publish,
                PublicationBackendStep::Reopen,
                PublicationBackendStep::VerifyReopened,
            ]
        );
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn staging_mutation_after_b505q_fails_before_local_copy_or_anchor() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-source-mutation.sqlite3");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&staging)
            .unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(&[0xff]).unwrap();
        file.sync_all().unwrap();
        drop(file);
        let mut backend = TestPublicationBackend::new();

        let error =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap_err();

        assert_eq!(error.stage(), CopyVerifyPublishStage::SourceVerification);
        assert!(matches!(
            error.source_error(),
            FreshDevicePublicationOperationError::SourceSqlCipher(
                BackupSqlCipherVerificationError::FileIdentityMismatch
            )
        ));
        assert!(backend.calls.is_empty());
        assert_eq!(protector.genesis_calls, 0);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Uninitialized
        );
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn tampered_published_manifest_fails_before_genesis_anchor() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-manifest-tamper.sqlite3");
        let mut backend = TestPublicationBackend::new();
        backend.tamper_published_read = true;

        let error =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap_err();

        assert_eq!(error.stage(), CopyVerifyPublishStage::Publication);
        assert!(matches!(
            error.source_error(),
            FreshDevicePublicationOperationError::PublishedManifestMismatch
        ));
        assert_eq!(protector.genesis_calls, 0);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Uninitialized
        );
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn protected_state_drift_is_rejected_without_overwrite() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-protector-drift.sqlite3");
        let competing_anchor = FreshnessAnchor::new(
            vault_id(),
            FreshnessEpoch::new(12).unwrap(),
            ManifestHash::from_bytes([0x99; 32]),
        );
        protector.record.as_mut().unwrap().1 = ProtectedFreshnessState::Present(competing_anchor);
        let mut backend = TestPublicationBackend::new();

        let error =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap_err();

        assert_eq!(error.stage(), CopyVerifyPublishStage::Anchor);
        assert!(matches!(
            error.source_error(),
            FreshDevicePublicationOperationError::ProtectedStateChanged
        ));
        assert_eq!(protector.genesis_calls, 0);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Present(competing_anchor)
        );
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn post_anchor_reopen_failure_preserves_installed_anchor_for_roll_forward() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-reopen-failure.sqlite3");
        let accepted_anchor = prepared.genesis().accepted_anchor();
        let mut backend = TestPublicationBackend::new();
        backend.fail_at = Some(PublicationBackendStep::Reopen);

        let error =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap_err();

        assert_eq!(error.stage(), CopyVerifyPublishStage::Reopen);
        assert_eq!(protector.genesis_calls, 1);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Present(accepted_anchor)
        );
        remove_database_candidate(&staging);
        cleanup(fixture);
    }
    #[test]
    fn protected_vrk_drift_before_anchor_is_rejected_without_genesis() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-vrk-drift.sqlite3");
        protector.record.as_mut().unwrap().0 = [0x88; 32];
        let mut backend = TestPublicationBackend::new();

        let error =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap_err();

        assert_eq!(error.stage(), CopyVerifyPublishStage::Anchor);
        assert!(matches!(
            error.source_error(),
            FreshDevicePublicationOperationError::ProtectedStateChanged
        ));
        assert_eq!(protector.genesis_calls, 0);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Uninitialized
        );
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn protected_vrk_drift_during_genesis_is_detected_before_backend_reopen() {
        let mut fixture = fixture();
        let (prepared, staging, mut protector) =
            prepared_material(&mut fixture, "publication-vrk-genesis-drift.sqlite3");
        let accepted_anchor = prepared.genesis().accepted_anchor();
        protector.replacement_key_on_genesis = Some([0x88; 32]);
        let mut backend = TestPublicationBackend::new();

        let error =
            publish_prepared_fresh_device_restore(&mut protector, &mut backend, &staging, prepared)
                .unwrap_err();

        assert_eq!(error.stage(), CopyVerifyPublishStage::Reopen);
        assert!(matches!(
            error.source_error(),
            FreshDevicePublicationOperationError::ProtectedStateChanged
        ));
        assert_eq!(protector.genesis_calls, 1);
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Present(accepted_anchor)
        );
        assert!(!backend.calls.contains(&PublicationBackendStep::Reopen));
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ExistingGateBackendError {
        Quiescence,
        InvalidInventory,
    }

    impl fmt::Display for ExistingGateBackendError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Self::Quiescence => "restore writes are not quiesced",
                Self::InvalidInventory => "current inventory verification failed",
            })
        }
    }

    impl StdError for ExistingGateBackendError {}

    struct TestExistingGateBackend {
        expected_key: [u8; 32],
        expected_generation: KeyGeneration,
        assert_calls: usize,
        inventory_calls: usize,
        fail_quiescence: bool,
        fail_inventory: bool,
    }

    impl TestExistingGateBackend {
        fn new(expected_key: [u8; 32], expected_generation: KeyGeneration) -> Self {
            Self {
                expected_key,
                expected_generation,
                assert_calls: 0,
                inventory_calls: 0,
                fail_quiescence: false,
                fail_inventory: false,
            }
        }
    }

    impl ExistingDeviceRestoreGateBackend for TestExistingGateBackend {
        type Error = ExistingGateBackendError;

        fn assert_normal_writes_quiesced(&mut self) -> Result<(), Self::Error> {
            self.assert_calls += 1;
            if self.fail_quiescence {
                Err(ExistingGateBackendError::Quiescence)
            } else {
                Ok(())
            }
        }

        fn verify_current_inventory(
            &mut self,
            current_vrk: &OwnedKeyMaterial,
            manifest: &ManifestPlaintext,
        ) -> Result<(), Self::Error> {
            self.inventory_calls += 1;
            let key_matches = current_vrk.with_bytes(|bytes| *bytes == self.expected_key);
            let inventory_matches = manifest
                .objects()
                .iter()
                .all(|object| object.key_generation() == self.expected_generation);
            if self.fail_inventory || !key_matches || !inventory_matches {
                Err(ExistingGateBackendError::InvalidInventory)
            } else {
                Ok(())
            }
        }
    }

    struct TestExistingTargetBackend {
        gate: TestExistingGateBackend,
        sqlcipher_paths: HashMap<[u8; 16], PathBuf>,
        staged_blobs: HashMap<[u8; 16], Vec<u8>>,
        retained_blobs: HashMap<NonceReservation, ExistingDeviceRetainedBlob>,
        force_unavailable: bool,
        availability_calls: usize,
    }

    impl TestExistingTargetBackend {
        fn new(expected_key: [u8; 32], expected_generation: KeyGeneration) -> Self {
            Self {
                gate: TestExistingGateBackend::new(expected_key, expected_generation),
                sqlcipher_paths: HashMap::new(),
                staged_blobs: HashMap::new(),
                retained_blobs: HashMap::new(),
                force_unavailable: false,
                availability_calls: 0,
            }
        }

        fn cleanup(&self) {
            for path in self.sqlcipher_paths.values() {
                remove_database_candidate(path);
            }
        }
    }

    impl ExistingDeviceRestoreGateBackend for TestExistingTargetBackend {
        type Error = ExistingGateBackendError;

        fn assert_normal_writes_quiesced(&mut self) -> Result<(), Self::Error> {
            self.gate.assert_normal_writes_quiesced()
        }

        fn verify_current_inventory(
            &mut self,
            current_vrk: &OwnedKeyMaterial,
            manifest: &ManifestPlaintext,
        ) -> Result<(), Self::Error> {
            self.gate.verify_current_inventory(current_vrk, manifest)
        }
    }

    impl ExistingDeviceRestoreTargetBackend for TestExistingTargetBackend {
        fn target_storage_available(&mut self, storage_id: [u8; 16]) -> Result<bool, Self::Error> {
            self.availability_calls += 1;
            Ok(!self.force_unavailable
                && !self.sqlcipher_paths.contains_key(&storage_id)
                && !self.staged_blobs.contains_key(&storage_id))
        }

        fn sqlcipher_target_path(&mut self, storage_id: [u8; 16]) -> Result<PathBuf, Self::Error> {
            Ok(self
                .sqlcipher_paths
                .entry(storage_id)
                .or_insert_with(|| unused_path("existing-target.sqlite3"))
                .clone())
        }

        fn stage_blob(&mut self, storage_id: [u8; 16], envelope: &[u8]) -> Result<(), Self::Error> {
            if self.staged_blobs.contains_key(&storage_id) {
                return Err(ExistingGateBackendError::InvalidInventory);
            }
            self.staged_blobs.insert(storage_id, envelope.to_vec());
            Ok(())
        }

        fn read_staged_blob(&mut self, storage_id: [u8; 16]) -> Result<Vec<u8>, Self::Error> {
            self.staged_blobs
                .get(&storage_id)
                .cloned()
                .ok_or(ExistingGateBackendError::InvalidInventory)
        }

        fn retained_blob_for_reservation(
            &mut self,
            reservation: NonceReservation,
        ) -> Result<Option<ExistingDeviceRetainedBlob>, Self::Error> {
            Ok(self.retained_blobs.get(&reservation).cloned())
        }
    }

    fn current_state(
        key: &OwnedKeyMaterial,
        current_generation: KeyGeneration,
        epoch_value: u64,
        previous_hash: ManifestHash,
        generations: Vec<ManifestGeneration>,
        objects: Vec<ManifestObject>,
        rotation: (RotationPhase, Option<KeyGeneration>),
    ) -> (Vec<u8>, FreshnessAnchor) {
        let epoch = FreshnessEpoch::new(epoch_value).unwrap();
        let manifest = ManifestPlaintext::new(
            vault_id(),
            epoch,
            previous_hash,
            current_generation,
            rotation,
            generations,
            objects,
        )
        .unwrap();
        let context = ManifestContext::new(vault_id(), current_generation, epoch);
        let mut ledger = NonceReservationLedger::new(vault_id());
        let (_, envelope) = encrypt_fresh_manifest(&mut ledger, key, context, &manifest).unwrap();
        let anchor = FreshnessAnchor::new(vault_id(), epoch, manifest_hash(&envelope));
        (envelope, anchor)
    }

    fn stable_current_state(
        key: &OwnedKeyMaterial,
        current_generation: KeyGeneration,
        epoch_value: u64,
    ) -> (Vec<u8>, FreshnessAnchor) {
        current_state(
            key,
            current_generation,
            epoch_value,
            ManifestHash::from_bytes([0xa5; 32]),
            vec![ManifestGeneration::new(
                current_generation,
                GenerationState::Active,
            )],
            vec![ManifestObject::new(
                [0; 16],
                [0x91; 16],
                current_generation,
                4096,
                [0x92; 32],
                ManifestAuthMetadata::StructuredStore,
            )],
            (RotationPhase::None, None),
        )
    }

    #[test]
    fn existing_device_restore_gate_accepts_stable_same_generation_identity() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "existing-gate-same.sqlite3");
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x31; 32], generation());

        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .unwrap();

        assert_eq!(gate.route(), ExistingDeviceRestoreRoute::SameGeneration);
        assert_eq!(gate.trusted_anchor(), current_anchor);
        assert_eq!(
            gate.current_manifest().decision(),
            FreshnessDecision::Current
        );
        assert!(gate.with_current_vrk(|key| key_material_equal(key, &current_key)));
        assert_eq!(backend.assert_calls, 2);
        assert_eq!(backend.inventory_calls, 1);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_routes_older_generation_to_rebase() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "existing-gate-cross.sqlite3");
        let current_generation = KeyGeneration::new(8).unwrap();
        let current_key = OwnedKeyMaterial::from_bytes([0x44; 32]);
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, current_generation, 10);
        let mut protector = TestProtector::present([0x44; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x44; 32], current_generation);

        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), current_generation),
            &current_envelope,
            material,
        )
        .unwrap();

        assert_eq!(gate.route(), ExistingDeviceRestoreRoute::CrossGeneration);
        assert_eq!(gate.current_identity().key_generation(), current_generation);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_target_staging_same_generation_preserves_ciphertext() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "target-same.sqlite3");
        let source_blob = material
            .verified_backup()
            .pre_sqlcipher()
            .generic_artifacts()[0]
            .envelope()
            .to_vec();
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingTargetBackend::new([0x31; 32], generation());
        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .unwrap();
        let mut nonce_ledger = NonceReservationLedger::new(vault_id());
        let staged = stage_existing_device_restore_targets(
            &mut backend,
            &mut protector,
            &staging,
            &mut nonce_ledger,
            gate,
        )
        .unwrap();

        assert_eq!(staged.staged_objects().len(), 2);
        assert!(
            staged
                .staged_objects()
                .iter()
                .all(|object| object.key_generation() == generation())
        );
        let structured = staged
            .staged_objects()
            .iter()
            .find(|object| object.auth_metadata() == ManifestAuthMetadata::StructuredStore)
            .unwrap();
        let target_path = backend
            .sqlcipher_paths
            .get(&structured.storage_id())
            .unwrap();
        assert_eq!(
            std::fs::read(target_path).unwrap(),
            std::fs::read(&staging).unwrap()
        );
        let blob = staged
            .staged_objects()
            .iter()
            .find(|object| object.logical_id() == RESTORE_BLOB_LOGICAL_ID)
            .unwrap();
        assert_eq!(
            backend.staged_blobs.get(&blob.storage_id()).unwrap(),
            &source_blob
        );
        assert_ne!(structured.storage_id(), [0x54; 16]);
        assert_ne!(blob.storage_id(), RESTORE_BLOB_STORAGE_ID);
        assert_ne!(structured.storage_id(), blob.storage_id());
        assert_eq!(
            protector.read_freshness_anchor(vault_id()).unwrap(),
            ProtectedFreshnessState::Present(current_anchor)
        );
        backend.cleanup();
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_target_staging_cross_generation_reencrypts_under_current_key() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "target-cross.sqlite3");
        let source_blob = material
            .verified_backup()
            .pre_sqlcipher()
            .generic_artifacts()[0]
            .envelope()
            .to_vec();
        let current_generation = KeyGeneration::new(8).unwrap();
        let current_key = OwnedKeyMaterial::from_bytes([0x44; 32]);
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, current_generation, 10);
        let mut protector = TestProtector::present([0x44; 32], current_anchor);
        let mut backend = TestExistingTargetBackend::new([0x44; 32], current_generation);
        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), current_generation),
            &current_envelope,
            material,
        )
        .unwrap();
        let mut nonce_ledger = NonceReservationLedger::new(vault_id());
        let staged = stage_existing_device_restore_targets(
            &mut backend,
            &mut protector,
            &staging,
            &mut nonce_ledger,
            gate,
        )
        .unwrap();

        assert!(
            staged
                .staged_objects()
                .iter()
                .all(|object| object.key_generation() == current_generation)
        );
        let blob = staged
            .staged_objects()
            .iter()
            .find(|object| object.logical_id() == RESTORE_BLOB_LOGICAL_ID)
            .unwrap();
        let target_blob = backend.staged_blobs.get(&blob.storage_id()).unwrap();
        assert_ne!(target_blob, &source_blob);
        let plaintext = decrypt_bounded_blob(
            &current_key,
            BoundedBlobContext::new(vault_id(), RESTORE_BLOB_LOGICAL_ID, current_generation),
            target_blob,
        )
        .unwrap();
        assert_eq!(plaintext, RESTORE_BLOB_PLAINTEXT);
        let structured = staged
            .staged_objects()
            .iter()
            .find(|object| object.auth_metadata() == ManifestAuthMetadata::StructuredStore)
            .unwrap();
        assert_ne!(
            structured.ciphertext_sha256(),
            materialized_hash_for_test(&staging)
        );
        backend.cleanup();
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_target_staging_rejects_conflicting_same_generation_nonce() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "target-conflict.sqlite3");
        let source_blob = &material
            .verified_backup()
            .pre_sqlcipher()
            .generic_artifacts()[0];
        let nonce = bounded_blob_nonce(source_blob.envelope()).unwrap();
        let reservation =
            NonceReservation::new(vault_id(), NoncePurpose::BoundedBlob, generation(), nonce);
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingTargetBackend::new([0x31; 32], generation());
        backend.retained_blobs.insert(
            reservation,
            ExistingDeviceRetainedBlob::new([0x77; 16], source_blob.envelope().to_vec()),
        );
        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .unwrap();
        let mut nonce_ledger = NonceReservationLedger::new(vault_id());
        nonce_ledger
            .record_authenticated_canonical_reservation(reservation)
            .unwrap();
        let error = stage_existing_device_restore_targets(
            &mut backend,
            &mut protector,
            &staging,
            &mut nonce_ledger,
            gate,
        )
        .err()
        .unwrap();
        assert!(matches!(
            error,
            ExistingDeviceRestoreStagingError::NonceReservationConflict
        ));
        backend.cleanup();
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_target_staging_fails_closed_when_no_target_id_is_available() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "target-id-exhausted.sqlite3");
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingTargetBackend::new([0x31; 32], generation());
        backend.force_unavailable = true;
        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .unwrap();
        let mut nonce_ledger = NonceReservationLedger::new(vault_id());
        let error = stage_existing_device_restore_targets(
            &mut backend,
            &mut protector,
            &staging,
            &mut nonce_ledger,
            gate,
        )
        .err()
        .unwrap();
        assert!(matches!(
            error,
            ExistingDeviceRestoreStagingError::TargetStorageIdExhausted
        ));
        assert_eq!(backend.availability_calls, TARGET_STORAGE_ID_ATTEMPTS);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_rejects_generation_ahead() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "existing-gate-ahead.sqlite3");
        let current_generation = KeyGeneration::new(6).unwrap();
        let current_key = OwnedKeyMaterial::from_bytes([0x44; 32]);
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, current_generation, 10);
        let mut protector = TestProtector::present([0x44; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x44; 32], current_generation);

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), current_generation),
            &current_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::RestoreGenerationAhead
        ));
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_rejects_same_generation_key_mismatch() {
        let mut fixture = fixture();
        let (material, staging) =
            verified_material(&mut fixture, "existing-gate-key-mismatch.sqlite3");
        let current_key = OwnedKeyMaterial::from_bytes([0x44; 32]);
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x44; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x44; 32], generation());

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::KeyGenerationIdentityMismatch
        ));
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_rejects_interrupted_publication() {
        let mut fixture = fixture();
        let (material, staging) =
            verified_material(&mut fixture, "existing-gate-interrupted.sqlite3");
        let current_key = vrk();
        let (old_envelope, old_anchor) = stable_current_state(&current_key, generation(), 10);
        assert_eq!(manifest_hash(&old_envelope), old_anchor.manifest_hash());
        let (candidate_envelope, _) = current_state(
            &current_key,
            generation(),
            11,
            old_anchor.manifest_hash(),
            vec![ManifestGeneration::new(
                generation(),
                GenerationState::Active,
            )],
            vec![ManifestObject::new(
                [0; 16],
                [0x91; 16],
                generation(),
                4096,
                [0x92; 32],
                ManifestAuthMetadata::StructuredStore,
            )],
            (RotationPhase::None, None),
        );
        let mut protector = TestProtector::present([0x31; 32], old_anchor);
        let mut backend = TestExistingGateBackend::new([0x31; 32], generation());

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &candidate_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::RestoreStateNotStable
        ));
        assert_eq!(backend.inventory_calls, 0);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_accepts_stable_retained_history() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "existing-gate-retained.sqlite3");
        let current_generation = KeyGeneration::new(8).unwrap();
        let current_key = OwnedKeyMaterial::from_bytes([0x44; 32]);
        let (current_envelope, current_anchor) = current_state(
            &current_key,
            current_generation,
            10,
            ManifestHash::from_bytes([0xa5; 32]),
            vec![
                ManifestGeneration::new(generation(), GenerationState::Retained),
                ManifestGeneration::new(current_generation, GenerationState::Active),
            ],
            vec![ManifestObject::new(
                [0; 16],
                [0x91; 16],
                current_generation,
                4096,
                [0x92; 32],
                ManifestAuthMetadata::StructuredStore,
            )],
            (RotationPhase::None, None),
        );
        let mut protector = TestProtector::present([0x44; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x44; 32], current_generation);

        let gate = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), current_generation),
            &current_envelope,
            material,
        )
        .unwrap();

        assert_eq!(gate.route(), ExistingDeviceRestoreRoute::CrossGeneration);
        assert_eq!(gate.current_manifest().manifest().generations().len(), 2);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_rejects_staged_generation_without_rotation() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "existing-gate-staged.sqlite3");
        let current_generation = KeyGeneration::new(8).unwrap();
        let current_key = OwnedKeyMaterial::from_bytes([0x44; 32]);
        let (current_envelope, current_anchor) = current_state(
            &current_key,
            current_generation,
            10,
            ManifestHash::from_bytes([0xa5; 32]),
            vec![
                ManifestGeneration::new(generation(), GenerationState::Staged),
                ManifestGeneration::new(current_generation, GenerationState::Active),
            ],
            vec![ManifestObject::new(
                [0; 16],
                [0x91; 16],
                current_generation,
                4096,
                [0x92; 32],
                ManifestAuthMetadata::StructuredStore,
            )],
            (RotationPhase::None, None),
        );
        let mut protector = TestProtector::present([0x44; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x44; 32], current_generation);

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), current_generation),
            &current_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::RestoreStateNotStable
        ));
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_rejects_non_older_source() {
        let mut fixture = fixture();
        let (material, staging) =
            verified_material(&mut fixture, "existing-gate-not-older.sqlite3");
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 9);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x31; 32], generation());

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::SourceNotOlder
        ));
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_requires_quiescence_and_inventory_proof() {
        let mut fixture = fixture();
        let (material, staging) = verified_material(&mut fixture, "existing-gate-backend.sqlite3");
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x31; 32], generation());
        backend.fail_inventory = true;

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::Backend(ExistingGateBackendError::InvalidInventory)
        ));
        assert_eq!(backend.assert_calls, 1);
        assert_eq!(backend.inventory_calls, 1);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }

    #[test]
    fn existing_device_restore_gate_rejects_missing_quiescence_before_state_read() {
        let mut fixture = fixture();
        let (material, staging) =
            verified_material(&mut fixture, "existing-gate-no-quiescence.sqlite3");
        let current_key = vrk();
        let (current_envelope, current_anchor) =
            stable_current_state(&current_key, generation(), 10);
        let mut protector = TestProtector::present([0x31; 32], current_anchor);
        let mut backend = TestExistingGateBackend::new([0x31; 32], generation());
        backend.fail_quiescence = true;

        let error = prepare_existing_device_restore_gate(
            &mut backend,
            &mut protector,
            VaultLeaseIdentity::new(vault_id(), generation()),
            &current_envelope,
            material,
        )
        .err()
        .unwrap();

        assert!(matches!(
            error,
            ExistingDeviceRestoreGateError::Backend(ExistingGateBackendError::Quiescence)
        ));
        assert_eq!(protector.unlock_calls, 0);
        assert_eq!(backend.inventory_calls, 0);
        remove_database_candidate(&staging);
        cleanup(fixture);
    }
}
