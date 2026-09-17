# 006 Capture Final Evidence

## Scope

This document records Specification 006 (capture abstraction and
capture health): shaping plus three implementation leaves (006A source
lifecycle and session machine, 006B health telemetry, 006C checkpoint
and metadata binding) with the B006A dependency-adoption gate. It adds
no dependency beyond the adopted workspace-events edge, adopts no donor
code, and changes no reviewed 003/004/005 byte.

## Authority

005 is CLOSED_CANONICAL (closeout merge `85b2cb2`, post-merge CI
`35244997886` / R3 `35244997858` SUCCESS). 006 shaping qualified
(shaping merge `61414cd`, post-merge CI `35249873459` / R3
`35249873475` SUCCESS; qualification recorded at merge `31b3e6c`).
This reconciliation re-binds the leaves against live main as
implementation Grains; no 006 leaf remains a candidate after it.

## Gate hardening — B006A exception (PR #178)

```text
BASE = 31b3e6cbe50a952f1f13743cfd30f0cff730e0d9
HEAD = 88aea36b9d408078fc3e88152b7f2174ed80009d (113 added lines)
CONTENT = B006A trusted-base predicate plus pinned six-file exception
  checker (REVIEW-only, exact manifest + lockfile reasons, exact
  blobs, ordinary 900-line bound), mirroring B403C/B501C
PREMERGE_CI = 35257228614_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35257228585_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = dc4f8147e11b013b8a6e3e88ff257824abbc2380
MERGE_PARENT_1 = 31b3e6cbe50a952f1f13743cfd30f0cff730e0d9
MERGE_PARENT_2 = 88aea36b9d408078fc3e88152b7f2174ed80009d
MERGE_TREE = 76c6db73b4c9dcb10276b93b0fabd82397fead2c_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35259150477_SUCCESS_PUSH_ATTEMPT_1 (head dc4f814)
POSTMERGE_R3 = 35259150538_SUCCESS_PUSH_ATTEMPT_1 (head dc4f814)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain 006A — session machine adoption (PR #179, supersedes #177)

PR #177 carried the identical content but red by design (base
predated the gate); it was closed superseded and reshipped
byte-identical as PR #179. No content was re-authored between them.

```text
BASE = dc4f8147e11b013b8a6e3e88ff257824abbc2380
HEAD = 6a53ed53504325d8ba46afe5a431833d463f6cd6 (766 added, 0 removed)
CONTENT = capture_session (descriptors, 11-state machine, 14 events,
  7 reasons, total transition, transplant refusal, 154-pair totality),
  lib wiring, workspace-events edge, 004P closure update, adoption
  evidence; zero external packages, zero transitive deps
PREMERGE_CI = 35261286883_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35261286917_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 0d0a03b3303825f0f12846bb8906e34e8b1a3b65
MERGE_PARENT_1 = dc4f8147e11b013b8a6e3e88ff257824abbc2380
MERGE_PARENT_2 = 6a53ed53504325d8ba46afe5a431833d463f6cd6
MERGE_TREE = 2781034ebd0f89dc68853c9a7274595832dd2bde_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35263138861_SUCCESS_PUSH_ATTEMPT_1 (head 0d0a03b)
POSTMERGE_R3 = 35263138840_SUCCESS_PUSH_ATTEMPT_1 (head 0d0a03b)
SELF_ADVERSARIAL_REVIEW = donor compare recorded (OpenSuperWhisper
  recorder state AppKit-coupled; native portable core chosen on
  merit); no adoption
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain 006B — health telemetry (PR #180)

```text
BASE = 0d0a03b3303825f0f12846bb8906e34e8b1a3b65
HEAD = 23742e09f7516fd9c3e144e73c6e4768c5c64af9 (532 added lines)
CONTENT = capture_health (clock-free samples, storage classifier,
  queue predicate, edge-triggered monitor with 003 sequences,
  snapshot digest, 8 tests). No new dependencies.
PREMERGE_CI = 35265775128_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35265775039_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 2251729ad66d9f99f217130ac62f85c6d8bccf3b
MERGE_PARENT_1 = 0d0a03b3303825f0f12846bb8906e34e8b1a3b65
MERGE_PARENT_2 = 23742e09f7516fd9c3e144e73c6e4768c5c64af9
MERGE_TREE = 14bc6320b7b1cc326e1f73440e0c3a94144130e7_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35267652412_SUCCESS_PUSH_ATTEMPT_1 (head 2251729)
POSTMERGE_R3 = 35267652411_SUCCESS_PUSH_ATTEMPT_1 (head 2251729)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain 006C — checkpoint and metadata (PR #181)

```text
BASE = 2251729ad66d9f99f217130ac62f85c6d8bccf3b
HEAD = 5918d4be4f9d2af40a9f50c2a03939bc90146fc6 (594 added lines)
CONTENT = capture_checkpoint (derive_checkpoints with LossAccount,
  latest_sealed_for, ChunkMetadata 005 mapping, 8 tests). No new
  dependencies.
PREMERGE_CI = 35269367774_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35269367826_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 82e53d2c0109cbfdab0aaaec9b4e1b8033699206
MERGE_PARENT_1 = 2251729ad66d9f99f217130ac62f85c6d8bccf3b
MERGE_PARENT_2 = 5918d4be4f9d2af40a9f50c2a03939bc90146fc6
MERGE_TREE = cd4502acbea795a38e9036a6d3e4b3eb9ebe26ec_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35271285085_SUCCESS_PUSH_ATTEMPT_1 (head 82e53d2)
POSTMERGE_R3 = 35271285074_SUCCESS_PUSH_ATTEMPT_1 (head 82e53d2)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Acceptance proof (final tree at 82e53d2)

```text
FMT = cargo fmt --check clean (local exact-head runs per grain)
CLIPPY = cargo clippy --workspace --all-targets --all-features -- -D warnings clean (per grain)
TESTS = cargo test --workspace --all-targets --locked green on all
  three CI operating systems per grain plus local exact-head runs:
  11 session tests (154-pair totality), 8 health tests, 8 checkpoint
  tests, plus all pre-existing suites, 0 failures
```

## Honesty bounds (not claims)

- The session machine classifies; platform truth still lives in 007+
  adapters, which bind native conditions to these events.
- Health thresholds are caller budgets; the core classifies and never
  invents them. Level smoothing and hysteresis stay adapter-side.
- Checkpoints derive from recovery outputs; the manifest writer that
  applies bindings stays caller-owned.
- No product chunk writer, deleter, UI, DSP, transcription, or sync
  path is implemented or claimed.

## Residual risks (carried, not blockers)

1. Pre-existing backup staging tests remain flaky on CI runners
   across the lineage; all 006 runs were green without reruns.
2. Sequence coordination across multiple monitors is caller-owned
   (003 order); the monitor starts each instance at zero.

## Post-merge qualification

006 is post-merge qualified on its exact canonical merges (gate
`dc4f814`, adoption `0d0a03b`, 006B `2251729`, 006C `82e53d2`, each
with CI and R3 SUCCESS on the merge SHA).

## Spec 006 closeout

Specification 006 satisfies its closeout rule: reviewed shaping
(`61414cd`, qualified and recorded at `31b3e6c`), three bounded
implementation leaves with R3 adversarial/platform evidence per
leaf (including the sanctioned B006A adoption exception),
reconciliation, expected-head merges with merge-tree equality
throughout, post-merge CI/R3 SUCCESS on every merge SHA, and this
durable closeout record. Specification 006 is CLOSED_CANONICAL. Only
now may Specification 007 shaping begin.
