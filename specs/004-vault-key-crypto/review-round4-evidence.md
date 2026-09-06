# Specification 004A — Round 4 Review Evidence

## Exact independent-review evidence

```text
BASE_CANONICAL_SHA = c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6
REVIEW_ONLY_PR = 27
REVIEWER = coderabbitai[bot]
REVIEW_COMMENT = 5562237044
REVIEWED_SHA = c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6
REVIEW_DISPOSITION = CHANGES_REQUIRED
BLOCKING_FINDINGS = B019-1
NON_BLOCKING_RECOMMENDATIONS = NONE
D001_D018_STATUS = RESOLVED_NO_REGRESSION
B019_STATUS = PARTIAL_B019_1_REMAINS
B020_STATUS = RESOLVED
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED
SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED
```

The review was substantive and tied to the exact canonical SHA above. It independently inspected the round-3 normative amendment, relevant Specification 004 security text, review lineage, and exact repository delta. It also independently checked the envelope arithmetic.

The review did not treat CI, R3, summaries, skipped output, rate-limit output, or older-SHA reviews as approval evidence.

## Resolved scope retained by the review

The reviewer found:

- B020 resolved by explicit protected `UNINITIALIZED`/`PRESENT(FreshnessAnchor)` state, missing-item-not-genesis behavior, atomic protected-state creation, serialized genesis compare-and-set, reread verification, fresh-device ordering, and fail-closed unsupported-policy behavior;
- recovery and bounded-blob v1 serialization, size bounds, AAD, parser, and no-plaintext-before-authentication rules resolved;
- D001-D018 substantively resolved with no identified regression;
- no new non-blocking recommendation.

These conclusions are historical evidence for the reviewed SHA only. The security-semantic remediation below requires a new exact-canonical review before it can become final approval evidence.

## B019-1 — remaining blocking ambiguity

The reviewer identified one remaining normative ambiguity in the manifest inventory contract for `GENERIC_ARTIFACT_BLOB`.

The prior text did not unambiguously require:

1. `ciphertext_length` to be the byte length of the **complete canonical bounded-blob envelope**, including the 107-byte public header and ciphertext/tag;
2. `ciphertext_sha256` to hash exactly that same complete envelope byte range from the first byte of `domain("HIMSAT/BLOB/ENVELOPE/v1")` through the final AEAD tag byte;
3. the manifest verifier to compare both values against the exact stored object before accepting the inventory entry.

The ambiguous slash wording in the previous `ciphertext_sha256` description could permit a non-conforming implementation to inventory only ciphertext-plus-tag while storing/parsing the public header separately.

That ambiguity is blocking because different implementations could disagree about the canonical stored-object boundary, inventory hashes, length checks, and corruption/transplant detection.

## Remediation

`round4-blob-inventory-contract.md` is a controlling normative Specification 004A amendment for `GENERIC_ARTIFACT_BLOB` inventory semantics. It freezes:

- the exact complete stored-object byte boundary;
- full-envelope `ciphertext_length` semantics and the inclusive 123..67,108,987 range;
- full-envelope SHA-256 input bytes;
- exclusion of filenames/filesystem/provider/transport metadata and exclusion of subset/re-serialized representations;
- verification of actual length, full-envelope hash, canonical parsing, `VaultId`, `ArtifactId`, key generation, and envelope length before inventory acceptance;
- `CorruptOrTampered` fail-closed behavior with no plaintext release;
- negative evidence for header omission, payload-only length/hash, prepend/append, context mismatch, and subset acceptance attempts.

The amendment changes no `STRUCTURED_STORE` semantics and adopts no crypto, SQLCipher, Cargo, native, model, dataset, asset, or donor dependency.

## Residual risks retained

The round-4 remediation does not weaken or erase the review's residual risks:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel, firmware, hardware, and physical-memory compromise remain outside this boundary;
- ciphertext sizes, counts, filesystem state, and operation/upload/rotation timing can leak metadata;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove an authenticated backup is globally newest without a prior trusted freshness anchor;
- crypto-erasure does not prove physical-media, snapshot, provider-copy, or user-copy erasure;
- detached recovery-enabled backups remain usable by a holder of the recovery passphrase;
- platform freshness guarantees remain unavailable on targets that cannot prove the required protected persistence and atomicity and those targets must fail with `UnsupportedPolicy`.

## Required successor evidence

This file is not approval and does not open 004P.

The round-4 remediation must be exact-head qualified by CI and Diffcipline R3, merged with explicit expected-head protection after live reconciliation, post-merge qualified on the exact canonical merge SHA, and then receive a new substantive independent exact-SHA security review with no unresolved blocking finding.

Only after that final design review is recorded canonically may dependency/provenance selection begin.
