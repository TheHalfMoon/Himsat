# Himsat Provenance Gate

This directory is the machine adoption authority for third-party material. `docs/donor-and-provenance.md` is research only and cannot authorize adoption.

## Files

- `policy.json` classifies license identifiers as `allow`, `manual`, or `deny`.
- `registry.json` contains exact adopted or reference-only entries. It starts with zero adopted entries.
- `fixtures/cases.json` contains positive and adversarial policy cases.
- `../../generated/sbom.json` is deterministic `himsat.sbom/v1` output.

## Entry contract

Non-reference adoption requires:

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
