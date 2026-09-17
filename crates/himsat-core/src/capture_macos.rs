//! 007A macOS microphone adapter core over an injected audio backend.
//!
//! Portable selection, identity, and fault-classification logic for the
//! first 007 microphone pathway. The OS binding (cpal DEPEND, decided
//! in the grain evidence) arrives in the next grain behind this
//! trait; this core already drives the closed 006A session machine
//! without ever producing a refused transition. Donor recorder
//! states couple the machine to dictation surfaces; this adapter
//! keeps the portable core and pushes every OS condition through
//! typed 006A events with the exact condition preserved for
//! telemetry.

use himsat_events::{SessionId, SourceId};

use crate::capture_session::{CaptureEvent, HealthReason, SourceDescriptor, SourceKind};

/// Maximum display-label length in characters. Device names are
/// OS-provided and unbounded; descriptors stay bounded.
pub const MAX_LABEL_CHARS: usize = 128;

/// Fallback label when a device reports an empty name.
pub const UNNAMED_MICROPHONE_LABEL: &str = "Unnamed microphone";

/// Domain tag folded into stable source ids so microphone ids never
/// collide with ids derived for other source kinds.
const SOURCE_ID_DOMAIN: &[u8] = b"himsat-007a-microphone";

/// Portable sample format of a device configuration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SampleFormat {
    /// 32-bit float samples.
    F32,
    /// 16-bit signed integer samples.
    I16,
    /// 16-bit unsigned integer samples.
    U16,
}

/// Preferred device configuration resolved from a backend.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeviceConfig {
    /// Channel count of the configuration.
    pub channels: u16,
    /// Sample rate in hertz.
    pub sample_rate_hz: u32,
    /// Sample format of the configuration.
    pub format: SampleFormat,
}

/// Input device surfaced by a backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDeviceInfo {
    /// OS-reported device name (unbounded, sanitized at use).
    pub name: String,
    /// Whether the OS marks this device as the default input.
    pub is_default: bool,
    /// Preferred configuration for capture.
    pub preferred_config: DeviceConfig,
}

/// Stage at which stream handling failed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StreamStage {
    /// The stream could not be constructed.
    Build,
    /// A constructed stream would not start flowing.
    Play,
}

/// Adapter fault. OS detail travels with telemetry; the machine only
/// ever sees the classified event from the mapping functions below.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MacosMicError {
    /// No usable input device exists right now.
    NoInputDevices,
    /// A device exists but its stream failed at the given stage.
    StreamFault(StreamStage),
}

impl MacosMicError {
    /// Short stable classifier for telemetry detail strings.
    #[must_use]
    pub const fn classifier(&self) -> &'static str {
        match self {
            Self::NoInputDevices => "no-input-devices",
            Self::StreamFault(StreamStage::Build) => "stream-build-fault",
            Self::StreamFault(StreamStage::Play) => "stream-play-fault",
        }
    }
}

/// Backend contract the OS binding implements. Enumeration only in
/// this grain; stream opening arrives with the cpal binding so the
/// callback types match the proven backend shape instead of a guess.
pub trait MicrophoneBackend {
    /// Lists currently usable input devices, default first when known.
    fn input_devices(&self) -> Result<Vec<InputDeviceInfo>, MacosMicError>;

    /// OS-reported name of the default input, when the OS names one.
    fn default_input_name(&self) -> Option<String>;
}

/// Microphone selected for a session: resolved device plus its stable
/// 003 source identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedInput {
    /// Resolved device description.
    pub info: InputDeviceInfo,
    /// Stable source id derived from the device name.
    pub source_id: SourceId,
}

impl SelectedInput {
    /// Binds the selection to a session as a microphone descriptor.
    /// The label is the sanitized OS device name: exactly what device
    /// pickers already show the user, nothing more (XIV minimization).
    #[must_use]
    pub fn descriptor(&self, session_id: SessionId) -> SourceDescriptor {
        SourceDescriptor::new(
            self.source_id,
            session_id,
            SourceKind::Microphone,
            sanitize_label(&self.info.name),
        )
    }
}

/// Selects the preferred device by exact name match, else the OS
/// default, else the first enumerated device. Empty enumeration is
/// `NoInputDevices`, never a panic and never a guessed device.
pub fn select_input(
    backend: &dyn MicrophoneBackend,
    preferred: Option<&str>,
) -> Result<SelectedInput, MacosMicError> {
    let devices = backend.input_devices()?;
    if devices.is_empty() {
        return Err(MacosMicError::NoInputDevices);
    }
    let default_name = backend.default_input_name();
    let info = preferred
        .and_then(|want| devices.iter().find(|info| info.name == want))
        .or_else(|| {
            default_name
                .as_deref()
                .and_then(|want| devices.iter().find(|info| info.name == want))
        })
        .or_else(|| devices.iter().find(|info| info.is_default))
        .or(devices.first());
    match info {
        Some(info) => Ok(SelectedInput {
            info: info.clone(),
            source_id: stable_source_id(&info.name),
        }),
        None => Err(MacosMicError::NoInputDevices),
    }
}

/// Classifies a start-time failure (session is `Preparing`) into a
/// machine event the `Preparing` state accepts. Best-fit portable
/// reasons; the exact OS condition stays in telemetry detail.
#[must_use]
pub const fn event_for_start_failure(error: &MacosMicError) -> CaptureEvent {
    match error {
        MacosMicError::NoInputDevices => CaptureEvent::FailRecoverable(HealthReason::SourceSilent),
        MacosMicError::StreamFault(_) => CaptureEvent::FailRecoverable(HealthReason::RouteChanged),
    }
}

/// Classifies a runtime fault (session is `RecordingHealthy` or
/// `RecordingDegraded`) into a machine event those states accept.
#[must_use]
pub const fn event_for_runtime_fault(error: &MacosMicError) -> CaptureEvent {
    match error {
        MacosMicError::NoInputDevices => CaptureEvent::Interrupted(HealthReason::RouteChanged),
        MacosMicError::StreamFault(_) => CaptureEvent::Interrupted(HealthReason::RouteChanged),
    }
}

/// Derives a stable 003 source id from a device name. Deterministic
/// across processes (dual FNV-1a64 over domain tag plus name); equal
/// names map to equal ids, distinct names overwhelmingly differ.
#[must_use]
pub fn stable_source_id(device_name: &str) -> SourceId {
    let high = u128::from(fnv1a64(0x00, device_name));
    let low = u128::from(fnv1a64(0x01, device_name));
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

/// Sanitizes an OS device name into a bounded descriptor label:
/// trims surrounding whitespace, falls back for empty names, and
/// truncates to [`MAX_LABEL_CHARS`] characters on a char boundary.
#[must_use]
pub fn sanitize_label(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return UNNAMED_MICROPHONE_LABEL.to_owned();
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
/// Never touches the OS; failures are injected explicitly.
#[derive(Clone, Debug, Default)]
pub struct MockMicrophoneBackend {
    devices: Vec<InputDeviceInfo>,
    default_name: Option<String>,
    failure: Option<MacosMicError>,
}

impl MockMicrophoneBackend {
    /// Backend with the given devices and optional default name.
    #[must_use]
    pub fn with_devices(devices: Vec<InputDeviceInfo>, default_name: Option<String>) -> Self {
        Self {
            devices,
            default_name,
            failure: None,
        }
    }

    /// Backend whose enumeration always fails with the given fault.
    #[must_use]
    pub const fn failing(failure: MacosMicError) -> Self {
        Self {
            devices: Vec::new(),
            default_name: None,
            failure: Some(failure),
        }
    }
}

impl MicrophoneBackend for MockMicrophoneBackend {
    fn input_devices(&self) -> Result<Vec<InputDeviceInfo>, MacosMicError> {
        match &self.failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.devices.clone()),
        }
    }

    fn default_input_name(&self) -> Option<String> {
        self.default_name.clone()
    }
}

/// Builds a device description for tests and callers without a live backend.
#[must_use]
pub const fn device_info(
    name: String,
    is_default: bool,
    channels: u16,
    sample_rate_hz: u32,
) -> InputDeviceInfo {
    InputDeviceInfo {
        name,
        is_default,
        preferred_config: DeviceConfig {
            channels,
            sample_rate_hz,
            format: SampleFormat::F32,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CaptureEvent, HealthReason, MacosMicError, MockMicrophoneBackend, StreamStage, device_info,
        event_for_runtime_fault, event_for_start_failure, sanitize_label, select_input,
        stable_source_id,
    };
    use crate::capture_session::{CaptureSession, SourceKind};
    use himsat_events::{SessionId, SourceId};

    const SESSION: SessionId = SessionId::new(7);

    fn two_mic_backend() -> MockMicrophoneBackend {
        MockMicrophoneBackend::with_devices(
            vec![
                device_info("Built-in Microphone".to_owned(), true, 1, 48_000),
                device_info("USB Headset".to_owned(), false, 2, 44_100),
            ],
            Some("Built-in Microphone".to_owned()),
        )
    }

    #[test]
    fn stable_ids_match_for_equal_names() {
        assert_eq!(
            stable_source_id("Built-in Microphone"),
            stable_source_id("Built-in Microphone")
        );
    }

    #[test]
    fn stable_ids_differ_for_distinct_names() {
        assert_ne!(
            stable_source_id("Built-in Microphone"),
            stable_source_id("USB Headset")
        );
    }

    #[test]
    fn labels_truncate_on_char_boundary_with_fallback() {
        assert_eq!(sanitize_label("  USB Headset  "), "USB Headset");
        assert_eq!(sanitize_label("   "), "Unnamed microphone");
        let long = "e".repeat(200);
        assert_eq!(sanitize_label(&long).chars().count(), 128);
        let wide = "h".repeat(127) + "é";
        let label = sanitize_label(&wide);
        assert_eq!(label.chars().count(), 128);
        assert_eq!(label, wide);
        let over = "é".repeat(200);
        let cut = sanitize_label(&over);
        assert_eq!(cut.chars().count(), 128);
        assert_eq!(cut, "é".repeat(128));
    }

    #[test]
    fn selection_prefers_exact_name_then_default_then_first() {
        let backend = two_mic_backend();
        let named = select_input(&backend, Some("USB Headset")).expect("named");
        assert_eq!(named.info.name, "USB Headset");
        assert_eq!(named.descriptor(SESSION).kind(), SourceKind::Microphone);
        let defaulted = select_input(&backend, None).expect("default");
        assert_eq!(defaulted.info.name, "Built-in Microphone");
        let unnamed = MockMicrophoneBackend::with_devices(
            vec![device_info("Only Mic".to_owned(), false, 1, 48_000)],
            None,
        );
        let first = select_input(&unnamed, Some("Missing")).expect("first");
        assert_eq!(first.info.name, "Only Mic");
    }

    #[test]
    fn empty_enumeration_is_no_input_devices() {
        let backend = MockMicrophoneBackend::with_devices(Vec::new(), None);
        assert_eq!(
            select_input(&backend, None),
            Err(MacosMicError::NoInputDevices)
        );
    }

    #[test]
    fn start_failures_map_to_preparing_accepted_events() {
        assert_eq!(
            event_for_start_failure(&MacosMicError::NoInputDevices),
            CaptureEvent::FailRecoverable(HealthReason::SourceSilent)
        );
        assert_eq!(
            event_for_start_failure(&MacosMicError::StreamFault(StreamStage::Build)),
            CaptureEvent::FailRecoverable(HealthReason::RouteChanged)
        );
    }

    #[test]
    fn runtime_faults_map_to_flowing_accepted_events() {
        assert_eq!(
            event_for_runtime_fault(&MacosMicError::StreamFault(StreamStage::Play)),
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        );
        assert_eq!(
            event_for_runtime_fault(&MacosMicError::NoInputDevices),
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        );
    }

    #[test]
    fn adapter_drives_session_without_refusal() {
        let backend = two_mic_backend();
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let selected = select_input(&backend, None).expect("device");
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
        let fault = MacosMicError::StreamFault(StreamStage::Play);
        assert!(session.apply(event_for_runtime_fault(&fault)).is_ok());
        assert!(session.apply(CaptureEvent::ResumeRequested).is_ok());
        assert!(session.apply(CaptureEvent::RecoveryConfirmed).is_ok());
        assert!(session.apply(CaptureEvent::StopRequested).is_ok());
        assert!(session.apply(CaptureEvent::StopCompleted).is_ok());
        assert!(session.apply(CaptureEvent::CheckpointSealed).is_ok());
    }

    #[test]
    fn start_failure_path_recovers_through_retry() {
        let backend = MockMicrophoneBackend::with_devices(Vec::new(), None);
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let failure = select_input(&backend, None).expect_err("no devices");
        assert!(session.apply(event_for_start_failure(&failure)).is_ok());
        assert!(session.apply(CaptureEvent::RetryRequested).is_ok());
    }

    #[test]
    fn error_classifiers_are_stable_strings() {
        assert_eq!(
            MacosMicError::NoInputDevices.classifier(),
            "no-input-devices"
        );
        assert_eq!(
            MacosMicError::StreamFault(StreamStage::Build).classifier(),
            "stream-build-fault"
        );
        assert_eq!(
            MacosMicError::StreamFault(StreamStage::Play).classifier(),
            "stream-play-fault"
        );
        assert_ne!(SourceId::new(1), SourceId::new(2));
    }
}
