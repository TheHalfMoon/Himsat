//! 008B Windows system-audio adapter core over an injected loopback backend.
//!
//! Portable render-endpoint selection, identity, and fault-classification
//! logic for the Windows system-audio pathway. The closed cpal/WASAPI
//! loopback binding arrives in the next grain behind this trait, so
//! nothing here touches the OS, and the portable core is proven on every
//! CI host before any Windows-only byte exists.
//!
//! The pathway is the OS-sanctioned WASAPI loopback mechanism only: the
//! adapter opens a *render* endpoint as a capture stream, and the closed
//! binding sets `AUDCLNT_STREAMFLAGS_LOOPBACK` itself. No private API, no
//! entitlement escape, and no driver trick is involved. The portable
//! vocabulary (configuration, sample format, stream stage) is the one
//! 008A already defines for this platform; the fault taxonomy and the
//! identity domain are separate so a loopback source never shares an id
//! or a classifier with a microphone source.

use himsat_events::{SessionId, SourceId};

use crate::capture_session::{CaptureEvent, HealthReason, SourceDescriptor, SourceKind};
use crate::capture_windows::{DeviceConfig, SampleFormat, StreamStage};

/// Maximum display-label length in characters. Device names are
/// OS-provided and unbounded; descriptors stay bounded.
pub const MAX_LABEL_CHARS: usize = 128;

/// Fallback label when an endpoint reports an empty name.
pub const UNNAMED_SYSTEM_AUDIO_LABEL: &str = "Unnamed system audio";

/// Domain tag folded into stable source ids so system-audio ids never
/// collide with microphone ids or with ids derived for other platforms.
const SOURCE_ID_DOMAIN: &[u8] = b"himsat-008b-system-audio";

/// Render endpoint available for loopback capture.
///
/// `endpoint_id` is the OS endpoint identifier of the *render* endpoint
/// being looped back. `loopback_config` is what the endpoint reports for
/// playback, because WASAPI shared-mode loopback delivers the render mix
/// format; `None` means the endpoint's default format is outside the
/// modelled set, so it may still be listed and selected but capture is
/// refused there rather than silently converted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderEndpointInfo {
    /// OS endpoint identifier of the render endpoint.
    pub endpoint_id: String,
    /// OS-reported endpoint name (unbounded, sanitized at use).
    pub name: String,
    /// Whether the OS marks this endpoint as the default render output.
    pub is_default: bool,
    /// Modelled loopback configuration, when the format is modelled.
    pub loopback_config: Option<DeviceConfig>,
}

/// Adapter fault for the system-audio pathway. OS detail travels with
/// telemetry; the machine only sees the classified outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowsSystemAudioError {
    /// No render endpoint exists to loop back from.
    NoRenderEndpoints,
    /// An endpoint exists but its loopback stream failed at the stage.
    StreamFault(StreamStage),
    /// The OS refused access to the endpoint or its policy surface.
    PermissionDenied,
    /// The endpoint is held by another client in a blocking mode.
    ExclusiveModeConflict,
    /// The endpoint disappeared while selected.
    EndpointUnavailable,
    /// The Windows audio service is not running.
    AudioServiceUnavailable,
    /// The OS rerouted the default render endpoint. The loopback stream
    /// built from the default endpoint follows it and stays active, so
    /// this is telemetry detail and never an interrupting fault.
    RouteRerouted,
}

impl WindowsSystemAudioError {
    /// Short stable classifier for telemetry detail strings. The shared
    /// vocabulary matches the microphone adapter's so telemetry reads
    /// uniformly across Windows pathways.
    #[must_use]
    pub const fn classifier(&self) -> &'static str {
        match self {
            Self::NoRenderEndpoints => "no-render-endpoints",
            Self::StreamFault(StreamStage::Build) => "stream-build-fault",
            Self::StreamFault(StreamStage::Play) => "stream-play-fault",
            Self::PermissionDenied => "permission-denied",
            Self::ExclusiveModeConflict => "exclusive-mode-conflict",
            Self::EndpointUnavailable => "endpoint-unavailable",
            Self::AudioServiceUnavailable => "audio-service-unavailable",
            Self::RouteRerouted => "route-rerouted",
        }
    }
}

/// Backend contract the OS binding implements.
pub trait LoopbackBackend {
    /// Lists render endpoints available for loopback capture. A backend
    /// must never report an empty endpoint id: identity derives from it,
    /// so empty ids would collide into one source. The OS binding skips
    /// endpoints whose id it cannot resolve.
    fn render_endpoints(&self) -> Result<Vec<RenderEndpointInfo>, WindowsSystemAudioError>;

    /// OS endpoint id of the default render endpoint, when the OS names
    /// one. Looping back the default render endpoint is the
    /// OS-sanctioned "system audio" source.
    fn default_render_endpoint_id(&self) -> Option<String>;
}

/// System-audio source selected for a session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedSystemAudio {
    /// Resolved endpoint description.
    pub info: RenderEndpointInfo,
    /// Stable source id derived from the endpoint id.
    pub source_id: SourceId,
}

impl SelectedSystemAudio {
    /// Binds the selection to a session as a system-audio descriptor.
    /// The label is the sanitized OS endpoint name: what a device picker
    /// already shows, nothing more (XIV minimization).
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

/// Selects the preferred render endpoint by exact endpoint-id match,
/// then by exact name match, then the OS default endpoint, then the
/// endpoint the backend flags as default, then the first enumerated one.
/// Empty enumeration is `NoRenderEndpoints`, never a guessed endpoint.
pub fn select_loopback_input(
    backend: &dyn LoopbackBackend,
    preferred: Option<&str>,
) -> Result<SelectedSystemAudio, WindowsSystemAudioError> {
    let endpoints = backend.render_endpoints()?;
    if endpoints.is_empty() {
        return Err(WindowsSystemAudioError::NoRenderEndpoints);
    }
    let default_id = backend.default_render_endpoint_id();
    let info = preferred
        .and_then(|want| endpoints.iter().find(|info| info.endpoint_id == want))
        .or_else(|| preferred.and_then(|want| endpoints.iter().find(|info| info.name == want)))
        .or_else(|| {
            default_id
                .as_deref()
                .and_then(|want| endpoints.iter().find(|info| info.endpoint_id == want))
        })
        .or_else(|| endpoints.iter().find(|info| info.is_default))
        .or(endpoints.first());
    match info {
        Some(info) => Ok(SelectedSystemAudio {
            info: info.clone(),
            source_id: stable_system_source_id(&info.endpoint_id),
        }),
        None => Err(WindowsSystemAudioError::NoRenderEndpoints),
    }
}

/// Classifies a start-time failure (session is `Preparing`) into a
/// machine event the `Preparing` state accepts.
#[must_use]
pub const fn event_for_start_failure(error: &WindowsSystemAudioError) -> CaptureEvent {
    match error {
        WindowsSystemAudioError::NoRenderEndpoints => {
            CaptureEvent::FailRecoverable(HealthReason::SourceSilent)
        }
        WindowsSystemAudioError::PermissionDenied => {
            CaptureEvent::FailRecoverable(HealthReason::PermissionRevoked)
        }
        // Stream faults, endpoint loss, exclusive-mode conflict, service
        // unavailability, and (defensively) a reroute are route or
        // availability failures at start; the OS detail keeps the exact
        // condition. `RouteRerouted` is unreachable here by contract.
        _ => CaptureEvent::FailRecoverable(HealthReason::RouteChanged),
    }
}

/// Classifies a runtime fault (session is `RecordingHealthy` or
/// `RecordingDegraded`) into a machine event those states accept.
/// `None` means "record the telemetry detail and do not move the
/// session machine": a rerouted default endpoint keeps the loopback
/// stream flowing, so interrupting it would be a false stop.
#[must_use]
pub const fn event_for_runtime_fault(error: &WindowsSystemAudioError) -> Option<CaptureEvent> {
    match error {
        WindowsSystemAudioError::RouteRerouted => None,
        WindowsSystemAudioError::PermissionDenied => {
            Some(CaptureEvent::Interrupted(HealthReason::PermissionRevoked))
        }
        _ => Some(CaptureEvent::Interrupted(HealthReason::RouteChanged)),
    }
}

/// Portable 006B degradation classification for a runtime fault, or
/// `None` when there is no degradation to report. Kept consistent with
/// [`event_for_runtime_fault`] by a test rather than by convention.
#[must_use]
pub const fn health_reason_for(error: &WindowsSystemAudioError) -> Option<HealthReason> {
    match error {
        WindowsSystemAudioError::RouteRerouted => None,
        WindowsSystemAudioError::PermissionDenied => Some(HealthReason::PermissionRevoked),
        _ => Some(HealthReason::RouteChanged),
    }
}

/// Derives a stable 003 source id from a render endpoint id. The domain
/// tag keeps these ids disjoint from microphone ids even for identical
/// endpoint strings.
#[must_use]
pub fn stable_system_source_id(endpoint_id: &str) -> SourceId {
    let high = u128::from(fnv1a64(0x00, endpoint_id));
    let low = u128::from(fnv1a64(0x01, endpoint_id));
    SourceId::new(high << 64 | low)
}

/// 64-bit FNV-1a over the domain tag, one lane byte, and the value.
fn fnv1a64(lane: u8, value: &str) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for byte in SOURCE_ID_DOMAIN
        .iter()
        .chain(std::slice::from_ref(&lane))
        .chain(value.as_bytes())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// Sanitizes an OS endpoint name into a bounded descriptor label.
#[must_use]
pub fn sanitize_label(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return UNNAMED_SYSTEM_AUDIO_LABEL.to_owned();
    }
    if trimmed.chars().count() <= MAX_LABEL_CHARS {
        return trimmed.to_owned();
    }
    let end = trimmed
        .char_indices()
        .nth(MAX_LABEL_CHARS)
        .map_or(trimmed.len(), |(index, _)| index);
    trimmed[..end].to_owned()
}

/// In-memory backend for tests and for hosts without audio hardware.
#[derive(Clone, Debug, Default)]
pub struct MockLoopbackBackend {
    endpoints: Vec<RenderEndpointInfo>,
    default_id: Option<String>,
    failure: Option<WindowsSystemAudioError>,
}

impl MockLoopbackBackend {
    /// Backend with the given endpoints and optional default endpoint id.
    #[must_use]
    pub fn with_endpoints(endpoints: Vec<RenderEndpointInfo>, default_id: Option<String>) -> Self {
        Self {
            endpoints,
            default_id,
            failure: None,
        }
    }

    /// Backend whose enumeration always fails with the given fault.
    #[must_use]
    pub const fn failing(failure: WindowsSystemAudioError) -> Self {
        Self {
            endpoints: Vec::new(),
            default_id: None,
            failure: Some(failure),
        }
    }
}

impl LoopbackBackend for MockLoopbackBackend {
    fn render_endpoints(&self) -> Result<Vec<RenderEndpointInfo>, WindowsSystemAudioError> {
        match &self.failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.endpoints.clone()),
        }
    }

    fn default_render_endpoint_id(&self) -> Option<String> {
        self.default_id.clone()
    }
}

/// Builds a render-endpoint description for tests and callers without a
/// live backend.
#[must_use]
pub fn endpoint_info(
    endpoint_id: &str,
    name: &str,
    is_default: bool,
    channels: u16,
    sample_rate_hz: u32,
) -> RenderEndpointInfo {
    RenderEndpointInfo {
        endpoint_id: endpoint_id.to_owned(),
        name: name.to_owned(),
        is_default,
        loopback_config: Some(DeviceConfig {
            channels,
            sample_rate_hz,
            format: SampleFormat::F32,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CaptureEvent, HealthReason, MAX_LABEL_CHARS, MockLoopbackBackend, StreamStage,
        UNNAMED_SYSTEM_AUDIO_LABEL, WindowsSystemAudioError, endpoint_info,
        event_for_runtime_fault, event_for_start_failure, health_reason_for, sanitize_label,
        select_loopback_input, stable_system_source_id,
    };
    use crate::capture_health::{HealthChange, HealthMonitor, SignalSample};
    use crate::capture_session::{CaptureSession, CaptureState, SourceKind};
    use himsat_events::SessionId;

    const SESSION: SessionId = SessionId::new(9);

    fn all_faults() -> Vec<WindowsSystemAudioError> {
        vec![
            WindowsSystemAudioError::NoRenderEndpoints,
            WindowsSystemAudioError::StreamFault(StreamStage::Build),
            WindowsSystemAudioError::StreamFault(StreamStage::Play),
            WindowsSystemAudioError::PermissionDenied,
            WindowsSystemAudioError::ExclusiveModeConflict,
            WindowsSystemAudioError::EndpointUnavailable,
            WindowsSystemAudioError::AudioServiceUnavailable,
            WindowsSystemAudioError::RouteRerouted,
        ]
    }

    fn two_endpoint_backend() -> MockLoopbackBackend {
        MockLoopbackBackend::with_endpoints(
            vec![
                endpoint_info("render-speakers", "Speakers (Built-in)", true, 2, 48_000),
                endpoint_info("render-hdmi", "HDMI Output", false, 2, 48_000),
            ],
            Some("render-speakers".to_owned()),
        )
    }

    fn quiet_sample(reason: Option<HealthReason>) -> SignalSample {
        SignalSample {
            elapsed_millis: 0,
            level_dbfs_tenths: None,
            silent: false,
            clipping: false,
            queue_depth: 0,
            queue_capacity: 16,
            free_bytes: 1024,
            warn_bytes: 256,
            critical_bytes: 64,
            reason,
        }
    }

    #[test]
    fn stable_ids_match_for_equal_endpoints_and_differ_otherwise() {
        assert_eq!(
            stable_system_source_id("render-speakers"),
            stable_system_source_id("render-speakers")
        );
        assert_ne!(
            stable_system_source_id("render-speakers"),
            stable_system_source_id("render-hdmi")
        );
    }

    #[test]
    fn system_ids_are_domain_separated_from_every_other_adapter() {
        assert_ne!(
            stable_system_source_id("shared-endpoint"),
            crate::capture_windows::stable_source_id("shared-endpoint")
        );
        assert_ne!(
            stable_system_source_id("shared-endpoint"),
            crate::capture_macos::stable_source_id("shared-endpoint")
        );
    }

    #[test]
    fn selection_prefers_endpoint_id_then_name_then_default() {
        let backend = two_endpoint_backend();
        let by_id = select_loopback_input(&backend, Some("render-hdmi")).expect("id match");
        assert_eq!(by_id.info.name, "HDMI Output");
        assert_eq!(by_id.source_id, stable_system_source_id("render-hdmi"));
        let by_name = select_loopback_input(&backend, Some("HDMI Output")).expect("name match");
        assert_eq!(by_name.source_id, stable_system_source_id("render-hdmi"));
        let defaulted = select_loopback_input(&backend, None).expect("default");
        assert_eq!(
            defaulted.source_id,
            stable_system_source_id("render-speakers")
        );
        assert_eq!(
            defaulted.descriptor(SESSION).kind(),
            SourceKind::SystemAudio
        );
    }

    #[test]
    fn selection_falls_back_to_flagged_default_then_first() {
        let flagged = MockLoopbackBackend::with_endpoints(
            vec![
                endpoint_info("render-a", "A", false, 2, 48_000),
                endpoint_info("render-b", "B", true, 2, 48_000),
            ],
            None,
        );
        assert_eq!(
            select_loopback_input(&flagged, None)
                .expect("flagged")
                .source_id,
            stable_system_source_id("render-b")
        );
        let unnamed = MockLoopbackBackend::with_endpoints(
            vec![endpoint_info("render-only", "", false, 2, 48_000)],
            None,
        );
        let first = select_loopback_input(&unnamed, Some("missing")).expect("first");
        assert_eq!(first.source_id, stable_system_source_id("render-only"));
        assert_eq!(
            first.descriptor(SESSION).label(),
            UNNAMED_SYSTEM_AUDIO_LABEL
        );
    }

    #[test]
    fn empty_enumeration_and_enumeration_faults_stay_typed() {
        let empty = MockLoopbackBackend::default();
        assert_eq!(
            select_loopback_input(&empty, None),
            Err(WindowsSystemAudioError::NoRenderEndpoints)
        );
        let failing =
            MockLoopbackBackend::failing(WindowsSystemAudioError::AudioServiceUnavailable);
        assert_eq!(
            select_loopback_input(&failing, None),
            Err(WindowsSystemAudioError::AudioServiceUnavailable)
        );
    }

    #[test]
    fn labels_truncate_on_char_boundary_with_system_audio_fallback() {
        assert_eq!(sanitize_label("  Speakers  "), "Speakers");
        assert_eq!(sanitize_label("   "), UNNAMED_SYSTEM_AUDIO_LABEL);
        let long = "o".repeat(MAX_LABEL_CHARS + 40);
        assert_eq!(sanitize_label(&long).chars().count(), MAX_LABEL_CHARS);
        let over = "é".repeat(MAX_LABEL_CHARS + 10);
        assert_eq!(sanitize_label(&over), "é".repeat(MAX_LABEL_CHARS));
    }

    #[test]
    fn start_failures_map_to_preparing_accepted_events() {
        for fault in all_faults() {
            let event = event_for_start_failure(&fault);
            let mut probe = CaptureSession::new(SESSION);
            assert!(probe.apply(CaptureEvent::Prepare).is_ok());
            assert!(
                probe.apply(event).is_ok(),
                "start mapping refused for {}",
                fault.classifier()
            );
        }
        assert_eq!(
            event_for_start_failure(&WindowsSystemAudioError::NoRenderEndpoints),
            CaptureEvent::FailRecoverable(HealthReason::SourceSilent)
        );
        assert_eq!(
            event_for_start_failure(&WindowsSystemAudioError::PermissionDenied),
            CaptureEvent::FailRecoverable(HealthReason::PermissionRevoked)
        );
    }

    #[test]
    fn runtime_faults_map_to_flowing_accepted_events() {
        for fault in all_faults() {
            let mut session = CaptureSession::new(SESSION);
            assert!(session.apply(CaptureEvent::Prepare).is_ok());
            let selected = select_loopback_input(&two_endpoint_backend(), None).expect("endpoint");
            assert!(session.attach(selected.descriptor(SESSION)).is_ok());
            assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
            match event_for_runtime_fault(&fault) {
                Some(event) => assert!(
                    session.apply(event).is_ok(),
                    "runtime mapping refused for {}",
                    fault.classifier()
                ),
                None => assert_eq!(fault, WindowsSystemAudioError::RouteRerouted),
            }
        }
    }

    #[test]
    fn rerouted_default_endpoint_keeps_flowing_without_a_machine_event() {
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let selected = select_loopback_input(&two_endpoint_backend(), None).expect("endpoint");
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
        assert_eq!(session.state(), CaptureState::RecordingHealthy);
        assert_eq!(
            event_for_runtime_fault(&WindowsSystemAudioError::RouteRerouted),
            None
        );
        assert_eq!(
            health_reason_for(&WindowsSystemAudioError::RouteRerouted),
            None
        );
    }

    #[test]
    fn runtime_health_reason_agrees_with_the_machine_mapping() {
        for fault in all_faults() {
            let from_event = match event_for_runtime_fault(&fault) {
                Some(CaptureEvent::Interrupted(reason)) => Some(reason),
                Some(other) => panic!("unexpected runtime mapping {other:?}"),
                None => None,
            };
            assert_eq!(
                health_reason_for(&fault),
                from_event,
                "health and machine mappings diverged for {}",
                fault.classifier()
            );
        }
    }

    #[test]
    fn exclusive_mode_conflict_surfaces_through_006b_health() {
        let mut monitor = HealthMonitor::new(SESSION);
        let state = CaptureState::RecordingHealthy;
        assert!(monitor.observe(state, &quiet_sample(None)).is_empty());
        let reason = health_reason_for(&WindowsSystemAudioError::ExclusiveModeConflict);
        let events = monitor.observe(state, &quiet_sample(reason));
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].change(),
            HealthChange::ReasonChanged {
                from: None,
                to: Some(HealthReason::RouteChanged),
            }
        );
        assert_eq!(events[0].session(), SESSION);
    }

    #[test]
    fn adapter_drives_session_without_refusal_and_recovers() {
        let backend = two_endpoint_backend();
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let selected = select_loopback_input(&backend, None).expect("endpoint");
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
        let fault = WindowsSystemAudioError::EndpointUnavailable;
        let event = event_for_runtime_fault(&fault).expect("interrupting fault");
        assert!(session.apply(event).is_ok());
        assert!(session.apply(CaptureEvent::ResumeRequested).is_ok());
        assert!(session.apply(CaptureEvent::RecoveryConfirmed).is_ok());
        assert!(session.apply(CaptureEvent::StopRequested).is_ok());
        assert!(session.apply(CaptureEvent::StopCompleted).is_ok());
        assert!(session.apply(CaptureEvent::CheckpointSealed).is_ok());

        let empty = MockLoopbackBackend::default();
        let mut retry = CaptureSession::new(SESSION);
        assert!(retry.apply(CaptureEvent::Prepare).is_ok());
        let failure = select_loopback_input(&empty, None).expect_err("no endpoints");
        assert!(retry.apply(event_for_start_failure(&failure)).is_ok());
        assert!(retry.apply(CaptureEvent::RetryRequested).is_ok());
    }

    #[test]
    fn error_classifiers_are_stable_and_distinct() {
        let classifiers: Vec<&str> = all_faults()
            .iter()
            .map(WindowsSystemAudioError::classifier)
            .collect();
        assert_eq!(classifiers[0], "no-render-endpoints");
        assert_eq!(classifiers[1], "stream-build-fault");
        assert_eq!(classifiers[2], "stream-play-fault");
        assert_eq!(classifiers[5], "endpoint-unavailable");
        assert_eq!(classifiers[7], "route-rerouted");
        let mut unique = classifiers.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), classifiers.len());
    }
}
