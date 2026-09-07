# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B103_CANONICAL_RECONCILING_B104_BOUND
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
B101_PR = 38
B101_HEAD = ea82e1246095bb921dc9e7e40716076edbda41a6
B101_CANONICAL_MERGE = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
B101_POSTMERGE_CI = 34149709088_SUCCESS
B101_POSTMERGE_R3 = 34149708977_SUCCESS
B101_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
B101_B102_RECONCILIATION_PR = 40
B101_B102_RECONCILIATION_HEAD = d99ceb841d834324d06774753b661bdf108f354b
B101_B102_RECONCILIATION_CANONICAL_MERGE = 0252bb31764c9178e270694f4087e8ac701271a0
B101_B102_RECONCILIATION_POSTMERGE_CI = 34153492791_SUCCESS
B101_B102_RECONCILIATION_POSTMERGE_R3 = 34153492769_SUCCESS
B102_DISPOSITION = CANONICAL_CLOSED
B102_PRE_AUTHORITY_PR = 39_CLOSED_UNMERGED
B102_PR = 41
B102_HEAD = 40b9705258a302a4e71340f4dd0a28746e86c8a7
B102_CANONICAL_MERGE = 4b27ede7d9bf17caac163b7607c331056634fc95
B102_POSTMERGE_CI = 34155726527_SUCCESS
B102_POSTMERGE_R3 = 34155726505_SUCCESS
B102_B103_RECONCILIATION_PR = 42
B102_B103_RECONCILIATION_HEAD = 47710251ade64e48741e94b4c23d784f04cbe809
B102_B103_RECONCILIATION_CANONICAL_MERGE = 9ed7ccd960cbf92a9718d424421dd40b71cfe0de
B102_B103_RECONCILIATION_POSTMERGE_CI = 34157271018_SUCCESS
B102_B103_RECONCILIATION_POSTMERGE_R3 = 34157271044_SUCCESS
B103_DISPOSITION = CANONICAL_CLOSED
B103_PR = 43
B103_INITIAL_HEAD = cbbf1d3a8e9c995f33cd5a375e583dea9950207d
B103_INITIAL_CI = 34159354566_FAILURE_FORMATTING_NOT_PASS
B103_HEAD = 47b8ed78705774d5e55f2eb1145385c725e325c0
B103_PREMERGE_CI = 34159489786_SUCCESS
B103_PREMERGE_R3 = 34159489849_SUCCESS
B103_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5575437514
B103_CANONICAL_MERGE = ead22ea8c0b248431a2f8a50264f6acdbc9f7a72
B103_POSTMERGE_CI = 34160202949_SUCCESS
B103_POSTMERGE_R3 = 34160202961_SUCCESS
NEXT_IMPLEMENTATION_LEAF = B104_KEY_HIERARCHY_SECRET_LIFETIME
SPEC_004_IMPLEMENTATION_AUTHORITY = B104_ONLY_IF_THIS_B103_B104_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B104_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub truth overrides this file if repository state changes after this reconciliation is authored.

## Canonical Specification 004A design state

Specification 004A has an exact independently reviewed cryptographic design. The final review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE`, no blocking or non-blocking findings, B019-1 resolved, B020 resolved, and D001-D018 resolved without regression.

`round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` inventory semantics and complete canonical-envelope verification before plaintext release.

Design approval does not approve implementation. Security-semantic changes to the reviewed surface invalidate stale review evidence for that changed surface.

## Canonical provider/provenance state

Specification 004P is closed by exact live evidence recorded in `provider-adoption-final-evidence.md`. Canonical merge `a4d32ee93e0ab95af8376ba0ca09e248070c5924` passed post-merge CI `34144623816` and R3 `34144623829`. P001-P006 are `CANONICAL_CLOSED`; selected dependency/native bytes are adopted on `main`; registry v2 plus deterministic SBOM/notices and registered-closure CI/R3 remain the machine provenance boundary.

Older failed runs, blocking reviews, tooling incidents, contradiction evidence, and unavailable review outputs remain part of the durable evidence lineage and are not erased by final closure.

## Historical transport evidence limitation

P011 reconciliation PR #37 and B101 PR #38 have exact head, canonical merge parentage, and exact pre/post-merge qualification evidence. Their historical merge API request bodies are not exposed by durable GitHub state, and no repository artifact records the exact historical `expected_head_sha` argument for those two merges. The repository does not reconstruct or retroactively promote that unavailable transport proof to PASS.

The frontier was repaired forward-only by reconciliation PR #40, whose exact head `d99ceb841d834324d06774753b661bdf108f354b` passed CI `34152839411` and R3 `34152839424`, was explicitly expected-head guarded, merged as `0252bb31764c9178e270694f4087e8ac701271a0`, and passed post-merge CI `34153492791` and R3 `34153492769`.

## Canonical B102 disposition

Pre-authority PR #39 is preserved as closed, unmerged negative history. The accepted B102 lineage was PR #41 from exact canonical base `0252bb31764c9178e270694f4087e8ac701271a0`.

B102 final head `40b9705258a302a4e71340f4dd0a28746e86c8a7` passed CI `34154993507` and R3 `34154993537`. Earlier CI `34154807417` was cancelled after a head change and remains stale negative evidence, not PASS. PR #41 merged through explicit expected-head transport as canonical `4b27ede7d9bf17caac163b7607c331056634fc95`, then passed exact post-merge CI `34155726527` and R3 `34155726505`.

The B102/B103 reconciliation PR #42 exact head `47710251ade64e48741e94b4c23d784f04cbe809` passed CI `34156621024` and R3 `34156621129`, was expected-head guarded into canonical merge `9ed7ccd960cbf92a9718d424421dd40b71cfe0de`, and passed exact post-merge CI `34157271018` and R3 `34157271044` before accepted B103 work began.

## Canonical B103 disposition

B103 implementation PR #43 started from exact canonical base `9ed7ccd960cbf92a9718d424421dd40b71cfe0de` and changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_protector.rs
```

The initial implementation head `cbbf1d3a8e9c995f33cd5a375e583dea9950207d` failed CI `34159354566` on rustfmt only. That failure remains negative evidence. A forward-only formatting repair produced final head `47b8ed78705774d5e55f2eb1145385c725e325c0`.

The final head passed CI `34159489786` and R3 `34159489849`. No submitted review or review thread existed; Qodo billing-blocked and CodeRabbit auto-skip output were not counted as PASS.

PR #43 was merged with explicit `expected_head_sha = 47b8ed78705774d5e55f2eb1145385c725e325c0`; durable comment `5575437514` records the successful transport and canonical merge SHA `ead22ea8c0b248431a2f8a50264f6acdbc9f7a72`.

Canonical B103 merge `ead22ea8c0b248431a2f8a50264f6acdbc9f7a72` has exact parents `9ed7ccd960cbf92a9718d424421dd40b71cfe0de` and `47b8ed78705774d5e55f2eb1145385c725e325c0`. Exact push qualification then succeeded: CI `34160202949` and R3 `34160202961` both reached terminal SUCCESS on that exact canonical SHA.

B103 is therefore canonical, exact-head qualified, guarded-merged, parentage-proven, and exact-post-merge qualified. Complete evidence is recorded in `b103-secret-protector-final-evidence.md`.

## Active objective

Canonicalize this B103 closeout/B104 rebound reconciliation without changing security semantics, product runtime behavior, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or release posture.

After this reconciliation itself is exact-head qualified, reconciled, merged with explicit expected-head protection, parentage-proven, and exact-post-merge qualified, execute only B104 as the next independently bounded Specification 004B1 implementation leaf.

B104 owns reviewed portable key-hierarchy/domain-separation identifiers and secret-lifetime teardown behavior. The canonical derivation domains are:

```text
HIMSAT/004/STRUCTURED/v1 || u64be(key_generation)
HIMSAT/004/BLOB/v1       || u64be(key_generation)
HIMSAT/004/MANIFEST/v1   || u64be(key_generation)
```

B104 may encode/freeze these reviewed domain identifiers and the reviewed ownership/lifetime contract, including the teardown order:

1. revoke the current vault lease first;
2. close keyed database/blob handles through a bounded abstraction without claiming B105's concrete I/O proof;
3. release VRK, Recovery KEK, and purpose-key objects;
4. zeroize owned secret buffers where the already reviewed runtime support provides that guarantee; and
5. discard plaintext caches owned by the vault session.

B104 must not implement actual HKDF-SHA-256 derivation or deterministic derivation vectors; those remain B201. B104 must not claim concrete DB/blob post-lock I/O rejection proof; that remains B105.

B104 does not authorize native protector adapters, SQLCipher integration, AEAD/recovery-envelope execution, freshness persistence, backup/restore/rotation/deletion implementation, Specification 005 media behavior, new dependency adoption, donor-code adoption, release, FIPS, or compliance claims.

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
  -> B103/B104 state reconciliation           ACTIVE
  -> B104 key hierarchy/secret lifetime       NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B105 concrete post-lock handle proof     BLOCKED_PENDING_B104
  -> B201-B206 crypto envelope foundation     BLOCKED_PENDING_PRIOR_LEAVES
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