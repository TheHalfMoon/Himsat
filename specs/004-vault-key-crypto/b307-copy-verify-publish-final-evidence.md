# B307 Copy-Verify-Publish Final Evidence

## Scope

B307 closes only the bounded copy-verify-publish migration coordinator and its encrypted SQLCipher qualification fixture. It does not implement full seven-phase VRK rotation, write quiescing, source retirement/deletion, platform protectors, freshness/backup behavior, Specification 005, release/FIPS/compliance posture, or independent security review.

The accepted implementation changes only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_migration.rs
crates/himsat-core/tests/b307_copy_verify_publish.rs
```

## Accepted implementation revision

```text
IMPLEMENTATION_PR = 76
BASE = 1132b94d2185efbdaadd5d60e1c98a3ce4ca08e4
HEAD = bcaf38abde83a55cb1caccd81141167911d05a92
TREE = 0b38c68b2455984242f4c0d01680e7f3e1291bc4
CHANGED_FILES = 3
```

The production coordinator exposes ordered boundaries:

```text
SOURCE_VERIFY -> COPY -> STAGED_VERIFY -> PUBLISH -> ANCHOR -> REOPEN -> REOPENED_VERIFY
```

The source adapter is available to the coordinator only through a shared reference, and the B307 API exposes no source-retirement or deletion capability. Later B503/B506 leaves own those behaviors.

## Preserved negative evidence

```text
HEAD = a7df04cf7e4fe6447fe452aecae65a9e91e125a9
CI = 34377675561 / run #195 / FAILURE / attempt 1
R3 = 34377675603 / run #172 / FAILURE / attempt 1
CAUSE = cargo fmt --all -- --check
DISPOSITION = NOT_PASS

HEAD = 8a5d3a0afafb9159e08da503fb7e9a04afd4a492
CI = 34378263024 / run #197 / FAILURE / attempt 1
R3 = 34378262986 / run #174 / SUCCESS / attempt 1
WINDOWS_JOB = 102556737162 / FAILURE
CAUSE = staged SQLCipher fixture opened the copied file read-only before sync_all on Windows
DISPOSITION = NOT_PASS
```

Neither failed head was rerun, rebased, rewritten, force-pushed, or retroactively upgraded. The accepted repair was forward-only and changed only the staged-file sync handle mode in the B307 SQLCipher fixture.

## Qualification behavior

The accepted fixture uses a genuine file-backed SQLCipher source containing a semantic marker and proves the source bytes do not expose that marker as plaintext. It then exercises the bounded coordinator and proves:

- the prior encrypted source reopens through the production SQLCipher path and passes integrity verification before copying;
- copy creates a separately named staged encrypted target and synchronizes it without mutating the source;
- staged verification reopens the staged target through the production SQLCipher path and verifies integrity before publication;
- publication moves only the already-verified staged target to the published path while the prior source remains intact;
- the fixture anchor binds the exact published ciphertext bytes;
- the exact published target reopens through the production path and passes integrity verification after anchoring;
- the final published bytes match the copied encrypted source bytes in the bounded fixture;
- the prior encrypted source remains byte-identical after successful migration;
- injected failure at each target boundary stops forward progress at that exact boundary; and
- after every injected target failure, the prior source remains byte-identical, production-reopenable, and integrity-valid.

The coordinator itself does not pretend that the fixture digest is an OS freshness anchor. B501 owns the actual protected freshness-anchor implementation. Likewise, B307 does not implement B503 full VRK rotation phase state or B506 retirement/deletion.

## Exact-head pre-merge qualification

```text
PREMERGE_CI = 34383104985 / run #198 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34383104987 / run #175 / SUCCESS / attempt 1 / pull_request
PREMERGE_WINDOWS_JOB = 102572549378 / SUCCESS
OWNER_RECONCILIATION_REVIEW = 5157850803 / NOT_Q009
INLINE_REVIEW_THREADS = 0
QODO = BILLING_BLOCKED / NOT_PASS / NOT_Q009
CODERABBIT = AUTO_SKIPPED / NOT_PASS / NOT_Q009
CUBIC = AUTOMATED_NEUTRAL_OUTPUT / NOT_Q009
```

Repository-owner reconciliation is governance evidence only and does not satisfy Q009.

## Guarded merge and canonical parentage

Durable pre-merge transport binding is PR #76 comment `5606187755`.

The actual observed merge invocation used:

```text
EXPECTED_HEAD_SHA = bcaf38abde83a55cb1caccd81141167911d05a92
MERGE_METHOD = merge
```

GitHub returned:

```text
MERGED = true
CANONICAL_MERGE = fdc1d17a8df49c007f5038336ce267ea5eaf28a8
```

Canonical parentage and tree are exact:

```text
PARENT_1 = 1132b94d2185efbdaadd5d60e1c98a3ce4ca08e4
PARENT_2 = bcaf38abde83a55cb1caccd81141167911d05a92
MERGE_TREE = 0b38c68b2455984242f4c0d01680e7f3e1291bc4
```

## Post-merge qualification

```text
POSTMERGE_CI = 34384441066 / run #199 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34384441018 / run #176 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 102577069978 / SUCCESS
POSTMERGE_QUALIFICATION_COMMENT = 5606324660
```

The canonical merge is therefore exact-post-merge qualified for B307 only.

## Preserved limitations

`P011` remains unchecked / NOT PASS. `B305R002` remains unchecked / NOT PASS. Neither historical transport gap is repaired by B307.

Q009 remains unsatisfied. Repository-owner review, implementation tests, CI/R3 automation, Qodo billing-blocked output, CodeRabbit auto-skip, Cubic output, or other automation do not substitute for a genuinely independent substantive crypto/security review of the exact implementation revision.

## B401 rebound

B401 may begin only after the B307/B401 evidence-state reconciliation itself becomes exact-head qualified, reconciled against live `main`/PR head/base/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

After that qualification, B401 is the only authorized implementation leaf. B401 remains bounded to the Apple Keychain adapter with exact scope/presence/platform evidence. B402-B406, B501-B506, Q009, Specification 005, release/FIPS/compliance claims, and unrelated donor adoption remain out of scope. Any native dependency introduced by B401 requires exact provenance/license/dependency-closure evidence before canonical adoption.
