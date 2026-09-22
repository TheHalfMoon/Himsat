//! 008C grain 2 Windows admission policy and checkpoint cadence.
//!
//! Portable, hardware-free policy halves of the Windows evidence matrix:
//! the admission decision a capture session must make when durable
//! storage or the pending-chunk queue runs short, and the checkpoint
//! cadence window a long session seals on.
//!
//! Both are pure functions over caller-supplied budgets and caller-stamped
//! elapsed time. The core does not invent thresholds and does not read a
//! clock: 006B already classifies storage pressure and queue fullness from
//! caller budgets, and this module turns that classification into an
//! explicit, testable admission decision and cadence window so refusal is
//! never a silent drop.

use crate::capture_health::{StoragePressure, classify_storage, queue_full};

/// Caller-supplied budgets for one capture session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageBudgets {
    /// Warn threshold in free bytes.
    pub warn_bytes: u64,
    /// Critical threshold in free bytes.
    pub critical_bytes: u64,
    /// Pending-chunk capacity before the adapter must shed or refuse.
    pub queue_capacity: u64,
}

/// Why a session refused to accept more audio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefusalReason {
    /// Durable storage reached the caller's critical threshold.
    StorageCritical,
    /// The pending-chunk queue is full.
    QueueFull,
    /// Both conditions hold at once; reported distinctly so telemetry can
    /// distinguish "disk full" from "writer stalled behind a full disk".
    StorageCriticalAndQueueFull,
}

impl RefusalReason {
    /// Short stable classifier for telemetry detail strings.
    #[must_use]
    pub const fn classifier(&self) -> &'static str {
        match self {
            Self::StorageCritical => "storage-critical",
            Self::QueueFull => "queue-full",
            Self::StorageCriticalAndQueueFull => "storage-critical-and-queue-full",
        }
    }
}

/// Admission decision for one observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionDecision {
    /// Accept new audio; storage is above the warn threshold and the
    /// queue has room.
    Accept,
    /// Accept new audio but report the degradation the adapter must
    /// surface through 006B.
    Degrade {
        /// The pressure class that caused the degradation.
        pressure: StoragePressure,
    },
    /// Refuse new audio with an explicit reason. A refusal is a reported
    /// condition, never a silent drop: the owner still decides how to
    /// drain, seal, or stop.
    Refuse(RefusalReason),
}

/// Decides whether a session may accept more audio. Storage pressure and
/// queue fullness come from the closed 006B classifier, so the adapter
/// never re-derives thresholds: critical storage refuses unconditionally,
/// a full queue refuses while storage is still usable, warn-level
/// pressure degrades without refusing, and zero queue capacity counts as
/// full (nothing may be accepted).
#[must_use]
pub const fn decide_admission(
    budgets: StorageBudgets,
    free_bytes: u64,
    queue_depth: u64,
) -> AdmissionDecision {
    let pressure = classify_storage(free_bytes, budgets.warn_bytes, budgets.critical_bytes);
    let full = queue_full(queue_depth, budgets.queue_capacity);
    match (pressure, full) {
        (StoragePressure::Critical, true) => {
            AdmissionDecision::Refuse(RefusalReason::StorageCriticalAndQueueFull)
        }
        (StoragePressure::Critical, false) => {
            AdmissionDecision::Refuse(RefusalReason::StorageCritical)
        }
        (_, true) => AdmissionDecision::Refuse(RefusalReason::QueueFull),
        (StoragePressure::Constrained, false) => AdmissionDecision::Degrade {
            pressure: StoragePressure::Constrained,
        },
        (StoragePressure::Normal, false) => AdmissionDecision::Accept,
    }
}

/// Reports whether a checkpoint is due, given the caller-stamped
/// milliseconds since the last seal and the caller's cadence. A zero
/// cadence means "seal whenever asked", which is the degenerate but
/// explicit reading rather than an implied infinite window.
#[must_use]
pub const fn checkpoint_due(elapsed_since_last_seal_millis: u64, cadence_millis: u64) -> bool {
    if cadence_millis == 0 {
        return true;
    }
    elapsed_since_last_seal_millis >= cadence_millis
}

/// Returns the elapsed time that carries into the next cadence window
/// after a seal. A long gap seals once and keeps the remainder, so time
/// that passed while a seal was in flight is never lost. A zero cadence
/// leaves no remainder.
#[must_use]
pub const fn advance_cadence_window(
    elapsed_since_last_seal_millis: u64,
    cadence_millis: u64,
) -> u64 {
    if cadence_millis == 0 {
        return 0;
    }
    elapsed_since_last_seal_millis % cadence_millis
}

#[cfg(test)]
mod tests {
    use super::{
        AdmissionDecision, RefusalReason, StorageBudgets, advance_cadence_window, checkpoint_due,
        decide_admission,
    };
    use crate::capture_health::{StoragePressure, classify_storage};

    const BUDGETS: StorageBudgets = StorageBudgets {
        warn_bytes: 1_000,
        critical_bytes: 100,
        queue_capacity: 8,
    };

    #[test]
    fn admission_accepts_degrades_and_refuses_explicitly() {
        assert_eq!(
            decide_admission(BUDGETS, 10_000, 0),
            AdmissionDecision::Accept
        );
        assert_eq!(
            decide_admission(BUDGETS, 1_000, 0),
            AdmissionDecision::Degrade {
                pressure: StoragePressure::Constrained,
            }
        );
        assert_eq!(
            decide_admission(BUDGETS, 100, 0),
            AdmissionDecision::Refuse(RefusalReason::StorageCritical)
        );
        assert_eq!(
            decide_admission(BUDGETS, 10_000, 8),
            AdmissionDecision::Refuse(RefusalReason::QueueFull)
        );
        assert_eq!(
            decide_admission(BUDGETS, 10_000, 9),
            AdmissionDecision::Refuse(RefusalReason::QueueFull)
        );
        assert_eq!(
            decide_admission(BUDGETS, 50, 8),
            AdmissionDecision::Refuse(RefusalReason::StorageCriticalAndQueueFull)
        );
    }

    #[test]
    fn admission_follows_the_closed_006b_classification() {
        for free_bytes in [0_u64, 50, 100, 101, 999, 1_000, 1_001, 10_000] {
            for depth in [0_u64, 7, 8, 64] {
                let decision = decide_admission(BUDGETS, free_bytes, depth);
                let pressure =
                    classify_storage(free_bytes, BUDGETS.warn_bytes, BUDGETS.critical_bytes);
                let full = depth >= BUDGETS.queue_capacity;
                match decision {
                    AdmissionDecision::Accept => {
                        assert!(!full);
                        assert_eq!(pressure, StoragePressure::Normal);
                    }
                    AdmissionDecision::Degrade { pressure: reported } => {
                        assert!(!full);
                        assert_eq!(reported, StoragePressure::Constrained);
                        assert_eq!(reported, pressure);
                    }
                    AdmissionDecision::Refuse(_) => {
                        assert!(full || pressure == StoragePressure::Critical);
                    }
                }
            }
        }
    }

    #[test]
    fn zero_queue_capacity_refuses_immediately() {
        let budgets = StorageBudgets {
            warn_bytes: 1_000,
            critical_bytes: 100,
            queue_capacity: 0,
        };
        assert_eq!(
            decide_admission(budgets, 10_000, 0),
            AdmissionDecision::Refuse(RefusalReason::QueueFull)
        );
    }

    #[test]
    fn refusal_reason_classifiers_are_stable_and_distinct() {
        let classifiers = [
            RefusalReason::StorageCritical.classifier(),
            RefusalReason::QueueFull.classifier(),
            RefusalReason::StorageCriticalAndQueueFull.classifier(),
        ];
        assert_eq!(classifiers[0], "storage-critical");
        assert_eq!(classifiers[1], "queue-full");
        assert_eq!(classifiers[2], "storage-critical-and-queue-full");
        let mut unique = classifiers;
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn checkpoint_cadence_seals_on_the_boundary_and_carries_the_remainder() {
        assert!(!checkpoint_due(999, 1_000));
        assert!(checkpoint_due(1_000, 1_000));
        assert!(checkpoint_due(9_999, 1_000));
        assert!(checkpoint_due(0, 0));
        assert_eq!(advance_cadence_window(1_000, 1_000), 0);
        assert_eq!(advance_cadence_window(1_250, 1_000), 250);
        assert_eq!(advance_cadence_window(9_999, 1_000), 999);
        assert_eq!(advance_cadence_window(0, 0), 0);
    }

    #[test]
    fn cadence_window_is_stable_across_saturated_values() {
        assert!(checkpoint_due(u64::MAX, 1));
        assert!(checkpoint_due(u64::MAX, u64::MAX));
        assert_eq!(advance_cadence_window(u64::MAX, u64::MAX), 0);
        assert_eq!(advance_cadence_window(u64::MAX, 2), 1);
        let saturated = StorageBudgets {
            warn_bytes: u64::MAX,
            critical_bytes: u64::MAX,
            queue_capacity: u64::MAX,
        };
        assert_eq!(
            decide_admission(saturated, u64::MAX, u64::MAX),
            AdmissionDecision::Refuse(RefusalReason::StorageCriticalAndQueueFull)
        );
    }
}
