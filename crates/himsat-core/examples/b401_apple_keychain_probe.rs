#[cfg(target_os = "macos")]
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
    access_group: &str,
) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut search = ItemSearchOptions::new();
    search
        .class(ItemClass::generic_password())
        .service(service)
        .account(account)
        .access_group(access_group)
        .cloud_sync(Some(false))
        .load_attributes(true)
        .limit(1)
        .ignore_legacy_keychains();
    let results = search.search()?;
    let Some(SearchResult::Dict(_)) = results.first() else {
        return Err("expected one Keychain attribute dictionary".into());
    };
    results[0]
        .simplify_dict()
        .ok_or_else(|| "failed to simplify Keychain attributes".into())
}

#[cfg(target_os = "macos")]
fn data_without_authentication(
    service: &str,
    account: &str,
    access_group: &str,
) -> Result<bool, security_framework::base::Error> {
    let mut search = ItemSearchOptions::new();
    search
        .class(ItemClass::generic_password())
        .service(service)
        .account(account)
        .access_group(access_group)
        .cloud_sync(Some(false))
        .load_data(true)
        .limit(1)
        .skip_authenticated_items(true)
        .ignore_legacy_keychains();
    search.search().map(|results| {
        results
            .iter()
            .any(|result| matches!(result, SearchResult::Data(_)))
    })
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_group = std::env::var("HIMSAT_APPLE_KEYCHAIN_ACCESS_GROUP")?;
    let service = "com.thehalfmoon.himsat.b401.qualifier";
    let vault_id = VaultId::from_bytes([0x41; 16]);
    let generation = KeyGeneration::new(1)?;

    let id = AppleProtectorId::generate()?;
    let config = AppleKeychainConfig::new(service, &access_group, id)?;
    let mut protector = AppleKeychainProtector::new(config);
    protector.create_protector(AccessScope::AppExclusive, UserPresencePolicy::NotRequired)?;
    assert_eq!(protector.actual_access_scope(), AccessScope::AppExclusive);
    assert_eq!(protector.hardware_backed_state(), HardwareBacking::Unknown);
    assert!(!protector.requires_user_presence());
    protector.protect_or_store_vrk(
        vault_id,
        generation,
        &OwnedKeyMaterial::from_bytes([0x5a; 32]),
    )?;
    let _unlocked = protector.unlock_vrk(vault_id, generation)?;

    let attrs = attributes(service, &id.account(), &access_group)?;
    assert_eq!(attrs.get("svce").map(String::as_str), Some(service));
    assert_eq!(
        attrs.get("acct").map(String::as_str),
        Some(id.account().as_str())
    );
    assert_eq!(
        attrs.get("agrp").map(String::as_str),
        Some(access_group.as_str())
    );
    assert_eq!(attrs.get("pdmn").map(String::as_str), Some("akpu"));
    println!("B401_NATIVE_NORMAL_ATTRIBUTES={attrs:?}");

    let mismatched = format!("{access_group}.mismatch");
    let mismatch_config =
        AppleKeychainConfig::new(service, mismatched, AppleProtectorId::generate()?)?;
    let mut mismatch = AppleKeychainProtector::new(mismatch_config);
    assert_eq!(
        mismatch.create_protector(AccessScope::AppExclusive, UserPresencePolicy::NotRequired),
        Err(ProtectorError::OwnerMismatch)
    );

    let presence_id = AppleProtectorId::generate()?;
    let presence_config = AppleKeychainConfig::new(service, &access_group, presence_id)?;
    let mut presence = AppleKeychainProtector::new(presence_config);
    presence.create_protector(
        AccessScope::AppExclusive,
        UserPresencePolicy::RequiredEachHimsatUnlock,
    )?;
    assert!(presence.requires_user_presence());
    presence.protect_or_store_vrk(
        vault_id,
        generation,
        &OwnedKeyMaterial::from_bytes([0x7b; 32]),
    )?;
    match data_without_authentication(service, &presence_id.account(), &access_group) {
        Ok(returned_data) => assert!(!returned_data),
        Err(error) if error.code() == -25300 => {}
        Err(error) => return Err(Box::new(error)),
    }
    println!("B401_NATIVE_PRESENCE_NO_AUTH_DATA=DENIED");

    presence.remove_protector(vault_id)?;
    protector.remove_protector(vault_id)?;
    println!("B401_NATIVE_KEYCHAIN_QUALIFICATION=PASS");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("B401_NATIVE_KEYCHAIN_QUALIFICATION=UNAVAILABLE_NON_MACOS");
}
