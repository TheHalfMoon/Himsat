# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_002_IMPLEMENTATION
ACTIVE_SPECIFICATION = 002-provenance-sbom
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_001_CLOSEOUT_MERGE = 90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0
SPEC_002_SHAPING_MERGE = 91b981ee89ab511444e5a17ca80f5312a31c2122
SPEC_002_POST_SHAPING_CI = 34043416988_SUCCESS
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
IMPLEMENTATION_AUTHORITY = SPEC_002_MACHINERY_ONLY
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Live GitHub truth closed Specification 002 shaping at signed merge `91b981ee89ab511444e5a17ca80f5312a31c2122`. The shaping merge authorizes only the bounded provenance/license/SBOM machinery defined by the canonical Specification 002 spec, plan, and tasks.

## Active objective

Implement and qualify the smallest deterministic admission gate that must exist before any donor or third-party runtime adoption:

- Python 3.13 standard-library-only offline validator/generator;
- versioned allow/manual/deny license policy;
- initially empty machine adoption registry;
- exact source revision/path identity;
- SHA-256 verification for copied/vendored/distributed artifacts;
- independent model/dataset/font/asset licensing;
- Cargo lockfile-to-registry closure;
- deterministic `THIRD_PARTY_NOTICES.md` and `himsat.sbom/v1`;
- adversarial fixtures and generated-output drift detection;
- exact CI/Diffcipline proof without new product dependencies.

Active artifacts:

```text
specs/002-provenance-sbom/spec.md
specs/002-provenance-sbom/plan.md
specs/002-provenance-sbom/tasks.md
docs/research/2026-09-06-spec002-provenance-design.md
```

## Authority boundary

Specification 002 implementation authority does **not** authorize third-party adoption.

- `docs/donor-and-provenance.md` remains research only.
- The real machine registry must contain zero adopted entries in this unit.
- No Meetily, Anarlog, Xberg, Whisper, model, font, dataset, asset, or other donor bytes may enter.
- No Cargo/Python/Node dependency additions are authorized.
- No product schema, audio, storage, UI, mobile, document, memory, agent, connector, sync, or release work is authorized.
- No SPDX/CycloneDX compliance claim is authorized.

## Successor rule

Specification 003 remains blocked until Specification 002 machinery is merged, post-merge CI succeeds, closeout records exact evidence, and the provenance admission contract is canonical.

Even after Specification 002 closes, donor adoption requires an exact future registry entry and bounded authority for the specific dependency/path/material. Research-list presence never creates authority.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        IMPLEMENTATION_ACTIVE
      -> 003 Core event/schema foundation  BLOCKED
          -> 004 Vault/key architecture    BLOCKED
              -> 005 Crash-safe media journal/chunk store
                  -> 006 Capture abstraction + Capture Health
                      -> 007 macOS capture
                      -> ...
```

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- No donor code may be copied until exact machine-readable provenance authority exists and passes its gates.
- Model/asset/data licensing is independent of engine/software licensing.
- `NOT RUN`, unavailable, manual-review, and unknown are never equivalent to PASS.
- Native SpecGrain lifecycle state comes only from validated tool state.
