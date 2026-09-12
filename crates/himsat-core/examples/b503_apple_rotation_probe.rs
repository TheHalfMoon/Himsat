#[cfg(target_os = "macos")]
use himsat_core::vault::{
    AccessScope, FreshnessAnchor, FreshnessEpoch, HardwareBacking, KeyGeneration, ManifestHash,
    ProtectedFreshnessState, ProtectorError, SecretProtector, UserPresencePolicy, VaultId,
    VaultLeaseIdentity, VaultLeaseState,
};
#[cfg(target_os = "macos")]
use himsat_core::vault_apple_keychain::{
    AppleKeychainConfig, AppleKeychainProtector, AppleProtectorId,
};
#[cfg(target_os = "macos")]
use himsat_core::vault_blob::{BoundedBlobContext, bounded_blob_nonce, decrypt_bounded_blob};
#[cfg(target_os = "macos")]
use himsat_core::vault_keys::{
    KeyedHandleCloser, OwnedKeyMaterial, PlaintextCache, VaultKeyMaterial, VaultSessionLifetime,
};
#[cfg(target_os = "macos")]
use himsat_core::vault_lease::VaultLease;
#[cfg(target_os = "macos")]
use himsat_core::vault_manifest::{
    GenerationState, ManifestAuthMetadata, ManifestContext, ManifestGeneration, ManifestObject,
    ManifestObjectKind, ManifestPlaintext, RotationPhase, encrypt_fresh_manifest, manifest_hash,
};
#[cfg(target_os = "macos")]
use himsat_core::vault_nonce::NonceReservationLedger;
#[cfg(target_os = "macos")]
use himsat_core::vault_protector::ProtectorPolicy;
#[cfg(target_os = "macos")]
use himsat_core::vault_rotation::{
    FullRotationBackend, FullRotationProgress, FullRotationProtectorBinding, advance_full_rotation,
    begin_full_rotation, quiesce_for_full_rotation, resume_full_rotation_after_restart,
};
#[cfg(target_os = "macos")]
use security_framework::item::{ItemClass, ItemSearchOptions, SearchResult};
#[cfg(target_os = "macos")]
use sha2::{Digest, Sha256};
#[cfg(target_os = "macos")]
use std::convert::Infallible;

#[cfg(target_os = "macos")]
const SOURCE_BLOB_STORAGE: [u8; 16] = [0x81; 16];
#[cfg(target_os = "macos")]
const TARGET_BLOB_STORAGE: [u8; 16] = [0x82; 16];
#[cfg(target_os = "macos")]
const BLOB_ID: [u8; 16] = [0x83; 16];

#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProbeError(&'static str);

#[cfg(target_os = "macos")]
impl std::fmt::Display for ProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[cfg(target_os = "macos")]
impl std::error::Error for ProbeError {}

#[cfg(target_os = "macos")]
struct ProbeBackend {
    vault_id: VaultId,
    quiesced: bool,
    binding: Option<FullRotationProtectorBinding>,
    checkpoint: Option<Vec<u8>>,
    published: Option<Vec<u8>>,
    source_blob: Vec<u8>,
    target_blob: Option<Vec<u8>>,
    staged: Vec<ManifestObject>,
    expected_plaintext: Vec<u8>,
    source_generation: KeyGeneration,
    target_generation: Option<KeyGeneration>,
    source_retired: bool,
    target_activated: bool,
}

#[cfg(target_os = "macos")]
impl ProbeBackend {
    fn new(
        vault_id: VaultId,
        source_blob: Vec<u8>,
        expected_plaintext: Vec<u8>,
        source_generation: KeyGeneration,
    ) -> Self {
        Self {
            vault_id,
            quiesced: false,
            binding: None,
            checkpoint: None,
            published: None,
            source_blob,
            target_blob: None,
            staged: Vec::new(),
            expected_plaintext,
            source_generation,
            target_generation: None,
            source_retired: false,
            target_activated: false,
        }
    }

    fn sha256(bytes: &[u8]) -> [u8; 32] {
        Sha256::digest(bytes).into()
    }

    fn verify_target(
        &self,
        vault_id: VaultId,
        target_vrk: &OwnedKeyMaterial,
        objects: &[ManifestObject],
    ) -> Result<(), ProbeError> {
        let generation = self
            .target_generation
            .ok_or(ProbeError("target generation missing"))?;
        let blob = self
            .target_blob
            .as_ref()
            .ok_or(ProbeError("target blob missing"))?;
        let plaintext = decrypt_bounded_blob(
            target_vrk,
            BoundedBlobContext::new(vault_id, BLOB_ID, generation),
            blob,
        )
        .map_err(|_| ProbeError("target blob authentication failed"))?;
        if plaintext != self.expected_plaintext || objects.len() != 1 {
            return Err(ProbeError("target plaintext or inventory mismatch"));
        }
        let nonce = bounded_blob_nonce(blob).map_err(|_| ProbeError("target nonce missing"))?;
        let expected_auth = ManifestAuthMetadata::GenericArtifactBlob { nonce };
        let object = &objects[0];
        if object.kind() != ManifestObjectKind::GenericArtifactBlob
            || object.logical_id() != BLOB_ID
            || object.storage_id() != TARGET_BLOB_STORAGE
            || object.key_generation() != generation
            || object.ciphertext_length() != blob.len() as u64
            || object.ciphertext_sha256() != Self::sha256(blob)
            || object.auth_metadata() != expected_auth
        {
            return Err(ProbeError("target manifest inventory mismatch"));
        }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
impl FullRotationBackend for ProbeBackend {
    type Error = ProbeError;

    fn assert_normal_writes_quiesced(&self) -> Result<(), Self::Error> {
        if self.quiesced {
            Ok(())
        } else {
            Err(ProbeError("ordinary writes are not quiesced"))
        }
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
        match self.binding {
            Some(existing) if existing != binding => Err(ProbeError("binding conflict")),
            _ => {
                self.binding = Some(binding);
                Ok(())
            }
        }
    }

    fn clear_rotation_protector_binding(&mut self) -> Result<(), Self::Error> {
        self.binding = None;
        Ok(())
    }

    fn read_rotation_checkpoint(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.checkpoint.clone())
    }

    fn persist_rotation_checkpoint(&mut self, envelope: &[u8]) -> Result<(), Self::Error> {
        self.checkpoint = Some(envelope.to_vec());
        Ok(())
    }

    fn clear_rotation_checkpoint(&mut self) -> Result<(), Self::Error> {
        self.checkpoint = None;
        Ok(())
    }

    fn read_target_recovery_wrap(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }

    fn persist_target_recovery_wrap(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        Err(ProbeError("unexpected recovery wrap"))
    }

    fn remove_target_recovery_wrap(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn stage_reencrypted_inventory(
        &mut self,
        source_vrk: &OwnedKeyMaterial,
        target_vrk: &OwnedKeyMaterial,
        source_manifest: &ManifestPlaintext,
        target_generation: KeyGeneration,
        nonce_ledger: &mut NonceReservationLedger,
    ) -> Result<Vec<ManifestObject>, Self::Error> {
        if source_manifest.objects().len() != 1 {
            return Err(ProbeError("source inventory mismatch"));
        }
        let source = &source_manifest.objects()[0];
        if source.kind() != ManifestObjectKind::GenericArtifactBlob
            || source.logical_id() != BLOB_ID
            || source.storage_id() != SOURCE_BLOB_STORAGE
            || source.key_generation() != self.source_generation
            || source.ciphertext_length() != self.source_blob.len() as u64
            || source.ciphertext_sha256() != Self::sha256(&self.source_blob)
        {
            return Err(ProbeError("source inventory mismatch"));
        }
        let vault_id = source_manifest.vault_id();
        let plaintext = decrypt_bounded_blob(
            source_vrk,
            BoundedBlobContext::new(vault_id, BLOB_ID, self.source_generation),
            &self.source_blob,
        )
        .map_err(|_| ProbeError("source blob authentication failed"))?;
        if plaintext != self.expected_plaintext {
            return Err(ProbeError("source plaintext mismatch"));
        }
        self.target_generation = Some(target_generation);
        if self.target_blob.is_none() {
            let candidate = nonce_ledger
                .encrypt_fresh_bounded_blob(
                    target_vrk,
                    BoundedBlobContext::new(vault_id, BLOB_ID, target_generation),
                    &plaintext,
                )
                .map_err(|_| ProbeError("target blob encryption failed"))?;
            self.target_blob = Some(candidate.into_parts().1);
        }
        let blob = self
            .target_blob
            .as_ref()
            .ok_or(ProbeError("target blob missing"))?;
        let nonce = bounded_blob_nonce(blob).map_err(|_| ProbeError("target nonce missing"))?;
        self.staged = vec![ManifestObject::new(
            BLOB_ID,
            TARGET_BLOB_STORAGE,
            target_generation,
            blob.len() as u64,
            Self::sha256(blob),
            ManifestAuthMetadata::GenericArtifactBlob { nonce },
        )];
        Ok(self.staged.clone())
    }

    fn verify_staged_inventory(
        &mut self,
        target_vrk: &OwnedKeyMaterial,
        staged_objects: &[ManifestObject],
    ) -> Result<(), Self::Error> {
        if staged_objects != self.staged.as_slice() {
            return Err(ProbeError("staged inventory mismatch"));
        }
        self.verify_target(self.vault_id, target_vrk, staged_objects)
    }

    fn quarantine_target_inventory(&mut self) -> Result<(), Self::Error> {
        self.target_blob = None;
        self.staged.clear();
        Ok(())
    }

    fn publish_manifest(&mut self, envelope: &[u8]) -> Result<(), Self::Error> {
        self.published = Some(envelope.to_vec());
        Ok(())
    }

    fn read_published_manifest(&self) -> Result<Vec<u8>, Self::Error> {
        self.published
            .clone()
            .ok_or(ProbeError("published manifest missing"))
    }

    fn reopen_and_verify_published(
        &mut self,
        target_vrk: &OwnedKeyMaterial,
        manifest: &ManifestPlaintext,
    ) -> Result<(), Self::Error> {
        self.verify_target(manifest.vault_id(), target_vrk, manifest.objects())
    }

    fn activate_target_protector(&mut self) -> Result<(), Self::Error> {
        self.target_activated = true;
        Ok(())
    }

    fn retire_source_inventory(
        &mut self,
        source_generation: KeyGeneration,
    ) -> Result<(), Self::Error> {
        if source_generation != self.source_generation {
            return Err(ProbeError("source generation mismatch"));
        }
        self.source_blob.clear();
        self.source_retired = true;
        Ok(())
    }

    fn retire_source_recovery_wrap(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
struct NoopCloser;

#[cfg(target_os = "macos")]
impl KeyedHandleCloser for NoopCloser {
    type Error = Infallible;
    fn close_keyed_handles(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
#[derive(Default)]
struct NoopCache;

#[cfg(target_os = "macos")]
impl PlaintextCache for NoopCache {
    fn discard_plaintext(&mut self) {}
}

#[cfg(target_os = "macos")]
fn attributes(
    service: &str,
    account: &str,
) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut search = ItemSearchOptions::new();
    search
        .class(ItemClass::generic_password())
        .service(service)
        .account(account)
        .cloud_sync(Some(false))
        .load_attributes(true)
        .limit(1)
        .ignore_legacy_keychains();
    let results = search.search()?;
    let Some(SearchResult::Dict(_)) = results.first() else {
        return Err("expected one Data Protection Keychain item".into());
    };
    results[0]
        .simplify_dict()
        .ok_or_else(|| "failed to simplify Keychain attributes".into())
}

#[cfg(target_os = "macos")]
fn item_exists(service: &str, account: &str) -> Result<bool, security_framework::base::Error> {
    let mut search = ItemSearchOptions::new();
    search
        .class(ItemClass::generic_password())
        .service(service)
        .account(account)
        .cloud_sync(Some(false))
        .load_attributes(true)
        .limit(1)
        .ignore_legacy_keychains();
    match search.search() {
        Ok(results) => Ok(!results.is_empty()),
        Err(error) if error.code() == -25300 => Ok(false),
        Err(error) => Err(error),
    }
}

#[cfg(target_os = "macos")]
fn synchronized_item_exists(
    service: &str,
    account: &str,
) -> Result<bool, security_framework::base::Error> {
    let mut search = ItemSearchOptions::new();
    search
        .class(ItemClass::generic_password())
        .service(service)
        .account(account)
        .cloud_sync(Some(true))
        .load_attributes(true)
        .limit(1)
        .ignore_legacy_keychains();
    match search.search() {
        Ok(results) => Ok(!results.is_empty()),
        Err(error) if error.code() == -25300 => Ok(false),
        Err(error) => Err(error),
    }
}

#[cfg(target_os = "macos")]
struct Cleanup {
    vault_id: VaultId,
    source: AppleKeychainConfig,
    target: AppleKeychainConfig,
}

#[cfg(target_os = "macos")]
impl Cleanup {
    fn remove_config(vault_id: VaultId, config: AppleKeychainConfig) -> Result<(), ProtectorError> {
        let mut protector = AppleKeychainProtector::new(config);
        protector.create_protector(
            AccessScope::SameUserAccount,
            UserPresencePolicy::NotRequired,
        )?;
        match protector.remove_protector(vault_id) {
            Ok(()) | Err(ProtectorError::ItemMissing) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn cleanup(&self) -> Result<(), ProtectorError> {
        Self::remove_config(self.vault_id, self.source.clone())?;
        Self::remove_config(self.vault_id, self.target.clone())
    }
}

#[cfg(target_os = "macos")]
impl Drop for Cleanup {
    fn drop(&mut self) {
        for (label, config) in [
            ("SOURCE", self.source.clone()),
            ("TARGET", self.target.clone()),
        ] {
            if let Err(error) = Self::remove_config(self.vault_id, config) {
                eprintln!("B503_NATIVE_APPLE_CLEANUP_{label}=ERROR:{error:?}");
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = "com.thehalfmoon.himsat.b503.rotation.qualifier";
    let vault_id = VaultId::from_bytes([0x91; 16]);
    let source_generation = KeyGeneration::new(17)?;
    let source_epoch = FreshnessEpoch::new(23)?;
    let source_id = AppleProtectorId::generate()?;
    let target_id = AppleProtectorId::generate()?;
    assert_ne!(source_id, target_id);
    let source_config = AppleKeychainConfig::new(service, source_id)?;
    let target_config = AppleKeychainConfig::new(service, target_id)?;
    let _cleanup = Cleanup {
        vault_id,
        source: source_config.clone(),
        target: target_config.clone(),
    };
    let mut source = AppleKeychainProtector::new(source_config);
    let mut target = AppleKeychainProtector::new(target_config);
    source.create_protector(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    )?;
    assert_eq!(source.actual_access_scope(), AccessScope::SameUserAccount);
    assert!(!source.requires_user_presence());
    assert_eq!(source.hardware_backed_state(), HardwareBacking::Unknown);

    let mut source_key_bytes = [0_u8; 32];
    getrandom::fill(&mut source_key_bytes)?;
    let source_vrk = OwnedKeyMaterial::from_bytes(source_key_bytes);
    source.protect_or_store_vrk(vault_id, source_generation, &source_vrk)?;

    let blob_plaintext = b"HIMSAT_B503_NATIVE_APPLE_ROTATION_BLOB_7D91".to_vec();
    let mut ledger = NonceReservationLedger::new(vault_id);
    let source_blob_candidate = ledger.encrypt_fresh_bounded_blob(
        &source_vrk,
        BoundedBlobContext::new(vault_id, BLOB_ID, source_generation),
        &blob_plaintext,
    )?;
    let source_blob_nonce = source_blob_candidate.reservation().nonce();
    let source_blob = source_blob_candidate.into_parts().1;
    let source_blob_hash: [u8; 32] = Sha256::digest(&source_blob).into();
    let manifest = ManifestPlaintext::new(
        vault_id,
        source_epoch,
        ManifestHash::from_bytes([0x92; 32]),
        source_generation,
        (RotationPhase::None, None),
        vec![ManifestGeneration::new(
            source_generation,
            GenerationState::Active,
        )],
        vec![ManifestObject::new(
            BLOB_ID,
            SOURCE_BLOB_STORAGE,
            source_generation,
            source_blob.len() as u64,
            source_blob_hash,
            ManifestAuthMetadata::GenericArtifactBlob {
                nonce: source_blob_nonce,
            },
        )],
    )?;
    let manifest_context = ManifestContext::new(vault_id, source_generation, source_epoch);
    let (_, current_manifest) =
        encrypt_fresh_manifest(&mut ledger, &source_vrk, manifest_context, &manifest)?;
    let source_anchor =
        FreshnessAnchor::new(vault_id, source_epoch, manifest_hash(&current_manifest));
    source.install_genesis_freshness_anchor(
        vault_id,
        ProtectedFreshnessState::Uninitialized,
        source_anchor,
    )?;
    let source_attrs = attributes(service, &source_id.account())?;
    assert_eq!(source_attrs.get("pdmn").map(String::as_str), Some("akpu"));
    assert!(!synchronized_item_exists(service, &source_id.account())?);

    let lease = VaultLease::new(VaultLeaseIdentity::new(vault_id, source_generation));
    let mut session = VaultSessionLifetime::new(
        lease,
        NoopCloser,
        VaultKeyMaterial::new(source_vrk),
        NoopCache,
    );
    let quiesced = quiesce_for_full_rotation(&mut session)?;
    assert_eq!(session.lease_state(), VaultLeaseState::Revoked);
    assert!(session.keys_released());

    let mut backend = ProbeBackend::new(
        vault_id,
        source_blob.clone(),
        blob_plaintext.clone(),
        source_generation,
    );
    backend.quiesced = true;
    let binding = FullRotationProtectorBinding::new(*source_id.as_bytes(), *target_id.as_bytes())
        .ok_or("distinct Apple protector ids required")?;
    let policy = ProtectorPolicy::new(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    );
    let prepared = begin_full_rotation(
        quiesced,
        &mut source,
        &mut target,
        &mut backend,
        &mut ledger,
        &current_manifest,
        binding,
        policy,
        None,
    )?;
    let identity = match prepared {
        FullRotationProgress::InProgress { identity, phase } => {
            assert_eq!(phase, RotationPhase::Prepare);
            identity
        }
        FullRotationProgress::Complete { .. } => {
            return Err("PREPARE completed unexpectedly".into());
        }
    };
    assert_eq!(identity.source_generation(), source_generation);
    assert_eq!(
        identity.target_generation().get(),
        source_generation.get() + 1
    );
    let source_check = source.unlock_vrk(vault_id, source_generation)?;
    assert_eq!(
        source.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Present(source_anchor)
    );
    assert_eq!(
        decrypt_bounded_blob(
            &source_check,
            BoundedBlobContext::new(vault_id, BLOB_ID, source_generation),
            &source_blob,
        )?,
        blob_plaintext
    );
    assert!(item_exists(service, &source_id.account())?);
    assert!(item_exists(service, &target_id.account())?);
    let target_attrs = attributes(service, &target_id.account())?;
    assert_eq!(target_attrs.get("pdmn").map(String::as_str), Some("akpu"));
    assert!(!synchronized_item_exists(service, &target_id.account())?);
    assert_eq!(target.actual_access_scope(), AccessScope::SameUserAccount);
    assert!(!target.requires_user_presence());
    let target_check = target.unlock_vrk(vault_id, identity.target_generation())?;
    assert!(
        decrypt_bounded_blob(
            &target_check,
            BoundedBlobContext::new(vault_id, BLOB_ID, source_generation),
            &source_blob,
        )
        .is_err()
    );

    for expected_phase in [
        RotationPhase::Stage,
        RotationPhase::Verify,
        RotationPhase::Publish,
        RotationPhase::Anchor,
        RotationPhase::Activate,
        RotationPhase::Retire,
    ] {
        let progress = advance_full_rotation(
            quiesced,
            identity,
            binding,
            &mut source,
            &mut target,
            &mut backend,
            &mut ledger,
        )?;
        assert_eq!(
            progress,
            FullRotationProgress::InProgress {
                identity,
                phase: expected_phase,
            }
        );
        assert!(item_exists(service, &source_id.account())?);
        assert!(!backend.source_retired);
        if expected_phase == RotationPhase::Stage {
            let target_blob = backend.target_blob.as_ref().ok_or("target blob missing")?;
            assert!(
                decrypt_bounded_blob(
                    &source_check,
                    BoundedBlobContext::new(vault_id, BLOB_ID, identity.target_generation(),),
                    target_blob,
                )
                .is_err()
            );
        }
    }
    let complete = advance_full_rotation(
        quiesced,
        identity,
        binding,
        &mut source,
        &mut target,
        &mut backend,
        &mut ledger,
    )?;
    let final_anchor = match complete {
        FullRotationProgress::Complete {
            identity: completed,
            final_anchor,
        } => {
            assert_eq!(completed, identity);
            final_anchor
        }
        FullRotationProgress::InProgress { .. } => return Err("RETIRE did not complete".into()),
    };
    assert!(backend.source_retired);
    assert!(backend.target_activated);
    assert!(!item_exists(service, &source_id.account())?);
    assert!(item_exists(service, &target_id.account())?);
    assert_eq!(
        target.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Present(final_anchor)
    );
    let target_vrk = target.unlock_vrk(vault_id, identity.target_generation())?;
    backend.verify_target(vault_id, &target_vrk, &backend.staged)?;

    let mut restarted_source = AppleKeychainProtector::new(source.config().clone());
    let mut restarted_target = AppleKeychainProtector::new(target.config().clone());
    restarted_source.create_protector(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    )?;
    restarted_target.create_protector(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    )?;
    let recovered = resume_full_rotation_after_restart(
        identity,
        binding,
        &mut restarted_source,
        &mut restarted_target,
        &mut backend,
        &mut ledger,
    )?;
    assert_eq!(
        recovered,
        FullRotationProgress::Complete {
            identity,
            final_anchor,
        }
    );
    _cleanup.cleanup()?;

    println!("B503_NATIVE_APPLE_CLEANUP=PASS");
    println!("B503_NATIVE_APPLE_SOURCE_SCOPE=SAME_USER_ACCOUNT");
    println!("B503_NATIVE_APPLE_TARGET_SCOPE=SAME_USER_ACCOUNT");
    println!("B503_NATIVE_APPLE_PRESENCE=NOT_REQUIRED");
    println!("B503_NATIVE_APPLE_ACCESSIBILITY=WHEN_PASSCODE_SET_THIS_DEVICE_ONLY");
    println!("B503_NATIVE_APPLE_SYNCHRONIZABLE=FALSE");
    println!("B503_NATIVE_APPLE_DISTINCT_PROTECTORS=PASS");
    println!("B503_NATIVE_APPLE_DISTINCT_VRK=PASS");
    println!("B503_NATIVE_APPLE_SOURCE_RETAINED_THROUGH_RETIRE=PASS");
    println!("B503_NATIVE_APPLE_TARGET_REOPEN=PASS");
    println!("B503_NATIVE_APPLE_STABLE_RECOVERY=PASS");
    println!("B503_NATIVE_APPLE_ROTATION_QUALIFICATION=PASS");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("B503_NATIVE_APPLE_ROTATION_QUALIFICATION=UNAVAILABLE_NON_MACOS");
}
