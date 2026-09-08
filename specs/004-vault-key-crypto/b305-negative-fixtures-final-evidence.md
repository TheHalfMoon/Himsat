# B305 SQLCipher Negative Fixtures Final Evidence

## Disposition

Specification 004 B305 is implementation-canonical and exact-post-merge qualified for the bounded wrong-key, corruption, unsupported-provider, and unsupported-version fixture boundary only.

```text
B305_PR = 68
B305_BASE = 5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be
B305_FINAL_HEAD = 55f512820d82100c1f60f6bc7bb79450a9b6181f
B305_FINAL_TREE = 2c7d73ca294bb6fe7970e258f0ed99c628a5ef8a
B305_CANONICAL_MERGE = e27a0b2921af2661037b7af581f6be737c1352de
```

B305 began only after the B304/B305 evidence-state reconciliation became exact-head qualified, expected-head guarded, parentage-proven, and exact-post-merge qualified.

## Canonical B304/B305 reconciliation evidence

```text
RECONCILIATION_PR = 67
BASE = bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7
HEAD = bfc9bea223975edf4a0f71812e38c58de6ed8443
PREMERGE_CI = 34285446663 / run #176 / SUCCESS / attempt 1
PREMERGE_R3 = 34285446650 / run #153 / SUCCESS / attempt 1
OWNER_RECONCILIATION_REVIEW = 5147723088 / COMMENTED / NOT_Q009
EXPECTED_HEAD_SHA = bfc9bea223975edf4a0f71812e38c58de6ed8443
EXPECTED_HEAD_TRANSPORT_COMMENT = 5592820085
CANONICAL_MERGE = 5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be
PARENT_1 = bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7
PARENT_2 = bfc9bea223975edf4a0f71812e38c58de6ed8443
MERGE_TREE = 9fa28951424b88d0c5bf3d22622ba6fcc1e11ebf
POSTMERGE_CI = 34286770543 / run #177 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34286770579 / run #154 / SUCCESS / attempt 1 / push
POSTMERGE_RECONCILIATION_COMMENT = 5592917396
POSTMERGE_QUALIFICATION_COMMENT = 5592919889
```

The prior task ledger left B304R001-B304R003 unchecked despite this proven evidence. This reconciliation repairs those entries forward-only.

## Implemented B305 boundary

B305 changed only:

```text
crates/himsat-core/src/vault_sqlcipher.rs
```

The accepted change adds four bounded adversarial fixtures and privately extracts the existing exact provider/version equality checks so the negative branches exercise the same production validators:

1. a genuine file-backed wrong-key fixture;
2. a genuine persisted encrypted-page corruption fixture;
3. unsupported SQLCipher runtime and embedded SQLite version mismatch tests; and
4. unsupported crypto-provider name and OpenSSL provider-version mismatch tests.

The private validator extraction does not weaken the existing exact-equality policy or expose a configurable fallback. The raw `rusqlite::Connection` remains private and B105 lease-gating remains unchanged.

## Wrong-key fixture behavior

The fixture creates a real encrypted structured database with the reviewed key path, then attempts the production open path with distinct key material.

The reviewed provider may reject the wrong key during the open boundary when an encrypted-page read is triggered, or a keyed handle may exist until a later integrity operation. The fixture accepts neither path as healthy: an open error is fail-closed; if a handle is returned, `verify_integrity()` must fail.

The same persisted database is then reopened with the correct key and must pass integrity. This proves the rejected wrong-key attempt did not rewrite, downgrade, or replace the encrypted database with plaintext state.

## Corruption fixture behavior

The fixture creates a multi-page encrypted database, closes the keyed connection, mutates one persisted byte in the encrypted file, and then uses the production open/integrity surfaces.

The corrupted database must never produce a healthy result. Either the production open boundary fails closed or a returned handle must fail `verify_integrity()`.

This is genuine persisted-byte corruption evidence. It does not claim B306 semantic-marker scanning or B307 migration behavior.

## Unsupported provider/version behavior

The exact production equality validators are exercised against both the accepted identities and explicit mismatches:

```text
SQLCIPHER_RUNTIME = 4.14.0 community
SQLITE_RUNTIME = 3.51.3
CRYPTO_PROVIDER = openssl
CRYPTO_PROVIDER_VERSION = OpenSSL 3.6.3 9 Jun 2026
```

Mismatched SQLCipher runtime, embedded SQLite version, provider name, or OpenSSL provider version returns its existing typed fail-closed `SqlCipherOpenError`; no downgrade or alternate provider path is attempted.

## Local supporting verification

Before GitHub qualification, the exact repository-pinned local toolchain reported:

```text
rustc = 1.98.1
cargo = 1.98.1
rustfmt = 1.9.0-stable
OPENSSL_RUST_USE_NASM = 0
cargo fmt --all -- --check = PASS
cargo clippy --workspace --all-targets -- -D warnings = PASS
cargo test --workspace = PASS
himsat-core = 81 passed / 0 failed
b204_recovery_negative = 3 passed / 0 failed
b205_crypto_adversarial = 6 passed / 0 failed
himsat-events = 10 passed / 0 failed
004P P005/P006 dependency closure = PASS
```

This local evidence is supporting evidence only and is not substituted for GitHub CI/R3.

## Duplicate transport reconciliation

A concurrent duplicate B305 transport was detected before canonical merge:

```text
PRIMARY_PR = 68
PRIMARY_HEAD = 55f512820d82100c1f60f6bc7bb79450a9b6181f
DUPLICATE_PR = 69
DUPLICATE_HEAD = 07e0b524ec2f35a88f95683bd2e337482e503730
COMMON_PARENT = 5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be
PRIMARY_TREE = 2c7d73ca294bb6fe7970e258f0ed99c628a5ef8a
DUPLICATE_TREE = 2c7d73ca294bb6fe7970e258f0ed99c628a5ef8a
TREE_DIFF = NONE
DUPLICATE_DISPOSITION = CLOSED_WITHOUT_MERGE_NOT_QUALIFICATION_EVIDENCE
DUPLICATE_RECONCILIATION_COMMENT = 5593017735
```

PR #68 was retained because it was the earlier transport and its exact-head CI/R3 were already running. PR #69 was closed without merge. No queued, cancelled, skipped, or other workflow result from PR #69 is reused as qualification evidence for B305.

## Exact-head qualification and live reconciliation

Final accepted implementation head:

```text
55f512820d82100c1f60f6bc7bb79450a9b6181f
```

Exact-head workflow evidence:

```text
PREMERGE_CI = 34288208755 / run #178 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34288208747 / run #155 / SUCCESS / attempt 1 / pull_request
```

Immediately before merge, canonical `main` remained `5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be`, PR #68 remained open/non-draft/mergeable at exact head `55f512820d82100c1f60f6bc7bb79450a9b6181f`, and the exact changed path remained only `crates/himsat-core/src/vault_sqlcipher.rs`.

No inline review thread existed. Qodo was billing-blocked and CodeRabbit auto-skipped; both remain NOT PASS.

Repository-owner exact-head reconciliation review:

```text
5147906837 / COMMENTED / NOT_Q009
```

That owner review is governance evidence only and does not satisfy Q009.

## Guarded merge and parentage

Durable transport comment `5593093765` bound the merge to:

```text
EXPECTED_HEAD_SHA = 55f512820d82100c1f60f6bc7bb79450a9b6181f
MERGE_METHOD = merge
```

GitHub accepted the guarded merge as:

```text
CANONICAL_MERGE = e27a0b2921af2661037b7af581f6be737c1352de
PARENT_1 = 5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be
PARENT_2 = 55f512820d82100c1f60f6bc7bb79450a9b6181f
MERGE_TREE = 2c7d73ca294bb6fe7970e258f0ed99c628a5ef8a
```

## Exact post-merge qualification

Exact push-triggered workflows on canonical merge `e27a0b2921af2661037b7af581f6be737c1352de` reached terminal SUCCESS on their original attempts:

```text
POSTMERGE_CI = 34289111683 / run #180 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34289111736 / run #157 / SUCCESS / attempt 1 / push
```

Durable post-merge qualification comment: `5593164672`.

B305 is therefore canonical and closed only for the bounded negative-fixture boundary above.

## Scope not claimed

B305 does not implement or claim:

- B306 plaintext-spill, semantic-marker, logical-ID, file-backed-temp, WAL/journal byte-scan, or public-filename qualification;
- B307 copy-verify-publish migration;
- B401-B406 native platform protector behavior;
- B501-B506 freshness, backup, rotation, or deletion;
- Q009 independent substantive crypto/security review;
- Specification 005 behavior;
- new dependency/provider/provenance adoption; or
- release, FIPS, or compliance qualification.

`P011` remains historical unchecked NOT PASS. Q009 remains unsatisfied.

## B306 rebound

B306 may begin only after this B305/B306 evidence-state reconciliation becomes exact-head qualified, reconciled against live `main`/PR head/base/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

After that qualification, B306 is bounded to proving that semantic fixture markers and logical IDs are absent from encrypted DB/WAL/rollback-journal/file-backed-temp bytes and from public filenames under the already qualified SQLCipher configuration. It must preserve `temp_store = MEMORY`, use genuine file-backed WAL and rollback-journal evidence where applicable, avoid embedding the tested semantic marker/logical ID in the qualification filename itself, and must not absorb B307 migration or later platform/freshness/qualification work.
