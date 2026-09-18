//! 007B system-audio tap adapter core over an injected tap backend.
//!
//! Portable selection, identity, and fault-classification logic for
//! the authorized system/screen-audio pathway. The OS binding (a
//! process-tap plus aggregate-device backend in the donor-proven
//! shape, arriving in the next grain behind [`SystemTapBackend`])
//! is the only OS surface; this core already drives the closed
//! 006A session machine without ever producing a refused
//! transition. Tap authorization keeps its own identity end to
//! end: denial is reported as revocation, never folded into a
//! generic fault and never silent.

use himsat_events::{SessionId, SourceId};

use crate::capture_macos::{DeviceConfig, sanitize_label};
use crate::capture_session::{CaptureEvent, HealthReason, SourceDescriptor, SourceKind};

/// Domain tag folded into stable system-tap source ids. Distinct
/// from the microphone domain so the same OS name under both kinds
/// never shares an id.
const SYSTEM_SOURCE_ID_DOMAIN: &[u8] = b"himsat-007b-system-audio";

/// Stage at which tap handling failed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TapStage {
    /// The tap could not be created.
    Create,
    /// A created tap would not start flowing.
    Start,
}

/// Adapter fault. OS detail travels with telemetry; the machine only
/// ever sees the classified event from the mapping functions below.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemTapError {
    /// No usable system-audio tap exists right now.
    NoTapAvailable,
    /// A tap exists but failed at the given stage.
    TapFault(TapStage),
    /// The OS denied tap authorization. Distinct from absence: the
    /// pathway exists but policy forbids capture.
    AuthorizationDenied,
}

impl SystemTapError {
    /// Short stable classifier for telemetry detail strings.
    #[must_use]
    pub const fn classifier(&self) -> &'static str {
        match self {
            Self::NoTapAvailable => "no-tap-available",
            Self::TapFault(TapStage::Create) => "tap-create-fault",
            Self::TapFault(TapStage::Start) => "tap-start-fault",
            Self::AuthorizationDenied => "tap-authorization-denied",
        }
    }
}

/// System-audio tap target surfaced by a backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TapTargetInfo {
    /// OS-reported endpoint name (unbounded, sanitized at use).
    pub name: String,
    /// Whether the OS marks this endpoint as the default route.
    pub is_default: bool,
    /// Preferred configuration for capture.
    pub preferred_config: DeviceConfig,
}

/// Backend contract the OS binding implements. Enumeration only in
/// this grain; tap creation arrives with the native binding so the
/// handle types match the proven backend shape instead of a guess.
pub trait SystemTapBackend {
    /// Lists currently usable system-audio tap targets.
    fn tap_targets(&self) -> Result<Vec<TapTargetInfo>, SystemTapError>;
}

/// System-audio target selected for a session: resolved endpoint
/// plus its stable 003 source identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedSystemInput {
    /// Resolved tap target description.
    pub info: TapTargetInfo,
    /// Stable source id derived from the endpoint name.
    pub source_id: SourceId,
}

impl SelectedSystemInput {
    /// Binds the selection to a session as a system-audio
    /// descriptor. The label is the sanitized OS endpoint name:
    /// exactly what route pickers already show the user, nothing
    /// more (XIV minimization).
    #[must_use]
    pub fn descriptor(&self, session_id: SessionId) -> SourceDescriptor {
        SourceDescriptor::new(
            self.source_id,
            session_id,
            SourceKind::SystemAudio,
            sanitize_label(&self.info.name),
        )
    }
}

/// Selects the preferred endpoint by exact name match, else the OS
/// default, else the first enumerated target. Empty enumeration is
/// `NoTapAvailable`, never a panic and never a guessed endpoint.
pub fn select_system_input(
    backend: &dyn SystemTapBackend,
    preferred: Option<&str>,
) -> Result<SelectedSystemInput, SystemTapError> {
    let targets = backend.tap_targets()?;
    if targets.is_empty() {
        return Err(SystemTapError::NoTapAvailable);
    }
    let info = preferred
        .and_then(|want| targets.iter().find(|info| info.name == want))
        .or_else(|| targets.iter().find(|info| info.is_default))
        .or(targets.first());
    match info {
        Some(info) => Ok(SelectedSystemInput {
            info: info.clone(),
            source_id: stable_system_source_id(&info.name),
        }),
        None => Err(SystemTapError::NoTapAvailable),
    }
}

/// Classifies a start-time failure (session is `Preparing`) into a
/// machine event the `Preparing` state accepts. Best-fit portable
/// reasons; the exact OS condition stays in telemetry detail.
#[must_use]
pub const fn event_for_tap_start_failure(error: &SystemTapError) -> CaptureEvent {
    match error {
        SystemTapError::NoTapAvailable => CaptureEvent::FailRecoverable(HealthReason::SourceSilent),
        SystemTapError::TapFault(_) => CaptureEvent::FailRecoverable(HealthReason::RouteChanged),
        SystemTapError::AuthorizationDenied => {
            CaptureEvent::FailRecoverable(HealthReason::PermissionRevoked)
        }
    }
}

/// Classifies a runtime fault (session is `RecordingHealthy` or
/// `RecordingDegraded`) into a machine event those states accept.
#[must_use]
pub const fn event_for_tap_runtime_fault(error: &SystemTapError) -> CaptureEvent {
    match error {
        SystemTapError::NoTapAvailable => CaptureEvent::Interrupted(HealthReason::RouteChanged),
        SystemTapError::TapFault(_) => CaptureEvent::Interrupted(HealthReason::RouteChanged),
        SystemTapError::AuthorizationDenied => {
            CaptureEvent::Interrupted(HealthReason::PermissionRevoked)
        }
    }
}

/// Derives a stable 003 source id from an endpoint name in the
/// system-audio id space. Deterministic across processes (dual
/// FNV-1a64 over the 007B domain tag plus name); the space is
/// disjoint from microphone ids by construction.
#[must_use]
pub fn stable_system_source_id(endpoint_name: &str) -> SourceId {
    let high = u128::from(fnv1a64(0x00, endpoint_name));
    let low = u128::from(fnv1a64(0x01, endpoint_name));
    SourceId::new(high << 64 | low)
}

/// 64-bit FNV-1a over the system-audio domain tag, one lane byte,
/// and the value.
fn fnv1a64(lane: u8, value: &str) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for byte in SYSTEM_SOURCE_ID_DOMAIN
        .iter()
        .chain(std::slice::from_ref(&lane))
        .chain(value.as_bytes())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// In-memory backend for tests and for hosts without tap support.
/// Never touches the OS; failures are injected explicitly.
#[derive(Clone, Debug, Default)]
pub struct MockSystemTapBackend {
    targets: Vec<TapTargetInfo>,
    failure: Option<SystemTapError>,
}

impl MockSystemTapBackend {
    /// Backend with the given tap targets.
    #[must_use]
    pub fn with_targets(targets: Vec<TapTargetInfo>) -> Self {
        Self {
            targets,
            failure: None,
        }
    }

    /// Backend whose enumeration always fails with the given fault.
    #[must_use]
    pub const fn failing(failure: SystemTapError) -> Self {
        Self {
            targets: Vec::new(),
            failure: Some(failure),
        }
    }
}

impl SystemTapBackend for MockSystemTapBackend {
    fn tap_targets(&self) -> Result<Vec<TapTargetInfo>, SystemTapError> {
        match &self.failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.targets.clone()),
        }
    }
}

/// Builds a tap target description for tests and callers without a live backend.
#[must_use]
pub const fn tap_target_info(
    name: String,
    is_default: bool,
    channels: u16,
    sample_rate_hz: u32,
) -> TapTargetInfo {
    TapTargetInfo {
        name,
        is_default,
        preferred_config: DeviceConfig {
            channels,
            sample_rate_hz,
            format: crate::capture_macos::SampleFormat::F32,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CaptureEvent, HealthReason, MockSystemTapBackend, SystemTapError, TapStage,
        event_for_tap_runtime_fault, event_for_tap_start_failure, select_system_input,
        stable_system_source_id, tap_target_info,
    };
    use crate::capture_macos::stable_source_id;
    use crate::capture_session::{CaptureSession, SourceKind};
    use himsat_events::SessionId;

    const SESSION: SessionId = SessionId::new(21);

    fn two_target_backend() -> MockSystemTapBackend {
        MockSystemTapBackend::with_targets(vec![
            tap_target_info("System Output".to_owned(), true, 2, 48_000),
            tap_target_info("External Display".to_owned(), false, 2, 48_000),
        ])
    }

    #[test]
    fn system_ids_are_stable_and_kind_separated() {
        assert_eq!(
            stable_system_source_id("System Output"),
            stable_system_source_id("System Output")
        );
        assert_ne!(
            stable_system_source_id("System Output"),
            stable_system_source_id("External Display")
        );
        assert_ne!(
            stable_system_source_id("Built-in Microphone"),
            stable_source_id("Built-in Microphone")
        );
    }

    #[test]
    fn selection_prefers_exact_name_then_default_then_first() {
        let backend = two_target_backend();
        let named = select_system_input(&backend, Some("External Display")).expect("named");
        assert_eq!(named.info.name, "External Display");
        assert_eq!(named.descriptor(SESSION).kind(), SourceKind::SystemAudio);
        let defaulted = select_system_input(&backend, None).expect("default");
        assert_eq!(defaulted.info.name, "System Output");
        let single = MockSystemTapBackend::with_targets(vec![tap_target_info(
            "Only Route".to_owned(),
            false,
            2,
            48_000,
        )]);
        let first = select_system_input(&single, Some("Missing")).expect("first");
        assert_eq!(first.info.name, "Only Route");
    }

    #[test]
    fn empty_enumeration_is_no_tap_available() {
        let backend = MockSystemTapBackend::with_targets(Vec::new());
        assert_eq!(
            select_system_input(&backend, None),
            Err(SystemTapError::NoTapAvailable)
        );
    }

    #[test]
    fn start_failures_map_to_preparing_accepted_events() {
        assert_eq!(
            event_for_tap_start_failure(&SystemTapError::NoTapAvailable),
            CaptureEvent::FailRecoverable(HealthReason::SourceSilent)
        );
        assert_eq!(
            event_for_tap_start_failure(&SystemTapError::TapFault(TapStage::Create)),
            CaptureEvent::FailRecoverable(HealthReason::RouteChanged)
        );
        assert_eq!(
            event_for_tap_start_failure(&SystemTapError::AuthorizationDenied),
            CaptureEvent::FailRecoverable(HealthReason::PermissionRevoked)
        );
    }

    #[test]
    fn runtime_faults_map_to_flowing_accepted_events() {
        assert_eq!(
            event_for_tap_runtime_fault(&SystemTapError::TapFault(TapStage::Start)),
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        );
        assert_eq!(
            event_for_tap_runtime_fault(&SystemTapError::NoTapAvailable),
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        );
        assert_eq!(
            event_for_tap_runtime_fault(&SystemTapError::AuthorizationDenied),
            CaptureEvent::Interrupted(HealthReason::PermissionRevoked)
        );
    }

    #[test]
    fn tap_drives_session_without_refusal() {
        let backend = two_target_backend();
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let selected = select_system_input(&backend, None).expect("target");
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
        let fault = SystemTapError::TapFault(TapStage::Start);
        assert!(session.apply(event_for_tap_runtime_fault(&fault)).is_ok());
        assert!(session.apply(CaptureEvent::ResumeRequested).is_ok());
        assert!(session.apply(CaptureEvent::RecoveryConfirmed).is_ok());
        assert!(session.apply(CaptureEvent::StopRequested).is_ok());
        assert!(session.apply(CaptureEvent::StopCompleted).is_ok());
        assert!(session.apply(CaptureEvent::CheckpointSealed).is_ok());
    }

    #[test]
    fn authorization_denial_path_recovers_through_retry() {
        let backend = MockSystemTapBackend::failing(SystemTapError::AuthorizationDenied);
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let failure = select_system_input(&backend, None).expect_err("denied");
        assert!(session.apply(event_for_tap_start_failure(&failure)).is_ok());
        assert!(session.apply(CaptureEvent::RetryRequested).is_ok());
    }

    #[test]
    fn error_classifiers_are_stable_strings() {
        assert_eq!(
            SystemTapError::NoTapAvailable.classifier(),
            "no-tap-available"
        );
        assert_eq!(
            SystemTapError::TapFault(TapStage::Create).classifier(),
            "tap-create-fault"
        );
        assert_eq!(
            SystemTapError::TapFault(TapStage::Start).classifier(),
            "tap-start-fault"
        );
        assert_eq!(
            SystemTapError::AuthorizationDenied.classifier(),
            "tap-authorization-denied"
        );
    }
}
