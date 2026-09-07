# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B102_CANONICAL_RECONCILING_B103_BOUND
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
B101_PREMERGE_CI = 34148418394_SUCCESS
B101_PREMERGE_R3 = 34148418341_SUCCESS
B101_CANONICAL_MERGE = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
B101_POSTMERGE_CI = 34149709088_SUCCESS
B101_POSTMERGE_R3 = 34149708977_SUCCESS
B101_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
B101_B102_RECONCILIATION_PR = 40
B101_B102_RECONCILIATION_HEAD = d99ceb841d834324d06774753b661bdf108f354b
B101_B102_RECONCILIATION_EXPECTED_HEAD_MERGE = PROVEN
B101_B102_RECONCILIATION_CANONICAL_MERGE = 0252bb31764c9178e270694f4087e8ac701271a0
B101_B102_RECONCILIATION_POSTMERGE_CI = 34153492791_SUCCESS
B101_B102_RECONCILIATION_POSTMERGE_R3 = 34153492769_SUCCESS
B102_DISPOSITION = CANONICAL_CLOSED
B102_PRE_AUTHORITY_PR = 39_CLOSED_UNMERGED
B102_PR = 41
B102_HEAD = 40b9705258a302a4e71340f4dd0a28746e86c8a7
B102_PREMERGE_CI = 34154993507_SUCCESS
B102_PREMERGE_R3 = 34154993537_SUCCESS
B102_EXPECTED_HEAD_TRANSPORT_PROOF = PROVEN_COMMENT_5574892895
B102_CANONICAL_MERGE = 4b27ede7d9bf17caac163b7607c331056634fc95
B102_POSTMERGE_CI = 34155726527_SUCCESS
B102_POSTMERGE_R3 = 34155726505_SUCCESS
NEXT_IMPLEMENTATION_LEAF = B103_SECRET_PROTECTOR_BEHAVIOR
SPEC_004_IMPLEMENTATION_AUTHORITY = B103_ONLY_IF_THIS_B102_B103_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B103_ONLY_UNDER_THE_CONDITION_ABOVE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub truth overrides this file if repository state changes after this reconciliation is authored.

## Canonical Specification 004A design state

Specification 004A has an exact independently reviewed cryptographic design. The final review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE`, no blocking or non-blocking findings, B019-1 resolved, B020 resolved, and D001-D018 resolved without regression.

`round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` inventory semantics and complete canonical-envelope verification before plaintext release.

Design approval does not approve implementation. Security-semantic changes to the reviewed surface invalidate stale review evidence for that changed surface.

## Canonical Specification 004P disposition

Specification 004P is closed by exact live evidence recorded in `provider-adoption-final-evidence.md`. Canonical merge `a4d32ee93e0ab95af8376ba0ca09e248070c5924` passed post-merge CI `34144623816` and R3 `34144623829`. P001-P006 are `CANONICAL_CLOSED`; selected dependency/native bytes are adopted on `main`; registry v2 plus deterministic SBOM/notices and registered-closure CI/R3 remain the machine provenance boundary.

Older failed runs, blocking reviews, tooling incidents, and contradiction evidence remain part of the durable evidence lineage and are not erased by final closure.

## Canonical P011 and B101 disposition

P011 reconciliation PR #37 used exact head `367348c2b11add2c15f7d25af92ca4679b2a19d1`. That head passed CI `34145714886` and R3 `34145714971`, then merged canonically as `7ecba93ae0763a5165dd99ce0ec190cb106906de`. That exact canonical merge passed post-merge CI `34146939013` and R3 `34146938921`.

GitHub's durable state proves the exact head, merge SHA, parentage, and qualification, but it does not expose the historical merge API request body. No repository artifact records the exact `expected_head_sha` transport argument for PR #37. That transport proof remains unavailable rather than reconstructed or silently promoted to PASS.

B101 completed through PR #38. The initial implementation head failed R3 formatting only; a forward-only rustfmt repair produced exact final head `ea82e1246095bb921dc9e7e40716076edbda41a6`, which passed CI `34148418394` and R3 `34148418341`.

PR #38 changed only `crates/himsat-core/src/lib.rs` and `crates/himsat-core/src/vault.rs`. No submitted review or review thread existed; Qodo billing-blocked output and CodeRabbit auto-skip output were not counted as independent PASS evidence. PR #38 merged canonically as `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c` and passed post-merge CI `34149709088` and R3 `34149708977`.

Historical transport-level `expected_head_sha` evidence for PR #38 is also not reconstructible post hoc. The repository did not rewrite that history. It repaired the frontier forward-only through PR #40.

## Canonical B101/B102 forward repair

Reconciliation PR #40 exact head `d99ceb841d834324d06774753b661bdf108f354b` passed CI `34152839411` and R3 `34152839424`, then was merged with explicit `expected_head_sha = d99ceb841d834324d06774753b661bdf108f354b`.

Canonical reconciliation merge `0252bb31764c9178e270694f4087e8ac701271a0` has parents `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c` and `d99ceb841d834324d06774753b661bdf108f354b`. Exact post-merge CI `34153492791` and R3 `34153492769` succeeded. This forward repair opened B102 authority without pretending the earlier historical transport records had become reconstructible.

## Canonical B102 disposition

Pre-authority PR #39 is preserved as closed, unmerged negative history. It was based on `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c` before the B101/B102 reconciliation was canonical and post-merge qualified. Its final head `cb392463dec68893891f9adc97dcb21ead4c5a22` is not an authority or qualification source for accepted B102 work.

The accepted forward-only lineage was PR #41 from exact canonical base `0252bb31764c9178e270694f4087e8ac701271a0`. Its final head `40b9705258a302a4e71340f4dd0a28746e86c8a7` changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_lease.rs
```

The exact final head passed CI `34154993507` and R3 `34154993537`. Earlier CI `34154807417` was cancelled after a head change and remains stale negative evidence, not PASS. No submitted review or review thread existed; Qodo billing-blocked and CodeRabbit auto-skip output were not counted as independent review PASS.

PR #41 merged through an explicit expected-head transport using `expected_head_sha = 40b9705258a302a4e71340f4dd0a28746e86c8a7`. Durable PR comment `5574892895` records `GITHUB_MERGED = true` and merge SHA `4b27ede7d9bf17caac163b7607c331056634fc95`.

Canonical merge `4b27ede7d9bf17caac163b7607c331056634fc95` has exact parents `0252bb31764c9178e270694f4087e8ac701271a0` and `40b9705258a302a4e71340f4dd0a28746e86c8a7`. Exact push qualification then succeeded: CI `34155726527` and R3 `34155726505` both reached terminal SUCCESS on that exact canonical SHA.

B102 is therefore canonical, exact-head qualified, guarded-merged, parentage-proven, and exact-post-merge qualified. Its complete evidence and preserved negative lineage are recorded in `b102-revocable-lease-final-evidence.md`.

## Active objective

Canonicalize this B102 closeout/B103 re-bound state/evidence reconciliation without changing security semantics, dependency bytes, provenance entries, generated artifacts, workflows, donor material, or product runtime behavior.

After this reconciliation itself is merged with an explicit expected-head guard and passes exact post-merge CI and R3, execute only B103 as the next independently bounded Specification 004B implementation leaf.

B103 is limited to the reviewed portable `SecretProtector` behavior contract. It may implement portable requested-policy validation, exact capability reporting, fail-closed owner/vault/generation/policy binding checks, provider-neutral behavior helpers, and Himsat-owned tests needed to prove that callers cannot silently weaken requested scope or presence policy.

B103 must preserve the platform differences frozen by the reviewed design. It must not claim universal `APP_EXCLUSIVE`, user-presence, atomic protected-state, or hardware-backed behavior when a provider cannot prove it. Unsupported requested policy must fail closed.

B103 does not authorize Apple/Android/Windows/Linux native secure-store adapters, entropy or cryptographic execution, VRK generation, B104 key hierarchy/domain separation/secret lifetime behavior, B105 concrete DB/blob post-lock I/O proof, SQLCipher/database implementation, freshness persistence, backup/restore/rotation/deletion, Specification 005 media behavior, new dependency adoption, or donor-code adoption.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> P011/B101 frontier                       CANONICAL_QUALIFIED_WITH_HISTORICAL_TRANSPORT_EVIDENCE_GAP
  -> B101 portable vault contracts            CANONICAL_QUALIFIED_WITH_HISTORICAL_TRANSPORT_EVIDENCE_GAP
  -> B101/B102 forward-repair reconciliation  CANONICAL_QUALIFIED
  -> B102 revocable keyed-handle lease        CANONICAL_QUALIFIED
  -> B102/B103 state reconciliation           ACTIVE
  -> B103 SecretProtector behavior            NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B104-B105 portable behavior              BLOCKED_PENDING_PRIOR_LEAVES
  -> B201-B206 crypto envelope foundation     BLOCKED_PENDING_PRIOR_LEAVES
  -> B301-B307 encrypted structured store     BLOCKED_PENDING_PRIOR_LEAVES
  -> B401-B406 platform protectors            BLOCKED_PENDING_PRIOR_LEAVES
  -> B501-B506 freshness/backup/rotation      BLOCKED_PENDING_PRIOR_LEAVES
  -> exact implementation review/R3           BLOCKED_PENDING_IMPLEMENTATION
  -> Specification 004 closeout               BLOCKED_PENDING_ALL_PRIOR_GATES
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
specs/004-vault-key-crypto/provider-package-source-license-review-remediation.md
specs/004-vault-key-crypto/provider-adoption-closure-evidence.md
specs/004-vault-key-crypto/provider-adoption-final-evidence.md
specs/004-vault-key-crypto/b101-portable-contracts-final-evidence.md
specs/004-vault-key-crypto/b102-revocable-lease-final-evidence.md
governance/provenance/policy.json
governance/provenance/registry.json
governance/generated/sbom.json
THIRD_PARTY_NOTICES.md
```

## Preserved residual risks

The reviewed Specification 004 limits remain unchanged:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel, firmware, hardware, and physical-memory compromise remain outside this boundary;
- ciphertext sizes, object counts, filesystem state, and operation/upload/backup/rotation timing can leak metadata;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove an authenticated backup is globally newest without a prior trusted freshness anchor;
- crypto-erasure does not prove physical-media, snapshot, provider-copy, or user-copy erasure;
- a detached recovery-enabled backup remains usable by a holder of the recovery passphrase;
- platforms unable to prove protected persistence and atomic freshness state must fail `UnsupportedPolicy`; rollbackable-file/plaintext fallback is prohibited.

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- `NOT RUN`, unavailable, manual-review, unknown, absent, skipped, billing-blocked, neutral, and self-review are never independent PASS evidence.
- Negative evidence remains evidence and must not be hidden by later approval.
- No custom cipher, MAC, KDF, PRNG, or protocol is authorized.
- No plaintext key-file fallback is authorized.
- No donor code may be copied without exact machine-readable provenance plus bounded adoption authority.
- Model/asset/data licensing is independent of software-engine licensing.
- Each implementation leaf must remain independently bounded and exact-head qualified.
- Specification 004 is not complete until all implementation, adversarial/platform, independent implementation review, reconciliation, guarded merge, post-merge, and closeout gates are proven.