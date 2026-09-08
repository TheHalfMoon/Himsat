# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B204_CANONICAL_RECONCILING_B205_BOUND
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
B203_B204_RECONCILIATION_DISPOSITION = CANONICAL_CLOSED
B203_B204_RECONCILIATION_PR = 55
B203_B204_RECONCILIATION_HEAD = 6c01a464b89521d99d42798ba4e4129ea6bc5413
B203_B204_RECONCILIATION_PREMERGE_CI = 34233929258_SUCCESS
B203_B204_RECONCILIATION_PREMERGE_R3 = 34233929295_SUCCESS
B203_B204_RECONCILIATION_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5586288776
B203_B204_RECONCILIATION_CANONICAL_MERGE = c453d56c00fc1fabca09987f1c11084099c6326e
B203_B204_RECONCILIATION_POSTMERGE_CI = 34235087839_SUCCESS
B203_B204_RECONCILIATION_POSTMERGE_R3 = 34235087861_SUCCESS
B204_DISPOSITION = CANONICAL_CLOSED
B204_PR = 56
B204_BASE = c453d56c00fc1fabca09987f1c11084099c6326e
B204_FINAL_HEAD = 2a071e21c483b239e76fb12af5cecd24eec8059b
B204_PREMERGE_CI = 34241383350_SUCCESS
B204_PREMERGE_R3 = 34241383280_SUCCESS
B204_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5587314774
B204_CANONICAL_MERGE = ac0925051cc128580b9794837fd03200f9a7374b
B204_POSTMERGE_CI = 34242682960_SUCCESS
B204_POSTMERGE_R3 = 34242683051_SUCCESS
B204_B205_RECONCILIATION_STATE = ACTIVE_NOT_YET_CANONICAL
NEXT_IMPLEMENTATION_LEAF = B205_AGGREGATE_CRYPTO_ADVERSARIAL_EVIDENCE
SPEC_004_IMPLEMENTATION_AUTHORITY = B205_ONLY_IF_THIS_B204_B205_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B205_TEST_EVIDENCE_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub/repository truth overrides this file whenever repository state changes after this reconciliation is authored.

## Canonical Specification 004 authority

Specification 004A has an exact independently reviewed design. Review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE` with no unresolved blocking finding. `round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, nonce, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` canonical stored-object and manifest-inventory semantics.

Specification 004P is closed canonically. The reviewed dependency closure already contains the exact providers required by the implemented leaves, including `hkdf`, `sha2`, `chacha20poly1305`, `getrandom`, and `argon2`. This reconciliation adopts no dependency.

`P011` remains unchecked in `tasks.md`. Historical PR #37 has exact head, canonical merge, parentage, and post-merge CI/R3 evidence, but the historical merge API request body containing the exact `expected_head_sha` argument is not reconstructible post hoc. The repository does not fabricate or retroactively upgrade that missing transport evidence.

## Canonical implementation lineage through B204

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
```

Complete leaf evidence remains in the Specification 004 evidence files, including `b201-hkdf-final-evidence.md`, `b202-bounded-blob-final-evidence.md`, `b203-nonce-lifecycle-final-evidence.md`, and `b204-recovery-envelope-final-evidence.md` once this reconciliation becomes canonical.

## Canonical B203/B204 reconciliation disposition

The B203/B204 reconciliation was exact-head qualified before B204 began:

```text
PR = 55
BASE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
HEAD = 6c01a464b89521d99d42798ba4e4129ea6bc5413
PREMERGE_CI = 34233929258 / run #137 / SUCCESS
PREMERGE_R3 = 34233929295 / run #114 / SUCCESS
EXPECTED_HEAD_SHA = 6c01a464b89521d99d42798ba4e4129ea6bc5413
MERGE_METHOD = merge
CANONICAL_MERGE = c453d56c00fc1fabca09987f1c11084099c6326e
PARENT_1 = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
PARENT_2 = 6c01a464b89521d99d42798ba4e4129ea6bc5413
POSTMERGE_CI = 34235087839 / run #138 / SUCCESS
POSTMERGE_R3 = 34235087861 / run #115 / SUCCESS
```

Durable guarded-transport comment: `5586288776`. Durable post-merge reconciliation comment: `5586418214`.

The prior ledger left B203R001-B203R003 unchecked even though the live transport, parentage, and post-merge evidence are now reconstructible and proven. This reconciliation repairs that ledger state forward-only; it does not alter historical commits.

## Canonical B204 disposition

B204 implementation PR #56 started from exact canonical B203/B204 reconciliation merge `c453d56c00fc1fabca09987f1c11084099c6326e` and changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_recovery.rs
crates/himsat-core/tests/b204_recovery_negative.rs
```

The accepted implementation provides the exact 167-byte recovery envelope, fixed Argon2id v1 profile, exact 116-byte canonical recovery AAD, OS-CSPRNG salt/nonce generation, XChaCha20-Poly1305 VRK wrapping, structural validation before KDF allocation, exact passphrase semantics, uniform recovery-authentication failure, transplant rejection, and source-level temporary-key zeroization required by the reviewed contract.

Preserved failed/cancelled heads remain NOT PASS. Durable negative/superseded comments include `5586570304`, `5586762590`, `5586996995`, and `5587124329`. No rerun, force-push, rebase, history rewrite, or retroactive reclassification was used.

Final exact head `2a071e21c483b239e76fb12af5cecd24eec8059b` passed CI `34241383350` and R3 `34241383280`. Exact-head R3 observed `64 passed; 0 failed` for `himsat-core`, `3 passed; 0 failed` for `b204_recovery_negative`, and `10 passed; 0 failed` for `himsat-events`.

No inline review thread existed. Qodo billing-blocked and CodeRabbit auto-skip outputs were NOT PASS. Cubic summary output was neutral automation. Repository-owner reconciliation review `5143469011` is durable evidence but is not an independent substantive crypto/security approval and does not satisfy Q009.

Guarded transport used:

```text
EXPECTED_HEAD_SHA = 2a071e21c483b239e76fb12af5cecd24eec8059b
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = ac0925051cc128580b9794837fd03200f9a7374b
```

Durable transport-result comment: `5587314774`.

Canonical parentage is exact:

```text
PARENT_1 = c453d56c00fc1fabca09987f1c11084099c6326e
PARENT_2 = 2a071e21c483b239e76fb12af5cecd24eec8059b
MERGE_TREE = d63407d8a6bc50514bd8ff1df8bbf239455e9ac8
```

Exact push-triggered post-merge CI `34242682960` and R3 `34242683051` both reached terminal SUCCESS on exact canonical merge `ac0925051cc128580b9794837fd03200f9a7374b`.

B204 is therefore canonical and closed only for the reviewed recovery-envelope v1 execution contract. It does not close B205 aggregate adversarial qualification, later implementation leaves, Q009 independent review, or Specification 004 itself.

## Active reconciliation objective

Canonicalize the B204 closeout/B205 rebound state without changing security semantics, product runtime behavior, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or release posture.

This reconciliation must itself:

1. pass exact-head CI and R3;
2. be reconciled against live reviews, review threads, comments, exact diff, `main`, and mergeability;
3. merge only with explicit `expected_head_sha` protection;
4. prove exact canonical parentage; and
5. pass exact push-triggered post-merge CI and R3.

Only after all five conditions are proven may B205 work begin from the resulting exact canonical `main`.

## B205 bounded scope after reconciliation qualification

B205 owns aggregate adversarial crypto evidence for the already implemented reviewed contracts. It may add tests/fixtures and only the minimum test-support surface necessary to exercise existing behavior.

Required evidence includes, where applicable to the bounded-blob and recovery envelopes:

- wrong key and wrong passphrase;
- independent tamper of fixed identifiers/public header, salt/nonce, authenticated context/AAD inputs, ciphertext, and tag;
- unknown suite/version/purpose/envelope type/recovery policy;
- zero/stale generation and vault/artifact/generation transplant;
- truncation and trailing data;
- declared-length mismatch and checked arithmetic/overflow paths;
- bounded-blob zero length, exact maximum, and maximum-plus-one without unsafe allocation behavior;
- weaker/larger/non-canonical recovery KDF parameter rejection before KDF allocation;
- uniform wrong-passphrase versus AEAD-failure recovery result;
- random-source failure; and
- duplicate/collision behavior owned by the canonical B203 nonce lifecycle.

B205 must not absorb:

- B206 or Specification 005 media streaming/journal sequencing;
- B301+ SQLCipher integration;
- B401+ native protector adapters;
- B501+ freshness/backup/rotation/deletion implementation;
- Q009 independent substantive crypto/security review;
- new dependency or donor adoption; or
- release, FIPS, or compliance claims.

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
  -> B204/B205 state reconciliation           ACTIVE
  -> B205 aggregate adversarial crypto        NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B206 Specification 005 boundary          BLOCKED_PENDING_B205
  -> B301-B307 encrypted structured store     BLOCKED_PENDING_PRIOR_LEAVES
  -> B401-B406 platform protectors            BLOCKED_PENDING_PRIOR_LEAVES
  -> B501-B506 freshness/backup/rotation      BLOCKED_PENDING_PRIOR_LEAVES
  -> Q001-Q012 exact implementation review/R3 BLOCKED_PENDING_IMPLEMENTATION
  -> C001-C005 Specification 004 closeout      BLOCKED_PENDING_ALL_PRIOR_GATES
  -> Specification 005                        BLOCKED_PENDING_SPEC_004_CLOSEOUT
```

## Active artifacts

```text
specs/004-vault-key-crypto/spec.md
specs/004-vault-key-crypto/plan.md
specs/004-vault-key-crypto/tasks.md
specs/004-vault-key-crypto/round3-normative-contracts.md
specs/004-vault-key-crypto/round4-blob-inventory-contract.md
specs/004-vault-key-crypto/review-round4-final-evidence.md
specs/004-vault-key-crypto/provider-adoption-final-evidence.md
specs/004-vault-key-crypto/b101-portable-contracts-final-evidence.md
specs/004-vault-key-crypto/b102-revocable-lease-final-evidence.md
specs/004-vault-key-crypto/b103-secret-protector-final-evidence.md
specs/004-vault-key-crypto/b104-key-lifetime-final-evidence.md
specs/004-vault-key-crypto/b105-post-lock-io-final-evidence.md
specs/004-vault-key-crypto/b201-hkdf-final-evidence.md
specs/004-vault-key-crypto/b202-bounded-blob-final-evidence.md
specs/004-vault-key-crypto/b203-nonce-lifecycle-final-evidence.md
specs/004-vault-key-crypto/b204-recovery-envelope-final-evidence.md
```
