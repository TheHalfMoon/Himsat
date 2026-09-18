# B007B Grain 1 — System-tap adapter core (zero new dependencies)

## Grain declaration

- Outcome: portable system-audio discovery, selection, stable
  identity, and fault classification driving the closed 006A
  session machine with zero refused transitions, with tap
  authorization keeping its own identity end to end.
- `scope_in`: `SystemTapBackend` enumeration contract, preferred/
  default/first selection, FNV-1a stable ids in a disjoint
  `himsat-007b-system-audio` space, tap fault classification
  (create/start/authorization), mock backend, 8 unit tests.
- `scope_out`: native tap creation and streaming (next grain with
  the cidre binding), microphone path (007A, closed), DSP (010).
- Dependencies: none new. `himsat-events` identities, the 006A
  machine, and the 007A `DeviceConfig`/`sanitize_label` helpers
  only. The closed 006 contract already carries
  `SourceKind::SystemAudio`; no closed code changed.
- Acceptance: exact-head CI + R3 SUCCESS; `cargo fmt --check`,
  `cargo clippy -- -D warnings`, `cargo test` green; machine-drive
  test proves no refusal on the start, fault, resume, recovery,
  and stop paths; kind-separation test proves mic/system ids never
  collide for equal names.
- Risk: R3 recording lifecycle (carried by the spec); this grain
  adds no OS surface, so platform risk lands in the binding grain.
- Recovery: repair forward; mapping changes stay additive.
- Context budget / change surface: `crates/himsat-core/src/
  capture_system_audio.rs` plus the `lib.rs` module line; no
  manifest, lockfile, governance, workflow, or SBOM change.
- Minimality: the native binding needs a fixed enumeration
  contract to implement against; this grain fixes exactly that
  contract plus the machine mapping, nothing more.
- Safety/security: no audio leaves the process; labels are bounded
  OS-provided names already shown in route pickers; ids are
  non-reversible hashes in a kind-separated space.

## Donor comparison (exact revisions inspected live)

No donor code copied or adapted in this grain.

| Candidate | Exact revision / path | License | Verdict |
|---|---|---|---|
| `fastrepl/anarlog` `crates/audio-actual/src/speaker/macos.rs` (+ `speaker.rs`, `mod.rs`) | `5af2f90e7b8c7b7c299c969ed32cfd041b82a742` | MIT (`LICENSE`) | Closest proven shape: CoreAudio process tap (`TapDesc::with_mono_global_tap_excluding_processes`) + private aggregate device via cidre, ringbuf async stream, drop counting, sample-rate probing. Validates the tap-contract split this grain contracts for. No transplant: Himsat mapping already covers it. |
| `Zackriya-Solutions/meetily` `frontend/src-tauri/src/audio/capture/system.rs` (+ `core_audio.rs`) | `a2cb62e827da7ef59f65064c97233efb2313878e` | MIT (`LICENSE.md`) | cpal output-device enumeration plus CoreAudio tap on macOS (same process-tap family). REJECTED for transplant: git-adjacent dependency posture already ruled out in 007A, and the mapping is Himsat-native already. |
| `Starmel/OpenSuperWhisper` Swift recorder | `c8e6fe79d6851078940f459ab7dbc8ee39e2a97d` | MIT | Microphone-only; no system-tap surface to compare. REFERENCE only. |
| Himsat-native raw CoreAudio tap | n/a | n/a | REJECTED for 007B: maximal new unsafe-adjacent surface for a pathway both Rust donors prove through cidre; revisit only if cidre fails Gate E evidence. |

## cidre DEPEND decision (binding grain, not this one)

- DEPEND `cidre` from crates.io (MIT, verified `cargo info`;
  Apple's-framework bindings in the anarlog-proven shape), exact
  `=x.y.z` pin plus full transitive closure at adoption. No
  git-sourced audio dependency ever.
- Tap authorization (Screen Recording policy surface) is evidenced
  in 007C, not assumed here: denial classifies as revocation with
  the OS condition in telemetry detail.
- The Apple-SDK-derivation caveat in the objc2-family license
  statement (reviewed during 007A closure) applies equally to any
  CoreAudio binding and is recorded for the binding grain.

## Verification

- `cargo fmt --check -p himsat-core`: clean.
- `cargo clippy -p himsat-core --all-targets --all-features -- -D warnings`: clean.
- `cargo test -p himsat-core --lib capture_system_audio`: 8 passed.
- Exact-head CI/R3 recorded at PR time; post-merge qualification
  follows the canonical rule before 007B grain 2 begins.
