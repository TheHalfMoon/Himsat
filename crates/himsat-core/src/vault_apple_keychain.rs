//! Apple data-protection Keychain adapter for Specification 004B4 B401.
//!
//! The adapter stores a VRK only inside a non-synchronizable, ThisDeviceOnly
//! generic-password item scoped to the configured Himsat access group. Provider-
//! visible lookup metadata is limited to fixed service/access-group identifiers
//! plus one opaque random protector identifier. Vault identity, key generation,
//! policy binding, and VRK bytes remain inside the protected item value.
//!
//! Freshness anchors, full protector replacement/rotation, and non-Apple
//! platform adapters remain separately authorized leaves.

use crate::vault::{
    AccessScope, FreshnessAnchor, HardwareBacking, KeyGeneration, ProtectedFreshnessState,
    ProtectorError, SecretProtector, UserPresencePolicy, VaultId,
};
use crate::vault_keys::{KEY_MATERIAL_BYTES, OwnedKeyMaterial};
use crate::vault_protector::ProtectorPolicy;
use getrandom::fill;
use security_framework::access_control::{ProtectionMode, SecAccessControl};
use security_framework::base::Error as SecurityFrameworkError;
use security_framework::passwords::{
    AccessControlOptions, PasswordOptions, delete_generic_password_options, generic_password,
    set_generic_password_options,
};

const RECORD_MAGIC: &[u8] = b"HIMSAT/APPLE/VRK/v1\0";
const PROTECTOR_ID_BYTES: usize = 16;
const RECORD_BYTES: usize = RECORD_MAGIC.len() + 16 + 8 + 1 + 1 + KEY_MATERIAL_BYTES;

const ERR_SEC_USER_CANCELED: i32 = -128;
const ERR_SEC_NOT_AVAILABLE: i32 = -25291;
const ERR_SEC_AUTH_FAILED: i32 = -25293;
const ERR_SEC_ITEM_NOT_FOUND: i32 = -25300;
const ERR_SEC_INTERACTION_NOT_ALLOWED: i32 = -25308;
const ERR_SEC_DECODE: i32 = -26275;
const ERR_SEC_MISSING_ENTITLEMENT: i32 = -34018;

/// Opaque provider-visible identifier used as the Keychain account attribute.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct AppleProtectorId([u8; PROTECTOR_ID_BYTES]);

impl AppleProtectorId {
    /// Generates a new opaque protector identifier from the reviewed OS CSPRNG.
    pub fn generate() -> Result<Self, ProtectorError> {
        let mut bytes = [0_u8; PROTECTOR_ID_BYTES];
        fill(&mut bytes).map_err(|_| ProtectorError::Unavailable)?;
        Ok(Self(bytes))
    }

    /// Reconstructs a previously persisted opaque protector identifier.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; PROTECTOR_ID_BYTES]) -> Self {
        Self(bytes)
    }

    /// Returns the opaque bytes for Himsat-owned metadata persistence.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; PROTECTOR_ID_BYTES] {
        &self.0
    }

    /// Returns the lowercase hexadecimal Keychain account value.
    #[must_use]
    pub fn account(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(PROTECTOR_ID_BYTES * 2);
        for byte in self.0 {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }
        output
    }
}

impl core::fmt::Debug for AppleProtectorId {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("AppleProtectorId([OPAQUE; 16 bytes])")
    }
}

/// Apple Keychain protector configuration supplied by the signed Himsat target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppleKeychainConfig {
    service: String,
    access_group: String,
    protector_id: AppleProtectorId,
}

impl AppleKeychainConfig {
    /// Creates configuration for one signed Himsat application/access group.
    pub fn new(
        service: impl Into<String>,
        access_group: impl Into<String>,
        protector_id: AppleProtectorId,
    ) -> Result<Self, ProtectorError> {
        let service = service.into();
        let access_group = access_group.into();
        if service.is_empty() || access_group.is_empty() {
            return Err(ProtectorError::UnsupportedPolicy);
        }
        Ok(Self {
            service,
            access_group,
            protector_id,
        })
    }

    /// Fixed service identifier visible to Keychain.
    #[must_use]
    pub fn service(&self) -> &str {
        &self.service
    }

    /// Signed target access group visible to Keychain.
    #[must_use]
    pub fn access_group(&self) -> &str {
        &self.access_group
    }

    /// Opaque provider-visible protector identifier.
    #[must_use]
    pub const fn protector_id(&self) -> AppleProtectorId {
        self.protector_id
    }
}

/// Apple Keychain implementation of the portable `SecretProtector` contract.
pub struct AppleKeychainProtector {
    config: AppleKeychainConfig,
    policy: Option<ProtectorPolicy>,
    app_scope_verified: bool,
}

impl AppleKeychainProtector {
    /// Creates an unconfigured adapter. `create_protector` must prove the signed
    /// target's access-group entitlement before `APP_EXCLUSIVE` is reported.
    #[must_use]
    pub const fn new(config: AppleKeychainConfig) -> Self {
        Self {
            config,
            policy: None,
            app_scope_verified: false,
        }
    }

    /// Returns the immutable native configuration used by this adapter.
    #[must_use]
    pub const fn config(&self) -> &AppleKeychainConfig {
        &self.config
    }

    fn policy(&self) -> Result<ProtectorPolicy, ProtectorError> {
        self.policy.ok_or(ProtectorError::UnsupportedPolicy)
    }

    fn read_options(&self) -> PasswordOptions {
        let mut options = PasswordOptions::new_generic_password(
            self.config.service(),
            &self.config.protector_id().account(),
        );
        options.set_access_group(self.config.access_group());
        options.set_access_synchronized(Some(false));
        options.use_protected_keychain();
        options
    }

    fn write_options(&self, policy: ProtectorPolicy) -> Result<PasswordOptions, ProtectorError> {
        let flags = match policy.user_presence_policy() {
            UserPresencePolicy::NotRequired => AccessControlOptions::empty(),
            UserPresencePolicy::RequiredEachHimsatUnlock => AccessControlOptions::USER_PRESENCE,
        };
        let access_control = SecAccessControl::create_with_protection(
            Some(ProtectionMode::AccessibleWhenPasscodeSetThisDeviceOnly),
            flags.bits(),
        )
        .map_err(map_security_error)?;
        let mut options = self.read_options();
        options.set_access_control(access_control);
        Ok(options)
    }

    fn verify_access_group_entitlement(&self) -> Result<(), ProtectorError> {
        let mut probe = PasswordOptions::new_generic_password(
            self.config.service(),
            &format!("{}.entitlement-probe", self.config.protector_id().account()),
        );
        probe.set_access_group(self.config.access_group());
        probe.set_access_synchronized(Some(false));
        probe.use_protected_keychain();
        match generic_password(probe) {
            Ok(_) => Ok(()),
            Err(error) if error.code() == ERR_SEC_ITEM_NOT_FOUND => Ok(()),
            Err(error) => Err(map_security_error(error)),
        }
    }
}

impl SecretProtector for AppleKeychainProtector {
    type VaultRootKey = OwnedKeyMaterial;

    fn create_protector(
        &mut self,
        requested_scope: AccessScope,
        user_presence_policy: UserPresencePolicy,
    ) -> Result<(), ProtectorError> {
        self.verify_access_group_entitlement()?;
        let policy = ProtectorPolicy::new(requested_scope, user_presence_policy);
        self.write_options(policy)?;
        self.policy = Some(policy);
        self.app_scope_verified = true;
        Ok(())
    }

    fn protect_or_store_vrk(
        &mut self,
        vault_id: VaultId,
        key_generation: KeyGeneration,
        vrk: &Self::VaultRootKey,
    ) -> Result<(), ProtectorError> {
        let policy = self.policy()?;
        if !self.app_scope_verified {
            return Err(ProtectorError::UnsupportedPolicy);
        }
        let record = encode_record(vault_id, key_generation, policy, vrk);
        set_generic_password_options(&record, self.write_options(policy)?)
            .map_err(map_security_error)
    }

    fn unlock_vrk(
        &mut self,
        vault_id: VaultId,
        key_generation: KeyGeneration,
    ) -> Result<Self::VaultRootKey, ProtectorError> {
        let policy = self.policy()?;
        let record = generic_password(self.read_options()).map_err(map_security_error)?;
        let vrk = decode_record(&record, vault_id, key_generation, policy)?;
        self.app_scope_verified = true;
        Ok(vrk)
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

    fn remove_protector(&mut self, _vault_id: VaultId) -> Result<(), ProtectorError> {
        delete_generic_password_options(self.read_options()).map_err(map_security_error)
    }

    fn actual_access_scope(&self) -> AccessScope {
        if self.app_scope_verified {
            AccessScope::AppExclusive
        } else {
            AccessScope::SameUserAccount
        }
    }

    fn requires_user_presence(&self) -> bool {
        matches!(
            self.policy.map(ProtectorPolicy::user_presence_policy),
            Some(UserPresencePolicy::RequiredEachHimsatUnlock)
        )
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

fn decode_record(
    record: &[u8],
    expected_vault_id: VaultId,
    expected_generation: KeyGeneration,
    expected_policy: ProtectorPolicy,
) -> Result<OwnedKeyMaterial, ProtectorError> {
    if record.len() != RECORD_BYTES || &record[..RECORD_MAGIC.len()] != RECORD_MAGIC {
        return Err(ProtectorError::CorruptOrTampered);
    }
    let mut offset = RECORD_MAGIC.len();
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
    offset += 2;
    if stored_vault != expected_vault_id || stored_generation != expected_generation {
        return Err(ProtectorError::OwnerMismatch);
    }
    if stored_policy != expected_policy {
        return Err(ProtectorError::PolicyMismatch);
    }
    let key: [u8; KEY_MATERIAL_BYTES] = record[offset..]
        .try_into()
        .map_err(|_| ProtectorError::CorruptOrTampered)?;
    Ok(OwnedKeyMaterial::from_bytes(key))
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

fn map_security_error(error: SecurityFrameworkError) -> ProtectorError {
    match error.code() {
        ERR_SEC_ITEM_NOT_FOUND => ProtectorError::ItemMissing,
        ERR_SEC_AUTH_FAILED | ERR_SEC_USER_CANCELED => ProtectorError::Denied,
        ERR_SEC_INTERACTION_NOT_ALLOWED => ProtectorError::Locked,
        ERR_SEC_MISSING_ENTITLEMENT => ProtectorError::OwnerMismatch,
        ERR_SEC_DECODE => ProtectorError::Invalidated,
        ERR_SEC_NOT_AVAILABLE => ProtectorError::Unavailable,
        _ => ProtectorError::UnsupportedPolicy,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AppleKeychainConfig, AppleProtectorId, decode_record, encode_record, map_security_error,
    };
    use crate::vault::{
        AccessScope, KeyGeneration, ProtectorError, UserPresencePolicy, VAULT_ID_BYTES, VaultId,
    };
    use crate::vault_keys::OwnedKeyMaterial;
    use crate::vault_protector::ProtectorPolicy;
    use security_framework::base::Error as SecurityFrameworkError;

    fn vault(byte: u8) -> VaultId {
        VaultId::from_bytes([byte; VAULT_ID_BYTES])
    }

    fn generation(value: u64) -> KeyGeneration {
        KeyGeneration::new(value).expect("non-zero generation")
    }

    #[test]
    fn opaque_identifier_never_embeds_vault_or_generation() {
        let id = AppleProtectorId::from_bytes([0xa5; 16]);
        assert_eq!(id.account(), "a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5");
        assert_eq!(format!("{id:?}"), "AppleProtectorId([OPAQUE; 16 bytes])");
    }

    #[test]
    fn configuration_rejects_empty_application_metadata() {
        let id = AppleProtectorId::from_bytes([1; 16]);
        assert_eq!(
            AppleKeychainConfig::new("", "group", id),
            Err(ProtectorError::UnsupportedPolicy)
        );
        assert_eq!(
            AppleKeychainConfig::new("service", "", id),
            Err(ProtectorError::UnsupportedPolicy)
        );
    }

    #[test]
    fn protected_record_binds_vault_generation_and_policy() {
        let vrk = OwnedKeyMaterial::from_bytes([0x5a; 32]);
        let policy = ProtectorPolicy::new(
            AccessScope::AppExclusive,
            UserPresencePolicy::RequiredEachHimsatUnlock,
        );
        let encoded = encode_record(vault(0x11), generation(7), policy, &vrk);
        assert!(decode_record(&encoded, vault(0x11), generation(7), policy).is_ok());
        assert!(matches!(
            decode_record(&encoded, vault(0x12), generation(7), policy),
            Err(ProtectorError::OwnerMismatch)
        ));
        assert!(matches!(
            decode_record(&encoded, vault(0x11), generation(8), policy),
            Err(ProtectorError::OwnerMismatch)
        ));
        let weaker =
            ProtectorPolicy::new(AccessScope::AppExclusive, UserPresencePolicy::NotRequired);
        assert!(matches!(
            decode_record(&encoded, vault(0x11), generation(7), weaker),
            Err(ProtectorError::PolicyMismatch)
        ));
    }

    #[test]
    fn protected_record_rejects_corruption() {
        let vrk = OwnedKeyMaterial::from_bytes([0x7c; 32]);
        let policy =
            ProtectorPolicy::new(AccessScope::AppExclusive, UserPresencePolicy::NotRequired);
        let mut encoded = encode_record(vault(0x21), generation(3), policy, &vrk);
        encoded[0] ^= 1;
        assert!(matches!(
            decode_record(&encoded, vault(0x21), generation(3), policy),
            Err(ProtectorError::CorruptOrTampered)
        ));
    }

    #[test]
    fn native_error_mapping_is_fail_closed() {
        assert_eq!(
            map_security_error(SecurityFrameworkError::from_code(-25300)),
            ProtectorError::ItemMissing
        );
        assert_eq!(
            map_security_error(SecurityFrameworkError::from_code(-34018)),
            ProtectorError::OwnerMismatch
        );
        assert_eq!(
            map_security_error(SecurityFrameworkError::from_code(-25308)),
            ProtectorError::Locked
        );
        assert_eq!(
            map_security_error(SecurityFrameworkError::from_code(-128)),
            ProtectorError::Denied
        );
    }
}
