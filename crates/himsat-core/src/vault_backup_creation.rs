use crate::vault::{
    FreshnessAnchor, FreshnessEpoch, KeyGeneration, ManifestHash, VaultLeaseIdentity,
};
use crate::vault_backup::BackupSetId;
use crate::vault_backup_packaging::{
    BackupPackagingError, BackupSourceFile, DiskBackupSourceError, prepare_disk_backed_backup_set,
};
use crate::vault_backup_provider::{
    PortableBackupProvider, StreamingBackupPublicationError, publish_and_reread_backup_source,
};
use crate::vault_backup_sqlcipher::{
    BackupSqlCipherVerificationError, verify_staged_sqlcipher_backup,
};
use crate::vault_backup_verify::{BackupSemanticError, verify_backup_semantics_before_sqlcipher};
use crate::vault_freshness::{FreshnessDecision, FreshnessError, authenticate_for_open};
use crate::vault_keys::{KeyDerivationContext, KeyPurpose, OwnedKeyMaterial};
use crate::vault_lease::KeyedHandleLease;
use crate::vault_manifest::{ManifestAuthMetadata, ManifestObjectKind, RotationPhase};
use crate::vault_sqlcipher::{
    BackupSnapshotQuiescenceGuard, SqlCipherBackupSnapshotError,
    snapshot_quiesced_sqlcipher_database,
};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

const FILE_HASH_BUFFER_BYTES: usize = 1024 * 1024;

#[derive(Debug)]
pub enum BackupCreationError<ProviderError, GuardError> {
    Freshness(FreshnessError),
    FreshnessNotCurrent,
    BackupStateNotStable,
    LeaseIdentityMismatch,
    InventoryMismatch,
    SourceReadFailed,
    SnapshotQuiescence(GuardError),
    Snapshot(SqlCipherBackupSnapshotError<GuardError>),
    Packaging(BackupPackagingError),
    Publication(StreamingBackupPublicationError<ProviderError, DiskBackupSourceError>),
    StagingPath,
    VerificationStaging,
    Semantic(BackupSemanticError<ProviderError>),
    SqlCipher(BackupSqlCipherVerificationError),
    CorruptOrTampered,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcceptedPortableBackup {
    set_id: BackupSetId,
    vault_id: crate::vault::VaultId,
    key_generation: KeyGeneration,
    source_freshness_epoch: FreshnessEpoch,
    source_manifest_hash: ManifestHash,
    data_object_count: u32,
}

impl AcceptedPortableBackup {
    #[must_use]
    pub const fn set_id(self) -> BackupSetId {
        self.set_id
    }
    #[must_use]
    pub const fn vault_id(self) -> crate::vault::VaultId {
        self.vault_id
    }
    #[must_use]
    pub const fn key_generation(self) -> KeyGeneration {
        self.key_generation
    }
    #[must_use]
    pub const fn source_freshness_epoch(self) -> FreshnessEpoch {
        self.source_freshness_epoch
    }
    #[must_use]
    pub const fn source_manifest_hash(self) -> ManifestHash {
        self.source_manifest_hash
    }
    #[must_use]
    pub const fn data_object_count(self) -> u32 {
        self.data_object_count
    }
}

fn regular_file_identity(path: &Path) -> Result<(u64, [u8; 32]), ()> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| ())?;
    if !metadata.file_type().is_file() {
        return Err(());
    }
    let mut file = File::open(path).map_err(|_| ())?;
    if !file.metadata().map_err(|_| ())?.is_file() {
        return Err(());
    }
    let mut buffer = vec![0_u8; FILE_HASH_BUFFER_BYTES];
    let mut hasher = Sha256::new();
    let mut total = 0_u64;
    loop {
        let read = file.read(&mut buffer).map_err(|_| ())?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(u64::try_from(read).map_err(|_| ())?)
            .ok_or(())?;
        hasher.update(&buffer[..read]);
    }
    Ok((total, hasher.finalize().into()))
}

fn remove_database_candidate(path: &Path) {
    let _ = std::fs::remove_file(path);
    for suffix in ["-wal", "-shm"] {
        let _ = std::fs::remove_file(PathBuf::from(format!("{}{}", path.display(), suffix)));
    }
}

fn normalize_absent_path(path: &Path) -> Result<PathBuf, ()> {
    if path.exists() || path.file_name().is_none() {
        return Err(());
    }
    let parent =
        std::fs::canonicalize(path.parent().unwrap_or_else(|| Path::new("."))).map_err(|_| ())?;
    Ok(parent.join(path.file_name().ok_or(())?))
}

fn validate_staging_paths(snapshot: &Path, package: &Path, verify: &Path) -> Result<(), ()> {
    let snapshot = normalize_absent_path(snapshot)?;
    let package = normalize_absent_path(package)?;
    let verify = normalize_absent_path(verify)?;
    if snapshot == package || snapshot == verify || package == verify {
        return Err(());
    }
    Ok(())
}

fn preflight_inventory<PE, GE>(
    active_vrk: &OwnedKeyMaterial,
    trusted_anchor: FreshnessAnchor,
    source_manifest_envelope: &[u8],
    source_sqlcipher_path: &Path,
    generic_sources: &[BackupSourceFile],
    lease: &KeyedHandleLease,
) -> Result<([u8; 16], crate::vault::VaultId, KeyGeneration), BackupCreationError<PE, GE>> {
    let authenticated = authenticate_for_open(active_vrk, trusted_anchor, source_manifest_envelope)
        .map_err(BackupCreationError::Freshness)?;
    if authenticated.decision() != FreshnessDecision::Current {
        return Err(BackupCreationError::FreshnessNotCurrent);
    }
    let manifest = authenticated.manifest();
    let generation = manifest.active_key_generation();
    let vault_id = manifest.vault_id();
    if manifest.rotation_phase() != RotationPhase::None
        || manifest.rotation_target_generation().is_some()
        || manifest
            .objects()
            .iter()
            .any(|object| object.key_generation() != generation)
    {
        return Err(BackupCreationError::BackupStateNotStable);
    }
    if lease.identity() != VaultLeaseIdentity::new(vault_id, generation) {
        return Err(BackupCreationError::LeaseIdentityMismatch);
    }

    let mut structured = None;
    let mut expected_generic = HashMap::new();
    expected_generic
        .try_reserve(manifest.objects().len())
        .map_err(|_| BackupCreationError::InventoryMismatch)?;
    for object in manifest.objects() {
        match (object.kind(), object.auth_metadata()) {
            (ManifestObjectKind::StructuredStore, ManifestAuthMetadata::StructuredStore) => {
                if structured.replace(object).is_some() {
                    return Err(BackupCreationError::InventoryMismatch);
                }
            }
            (
                ManifestObjectKind::GenericArtifactBlob,
                ManifestAuthMetadata::GenericArtifactBlob { .. },
            ) => {
                if expected_generic
                    .insert(object.storage_id(), object)
                    .is_some()
                {
                    return Err(BackupCreationError::InventoryMismatch);
                }
            }
            _ => return Err(BackupCreationError::InventoryMismatch),
        }
    }
    let structured = structured.ok_or(BackupCreationError::InventoryMismatch)?;
    let structured_identity = regular_file_identity(source_sqlcipher_path)
        .map_err(|_| BackupCreationError::SourceReadFailed)?;
    if structured_identity
        != (
            structured.ciphertext_length(),
            structured.ciphertext_sha256(),
        )
    {
        return Err(BackupCreationError::InventoryMismatch);
    }
    if generic_sources.len() != expected_generic.len() {
        return Err(BackupCreationError::InventoryMismatch);
    }
    let mut seen = HashMap::new();
    seen.try_reserve(generic_sources.len())
        .map_err(|_| BackupCreationError::InventoryMismatch)?;
    for source in generic_sources {
        let object = expected_generic
            .get(&source.storage_id())
            .ok_or(BackupCreationError::InventoryMismatch)?;
        if seen.insert(source.storage_id(), ()).is_some() {
            return Err(BackupCreationError::InventoryMismatch);
        }
        let identity = regular_file_identity(source.path())
            .map_err(|_| BackupCreationError::SourceReadFailed)?;
        if identity != (object.ciphertext_length(), object.ciphertext_sha256()) {
            return Err(BackupCreationError::InventoryMismatch);
        }
    }
    Ok((structured.storage_id(), vault_id, generation))
}

#[allow(clippy::too_many_arguments)]
pub fn create_publish_and_verify_portable_backup<G, P>(
    guard: &mut G,
    provider: &mut P,
    trusted_anchor: FreshnessAnchor,
    lease: KeyedHandleLease,
    active_vrk: &OwnedKeyMaterial,
    recovery_envelope: &[u8],
    passphrase: &str,
    source_manifest_envelope: &[u8],
    source_sqlcipher_path: &Path,
    generic_sources: &[BackupSourceFile],
    snapshot_path: &Path,
    packaging_directory: &Path,
    verification_sqlcipher_path: &Path,
) -> Result<AcceptedPortableBackup, BackupCreationError<P::Error, G::Error>>
where
    G: BackupSnapshotQuiescenceGuard,
    P: PortableBackupProvider,
{
    validate_staging_paths(
        snapshot_path,
        packaging_directory,
        verification_sqlcipher_path,
    )
    .map_err(|_| BackupCreationError::StagingPath)?;
    guard
        .assert_normal_writes_quiesced()
        .map_err(BackupCreationError::SnapshotQuiescence)?;
    let (structured_storage_id, vault_id, generation) = preflight_inventory(
        active_vrk,
        trusted_anchor,
        source_manifest_envelope,
        source_sqlcipher_path,
        generic_sources,
        &lease,
    )?;
    guard
        .assert_normal_writes_quiesced()
        .map_err(BackupCreationError::SnapshotQuiescence)?;
    let context = KeyDerivationContext::new(vault_id, generation, KeyPurpose::StructuredStore);
    let snapshot = snapshot_quiesced_sqlcipher_database(
        guard,
        source_sqlcipher_path,
        snapshot_path,
        lease,
        context,
        active_vrk,
    )
    .map_err(BackupCreationError::Snapshot)?;

    let mut packaging_owned = false;
    let mut verification_owned = false;
    let mut sources = generic_sources.to_vec();
    sources.push(BackupSourceFile::from_sqlcipher_snapshot(
        structured_storage_id,
        &snapshot,
    ));
    let result = (|| {
        guard
            .assert_normal_writes_quiesced()
            .map_err(BackupCreationError::SnapshotQuiescence)?;
        let mut prepared = prepare_disk_backed_backup_set(
            active_vrk,
            recovery_envelope,
            passphrase,
            source_manifest_envelope,
            &sources,
            packaging_directory,
        )
        .map_err(BackupCreationError::Packaging)?;
        packaging_owned = true;
        guard
            .assert_normal_writes_quiesced()
            .map_err(BackupCreationError::SnapshotQuiescence)?;
        let descriptor = prepared.descriptor_bytes().to_vec();
        let publication = publish_and_reread_backup_source(provider, &descriptor, &mut prepared)
            .map_err(BackupCreationError::Publication)?;
        guard
            .assert_normal_writes_quiesced()
            .map_err(BackupCreationError::SnapshotQuiescence)?;

        let mut verification_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(verification_sqlcipher_path)
            .map_err(|_| BackupCreationError::VerificationStaging)?;
        verification_owned = true;
        let pre_sqlcipher = verify_backup_semantics_before_sqlcipher(
            provider,
            &descriptor,
            passphrase,
            &mut verification_file,
        )
        .map_err(BackupCreationError::Semantic)?;
        verification_file
            .sync_all()
            .map_err(|_| BackupCreationError::VerificationStaging)?;
        drop(verification_file);
        if pre_sqlcipher.set_id() != publication.set_id()
            || pre_sqlcipher.vault_id() != vault_id
            || pre_sqlcipher.key_generation() != generation
            || pre_sqlcipher.source_freshness_epoch() != trusted_anchor.highest_epoch()
            || pre_sqlcipher.source_manifest_hash() != trusted_anchor.manifest_hash()
        {
            return Err(BackupCreationError::CorruptOrTampered);
        }
        let verified = verify_staged_sqlcipher_backup(verification_sqlcipher_path, pre_sqlcipher)
            .map_err(BackupCreationError::SqlCipher)?;
        guard
            .assert_normal_writes_quiesced()
            .map_err(BackupCreationError::SnapshotQuiescence)?;
        let pre = verified.pre_sqlcipher();
        Ok(AcceptedPortableBackup {
            set_id: pre.set_id(),
            vault_id,
            key_generation: generation,
            source_freshness_epoch: pre.source_freshness_epoch(),
            source_manifest_hash: pre.source_manifest_hash(),
            data_object_count: publication.data_object_count(),
        })
    })();

    if packaging_owned {
        let _ = std::fs::remove_dir_all(packaging_directory);
    }
    remove_database_candidate(snapshot_path);
    if verification_owned {
        remove_database_candidate(verification_sqlcipher_path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{FreshnessEpoch, ManifestHash, VaultId, VaultLeaseIdentity};
    use crate::vault_blob::{BoundedBlobContext, encrypt_bounded_blob};
    use crate::vault_lease::VaultLease;
    use crate::vault_manifest::{
        GenerationState, ManifestAuthMetadata, ManifestContext, ManifestGeneration, ManifestObject,
        ManifestPlaintext, encrypt_fresh_manifest, manifest_hash,
    };
    use crate::vault_nonce::NonceReservationLedger;
    use crate::vault_recovery::{RecoveryContext, encrypt_recovery_envelope};
    use rusqlite::Connection;
    use std::collections::HashMap;
    use std::fmt::Write as _;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

    const PASSPHRASE: &str = "correct horse battery staple";

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ProviderError {
        Missing,
        Exists,
    }

    #[derive(Default)]
    struct MemoryProvider {
        objects: HashMap<String, Vec<u8>>,
        writes: usize,
    }
    impl PortableBackupProvider for MemoryProvider {
        type Error = ProviderError;
        fn put_if_absent(&mut self, key: &str, bytes: &[u8]) -> Result<(), Self::Error> {
            if self.objects.contains_key(key) {
                return Err(ProviderError::Exists);
            }
            self.objects.insert(key.to_owned(), bytes.to_vec());
            self.writes += 1;
            Ok(())
        }
        fn read(&mut self, key: &str) -> Result<Vec<u8>, Self::Error> {
            self.objects.get(key).cloned().ok_or(ProviderError::Missing)
        }
    }

    #[derive(Default)]
    struct Guard {
        checks: usize,
    }
    impl BackupSnapshotQuiescenceGuard for Guard {
        type Error = ();
        fn assert_normal_writes_quiesced(&mut self) -> Result<(), Self::Error> {
            self.checks += 1;
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
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let unique = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "himsat-b505o-{label}-{}-{nanos}-{unique}",
            std::process::id()
        ))
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

    struct Fixture {
        key: OwnedKeyMaterial,
        source_db: PathBuf,
        open_source: Connection,
        blob_path: PathBuf,
        manifest: Vec<u8>,
        recovery: Vec<u8>,
        anchor: FreshnessAnchor,
        blob_source: BackupSourceFile,
        precheckpoint_db_identity: (u64, [u8; 32]),
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
        open_source.execute_batch(
            "PRAGMA wal_autocheckpoint = 0; CREATE TABLE backup_e2e (value INTEGER NOT NULL); BEGIN IMMEDIATE; INSERT INTO backup_e2e VALUES (41); COMMIT;",
        ).unwrap();
        let wal = PathBuf::from(format!("{}-wal", source_db.display()));
        assert!(std::fs::metadata(&wal).unwrap().len() > 0);
        let (db_len, db_sha) = regular_file_identity(&source_db).unwrap();

        let blob_id = [0x51; 16];
        let blob_storage = [0x52; 16];
        let blob_nonce = [0x53; 24];
        let blob = encrypt_bounded_blob(
            &key,
            BoundedBlobContext::new(vault_id(), blob_id, generation()),
            blob_nonce,
            b"B505O authenticated blob",
        )
        .unwrap();
        let blob_path = unused_path("blob.bin");
        std::fs::write(&blob_path, &blob).unwrap();
        let blob_sha: [u8; 32] = Sha256::digest(&blob).into();

        let epoch = FreshnessEpoch::new(9).unwrap();
        let plaintext = ManifestPlaintext::new(
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
                    blob_id,
                    blob_storage,
                    generation(),
                    blob.len() as u64,
                    blob_sha,
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
        let mut ledger = NonceReservationLedger::new(vault_id());
        let (_, manifest) = encrypt_fresh_manifest(
            &mut ledger,
            &key,
            ManifestContext::new(vault_id(), generation(), epoch),
            &plaintext,
        )
        .unwrap();
        let anchor = FreshnessAnchor::new(vault_id(), epoch, manifest_hash(&manifest));
        let recovery = encrypt_recovery_envelope(
            &key,
            RecoveryContext::new(vault_id(), generation()),
            PASSPHRASE,
        )
        .unwrap();
        Fixture {
            key,
            source_db,
            open_source,
            blob_path: blob_path.clone(),
            manifest,
            recovery,
            anchor,
            blob_source: BackupSourceFile::new(blob_storage, blob_path),
            precheckpoint_db_identity: (db_len, db_sha),
        }
    }

    fn cleanup(fixture: Fixture) {
        drop(fixture.open_source);
        remove_database_candidate(&fixture.source_db);
        let _ = std::fs::remove_file(fixture.blob_path);
    }

    #[test]
    fn wal_backed_creation_is_published_reread_and_sqlcipher_verified() {
        let fixture = fixture();
        let snapshot = unused_path("snapshot.sqlite3");
        let package = unused_path("package");
        let verify = unused_path("verify.sqlite3");
        let lease = VaultLease::new(VaultLeaseIdentity::new(vault_id(), generation()));
        let mut provider = MemoryProvider::default();
        let mut guard = Guard::default();
        let accepted = create_publish_and_verify_portable_backup(
            &mut guard,
            &mut provider,
            fixture.anchor,
            lease.keyed_handle_lease(),
            &fixture.key,
            &fixture.recovery,
            PASSPHRASE,
            &fixture.manifest,
            &fixture.source_db,
            std::slice::from_ref(&fixture.blob_source),
            &snapshot,
            &package,
            &verify,
        )
        .unwrap();
        assert_eq!(accepted.vault_id(), vault_id());
        assert_eq!(accepted.key_generation(), generation());
        assert_eq!(
            accepted.source_freshness_epoch(),
            fixture.anchor.highest_epoch()
        );
        assert_eq!(
            accepted.source_manifest_hash(),
            fixture.anchor.manifest_hash()
        );
        assert!(accepted.data_object_count() >= 3);
        assert!(provider.writes >= 4);
        assert!(guard.checks >= 6);
        assert_ne!(
            regular_file_identity(&fixture.source_db).unwrap(),
            fixture.precheckpoint_db_identity
        );
        assert!(!snapshot.exists() && !package.exists() && !verify.exists());
        cleanup(fixture);
    }

    #[test]
    fn wrong_trusted_anchor_fails_before_snapshot_or_provider_write() {
        let fixture = fixture();
        let snapshot = unused_path("wrong-snapshot.sqlite3");
        let package = unused_path("wrong-package");
        let verify = unused_path("wrong-verify.sqlite3");
        let wrong = FreshnessAnchor::new(
            vault_id(),
            fixture.anchor.highest_epoch(),
            ManifestHash::from_bytes([0xEE; 32]),
        );
        let lease = VaultLease::new(VaultLeaseIdentity::new(vault_id(), generation()));
        let mut provider = MemoryProvider::default();
        let mut guard = Guard::default();
        let result = create_publish_and_verify_portable_backup(
            &mut guard,
            &mut provider,
            wrong,
            lease.keyed_handle_lease(),
            &fixture.key,
            &fixture.recovery,
            PASSPHRASE,
            &fixture.manifest,
            &fixture.source_db,
            std::slice::from_ref(&fixture.blob_source),
            &snapshot,
            &package,
            &verify,
        );
        assert!(matches!(result, Err(BackupCreationError::Freshness(_))));
        assert_eq!(provider.writes, 0);
        assert!(!snapshot.exists() && !package.exists() && !verify.exists());
        cleanup(fixture);
    }

    #[test]
    fn preexisting_verification_target_fails_before_snapshot_or_provider_write() {
        let fixture = fixture();
        let snapshot = unused_path("occupied-snapshot.sqlite3");
        let package = unused_path("occupied-package");
        let verify = unused_path("occupied-verify.sqlite3");
        std::fs::write(&verify, b"existing verification target").unwrap();
        let lease = VaultLease::new(VaultLeaseIdentity::new(vault_id(), generation()));
        let mut provider = MemoryProvider::default();
        let mut guard = Guard::default();
        let result = create_publish_and_verify_portable_backup(
            &mut guard,
            &mut provider,
            fixture.anchor,
            lease.keyed_handle_lease(),
            &fixture.key,
            &fixture.recovery,
            PASSPHRASE,
            &fixture.manifest,
            &fixture.source_db,
            std::slice::from_ref(&fixture.blob_source),
            &snapshot,
            &package,
            &verify,
        );
        assert!(matches!(result, Err(BackupCreationError::StagingPath)));
        assert_eq!(provider.writes, 0);
        assert_eq!(
            std::fs::read(&verify).unwrap(),
            b"existing verification target"
        );
        assert!(!snapshot.exists() && !package.exists());
        std::fs::remove_file(&verify).unwrap();
        cleanup(fixture);
    }
}
