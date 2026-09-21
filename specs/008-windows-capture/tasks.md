# Specification 008 Tasks — Windows Capture

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and review evidence remain authoritative.

## Shaping — canonical

- [x] S001 Re-read canonical `main` at Specification 007 closeout merge `c96abba6ed5cd760611d0f407e7c3a8df9fb1879`.
- [x] S002 Confirm Specification 007 post-closeout CI `35627161924` and R3 `35627161891` succeeded.
- [x] S003 Re-read the constitution, master-plan unit 008, Gate E platform-qualification rule, the 003/004/005/006/007 consumable contracts, the donor registry, and the Windows-relevant research records without adopting material.
- [x] S004 Note upstream facts to refresh at implementation time (current supported WASAPI capture APIs, the cpal `wasapi` backend path, microphone privacy-control behavior, exclusive/shared-mode rules, Bluetooth/USB endpoint behavior, loopback restrictions) without adopting dependencies.
- [x] S005 Split prospective leaves 008A-008C as candidates only; exact split re-bounds after shaping qualifies.
- [x] S006 Define scope, additive-only 003/004/005/006/007 boundary discipline, donor-compare posture with 004P-style adoption gate, carried 006 residuals, acceptance, and R3 evidence contracts in `spec.md`/`plan.md`.
- [x] S007 Exact-head qualify shaping head: CI and R3 SUCCESS.
  - Shaping head `3338fd0df0e15f42725b4b11e2ce8a92cde176cf` (PR #202): pre-merge CI `35629526180` SUCCESS, R3 `35629526000` SUCCESS.
- [x] S008 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
  - Docs-only diff; Open Code Review 1.12.7 delegation mode reported 0 reviewable files (markdown/TOML only); zero submitted reviews, bot-only comments, no blocking finding.
- [x] S009 Merge shaping; canonical merge contains the exact shaping head as parent.
  - Canonical shaping merge `fa797a8288f26e98c5460e8f766f2acb44957101` (PR #202, parents `c96abba` + `3338fd0`; merge tree equals shaping-head tree).
- [x] S010 Require post-shaping qualification: CI and R3 SUCCESS on the exact canonical merge; only then may 008 implementation grains be bound.
  - Post-merge CI `35631397479` SUCCESS, R3 `35631396992` SUCCESS on `fa797a8288f26e98c5460e8f766f2acb44957101`.

## Implementation — 008A dependency adoption (Windows `cpal`)

- [x] I001 Verify registry entry for `cpal 0.18.2` (checksum, source repository/revision, Apache-2.0 license, registry count 211) before any product change.
- [x] I002 Bind the Windows-only target edge `cpal = { version = "=0.18.2", default-features = false }` in `crates/himsat-core/Cargo.toml`; no global dependency, no feature widening, no other target touched.
- [x] I003 Extend `tools/004p_dependency_closure.py` `EXPECTED_WINDOWS_DIRECT` so the closure gate fails closed on any drift of the new edge.
- [x] I004 Prove the WASAPI backend subset from immutable crates.io index metadata (`default = []`; non-optional `windows ^0.62` with the Win32_Media* features and `windows-core ^0.62`; ASIO/JACK/`audio_thread_priority`/`num-traits` disabled) and from `cargo tree --target x86_64-pc-windows-msvc -p cpal`.
- [x] I005 Record that no `Cargo.lock`, registry, notice, SBOM, or workflow byte changes: 211 external packages, 41 provider packages, unchanged.
- [x] I006 Run the local gates: `004p_dependency_closure.py`, `provenance_gate.py validate`, `provenance_gate.py check-generated`, `cargo metadata --locked`, `cargo fmt --check`.
- [x] I007 Exact-head qualify the adoption head: PR #206 head `f0009d0eceee7d5d1d0acc507122686215ab5b6b` passed pull-request CI `35646948025` and R3 `35646948045` on attempt 1, with `Rust / windows-latest` SUCCESS (17m42s).
  - Supersede record: PR #204 head `66ea9815c40c70e42663c775e1337502cf55e076` was never mergeable because a pull request's recorded comparison base cannot follow a base-branch move; its windows CI `35641324753` / job `106471568452` still proved the Windows build, lint, test, and dependency-closure steps SUCCESS on the identical content. It was closed unmerged, not deleted.
- [x] I008 Reconcile diff/reviews/threads/`main`/mergeability: zero submitted reviews (CodeRabbit "manual review required for this OSS repository", cubic skipping), zero review threads, OCR delegation ledger comment `5765808063`, `MERGEABLE`/`CLEAN`, proof verdict `REVIEW` with the single manifest reason and `DIFFCIPLINE B008A DEPENDENCY EXCEPTION PASS`.
- [x] I009 Merge under explicit expected-head protection (`expected_head_sha = f0009d0eceee7d5d1d0acc507122686215ab5b6b`, `merge_method = merge`): canonical merge `c1ad38d2dd151ceb0139ec7fd7caf2c4a337ed4c`, parents `c2b4bb54cb72014d3784d925d7e4433783b0a18b` + `f0009d0eceee7d5d1d0acc507122686215ab5b6b`, merge tree `a79615b1508cb2543ee81678175c793a1a49694a` equal to the accepted head tree, signature verified.
- [x] I010 Post-merge qualification on the exact canonical merge: push CI `35649474044` SUCCESS and R3 `35649473639` SUCCESS, both attempt 1. `SPEC_008A_DEPENDENCY_ADOPTION_STATE = CANONICAL_QUALIFIED`.
- [x] I011 Trusted-base mechanism: the Diffcipline adoption gate had to authorize the manifest-change diff from its own immutable base. PR #205 (merge `c2b4bb54cb72014d3784d925d7e4433783b0a18b`, pre-merge CI `35643377757` / R3 `35643377577`, post-merge CI `35645592708` / R3 `35645592747`) added the pinned B008A exception; the candidate then qualified from that base.

## Implementation — 008A grain 1 (Windows microphone adapter core)

- [x] J001 Declare the portable core in `crates/himsat-core/src/capture_windows.rs` and register the module in `crates/himsat-core/src/lib.rs`; no OS call and no `cpal` reference in this grain.
- [x] J002 Model Windows endpoint identity on the OS endpoint id (not the display name) and derive stable 003 source identity from it; report `preferred_config` as `Option` instead of mislabelling an unmodelled format.
- [x] J003 Define the Windows fault taxonomy (absence, stream fault build/play, privacy denial, exclusive-mode conflict, endpoint unavailable, audio-service unavailable, route rerouted) with stable classifier strings.
- [x] J004 Map every fault class onto the closed 006A machine for start and runtime, keeping the runtime mapping non-interrupting for a reroute and proving the mapping never produces a refused transition.
- [x] J005 Derive the portable 006B degradation reason from the same taxonomy, with a test asserting the event mapping and the health mapping cannot drift.
- [x] J006 Prove privacy revocation surfaces through `HealthMonitor` as a `ReasonChanged` edge instead of a silent stop.
- [x] J007 Run the local gates: `cargo fmt --all -- --check`, provenance validate/check-generated, and the 004P closure on the candidate tree.
- [x] J008 Exact-head qualify grain 1: PR #207 head `9e7237f21b6fd8f0aa906a303bc8924c66884b2f` passed pull-request CI `35652021251` and R3 `35652021195` on attempt 1, with `Rust / ubuntu-latest` 7m30s, `Rust / macos-latest` 8m1s, and `Rust / windows-latest` 17m29s all SUCCESS (compile, `clippy -D warnings`, 16 tests each).
- [x] J009 Reconcile and merge under expected-head protection: zero submitted reviews, zero review threads, OCR delegation ledger `5767157896` (2/2 reviewable files, 3 markdown exclusions stated, 2 findings repaired forward in `9e7237f`), `MERGEABLE`/`CLEAN`. Canonical merge `6532f9747a0bd65b11d2d5541482097bd2e2144c`, parents `c1ad38d` + `9e7237f`, merge tree `0b452816c9818229bcf1a33d4ebaab067314d059` equal to the accepted head tree, signature verified, post-merge CI `35654462537` / R3 `35654462544` SUCCESS.
- [x] J010 Promote the next authorized leaf: the Windows cpal/WASAPI binding over this core. `NEXT_IMPLEMENTATION_LEAF = SPEC_008_WINDOWS_WASAPI_BINDING_GRAIN`.

## Implementation — 008A grain 2 (cpal/WASAPI microphone binding)

- [x] K001 Implement `classify_error` over cpal `ErrorKind` with the stage carried into fallbacks, `portable_sample_format`, and `config_from_supported` inside `cfg(target_os = "windows")`.
- [x] K002 Implement `CpalMicrophoneBackend` over `cpal::default_host()` with `cpal_default_id` and `cpal_device_info`; endpoints whose id cannot be resolved are skipped rather than name-keyed.
- [x] K003 Implement `LiveMicStream` (`device_id`, `config`, `pause`, `resume`) and `open_f32_input_stream`, which resolves by endpoint id, refuses non-F32 defaults as build faults, and never silently converts.
- [x] K004 Keep the audio callback to a single borrowed-frame forward with no allocation, locking, I/O, logging, or inference in the adapter.
- [x] K005 Add the five Windows-only tests, including the hardware-free classifier and format-mapping tests and the `HIMSAT_LIVE_MIC_TEST=1` env-gated live opener that is NOT RUN in CI.
- [x] K006 Run the local gates: `cargo fmt --all -- --check`, provenance validate/check-generated, and the 004P closure on the candidate tree.
- [x] K007 Exact-head qualify grain 2: PR #208 head `efacffe69561d482c44d206dc90f8dd3546a8848` passed pull-request CI `35656490885` and R3 `35656490720` on attempt 1; `Rust / windows-latest` (job `106521341028`, 17m49s) was the first compile of the binding anywhere, alongside ubuntu 7m45s and macOS 7m28s.
- [x] K008 Reconcile and merge under expected-head protection: zero submitted reviews, zero review threads, OCR delegation ledger `5767673392` (1/1 reviewable file, 3 markdown exclusions stated, no blocking finding), `MERGEABLE`/`CLEAN`. Canonical merge `5787979e6dc62c7a4454c83aadbdc77623a3fe3a`, parents `6532f97` + `efacffe`, merge tree `475fdcdd71e19b9cc2749785b66131ce29b7ae1f` equal to the accepted head tree, signature verified, post-merge CI `35658997556` / R3 `35658997511` SUCCESS.

## Implementation — 008B grain 1 (Windows system-audio adapter core)

- [x] L001 Re-bind the next leaf against live canonical state: 008A is complete (core + binding canonical-qualified), so the next dependency-authorized leaf is 008B (OS-sanctioned WASAPI loopback), with no conflicting open PR or issue.
- [x] L002 Declare the portable core in `crates/himsat-core/src/capture_windows_system_audio.rs` and register the module; no OS call and no `cpal` reference in this grain.
- [x] L003 Record the OS-sanctioned loopback evidence from the pinned binding: `build_input_stream_raw_inner` sets `AUDCLNT_STREAMFLAGS_LOOPBACK` for a render endpoint, `supports_input()` is capture-only (so 008A enumeration cannot see render endpoints), and `default_input_config()` errors on render endpoints so the loopback configuration must come from `default_output_config()`.
- [x] L004 Model render-endpoint identity on the OS endpoint id with a separate identity domain, `SourceKind::SystemAudio` descriptors, and `loopback_config: Option<DeviceConfig>` instead of a mislabelled format.
- [x] L005 Define the system-audio fault taxonomy and map every class onto the closed 006A machine for start and runtime, keeping a reroute non-interrupting and proving no mapping produces a refused transition.
- [x] L006 Derive the portable 006B reason from the same taxonomy with a no-drift test, and prove an exclusivity conflict surfaces through `HealthMonitor`.
- [x] L007 Run the local gates: `cargo fmt --all -- --check`, provenance validate/check-generated, and the 004P closure on the candidate tree.
- [ ] L008 Exact-head qualify grain 1: pull-request CI and R3 SUCCESS with all three Rust platform jobs green.
- [ ] L009 Reconcile and merge under expected-head protection, then record grain 1 merge truth.
- [ ] L010 Promote the loopback binding grain: `CpalLoopbackBackend`, `LiveLoopbackStream`, `open_f32_loopback_stream`, the cpal error-kind classifier with a no-drift test against the microphone classifier, and the Windows-only tests.
