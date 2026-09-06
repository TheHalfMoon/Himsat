# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_003_SHAPING
ACTIVE_SPECIFICATION = 003-core-identities-events
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_002_DISPOSITION = CLOSED_CANONICAL
SPEC_001_CLOSEOUT_MERGE = 90fc14196a4d5ce486d068e1fc6c45c19e2cc7f0
SPEC_002_SHAPING_MERGE = 91b981ee89ab511444e5a17ca80f5312a31c2122
SPEC_002_IMPLEMENTATION_MERGE = dead41f7254386c643973af081cccc2f238d3a67
SPEC_002_CLOSEOUT_MERGE = 42648f015ba611929fb0aa8f2dba90aab72fc80e
SPEC_002_POST_CLOSEOUT_CI = 34044962282_SUCCESS
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
IMPLEMENTATION_AUTHORITY = NONE_DURING_SPEC_003_SHAPING
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Live GitHub truth proves Specification 002 closeout merged at `42648f015ba611929fb0aa8f2dba90aab72fc80e` and post-closeout CI run `34044962282` completed successfully. Specification 002 is therefore `CLOSED_CANONICAL`.

The active frontier is **Specification 003 shaping only**. No Specification 003 implementation exists until its shaping candidate is independently qualified and merged.

## Active objective

Shape the smallest dependency-free Rust contract for foundational cross-module identity and evidence lineage:

- strongly typed `SessionId`, `SourceId`, `EventId`, and `ArtifactId`;
- explicit schema version and per-session event sequence types;
- minimal `Session`, `Source`, and `Artifact` logical records;
- source-aware `EvidenceRef` locators with checked ranges/regions;
- model/parser producer identities distinct from legal provenance;
- a minimal logical `EventEnvelope` and foundational `CoreEvent` variants;
- focused and adversarial tests for the invariants above.

Active artifacts:

```text
specs/003-core-identities-events/spec.md
specs/003-core-identities-events/plan.md
specs/003-core-identities-events/tasks.md
```

## Authority boundary

During Specification 003 shaping:

- no `himsat-events` crate implementation is authorized yet;
- no database/event-log, serialization/wire format, timestamp/clock model, ID generation, migration, encryption, key-management, media storage, capture, transcription, document, search, memory, sync, plugin, agent, connector, UI, or release implementation is authorized;
- no donor code/dependency/model/dataset/font/asset adoption is authorized;
- the Specification 002 provenance registry remains the sole machine adoption boundary and currently authorizes zero third-party entries;
- no public compatibility or superiority claim is authorized.

## Successor rule

If the Specification 003 shaping candidate merges after exact-head qualification, only the bounded dependency-free core-identity/event implementation defined by that canonical shaping change becomes authorized.

Specification 004 remains blocked until Specification 003 implementation and closeout complete with exact evidence.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        CLOSED_CANONICAL
      -> 003 Core event/schema foundation  SHAPING
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
