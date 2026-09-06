# Specification 001 Tasks — Repository and Delivery-Control Foundation

> Checkboxes record authoring/reconciliation progress. Durable proof is recorded in `evidence.md`.

## Shaping

- [x] S001 Re-read canonical `main` after Specification 000 merge.
- [x] S002 Confirm Specification 000 merge commit and exact parent lineage.
- [x] S003 Re-read canonical governance and active plan.
- [x] S004 Confirm the repository lacked executable delivery controls before this unit.
- [x] S005 Select Rust 1.98.1 and defer Node/Tauri.
- [x] S006 Pin SpecGrain and Diffcipline revisions.
- [x] S007 Define bounded R2 scope, recovery, acceptance, evidence, minimality, and safety.
- [x] S008 Qualify and merge shaping as `c7fadaebfc44a60f982763bfc20e3a7d41551f4e`.

## Implementation

- [x] I001 Add Apache-2.0, contribution/security guidance, `.gitignore`, and local control state.
- [x] I002 Add Rust 1.98.1 / Rust 2024 workspace and dependency-free `himsat-core`.
- [x] I003 Add least-privilege Linux/macOS/Windows CI.
- [x] I004 Validate tracked SpecGrain report-mode state at pinned revision.
- [x] I005 Execute Diffcipline R2 proof at pinned revision.
- [x] I006 Add and execute negative controls for formatting, clippy, tests, SpecGrain corruption, missing proof execution, and out-of-scope paths.
- [x] I007 Confirm locked dependency tree contains no third-party package.

## Qualification and closeout

- [x] Q001 Exact implementation head `34a0b1a4b7a5f4873a6de7141f160de45fd37740` qualified by CI run `34042429773`.
- [x] Q002 Expected-head implementation merge `4d1f8843577059d5cfa0cfeeb97df05feb2da8be`.
- [x] Q003 Post-implementation-merge run `34042553778` succeeded.
- [x] Q004 Tighten Cargo manifest/lockfile policy from bootstrap `allow` to `review`.
- [x] Q005 Exact closeout head `c21b611de8b799dceea148a86578fa53028458d2` qualified by run `34042761301`.
- [x] Q006 Expected-head closeout merge `90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0`.
- [x] Q007 Re-read canonical `main` and observe post-closeout run `34042822934` succeed across all configured jobs.
- [x] Q008 Record exact evidence and close Specification 001 canonical.

## Explicit non-tasks

Specification 001 did not copy donors, add speech/model dependencies, create product app shells, implement vault/storage/documents/memory/agents/sync, publish releases, or claim platform superiority.
