# Specification 001 Plan — Repository and Delivery-Control Foundation

## Objective

Turn the planning-only Himsat repository into the smallest reproducible Rust engineering repository that can prove future changes with SpecGrain and Diffcipline without introducing any Himsat product behavior or donor material.

## Canonical base

```text
base = 3f6687b34530e55098f7854042b12adfa607f394
```

Specification 000 is canonical at that merge. This shaping plan must be revised if live `main` changes before implementation begins.

## Design choices

### 1. Rust-only initial implementation surface

Use Rust 1.98.1 and edition 2024 for the first code workspace.

The first crate is deliberately dependency-free. Its purpose is to prove repository plumbing, not to choose audio, storage, UI, serialization, async, database, or AI libraries prematurely.

### 2. No Tauri or Node yet

Tauri/React is still the likely desktop direction, but a desktop app shell is downstream of capture/platform work and is not needed to prove the repository foundation. Node.js 24.20.0 is current LTS at shaping time but remains unpinned in Himsat until a JS/TS surface is actually selected.

### 3. Tracked SpecGrain local state starts conservative

Create a native `.specgrain` store with:

- project ID `himsat`;
- default policy;
- readiness mode `report` initially;
- no fabricated native `GRAIN` record.

Validate tracked state using exact SpecGrain source revision `faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed`.

The Markdown program/specification system remains repository governance. Native SpecGrain lifecycle state may be introduced only through actual validated state transitions.

### 4. Diffcipline owns the implementation finish line

Create `.diffcipline.toml` with a narrow change-surface contract and cumulative verification. R2 verification must at minimum execute locked clippy and workspace tests; formatting is also required.

The exact final policy should account for the implementation files actually selected and fail closed on unexpected dependency-manifest, lockfile, or untracked changes.

### 5. CI is intentionally boring

The foundation CI should prioritize portability and reproducibility:

- Linux;
- macOS;
- Windows;
- checked-in Rust toolchain pin;
- `cargo fmt`;
- `cargo clippy`;
- `cargo test`;
- SpecGrain validation in a dedicated deterministic control job;
- Diffcipline exact-diff proof in the PR path when technically compatible with the workflow event.

No release, coverage SaaS, telemetry, cloud test service, model download, package publication, or installer build belongs here.

Workflow permissions must default to read-only contents unless a specific check requires more.

### 6. Apache-2.0 for Himsat-owned source

Specification 001 should establish Apache-2.0 for new Himsat-owned implementation. It provides a permissive open-source license with an explicit patent grant.

This license does not relicense copied donor code. Future provenance records retain each donor's controlling license and notice obligations.

## Implementation sequence

### Phase A — repository state and policy

1. reverify live canonical base;
2. create `.gitignore` appropriate to Rust/local tooling without hiding evidence files that should be tracked;
3. add Apache-2.0 `LICENSE`;
4. add minimal `CONTRIBUTING.md` and `SECURITY.md`;
5. add `.specgrain` store and validate its exact schema against the pinned SpecGrain source;
6. add `.diffcipline.toml` with initially bounded scope and R2 commands.

### Phase B — minimal Rust workspace

1. add `rust-toolchain.toml` pinned to `1.98.1` with required components for fmt/clippy;
2. add root Cargo workspace manifest;
3. add `crates/himsat-core` with no dependencies;
4. add one or more deterministic smoke/unit tests proving the crate compiles/tests on all target hosts;
5. generate and commit `Cargo.lock` if Cargo creates it for the workspace, then treat it as exact evidence.

The crate must not define meeting/audio/document/memory domain APIs merely to appear productive.

### Phase C — CI

1. add one bounded GitHub Actions workflow;
2. pin GitHub actions to immutable SHAs where practical;
3. run Rust verification across Linux/macOS/Windows;
4. run SpecGrain state validation using the pinned SpecGrain source revision;
5. integrate Diffcipline exact-diff verification without granting write permissions or weakening the local policy;
6. preserve failures as evidence and repair forward.

### Phase D — qualification and reconciliation

1. run focused/local verification where the environment permits;
2. push exact candidate;
3. inspect all CI cells on exact head;
4. inspect Diffcipline proof and SpecGrain check output;
5. inspect dependency/license/diff closure;
6. reconcile review comments/threads;
7. recheck base/head/scope/mergeability immediately before merge;
8. merge with expected-head protection;
9. re-read canonical `main`;
10. require post-merge applicable verification before canonical closeout.

## Expected files

Implementation may refine this list downward but should not expand it casually:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
crates/himsat-core/Cargo.toml
crates/himsat-core/src/lib.rs
.gitignore
.github/workflows/ci.yml
.specgrain/project.json
.specgrain/policies/default.json
.diffcipline.toml
LICENSE
CONTRIBUTING.md
SECURITY.md
specs/001-repository-foundation/spec.md
specs/001-repository-foundation/plan.md
specs/001-repository-foundation/tasks.md
specs/CURRENT.md
```

If a helper script or workflow support file becomes genuinely necessary, record the reason and update the spec/change-surface before calling the candidate ready.

## Verification command baseline

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
```

SpecGrain and Diffcipline commands must be recorded with their exact pinned revisions and actual outputs in the PR evidence. Do not copy example output into a claim.

## Failure cases to test

At minimum verify that the control plane fails/reviews correctly when feasible for this unit:

- formatting violation;
- clippy warning promoted to error;
- failing Rust test;
- malformed SpecGrain state;
- unexpected path outside Diffcipline expected surface;
- changed dependency/lockfile state that policy marks for review/failure;
- verification configured but not run;
- unexpected untracked file in a local proof environment.

These may be deterministic policy fixtures or bounded temporary mutations; do not commit deliberately broken code to canonical main merely to create evidence.

## Residual risks

- GitHub repository administration/ruleset enforcement cannot be assumed from repository files alone.
- GitHub-hosted runner images change over time; exact workflow/action/toolchain pins reduce but do not remove host drift.
- Installing SpecGrain from a GitHub source SHA requires network access in CI; a later supply-chain unit may improve artifact pinning/offline verification.
- Diffcipline v1 action/CLI integration needs exact event/base semantics verified in Himsat rather than copied from examples blindly.
- Apache-2.0 is a project governance choice, not legal advice about every future donor/model/data artifact.

## Closeout rule

Specification 001 is not closed merely because the files exist. Closeout requires exact-head successful execution of the declared checks, review of failures/negative evidence, expected-head merge, and post-merge canonical verification.
