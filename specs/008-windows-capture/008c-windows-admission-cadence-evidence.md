# 008C grain 2 — Windows admission policy and checkpoint cadence (candidate)

## Grain declaration

- Outcome: the portable, hardware-free policy halves of the Windows
  evidence matrix — the admission decision a session makes when durable
  storage or the pending-chunk queue runs short, and the checkpoint
  cadence window a long session seals on.
- `scope_in`: `crates/himsat-core/src/capture_windows_pressure.rs`
  (budgets, refusal reasons, decision function, cadence window, and their
  tests) plus its module declaration.
- `scope_out`: journal-sink wiring (still a later grain), any UI surface,
  and every runtime evidence row that needs real Windows hardware.
- Dependencies: none new; the module builds only on the closed 006B
  classifier and reads no clock.
- Acceptance: exact-head CI + R3 SUCCESS with all three Rust platform jobs
  green and Diffcipline policy bounds respected with no exception.
- Risk: R2/R3 policy surface. No device, file, or clock is touched.
- Recovery: repair forward; the module is additive and one file deep.
- Minimality: one new module plus one module declaration.
- Safety/security: no `unsafe`, no OS call, no I/O, and no threshold
  invention — budgets stay caller-supplied, so a refusal can never be
  manufactured by the adapter.

## Contents

```text
StorageBudgets            warn_bytes, critical_bytes, queue_capacity (caller-supplied)
RefusalReason             StorageCritical | QueueFull | StorageCriticalAndQueueFull
AdmissionDecision         Accept | Degrade { pressure } | Refuse(RefusalReason)
decide_admission          pure decision over 006B classify_storage + queue_full
checkpoint_due            pure cadence boundary test over caller-stamped millis
advance_cadence_window    remainder that carries into the next window after a seal
```

Two policy properties are the reason this grain exists:

1. **A refusal is always explicit and attributable.** Critical storage
   and a full queue are reported distinctly, and both together are
   reported as their own classifier, so telemetry can tell "disk full"
   apart from "writer stalled behind a full disk" instead of collapsing
   them into one boolean;
2. **Warn-level pressure degrades without refusing.** `Constrained`
   pressure returns `Degrade`, which the owner surfaces through 006B,
   while capture continues — refusal is reserved for conditions under
   which accepting more audio would lose it.

A zero queue capacity counts as full (nothing may be accepted), matching
the closed 006B `queue_full` reading, and a zero cadence means "seal
whenever asked" rather than an implied infinite window.

## Tests

```text
admission_accepts_degrades_and_refuses_explicitly
admission_follows_the_closed_006b_classification      (matrix over thresholds and depths)
zero_queue_capacity_refuses_immediately
refusal_reason_classifiers_are_stable_and_distinct
checkpoint_cadence_seals_on_the_boundary_and_carries_the_remainder
cadence_window_is_stable_across_saturated_values
```

The matrix test re-derives the closed 006B classification beside every
decision and asserts the decision agrees with it, so the adapter cannot
drift from the shared contract; the saturation test pins behaviour at
`u64::MAX` thresholds, depths, and timestamps rather than assuming sane
inputs.

## Evidence-matrix status after this grain

```text
PROVEN (pure, all three CI hosts)   endpoint-change detection + mapping, suspend/resume
                                   detection + mapping, loss accounting, storage/queue
                                   admission policy, checkpoint cadence window
UNPROVEN (needs real hardware)     live endpoint change, real suspend/resume, multi-hour
                                   capture, live privacy toggle, live exclusive-mode
                                   contention, real storage-exhaustion refusal
NOT YET WIRED                      journal-sink wiring of sealed chunks and checkpoints
```

## What is not claimed

- No Windows runtime claim: nothing here opens a device, writes a file, or
  measures storage.
- `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- The unproven rows are not implied by pure-function coverage and are not
  scheduled by this grain.
