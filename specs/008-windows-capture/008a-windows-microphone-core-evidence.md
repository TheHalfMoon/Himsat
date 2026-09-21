# 008A grain 1 — Windows microphone adapter core (candidate)

## Grain declaration

- Outcome: the portable Windows microphone adapter core that consumes
  the closed 006A session machine, 006B health telemetry, and 006C
  checkpoint discipline, with endpoint identity, selection, and fault
  classification proven on every CI host before any Windows-only byte
  exists.
- `scope_in`: `crates/himsat-core/src/capture_windows.rs` (portable
  core plus its unit tests) and the `capture_windows` module
  declaration in `crates/himsat-core/src/lib.rs`.
- `scope_out`: the cpal/WASAPI OS binding (`CpalMicrophoneBackend`,
  `LiveMicStream`, `open_f32_input_stream`, the cpal error-kind
  classifier, the cpal sample-format mapping, and the Windows-only
  tests). Those are grain 2 and no OS call is made here.
- Dependencies: none new. `cpal =0.18.2` was adopted for the Windows
  target in the preceding 008A dependency-adoption unit; this grain
  does not reference it yet, so the crate builds identically on Linux
  and macOS.
- Acceptance: exact-head CI + R3 SUCCESS with `windows-latest`,
  `ubuntu-latest`, and `macos-latest` Rust jobs green; the portable
  tests below pass on all three; Diffcipline policy bounds respected
  without any exception.
- Risk: R3 (platform capture surface). This grain cannot open a device,
  so its risk is bounded to mapping semantics rather than OS behavior.
- Recovery: repair forward; the module is additive and one file deep.
- Minimality: one new module plus one module declaration; no manifest,
  lockfile, workflow, registry, notice, or SBOM change.
- Safety/security: no `unsafe` (workspace lint `unsafe_code = "forbid"`),
  no OS call, no plaintext or device-name-derived logging, and no
  fallback that could silently capture the wrong endpoint or format.

## Portable core contents

```text
SampleFormat                 F32 | I16 | U16 (modelled set)
DeviceConfig                 channels, sample_rate_hz, format
InputDeviceInfo              device_id (OS endpoint id), name, is_default,
                             preferred_config: Option<DeviceConfig>
StreamStage                  Build | Play
WindowsMicError              NoInputDevices, StreamFault(B|P), PermissionDenied,
                             ExclusiveModeConflict, EndpointUnavailable,
                             AudioServiceUnavailable, RouteRerouted
MicrophoneBackend            input_devices() + default_input_id()
SelectedInput                resolved device + stable 003 SourceId
select_input                 endpoint id -> name -> OS default -> default flag -> first
stable_source_id             dual FNV-1a64 over domain tag + endpoint id
sanitize_label               trim, fallback, 128-char char-boundary truncation
event_for_start_failure      total mapping accepted by `Preparing`
event_for_runtime_fault      Option<CaptureEvent>; None means "no machine event"
health_reason_for            portable 006B reason, or None for reroutes
MockMicrophoneBackend        injected backend for tests and hardware-free hosts
device_info                  test/helper constructor
```

Two deliberate differences from the 007A macOS core, both recorded
rather than silent:

1. identity is the OS endpoint id, not the display name, so two
   endpoints with the same friendly name stay distinct;
2. `preferred_config` is `Option<DeviceConfig>`: an endpoint whose
   default input format is outside the modelled set is reported without
   a configuration instead of being mislabelled F32 (007A maps unknown
   formats onto F32, which this core does not repeat).

## Fault mapping (the contract grain 2 must implement)

```text
fault                     start (Preparing)          runtime (flowing)             006B reason
NoInputDevices            FailRecoverable(SourceSilent)  Interrupted(RouteChanged)   RouteChanged
StreamFault(_)            FailRecoverable(RouteChanged)  Interrupted(RouteChanged)   RouteChanged
PermissionDenied          FailRecoverable(PermRevoked)   Interrupted(PermRevoked)    PermissionRevoked
ExclusiveModeConflict     FailRecoverable(RouteChanged)  Interrupted(RouteChanged)   RouteChanged
EndpointUnavailable       FailRecoverable(RouteChanged)  Interrupted(RouteChanged)   RouteChanged
AudioServiceUnavailable   FailRecoverable(RouteChanged)  Interrupted(RouteChanged)   RouteChanged
RouteRerouted             FailRecoverable(RouteChanged)* none (stream keeps flowing)  none
```

\* `RouteRerouted` is unreachable at start by the backend contract; the
start mapping stays total so `Preparing` can never be stranded. The
runtime mapping and the health reason deliberately return "nothing to
do" for a reroute, because cpal documents that a rerouted stream stays
active and needs no rebuild, and interrupting it would be a false stop.
A test asserts that the event mapping and the health mapping agree for
every fault class, so the two cannot drift apart.

The Windows-specific classes come from the closed binding's own
HRESULT mapping, read from the pinned cpal source (evidence recorded in
`008a-windows-cpal-adoption-evidence.md`): `AUDCLNT_E_DEVICE_IN_USE` ->
device busy (exclusive/shared conflict), `AUDCLNT_E_DEVICE_INVALIDATED`
and `AUDCLNT_E_ENDPOINT_CREATE_FAILED` -> endpoint unavailable,
`AUDCLNT_E_SERVICE_NOT_RUNNING` -> audio service unavailable, and the
microphone-privacy denial -> permission denied. Grain 2 must classify
those kinds into exactly these classes and unit-test the mapping on the
Windows CI host without audio hardware.

## Tests (portable, run on ubuntu + macOS + Windows CI)

```text
stable_ids_match_for_equal_endpoints
stable_ids_differ_for_distinct_endpoints
stable_ids_are_domain_separated_from_other_source_kinds
selection_prefers_endpoint_id_then_name_then_default
selection_falls_back_to_flagged_default_then_first
empty_enumeration_is_no_input_devices
enumeration_faults_propagate_unchanged
labels_truncate_on_char_boundary_with_fallback
start_failures_map_to_preparing_accepted_events      (every fault class)
runtime_faults_map_to_flowing_accepted_events        (every fault class)
rerouted_stream_keeps_flowing_without_a_machine_event
runtime_health_reason_agrees_with_the_machine_mapping
permission_denial_surfaces_through_006b_health
adapter_drives_session_without_refusal
start_failure_path_recovers_through_retry
error_classifiers_are_stable_and_distinct
```

The mapping tests feed every fault class into a live `CaptureSession`
and assert the machine accepts it, which is the anti-refusal property
the 006A contract requires. The health test drives `HealthMonitor`
through a baseline sample and a permission-denied sample and asserts a
single `ReasonChanged` edge from `None` to `PermissionRevoked`, so
privacy revocation is surfaced rather than silently stopping capture.

## What is not claimed

- No Windows device is opened here: this grain makes no Windows runtime
  claim, no first-audio claim, and no Gate E qualification.
- `SPEC_008_PLATFORM_SCOPE` remains
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- Non-F32 negotiation stays refused; widening it needs Gate E evidence
  for the exact negotiated format.
- Endpoint-id stability is bounded: Windows may issue a new id when an
  endpoint is re-enumerated or reinstalled, so identity is stable for a
  given endpoint, not permanent.
