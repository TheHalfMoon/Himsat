# B007B Grain 2 — cidre tap-lifecycle binding and adoption (candidate)

## Grain declaration

- Outcome: live CoreAudio process-tap authorization, creation,
  and format proof over cidre 0.29.0, implementing the grain-1
  `SystemTapBackend` contract for enumeration plus a tap probe.
- `scope_in`: `CidreSystemTapBackend` (cpal output-route
  enumeration), `probe_process_tap` (mono global tap create, uid +
  format snapshot, guard drop), `TapDescription`, F32-only ASBD
  mapping, `DeviceConfig::from_supported` widened to `pub(crate)`,
  cidre `=0.29.0` macOS-target DEPEND with features
  `av`/`core_audio`/`dispatch`/`macos_15_0`, full closure for the
  2-crate subtree (both MIT).
- `scope_out`: sample streaming (IOProc sample extraction needs an
  unsafe block, which the workspace forbids; authorization,
  creation, and format proof land here while streaming waits for a
  sanctioned sample path — recorded as the 007B residual, not
  hidden), aggregate-device assembly, system-tap evidence matrix
  (007C).
- Dependencies: one new direct (`cidre =0.29.0`, macOS only); one
  new transitive (`cidre-macros =0.12.0`). Everything else reuses
  the closed cpal/objective-C closure.
- Acceptance: exact-head CI + R3 SUCCESS via a B007B gate
  exception; cargo trio green; live-hardware tap proof below.
- Risk: R3 recording lifecycle; OS surface is tap create/query
  only — no streaming, no aggregate device, no private API.
- Recovery: repair forward; mapping additions stay additive.
- Minimality: cidre carries the donor-proven tap shape with safe
  bindings under `forbid(unsafe_code)`; the tap probe is the
  smallest live authorization proof. Features are the minimal
  compiling set (donor uses a superset).
- Safety/security: no donor code copied; the probe creates a tap,
  snapshots metadata, and destroys it — no audio flows, no stream
  persists, no prompt is triggered in headless runs.

## Donor decision record

- anarlog `speaker/macos.rs` at `5af2f90e` proves the process-tap
  + aggregate shape through cidre; adopted as the SHAPE only
  (aggregate assembly and async streaming stay out of this grain
  per the unsafe rule, documented above).
- meetily `system.rs`/`core_audio.rs` at `a2cb62e8` proves the
  same tap family; no transplant (git-adjacent posture already
  ruled out; mapping is Himsat-native).
- Feature set `av`/`core_audio`/`dispatch`/`macos_15_0` derived
  empirically: à-la-carte `core_audio` alone does not compile
  (cidre-internal `dispatch` import and weak-linkage `alloc`
  need the availability feature); `macos_15_0` matches the donor
  and this machine's SDK. Recorded exactly so the closure pins
  the working combination.

## Live-hardware proof (this Mac, darwin/arm64)

- Routes: `MacBook Pro Speakers`, default, 2ch/48kHz/F32.
- `probe_process_tap`: `TAP_OK`, 36-char uid, 1ch/48kHz — tap
  creation permitted here, snapshot sane, guard dropped clean.
- Env-gated live tests pass; default CI runs the
  hardware-tolerant enumeration plus pure ASBD-mapping tests
  (F32 accepted; integer/narrow/zero-channel/zero-rate refused).

## Closure method (mechanical, all values exact)

- Checksums: `Cargo.lock` stanzas, cross-checked programmatically.
- Revisions: `.cargo_vcs_info.json` embedded SHAs, both verified
  to exist upstream (`yury/cidre`).
- `cidre-macros` ships no `repository` field; the repo is
  `yury/cidre` per its embedded `path_in_vcs`
  (`cidre-macros`) — recorded, not guessed.
- Notices: upstream `LICENSE.txt` (MIT, Yury Korolev) fetched at
  the pinned SHA; both crates select MIT.
- Registry extended 209 -> 211; notices/SBOM regenerated through
  the v2 gate renderer; all gates PASS.

## Verification

- `cargo fmt --check`: clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean.
- `cargo test --workspace --all-targets --locked`: all suites pass
  (13 `capture_system_audio` tests).
- `OPENSSL_RUST_USE_NASM=0 python3 tools/004p_dependency_closure.py`:
  `004P P005/P006 CLOSURE PASS`.
- `python3 tools/provenance_gate.py validate`: `PROVENANCE V2 PASS`.
- `python3 tools/provenance_gate.py check-generated`:
  `GENERATED OUTPUTS PASS`.
- Exact-head CI/R3 recorded at PR time; the B007B gate exception
  carries this exact byte set (pins computed after final review).
