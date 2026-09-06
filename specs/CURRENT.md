# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_002_SHAPING
ACTIVE_SPECIFICATION = 002-provenance-sbom
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_001_CLOSEOUT_MERGE = 90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0
SPEC_001_POST_CLOSEOUT_CI = 34042822934_SUCCESS
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
IMPLEMENTATION_AUTHORITY = NONE_DURING_SPEC_002_SHAPING
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Specification 001 is closed canonical. Its implementation, closeout, and post-merge verification are recorded in `specs/001-repository-foundation/evidence.md`.

The active frontier is now **Specification 002 shaping only**. This does not authorize donor copying, donor dependencies, models, assets, product features, or release work.

## Active objective

Shape the smallest deterministic provenance/license/SBOM control plane that can safely precede donor adoption:

- machine-readable adoption registry distinct from the research donor list;
- fail-closed license policy for allow/manual/deny/unknown cases;
- exact repository/revision/path identity requirements;
- SHA-256 integrity requirements for copied/vendored/model/asset material;
- Cargo dependency-to-registry closure;
- deterministic third-party notices and a Himsat-owned machine SBOM format;
- positive and adversarial fixtures proving policy behavior;
- CI integration without adding product runtime dependencies.

Active artifacts:

```text
specs/002-provenance-sbom/spec.md
specs/002-provenance-sbom/plan.md
specs/002-provenance-sbom/tasks.md
docs/research/2026-09-06-spec002-provenance-design.md
```

## Authority boundary

During shaping, no implementation is authorized. In particular:

- `docs/donor-and-provenance.md` remains a research registry, not adoption authority;
- the future machine registry must start with no donor adoption authorization;
- no Meetily, Anarlog, Xberg, Whisper, model, font, dataset, or other donor bytes may enter;
- no Cargo/package dependency changes are authorized;
- product runtime remains Rust-first, but governance tooling language may be selected by this specification based on minimality;
- no SPDX/CycloneDX compliance claim may be made unless a later implementation actually satisfies the relevant standard.

## Successor rule

If the Specification 002 shaping change merges after exact-head qualification, only its bounded provenance/SBOM implementation becomes authorized. Donor adoption remains blocked until that implementation and its closeout prove the exact adoption contract.

Specification 003 and all product behavior remain blocked until their dependency gates are satisfied.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        SHAPING
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
