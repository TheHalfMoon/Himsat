#![forbid(unsafe_code)]
//! Core provider-neutral contracts shared by Himsat runtime layers.
//!
//! Specification 004B1 adds portable vault contracts, revocable keyed-handle
//! lease foundations, secret-protector behavior, reviewed key-lifetime
//! contracts, and lease-gated database/blob I/O proof. B201 adds reviewed
//! HKDF-SHA-256 purpose-key derivation. B202 adds only the reviewed bounded-blob
//! XChaCha20-Poly1305 envelope. Persistence providers and platform secure-store
//! implementations remain outside this module until separately authorized leaves.

/// Provider-neutral vault identity, freshness, lease identity, capability, and
/// secret-protector contract surface.
pub mod vault;

/// B202 versioned bounded-blob XChaCha20-Poly1305 envelope execution/parsing.
pub mod vault_blob;

/// B105 lease-gated database/blob read/write wrappers and stale-handle proof.
pub mod vault_io;

/// B104/B201 portable key-domain, derivation, and secret-lifetime contracts.
pub mod vault_keys;

/// B102 in-process revocable keyed-handle lease and typed access errors.
pub mod vault_lease;

/// B103 portable secret-protector policy, capability, and binding validation.
pub mod vault_protector;

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
