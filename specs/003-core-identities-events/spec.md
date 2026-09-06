# Specification 003 — Core Identities, Schemas, and Events

## Status

`SHAPING_CANDIDATE`

No implementation authority exists until this shaping change is qualified and merged.

## Outcome

Create the smallest Himsat-owned, dependency-free Rust contract for durable cross-module identity and evidence lineage before storage, capture, transcription, documents, memory, or sync build on incompatible ad-hoc types.

The implementation unit may introduce one real `himsat-events` crate containing:

- strongly typed `SessionId`, `SourceId`, `EventId`, and `ArtifactId`;
- explicit schema version and event sequence types;
- minimal logical `Session`, `Source`, and `Artifact` records;
- source-aware `EvidenceRef` locator types;
- model and parser runtime identity types;
- a minimal versioned `EventEnvelope` and foundational `CoreEvent` variants;
- invariant-preserving constructors/parsers and focused/adversarial tests.

This is a logical contract only. It does not choose the event database, serialization/wire format, encryption scheme, clock source, random-ID generator, CRDT/sync semantics, or domain intelligence model.

## Why this unit exists now

Later specifications need a common language for ownership and provenance:

- capture must identify sessions/sources/artifacts;
- media journaling must link durable chunks without defining its own IDs;
- transcription/document pipelines must produce revisioned artifacts;
- evidence must point back to source locations;
- model/parser outputs must identify the implementation that produced them;
- future persistence and migrations need an explicit schema-version boundary.

Creating these contracts before those units reduces schema fragmentation while keeping persistence and product-domain decisions deferred.

## Scope in

### Identity

Implement four distinct 128-bit identity newtypes:

```text
SessionId
SourceId
EventId
ArtifactId
```

Required behavior:

- value semantics (`Copy`, equality, ordering, hashing);
- explicit construction from `u128`;
- retrieval as `u128`;
- canonical lowercase 32-hex display;
- strict parse from exactly 32 hexadecimal digits;
- type separation prevents accidentally substituting one ID class for another.

This specification does not generate IDs and makes no uniqueness/entropy claim. Random/monotonic generation belongs to a later runtime/persistence decision.

### Schema version

Define a compact `SchemaVersion { major, minor }` value and a `CORE_SCHEMA_VERSION` constant initially representing `1.0`.

Compatibility semantics are intentionally narrow:

- equal major versions are structurally related;
- a reader may recognize an older/equal minor version only through an explicit compatibility predicate;
- no migration is performed by this crate;
- persistence/wire encodings remain unspecified.

### Logical records

Define minimal records only:

```text
Session { id }
Source { id, session_id }
Artifact { id, session_id, source_id?, kind }
```

`ArtifactKind` is a Himsat-owned semantic category needed for evidence validation. Initial categories may cover audio, transcript, document, screen frame, manual note, imported media, and derived material without embedding storage details.

### EvidenceRef

`EvidenceRef` must be source-type aware instead of an untyped string citation.

The minimal locator family is:

```text
WholeArtifact
AudioRange      -> ArtifactId + optional SourceId + TimeRangeMicros
TranscriptRange -> ArtifactId + ordinal range
DocumentPage    -> ArtifactId + zero-based page index + optional NormalizedRegion
ScreenFrame     -> ArtifactId + frame ordinal + optional NormalizedRegion
ManualNote      -> ArtifactId
```

Required invariants:

- range end cannot precede range start;
- normalized regions use integer millionths (`0..=1_000_000`) rather than floating-point coordinates;
- normalized region origin + extent must remain within the unit square;
- page/frame/segment semantics are explicitly zero-based/ordinal where applicable;
- `EvidenceRef` carries no claim that the referenced evidence is semantically sufficient; it is only a typed locator.

### Runtime producer identity

Define Himsat-owned `ModelIdentity` and `ParserIdentity` records sufficient to distinguish which implementation/revision produced an artifact.

They contain non-empty textual identity fields and optional 32-byte digest identity. They do not duplicate legal provenance; machine adoption/license authority remains in `governance/provenance/registry.json`.

### Event envelope

Define a minimal logical envelope:

```text
EventEnvelope {
  schema_version,
  event_id,
  session_id,
  sequence,
  event
}
```

`EventSequence` is a per-session non-negative ordinal represented as `u64`. The crate does not allocate sequences or claim global ordering.

Initial `CoreEvent` variants are limited to foundational relationships:

```text
SessionCreated
SourceRegistered { source_id }
ArtifactRegistered { artifact_id, source_id?, kind }
EvidenceLinked { evidence }
```

Later domain specifications add capture/transcript/document/etc. event payloads rather than prematurely placing them here.

## Scope out

Specification 003 does **not** authorize:

- database/event-log implementation or exact persistence format;
- serde/postcard/protobuf/FlatBuffers/MessagePack or any serialization dependency;
- UUID/ULID/randomness/time crates or ID-generation semantics;
- timestamps/source-clock synchronization;
- encryption, key hierarchy, vault/storage, migrations that touch persisted user data;
- capture/audio/transcription/diarization/document/search/memory implementation;
- model execution or parser execution;
- donor code/dependencies/models/assets;
- FFI bindings, desktop/mobile shells, sync/CRDT, networking, plugins, agents, connectors;
- release/public compatibility promises.

## Dependencies

Required canonical predecessors:

- Specification 001 `CLOSED_CANONICAL`;
- Specification 002 `CLOSED_CANONICAL`, including provenance admission machinery.

Implementation should remain Himsat-owned and third-party-dependency-free.

## Risk

`R2` — these contracts become foundational dependencies for later data-bearing subsystems. A small semantic mistake can propagate widely even though no user data persistence exists yet.

## Recovery

Before public persistence compatibility exists:

- repair forward with explicit schema-version changes;
- update focused fixtures/tests;
- if a contract is materially wrong before release, a documented pre-release schema reset is acceptable;
- do not silently reinterpret an existing version number.

Once durable persistence/export consumers exist, incompatible changes require a separately authorized migration specification.

## Acceptance

A Specification 003 implementation candidate is acceptable only when:

1. implementation base is the canonical shaping merge;
2. one dependency-free `himsat-events` crate is introduced and workspace membership is explicit;
3. Cargo changes contain no external package;
4. all identity classes are strongly typed and canonical parse/display round trips are tested;
5. malformed ID strings are rejected deterministically;
6. schema-version compatibility semantics are explicit and tested;
7. range/region constructors reject invalid boundaries/overflow;
8. `EvidenceRef` variants preserve source-location type information;
9. model/parser identities reject empty required identity fields;
10. foundational event-envelope relationships are tested without persistence assumptions;
11. provenance validation and generated closure still pass;
12. Rust fmt/clippy/tests pass on Ubuntu, macOS, and Windows;
13. SpecGrain and Diffcipline R2 exact-diff verification pass;
14. negative controls continue to prove failure detection;
15. exact changed paths, Cargo manifests/lockfile, reviews/threads/comments, `main`, and mergeability are reconciled before merge;
16. merge uses expected-head protection;
17. post-merge CI passes before implementation closeout;
18. no donor/product/persistence/release authority is inferred from this unit.

## Evidence

Required evidence includes:

- exact base/head SHAs and changed-path compare;
- Cargo package/dependency closure;
- focused positive/adversarial Rust tests;
- exact-head multi-platform CI;
- provenance gate and generated-output closure;
- SpecGrain validation;
- Diffcipline R2 executed proof;
- review/thread/comment truth;
- expected-head merge result;
- post-merge CI result;
- closeout record preserving residual non-authority.

## Minimality

Prefer a single dependency-free crate and a compact public API. Do not create future crates, persistence adapters, serializers, compatibility shims, or generic framework abstractions merely because later units may need them.
