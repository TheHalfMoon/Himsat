# 005D Fault-Injection Final Evidence

## Scope

This document records the Specification 005 fourth implementation leaf
(005D): kill-style fault injection and bounded-loss quantification for
the session journal. It implements a real-OS-kill harness, a
kill-at-every-commit-point matrix, and a filename-opacity proof. It
changes no reviewed 004/005A/005B/005C byte, adopts no donor code, and
adds no dependency, provenance record, SBOM entry, or workflow change
(standard library process and filesystem APIs only).

Shipped as grain D1 (kill harness, 212 added lines) plus grain D2 (kill
matrix and filename-opacity proof, 73 added / 3 removed), each under
the R3 900-line size gate.

## Authority

005C is canonically closed (implementation merges `a0e779b` /
`44d7659` / `08578a8` / `7e1c0d4`, each post-merge CI/R3 SUCCESS;
reconciliation merge `e5023f3`, post-merge CI `35231902760` / R3
`35231902791` SUCCESS). The 005 plan re-bounds one leaf at a time; this
leaf re-bounds prospective candidate 005D against live main as the
fourth and final implementation Grain. No 005 leaf remains a candidate
after this leaf.

## Grain D1 — kill harness (PR #172)

```text
BASE = e5023f3052ec8b0b83d8253078d96bbb99296055
HEAD = b18876c78ecf5a0fcd4466227297f0a231e4ba23 (212 added lines)
CONTENT = tests/b005d_journal_kill.rs: crash-writer child entry
  (ignored unless re-spawned with the kill protocol), synced progress
  file, realistic seal-then-commit pipeline order, timing-independent
  recovery bounds (at most one torn tail, zero duplicates, 3..8
  classified commits, stray bytes limited to own sealed envelopes)
PREMERGE_CI = 35235023561_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35235023599_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 88d171c9506253aeb6f6b31765f6f701913d242d
MERGE_PARENT_1 = e5023f3052ec8b0b83d8253078d96bbb99296055
MERGE_PARENT_2 = b18876c78ecf5a0fcd4466227297f0a231e4ba23
MERGE_TREE = 4402f6364a8c2aee612c273cb3212e145c823331_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35237042494_SUCCESS_PUSH_ATTEMPT_1 (head 88d171c)
POSTMERGE_R3 = 35237042688_SUCCESS_PUSH_ATTEMPT_1 (head 88d171c)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain D2 — kill matrix and filename-opacity proof (PR #173)

```text
BASE = 88d171c9506253aeb6f6b31765f6f701913d242d
HEAD = 5915bbc1ce95e71bd9af129036cee2bd0173d710 (73 added, 3 removed)
CONTENT = kill-at-every-commit-point matrix (0..8, [k, 8] classified
  bounds per point), misleading-names content-only proof
  (magic-discovered journal, digest-matched envelope), kill-point
  parameter on the progress waiter
PREMERGE_CI = 35239248735_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35239248979_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 8f989f5960c219731bc29a27741591c22472de8c
MERGE_PARENT_1 = 88d171c9506253aeb6f6b31765f6f701913d242d
MERGE_PARENT_2 = 5915bbc1ce95e71bd9af129036cee2bd0173d710
MERGE_TREE = 76fa16d437f1bb6553d85c8a75be029a7d348f1a_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35240809084_SUCCESS_PUSH_ATTEMPT_1 (head 8f989f5)
POSTMERGE_R3 = 35240809088_SUCCESS_PUSH_ATTEMPT_1 (head 8f989f5)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Bounded-loss quantification

Per-record `sync_all` bounds crash loss to the single in-flight tail
record: every earlier record is durable before the next append begins,
so a kill can tear at most the final record and every complete record
must classify. The matrix test executes this bound at all nine kill
points (0..8): progress proving `k` commits durable forces at least
`k` complete records to classify and at most all eight ran, with zero
duplicates and at most one torn tail, at every point. Loss beyond the
tail window would require a synced record to vanish, which the
filesystem durability contract forbids; no such loss was observed in
any kill run (local repeats plus all three CI operating systems per
grain).

## Platform evidence

The kill harness and matrix execute as ordinary lib/integration tests
on all three CI operating systems (ubuntu/macos/windows Rust jobs all
green per grain), which is the per-OS durability proof claimed: no
universal durability claim is made beyond a successful `sync_all`
return on each platform, and no `F_FULLFSYNC`, directory-entry, or
power-loss-proof claim is made at all.

## Adversarial coverage map (005D vs plan)

- Process-kill at every journal commit point: D1 harness + D2 matrix.
- Torn/truncated/corrupt tails: 005B exhaustive matrices (reused, not
  re-proven).
- Orphaned/duplicated chunks: 005C fixtures (reused, not re-proven).
- Wrong-key and cross-session transplant chunks: 005A adversarial
  tests (reused, not re-proven).
- Oversized chunk rejection at the B202 ceiling: 005A ceiling tests
  (reused, not re-proven).
- Opaque filenames: D2 misleading-names proof (new in this leaf).
- Deletion: no 005 deletion path exists to test. Chunks and journals
  fall under the existing B506 surface families with unchanged
  detached-backup and physical-erasure limits; no new deletion
  semantics are claimed.

## Acceptance proof (final tree at 8f989f5)

```text
FMT = cargo fmt --check clean (local exact-head runs per grain)
CLIPPY = cargo clippy --workspace --all-targets --all-features -- -D warnings clean (per grain)
TESTS = cargo test --workspace --all-targets --locked green on all
  three CI operating systems per grain plus local exact-head runs:
  kill harness 5/5 local repeats, 8-point kill matrix, misleading-names
  proof, plus all pre-existing suites, 0 failures
```

## Honesty bounds (not claims)

- Kill coverage is process-death (SIGKILL/TerminateProcess), not power
  loss: OS crash-consistency below `sync_all` is out of scope by
  contract, matching the 005B durability bound.
- The matrix kills one writer with eight commits; long-session and
  multi-journal kill matrices are not run.
- Filename opacity is proven for discovery and association only;
  metadata-secrecy beyond names is unchanged from 004 rules.

## Residual risks (carried, not blockers)

1. Pre-existing backup staging tests remain flaky on CI runners across
   the 005 lineage; all 005D runs were green without reruns. Watch
   item for 006+, not a 005 defect.
2. No product chunk-store writer/deleter exists yet: envelope sealing
   order (seal-then-commit) is proven only inside the test child until
   006+ capture consumes the journal.

## Post-merge qualification

005D is post-merge qualified on its exact canonical merges (grain-D1
`88d171c`, grain-D2 `8f989f5`, each with CI and R3 SUCCESS on the
merge SHA).

## Spec 005 closeout

Specification 005 satisfies its closeout rule: reviewed shaping
(`2d7ec3e`, qualified), four bounded implementation leaves (005A
envelope, 005B journal, 005C reconcile, 005D fault injection), R3
adversarial/platform evidence per leaf, expected-head merges with
merge-tree equality throughout, post-merge CI/R3 SUCCESS on every
merge SHA, reconciliations for 005B/005C/005D, and this durable
closeout record. Specification 005 is CLOSED_CANONICAL. Only now may
Specification 006 shaping begin.
