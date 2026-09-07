# Specification 004B1 B102 Final Evidence — Revocable Keyed-Handle Lease

## Disposition

```text
LEAF = B102
DISPOSITION = CANONICAL_QUALIFIED
CANONICAL_MERGE = 4b27ede7d9bf17caac163b7607c331056634fc95
POSTMERGE_CI = 34155726527_SUCCESS
POSTMERGE_R3 = 34155726505_SUCCESS
NEXT_LEAF = B103_SECRET_PROTECTOR_BEHAVIOR
```

This record closes only Specification 004B1 leaf B102. It does not approve B103-B105, cryptographic primitives, SQLCipher/database behavior, native platform protectors, freshness persistence, backup/restore/rotation/deletion, Specification 005 media behavior, donor adoption, or release authority.

## Authority baseline

B102 was not authorized merely because B101 code had merged. Historical transport-level `expected_head_sha` evidence for PR #37 and PR #38 was not reconstructible post hoc, so the repository repaired that gap forward-only through reconciliation PR #40.

The exact reconciliation head was:

```text
RECONCILIATION_PR = 40
RECONCILIATION_HEAD = d99ceb841d834324d06774753b661bdf108f354b
RECONCILIATION_PREMERGE_CI = 34152839411_SUCCESS
RECONCILIATION_PREMERGE_R3 = 34152839424_SUCCESS
RECONCILIATION_EXPECTED_HEAD_MERGE = PROVEN
RECONCILIATION_CANONICAL_MERGE = 0252bb31764c9178e270694f4087e8ac701271a0
RECONCILIATION_POSTMERGE_CI = 34153492791_SUCCESS
RECONCILIATION_POSTMERGE_R3 = 34153492769_SUCCESS
```

Canonical merge `0252bb31764c9178e270694f4087e8ac701271a0` has parents `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c` and `d99ceb841d834324d06774753b661bdf108f354b`. PR #40 was merged with explicit `expected_head_sha = d99ceb841d834324d06774753b661bdf108f354b`, and its durable PR comment records the successful transport result. B102 authority opened only after exact post-merge CI and R3 both reached terminal SUCCESS on that canonical reconciliation merge.

## Preserved pre-authority lineage

PR #39 is preserved as negative historical evidence rather than retroactively authorized work.

```text
PRE_AUTHORITY_PR = 39
BASE_MAIN = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
INITIAL_HEAD = 2147f9d1a9f0714f2b459f84f39ba43add1c62ef
INITIAL_CI = 34150565566_FAILURE_FORMAT_ONLY
FINAL_HEAD = cb392463dec68893891f9adc97dcb21ead4c5a22
STATE = CLOSED_UNMERGED
```

PR #39 was created before the forward-repair reconciliation had become canonical and post-merge qualified. Its implementation therefore grants no authority or qualification to the accepted B102 lineage. It also introduced a general `FreshnessError::Gap` surface not present in the canonical reviewed portable contract. That surface was deliberately not carried into the authorized lineage.

## Authorized B102 lineage

The accepted branch was created from exact canonical base `0252bb31764c9178e270694f4087e8ac701271a0` after the authority gate opened.

```text
PR = 41
BASE_MAIN = 0252bb31764c9178e270694f4087e8ac701271a0
FINAL_HEAD = 40b9705258a302a4e71340f4dd0a28746e86c8a7
COMMITS_AHEAD = 4
COMMITS_BEHIND = 0
CHANGED_FILES = 2
ADDITIONS = 499
DELETIONS = 4
```

Exact changed paths:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_lease.rs
```

No `Cargo.toml`, `Cargo.lock`, provenance registry, generated SBOM/notices, workflow, donor material, SQLCipher/native provider, or Specification 005 file changed.

## Implemented behavior

B102 adds only the reviewed portable in-process revocation behavior needed by keyed handles:

- a live `VaultLease` bound to one existing `VaultLeaseIdentity` (`VaultId` plus `KeyGeneration`);
- cloneable `KeyedHandleLease` references that can authorize operations but cannot revoke or reactivate a lease;
- operation-scoped `KeyedHandlePermit` values retaining a shared operation barrier while one authorized keyed operation is in flight;
- atomic terminal-state publication before the revocation path waits for previously authorized permits to drain;
- active/revoked state through the existing `VaultLeaseState` value;
- distinct typed keyed-handle `Locked` and `Revoked` failures;
- `VaultAccessError` propagation that preserves exact nested `ProtectorError` values without creating a second freshness state machine;
- idempotent terminal revocation and first-terminal-reason preservation;
- fail-closed synchronization-poison behavior;
- Himsat-owned tests for identity binding, active authorization, stale-handle rejection, publication-before-drain ordering, repeated revocation, first-terminal-reason preservation, synchronization poisoning, and typed error propagation.

A forward-only repair in the accepted lineage changed revocation ordering from lock-mediated publication to atomic terminal-state publication followed by drain synchronization. This prevents new operations from being admitted after terminal publication while already-authorized permits are still draining and matches the reviewed requirement to revoke first.

## Exact-head qualification

The accepted final head `40b9705258a302a4e71340f4dd0a28746e86c8a7` passed:

```text
PREMERGE_CI = 34154993507_SUCCESS
PREMERGE_R3 = 34154993537_SUCCESS
```

The earlier CI run `34154807417` is preserved as stale/cancelled after a head change and is not counted as PASS.

Final pre-merge reconciliation established:

```text
REVIEWS = NONE
REVIEW_THREADS = NONE
SUBSTANTIVE_BLOCKERS = NONE
QODO = BILLING_BLOCKED_NOT_PASS
CODERABBIT = AUTO_SKIP_NOT_PASS
MERGEABLE = TRUE
CHANGED_PATHS = crates/himsat-core/src/lib.rs; crates/himsat-core/src/vault_lease.rs
```

No submitted review or review thread existed. Unavailable automated review output was not promoted to independent PASS evidence.

## Guarded merge evidence

PR #41 was merged with the explicit transport argument:

```text
EXPECTED_HEAD_SHA_SUPPLIED = 40b9705258a302a4e71340f4dd0a28746e86c8a7
GITHUB_MERGED = true
GITHUB_MERGE_SHA = 4b27ede7d9bf17caac163b7607c331056634fc95
```

PR #41 durable comment `5574892895` records this actual forward transport result.

Canonical merge parentage was then verified exactly:

```text
MERGE = 4b27ede7d9bf17caac163b7607c331056634fc95
PARENT_1 = 0252bb31764c9178e270694f4087e8ac701271a0
PARENT_2 = 40b9705258a302a4e71340f4dd0a28746e86c8a7
```

## Exact post-merge qualification

The exact canonical merge passed both required push gates:

```text
POSTMERGE_CI_RUN = 34155726527
POSTMERGE_CI_RUN_NUMBER = 101
POSTMERGE_CI = SUCCESS
POSTMERGE_R3_RUN = 34155726505
POSTMERGE_R3_RUN_NUMBER = 78
POSTMERGE_R3 = SUCCESS
```

CI #101 completed successfully across the claimed matrix. The Windows Rust job was slower than the other jobs but completed formatting, lint, tests, and registered dependency-closure checks successfully; the delay was not treated as PASS until terminal SUCCESS was observed.

## Scope exclusions preserved

B102 does not implement or authorize:

- entropy acquisition or VRK generation;
- VRK wrapping, storage, unlocking, release, or zeroization behavior;
- HKDF, Argon2id, XChaCha20-Poly1305, nonce handling, recovery/blob envelopes, or any cryptographic execution;
- SQLCipher/database behavior or concrete database/blob handles;
- Apple, Android, Windows, or Linux native protector mechanics;
- protected freshness persistence, genesis installation, compare-and-advance implementation, rollback recovery, or restore behavior;
- B104 key hierarchy/domain-separation/secret-lifetime behavior;
- B105 concrete post-lock/post-revocation database/blob I/O proof;
- backup, rotation, deletion, Specification 005 media behavior, donor adoption, or release behavior.

## Successor boundary

B103 may be shaped only after this state/evidence reconciliation itself becomes canonical through an explicit expected-head guarded merge and passes exact post-merge CI and R3.

B103 is limited to the reviewed portable `SecretProtector` behavior contract: fail-closed requested-policy validation, exact provider capability reporting, binding/ownership/policy checks at the portable behavior layer, and Himsat-owned tests. B103 must not pretend platform mechanics or capabilities are identical and must not implement native secure-store adapters, cryptographic execution, database behavior, B104/B105 behavior, or Specification 005.