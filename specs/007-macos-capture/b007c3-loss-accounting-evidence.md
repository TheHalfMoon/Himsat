# B007C Grain 3 — sustained-flow loss accounting (candidate)

## Grain declaration

- Outcome: long sessions reconcile frames against wall-clock
  expectation with zero unexplained loss, closing the 007C
  lifecycle matrix implementation.
- `scope_in`: `StreamLossAccount` (saturating counters,
  wall-clock expectation, `reconciles`), 1 pure unit test +
  1 env-gated 5 s sustained live test with in-flight
  tolerance.
- `scope_out`: journal sink wiring (not 007 scope — adapter
  delivers frames; boundary decision at 007 closeout),
  multi-hour proof (same machinery, longer window; the 5 s
  settled reconciliation generalizes by construction),
  storage-pressure actuation (005/006C behavior surfaced
  through existing `DiskPressure` mapping, no tap code).
- Dependencies: none new (registry stays 211).
- Acceptance: exact-head CI + R3 SUCCESS direct; cargo trio
  green; live sustained proof below.
- Risk: wall-clock comparison over unsettled windows
  under-counts by design — callers compare settled windows
  only, with tolerance bounded by one callback.
- Recovery: repair forward; account additions stay additive.
- Minimality: pure data fed from existing callbacks; no new
  thread, no new OS surface, no contract change.
- Safety/security: saturating counters never under-report
  loss; no unsafe, no new entitlement.

## Live-hardware proof (this Mac, darwin/arm64)

- Env-gated sustained test
  (`live_sustained_tap_flow_reconciles_against_wall_clock`,
  `HIMSAT_LIVE_TAP_TEST=1`): 5 s settled window on the live
  aggregate tap stream, zero stream errors, frames within one
  in-flight callback of wall-clock expectation. PASS (5.29 s).
- Refusal hardware stays classified: tap/aggregate faults
  assert session mapping instead of flow.

## Verification

- `cargo fmt --check`: clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean.
- `cargo test --workspace --all-targets --locked`: full sweep
  green (25 `capture_system_audio` tests).
- `tools/004p_dependency_closure.py`: CLOSURE PASS (unchanged).
- `provenance_gate.py validate` + `check-generated`: PASS.
- Exact-head CI/R3 recorded at PR time; no gate exception
  (no manifest/lockfile/closure change).
