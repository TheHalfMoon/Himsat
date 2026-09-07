# Himsat Provenance Gate

This directory is the machine adoption authority for third-party material. `docs/donor-and-provenance.md`, research documents, founder permission records, and planning matrices inform eligibility and planning but cannot authorize adopted bytes by themselves.

## Files

- `policy.json` classifies license identifiers as `allow`, `manual`, or `deny`.
- `registry.json` contains exact adopted or reference-only entries and, for adopted dependencies that embed independently licensed native source, exact nested `native_components` records.
- `source-use-authorization.md` records the founder/user source-use attestation for every external source referenced in the repository at or before its covered canonical snapshot.
- `special-permission-donor-matrix.md` records planning posture for implementation-relevant covered donors; it is not the byte-adoption registry.
- `fixtures/cases.json` preserves the original positive/adversarial registry-policy matrix.
- `fixtures/native-cases.json` contains positive/adversarial tests for independently licensed embedded native components.
- `../../generated/sbom.json` is deterministic `himsat.sbom/v1` output.

## Canonical gate

`tools/provenance_gate.py` is the canonical composite gate. It preserves the complete legacy Cargo/artifact closure checks from `tools/provenance.py` and adds exact representation/validation of independently licensed embedded native components.

`tools/provenance.py` remains the compatibility/base validator used internally by the composite gate and by its legacy adversarial self-test. It is not the canonical command entrypoint after the v2 registry is adopted.

The composite gate is Python 3.13 standard-library only plus the repository-local `provenance` module. It performs no network access and does not execute registry-derived commands. Immutable upstream/native verification must already have been established by the controlling specification evidence before an entry is adopted.

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

### Embedded native components

An adopted Cargo dependency that embeds independently licensed native source MUST represent that source separately in `native_components` instead of inheriting or relabeling the Cargo wrapper's license.

Each native component requires exactly:

```text
id
name
source_repository
source_revision
source_paths
source_license
embedded_paths
notice
evidence_reference
```

The gate requires:

- globally unique native-component ID;
- HTTPS upstream repository;
- immutable 40- or 64-hex source revision;
- non-empty safe source paths and embedded paths;
- an `allow` license disposition for the exact native source;
- non-empty notice text;
- repository-relative evidence reference;
- attachment only to an adopted Cargo `depend` entry.

Native components are emitted separately under their parent dependency in generated notices and SBOM provenance. A wrapper's permissive license never substitutes for the independently licensed embedded source.

Specification 004P currently uses this model for the exact SQLCipher, embedded SQLite, and OpenSSL source carried through the selected `libsqlite3-sys` / `openssl-src` closure.

`LicenseRef-SQLite-Public-Domain` is a narrow Himsat policy identifier for the exact SQLite implementation source proven public-domain by the pinned SQLCipher/SQLite evidence. It is not a claim that unrelated material is CC0/MIT or that every file in an upstream repository is public domain.

## Separate-permission machine gap

The founder source-use authorization changes planning eligibility for covered sources, including sources whose public terms are restrictive. The v2 embedded-native extension does **not** implement separate permission as an alternative non-reference rights basis; normal adopted source still requires an `allow` public source license.

Issue #14 remains the bounded future extension. Its intended model preserves public-license truth and adds explicit fields such as:

```text
public_license
permission_basis
permission_scope
permission_evidence_reference
rights_holder_or_grantor
third_party_exclusions
```

Until that extension is implemented and an active SpecGrain leaf authorizes an exact source path, publicly restricted source remains planning-eligible under the founder authorization but machine-adoption blocked. Do not weaken `policy.json`, relabel a restrictive public license, or encode founder permission as an `allow` license to bypass this gate.

This gap does not affect ordinary adoption under an already compatible public license; those entries still require the full provenance, notice, digest, Cargo/native closure, and exact-head qualification.

## Commands

```text
python tools/provenance_gate.py validate
python tools/provenance_gate.py check-generated
python tools/provenance_gate.py self-test
python tools/provenance_gate.py generate
python tools/provenance_gate.py render-notices
python tools/provenance_gate.py render-sbom
```

`himsat.sbom/v1` remains Himsat's deterministic project schema. Native-component records enrich its component objects; this is not an SPDX or CycloneDX conformance claim.
