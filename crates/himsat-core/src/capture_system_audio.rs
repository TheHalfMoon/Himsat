//! 007B system-audio tap adapter: portable selection, identity,
//! and fault classification over the closed cidre/cpal OS binding
//! (process tap plus private aggregate device, donor-proven
//! shape). The binding is the only OS surface; this adapter
//! drives the closed 006A session machine without ever producing
//! a refused transition. Tap authorization keeps its own identity
//! end to end: denial is reported as revocation, never folded
//! into a generic fault and never silent. Streaming crosses no
//! FFI sample boundary in Himsat code: tap -> aggregate ->
//! cpal keeps every unsafe sample crossing inside the closed
//! bindings.

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
    /// The tap exists but the aggregate device would not assemble
    /// or publish (007C-1).
    Assemble,
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
            Self::TapFault(TapStage::Assemble) => "tap-assemble-fault",
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

/// Backend contract the OS binding implements: route
/// enumeration. Tap creation and streaming live beside the
/// backend as free functions so the handle types match the
/// proven binding shape instead of a guess.
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

/// cpal trait imports for route enumeration (macOS only).
#[cfg(target_os = "macos")]
use cpal::traits::{DeviceTrait, HostTrait};

/// Process-tap description read from a live tap: stable uid plus the
/// tap stream format. Created by [`probe_process_tap`]; dropping the
/// underlying guard destroys the tap, so this snapshot is data only.
#[cfg(target_os = "macos")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TapDescription {
    /// OS tap uid (opaque, non-reversible handle string).
    pub uid: String,
    /// Tap stream configuration.
    pub config: DeviceConfig,
}

/// cidre-backed implementation of [`SystemTapBackend`]: the 007B OS
/// binding. Route enumeration reuses the adopted cpal host (output
/// routes); tap lifecycle uses cidre process taps; streaming uses
/// [`open_aggregate_tap_stream`] (tap -> private aggregate ->
/// cpal), which keeps every unsafe sample crossing inside the
/// closed bindings.
#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CidreSystemTapBackend;

#[cfg(target_os = "macos")]
impl SystemTapBackend for CidreSystemTapBackend {
    fn tap_targets(&self) -> Result<Vec<TapTargetInfo>, SystemTapError> {
        let host = cpal::default_host();
        let default_name = host
            .default_output_device()
            .and_then(|device| device.description().ok())
            .map(|description| description.name().to_owned());
        let devices = host
            .output_devices()
            .map_err(|_| SystemTapError::NoTapAvailable)?;
        let mut out = Vec::new();
        for device in devices {
            let name = device
                .description()
                .map(|description| description.name().to_owned())
                .map_err(|_| SystemTapError::NoTapAvailable)?;
            let supported = device
                .default_output_config()
                .map_err(|_| SystemTapError::TapFault(TapStage::Create))?;
            out.push(TapTargetInfo {
                is_default: default_name.as_deref() == Some(name.as_str()),
                name,
                preferred_config: DeviceConfig::from_supported(&supported),
            });
        }
        Ok(out)
    }
}

/// Creates a mono global process tap, snapshots its uid and stream
/// format, then destroys the tap by dropping the guard. Proves the
/// authorization boundary live: without Screen Recording policy the
/// OS refuses creation and the refusal classifies. Returns the OS
/// detail string alongside faults for telemetry; success carries no
/// audio and starts no stream.
#[cfg(target_os = "macos")]
pub fn probe_process_tap() -> Result<TapDescription, (SystemTapError, String)> {
    let excluded = cidre::ns::Array::new();
    let desc = cidre::core_audio::TapDesc::with_mono_global_tap_excluding_processes(&excluded);
    let tap = desc.create_process_tap().map_err(|status| {
        (
            SystemTapError::TapFault(TapStage::Create),
            status_string(status),
        )
    })?;
    let uid = tap.uid().map(|uid| uid.to_string()).map_err(|status| {
        (
            SystemTapError::TapFault(TapStage::Create),
            status_string(status),
        )
    })?;
    let asbd = tap.asbd().map_err(|status| {
        (
            SystemTapError::TapFault(TapStage::Create),
            status_string(status),
        )
    })?;
    let config = config_from_asbd(&asbd)
        .map_err(|detail| (SystemTapError::TapFault(TapStage::Create), detail))?;
    Ok(TapDescription { uid, config })
}

/// Renders an OS status for telemetry detail.
#[cfg(target_os = "macos")]
fn status_string(status: cidre::os::Error) -> String {
    format!("{status:?}")
}

/// Aggregate device uid published for the streaming tap. Private
/// to this process and destroyed with the stream; a fixed uid
/// faults honestly on a second concurrent open instead of
/// shadowing it.
#[cfg(target_os = "macos")]
const AGGREGATE_DEVICE_UID: &str = "com.himsat.system-tap-aggregate";

/// Aggregate device name the OS publishes and cpal enumerates.
#[cfg(target_os = "macos")]
const AGGREGATE_DEVICE_NAME: &str = "Himsat System Tap";

/// Bounded wait for asynchronous aggregate publication: the OS
/// publishes the new device after creation returns, so cpal
/// visibility is polled, never assumed and never awaited
/// without a deadline.
#[cfg(target_os = "macos")]
const AGGREGATE_VISIBILITY_POLLS: u32 = 100;
/// Poll step for aggregate visibility, in milliseconds.
#[cfg(target_os = "macos")]
const AGGREGATE_VISIBILITY_STEP_MS: u64 = 50;

/// Live tap-to-cpal stream (007C-1). Fields drop in declaration
/// order — stream, aggregate, tap — so teardown reverses
/// assembly: the cpal stream stops before its device is
/// destroyed, and the device is destroyed before its tap.
#[cfg(target_os = "macos")]
pub struct LiveSystemTapStream {
    _stream: cpal::Stream,
    _aggregate: cidre::core_audio::AggregateDevice,
    _tap: cidre::core_audio::TapGuard,
    /// Aggregate device name cpal enumerated.
    pub device_name: String,
    /// Running stream configuration.
    pub config: DeviceConfig,
}

/// Encodes one tap-list entry pointing at a tap uid. Pure over
/// its input so the encoding is unit-testable without touching
/// the OS.
#[cfg(target_os = "macos")]
fn tap_list_entry(
    tap_uid: &cidre::cf::String,
) -> cidre::arc::R<cidre::cf::DictionaryOf<cidre::cf::String, cidre::cf::Type>> {
    use cidre::core_audio::aggregate_device_keys as keys;
    cidre::cf::DictionaryOf::with_keys_values(&[keys::uid()], &[tap_uid.as_type_ref()])
}

/// Builds the private aggregate composition for a tap uid: our
/// uid and name, process-private flag, and the single-tap list.
/// Pure over its input so the composition is unit-testable
/// without touching the OS.
#[cfg(target_os = "macos")]
fn aggregate_composition(
    tap_uid: &cidre::cf::String,
) -> cidre::arc::R<cidre::cf::DictionaryOf<cidre::cf::String, cidre::cf::Type>> {
    use cidre::core_audio::aggregate_device_keys as keys;
    let uid = cidre::cf::String::from_str(AGGREGATE_DEVICE_UID);
    let name = cidre::cf::String::from_str(AGGREGATE_DEVICE_NAME);
    let one = cidre::cf::Number::from_i32(1);
    let tap_entry = tap_list_entry(tap_uid);
    let taps =
        cidre::cf::ArrayOf::<cidre::cf::Type>::from_slice(&[tap_entry.as_ref().as_type_ref()]);
    cidre::cf::DictionaryOf::with_keys_values(
        &[
            keys::uid(),
            keys::name(),
            keys::is_private(),
            keys::tap_list(),
        ],
        &[
            uid.as_type_ref(),
            name.as_type_ref(),
            one.as_type_ref(),
            taps.as_type_ref(),
        ],
    )
}

/// Opens a live system-tap stream: mono global tap, private
/// aggregate device containing it, then a cpal F32 input stream
/// on the aggregate. The sanctioned sample path is tap ->
/// aggregate -> cpal: every FFI sample crossing stays inside
/// the closed cidre/cpal bindings, so this function contains no
/// unsafe block. Aggregate publication is asynchronous, hence
/// the bounded visibility poll; expiry is an assemble-stage
/// fault with detail, never a hang and never silent.
#[cfg(target_os = "macos")]
pub fn open_aggregate_tap_stream(
    mut on_frames: impl FnMut(&[f32]) + Send + 'static,
    mut on_error: impl FnMut(SystemTapError, String) + Send + 'static,
) -> Result<LiveSystemTapStream, (SystemTapError, String)> {
    let excluded = cidre::ns::Array::new();
    let desc = cidre::core_audio::TapDesc::with_mono_global_tap_excluding_processes(&excluded);
    let tap = desc.create_process_tap().map_err(|status| {
        (
            SystemTapError::TapFault(TapStage::Create),
            status_string(status),
        )
    })?;
    let tap_uid = tap.uid().map_err(|status| {
        (
            SystemTapError::TapFault(TapStage::Create),
            status_string(status),
        )
    })?;
    let composition = aggregate_composition(&tap_uid);
    let aggregate =
        cidre::core_audio::AggregateDevice::with_desc(&composition).map_err(|status| {
            (
                SystemTapError::TapFault(TapStage::Assemble),
                status_string(status),
            )
        })?;
    let host = cpal::default_host();
    let mut visible = false;
    for _ in 0..AGGREGATE_VISIBILITY_POLLS {
        let found = host.input_devices().map_err(|_| {
            (
                SystemTapError::TapFault(TapStage::Assemble),
                "aggregate visibility poll lost the device list".to_owned(),
            )
        })?;
        visible = found
            .filter_map(|device| device.description().ok())
            .any(|description| description.name().contains(AGGREGATE_DEVICE_NAME));
        if visible {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(
            AGGREGATE_VISIBILITY_STEP_MS,
        ));
    }
    if !visible {
        return Err((
            SystemTapError::TapFault(TapStage::Assemble),
            "private aggregate device never published to cpal".to_owned(),
        ));
    }
    let device = host
        .input_devices()
        .map_err(|_| {
            (
                SystemTapError::TapFault(TapStage::Assemble),
                "aggregate visibility poll lost the device list".to_owned(),
            )
        })?
        .find(|candidate| {
            candidate
                .description()
                .is_ok_and(|description| description.name().contains(AGGREGATE_DEVICE_NAME))
        })
        .ok_or_else(|| {
            (
                SystemTapError::TapFault(TapStage::Assemble),
                "private aggregate device unpublished between poll and open".to_owned(),
            )
        })?;
    let supported = device.default_input_config().map_err(|_| {
        (
            SystemTapError::TapFault(TapStage::Start),
            "aggregate device reports no input configuration".to_owned(),
        )
    })?;
    if supported.sample_format() != cpal::SampleFormat::F32 {
        return Err((
            SystemTapError::TapFault(TapStage::Start),
            "aggregate device is not F32".to_owned(),
        ));
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
                on_error(SystemTapError::TapFault(TapStage::Start), detail);
            },
            None,
        )
        .map_err(|_| {
            (
                SystemTapError::TapFault(TapStage::Start),
                "aggregate input stream would not build".to_owned(),
            )
        })?;
    {
        use cpal::traits::StreamTrait;
        stream.play().map_err(|_| {
            (
                SystemTapError::TapFault(TapStage::Start),
                "aggregate input stream would not start".to_owned(),
            )
        })?;
    }
    Ok(LiveSystemTapStream {
        _stream: stream,
        _aggregate: aggregate,
        _tap: tap,
        device_name: AGGREGATE_DEVICE_NAME.to_owned(),
        config,
    })
}

/// Reads a portable device configuration out of a tap stream
/// description. Only native-endian interleaved 32-bit float PCM is
/// accepted; anything else is a create-stage fault with detail, never
/// a silent conversion.
#[cfg(target_os = "macos")]
fn config_from_asbd(asbd: &cidre::cat::AudioStreamBasicDesc) -> Result<DeviceConfig, String> {
    use cidre::cat::{AudioFormat as Format, AudioFormatFlags as FormatFlags};
    if asbd.format != Format::LINEAR_PCM {
        return Err(format!("unsupported tap format: {:?}", asbd.format));
    }
    if asbd.format_flags.0 & FormatFlags::IS_FLOAT.0 == 0 {
        return Err("tap stream is not floating-point".to_owned());
    }
    if asbd.bits_per_channel != 32 {
        return Err(format!(
            "unsupported tap word size: {}",
            asbd.bits_per_channel
        ));
    }
    let channels = u16::try_from(asbd.channels_per_frame)
        .map_err(|_| "tap channel count out of range".to_owned())?;
    if channels == 0 {
        return Err("tap reports zero channels".to_owned());
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let sample_rate_hz = asbd.sample_rate as u32;
    if sample_rate_hz == 0 {
        return Err("tap reports zero sample rate".to_owned());
    }
    Ok(DeviceConfig {
        channels,
        sample_rate_hz,
        format: crate::capture_macos::SampleFormat::F32,
    })
}

/// Lifecycle signal observed beneath a running tap session
/// (007C-2). Portable data: the watch thread reports these over
/// a channel and the owner maps each to a session event. No OS
/// type crosses this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TapLifecycleSignal {
    /// The enumerated device-name set changed (route change,
    /// device yank, aggregate unpublished).
    DevicesChanged,
    /// The watch thread slept far longer than its cadence: the
    /// process was suspended (sleep) and has now resumed. Carries
    /// the observed gap in whole seconds.
    WakeNotified(u64),
}

/// Minimum watch cadence in milliseconds. The watch never spins:
/// shorter requests clamp to this floor.
pub const LIFECYCLE_WATCH_MIN_CADENCE_MS: u64 = 100;

/// A gap counts as suspension when the observed poll interval
/// exceeds this multiple of the cadence. Scheduling jitter stays
/// far below it; real sleep exceeds it by orders of magnitude.
pub const LIFECYCLE_SLEEP_GAP_MULTIPLE: u32 = 10;

/// Reports whether the enumerated device-name set changed
/// between polls. Order-insensitive; duplicates collapse. Pure
/// so the matrix row is unit-testable without hardware.
#[must_use]
pub fn detect_device_delta(before: &[String], after: &[String]) -> bool {
    if before.len() != after.len() {
        return true;
    }
    let mut previous: Vec<&str> = before.iter().map(String::as_str).collect();
    let mut current: Vec<&str> = after.iter().map(String::as_str).collect();
    previous.sort_unstable();
    current.sort_unstable();
    previous != current
}

/// Reports suspension when the observed poll interval overshoots
/// the cadence by the sleep-gap multiple, returning the gap in
/// whole seconds. Pure so the wake row is unit-testable without
/// sleeping a machine.
#[must_use]
pub fn detect_sleep_gap(
    cadence: std::time::Duration,
    observed: std::time::Duration,
) -> Option<u64> {
    let threshold = cadence.checked_mul(LIFECYCLE_SLEEP_GAP_MULTIPLE)?;
    if observed >= threshold {
        Some(observed.as_secs())
    } else {
        None
    }
}

/// Classifies a lifecycle signal into the machine event the
/// `RecordingHealthy`, `RecordingDegraded`, or `Interrupted`
/// states accept. Wake maps to interruption (the process was
/// suspended); the owner drives resume and recovery, never the
/// watch. The closed 006 contract is untouched: only existing
/// events and reasons are used.
#[must_use]
pub const fn event_for_lifecycle_signal(signal: &TapLifecycleSignal) -> CaptureEvent {
    match signal {
        TapLifecycleSignal::DevicesChanged => CaptureEvent::Interrupted(HealthReason::RouteChanged),
        TapLifecycleSignal::WakeNotified(_) => {
            CaptureEvent::Interrupted(HealthReason::ProcessInterrupted)
        }
    }
}

/// Polling lifecycle watch (007C-2). A background thread lists
/// device names each cadence and reports [`TapLifecycleSignal`]
/// deltas plus suspension gaps over the sender. Polling keeps
/// the whole path safe and portable: no property-listener
/// callback (which would need an unsafe client-data
/// dereference) and no block-observer feature (which would
/// change the closed dependency closure). Dropping the watch
/// stops and joins the thread; signals already sent are never
/// retracted.
pub struct LifecycleWatch {
    stop: std::sync::mpsc::Sender<()>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl LifecycleWatch {
    /// Starts watching with the given device lister. Cadence
    /// clamps to [`LIFECYCLE_WATCH_MIN_CADENCE_MS`]; a lister
    /// fault ends the thread after one final poll attempt, never
    /// a signal and never a spin.
    pub fn spawn(
        cadence: std::time::Duration,
        list_devices: impl Fn() -> Result<Vec<String>, SystemTapError> + Send + 'static,
        signals: std::sync::mpsc::Sender<TapLifecycleSignal>,
    ) -> Self {
        let floor = std::time::Duration::from_millis(LIFECYCLE_WATCH_MIN_CADENCE_MS);
        let cadence = cadence.max(floor);
        let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
        let thread = std::thread::spawn(move || {
            let mut previous = match list_devices() {
                Ok(names) => names,
                Err(_) => return,
            };
            let mut last_poll = std::time::Instant::now();
            loop {
                if stop_rx.recv_timeout(cadence).is_ok() {
                    return;
                }
                let now = std::time::Instant::now();
                let gap = detect_sleep_gap(cadence, now - last_poll);
                if gap
                    .is_some_and(|gap| signals.send(TapLifecycleSignal::WakeNotified(gap)).is_err())
                {
                    return;
                }
                last_poll = now;
                match list_devices() {
                    Ok(names) => {
                        if detect_device_delta(&previous, &names) {
                            previous = names;
                            if signals.send(TapLifecycleSignal::DevicesChanged).is_err() {
                                return;
                            }
                        }
                    }
                    Err(_) => return,
                }
            }
        });
        Self {
            stop: stop_tx,
            thread: Some(thread),
        }
    }
}

impl Drop for LifecycleWatch {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// cpal device-name lister for the production watch (macOS).
/// Unusable routes are skipped, never faulted: a momentarily
/// undescribable device is not a route change.
#[cfg(target_os = "macos")]
pub fn cpal_input_names() -> Result<Vec<String>, SystemTapError> {
    let host = cpal::default_host();
    let devices = host
        .input_devices()
        .map_err(|_| SystemTapError::NoTapAvailable)?;
    Ok(devices
        .filter_map(|device| device.description().ok())
        .map(|description| description.name().to_owned())
        .collect())
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
    fn tap_target_labels_sanitize_like_microphone_labels() {
        use crate::capture_macos::sanitize_label;
        assert_eq!(sanitize_label("  System Output  "), "System Output");
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
            SystemTapError::TapFault(TapStage::Assemble).classifier(),
            "tap-assemble-fault"
        );
        assert_eq!(
            SystemTapError::AuthorizationDenied.classifier(),
            "tap-authorization-denied"
        );
    }

    #[test]
    fn device_delta_detects_set_changes_order_insensitively() {
        let names = |list: &[&str]| list.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert!(!super::detect_device_delta(&names(&[]), &names(&[])));
        assert!(!super::detect_device_delta(
            &names(&["mic", "tap"]),
            &names(&["tap", "mic"])
        ));
        assert!(super::detect_device_delta(
            &names(&["mic"]),
            &names(&["mic", "tap"])
        ));
        assert!(super::detect_device_delta(
            &names(&["mic", "tap"]),
            &names(&["mic"])
        ));
        assert!(super::detect_device_delta(
            &names(&["mic"]),
            &names(&["speakers"])
        ));
    }

    #[test]
    fn sleep_gap_needs_tenfold_overshoot() {
        use std::time::Duration;
        assert_eq!(
            super::detect_sleep_gap(Duration::from_secs(1), Duration::from_millis(1500)),
            None
        );
        assert_eq!(
            super::detect_sleep_gap(Duration::from_secs(1), Duration::from_secs(10)),
            Some(10)
        );
        assert_eq!(
            super::detect_sleep_gap(Duration::from_secs(2), Duration::from_secs(3600)),
            Some(3600)
        );
    }

    #[test]
    fn lifecycle_signals_map_to_accepted_events() {
        assert_eq!(
            super::event_for_lifecycle_signal(&super::TapLifecycleSignal::DevicesChanged),
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        );
        assert_eq!(
            super::event_for_lifecycle_signal(&super::TapLifecycleSignal::WakeNotified(42)),
            CaptureEvent::Interrupted(HealthReason::ProcessInterrupted)
        );
    }

    #[test]
    fn lifecycle_signals_drive_session_without_refusal() {
        let backend = two_target_backend();
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        let selected = select_system_input(&backend, None).expect("target");
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
        assert!(
            session
                .apply(super::event_for_lifecycle_signal(
                    &super::TapLifecycleSignal::DevicesChanged
                ))
                .is_ok()
        );
        assert!(session.apply(CaptureEvent::ResumeRequested).is_ok());
        assert!(session.apply(CaptureEvent::RecoveryConfirmed).is_ok());
        assert!(
            session
                .apply(super::event_for_lifecycle_signal(
                    &super::TapLifecycleSignal::WakeNotified(300)
                ))
                .is_ok()
        );
        assert!(session.apply(CaptureEvent::ResumeRequested).is_ok());
        assert!(session.apply(CaptureEvent::RecoveryConfirmed).is_ok());
    }

    #[test]
    fn lifecycle_watch_reports_scripted_delta_then_stops() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        let calls = Arc::new(AtomicUsize::new(0));
        let lister = {
            let calls = Arc::clone(&calls);
            move || {
                let n = calls.fetch_add(1, Ordering::SeqCst);
                if n == 0 {
                    Ok(vec!["mic".to_owned()])
                } else {
                    Ok(vec!["mic".to_owned(), "agg".to_owned()])
                }
            }
        };
        let (tx, rx) = std::sync::mpsc::channel();
        let watch = super::LifecycleWatch::spawn(std::time::Duration::from_millis(100), lister, tx);
        let signal = rx
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("scripted device delta surfaces");
        assert_eq!(signal, super::TapLifecycleSignal::DevicesChanged);
        drop(watch);
    }

    #[test]
    fn lifecycle_watch_stays_quiet_on_stable_routes() {
        let (tx, rx) = std::sync::mpsc::channel();
        let watch = super::LifecycleWatch::spawn(
            std::time::Duration::from_millis(100),
            || Ok(vec!["mic".to_owned()]),
            tx,
        );
        std::thread::sleep(std::time::Duration::from_millis(350));
        assert!(rx.try_recv().is_err());
        drop(watch);
    }

    #[test]
    fn assemble_faults_classify_as_route_events_without_detail_loss() {
        let error = SystemTapError::TapFault(TapStage::Assemble);
        assert!(matches!(
            event_for_tap_start_failure(&error),
            CaptureEvent::FailRecoverable(_)
        ));
        assert!(matches!(
            event_for_tap_runtime_fault(&error),
            CaptureEvent::Interrupted(_)
        ));
    }

    /// Composition is pure: the private aggregate dictionary
    /// carries our uid, name, process-private flag, and tap list,
    /// and the tap entry round-trips exactly the given tap uid,
    /// all without touching the OS and without unsafe code.
    #[cfg(target_os = "macos")]
    #[test]
    fn aggregate_composition_pins_private_tap_shape() {
        use cidre::core_audio::aggregate_device_keys as keys;
        let tap_uid = cidre::cf::String::from_str("tap-uid-under-test");
        let entry = super::tap_list_entry(&tap_uid);
        let round_tripped = entry.get(keys::uid()).expect("tap entry uid");
        assert!(round_tripped.equal(tap_uid.as_type_ref()));
        let composition = super::aggregate_composition(&tap_uid);
        let expected_uid = cidre::cf::String::from_str(super::AGGREGATE_DEVICE_UID);
        let uid = composition.get(keys::uid()).expect("aggregate uid entry");
        assert!(uid.equal(expected_uid.as_type_ref()));
        let expected_name = cidre::cf::String::from_str(super::AGGREGATE_DEVICE_NAME);
        let name = composition.get(keys::name()).expect("aggregate name entry");
        assert!(name.equal(expected_name.as_type_ref()));
        assert!(composition.get(keys::is_private()).is_some());
        assert!(composition.get(keys::tap_list()).is_some());
    }
}

/// Live cidre binding tests (macOS only). Route enumeration is
/// read-only and safe headless; tap creation stays behind
/// `HIMSAT_LIVE_TAP_TEST=1` so CI never touches the tap policy
/// surface without an explicit opt-in.
#[cfg(all(test, target_os = "macos"))]
mod live_tests {
    use super::{
        CaptureEvent, CidreSystemTapBackend, SystemTapBackend, SystemTapError, config_from_asbd,
        event_for_tap_runtime_fault, event_for_tap_start_failure, probe_process_tap,
        select_system_input,
    };
    use crate::capture_macos::SampleFormat;
    use crate::capture_session::CaptureSession;
    use cidre::cat::{AudioFormat, AudioFormatFlags, AudioStreamBasicDesc};
    use himsat_events::SessionId;

    const SESSION: SessionId = SessionId::new(22);

    fn f32_stereo_48k() -> AudioStreamBasicDesc {
        AudioStreamBasicDesc {
            sample_rate: 48_000.0,
            format: AudioFormat::LINEAR_PCM,
            format_flags: AudioFormatFlags::IS_FLOAT,
            bytes_per_packet: 8,
            frames_per_packet: 1,
            bytes_per_frame: 8,
            channels_per_frame: 2,
            bits_per_channel: 32,
            reserved: 0,
        }
    }

    #[test]
    fn tap_format_mapping_accepts_f32_and_refuses_the_rest() {
        let config = config_from_asbd(&f32_stereo_48k()).expect("f32 stereo");
        assert_eq!(config.channels, 2);
        assert_eq!(config.sample_rate_hz, 48_000);
        assert_eq!(config.format, SampleFormat::F32);

        let mut integer = f32_stereo_48k();
        integer.format_flags = AudioFormatFlags(0);
        assert!(config_from_asbd(&integer).is_err());

        let mut narrow = f32_stereo_48k();
        narrow.bits_per_channel = 16;
        assert!(config_from_asbd(&narrow).is_err());

        let mut silent = f32_stereo_48k();
        silent.channels_per_frame = 0;
        assert!(config_from_asbd(&silent).is_err());

        let mut flat = f32_stereo_48k();
        flat.sample_rate = 0.0;
        assert!(config_from_asbd(&flat).is_err());
    }

    #[test]
    fn cidre_route_enumeration_never_panics_and_selects_consistently() {
        let backend = CidreSystemTapBackend;
        match backend.tap_targets() {
            Ok(targets) => {
                let selected = select_system_input(&backend, None);
                if targets.is_empty() {
                    assert!(selected.is_err());
                } else {
                    let selected = selected.expect("non-empty enumerates");
                    assert!(!selected.descriptor(SESSION).label().is_empty());
                }
            }
            Err(error) => {
                assert_eq!(error, SystemTapError::NoTapAvailable);
            }
        }
    }

    #[test]
    fn live_tap_probe_reports_description_or_classified_fault() {
        if std::env::var("HIMSAT_LIVE_TAP_TEST").is_err() {
            return;
        }
        let mut session = CaptureSession::new(SESSION);
        match probe_process_tap() {
            Ok(description) => {
                assert!(!description.uid.is_empty());
                assert!(description.config.channels > 0);
                assert!(description.config.sample_rate_hz > 0);
            }
            Err((error, detail)) => {
                assert!(!detail.is_empty());
                assert!(session.apply(CaptureEvent::Prepare).is_ok());
                assert!(session.apply(event_for_tap_start_failure(&error)).is_ok());
                assert!(session.apply(CaptureEvent::RetryRequested).is_ok());
            }
        }
    }

    #[test]
    fn live_aggregate_tap_stream_flows_or_classifies() {
        if std::env::var("HIMSAT_LIVE_TAP_TEST").is_err() {
            return;
        }
        let (frames_tx, frames_rx) = std::sync::mpsc::channel::<usize>();
        match super::open_aggregate_tap_stream(
            move |frames| {
                let _ = frames_tx.send(frames.len());
            },
            move |_error, _detail| {},
        ) {
            Ok(stream) => {
                assert_eq!(stream.device_name, "Himsat System Tap");
                assert!(stream.config.channels > 0);
                assert!(stream.config.sample_rate_hz > 0);
                let deadline = std::time::Instant::now() + std::time::Duration::from_millis(1500);
                let mut callbacks = 0_u32;
                let mut frames = 0_usize;
                while std::time::Instant::now() < deadline {
                    match frames_rx.recv_timeout(std::time::Duration::from_millis(300)) {
                        Ok(len) => {
                            callbacks += 1;
                            frames += len;
                        }
                        Err(_) => break,
                    }
                }
                assert!(callbacks > 0, "aggregate stream delivered no callbacks");
                assert!(frames > 0, "aggregate stream delivered no frames");
                drop(stream);
            }
            Err((error, detail)) => {
                assert!(!detail.is_empty());
                let mut session = CaptureSession::new(SESSION);
                assert!(session.apply(CaptureEvent::Prepare).is_ok());
                assert!(session.apply(event_for_tap_start_failure(&error)).is_ok());
                assert!(session.apply(CaptureEvent::RetryRequested).is_ok());
            }
        }
    }

    #[test]
    fn live_lifecycle_watch_runs_against_real_routes() {
        if std::env::var("HIMSAT_LIVE_TAP_TEST").is_err() {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let watch = super::LifecycleWatch::spawn(
            std::time::Duration::from_millis(100),
            super::cpal_input_names,
            tx,
        );
        std::thread::sleep(std::time::Duration::from_millis(400));
        drop(watch);
        let mut drained = 0_u32;
        while rx.try_recv().is_ok() {
            drained += 1;
        }
        assert!(drained < 100, "watch spammed signals on stable routes");
    }

    #[test]
    fn live_tap_runtime_fault_event_is_accepted_when_flowing() {
        if std::env::var("HIMSAT_LIVE_TAP_TEST").is_err() {
            return;
        }
        let backend = CidreSystemTapBackend;
        if backend.tap_targets().is_ok() {
            let mut session = CaptureSession::new(SESSION);
            assert!(session.apply(CaptureEvent::Prepare).is_ok());
            let selected = select_system_input(&backend, None).expect("routes");
            assert!(session.attach(selected.descriptor(SESSION)).is_ok());
            assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
            let fault = SystemTapError::AuthorizationDenied;
            assert!(session.apply(event_for_tap_runtime_fault(&fault)).is_ok());
        }
    }
}
