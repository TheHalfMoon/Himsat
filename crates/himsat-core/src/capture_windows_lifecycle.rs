//! 008C grain 1 Windows capture lifecycle observation and loss accounting.
//!
//! Portable, hardware-free halves of the Windows lifecycle and
//! evidence-matrix work: the process-suspension and endpoint-change
//! signals a long Windows session must survive, the mapping of those
//! signals onto the closed 006A machine, and the sustained-flow loss
//! account that lets a long session reconcile against wall clock.
//!
//! Everything here is pure data plus one polling watch thread; no OS
//! audio API is touched, so the rows of the 008 evidence matrix that can
//! be proven without a device are proven on every CI host, and the rows
//! that need a real Windows endpoint stay explicitly unclaimed.

use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::capture_session::{CaptureEvent, HealthReason};

/// Minimum watch cadence in milliseconds. The watch never spins:
/// shorter requests clamp to this floor.
pub const LIFECYCLE_WATCH_MIN_CADENCE_MS: u64 = 100;

/// A poll interval longer than this multiple of the cadence counts as
/// suspension. Scheduling jitter stays far below it, while a real
/// suspend/resume overshoots it by orders of magnitude.
pub const LIFECYCLE_SLEEP_GAP_MULTIPLE: u32 = 10;

/// Lifecycle signal observed beneath a running Windows capture session.
/// Portable data: the watch thread reports these over a channel and the
/// owner maps each onto a 006A event. No OS type crosses this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowsLifecycleSignal {
    /// The enumerated endpoint-id set changed: a device connected, was
    /// removed, or the default endpoint was re-enumerated (Bluetooth and
    /// USB endpoints included).
    EndpointsChanged,
    /// The watch slept far longer than its cadence, so the machine was
    /// suspended and has now resumed. Carries the observed gap in whole
    /// seconds.
    WakeNotified(u64),
}

/// Reports whether the enumerated endpoint-id set changed between polls.
/// Order-insensitive; duplicates collapse, so a host that lists the same
/// endpoint twice is not reported as a change. Pure, so the endpoint
/// row of the evidence matrix is unit-testable without hardware.
#[must_use]
pub fn detect_endpoint_delta(before: &[String], after: &[String]) -> bool {
    if before.len() != after.len() {
        return true;
    }
    let mut previous: Vec<&str> = before.iter().map(String::as_str).collect();
    let mut current: Vec<&str> = after.iter().map(String::as_str).collect();
    previous.sort_unstable();
    current.sort_unstable();
    previous != current
}

/// Reports suspension when the observed poll interval overshoots the
/// cadence by [`LIFECYCLE_SLEEP_GAP_MULTIPLE`], returning the gap in
/// whole seconds. A cadence whose threshold would overflow yields
/// `None` rather than a wrapped comparison. Pure, so the wake row is
/// unit-testable without suspending a machine.
#[must_use]
pub fn detect_sleep_gap(cadence: Duration, observed: Duration) -> Option<u64> {
    let threshold = cadence.checked_mul(LIFECYCLE_SLEEP_GAP_MULTIPLE)?;
    if observed >= threshold {
        Some(observed.as_secs())
    } else {
        None
    }
}

/// Classifies a lifecycle signal into the machine event the
/// `RecordingHealthy`, `RecordingDegraded`, or `Interrupted` states
/// accept. An endpoint change is a route change; a wake is a process
/// interruption, because capture was suspended with the machine. The
/// owner drives resume and recovery, never the watch, and only existing
/// 006A events and reasons are used.
#[must_use]
pub const fn event_for_lifecycle_signal(signal: &WindowsLifecycleSignal) -> CaptureEvent {
    match signal {
        WindowsLifecycleSignal::EndpointsChanged => {
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        }
        WindowsLifecycleSignal::WakeNotified(_) => {
            CaptureEvent::Interrupted(HealthReason::ProcessInterrupted)
        }
    }
}

/// Polling lifecycle watch. A background thread lists endpoint ids each
/// cadence and reports [`WindowsLifecycleSignal`] changes plus suspension
/// gaps over the sender. Polling keeps the whole path safe and portable:
/// no device-notification callback and no extra dependency enters the
/// closure. Dropping the watch stops and joins the thread; signals
/// already sent are never retracted.
///
/// The lister returns `None` when it cannot enumerate. A lister fault
/// ends the thread after that poll, never emits a signal, and never
/// spins — the owner observes the ended thread as a stopped watch rather
/// than as a false route change.
pub struct LifecycleWatch {
    stop: Sender<()>,
    thread: Option<JoinHandle<()>>,
}

impl LifecycleWatch {
    /// Starts watching with the given endpoint lister. Cadence clamps to
    /// [`LIFECYCLE_WATCH_MIN_CADENCE_MS`].
    #[must_use]
    pub fn spawn(
        cadence: Duration,
        list_endpoints: impl Fn() -> Option<Vec<String>> + Send + 'static,
        signals: Sender<WindowsLifecycleSignal>,
    ) -> Self {
        let floor = Duration::from_millis(LIFECYCLE_WATCH_MIN_CADENCE_MS);
        let cadence = cadence.max(floor);
        let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
        let thread = std::thread::spawn(move || {
            let Some(mut previous) = list_endpoints() else {
                return;
            };
            let mut last_poll = Instant::now();
            loop {
                if stop_rx.recv_timeout(cadence).is_ok() {
                    return;
                }
                let now = Instant::now();
                if let Some(gap) = detect_sleep_gap(cadence, now - last_poll)
                    && signals
                        .send(WindowsLifecycleSignal::WakeNotified(gap))
                        .is_err()
                {
                    return;
                }
                last_poll = now;
                let Some(current) = list_endpoints() else {
                    return;
                };
                if detect_endpoint_delta(&previous, &current) {
                    previous = current;
                    if signals
                        .send(WindowsLifecycleSignal::EndpointsChanged)
                        .is_err()
                    {
                        return;
                    }
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

/// Drains any pending signals without blocking.
#[must_use]
pub fn drain_signals(receiver: &Receiver<WindowsLifecycleSignal>) -> Vec<WindowsLifecycleSignal> {
    receiver.try_iter().collect()
}

/// Sustained-flow loss account for a running Windows capture stream.
/// Counts callbacks, frames, and stream errors against wall-clock
/// expectation so a long session reconciles to zero unexplained loss.
/// Portable data; the owner feeds it from the stream callbacks. All
/// counters saturate instead of wrapping, so an overflowed account
/// reports saturation rather than a smaller loss than reality, and the
/// short-window under-count stays the caller's problem to avoid by
/// comparing only over settled windows.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StreamLossAccount {
    callbacks: u64,
    frames: u64,
    stream_errors: u32,
    saturated: bool,
}

impl StreamLossAccount {
    /// Records one callback delivering `frames` samples.
    pub fn note_callback(&mut self, frames: usize) {
        let frames = u64::try_from(frames).unwrap_or(u64::MAX);
        match (
            self.callbacks.checked_add(1),
            self.frames.checked_add(frames),
        ) {
            (Some(callbacks), Some(total)) => {
                self.callbacks = callbacks;
                self.frames = total;
            }
            _ => self.saturated = true,
        }
    }

    /// Records one stream error callback.
    pub fn note_stream_error(&mut self) {
        self.stream_errors = self.stream_errors.saturating_add(1);
    }

    /// Frames the wall clock expects at `sample_rate_hz` over `elapsed`.
    /// Saturates instead of wrapping.
    #[must_use]
    pub fn expected_frames(sample_rate_hz: u32, elapsed: Duration) -> u64 {
        u64::from(sample_rate_hz).saturating_mul(elapsed.as_secs())
            + u64::from(sample_rate_hz).saturating_mul(u64::from(elapsed.subsec_millis())) / 1_000
    }

    /// True when every expected frame arrived with no stream error and no
    /// saturation.
    #[must_use]
    pub fn reconciles(&self, expected: u64) -> bool {
        !self.saturated && self.stream_errors == 0 && self.frames >= expected
    }

    /// Reports whether a counter saturated, which makes the account
    /// unusable for a reconciliation claim.
    #[must_use]
    pub const fn saturated(&self) -> bool {
        self.saturated
    }

    /// Callback count observed.
    #[must_use]
    pub const fn callbacks(&self) -> u64 {
        self.callbacks
    }

    /// Frame count observed.
    #[must_use]
    pub const fn frames(&self) -> u64 {
        self.frames
    }

    /// Stream-error count observed.
    #[must_use]
    pub const fn stream_errors(&self) -> u32 {
        self.stream_errors
    }

    /// Frames the account is short of `expected`, or zero when it is not
    /// short. Reported so a long-session evidence row can name the loss
    /// instead of only failing a boolean.
    #[must_use]
    pub const fn unexplained_shortfall(&self, expected: u64) -> u64 {
        expected.saturating_sub(self.frames)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LIFECYCLE_SLEEP_GAP_MULTIPLE, LIFECYCLE_WATCH_MIN_CADENCE_MS, LifecycleWatch,
        StreamLossAccount, WindowsLifecycleSignal, detect_endpoint_delta, detect_sleep_gap,
        drain_signals, event_for_lifecycle_signal,
    };
    use crate::capture_session::{CaptureEvent, CaptureSession, CaptureState, HealthReason};
    use himsat_events::SessionId;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, mpsc};
    use std::time::Duration;

    const SESSION: SessionId = SessionId::new(12);

    fn ids(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    fn flowing_session() -> CaptureSession {
        let backend = crate::capture_windows::MockMicrophoneBackend::with_devices(
            vec![crate::capture_windows::device_info(
                "endpoint-builtin",
                "Microphone (Built-in)",
                true,
                1,
                48_000,
            )],
            Some("endpoint-builtin".to_owned()),
        );
        let selected = crate::capture_windows::select_input(&backend, None).expect("device");
        let mut session = CaptureSession::new(SESSION);
        assert!(session.apply(CaptureEvent::Prepare).is_ok());
        assert!(session.attach(selected.descriptor(SESSION)).is_ok());
        assert!(session.apply(CaptureEvent::SourcesReady).is_ok());
        assert_eq!(session.state(), CaptureState::RecordingHealthy);
        session
    }

    #[test]
    fn endpoint_delta_is_order_insensitive_and_duplicate_insensitive() {
        assert!(!detect_endpoint_delta(&ids(&["a", "b"]), &ids(&["b", "a"])));
        assert!(!detect_endpoint_delta(&ids(&["a", "a"]), &ids(&["a"])));
        assert!(detect_endpoint_delta(&ids(&["a", "b"]), &ids(&["a"])));
        assert!(detect_endpoint_delta(&ids(&["a"]), &ids(&["a", "b"])));
        assert!(!detect_endpoint_delta(&[], &[]));
    }

    #[test]
    fn sleep_gap_detection_uses_the_cadence_multiple() {
        let cadence = Duration::from_millis(LIFECYCLE_WATCH_MIN_CADENCE_MS);
        assert_eq!(detect_sleep_gap(cadence, Duration::from_millis(150)), None);
        let threshold = cadence * LIFECYCLE_SLEEP_GAP_MULTIPLE;
        assert_eq!(detect_sleep_gap(cadence, threshold), Some(1));
        assert_eq!(detect_sleep_gap(cadence, Duration::from_secs(90)), Some(90));
        assert_eq!(detect_sleep_gap(Duration::MAX, Duration::MAX), None);
    }

    #[test]
    fn lifecycle_signals_map_to_events_the_flowing_states_accept() {
        for signal in [
            WindowsLifecycleSignal::EndpointsChanged,
            WindowsLifecycleSignal::WakeNotified(90),
        ] {
            let mut session = flowing_session();
            assert!(
                session.apply(event_for_lifecycle_signal(&signal)).is_ok(),
                "lifecycle mapping refused for {signal:?}"
            );
        }
        assert_eq!(
            event_for_lifecycle_signal(&WindowsLifecycleSignal::EndpointsChanged),
            CaptureEvent::Interrupted(HealthReason::RouteChanged)
        );
        assert_eq!(
            event_for_lifecycle_signal(&WindowsLifecycleSignal::WakeNotified(5)),
            CaptureEvent::Interrupted(HealthReason::ProcessInterrupted)
        );
    }

    #[test]
    fn watch_reports_endpoint_changes_and_stops_on_drop() {
        let polls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&polls);
        let (tx, rx) = mpsc::channel();
        let watch = LifecycleWatch::spawn(
            Duration::from_millis(LIFECYCLE_WATCH_MIN_CADENCE_MS),
            move || {
                let seen = counter.fetch_add(1, Ordering::SeqCst);
                // The second poll reports a different endpoint set.
                Some(if seen == 0 {
                    vec!["endpoint-a".to_owned()]
                } else {
                    vec!["endpoint-a".to_owned(), "endpoint-b".to_owned()]
                })
            },
            tx,
        );
        let signal = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("watch reports the endpoint change");
        assert_eq!(signal, WindowsLifecycleSignal::EndpointsChanged);
        drop(watch);
        assert!(polls.load(Ordering::SeqCst) >= 2);
    }

    #[test]
    fn watch_ends_quietly_when_the_lister_fails() {
        let polls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&polls);
        let (tx, rx) = mpsc::channel();
        let watch = LifecycleWatch::spawn(
            Duration::from_millis(LIFECYCLE_WATCH_MIN_CADENCE_MS),
            move || {
                counter.fetch_add(1, Ordering::SeqCst);
                None
            },
            tx,
        );
        assert!(drain_signals(&rx).is_empty());
        drop(watch);
        assert_eq!(polls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn loss_account_reconciles_exact_flow_and_names_the_shortfall() {
        assert_eq!(
            StreamLossAccount::expected_frames(48_000, Duration::from_secs(2)),
            96_000
        );
        assert_eq!(
            StreamLossAccount::expected_frames(48_000, Duration::from_millis(1500)),
            72_000
        );
        let mut account = StreamLossAccount::default();
        account.note_callback(512);
        account.note_callback(512);
        assert_eq!(account.callbacks(), 2);
        assert_eq!(account.frames(), 1_024);
        assert_eq!(account.stream_errors(), 0);
        assert!(account.reconciles(1_024));
        assert!(!account.reconciles(1_025));
        assert_eq!(account.unexplained_shortfall(1_024), 0);
        assert_eq!(account.unexplained_shortfall(2_000), 976);
        account.note_stream_error();
        assert_eq!(account.stream_errors(), 1);
        assert!(!account.reconciles(1_024));
    }

    #[test]
    fn loss_account_saturates_instead_of_wrapping() {
        let mut account = StreamLossAccount::default();
        account.note_callback(usize::MAX);
        account.note_callback(usize::MAX);
        assert!(account.saturated());
        assert!(!account.reconciles(0));
        assert_eq!(account.unexplained_shortfall(0), 0);
        let mut errors = StreamLossAccount::default();
        errors.note_stream_error();
        errors.note_stream_error();
        assert_eq!(errors.stream_errors(), 2);
        assert!(!errors.reconciles(0));
    }

    #[test]
    fn loss_account_semantics_match_the_closed_macos_implementation() {
        use crate::capture_system_audio::StreamLossAccount as MacosLossAccount;
        let mut windows = StreamLossAccount::default();
        let mut macos = MacosLossAccount::default();
        for frames in [0_usize, 128, 512, 4096] {
            windows.note_callback(frames);
            macos.note_callback(frames);
        }
        windows.note_stream_error();
        macos.note_stream_error();
        assert_eq!(windows.callbacks(), macos.callbacks());
        assert_eq!(windows.frames(), macos.frames());
        assert_eq!(windows.stream_errors(), macos.stream_errors());
        for expected in [0_u64, 4_736, 4_737, 10_000] {
            assert_eq!(windows.reconciles(expected), macos.reconciles(expected));
        }
        for rate in [8_000_u32, 44_100, 48_000] {
            for elapsed in [
                Duration::ZERO,
                Duration::from_millis(1_500),
                Duration::from_secs(2),
            ] {
                assert_eq!(
                    StreamLossAccount::expected_frames(rate, elapsed),
                    MacosLossAccount::expected_frames(rate, elapsed)
                );
            }
        }
    }
}
