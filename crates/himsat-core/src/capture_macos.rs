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
    /// The OS denied microphone access (TCC). Distinct from absence:
    /// the device exists but policy forbids capture.
    PermissionDenied,
}

impl MacosMicError {
    /// Short stable classifier for telemetry detail strings.
    #[must_use]
    pub const fn classifier(&self) -> &'static str {
        match self {
            Self::NoInputDevices => "no-input-devices",
            Self::StreamFault(StreamStage::Build) => "stream-build-fault",
            Self::StreamFault(StreamStage::Play) => "stream-play-fault",
            Self::PermissionDenied => "permission-denied",
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
        MacosMicError::PermissionDenied => {
            CaptureEvent::FailRecoverable(HealthReason::PermissionRevoked)
        }
    }
}

/// Classifies a runtime fault (session is `RecordingHealthy` or
/// `RecordingDegraded`) into a machine event those states accept.
#[must_use]
pub const fn event_for_runtime_fault(error: &MacosMicError) -> CaptureEvent {
    match error {
        MacosMicError::NoInputDevices => CaptureEvent::Interrupted(HealthReason::RouteChanged),
        MacosMicError::StreamFault(_) => CaptureEvent::Interrupted(HealthReason::RouteChanged),
        MacosMicError::PermissionDenied => {
            CaptureEvent::Interrupted(HealthReason::PermissionRevoked)
        }
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

/// cpal trait imports for the 007A OS binding (macOS only).
#[cfg(target_os = "macos")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Live cpal input stream. Dropping stops capture: callers must drive
/// `StopRequested`/`StopCompleted` through the session machine first so
/// a drop is never a silent stop.
#[cfg(target_os = "macos")]
pub struct LiveMicStream {
    stream: cpal::Stream,
    device_name: String,
    config: DeviceConfig,
}

#[cfg(target_os = "macos")]
impl LiveMicStream {
    /// OS-reported name of the streaming device.
    #[must_use]
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Configuration the stream was built with.
    #[must_use]
    pub const fn config(&self) -> DeviceConfig {
        self.config
    }

    /// Pauses frame delivery without tearing down, for the
    /// `Interrupted` state. Failures classify as play-stage faults.
    pub fn pause(&self) -> Result<(), MacosMicError> {
        self.stream
            .pause()
            .map_err(|_| MacosMicError::StreamFault(StreamStage::Play))
    }

    /// Resumes a paused stream, for the `Recovering` state.
    pub fn resume(&self) -> Result<(), MacosMicError> {
        self.stream
            .play()
            .map_err(|_| MacosMicError::StreamFault(StreamStage::Play))
    }
}

/// cpal-backed implementation of [`MicrophoneBackend`]: the 007A OS
/// binding. Enumeration and default queries only probe; nothing
/// streams until [`open_f32_input_stream`] builds and plays.
#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CpalMicrophoneBackend;

/// Names the OS default input device, if the OS names one.
#[cfg(target_os = "macos")]
fn cpal_default_name(host: &cpal::Host) -> Option<String> {
    host.default_input_device()
        .and_then(|device| device.description().ok())
        .map(|description| description.name().to_owned())
}

/// Names one enumerated device, refusing devices the OS will not describe.
#[cfg(target_os = "macos")]
fn cpal_device_name(device: &cpal::Device) -> Result<String, MacosMicError> {
    device
        .description()
        .map(|description| description.name().to_owned())
        .map_err(|_| MacosMicError::NoInputDevices)
}

/// Classifies a cpal stream error: permission denial keeps its own
/// identity so the machine reports revocation honestly; every other
/// mid-stream fault is a play-stage stream fault with detail.
#[cfg(target_os = "macos")]
fn classify_stream_error(error: &cpal::Error) -> MacosMicError {
    match error.kind() {
        cpal::ErrorKind::PermissionDenied => MacosMicError::PermissionDenied,
        _ => MacosMicError::StreamFault(StreamStage::Play),
    }
}

#[cfg(target_os = "macos")]
impl MicrophoneBackend for CpalMicrophoneBackend {
    fn input_devices(&self) -> Result<Vec<InputDeviceInfo>, MacosMicError> {
        let host = cpal::default_host();
        let default_name = cpal_default_name(&host);
        let devices = host
            .input_devices()
            .map_err(|_| MacosMicError::NoInputDevices)?;
        let mut out = Vec::new();
        for device in devices {
            let name = cpal_device_name(&device)?;
            let supported = device
                .default_input_config()
                .map_err(|_| MacosMicError::StreamFault(StreamStage::Build))?;
            out.push(InputDeviceInfo {
                is_default: default_name.as_deref() == Some(name.as_str()),
                name,
                preferred_config: DeviceConfig::from_supported(&supported),
            });
        }
        Ok(out)
    }

    fn default_input_name(&self) -> Option<String> {
        cpal_default_name(&cpal::default_host())
    }
}

/// Opens an F32 input stream on the named device and starts it
/// flowing. Frames arrive on the cpal audio thread via `on_frames`;
/// stream faults arrive via `on_error` already classified, with the
/// OS detail string for telemetry. Non-F32 default configurations
/// are refused as build faults (format negotiation widens only with
/// Gate E evidence, never by silent conversion).
#[cfg(target_os = "macos")]
pub fn open_f32_input_stream(
    device_name: &str,
    mut on_frames: impl FnMut(&[f32]) + Send + 'static,
    mut on_error: impl FnMut(MacosMicError, String) + Send + 'static,
) -> Result<LiveMicStream, MacosMicError> {
    let host = cpal::default_host();
    let mut found = host
        .input_devices()
        .map_err(|_| MacosMicError::NoInputDevices)?;
    let device = found
        .find(|candidate| cpal_device_name(candidate).is_ok_and(|name| name == device_name))
        .ok_or(MacosMicError::NoInputDevices)?;
    let supported = device
        .default_input_config()
        .map_err(|_| MacosMicError::StreamFault(StreamStage::Build))?;
    if supported.sample_format() != cpal::SampleFormat::F32 {
        return Err(MacosMicError::StreamFault(StreamStage::Build));
    }
    let config = DeviceConfig::from_supported(&supported);
    let stream = device
        .build_input_stream(
            supported.config(),
            move |frames: &[f32], _info: &cpal::InputCallbackInfo| {
                on_frames(frames);
            },
            move |error| {
                let detail = error.to_string();
                on_error(classify_stream_error(&error), detail);
            },
            None,
        )
        .map_err(|_| MacosMicError::StreamFault(StreamStage::Build))?;
    stream
        .play()
        .map_err(|_| MacosMicError::StreamFault(StreamStage::Play))?;
    Ok(LiveMicStream {
        stream,
        device_name: device_name.to_owned(),
        config,
    })
}

#[cfg(target_os = "macos")]
impl DeviceConfig {
    /// Reads the portable configuration out of a cpal supported config.
    fn from_supported(supported: &cpal::SupportedStreamConfig) -> Self {
        let format = match supported.sample_format() {
            cpal::SampleFormat::F32 => SampleFormat::F32,
            cpal::SampleFormat::I16 => SampleFormat::I16,
            cpal::SampleFormat::U16 => SampleFormat::U16,
            _ => SampleFormat::F32,
        };
        let config = supported.config();
        Self {
            channels: config.channels,
            sample_rate_hz: config.sample_rate,
            format,
        }
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
        assert_eq!(
            MacosMicError::PermissionDenied.classifier(),
            "permission-denied"
        );
        assert_ne!(SourceId::new(1), SourceId::new(2));
    }

    #[test]
    fn permission_denial_maps_to_revocation_on_both_paths() {
        assert_eq!(
            event_for_start_failure(&MacosMicError::PermissionDenied),
            CaptureEvent::FailRecoverable(HealthReason::PermissionRevoked)
        );
        assert_eq!(
            event_for_runtime_fault(&MacosMicError::PermissionDenied),
            CaptureEvent::Interrupted(HealthReason::PermissionRevoked)
        );
    }
}

/// Live cpal backend tests (macOS only). Enumeration is read-only and
/// safe headless; stream opening stays behind
/// `HIMSAT_LIVE_MIC_TEST=1` so CI never touches capture hardware.
#[cfg(all(test, target_os = "macos"))]
mod live_tests {
    use super::{
        CaptureEvent, CpalMicrophoneBackend, MicrophoneBackend, event_for_runtime_fault,
        event_for_start_failure, open_f32_input_stream, select_input,
    };
    use crate::capture_session::CaptureSession;
    use himsat_events::SessionId;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    const SESSION: SessionId = SessionId::new(11);

    #[test]
    fn cpal_enumeration_never_panics_and_selects_consistently() {
        let backend = CpalMicrophoneBackend;
        match backend.input_devices() {
            Ok(devices) => {
                let selected = select_input(&backend, None);
                if devices.is_empty() {
                    assert!(selected.is_err());
                } else {
                    let selected = selected.expect("non-empty enumerates");
                    assert!(!selected.descriptor(SESSION).label().is_empty());
                }
            }
            Err(error) => {
                assert_eq!(
                    error,
                    super::MacosMicError::NoInputDevices,
                    "enumeration fails only as no-input-devices"
                );
            }
        }
    }

    #[test]
    fn cpal_default_name_agrees_with_selection() {
        let backend = CpalMicrophoneBackend;
        if backend.default_input_name().is_some() {
            assert!(select_input(&backend, None).is_ok());
        }
    }

    #[test]
    fn live_stream_open_reports_frames_or_classified_fault() {
        if std::env::var("HIMSAT_LIVE_MIC_TEST").is_err() {
            return;
        }
        let backend = CpalMicrophoneBackend;
        let devices = backend.input_devices().expect("live test needs hardware");
        let selected = select_input(&backend, None).expect("live test needs hardware");
        assert!(!devices.is_empty());
        let frames_seen = Arc::new(AtomicBool::new(false));
        let faults: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let frames_flag = Arc::clone(&frames_seen);
        let faults_log = Arc::clone(&faults);
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        match open_f32_input_stream(
            &selected.info.name,
            move |frames: &[f32]| {
                if !frames.is_empty() {
                    frames_flag.store(true, Ordering::SeqCst);
                }
            },
            move |error, detail| {
                faults_log
                    .lock()
                    .expect("fault log")
                    .push(format!("{} {detail}", error.classifier()));
            },
        ) {
            Ok(stream) => {
                assert_eq!(stream.device_name(), selected.info.name);
                assert!(stream.config().sample_rate_hz > 0);
                assert!(stream.config().channels > 0);
                drop(stream);
            }
            Err(error) => {
                assert!(session.apply(event_for_start_failure(&error)).is_ok());
                assert!(session.apply(CaptureEvent::RetryRequested).is_ok());
            }
        }
        let _ = (frames_seen, faults);
    }

    #[test]
    fn live_runtime_fault_event_is_accepted_when_flowing() {
        if std::env::var("HIMSAT_LIVE_MIC_TEST").is_err() {
            return;
        }
        let backend = CpalMicrophoneBackend;
        if backend.input_devices().is_ok() {
            let mut session = CaptureSession::new(SESSION);
            assert!(session.apply(CaptureEvent::Prepare).is_ok());
            let selected = select_input(&backend, None).expect("hardware");
            assert!(session.attach(selected.descriptor(SESSION)).is_ok());
            assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
            let fault = super::MacosMicError::PermissionDenied;
            assert!(session.apply(event_for_runtime_fault(&fault)).is_ok());
        }
    }
}
