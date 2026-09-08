# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B201_CANONICAL_RECONCILING_B202_BOUND
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
SPEC_004P_ADOPTION_PR = 36
SPEC_004P_CANONICAL_MERGE = a4d32ee93e0ab95af8376ba0ca09e248070c5924
SPEC_004P_POSTMERGE_CI = 34144623816_SUCCESS
SPEC_004P_POSTMERGE_R3 = 34144623829_SUCCESS
P001 = CANONICAL_CLOSED
P002 = CANONICAL_CLOSED
P003 = CANONICAL_CLOSED
P004 = CANONICAL_CLOSED
P005 = CANONICAL_CLOSED
P006 = CANONICAL_CLOSED
P011_CANONICAL_MERGE = 7ecba93ae0763a5165dd99ce0ec190cb106906de
P011_POSTMERGE_CI = 34146939013_SUCCESS
P011_POSTMERGE_R3 = 34146938921_SUCCESS
P011_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
DEPENDENCY_BYTES_ADOPTED_ON_MAIN = YES
B101_DISPOSITION = CANONICAL_CLOSED
B102_DISPOSITION = CANONICAL_CLOSED
B103_DISPOSITION = CANONICAL_CLOSED
B104_DISPOSITION = CANONICAL_CLOSED
B105_DISPOSITION = CANONICAL_CLOSED
B105_B201_RECONCILIATION_PR = 49
B105_B201_RECONCILIATION_HEAD = 267351bcad78434b612c929d3165bc03ad1fb08c
B105_B201_RECONCILIATION_PREMERGE_CI = 34171637803_SUCCESS
B105_B201_RECONCILIATION_PREMERGE_R3 = 34171637800_SUCCESS
B105_B201_RECONCILIATION_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5577099576
B105_B201_RECONCILIATION_CANONICAL_MERGE = 33fc791443d25dba0ed7a5958710cb52ebb5bfad
B105_B201_RECONCILIATION_POSTMERGE_CI = 34172683259_SUCCESS
B105_B201_RECONCILIATION_POSTMERGE_R3 = 34172683357_SUCCESS
B201_DISPOSITION = CANONICAL_CLOSED
B201_PR = 50
B201_BASE = 33fc791443d25dba0ed7a5958710cb52ebb5bfad
B201_HEAD = 3c1706dfd0dfc7c745e19b81d19f31e4beb39e38
B201_PREMERGE_CI = 34173364034_SUCCESS
B201_PREMERGE_R3 = 34173364032_SUCCESS
B201_PREMERGE_RECONCILIATION = PROVEN_COMMENT_5577286248
B201_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5577371979
B201_CANONICAL_MERGE = 3785561963b6eadab219b330367a9b6295755939
B201_POSTMERGE_CI = 34174517394_SUCCESS
B201_POSTMERGE_R3 = 34174517397_SUCCESS
B201_POSTMERGE_EVIDENCE = PROVEN_COMMENT_5577459951
B201_B202_RECONCILIATION_STATE = ACTIVE_NOT_YET_CANONICAL
NEXT_IMPLEMENTATION_LEAF = B202_VERSIONED_BOUNDED_BLOB_XCHACHA20_POLY1305_ENVELOPE
SPEC_004_IMPLEMENTATION_AUTHORITY = B202_ONLY_IF_THIS_B201_B202_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B202_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub/repository truth overrides this file if repository state changes after this reconciliation is authored.

## Canonical Specification 004A design state

Specification 004A has an exact independently reviewed cryptographic design. The final review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE`, no blocking or non-blocking findings, B019-1 resolved, B020 resolved, and D001-D018 resolved without regression.

`round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` inventory semantics and complete canonical-envelope verification before plaintext release.

Design approval does not approve implementation. Security-semantic changes to the reviewed surface invalidate stale review evidence for that changed surface.

## Canonical provider/provenance state

Specification 004P is closed by exact live evidence recorded in `provider-adoption-final-evidence.md`. Canonical merge `a4d32ee93e0ab95af8376ba0ca09e248070c5924` passed post-merge CI `34144623816` and R3 `34144623829`. P001-P006 are `CANONICAL_CLOSED`; selected dependency/native bytes are adopted on `main`; registry v2 plus deterministic SBOM/notices and registered-closure CI/R3 remain the machine provenance boundary.

The currently adopted cryptographic dependency closure already contains the reviewed `hkdf`, `sha2`, `chacha20poly1305`, `getrandom`, and `argon2` providers needed by the authorized Specification 004 leaves. No new dependency adoption is implied by this state file.

Older failed runs, blocking reviews, tooling incidents, contradiction evidence, and unavailable review outputs remain part of the durable evidence lineage and are not erased by later closure.

## Historical transport evidence limitation

P011 reconciliation PR #37 and B101 PR #38 have exact head, canonical merge parentage, and exact pre/post-merge qualification evidence. Their historical merge API request bodies are not exposed by durable GitHub state, and no repository artifact records the exact historical `expected_head_sha` argument for those two merges. The repository does not reconstruct or retroactively promote that unavailable transport proof to PASS.

The B101 frontier was repaired forward-only by reconciliation PR #40, whose exact head `d99ceb841d834324d06774753b661bdf108f354b` passed CI `34152839411` and R3 `34152839424`, was explicitly expected-head guarded, merged as `0252bb31764c9178e270694f4087e8ac701271a0`, and passed post-merge CI `34153492791` and R3 `34153492769`.

`P011` therefore remains unchecked in `tasks.md`.

## Canonical 004B1 lineage

The accepted 004B1 implementation and reconciliation line is closed through B105:

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
```

Complete per-leaf evidence remains in:

- `b101-portable-contracts-final-evidence.md`;
- `b102-revocable-lease-final-evidence.md`;
- `b103-secret-protector-final-evidence.md`;
- `b104-key-lifetime-final-evidence.md`; and
- `b105-post-lock-io-final-evidence.md`.

The B105/B201 reconciliation PR #49 exact head `267351bcad78434b612c929d3165bc03ad1fb08c` passed CI `34171637803` and R3 `34171637800`, was merged with explicit expected-head protection as `33fc791443d25dba0ed7a5958710cb52ebb5bfad`, has exact parents `73b13d38ac3e34143c813bda679e6e96ce01762e` and `267351bcad78434b612c929d3165bc03ad1fb08c`, and passed post-merge CI `34172683259` and R3 `34172683357`.

The prior task-ledger omission that left `B105R001`-`B105R003` unchecked is repaired forward-only by this reconciliation from live evidence. No historical state is rewritten.

## Canonical B201 disposition

B201 implementation PR #50 started from exact canonical B105/B201 reconciliation merge `33fc791443d25dba0ed7a5958710cb52ebb5bfad` and changed only:

```text
crates/himsat-core/src/vault_keys.rs
```

The accepted exact head `3c1706dfd0dfc7c745e19b81d19f31e4beb39e38` passed CI `34173364034` and R3 `34173364032`. Observed exact-head tests were `36 passed; 0 failed` for `himsat-core` and `10 passed; 0 failed` for `himsat-events`, including deterministic HKDF vectors and purpose/vault/generation separation proof.

No submitted review or review thread existed. Qodo billing-blocked and CodeRabbit auto-skip output were not counted as PASS. Durable final pre-merge reconciliation comment: `5577286248`.

PR #50 was merged with explicit `expected_head_sha = 3c1706dfd0dfc7c745e19b81d19f31e4beb39e38`. Durable comment `5577371979` records successful guarded transport to canonical merge `3785561963b6eadab219b330367a9b6295755939`.

Canonical B201 merge parentage is exact: parent 1 is `33fc791443d25dba0ed7a5958710cb52ebb5bfad`; parent 2 is `3c1706dfd0dfc7c745e19b81d19f31e4beb39e38`. Exact push-triggered post-merge CI `34174517394` and R3 `34174517397` both reached terminal SUCCESS. Durable post-merge comment: `5577459951`.

B201 is therefore canonical and closed only for the reviewed HKDF-SHA-256 purpose-key derivation contract. Complete evidence is recorded in `b201-hkdf-final-evidence.md`.

Provider-internal temporary state, compiler/runtime/register/allocator copies, swap/pagefile content, crash/core dumps, kernel memory, and physical memory remain outside the owned-buffer erasure guarantee.

## Active objective

Canonicalize this B201 closeout/B202 rebound reconciliation without changing security semantics, product runtime behavior, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or release posture.

B202 remains blocked until this exact reconciliation is itself:

1. exact-head CI and R3 qualified;
2. reconciled against live reviews, threads, comments, exact diff, `main`, and mergeability;
3. merged with explicit `expected_head_sha` protection;
4. parentage-proven; and
5. exact push-triggered post-merge CI and R3 qualified.

Only then may B202 begin from the resulting exact canonical `main`.

## B202 bounded scope after reconciliation qualification

B202 owns only the already reviewed versioned `GENERIC_ARTIFACT_BLOB` XChaCha20-Poly1305 envelope execution/parsing contract.

Its v1 public envelope remains controlling exactly as defined in `round3-normative-contracts.md`:

```text
domain("HIMSAT/BLOB/ENVELOPE/v1")
u16(1)                         # envelope_version
u16(1)                         # cipher_suite = XChaCha20-Poly1305
u16(1)                         # object_purpose = GENERIC_ARTIFACT_BLOB
id128(VaultId)
id128(ArtifactId)
u64(key_generation)              # non-zero
u64(plaintext_length_bytes)     # 0..67_108_864
bytes24(nonce)
u32(ciphertext_and_tag_length) # exactly plaintext_length_bytes + 16
bytes(ciphertext_and_tag)
```

The fixed public header before ciphertext/tag is 107 bytes. The complete envelope is 123..67,108,987 bytes inclusive. Parser arithmetic must be checked before allocation or AEAD. No plaintext may be released before successful authentication.

The exact v1 AAD must authenticate every public context/allocation field defined by the reviewed contract.

B202 must accept an explicit 24-byte nonce supplied by its caller. It must not generate, reserve, retry, restore, or classify nonce collisions; those semantics belong to B203.

B202 must not expand into B203 nonce lifecycle, B204 recovery Argon2id/envelope execution, B301 SQLCipher, B401 native protectors, B501 freshness/backup/rotation/deletion, Specification 005 media behavior, new dependency adoption, donor-code adoption, release, FIPS, or compliance claims.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> P011/B101 frontier                       CANONICAL_QUALIFIED_WITH_HISTORICAL_TRANSPORT_EVIDENCE_GAP
  -> B101 portable vault contracts            CANONICAL_QUALIFIED_WITH_HISTORICAL_TRANSPORT_EVIDENCE_GAP
  -> B101/B102 forward-repair reconciliation  CANONICAL_QUALIFIED
  -> B102 revocable keyed-handle lease        CANONICAL_QUALIFIED
  -> B102/B103 state reconciliation           CANONICAL_QUALIFIED
  -> B103 SecretProtector behavior            CANONICAL_QUALIFIED
  -> B103/B104 state reconciliation           CANONICAL_QUALIFIED
  -> B104 key hierarchy/secret lifetime       CANONICAL_QUALIFIED
  -> B104/B105 state reconciliation           CANONICAL_QUALIFIED
  -> B105 concrete post-lock handle proof     CANONICAL_QUALIFIED
  -> B105/B201 state reconciliation           CANONICAL_QUALIFIED
  -> B201 HKDF-SHA-256 derivation             CANONICAL_QUALIFIED
  -> B201/B202 state reconciliation           ACTIVE
  -> B202 bounded-blob envelope               NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B203-B206 remaining crypto foundation    BLOCKED_PENDING_PRIOR_LEAVES
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
specs/004-vault-key-crypto/provider-provenance-selection.md
specs/004-vault-key-crypto/provider-resolution-evidence.md
specs/004-vault-key-crypto/provider-package-source-license-closure.md
specs/004-vault-key-crypto/provider-adoption-final-evidence.md
specs/004-vault-key-crypto/b101-portable-contracts-final-evidence.md
specs/004-vault-key-crypto/b102-revocable-lease-final-evidence.md
specs/004-vault-key-crypto/b103-secret-protector-final-evidence.md
specs/004-vault-key-crypto/b104-key-lifetime-final-evidence.md
specs/004-vault-key-crypto/b105-post-lock-io-final-evidence.md
specs/004-vault-key-crypto/b201-hkdf-final-evidence.md
governance/provenance/policy.json
governance/provenance/registry.json
governance/generated/sbom.json
THIRD_PARTY_NOTICES.md
```

## Preserved residual risks

The reviewed Specification 004 limits remain unchanged:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel, firmware, hardware, and physical-memory compromise remain outside this boundary;
- language/runtime/allocator copies, swap, crash dumps, and process teardown limit claims about complete secret erasure;
- ciphertext sizes, object counts, filesystem state, and operation/upload/backup/rotation timing can leak metadata;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove an authenticated backup is globally newest without a prior trusted freshness anchor;
- crypto-erasure does not prove physical-media, snapshot, provider-copy, or user-copy erasure;
- a detached recovery-enabled backup remains usable by a holder of the recovery passphrase;
- platforms unable to prove protected persistence and atomic freshness state must fail `UnsupportedPolicy`; rollbackable-file/plaintext fallback is prohibited.

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- `NOT RUN`, unavailable, manual-review, unknown, absent, skipped, billing-blocked, neutral, queued, cancelled, stale, and self-review are never independent PASS evidence.
- Negative evidence remains evidence and must not be hidden by later approval.
- No custom cipher, MAC, KDF, PRNG, or protocol is authorized.
- No plaintext key-file fallback is authorized.
- No donor code may be copied without exact machine-readable provenance plus bounded adoption authority.
- Model/asset/data licensing is independent of software-engine licensing.
- Each implementation leaf must remain independently bounded and exact-head qualified.
- Specification 004 is not complete until all implementation, adversarial/platform, independent implementation review, reconciliation, guarded merge, post-merge, and closeout gates are proven.