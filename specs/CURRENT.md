# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B101_CANONICAL_RECONCILING_B102_BOUND
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
SPEC_004P_ADOPTION_HEAD = ee753debb23ac5a925a0736cec116994166953c5
SPEC_004P_PREMERGE_CI = 34142484081_SUCCESS
SPEC_004P_PREMERGE_R3 = 34142484053_SUCCESS
SPEC_004P_FINAL_REVIEW_COMMENT = 5573320081
SPEC_004P_FINAL_REVIEW_DISPOSITION = APPROVE
SPEC_004P_FINAL_BLOCKING_FINDINGS = NONE
SPEC_004P_EXPECTED_HEAD_GUARD = PROVEN
SPEC_004P_CANONICAL_MERGE = a4d32ee93e0ab95af8376ba0ca09e248070c5924
SPEC_004P_POSTMERGE_CI = 34144623816_SUCCESS
SPEC_004P_POSTMERGE_R3 = 34144623829_SUCCESS
P001 = CANONICAL_CLOSED
P002 = CANONICAL_CLOSED
P003 = CANONICAL_CLOSED
P004 = CANONICAL_CLOSED
P005 = CANONICAL_CLOSED
P006 = CANONICAL_CLOSED
P011 = CANONICAL_CLOSED
DEPENDENCY_BYTES_ADOPTED_ON_MAIN = YES
B101_DISPOSITION = CANONICAL_CLOSED
B101_PR = 38
B101_HEAD = ea82e1246095bb921dc9e7e40716076edbda41a6
B101_PREMERGE_CI = 34148418394_SUCCESS
B101_PREMERGE_R3 = 34148418341_SUCCESS
B101_CANONICAL_MERGE = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
B101_POSTMERGE_CI = 34149709088_SUCCESS
B101_POSTMERGE_R3 = 34149708977_SUCCESS
NEXT_IMPLEMENTATION_LEAF = B102_REVOCABLE_KEYED_HANDLE_LEASE
SPEC_004_IMPLEMENTATION_AUTHORITY = B102_ONLY_IF_THIS_RECONCILIATION_IS_CANONICAL_AND_POSTMERGE_QUALIFIED
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B102_ONLY_UNDER_THE_CONDITION_ABOVE
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

Specification 004P is closed by exact live evidence recorded in `provider-adoption-final-evidence.md`.

The final adoption PR #36 used exact head `ee753debb23ac5a925a0736cec116994166953c5`. Its pre-merge CI `34142484081` and R3 `34142484053` succeeded. The final substantive CodeRabbit response in comment `5573320081` reviewed that exact SHA and returned:

```text
DISPOSITION = APPROVE
BLOCKING_FINDINGS = NONE
NATIVE_EVIDENCE_REFERENCE_BLOCKER = RESOLVED
DIFFCIPLINE_REPOSITORY_POLICY_BLOCKER = RESOLVED
DIFFCIPLINE_PROOF_PARSER_BLOCKER = RESOLVED
DIFFCIPLINE_PROOF_TO_GIT_DIFF_BINDING_BLOCKER = RESOLVED
DIFFCIPLINE_COMMITTED_HEAD_BLOB_BINDING_BLOCKER = RESOLVED
004B_AUTHORITY_BOUNDARY = PRESERVED
```

PR #36 was merged with explicit expected-head protection. Canonical merge `a4d32ee93e0ab95af8376ba0ca09e248070c5924` then passed post-merge CI `34144623816` and R3 `34144623829` on that exact canonical SHA.

Therefore P001-P006 are `CANONICAL_CLOSED` and the selected dependency/native bytes are adopted on `main`. The machine provenance boundary remains registry v2 plus deterministic SBOM/notices and the registered-closure CI/R3 gates.

Older failed runs, blocking reviews, tooling incidents, and contradiction evidence remain part of the durable evidence lineage and are not erased by final closure.

## Canonical P011 and B101 disposition

P011 completed through PR #37. Exact reconciliation head `367348c2b11add2c15f7d25af92ca4679b2a19d1` passed CI `34145714886` and R3 `34145714971`, then merged canonically as `7ecba93ae0763a5165dd99ce0ec190cb106906de`. That exact canonical merge passed post-merge CI `34146939013` and R3 `34146938921`, activating B101-only authority.

B101 then completed through PR #38. The first implementation head failed R3 formatting only; the forward-only rustfmt repair produced exact final head `ea82e1246095bb921dc9e7e40716076edbda41a6`. That exact head passed CI `34148418394` and R3 `34148418341`.

PR #38 changed only `crates/himsat-core/src/lib.rs` and `crates/himsat-core/src/vault.rs`, with 666 additions and 2 deletions. `Cargo.toml`, `Cargo.lock`, provenance registry, generated SBOM/notices, workflows, native provider code, and donor material were unchanged. No submitted review or review thread existed; Qodo billing-blocked output and CodeRabbit auto-skip output were observed but were not counted as independent PASS evidence.

PR #38 merged canonically as `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c`, with parents `7ecba93ae0763a5165dd99ce0ec190cb106906de` and `ea82e1246095bb921dc9e7e40716076edbda41a6`. Exact post-merge CI `34149709088` and R3 `34149708977` both succeeded. B101 is therefore `CANONICAL_CLOSED`.

## Active objective

Canonicalize this state-only B101 closeout/B102 re-bound reconciliation without changing security semantics, dependency bytes, provenance entries, generated artifacts, workflows, or product runtime behavior.

After this reconciliation itself is merged with expected-head protection and passes exact post-merge CI/R3, execute only B102 as the next independently bounded Specification 004B implementation leaf.

B102 is limited to typed lock/unlock/protector/freshness behavior errors plus a revocable in-process `VaultLease`/keyed-handle authorization contract that is bound to one `VaultId` and `KeyGeneration`, reports active/revoked state, and fails closed after revocation. It may add Himsat-owned tests for allocation/state/revocation/idempotence/identity checks owned by this leaf.

B102 does not authorize cryptographic operations, entropy acquisition, VRK generation/wrapping/unlocking behavior, native platform protector mechanics, SQLCipher/database implementation, freshness persistence/CAS mechanics, key derivation/zeroization behavior owned by B104, DB/blob handle implementations and post-lock I/O proof owned by B105, backup/restore/rotation/deletion, Specification 005 media behavior, new dependencies, or donor-code adoption.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> 004P/B101 frontier reconciliation        CLOSED_CANONICAL
  -> B101 portable vault contracts            CLOSED_CANONICAL
  -> B101/B102 frontier reconciliation        ACTIVE
  -> B102 revocable keyed-handle lease        NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B103-B105 portable behavior              BLOCKED_PENDING_PRIOR_LEAVES
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
