# B403A Post-Merge Qualification Recovery Evidence

## Incident disposition

B403A dependency adoption PR #85 was correctly rebuilt from the canonical trusted-gate successor, passed exact-head pull-request CI/R3, and was merged through an observed expected-head-guarded merge. Its first push-triggered post-merge CI and R3 attempts then failed only in the Diffcipline bounded-adoption bridge.

```text
B403A_PR = 85
B403A_BASE = 5861f65966b76a8a132a4a5a8591a2ff96c74843
B403A_HEAD = 7a947c3ee3a0517a5d3ae2f01b7efb74eece6e66
B403A_TREE = e1e5c57a80cb02be800d8ba5d2774291e57dae8c
PREMERGE_CI = 34514803021 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34514803033 / SUCCESS / attempt 1 / pull_request
EXPECTED_HEAD_SHA = 7a947c3ee3a0517a5d3ae2f01b7efb74eece6e66
MERGE_METHOD = merge
CANONICAL_MERGE = 37e0ef305ef379aed7b588efd6878608569766ec
PARENT_1 = 5861f65966b76a8a132a4a5a8591a2ff96c74843
PARENT_2 = 7a947c3ee3a0517a5d3ae2f01b7efb74eece6e66
MERGE_TREE = e1e5c57a80cb02be800d8ba5d2774291e57dae8c
```

The canonical merge therefore has the intended exact candidate tree and exact accepted head as parent 2. This recovery does not question or replace that transport evidence.
## Original post-merge result

The required first post-merge attempts are permanently recorded as NOT PASS:

```text
POSTMERGE_CI = 34516031395 / FAILURE / attempt 1 / push
POSTMERGE_R2_JOB = 103001551492 / FAILURE
POSTMERGE_R3 = 34516031278 / FAILURE / attempt 1 / push
POSTMERGE_R3_JOB = 103001551410 / FAILURE
POSTMERGE_WINDOWS_JOB = 103001551930 / SUCCESS
B403A_DISPOSITION = CANONICAL_MERGED_NOT_QUALIFIED
B403B_AUTHORITY = HELD
Q009 = UNSATISFIED
```

All non-Diffcipline jobs in post-merge CI completed successfully: Windows/macOS/Ubuntu Rust, Windows/macOS/Ubuntu provenance, provenance adversarial self-test, SpecGrain, and negative controls. The failed R2/R3 jobs still control the qualification result.

No failed job is rerun to manufacture a passing original attempt. The failed runs remain immutable incident evidence and are not retroactively upgraded.
## Root cause

The B403 trusted adoption gate executes from the immutable comparison base, but its `b403_trusted_base(base)` predicate also required:

```text
base == refs/remotes/origin/main
```

That condition is true during the pull-request run because `origin/main` is the trusted-gate base. It is necessarily false after the guarded merge: the push workflow correctly sets `DIFFCIPLINE_BASE = github.event.before`, while the full checkout fetch advances `origin/main` to the new merge commit.

For PR #85 the post-merge proof therefore compared the exact seven adopted artifacts against `5861f65966b76a8a132a4a5a8591a2ff96c74843`, all configured verification commands passed, and Diffcipline produced only the expected oversized/dependency/lockfile reasons. The bridge then rejected that proof solely because current `origin/main` was already `37e0ef305ef379aed7b588efd6878608569766ec`.

This is a post-merge trust-transport defect in the one-time adoption bridge. It is not evidence of dependency-byte drift, provenance failure, generated-output drift, platform compilation failure, or test failure.
## Forward-only recovery rule

This recovery does not amend PR #85, alter its merge tree, change the trusted gate, rerun the failed workflows, or reinterpret their result. It creates only new forward evidence from canonical `37e0ef305ef379aed7b588efd6878608569766ec`.

The recovery may release B403A qualification only after this exact recovery revision independently proves all of:

1. first-attempt pull-request CI and R3 terminal SUCCESS for the exact recovery revision;
2. live reconciliation of canonical `main`, exact base/head/diff, reviews, threads, comments, and mergeability;
3. an observed merge invocation using the exact recovery head as `expected_head_sha` with `merge_method = merge`;
4. exact canonical parentage and exact recovery-tree equality; and
5. first-attempt push-triggered post-merge CI and R3 terminal SUCCESS for the exact recovery merge.

If and only if all five conditions become true, B403A may be treated as `CANONICAL_QUALIFIED_BY_FORWARD_RECOVERY`. The original PR #85 post-merge failures remain NOT PASS evidence. No additional state mutation is required to release the already-authorized bounded successor: authority resolves to `B403B_WINDOWS_DPAPI_RUNTIME_ONLY` by this recovery condition.

## Recovery scope

This recovery changes only this evidence document. It changes no runtime/security code, Cargo manifest or lockfile, dependency bytes, provenance registry, generated artifact, workflow, adoption gate, donor material, `tasks.md`, `specs/CURRENT.md`, B403B implementation byte, later platform protector, freshness/backup/rotation/deletion behavior, Specification 005 behavior, or release/FIPS/compliance posture.
## Successor boundary

After recovery qualification, B403B remains bounded to the Windows current-user DPAPI/CNG-class protector. The adapter may report only `SAME_USER_ACCOUNT` absent stronger separately reviewed Windows evidence. `APP_EXCLUSIVE` and `REQUIRED_EACH_HIMSAT_UNLOCK` requests must fail closed with `UnsupportedPolicy`.

Protector ciphertext must remain in Himsat-owned ACL-restricted storage. B403B must produce genuine Windows-native evidence for the implemented boundary and must not infer native behavior from dependency inspection or cross-platform compilation.

B404-B406, B501-B506, Specification 005, release authority, `P011`, `B305R002`, and Q009 remain unchanged. Q009 still requires genuinely independent substantive crypto/security review of the exact implementation revision; this recovery, repository-owner reconciliation, CI/R3, or review automation cannot satisfy it.
