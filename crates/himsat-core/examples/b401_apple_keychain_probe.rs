use himsat_core::vault::{
    AccessScope, HardwareBacking, KeyGeneration, ProtectorError, SecretProtector,
    UserPresencePolicy, VaultId,
};
#[cfg(target_os = "macos")]
use himsat_core::vault_apple_keychain::{
    AppleKeychainConfig, AppleKeychainProtector, AppleProtectorId,
};
#[cfg(target_os = "macos")]
use himsat_core::vault_keys::OwnedKeyMaterial;
#[cfg(target_os = "macos")]
use security_framework::item::{ItemClass, ItemSearchOptions, SearchResult};

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
        return Err("expected one data-protection Keychain attribute dictionary".into());
    };
    results[0]
        .simplify_dict()
        .ok_or_else(|| "failed to simplify Keychain attributes".into())
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = "com.thehalfmoon.himsat.b401.qualifier";
    let vault_id = VaultId::from_bytes([0x41; 16]);
    let wrong_vault = VaultId::from_bytes([0x42; 16]);
    let generation = KeyGeneration::new(1)?;
    let id = AppleProtectorId::generate()?;
    let config = AppleKeychainConfig::new(service, id)?;
    let mut protector = AppleKeychainProtector::new(config);

    assert_eq!(
        protector.create_protector(AccessScope::AppExclusive, UserPresencePolicy::NotRequired),
        Err(ProtectorError::UnsupportedPolicy)
    );
    assert_eq!(
        protector.create_protector(
            AccessScope::SameUserAccount,
            UserPresencePolicy::RequiredEachHimsatUnlock,
        ),
        Err(ProtectorError::UnsupportedPolicy)
    );
    protector.create_protector(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    )?;
    assert_eq!(
        protector.actual_access_scope(),
        AccessScope::SameUserAccount
    );
    assert!(!protector.requires_user_presence());
    assert_eq!(protector.hardware_backed_state(), HardwareBacking::Unknown);

    protector.protect_or_store_vrk(
        vault_id,
        generation,
        &OwnedKeyMaterial::from_bytes([0x5a; 32]),
    )?;
    let _unlocked = protector.unlock_vrk(vault_id, generation)?;
    let attrs = attributes(service, &id.account())?;
    assert_eq!(attrs.get("svce").map(String::as_str), Some(service));
    assert_eq!(
        attrs.get("acct").map(String::as_str),
        Some(id.account().as_str())
    );
    assert_eq!(attrs.get("pdmn").map(String::as_str), Some("akpu"));
    assert!(!synchronized_item_exists(service, &id.account())?);

    assert_eq!(
        protector.remove_protector(wrong_vault),
        Err(ProtectorError::OwnerMismatch)
    );
    let _still_unlocked = protector.unlock_vrk(vault_id, generation)?;
    protector.remove_protector(vault_id)?;
    assert!(matches!(
        protector.unlock_vrk(vault_id, generation),
        Err(ProtectorError::UnsupportedPolicy)
    ));

    println!("B401_NATIVE_SCOPE=SAME_USER_ACCOUNT");
    println!("B401_NATIVE_PRESENCE=NOT_REQUIRED");
    println!("B401_NATIVE_ACCESSIBILITY=WHEN_PASSCODE_SET_THIS_DEVICE_ONLY");
    println!("B401_NATIVE_SYNCHRONIZABLE=FALSE");
    println!("B401_NATIVE_STRONGER_POLICY=UNSUPPORTED_POLICY");
    println!("B401_NATIVE_KEYCHAIN_QUALIFICATION=PASS");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("B401_NATIVE_KEYCHAIN_QUALIFICATION=UNAVAILABLE_NON_MACOS");
}
