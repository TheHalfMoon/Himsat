# Specification 004A — Final Round-4 Independent Review Evidence

## Purpose

This file records the final substantive independent crypto/security review of the exact canonical Specification 004A design after the round-4 B019-1 remediation.

It is durable evidence for the reviewed SHA only. It does not approve implementation code, dependency bytes, provider builds, platform adapters, release claims, or Specification 005 work.

## Exact canonical review target

```text
REVIEW_ONLY_PR = 29
REVIEW_REQUEST_COMMENT = 5562291428
REVIEW_RESPONSE_COMMENT = 5562296249
REVIEWER = coderabbitai[bot]
REVIEWED_SHA = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
CANONICAL_MAIN_SHA_AT_REVIEW = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
ROUND4_REMEDIATION_PR = 28
ROUND4_REMEDIATION_HEAD = 81587ab6af6ad88a1f82114dfd5ddd78767a6d47
ROUND4_PREMERGE_CI = 34060963107_SUCCESS
ROUND4_PREMERGE_R3 = 34060963141_SUCCESS
ROUND4_EXPECTED_HEAD_GUARD = PROVEN
ROUND4_CANONICAL_MERGE = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
ROUND4_POSTMERGE_CI = 34061056655_SUCCESS
ROUND4_POSTMERGE_R3 = 34061056698_SUCCESS
```

PR #29 is review-only and MUST NOT be merged. Its head points directly to the exact canonical reviewed SHA and its base is the prior canonical SHA `c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6`, exposing only the round-4 remediation delta.

## Independent disposition

The substantive CodeRabbit response on PR #29 comment `5562296249` returned:

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

## B019-1 closure

The reviewer confirmed that `round4-blob-inventory-contract.md` is controlling for `GENERIC_ARTIFACT_BLOB` and closes the prior ambiguity by requiring:

- `manifest.ciphertext_length` to cover the complete canonical bounded-blob envelope;
- `manifest.ciphertext_sha256` to hash exactly the same complete canonical byte sequence;
- the covered range to begin at the first byte of `domain("HIMSAT/BLOB/ENVELOPE/v1")` and end at the final AEAD tag byte;
- actual stored length and SHA-256 verification before inventory acceptance;
- canonical parsing of those same bytes followed by `VaultId`, `ArtifactId`, key-generation, and total-length equality checks;
- any mismatch to fail as `CorruptOrTampered` with no plaintext release;
- payload-only, ciphertext/tag-only, reserialized, filesystem, provider, transport, and alternate-representation interpretations to remain prohibited.

The independent review also confirmed that the 107-byte public header, 16-byte AEAD tag, and complete-envelope range `123..67_108_987` are consistent with the controlling bounded-blob layout.

## Regression disposition

The final review found no regression in:

- B020 protected freshness genesis;
- D001-D018;
- recovery envelope rules;
- bounded-blob serialization and parser rules;
- AAD and nonce rules;
- rotation invariants;
- retained-manifest rules;
- fail-closed unsupported-platform behavior;
- residual-risk boundaries.

## Preserved residual risks

The final review preserved the following residual risks without weakening or hiding them:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel, firmware, hardware, and physical-memory compromise remain outside this boundary;
- ciphertext sizes, object counts, filesystem state, and operation/upload/backup/rotation timing can leak metadata;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove that an authenticated backup is globally newest without a prior trusted freshness anchor;
- crypto-erasure does not prove physical-media, snapshot, provider-copy, or user-copy erasure;
- a detached recovery-enabled backup remains usable by a holder of the recovery passphrase;
- a platform that cannot prove protected persistence and atomicity for freshness must return `UnsupportedPolicy` and must not use a rollbackable-file or plaintext fallback.

## Governance disposition

All known Specification 004A design blockers are resolved for exact canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98`.

This evidence closes the design-review precondition for 004P only.

```text
SPEC_004A_DESIGN_DISPOSITION = APPROVED_EXACT_CANONICAL
UNRESOLVED_DESIGN_BLOCKERS = NONE
004P_PROVIDER_PROVENANCE_SELECTION_AUTHORITY = OPEN_AFTER_CANONICAL_RECONCILIATION
004B_IMPLEMENTATION_AUTHORITY = BLOCKED_PENDING_004P_PROVENANCE_CLOSURE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Design approval does not approve implementation. Any later change to the reviewed security semantics invalidates this approval for the changed design and requires a new exact-SHA independent review.
