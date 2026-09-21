# Specification 007 — macOS Capture

```text
LIFECYCLE = CLOSED_CANONICAL
RISK = R3_RECORDING_LIFECYCLE
DEPENDS_ON = 003_CLOSED_CANONICAL, 004_CLOSED_CANONICAL, 005_CLOSED_CANONICAL, 006_CLOSED_CANONICAL
IMPLEMENTATION_AUTHORITY = NONE_SPEC_007_COMPLETE
CLOSEOUT = 007-macos-capture-closeout-final-evidence.md
NEXT = SPEC_008_WINDOWS_CAPTURE_SHAPING
```

## Problem

The portable capture contract (006) is proven only against fixtures
and the host test runner. The first platform adapter must prove that
contract against a real operating system: live microphone input and
the authorized system/screen-audio pathway on macOS, with route
changes, permission transitions, sleep/wake cycles, and long sessions
all flowing through the 006 session machine, health telemetry, and
durable checkpoint — so "recording" on macOS means audio reaching
durable storage with loss always accounted, never silent
(architecture; master-plan unit 007).

## Canonical dependency state

Specification 006 is `CLOSED_CANONICAL` at closeout merge
`ea5a5e4b43dcc7e9de756a9ea3ce619179718687` with post-merge CI
`35275083648` and R3 `35275083661` SUCCESS. Specifications 003, 004,
and 005 remain `CLOSED_CANONICAL` beneath it. Shaping starts from
that exact canonical `main`.

Consumable contracts (no reinvention, no rewriting):

- 006A session machine (`capture_session`): portable states,
  transitions, and typed reason codes; attach/detach, route change,
  permission revoked, projection stopped, silence, disconnect/
  reconnect are already-classified inputs the adapter maps onto,
  never redefines;
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
  for captured chunks;
- 003 session/source/artifact/event identities for captured
  artifacts.

## Scope in

- microphone capture pathway on macOS using current supported Apple
  APIs (exact API/SDK pins refreshed at implementation time against
  the then-current toolchain);
- authorized system/screen-audio pathway on macOS through the
  OS-sanctioned mechanism only; no private-API or entitlement-escape
  capture, no universal audio-access claims;
- OS event mapping: route change, device connect/disconnect,
  permission granted/denied/revoked, audio-session interruption,
  sleep/wake, and background-policy transitions mapped onto 006A
  states and typed reasons with evidence per mapping;
- permission lifecycle: request flow, denied and revoked handling,
  re-grant recovery, all surfaced through 006B health (never a
  silent stop);
- long-session operation: multi-hour capture with checkpoint cadence
  from 006C, storage-pressure refusal behavior, and loss accounting
  end to end;
- evidence matrix: route changes, permissions, sleep/wake, and long
  sessions, each with positive and adversarial cases (master-plan
  unit 007 evidence contract; Gate E platform qualification before
  any support claim).

## Scope out

- Windows and Linux adapters (008/009); nothing in 007 constrains
  their later shaping except the shared 006 contract;
- codecs, DSP, AEC/NS/AGC, VAD, resampling (010); the adapter
  delivers captured frames, it does not process them;
- transcription, diarization, models, intelligence (011+);
- UI surfaces, notifications, lock-screen controls, meeting
  detection (015);
- sync/pairing/relay, connectors, plugins, network egress;
- custom cryptographic primitives or new KDFs;
- rewriting any reviewed 003 identity, 004 envelope/manifest/
  rotation/backup/deletion, 005 journal/chunk/recovery, or 006
  session/health/checkpoint/metadata boundary;
- release/compliance claims without separate qualification.

## Donor evaluation posture

Recorder-state, file-queue, engine-abstraction, and macOS
dictation/capture patterns in `Starmel/OpenSuperWhisper`,
`Zackriya-Solutions/meetily` (desktop audio/recording paths), and
`fastrepl/anarlog` (community non-enterprise paths), plus native API
patterns and any stronger covered source available at shaping time,
are recorded planning inputs (see `docs/donor-and-provenance.md`
and `docs/research/2026-09-07-superwhisper-reference.md`).
Implementation leaves must COMPARE selective COPY/ADAPT against a
Himsat-native alternative on engineering merit with exact revision/
path/permission/provenance evidence before any adoption, under the
004P-style adoption gate for any new external dependency. Shaping
adopts nothing.

## Carried inputs (not blockers)

- Pre-existing backup staging tests remain flaky on CI runners
  across the lineage (006 closeout residual 1); long-session
  evidence must distinguish product loss accounting from runner
  flakes, and must not claim a silent repair.
- Sequence coordination across multiple monitors stays caller-owned
  under 003 order (006 closeout residual 2); the adapter starts each
  monitor instance at zero and documents the boundary.

## Acceptance criteria

Specification 007 shaping is complete when this spec/plan/tasks
packet is exact-head qualified (CI + R3 SUCCESS), reconciled,
expected-head merged, and post-merge qualified. Implementation
leaves are re-bounded only after shaping qualifies; no
implementation authority opens in this unit.
