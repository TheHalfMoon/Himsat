# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B205_CANONICAL_RECONCILING_B301_BOUND
ACTIVE_SPECIFICATION = 004-vault-key-crypto
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_002_DISPOSITION = CLOSED_CANONICAL
SPEC_003_DISPOSITION = CLOSED_CANONICAL
SPEC_003_CLOSEOUT_MERGE = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
SPEC_004A_DESIGN_AUTHORITY = APPROVED_EXACT_CANONICAL
SPEC_004_FINAL_REVIEWED_SHA = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
SPEC_004_FINAL_REVIEW_DISPOSITION = APPROVE
SPEC_004_FINAL_BLOCKING_FINDINGS = NONE
SPEC_004P_DISPOSITION = CLOSED_CANONICAL
SPEC_004P_CANONICAL_MERGE = a4d32ee93e0ab95af8376ba0ca09e248070c5924
P011_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
B101_DISPOSITION = CANONICAL_CLOSED
B102_DISPOSITION = CANONICAL_CLOSED
B103_DISPOSITION = CANONICAL_CLOSED
B104_DISPOSITION = CANONICAL_CLOSED
B105_DISPOSITION = CANONICAL_CLOSED
B201_DISPOSITION = CANONICAL_CLOSED
B202_DISPOSITION = CANONICAL_CLOSED
B203_DISPOSITION = CANONICAL_CLOSED
B204_DISPOSITION = CANONICAL_CLOSED
B205_DISPOSITION = CANONICAL_CLOSED
B206_DISPOSITION = CLOSED_SCOPE_BOUNDARY_SPEC_005
B204_B205_RECONCILIATION_DISPOSITION = CANONICAL_CLOSED
B204_B205_RECONCILIATION_PR = 57
B204_B205_RECONCILIATION_HEAD = 7224dbb1fd005813796938af2cb783d500fdf7ef
B204_B205_RECONCILIATION_PREMERGE_CI = 34245532899_SUCCESS
B204_B205_RECONCILIATION_PREMERGE_R3 = 34245532870_SUCCESS
B204_B205_RECONCILIATION_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5588329735
B204_B205_RECONCILIATION_CANONICAL_MERGE = 854373dc4fd44cc496b9e52f9f14804cdbf750aa
B204_B205_RECONCILIATION_POSTMERGE_CI = 34250181834_SUCCESS
B204_B205_RECONCILIATION_POSTMERGE_R3 = 34250182026_SUCCESS
B205_PR = 58
B205_BASE = 854373dc4fd44cc496b9e52f9f14804cdbf750aa
B205_FINAL_HEAD = bcb317b8b2e4417a8a22534f5fed375305915161
B205_PREMERGE_CI = 34251969171_SUCCESS
B205_PREMERGE_R3 = 34251969189_SUCCESS
B205_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5588704920
B205_CANONICAL_MERGE = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
B205_POSTMERGE_CI = 34253283595_SUCCESS
B205_POSTMERGE_R3 = 34253283658_SUCCESS
B205_B301_RECONCILIATION_STATE = ACTIVE_NOT_YET_CANONICAL
NEXT_IMPLEMENTATION_LEAF = B301_REVIEWED_SQLCIPHER_INTEGRATION_ONLY
SPEC_004_IMPLEMENTATION_AUTHORITY = B301_ONLY_IF_THIS_B205_B301_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B301_STRUCTURED_STORE_PROVIDER_INTEGRATION_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub/repository truth overrides this file whenever repository state changes after this reconciliation is authored.

## Canonical Specification 004 authority

Specification 004A has an exact independently reviewed design. Review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE` with no unresolved blocking finding. `round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, nonce, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` canonical stored-object and manifest-inventory semantics.

Specification 004P is closed canonically. The reviewed dependency closure already contains the exact providers required by B301; this reconciliation adopts no dependency and changes no provider/provenance bytes.

`P011` remains unchecked in `tasks.md`. Historical PR #37 has exact head, canonical merge, parentage, and post-merge CI/R3 evidence, but the historical merge API request body containing the exact `expected_head_sha` argument is not reconstructible post hoc. The repository does not fabricate or retroactively upgrade that missing transport evidence.

## Canonical implementation lineage through B205

```text
B101_CANONICAL_MERGE = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
B101_B102_RECONCILIATION = 0252bb31764c9178e270694f4087e8ac701271a0
B102_CANONICAL_MERGE = 4b27ede7d9bf17caac163b7607c331056634fc95
B102_B103_RECONCILIATION = 9ed7ccd960cbf92a9718d424421dd40b71cfe0de
B103_CANONICAL_MERGE = ead22ea8c0b248431a2f8a50264f6acdbc9f7a72
B103_B104_RECONCILIATION = 57217a6614c07ac5e8a00d85114dd06eee1a0120
B104_CANONICAL_MERGE = 6578936f7051b548339f3cf95ca0d41e6629d8bc
B104_B105_RECONCILIATION = 623c0ad4207ecbbd5e404a2a32ba3cdb61006778
B105_CANONICAL_MERGE = 73b13d38ac3e34143c813bda679e6e96ce01762e
B105_B201_RECONCILIATION = 33fc791443d25dba0ed7a5958710cb52ebb5bfad
B201_CANONICAL_MERGE = 3785561963b6eadab219b330367a9b6295755939
B201_B202_RECONCILIATION = 5ed2c3f79ef5af783c479871217e4aba0dcf0fd2
B202_CANONICAL_MERGE = 483857be2abf017c93fd94c210beef8080bc47fb
B202_B203_RECONCILIATION = 96442f49a834cd88195fd934194485ca0c4fad7a
B203_CANONICAL_MERGE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
B203_B204_RECONCILIATION = c453d56c00fc1fabca09987f1c11084099c6326e
B204_CANONICAL_MERGE = ac0925051cc128580b9794837fd03200f9a7374b
B204_B205_RECONCILIATION = 854373dc4fd44cc496b9e52f9f14804cdbf750aa
B205_CANONICAL_MERGE = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
```

Complete leaf evidence remains in the Specification 004 evidence files, including `b201-hkdf-final-evidence.md`, `b202-bounded-blob-final-evidence.md`, `b203-nonce-lifecycle-final-evidence.md`, `b204-recovery-envelope-final-evidence.md`, and `b205-crypto-adversarial-final-evidence.md` once this reconciliation becomes canonical.

## Canonical B204/B205 reconciliation disposition

The B204/B205 reconciliation was exact-head qualified before B205 began:

```text
PR = 57
BASE = ac0925051cc128580b9794837fd03200f9a7374b
HEAD = 7224dbb1fd005813796938af2cb783d500fdf7ef
PREMERGE_CI = 34245532899 / run #147 / SUCCESS
PREMERGE_R3 = 34245532870 / run #124 / SUCCESS
EXPECTED_HEAD_SHA = 7224dbb1fd005813796938af2cb783d500fdf7ef
MERGE_METHOD = merge
CANONICAL_MERGE = 854373dc4fd44cc496b9e52f9f14804cdbf750aa
PARENT_1 = ac0925051cc128580b9794837fd03200f9a7374b
PARENT_2 = 7224dbb1fd005813796938af2cb783d500fdf7ef
POSTMERGE_CI = 34250181834 / run #148 / SUCCESS
POSTMERGE_R3 = 34250182026 / run #125 / SUCCESS
```

Durable guarded-transport comment: `5588329735`. Superseded head `63029aad7a2ab94f6592b3b31afd15c2df510cf6` remains cancelled/non-PASS evidence under durable comment `5587750970`.

The prior task ledger left B204R001-B204R003 unchecked despite this proven live evidence. This reconciliation repairs that state forward-only; it does not alter historical commits or upgrade any unavailable evidence.

## Canonical B205 disposition

B205 implementation PR #58 started from exact canonical B204/B205 reconciliation merge `854373dc4fd44cc496b9e52f9f14804cdbf750aa` and changed only:

```text
crates/himsat-core/tests/b205_crypto_adversarial.rs
```

The accepted aggregate test suite covers the reviewed bounded-blob and recovery negative surface: wrong key/passphrase, structural and fixed-identifier tamper, nonce/salt/context/ciphertext/tag tamper, transplants, truncation/trailing data, declared-length mismatch, bounded-length zero/max/max-plus-one/overflow handling, invalid/non-canonical Argon2 profile rejection before KDF allocation, uniform recovery-authentication failure, and duplicate canonical nonce rejection. Existing canonical B203/B204 unit tests retain the fault-injection evidence for random-source failure, collision regeneration, and fixed-profile resource failure.

Preserved predecessor heads remain NOT PASS:

```text
32199dda56f75d9bfcbe852c54e9da32675d9fd8
  CI = 34251440853_FAILURE_NOT_PASS
  R3 = 34251440888_CANCELLED_NOT_PASS

d7dff7e9f7388c85fc26de882cde228bfbb820b9
  CI = 34251729510_FAILURE_NOT_PASS
  R3 = 34251729365_CANCELLED_NOT_PASS
```

Durable negative-evidence comments: `5588499095`, `5588530997`. No rerun, force-push, rebase, history rewrite, or retroactive reclassification was used.

Final exact head `bcb317b8b2e4417a8a22534f5fed375305915161` passed CI `34251969171` and R3 `34251969189`. Ubuntu exact-head tests observed `64 + 3 + 6 + 10` passing tests, including all six B205 aggregate tests.

No inline review thread existed. Qodo billing-blocked and CodeRabbit auto-skip outputs were NOT PASS. Repository-owner reconciliation review `5144591172` is durable evidence but is not an independent substantive crypto/security approval and does not satisfy Q009.

Guarded transport used:

```text
EXPECTED_HEAD_SHA = bcb317b8b2e4417a8a22534f5fed375305915161
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
```

Durable transport-result comment: `5588704920`.

Canonical parentage is exact:

```text
PARENT_1 = 854373dc4fd44cc496b9e52f9f14804cdbf750aa
PARENT_2 = bcb317b8b2e4417a8a22534f5fed375305915161
MERGE_TREE = 0230ab7cc876d4d70db2dce563d0705429116049
```

Exact push-triggered post-merge CI `34253283595` and R3 `34253283658` both reached terminal SUCCESS on exact canonical merge `4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf`.

B205 is therefore canonical and closed only for aggregate adversarial evidence over the already reviewed B202-B204 contracts.

## B206 boundary disposition

B206 is closed as a boundary statement only. B201-B205 did not implement or absorb media streaming or media-journal sequencing. Those behaviors remain owned by Specification 005, which itself remains blocked pending Specification 004 closeout.

Closing this boundary does not grant Specification 005 implementation authority.

## Active reconciliation objective

Canonicalize the B205 closeout/B301 rebound state without changing security semantics, product runtime behavior, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or release posture.

This reconciliation must itself:

1. pass exact-head CI and R3;
2. be reconciled against live reviews, review threads, comments, exact diff, `main`, and mergeability;
3. merge only with explicit `expected_head_sha` protection;
4. prove exact canonical parentage; and
5. pass exact push-triggered post-merge CI and R3.

Only after all five conditions are proven may B301 work begin from the resulting exact canonical `main`.

## B301 bounded scope after reconciliation qualification

B301 owns only integration of the already reviewed and provenance-registered SQLCipher structured-store provider strategy. No dependency adoption is needed or authorized by this leaf.

Canonical provider identities are:

```text
RUSQLITE_PACKAGE = 0.40.1
LIBSQLITE3_SYS_PACKAGE = 0.38.2
RUSQLITE_FEATURE = bundled-sqlcipher-vendored-openssl
SQLCIPHER_VERSION = 4.14.0
SQLCIPHER_EMBEDDED_SQLITE_VERSION = 3.51.3
OPENSSL_UPSTREAM_VERSION = 3.6.3
```

The selected bundled SQLCipher build includes the reviewed codec/temp-store compile posture. The direct `libsqlite3-sys` dependency is a provenance/resolver pin only; application code must use the higher-level reviewed `rusqlite` surface unless a separately authorized FFI use is later proven necessary.

B301 may introduce only the minimum structured-store module/API and tests required to instantiate the exact reviewed provider/keying strategy through the existing `KeyPurpose::Structured` derivation. B301 must not absorb:

- B302 encryption-active proof/claim;
- B303 runtime temp/WAL/journal/provider-setting enforcement;
- B304 normal/cipher integrity-check qualification;
- B305 wrong-key/corruption/unsupported-provider/version fixtures;
- B306 plaintext-spill/file-name qualification;
- B307 copy-verify-publish migration;
- B401+ platform protector implementation;
- B501+ freshness/backup/rotation/deletion;
- Q009 independent review;
- Specification 005 behavior;
- new donor/dependency adoption; or
- release, FIPS, or compliance claims.

If the selected exact SQLCipher/provider build cannot be reproduced on a qualified target, B301 must fail closed rather than silently switch to a system SQLite/SQLCipher/OpenSSL provider.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> B101-B105 portable foundation            CANONICAL_QUALIFIED
  -> B201 HKDF-SHA-256 derivation             CANONICAL_QUALIFIED
  -> B201/B202 state reconciliation           CANONICAL_QUALIFIED
  -> B202 bounded-blob envelope               CANONICAL_QUALIFIED
  -> B202/B203 state reconciliation           CANONICAL_QUALIFIED
  -> B203 nonce lifecycle                     CANONICAL_QUALIFIED
  -> B203/B204 state reconciliation           CANONICAL_QUALIFIED
  -> B204 recovery envelope                   CANONICAL_QUALIFIED
  -> B204/B205 state reconciliation           CANONICAL_QUALIFIED
  -> B205 aggregate adversarial crypto        CANONICAL_QUALIFIED
  -> B206 Specification 005 boundary          CLOSED_SCOPE_BOUNDARY
  -> B205/B301 state reconciliation           ACTIVE
  -> B301-B307 encrypted structured store     B301_NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B401-B406 platform protectors            BLOCKED_PENDING_PRIOR_LEAVES
  -> B501-B506 freshness/backup/rotation      BLOCKED_PENDING_PRIOR_LEAVES
  -> Q001-Q012 exact implementation review/R3 BLOCKED_PENDING_IMPLEMENTATION
  -> C001-C005 Specification 004 closeout      BLOCKED_PENDING_ALL_PRIOR_GATES
  -> Specification 005                        BLOCKED_PENDING_SPEC_004_CLOSEOUT
```
