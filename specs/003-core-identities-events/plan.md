# Specification 003 Plan — Core Identities, Schemas, and Events

## Objective

Establish the smallest dependency-free Rust contract for Himsat-owned identity, source-aware evidence locators, producer identity, schema versioning, and foundational event relationships before any persistence, encryption, capture, transcription, document, memory, or sync implementation depends on incompatible ad-hoc types.

## Canonical base

```text
base = 42648f015ba611929fb0aa8f2dba90aab72fc80e
post_closeout_ci = 34044962282_SUCCESS
```

Live GitHub truth proves Specification 002 closeout and post-closeout CI succeeded. This authorizes Specification 003 shaping only. Implementation remains blocked until this shaping candidate is qualified and merged.

## Design choices

### 1. One dependency-free crate

The implementation unit may add exactly one new workspace crate:

```text
crates/himsat-events
```

Use only the Rust standard library. Do not add UUID, ULID, serde, time, random, serialization, database, crypto, or other external dependencies.

### 2. IDs are logical values, not allocation policy

Use four distinct `u128` newtypes: `SessionId`, `SourceId`, `EventId`, and `ArtifactId`.

Each type must provide deterministic value construction, canonical lowercase 32-hex display, and strict 32-hex parsing. The crate does not allocate IDs and makes no entropy, monotonicity, time-ordering, or global-uniqueness claim.

### 3. Schema versioning is explicit but migration-free

Introduce `SchemaVersion { major, minor }` and `CORE_SCHEMA_VERSION = 1.0`. Compatibility logic must be explicit and narrow; the crate performs no migration and defines no persistence/wire encoding.

### 4. Evidence locators preserve source type

Represent evidence with typed variants rather than string citations. Use integer microseconds/ordinals and integer millionths for normalized regions to avoid floating-point comparison ambiguity.

Constructors must reject reversed ranges, coordinate overflow, and invalid normalized regions before values enter durable downstream contracts.

### 5. Producer identity is operational identity, not legal provenance

`ModelIdentity` and `ParserIdentity` identify the runtime/revision that produced an artifact. They must not duplicate or weaken the machine adoption/license authority in `governance/provenance/registry.json`.

### 6. Event envelope is logical only

Introduce a minimal `EventEnvelope` containing schema version, event ID, session ID, per-session sequence, and a foundational `CoreEvent` payload. Do not choose serialization, timestamps, storage, event-log durability, global ordering, or sync semantics here.

### 7. Relationship validation stays local and explicit

Constructors/helpers should enforce only relationships this crate can prove from supplied values. Do not invent repository/database lookups or hidden global registries.

## Proposed implementation files

```text
Cargo.toml
Cargo.lock
crates/himsat-events/Cargo.toml
crates/himsat-events/src/lib.rs
.diffcipline.toml
specs/003-core-identities-events/spec.md
specs/003-core-identities-events/plan.md
specs/003-core-identities-events/tasks.md
specs/CURRENT.md
```

`Cargo.lock` may change only to add the local workspace package representation; no external package is authorized.

## Implementation sequence

### Phase A — crate and identity foundation

1. reverify shaping merge and exact implementation base;
2. add `himsat-events` as an explicit workspace member;
3. define four identity newtypes with parse/display invariants;
4. define `SchemaVersion`, compatibility predicate, and `EventSequence`.

### Phase B — logical records and evidence

1. define minimal `Session`, `Source`, and `Artifact` records;
2. define `ArtifactKind` without persistence details;
3. define range/region value types with checked constructors;
4. define source-aware `EvidenceRef` variants.

### Phase C — producer and event contracts

1. define validated model/parser identities;
2. define foundational `CoreEvent` variants;
3. define `EventEnvelope` and relationship-preserving constructors/helpers;
4. keep all APIs deterministic and side-effect free.

### Phase D — adversarial proof

Add tests for:

- identity round trips and type separation;
- short/long/non-hex/mixed malformed ID inputs;
- schema compatibility boundaries;
- reversed time/ordinal ranges;
- normalized region overflow and boundary-valid regions;
- empty producer identity fields;
- event/session/source/artifact relationship invariants;
- dependency-free workspace closure.

### Phase E — qualification and closeout

1. preserve provenance validation and generated closure;
2. run fmt/clippy/tests on exact head across Ubuntu/macOS/Windows;
3. require SpecGrain and Diffcipline R2 executed proof;
4. reconcile exact paths, Cargo changes, reviews/threads/comments, main, and mergeability;
5. repair forward only;
6. merge with expected-head protection;
7. require post-merge CI;
8. record durable closeout evidence before Specification 004 shaping becomes authorized.

## Verification baseline

Expected implementation commands include equivalents of:

```text
python tools/provenance.py validate
python tools/provenance.py check-generated
python tools/provenance.py self-test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo tree --workspace --locked --depth 1
```

These are planned proof commands, not evidence until executed on the exact candidate.

## Failure cases

- ID string not exactly 32 hex digits;
- wrong ID type used where another logical identity is required;
- schema compatibility incorrectly accepts a future major version;
- range end before start;
- normalized coordinate/extent outside `0..=1_000_000`;
- region origin plus extent overflows the normalized unit square;
- empty model/parser identity field;
- foundational event carrying inconsistent session/source/artifact relationship;
- accidental third-party Cargo dependency;
- provenance/generated-output regression;
- exact-head CI or Diffcipline not executed successfully.

## Residual risks

- 128-bit value representation does not itself define secure or collision-resistant ID generation;
- logical event contracts may need explicit version evolution once persisted consumers exist;
- no timestamp or cross-source clock model exists yet;
- evidence locators prove addressability, not semantic sufficiency;
- this crate does not validate that referenced artifacts actually exist in storage;
- pre-release contract resets remain possible until durable compatibility is separately authorized.

## Closeout rule

Specification 003 closes only after its implementation is merged, exact-head and post-merge CI pass, dependency closure proves no external package addition, adversarial tests cover the defined invariants, and durable closeout evidence records that persistence, encryption, capture, donor adoption, and release authority remain outside this unit.
