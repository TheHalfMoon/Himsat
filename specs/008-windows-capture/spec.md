# Specification 008 — Windows Capture

```text
LIFECYCLE = SHAPING
RISK = R3_CROSS_PLATFORM_CAPTURE
DEPENDS_ON = 001_CLOSED_CANONICAL, 002_CLOSED_CANONICAL, 003_CLOSED_CANONICAL, 004_CLOSED_CANONICAL, 005_CLOSED_CANONICAL, 006_CLOSED_CANONICAL, 007_CLOSED_CANONICAL
IMPLEMENTATION_AUTHORITY = NONE_UNTIL_SHAPING_QUALIFIES
```

## Problem

The portable capture contract (006) and the first platform adapter
(007, macOS) are proven. Windows is the next required desktop
platform, and it differs materially from macOS: microphone privacy
is enforced by an OS capability check with a distinct denied state,
system audio is captured by a sanctioned in-box loopback mode
rather than a tap-and-aggregate composition, endpoint routing is
enumerated through cpal but resolved through Windows device
semantics, and the `audio` feature dependency edge is already
carried by the workspace. Specification 008 binds the same 006A
session machine, 006B health telemetry, and 006C checkpoint
discipline to Windows without weakening the portable contract and
without claiming macOS behavior on Windows or the reverse
(architecture; master-plan unit 008; Constitution XI).

## Canonical dependency state

Specification 007 is `CLOSED_CANONICAL` at closeout merge
`c96abba6ed5cd760611d0f407e7c3a8df9fb1879` with post-merge CI
`35627161924` / R3 `35627161891` SUCCESS. Specifications 001-006
remain `CLOSED_CANONICAL` beneath it. Shaping starts from that exact
canonical `main`.

Consumable contracts (no reinvention, no rewriting):

- 006A session machine (`capture_session`): portable states,
  transitions, and typed reason codes; attach/detach, route change,
  permission revoked, projection/interruption, silence, and
  disconnect/reconnect are already-classified inputs the adapter
  maps onto, never redefines;
- 006B health telemetry (`capture_health`): monitor, snapshot,
  classifier, and `CaptureHealthChanged` event semantics; thresholds
  stay caller budgets; level smoothing and hysteresis stay
  adapter-side;
- 006C checkpoint and metadata (`capture_checkpoint`): derivation
  from recovery outputs, loss accounting, latest-wins rule, and the
  chunk metadata mapping onto 005A envelope fields;
- 005 journal discipline (open/commit/close, torn-tail reporting,
  resume refusals), bounded envelope codec, and the reconcile
  taxonomy as the durable sink for captured audio;
- 004 manifest inventory / freshness / backup / deletion boundaries
  for captured chunks; the Windows protector adapter and its
  freshness semantics are owned by 004 and are consumed, not
  redefined, here;
- 003 session/source/artifact/event identities for captured
  artifacts;
- 007 macOS adapter shapes, used as a reference implementation of
  the same contract on a different OS (compare-only; Windows code
  must not port macOS-specific mechanism).

## Scope in

- microphone capture pathway on Windows using current supported
  APIs (cpal over WASAPI; exact API/SDK pins refreshed at
  implementation time against the then-current toolchain);
- authorized system-audio pathway on Windows using the OS-sanctioned
  WASAPI loopback mechanism only; no private-API or entitlement-
  escape capture, no universal audio-access claims;
- Windows endpoint enumeration, selection, stable 003 source
  identity, and fault classification over the closed `cpal` binding;
- OS-event mapping: endpoint/default change, device connect/disconnect
  (including Bluetooth/USB), microphone privacy denial/revocation,
  shared/exclusive-mode conflict, and audio-session interruption
  mapped onto 006A states and typed reasons with evidence per mapping;
- permission and policy lifecycle: microphone privacy
  granted/denied/revoked and exclusive-access conflict surfaced
  through 006B health, never a silent stop, with no plaintext or
  silent-fallback workaround;
- long-session operation: long capture with 006C checkpoint cadence,
  storage-pressure refusal behavior, and loss accounting end to end;
- evidence matrix: endpoint changes, exclusive-mode conflicts,
  Bluetooth/USB device changes, permission changes, and long
  sessions, each with positive and adversarial cases (master-plan
  unit 008 evidence contract; Gate E platform qualification before
  any Windows support claim).

## Scope out

- macOS (007, `CLOSED_CANONICAL`) and Linux (009) adapters; nothing
  in 008 constrains their later shaping except the shared 006
  contract;
- iOS/iPadOS and Android (016/017);
- codecs, DSP, AEC/NS/AGC, VAD, resampling (010); the adapter
  delivers captured frames, it does not process them;
- transcription, diarization, models, intelligence (011+);
- UI surfaces, notifications, meeting detection (015);
- sync/pairing/relay, connectors, plugins, network egress;
- custom cryptographic primitives or new KDFs; the 004 Windows
  protector/freshness adapter is consumed, not extended here;
- rewriting any reviewed 003 identity, 004 envelope/manifest/
  rotation/backup/deletion, 005 journal/chunk/recovery, or 006
  session/health/checkpoint/metadata boundary;
- release/compliance claims without separate qualification.

## Donor evaluation posture

Recorder-state, WASAPI loopback, endpoint-routing, and
engine-abstraction patterns in `Zackriya-Solutions/meetily`,
`debpalash/VoiceStudio`, and any stronger covered source available
at shaping time are recorded planning inputs (see
`docs/donor-and-provenance.md` and the Windows-relevant research
records). Implementation leaves must COMPARE selective
COPY/ADAPT against a Himsat-native alternative on engineering merit
with exact revision/path/permission/provenance evidence before any
adoption, under the 004P-style adoption gate for any new external
dependency. Shaping adopts nothing.

## Carried inputs (not blockers)

- Pre-existing backup staging tests remain flaky on CI runners
  across the lineage (006 closeout residual 1); the `windows-latest`
  Rust job must distinguish product loss accounting from runner
  flakes and must not claim a silent repair.
- Sequence coordination across multiple monitors stays caller-owned
  under 003 order (006 closeout residual 2); the adapter starts each
  monitor instance at zero and documents the boundary.
- Non-F32 negotiation stays refused on Windows until Gate E evidence
  exists for exactly the negotiated format, mirroring 007.

## Acceptance criteria

Specification 008 shaping is complete when this spec/plan/tasks
packet is exact-head qualified (CI + R3 SUCCESS), reconciled,
expected-head merged, and post-merge qualified. Implementation
leaves are re-bounded only after shaping qualifies; no
implementation authority opens in this unit.
