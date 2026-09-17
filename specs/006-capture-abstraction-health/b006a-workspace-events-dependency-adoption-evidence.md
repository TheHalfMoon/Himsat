# B006A Workspace-Events Dependency Adoption Evidence

## Decision

`himsat-core` gains one intra-workspace direct dependency,
`himsat-events` (`version = "=0.0.0"`, `path = "../himsat-events"`),
so the 006A capture session owns 003 identities (`SessionId`,
`SourceId`) in its source descriptors, session binding, and
transplant-refusal errors instead of reinventing them.

## Alternatives considered and rejected

- Duplicate identity types in core: reinvents the closed 003
  contract and violates the additive-only boundary discipline.
- Generic identity parameters on the session machine: fights the 003
  consumable contract for zero gain and a worse adapter API.
- Place the machine in `himsat-events`: behavior belongs in the
  portable core; 006C checkpoint operations need core journal and
  manifest types regardless, so the edge is inevitable and is faced
  here, once.

## Impact (verified, not claimed)

- Zero new external packages: `lock_external` stays at 161.
- Zero transitive dependencies: `himsat-events` declares no
  `[dependencies]`.
- Same license, version, edition, and rust-version (workspace
  inheritance); no new source, registry, checksum, SBOM, or
  third-party notice.
- Lockfile gains one path edge only; `provenance_gate validate` and
  `check-generated` pass unchanged.
- `tools/004p_dependency_closure.py` gains the exact manifest table
  and the full closure passes (`004P P005/P006 CLOSURE PASS`).

## Donor posture

No donor code is adopted or adapted; the donor compare for the 006A
machine itself is recorded in the adoption PR body.

## Verification

Closure, provenance validate, check-generated, fmt, clippy
(`-D warnings`), and the full workspace test suite were run on the
exact adoption head; the Diffcipline proof carries the verdict and
verification evidence under the B006A exception.
