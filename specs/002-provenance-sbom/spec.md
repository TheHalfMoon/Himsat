# Specification 002 — Provenance, License, and SBOM Machinery

## Status

```text
SPECIFICATION = 002-provenance-sbom
STATE = SHAPING_CANDIDATE
IMPLEMENTATION_AUTHORITY = NONE_UNTIL_SHAPING_CLOSEOUT
DONOR_ADOPTION_AUTHORITY = NONE
PRODUCT_FEATURE_AUTHORITY = NONE
```

Specification 001 is canonical at closeout merge `90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0` with post-closeout CI run `34042822934` successful.

## Outcome

Create the smallest deterministic control plane that can prove whether third-party code, dependencies, models, datasets, fonts, and assets are eligible for future Himsat adoption. The machinery must bind adoption to exact source identity, license disposition, notices, integrity evidence, and dependency closure before donor bytes enter the project.

## Scope in

1. Add an offline, dependency-free provenance validator/generator using Python 3.13 standard library only.
2. Add a versioned machine policy defining `allow`, `manual`, and `deny` license dispositions.
3. Add an initially empty machine adoption registry distinct from the research donor list.
4. Define exact entry schemas for code/dependency/model/dataset/font/asset records and COPY/DEPEND/VENDOR/REFERENCE modes.
5. Require immutable revisions and exact source paths for non-reference source adoption.
6. Require SHA-256 integrity fields for copied/vendored or separately distributed artifacts where bytes are adopted.
7. Enforce that models/assets/data carry their own license/integrity identity rather than inheriting engine licenses.
8. Reconcile every external Cargo lockfile package to an approved dependency record.
9. Deterministically generate `THIRD_PARTY_NOTICES.md` and `governance/generated/sbom.json` using Himsat schema `himsat.sbom/v1`.
10. Add positive/negative fixtures for policy, identity, path, digest, dependency, notice, and SBOM behavior.
11. Integrate provenance validation/generation checks into least-privilege CI.
12. Preserve SpecGrain/Diffcipline exact-proof behavior and current Rust CI.

## Explicit scope out

- copying or adapting Meetily, Anarlog, Xberg, Whisper, or any other donor source;
- adding third-party Cargo/Python/Node dependencies;
- downloading models, datasets, fonts, binaries, or donor archives;
- network fetching from the provenance tool or CI gate;
- implementing product-domain code;
- Tauri/mobile/app shells;
- SPDX/CycloneDX conformance claims;
- legal conclusions beyond repository policy classification;
- authorizing a donor merely because it appears in `docs/donor-and-provenance.md`.

## Dependency

Specification 001 `CLOSED_CANONICAL` at `90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0`.

## Design contract

### Research registry vs adoption registry

Research candidates are non-authoritative. The new machine registry begins empty. Future adoption authority is created only by exact entries in bounded successor/adoption PRs that pass the machine gate.

### Tooling choice

Use Python 3.13 standard library only. This is governance tooling, not Himsat runtime. It avoids adding a Rust dependency before dependency governance exists.

### Real-registry validation behavior

A real adoption registry passes only when every non-reference entry is complete, policy-eligible, internally consistent, and reconciled with observed local dependency/artifact state. `manual`, `deny`, unknown, malformed, or incomplete adoption records cannot be represented as PASS.

Reference-only entries may document restricted sources without importing bytes, provided the registry explicitly marks them as non-adopted and excludes them from notices/SBOM distribution components.

### Integrity and path behavior

The gate must reject unsafe local paths, malformed immutable revisions, missing digests where required, duplicate IDs, digest mismatches, and unregistered external dependencies.

### Generated outputs

Generation is deterministic and idempotent. CI regenerates notices/SBOM and fails on drift.

`himsat.sbom/v1` is a project schema only; it must not be described as standards-compliant SPDX/CycloneDX output.

## Risk

**R2 — governance and supply-chain.**

The implementation does not process user data or product runtime behavior, but mistakes can permanently contaminate a permissive open-source codebase or create redistribution/compliance risk.

## Recovery

Before merge, delete/revert the branch. After merge, repair policy/schema forward or revert the exact machinery change. No donor bytes are permitted in this specification, so recovery does not require removing adopted third-party code.

## Context budget

```text
context_budget_tokens = 12000
context_estimate_tokens = 8000
```

Required context: canonical governance, current state, execution-plan unit 002, donor/provenance research plan, Spec001 evidence/control files, Cargo workspace/lockfile, exact GitHub state, and this spec/plan/tasks.

## Expected implementation surface

```text
tools/provenance.py
governance/provenance/policy.json
governance/provenance/registry.json
governance/provenance/README.md
governance/provenance/fixtures/**
governance/generated/sbom.json
THIRD_PARTY_NOTICES.md
.github/workflows/ci.yml
.diffcipline.toml
specs/002-provenance-sbom/**
specs/CURRENT.md
```

Cargo manifests/lockfile should remain unchanged.

## Acceptance conditions

1. Provenance tooling imports only Python standard-library modules.
2. Machine adoption registry starts with zero adopted third-party entries.
3. Real registry validation fails closed for unknown/manual/denied/incomplete non-reference adoption.
4. Exact revision/path/integrity rules are tested.
5. Model/data/asset independent licensing is tested.
6. External Cargo package closure is deterministic and tested; current dependency-free workspace passes with empty adoption registry.
7. Notices and `himsat.sbom/v1` output are deterministic/idempotent and checked for drift.
8. Positive and negative fixtures cover every required policy family.
9. Provenance jobs execute in CI without write permission and without network fetches performed by the tool itself.
10. Existing Rust matrix, SpecGrain, Diffcipline, and negative controls remain green.
11. No donor bytes, third-party package additions, product behavior, model, binary, or release artifact enter the diff.
12. Exact PR head/base/checks/reviews/threads/mergeability are reconciled before expected-head merge.
13. Post-merge canonical CI succeeds before closeout.

## Evidence requirements

- exact base/head/path comparison;
- tool import/dependency inspection;
- fixture matrix with expected/observed dispositions;
- deterministic regeneration/diff proof;
- Cargo-lock closure proof;
- CI run/job IDs on exact head;
- SpecGrain and Diffcipline exact proof;
- review/thread/check reconciliation;
- expected-head merge and post-merge CI.

## Minimality

Do not add legal-scanning SaaS, package-manager libraries, network fetchers, Rust JSON/TOML crates, SPDX/CycloneDX libraries, or donor source. The first gate should be small enough to audit directly.

## Safety status

`requirements-defined`

Safety rules: offline deterministic parsing; no registry-derived shell execution; no path traversal; no secret/network credential requirement; fail closed for unknown identity/license; no donor authority during shaping.
