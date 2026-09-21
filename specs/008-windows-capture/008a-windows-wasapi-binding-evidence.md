# 008A grain 2 — cpal/WASAPI microphone binding (candidate)

## Grain declaration

- Outcome: the Windows microphone pathway is addressable through the
  closed `cpal =0.18.2` binding — endpoint enumeration with stable
  identity, F32 input-stream open/play/pause/resume, and classified
  fault propagation — without touching the portable 006A/006B contract
  proven in grain 1.
- `scope_in`: the `cfg(target_os = "windows")` half of
  `crates/himsat-core/src/capture_windows.rs`
  (`classify_error`, `portable_sample_format`, `config_from_supported`,
  `CpalMicrophoneBackend`, `cpal_default_id`, `cpal_device_info`,
  `LiveMicStream`, `open_f32_input_stream`) plus its Windows-only tests.
- `scope_out`: system-audio loopback (008B), the lifecycle/evidence
  matrix (008C), format conversion or resampling (010), chaining the
  stream into the 005 journal, and any UI or permission-prompt surface.
- Dependencies: the Windows target edge `cpal =0.18.2` adopted in
  `008a-windows-cpal-adoption-evidence.md`; no new dependency, no
  manifest or lockfile change in this grain.
- Acceptance: exact-head CI + R3 SUCCESS with the `windows-latest` Rust
  job green (compile, `clippy -D warnings`, tests) and macOS/Ubuntu
  unchanged and green.
- Risk: R3 (platform capture). The binding can open a real endpoint, but
  it is never opened in CI and no runtime claim is made here.
- Recovery: repair forward; the binding is additive inside one module.
- Minimality: one module's Windows half plus its tests and records; the
  portable core is untouched except for the module doc.
- Safety/security: no `unsafe` (workspace lint `unsafe_code = "forbid"`);
  no private API, no entitlement escape, no ASIO/JACK subsystem (those
  cpal features stay disabled), and no automatic format conversion.

## Binding surface

```text
classify_error(kind, stage)      cpal ErrorKind -> WindowsMicError, carrying the stage on fallbacks
portable_sample_format(format)   F32/I16/U16 -> Some, every other cpal format -> None
config_from_supported(...)       SupportedStreamConfig -> Option<DeviceConfig>
CpalMicrophoneBackend            MicrophoneBackend over cpal::default_host()
cpal_default_id(host)            default input endpoint id, when the OS names one
cpal_device_info(device, ...)    endpoint id + friendly name + optional config; None when
                                 the endpoint id cannot be resolved
LiveMicStream                    holds the cpal stream; device_id(), config(), pause(), resume()
open_f32_input_stream(...)       resolve by endpoint id, refuse non-F32, build, play
```

Enumeration deliberately skips endpoints whose id cannot be resolved
instead of falling back to a display name: a name key would merge two
distinct endpoints that share a friendly name, which is exactly the
identity error the endpoint-id design exists to prevent. An endpoint
whose default input format is outside the modelled set still enumerates
(and can be selected by the user) but reports
`preferred_config = None`, and opening it fails as a build-stage stream
fault rather than silently converting.

## Fault classification (from the pinned binding's own HRESULT map)

cpal 0.18.2 maps WASAPI failures in `src/host/wasapi/mod.rs`; this grain
consumes the mapped `ErrorKind` and adds the stage:

```text
AUDCLNT_E_DEVICE_IN_USE              -> ErrorKind::DeviceBusy           -> ExclusiveModeConflict
AUDCLNT_E_DEVICE_INVALIDATED         -> ErrorKind::DeviceNotAvailable   -> EndpointUnavailable
AUDCLNT_E_ENDPOINT_CREATE_FAILED     -> ErrorKind::DeviceNotAvailable   -> EndpointUnavailable
AUDCLNT_E_SERVICE_NOT_RUNNING        -> ErrorKind::HostUnavailable      -> AudioServiceUnavailable
AUDCLNT_E_RESOURCES_INVALIDATED      -> ErrorKind::StreamInvalidated    -> StreamFault(stage)
AUDCLNT_E_UNSUPPORTED_FORMAT and the
  buffer-size/exclusive-mode family   -> ErrorKind::UnsupportedConfig    -> StreamFault(stage)
microphone privacy denial            -> ErrorKind::PermissionDenied     -> PermissionDenied
route rerouted (live stream)         -> ErrorKind::DeviceChanged        -> RouteRerouted
```

`ExclusiveModeConflict` is therefore the shared/exclusive-mode conflict
the 008 specification requires to be surfaced, and it is distinguishable
from endpoint loss and from service unavailability rather than being
collapsed into one generic failure.

## Audio-callback discipline

The cpal data callback runs on the audio thread. This binding's callback
does exactly one thing: forward the borrowed frame slice to the caller's
`FnMut(&[f32])`. There is no allocation, no lock, no file or network
I/O, no logging, and no model inference inside the adapter's callback,
so the real-time context stays bounded by whatever the caller does. The
error callback converts the classified fault plus the OS detail string
and forwards it; it performs no blocking work either. Bounded handoff
and dropping policy remain the caller's contract, and the 005 sink is
wired in a later grain.

## Tests

Windows-only, added in this grain:

```text
cpal_kinds_classify_deterministically            (hardware-free; every mapped kind + fallback)
only_modelled_sample_formats_map                 (hardware-free)
cpal_enumeration_never_panics_and_selects_consistently
cpal_default_id_agrees_with_selection
live_stream_open_reports_frames_or_classified_fault   (env-gated: HIMSAT_LIVE_MIC_TEST=1)
```

The portable 16 tests from grain 1 continue to run on every host. The
env-gated live test is **NOT RUN** in CI, because CI runners have no
capture hardware and this grain must not open a device there; it exists
for a real Windows qualification run and reports frames or a classified
fault rather than asserting audio content.

## What is not claimed

- No Windows runtime, first-audio, loopback, or Gate E claim: no device
  is opened by CI, and the live path is unexecuted here.
- `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- Non-F32 negotiation remains refused; widening it requires Gate E
  evidence for the exact negotiated format.
- Endpoint-id stability is bounded (Windows may reissue ids on
  re-enumeration or reinstall), and Bluetooth/USB device-change
  behavior is an 008C evidence-matrix obligation, not a claim here.
