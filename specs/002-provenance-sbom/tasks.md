# Specification 002 Tasks — Provenance, License, and SBOM Machinery

> Checkboxes track authoring/reconciliation work; exact CI/proof remains authoritative.

## Shaping

- [x] S001 Re-read canonical `main` at Spec001 closeout merge `90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0`.
- [x] S002 Confirm post-closeout run `34042822934` succeeded across all configured jobs.
- [x] S003 Re-read execution-plan unit 002 and donor/provenance research policy.
- [x] S004 Separate research donor candidates from machine adoption authority.
- [x] S005 Select dependency-free Python 3.13 stdlib tooling for this governance unit and preserve Rust-first product runtime.
- [x] S006 Define machine policy/registry, digest/path, Cargo closure, notices, SBOM, fixture, risk, recovery, evidence, and minimality contracts.
- [x] S007 Qualify and merge shaping. Canonical shaping merge: `91b981ee89ab511444e5a17ca80f5312a31c2122`; post-shaping CI: `34043416988` SUCCESS.

## Implementation — policy and registry

- [x] I001 Add versioned provenance policy with allow/manual/deny dispositions.
- [x] I002 Add initially empty real adoption registry.
- [x] I003 Document registry schema and research-vs-authority distinction.
- [x] I004 Implement fail-closed schema/identity/license/adoption-mode validation.
- [x] I005 Implement immutable revision and safe repository-path validation.
- [x] I006 Implement SHA-256 integrity validation for applicable local adopted artifacts.
- [x] I007 Enforce independent model/dataset/font/asset license/integrity identity.

## Implementation — dependency and generated closure

- [x] I008 Reconcile every external Cargo lockfile package to an approved dependency entry.
- [x] I009 Keep current dependency-free Himsat workspace valid with empty adoption registry.
- [x] I010 Generate deterministic `THIRD_PARTY_NOTICES.md`.
- [x] I011 Generate deterministic `governance/generated/sbom.json` using `himsat.sbom/v1`.
- [x] I012 Add generated-file drift/idempotence checks.

## Negative and positive evidence

- [x] N001 Allowed permissive fixture is represented and expected to pass.
- [x] N002 Manual-review license fixture cannot become adopted PASS.
- [x] N003 Denied/restricted and unknown-license fixtures fail non-reference adoption.
- [x] N004 Malformed/non-immutable revision and missing source path fixtures fail.
- [x] N005 Unsafe traversal path fixture fails.
- [x] N006 Missing/invalid/mismatched digest fixtures fail where required.
- [x] N007 Model/data/asset independent-license requirement is covered.
- [x] N008 Unregistered external Cargo dependency fixture fails.
- [x] N009 Registered dependency checksum mismatch fixture fails where observed.
- [x] N010 Notice/SBOM deterministic regeneration and drift failure are exercised by `self-test`.
- [x] N011 Unsupported registry schema and duplicate IDs fail closed.
- [x] N012 Restricted reference-only entry remains non-adopted and excluded from distribution output.

## CI and qualification

- [x] Q001 Extend existing least-privilege CI for provenance validation/self-test/generated closure.
- [x] Q002 Preserve Rust Ubuntu/macOS/Windows, SpecGrain, Diffcipline, and existing negative controls.
- [x] Q003 Keep Cargo manifests/lockfile unchanged and add no third-party package.
- [ ] Q004 Push exact bounded implementation candidate and record head.
- [ ] Q005 Require exact-head provenance, Rust, SpecGrain, Diffcipline, and negative-control success.
- [ ] Q006 Inspect exact paths, registry, generated outputs, dependency closure, reviews/threads/checks, and mergeability.
- [ ] Q007 Merge only with expected-head protection.
- [ ] Q008 Re-read canonical `main` and require post-merge CI success.
- [ ] Q009 Close Specification 002 only with exact evidence; otherwise repair forward.

## Explicit non-tasks

- copying or depending on Meetily/Anarlog/Xberg/Whisper/etc.;
- downloading models/assets/datasets;
- product schemas/audio/storage/UI/mobile/documents/memory/agents/sync;
- SPDX/CycloneDX compliance claims;
- release publication.
