# B304 SQLCipher Integrity Final Evidence

## Disposition

Specification 004 B304 is implementation-canonical and exact-post-merge qualified for the lease-gated normal SQLite integrity and SQLCipher cipher/page-authentication integrity boundary only.

```text
B304_PR = 66
B304_BASE = 76f6a3ccacc56d58daf4bd9f40d21234a6e50002
B304_FINAL_HEAD = 751a02c13129face0fb5ca828ccf846a483e4941
B304_CANONICAL_MERGE = bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7
```

The canonical base is the exact B303/B304 reconciliation merge. That reconciliation was exact-head qualified, expected-head guarded, parentage-proven, and exact-post-merge qualified before B304 implementation began.

## Implemented boundary

B304 changed only:

```text
crates/himsat-core/src/vault_sqlcipher.rs
```

The accepted implementation adds a typed fail-closed integrity operation to the existing private SQLCipher handle:

```text
SQLCIPHER_INTEGRITY_SUCCESS = ZERO_ROWS
SQLITE_INTEGRITY_SUCCESS = EXACTLY_ONE_ROW_WITH_TEXT_ok
```

`SqlCipherDatabaseHandle::verify_integrity()` holds one B105 operation-scoped read permit across the complete SQLCipher and SQLite integrity sequence. The raw `rusqlite::Connection` remains private and is not exposed through a getter or callback surface.

`PRAGMA cipher_integrity_check` is accepted only when it produces no diagnostic rows. Any diagnostic row, prepare/query/step failure, or inability to prove the reviewed result shape fails closed with a typed `SqlCipherIntegrityError`.

`PRAGMA integrity_check` is accepted only when it produces exactly one text row equal to exact `ok` and no additional rows. Zero rows, non-text output, non-`ok` text, additional rows, or query/step failure fails closed.

## Qualification tests

The B304 test surface proves:

- a pristine file-backed encrypted database passes the exact SQLCipher and SQLite integrity result shapes;
- the pinned in-memory SQLCipher `cipher_integrity_check` diagnostic shape fails closed without being represented as B305 corruption evidence;
- the normal SQLite integrity result on the bounded helper path is exact; and
- a revoked B105 lease rejects `verify_integrity()` before backend access.

The accepted tests are:

```text
b304_file_backed_database_passes_exact_integrity_shapes
b304_in_memory_cipher_integrity_diagnostic_fails_closed
b304_revoked_handle_rejects_integrity_before_backend_query
```

## Preserved negative lineage

The initial B304 implementation head is permanently NOT PASS:

```text
HEAD = 4841362fac8ea62c4de01ce18ef1465557881a9c
CI = 34280979977 / run #173 / FAILURE_NOT_PASS / attempt 1
R3 = 34280979912 / run #150 / FAILURE_NOT_PASS / attempt 1
OBSERVED = UBUNTU_MACOS_WINDOWS_FORMATTING_FAILURE_NOT_PASS
DISPOSITION = SUPERSEDED_NOT_QUALIFIED
```

Durable negative-evidence comments: `5592164751` and `5592187298`.

The R3 run for that failed head did execute clippy and the full test set successfully, including the B304 tests, but its final verdict remained FAILURE because `cargo fmt --all -- --check` failed. That partial positive evidence does not upgrade the head. The failed CI/R3 runs were not rerun, rewritten, force-pushed, rebased, or retroactively reclassified.

The exact pinned-rustfmt output identified two formatting deltas. The accepted successor applied those deltas forward-only.

## Exact-head qualification

Final accepted implementation head:

```text
751a02c13129face0fb5ca828ccf846a483e4941
```

Exact-head workflow evidence:

```text
CI = 34281346220 / run #174 / SUCCESS / attempt 1 / pull_request
R3 = 34281346213 / run #151 / SUCCESS / attempt 1 / pull_request
```

The final head passed formatting, clippy, tests, registered dependency closure, provenance checks, SpecGrain, negative controls, Diffcipline R2, and Diffcipline R3 on the governed matrix.

## Review and live reconciliation state

Immediately before merge, PR #66 was open, non-draft, mergeable, based on exact canonical `76f6a3ccacc56d58daf4bd9f40d21234a6e50002`, and headed by exact `751a02c13129face0fb5ca828ccf846a483e4941`. The exact diff changed only `crates/himsat-core/src/vault_sqlcipher.rs`.

No inline review thread existed. Before the repository-owner reconciliation review, no submitted review existed. Qodo was billing-blocked and CodeRabbit auto-skipped because of repository eligibility; both outputs are NOT PASS.

Durable repository-owner exact-head reconciliation review: `5147456689`.

That repository-owner review is governance reconciliation only. Repository-owner evidence, implementation tests, CI, R3 automation, Qodo billing-blocked output, CodeRabbit skipped output, Cubic neutral output, or other automation do not satisfy Q009. Q009 remains unsatisfied and requires a genuinely independent substantive crypto/security review of the exact implementation revision at the later qualification gate.

## Guarded merge and parentage

The merge request used exact expected-head protection:

```text
EXPECTED_HEAD_SHA = 751a02c13129face0fb5ca828ccf846a483e4941
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7
```

Durable guarded-transport comment: `5592463767`.

Canonical parentage is exact:

```text
PARENT_1 = 76f6a3ccacc56d58daf4bd9f40d21234a6e50002
PARENT_2 = 751a02c13129face0fb5ca828ccf846a483e4941
MERGE_TREE = 5b17fae1ea71f90f3293f94733de76bc6f497851
```

Durable parentage comment: `5592503089`.

## Exact post-merge qualification

Exact push-triggered workflows on canonical merge `bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7` reached terminal SUCCESS on their original attempts:

```text
POSTMERGE_CI = 34283841909 / run #175 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34283842070 / run #152 / SUCCESS / attempt 1 / push
```

Durable post-merge qualification comment: `5592582109`.

B304 is therefore canonical and closed only for the normal SQLite integrity and SQLCipher cipher/page-authentication integrity boundary described above.

## B303/B304 reconciliation evidence repaired forward-only

The prior task ledger still shows B303R001-B303R003 unchecked even though their exact live evidence is proven. This reconciliation repairs only that ledger state; it does not rewrite history or upgrade unavailable evidence.

```text
B303_B304_RECONCILIATION_PR = 65
B303_B304_RECONCILIATION_BASE = b3d4c7de77ec7a072fdcdd09d2d798519236db34
B303_B304_RECONCILIATION_HEAD = 798fe493e99c9524b76dfaf065de4514ac511acf
PREMERGE_CI = 34278147166 / run #171 / SUCCESS / attempt 1
PREMERGE_R3 = 34278147198 / run #148 / SUCCESS / attempt 1
OWNER_RECONCILIATION_REVIEW = 5147061700
EXPECTED_HEAD_SHA = 798fe493e99c9524b76dfaf065de4514ac511acf
EXPECTED_HEAD_TRANSPORT_COMMENT = 5591956575
CANONICAL_MERGE = 76f6a3ccacc56d58daf4bd9f40d21234a6e50002
PARENT_1 = b3d4c7de77ec7a072fdcdd09d2d798519236db34
PARENT_2 = 798fe493e99c9524b76dfaf065de4514ac511acf
MERGE_TREE = ade65b8f172c559341cbe1177b9c9a20f975ff4e
POSTMERGE_CI = 34279419817 / run #172 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34279419704 / run #149 / SUCCESS / attempt 1 / push
POSTMERGE_RECONCILIATION_COMMENT = 5592110972
```

## Scope not claimed

B304 does not implement or claim:

- B305 wrong-key/corruption/unsupported-provider/version fixture expansion;
- B306 plaintext-spill, logical-ID, semantic-marker, file-backed-temp, or public-filename qualification;
- B307 copy-verify-publish migration;
- B401-B406 native platform protector adapters or platform qualification;
- B501-B506 freshness, backup, rotation, or deletion implementation;
- Q009 independent substantive crypto/security review;
- Specification 005 behavior;
- new dependency or donor adoption; or
- release, FIPS, or compliance qualification.

`P011` remains historical NOT PASS and remains unchecked.

## B305 rebound

B305 may begin only after this B304/B305 evidence-state reconciliation becomes exact-head qualified, reconciled against live `main`/PR head/base/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

After that qualification, B305 is bounded to wrong-key, corruption, unsupported-provider, and unsupported-version fixtures for the exact adopted SQLCipher provider path. B305 must preserve fail-closed behavior and must not absorb B306 plaintext-spill/logical-ID/semantic-marker/file-backed-temp/public-filename qualification, B307 migration, platform protector behavior, freshness/backup/rotation/deletion behavior, Specification 005 behavior, or Q009 satisfaction.