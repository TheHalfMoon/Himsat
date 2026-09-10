//! Windows current-user DPAPI adapter for Specification 004B4 B403.
//!
//! Protects one VRK with `Scope::User` in an ACL-restricted Himsat directory.
//! Provider-visible names are opaque; binding metadata and VRK stay inside DPAPI.
//! Reports `SAME_USER_ACCOUNT`, no per-unlock presence, and unknown hardware.

use crate::vault::{
    AccessScope, FreshnessAnchor, HardwareBacking, KeyGeneration, ProtectedFreshnessState,
    ProtectorCapabilities, ProtectorError, SecretProtector, UserPresencePolicy, VaultId,
};
use crate::vault_keys::{KEY_MATERIAL_BYTES, OwnedKeyMaterial};
use crate::vault_protector::{ProtectorPolicy, validate_requested_policy};
use getrandom::fill;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use windows_acl::acl::{ACL, AceType};
use windows_acl::helper::string_to_sid;
use windows_dpapi::{Scope, decrypt_data, encrypt_data};
use winsafe::{self as w, co};
use zeroize::Zeroizing;
const RECORD_MAGIC: &[u8] = b"HIMSAT/WINDOWS/DPAPI/VRK/v1\0";
const OWNER_TAG: &[u8] = b"HIMSAT/APP/v1\0";
const ENTROPY_MAGIC: &[u8] = b"HIMSAT/WINDOWS/DPAPI/ENTROPY/v1\0";
const PROTECTOR_ID_BYTES: usize = 16;
const RECORD_BYTES: usize =
    RECORD_MAGIC.len() + OWNER_TAG.len() + 16 + 8 + 1 + 1 + KEY_MATERIAL_BYTES;
const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
const FILE_ALL_ACCESS: u32 = 0x001f_01ff;
const INHERITED_ACE: u8 = 0x10;
const OBJECT_INHERIT_ACE: u8 = 0x01;
const CONTAINER_INHERIT_ACE: u8 = 0x02;
const MAX_CIPHERTEXT_BYTES: u64 = 16 * 1024;
/// Opaque provider-visible identifier used only for the ciphertext filename and
/// DPAPI additional-entropy domain binding.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct WindowsProtectorId([u8; PROTECTOR_ID_BYTES]);
impl WindowsProtectorId {
    /// Generates one opaque identifier from the approved OS CSPRNG path.
    pub fn generate() -> Result<Self, ProtectorError> {
        let mut bytes = [0_u8; PROTECTOR_ID_BYTES];
        fill(&mut bytes).map_err(|_| ProtectorError::Unavailable)?;
        Ok(Self(bytes))
    }
    #[must_use]
    pub const fn from_bytes(bytes: [u8; PROTECTOR_ID_BYTES]) -> Self {
        Self(bytes)
    }
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; PROTECTOR_ID_BYTES] {
        &self.0
    }
    #[must_use]
    pub fn file_name(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(PROTECTOR_ID_BYTES * 2 + 6);
        for byte in self.0 {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        output.push_str(".dpapi");
        output
    }
}
impl core::fmt::Debug for WindowsProtectorId {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("WindowsProtectorId([OPAQUE; 16 bytes])")
    }
}
/// Immutable Windows DPAPI storage configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsDpapiConfig {
    storage_root: PathBuf,
    protector_id: WindowsProtectorId,
}
impl WindowsDpapiConfig {
    pub fn new(
        storage_root: impl Into<PathBuf>,
        protector_id: WindowsProtectorId,
    ) -> Result<Self, ProtectorError> {
        let storage_root = storage_root.into();
        if storage_root.as_os_str().is_empty() || !storage_root.is_absolute() {
            return Err(ProtectorError::UnsupportedPolicy);
        }
        Ok(Self {
            storage_root,
            protector_id,
        })
    }
    #[must_use]
    pub fn storage_root(&self) -> &Path {
        &self.storage_root
    }
    #[must_use]
    pub const fn protector_id(&self) -> WindowsProtectorId {
        self.protector_id
    }
    fn record_path(&self) -> PathBuf {
        self.storage_root.join(self.protector_id.file_name())
    }
}
/// Windows current-user DPAPI implementation of the portable `SecretProtector` contract.
pub struct WindowsDpapiProtector {
    config: WindowsDpapiConfig,
    policy: Option<ProtectorPolicy>,
}
impl WindowsDpapiProtector {
    #[must_use]
    pub const fn new(config: WindowsDpapiConfig) -> Self {
        Self {
            config,
            policy: None,
        }
    }
    #[must_use]
    pub const fn config(&self) -> &WindowsDpapiConfig {
        &self.config
    }

    fn policy(&self) -> Result<ProtectorPolicy, ProtectorError> {
        self.policy.ok_or(ProtectorError::UnsupportedPolicy)
    }

    fn entropy(&self) -> Vec<u8> {
        let mut entropy = Vec::with_capacity(ENTROPY_MAGIC.len() + PROTECTOR_ID_BYTES);
        entropy.extend_from_slice(ENTROPY_MAGIC);
        entropy.extend_from_slice(self.config.protector_id().as_bytes());
        entropy
    }
    fn ensure_storage_root(&self) -> Result<(), ProtectorError> {
        match metadata_no_reparse(self.config.storage_root())? {
            Some(metadata) => {
                if !metadata.is_dir() {
                    return Err(ProtectorError::CorruptOrTampered);
                }
                verify_restricted_acl(self.config.storage_root(), true)
            }
            None => {
                fs::create_dir_all(self.config.storage_root()).map_err(map_io_error)?;
                reject_reparse_point(self.config.storage_root())?;
                restrict_acl_to_current_user(self.config.storage_root(), true)
            }
        }
    }

    fn verify_storage_root(&self) -> Result<(), ProtectorError> {
        reject_reparse_point(self.config.storage_root())?;
        if !self.config.storage_root().is_dir() {
            return Err(ProtectorError::CorruptOrTampered);
        }
        verify_restricted_acl(self.config.storage_root(), true)
    }

    fn read_ciphertext(&self) -> Result<Vec<u8>, ProtectorError> {
        self.verify_storage_root()?;
        let path = self.config.record_path();
        let metadata = metadata_no_reparse(&path)?.ok_or(ProtectorError::ItemMissing)?;
        verify_current_user_file_acl(&path)?;
        if metadata.len() == 0 || metadata.len() > MAX_CIPHERTEXT_BYTES {
            return Err(ProtectorError::CorruptOrTampered);
        }
        let mut file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(map_io_error)?;
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.read_to_end(&mut bytes).map_err(map_io_error)?;
        if bytes.len() as u64 != metadata.len() {
            return Err(ProtectorError::CorruptOrTampered);
        }
        Ok(bytes)
    }
    fn write_ciphertext(&self, ciphertext: &[u8]) -> Result<(), ProtectorError> {
        if ciphertext.is_empty() || ciphertext.len() as u64 > MAX_CIPHERTEXT_BYTES {
            return Err(ProtectorError::CorruptOrTampered);
        }
        self.verify_storage_root()?;
        let target = self.config.record_path();
        if metadata_no_reparse(&target)?.is_some() {
            return Err(ProtectorError::CorruptOrTampered);
        }

        let mut temporary = None;
        let mut file = None;
        for _ in 0..8 {
            let candidate_id = WindowsProtectorId::generate()?;
            let candidate = self
                .config
                .storage_root()
                .join(format!("{}.tmp", candidate_id.file_name()));
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&candidate)
            {
                Ok(opened) => {
                    temporary = Some(candidate);
                    file = Some(opened);
                    break;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(map_io_error(error)),
            }
        }
        let temporary = temporary.ok_or(ProtectorError::Unavailable)?;
        let mut file = file.ok_or(ProtectorError::Unavailable)?;
        let staged = (|| {
            reject_reparse_point(&temporary)?;
            restrict_acl_to_current_user(&temporary, false)?;
            file.write_all(ciphertext).map_err(map_io_error)?;
            file.sync_all().map_err(map_io_error)?;
            Ok::<(), ProtectorError>(())
        })();
        drop(file);
        if let Err(error) = staged {
            let _ = fs::remove_file(&temporary);
            return Err(error);
        }

        if let Err(error) = fs::rename(&temporary, &target) {
            let _ = fs::remove_file(&temporary);
            return Err(map_io_error(error));
        }
        reject_reparse_point(&target)?;
        verify_current_user_file_acl(&target)
    }

    fn decrypt_record(&self) -> Result<Zeroizing<Vec<u8>>, ProtectorError> {
        let ciphertext = self.read_ciphertext()?;
        let entropy = self.entropy();
        let plaintext = Zeroizing::new(
            decrypt_data(&ciphertext, Scope::User, Some(&entropy))
                .map_err(|_| ProtectorError::CorruptOrTampered)?,
        );
        if plaintext.len() != RECORD_BYTES {
            return Err(ProtectorError::CorruptOrTampered);
        }
        Ok(plaintext)
    }
}
impl SecretProtector for WindowsDpapiProtector {
    type VaultRootKey = OwnedKeyMaterial;

    fn create_protector(
        &mut self,
        requested_scope: AccessScope,
        user_presence_policy: UserPresencePolicy,
    ) -> Result<(), ProtectorError> {
        let policy = ProtectorPolicy::new(requested_scope, user_presence_policy);
        validate_requested_policy(
            ProtectorCapabilities::new(
                AccessScope::SameUserAccount,
                false,
                HardwareBacking::Unknown,
            ),
            policy,
        )?;
        self.ensure_storage_root()?;
        self.policy = Some(policy);
        Ok(())
    }

    fn protect_or_store_vrk(
        &mut self,
        vault_id: VaultId,
        key_generation: KeyGeneration,
        vrk: &Self::VaultRootKey,
    ) -> Result<(), ProtectorError> {
        let policy = self.policy()?;
        self.verify_storage_root()?;
        if metadata_no_reparse(&self.config.record_path())?.is_some() {
            let record = self.decrypt_record()?;
            validate_record_binding(&record, vault_id, Some(key_generation), policy)?;
            return if record_matches_vrk(&record, vrk)? {
                Ok(())
            } else {
                Err(ProtectorError::CorruptOrTampered)
            };
        }
        let record = Zeroizing::new(encode_record(vault_id, key_generation, policy, vrk));
        let entropy = self.entropy();
        let ciphertext = encrypt_data(&record[..], Scope::User, Some(&entropy))
            .map_err(|_| ProtectorError::Unavailable)?;
        self.write_ciphertext(&ciphertext)
    }

    fn unlock_vrk(
        &mut self,
        vault_id: VaultId,
        key_generation: KeyGeneration,
    ) -> Result<Self::VaultRootKey, ProtectorError> {
        let policy = self.policy()?;
        let record = self.decrypt_record()?;
        decode_record(&record, vault_id, key_generation, policy)
    }

    fn read_freshness_anchor(
        &self,
        _vault_id: VaultId,
    ) -> Result<ProtectedFreshnessState, ProtectorError> {
        Err(ProtectorError::UnsupportedPolicy)
    }

    fn install_genesis_freshness_anchor(
        &mut self,
        _vault_id: VaultId,
        _expected_state: ProtectedFreshnessState,
        _new_anchor: FreshnessAnchor,
    ) -> Result<(), ProtectorError> {
        Err(ProtectorError::UnsupportedPolicy)
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
        let policy = self.policy()?;
        let path = self.config.record_path();
        if metadata_no_reparse(&path)?.is_none() {
            self.policy = None;
            return Ok(());
        }
        let record = self.decrypt_record()?;
        validate_record_binding(&record, vault_id, None, policy)?;
        fs::remove_file(&path).map_err(map_io_error)?;
        self.policy = None;
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

fn encode_record(
    vault_id: VaultId,
    key_generation: KeyGeneration,
    policy: ProtectorPolicy,
    vrk: &OwnedKeyMaterial,
) -> [u8; RECORD_BYTES] {
    let mut output = [0_u8; RECORD_BYTES];
    let mut offset = 0;
    output[offset..offset + RECORD_MAGIC.len()].copy_from_slice(RECORD_MAGIC);
    offset += RECORD_MAGIC.len();
    output[offset..offset + OWNER_TAG.len()].copy_from_slice(OWNER_TAG);
    offset += OWNER_TAG.len();
    output[offset..offset + 16].copy_from_slice(vault_id.as_bytes());
    offset += 16;
    output[offset..offset + 8].copy_from_slice(&key_generation.get().to_be_bytes());
    offset += 8;
    output[offset] = encode_scope(policy.access_scope());
    offset += 1;
    output[offset] = encode_presence(policy.user_presence_policy());
    offset += 1;
    vrk.with_bytes(|bytes| output[offset..].copy_from_slice(bytes));
    output
}

fn record_binding(
    record: &[u8],
) -> Result<(VaultId, KeyGeneration, ProtectorPolicy), ProtectorError> {
    if record.len() != RECORD_BYTES || &record[..RECORD_MAGIC.len()] != RECORD_MAGIC {
        return Err(ProtectorError::CorruptOrTampered);
    }
    let mut offset = RECORD_MAGIC.len();
    if &record[offset..offset + OWNER_TAG.len()] != OWNER_TAG {
        return Err(ProtectorError::OwnerMismatch);
    }
    offset += OWNER_TAG.len();
    let stored_vault = VaultId::try_from_slice(&record[offset..offset + 16])
        .map_err(|_| ProtectorError::CorruptOrTampered)?;
    offset += 16;
    let generation_bytes: [u8; 8] = record[offset..offset + 8]
        .try_into()
        .map_err(|_| ProtectorError::CorruptOrTampered)?;
    let stored_generation = KeyGeneration::new(u64::from_be_bytes(generation_bytes))
        .map_err(|_| ProtectorError::CorruptOrTampered)?;
    offset += 8;
    let stored_policy = ProtectorPolicy::new(
        decode_scope(record[offset])?,
        decode_presence(record[offset + 1])?,
    );
    Ok((stored_vault, stored_generation, stored_policy))
}

fn validate_record_binding(
    record: &[u8],
    expected_vault_id: VaultId,
    expected_generation: Option<KeyGeneration>,
    expected_policy: ProtectorPolicy,
) -> Result<(), ProtectorError> {
    let (stored_vault, stored_generation, stored_policy) = record_binding(record)?;
    if stored_vault != expected_vault_id
        || expected_generation.is_some_and(|generation| stored_generation != generation)
    {
        return Err(ProtectorError::OwnerMismatch);
    }
    if stored_policy != expected_policy {
        return Err(ProtectorError::PolicyMismatch);
    }
    Ok(())
}

fn decode_record(
    record: &[u8],
    expected_vault_id: VaultId,
    expected_generation: KeyGeneration,
    expected_policy: ProtectorPolicy,
) -> Result<OwnedKeyMaterial, ProtectorError> {
    validate_record_binding(
        record,
        expected_vault_id,
        Some(expected_generation),
        expected_policy,
    )?;
    let key_offset = RECORD_BYTES - KEY_MATERIAL_BYTES;
    let key: [u8; KEY_MATERIAL_BYTES] = record[key_offset..]
        .try_into()
        .map_err(|_| ProtectorError::CorruptOrTampered)?;
    Ok(OwnedKeyMaterial::from_bytes(key))
}

fn record_matches_vrk(record: &[u8], expected: &OwnedKeyMaterial) -> Result<bool, ProtectorError> {
    if record.len() != RECORD_BYTES {
        return Err(ProtectorError::CorruptOrTampered);
    }
    let key_offset = RECORD_BYTES - KEY_MATERIAL_BYTES;
    Ok(expected.with_bytes(|bytes| record[key_offset..] == bytes[..]))
}

const fn encode_scope(scope: AccessScope) -> u8 {
    match scope {
        AccessScope::AppExclusive => 1,
        AccessScope::SameUserAccount => 2,
        AccessScope::SameUserSession => 3,
    }
}

fn decode_scope(value: u8) -> Result<AccessScope, ProtectorError> {
    match value {
        1 => Ok(AccessScope::AppExclusive),
        2 => Ok(AccessScope::SameUserAccount),
        3 => Ok(AccessScope::SameUserSession),
        _ => Err(ProtectorError::CorruptOrTampered),
    }
}
const fn encode_presence(policy: UserPresencePolicy) -> u8 {
    match policy {
        UserPresencePolicy::NotRequired => 0,
        UserPresencePolicy::RequiredEachHimsatUnlock => 1,
    }
}

fn decode_presence(value: u8) -> Result<UserPresencePolicy, ProtectorError> {
    match value {
        0 => Ok(UserPresencePolicy::NotRequired),
        1 => Ok(UserPresencePolicy::RequiredEachHimsatUnlock),
        _ => Err(ProtectorError::CorruptOrTampered),
    }
}

fn metadata_no_reparse(path: &Path) -> Result<Option<fs::Metadata>, ProtectorError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 => {
            Err(ProtectorError::CorruptOrTampered)
        }
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(map_io_error(error)),
    }
}

fn reject_reparse_point(path: &Path) -> Result<(), ProtectorError> {
    metadata_no_reparse(path)?
        .map(|_| ())
        .ok_or(ProtectorError::ItemMissing)
}

fn map_io_error(error: std::io::Error) -> ProtectorError {
    match error.kind() {
        std::io::ErrorKind::NotFound => ProtectorError::ItemMissing,
        std::io::ErrorKind::PermissionDenied => ProtectorError::Denied,
        std::io::ErrorKind::InvalidData => ProtectorError::CorruptOrTampered,
        _ => ProtectorError::Unavailable,
    }
}
fn current_user_sid() -> Result<(Vec<u8>, String), ProtectorError> {
    let token = w::HPROCESS::GetCurrentProcess()
        .OpenProcessToken(co::TOKEN::QUERY)
        .map_err(|_| ProtectorError::Unavailable)?;
    let info = token
        .GetTokenInformation(co::TOKEN_INFORMATION_CLASS::User)
        .map_err(|_| ProtectorError::Unavailable)?;
    let w::TokenInfo::User(user) = info else {
        return Err(ProtectorError::Unavailable);
    };
    let token_sid = user.User.Sid().ok_or(ProtectorError::Unavailable)?;
    let sid_string =
        w::ConvertSidToStringSid(token_sid).map_err(|_| ProtectorError::Unavailable)?;
    let sid = string_to_sid(&sid_string).map_err(map_acl_error)?;
    Ok((sid, sid_string))
}

fn restrict_acl_to_current_user(path: &Path, inheritable: bool) -> Result<(), ProtectorError> {
    let path = path.to_str().ok_or(ProtectorError::UnsupportedPolicy)?;
    let (current_sid, _) = current_user_sid()?;
    let mut acl = ACL::from_file_path(path, false).map_err(map_acl_error)?;
    let entries = acl.all().map_err(map_acl_error)?;
    for entry in entries {
        let Some(entry_sid) = entry.sid else {
            return Err(ProtectorError::CorruptOrTampered);
        };
        acl.remove(entry_sid.as_ptr() as *mut _, None, None)
            .map_err(map_acl_error)?;
    }
    acl.allow(current_sid.as_ptr() as *mut _, inheritable, FILE_ALL_ACCESS)
        .map_err(map_acl_error)?;
    verify_restricted_acl(Path::new(path), inheritable)
}
fn verify_restricted_acl(path: &Path, inheritable: bool) -> Result<(), ProtectorError> {
    reject_reparse_point(path)?;
    let path_text = path.to_str().ok_or(ProtectorError::UnsupportedPolicy)?;
    let (_, current_sid) = current_user_sid()?;
    let acl = ACL::from_file_path(path_text, false).map_err(map_acl_error)?;
    let entries = acl.all().map_err(map_acl_error)?;
    if entries.len() != 1 {
        return Err(ProtectorError::PolicyMismatch);
    }
    let entry = &entries[0];
    let expected_flags = if inheritable {
        OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE
    } else {
        0
    };
    if entry.entry_type != AceType::AccessAllow
        || entry.string_sid != current_sid
        || entry.mask != FILE_ALL_ACCESS
        || entry.flags & INHERITED_ACE != 0
        || entry.flags != expected_flags
    {
        return Err(ProtectorError::PolicyMismatch);
    }
    Ok(())
}

fn verify_current_user_file_acl(path: &Path) -> Result<(), ProtectorError> {
    verify_restricted_acl(path, false)
}

fn map_acl_error(code: u32) -> ProtectorError {
    match code {
        2 | 3 => ProtectorError::ItemMissing,
        5 => ProtectorError::Denied,
        _ => ProtectorError::Unavailable,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FILE_ALL_ACCESS, OWNER_TAG, RECORD_BYTES, RECORD_MAGIC, WindowsDpapiConfig,
        WindowsDpapiProtector, WindowsProtectorId, encode_record, restrict_acl_to_current_user,
        verify_current_user_file_acl, verify_restricted_acl,
    };
    use crate::vault::{
        AccessScope, HardwareBacking, KeyGeneration, ProtectorError, SecretProtector,
        UserPresencePolicy, VAULT_ID_BYTES, VaultId,
    };
    use crate::vault_keys::{KEY_MATERIAL_BYTES, OwnedKeyMaterial};
    use crate::vault_protector::ProtectorPolicy;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use windows_acl::acl::ACL;
    use windows_acl::helper::string_to_sid;
    use windows_dpapi::{Scope, decrypt_data, encrypt_data};
    struct TestStore {
        root: PathBuf,
        config: WindowsDpapiConfig,
    }
    impl TestStore {
        fn new() -> Self {
            let protector_id = WindowsProtectorId::generate().expect("OS CSPRNG must be available");
            let root_id = WindowsProtectorId::generate().expect("OS CSPRNG must be available");
            let root = std::env::temp_dir().join(format!("himsat-b403b-{}", root_id.file_name()));
            let config = WindowsDpapiConfig::new(root.clone(), protector_id)
                .expect("absolute temp path must be accepted");
            Self { root, config }
        }
    }
    impl Drop for TestStore {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn vault(byte: u8) -> VaultId {
        VaultId::from_bytes([byte; VAULT_ID_BYTES])
    }
    fn generation(value: u64) -> KeyGeneration {
        KeyGeneration::new(value).expect("test generation must be non-zero")
    }
    fn key(byte: u8) -> OwnedKeyMaterial {
        OwnedKeyMaterial::from_bytes([byte; KEY_MATERIAL_BYTES])
    }
    fn assert_key_eq(actual: &OwnedKeyMaterial, expected: u8) {
        actual.with_bytes(|bytes| assert_eq!(bytes, &[expected; KEY_MATERIAL_BYTES]));
    }
    fn access_rules_are_protected(path: &Path) -> bool {
        let output = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "(Get-Acl -LiteralPath $env:HIMSAT_B403_ACL_TARGET).AreAccessRulesProtected",
            ])
            .env("HIMSAT_B403_ACL_TARGET", path)
            .output()
            .expect("PowerShell must be available on qualified Windows targets");
        output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "True"
    }
    #[test]
    fn stronger_policy_is_rejected_before_storage_creation() {
        let store = TestStore::new();
        let mut protector = WindowsDpapiProtector::new(store.config.clone());
        assert_eq!(
            protector.create_protector(AccessScope::AppExclusive, UserPresencePolicy::NotRequired,),
            Err(ProtectorError::UnsupportedPolicy)
        );
        assert_eq!(
            protector.create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::RequiredEachHimsatUnlock,
            ),
            Err(ProtectorError::UnsupportedPolicy)
        );
        assert_eq!(
            protector.create_protector(
                AccessScope::SameUserSession,
                UserPresencePolicy::NotRequired,
            ),
            Err(ProtectorError::UnsupportedPolicy)
        );
        assert!(!store.root.exists());
        protector
            .create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::NotRequired,
            )
            .expect("supported policy");
        assert!(matches!(
            protector.read_freshness_anchor(vault(1)),
            Err(ProtectorError::UnsupportedPolicy)
        ));
        assert_eq!(
            protector.replace_protector(vault(1)),
            Err(ProtectorError::UnsupportedPolicy)
        );
        assert_eq!(
            protector.actual_access_scope(),
            AccessScope::SameUserAccount
        );
        assert!(!protector.requires_user_presence());
        assert_eq!(protector.hardware_backed_state(), HardwareBacking::Unknown);
    }
    #[test]
    fn native_round_trip_restart_acl_and_removal_are_fail_closed() {
        let store = TestStore::new();
        let vault_id = vault(0x31);
        let generation = generation(7);
        let vrk = key(0xA7);
        let record_path = store.config.record_path();
        let mut protector = WindowsDpapiProtector::new(store.config.clone());
        protector
            .create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::NotRequired,
            )
            .expect("supported Windows policy must configure");
        protector
            .protect_or_store_vrk(vault_id, generation, &vrk)
            .expect("current-user DPAPI protection must succeed");
        verify_restricted_acl(&store.root, true).expect("storage root ACL must be restricted");
        verify_current_user_file_acl(&record_path).expect("ciphertext ACL must be restricted");
        assert!(access_rules_are_protected(&store.root));
        assert!(access_rules_are_protected(&record_path));
        let ciphertext = fs::read(&record_path).expect("ciphertext file must be readable");
        assert!(!ciphertext.is_empty());
        assert_ne!(ciphertext.len(), RECORD_BYTES);
        assert!(
            !ciphertext
                .windows(KEY_MATERIAL_BYTES)
                .any(|window| window == [0xA7; KEY_MATERIAL_BYTES])
        );
        let unlocked = protector
            .unlock_vrk(vault_id, generation)
            .expect("same process unlock must succeed");
        assert_key_eq(&unlocked, 0xA7);
        drop(protector);
        let mut restarted = WindowsDpapiProtector::new(store.config.clone());
        assert!(matches!(
            restarted.unlock_vrk(vault_id, generation),
            Err(ProtectorError::UnsupportedPolicy)
        ));
        restarted
            .create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::NotRequired,
            )
            .expect("restart must reconfigure the policy explicitly");
        let unlocked = restarted
            .unlock_vrk(vault_id, generation)
            .expect("current-user DPAPI must survive process-adapter restart");
        assert_key_eq(&unlocked, 0xA7);
        restarted
            .remove_protector(vault_id)
            .expect("bound protector removal must succeed");
        assert!(!record_path.exists());
        restarted
            .create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::NotRequired,
            )
            .expect("policy may be configured again after removal");
        assert!(matches!(
            restarted.unlock_vrk(vault_id, generation),
            Err(ProtectorError::ItemMissing)
        ));
        restarted
            .remove_protector(vault_id)
            .expect("removal of an already-missing protector must be idempotent");
    }
    #[test]
    fn wrong_vault_generation_and_same_generation_replacement_never_return_or_overwrite_vrk() {
        let store = TestStore::new();
        let mut protector = WindowsDpapiProtector::new(store.config.clone());
        protector
            .create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::NotRequired,
            )
            .expect("supported policy");
        let vault_id = vault(0x42);
        let key_generation = generation(11);
        let original = key(0x11);
        protector
            .protect_or_store_vrk(vault_id, key_generation, &original)
            .expect("initial protection");
        assert!(matches!(
            protector.unlock_vrk(vault(0x43), key_generation),
            Err(ProtectorError::OwnerMismatch)
        ));
        assert!(matches!(
            protector.unlock_vrk(vault_id, generation(12)),
            Err(ProtectorError::OwnerMismatch)
        ));
        assert_eq!(
            protector.protect_or_store_vrk(vault_id, key_generation, &key(0x22)),
            Err(ProtectorError::CorruptOrTampered)
        );
        let unlocked = protector
            .unlock_vrk(vault_id, key_generation)
            .expect("rejected overwrite must preserve original VRK");
        assert_key_eq(&unlocked, 0x11);
        let everyone = string_to_sid("S-1-1-0").expect("well-known Everyone SID");
        let mut acl = ACL::from_file_path(store.root.to_str().expect("Unicode temp path"), false)
            .expect("root ACL");
        acl.allow(everyone.as_ptr() as *mut _, false, FILE_ALL_ACCESS)
            .expect("test ACL tamper");
        assert!(matches!(
            protector.unlock_vrk(vault_id, key_generation),
            Err(ProtectorError::PolicyMismatch)
        ));
    }
    #[test]
    fn protector_entropy_transplant_is_rejected() {
        let source = TestStore::new();
        let target = TestStore::new();
        let vault_id = vault(0x67);
        let key_generation = generation(23);
        let mut source_protector = WindowsDpapiProtector::new(source.config.clone());
        let mut target_protector = WindowsDpapiProtector::new(target.config.clone());
        for protector in [&mut source_protector, &mut target_protector] {
            protector
                .create_protector(
                    AccessScope::SameUserAccount,
                    UserPresencePolicy::NotRequired,
                )
                .expect("supported policy");
        }
        source_protector
            .protect_or_store_vrk(vault_id, key_generation, &key(0x67))
            .expect("source protection");
        fs::copy(source.config.record_path(), target.config.record_path())
            .expect("test transplant");
        restrict_acl_to_current_user(&target.config.record_path(), false).expect("target file ACL");
        assert!(matches!(
            target_protector.unlock_vrk(vault_id, key_generation),
            Err(ProtectorError::CorruptOrTampered)
        ));
    }
    #[test]
    fn owner_policy_and_ciphertext_tamper_fail_closed() {
        let store = TestStore::new();
        let vault_id = vault(0x55);
        let generation = generation(19);
        let policy = ProtectorPolicy::new(
            AccessScope::SameUserAccount,
            UserPresencePolicy::NotRequired,
        );
        let mut protector = WindowsDpapiProtector::new(store.config.clone());
        protector
            .create_protector(
                AccessScope::SameUserAccount,
                UserPresencePolicy::NotRequired,
            )
            .expect("supported policy");
        protector
            .protect_or_store_vrk(vault_id, generation, &key(0x5A))
            .expect("initial protection");
        let record_path = store.config.record_path();
        let entropy = protector.entropy();
        let original_ciphertext = fs::read(&record_path).expect("ciphertext");
        let original_record = decrypt_data(&original_ciphertext, Scope::User, Some(&entropy))
            .expect("test may inspect its own DPAPI record");
        let mut owner_mismatch = original_record.clone();
        owner_mismatch[RECORD_MAGIC.len()] ^= 0x01;
        let ciphertext = encrypt_data(&owner_mismatch, Scope::User, Some(&entropy))
            .expect("test re-protection must succeed");
        fs::write(&record_path, ciphertext).expect("test rewrite");
        assert!(matches!(
            protector.unlock_vrk(vault_id, generation),
            Err(ProtectorError::OwnerMismatch)
        ));
        let mut policy_mismatch = encode_record(vault_id, generation, policy, &key(0x5A));
        let policy_offset = RECORD_MAGIC.len() + OWNER_TAG.len() + 16 + 8;
        policy_mismatch[policy_offset] = 1;
        let ciphertext = encrypt_data(&policy_mismatch, Scope::User, Some(&entropy))
            .expect("test re-protection must succeed");
        fs::write(&record_path, ciphertext).expect("test rewrite");
        assert!(matches!(
            protector.unlock_vrk(vault_id, generation),
            Err(ProtectorError::PolicyMismatch)
        ));
        fs::write(&record_path, &original_ciphertext).expect("restore original ciphertext");
        let mut tampered = original_ciphertext;
        let last = tampered.len() - 1;
        tampered[last] ^= 0x80;
        fs::write(&record_path, tampered).expect("write tampered ciphertext");
        assert!(matches!(
            protector.unlock_vrk(vault_id, generation),
            Err(ProtectorError::CorruptOrTampered)
        ));
    }
    #[test]
    fn opaque_filename_and_record_encoding_do_not_expose_binding_metadata_in_path() {
        let protector_id = WindowsProtectorId::from_bytes([0xA5; 16]);
        let file_name = protector_id.file_name();
        let vault_id = vault(0x11);
        let generation = generation(0x0102_0304_0506_0708);
        let policy = ProtectorPolicy::new(
            AccessScope::SameUserAccount,
            UserPresencePolicy::NotRequired,
        );
        let record = encode_record(vault_id, generation, policy, &key(0x77));
        assert_eq!(file_name, "a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5.dpapi");
        assert!(!file_name.contains("11111111"));
        assert!(!file_name.contains("0102030405060708"));
        assert_eq!(&record[..RECORD_MAGIC.len()], RECORD_MAGIC);
        assert_eq!(
            &record[RECORD_MAGIC.len()..RECORD_MAGIC.len() + OWNER_TAG.len()],
            OWNER_TAG
        );
    }
}
