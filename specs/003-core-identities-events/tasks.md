# Specification 003 Tasks — Core Identities, Schemas, and Events

> Checkboxes track authored/reconciled work. Exact repository state, CI, and proof remain authoritative.

## Shaping

- [x] S001 Re-read canonical `main` at Specification 002 closeout merge `42648f015ba611929fb0aa8f2dba90aab72fc80e`.
- [x] S002 Confirm post-closeout CI run `34044962282` completed successfully on the closeout merge.
- [x] S003 Re-read constitution, execution-plan unit 003, architecture, qualification, and provenance authority boundaries.
- [x] S004 Define the smallest dependency-free contract for typed IDs, schema version, evidence locators, producer identity, and foundational events.
- [x] S005 Keep persistence, serialization, timestamps, ID generation, encryption, capture, domain intelligence, sync, donor adoption, and release work out of scope.
- [x] S006 Define implementation sequence, adversarial cases, evidence requirements, risk, recovery, and closeout rule.
- [x] S007 Qualify and merge shaping. Exact-head CI `34045966688` succeeded; shaping merge `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07`; post-shaping CI `34046036952` succeeded.

## Implementation — crate and identities

Authorized only by canonical shaping merge `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07`.

- [x] I001 Add exactly one dependency-free `himsat-events` workspace crate.
- [x] I002 Define strongly typed `SessionId`, `SourceId`, `EventId`, and `ArtifactId` newtypes.
- [x] I003 Implement canonical lowercase 32-hex display and strict 32-hex parsing.
- [x] I004 Define `SchemaVersion`, `CORE_SCHEMA_VERSION`, and explicit compatibility semantics.
- [x] I005 Define non-negative per-session `EventSequence` as a `u64` value type.

## Implementation — logical records and evidence

- [x] I006 Define minimal `Session`, `Source`, and `Artifact` records.
- [x] I007 Define bounded Himsat-owned `ArtifactKind` categories without storage semantics.
- [x] I008 Define checked time and ordinal range value types.
- [x] I009 Define checked integer-millionth `NormalizedRegion` values.
- [x] I010 Define source-aware `EvidenceRef` variants for whole artifact, audio, transcript, document page, screen frame, and manual note evidence.

## Implementation — producer and event contracts

- [x] I011 Define validated `ModelIdentity` and `ParserIdentity` records.
- [x] I012 Define foundational `CoreEvent` variants only.
- [x] I013 Define versioned `EventEnvelope` with event/session/sequence relationships.
- [x] I014 Keep constructors/helpers deterministic, side-effect free, and independent of storage/network/global registries.

## Positive and adversarial evidence

Executed on final exact head `ff24d5c3084d54627d50004f1e527ba06683696b` in CI run `34046989073`.

- [x] N001 Identity parse/display round trips pass for each ID class.
- [x] N002 Short, long, and non-hex identity strings fail deterministically.
- [x] N003 Schema compatibility boundaries reject incompatible future major versions.
- [x] N004 Reversed time/ordinal ranges fail.
- [x] N005 Normalized region coordinates/extents outside the unit square fail.
- [x] N006 Boundary-valid normalized regions succeed without floating-point state.
- [x] N007 Empty required model/parser identity fields fail.
- [x] N008 Foundational event/session/source/artifact relationship invariants are exercised.
- [x] N009 Workspace dependency closure contains no external Cargo package.
- [x] N010 Existing provenance self-test/generated-output failure detection remains intact.

## Qualification

- [x] Q001 Extend Diffcipline expected paths for Specification 003 and `himsat-events` without weakening forbidden surfaces or R2 commands.
- [x] Q002 Push bounded implementation candidate and reconcile final exact head `ff24d5c3084d54627d50004f1e527ba06683696b` against shaping base `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07`.
- [x] Q003 Confirm Cargo manifest/lockfile changes add only local `himsat-events`; final lockfile contains only `himsat-core` and `himsat-events`.
- [x] Q004 Require provenance validate/check-generated/self-test success; final exact-head run `34046989073` passed.
- [x] Q005 Require Rust fmt/clippy/tests on Ubuntu, macOS, and Windows; final exact-head run `34046989073` passed all three Rust jobs.
- [x] Q006 Require SpecGrain and Diffcipline R2 exact-head success; final exact-head run `34046989073` passed both jobs.
- [x] Q007 Require existing negative controls to remain successful; final exact-head run `34046989073` passed the negative-controls job.
- [x] Q008 Reconcile exact changed paths, reviews, threads, comments, `main`, and mergeability. PR #9 had no submitted reviews or inline threads; Qodo was billing-blocked and CodeRabbit skipped automatic review, neither counted as PASS; PR was mergeable at exact qualified head.
- [x] Q009 Reconcile merge-head safety. Canonical merge `21946f7abc9247cacf784220b1932ab5946c7540` has exact shaping base `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07` and exact qualified head `ff24d5c3084d54627d50004f1e527ba06683696b` as parents, proving no stale-head merge. Historical transport-level `expected_head_sha` request metadata is not exposed post-hoc and is explicitly not claimed as PASS evidence; closeout records this limitation and requires expected-head protection prospectively.
- [x] Q010 Re-read canonical `main` and require post-merge CI success; push run `34047078983` passed on exact merge `21946f7abc9247cacf784220b1932ab5946c7540`.
- [x] Q011 Record durable implementation evidence in `evidence.md` and restore dependency manifest/lockfile disposition to `review` for successor work.

## Closeout

- [ ] C001 Exact-head qualify the Specification 003 closeout candidate.
- [ ] C002 Reconcile closeout paths, reviews/threads/comments, canonical `main`, and mergeability immediately before closeout merge.
- [ ] C003 Merge closeout with explicit expected-head protection.
- [ ] C004 Re-read canonical closeout merge and require post-closeout CI success.
- [ ] C005 Mark Specification 003 `CLOSED_CANONICAL` only from successor shaping after C001-C004 are proven.

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
