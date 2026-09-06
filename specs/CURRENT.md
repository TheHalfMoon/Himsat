# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_003_IMPLEMENTATION
ACTIVE_SPECIFICATION = 003-core-identities-events
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_002_DISPOSITION = CLOSED_CANONICAL
SPEC_002_CLOSEOUT_MERGE = 42648f015ba611929fb0aa8f2dba90aab72fc80e
SPEC_002_POST_CLOSEOUT_CI = 34044962282_SUCCESS
SPEC_003_SHAPING_MERGE = f0cc839cbe908cd7c4bb4623c668a01db3bbbb07
SPEC_003_EXACT_SHAPING_CI = 34045966688_SUCCESS
SPEC_003_POST_SHAPING_CI = 34046036952_SUCCESS
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
IMPLEMENTATION_AUTHORITY = SPEC_003_CORE_CONTRACTS_ONLY
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Live GitHub truth proves Specification 003 shaping merged at `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07` after exact-head CI run `34045966688` succeeded, and post-shaping CI run `34046036952` also succeeded. This authorizes only the bounded Specification 003 implementation defined by the canonical spec/plan/tasks.

## Active objective

Implement and qualify one dependency-free Rust crate containing foundational logical contracts:

- strongly typed `SessionId`, `SourceId`, `EventId`, and `ArtifactId`;
- explicit `SchemaVersion`, `CORE_SCHEMA_VERSION`, and `EventSequence`;
- minimal `Session`, `Source`, and `Artifact` records;
- checked source-aware `EvidenceRef` locators;
- validated model/parser producer identities;
- a minimal logical `EventEnvelope` and foundational `CoreEvent` variants;
- focused/adversarial tests for all defined invariants.

The implementation must remain deterministic, side-effect free, and third-party-dependency-free.

## Authority boundary

Specification 003 implementation does **not** authorize:

- database/event-log persistence or a wire/serialization format;
- ID allocation/randomness/time/clock semantics;
- persisted-data migration;
- encryption, vaults, or key management;
- media journal/chunk storage;
- capture, transcription, diarization, documents, search, memory, sync, UI, plugins, agents, connectors, or release work;
- donor code/dependencies/models/datasets/fonts/assets.

The Specification 002 provenance registry remains the sole machine adoption boundary and still contains zero adopted third-party entries.

## Successor rule

Specification 004 remains blocked until Specification 003 implementation is exact-head qualified, merged with expected-head protection, post-merge CI succeeds, and durable closeout evidence is canonical.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        CLOSED_CANONICAL
      -> 003 Core event/schema foundation  IMPLEMENTATION_ACTIVE
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
