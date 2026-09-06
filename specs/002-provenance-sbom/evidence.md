# Specification 002 Evidence — Provenance, License, and SBOM Machinery

## Disposition

```text
SPECIFICATION = 002-provenance-sbom
SHAPING_MERGE = 91b981ee89ab511444e5a17ca80f5312a31c2122
IMPLEMENTATION_PR = 6
IMPLEMENTATION_HEAD = 7c20f2dc1a2a3c8c0cc7674af4241f10f50c6cfb
IMPLEMENTATION_MERGE = dead41f7254386c643973af081cccc2f238d3a67
CLOSEOUT = PENDING_CLOSEOUT_MERGE
```

This file records executed evidence. It does not turn unavailable review services into PASS and does not authorize donor adoption, product implementation, release publication, or SPDX/CycloneDX conformance claims.

## Canonical lineage

- Specification 001 closeout merge: `90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0`.
- Specification 002 shaping merge: `91b981ee89ab511444e5a17ca80f5312a31c2122`.
- Specification 002 shaping exact-head CI: `34043335797` SUCCESS.
- Specification 002 post-shaping CI: `34043416988` SUCCESS.
- Specification 002 implementation PR: `#6`.
- Exact qualified implementation head: `7c20f2dc1a2a3c8c0cc7674af4241f10f50c6cfb`.
- Expected-head implementation merge: `dead41f7254386c643973af081cccc2f238d3a67`.

## Exact implementation scope

The final compare from shaping merge to qualified implementation head was 11 commits ahead, zero behind, with exactly 11 changed paths and 649 added lines:

```text
.diffcipline.toml
.github/workflows/ci.yml
THIRD_PARTY_NOTICES.md
governance/generated/sbom.json
governance/provenance/README.md
governance/provenance/fixtures/cases.json
governance/provenance/policy.json
governance/provenance/registry.json
specs/002-provenance-sbom/tasks.md
specs/CURRENT.md
tools/provenance.py
```

`Cargo.toml` and `Cargo.lock` were unchanged. No third-party Cargo/Python/Node dependency, donor source, model, dataset, font, asset, product feature, binary, or release artifact entered the implementation diff.

The canonical machine adoption registry remained:

```json
{"entries":[],"schema":"himsat.provenance-registry/v1"}
```

Therefore Specification 002 implemented the admission machinery while authorizing and adopting zero third-party components.

## Exact-head implementation verification

GitHub Actions run `34044644315` executed against exact PR head `7c20f2dc1a2a3c8c0cc7674af4241f10f50c6cfb` and completed successfully across all 10 configured jobs:

- `Provenance / ubuntu-latest`: real registry validation and generated closure PASS;
- `Provenance / macos-latest`: same checks PASS;
- `Provenance / windows-latest`: same checks PASS;
- `Provenance / adversarial self-test`: fixture matrix and drift/idempotence checks PASS;
- `Rust / ubuntu-latest`: format, clippy with warnings denied, locked tests, dependency-free core invariant PASS;
- `Rust / macos-latest`: same checks PASS;
- `Rust / windows-latest`: same checks PASS;
- `SpecGrain / pinned source`: tracked native state validation PASS;
- `Diffcipline / R2 exact diff`: exact-diff proof with executed provenance + Rust verification PASS;
- `Negative controls`: existing formatting, clippy, test, malformed SpecGrain, configured-but-not-run verification, and out-of-scope path controls PASS by rejecting the injected defects.

The provenance fixture matrix exercised permissive adoption, manual/denied/unknown license rejection, immutable revision enforcement, missing/unsafe paths, digest requirements and mismatches, independent model artifact licensing, unregistered Cargo dependency rejection, Cargo checksum mismatch, exact registered dependency success, restricted reference-only handling, unsupported schema rejection, duplicate-ID rejection, and generated-output drift rejection.

## Tool minimality and safety evidence

`tools/provenance.py` uses Python 3.13 standard-library modules only. Its source audit rejects non-stdlib imports and explicitly rejects process/network-oriented imports and dynamic execution primitives used by the admission tool.

The gate performs local deterministic parsing, hashing, registry validation, Cargo lock closure, notice/SBOM generation, and fixture testing. It does not fetch donor repositories, download artifacts, execute registry-derived shell commands, or require repository write permission in CI.

`himsat.sbom/v1` is explicitly a Himsat project schema and is not represented as SPDX or CycloneDX conformance.

## Review and pre-merge reconciliation

Immediately before implementation merge:

- PR #6 was open, non-draft, and mergeable;
- `main` still matched shaping merge `91b981ee89ab511444e5a17ca80f5312a31c2122`;
- PR head still matched exact qualified head `7c20f2dc1a2a3c8c0cc7674af4241f10f50c6cfb`;
- exact compare remained 11 commits ahead, zero behind, with the same 11 changed paths;
- no submitted pull-request reviews existed;
- no inline review threads existed;
- Qodo reported review unavailable because its trial had ended;
- CodeRabbit reported automatic substantive review was skipped because the repository had fewer than 10 stars;
- those unavailable/skipped review services were not represented as substantive review PASS;
- branch protection/required status checks were not configured and their absence was not represented as successful required-check policy.

Merge used GitHub expected-head protection with exact head `7c20f2dc1a2a3c8c0cc7674af4241f10f50c6cfb`.

## Post-merge verification

Canonical `main` was re-read at signed merge `dead41f7254386c643973af081cccc2f238d3a67`.

Push-triggered GitHub Actions run `34044759392` executed on that exact merge and completed successfully across the same 10 job groups:

- Provenance / Ubuntu;
- Provenance / macOS;
- Provenance / Windows;
- Provenance adversarial self-test;
- Rust / Ubuntu;
- Rust / macOS;
- Rust / Windows;
- pinned SpecGrain validation;
- Diffcipline R2 exact-diff proof;
- negative controls.

This provides executable confirmation that canonical merged state retains the admission gate, generated closure, dependency-free Rust foundation, and fail-closed verification behavior.

## Local prototype evidence

An isolated Python 3.13 prototype of the same gate design returned `PROVENANCE PASS`, `SELF-TEST PASS`, `GENERATED`, and `GENERATED OUTPUTS PASS`. That prototype was useful development evidence but was not used as merge qualification; exact GitHub-hosted evidence above is authoritative.

## Remaining closeout condition

Specification 002 becomes `CLOSED_CANONICAL` only after the closeout PR containing this evidence and final ledger reconciliation merges, canonical `main` is re-read, and applicable post-closeout CI succeeds. Until then Specification 003 remains blocked.
