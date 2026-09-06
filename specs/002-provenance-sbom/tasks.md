# Specification 002 Tasks — Provenance, License, and SBOM Machinery

> Checkboxes track work; exact CI/proof remains authoritative.

## Shaping

- [x] S001 Re-read canonical `main` at Spec001 closeout merge `90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0`.
- [x] S002 Confirm post-closeout run `34042822934` succeeded across all configured jobs.
- [x] S003 Re-read execution-plan unit 002 and donor/provenance research policy.
- [x] S004 Separate research donor candidates from machine adoption authority.
- [x] S005 Select dependency-free Python 3.13 stdlib tooling for this governance unit and preserve Rust-first product runtime.
- [x] S006 Define machine policy/registry, digest/path, Cargo closure, notices, SBOM, fixture, risk, recovery, evidence, and minimality contracts.
- [ ] S007 Qualify and merge this shaping change before implementation authority is granted.

## Implementation — policy and registry

Not authorized until S007 closes.

- [ ] I001 Add versioned provenance policy with allow/manual/deny dispositions.
- [ ] I002 Add initially empty real adoption registry.
- [ ] I003 Document registry schema and research-vs-authority distinction.
- [ ] I004 Implement fail-closed schema/identity/license/adoption-mode validation.
- [ ] I005 Implement immutable revision and safe repository-path validation.
- [ ] I006 Implement SHA-256 integrity validation for applicable local adopted artifacts.
- [ ] I007 Enforce independent model/dataset/font/asset license/integrity identity.

## Implementation — dependency and generated closure

- [ ] I008 Reconcile every external Cargo lockfile package to an approved dependency entry.
- [ ] I009 Keep current dependency-free Himsat workspace valid with empty adoption registry.
- [ ] I010 Generate deterministic `THIRD_PARTY_NOTICES.md`.
- [ ] I011 Generate deterministic `governance/generated/sbom.json` using `himsat.sbom/v1`.
- [ ] I012 Add generated-file drift/idempotence checks.

## Negative and positive evidence

- [ ] N001 Allowed permissive fixture passes.
- [ ] N002 Manual-review license cannot become adopted PASS.
- [ ] N003 Denied/restricted and unknown licenses fail non-reference adoption.
- [ ] N004 Malformed/non-immutable revision and missing source path fail.
- [ ] N005 Unsafe traversal/absolute destination path fails.
- [ ] N006 Missing/invalid/mismatched digest fails where required.
- [ ] N007 Model/data/asset without independent license/digest fails.
- [ ] N008 Unregistered external Cargo dependency fails.
- [ ] N009 Registered dependency identity/checksum mismatch fails where observed.
- [ ] N010 Notice/SBOM drift fails and deterministic regeneration succeeds.

## CI and qualification

- [ ] Q001 Extend existing least-privilege CI for provenance validation/self-test/generated closure.
- [ ] Q002 Preserve Rust Ubuntu/macOS/Windows, SpecGrain, Diffcipline, and existing negative controls.
- [ ] Q003 Confirm no Cargo manifest/lockfile or third-party package addition appears in implementation diff.
- [ ] Q004 Push exact bounded implementation candidate and record head.
- [ ] Q005 Require exact-head provenance, Rust, SpecGrain, and Diffcipline success.
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
