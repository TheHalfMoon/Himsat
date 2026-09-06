# Specification 001 Evidence — Repository and Delivery-Control Foundation

## Disposition

```text
SPECIFICATION = 001-repository-foundation
IMPLEMENTATION_PR = 3
IMPLEMENTATION_HEAD = 34a0b1a4b7a5f4873a6de7141f160de45fd37740
IMPLEMENTATION_MERGE = 4d1f8843577059d5cfa0cfeeb97df05feb2da8be
CLOSEOUT = PENDING_CLOSEOUT_MERGE
```

This file records executed evidence. It does not convert unavailable or skipped checks into PASS and does not claim any Himsat product feature, donor adoption, release, or native SpecGrain `GRAIN` state.

## Canonical lineage

- Specification 000 merge: `3f6687b34530e55098f7854042b12adfa607f394`.
- Specification 001 shaping merge: `c7fadaebfc44a60f982763bfc20e3a7d41551f4e`.
- Specification 001 implementation PR: `#3`.
- Exact qualified implementation head: `34a0b1a4b7a5f4873a6de7141f160de45fd37740`.
- Expected-head merge result: `4d1f8843577059d5cfa0cfeeb97df05feb2da8be`.
- The merge commit is GitHub-signed and has shaping merge `c7fadae...` and qualified implementation head `34a0b1a...` as its parents.

## Exact implementation scope

The final compare from shaping merge to qualified head was six commits ahead, zero behind, with exactly 16 changed paths:

```text
.diffcipline.toml
.github/workflows/ci.yml
.gitignore
.specgrain/policies/default.json
.specgrain/project.json
.specgrain/specs/.gitkeep
CONTRIBUTING.md
Cargo.lock
Cargo.toml
LICENSE
SECURITY.md
crates/himsat-core/Cargo.toml
crates/himsat-core/src/lib.rs
rust-toolchain.toml
specs/001-repository-foundation/tasks.md
specs/CURRENT.md
```

No donor source, model, product feature, Tauri/Node/mobile shell, telemetry, binary, release artifact, or third-party Rust dependency entered the implementation diff.

## Exact-head PR verification

GitHub Actions run `34042429773` executed against exact PR head `34a0b1a4b7a5f4873a6de7141f160de45fd37740` and completed successfully.

Successful jobs:

- `Rust / ubuntu-latest`: format, clippy with `-D warnings`, locked tests, dependency-free core invariant.
- `Rust / macos-latest`: same checks.
- `Rust / windows-latest`: same checks.
- `SpecGrain / pinned source`: tracked state validated using `TheHalfMoon/SpecGrain@faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed`.
- `Diffcipline / R2 exact diff`: proof executed with verification enabled using `TheHalfMoon/Diffcipline@5cb1c77340b75649f6168e0e8f66479ea047ea96`.
- `Negative controls`: deliberately broken formatting, clippy, test, SpecGrain state, configured-but-not-run Diffcipline verification, and an out-of-scope path were all detected/rejected as required.

The CI workflow grants only `contents: read` repository permission.

## Review and reconciliation

Immediately before merge:

- PR #3 was open, non-draft, and mergeable.
- `main` still matched shaping merge `c7fadaebfc44a60f982763bfc20e3a7d41551f4e`.
- PR head matched exact qualified head `34a0b1a4b7a5f4873a6de7141f160de45fd37740`.
- no submitted pull-request reviews existed;
- no inline review threads existed;
- Qodo reported that review was unavailable because its trial had ended;
- CodeRabbit reported that automatic substantive review was skipped because the repository had fewer than 10 stars. A success integration status therefore was not treated as a substantive review.
- repository branch protection/required status checks were not configured; this absence was not represented as a successful required-check policy.

Merge used GitHub expected-head protection with the exact qualified head.

## Post-merge verification

Canonical `main` was re-read at signed merge `4d1f8843577059d5cfa0cfeeb97df05feb2da8be`.

Push-triggered GitHub Actions run `34042553778` executed on that exact merge and completed successfully with the same six job groups:

- Rust / Ubuntu;
- Rust / macOS;
- Rust / Windows;
- pinned SpecGrain validation;
- Diffcipline R2 proof;
- negative controls.

This provides post-merge executable confirmation that the merged repository state retains the qualified foundation behavior.

## Local execution availability

The chat execution environment did not provide a usable local Rust installation/network path for repository-local Rust verification. Local execution therefore remained unavailable/NOT RUN rather than PASS. Exact GitHub-hosted verification described above supplied the executable evidence used for merge qualification.

## Successor policy hardening

The implementation bootstrap temporarily allowed Cargo manifest and lockfile changes because those files had to be created by Specification 001 itself.

The closeout change tightens both:

```text
dependency_manifest_changes = "review"
lockfile_changes = "review"
```

No third-party dependency adoption is authorized by this change. Future manifest/lockfile changes require explicit successor scope and cannot silently inherit the bootstrap allowance.

## Remaining closeout condition

Specification 001 becomes `CLOSED_CANONICAL` only after the closeout PR containing this evidence and the policy hardening merges successfully and canonical `main` is re-read. Until then Specification 002 remains blocked.
