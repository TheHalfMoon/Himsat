#![forbid(unsafe_code)]
//! Core provider-neutral contracts shared by Himsat runtime layers.
//!
//! Specification 004B1 adds portable vault contracts, revocable keyed-handle
//! lease foundations, secret-protector behavior, reviewed key-lifetime
//! contracts, and lease-gated database/blob I/O proof. B201 adds reviewed
//! HKDF-SHA-256 purpose-key derivation. B202 adds the reviewed bounded-blob
//! XChaCha20-Poly1305 envelope. B203 adds the reviewed OS-CSPRNG nonce lifecycle
//! and in-process reservation/collision discipline. B204 adds the fixed reviewed
//! Argon2id v1 recovery envelope. B301 integrates the exact reviewed SQLCipher
//! provider behind the existing keyed-handle lease. B307 adds bounded
//! copy-verify-publish migration sequencing without source-retirement behavior.
//!
//! B503A exposes the bounded full-VRK rotation contract and quiescence proof.
//! B506 exposes the bounded active-vault deletion coordinator; concrete durable
//! backend removals, portable backup transport, and Specification 005 remain
//! outside this sub-leaf.

/// Provider-neutral vault identity, freshness, lease identity, capability, and
/// secret-protector contract surface.
pub mod vault;

/// B401 Apple data-protection Keychain adapter with bounded B501D freshness state.
#[cfg(target_os = "macos")]
pub mod vault_apple_keychain;

/// B402 Android Keystore policy adapter over an injected native backend.
pub mod vault_android_keystore;

/// B403 Windows current-user DPAPI adapter with ACL-restricted ciphertext storage.
#[cfg(target_os = "windows")]
pub mod vault_windows_dpapi;

/// B404 Linux Secret Service adapter using an encrypted session and opaque lookup metadata.
#[cfg(target_os = "linux")]
pub mod vault_linux_secret_service;

/// B202 versioned bounded-blob XChaCha20-Poly1305 envelope execution/parsing.
pub mod vault_blob;

/// B505 opaque portable-backup provider codec/key foundation.
pub mod vault_backup;
/// B505 end-to-end portable backup creation coordination.
pub mod vault_backup_creation;

/// B505 recovery-bootstrap outer privacy/authentication boundary.
pub mod vault_backup_bootstrap;

/// B505 encrypted portable-backup index codec and authenticated binding.
pub mod vault_backup_index;

/// B505 immutable provider publication and reread-acceptance boundary.
pub mod vault_backup_provider;
/// Portable-backup fresh-device restore preparation.
pub mod vault_backup_restore;

/// B505 fresh-device durable local publication and protected-genesis commit.
pub mod vault_backup_restore_publication;

pub mod vault_backup_packaging;

/// Canonical staged SQLCipher verification for authenticated portable backups.
pub mod vault_backup_sqlcipher;
/// B505 provider reconstruction and pre-SQLCipher semantic verification.
pub mod vault_backup_verify;

/// B105 lease-gated database/blob read/write wrappers and stale-handle proof.
pub mod vault_io;

/// B104/B201 portable key-domain, derivation, and secret-lifetime contracts.
pub mod vault_keys;

/// B102 in-process revocable keyed-handle lease and typed access errors.
pub mod vault_lease;

/// B307 bounded copy-verify-publish migration sequencing.
pub mod vault_migration;

/// B203 OS-CSPRNG nonce generation, reservation, retry, restore, and collision policy.
pub mod vault_nonce;

/// B501A canonical authenticated freshness-manifest codec.
pub mod vault_manifest;

/// B501 portable freshness open/genesis decision logic.
pub mod vault_freshness;

/// B502 explicit older-backup and fresh-device restore state transitions.
pub mod vault_restore;

/// B503A full-VRK rotation identities, backend boundary, and quiescence proof.
pub mod vault_rotation;

/// B103 portable secret-protector policy, capability, and binding validation.
pub mod vault_protector;

/// B204 fixed Argon2id v1 recovery-envelope execution/parsing.
pub mod vault_recovery;

/// B506 bounded active-vault deletion coordinator and retention limits.
pub mod vault_deletion;

/// 005A media-chunk authenticated-encryption envelope.
pub mod vault_media_chunk;
/// 005B session journal: append path and replay (resume follows).
pub mod vault_media_journal;
/// 005C crash-recovery scan: journal replay and envelope inventory (first grain).
pub mod vault_media_recovery;

/// 006A capture source lifecycle and session state machine.
pub mod capture_session;

/// 006B health telemetry model and change events.
pub mod capture_health;

/// 006C checkpoint derivation and chunk metadata binding.
pub mod capture_checkpoint;

/// B301 exact reviewed SQLCipher provider integration behind the B105 lease gate.
pub mod vault_sqlcipher;

/// Immutable package identity exposed for repository smoke verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildIdentity {
    /// Cargo package name.
    pub package: &'static str,
    /// Cargo package version.
    pub version: &'static str,
}

/// Returns the compile-time identity of the foundation crate.
#[must_use]
pub const fn build_identity() -> BuildIdentity {
    BuildIdentity {
        package: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildIdentity, build_identity};

    #[test]
    fn build_identity_is_deterministic() {
        assert_eq!(
            build_identity(),
            BuildIdentity {
                package: "himsat-core",
                version: "0.0.0",
            }
        );
    }
}
