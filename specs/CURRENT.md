# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B202_CANONICAL_RECONCILING_B203_BOUND
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
B202_PR = 52
B202_BASE = 5ed2c3f79ef5af783c479871217e4aba0dcf0fd2
B202_FINAL_HEAD = 5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510
B202_PREMERGE_CI = 34224041244_SUCCESS
B202_PREMERGE_R3 = 34224041193_SUCCESS
B202_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5584971324
B202_CANONICAL_MERGE = 483857be2abf017c93fd94c210beef8080bc47fb
B202_POSTMERGE_CI = 34225052523_SUCCESS
B202_POSTMERGE_R3 = 34225052492_SUCCESS
B202_POSTMERGE_EVIDENCE = PROVEN_REVIEW_5141620618
B202_B203_RECONCILIATION_STATE = ACTIVE_NOT_YET_CANONICAL
NEXT_IMPLEMENTATION_LEAF = B203_OS_CSPRNG_NONCE_LIFECYCLE
SPEC_004_IMPLEMENTATION_AUTHORITY = B203_ONLY_IF_THIS_B202_B203_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B203_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub/repository truth overrides this file whenever repository state changes after this reconciliation is authored.

## Canonical Specification 004 authority

Specification 004A has an exact independently reviewed design. Review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE` with no unresolved blocking finding. `round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, nonce, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` canonical stored-object and manifest-inventory semantics.

Specification 004P is closed canonically. The reviewed dependency closure already contains the exact providers selected for the currently authorized leaves, including `hkdf`, `sha2`, `chacha20poly1305`, `getrandom`, and `argon2`. No new dependency adoption is implied by this state file.

`P011` remains unchecked in `tasks.md`. Historical PR #37 has exact head, canonical merge, parentage, and post-merge CI/R3 evidence, but the historical merge request body containing the exact `expected_head_sha` argument is not reconstructible post hoc. The repository does not fabricate or retroactively upgrade that missing transport evidence.

## Canonical implementation lineage through B202

The accepted Specification 004 implementation line is canonical and exact-post-merge qualified through B202:

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
```

Complete leaf evidence remains in the Specification 004 evidence files, including `b201-hkdf-final-evidence.md` and `b202-bounded-blob-final-evidence.md`.

## Canonical B202 disposition

B202 implementation PR #52 started from exact canonical B201/B202 reconciliation merge `5ed2c3f79ef5af783c479871217e4aba0dcf0fd2` and changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_blob.rs
crates/himsat-core/src/vault_keys.rs
```

The accepted exact head `5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510` passed CI `34224041244` and R3 `34224041193`. Exact accepted-head tests included `44 passed; 0 failed` for `himsat-core` and `10 passed; 0 failed` for `himsat-events`.

The failed B202 lineages remain durable negative evidence and are not reclassified:

```text
fa7ca6e9edee4301a6382adca10da3aea3106f49 -> CI 34177306333 FAILURE_NOT_PASS
bf7ab08c480088bd5c286cf66d48ace70a7abb92 -> CI 34223420708 FAILURE_NOT_PASS
6ddee1e41349d8dfe8cb373b4308f77157091853 -> CI 34223784461 / R3 34223784391 FAILURE_NOT_PASS
```

Durable final pre-merge reconciliation review `5141534650` recorded the exact accepted head and evidence. Qodo billing-blocked and CodeRabbit auto-skip outputs were not counted as PASS. Review threads were empty. Repository-owner evidence comments/reviews are not independent approvals.

Durable guarded transport comment `5584971324` records:

```text
EXPECTED_HEAD_SHA = 5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = 483857be2abf017c93fd94c210beef8080bc47fb
```

Canonical parentage is exact:

```text
PARENT_1 = 5ed2c3f79ef5af783c479871217e4aba0dcf0fd2
PARENT_2 = 5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510
MERGE_TREE = 8b8b7657ceb6d5bdd032727c95e4536847031aa2
```

Exact push-triggered post-merge CI `34225052523` and R3 `34225052492` both reached terminal SUCCESS. CI proved formatting, lint, tests, and registered dependency closure on Ubuntu, macOS, and Windows, plus Diffcipline R2, SpecGrain, provenance validation/generated closure, provenance adversarial self-test, and negative controls. Durable post-merge evidence review: `5141620618`.

B202 is therefore canonical and closed only for the reviewed generic-artifact bounded-blob XChaCha20-Poly1305 v1 envelope execution/parsing contract. It does not claim production nonce lifecycle, recovery, SQLCipher, platform protectors, freshness anchors, backup, full rotation, deletion, Specification 005 media behavior, or release/compliance qualification.

## Active reconciliation objective

Canonicalize the B202 closeout/B203 rebound state without changing security semantics, product runtime behavior, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or release posture.

This reconciliation must itself:

1. pass exact-head CI and R3;
2. be reconciled against live reviews, review threads, comments, exact diff, `main`, and mergeability;
3. merge only with explicit `expected_head_sha` protection;
4. prove exact canonical parentage; and
5. pass exact push-triggered post-merge CI and R3.

Only after all five conditions are proven may B203 implementation begin from the resulting exact canonical `main`.

## B203 bounded scope after reconciliation qualification

B203 owns only the reviewed nonce lifecycle required by Specification 004.

Production randomness contract:

- every XChaCha20-Poly1305 encryption attempt obtains a fresh 24-byte nonce from the approved OS CSPRNG through the already reviewed provider path;
- randomness failure aborts before a new envelope/publication becomes canonical;
- there is no time seed, process/device identifier, ordinary PRNG, unknown UUID source, zero-filled fallback, counter-only fallback, or downgrade;
- deterministic RNG behavior is permitted only in explicitly marked test fixtures that production code cannot select.

Bounded-blob nonce protocol:

- nonce uniqueness domain is `(VaultId, key_generation, blob-purpose key, nonce)`;
- a retry after ambiguous/failed publication uses a new nonce and never reuses the abandoned candidate;
- copying/restoring an existing authenticated envelope preserves its nonce and ciphertext exactly;
- any re-encryption uses a fresh nonce;
- canonical publication reserves `(key_generation, nonce)` in authenticated manifest/inventory state;
- a pre-publication collision is discarded and regenerated;
- duplicate nonce discovery in canonical inventory for the same generation is `CorruptOrTampered` and blocks new writes pending recovery.

Manifest-nonce rules remain similarly bounded: fresh nonce per attempt, new nonce after ambiguous/failed publication, retained authenticated-history collision scanning for the same generation, and duplicate retained nonce detection as `CorruptOrTampered`.

B203 must not absorb:

- B204 Argon2id/recovery-envelope execution;
- B205 aggregate adversarial qualification beyond tests required to prove B203 itself;
- B301+ SQLCipher integration;
- B401+ native protector adapters;
- B501 full authenticated freshness-manifest/OS-anchor implementation, backup/restore workflow, full rotation, or deletion;
- Specification 005 streaming/media journal behavior;
- new dependency or donor adoption;
- release, FIPS, or compliance claims.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> B101-B105 portable foundation            CANONICAL_QUALIFIED
  -> B201 HKDF-SHA-256 derivation             CANONICAL_QUALIFIED
  -> B201/B202 state reconciliation           CANONICAL_QUALIFIED
  -> B202 bounded-blob envelope               CANONICAL_QUALIFIED
  -> B202/B203 state reconciliation           ACTIVE
  -> B203 nonce lifecycle                     NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B204-B206 remaining crypto foundation    BLOCKED_PENDING_PRIOR_LEAVES
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
```
