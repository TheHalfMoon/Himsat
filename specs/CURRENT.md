# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_FINAL_DESIGN_RECONCILIATION_FOR_004P
ACTIVE_SPECIFICATION = 004-vault-key-crypto
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_002_DISPOSITION = CLOSED_CANONICAL
SPEC_003_DISPOSITION = CLOSED_CANONICAL
SPEC_003_CLOSEOUT_MERGE = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
SPEC_003_POST_CLOSEOUT_CI = 34047579266_SUCCESS
SPEC_004_SHAPING_MERGE = 384608c8fc13531c399f3726caa5022eb3612aa2
SPEC_004_FIRST_REMEDIATION_MERGE = 6d1bbc9b55690939833917eebd447c361627f48a
SPEC_004_ROUND2_REMEDIATION_MERGE = cb8511c1b420c58f714768c3561e74f04f026b3a
SPEC_004_LEDGER_RECONCILIATION_MERGE = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
SPEC_004_ROUND3_REMEDIATION_HEAD = 4ee8f83975dd074003df2d151e80d56bc5f1c005
SPEC_004_ROUND3_PREMERGE_CI = 34059290985_SUCCESS
SPEC_004_ROUND3_PREMERGE_R3 = 34059290977_SUCCESS
SPEC_004_ROUND3_CANONICAL_MERGE = c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6
SPEC_004_ROUND3_POSTMERGE_CI = 34059426441_SUCCESS
SPEC_004_ROUND3_POSTMERGE_R3 = 34059426437_SUCCESS
SPEC_004_ROUND3_FINAL_REVIEW_PR = 27
SPEC_004_ROUND3_FINAL_REVIEW_COMMENT = 5562237044
SPEC_004_ROUND3_FINAL_REVIEW_DISPOSITION = CHANGES_REQUIRED_B019_1
SPEC_004_ROUND4_REMEDIATION_HEAD = 81587ab6af6ad88a1f82114dfd5ddd78767a6d47
SPEC_004_ROUND4_PREMERGE_CI = 34060963107_SUCCESS
SPEC_004_ROUND4_PREMERGE_R3 = 34060963141_SUCCESS
SPEC_004_ROUND4_EXPECTED_HEAD_GUARD = PROVEN
SPEC_004_ROUND4_CANONICAL_MERGE = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
SPEC_004_ROUND4_POSTMERGE_CI = 34061056655_SUCCESS
SPEC_004_ROUND4_POSTMERGE_R3 = 34061056698_SUCCESS
SPEC_004_FINAL_REVIEW_PR = 29
SPEC_004_FINAL_REVIEW_REQUEST = 5562291428
SPEC_004_FINAL_REVIEW_RESPONSE = 5562296249
SPEC_004_FINAL_REVIEWED_SHA = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
SPEC_004_FINAL_REVIEW_DISPOSITION = APPROVE
SPEC_004_FINAL_BLOCKING_FINDINGS = NONE
SPEC_004_FINAL_NON_BLOCKING_RECOMMENDATIONS = NONE
SPEC_004_B019_1 = RESOLVED
SPEC_004_B020 = RESOLVED
SPEC_004_D001_D018 = RESOLVED_NO_REGRESSION
SPEC_004A_DESIGN_AUTHORITY = APPROVED_EXACT_CANONICAL
DEPENDENCY_ADOPTION_AUTHORITY = 004P_BOUNDED_SELECTION_ONLY_AFTER_THIS_RECONCILIATION_IS_CANONICAL
SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED_PENDING_004P_PROVENANCE_CLOSURE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub truth proves canonical `main` remains exact SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` at the start of this reconciliation leaf.

Specification 003 is `CLOSED_CANONICAL`. Specification 004 design shaping and four rounds of review/remediation produced the exact canonical design now under final ledger reconciliation.

## Final Specification 004A design disposition

The final substantive independent review on review-only PR #29 examined exact canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned in comment `5562296249`:

```text
REVIEWED_SHA = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
DISPOSITION = APPROVE
BLOCKING_FINDINGS = NONE
NON_BLOCKING_RECOMMENDATIONS = NONE
B019_1_STATUS = RESOLVED
B020_STATUS = RESOLVED
D001_D018_STATUS = RESOLVED_NO_REGRESSION
```

The reviewer explicitly stated that it examined the exact canonical SHA and did not use CI, R3, summaries, self-review, or older-SHA reviews as approval evidence.

`round3-normative-contracts.md` remains controlling for the round-3 recovery/bounded-blob/freshness/rotation contracts. `round4-blob-inventory-contract.md` is controlling for `GENERIC_ARTIFACT_BLOB` manifest inventory semantics and requires full canonical-envelope length/hash verification before inventory acceptance and plaintext release.

The durable final-review record is `specs/004-vault-key-crypto/review-round4-final-evidence.md`.

## Preserved negative and conflict evidence

Negative evidence remains part of the canonical design lineage and is not erased by the final approval.

- PR #12: 16 actionable findings D001-D016.
- PR #18: D017-D018 remained blocking.
- PR #24 comment `5561981955`: B019-B020 remained blocking.
- PR #25 initial approval was explicitly withdrawn after focused contradiction reconciliation; response `5562034608` returned `CHANGES_REQUIRED` for B019-B020.
- PR #27 comment `5562237044`: B020 and D001-D018 were resolved, but B019-1 remained blocking.
- PR #29 comment `5562296249`: B019-1 resolved, B020 resolved, D001-D018 resolved with no regression, no blocking or non-blocking findings.

## Preserved residual risks

The final independent review preserves these limits:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel, firmware, hardware, and physical-memory compromise remain outside this boundary;
- ciphertext sizes, object counts, filesystem state, and operation/upload/backup/rotation timing can leak metadata;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove an authenticated backup is globally newest without a prior trusted freshness anchor;
- crypto-erasure does not prove physical-media, snapshot, provider-copy, or user-copy erasure;
- a detached recovery-enabled backup remains usable by a holder of the recovery passphrase;
- platforms unable to prove protected persistence and atomic freshness state must fail `UnsupportedPolicy`; rollbackable-file/plaintext fallback is prohibited.

## Active objective

Canonicalize this final design-review reconciliation, then execute 004P dependency/provenance selection in dependency order:

1. select exact established crypto provider/library strategy;
2. select exact SQLCipher core/binding/provider/build strategy if SQLCipher remains selected;
3. inventory every direct/transitive Cargo/native dependency and immutable source/checksum identity;
4. verify controlling licenses/notices under `governance/provenance/policy.json`;
5. register the complete adopted package/native closure and deterministic SBOM/notices before accepting unregistered dependency bytes;
6. prove the resulting dependency/native graph contains no unregistered, checksum-mismatched, denied, unknown-license, or unresolved-manual component;
7. only after P001-P006 close, re-bound the smallest 004B implementation leaf from live canonical truth.

## 004P authority boundary

After this reconciliation itself becomes canonical and qualifies under exact-head/post-merge CI/R3, 004P may:

- inspect and select exact upstream package/native candidates;
- freeze immutable repository/tag/commit/package/checksum identities;
- select exact feature/provider/build strategies;
- write provenance decisions, registry entries, deterministic SBOM/notices, and dependency-closure evidence;
- reject candidates that cannot satisfy provenance/security/platform requirements.

004P must not:

- treat research identities as adopted dependencies;
- accept unregistered external Cargo/native bytes into the canonical lockfile/build graph before the machine adoption boundary is satisfied;
- begin 004B implementation before the exact provenance closure is complete;
- weaken license/provenance policy to make a preferred dependency fit;
- copy donor code or adopt model/data/asset material under this authority;
- make release, FIPS, compliance, privacy-superiority, or benchmark-superiority claims.

The Specification 002 provenance registry remains the machine adoption boundary.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> final design-review reconciliation       ACTIVE
  -> 004P provider/provenance choice          NEXT_AFTER_RECONCILIATION
  -> 004B encrypted storage foundation        BLOCKED_PENDING_004P
  -> exact implementation review/R3           BLOCKED_PENDING_004B
  -> closeout                                  BLOCKED_PENDING_ALL_PRIOR_GATES
  -> 005 crash-safe media journal/chunk store BLOCKED_PENDING_SPEC_004_CLOSEOUT
```

## Active artifacts

```text
specs/004-vault-key-crypto/spec.md
specs/004-vault-key-crypto/plan.md
specs/004-vault-key-crypto/tasks.md
specs/004-vault-key-crypto/review-evidence.md
specs/004-vault-key-crypto/review-round2-evidence.md
specs/004-vault-key-crypto/review-round3-evidence.md
specs/004-vault-key-crypto/review-round4-evidence.md
specs/004-vault-key-crypto/review-round4-final-evidence.md
specs/004-vault-key-crypto/round3-normative-contracts.md
specs/004-vault-key-crypto/round4-blob-inventory-contract.md
docs/research/2026-09-06-vault-crypto-foundation.md
governance/provenance/policy.json
governance/provenance/registry.json
```

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- `NOT RUN`, unavailable, manual-review, unknown, absent, skipped, billing-blocked, and self-review are never independent PASS evidence.
- Negative evidence remains evidence and must not be hidden by a later approval.
- No custom cipher/MAC/KDF/PRNG/protocol is authorized.
- No plaintext key-file fallback is authorized.
- No donor code may be copied without exact machine-readable provenance plus bounded adoption authority.
- Model/asset/data licensing is independent of software-engine licensing.
- Design approval does not approve implementation; security-semantic changes invalidate the final design approval for the changed surface.
