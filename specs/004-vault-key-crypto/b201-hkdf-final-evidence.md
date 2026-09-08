# B201 HKDF-SHA-256 Final Evidence

## Disposition

```text
TASK = B201
DISPOSITION = CANONICAL_CLOSED_PENDING_THIS_RECONCILIATION
IMPLEMENTATION_PR = 50
IMPLEMENTATION_BASE = 33fc791443d25dba0ed7a5958710cb52ebb5bfad
FINAL_HEAD = 3c1706dfd0dfc7c745e19b81d19f31e4beb39e38
CANONICAL_MERGE = 3785561963b6eadab219b330367a9b6295755939
POSTMERGE_CI = 34174517394_SUCCESS
POSTMERGE_R3 = 34174517397_SUCCESS
```

This record closes only the reviewed B201 HKDF-SHA-256 purpose-key derivation leaf. Live GitHub truth remains authoritative over this file.

## Authority entering B201

B201 implementation authority became available only after the B105/B201 reconciliation PR #49 exact head `267351bcad78434b612c929d3165bc03ad1fb08c` passed CI `34171637803` and R3 `34171637800`, was merged with explicit `expected_head_sha = 267351bcad78434b612c929d3165bc03ad1fb08c` as canonical merge `33fc791443d25dba0ed7a5958710cb52ebb5bfad`, had exact parentage `73b13d38ac3e34143c813bda679e6e96ce01762e` + `267351bcad78434b612c929d3165bc03ad1fb08c`, and passed exact push-triggered CI `34172683259` and R3 `34172683357`.

The accepted B201 branch `feat/004b2-b201-hkdf` started directly from that exact canonical SHA. The canonical task ledger had not marked `B105R001`-`B105R003` complete even though those gates were proven by live GitHub evidence; this reconciliation repairs that state forward-only without rewriting history.

## Exact implementation surface

The accepted PR #50 diff changed exactly:

```text
crates/himsat-core/src/vault_keys.rs
```

No Cargo manifest, lockfile, dependency version/feature, provenance registry, generated SBOM/notices, workflow, donor-code, native-provider, SQLCipher, or Specification 005 file changed.

## Implemented behavior

B201 executes the already reviewed HKDF-SHA-256 purpose-key derivation contract using the already adopted `hkdf = 0.13.0` and `sha2 = 0.11.0` providers.

For every derivation:

- input key material is the already-owned 32-byte VRK;
- salt is exactly the raw 16-byte `VaultId`;
- `info` is exactly the frozen purpose-domain ASCII bytes followed by `u64be(key_generation)`;
- output is exactly 32 bytes; and
- the returned secret is represented as `OwnedKeyMaterial`, preserving its owned-buffer zeroization and redacted-debug behavior.

The exact v1 domains remain:

```text
HIMSAT/004/STRUCTURED/v1
HIMSAT/004/BLOB/v1
HIMSAT/004/MANIFEST/v1
```

B201 does not claim erasure of provider-internal state, compiler/runtime/register/allocator copies, swap/pagefile content, crash/core dumps, kernel memory, or physical memory.

## Deterministic vectors and separation proof

The implementation hard-codes deterministic expected outputs independently recomputed outside the Rust provider for one fixed VRK, VaultId, and key generation:

```text
STRUCTURED = 0a6857ca8e7804d89a825e7e5bc515bb4cad0e87f2ee6d0b67e54f0313d0808f
BLOB       = 602177c7d772ff6a69462540eb6612b105ec9b9f18115a024c3a410b0008e476
MANIFEST   = 19540379cb9fd9b63cad8f3be133c0a985339bcfeb323c26b1bd95e20202d019
```

The tests also prove that changing purpose, VaultId, or key generation changes the derived output. These tests verify domain separation without expanding B201 into envelope or nonce behavior.

## Exact-head qualification

Final implementation head `3c1706dfd0dfc7c745e19b81d19f31e4beb39e38` passed:

```text
CI = 34173364034_SUCCESS
R3 = 34173364032_SUCCESS
COMPARE = 1_AHEAD_0_BEHIND
CHANGED_FILES = 1
```

Observed exact-head workspace tests included:

```text
himsat-core = 36 passed; 0 failed
himsat-events = 10 passed; 0 failed
```

The B201 deterministic-vector and purpose/vault/generation separation tests were observed PASS. Formatting, clippy with warnings denied, registered dependency closure, provenance validation/generated closure, SpecGrain pinned-state validation, Diffcipline R2/R3, and negative controls also passed.

Immediately before merge, live `main` remained exact base `33fc791443d25dba0ed7a5958710cb52ebb5bfad`; PR #50 remained open and mergeable; the compare remained one ahead and zero behind; and the exact diff remained only `crates/himsat-core/src/vault_keys.rs`. There were zero submitted reviews and zero review threads. Qodo billing-blocked output and CodeRabbit automatic-skip output were not treated as PASS. Durable final reconciliation comment: `5577286248`.

No failed B201 implementation CI run preceded the accepted head. This does not erase negative evidence from earlier Specification 004 leaves.

## Guarded canonical transport

PR #50 was merged only with:

```text
EXPECTED_HEAD_SHA = 3c1706dfd0dfc7c745e19b81d19f31e4beb39e38
MERGE_METHOD = merge
MERGED = true
MERGE_SHA = 3785561963b6eadab219b330367a9b6295755939
```

Durable guarded-transport result comment: `5577371979`.

Canonical merge parentage is exact:

```text
PARENT_1 = 33fc791443d25dba0ed7a5958710cb52ebb5bfad
PARENT_2 = 3c1706dfd0dfc7c745e19b81d19f31e4beb39e38
```

The canonical merge tree is `9c8595973b2007428ee0d3b1aab122124b838ce7`, matching the qualified B201 implementation tree.

## Exact post-merge qualification

Push-triggered qualification on exact canonical merge `3785561963b6eadab219b330367a9b6295755939` completed successfully:

```text
CI = 34174517394_SUCCESS
R3 = 34174517397_SUCCESS
```

The Windows Rust path completed formatting, lint, tests, and registered dependency closure successfully before overall CI reached terminal SUCCESS. Durable post-merge evidence comment: `5577459951`.

## Historical evidence limitation preserved

`P011` remains unchecked. Historical PR #37 expected-head request-body evidence is still not reconstructible post hoc and is not retroactively promoted to PASS.

## Scope not claimed

B201 does not implement or claim:

- B202 bounded-blob XChaCha20-Poly1305 envelope execution/parsing;
- B203 OS-CSPRNG nonce generation, reservation, retry, restore, or collision behavior;
- B204 recovery Argon2id/envelope execution;
- B205 aggregate adversarial envelope qualification;
- SQLCipher integration or encryption-active proof — B301+;
- Apple/Android/Windows/Linux native protector adapters — B401+;
- protected freshness persistence, backup/restore, full rotation, or deletion — B501+;
- Specification 005 media streaming/journal behavior;
- new dependency or donor adoption;
- release, FIPS, or compliance qualification.

## B202 rebound boundary

B202 may begin only after this B201/B202 reconciliation itself is exact-head qualified, reconciled against live `main`, merged with explicit expected-head protection, parentage-proven, and exact-post-merge CI/R3 qualified.

B202 is bounded to the already reviewed versioned `GENERIC_ARTIFACT_BLOB` XChaCha20-Poly1305 envelope contract: exact public bytes, exact v1 AAD, checked parser arithmetic and bounds, 64 MiB plaintext ceiling, and no plaintext release before successful authentication. B202 must accept an explicit 24-byte nonce supplied by its caller; B203 owns production OS-CSPRNG nonce generation/reservation/retry/restore/collision semantics. B202 must not absorb B203 or later recovery, SQLCipher, native-protector, freshness, backup, rotation, deletion, Specification 005, dependency-adoption, or donor-code work.