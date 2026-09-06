# Himsat Provenance Gate

This directory is the machine adoption authority for third-party material. `docs/donor-and-provenance.md` and donor research documents are planning/research only and cannot authorize adoption by themselves.

## Files

- `policy.json` classifies public license identifiers as `allow`, `manual`, or `deny` under the current machine model.
- `registry.json` contains exact adopted or reference-only entries. It starts with zero adopted entries.
- `fixtures/cases.json` contains positive and adversarial policy cases.
- `founder-source-reuse-authority.md` records the founder's attestation of special source-code reuse permission from the discussed source projects.
- `special-permission-donor-matrix.md` maps how that permission changes planning eligibility without granting byte-level adoption authority.
- `../../generated/sbom.json` is deterministic `himsat.sbom/v1` output.

## Founder special-permission overlay

The founder has stated that Himsat has permission from the owners of all source-code projects discussed through 2026-09-06 to use/copy/modify/incorporate their project-owned source code as needed.

That statement changes the **rights-planning layer** but does not currently change the machine validator automatically. The existing validator still requires an `allow` public source license for non-reference adoption.

Issue #14 tracks the bounded machine-model extension required before Himsat can adopt a publicly non-permissive source path under special permission without falsifying the public license truth.

Until that extension is implemented and an active SpecGrain unit authorizes the exact path:

- publicly permissive code may continue through the current normal provenance path;
- publicly restricted code with special permission remains planning-eligible but machine-adoption blocked;
- `registry.json` stays authoritative for what has actually entered Himsat.

Special permission is source-code scoped. Third-party/vendor/generated code, models, datasets, fonts, assets, trademarks, and prebuilt binaries remain independently controlled unless separately authorized.

## Entry contract — current machine model

Non-reference adoption currently requires:

- unique machine ID and artifact kind;
- explicit `copy`, `depend`, or `vendor` mode with `adopted: true`;
- HTTPS source repository;
- immutable 40- or 64-hex Git revision;
- exact repository-relative source paths;
- an `allow` source license under the current validator;
- notice text;
- for copied/vendored bytes, exact destination paths plus matching SHA-256 records;
- for models, datasets, fonts, and assets, an independent `artifact_license`;
- for Cargo dependencies, exact lockfile name/version/source/checksum identity.

Reference-only records use `adoption_mode: "reference"` and `adopted: false`. They may document restricted or unknown-license systems, but cannot contain destination artifacts or package adoption and are excluded from notices/SBOM distribution components.

## Planned special-permission machine extension

The future extension tracked by Issue #14 should preserve `public_license` as factual provenance and add explicit permission fields such as:

```text
permission_basis
permission_scope
permission_evidence_reference
rights_holder_or_grantor
third_party_exclusions
```

A special permission must never work as a global `deny -> allow` switch. It must be path-scoped, evidence-bearing, and unable to authorize material outside its stated scope.

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
