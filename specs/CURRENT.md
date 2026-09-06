# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_002_CLOSEOUT
ACTIVE_SPECIFICATION = 002-provenance-sbom
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_001_CLOSEOUT_MERGE = 90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0
SPEC_002_SHAPING_MERGE = 91b981ee89ab511444e5a17ca80f5312a31c2122
SPEC_002_IMPLEMENTATION_MERGE = dead41f7254386c643973af081cccc2f238d3a67
SPEC_002_EXACT_HEAD_CI = 34044644315_SUCCESS
SPEC_002_POST_MERGE_CI = 34044759392_SUCCESS
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
IMPLEMENTATION_AUTHORITY = SPEC_002_CLOSEOUT_ONLY
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Specification 002 shaping and implementation have merged and passed exact-head plus post-merge verification. Durable executed proof is recorded in `specs/002-provenance-sbom/evidence.md`.

The only current authority is bounded Specification 002 closeout/reconciliation. Specification 003 remains blocked until this closeout merges, canonical `main` is re-read, and applicable post-closeout CI succeeds.

## Implemented admission boundary

Canonical implementation now contains:

- Python 3.13 standard-library-only offline provenance validator/generator;
- versioned allow/manual/deny license policy;
- machine adoption registry containing zero adopted third-party entries;
- exact revision/path and SHA-256 integrity checks;
- independent model/dataset/font/asset licensing rules;
- Cargo lockfile-to-registry closure;
- deterministic third-party notices and `himsat.sbom/v1`;
- adversarial fixture coverage and generated-output drift detection;
- Linux/macOS/Windows provenance CI bound into Diffcipline R2 proof.

This machinery does not itself authorize any donor.

## Authority boundary

- `docs/donor-and-provenance.md` remains research only.
- No Meetily, Anarlog, Xberg, Whisper, model, font, dataset, asset, or other donor bytes were adopted by Specification 002.
- No Cargo/Python/Node dependency addition was made.
- No product schema, audio, storage, UI, mobile, document, memory, agent, connector, sync, or release work is authorized during closeout.
- No SPDX/CycloneDX conformance claim is authorized.

## Successor rule

After Specification 002 is `CLOSED_CANONICAL`, Specification 003 may be shaped only. Its implementation remains blocked until its own shaping merge creates exact authority.

Future third-party adoption additionally requires an exact machine registry entry and explicit bounded authority for the specific material. A research-list entry alone never grants adoption authority.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        CLOSEOUT_ACTIVE
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
- No donor code may be copied without exact machine-readable provenance plus bounded adoption authority.
- Model/asset/data licensing is independent of engine/software licensing.
- `NOT RUN`, unavailable, manual-review, and unknown are never equivalent to PASS.
- Native SpecGrain lifecycle state comes only from validated tool state.
