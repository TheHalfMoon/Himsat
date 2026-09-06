# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_001_SHAPING
ACTIVE_SPECIFICATION = 001-repository-foundation
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_000_MERGE = 3f6687b34530e55098f7854042b12adfa607f394
NATIVE_SPEC_GRAIN_STATE = NOT_YET_ESTABLISHED
PRODUCT_IMPLEMENTATION_AUTHORITY = NONE_DURING_SHAPING
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Specification 000 established the canonical planning foundation and was merged to `main` as signed GitHub merge `3f6687b34530e55098f7854042b12adfa607f394` with exact planning head `970ddab1de876afd2a3d88c851783f9087d1efe8` as its second parent.

The active frontier is now **shaping Specification 001 only**. No implementation authority is implied until the Specification 001 shaping change itself is reconciled and canonical.

This file records repository governance state. It does not assert that native SpecGrain tooling has promoted Specification 001 to `GRAIN`.

## Active objective

Shape the smallest repository/delivery-control implementation unit required before Himsat can safely accept product code or donor adoption:

- pinned Rust toolchain;
- minimal dependency-free Rust workspace/core crate;
- native SpecGrain local state/check integration;
- Diffcipline policy/proof integration;
- least-privilege multi-platform CI;
- open-source license and minimal contribution/security surfaces.

The active specification is:

```text
specs/001-repository-foundation/spec.md
specs/001-repository-foundation/plan.md
specs/001-repository-foundation/tasks.md
```

Toolchain research:

```text
docs/research/2026-09-06-spec001-toolchain-baseline.md
```

## Why product implementation remains intentionally unauthorized during shaping

The current canonical repository contains planning/governance documents but no executable workspace or delivery-control configuration. Starting donor or product work now would bypass the predecessor gate established by the execution master plan.

Specification 001 shaping therefore selects and bounds the repository foundation first. Its implementation tasks remain explicitly unauthorized until the shaping PR closes canonically.

## Specification 001 shaping closeout conditions

The shaping change can authorize implementation only when:

1. live canonical base remains `3f6687b34530e55098f7854042b12adfa607f394` or any intervening change is explicitly reconciled;
2. scope-in and scope-out are explicit;
3. Rust/toolchain decision is bound to current official evidence;
4. Node/Tauri/mobile dependencies are explicitly deferred as unnecessary to this bounded unit;
5. exact SpecGrain and Diffcipline revisions/contracts are identified;
6. risk, recovery, context budget, expected change surface, acceptance, evidence, minimality, and safety requirements are explicit;
7. no product implementation, donor code, workflow, dependency manifest, model, or release artifact is smuggled into the shaping diff;
8. exact changed paths and PR state are reviewed before merge;
9. absent/unavailable verification remains `NOT RUN`/absent rather than PASS;
10. shaping merges with expected-head protection and canonical `main` is re-read.

## Successor rule

Once Specification 001 shaping is canonical, only the bounded Specification 001 implementation defined by its exact spec/plan/tasks becomes authorized.

Specification 002 donor/provenance machinery does **not** become implementation-authorized merely because Specification 001 exists. Donor adoption remains blocked until Specification 002 later establishes machine-readable provenance/license controls.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       SHAPING
      -> 002 Provenance/license/SBOM        BLOCKED
      -> 003 Core event/schema foundation  BLOCKED
          -> 004 Vault/key architecture    BLOCKED
              -> 005 Crash-safe media journal/chunk store
                  -> 006 Capture abstraction + Capture Health
                      -> 007 macOS capture
                      -> ...
```

The detailed program remains canonical in `docs/execution-master-plan.md`.

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- No donor code may be copied until a future provenance/license unit authorizes the exact material.
- No benchmark superiority claim may be made before reproducible comparative evidence exists.
- No platform capability may be claimed merely because an API exists.
- No specification may be called `CLOSED_CANONICAL` solely because a checklist is complete; exact evidence is required.
- Native SpecGrain lifecycle state must come from actual validated tool state, never from this Markdown label.
