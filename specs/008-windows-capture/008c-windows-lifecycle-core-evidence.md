# 008C grain 1 — Windows lifecycle observation and loss accounting (candidate)

## Grain declaration

- Outcome: the portable, hardware-free halves of the Windows lifecycle
  and evidence-matrix work — endpoint-change and suspend/resume signals,
  their mapping onto the closed 006A machine, and the sustained-flow loss
  account a long session reconciles against wall clock.
- `scope_in`: `crates/himsat-core/src/capture_windows_lifecycle.rs`
  (signals, detectors, polling watch, event mapping, loss account, and
  their unit tests) plus its module declaration.
- `scope_out`: the OS-facing lifecycle probes, storage-pressure refusal
  policy, checkpoint-cadence wiring, and the evidence-matrix execution
  rows, which need either a device or their own grain.
- Dependencies: none new; the module compiles on every host and touches no
  OS audio API.
- Acceptance: exact-head CI + R3 SUCCESS with all three Rust platform jobs
  green and Diffcipline policy bounds respected with no exception.
- Risk: R3 (capture lifecycle). Nothing here opens a device, so the risk
  is bounded to signal semantics and accounting.
- Recovery: repair forward; the module is additive and one file deep.
- Minimality: one new module plus one module declaration; no manifest,
  lockfile, workflow, registry, notice, or SBOM change.
- Safety/security: no `unsafe`, no OS call, no audio content, no file or
  network I/O; the watch thread exists only to poll a caller-supplied
  lister and terminates on drop, on a channel error, or on lister failure.

## Contents

```text
WindowsLifecycleSignal             EndpointsChanged | WakeNotified(seconds)
LIFECYCLE_WATCH_MIN_CADENCE_MS     100 (the watch never spins below this)
LIFECYCLE_SLEEP_GAP_MULTIPLE       10 (suspension threshold relative to cadence)
detect_endpoint_delta              order-insensitive, duplicate-insensitive set comparison
detect_sleep_gap                   overflow-checked suspension detection, gap in whole seconds
event_for_lifecycle_signal         EndpointsChanged -> Interrupted(RouteChanged)
                                   WakeNotified(_)   -> Interrupted(ProcessInterrupted)
LifecycleWatch::spawn              polling watch thread reporting signals over a channel
drain_signals                      non-blocking drain for owners that poll
StreamLossAccount                  saturating callbacks/frames/stream-error accounting
                                   plus expected_frames(), reconciles(), unexplained_shortfall()
```

Two behaviours are stated explicitly because they are what the evidence
rows depend on:

- a lister that cannot enumerate ends the watch quietly: it emits **no**
  signal, so an unreadable endpoint list is never reported as a route
  change, and the owner observes a stopped watch rather than a fabricated
  interruption;
- `StreamLossAccount` saturates rather than wraps, and
  `unexplained_shortfall(expected)` exists so a long-session row can name
  the missing frames instead of only returning a boolean.

Endpoint identity here is the endpoint-id set, not a display name, so two
endpoints sharing a friendly name still register as two endpoints — the
same reasoning that drives the 008A/008B identity design.

## Tests

```text
endpoint_delta_is_order_insensitive_and_duplicate_insensitive
sleep_gap_detection_uses_the_cadence_multiple          (includes the overflow case)
lifecycle_signals_map_to_events_the_flowing_states_accept
watch_reports_endpoint_changes_and_stops_on_drop       (bounded thread test, 10 s ceiling)
watch_ends_quietly_when_the_lister_fails               (no signal, one poll)
loss_account_reconciles_exact_flow_and_names_the_shortfall
loss_account_saturates_instead_of_wrapping
loss_account_semantics_match_the_closed_macos_implementation
```

The mapping test feeds both signals into a real `CaptureSession` built
through the 008A selection path and asserts the machine accepts them,
which is the anti-refusal property the 006A contract requires. The final
test drives the Windows account and the closed macOS account
(`capture_system_audio::StreamLossAccount`) with the same scripted
callback sequence and asserts identical counters, identical reconciliation
verdicts, and identical `expected_frames` values, so the two platform
implementations cannot drift apart without a visible failure.

## Evidence-matrix status (what this grain does and does not prove)

```text
endpoint change        pure detection + 006A mapping proven here; live device
                       add/remove on Windows remains UNPROVEN
suspend/resume         pure detection + 006A mapping proven here; a real
                       Windows suspend/resume run remains UNPROVEN
long session / loss    accounting proven here, including saturation and
                       cross-implementation agreement; a multi-hour Windows run
                       remains UNPROVEN
privacy revocation     classification proven in 008A; live OS privacy toggle
                       remains UNPROVEN
exclusive-mode conflict classification proven in 008A/008B; live contention
                       with another application remains UNPROVEN
storage pressure       not addressed by this grain
```

## What is not claimed

- No Windows runtime claim of any kind: no device is opened, nothing is
  suspended, and no audio is captured or measured.
- `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- The unproven rows above require real Windows hardware interaction;
  they are not implied by the pure-function coverage.
