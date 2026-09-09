# B306 Plaintext-Spill Final Evidence

## Scope

B306 qualifies only the already reviewed SQLCipher structured-store path against plaintext spill of genuine semantic fixture markers and logical IDs. It does not implement B307 migration, platform protectors, freshness/backup/rotation/deletion, Specification 005 behavior, release/FIPS/compliance posture, or independent review.

The accepted implementation changes only:

```text
crates/himsat-core/tests/b306_plaintext_spill.rs
```

The qualification fixture uses exact sensitive values inside SQLCipher data while the public fixture path contains neither value. It proves the values round-trip logically before direct byte inspection.

## Accepted implementation revision

```text
IMPLEMENTATION_PR = 74
BASE = 52f7d8bba3ca07aeff20b4b457db6985c13492c0
HEAD = 35bd5ace1f72fed39ff34236c17ae46a0e73f705
TREE = 6f83aa4d19bc4cc39dc0a1d7c41bac5a0e3e6860
CHANGED_FILES = 1
CHANGED_PATH = crates/himsat-core/tests/b306_plaintext_spill.rs
```

Two predecessor heads remain NOT PASS and are not reused as qualification evidence:

```text
3c9550cba175f8f63aba265b2cac7e7bd429e1a7
  CI = 34365909898 / run #189 / FAILURE / attempt 1
  R3 = 34365909795 / run #166 / FAILURE / attempt 1
  CAUSE = cargo fmt --all -- --check

7872315dff90c7f3d4112a171a989289d7dc3fb0
  CI = 34366447120 / run #190 / FAILURE / attempt 1
  R3 = 34366447403 / run #167 / FAILURE / attempt 1
  CAUSE = final EOF newline formatting drift
```

No failed run was rerun-to-green or retroactively upgraded. Repairs were forward-only.

## Qualification behavior

The accepted exact-head test proves all of the following under the already reviewed SQLCipher provider/runtime and `StructuredStore` derivation posture:

- exact SQLCipher, embedded SQLite, OpenSSL provider, encryption-active, and temp-store compile identities remain enforced;
- `temp_store = MEMORY` remains positively observed;
- a genuine semantic marker and logical ID are inserted and round-trip through SQLCipher before byte inspection;
- a genuine non-empty WAL exists while live and contains neither sensitive plaintext value;
- after WAL checkpoint, the encrypted main database contains neither sensitive plaintext value;
- a genuine non-empty DELETE rollback journal exists during an open write transaction and contains neither sensitive plaintext value;
- optional shared-memory sidecar bytes, when present, contain neither sensitive plaintext value;
- sensitive TEMP-table and sort work preserves `temp_store = MEMORY`, `temp.journal_mode = memory`, and an exact empty `temp` backing filename from `PRAGMA database_list`;
- database and sidecar public filenames contain neither tested sensitive value; and
- the exact persisted fixture reopens through production `open_sqlcipher_database` and passes `verify_integrity()`.

Unavailable evidence is not treated as PASS. File-backed WAL and rollback-journal surfaces are inspected only after their genuine presence and non-empty content are proven. The temp-store claim is positive configuration/runtime evidence rather than an inference from a missing file.

## Exact-head pre-merge qualification

```text
PREMERGE_CI = 34366925805 / run #191 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34366925788 / run #168 / SUCCESS / attempt 1 / pull_request
OWNER_RECONCILIATION_REVIEW = 5156261043 / NOT_Q009
INLINE_REVIEW_THREADS = 0
QODO = BILLING_BLOCKED / NOT_PASS / NOT_Q009
CODERABBIT = AUTO_SKIPPED / NOT_PASS / NOT_Q009
```

Repository-owner reconciliation is governance evidence only and does not satisfy Q009.

## Guarded merge and canonical parentage

Durable pre-merge transport binding is PR #74 comment `5604219116`.

The actual observed merge invocation used:

```text
EXPECTED_HEAD_SHA = 35bd5ace1f72fed39ff34236c17ae46a0e73f705
MERGE_METHOD = merge
```

GitHub returned:

```text
MERGED = true
CANONICAL_MERGE = 7359ca64f6679409ddc81a025fe556c7297845e2
```

Canonical parentage and tree are exact:

```text
PARENT_1 = 52f7d8bba3ca07aeff20b4b457db6985c13492c0
PARENT_2 = 35bd5ace1f72fed39ff34236c17ae46a0e73f705
MERGE_TREE = 6f83aa4d19bc4cc39dc0a1d7c41bac5a0e3e6860
```

## Post-merge qualification

```text
POSTMERGE_CI = 34369011280 / run #192 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34369011241 / run #169 / SUCCESS / attempt 1 / push
POSTMERGE_QUALIFICATION_COMMENT = 5604388541
```

The canonical merge is therefore exact-post-merge qualified for B306 only.

## Preserved limitations

`P011` remains unchecked / NOT PASS. `B305R002` remains unchecked / NOT PASS. Neither historical transport gap is repaired by B306.

Q009 remains unsatisfied. Repository-owner review, implementation tests, CI/R3 automation, Qodo billing-blocked output, CodeRabbit auto-skip, Cubic output, or other automation do not substitute for a genuinely independent substantive crypto/security review of the exact implementation revision.

## B307 rebound

B307 may begin only after this B306/B307 evidence-state reconciliation itself becomes exact-head qualified, reconciled against live `main`/PR head/base/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

After that qualification, B307 is the only authorized implementation leaf. B307 remains bounded to copy-verify-publish migration: the verified prior encrypted state must remain recoverable until the new state is complete, authenticated, published, anchored, reopened, and integrity-verified. B307 must not absorb B401-B406 platform protectors, B501-B506 freshness/backup/full seven-phase VRK rotation/deletion behavior, Q009, Specification 005, new donor/dependency adoption, or release/FIPS/compliance claims.