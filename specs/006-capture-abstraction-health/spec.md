# Specification 006 — Capture Abstraction and Capture Health

```text
LIFECYCLE = SHAPING
RISK = R2
DEPENDS_ON = 003_CLOSED_CANONICAL, 004_CLOSED_CANONICAL, 005_CLOSED_CANONICAL
IMPLEMENTATION_AUTHORITY = NONE_UNTIL_SHAPING_QUALIFIES
```

## Problem

Platform capture (007+) needs a stable portable contract to implement.
Without one, every adapter invents its own source lifecycle and health
semantics, and the product cannot distinguish "recording indicator is
on" from "meaningful source audio is reaching durable storage"
(architecture). Specification 006 defines the portable source
lifecycle, the capture session state machine, the health telemetry
model, the durable checkpoint, and the chunk metadata binding over the
closed 005 substrate — so adapters are interchangeable, health is
comparable across platforms, and loss is always accounted, never
silent.

## Canonical dependency state

Specification 003 is `CLOSED_CANONICAL` (session/source/event/object
identities) at closeout merge `1f14bbe004602e6e6c7f9c046cf5b489`.
Specification 004 is `CLOSED_CANONICAL` at closeout merge
`c5255f6a60044edbdd7562fe1f295ce59e14c5a0` with post-merge CI
`35091125997` and R3 `35091125863` SUCCESS. Specification 005 is
`CLOSED_CANONICAL` at closeout merge `85b2cb2ee5b7f8b043c66920b7ebc465edf48504`
with post-merge CI `35244997886` and R3 `35244997858` SUCCESS. Shaping
starts from that exact canonical `main`.

Consumable contracts (no reinvention, no rewriting):

- 003 session/source/artifact/event identities (`himsat-events`):
  sources attach to sessions, chunks land as artifacts, health flows as
  typed events with sequence order;
- 004 B501A manifest inventory (media chunks join through the additive
  kind proven by 005C), B501 freshness (session epochs advance under
  existing anchor rules), B505 backup (chunks are restorable content),
  B506 deletion (chunks fall under existing surface families);
- 005 journal discipline (open/commit/close markers, torn-tail
  reporting, resume refusals), bounded envelope codec, reconcile
  taxonomy (verified/orphan/duplicate/unbound/diverged/unlogged/
  unreferenced/torn/mismatch/close-count), bounded-loss window (at most
  the single in-flight tail record, kill-matrix proven);
- architecture reliability states, durable media format direction,
  audio pipeline stage order, session model, and candidate health
  events as the shaping baseline;
- product-plan Capture Health signal list as the telemetry scope.

## Scope in

- source lifecycle contract: attach/detach, route change, permission
  revoked, projection stopped, source silent, disconnect/reconnect;
  opaque source descriptors carrying no more identity than the user
  authorized (XIV minimization);
- capture session state machine: portable states, transitions, and
  typed reason codes over the architecture candidates, with healthy,
  degraded, interrupted, recovering, and terminally failed
  distinguished for UI and telemetry;
- health telemetry model: signal set, sampling/reporting discipline,
  `CaptureHealthChanged` event semantics with sequence order, plus
  backpressure and storage-pressure signals;
- durable checkpoint definition: journal close plus manifest bind as
  the checkpoint unit, last-durable-checkpoint query, loss accounting
  surfaced from the 005D bounds (never silent);
- chunk metadata schema: timestamp range, codec/sample format,
  source/device id, digest, chain-reference decision (tamper-evident
  chain selected or explicitly deferred with reason), mapped onto 005A
  envelope fields and the manifest blob inventory.

## Scope out

- platform adapters, permissions plumbing, background services
  (007+);
- codecs, DSP, AEC/NS/AGC, VAD, transcription, diarization, models
  (010+);
- UI surfaces, notifications, lock-screen controls;
- sync/pairing/relay, connectors, plugins, network egress;
- custom cryptographic primitives or new KDFs;
- rewriting any reviewed 003 identity, 004 envelope/manifest/rotation/
  backup/deletion, or 005 journal/chunk/recovery boundary;
- physical secure erase or universal metadata-secrecy claims;
- release/FIPS/compliance claims without separate qualification.

## Donor evaluation posture

Recording-manager, source-lifecycle, recorder-state, and file-queue
patterns in `Zackriya-Solutions/meetily`, `fastrepl/anarlog`
(non-enterprise paths), and `Starmel/OpenSuperWhisper` are recorded
planning inputs (see `docs/donor-and-provenance.md`). Implementation
leaves must COMPARE selective COPY/ADAPT against a Himsat-native
alternative on engineering merit with exact revision/path/permission/
provenance evidence before any adoption. Shaping adopts nothing.

## Acceptance criteria

Specification 006 shaping is complete when this spec/plan/tasks packet
is exact-head qualified (CI + R3 SUCCESS), reconciled, expected-head
merged, and post-merge qualified. Implementation leaves are re-bounded
only after shaping qualifies; no implementation authority opens in this
unit.
