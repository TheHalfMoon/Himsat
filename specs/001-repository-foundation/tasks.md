# Specification 001 Tasks — Repository and Delivery-Control Foundation

> Checkboxes record authoring/reconciliation progress. They are not substitutes for exact CI, SpecGrain, or Diffcipline evidence. Durable proof is recorded in `evidence.md`.

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
- [x] I006 Validate native state using pinned SpecGrain revision `faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed` on exact implementation head `34a0b1a4b7a5f4873a6de7141f160de45fd37740`.
- [x] I007 Validate bounded `.diffcipline.toml` parsing, expected scope, and R2 proof behavior on the exact candidate.

## Implementation — Rust workspace

- [x] I008 Add `rust-toolchain.toml` pinned to Rust `1.98.1` with fmt/clippy components.
- [x] I009 Add root Rust 2024 Cargo workspace.
- [x] I010 Add dependency-free `crates/himsat-core` library crate.
- [x] I011 Add deterministic smoke/unit tests with no product-domain implementation.
- [x] I012 Confirm checked-in `Cargo.lock` is accepted by locked Cargo verification and contains no third-party package.

## Implementation — CI

- [x] I013 Add one least-privilege CI workflow.
- [x] I014 Observe exact-head fmt/clippy/test success on Linux in run `34042429773`.
- [x] I015 Observe exact-head fmt/clippy/test success on macOS in run `34042429773`.
- [x] I016 Observe exact-head fmt/clippy/test success on Windows in run `34042429773`.
- [x] I017 Add and execute deterministic pinned SpecGrain validation job.
- [x] I018 Add and execute Diffcipline exact-diff proof path at R2 without weakening repository permissions.
- [x] I019 Pin third-party GitHub Actions and repository control tools to immutable revisions used by this workflow.

## Negative / policy evidence

- [x] N001 Observe formatting-failure negative control succeed by rejecting malformed formatting.
- [x] N002 Observe clippy and failing-test negative controls reject their injected defects.
- [x] N003 Observe malformed SpecGrain state fail validation.
- [x] N004 Observe unexpected/out-of-scope path fail Diffcipline policy.
- [x] N005 Observe configured-but-not-run Diffcipline verification remain non-PASS.
- [x] N006 Inspect workflow permissions and confirm no write permission is granted.

## Qualification

- [x] Q001 Preserve unavailable local Rust execution honestly as NOT RUN while using executed GitHub-hosted evidence for qualification.
- [x] Q002 Push bounded implementation candidate and record exact head `34a0b1a4b7a5f4873a6de7141f160de45fd37740`.
- [x] Q003 Require exact-head CI success across every configured cell; run `34042429773` succeeded.
- [x] Q004 Require successful SpecGrain state validation on exact head.
- [x] Q005 Require Diffcipline proof tied to exact base/head and executed verification.
- [x] Q006 Inspect exact changed paths, dependency manifests, lockfile, license, and generated/untracked state.
- [x] Q007 Reconcile submitted reviews, inline threads, comments, and check availability without treating unavailable reviews as PASS.
- [x] Q008 Recheck `main`, PR base/head, scope, checks, mergeability, and branch-protection status immediately before merge.
- [x] Q009 Merge with expected-head protection. Merge: `4d1f8843577059d5cfa0cfeeb97df05feb2da8be`.
- [x] Q010 Re-read canonical `main` after merge and observe post-merge run `34042553778` succeed across all configured jobs.
- [x] Q011 Tighten dependency-manifest/lockfile policy for successor work from bootstrap `allow` to `review` in the closeout candidate.
- [ ] Q012 Merge the closeout candidate, re-read canonical `main`, and only then mark Specification 001 `CLOSED_CANONICAL`.

## Explicit non-tasks

The following are not Specification 001 tasks:

- copying Meetily/Anarlog/Xberg code;
- adding audio/STT/LLM/model dependencies;
- creating Tauri/React/Swift/Kotlin app shells;
- implementing vault/crypto/storage;
- implementing product schemas, recording, documents, memory, agents, sync, or publishing;
- publishing a release;
- claiming platform support or benchmark superiority.
