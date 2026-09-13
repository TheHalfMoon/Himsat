# B505A Portable Backup Provider-Privacy Conflict Evidence

## Purpose

This record preserves the exact reason B505 implementation was stopped before product code and re-bound through a security-semantic design amendment.

```text
DISCOVERY_CANONICAL_MAIN = 1f6cf11a0df427521adfd1d728c78483e09285d1
DISCOVERY_TREE = 438b690e7699abb92b4bb9a9fa175f4a6794ce2b
B504_B505_RECONCILIATION_PR = 122
B504_B505_RECONCILIATION_HEAD = 0f001979360ab65168a51d1adaa339cdfe96307a
B504_B505_RECONCILIATION_MERGE = 1f6cf11a0df427521adfd1d728c78483e09285d1
POSTMERGE_CI = 34733251420_SUCCESS_ATTEMPT_1
POSTMERGE_R3 = 34733251409_SUCCESS_ATTEMPT_1
Q009 = UNSATISFIED
```

PR #122's exact final head had already passed pull-request CI `34732381630` and R3 `34732381732` before guarded merge. The canonical merge then passed the push-triggered post-merge runs above. Live GitHub truth therefore resolves the prior conditional B504/B505 reconciliation state to canonical-qualified.

## Blocking conflict

The B505 leaf was originally described as opaque-name portable backup/restore under D013. Re-reading the exact canonical contracts before implementation exposed a provider-boundary conflict:

- canonical B202 public envelope bytes contain `VaultId`, `ArtifactId`, and `key_generation`;
- canonical B204 recovery envelope bytes contain `VaultId`, `key_generation`, and public recovery fields;
- canonical B501 manifest envelope bytes contain `VaultId`, `key_generation`, and `freshness_epoch`;
- Round 4 requires the exact complete B202 envelope bytes to remain the stored-object/hash boundary, so stripping or rewriting the inner B202 header is non-conforming;
- D013 simultaneously forbids deliberate provider exposure of `VaultId`, `ArtifactId`, logical IDs, names, titles, and related semantic metadata.

Direct provider upload of these canonical inner envelopes would therefore violate the reviewed provider-view contract even if provider keys themselves were opaque.

The conflict changes security semantics. B505 product implementation was not started. The smallest forward-only remedy is `round5-portable-backup-provider-privacy-contract.md`, which preserves every prior inner format byte-for-byte and adds an outer provider-privacy boundary.

## Structured-store diagnostic

A temporary, uncommitted Windows diagnostic copied the existing B306 SQLCipher fixture, built the exact canonical provider closure, read persisted SQLCipher bytes, printed only file length/public prefix diagnostics, and removed itself after execution.

Observed result on the unchanged canonical code:

```text
B505_SQLCIPHER_LENGTH=8192
B505_SQLITE_MAGIC_VISIBLE=false
TEST_RESULT=1_PASSED
```

The observed 64-byte prefix was high-entropy-looking ciphertext and contained no SQLite magic. This is supporting diagnostic evidence only. It does **not** prove a provider-safe SQLCipher format, does not qualify all sizes/builds/platforms, and is not used as the B505 privacy boundary.

Round 5 deliberately outer-encrypts SQLCipher payload chunks in the same opaque object format as manifest/blob chunks. That choice eliminates dependency on unreviewed provider-visible SQLCipher-format assumptions and keeps provider qualification uniform.

The temporary probe file is absent from the working tree and is not part of the remediation diff.

## Remediation scope

The Round 5 remediation is docs/design/evidence/state only. It must not contain product Rust/native code, Cargo or native dependency changes, provenance adoption, generated SBOM/notices changes, workflows, donor material, models, datasets, provider SDKs, B506 deletion behavior, Specification 005 work, or release/FIPS/compliance claims.

The design uses only already-reviewed primitive families but introduces new domain-separated backup key uses and exact outer formats. Those semantics require exact-revision independent crypto/security review before implementation.

## Required disposition

B505 implementation authority remains `NONE` until the exact Round 5 canonical revision:

1. passes exact-head CI and Diffcipline R3;
2. is merged with explicit expected-head protection after live reconciliation;
3. has exact canonical parent/tree and push-triggered post-merge CI/R3 proven;
4. receives a genuinely independent substantive crypto/security review covering the complete Round 5 contract;
5. has every blocking finding resolved on the exact final security-semantic revision.

Owner review, local diagnostics, CI/R3, CodeRabbit automation, Cubic, Qodo, or other automation are not represented as Q009 and do not satisfy the later final implementation-review gate.
