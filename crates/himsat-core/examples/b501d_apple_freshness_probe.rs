#[cfg(target_os = "macos")]
use himsat_core::vault::{
    AccessScope, FreshnessAnchor, FreshnessEpoch, HardwareBacking, KeyGeneration, ManifestHash,
    ProtectedFreshnessState, ProtectorError, SecretProtector, UserPresencePolicy, VaultId,
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
fn epoch(value: u64) -> FreshnessEpoch {
    FreshnessEpoch::new(value).expect("non-zero freshness epoch")
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
fn fixed_probe() -> Result<AppleKeychainProtector, ProtectorError> {
    let config = AppleKeychainConfig::new(
        "com.thehalfmoon.himsat.b501d.crash",
        AppleProtectorId::from_bytes([0x5e; 16]),
    )?;
    let mut protector = AppleKeychainProtector::new(config);
    protector.create_protector(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    )?;
    Ok(protector)
}

#[cfg(target_os = "macos")]
fn fixed_vault() -> VaultId {
    VaultId::from_bytes([0x5f; 16])
}

#[cfg(target_os = "macos")]
fn fixed_anchor(epoch_value: u64, hash_byte: u8) -> FreshnessAnchor {
    FreshnessAnchor::new(
        fixed_vault(),
        epoch(epoch_value),
        ManifestHash::from_bytes([hash_byte; 32]),
    )
}

#[cfg(target_os = "macos")]
fn cleanup_fixed() -> Result<(), ProtectorError> {
    let mut protector = fixed_probe()?;
    match protector.remove_protector(fixed_vault()) {
        Ok(()) | Err(ProtectorError::ItemMissing) => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(target_os = "macos")]
fn run_control_mode(mode: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match mode {
        "crash-setup" => {
            cleanup_fixed()?;
            let mut protector = fixed_probe()?;
            protector.protect_or_store_vrk(
                fixed_vault(),
                KeyGeneration::new(1)?,
                &OwnedKeyMaterial::from_bytes([0x6a; 32]),
            )?;
            assert_eq!(
                protector.read_freshness_anchor(fixed_vault())?,
                ProtectedFreshnessState::Uninitialized
            );
            println!("B501D_CRASH_STATE=UNINITIALIZED");
        }
        "crash-genesis" => {
            fixed_probe()?.install_genesis_freshness_anchor(
                fixed_vault(),
                ProtectedFreshnessState::Uninitialized,
                fixed_anchor(1, 0x71),
            )?;
            println!("B501D_CRASH_GENESIS=SUCCESS");
        }
        "race-setup" => {
            cleanup_fixed()?;
            let mut protector = fixed_probe()?;
            protector.protect_or_store_vrk(
                fixed_vault(),
                KeyGeneration::new(1)?,
                &OwnedKeyMaterial::from_bytes([0x6a; 32]),
            )?;
            protector.install_genesis_freshness_anchor(
                fixed_vault(),
                ProtectedFreshnessState::Uninitialized,
                fixed_anchor(1, 0x71),
            )?;
            println!("B501D_RACE_STATE=PRESENT_1");
        }
        "race-advance-a" | "race-advance-b" => {
            let hash = if mode.ends_with('a') { 0x72 } else { 0x73 };
            fixed_probe()?.advance_freshness_anchor(
                fixed_vault(),
                fixed_anchor(1, 0x71),
                fixed_anchor(2, hash),
            )?;
            println!("B501D_RACE_WINNER_HASH={hash:02x}");
        }
        "state" => {
            let state = fixed_probe()?.read_freshness_anchor(fixed_vault())?;
            match state {
                ProtectedFreshnessState::Uninitialized => println!("B501D_STATE=UNINITIALIZED"),
                ProtectedFreshnessState::Present(anchor) => println!(
                    "B501D_STATE=PRESENT:{}:{:02x}",
                    anchor.highest_epoch().get(),
                    anchor.manifest_hash().as_bytes()[0]
                ),
            }
        }
        "cleanup" => {
            cleanup_fixed()?;
            println!("B501D_CLEANUP=PASS");
        }
        _ => return Ok(false),
    }
    Ok(true)
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(mode) = std::env::args().nth(1)
        && run_control_mode(&mode)?
    {
        return Ok(());
    }

    let service = "com.thehalfmoon.himsat.b501d.qualifier";
    let vault_id = VaultId::from_bytes([0x51; 16]);
    let wrong_vault = VaultId::from_bytes([0x52; 16]);
    let generation = KeyGeneration::new(1)?;
    let id = AppleProtectorId::generate()?;
    let config = AppleKeychainConfig::new(service, id)?;
    let mut protector = AppleKeychainProtector::new(config);

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
    assert_eq!(
        protector.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Uninitialized
    );
    assert_eq!(
        protector.read_freshness_anchor(wrong_vault),
        Err(ProtectorError::OwnerMismatch)
    );

    let anchor1 = FreshnessAnchor::new(vault_id, epoch(1), ManifestHash::from_bytes([0x61; 32]));
    protector.install_genesis_freshness_anchor(
        vault_id,
        ProtectedFreshnessState::Uninitialized,
        anchor1,
    )?;
    assert_eq!(
        protector.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Present(anchor1)
    );
    assert_eq!(
        protector.install_genesis_freshness_anchor(
            vault_id,
            ProtectedFreshnessState::Uninitialized,
            anchor1,
        ),
        Err(ProtectorError::AnchorAlreadyInitialized)
    );

    let wrong_old = FreshnessAnchor::new(vault_id, epoch(1), ManifestHash::from_bytes([0x62; 32]));
    let anchor2 = FreshnessAnchor::new(vault_id, epoch(2), ManifestHash::from_bytes([0x63; 32]));
    assert_eq!(
        protector.advance_freshness_anchor(vault_id, wrong_old, anchor2),
        Err(ProtectorError::AnchorConflict)
    );
    assert_eq!(
        protector.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Present(anchor1)
    );

    let gap = FreshnessAnchor::new(vault_id, epoch(3), ManifestHash::from_bytes([0x64; 32]));
    assert_eq!(
        protector.advance_freshness_anchor(vault_id, anchor1, gap),
        Err(ProtectorError::AnchorConflict)
    );
    protector.advance_freshness_anchor(vault_id, anchor1, anchor2)?;
    assert_eq!(
        protector.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Present(anchor2)
    );

    // Re-storing the same bound VRK must preserve the established anchor.
    protector.protect_or_store_vrk(
        vault_id,
        generation,
        &OwnedKeyMaterial::from_bytes([0x5a; 32]),
    )?;
    assert_eq!(
        protector.read_freshness_anchor(vault_id)?,
        ProtectedFreshnessState::Present(anchor2)
    );

    let attrs = attributes(service, &id.account())?;
    assert_eq!(attrs.get("svce").map(String::as_str), Some(service));
    assert_eq!(
        attrs.get("acct").map(String::as_str),
        Some(id.account().as_str())
    );
    assert_eq!(attrs.get("pdmn").map(String::as_str), Some("akpu"));
    assert!(!synchronized_item_exists(service, &id.account())?);

    protector.remove_protector(vault_id)?;
    protector.create_protector(
        AccessScope::SameUserAccount,
        UserPresencePolicy::NotRequired,
    )?;
    assert_eq!(
        protector.read_freshness_anchor(vault_id),
        Err(ProtectorError::ItemMissing)
    );
    assert_eq!(
        protector.install_genesis_freshness_anchor(
            vault_id,
            ProtectedFreshnessState::Uninitialized,
            anchor1,
        ),
        Err(ProtectorError::ItemMissing)
    );

    println!("B501D_NATIVE_INITIAL_STATE=UNINITIALIZED");
    println!("B501D_NATIVE_GENESIS_INSTALL=PASS");
    println!("B501D_NATIVE_SECOND_GENESIS=ANCHOR_ALREADY_INITIALIZED");
    println!("B501D_NATIVE_WRONG_OLD=ANCHOR_CONFLICT");
    println!("B501D_NATIVE_GAP=ANCHOR_CONFLICT");
    println!("B501D_NATIVE_ADVANCE=PASS");
    println!("B501D_NATIVE_REREAD=PASS");
    println!("B501D_NATIVE_RESTORE_VRK_PRESERVES_ANCHOR=PASS");
    println!("B501D_NATIVE_MISSING_ITEM=ITEM_MISSING");
    println!("B501D_NATIVE_SYNCHRONIZABLE=FALSE");
    println!("B501D_NATIVE_ACCESSIBILITY=WHEN_PASSCODE_SET_THIS_DEVICE_ONLY");
    println!("B501D_NATIVE_FRESHNESS_QUALIFICATION=PASS");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("B501D_NATIVE_FRESHNESS_QUALIFICATION=UNAVAILABLE_NON_MACOS");
}
