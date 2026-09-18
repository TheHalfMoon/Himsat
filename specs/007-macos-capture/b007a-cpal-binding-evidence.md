# B007A Grain 2 — cpal OS binding and dependency adoption (candidate)

## Grain declaration

- Outcome: live macOS microphone enumeration, F32 stream
  open/play/pause/resume, and permission-denial classification over
  cpal 0.18.2, implementing the grain-1 `MicrophoneBackend`
  contract with zero changes to its portable surface except one
  additive error variant.
- `scope_in`: `CpalMicrophoneBackend`, `open_f32_input_stream`,
  `LiveMicStream` (pause/resume/drop lifecycle),
  `MacosMicError::PermissionDenied` with start/runtime mapping to
  `PermissionRevoked`, cpal `=0.18.2` macOS-target DEPEND, full
  004P/provenance/notices/SBOM closure for the 48-crate subtree.
- `scope_out`: system/screen-audio tap (007B, cidre stays MIT and
  separate), non-F32 negotiation, DSP (010), evidence matrix (007C).
- Dependencies: one new direct (`cpal =0.18.2`, macOS only);
  47 new transitive lock packages across all platforms.
- Acceptance: exact-head CI + R3 SUCCESS via the B007 gate
  exception; cargo trio green; live-hardware open proven below.
- Risk: R3 recording lifecycle; OS surface is cpal's CoreAudio
  backend only, no private API.
- Recovery: repair forward; mapping additions stay additive.
- Minimality: cpal carries the donor-proven enumerate/config/
  callback shape and a safe API under the `unsafe_code = "forbid"`
  workspace rule; a raw CoreAudio binding would need unsafe or an
  unproven wrapper. The 48-package lock delta is resolver-mechanical
  (all-platform transitive closure), not adopted surface.
- Safety/security: no donor code copied; frames stay in-process;
  TCC denial classifies honestly instead of failing silently.

## Donor decision record (supersedes the grain-1 license assumption)

Grain 1 recorded cpal as "MIT OR Apache-2.0". Live verification
against upstream tags `v0.15.3`, `v0.16.0`, `v0.17.3`, `v0.18.0`,
`v0.18.2` proves cpal is **Apache-2.0-only on every relevant line**
(`license = "Apache-2.0"`, single `LICENSE` file). Correction:

- Selected basis for `cpal 0.18.2` is `Apache-2.0`, not MIT.
- This is schema-compliant, not a bound change: the MIT-only
  assertion in `tools/004p_dependency_closure.py` scopes to the
  markdown closure table; `cpal` enters through
  `EXPECTED_PLATFORM_PACKAGES` (like the existing Apple direct
  deps), whose entries are data-driven. `Apache-2.0` sits in the
  policy `allow` list (`governance/provenance/policy.json`).
- It is the only non-MIT selection in the 48: every other new
  package offers MIT (dual with Apache-2.0, BSD, Zlib, ISC, or
  Unlicense) and is MIT-selected with MIT notice text.
- Alternatives rejected on record: raw CoreAudio via `coreaudio-rs`
  (MIT-clean but needs an unsafe or unproven binding layer under a
  `forbid(unsafe_code)` workspace, with no donor proving the shape);
  meetily's git-patched cpal (`[patch.crates-io]` rev `51c3b43`,
  provenance burden); copying anarlog/meetily capture modules
  (unnecessary — the mapping is Himsat-native already).

## API verification trail (cpal 0.18.2, vendored source read)

- `SampleRate` is now a `u32` alias (no `.0` accessor).
- Enumeration/description/stream control live on `HostTrait` /
  `DeviceTrait` / `StreamTrait` (`use cpal::traits::{...}`).
- Device identity is `Device::description()` (`DeviceDescription`
  with infallible `name()`); framework-crate style paths avoided.
- `build_input_stream` takes `StreamConfig` by value; data callback
  is `FnMut(&[T], &InputCallbackInfo)`; error callback receives
  unified `cpal::Error` with `.kind()`.
- `ErrorKind::PermissionDenied` is real and surfaced: TCC denial
  maps to `MacosMicError::PermissionDenied`, never folded into a
  generic fault.

## Live-hardware proof (this Mac, darwin/arm64)

- Enumeration: `MacBook Pro Microphone`, default, 1ch/48kHz/F32.
- `open_f32_input_stream` on the default device: `OPEN_OK`,
  config echoed back (1ch/48kHz), immediate drop clean.
- Env-gated live tests (`HIMSAT_LIVE_MIC_TEST=1`) pass; default CI
  runs the hardware-tolerant enumeration/selection tests only.

## Closure method (mechanical, all values exact)

- Checksums: `Cargo.lock` stanzas (full 64-hex, re-verified after a
  truncation bug in a scratch probe script — caught by the
  lock cross-check before any artifact was written).
- Revisions: `.cargo_vcs_info.json` embedded in each published
  `.crate` (exact publish SHAs, with `path_in_vcs` as source path);
  spot-verified upstream (`cpal`, `coreaudio-rs`, `windows-rs`,
  `thiserror`, `objc2` framework batch).
- Repositories/license expressions: each crate's published
  `Cargo.toml` (`repository`, `license` fields); `.git` suffixes
  trimmed, `web-sys` tree suffix normalized to the repo root.
- Notice texts: in-crate `LICENSE-MIT`/`LICENSE`/MIT files where
  present; otherwise the upstream license file fetched at the
  pinned SHA (`jni-rs`, `rust-mobile/ndk`, `android-ndk-rs`,
  `rustaudio/sample`, `madsmtm/objc2` `LICENSE.md` statement).
- Registry entries generated from that research, then
  `THIRD_PARTY_NOTICES.md` + `governance/generated/sbom.json`
  rendered through the v2 gate renderer.
- Pre-existing defect noted (not introduced here):
  `tools/provenance.py` still expects registry schema v1 while the
  registry is v2, so its `generate`/`validate` fail on main too;
  regeneration used the v2 `provenance_gate.py` renderer, which CI
  checks. Repairing the stale tool is deferred as a carried item.

## Verification

- `cargo fmt --check`: clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean.
- `cargo test --workspace --all-targets --locked`: all suites pass
  (15 `capture_macos` tests: 11 portable + 4 live-backend).
- `OPENSSL_RUST_USE_NASM=0 python3 tools/004p_dependency_closure.py`:
  `004P P005/P006 CLOSURE PASS`.
- `python3 tools/provenance_gate.py validate`: `PROVENANCE V2 PASS`.
- `python3 tools/provenance_gate.py check-generated`:
  `GENERATED OUTPUTS PASS`.
- Exact-head CI/R3 recorded at PR time; the B007 gate exception
  carries this exact byte set (pins computed after final review).

## Regeneration set-check (additive discipline)

- SBOM components: 161 -> 209 (+48, zero lost).
- Notices sections: 161 -> 209 (+48, zero lost).
- The ~300 removed diff lines are regeneration reordering/render
  alignment, not content loss: entry rendering is byte-identical
  per entry (verified on `cpubits`), and the stale files on main
  predate the v2 renderer path used here.
- `ndk-sys 0.6.0+11769913`: registry id uses
  `cargo-ndk-sys-0.6.0-11769913` because entry ids admit
  `[a-z0-9._-]` only; the package tuple keeps the exact version.
