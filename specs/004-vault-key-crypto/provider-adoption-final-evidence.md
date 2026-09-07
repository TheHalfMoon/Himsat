# Specification 004P — Canonical Adoption Final Evidence

## Purpose

This record closes the Specification 004P provider/provenance decision from exact live GitHub evidence and re-bounds the next implementation frontier. It does not implement Specification 004B and does not authorize any implementation leaf broader than B101.

## Canonical adoption

```text
PRE_ADOPTION_MAIN = 0e6a3422bfe4a18a248d66027f5f538fd68542d3
ADOPTION_PR = 36
ADOPTION_HEAD = ee753debb23ac5a925a0736cec116994166953c5
ADOPTION_DIFF = 16_FILES_+5419_-48
PREMERGE_CI = 34142484081_SUCCESS
PREMERGE_R3 = 34142484053_SUCCESS
INDEPENDENT_REVIEW_COMMENT = 5573320081
INDEPENDENT_REVIEW_DISPOSITION = APPROVE
INDEPENDENT_REVIEW_BLOCKING_FINDINGS = NONE
EXPECTED_HEAD_GUARD = PROVEN
CANONICAL_ADOPTION_MERGE = a4d32ee93e0ab95af8376ba0ca09e248070c5924
POSTMERGE_CI = 34144623816_SUCCESS
POSTMERGE_R3 = 34144623829_SUCCESS
```

The guarded merge used `expected_head_sha = ee753debb23ac5a925a0736cec116994166953c5`. Canonical `main` was re-read after the merge and resolved to `a4d32ee93e0ab95af8376ba0ca09e248070c5924` before this reconciliation branch was created.

## Independent review disposition

The final substantive CodeRabbit review examined exact adoption head `ee753debb23ac5a925a0736cec116994166953c5` and returned:

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

Older reviews and failed/blocked attempts remain negative evidence and are not erased by this approval.

## P001–P006 disposition

Live canonical truth now closes all six dependency/provenance tasks:

```text
P001 = CANONICAL_CLOSED
P002 = CANONICAL_CLOSED
P003 = CANONICAL_CLOSED
P004 = CANONICAL_CLOSED
P005 = CANONICAL_CLOSED
P006 = CANONICAL_CLOSED
DEPENDENCY_BYTES_ADOPTED_ON_MAIN = YES
```

Controlling evidence remains split by responsibility:

- `provider-provenance-selection.md` and `provider-resolution-evidence.md` control the selected crypto/SQLCipher strategy;
- `provider-package-source-license-closure.md` and `provider-package-source-license-review-remediation.md` control exact package/native source, checksum, license, and notice closure;
- `provider-adoption-closure-evidence.md`, the canonical lockfile, provenance registry v2, deterministic SBOM/notices, CI, R3, and PR #36 review control machine adoption and final closure.

The canonical graph contains the exact reviewed direct pins and the complete 41-package Cargo registry closure, with separately represented native SQLCipher, SQLite, and OpenSSL components. No broader donor, model, dataset, asset, release, FIPS, compliance, or superiority authority is created by this closeout.

## Re-bounded next leaf — B101 only

The smallest next implementation leaf is B101 from `tasks.md`:

```text
B101 = PORTABLE_VAULT_CONTRACT_SURFACE
RISK = R3
AUTHORITY = CONDITIONAL_ON_THIS_RECONCILIATION_CANONICAL_AND_POSTMERGE_QUALIFIED
```

### Scope in

B101 may add only provider-neutral reviewed contract types and trait/interface surface needed to represent:

- `VaultId` as exactly 16 bytes;
- non-zero key generation and freshness epoch identities;
- freshness-anchor identity/state values without persistence mechanics;
- lock-state and lease identity/state contract values without revocation implementation;
- actual access-scope capability classes;
- user-presence policy;
- hardware-backed capability reporting state;
- provider-neutral `SecretProtector` operation signatures and associated typed result/error surface only to the extent required to make the contract coherent and compilable.

### Scope out

B101 must not implement:

- random generation or entropy acquisition;
- VRK generation, storage, wrapping, unlocking, or plaintext secret handling;
- HKDF, Argon2id, XChaCha20-Poly1305, nonce generation, recovery, or envelope serialization;
- SQLCipher/database behavior;
- Apple, Android, Windows, or Linux native protector mechanics;
- freshness persistence, compare-and-advance behavior, backup, restore, rotation, or deletion;
- keyed-handle revocation behavior owned by B102+;
- Specification 005 media/journal behavior;
- donor-code adoption or new dependencies.

### Acceptance

Before B101 can become canonical:

1. re-read canonical `main` after this reconciliation merge and its post-merge CI/R3 qualification;
2. preserve the reviewed Specification 004A semantics exactly;
3. keep the implementation diff inside the repository's ordinary Diffcipline bounds without another adoption exception;
4. add Himsat-owned tests for type invariants and fail-closed constructors/validation that B101 itself owns;
5. run exact-head CI and R3;
6. reconcile exact diff, reviews/comments/threads, live `main`, and mergeability;
7. merge only with expected-head protection;
8. require post-merge CI and R3 before advancing to B102.

Design approval and 004P closure do not by themselves approve B101 code. Later implementation review obligations remain unchanged.

## Preserved residual risks and authority limits

The reviewed Specification 004 residual risks remain controlling, including compromise of an unlocked process, kernel/firmware/hardware compromise, observable ciphertext metadata/timing, offline recovery-passphrase attack, fresh-device inability to prove global newest backup without a trusted anchor, limits of crypto-erasure, and detached recovery-enabled backup retention.

Specification 005 remains blocked pending complete Specification 004 closeout. Release authority remains absent.