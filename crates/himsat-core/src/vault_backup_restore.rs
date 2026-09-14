//! B505 portable-backup fresh-device restore material verification.
//!
//! Provider authentication, payload reconstruction, inner-format verification,
//! and SQLCipher integrity complete before any protector creation, VRK storage,
//! protected genesis decision, local canonical publication, or anchor mutation.

use crate::vault::VaultId;
use crate::vault_backup_provider::PortableBackupProvider;
use crate::vault_backup_sqlcipher::{
    BackupSqlCipherVerificationError, SqlCipherVerifiedBackupSet, verify_staged_sqlcipher_backup,
};
use crate::vault_backup_verify::{BackupSemanticError, verify_backup_semantics_before_sqlcipher};
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

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

    struct Fixture {
        provider: MemoryProvider,
        descriptor: Vec<u8>,
        source_db: PathBuf,
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
        }
    }

    fn cleanup(fixture: Fixture) {
        drop(fixture.open_source);
        remove_database_candidate(&fixture.source_db);
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
}
