# Specification 001 Tasks — Repository and Delivery-Control Foundation

> Checkboxes record authoring/reconciliation progress. They are not substitutes for exact CI, SpecGrain, or Diffcipline evidence.

## Shaping

- [x] S001 Re-read canonical `main` after Specification 000 merge.
- [x] S002 Confirm Specification 000 merge commit and exact parent lineage.
- [x] S003 Re-read `AGENTS.md`, constitution, `specs/CURRENT.md`, and execution master plan.
- [x] S004 Confirm live repository has no Rust workspace, CI, native SpecGrain store, or Diffcipline policy.
- [x] S005 Recheck current Rust stable and reject the affected 1.98.0 compiler in favor of 1.98.1.
- [x] S006 Check current Node LTS and explicitly defer Node because no JS/TS surface is in scope.
- [x] S007 Pin current SpecGrain source revision used for native state validation.
- [x] S008 Pin immutable Diffcipline v1 source/release identity.
- [x] S009 Define scope-in, scope-out, risk, recovery, context, change surface, acceptance, evidence, minimality, and safety requirements.
- [x] S010 Qualify and merge the shaping change before implementation authority is granted. Canonical shaping merge: `c7fadaebfc44a60f982763bfc20e3a7d41551f4e`.

## Implementation — repository policy

- [x] I001 Reverify implementation base is the canonical Specification 001 shaping merge.
- [x] I002 Add `.gitignore` without hiding required proof artifacts.
- [x] I003 Add Apache-2.0 `LICENSE` for Himsat-owned source.
- [x] I004 Add minimal `CONTRIBUTING.md` and `SECURITY.md` consistent with repository governance.
- [x] I005 Add native `.specgrain` project/policy state in report mode with no fabricated Grain state.
- [ ] I006 Validate native state using pinned SpecGrain revision `faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed` on the exact candidate.
- [ ] I007 Validate bounded `.diffcipline.toml` parsing, expected scope, and R2 proof behavior on the exact candidate.

## Implementation — Rust workspace

- [x] I008 Add `rust-toolchain.toml` pinned to Rust `1.98.1` with fmt/clippy components.
- [x] I009 Add root Rust 2024 Cargo workspace.
- [x] I010 Add dependency-free `crates/himsat-core` library crate.
- [x] I011 Add deterministic smoke/unit tests with no product-domain implementation.
- [ ] I012 Confirm checked-in `Cargo.lock` is accepted by locked Cargo verification and contains no third-party package.

## Implementation — CI

- [x] I013 Add one least-privilege CI workflow.
- [ ] I014 Observe exact-head fmt/clippy/test success on Linux.
- [ ] I015 Observe exact-head fmt/clippy/test success on macOS.
- [ ] I016 Observe exact-head fmt/clippy/test success on Windows.
- [x] I017 Add deterministic pinned SpecGrain validation job.
- [x] I018 Add Diffcipline exact-diff proof path at R2 without weakening repository permissions.
- [x] I019 Pin third-party GitHub Actions and repository control tools to immutable revisions used by this workflow.

## Negative / policy evidence

- [ ] N001 Observe formatting-failure negative control succeed by rejecting malformed formatting.
- [ ] N002 Observe clippy and failing-test negative controls reject their injected defects.
- [ ] N003 Observe malformed SpecGrain state fail validation.
- [ ] N004 Observe unexpected/out-of-scope path fail Diffcipline policy.
- [ ] N005 Observe configured-but-not-run Diffcipline verification remain non-PASS.
- [ ] N006 Inspect workflow permissions and confirm no write permission is granted.

## Qualification

- [ ] Q001 Run exact local verification where the execution environment supports it and preserve unavailable checks honestly.
- [ ] Q002 Push bounded implementation candidate and record exact head SHA.
- [ ] Q003 Require exact-head CI success across every configured required cell.
- [ ] Q004 Require successful SpecGrain state validation on exact head.
- [ ] Q005 Require Diffcipline proof tied to exact base/head and executed verification.
- [ ] Q006 Inspect exact changed paths, dependency manifests, lockfile, license, and generated/untracked state.
- [ ] Q007 Reconcile submitted reviews, inline threads, comments, and any check failures.
- [ ] Q008 Recheck `main`, PR base/head, scope, statuses/checks, mergeability, and rulesets immediately before merge.
- [ ] Q009 Merge only with expected-head protection.
- [ ] Q010 Re-read canonical `main` after merge and record post-merge verification.
- [ ] Q011 Tighten dependency-manifest/lockfile policy for successor work before third-party dependency adoption.
- [ ] Q012 Close Specification 001 only if exact evidence supports the closure; otherwise repair forward.

## Explicit non-tasks

The following are not Specification 001 tasks:

- copying Meetily/Anarlog/Xberg code;
- adding audio/STT/LLM/model dependencies;
- creating Tauri/React/Swift/Kotlin app shells;
- implementing vault/crypto/storage;
- implementing product schemas, recording, documents, memory, agents, sync, or publishing;
- publishing a release;
- claiming platform support or benchmark superiority.
