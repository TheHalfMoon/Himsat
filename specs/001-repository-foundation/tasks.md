# Specification 001 Tasks — Repository and Delivery-Control Foundation

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
- [ ] S010 Qualify and merge this shaping change before implementation authority is granted.

## Implementation — repository policy

Not authorized until S010 closes.

- [ ] I001 Reverify implementation base is the canonical Specification 001 shaping merge.
- [ ] I002 Add `.gitignore` without hiding required proof artifacts.
- [ ] I003 Add Apache-2.0 `LICENSE` for Himsat-owned source.
- [ ] I004 Add minimal `CONTRIBUTING.md` and `SECURITY.md` consistent with repository governance.
- [ ] I005 Add native `.specgrain` project/policy state in report mode with no fabricated Grain state.
- [ ] I006 Validate native state using pinned SpecGrain revision `faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed`.
- [ ] I007 Add bounded `.diffcipline.toml` and verify policy parsing/expected scope.

## Implementation — Rust workspace

- [ ] I008 Add `rust-toolchain.toml` pinned to Rust `1.98.1` with required fmt/clippy components.
- [ ] I009 Add root Rust 2024 Cargo workspace.
- [ ] I010 Add dependency-free `crates/himsat-core` library crate.
- [ ] I011 Add deterministic smoke/unit tests with no product-domain implementation.
- [ ] I012 Generate/inspect `Cargo.lock` and retain only deterministic required state.

## Implementation — CI

- [ ] I013 Add one least-privilege CI workflow.
- [ ] I014 Run fmt/clippy/test on Linux.
- [ ] I015 Run fmt/clippy/test on macOS.
- [ ] I016 Run fmt/clippy/test on Windows.
- [ ] I017 Add deterministic pinned SpecGrain validation job.
- [ ] I018 Add Diffcipline exact-diff proof path at R2 without weakening repository policy.
- [ ] I019 Pin third-party GitHub Actions to immutable revisions where practical and document any unavoidable floating surface.

## Negative / policy evidence

- [ ] N001 Demonstrate formatting failure is detected.
- [ ] N002 Demonstrate clippy/test failure is detected by the configured verification path.
- [ ] N003 Demonstrate malformed SpecGrain state fails validation.
- [ ] N004 Demonstrate unexpected/out-of-scope path causes Diffcipline review/failure according to policy.
- [ ] N005 Demonstrate configured-but-not-run verification is not represented as PASS.
- [ ] N006 Inspect workflow permissions and prove no write permission is granted without necessity.

## Qualification

- [ ] Q001 Run exact local verification where the execution environment supports it and preserve unavailable checks honestly.
- [ ] Q002 Push bounded implementation candidate and record exact head SHA.
- [ ] Q003 Require exact-head CI success across every configured required cell.
- [ ] Q004 Require successful SpecGrain state validation on exact head.
- [ ] Q005 Require Diffcipline proof tied to exact base/head and executed verification.
- [ ] Q006 Inspect exact changed paths, dependency manifests, lockfile, license, and untracked/generated state.
- [ ] Q007 Reconcile submitted reviews, inline threads, comments, and any check failures.
- [ ] Q008 Recheck `main`, PR base/head, scope, statuses/checks, mergeability, and rulesets immediately before merge.
- [ ] Q009 Merge only with expected-head protection.
- [ ] Q010 Re-read canonical `main` after merge and record post-merge verification.
- [ ] Q011 Close Specification 001 only if exact evidence supports the closure; otherwise repair forward.

## Explicit non-tasks

The following are not Specification 001 tasks:

- copying Meetily/Anarlog/Xberg code;
- adding audio/STT/LLM/model dependencies;
- creating Tauri/React/Swift/Kotlin app shells;
- implementing vault/crypto/storage;
- implementing product schemas, recording, documents, memory, agents, sync, or publishing;
- publishing a release;
- claiming platform support or benchmark superiority.
