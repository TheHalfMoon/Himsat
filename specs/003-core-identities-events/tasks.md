# Specification 003 Tasks — Core Identities, Schemas, and Events

> Checkboxes track authored/reconciled work. Exact repository state, CI, and proof remain authoritative.

## Shaping

- [x] S001 Re-read canonical `main` at Specification 002 closeout merge `42648f015ba611929fb0aa8f2dba90aab72fc80e`.
- [x] S002 Confirm post-closeout CI run `34044962282` completed successfully on the closeout merge.
- [x] S003 Re-read constitution, execution-plan unit 003, architecture, qualification, and provenance authority boundaries.
- [x] S004 Define the smallest dependency-free contract for typed IDs, schema version, evidence locators, producer identity, and foundational events.
- [x] S005 Keep persistence, serialization, timestamps, ID generation, encryption, capture, domain intelligence, sync, donor adoption, and release work out of scope.
- [x] S006 Define implementation sequence, adversarial cases, evidence requirements, risk, recovery, and closeout rule.
- [ ] S007 Qualify and merge this shaping candidate before implementation authority exists.

## Implementation — crate and identities

Not authorized until S007 closes.

- [ ] I001 Add exactly one dependency-free `himsat-events` workspace crate.
- [ ] I002 Define strongly typed `SessionId`, `SourceId`, `EventId`, and `ArtifactId` newtypes.
- [ ] I003 Implement canonical lowercase 32-hex display and strict 32-hex parsing.
- [ ] I004 Define `SchemaVersion`, `CORE_SCHEMA_VERSION`, and explicit compatibility semantics.
- [ ] I005 Define non-negative per-session `EventSequence` as a `u64` value type.

## Implementation — logical records and evidence

- [ ] I006 Define minimal `Session`, `Source`, and `Artifact` records.
- [ ] I007 Define bounded Himsat-owned `ArtifactKind` categories without storage semantics.
- [ ] I008 Define checked time and ordinal range value types.
- [ ] I009 Define checked integer-millionth `NormalizedRegion` values.
- [ ] I010 Define source-aware `EvidenceRef` variants for whole artifact, audio, transcript, document page, screen frame, and manual note evidence.

## Implementation — producer and event contracts

- [ ] I011 Define validated `ModelIdentity` and `ParserIdentity` records.
- [ ] I012 Define foundational `CoreEvent` variants only.
- [ ] I013 Define versioned `EventEnvelope` with event/session/sequence relationships.
- [ ] I014 Keep constructors/helpers deterministic, side-effect free, and independent of storage/network/global registries.

## Positive and adversarial evidence

- [ ] N001 Identity parse/display round trips pass for each ID class.
- [ ] N002 Short, long, and non-hex identity strings fail deterministically.
- [ ] N003 Schema compatibility boundaries reject incompatible future major versions.
- [ ] N004 Reversed time/ordinal ranges fail.
- [ ] N005 Normalized region coordinates/extents outside the unit square fail.
- [ ] N006 Boundary-valid normalized regions succeed without floating-point state.
- [ ] N007 Empty required model/parser identity fields fail.
- [ ] N008 Foundational event/session/source/artifact relationship invariants are exercised.
- [ ] N009 Workspace dependency closure contains no external Cargo package.
- [ ] N010 Existing provenance self-test/generated-output failure detection remains intact.

## Qualification

- [ ] Q001 Extend Diffcipline expected paths for Specification 003 and `himsat-events` without weakening forbidden surfaces or R2 commands.
- [ ] Q002 Push exact bounded implementation candidate and record head/base compare.
- [ ] Q003 Confirm Cargo manifest/lockfile changes add only the local workspace crate and no external package.
- [ ] Q004 Require provenance validate/check-generated/self-test success.
- [ ] Q005 Require Rust fmt/clippy/tests on Ubuntu, macOS, and Windows.
- [ ] Q006 Require SpecGrain and Diffcipline R2 exact-head success.
- [ ] Q007 Require existing negative controls to remain successful.
- [ ] Q008 Reconcile exact changed paths, reviews, threads, comments, `main`, and mergeability.
- [ ] Q009 Merge only with expected-head protection.
- [ ] Q010 Re-read canonical `main` and require post-merge CI success.
- [ ] Q011 Record closeout evidence and close Specification 003 only after all required proof is exact and durable.

## Explicit non-tasks

- database/event-log implementation;
- serialization or wire-format selection;
- UUID/ULID/random/time dependency adoption;
- persistent migration implementation;
- encryption or key management;
- media journal/chunk storage;
- capture/audio/transcription/diarization/document/search/memory behavior;
- donor code/dependencies/models/assets;
- FFI/mobile/desktop UI/network/sync/plugins/agents/connectors;
- release publication or public compatibility claims.
