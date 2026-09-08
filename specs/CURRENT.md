# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B203_CANONICAL_RECONCILING_B204_BOUND
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
B203_PR = 54
B203_BASE = 96442f49a834cd88195fd934194485ca0c4fad7a
B203_INITIAL_HEAD = beaf8345c8b23fda2b56df77801c2b675ac4cfff
B203_INITIAL_CI = 34230105312_FAILURE_NOT_PASS
B203_FINAL_HEAD = b6fdde9da02b8cc92363095e26cc8971e0df7b6e
B203_PREMERGE_CI = 34230572075_SUCCESS
B203_PREMERGE_R3 = 34230572042_SUCCESS
B203_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5585875796
B203_CANONICAL_MERGE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
B203_POSTMERGE_CI = 34231954517_SUCCESS
B203_POSTMERGE_R3 = 34231954422_SUCCESS
B203_B204_RECONCILIATION_STATE = ACTIVE_NOT_YET_CANONICAL
NEXT_IMPLEMENTATION_LEAF = B204_RECOVERY_ENVELOPE_ARGON2ID_V1
SPEC_004_IMPLEMENTATION_AUTHORITY = B204_ONLY_IF_THIS_B203_B204_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B204_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub/repository truth overrides this file whenever repository state changes after this reconciliation is authored.

## Canonical Specification 004 authority

Specification 004A has an exact independently reviewed design. Review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE` with no unresolved blocking finding. `round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, nonce, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` canonical stored-object and manifest-inventory semantics.

Specification 004P is closed canonically. The reviewed dependency closure already contains the exact providers selected for the currently authorized leaves, including `hkdf`, `sha2`, `chacha20poly1305`, `getrandom`, and `argon2`. This reconciliation adopts no new dependency.

`P011` remains unchecked in `tasks.md`. Historical PR #37 has exact head, canonical merge, parentage, and post-merge CI/R3 evidence, but the historical merge API request body containing the exact `expected_head_sha` argument is not reconstructible post hoc. The repository does not fabricate or retroactively upgrade that missing transport evidence.

## Canonical implementation lineage through B203

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
```

Complete leaf evidence remains in the Specification 004 evidence files, including `b201-hkdf-final-evidence.md`, `b202-bounded-blob-final-evidence.md`, and `b203-nonce-lifecycle-final-evidence.md` once this reconciliation becomes canonical.

## Canonical B203 disposition

B203 implementation PR #54 started from exact canonical B202/B203 reconciliation merge `96442f49a834cd88195fd934194485ca0c4fad7a` and changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_nonce.rs
```

Initial exact head `beaf8345c8b23fda2b56df77801c2b675ac4cfff` produced CI `34230105312` with macOS and Windows formatting failures. Downstream lint/tests/registered dependency closure on those failed jobs were skipped and remain NOT PASS. Durable negative-lineage comment: `5585698183`.

No rerun or history rewrite was used. Forward-only successor head `b6fdde9da02b8cc92363095e26cc8971e0df7b6e` passed exact-head CI `34230572075` and R3 `34230572042`. Exact accepted-head tests included `56 passed; 0 failed` for `himsat-core` and `10 passed; 0 failed` for `himsat-events`.

No inline review thread existed. Qodo billing-blocked and CodeRabbit auto-skip outputs were NOT PASS. Repository-owner reconciliation review `5142309480` is durable evidence but is not an independent security approval.

Guarded transport used:

```text
EXPECTED_HEAD_SHA = b6fdde9da02b8cc92363095e26cc8971e0df7b6e
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
```

Durable transport-result comment: `5585875796`.

Canonical parentage is exact:

```text
PARENT_1 = 96442f49a834cd88195fd934194485ca0c4fad7a
PARENT_2 = b6fdde9da02b8cc92363095e26cc8971e0df7b6e
MERGE_TREE = 5c2b4a5ea595e2761eb8810d0203b0a0f79465ed
```

Exact push-triggered post-merge CI `34231954517` and R3 `34231954422` both reached terminal SUCCESS. CI proved formatting, lint, tests, and registered dependency closure on Ubuntu, macOS, and Windows, plus Diffcipline R2, SpecGrain, provenance validation/generated closure, provenance adversarial self-test, and negative controls.

B203 is therefore canonical and closed only for the reviewed OS-CSPRNG nonce-generation, reservation, retry, restore, collision, and duplicate-detection lifecycle. It does not claim recovery, SQLCipher, platform protectors, freshness anchors, backup/restore workflow, full rotation, deletion, Specification 005 media behavior, or release/compliance qualification.

## Active reconciliation objective

Canonicalize the B203 closeout/B204 rebound state without changing security semantics, product runtime behavior, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or release posture.

This reconciliation must itself:

1. pass exact-head CI and R3;
2. be reconciled against live reviews, review threads, comments, exact diff, `main`, and mergeability;
3. merge only with explicit `expected_head_sha` protection;
4. prove exact canonical parentage; and
5. pass exact push-triggered post-merge CI and R3.

Only after all five conditions are proven may B204 implementation begin from the resulting exact canonical `main`.

## B204 bounded scope after reconciliation qualification

B204 owns only the reviewed recovery-envelope v1 execution contract.

Exact public representation:

```text
domain("HIMSAT/RECOVERY/ENVELOPE/v1")
u16(1)                         # envelope_version
u16(1)                         # envelope_type = VRK_WRAP
u16(1)                         # cipher_suite = XChaCha20-Poly1305
u16(1)                         # recovery_policy = ARGON2ID_RFC9106_64M_V1
id128(VaultId)
u64(key_generation)              # non-zero
u16(0x0013)                    # Argon2 version
u32(65536)                     # memory_kib
u32(3)                         # passes
u16(4)                         # parallelism
u16(32)                        # KDF output bytes
bytes16(salt)
bytes24(nonce)
u32(48)                        # ciphertext_and_tag_length
bytes48(ciphertext_and_tag)
```

The complete v1 recovery envelope is exactly 167 bytes. The parser must reject wrong total length, wrong domain, unsupported identifier, zero generation, non-canonical Argon2 parameters, truncation, trailing input, or impossible length relations before attacker-controlled values can select policy or cause KDF allocation.

Exact recovery AAD is the canonical `HIMSAT/RECOVERY/AAD/v1` representation from `round3-normative-contracts.md`, binding envelope schema/version/type/suite/policy, `VaultId`, generation, exact Argon2 parameters, salt, nonce, and fixed ciphertext/tag length.

The only permitted v1 KDF policy is:

```text
argon2_version = 0x0013
memory_kib = 65536
passes = 3
parallelism = 4
kdf_output_bytes = 32
salt_bytes = 16
nonce_bytes = 24
plaintext_vrk_bytes = 32
ciphertext_and_tag_bytes = 48
```

Wrong passphrase and AEAD authentication failure must return the same externally visible `RecoveryAuthenticationFailed` and release no VRK. Unsupported versions/policies fail without downgrade. Vault/generation/policy/AAD transplants fail closed.

B204 must not absorb:

- B205 aggregate adversarial qualification beyond tests required to prove B204 itself;
- B301+ SQLCipher integration;
- B401+ native protector adapters;
- B501 full authenticated freshness-manifest/OS-anchor implementation, backup/restore workflow, full rotation, or deletion;
- Specification 005 streaming/media journal behavior;
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
  -> B203/B204 state reconciliation           ACTIVE
  -> B204 recovery envelope                   NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B205-B206 remaining crypto foundation    BLOCKED_PENDING_PRIOR_LEAVES
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
```
