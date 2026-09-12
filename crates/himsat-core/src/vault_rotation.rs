//! B503 resumable seven-phase full-VRK rotation.
//!
//! The coordinator executes one durable transition per call. Source and target
//! protectors are distinct opaque provider records, so the source generation is
//! retained until the target state has been published, anchored, reopened, and
//! verified. B504 owns exhaustive before/after fault-injection qualification.

use crate::vault::{FreshnessAnchor, KeyGeneration, ProtectorError, VaultId, VaultLeaseIdentity};
use crate::vault_keys::{
    KeyedHandleCloser, OwnedKeyMaterial, PlaintextCache, VaultSessionLifetime, VaultTeardownReason,
};
use crate::vault_manifest::{
    FreshManifestError, ManifestError, ManifestObject, ManifestPlaintext, RotationPhase,
};
use crate::vault_nonce::NonceReservationLedger;
use crate::vault_recovery::RecoveryEnvelopeError;
use std::error::Error;
use std::fmt;

/// Non-secret identities that bind one full-VRK rotation attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FullRotationIdentity {
    vault_id: VaultId,
    source_generation: KeyGeneration,
    target_generation: KeyGeneration,
}

impl FullRotationIdentity {
    /// Reconstructs the deterministic B503 identity used to resume a persisted
    /// rotation after process restart. The target generation is exactly source+1.
    #[must_use]
    pub fn for_next_generation(
        vault_id: VaultId,
        source_generation: KeyGeneration,
    ) -> Option<Self> {
        let value = source_generation.get().checked_add(1)?;
        let target_generation = KeyGeneration::new(value).ok()?;
        Some(Self {
            vault_id,
            source_generation,
            target_generation,
        })
    }

    #[must_use]
    pub const fn vault_id(self) -> VaultId {
        self.vault_id
    }

    #[must_use]
    pub const fn source_generation(self) -> KeyGeneration {
        self.source_generation
    }

    #[must_use]
    pub const fn target_generation(self) -> KeyGeneration {
        self.target_generation
    }
}

/// Fixed byte length of one opaque provider-visible protector identifier.
pub const ROTATION_PROTECTOR_ID_BYTES: usize = 16;

/// Durable non-secret binding for the exact source and target protector records
/// used by one B503 rotation attempt. The opaque identifiers must be distinct.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FullRotationProtectorBinding {
    source_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
    target_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
}

impl FullRotationProtectorBinding {
    #[must_use]
    pub fn new(
        source_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
        target_protector_id: [u8; ROTATION_PROTECTOR_ID_BYTES],
    ) -> Option<Self> {
        if source_protector_id == target_protector_id {
            return None;
        }
        Some(Self {
            source_protector_id,
            target_protector_id,
        })
    }

    #[must_use]
    pub const fn source_protector_id(self) -> [u8; ROTATION_PROTECTOR_ID_BYTES] {
        self.source_protector_id
    }

    #[must_use]
    pub const fn target_protector_id(self) -> [u8; ROTATION_PROTECTOR_ID_BYTES] {
        self.target_protector_id
    }
}

/// Proof that the prior live session published rotation revocation, drained
/// already-authorized operations, closed keyed handles, and released session keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RotationQuiesced {
    identity: VaultLeaseIdentity,
}

impl RotationQuiesced {
    #[must_use]
    pub const fn identity(self) -> VaultLeaseIdentity {
        self.identity
    }
}

/// Quiesces normal vault operations before a full root rotation begins or resumes.
pub fn quiesce_for_full_rotation<H, C>(
    session: &mut VaultSessionLifetime<H, C>,
) -> Result<RotationQuiesced, H::Error>
where
    H: KeyedHandleCloser,
    C: PlaintextCache,
{
    let identity = session.identity();
    session.teardown(VaultTeardownReason::Rotation)?;
    Ok(RotationQuiesced { identity })
}

/// Existing opt-in recovery material that must be rewrapped for the target VRK.
#[derive(Clone, Copy, Debug)]
pub struct RecoveryRotationInput<'a> {
    source_envelope: &'a [u8],
    passphrase: &'a str,
}

impl<'a> RecoveryRotationInput<'a> {
    #[must_use]
    pub const fn new(source_envelope: &'a [u8], passphrase: &'a str) -> Self {
        Self {
            source_envelope,
            passphrase,
        }
    }

    /// Returns the existing authenticated recovery envelope for the source VRK.
    #[must_use]
    pub const fn source_envelope(self) -> &'a [u8] {
        self.source_envelope
    }

    /// Returns the exact user passphrase used to authenticate and rewrap recovery.
    #[must_use]
    pub const fn passphrase(self) -> &'a str {
        self.passphrase
    }
}

/// Durable B503 progress returned after exactly one accepted phase transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FullRotationProgress {
    InProgress {
        identity: FullRotationIdentity,
        phase: RotationPhase,
    },
    Complete {
        identity: FullRotationIdentity,
        final_anchor: FreshnessAnchor,
    },
}

/// Storage/data boundary used by the B503 coordinator.
///
/// Implementations must make every method idempotent for an exact repeated input.
/// Publication means durable write/fsync plus exact reread support. Target staging
/// must create separately named encrypted objects under `target_generation`; it
/// must never mutate the only source-generation canonical object in place.
pub trait FullRotationBackend {
    type Error;

    /// Acquires or verifies the concrete exclusion that prevents ordinary vault
    /// writes for the complete coordinator call. On success the implementation
    /// must keep that exclusion effective through all subsequent backend operations
    /// performed by this call, including after process restart. A point-in-time
    /// observation that another writer is merely absent is insufficient.
    fn assert_normal_writes_quiesced(&self) -> Result<(), Self::Error>;

    fn read_rotation_protector_binding(
        &self,
    ) -> Result<Option<FullRotationProtectorBinding>, Self::Error>;
    fn persist_rotation_protector_binding(
        &mut self,
        binding: FullRotationProtectorBinding,
    ) -> Result<(), Self::Error>;
    fn clear_rotation_protector_binding(&mut self) -> Result<(), Self::Error>;

    fn read_rotation_checkpoint(&self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn persist_rotation_checkpoint(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;
    fn clear_rotation_checkpoint(&mut self) -> Result<(), Self::Error>;

    fn read_target_recovery_wrap(&self) -> Result<Option<Vec<u8>>, Self::Error>;
    fn persist_target_recovery_wrap(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;
    fn remove_target_recovery_wrap(&mut self) -> Result<(), Self::Error>;

    fn stage_reencrypted_inventory(
        &mut self,
        source_vrk: &OwnedKeyMaterial,
        target_vrk: &OwnedKeyMaterial,
        source_manifest: &ManifestPlaintext,
        target_generation: KeyGeneration,
        nonce_ledger: &mut NonceReservationLedger,
    ) -> Result<Vec<ManifestObject>, Self::Error>;

    fn verify_staged_inventory(
        &mut self,
        target_vrk: &OwnedKeyMaterial,
        staged_objects: &[ManifestObject],
    ) -> Result<(), Self::Error>;

    fn quarantine_target_inventory(&mut self) -> Result<(), Self::Error>;

    fn publish_manifest(&mut self, envelope: &[u8]) -> Result<(), Self::Error>;
    fn read_published_manifest(&self) -> Result<Vec<u8>, Self::Error>;

    fn reopen_and_verify_published(
        &mut self,
        target_vrk: &OwnedKeyMaterial,
        manifest: &ManifestPlaintext,
    ) -> Result<(), Self::Error>;

    fn activate_target_protector(&mut self) -> Result<(), Self::Error>;
    fn retire_source_inventory(
        &mut self,
        source_generation: KeyGeneration,
    ) -> Result<(), Self::Error>;
    fn retire_source_recovery_wrap(&mut self) -> Result<(), Self::Error>;
}

/// Exact B503 failure class. No variant authorizes rollback from PUBLISH or later.
#[derive(Debug)]
pub enum FullRotationError<E> {
    Backend { phase: RotationPhase, source: E },
    Protector(ProtectorError),
    Manifest(ManifestError),
    FreshManifest(FreshManifestError),
    Recovery(RecoveryEnvelopeError),
    InvalidState,
    GenerationOverflow,
    EpochOverflow,
    RandomnessUnavailable,
    KeyMaterialCollision,
    TargetProtectorNotEmpty,
    InventoryMismatch,
}

impl<E: fmt::Display> fmt::Display for FullRotationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend { phase, source } => {
                write!(f, "B503 {phase:?} backend failure: {source}")
            }
            Self::Protector(error) => write!(f, "B503 protector failure: {error}"),
            Self::Manifest(error) => write!(f, "B503 manifest failure: {error}"),
            Self::FreshManifest(error) => write!(f, "B503 fresh-manifest failure: {error}"),
            Self::Recovery(error) => write!(f, "B503 recovery-wrap failure: {error}"),
            Self::InvalidState => f.write_str("B503 rotation state is inconsistent"),
            Self::GenerationOverflow => f.write_str("B503 target key generation overflowed"),
            Self::EpochOverflow => f.write_str("B503 freshness epoch overflowed"),
            Self::RandomnessUnavailable => f.write_str("B503 OS randomness is unavailable"),
            Self::KeyMaterialCollision => {
                f.write_str("B503 generated target VRK equals source VRK")
            }
            Self::TargetProtectorNotEmpty => {
                f.write_str("B503 target protector is not a fresh independent record")
            }
            Self::InventoryMismatch => {
                f.write_str("B503 staged inventory does not exactly replace source logical objects")
            }
        }
    }
}

impl<E: Error + 'static> Error for FullRotationError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend { source, .. } => Some(source),
            Self::Protector(error) => Some(error),
            Self::Manifest(error) => Some(error),
            Self::FreshManifest(error) => Some(error),
            Self::Recovery(error) => Some(error),
            Self::InvalidState
            | Self::GenerationOverflow
            | Self::EpochOverflow
            | Self::RandomnessUnavailable
            | Self::KeyMaterialCollision
            | Self::TargetProtectorNotEmpty
            | Self::InventoryMismatch => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{KeyGeneration, VAULT_ID_BYTES, VaultLeaseState};
    use crate::vault_keys::{VaultKeyMaterial, VaultSessionLifetime};
    use crate::vault_lease::{KeyedHandleError, VaultLease};
    use std::convert::Infallible;

    struct NoopCloser;

    impl KeyedHandleCloser for NoopCloser {
        type Error = Infallible;

        fn close_keyed_handles(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct NoopCache;

    impl PlaintextCache for NoopCache {
        fn discard_plaintext(&mut self) {}
    }

    fn identity() -> VaultLeaseIdentity {
        VaultLeaseIdentity::new(
            VaultId::from_bytes([0x51; VAULT_ID_BYTES]),
            KeyGeneration::new(9).expect("non-zero generation"),
        )
    }

    #[test]
    fn next_generation_identity_is_exact_and_overflow_fails_closed() {
        let current = identity();
        let rotation =
            FullRotationIdentity::for_next_generation(current.vault_id(), current.key_generation())
                .expect("next generation exists");
        assert_eq!(rotation.vault_id(), current.vault_id());
        assert_eq!(rotation.source_generation(), current.key_generation());
        assert_eq!(
            rotation.target_generation().get(),
            current.key_generation().get() + 1
        );

        let max = KeyGeneration::new(u64::MAX).expect("maximum is non-zero");
        assert_eq!(
            FullRotationIdentity::for_next_generation(current.vault_id(), max),
            None
        );
    }

    #[test]
    fn protector_binding_requires_distinct_opaque_records() {
        assert!(FullRotationProtectorBinding::new([0x11; 16], [0x11; 16]).is_none());
        let binding = FullRotationProtectorBinding::new([0x11; 16], [0x12; 16])
            .expect("distinct protector records");
        assert_eq!(binding.source_protector_id(), [0x11; 16]);
        assert_eq!(binding.target_protector_id(), [0x12; 16]);
    }

    #[test]
    fn quiesce_revokes_existing_handles_and_releases_session_keys() {
        let current = identity();
        let lease = VaultLease::new(current);
        let stale = lease.keyed_handle_lease();
        let mut session = VaultSessionLifetime::new(
            lease,
            NoopCloser,
            VaultKeyMaterial::new(OwnedKeyMaterial::from_bytes([0x44; 32])),
            NoopCache,
        );

        let proof = quiesce_for_full_rotation(&mut session).expect("quiescence succeeds");
        assert_eq!(proof.identity(), current);
        assert_eq!(session.lease_state(), VaultLeaseState::Revoked);
        assert!(session.keys_released());
        assert!(matches!(stale.authorize(), Err(KeyedHandleError::Revoked)));
    }
}
