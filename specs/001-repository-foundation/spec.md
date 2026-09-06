# Specification 001 — Repository and Delivery-Control Foundation

## Status

```text
SPECIFICATION = 001-repository-foundation
STATE = SHAPING_CANDIDATE
NATIVE_SPEC_GRAIN_STATE = NOT_YET_ESTABLISHED
IMPLEMENTATION_AUTHORITY = NONE_UNTIL_SHAPING_CLOSEOUT
DONOR_ADOPTION_AUTHORITY = NONE
PRODUCT_FEATURE_AUTHORITY = NONE
```

This specification is shaped from canonical `main` after Specification 000 merged as `3f6687b34530e55098f7854042b12adfa607f394`.

## Outcome

Create the smallest real Himsat engineering foundation that can be built and independently proved across supported development hosts: a pinned Rust workspace, minimal Himsat-owned core crate, repository-local SpecGrain and Diffcipline control state/policy, exact CI verification, and essential open-source contribution/security/license surfaces.

The unit exists to make later work reproducible and governable. It does **not** implement Himsat product behavior.

## Why this unit is selected now

Specification 000 established that donor adoption and product implementation must not begin before repository/toolchain and provenance controls exist. Live `main` currently contains planning/governance Markdown only: no Rust workspace, no dependency manifest, no CI workflow, no native SpecGrain store, and no Diffcipline policy.

The repository therefore cannot yet prove even a trivial implementation change using its chosen delivery methods. This is the smallest predecessor needed by every later implementation unit.

## Scope in

1. Pin the Rust toolchain to the current qualified stable point release selected during shaping.
2. Create a minimal Cargo workspace using Rust 2024 edition.
3. Create one dependency-free Himsat-owned library crate, `crates/himsat-core`, containing only foundation metadata/smoke-test behavior and no product domain implementation.
4. Commit repository-local SpecGrain state in report mode and validate it with a pinned exact SpecGrain source revision.
5. Add a bounded `.diffcipline.toml` policy with risk-scaled verification commands.
6. Add GitHub CI that executes formatting, lint, and tests on Linux, macOS, and Windows against the exact PR revision.
7. Add a deterministic SpecGrain validation job and Diffcipline proof path appropriate to the exact change.
8. Establish the Himsat-owned source license as Apache-2.0 while preserving the rule that future donor material retains its controlling license/notices.
9. Add minimal `CONTRIBUTING.md`, `SECURITY.md`, `.gitignore`, and any small repository metadata required to use the toolchain safely.
10. Update canonical program state only after exact implementation evidence closes.

## Explicit scope out

- Meetily, Anarlog, Xberg, or any other donor code;
- donor dependencies or vendored third-party source;
- audio capture, audio processing, transcription, diarization, models, model downloads, or media parsing;
- encrypted vault/storage/crypto implementation;
- Tauri, React, TypeScript, Node.js, pnpm, Swift, Kotlin, iOS, Android, watchOS, or desktop application shells;
- MCP, agents, plugins, connectors, sync, document intelligence, publishing, memory, or search;
- telemetry/analytics;
- release publication, installers, signing/notarization, package registries, or auto-update;
- benchmark/superiority/platform-support claims;
- repository/organization ruleset administration that cannot be changed through the available repository-authorized interface.

## Dependency

- Specification 000 canonical foundation merge: `3f6687b34530e55098f7854042b12adfa607f394`.

No product or donor dependency is permitted.

## Toolchain decisions selected during shaping

### Rust

Pin `1.98.1` for the first workspace implementation.

Reason: the Rust project published 1.98.1 on 2026-09-03 as the stable point release fixing a 1.98.0 vtable-generation miscompilation. A project foundation should not intentionally pin the affected 1.98.0 compiler.

Official source: `https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/`.

### JavaScript/Node

Do not introduce Node.js in Specification 001.

Node.js 24.20.0 is the current LTS observed during shaping, but no JavaScript/TypeScript product surface is authorized in this unit. Introducing Node merely because a future Tauri/React shell may use it would violate dependency restraint. The relevant app-shell specification must pin Node/package-manager versions when it actually needs them.

Official source: `https://nodejs.org/en/download`.

### SpecGrain

Initial Himsat integration must bind validation to exact SpecGrain source revision:

`TheHalfMoon/SpecGrain@faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed`

This current-main revision exposes the native `DRAFT -> SHAPED -> REFINING -> GRAIN` preparation/check surface that the historical v0.3.0 release does not fully include. No later source behavior is implied without a new pin.

### Diffcipline

Use immutable Diffcipline v1 source/release identity:

`TheHalfMoon/Diffcipline@5cb1c77340b75649f6168e0e8f66479ea047ea96`

Repository verification must still report `NOT RUN` until commands actually execute.

## Risk

**R2 — repository/toolchain/governance foundation.**

The change does not touch user data or product behavior, but it defines compiler, CI, license, and delivery-control contracts inherited by all later work. A bad foundation can create broad supply-chain or verification drift.

## Recovery

Before merge, recovery is branch deletion/revert with no product/user-data migration.

After merge, a defect is repaired forward through a bounded PR or the exact foundation merge is reverted if the repository cannot build/validate. No compatibility guarantee is created for pre-1.0 internal scaffolding by this unit.

## Context budget

```text
context_budget_tokens = 10000
context_estimate_tokens = 7000
```

Required context is limited to:

- `AGENTS.md`;
- `specs/CURRENT.md`;
- constitution;
- execution-master-plan unit 001;
- this specification/plan/tasks;
- SpecGrain current CLI/store contracts at the pinned SHA;
- Diffcipline v1 proof/policy contract;
- official Rust release evidence;
- exact live Himsat GitHub state.

No donor repository implementation context is required.

## Expected implementation change surface

The implementation candidate should remain within this bounded surface unless a separately justified exception is recorded:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
crates/himsat-core/Cargo.toml
crates/himsat-core/src/lib.rs
.gitignore
.github/workflows/ci.yml
.specgrain/**
.diffcipline.toml
LICENSE
CONTRIBUTING.md
SECURITY.md
specs/001-repository-foundation/**
specs/CURRENT.md
```

A smaller exact surface is preferred when it still satisfies acceptance.

## Acceptance conditions

1. Workspace metadata is valid and the toolchain pin is explicit.
2. `himsat-core` has zero runtime dependencies and no product feature implementation.
3. `cargo fmt --all -- --check` succeeds on the exact candidate.
4. `cargo clippy --workspace --all-targets --locked -- -D warnings` succeeds on the exact candidate.
5. `cargo test --workspace --all-targets --locked` succeeds on the exact candidate.
6. GitHub CI runs the applicable Rust verification on Linux, macOS, and Windows for the exact PR head and all required cells succeed.
7. Tracked SpecGrain state is accepted by the pinned SpecGrain revision; no native `GRAIN` claim is made unless the tool actually establishes it.
8. Diffcipline is configured with bounded scope/risk/verification and an exact-change proof is executed rather than inferred.
9. Apache-2.0 applies to Himsat-owned code introduced by this unit; documentation explicitly preserves future donor licensing/provenance requirements.
10. No donor source/dependency, product feature, telemetry, model, app-shell dependency, or release artifact appears in the diff.
11. No untracked/generated repository artifact required for reproducible verification is silently omitted.
12. PR base/head/scope/mergeability/reviews/threads/checks are reverified immediately before merge.
13. Any unavailable/skipped/absent check remains recorded as unavailable/skipped/absent rather than PASS.
14. Post-merge `main` is re-read and exact canonical verification is recorded before Specification 001 is closed.

## Evidence requirements

- exact base/head comparison and changed-path list;
- exact Rust toolchain/version evidence;
- Cargo metadata/build/lint/test outputs;
- exact CI run IDs/results for all configured cells;
- SpecGrain check output tied to pinned source revision;
- Diffcipline machine/human proof for exact diff;
- license and dependency-manifest inspection;
- PR review/thread/status/mergeability observations;
- signed/expected-head merge evidence where available;
- post-merge `main` SHA and verification results.

## Minimality choice

**Native minimal foundation.**

Do not scaffold Tauri/mobile/web layers or add general utility crates yet. The core crate should use the Rust standard library only. Every additional tool/dependency must satisfy an acceptance requirement that cannot be met more simply.

## Safety status

`requirements-defined`

Safety requirements:

- do not introduce networked runtime behavior;
- do not introduce secrets or telemetry;
- pin external delivery tools to immutable revisions where practical;
- workflow permissions must be least-privilege;
- verification commands executed in CI must be repository-controlled and reviewed;
- no donor material may enter before Specification 002 provenance machinery authorizes it.
