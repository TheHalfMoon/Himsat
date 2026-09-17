# 005C Crash-Recovery Reconciliation Final Evidence

## Scope

This document records the Specification 005 third implementation leaf
(005C): read-only crash-recovery reconciliation of session journals
against envelope files and the authenticated manifest inventory. It
implements journal scan, orphan/duplicate classification with canonical
resolution, manifest rebind proposals, close-count cross-check, and torn /
vault-mismatch reporting. It performs no manifest rewrite, no file
repair, and no deletion; callers apply proposed bindings through the
authenticated manifest writer they already own. It changes no reviewed
004/005A/005B byte, adopts no donor code, and adds no dependency,
provenance record, SBOM entry, or workflow change (`sha2` was already in
the crate closure).

Shipped as grain 1 (scan) plus grain 2a (reconcile core), grain 2b
(anomaly read API + adversarial fixtures), and grain 2c (close-count
cross-check), each under the R3 900-line size gate. Grain 2c
additionally closes 005B residual risk 2 (close-marker count
cross-check against the replayed file count; manifest-side binding is
covered by the verified/unbound lists).

## Authority

005B is canonically closed (implementation merges `477eecc` / `889cce2` /
`59ebf17`, each post-merge CI/R3 SUCCESS; reconciliation merge `37ac0d9`
post-merge CI `35205561976` / R3 `35205561880` SUCCESS). The 005 plan
re-bounds one leaf at a time; this leaf re-bounds prospective candidate
005C against live main as the third implementation Grain. 005D
(bounded-loss quantification and fault-injection evidence) remains a
shaping-level candidate with no implementation authority.

## Grain 1 — recovery scan (PR #167)

```text
BASE = 37ac0d90ca81560cb60d91f0ba13527ab396277b
HEAD = 37dfa451941b1593834388a93a07a1e3c62f9d03 (516 added lines)
CONTENT = vault_media_recovery.rs scan: scan_journal_dir (magic filter,
  symlink/subdirectory/foreign discipline, 8 MiB cap, name-sorted),
  inventory_envelopes (005A digest recipe), ScannedJournal /
  JournalScan / EnvelopeFile / EnvelopeInventory, 10 file-based tests
PREMERGE_CI = 35212086607_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35212086636_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = a0e779b59966a1697c5f80390b35897882a9e421
MERGE_PARENT_1 = 37ac0d90ca81560cb60d91f0ba13527ab396277b
MERGE_PARENT_2 = 37dfa451941b1593834388a93a07a1e3c62f9d03
MERGE_TREE = 4b9ec193793825160de84adcb98f311677ec51ff_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35213728380_SUCCESS_PUSH_ATTEMPT_1 (head a0e779b)
POSTMERGE_R3 = 35213728400_SUCCESS_PUSH_ATTEMPT_1 (head a0e779b)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain 2a — reconcile core (PR #168)

```text
BASE = a0e779b59966a1697c5f80390b35897882a9e421
HEAD = 601d3e98ed61b54d4f268c6582e960fb500cf678 (873 added lines)
CONTENT = CommitView accessors (journal side), derived media logical ids
  (HJM-0001 tag), reconcile_recovery with verified/orphan/duplicate
  (first-by-(file, seq) canonical)/unbound-plus-proposal/diverged/
  unlogged/unreferenced/torn/vault-mismatch classification, is_clean,
  clean + manifest-gap proof tests
PREMERGE_CI = 35214920052_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35214920044_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 44d76598584258c29b732d523aec4428d001a2e0
MERGE_PARENT_1 = a0e779b59966a1697c5f80390b35897882a9e421
MERGE_PARENT_2 = 601d3e98ed61b54d4f268c6582e960fb500cf678
MERGE_TREE = e39a13d9bb3665d889c8a1830a28a6024c12664b_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35216695150_SUCCESS_PUSH_ATTEMPT_1 (head 44d7659)
POSTMERGE_R3 = 35216695173_SUCCESS_PUSH_ATTEMPT_1 (head 44d7659)
SELF_ADVERSARIAL_REVIEW = 3 findings recorded in PR body (anomaly field
  API phased to 2b; tag-prefix foreign-blob theory documented; detail
  composition through journal API by design)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain 2b — anomaly read API and adversarial fixtures (PR #169)

```text
BASE = 44d76598584258c29b732d523aec4428d001a2e0
HEAD = eca3932cd055081ffe34e11b409ce055eba83f7f (346 added lines)
CONTENT = field accessors for all six anomaly structs; fixture tests for
  orphan, duplicate, diverged expected-vs-found, unlogged, unreferenced,
  torn-prefix reconciliation, vault-mismatch skip, empty clean,
  determinism
PREMERGE_CI = 35218315604_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35218315544_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 08578a87fe0303ee68f713ffc540d4819f9a0e0d
MERGE_PARENT_1 = 44d76598584258c29b732d523aec4428d001a2e0
MERGE_PARENT_2 = eca3932cd055081ffe34e11b409ce055eba83f7f
MERGE_TREE = 30cc36586c8c9cdf508ed3872adbdc35a3111b7d_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35220416676_SUCCESS_PUSH_ATTEMPT_1 (head 08578a8)
POSTMERGE_R3 = 35220416779_SUCCESS_PUSH_IDENTICAL_HEAD_RERUN (head 08578a8)
SELF_ADVERSARIAL_REVIEW = 3 findings recorded in PR body (field API
  completeness; torn valid-record determinism; mismatch semantics pinned)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

R3 note: the first post-merge R3 attempt failed on pre-existing 004
test `staged_byte_identity_mismatch` (sqlcipher backup area, untouched;
1-file diff; all 274 other tests passed; PR-merge CI green on all three
OS jobs). Same-head evidence (full local suite green, PR CI green)
proved no regression; the failed job re-ran green on the identical
head. Recorded as a runner flake; no code changed.

## Grain 2c — close-count cross-check (PR #170)

```text
BASE = 08578a87fe0303ee68f713ffc540d4819f9a0e0d
HEAD = 8939e17ee4d03b7dd39e9fab94060b34745c039f (169 added, 5 removed)
CONTENT = CloseCountMismatch report (file, claimed, actual) with field
  accessors, CleanEof-only cross-check inside reconcile_recovery,
  matching-close and forged-close tests, fixture writer-setup refactor
PREMERGE_CI = 35222959729_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35222959937_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 7e1c0d4d0bae193e378d41560a44c0ecc1bea6c1
MERGE_PARENT_1 = 08578a87fe0303ee68f713ffc540d4819f9a0e0d
MERGE_PARENT_2 = 8939e17ee4d03b7dd39e9fab94060b34745c039f
MERGE_TREE = 46e2ee04863b2a0ecaa20b3f896ca80013a63f58_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35225143152_SUCCESS_PUSH_ATTEMPT_1 (head 7e1c0d4)
POSTMERGE_R3 = 35225143131_SUCCESS_PUSH_ATTEMPT_1 (head 7e1c0d4)
SELF_ADVERSARIAL_REVIEW = writer-emitted close counts are debug-asserted
  correct, so the mismatch fixture forges the payload and reseals the
  per-record integrity trailer; development caught a whole-file vs
  per-record digest scoping mistake through the failing test
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Acceptance proof (final tree at 7e1c0d4)

```text
FMT = cargo fmt --check clean (local exact-head runs per grain)
CLIPPY = cargo clippy --workspace --all-targets --all-features -- -D warnings clean (per grain)
TESTS = cargo test --workspace --all-targets --locked green on all
  three CI operating systems per grain plus local exact-head runs:
  23 vault_media_recovery tests (10 scan + 13 reconcile) plus all
  integration suites, 0 failures
COVERAGE = clean recovery binds every commit; manifest gap proposes the
  exact binding; orphan; duplicate first-by-(file, seq) canonical;
  diverged expected-vs-found; unlogged; unreferenced; torn prefix still
  reconciles; vault-mismatch skip; empty clean; determinism;
  matching/disagreeing close markers; scan sorting/caps/magic
  discipline from grain 1
```

Local reconcile suites execute on all three CI operating systems as lib
tests (ubuntu/macos/windows Rust jobs all green per grain).

## Honesty bounds (not claims)

- Reconciliation is classification plus binding proposals, not repair:
  no manifest rewrite, no file deletion, no byte recovery is performed
  or claimed. Manifest rebind executes only through the authenticated
  manifest writer owned by the caller.
- Duplicate resolution is canonical selection (first-by-(file, seq)),
  never silent merge: every occurrence stays reported.
- Association is by content hash (journal-quoted digest); filenames are
  never parsed and non-media manifest blobs are ignored by tag prefix.
- Bounded-loss quantification (un-fsynced tail window proof) and
  kill-style fault injection belong to 005D and are not claimed here.
- Uncapped `read_to_end` replay, single-writer races, and
  FULLFSYNC/dir-fsync limits carry forward as 005D inputs.

## Residual risks (carried, not blockers)

1. Pre-existing backup staging tests flaked again on CI runners
   (`staged_byte_identity_mismatch` on the 2b post-merge R3 attempt);
   passes everywhere else including the identical-head rerun. Watch
   item for 005D fault-injection work, not a 005C defect.
2. Tag-prefix recognition admits a theoretical foreign blob carrying the
   media tag; manifest authentication still gates all writes.

## Post-merge qualification

005C is post-merge qualified on its exact canonical merges (grain-1
`a0e779b`, grain-2a `44d7659`, grain-2b `08578a8`, grain-2c `7e1c0d4`,
each with CI and R3 SUCCESS on the merge SHA).
