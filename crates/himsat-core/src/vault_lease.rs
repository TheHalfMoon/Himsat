//! Revocable keyed-handle lease and typed access errors for Specification 004B1 B102.
//!
//! This module implements only in-process lease revocation and error propagation.
//! It performs no cryptographic operation, key destruction, persistence, protector
//! behavior, database/blob I/O, or platform secure-store work.

use crate::vault::{ProtectorError, VaultLeaseIdentity, VaultLeaseState};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard};

/// Freshness failures that must remain distinguishable from ordinary lock state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FreshnessError {
    /// The observed manifest epoch is separated from the protected anchor by an
    /// unexplained gap and must fail closed pending explicit recovery.
    Gap,
    /// The underlying protected freshness operation failed through the
    /// provider-neutral protector contract.
    Protector(ProtectorError),
}

impl fmt::Display for FreshnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gap => f.write_str("freshness epoch gap requires explicit recovery"),
            Self::Protector(error) => write!(f, "freshness protector failure: {error}"),
        }
    }
}

impl Error for FreshnessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Gap => None,
            Self::Protector(error) => Some(error),
        }
    }
}

impl From<ProtectorError> for FreshnessError {
    fn from(error: ProtectorError) -> Self {
        Self::Protector(error)
    }
}

/// Typed rejection returned by an existing keyed handle at its operation gate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyedHandleError {
    /// The owning vault was explicitly locked.
    Locked,
    /// The owning lease was revoked for another fail-closed lifecycle reason.
    Revoked,
}

impl fmt::Display for KeyedHandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Locked => f.write_str("vault is locked"),
            Self::Revoked => f.write_str("vault lease is revoked"),
        }
    }
}

impl Error for KeyedHandleError {}

/// Higher-level typed access error preserving lock, revocation, protector, and
/// freshness failure classes without collapsing them to strings.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VaultAccessError {
    /// The vault has been locked.
    Locked,
    /// The live keyed-handle lease was revoked.
    Revoked,
    /// A provider-neutral protector operation failed.
    Protector(ProtectorError),
    /// A freshness validation or protected-anchor operation failed.
    Freshness(FreshnessError),
}

impl fmt::Display for VaultAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Locked => f.write_str("vault is locked"),
            Self::Revoked => f.write_str("vault lease is revoked"),
            Self::Protector(error) => write!(f, "vault protector failure: {error}"),
            Self::Freshness(error) => write!(f, "vault freshness failure: {error}"),
        }
    }
}

impl Error for VaultAccessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Locked | Self::Revoked => None,
            Self::Protector(error) => Some(error),
            Self::Freshness(error) => Some(error),
        }
    }
}

impl From<KeyedHandleError> for VaultAccessError {
    fn from(error: KeyedHandleError) -> Self {
        match error {
            KeyedHandleError::Locked => Self::Locked,
            KeyedHandleError::Revoked => Self::Revoked,
        }
    }
}

impl From<ProtectorError> for VaultAccessError {
    fn from(error: ProtectorError) -> Self {
        Self::Protector(error)
    }
}

impl From<FreshnessError> for VaultAccessError {
    fn from(error: FreshnessError) -> Self {
        Self::Freshness(error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LeaseLifecycle {
    Active,
    Locked,
    Revoked,
}

impl LeaseLifecycle {
    const fn public_state(self) -> VaultLeaseState {
        match self {
            Self::Active => VaultLeaseState::Active,
            Self::Locked | Self::Revoked => VaultLeaseState::Revoked,
        }
    }

    const fn access_error(self) -> Option<KeyedHandleError> {
        match self {
            Self::Active => None,
            Self::Locked => Some(KeyedHandleError::Locked),
            Self::Revoked => Some(KeyedHandleError::Revoked),
        }
    }
}

/// Controller for one in-process vault lease.
///
/// The controller is intentionally not `Clone`: keyed consumers receive
/// `KeyedHandleLease` values that can authorize operations but cannot revoke or
/// reactivate the shared lease. A terminal lease never becomes active again.
#[derive(Debug)]
pub struct VaultLease {
    identity: VaultLeaseIdentity,
    lifecycle: Arc<RwLock<LeaseLifecycle>>,
}

impl VaultLease {
    /// Creates one active in-process lease bound to exactly one vault and key
    /// generation.
    #[must_use]
    pub fn new(identity: VaultLeaseIdentity) -> Self {
        Self {
            identity,
            lifecycle: Arc::new(RwLock::new(LeaseLifecycle::Active)),
        }
    }

    /// Returns the immutable vault/generation identity bound to this lease.
    #[must_use]
    pub const fn identity(&self) -> VaultLeaseIdentity {
        self.identity
    }

    /// Returns the externally visible active/revoked state.
    ///
    /// A poisoned synchronization primitive is treated as revoked so a panic
    /// cannot silently preserve authorization.
    #[must_use]
    pub fn state(&self) -> VaultLeaseState {
        match self.lifecycle.read() {
            Ok(state) => state.public_state(),
            Err(_) => VaultLeaseState::Revoked,
        }
    }

    /// Creates a non-revoking authorization token for a keyed handle.
    #[must_use]
    pub fn keyed_handle_lease(&self) -> KeyedHandleLease {
        KeyedHandleLease {
            identity: self.identity,
            lifecycle: Arc::clone(&self.lifecycle),
        }
    }

    /// Revokes the lease because the vault is being locked.
    ///
    /// Returns `true` only for the first transition out of `Active`. Later
    /// terminal transitions are rejected so the first fail-closed reason wins.
    pub fn revoke_for_lock(&self) -> bool {
        self.terminate(LeaseLifecycle::Locked)
    }

    /// Revokes the lease for a non-lock lifecycle failure such as protector
    /// revocation, rotation quiescence, authentication failure, or fatal
    /// integrity/freshness failure.
    ///
    /// This method establishes only the lease barrier. B104 owns closing keyed
    /// handles, releasing key objects, and reviewed zeroization behavior.
    pub fn revoke(&self) -> bool {
        self.terminate(LeaseLifecycle::Revoked)
    }

    fn terminate(&self, requested: LeaseLifecycle) -> bool {
        match self.lifecycle.write() {
            Ok(mut state) => {
                if *state != LeaseLifecycle::Active {
                    return false;
                }
                *state = requested;
                true
            }
            Err(poisoned) => {
                // Panic while mutating lease state is security-significant. Keep
                // the synchronization primitive poisoned and force the inner
                // value terminal so every future authorization fails closed.
                let mut state = poisoned.into_inner();
                *state = LeaseLifecycle::Revoked;
                false
            }
        }
    }
}

/// Cloneable lease reference held by a keyed database/blob handle.
///
/// It cannot change lifecycle state. Callers must acquire an authorization
/// permit immediately around each keyed operation; retaining only this token is
/// not evidence that the lease remains active.
#[derive(Clone, Debug)]
pub struct KeyedHandleLease {
    identity: VaultLeaseIdentity,
    lifecycle: Arc<RwLock<LeaseLifecycle>>,
}

impl KeyedHandleLease {
    /// Returns the immutable vault/generation identity this handle must match.
    #[must_use]
    pub const fn identity(&self) -> VaultLeaseIdentity {
        self.identity
    }

    /// Acquires an operation-scoped authorization permit.
    ///
    /// The read guard remains held for the permit lifetime. Revocation requires
    /// the corresponding write lock, so a revocation transition completes only
    /// after already-authorized operations release their permits; after that
    /// transition, no new permit can be acquired.
    pub fn authorize(&self) -> Result<KeyedHandlePermit<'_>, KeyedHandleError> {
        let guard = self
            .lifecycle
            .read()
            .map_err(|_| KeyedHandleError::Revoked)?;

        if let Some(error) = guard.access_error() {
            return Err(error);
        }

        Ok(KeyedHandlePermit {
            identity: self.identity,
            _guard: guard,
        })
    }
}

/// Operation-scoped evidence that a keyed handle observed a live lease.
///
/// The permit must not be cached across operations. Its held read guard is the
/// synchronization barrier paired with `VaultLease` revocation.
#[must_use = "keep the permit alive for the complete keyed operation"]
pub struct KeyedHandlePermit<'a> {
    identity: VaultLeaseIdentity,
    _guard: RwLockReadGuard<'a, LeaseLifecycle>,
}

impl KeyedHandlePermit<'_> {
    /// Returns the immutable vault/generation identity authorized by this permit.
    #[must_use]
    pub const fn identity(&self) -> VaultLeaseIdentity {
        self.identity
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FreshnessError, KeyedHandleError, VaultAccessError, VaultLease,
    };
    use crate::vault::{
        KeyGeneration, ProtectorError, VAULT_ID_BYTES, VaultId, VaultLeaseIdentity,
        VaultLeaseState,
    };
    use std::error::Error;

    fn identity() -> VaultLeaseIdentity {
        VaultLeaseIdentity::new(
            VaultId::from_bytes([0x42; VAULT_ID_BYTES]),
            KeyGeneration::new(7).expect("test generation is non-zero"),
        )
    }

    #[test]
    fn active_lease_authorizes_identity_bound_handle() {
        let identity = identity();
        let lease = VaultLease::new(identity);
        let handle = lease.keyed_handle_lease();

        assert_eq!(lease.identity(), identity);
        assert_eq!(lease.state(), VaultLeaseState::Active);
        assert_eq!(handle.identity(), identity);

        let permit = handle.authorize().expect("active lease must authorize");
        assert_eq!(permit.identity(), identity);
    }

    #[test]
    fn lock_revocation_rejects_existing_and_future_handle_tokens() {
        let lease = VaultLease::new(identity());
        let existing = lease.keyed_handle_lease();

        assert!(lease.revoke_for_lock());
        assert_eq!(lease.state(), VaultLeaseState::Revoked);
        assert_eq!(existing.authorize().err(), Some(KeyedHandleError::Locked));

        let future = lease.keyed_handle_lease();
        assert_eq!(future.authorize().err(), Some(KeyedHandleError::Locked));
        assert!(!lease.revoke());
    }

    #[test]
    fn non_lock_revocation_rejects_stale_handles_as_revoked() {
        let lease = VaultLease::new(identity());
        let handle = lease.keyed_handle_lease();

        assert!(lease.revoke());
        assert_eq!(handle.authorize().err(), Some(KeyedHandleError::Revoked));
        assert!(!lease.revoke_for_lock());
    }

    #[test]
    fn first_terminal_reason_wins_and_lease_never_reactivates() {
        let lease = VaultLease::new(identity());
        let handle = lease.keyed_handle_lease();

        assert!(lease.revoke_for_lock());
        assert!(!lease.revoke_for_lock());
        assert!(!lease.revoke());
        assert_eq!(lease.state(), VaultLeaseState::Revoked);
        assert_eq!(handle.authorize().err(), Some(KeyedHandleError::Locked));
    }

    #[test]
    fn access_error_preserves_lock_protector_and_freshness_classes() {
        assert_eq!(
            VaultAccessError::from(KeyedHandleError::Locked),
            VaultAccessError::Locked
        );
        assert_eq!(
            VaultAccessError::from(KeyedHandleError::Revoked),
            VaultAccessError::Revoked
        );
        assert_eq!(
            VaultAccessError::from(ProtectorError::Unavailable),
            VaultAccessError::Protector(ProtectorError::Unavailable)
        );
        assert_eq!(
            VaultAccessError::from(FreshnessError::Gap),
            VaultAccessError::Freshness(FreshnessError::Gap)
        );
    }

    #[test]
    fn wrapped_error_sources_remain_typed() {
        let protector = FreshnessError::from(ProtectorError::AnchorConflict);
        assert!(protector.source().is_some());

        let access = VaultAccessError::from(protector);
        assert!(access.source().is_some());
    }
}
