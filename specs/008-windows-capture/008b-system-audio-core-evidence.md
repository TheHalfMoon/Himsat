# 008B grain 1 — Windows system-audio adapter core (candidate)

## Grain declaration

- Outcome: the portable Windows system-audio adapter core that consumes
  the closed 006A session machine, 006B health telemetry, and 006C
  checkpoint discipline for an OS-sanctioned loopback source, with
  render-endpoint identity, selection, and fault classification proven
  on every CI host before any Windows-only byte exists.
- `scope_in`: `crates/himsat-core/src/capture_windows_system_audio.rs`
  (portable core plus its unit tests) and its module declaration in
  `crates/himsat-core/src/lib.rs`.
- `scope_out`: the cpal/WASAPI loopback binding (`CpalLoopbackBackend`,
  `LiveLoopbackStream`, `open_f32_loopback_stream`, the cpal error-kind
  classifier, and the Windows-only tests) — grain 2. No OS call is made
  here.
- Dependencies: none new. `cpal =0.18.2` for the Windows target was
  adopted in PR #206; this grain does not reference it.
- Acceptance: exact-head CI + R3 SUCCESS with the `windows-latest`,
  `ubuntu-latest`, and `macos-latest` Rust jobs green; Diffcipline policy
  bounds respected with no exception.
- Risk: R3. The core cannot open a device, so its risk is limited to
  mapping and identity semantics.
- Recovery: repair forward; the module is additive and one file deep.
- Minimality: one new module plus one module declaration; no manifest,
  lockfile, workflow, registry, notice, or SBOM change.
- Safety/security: no `unsafe`, no OS call, no private API, no
  entitlement escape, no audio content handled, and no fallback that
  could select a different endpoint or format than requested.

## Portable core contents

```text
RenderEndpointInfo            endpoint_id (render endpoint), name, is_default,
                              loopback_config: Option<DeviceConfig>
WindowsSystemAudioError       NoRenderEndpoints, StreamFault(B|P), PermissionDenied,
                              ExclusiveModeConflict, EndpointUnavailable,
                              AudioServiceUnavailable, RouteRerouted
LoopbackBackend               render_endpoints() + default_render_endpoint_id()
SelectedSystemAudio           resolved endpoint + stable 003 SourceId
select_loopback_input         endpoint id -> name -> OS default -> default flag -> first
stable_system_source_id       dual FNV-1a64 over the 008B domain tag + endpoint id
sanitize_label                trim, system-audio fallback, 128-char char-boundary truncation
event_for_start_failure       total mapping accepted by `Preparing`
event_for_runtime_fault       Option<CaptureEvent>; None means "no machine event"
health_reason_for             portable 006B reason, or None for reroutes
MockLoopbackBackend           injected backend for tests and hardware-free hosts
endpoint_info                 test/helper constructor
```

The descriptor kind is `SourceKind::SystemAudio`, and the identity domain
tag (`himsat-008b-system-audio`) keeps loopback sources disjoint from
microphone sources and from the macOS adapter even when the underlying
endpoint string is identical; a test asserts that separation for both.
`loopback_config` reuses the shared portable `DeviceConfig`/`SampleFormat`
vocabulary defined for this platform by 008A, while the fault taxonomy and
identity helper stay module-local, mirroring the 007A/007B precedent of
independently auditable per-pathway adapters.

## OS-sanctioned loopback evidence (the contract grain 2 must implement)

The pathway is the OS-sanctioned WASAPI loopback mechanism, and the pinned
closed binding already implements it rather than requiring any
entitlement escape or private API:

```text
build_input_stream_raw_inner (cpal 0.18.2 src/host/wasapi/device.rs):
    if self.data_flow() == Audio::eRender {
        stream_flags |= Audio::AUDCLNT_STREAMFLAGS_LOOPBACK;
    }
```

so opening a render endpoint *as an input stream* sets the loopback flag
itself. Two consequences the binding grain must honour, both read from
the same pinned source:

1. `supports_input()` is `data_flow() == eCapture`, so
   `HostTrait::input_devices()` never yields render endpoints — the
   loopback pathway must enumerate `output_devices()`. This is also why
   008A's microphone enumeration cannot accidentally pick up a loopback
   endpoint;
2. `default_input_config()` returns `UnsupportedOperation` for a render
   endpoint, so the loopback stream configuration must come from
   `default_output_config()` — the render mix format that shared-mode
   loopback actually delivers.

Shared-mode loopback also does not pass through a sample-rate converter,
so a non-F32 render mix format cannot be silently converted; grain 2 must
refuse it as a build-stage fault, exactly as 008A refuses a non-F32
microphone format.

## Fault mapping

```text
fault                     start (Preparing)              runtime (flowing)            006B reason
NoRenderEndpoints         FailRecoverable(SourceSilent)  Interrupted(RouteChanged)    RouteChanged
StreamFault(_)            FailRecoverable(RouteChanged)  Interrupted(RouteChanged)    RouteChanged
PermissionDenied          FailRecoverable(PermRevoked)   Interrupted(PermRevoked)     PermissionRevoked
ExclusiveModeConflict     FailRecoverable(RouteChanged)  Interrupted(RouteChanged)    RouteChanged
EndpointUnavailable       FailRecoverable(RouteChanged)  Interrupted(RouteChanged)    RouteChanged
AudioServiceUnavailable   FailRecoverable(RouteChanged)  Interrupted(RouteChanged)    RouteChanged
RouteRerouted             FailRecoverable(RouteChanged)* none (stream keeps flowing) none
```

\* unreachable at start by contract; the mapping stays total so
`Preparing` can never be stranded. A rerouted default endpoint is not a
fault because a loopback stream built from the default endpoint follows
the OS reroute and stays active; interrupting it would be a false stop.
A test asserts that the event mapping and the health mapping agree for
every fault class, and (in grain 2) that the Windows loopback classifier
agrees with the microphone adapter's classification of the same cpal
kinds, so the two Windows pathways cannot silently diverge.

## Tests (portable, run on ubuntu + macOS + Windows CI)

```text
stable_ids_match_for_equal_endpoints_and_differ_otherwise
system_ids_are_domain_separated_from_every_other_adapter
selection_prefers_endpoint_id_then_name_then_default
selection_falls_back_to_flagged_default_then_first
empty_enumeration_and_enumeration_faults_stay_typed
labels_truncate_on_char_boundary_with_system_audio_fallback
start_failures_map_to_preparing_accepted_events
runtime_faults_map_to_flowing_accepted_events
rerouted_default_endpoint_keeps_flowing_without_a_machine_event
runtime_health_reason_agrees_with_the_machine_mapping
exclusive_mode_conflict_surfaces_through_006b_health
adapter_drives_session_without_refusal_and_recovers
error_classifiers_are_stable_and_distinct
```

The mapping tests feed every fault class into a live `CaptureSession` and
assert the machine accepts it, which is the anti-refusal property the 006A
contract requires. The health test drives `HealthMonitor` from a baseline
sample to an exclusive-mode-conflict sample and asserts a single
`ReasonChanged` edge, so a shared/exclusive conflict is surfaced instead
of silently stopping or silently continuing.

## What is not claimed

- No endpoint is opened, so there is no Windows runtime, first-audio,
  loopback, or Gate E claim; `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- No loopback of protected content is attempted or promised; endpoint
  and content policy behavior is an 008C evidence-matrix obligation.
- Non-F32 negotiation stays refused until Gate E evidence exists for the
  exact negotiated format.
