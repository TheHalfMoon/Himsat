# Specification 002 Research — Provenance, License, and SBOM Design

Date: 2026-09-06

## Problem

Himsat intends to reuse excellent permissively licensed software, models, and assets. The risk is not lack of candidates; it is accidentally turning a research list into blanket adoption authority, losing exact source identity, inheriting mixed-license subtrees, or shipping third-party material without notices or verifiable integrity.

Specification 002 must therefore create a deterministic admission gate before any donor bytes enter Himsat.

## Design conclusions

### 1. Research and authority are separate

`docs/donor-and-provenance.md` remains research. A candidate appearing there conveys no permission to copy, depend, vendor, bundle, or redistribute it.

The machine registry introduced by Specification 002 starts empty. Each future adopted object receives an exact entry only in the PR that proposes its adoption.

### 2. Use standard-library Python for governance tooling

The proposed validator/generator uses Python 3.13 standard library only (`json`, `tomllib`, `hashlib`, `pathlib`, `subprocess` where bounded). Himsat product runtime remains Rust-first.

This is a minimality and supply-chain decision: adding a Rust parsing/CLI dependency before the dependency-governance system exists would expand the very supply chain this unit is supposed to control. Python is already a pinned CI control-plane runtime from Specification 001.

### 3. Fail closed on incomplete identity

A non-reference adoption must identify at minimum:

```text
entry id and kind
adoption mode
source repository
immutable source revision
source path(s)
controlling license
notice/attribution requirements
Himsat destination or package identity
approval evidence
verification/test references
```

Copy/vendor/model/asset records additionally require SHA-256 integrity data appropriate to the artifact. A symbolic branch such as `main`, missing path, missing license, unknown license, or missing digest cannot become authorized adoption.

### 4. License policy has three machine dispositions

- `allow`: permissive licenses selected by repository policy, still subject to exact-path review;
- `manual`: mixed/weak-copyleft/ambiguous cases requiring explicit human/legal governance before adoption;
- `deny`: restricted, non-commercial, source-available, strong-copyleft-by-default, unknown/no-license, or explicitly incompatible cases.

A denied/manual source may be kept as a **reference** record if it contains no imported material. It may not silently become COPY/DEPEND/VENDOR authority.

### 5. Models, datasets, fonts, and assets are independent supply-chain objects

An engine's MIT/Apache license does not authorize its weights, tokenizer, dataset, font, icon, template, or bundled native library. The schema must represent separate artifact license and digest fields where applicable.

### 6. Cargo closure is a hard invariant

The validator should parse workspace metadata/`Cargo.lock` without adding dependencies. Every external Cargo package in the lockfile must map to an approved `depend` provenance entry. Current Himsat has no external Rust packages, so an empty adoption registry is valid.

Future Cargo manifest/lockfile edits remain Diffcipline `review` and cannot pass provenance closure without explicit registry entries.

### 7. Generated notices must be deterministic

`THIRD_PARTY_NOTICES.md` is generated from approved distributed entries only, in a stable order. Behavioral references are excluded. Generation must be idempotent, and CI fails when the checked-in file differs from regenerated output.

### 8. Start with an honest Himsat SBOM format

Generate `governance/generated/sbom.json` with schema identity `himsat.sbom/v1` containing Himsat workspace packages and approved third-party adoption entries.

Do not claim SPDX or CycloneDX compliance merely because the file resembles an SBOM. A later specification may add standards-compliant exports with conformance tests.

### 9. Local path safety matters

Registry destination/source snapshot paths must reject:

- absolute paths;
- `..` traversal;
- paths escaping the repository after resolution;
- unexpected symlink escapes where a verification path follows local files.

This prevents provenance tooling from becoming a file-read primitive against the CI host.

### 10. The gate must prove that it fails

Positive and negative fixtures are first-class evidence. Required cases include:

- permissive allowed record;
- manual-review license;
- denied/restricted license in adoption mode;
- unknown license;
- malformed/non-immutable revision;
- missing source path;
- model/asset missing independent license or digest;
- unsafe destination path;
- digest mismatch;
- unregistered external Cargo dependency;
- registered dependency whose expected package identity/checksum does not match observed lock state;
- generated notice/SBOM drift.

## Proposed implementation surface

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

No Cargo manifest/lockfile modification should be needed.

## Security boundaries

- validator/generator runs offline and does not fetch donor repositories;
- no shell command may be derived from registry content;
- source revisions are data, not commands;
- JSON output uses deterministic serialization;
- malformed or duplicate IDs fail closed;
- registry cannot authorize a path just because a repository-level license appears compatible;
- third-party notices are generated from explicit fields, not scraped dynamically during CI.

## Why this precedes donor copying

Himsat's ambition makes donor intake a permanent process, not a one-time import. Building the admission gate before Meetily/Anarlog/other adoption is slower for a few hours and dramatically cheaper than reconstructing provenance after thousands of copied lines, models, fonts, and native binaries already exist.
