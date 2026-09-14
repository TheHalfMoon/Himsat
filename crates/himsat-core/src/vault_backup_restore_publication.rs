//! B505 fresh-device durable local publication and protected-genesis commit.
//!
//! This layer consumes only B505R-prepared material and reuses the already-qualified
//! B307 copy-verify-publish ordering. Durable object publication and reopen behavior
//! remain behind an explicit backend contract; protected freshness mutation stays in
//! the coordinator so the accepted manifest and installed anchor cannot diverge.

use crate::vault::{FreshnessAnchor, ProtectedFreshnessState, ProtectorError, SecretProtector};
use crate::vault_backup_restore::{FreshDevicePreparedGenesis, key_material_equal};
use crate::vault_backup_sqlcipher::{
    BackupSqlCipherVerificationError, SqlCipherVerifiedBackupSet, reverify_staged_sqlcipher_backup,
};
use crate::vault_backup_verify::PreSqlCipherVerifiedBackupSet;
use crate::vault_keys::OwnedKeyMaterial;
use crate::vault_manifest::{
    ManifestError, ManifestPlaintext, decrypt_manifest, manifest_context, manifest_hash,
};
use crate::vault_migration::{
    CopyVerifyPublishError, EncryptedMigrationSource, EncryptedMigrationTarget,
    run_copy_verify_publish,
};
use crate::vault_restore::FreshDeviceRestoreGenesis;
use std::error::Error;
use std::fmt;
use std::path::Path;

/// Storage boundary for fresh-device restore publication.
///
/// Implementations must keep every operation idempotent for the exact same input.
/// `stage_verified_backup` must durably copy/fsync the structured store and every
/// authenticated B202 artifact into non-canonical local candidate storage.
/// `verify_staged_backup` must reread the complete candidate set, require exact
/// authenticated identities, and run the applicable SQLCipher/B202 integrity checks.
/// `publish_manifest` must durably publish the exact supplied B501 envelope and make
/// the same bytes available through `read_published_manifest` before it returns.
/// No method may mutate the protected freshness anchor or retire recovery material.
pub trait FreshDeviceRestoreBackend {
    type Error;

    fn stage_verified_backup(
        &mut self,
        structured_store_source: &Path,
        verified: &PreSqlCipherVerifiedBackupSet,
    ) -> Result<(), Self::Error>;

    fn verify_staged_backup(
        &mut self,
        vrk: &OwnedKeyMaterial,
        verified: &PreSqlCipherVerifiedBackupSet,
    ) -> Result<(), Self::Error>;

    fn publish_manifest(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;

    fn read_published_manifest(&self) -> Result<Vec<u8>, Self::Error>;

    fn reopen_published(
        &mut self,
        vrk: &OwnedKeyMaterial,
        manifest: &ManifestPlaintext,
    ) -> Result<(), Self::Error>;

    fn verify_reopened(
        &mut self,
        vrk: &OwnedKeyMaterial,
        manifest: &ManifestPlaintext,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum FreshDevicePublicationOperationError<E> {
    SourceSqlCipher(BackupSqlCipherVerificationError),
    Backend(E),
    Protector(ProtectorError),
    Manifest(ManifestError),
    PublishedManifestMismatch,
    ProtectedStateChanged,
}

impl<E: fmt::Display> fmt::Display for FreshDevicePublicationOperationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceSqlCipher(error) => {
                write!(
                    f,
                    "fresh-device restore source re-verification failed: {error}"
                )
            }
            Self::Backend(error) => write!(f, "fresh-device restore backend failed: {error}"),
            Self::Protector(error) => write!(f, "fresh-device restore protector failed: {error}"),
            Self::Manifest(error) => write!(f, "fresh-device restore manifest failed: {error}"),
            Self::PublishedManifestMismatch => {
                f.write_str("published fresh-device manifest does not match accepted backup")
            }
            Self::ProtectedStateChanged => {
                f.write_str("protected fresh-device state changed during publication")
            }
        }
    }
}

impl<E: Error + 'static> Error for FreshDevicePublicationOperationError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SourceSqlCipher(error) => Some(error),
            Self::Backend(error) => Some(error),
            Self::Protector(error) => Some(error),
            Self::Manifest(error) => Some(error),
            Self::PublishedManifestMismatch | Self::ProtectedStateChanged => None,
        }
    }
}

pub type FreshDeviceRestorePublicationError<E> =
    CopyVerifyPublishError<FreshDevicePublicationOperationError<E>>;

/// Proof returned only after the accepted backup is locally published, its exact
/// protected genesis anchor is installed and reread, and the anchored state is
/// reopened and integrity-verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FreshDeviceRestoreComplete {
    accepted_anchor: FreshnessAnchor,
    global_newestness_proven: bool,
}

impl FreshDeviceRestoreComplete {
    #[must_use]
    pub const fn accepted_anchor(self) -> FreshnessAnchor {
        self.accepted_anchor
    }

    #[must_use]
    pub const fn global_newestness_proven(self) -> bool {
        self.global_newestness_proven
    }
}

struct FreshDeviceRestoreSource<'a> {
    structured_store_source: &'a Path,
    verified: &'a SqlCipherVerifiedBackupSet,
}

struct FreshDeviceRestoreTarget<'a, P, B> {
    protector: &'a mut P,
    backend: &'a mut B,
    verified: &'a PreSqlCipherVerifiedBackupSet,
    genesis: &'a FreshDeviceRestoreGenesis,
}

impl<'source, 'target, P, B> EncryptedMigrationSource<FreshDeviceRestoreTarget<'target, P, B>>
    for FreshDeviceRestoreSource<'source>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
    B: FreshDeviceRestoreBackend,
{
    type Error = FreshDevicePublicationOperationError<B::Error>;

    fn verify_source(&self) -> Result<(), Self::Error> {
        reverify_staged_sqlcipher_backup(self.structured_store_source, self.verified)
            .map_err(FreshDevicePublicationOperationError::SourceSqlCipher)
    }

    fn copy_into(
        &self,
        target: &mut FreshDeviceRestoreTarget<'target, P, B>,
    ) -> Result<(), Self::Error> {
        target
            .backend
            .stage_verified_backup(self.structured_store_source, target.verified)
            .map_err(FreshDevicePublicationOperationError::Backend)
    }
}

impl<P, B> FreshDeviceRestoreTarget<'_, P, B>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
    B: FreshDeviceRestoreBackend,
{
    fn verify_published_manifest(
        &mut self,
    ) -> Result<(), FreshDevicePublicationOperationError<B::Error>> {
        let expected = self.verified.source_manifest_envelope();
        let published = self
            .backend
            .read_published_manifest()
            .map_err(FreshDevicePublicationOperationError::Backend)?;
        if published != expected
            || manifest_hash(&published) != self.genesis.accepted_anchor().manifest_hash()
        {
            return Err(FreshDevicePublicationOperationError::PublishedManifestMismatch);
        }

        self.verified.with_structured_store_verification_key(
            |vrk, expected_vault_id, expected_generation| {
                let context = manifest_context(&published)
                    .map_err(FreshDevicePublicationOperationError::Manifest)?;
                if context.vault_id() != expected_vault_id
                    || context.key_generation() != expected_generation
                {
                    return Err(FreshDevicePublicationOperationError::PublishedManifestMismatch);
                }
                let manifest = decrypt_manifest(vrk, context, &published)
                    .map_err(FreshDevicePublicationOperationError::Manifest)?;
                if &manifest != self.genesis.manifest() {
                    return Err(FreshDevicePublicationOperationError::PublishedManifestMismatch);
                }
                Ok(())
            },
        )
    }

    fn unlock_matching_protected_vrk(
        &mut self,
    ) -> Result<OwnedKeyMaterial, FreshDevicePublicationOperationError<B::Error>> {
        self.verified.with_structured_store_verification_key(
            |recovered_vrk, vault_id, key_generation| {
                let protected_vrk = self
                    .protector
                    .unlock_vrk(vault_id, key_generation)
                    .map_err(FreshDevicePublicationOperationError::Protector)?;
                if !key_material_equal(recovered_vrk, &protected_vrk) {
                    return Err(FreshDevicePublicationOperationError::ProtectedStateChanged);
                }
                Ok(protected_vrk)
            },
        )
    }

    fn require_installed_anchor(
        &self,
    ) -> Result<(), FreshDevicePublicationOperationError<B::Error>> {
        let expected = self.genesis.accepted_anchor();
        match self
            .protector
            .read_freshness_anchor(expected.vault_id())
            .map_err(FreshDevicePublicationOperationError::Protector)?
        {
            ProtectedFreshnessState::Present(actual) if actual == expected => Ok(()),
            ProtectedFreshnessState::Uninitialized | ProtectedFreshnessState::Present(_) => {
                Err(FreshDevicePublicationOperationError::ProtectedStateChanged)
            }
        }
    }
}

impl<P, B> EncryptedMigrationTarget for FreshDeviceRestoreTarget<'_, P, B>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
    B: FreshDeviceRestoreBackend,
{
    type Error = FreshDevicePublicationOperationError<B::Error>;

    fn verify_staged_copy(&mut self) -> Result<(), Self::Error> {
        self.verified
            .with_structured_store_verification_key(|vrk, _, _| {
                self.backend
                    .verify_staged_backup(vrk, self.verified)
                    .map_err(FreshDevicePublicationOperationError::Backend)
            })
    }

    fn publish_verified_copy(&mut self) -> Result<(), Self::Error> {
        self.backend
            .publish_manifest(self.verified.source_manifest_envelope())
            .map_err(FreshDevicePublicationOperationError::Backend)?;
        self.verify_published_manifest()
    }

    fn anchor_published_copy(&mut self) -> Result<(), Self::Error> {
        let accepted_anchor = self.genesis.accepted_anchor();
        match self
            .protector
            .read_freshness_anchor(accepted_anchor.vault_id())
            .map_err(FreshDevicePublicationOperationError::Protector)?
        {
            ProtectedFreshnessState::Uninitialized => {}
            ProtectedFreshnessState::Present(_) => {
                return Err(FreshDevicePublicationOperationError::ProtectedStateChanged);
            }
        }
        let _protected_vrk = self.unlock_matching_protected_vrk()?;
        self.protector
            .install_genesis_freshness_anchor(
                accepted_anchor.vault_id(),
                ProtectedFreshnessState::Uninitialized,
                accepted_anchor,
            )
            .map_err(FreshDevicePublicationOperationError::Protector)?;
        self.require_installed_anchor()
    }

    fn reopen_published_copy(&mut self) -> Result<(), Self::Error> {
        let protected_vrk = self.unlock_matching_protected_vrk()?;
        self.backend
            .reopen_published(&protected_vrk, self.genesis.manifest())
            .map_err(FreshDevicePublicationOperationError::Backend)
    }

    fn verify_reopened_copy(&mut self) -> Result<(), Self::Error> {
        let protected_vrk = self.unlock_matching_protected_vrk()?;
        self.backend
            .verify_reopened(&protected_vrk, self.genesis.manifest())
            .map_err(FreshDevicePublicationOperationError::Backend)?;
        self.verify_published_manifest()?;
        self.require_installed_anchor()
    }
}

/// Completes fresh-device local publication from B505R-prepared material.
///
/// The external SQLCipher staging source is re-proved immediately before copy.
/// B307 sequencing then requires complete durable staging verification and exact
/// manifest reread/authentication before the first protected anchor is installed.
/// Any failure before anchoring leaves `UNINITIALIZED` authoritative. A failure
/// after a successful anchor installation never rolls the anchor back; restart
/// must recover by opening and verifying the exact anchored manifest.
pub fn publish_prepared_fresh_device_restore<P, B>(
    protector: &mut P,
    backend: &mut B,
    structured_store_source: &Path,
    prepared: FreshDevicePreparedGenesis,
) -> Result<FreshDeviceRestoreComplete, FreshDeviceRestorePublicationError<B::Error>>
where
    P: SecretProtector<VaultRootKey = OwnedKeyMaterial>,
    B: FreshDeviceRestoreBackend,
{
    let accepted_anchor = prepared.genesis().accepted_anchor();
    let global_newestness_proven = prepared.genesis().global_newestness_proven();
    let verified_sqlcipher = prepared.verified_backup().verified_backup();
    let source = FreshDeviceRestoreSource {
        structured_store_source,
        verified: verified_sqlcipher,
    };
    let mut target = FreshDeviceRestoreTarget {
        protector,
        backend,
        verified: verified_sqlcipher.pre_sqlcipher(),
        genesis: prepared.genesis(),
    };
    run_copy_verify_publish(&source, &mut target)?;

    Ok(FreshDeviceRestoreComplete {
        accepted_anchor,
        global_newestness_proven,
    })
}
