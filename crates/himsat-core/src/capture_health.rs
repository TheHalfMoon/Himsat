//! 006B health telemetry model and change events.
//!
//! Clock-free portable contract over the 006A session machine: adapters
//! measure signals on their own clocks and hand caller-stamped samples
//! to [`HealthMonitor`], which digests them into UI-ready snapshots and
//! edge-triggered [`HealthEvent`]s with 003 sequence order. Level values
//! are recorded but never emit (adapters own smoothing and hysteresis);
//! only discrete edges emit. Thresholds for storage classes are
//! caller-supplied: the core classifies, it never invents budgets.

use super::capture_session::{CaptureState, HealthReason};
use himsat_events::{EventSequence, SessionId};

/// Caller-measured signal sample, stamped with session-relative
/// milliseconds from the adapter clock. The core never reads a clock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignalSample {
    /// Milliseconds since session start, adapter clock.
    pub elapsed_millis: u64,
    /// Input level in tenths of dBFS (`None` means unknown). Value
    /// changes alone never emit; silence/clipping edges do.
    pub level_dbfs_tenths: Option<i16>,
    /// No meaningful audio at the source.
    pub silent: bool,
    /// Input exceeds the clean range.
    pub clipping: bool,
    /// Pending chunks awaiting durable write.
    pub queue_depth: u64,
    /// Pending-chunk capacity before the adapter must shed or block.
    pub queue_capacity: u64,
    /// Durable-store bytes currently free.
    pub free_bytes: u64,
    /// Warn threshold in free bytes (adapter budget).
    pub warn_bytes: u64,
    /// Critical threshold in free bytes (adapter budget).
    pub critical_bytes: u64,
    /// Adapter's current degradation classification, if any.
    pub reason: Option<HealthReason>,
}

/// Portable storage pressure classes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StoragePressure {
    /// Above the warn threshold.
    Normal,
    /// At or below warn, above critical.
    Constrained,
    /// At or below critical.
    Critical,
}

/// Classifies storage pressure from caller budgets. Critical wins on
/// ties; no assumption about warn-vs-critical ordering is needed.
#[must_use]
pub const fn classify_storage(
    free_bytes: u64,
    warn_bytes: u64,
    critical_bytes: u64,
) -> StoragePressure {
    if free_bytes <= critical_bytes {
        StoragePressure::Critical
    } else if free_bytes <= warn_bytes {
        StoragePressure::Constrained
    } else {
        StoragePressure::Normal
    }
}

/// Reports whether the durable-write queue is full. A zero capacity
/// counts as full: nothing may be accepted.
#[must_use]
pub const fn queue_full(depth: u64, capacity: u64) -> bool {
    depth >= capacity
}

/// Discrete health changes. Each is an edge: steady signals emit
/// nothing, whatever their values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HealthChange {
    /// Session machine moved state.
    StateChanged {
        /// Previous state.
        from: CaptureState,
        /// New state.
        to: CaptureState,
    },
    /// Silence edge in either direction.
    SilenceChanged {
        /// New silence value.
        silent: bool,
    },
    /// Clipping edge in either direction.
    ClippingChanged {
        /// New clipping value.
        clipping: bool,
    },
    /// Queue-full edge in either direction.
    QueueChanged {
        /// New fullness value.
        full: bool,
    },
    /// Storage class moved.
    StorageChanged {
        /// Previous class.
        from: StoragePressure,
        /// New class.
        to: StoragePressure,
    },
    /// Adapter classification moved (including to/from none).
    ReasonChanged {
        /// Previous classification.
        from: Option<HealthReason>,
        /// New classification.
        to: Option<HealthReason>,
    },
}

/// Sequenced session health event for 003-ordered telemetry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HealthEvent {
    sequence: EventSequence,
    session: SessionId,
    change: HealthChange,
}

impl HealthEvent {
    /// Returns the 003 sequence number.
    #[must_use]
    pub const fn sequence(&self) -> EventSequence {
        self.sequence
    }

    /// Returns the owning session identity.
    #[must_use]
    pub const fn session(&self) -> SessionId {
        self.session
    }

    /// Returns the discrete change.
    #[must_use]
    pub const fn change(&self) -> HealthChange {
        self.change
    }
}

/// UI-ready digest of the last observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HealthSnapshot {
    state: CaptureState,
    level_dbfs_tenths: Option<i16>,
    silent: bool,
    clipping: bool,
    queue_full: bool,
    storage: StoragePressure,
    reason: Option<HealthReason>,
}

impl HealthSnapshot {
    /// Returns the session machine state.
    #[must_use]
    pub const fn state(&self) -> CaptureState {
        self.state
    }

    /// Returns the last reported level.
    #[must_use]
    pub const fn level_dbfs_tenths(&self) -> Option<i16> {
        self.level_dbfs_tenths
    }

    /// Returns the last reported silence value.
    #[must_use]
    pub const fn silent(&self) -> bool {
        self.silent
    }

    /// Returns the last reported clipping value.
    #[must_use]
    pub const fn clipping(&self) -> bool {
        self.clipping
    }

    /// Returns the last reported queue fullness.
    #[must_use]
    pub const fn queue_full(&self) -> bool {
        self.queue_full
    }

    /// Returns the last reported storage class.
    #[must_use]
    pub const fn storage(&self) -> StoragePressure {
        self.storage
    }

    /// Returns the last reported classification.
    #[must_use]
    pub const fn reason(&self) -> Option<HealthReason> {
        self.reason
    }
}

/// Edge-triggered health monitor. The first observation sets the
/// baseline and emits nothing; later observations emit one event per
/// flipped dimension in fixed order (state, reason, silence, clipping,
/// queue, storage) with consecutive 003 sequence numbers from zero.
#[derive(Clone, Debug, PartialEq)]
pub struct HealthMonitor {
    session: SessionId,
    next_sequence: u64,
    last: Option<HealthSnapshot>,
}

impl HealthMonitor {
    /// Creates a monitor with no baseline and sequence zero.
    #[must_use]
    pub const fn new(session: SessionId) -> Self {
        Self {
            session,
            next_sequence: 0,
            last: None,
        }
    }

    /// Returns the owning session identity.
    #[must_use]
    pub const fn session(&self) -> SessionId {
        self.session
    }

    /// Returns the last observation digest, if any.
    #[must_use]
    pub const fn snapshot(&self) -> Option<HealthSnapshot> {
        self.last
    }

    /// Observes one sample against the machine state, returning the
    /// emitted edge events (possibly empty).
    pub fn observe(&mut self, state: CaptureState, sample: &SignalSample) -> Vec<HealthEvent> {
        let current = HealthSnapshot {
            state,
            level_dbfs_tenths: sample.level_dbfs_tenths,
            silent: sample.silent,
            clipping: sample.clipping,
            queue_full: queue_full(sample.queue_depth, sample.queue_capacity),
            storage: classify_storage(sample.free_bytes, sample.warn_bytes, sample.critical_bytes),
            reason: sample.reason,
        };
        let Some(previous) = self.last else {
            self.last = Some(current);
            return Vec::new();
        };
        let mut events = Vec::new();
        let mut emit = |change: HealthChange, events: &mut Vec<HealthEvent>| {
            events.push(HealthEvent {
                sequence: EventSequence::new(self.next_sequence),
                session: self.session,
                change,
            });
            self.next_sequence += 1;
        };
        if previous.state != current.state {
            emit(
                HealthChange::StateChanged {
                    from: previous.state,
                    to: current.state,
                },
                &mut events,
            );
        }
        if previous.reason != current.reason {
            emit(
                HealthChange::ReasonChanged {
                    from: previous.reason,
                    to: current.reason,
                },
                &mut events,
            );
        }
        if previous.silent != current.silent {
            emit(
                HealthChange::SilenceChanged {
                    silent: current.silent,
                },
                &mut events,
            );
        }
        if previous.clipping != current.clipping {
            emit(
                HealthChange::ClippingChanged {
                    clipping: current.clipping,
                },
                &mut events,
            );
        }
        if previous.queue_full != current.queue_full {
            emit(
                HealthChange::QueueChanged {
                    full: current.queue_full,
                },
                &mut events,
            );
        }
        if previous.storage != current.storage {
            emit(
                HealthChange::StorageChanged {
                    from: previous.storage,
                    to: current.storage,
                },
                &mut events,
            );
        }
        self.last = Some(current);
        events
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HealthChange, HealthMonitor, SignalSample, StoragePressure, classify_storage, queue_full,
    };
    use crate::capture_session::{CaptureState, HealthReason};
    use himsat_events::SessionId;

    const SESSION: SessionId = SessionId::new(1);

    fn sample() -> SignalSample {
        SignalSample {
            elapsed_millis: 0,
            level_dbfs_tenths: Some(-120),
            silent: false,
            clipping: false,
            queue_depth: 0,
            queue_capacity: 8,
            free_bytes: 1_000_000,
            warn_bytes: 100_000,
            critical_bytes: 10_000,
            reason: None,
        }
    }

    fn monitor() -> HealthMonitor {
        HealthMonitor::new(SESSION)
    }

    #[test]
    fn baseline_and_steady_state_emit_nothing() {
        let mut monitor = monitor();
        assert!(monitor.snapshot().is_none());
        assert!(monitor.observe(CaptureState::Idle, &sample()).is_empty());
        let snapshot = monitor.snapshot().expect("baseline recorded");
        assert_eq!(snapshot.state(), CaptureState::Idle);
        assert_eq!(snapshot.level_dbfs_tenths(), Some(-120));
        assert!(!snapshot.silent());
        assert!(!snapshot.clipping());
        assert!(!snapshot.queue_full());
        assert_eq!(snapshot.storage(), StoragePressure::Normal);
        assert_eq!(snapshot.reason(), None);
        assert!(monitor.observe(CaptureState::Idle, &sample()).is_empty());
    }

    #[test]
    fn level_only_changes_never_emit() {
        let mut monitor = monitor();
        assert!(monitor.observe(CaptureState::Idle, &sample()).is_empty());
        let mut moved = sample();
        moved.level_dbfs_tenths = Some(-30);
        moved.elapsed_millis = 500;
        assert!(monitor.observe(CaptureState::Idle, &moved).is_empty());
        assert_eq!(
            monitor.snapshot().expect("snapshot").level_dbfs_tenths(),
            Some(-30)
        );
    }

    #[test]
    fn state_change_emits_sequenced_event() {
        let mut monitor = monitor();
        assert!(monitor.observe(CaptureState::Idle, &sample()).is_empty());
        let events = monitor.observe(CaptureState::Preparing, &sample());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].sequence().get(), 0);
        assert_eq!(events[0].session(), SESSION);
        assert_eq!(
            events[0].change(),
            HealthChange::StateChanged {
                from: CaptureState::Idle,
                to: CaptureState::Preparing,
            }
        );
        let events = monitor.observe(CaptureState::RecordingHealthy, &sample());
        assert_eq!(events[0].sequence().get(), 1);
    }

    #[test]
    fn every_dimension_flip_emits_in_fixed_order() {
        let mut monitor = monitor();
        assert!(
            monitor
                .observe(CaptureState::RecordingHealthy, &sample())
                .is_empty()
        );
        let mut flipped = sample();
        flipped.reason = Some(HealthReason::RouteChanged);
        flipped.silent = true;
        flipped.clipping = true;
        flipped.queue_depth = 8;
        flipped.free_bytes = 5_000;
        let events = monitor.observe(CaptureState::RecordingDegraded, &flipped);
        assert_eq!(events.len(), 6);
        assert_eq!(
            events[0].change(),
            HealthChange::StateChanged {
                from: CaptureState::RecordingHealthy,
                to: CaptureState::RecordingDegraded,
            }
        );
        assert_eq!(
            events[1].change(),
            HealthChange::ReasonChanged {
                from: None,
                to: Some(HealthReason::RouteChanged),
            }
        );
        assert_eq!(
            events[2].change(),
            HealthChange::SilenceChanged { silent: true }
        );
        assert_eq!(
            events[3].change(),
            HealthChange::ClippingChanged { clipping: true }
        );
        assert_eq!(
            events[4].change(),
            HealthChange::QueueChanged { full: true }
        );
        assert_eq!(
            events[5].change(),
            HealthChange::StorageChanged {
                from: StoragePressure::Normal,
                to: StoragePressure::Critical,
            }
        );
        for (index, event) in events.iter().enumerate() {
            assert_eq!(event.sequence().get(), index as u64);
        }
    }

    #[test]
    fn edges_fire_in_both_directions() {
        let mut monitor = monitor();
        assert!(monitor.observe(CaptureState::Idle, &sample()).is_empty());
        let mut on = sample();
        on.silent = true;
        on.clipping = true;
        on.queue_depth = 8;
        on.free_bytes = 50_000;
        let events = monitor.observe(CaptureState::Idle, &on);
        assert_eq!(events.len(), 4);
        let back = monitor.observe(CaptureState::Idle, &sample());
        assert_eq!(back.len(), 4);
        assert_eq!(
            back[0].change(),
            HealthChange::SilenceChanged { silent: false }
        );
        assert_eq!(
            back[1].change(),
            HealthChange::ClippingChanged { clipping: false }
        );
        assert_eq!(back[2].change(), HealthChange::QueueChanged { full: false });
        assert_eq!(
            back[3].change(),
            HealthChange::StorageChanged {
                from: StoragePressure::Constrained,
                to: StoragePressure::Normal,
            }
        );
    }

    #[test]
    fn storage_classifier_prefers_critical_on_ties() {
        assert_eq!(classify_storage(0, 100, 10), StoragePressure::Critical);
        assert_eq!(classify_storage(10, 100, 10), StoragePressure::Critical);
        assert_eq!(classify_storage(11, 100, 10), StoragePressure::Constrained);
        assert_eq!(classify_storage(100, 100, 10), StoragePressure::Constrained);
        assert_eq!(classify_storage(101, 100, 10), StoragePressure::Normal);
        assert_eq!(
            classify_storage(u64::MAX, u64::MAX, u64::MAX),
            StoragePressure::Critical
        );
    }

    #[test]
    fn zero_capacity_queue_is_full() {
        assert!(queue_full(0, 0));
        assert!(queue_full(8, 8));
        assert!(!queue_full(7, 8));
    }

    #[test]
    fn reason_clears_back_to_none() {
        let mut monitor = monitor();
        assert!(
            monitor
                .observe(CaptureState::RecordingDegraded, &sample())
                .is_empty()
        );
        let mut set = sample();
        set.reason = Some(HealthReason::SourceSilent);
        let events = monitor.observe(CaptureState::RecordingDegraded, &set);
        assert_eq!(events.len(), 1);
        let cleared = monitor.observe(CaptureState::RecordingHealthy, &sample());
        assert_eq!(cleared.len(), 2);
        assert_eq!(
            cleared[0].change(),
            HealthChange::StateChanged {
                from: CaptureState::RecordingDegraded,
                to: CaptureState::RecordingHealthy,
            }
        );
        assert_eq!(
            cleared[1].change(),
            HealthChange::ReasonChanged {
                from: Some(HealthReason::SourceSilent),
                to: None,
            }
        );
    }
}
