# Specification 007 Plan — macOS Capture

## Prospective implementation leaves

The exact split is re-bounded only after shaping qualifies and the first leaf reconciles against then-live canonical truth. Current candidates (not Grains until SpecGrain readiness establishes them):

```text
007A microphone capture pathway (device enumeration, stream start/
  stop/abandon mapped onto the 006A session machine, format
  negotiation, first-audio evidence)
007B authorized system/screen-audio pathway (OS-sanctioned mechanism
  only, authorization lifecycle, mixed-or-separate routing decision
  with reason, no private-API capture)
007C lifecycle evidence matrix (permission granted/denied/revoked,
  route change, device yank, sleep/wake, interruption, long-session
  checkpoint/loss accounting, storage-pressure refusal)
```

Each leaf must declare outcome, `scope_in`/`scope_out`, dependencies, acceptance, risk, recovery path, context budget, change surface, evidence requirements, minimality rationale, and safety/security implications before implementation begins.

## R3 verification strategy

- pinned provider/toolchains; fmt/lint/build/tests on claimed targets including macos-latest (Gate E: the platform claim is proven on the platform, never by redefinition);
- exact dependency/license/SBOM/provenance closure: any Apple-API binding crate or new external dependency enters only through a 004P-style adoption gate with exact revision/path/permission/provenance evidence;
- positive: microphone start/stop round-trip into the 005 journal, route-change survival, permission grant flow, sleep/wake resume, multi-hour session with checkpoint cadence and loss account reconciling to zero unexplained;
- adversarial: device yank mid-session, permission revoked mid-session, sleep mid-session, exclusive-access conflict, sample-rate/format change mid-stream, clock drift and discontinuity injection, torn journal beneath a live session, storage-pressure refusal, transplanted source descriptors;
- platform: macOS adapter behavior evidenced on macOS only (XI); the portable 006 contract never promises what macOS cannot prove, and 007 never narrows the portable contract to fit one platform;
- deletion: capture artifacts and hooks removed through the B506 boundary with reread verification.

## Diffcipline scope control

Shaping is docs/governance only: no product code, Cargo manifest/lockfile, provenance-adoption, SBOM, workflow, donor, model, dataset, or asset change. The shaping packet admits `specs/007-macos-capture/**` to expected_files (forward-only scope authorization mirroring the 005/006 precedent) plus the shaping ledger line. If shaping exceeds diff bounds, split it rather than weakening bounds.

## Recovery plan

- shaping review finds a design flaw: revise forward and re-qualify the exact new head;
- implementation failure later: preserve failed evidence and repair forward;
- platform truth contradicts the plan (API deprecation, permission-model change, background-policy change): refresh pins at implementation time and narrow claims to what the current OS proves, never widen the promise by redefinition.

## Closeout rule

Specification 007 becomes `CLOSED_CANONICAL` only after reviewed shaping, bounded implementation leaves, R3 adversarial/platform evidence, reconciliation, expected-head merges, post-merge verification, and durable closeout evidence all close. Only then may Specification 008 shaping begin.
