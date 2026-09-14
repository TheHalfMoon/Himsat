//! B505 portable-backup fresh-device restore preparation.
//!
//! Provider authentication, payload reconstruction, inner-format verification,
//! and SQLCipher integrity all complete before B502 inspects protected genesis
//! state. This leaf does not publish local canonical objects or mutate a
//! protected freshness anchor.

use crate::vault::{ProtectedFreshnessState, VaultId};
use crate::vault_backup_provider::PortableBackupProvider;
use crate::vault_backup_sqlcipher::{
    BackupSqlCipherVerificationError, SqlCipherVerifiedBackupSet, verify_staged_sqlcipher_backup,
};
use crate::vault_backup_verify::{BackupSemanticError, verify_backup_semantics_before_sqlcipher};
use crate::vault_restore::{
    FreshDeviceNewestnessRiskAccepted, FreshDeviceRestoreGenesis, RestoreError,
    prepare_fresh_device_restore_genesis,
};
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum FreshDeviceBackupRestoreError<E> {
    StagingCreateFailed,
    StagingSyncFailed,
    Semantic(BackupSemanticError<E>),
    SqlCipher(BackupSqlCipherVerificationError),
    Restore(RestoreError),
    CorruptOrTampered,
}

impl<E: fmt::Display> fmt::Display for FreshDeviceBackupRestoreError<E> {
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
            Self::Restore(error) => write!(f, "fresh-device restore genesis failed: {error}"),
            Self::CorruptOrTampered => {
                f.write_str("fresh-device restore candidate is corrupt or tampered")
            }
        }
    }
}

impl<E: Error + 'static> Error for FreshDeviceBackupRestoreError<E> {}

pub struct FreshDeviceRestoreCandidate {
    verified_backup: SqlCipherVerifiedBackupSet,
    genesis: FreshDeviceRestoreGenesis,
}

impl FreshDeviceRestoreCandidate {
    #[must_use]
    pub fn verified_backup(&self) -> &SqlCipherVerifiedBackupSet {
        &self.verified_backup
    }

    #[must_use]
    pub const fn genesis(&self) -> &FreshDeviceRestoreGenesis {
        &self.genesis
    }

    #[must_use]
    pub fn into_verified_backup(self) -> SqlCipherVerifiedBackupSet {
        self.verified_backup
    }
}

fn remove_database_candidate(path: &Path) {
    let _ = std::fs::remove_file(path);
    for suffix in ["-wal", "-shm"] {
        let _ = std::fs::remove_file(PathBuf::from(format!("{}{}", path.display(), suffix)));
    }
}

#[allow(clippy::too_many_arguments)]
pub fn prepare_fresh_device_backup_restore<P: PortableBackupProvider>(
    provider: &mut P,
    descriptor_bytes: &[u8],
    passphrase: &str,
    expected_vault_id: VaultId,
    protected_state: ProtectedFreshnessState,
    structured_store_staging_path: &Path,
    risk_acceptance: FreshDeviceNewestnessRiskAccepted,
) -> Result<FreshDeviceRestoreCandidate, FreshDeviceBackupRestoreError<P::Error>> {
    let mut staging_owned = false;
    let result = (|| {
        let mut staging = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(structured_store_staging_path)
            .map_err(|_| FreshDeviceBackupRestoreError::StagingCreateFailed)?;
        staging_owned = true;

        let pre_sqlcipher = verify_backup_semantics_before_sqlcipher(
            provider,
            descriptor_bytes,
            passphrase,
            &mut staging,
        )
        .map_err(FreshDeviceBackupRestoreError::Semantic)?;
        staging
            .sync_all()
            .map_err(|_| FreshDeviceBackupRestoreError::StagingSyncFailed)?;
        drop(staging);

        let verified = verify_staged_sqlcipher_backup(structured_store_staging_path, pre_sqlcipher)
            .map_err(FreshDeviceBackupRestoreError::SqlCipher)?;
        let pre = verified.pre_sqlcipher();
        let genesis = pre
            .with_structured_store_verification_key(|vrk, _, _| {
                prepare_fresh_device_restore_genesis(
                    vrk,
                    expected_vault_id,
                    protected_state,
                    pre.source_manifest_envelope(),
                    risk_acceptance,
                )
            })
            .map_err(FreshDeviceBackupRestoreError::Restore)?;

        if genesis.manifest() != pre.manifest()
            || genesis.accepted_anchor().vault_id() != pre.vault_id()
            || genesis.accepted_anchor().highest_epoch() != pre.source_freshness_epoch()
            || genesis.accepted_anchor().manifest_hash() != pre.source_manifest_hash()
            || genesis.global_newestness_proven()
        {
            return Err(FreshDeviceBackupRestoreError::CorruptOrTampered);
        }

        Ok(FreshDeviceRestoreCandidate {
            verified_backup: verified,
            genesis,
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
        FreshnessAnchor, FreshnessEpoch, KeyGeneration, ManifestHash, VaultLeaseIdentity,
    };
    use crate::vault_backup::descriptor_provider_key;
    use crate::vault_backup_creation::create_publish_and_verify_portable_backup;
    use crate::vault_backup_packaging::BackupSourceFile;
    use crate::vault_keys::{KeyDerivationContext, KeyPurpose, OwnedKeyMaterial};
    use crate::vault_lease::VaultLease;
    use crate::vault_manifest::{
        GenerationState, ManifestAuthMetadata, ManifestContext, ManifestGeneration, ManifestObject,
        ManifestPlaintext, RotationPhase, encrypt_fresh_manifest, manifest_hash,
    };
    use crate::vault_nonce::NonceReservationLedger;
    use crate::vault_recovery::{RecoveryContext, encrypt_recovery_envelope};
    use crate::vault_sqlcipher::BackupSnapshotQuiescenceGuard;
    use rusqlite::Connection;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::error::Error as StdError;
    use std::fmt::Write as _;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    const PASSPHRASE: &str = "correct horse battery staple";
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
    fn remove_database(path: &Path) {
        remove_database_candidate(path);
    }

    struct Fixture {
        provider: MemoryProvider,
        descriptor: Vec<u8>,
        source_db: PathBuf,
        open_source: Connection,
        anchor: FreshnessAnchor,
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
            vec![ManifestObject::new(
                [0; 16],
                [0x54; 16],
                generation(),
                db_len,
                db_sha,
                ManifestAuthMetadata::StructuredStore,
            )],
        )
        .unwrap();
        let mut ledger = NonceReservationLedger::new(vault_id());
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
            &[] as &[BackupSourceFile],
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
            open_source,
            anchor,
        }
    }

    fn cleanup(fixture: Fixture) {
        drop(fixture.open_source);
        remove_database(&fixture.source_db);
    }

    #[test]
    fn authenticated_backup_prepares_fresh_device_genesis_without_claiming_newestness() {
        let mut fixture = fixture();
        let staging = unused_path("restore.sqlite3");
        let candidate = prepare_fresh_device_backup_restore(
            &mut fixture.provider,
            &fixture.descriptor,
            PASSPHRASE,
            vault_id(),
            ProtectedFreshnessState::Uninitialized,
            &staging,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        )
        .unwrap();
        assert_eq!(candidate.genesis().accepted_anchor(), fixture.anchor);
        assert!(!candidate.genesis().global_newestness_proven());
        assert_eq!(
            candidate.verified_backup().pre_sqlcipher().vault_id(),
            vault_id()
        );
        assert!(staging.is_file());
        remove_database(&staging);
        cleanup(fixture);
    }

    #[test]
    fn present_genesis_state_is_rejected_after_full_backup_verification() {
        let mut fixture = fixture();
        let staging = unused_path("present.sqlite3");
        let result = prepare_fresh_device_backup_restore(
            &mut fixture.provider,
            &fixture.descriptor,
            PASSPHRASE,
            vault_id(),
            ProtectedFreshnessState::Present(fixture.anchor),
            &staging,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        );
        assert!(matches!(
            result,
            Err(FreshDeviceBackupRestoreError::Restore(
                RestoreError::AnchorAlreadyInitialized
            ))
        ));
        assert!(!staging.exists());
        cleanup(fixture);
    }

    #[test]
    fn provider_authentication_precedes_present_state_decision() {
        let mut fixture = fixture();
        let mut descriptor = fixture.descriptor.clone();
        let last = descriptor.len() - 1;
        descriptor[last] ^= 1;
        let staging = unused_path("tampered.sqlite3");
        let result = prepare_fresh_device_backup_restore(
            &mut fixture.provider,
            &descriptor,
            PASSPHRASE,
            vault_id(),
            ProtectedFreshnessState::Present(fixture.anchor),
            &staging,
            FreshDeviceNewestnessRiskAccepted::ACCEPTED,
        );
        assert!(matches!(
            result,
            Err(FreshDeviceBackupRestoreError::Semantic(_))
        ));
        assert!(!staging.exists());
        cleanup(fixture);
    }
}
