# B007C Grain 2 — lifecycle event observation (candidate)

## Grain declaration

- Outcome: route-change and sleep/wake lifecycle rows observed
  live and mapped onto the closed 006 session machine, with
  zero unsafe code, zero new dependencies, and zero 006
  contract change.
- `scope_in`: `TapLifecycleSignal` (DevicesChanged,
  WakeNotified), pure `detect_device_delta` /
  `detect_sleep_gap`, pure `event_for_lifecycle_signal`,
  polling `LifecycleWatch` (stop-and-join Drop),
  macOS `cpal_input_names` lister, 7 new tests (6 portable +
  1 env-gated live).
- `scope_out`: journal sink wiring (not 007 scope — the
  adapter delivers frames; recorded at closeout), block-based
  OS listeners (would need the unadopted cidre `blocks`
  feature), revocation callbacks (no OS callback exists;
  revocation surfaces through the stream error path, already
  mapped), long-session multi-hour proof (007C-3).
- Dependencies: none new (registry stays 211).
- Acceptance: exact-head CI + R3 SUCCESS direct; cargo trio
  green; live watch proof below.
- Risk: polling latency (bounded by cadence, floor 100 ms) —
  a route change surfaces at most one cadence late; expiry and
  lister faults end the thread, never a spin, never silent.
- Recovery: repair forward; signal/mapping additions stay
  additive.
- Minimality: polling reuses the closed cpal enumerator; no
  listener callback, no new feature, no new crate, no 006
  edit.
- Safety/security: thread stops and joins on drop; no raw
  pointers, no unsafe, no new OS entitlement.

## Design record (why polling, not listeners)

Two safe listener APIs exist in cidre 0.29.0 —
`AudioObjectAddPropertyListenerBlock` and
`NotificationCenter::add_observer_block` — but both hide
behind the unadopted `blocks` feature, and the C-fn property
listener would need an unsafe client-data dereference in
Himsat code. Adopting `blocks` would change the closed
dependency closure for an observer; polling the already-closed
cpal enumerator observes the same rows with no manifest,
closure, or safety change. Sleep is detected retroactively by
timestamp overshoot (10x cadence multiple): scheduling jitter
stays far below it, real suspension exceeds it by orders of
magnitude. Mapping uses only existing 006 events/reasons —
wake maps to `Interrupted(ProcessInterrupted)` and the owner
drives resume/recovery, never the watch.

## Live-hardware proof (this Mac, darwin/arm64)

- Env-gated live test (`live_lifecycle_watch_runs_against_real_routes`,
  `HIMSAT_LIVE_TAP_TEST=1`): watch spawns against the real
  cpal enumerator, runs 400 ms, drops and joins clean with no
  signal spam on stable routes. PASS.
- Scripted delta test proves the DevicesChanged row end to
  end through the real thread machinery (fake lister); pure
  tests pin delta/gap thresholds and both mappings, including
  a full session drive (interrupt → resume → recover for each
  signal) with zero refused transitions.

## Verification

- `cargo fmt --check`: clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean (two findings fixed: collapsible if, redundant must_use).
- `cargo test --workspace --all-targets --locked`: full sweep
  green (23 `capture_system_audio` tests; 6 live tests pass
  under `HIMSAT_LIVE_TAP_TEST=1`).
- `tools/004p_dependency_closure.py`: CLOSURE PASS (unchanged).
- `provenance_gate.py validate` + `check-generated`: PASS.
- Exact-head CI/R3 recorded at PR time; no gate exception
  (no manifest/lockfile/closure change).
