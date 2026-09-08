# B105 Post-Lock Handle I/O Final Evidence

## Disposition

```text
TASK = B105
DISPOSITION = CANONICAL_CLOSED_PENDING_THIS_RECONCILIATION
IMPLEMENTATION_PR = 48
IMPLEMENTATION_BASE = 623c0ad4207ecbbd5e404a2a32ba3cdb61006778
INITIAL_HEAD = 39d63bbf647f0f13aff1cefa9da0d7eb1a9fb119
FINAL_HEAD = f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf
CANONICAL_MERGE = 73b13d38ac3e34143c813bda679e6e96ce01762e
POSTMERGE_CI = 34170861437_SUCCESS
POSTMERGE_R3 = 34170861436_SUCCESS
```

This record closes only the reviewed B105 portable post-lock/post-revocation/fatal-failure database/blob I/O authorization proof. Live GitHub truth remains authoritative over this file.

## Authority entering B105

B105 implementation authority became available only after the B104/B105 state/evidence reconciliation in PR #47 was exact-head qualified, explicitly expected-head guarded, merged canonically as `623c0ad4207ecbbd5e404a2a32ba3cdb61006778`, parentage-proven, and exact-post-merge qualified by CI `34168619451` and R3 `34168619510`.

The accepted B105 branch started directly from that exact canonical SHA. No stale predecessor branch or pre-authority implementation was reused.

## Exact implementation surface

The accepted PR #48 final diff changed exactly:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_io.rs
```

No Cargo manifest, lockfile, provenance registry, generated SBOM/notices, workflow, donor-code, native-provider, or Specification 005 file changed.

## Implemented behavior

B105 adds provider-neutral lease-gated structured-database and bounded-blob handle wrappers. Every read or write obtains a B102 operation-scoped keyed-handle permit and retains that permit for the complete backend closure. A terminal lease therefore rejects a stale operation before the backend closure is called.

The exact implementation tests prove:

- live database and blob reads/writes reach their backends under an active permit;
- previously obtained database/blob handles reject reads and writes after explicit lock;
- the same stale handles reject after protector/session revocation;
- the same stale handles reject after fatal failure;
- the relevant rotation teardown path is terminal and rejects stale operations;
- rejected stale operations do not increment backend read/write counters;
- revocation publishes terminal state, rejects a second stale handle, and waits for an already-authorized database operation holding its permit to drain;
- backend errors remain distinct from lease authorization failures; and
- wrapper `Debug` output does not expose backend debug content.

This is an in-process authorization/lifetime boundary. It does not claim SQLCipher encryption, bounded-blob AEAD, or persistence behavior.

## Preserved negative evidence

Initial implementation head `39d63bbf647f0f13aff1cefa9da0d7eb1a9fb119` triggered CI `34169460825`. Ubuntu and Windows Rust jobs failed at formatting. Their downstream lint, tests, and registered-dependency-closure steps were skipped and are not PASS.

The failure was preserved durably in PR #48 comment `5576654626`. A forward-only formatting-only commit produced final head `f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf`; no force push, rebase, or history rewrite was used.

## Exact-head qualification

Final head `f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf` passed:

```text
CI = 34169567748_SUCCESS
R3 = 34169567620_SUCCESS
COMPARE = 2_AHEAD_0_BEHIND
```

The qualified matrix included formatting, clippy with warnings denied, workspace tests, registered dependency closure, provenance validation/generated closure, SpecGrain pinned-state validation, Diffcipline R2/R3, and negative-control verification. Observed workspace tests on the exact head included `34 passed; 0 failed` for `himsat-core` and `10 passed; 0 failed` for `himsat-events`.

Immediately before merge, PR #48 remained mergeable, `main` remained exact base `623c0ad4207ecbbd5e404a2a32ba3cdb61006778`, the compare remained ahead-only, and the exact diff remained the two paths above. There were zero submitted reviews and zero review threads. Qodo billing-blocked output and CodeRabbit automatic skip output were explicitly not treated as PASS. Durable final reconciliation comment: `5576822047`.

## Guarded canonical transport

PR #48 was merged only with:

```text
EXPECTED_HEAD_SHA = f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf
MERGE_METHOD = merge
MERGED = true
MERGE_SHA = 73b13d38ac3e34143c813bda679e6e96ce01762e
```

Durable guarded-transport result comment: `5576824739`.

Canonical merge parentage is exact:

```text
PARENT_1 = 623c0ad4207ecbbd5e404a2a32ba3cdb61006778
PARENT_2 = f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf
```

The canonical merge tree matches the qualified B105 implementation tree.

## Exact post-merge qualification

Push-triggered qualification on exact canonical merge `73b13d38ac3e34143c813bda679e6e96ce01762e` completed successfully:

```text
CI = 34170861437_SUCCESS
R3 = 34170861436_SUCCESS
```

The Windows Rust path also completed formatting, lint, tests, and registered dependency closure successfully before the overall CI workflow reached terminal SUCCESS.

## Residual-risk boundary

B105 does not claim complete memory erasure. Ordinary process abort or teardown can prevent cleanup code from running. Compiler/runtime/register/allocator copies, swap/pagefile content, crash or core dumps, kernel memory, physical memory/media, and secrets already observed by a fully compromised unlocked process remain outside this proof.

Future concrete database/blob providers must remain behind an equivalent lease gate and must not expose an ungated keyed backend path that bypasses B105 authorization semantics.

## Scope not claimed

B105 does not implement or claim:

- HKDF-SHA-256 execution or deterministic derivation vectors — B201;
- versioned bounded-blob XChaCha20-Poly1305 execution or envelope parsing — B202+;
- OS-CSPRNG nonce generation/reservation — B203;
- Argon2id recovery-envelope execution — B204+;
- SQLCipher integration or encryption-active proof — B301+;
- Apple/Android/Windows/Linux native protector adapters — B401+;
- protected freshness persistence, backup/restore, full rotation, or deletion — B501+;
- Specification 005 media behavior;
- new dependency or donor adoption;
- release, FIPS, or compliance qualification.

## B201 rebound boundary

B201 may begin only after this B105/B201 reconciliation itself is exact-head qualified, reconciled against live `main`, merged with explicit expected-head protection, parentage-proven, and exact-post-merge CI/R3 qualified.

B201 is bounded to the already reviewed and already provenance-adopted HKDF-SHA-256 derivation contract: raw 16-byte `VaultId` salt, exact purpose `info` domain plus `u64be(key_generation)`, 32-byte output, and deterministic independent test vectors. B201 must not absorb B202 or later envelope, nonce, recovery, SQLCipher, native-protector, freshness, backup, rotation, deletion, Specification 005, dependency-adoption, or donor-code work.
