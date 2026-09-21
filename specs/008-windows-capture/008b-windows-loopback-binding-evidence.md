# 008B grain 2 — cpal/WASAPI loopback binding (candidate)

## Grain declaration

- Outcome: the Windows system-audio pathway is addressable through the
  closed `cpal =0.18.2` binding — render-endpoint enumeration with
  stable identity, F32 loopback stream open/play/pause/resume, and
  classified fault propagation — without touching the portable 006A/006B
  contract proven in grain 1.
- `scope_in`: the `cfg(target_os = "windows")` half of
  `crates/himsat-core/src/capture_windows_system_audio.rs`
  (`classify_error`, `config_from_supported`, `CpalLoopbackBackend`,
  `cpal_default_endpoint_id`, `cpal_endpoint_info`, `LiveLoopbackStream`,
  `open_f32_loopback_stream`) plus its Windows-only tests.
- `scope_out`: the microphone pathway (008A), the lifecycle/evidence
  matrix (008C), resampling or format conversion (010), journal sink
  wiring, protected-content or DRM behavior, and any UI surface.
- Dependencies: the Windows target edge `cpal =0.18.2` adopted in PR #206
  and the shared portable vocabulary from 008A; no new dependency and no
  manifest or lockfile change in this grain.
- Acceptance: exact-head CI + R3 SUCCESS with the `windows-latest` Rust
  job green and macOS/Ubuntu unchanged and green.
- Risk: R3 (platform capture). The binding can open a loopback stream,
  but CI never does, and no runtime claim is made here.
- Recovery: repair forward; the binding is additive inside one module.
- Minimality: one module's Windows half plus its tests and records; the
  portable core changes only in its module doc.
- Safety/security: no `unsafe`; no private API, entitlement escape, or
  driver workaround; no audio content is stored, logged, or inspected by
  the adapter; and a non-F32 render format is refused rather than
  converted.

## Binding surface

```text
classify_error(kind, stage)        cpal ErrorKind -> WindowsSystemAudioError, stage on fallbacks
config_from_supported(...)         SupportedStreamConfig -> Option<DeviceConfig> via the shared
                                   sample-format mapping
CpalLoopbackBackend                LoopbackBackend over cpal::default_host() output endpoints
cpal_default_endpoint_id(host)     default render endpoint id, when the OS names one
cpal_endpoint_info(device, ...)    endpoint id + friendly name + loopback config from
                                   default_output_config(); None when the id cannot be resolved
LiveLoopbackStream                 endpoint_id(), config(), pause(), resume()
open_f32_loopback_stream(...)      resolve render endpoint by id, refuse non-F32, build, play
```

Three behaviours are load-bearing and recorded rather than implied:

1. the stream is built with `build_input_stream` on a **render** endpoint,
   which is what makes the closed binding set
   `AUDCLNT_STREAMFLAGS_LOOPBACK` — the OS-sanctioned loopback mechanism,
   with no private API and no entitlement escape;
2. the configuration comes from `default_output_config()`, because
   `default_input_config()` returns `UnsupportedOperation` for a render
   endpoint and shared-mode loopback delivers the render mix format;
3. `preferred`/`loopback_config` is `Option`: a render endpoint whose mix
   format is outside the modelled set still enumerates and stays
   user-selectable, but opening it fails as a build-stage fault instead of
   silently converting.

## Fault classification

The same cpal `ErrorKind` mapping the microphone binding uses, with the
system-audio vocabulary: `PermissionDenied`, `DeviceBusy` (shared/
exclusive conflict), `DeviceNotAvailable` (endpoint loss),
`HostUnavailable` (audio service), `DeviceChanged` (reroute), and the
fallback `StreamFault(stage)`. A Windows-only test asserts that the
loopback classifier returns the same classifier string as the microphone
classifier for every listed kind at both stages, so the two Windows
pathways cannot silently diverge, and a second test pins that the stage
is carried on fallbacks.

## Audio-callback discipline

The cpal data callback forwards the borrowed frame slice to the caller and
nothing else: no allocation, lock, file or network I/O, logging, or
inference in the adapter's real-time context. The error callback converts
the classified fault plus the OS detail and forwards it. Bounded handoff
and drop policy remain the caller's contract, and the 005 sink is wired in
a later grain.

## Tests

Windows-only, added in this grain:

```text
loopback_classification_agrees_with_the_microphone_adapter   (hardware-free; no-drift guard)
loopback_classification_carries_the_stage_on_fallbacks       (hardware-free)
cpal_enumeration_never_panics_and_selects_consistently
cpal_default_endpoint_agrees_with_selection
live_loopback_open_reports_frames_or_classified_fault        (env-gated: HIMSAT_LIVE_LOOPBACK_TEST=1)
```

The portable 13 tests from grain 1 continue to run on every host. The
env-gated live loopback test is **NOT RUN** in CI, because CI runners have
no audio endpoints and this grain must not capture audio there; it exists
for a real Windows qualification run and reports frames or a classified
fault rather than asserting audio content.

## What is not claimed

- No loopback stream is opened in CI, so there is no Windows runtime,
  first-audio, loopback-capture, or Gate E claim; a real Windows endpoint
  is required to exercise the live path.
- Protected-content behavior, endpoint change during capture, exclusive
  mode held by another application, and long-session loss accounting are
  008C evidence-matrix obligations, not claims made here.
- Non-F32 render formats stay refused; widening requires Gate E evidence
  for the exact negotiated format.
- `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
