# Specification 003 Evidence — Core Identities, Schemas, and Events

## Disposition

```text
SPECIFICATION = 003-core-identities-events
SHAPING_MERGE = f0cc839cbe908cd7c4bb4623c668a01db3bbbb07
IMPLEMENTATION_PR = 9
IMPLEMENTATION_HEAD = ff24d5c3084d54627d50004f1e527ba06683696b
IMPLEMENTATION_MERGE = 21946f7abc9247cacf784220b1932ab5946c7540
IMPLEMENTATION_EXACT_HEAD_CI = 34046989073_SUCCESS
IMPLEMENTATION_POST_MERGE_CI = 34047078983_SUCCESS
CLOSEOUT = PENDING_CLOSEOUT_MERGE
```

This file records executed repository evidence. It does not convert unavailable review systems, unknown historical API request metadata, or unexecuted checks into PASS. It does not authorize persistence, encryption, capture, donor adoption, product feature expansion, or release publication.

## Canonical lineage

- Specification 002 closeout merge: `42648f015ba611929fb0aa8f2dba90aab72fc80e`.
- Specification 002 post-closeout CI: `34044962282` SUCCESS.
- Specification 003 shaping PR: `#8`.
- Specification 003 shaping exact-head CI: `34045966688` SUCCESS.
- Specification 003 shaping merge: `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07`.
- Specification 003 post-shaping CI: `34046036952` SUCCESS.
- Specification 003 implementation PR: `#9`.
- Final qualified implementation head: `ff24d5c3084d54627d50004f1e527ba06683696b`.
- Exact-head implementation CI: `34046989073` SUCCESS.
- Canonical implementation merge: `21946f7abc9247cacf784220b1932ab5946c7540`.
- Post-implementation-merge CI: `34047078983` SUCCESS.

## Exact implementation scope

The final implementation PR reported 12 commits over shaping base, zero base drift before merge, 9 changed paths, 797 additions, and 64 deletions. The changed paths were:

```text
.diffcipline.toml
.github/workflows/ci.yml
Cargo.lock
Cargo.toml
crates/himsat-events/Cargo.toml
crates/himsat-events/src/lib.rs
specs/003-core-identities-events/tasks.md
specs/CURRENT.md
tools/provenance.py
```

The product implementation itself introduced exactly one new Rust workspace crate, `crates/himsat-events`. The other non-ledger changes repaired and strengthened qualification machinery exposed by executed CI: provenance self-test workspace-member copying and the exact dependency-free workspace invariant.

No persistence/database, serialization, timestamp/clock, ID allocation, encryption/key-management, media storage, capture, transcription, document, search, memory, sync, UI, plugin, agent, connector, donor, model, dataset, font, asset, binary, or release implementation entered this unit.

## Dependency closure

`Cargo.lock` contains exactly two local workspace packages:

```text
himsat-core 0.0.0
himsat-events 0.0.0
```

The Rust CI dependency-free workspace invariant executes `cargo tree --workspace --locked --depth 1`, requires exactly those two lockfile package blocks, and rejects any extra package. The provenance registry remained the sole machine adoption boundary and no third-party component was adopted by Specification 003.

The temporary implementation-unit Diffcipline disposition allowed the explicitly reviewed local manifest/lockfile changes only after the CI dependency-closure gate was strengthened. Closeout restores manifest and lockfile changes to `review` so successor units do not inherit this allowance.

## Positive and adversarial Rust evidence

The final exact-head CI executed the workspace tests on Ubuntu, macOS, and Windows. The workspace contained 1 existing `himsat-core` test and 10 `himsat-events` tests. The `himsat-events` tests exercised:

- all four ID parse/display round trips;
- malformed short, long, and non-hex identity rejection;
- schema compatibility boundaries;
- reversed time and ordinal range rejection;
- normalized-region component and unit-square bounds;
- typed evidence locator preservation;
- empty producer-identity field rejection and digest preservation;
- session/source/artifact relationship inheritance in foundational events;
- explicit-session preservation for evidence-link events.

All executed tests passed on the final exact implementation head.

## Qualification history and forward repairs

Two failed qualification runs are retained as evidence rather than erased:

1. Run `34046538162` on head `23e0159d17e952304f77ddc3a1fbe13d1537c035` exposed `cargo fmt --all -- --check` drift. The rustfmt diff was applied forward.
2. Run `34046652982` on head `a6ba03de2311079ca77ab2ff6237cc431485c570` proved Rust tests themselves passed but exposed two governance issues: the provenance self-test fixture hard-coded the single previous workspace member, and Diffcipline correctly returned a non-PASS disposition for manifest/lockfile changes. The fixture was generalized to copy every workspace member manifest, and the implementation-unit dependency policy was paired with an explicit exact two-package workspace invariant.

Final run `34046989073` executed on exact head `ff24d5c3084d54627d50004f1e527ba06683696b` and completed successfully across all 10 configured jobs:

- Rust / Ubuntu;
- Rust / macOS;
- Rust / Windows;
- Provenance / Ubuntu;
- Provenance / macOS;
- Provenance / Windows;
- Provenance / adversarial self-test;
- SpecGrain / pinned source;
- Diffcipline / R2 exact diff;
- Negative controls.

The negative-controls job proved formatting, clippy, test, malformed SpecGrain state, configured-but-not-run Diffcipline verification, and out-of-scope working-tree changes are detected rather than silently accepted.

## Review and pre-merge reconciliation

Immediately before the implementation merge was observed:

- PR #9 was open, non-draft, and mergeable;
- PR head was exact qualified head `ff24d5c3084d54627d50004f1e527ba06683696b`;
- base remained shaping merge `f0cc839cbe908cd7c4bb4623c668a01db3bbbb07`;
- exact-head CI run `34046989073` was terminal `SUCCESS`;
- no submitted pull-request reviews existed;
- no inline review threads existed;
- Qodo reported review unavailable because its trial had ended;
- CodeRabbit reported automatic substantive review was skipped because the repository had fewer than 10 stars;
- neither unavailable/skipped service was represented as substantive review PASS;
- branch `main` was not protected and no required-status-check rule was configured; this absence was not represented as a successful protection policy.

## Merge-guard evidence limitation and forward recovery

Live GitHub truth proves PR #9 merged at `21946f7abc9247cacf784220b1932ab5946c7540` and that the merge commit has exactly these parents:

```text
f0cc839cbe908cd7c4bb4623c668a01db3bbbb07
ff24d5c3084d54627d50004f1e527ba06683696b
```

Therefore the exact qualified implementation head, not an earlier or later branch revision, is the implementation parent of canonical `main`, and the exact shaping base is the other parent. No stale-head merge occurred.

GitHub's post-hoc PR/issue-event endpoints do not expose whether the historical merge request itself carried an `expected_head_sha` precondition. That transport-level request metadata is therefore **UNKNOWN and is not claimed as PASS evidence**. This closeout records the limitation explicitly instead of reconstructing or fabricating it.

Forward recovery is:

- preserve the exact-parent and pre-merge reconciliation evidence above;
- require expected-head protection on the closeout merge itself and all future merges where the connector supports it;
- keep unavailable historical request metadata distinct from the proven no-stale-head outcome.

## Post-merge verification

Canonical `main` was re-read at signed merge `21946f7abc9247cacf784220b1932ab5946c7540`.

Push-triggered GitHub Actions run `34047078983` executed on that exact merge and completed `SUCCESS`. This confirms canonical merged state retained the same multi-platform Rust/provenance checks, pinned SpecGrain validation, Diffcipline proof path, negative controls, and dependency-free workspace invariant.

## Residual non-authority

Specification 003 establishes logical contracts only. It does not authorize or claim completion of:

- durable event storage or a wire format;
- cryptographic vault/key architecture;
- media chunk/journal storage;
- capture or background recording;
- speech/model execution;
- document/PDF/search/memory intelligence;
- sync, networking, plugins, agents, connectors, or external writes;
- donor/component adoption;
- release/public compatibility claims.

## Remaining closeout condition

Specification 003 becomes `CLOSED_CANONICAL` only after the closeout PR containing this evidence, final task/state reconciliation, and restored dependency-change review posture is exact-head qualified and merged with expected-head protection, followed by successful post-closeout CI on canonical `main`.
