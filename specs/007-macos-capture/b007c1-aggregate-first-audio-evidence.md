# B007C Grain 1 — aggregate first-audio streaming (candidate)

## Grain declaration

- Outcome: live system-audio samples flow tap -> private
  aggregate device -> cpal F32 input stream, with zero unsafe
  code in Himsat and zero new dependencies.
- `scope_in`: `open_aggregate_tap_stream` (tap create, private
  aggregate assembly, bounded visibility poll, F32-only cpal
  open/play), `LiveSystemTapStream` (drop-ordered teardown:
  stream, aggregate, tap), `TapStage::Assemble` +
  `tap-assemble-fault` classifier, pure `tap_list_entry` /
  `aggregate_composition` builders, stale 007B doc refresh,
  3 new tests (1 pure, 1 classifier, 1 env-gated live).
- `scope_out`: journal sink wiring (007C-2), lifecycle matrix
  rows (007C-3), non-F32 negotiation, multi-tap mixes.
- Dependencies: none new (cidre 0.29.0 + cpal 0.18.2 already
  closed, registry stays 211).
- Acceptance: exact-head CI + R3 SUCCESS direct (docs/code
  only, no manifest); cargo trio green; live first-audio proof
  below.
- Risk: aggregate publication is asynchronous (bounded poll,
  5 s deadline, assemble fault on expiry); fixed aggregate uid
  faults honestly on concurrent opens.
- Recovery: repair forward; stage mapping additions stay
  additive.
- Minimality: reuses the closed cidre tap API, the closed cpal
  input-stream machinery (`open_f32_input_stream` shapes), and
  the 006 session/health classifiers; no new crate, no new
  contract, no unsafe.
- Safety/security: private aggregate (process-local,
  destroyed with the stream); no donor code copied; no
  private API.

## Sanction decision record (the 007B residual, closed)

007B deferred streaming because IOProc sample extraction needs
an unsafe block. Direct inspection of cidre 0.29.0
(`cat/audio/base_types.rs`) confirms there is no safe sample
view: `DeviceIoBlock` delivers `&AudioBufList` safely and
`BufList::as_slice` is safe, but `Buf.data` is a bare `*mut u8`
with no safe accessor — the final dereference would have to be
Himsat unsafe. The sanctioned path chosen instead: never touch
IOProc buffers at all. The tap is assembled into a private
aggregate device (`AggregateDevice::with_desc`, safe; `Drop`
destroys it) and the aggregate is opened as an ordinary cpal
input device, whose callback delivers safe `&[f32]`. Every FFI
sample crossing stays inside the already-closed,
already-audited cidre/cpal bindings. No constitution exception,
no new gate, no unsafe in Himsat code (workspace
`forbid(unsafe_code)` holds, including tests).

## Live-hardware proof (this Mac, darwin/arm64)

- Spike (throwaway, `/tmp`, not committed): tap TAP_OK,
  aggregate AGG_OK (1 stream, transport `grup`), cpal
  visibility after async publication (first poll missed,
  second saw it — the race the bounded poll exists for).
- Signal run with `afplay Glass.aiff` playing: 318 callbacks,
  162,816 frames, 1ch/48kHz/F32, **79,298 nonzero samples** —
  the tap carries real system audio end to end.
- Committed env-gated live test
  (`live_aggregate_tap_stream_flows_or_classifies`,
  `HIMSAT_LIVE_TAP_TEST=1`): PASS on the success path here
  (1.9 s: poll + 1.5 s capture window); on refusal hardware it
  asserts session classification instead of success.

## Verification

- `cargo fmt --check`: clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean.
- `cargo test --workspace --all-targets --locked`: all suites
  pass (16 `capture_system_audio` tests; one full-sweep scare
  was ENOSPC flakes in unrelated vault modules with the disk
  at 100% — re-ran green after freeing spike build space, no
  code cause).
- `tools/004p_dependency_closure.py`: CLOSURE PASS (unchanged).
- `provenance_gate.py validate` + `check-generated`: PASS.
- Exact-head CI/R3 recorded at PR time; no gate exception
  (no manifest/lockfile/closure change).
