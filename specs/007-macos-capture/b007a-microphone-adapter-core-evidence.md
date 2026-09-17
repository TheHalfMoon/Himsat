# B007A Grain 1 — Microphone adapter core (zero new dependencies)

## Grain declaration

- Outcome: portable microphone discovery, selection, stable identity,
  and fault classification driving the closed 006A session machine
  with zero refused transitions.
- `scope_in`: `MicrophoneBackend` enumeration contract, preferred/
  default/first selection, FNV-1a stable source ids, bounded labels,
  start/runtime fault classification, mock backend, 10 unit tests.
- `scope_out`: OS stream opening and callbacks (next grain with the
  cpal binding), system/screen-audio tap (007B), DSP (010).
- Dependencies: none new. `himsat-events` identities and the 006A
  machine only.
- Acceptance: exact-head CI + R3 SUCCESS; `cargo fmt --check`,
  `cargo clippy -- -D warnings`, `cargo test` green; machine-drive
  test proves no refusal on the start, fault, resume, recovery, and
  stop paths.
- Risk: R3 recording lifecycle (carried by the spec); this grain
  adds no OS surface, so platform risk lands in the binding grain.
- Recovery: repair forward; mapping changes stay additive (new
  error variants map, never re-map old ones silently).
- Context budget / change surface: `crates/himsat-core/src/
  capture_macos.rs` plus the `lib.rs` module line; no manifest,
  lockfile, governance, workflow, or SBOM change.
- Minimality: the cpal binding needs a fixed enumeration contract
  to implement against; this grain fixes exactly that contract plus
  the machine mapping, nothing more.
- Safety/security: no audio leaves the process; labels are bounded
  OS-provided names already shown in device pickers; ids are
  non-reversible hashes, never raw device identifiers.

## Donor comparison (exact revisions inspected live)

All three donors reviewed at pinned immutable revisions; no donor
code copied or adapted in this grain.

| Candidate | Exact revision / path | License | Verdict |
|---|---|---|---|
| `fastrepl/anarlog` `crates/audio-actual` (`capture/stream.rs`, `mic.rs`, `speaker/macos.rs`), `crates/audio-device`, `crates/audio-chunking` | `5af2f90e7b8c7b7c299c969ed32cfd041b82a742` | MIT (`LICENSE`) | Closest proven shape: cpal 0.17 + dasp 0.11 + rodio 0.22 portable core, cidre 0.15 target-gated per OS. Validates the enumerate/default-config/callback split this grain contracts for. No transplant: Himsat mapping already covers it. |
| `Zackriya-Solutions/meetily` `frontend/src-tauri/src/audio/capture/` (`mod.rs`, `core_audio.rs`, `microphone.rs`, `system.rs`, `backend_config.rs`) | `a2cb62e827da7ef59f65064c97233efb2313878e` | MIT (`LICENSE.md`) | cpal 0.15.3 **patched to git rev `51c3b43`** (`[patch.crates-io]`), cidre aggregate-device tap for system audio. REJECTED for Himsat: git-patched audio dependency breaks reproducible closure; aggregate-tap system capture belongs to 007B, not the mic path. Behavior inputs (device selection UX, backend config switching) noted for 007C. |
| `Starmel/OpenSuperWhisper` `OpenSuperWhisper/AudioRecorder.swift`, `RecordingSessionController.swift`, `PCMRecording.swift` | `c8e6fe79d6851078940f459ab7dbc8ee39e2a97d` (`develop`) | MIT (`LICENSE`); submodules unexamined | Swift AVFoundation/CoreAudio; language boundary means REFERENCE only. Behavior inputs already covered: single-session guard (006A machine), 0.25s stop-tail (007C evidence), start/storage failure test list (007C matrix). |
| Himsat-native raw CoreAudio via `objc2`/hand-rolled bindings | n/a | n/a | REJECTED for 007A: maximal new surface for the mic path both donors prove cpal already covers; revisit only if cpal fails Gate E evidence. |

## cpal DEPEND decision (binding grain, not this one)

- DEPEND `cpal` from crates.io (Apache-2.0, selected basis MIT OR
  Apache-2.0 both inside the house closure set), newest 0.17+ release
  preserving the proven enumerate/default-config/callback shape,
  exact `=x.y.z` pin plus full transitive closure at adoption.
- System-tap binding (`cidre`, MIT) deferred to the 007B grain with
  its own closure; no git-sourced or patched audio dependency ever.
- This grain fixes the contract the binding implements, so the
  binding grain is mechanical: implement `MicrophoneBackend` for the
  cpal host, add stream opening with the proven callback shape, and
  run the 004P closure plus notices/SBOM updates then.

## Verification

- `cargo fmt --check -p himsat-core`: clean.
- `cargo clippy -p himsat-core --all-targets --all-features -- -D warnings`: clean.
- `cargo test -p himsat-core --lib capture_macos`: 10 passed.
- Exact-head CI/R3 recorded at PR time; post-merge qualification
  follows the canonical rule before 007A grain 2 begins.
