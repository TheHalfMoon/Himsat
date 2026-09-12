//! B503 resumable seven-phase full-VRK rotation.
//!
//! The coordinator executes one durable transition per call. Source and target
//! protectors are distinct opaque provider records, so the source generation is
//! retained until the target state has been published, anchored, reopened, and
//! verified. B504 owns exhaustive before/after fault-injection qualification.

use crate::vault::{
    FreshnessAnchor, KeyGeneration, ProtectedFreshnessState, ProtectorError, SecretProtector,
    VaultId, VaultLeaseIdentity,
};
use crate::vault_keys::{
    KEY_MATERIAL_BYTES, KeyedHandleCloser, OwnedKeyMaterial, PlaintextCache, VaultSessionLifetime,
    VaultTeardownReason,
};
use crate::vault_manifest::{
    FreshManifestError, GenerationState, ManifestContext, ManifestError, ManifestGeneration,
    ManifestObject, ManifestPlaintext, RotationPhase, decrypt_manifest, encrypt_fresh_manifest,
    manifest_context, manifest_hash, manifest_nonce_reservation,
};
use crate::vault_nonce::NonceReservationLedger;
use crate::vault_protector::ProtectorPolicy;
use crate::vault_recovery::{
    RecoveryContext, RecoveryEnvelopeError, decrypt_recovery_envelope, encrypt_recovery_envelope,
};
use std::error::Error;
use std::fmt;

/// Non-secret identities that bind one full-VRK rotation attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FullRotationIdentity {
    vault_id: VaultId,
    source_generation: KeyGeneration,
    target_generation: KeyGeneration,
}

impl FullRotationIdentity {
    /// Reconstructs the deterministic B503 identity used to resume a persisted
    /// rotation after process restart. The target generation is exactly source+1.
    #[must_use]
    pub fn for_next_generation(
        vault_id: VaultId,
        source_generation: KeyGeneration,
    ) -> Option<Self> {
        let value = source_generation.get().checked_add(1)?;
        let target_generation = KeyGeneration::new(value).ok()?;
        Some(Self {
            vault_id,
            source_generation,
            target_generation,
        })
    }

    #[must_use]
    pub const fn vault_id(self) -> VaultId {
        self.vault_id
    }

    #[must_use]
    pub const fn source_generation(self) -> KeyGeneration {
        self.source_generation
    }

    #[must_use]
    pub const fn target_generation(self) -> KeyGeneration {
        self.target_generation
    }
}

/// Fixed byte length of one opaque provider-visible protector identifier.
pub const ROTATION_PROTECTOR_ID_BYTES: usize = 16;

/// Durable non-secret binding for the exact source and target protector records
/// used by one B503 rotation attempt. The opaque identifiers must be distinct.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FullRotationProtectorBinding {
    source_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
    target_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
}

impl FullRotationProtectorBinding {
    #[must_use]
    pub fn new(
        source_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
        target_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
    ) -> Option<Self> {
        if source_protector_id == target_protector_id {
            return None;
        }
        Some(Self {
            source_protector_id,
            target_protector_id,
        })
    }

    #[must_use]
    pub const fn source_protector_id(self) -> [u8; ROTATION_PROTECTOR_ID_BYTES] {
        self.source_protector_id
    }

    #[must_use]
    pub const fn target_protector_id(self) -> [u8; ROTATION_PROTECTOR_ID_BYTES] {
        self.target_protector_id
    }
}

/// Proof that the prior live session published rotation revocation, drained
/// already-authorized operations, closed keyed handles, and released session keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RotationQuiesced {
    identity: VaultLeaseIdentity,
}

impl RotationQuiesced {
    #[must_use]
    pub const fn identity(self) -> VaultLeaseIdentity {
        self.identity
    }
}

/// Quiesces normal vault operations before a full root rotation begins or resumes.
pub fn quiesce_for_full_rotation<H, C>(
    session: &mut VaultSessionLifetime<H, C>,
) -> Result<RotationQuiesced, H::Error>
where
    H: KeyedHandleCloser,
    C: PlaintextCache,
{
    let identity = session.identity();
    session.teardown(VaultTeardownReason::Rotation)?;
    Ok(RotationQuiesced { identity })
}

/// Existing opt-in recovery material that must be rewrapped for the target VRK.
#[derive(Clone, Copy, Debug)]
pub struct RecoveryRotationInput<'a> {
    source_envelope: &'a [u8],
    passphrase: &'a str,
}

impl<'a> RecoveryRotationInput<'a> {
    #[must_use]
    pub const fn new(source_envelope: &'a [u8], passphrase: &'a str) -> Self {
        Self {
            source_envelope,
            passphrase,
        }
    }

    /// Returns the existing authenticated recovery envelope for the source VRK.
    #[must_use]
    pub const fn source_envelope(self) -> &'a [u8] {
        self.source_envelope
    }

    /// Returns the exact user passphrase used to authenticate and rewrap recovery.
    #[must_use]
    pub const fn passphrase(self) -> &'a str {
        self.passphrase
    }
}

/// Durable B503 progress returned after exactly one accepted phase transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FullRotationProgress {
    InProgress {
        identity: FullRotationIdentity,
        phase: RotationPhase,
    },
    Complete {
        identity: FullRotationIdentity,
        final_anchor: FreshnessAnchor,
    },
}

/// Storage/data boundary used by the B503 coordinator.
///
/// Implementations must make every method idempotent for an exact repeated input.
/// Publication means durable write/fsync plus exact reread support. Target staging
/// must create separately named encrypted objects under `target_generation`; it
/// must never mutate the only source-generation canonical object in place.
pub trait FullRotationBackend {
    type Error;

    /// Acquires or verifies the concrete exclusion that prevents ordinary vault
    /// writes for the complete coordinator call. On success the implementation
    /// must keep that exclusion effective through all subsequent backend operations
    /// performed by this call, including after process restart. A point-in-time
    /// observation that another writer is merely absent is insufficient.
    fn assert_normal_writes_quiesced(&self) -> Result<(), Self::Error>;

    fn read_rotation_protector_binding(
        &self,
    ) -> Result<Option<FullRotationProtectorBinding>, Self::Error>;
    fn persist_rotation_protector_binding(
        &mut self,
        binding: FullRotationProtectorBinding,
    ) -> Result<(), Self::Error>;
    fn clear_rotation_protector_binding(&mut self) -> Result<(), Self::Error>;

    fn read_rotation_checkpoint(&self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn persist_rotation_checkpoint(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;
    fn clear_rotation_checkpoint(&mut self) -> Result<(), Self::Error>;

    fn read_target_recovery_wrap(&self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn persist_target_recovery_wrap(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;
    fn remove_target_recovery_wrap(&mut self) -> Result<(), Self::Error>;

    fn stage_reencrypted_inventory(
        &mut self,
        source_vrk: &OwnedKeyMaterial,
        target_vrk: &OwnedKeyMaterial,
        source_manifest: &ManifestPlaintext,
        target_generation: KeyGeneration,
        nonce_ledger: &mut NonceReservationLedger,
    ) -> Result<Vec<ManifestObject>, Self::Error>;

    fn verify_staged_inventory(
        &mut self,
        target_vrk: &OwnedKeyMaterial,
        staged_objects: &[ManifestObject],
    ) -> Result<(), Self::Error>;

    fn quarantine_target_inventory(&mut self) -> Result<(), Self::Error>;

    fn publish_manifest(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;
    fn read_published_manifest(&self) -> Result<Vec<u8>, Self::Error>;

    fn reopen_and_verify_published(
        &mut self,
        target_vrk: &OwnedKeyMaterial,
        manifest: &ManifestPlaintext,
    ) -> Result<(), Self::Error>;

    fn activate_target_protector(&mut self) -> Result<(), Self::Error>;
    fn retire_source_inventory(
        &mut self,
        source_generation: KeyGeneration,
    ) -> Result<(), Self::Error>;
    fn retire_source_recovery_wrap(&mut self) -> Result<(), Self::Error>;
}

/// Exact B503 failure class. No variant authorizes rollback from PUBLISH or later.
#[derive(Debug)]
pub enum FullRotationError<E> {
    Backend { phase: RotationPhase, source: E },
    Protector(ProtectorError),
    Manifest(ManifestError),
    FreshManifest(FreshManifestError),
    Recovery(RecoveryEnvelopeError),
    InvalidState,
    GenerationOverflow,
    EpochOverflow,
    RandomnessUnavailable,
    KeyMaterialCollision,
    TargetProtectorNotEmpty,
    InventoryMismatch,
}

impl<E: fmt::Display> fmt::Display for FullRotationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend { phase, source } => {
                write!(f, "B503 {phase:?} backend failure: {source}")
            }
            Self::Protector(error) => write!(f, "B503 protector failure: {error}"),
            Self::Manifest(error) => write!(f, "B503 manifest failure: {error}"),
            Self::FreshManifest(error) => write!(f, "B503 fresh-manifest failure: {error}"),
            Self::Recovery(error) => write!(f, "B503 recovery-wrap failure: {error}"),
            Self::InvalidState => f.write_str("B503 rotation state is inconsistent"),
            Self::GenerationOverflow => f.write_str("B503 target key generation overflowed"),
            Self::EpochOverflow => f.write_str("B503 freshness epoch overflowed"),
            Self::RandomnessUnavailable => f.write_str("B503 OS randomness is unavailable"),
            Self::KeyMaterialCollision => {
                f.write_str("B503 generated target VRK equals source VRK")
            }
            Self::TargetProtectorNotEmpty => {
                f.write_str("B503 target protector is not a fresh independent record")
            }
            Self::InventoryMismatch => {
                f.write_str("B503 staged inventory does not exactly replace source logical objects")
            }
        }
    }
}

impl<E: Error + 'static> Error for FullRotationError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend { source, .. } => Some(source),
            Self::Protector(error) => Some(error),
            Self::Manifest(error) => Some(error),
            Self::FreshManifest(error) => Some(error),
            Self::Recovery(error) => Some(error),
            Self::InvalidState
            | Self::GenerationOverflow
            | Self::EpochOverflow
            | Self::RandomnessUnavailable
            | Self::KeyMaterialCollision
            | Self::TargetProtectorNotEmpty
            | Self::InventoryMismatch => None,
        }
    }
}

fn backend<E>(phase: RotationPhase, source: E) -> FullRotationError<E> {
    FullRotationError::Backend { phase, source }
}

fn require_retained_manifest_reservation<E>(
    ledger: &NonceReservationLedger,
    envelope: &[u8],
) -> Result<(), FullRotationError<E>> {
    let reservation = manifest_nonce_reservation(envelope).map_err(FullRotationError::Manifest)?;
    if !ledger.contains(reservation) {
        return Err(FullRotationError::InvalidState);
    }
    Ok(())
}

fn assert_backend_quiesced<B: FullRotationBackend>(
    backend_impl: &B,
    phase: RotationPhase,
) -> Result<(), FullRotationError<B::Error>> {
    backend_impl
        .assert_normal_writes_quiesced()
        .map_err(|error| backend(phase, error))
}

fn key_material_equal(left: &OwnedKeyMaterial, right: &OwnedKeyMaterial) -> bool {
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

fn generate_target_vrk<E>(
    source_vrk: &OwnedKeyMaterial,
) -> Result<OwnedKeyMaterial, FullRotationError<E>> {
    let mut bytes = [0_u8; KEY_MATERIAL_BYTES];
    getrandom::fill(&mut bytes).map_err(|_| FullRotationError::RandomnessUnavailable)?;
    let target = OwnedKeyMaterial::from_bytes(bytes);
    if key_material_equal(source_vrk, &target) {
        return Err(FullRotationError::KeyMaterialCollision);
    }
    Ok(target)
}

fn target_generations_pre<E>(
    source: &ManifestPlaintext,
    target: KeyGeneration,
) -> Result<Vec<ManifestGeneration>, FullRotationError<E>> {
    if source
        .generations()
        .iter()
        .any(|entry| entry.generation() >= target)
    {
        return Err(FullRotationError::InvalidState);
    }
    let mut generations = source.generations().to_vec();
    generations.push(ManifestGeneration::new(target, GenerationState::Staged));
    Ok(generations)
}

fn build_checkpoint<E>(
    basis: &ManifestPlaintext,
    identity: FullRotationIdentity,
    phase: RotationPhase,
    generations: Vec<ManifestGeneration>,
    objects: Vec<ManifestObject>,
) -> Result<ManifestPlaintext, FullRotationError<E>> {
    ManifestPlaintext::new(
        identity.vault_id,
        basis.freshness_epoch(),
        basis.previous_manifest_hash(),
        if matches!(
            phase,
            RotationPhase::Prepare | RotationPhase::Stage | RotationPhase::Verify
        ) {
            identity.source_generation
        } else {
            identity.target_generation
        },
        (phase, Some(identity.target_generation)),
        generations,
        objects,
    )
    .map_err(FullRotationError::Manifest)
}

fn persist_encrypted_checkpoint<B: FullRotationBackend>(
    backend_impl: &mut B,
    ledger: &mut NonceReservationLedger,
    vrk: &OwnedKeyMaterial,
    identity: FullRotationIdentity,
    plaintext: &ManifestPlaintext,
) -> Result<Vec<u8>, FullRotationError<B::Error>> {
    let context = ManifestContext::new(
        identity.vault_id,
        plaintext.active_key_generation(),
        plaintext.freshness_epoch(),
    );
    let (_, envelope) = encrypt_fresh_manifest(ledger, vrk, context, plaintext)
        .map_err(FullRotationError::FreshManifest)?;
    backend_impl
        .persist_rotation_checkpoint(&envelope)
        .map_err(|error| backend(plaintext.rotation_phase(), error))?;
    let reread = backend_impl
        .read_rotation_checkpoint()
        .map_err(|error| backend(plaintext.rotation_phase(), error))?
        .ok_or(FullRotationError::InvalidState)?;
    if reread != envelope {
        return Err(FullRotationError::InvalidState);
    }
    let authenticated =
        decrypt_manifest(vrk, context, &reread).map_err(FullRotationError::Manifest)?;
    if authenticated != *plaintext {
        return Err(FullRotationError::InvalidState);
    }
    Ok(envelope)
}

fn protector_anchor<P, E>(
    protector: &P,
    vault_id: VaultId,
) -> Result<FreshnessAnchor, FullRotationError<E>>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
{
    match protector
        .read_freshness_anchor(vault_id)
        .map_err(FullRotationError::Protector)?
    {
        ProtectedFreshnessState::Present(anchor) => Ok(anchor),
        ProtectedFreshnessState::Uninitialized => Err(FullRotationError::InvalidState),
    }
}

fn unlock_verified<P, E>(
    protector: &mut P,
    vault_id: VaultId,
    generation: KeyGeneration,
) -> Result<OwnedKeyMaterial, FullRotationError<E>>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
{
    protector
        .unlock_vrk(vault_id, generation)
        .map_err(FullRotationError::Protector)
}

/// Begins B503 at PREPARE after normal writes have been quiesced.
///
/// The target protector must be a distinct fresh opaque provider record. Its VRK
/// and optional recovery envelope are verified before the PREPARE checkpoint is
/// persisted. Its freshness slot is seeded with the exact source anchor so later
/// ANCHOR can perform ordinary compare-and-advance without mutating the source item.
#[allow(clippy::too_many_arguments)]
pub fn begin_full_rotation<Source, Target, B>(
    quiesced: RotationQuiesced,
    source_protector: &mut Source,
    target_protector: &mut Target,
    backend_impl: &mut B,
    nonce_ledger: &mut NonceReservationLedger,
    current_manifest_envelope: &[u8],
    protector_binding: FullRotationProtectorBinding,
    target_policy: ProtectorPolicy,
    recovery: Option<RecoveryRotationInput<'_>>,
) -> Result<FullRotationProgress, FullRotationError<B::Error>>
where
    Source: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
    Target: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
    B: FullRotationBackend,
{
    assert_backend_quiesced(backend_impl, RotationPhase::Prepare)?;
    if backend_impl
        .read_rotation_checkpoint()
        .map_err(|error| backend(RotationPhase::Prepare, error))?
        .is_some()
    {
        return Err(FullRotationError::InvalidState);
    }

    let source_identity = quiesced.identity();
    require_retained_manifest_reservation(nonce_ledger, current_manifest_envelope)?;
    let context =
        manifest_context(current_manifest_envelope).map_err(FullRotationError::Manifest)?;
    if context.vault_id() != source_identity.vault_id()
        || context.key_generation() != source_identity.key_generation()
        || nonce_ledger.vault_id() != source_identity.vault_id()
    {
        return Err(FullRotationError::InvalidState);
    }

    let source_vrk = unlock_verified(
        source_protector,
        source_identity.vault_id(),
        source_identity.key_generation(),
    )?;
    let source_manifest = decrypt_manifest(&source_vrk, context, current_manifest_envelope)
        .map_err(FullRotationError::Manifest)?;
    if source_manifest.rotation_phase() != RotationPhase::None
        || source_manifest.active_key_generation() != source_identity.key_generation()
        || source_manifest.freshness_epoch() != context.freshness_epoch()
    {
        return Err(FullRotationError::InvalidState);
    }

    let source_anchor = protector_anchor(source_protector, source_identity.vault_id())?;
    if source_anchor.vault_id() != source_identity.vault_id()
        || source_anchor.highest_epoch() != context.freshness_epoch()
        || source_anchor.manifest_hash() != manifest_hash(current_manifest_envelope)
    {
        return Err(FullRotationError::InvalidState);
    }

    let identity = FullRotationIdentity::for_next_generation(
        source_identity.vault_id(),
        source_identity.key_generation(),
    )
    .ok_or(FullRotationError::GenerationOverflow)?;
    let target_generation = identity.target_generation;
    let generations = target_generations_pre(&source_manifest, target_generation)?;

    match backend_impl
        .read_rotation_protector_binding()
        .map_err(|error| backend(RotationPhase::Prepare, error))?
    {
        Some(existing) if existing == protector_binding => {}
        Some(_) => return Err(FullRotationError::InvalidState),
        None => {
            backend_impl
                .persist_rotation_protector_binding(protector_binding)
                .map_err(|error| backend(RotationPhase::Prepare, error))?;
            if backend_impl
                .read_rotation_protector_binding()
                .map_err(|error| backend(RotationPhase::Prepare, error))?
                != Some(protector_binding)
            {
                return Err(FullRotationError::InvalidState);
            }
        }
    }

    target_protector
        .create_protector(
            target_policy.access_scope(),
            target_policy.user_presence_policy(),
        )
        .map_err(FullRotationError::Protector)?;
    let target_vrk = match target_protector.unlock_vrk(identity.vault_id, target_generation) {
        Ok(existing) => existing,
        Err(ProtectorError::ItemMissing) => {
            let generated = generate_target_vrk(&source_vrk)?;
            target_protector
                .protect_or_store_vrk(identity.vault_id, target_generation, &generated)
                .map_err(FullRotationError::Protector)?;
            let reread = unlock_verified(target_protector, identity.vault_id, target_generation)?;
            if !key_material_equal(&generated, &reread) {
                return Err(FullRotationError::InvalidState);
            }
            generated
        }
        Err(_) => return Err(FullRotationError::TargetProtectorNotEmpty),
    };
    if key_material_equal(&source_vrk, &target_vrk) {
        return Err(FullRotationError::KeyMaterialCollision);
    }

    match target_protector
        .read_freshness_anchor(identity.vault_id)
        .map_err(FullRotationError::Protector)?
    {
        ProtectedFreshnessState::Uninitialized => {
            target_protector
                .install_genesis_freshness_anchor(
                    identity.vault_id,
                    ProtectedFreshnessState::Uninitialized,
                    source_anchor,
                )
                .map_err(FullRotationError::Protector)?;
        }
        ProtectedFreshnessState::Present(anchor) if anchor == source_anchor => {}
        ProtectedFreshnessState::Present(_) => {
            return Err(FullRotationError::TargetProtectorNotEmpty);
        }
    }
    if protector_anchor(target_protector, identity.vault_id)? != source_anchor {
        return Err(FullRotationError::InvalidState);
    }

    if recovery.is_none()
        && backend_impl
            .read_target_recovery_wrap()
            .map_err(|error| backend(RotationPhase::Prepare, error))?
            .is_some()
    {
        return Err(FullRotationError::InvalidState);
    }

    if let Some(recovery) = recovery {
        let recovered_source = decrypt_recovery_envelope(
            RecoveryContext::new(identity.vault_id, identity.source_generation),
            recovery.passphrase,
            recovery.source_envelope,
        )
        .map_err(FullRotationError::Recovery)?;
        if !key_material_equal(&source_vrk, &recovered_source) {
            return Err(FullRotationError::InvalidState);
        }
        let target_envelope = match backend_impl
            .read_target_recovery_wrap()
            .map_err(|error| backend(RotationPhase::Prepare, error))?
        {
            Some(existing) => existing,
            None => {
                let envelope = encrypt_recovery_envelope(
                    &target_vrk,
                    RecoveryContext::new(identity.vault_id, identity.target_generation),
                    recovery.passphrase,
                )
                .map_err(FullRotationError::Recovery)?;
                backend_impl
                    .persist_target_recovery_wrap(&envelope)
                    .map_err(|error| backend(RotationPhase::Prepare, error))?;
                let reread = backend_impl
                    .read_target_recovery_wrap()
                    .map_err(|error| backend(RotationPhase::Prepare, error))?
                    .ok_or(FullRotationError::InvalidState)?;
                if reread != envelope {
                    return Err(FullRotationError::InvalidState);
                }
                envelope
            }
        };
        let recovered_target = decrypt_recovery_envelope(
            RecoveryContext::new(identity.vault_id, identity.target_generation),
            recovery.passphrase,
            &target_envelope,
        )
        .map_err(FullRotationError::Recovery)?;
        if !key_material_equal(&target_vrk, &recovered_target) {
            return Err(FullRotationError::InvalidState);
        }
    }

    let checkpoint = build_checkpoint(
        &source_manifest,
        identity,
        RotationPhase::Prepare,
        generations,
        source_manifest.objects().to_vec(),
    )?;
    persist_encrypted_checkpoint(
        backend_impl,
        nonce_ledger,
        &source_vrk,
        identity,
        &checkpoint,
    )?;

    Ok(FullRotationProgress::InProgress {
        identity,
        phase: RotationPhase::Prepare,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{
        AccessScope, FreshnessAnchor, FreshnessEpoch, HardwareBacking, KeyGeneration, ManifestHash,
        ProtectedFreshnessState, UserPresencePolicy, VAULT_ID_BYTES, VaultLeaseState,
    };
    use crate::vault_keys::{VaultKeyMaterial, VaultSessionLifetime};
    use crate::vault_lease::{KeyedHandleError, VaultLease};
    use std::convert::Infallible;

    const SOURCE_KEY: [u8; KEY_MATERIAL_BYTES] = [0x33; KEY_MATERIAL_BYTES];

    #[derive(Clone)]
    struct Record {
        vault_id: VaultId,
        generation: KeyGeneration,
        key: [u8; KEY_MATERIAL_BYTES],
        freshness: ProtectedFreshnessState,
    }

    struct MemoryProtector {
        configured: bool,
        record: Option<Record>,
    }

    impl MemoryProtector {
        fn empty() -> Self {
            Self {
                configured: false,
                record: None,
            }
        }

        fn source(vault_id: VaultId, generation: KeyGeneration, anchor: FreshnessAnchor) -> Self {
            Self {
                configured: true,
                record: Some(Record {
                    vault_id,
                    generation,
                    key: SOURCE_KEY,
                    freshness: ProtectedFreshnessState::Present(anchor),
                }),
            }
        }

        fn key_bytes(&self) -> [u8; KEY_MATERIAL_BYTES] {
            self.record.as_ref().expect("record exists").key
        }
    }

    impl SecretProtector for MemoryProtector {
        type VaultRootKey = OwnedKeyMaterial;

        fn create_protector(
            &mut self,
            _requested_scope: AccessScope,
            _user_presence_policy: UserPresencePolicy,
        ) -> Result<(), ProtectorError> {
            self.configured = true;
            Ok(())
        }

        fn protect_or_store_vrk(
            &mut self,
            vault_id: VaultId,
            key_generation: KeyGeneration,
            vrk: &OwnedKeyMaterial,
        ) -> Result<(), ProtectorError> {
            if !self.configured {
                return Err(ProtectorError::Unavailable);
            }
            let key = vrk.with_bytes(|bytes| *bytes);
            match &self.record {
                Some(record)
                    if record.vault_id == vault_id
                        && record.generation == key_generation
                        && record.key == key =>
                {
                    Ok(())
                }
                Some(_) => Err(ProtectorError::CorruptOrTampered),
                None => {
                    self.record = Some(Record {
                        vault_id,
                        generation: key_generation,
                        key,
                        freshness: ProtectedFreshnessState::Uninitialized,
                    });
                    Ok(())
                }
            }
        }

        fn unlock_vrk(
            &mut self,
            vault_id: VaultId,
            key_generation: KeyGeneration,
        ) -> Result<OwnedKeyMaterial, ProtectorError> {
            let record = self.record.as_ref().ok_or(ProtectorError::ItemMissing)?;
            if record.vault_id != vault_id || record.generation != key_generation {
                return Err(ProtectorError::OwnerMismatch);
            }
            Ok(OwnedKeyMaterial::from_bytes(record.key))
        }

        fn read_freshness_anchor(
            &self,
            vault_id: VaultId,
        ) -> Result<ProtectedFreshnessState, ProtectorError> {
            let record = self.record.as_ref().ok_or(ProtectorError::ItemMissing)?;
            if record.vault_id != vault_id {
                return Err(ProtectorError::OwnerMismatch);
            }
            Ok(record.freshness)
        }

        fn install_genesis_freshness_anchor(
            &mut self,
            vault_id: VaultId,
            expected_state: ProtectedFreshnessState,
            new_anchor: FreshnessAnchor,
        ) -> Result<(), ProtectorError> {
            let record = self.record.as_mut().ok_or(ProtectorError::ItemMissing)?;
            if record.vault_id != vault_id || record.freshness != expected_state {
                return Err(ProtectorError::AnchorConflict);
            }
            record.freshness = ProtectedFreshnessState::Present(new_anchor);
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

        fn remove_protector(&mut self, vault_id: VaultId) -> Result<(), ProtectorError> {
            match &self.record {
                Some(record) if record.vault_id == vault_id => {
                    self.record = None;
                    Ok(())
                }
                Some(_) => Err(ProtectorError::OwnerMismatch),
                None => Err(ProtectorError::ItemMissing),
            }
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

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct BackendError(&'static str);

    impl std::fmt::Display for BackendError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0)
        }
    }
    impl std::error::Error for BackendError {}

    #[derive(Default)]
    struct PrepareBackend {
        binding: Option<FullRotationProtectorBinding>,
        checkpoint: Option<Vec<u8>>,
        recovery: Option<Vec<u8>>,
        fail_checkpoint_once: bool,
    }

    impl FullRotationBackend for PrepareBackend {
        type Error = BackendError;
        fn assert_normal_writes_quiesced(&self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn read_rotation_protector_binding(
            &self,
        ) -> Result<Option<FullRotationProtectorBinding>, Self::Error> {
            Ok(self.binding)
        }
        fn persist_rotation_protector_binding(
            &mut self,
            binding: FullRotationProtectorBinding,
        ) -> Result<(), Self::Error> {
            self.binding = Some(binding);
            Ok(())
        }
        fn clear_rotation_protector_binding(&mut self) -> Result<(), Self::Error> {
            self.binding = None;
            Ok(())
        }
        fn read_rotation_checkpoint(&self) -> Result<Option<Vec<u8>>, Self::Error> {
            Ok(self.checkpoint.clone())
        }
        fn persist_rotation_checkpoint(&mut self, envelope: &[u8]) -> Result<(), Self::Error> {
            if self.fail_checkpoint_once {
                self.fail_checkpoint_once = false;
                return Err(BackendError("checkpoint failure"));
            }
            self.checkpoint = Some(envelope.to_vec());
            Ok(())
        }
        fn clear_rotation_checkpoint(&mut self) -> Result<(), Self::Error> {
            self.checkpoint = None;
            Ok(())
        }
        fn read_target_recovery_wrap(&self) -> Result<Option<Vec<u8>>, Self::Error> {
            Ok(self.recovery.clone())
        }
        fn persist_target_recovery_wrap(&mut self, envelope: &[u8]) -> Result<(), Self::Error> {
            self.recovery = Some(envelope.to_vec());
            Ok(())
        }
        fn remove_target_recovery_wrap(&mut self) -> Result<(), Self::Error> {
            self.recovery = None;
            Ok(())
        }
        fn stage_reencrypted_inventory(
            &mut self,
            _: &OwnedKeyMaterial,
            _: &OwnedKeyMaterial,
            _: &ManifestPlaintext,
            _: KeyGeneration,
            _: &mut NonceReservationLedger,
        ) -> Result<Vec<ManifestObject>, Self::Error> {
            Err(BackendError("STAGE outside B503B"))
        }
        fn verify_staged_inventory(
            &mut self,
            _: &OwnedKeyMaterial,
            _: &[ManifestObject],
        ) -> Result<(), Self::Error> {
            Err(BackendError("VERIFY outside B503B"))
        }
        fn quarantine_target_inventory(&mut self) -> Result<(), Self::Error> {
            Err(BackendError("abort outside B503B"))
        }
        fn publish_manifest(&mut self, _: &[u8]) -> Result<(), Self::Error> {
            Err(BackendError("PUBLISH outside B503B"))
        }
        fn read_published_manifest(&self) -> Result<Vec<u8>, Self::Error> {
            Err(BackendError("PUBLISH outside B503B"))
        }
        fn reopen_and_verify_published(
            &mut self,
            _: &OwnedKeyMaterial,
            _: &ManifestPlaintext,
        ) -> Result<(), Self::Error> {
            Err(BackendError("ACTIVATE outside B503B"))
        }
        fn activate_target_protector(&mut self) -> Result<(), Self::Error> {
            Err(BackendError("ACTIVATE outside B503B"))
        }
        fn retire_source_inventory(&mut self, _: KeyGeneration) -> Result<(), Self::Error> {
            Err(BackendError("RETIRE outside B503B"))
        }
        fn retire_source_recovery_wrap(&mut self) -> Result<(), Self::Error> {
            Err(BackendError("RETIRE outside B503B"))
        }
    }

    struct NoopCloser;

    impl KeyedHandleCloser for NoopCloser {
        type Error = Infallible;

        fn close_keyed_handles(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct NoopCache;

    impl PlaintextCache for NoopCache {
        fn discard_plaintext(&mut self) {}
    }

    fn identity() -> VaultLeaseIdentity {
        VaultLeaseIdentity::new(
            VaultId::from_bytes([0x51; VAULT_ID_BYTES]),
            KeyGeneration::new(9).expect("non-zero generation"),
        )
    }

    #[test]
    fn next_generation_identity_is_exact_and_overflow_fails_closed() {
        let current = identity();
        let rotation =
            FullRotationIdentity::for_next_generation(current.vault_id(), current.key_generation())
                .expect("next generation exists");
        assert_eq!(rotation.vault_id(), current.vault_id());
        assert_eq!(rotation.source_generation(), current.key_generation());
        assert_eq!(
            rotation.target_generation().get(),
            current.key_generation().get() + 1
        );

        let max = KeyGeneration::new(u64::MAX).expect("maximum is non-zero");
        assert_eq!(
            FullRotationIdentity::for_next_generation(current.vault_id(), max),
            None
        );
    }

    #[test]
    fn protector_binding_requires_distinct_opaque_records() {
        assert!(FullRotationProtectorBinding::new([0x11; 16], [0x11; 16]).is_none());
        let binding = FullRotationProtectorBinding::new([0x11; 16], [0x12; 16])
            .expect("distinct protector records");
        assert_eq!(binding.source_protector_id(), [0x11; 16]);
        assert_eq!(binding.target_protector_id(), [0x12; 16]);
    }

    #[test]
    fn quiesce_revokes_existing_handles_and_releases_session_keys() {
        let current = identity();
        let lease = VaultLease::new(current);
        let stale = lease.keyed_handle_lease();
        let mut session = VaultSessionLifetime::new(
            lease,
            NoopCloser,
            VaultKeyMaterial::new(OwnedKeyMaterial::from_bytes([0x44; 32])),
            NoopCache,
        );

        let proof = quiesce_for_full_rotation(&mut session).expect("quiescence succeeds");
        assert_eq!(proof.identity(), current);
        assert_eq!(session.lease_state(), VaultLeaseState::Revoked);
        assert!(session.keys_released());
        assert!(matches!(stale.authorize(), Err(KeyedHandleError::Revoked)));
    }

    struct PrepareFixture {
        vault_id: VaultId,
        generation: KeyGeneration,
        envelope: Vec<u8>,
        source: MemoryProtector,
        target: MemoryProtector,
        backend: PrepareBackend,
        ledger: NonceReservationLedger,
        session: VaultSessionLifetime<NoopCloser, NoopCache>,
        source_anchor: FreshnessAnchor,
    }

    fn prepare_fixture() -> PrepareFixture {
        let vault_id = VaultId::from_bytes([0x62; VAULT_ID_BYTES]);
        let generation = KeyGeneration::new(11).expect("non-zero generation");
        let epoch = FreshnessEpoch::new(13).expect("non-zero epoch");
        let source_vrk = OwnedKeyMaterial::from_bytes(SOURCE_KEY);
        let manifest = ManifestPlaintext::new(
            vault_id,
            epoch,
            ManifestHash::from_bytes([0x35; 32]),
            generation,
            (RotationPhase::None, None),
            vec![ManifestGeneration::new(generation, GenerationState::Active)],
            vec![ManifestObject::new(
                [0; 16],
                [0x41; 16],
                generation,
                512,
                [0x52; 32],
                crate::vault_manifest::ManifestAuthMetadata::StructuredStore,
            )],
        )
        .expect("source manifest is canonical");
        let mut ledger = NonceReservationLedger::new(vault_id);
        let context = ManifestContext::new(vault_id, generation, epoch);
        let (_, envelope) = encrypt_fresh_manifest(&mut ledger, &source_vrk, context, &manifest)
            .expect("source manifest encryption succeeds");
        let anchor = FreshnessAnchor::new(vault_id, epoch, manifest_hash(&envelope));
        let source = MemoryProtector::source(vault_id, generation, anchor);
        let target = MemoryProtector::empty();
        let lease = VaultLease::new(VaultLeaseIdentity::new(vault_id, generation));
        let session = VaultSessionLifetime::new(
            lease,
            NoopCloser,
            VaultKeyMaterial::new(OwnedKeyMaterial::from_bytes(SOURCE_KEY)),
            NoopCache,
        );
        PrepareFixture {
            vault_id,
            generation,
            envelope,
            source,
            target,
            backend: PrepareBackend::default(),
            ledger,
            session,
            source_anchor: anchor,
        }
    }

    fn prepare_binding() -> FullRotationProtectorBinding {
        FullRotationProtectorBinding::new([0x21; 16], [0x22; 16])
            .expect("distinct protector identifiers")
    }

    fn prepare_policy() -> ProtectorPolicy {
        ProtectorPolicy::new(
            AccessScope::SameUserAccount,
            UserPresencePolicy::NotRequired,
        )
    }

    #[test]
    fn prepare_creates_distinct_target_and_authenticated_checkpoint() {
        let PrepareFixture {
            vault_id,
            generation,
            envelope,
            mut source,
            mut target,
            mut backend,
            mut ledger,
            mut session,
            source_anchor,
        } = prepare_fixture();
        let quiesced = quiesce_for_full_rotation(&mut session).expect("quiescence succeeds");
        let progress = begin_full_rotation(
            quiesced,
            &mut source,
            &mut target,
            &mut backend,
            &mut ledger,
            &envelope,
            prepare_binding(),
            prepare_policy(),
            None,
        )
        .expect("PREPARE succeeds");
        let identity = match progress {
            FullRotationProgress::InProgress { identity, phase } => {
                assert_eq!(phase, RotationPhase::Prepare);
                identity
            }
            FullRotationProgress::Complete { .. } => panic!("PREPARE cannot complete rotation"),
        };
        assert_eq!(identity.source_generation(), generation);
        assert_eq!(identity.target_generation().get(), generation.get() + 1);
        assert_ne!(target.key_bytes(), SOURCE_KEY);
        assert_eq!(
            target
                .read_freshness_anchor(vault_id)
                .expect("target anchor"),
            ProtectedFreshnessState::Present(source_anchor)
        );
        let checkpoint = backend
            .checkpoint
            .as_ref()
            .expect("PREPARE checkpoint persists");
        let context = manifest_context(checkpoint).expect("checkpoint context parses");
        let decoded = decrypt_manifest(
            &OwnedKeyMaterial::from_bytes(SOURCE_KEY),
            context,
            checkpoint,
        )
        .expect("PREPARE checkpoint authenticates under source generation");
        assert_eq!(decoded.rotation_phase(), RotationPhase::Prepare);
        assert_eq!(
            decoded.rotation_target_generation(),
            Some(identity.target_generation())
        );
        assert!(decoded.generations().iter().any(|entry| {
            entry.generation() == identity.target_generation()
                && entry.state() == GenerationState::Staged
        }));
    }

    #[test]
    fn prepare_retry_reuses_target_key_after_checkpoint_failure() {
        let PrepareFixture {
            envelope,
            mut source,
            mut target,
            mut backend,
            mut ledger,
            mut session,
            ..
        } = prepare_fixture();
        backend.fail_checkpoint_once = true;
        let quiesced = quiesce_for_full_rotation(&mut session).expect("quiescence succeeds");
        let first = begin_full_rotation(
            quiesced,
            &mut source,
            &mut target,
            &mut backend,
            &mut ledger,
            &envelope,
            prepare_binding(),
            prepare_policy(),
            None,
        );
        assert!(matches!(
            first,
            Err(FullRotationError::Backend {
                phase: RotationPhase::Prepare,
                ..
            })
        ));
        let retained_target_key = target.key_bytes();
        assert!(backend.checkpoint.is_none());

        let second = begin_full_rotation(
            quiesced,
            &mut source,
            &mut target,
            &mut backend,
            &mut ledger,
            &envelope,
            prepare_binding(),
            prepare_policy(),
            None,
        )
        .expect("PREPARE retry succeeds");
        assert!(matches!(
            second,
            FullRotationProgress::InProgress {
                phase: RotationPhase::Prepare,
                ..
            }
        ));
        assert_eq!(target.key_bytes(), retained_target_key);
        assert!(backend.checkpoint.is_some());
    }
}
