# Specification 006 Plan — Capture Abstraction and Capture Health

## Prospective implementation leaves

The exact split is re-bounded only after shaping qualifies and the first leaf reconciles against then-live canonical truth. Current candidates (not Grains until SpecGrain readiness establishes them):

```text
006A source lifecycle and session state machine (attach/detach/route
  discipline, portable states and typed reasons, UI-distinguishable health)
006B health telemetry model and events (signal set, reporting discipline,
  CaptureHealthChanged semantics, backpressure and storage pressure)
006C checkpoint and chunk metadata binding (close-plus-bind checkpoint
  unit, last-durable query, loss accounting, metadata schema onto 005)
```

Each leaf must declare outcome, `scope_in`/`scope_out`, dependencies, acceptance, risk, recovery path, context budget, change surface, evidence requirements, minimality rationale, and safety/security implications before implementation begins.

## R3 verification strategy

- pinned provider/toolchains; fmt/lint/build/tests on claimed targets;
- exact dependency/license/SBOM/provenance closure (no new external dependency without a 004P-style adoption gate);
- positive: source attach/detach round-trip, state-machine transition coverage, checkpoint write/read-back, metadata bind round-trip onto 005;
- adversarial: route change mid-session, permission revoked mid-session, source silence, adapter death (kill-matrix pattern reused), torn journal beneath a live session, storage-pressure refusal, malformed metadata rejection, transplanted source descriptors;
- platform: per-OS adapter behavior evidenced on its own OS only (XI); the portable contract never promises what a platform cannot prove;
- deletion: source/hook removal through the B506 boundary with reread verification.

## Diffcipline scope control

Shaping is docs/governance only: no product code, Cargo manifest/lockfile, provenance-adoption, SBOM, workflow, donor, model, dataset, or asset change. The shaping packet admits `specs/006-capture-abstraction-health/**` to expected_files (forward-only scope authorization mirroring the 005 precedent) plus the shaping ledger line. If shaping exceeds diff bounds, split it rather than weakening bounds.

## Recovery plan

- shaping review finds a design flaw: revise forward and re-qualify the exact new head;
- implementation failure later: preserve failed evidence and repair forward;
- platform truth contradicts a portable promise: narrow the contract to what platforms prove, never widen the promise by redefinition.

## Closeout rule

Specification 006 becomes `CLOSED_CANONICAL` only after reviewed shaping, bounded implementation leaves, R3 adversarial/platform evidence, reconciliation, expected-head merges, post-merge verification, and durable closeout evidence all close. Only then may Specification 007 shaping begin.
