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

## 007C leaf declaration (shaped; exact split re-bounds after shaping qualifies)

- Outcome: the system-tap pathway streams first audio into the
  005 journal through a sanctioned sample path, with the
  lifecycle evidence matrix proven on live hardware.
- `scope_in`: sanctioned streaming decision (how samples cross
  the IOProc boundary under `forbid(unsafe_code)`, decided here
  as Gate E evidence, implemented in the grain); aggregate-device
  assembly and proof; permission granted/denied/revoked,
  route-change, device-yank, sleep/wake, interruption,
  long-session checkpoint/loss-accounting, and storage-pressure
  refusal evidence rows, each positive plus adversarial, mapped
  onto 006A states and surfaced through 006B health (never a
  silent stop); non-F32 negotiation only with Gate E evidence.
- `scope_out`: codecs/DSP/resampling (010), transcription and
  beyond (011+), UI (015), Windows/Linux adapters (008/009),
  release/compliance claims, any narrowing of the portable 006
  contract to fit macOS.
- Dependencies: no new external dependency anticipated — cidre
  0.29.0 (closed, registry 211) already covers the tap plus
  aggregate-device surface; exact API/SDK pins refresh at
  implementation time against the then-current toolchain. If
  implementation finds otherwise, a new 004P-style adoption gate
  is required before any manifest change.
- Acceptance: exact-head CI + R3 SUCCESS (gate exception only
  for a newly authorized adoption); cargo trio green; live
  darwin/arm64 matrix all rows evidenced; loss account
  reconciles to zero unexplained; no silent stops.
- Risk: the streaming sanction is the hard unknown — if no
  safe path exists, 007C narrows to authorization-plus-matrix
  evidence and streaming waits for a sanctioned primitive
  (recorded, not redefined).
- Recovery: matrix failure preserves failed evidence and
  repairs forward; platform truth contradicting the plan
  narrows claims to what the current OS proves.
- Context budget: docs-only shaping here; implementation grains
  stay under the 900-line gate each, split rather than excepted
  wherever possible.
- Change surface: `crates/himsat-core/src/capture_system_audio.rs`
  (streaming + matrix hooks), tests, evidence files, tasks
  bindings; manifest/lockfile/closure only through a new gate.
- Evidence requirements: per-row live proof plus the R3
  verification commands green; donor posture stays
  compare-only (anarlog/meetily shape references, no code).
- Minimality: reuse the closed cidre binding and the 006
  session/health/checkpoint machinery; no new crate, no new
  platform, no new contract.
- Safety/security: OS-sanctioned tap mechanism only, no
  private API or entitlement escape; microphones/taps never
  persist beyond their guard; permission-denied and revoked
  paths are first-class evidence, not error branches.
- Candidate grain split (not Grains until readiness): 007C-1
  streaming sanction + first-audio evidence; 007C-2 aggregate
  device; 007C-3 lifecycle matrix. No 007C conclusion is
  recorded until its own evidence is constructed and verified.

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
