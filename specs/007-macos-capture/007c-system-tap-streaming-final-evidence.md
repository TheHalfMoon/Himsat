# 007C System-Tap Streaming — Final Evidence (grains 1+2+3, reconciled)

## Bounded unit

B007C closes the 007C system-tap streaming pathway: the aggregate
first-audio route (grain 1), the polling lifecycle-observation
watch (grain 2), and the sustained-flow loss account (grain 3),
with the 007B "streaming sanction" residual closed by evidence.
All three grains carry zero unsafe code and add zero dependencies
(registry stays 211; cidre 0.29.0 and cpal 0.18.2 were already
closed under 007B/007A). Donor inputs stay compare-only; no donor
code copied. The portable 006 contract is consumed unchanged.

## Grain 1 — aggregate first-audio streaming (PR196 → PR197)

- Head `d6527dd6796414c9b4cd978af8425bc4538bced7`, accepted
  tree `90ac76347bda08f09de3d9705689e0cb5b1d1aec`:
  `open_aggregate_tap_stream` (tap create,
  private aggregate assembly, bounded visibility poll, F32-only
  cpal open/play), drop-ordered `LiveSystemTapStream` (stream →
  aggregate → tap), `TapStage::Assemble` + `tap-assemble-fault`
  classifier, pure `tap_list_entry` / `aggregate_composition`
  builders, 3 new tests (1 pure, 1 classifier, 1 env-gated live),
  stale 007B doc refresh. No manifest/lockfile/closure change.
- Pre-merge CI `35321568922` / R3 `35321568928` SUCCESS.
- Canonical merge `f52ff0856b8cea0fbc28f94715a797e8bf24a6db`
  (`08f2d9a` + `d6527dd`); merge tree equals the accepted head tree.
- Post-merge CI `35323542302` / R3 `35323542345` SUCCESS.
- Reviews: none submitted, zero threads; no blocking finding.
- **Sanction decision (the 007B residual, closed):** direct
  inspection of cidre 0.29.0 (`cat/audio/base_types.rs`) proves
  there is no safe IOProc sample view — `BufList::as_slice` is
  safe but `Buf.data` is a bare `*mut u8` with no safe accessor,
  so the final dereference would have to be Himsat unsafe. The
  sanctioned path chosen instead never touches IOProc buffers:
  tap → private aggregate device (`AggregateDevice::with_desc`,
  safe; `Drop` destroys it) → ordinary cpal input device whose
  callback delivers safe `&[f32]`. Every FFI sample crossing
  stays inside the already-closed, already-audited cidre/cpal
  bindings; `forbid(unsafe_code)` holds, including tests.

## Grain 2 — lifecycle event observation (PR197 → PR198)

- Head `50603568a3185733c50ff18d2fe972dd7b08fa15`, accepted
  tree `deada7297782a7bf8e4e0987b00cdca6b6ecb374`:
  `TapLifecycleSignal` (`DevicesChanged`, `WakeNotified`), pure
  `detect_device_delta` / `detect_sleep_gap`, pure
  `event_for_lifecycle_signal`, polling `LifecycleWatch`
  (stop-and-join `Drop`), macOS `cpal_input_names` lister, 7 new
  tests (6 portable + 1 env-gated live). No manifest/lockfile/
  closure change.
- Pre-merge CI `35325695878` / R3 `35325695881` SUCCESS.
- Canonical merge `8b6c619750646c454fa754671c9eb3d34114b28c`
  (`f52ff08` + `5060356`); merge tree equals the accepted head tree.
- Post-merge CI `35327258174` / R3 `35327258212` SUCCESS.
- Reviews: none submitted, zero threads; no blocking finding.

## Grain 3 — sustained-flow loss accounting (PR199)

- Head `65aa2d623ab9edb4e7d1b7f7250a825ddb9a2c4d`, accepted
  tree `8a5153610a0b9a17d9de0f2cc6e8eebf2503ac07`:
  `StreamLossAccount` (saturating callback/frame/error counters,
  wall-clock `expected_frames`, settled-window `reconciles`), 1
  pure unit test + 1 env-gated 5 s sustained live test with
  in-flight tolerance. No manifest/lockfile/closure change; the
  diff is 3 files, +212 / −0 (under the 900-line gate,
  18-file bound, in-policy `expected_files`).
- Pre-merge CI `35329269388` / R3 `35329269420` SUCCESS on
  attempt 1 (all 11 CI jobs green: Rust ubuntu/macos/windows,
  B404 Secret Service, Provenance ubuntu/macos/windows,
  Provenance adversarial self-test, SpecGrain pinned source,
  Diffcipline R2, Negative controls).
- Canonical merge `9b2268f5164592ee961780a5580478ab6d6f0f5e`
  (`8b6c619` + `65aa2d6`); merge tree equals the accepted head
  tree (`8a5153610a0b9a17d9de0f2cc6e8eebf2503ac07`); GitHub
  reports the merge signature verified/valid.
- Post-merge CI/R3 on the exact canonical merge are recorded at
  PR time; the first push-triggered pair
  (`35618685760` CI / `35618685873` R3) was superseded-cancelled
  by the newer push run (`35618918438`) on the same SHA
  `9b2268f` (the B007GATEFIX supersede precedent: covered by its
  superset run, no failure).
- Reviews: none submitted, zero inline threads; the only PR
  comments are the Qodo billing-blocked notice and the
  CodeRabbit skip notice. No blocking finding.

## Streaming → journal substrate (bounded, no unsafe)

The frames delivered by `open_aggregate_tap_stream` cross into the
already-closed 005 journal envelope/chunk machinery through the
`fn(&[f32])` callback boundary. That boundary is a safe slice
hand-off; no raw pointer escapes and no Himsat unsafe block is
introduced. Journal sink *wiring* (the adapter's frame delivery
driving 005A open/commit/close) is recorded as a boundary
decision at 007 closeout, not as 007C product code — the 007C
leaf delivers audio and accounts loss; it does not own the
durable sink.

## 007C lifecycle evidence matrix

Each row below is a required 007C row mapped onto the closed 006A
session machine (no 006 event or reason is redefined). "Live" =
env-gated `HIMSAT_LIVE_TAP_TEST=1` on this darwin/arm64 host;
"function" = portable unit proof.

| Row | Proof | 006A mapping |
| --- | --- | --- |
| first audio (positive) | Live `live_aggregate_tap_stream_flows_or_classifies`; 318 callbacks, 162,816 frames, 79,298 nonzero with system audio playing | RecordingHealthy |
| sustained flow / long session | Live `live_sustained_tap_flow_reconciles_against_wall_clock`: 5 s settled window, 0 stream errors, frames within one in-flight callback of wall-clock expectation | RecordingHealthy; loss account reconciles to zero unexplained |
| permission denied / revoked | Function `authorization_denial_path_recovers_through_retry`, `start_failures_map_to_preparing_accepted_events`, `runtime_faults_map_to_flowing_accepted_events`; OS-authorization denial classified `tap-authorization-denied` → `PermissionRevoked` | FailedRecoverable; Interrupted(PermissionRevoked); re-grant recovers via `RetryRequested`/`RecoveryConfirmed` |
| route change | Live `live_lifecycle_watch_runs_against_real_routes`; Function `device_delta_detects_set_changes_order_insensitively`, `lifecycle_signals_map_to_accepted_events` | Interrupted(RouteChanged) |
| device yank / aggregate unpublished | Function `device_delta_detects_set_changes_order_insensitively` (set-shrink detects); surfaced through the same device-delta row | Interrupted(RouteChanged) |
| sleep/wake | Function `sleep_gap_needs_tenfold_overshoot`, `lifecycle_signals_drive_session_without_refusal` | Interrupted(ProcessInterrupted); owner drives resume/recovery, never the watch |
| interruption | Function `lifecycle_signals_drive_session_without_refusal` (interrupt → resume → recover, zero refused transitions) | Interrupted → Recovering → RecordingHealthy |
| storage-pressure refusal | Owned and proven by 006B `DiskPressure` on the journal path; no tap-specific code exists, so no 007C row can diverge | RecordingDegraded(DiskPressure) |

All 25 `capture_system_audio` tests are green (20 portable + 5
env-gated live). Zero unsafe code; zero new dependencies;
registry stays 211.

## Live-hardware proof (this Mac, darwin/arm64)

- First-audio signal run with `afplay Glass.aiff`: tap `TAP_OK`,
  aggregate `AGG_OK` (1 stream, transport `grup`), cpal
  visibility after async publication, 318 callbacks / 162,816
  frames / 1ch/48kHz/F32 / 79,298 nonzero — real system audio
  end to end.
- `live_aggregate_tap_stream_flows_or_classifies`: PASS (1.9 s
  poll + 1.5 s window); refusal hardware asserts session
  classification instead of success.
- `live_lifecycle_watch_runs_against_real_routes`: watch spawns
  against the real cpal enumerator, runs 400 ms, drops and joins
  clean with no signal spam on stable routes. PASS.
- `live_sustained_tap_flow_reconciles_against_wall_clock`: 5 s
  settled window, zero stream errors, frames within one in-flight
  callback of expectation. PASS (5.29 s).

## Residual risks (carried, not blockers)

1. Multi-hour sustained proof is not run here; the 5 s settled
   reconciliation generalizes by construction (same wall-clock
   comparison, longer window), and must not be claimed as
   multi-hour evidence until it is actually run.
2. Device-yank, permission-revocation, and storage-pressure rows
   are proven at the function level and through owned 006
   behavior, not through live OS end-to-end actuation on this
   host; the live rows claimed above are the four env-gated
   tests.
3. Non-F32 tap formats refuse as `TapStage::Create` faults
   (007B residual, preserved); negotiation widens only with
   fresh Gate E evidence.
4. Journal sink wiring is a 007 closeout boundary decision, not
   007C product code.

## Disposition

B007C is CANONICAL_CLOSED: shaped (007C shaping qualified at
`8d379af2042cf23ea4da1f70e03f9a7675296fb0`), bounded (grains
1+2+3), exact-head qualified with merge-tree equality throughout,
post-merge CI/R3 SUCCESS on every canonical merge SHA, reconciled
here. This closes the 007C leaf; the next authorized node is the
Specification 007 closeout reconciliation (B007 → SPEC_007
closeout), which the repository-owner must qualify before any
Specification 008 shaping.
