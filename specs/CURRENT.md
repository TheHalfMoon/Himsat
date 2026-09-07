# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004P_CLOSED_RECONCILING_B101_BOUND
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
DEPENDENCY_BYTES_ADOPTED_ON_MAIN = YES
NEXT_IMPLEMENTATION_LEAF = B101_PORTABLE_VAULT_CONTRACT_SURFACE
SPEC_004_IMPLEMENTATION_AUTHORITY = B101_ONLY_IF_THIS_RECONCILIATION_IS_CANONICAL_AND_POSTMERGE_QUALIFIED
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B101_ONLY_UNDER_THE_CONDITION_ABOVE
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

## Active objective

Canonicalize this state-only reconciliation without changing security semantics or product code. After this reconciliation itself is merged with expected-head protection and passes exact post-merge CI/R3, execute only B101 as the smallest re-bounded Specification 004B implementation leaf.

B101 is limited to portable contract surface for reviewed vault identity, non-zero generation/freshness values, lock/lease state representation, capability reporting, and provider-neutral `SecretProtector` interfaces. Its exact scope and exclusions are frozen in `provider-adoption-final-evidence.md`.

B101 does not authorize cryptographic operations, SQLCipher behavior, platform secure-store implementations, persistence, recovery, backup, rotation, deletion, media behavior, new dependencies, or donor-code adoption.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> 004P/B101 frontier reconciliation        ACTIVE
  -> B101 portable vault contracts            NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B102-B105 portable behavior              BLOCKED_PENDING_PRIOR_LEAVES
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
governance/provenance/policy.json
governance/provenance/registry.json
governance/generated/sbom.json
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