# Specification 005 Plan — Media Journal and Chunk Store

## Prospective implementation leaves

The exact split is re-bounded only after shaping qualifies and the first leaf reconciles against then-live canonical truth. Current candidates (not Grains until SpecGrain readiness establishes them):

```text
005A media chunk envelope binding (purpose id, layout, ceiling proof, wrong-key/transplant rejection)
005B session journal codec and fsync discipline (markers, commit records, truncation safety)
005C crash-recovery reconciliation (scan, orphan/duplicate resolution, manifest rebind)
005D bounded-loss quantification and fault-injection evidence (kill-style faults, corruption fixtures)
```

Each leaf must declare outcome, `scope_in`/`scope_out`, dependencies, acceptance, risk, recovery path, context budget, change surface, evidence requirements, minimality rationale, and safety/security implications before implementation begins.

## R3 verification strategy

- pinned provider/toolchains; fmt/lint/build/tests on claimed targets;
- exact dependency/license/SBOM/provenance closure (no new external dependency without a 004P-style adoption gate);
- positive: chunk encrypt/decrypt round-trip, journal append/replay, clean restart recovery with zero loss beyond the quantified tail;
- adversarial: process-kill at every journal commit point, torn/truncated/corrupt journal tails, orphaned/duplicated chunks, wrong-key and cross-session transplant chunks, oversized chunk rejection at the B202 ceiling, opaque-filename metadata scan;
- platform: fsync/durability semantics evidenced per claimed OS; no universal durability claim beyond what each platform proves;
- deletion: chunk/journal removal through the B506 boundary with reread verification.

## Diffcipline scope control

Shaping is docs/governance only: no product code, Cargo manifest/lockfile, provenance-adoption, SBOM, workflow, donor, model, dataset, or asset change. If shaping exceeds diff bounds, split it rather than weakening bounds.

## Recovery plan

- shaping review finds a design flaw: revise forward and re-qualify the exact new head;
- implementation failure later: preserve failed evidence and repair forward;
- fault-injection proves loss beyond the bounded window: redesign the journal discipline, never weaken the bound by redefinition.

## Closeout rule

Specification 005 becomes `CLOSED_CANONICAL` only after reviewed shaping, bounded implementation leaves, R3 adversarial/platform evidence, reconciliation, expected-head merges, post-merge verification, and durable closeout evidence all close. Only then may Specification 006 shaping begin.
