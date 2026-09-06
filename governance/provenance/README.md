# Himsat Provenance Gate

This directory is the machine adoption authority for third-party material. `docs/donor-and-provenance.md`, research documents, founder permission records, and planning matrices inform eligibility and planning but cannot authorize adopted bytes by themselves.

## Files

- `policy.json` classifies license identifiers as `allow`, `manual`, or `deny`.
- `registry.json` contains exact adopted or reference-only entries. It starts with zero adopted entries.
- `source-use-authorization.md` records the founder/user source-use attestation for every external source referenced in the repository at or before its covered canonical snapshot.
- `special-permission-donor-matrix.md` records planning posture for implementation-relevant covered donors; it is not the byte-adoption registry.
- `fixtures/cases.json` contains positive and adversarial policy cases.
- `../../generated/sbom.json` is deterministic `himsat.sbom/v1` output.

## Entry contract

Non-reference adoption currently requires:

- unique machine ID and artifact kind;
- explicit `copy`, `depend`, or `vendor` mode with `adopted: true`;
- HTTPS source repository;
- immutable 40- or 64-hex Git revision;
- exact repository-relative source paths;
- an `allow` source license;
- notice text;
- for copied/vendored bytes, exact destination paths plus matching SHA-256 records;
- for models, datasets, fonts, and assets, an independent `artifact_license`;
- for Cargo dependencies, exact lockfile name/version/source/checksum identity.

Reference-only records use `adoption_mode: "reference"` and `adopted: false`. They may document restricted or unknown-license systems, but cannot contain destination artifacts or package adoption and are excluded from notices/SBOM distribution components.

## Separate-permission machine gap

The founder source-use authorization changes planning eligibility for covered sources, including sources whose public terms are restrictive. The current `registry.json` schema and validator do **not** yet model a separate permission as an alternative non-reference rights basis; they still require an `allow` public source license.

Issue #14 tracks the bounded extension. The intended model preserves public-license truth and adds explicit fields such as:

```text
public_license
permission_basis
permission_scope
permission_evidence_reference
rights_holder_or_grantor
third_party_exclusions
```

Until that extension is implemented and an active SpecGrain leaf authorizes an exact source path, publicly restricted source remains planning-eligible under the founder authorization but machine-adoption blocked. Do not weaken `policy.json`, relabel a restrictive public license, or encode founder permission as an `allow` license to bypass this gate.

This machine gap does not affect ordinary adoption under an already compatible public license; those entries still require the full existing provenance, notice, digest, dependency/native closure, and exact-head qualification.

## Commands

```text
python tools/provenance.py validate
python tools/provenance.py check-generated
python tools/provenance.py self-test
python tools/provenance.py generate
python tools/provenance.py render-notices
python tools/provenance.py render-sbom
```

The tool is Python 3.13 standard-library only. It performs no network access and does not execute registry-derived commands.

`himsat.sbom/v1` is Himsat's deterministic project schema. It is not an SPDX or CycloneDX conformance claim.
