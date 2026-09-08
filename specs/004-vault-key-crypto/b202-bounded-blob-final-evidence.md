# B202 Bounded-Blob Envelope Final Evidence

## Disposition

```text
TASK = B202
DISPOSITION = CANONICAL_CLOSED_PENDING_THIS_RECONCILIATION
IMPLEMENTATION_PR = 52
IMPLEMENTATION_BASE = 5ed2c3f79ef5af783c479871217e4aba0dcf0fd2
FINAL_HEAD = 5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510
CANONICAL_MERGE = 483857be2abf017c93fd94c210beef8080bc47fb
POSTMERGE_CI = 34225052523_SUCCESS
POSTMERGE_R3 = 34225052492_SUCCESS
```

This record closes only the reviewed B202 generic-artifact bounded-blob XChaCha20-Poly1305 v1 envelope leaf. Live GitHub truth remains authoritative over this file.

## Authority entering B202

B202 implementation authority became available only after the B201/B202 reconciliation PR #51 exact head `f79adcfe661f17eaee4ca0a96de8489abb5109ac` was merged as canonical `5ed2c3f79ef5af783c479871217e4aba0dcf0fd2`, with exact parentage `3785561963b6eadab219b330367a9b6295755939` + `f79adcfe661f17eaee4ca0a96de8489abb5109ac`, and exact push-triggered CI `34176163215` and R3 `34176163219` both reached terminal SUCCESS.

The accepted B202 branch `feat/004b2-b202-bounded-blob-envelope` started from that exact canonical SHA.

## Exact implementation surface

The accepted PR #52 diff changed exactly:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_blob.rs
crates/himsat-core/src/vault_keys.rs
```

No Cargo manifest, lockfile, dependency version/feature, provenance registry, generated SBOM/notices, workflow, donor-code, native-provider, SQLCipher, platform-protector, freshness/backup/rotation/deletion, or Specification 005 file changed.

## Implemented behavior

B202 executes only the already reviewed `GENERIC_ARTIFACT_BLOB` v1 envelope using the already adopted `chacha20poly1305 = 0.11.0` provider and the B201 bounded-blob purpose key.

The implementation:

- emits and parses the exact reviewed 107-byte public header;
- authenticates the exact reviewed 104-byte v1 AAD;
- encodes all reviewed integer fields big-endian;
- keeps the cryptographic `ArtifactId` boundary at exactly 16 raw bytes;
- enforces the 64 MiB (`67_108_864`) plaintext ceiling;
- enforces complete-envelope size `123..67_108_987` bytes with checked arithmetic;
- rejects invalid domain/version/suite/purpose, zero generation, declared-length mismatch, truncation, trailing bytes, requested vault/artifact/generation transplant, wrong key, ciphertext tampering, and nonce tampering;
- derives only the B201 `BoundedBlob` purpose key;
- keeps derived secret bytes behind a crate-internal one-operation borrow instead of adding a public raw-secret accessor;
- accepts an explicit caller-supplied 24-byte nonce and does not generate, reserve, retry, restore, or classify nonce collisions; and
- returns plaintext only after XChaCha20-Poly1305 authentication succeeds.

B203 owns production OS-CSPRNG nonce generation and the reviewed publication/reservation/collision lifecycle.

## Deterministic and adversarial evidence

The B202 test surface includes an independently recomputed deterministic complete-envelope vector plus tests covering:

- deterministic encryption/decryption;
- empty plaintext and minimum envelope size;
- checked 64 MiB boundary arithmetic without allocating a 64 MiB fixture;
- unsupported version/cipher-suite/object-purpose identifiers;
- malformed length, truncation, and trailing bytes;
- zero generation;
- ciphertext and nonce tampering;
- wrong VRK authentication failure; and
- vault/artifact/generation transplant rejection.

Final accepted-head test evidence was:

```text
himsat-core = 44 passed; 0 failed
himsat-events = 10 passed; 0 failed
```

## Preserved negative implementation lineage

B202 required multiple forward-only corrections. Every failed lineage remains NOT PASS and is preserved rather than erased or reclassified.

### Initial head

```text
HEAD = fa7ca6e9edee4301a6382adca10da3aea3106f49
CI = 34177306333_FAILURE_NOT_PASS
```

Ubuntu, macOS, and Windows Rust jobs failed at `cargo fmt --all -- --check`; their lint, tests, and registered dependency closure steps were skipped and remain NOT PASS. Durable PR comment: `5577799792`.

### Formatting-repair head

```text
HEAD = bf7ab08c480088bd5c286cf66d48ace70a7abb92
CI = 34223420708_FAILURE_NOT_PASS
```

Formatting passed, then Clippy rejected one test-only `vec!` allocation under `-D warnings`. Tests and registered dependency closure on the failed Rust job were skipped and remain NOT PASS. Durable PR comment: `5584785844`.

### Clippy-repair head

```text
HEAD = 6ddee1e41349d8dfe8cb373b4308f77157091853
CI = 34223784461_FAILURE_NOT_PASS
R3 = 34223784391_FAILURE_NOT_PASS
```

The failure was formatting-only after the test expression changed. The separate Diffcipline proof on this exact head independently showed Clippy PASS, `himsat-core` 44/44 PASS, `himsat-events` 10/10 PASS, provenance/generated-output validation PASS, and registered 004P dependency closure PASS. The top-level Rust matrix jobs still stopped at formatting, so their later steps remain skipped NOT PASS. Repository-owner COMMENT review `5141479842` preserves this evidence and is not an approval.

The final formatting-only successor `5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510` changed only the rustfmt-required assertion layout relative to `6ddee1e4...`.

## Exact-head qualification

Final implementation head `5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510` passed:

```text
CI = 34224041244_SUCCESS
R3 = 34224041193_SUCCESS
COMPARE = 4_AHEAD_0_BEHIND
CHANGED_FILES = 3
```

Exact-head CI proved Ubuntu/macOS/Windows formatting, lint, tests, and registered dependency closure SUCCESS; Diffcipline R2 SUCCESS; SpecGrain pinned-source validation SUCCESS; provenance validation/generated closure across supported runners SUCCESS; provenance adversarial self-test SUCCESS; and negative controls SUCCESS.

Immediately before merge, live `main` remained exact base `5ed2c3f79ef5af783c479871217e4aba0dcf0fd2`; PR #52 remained open and mergeable; the compare remained four ahead and zero behind; and the exact diff remained the three implementation paths listed above. There were zero review threads. Submitted repository-owner COMMENT reviews were evidence records and not approvals. Qodo billing-blocked and CodeRabbit auto-skip output were not treated as PASS. Durable final pre-merge reconciliation review: `5141534650`.

## Guarded canonical transport

Durable PR comment `5584971324` records the guarded merge result:

```text
EXPECTED_HEAD_SHA = 5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = 483857be2abf017c93fd94c210beef8080bc47fb
```

Canonical merge parentage is exact:

```text
PARENT_1 = 5ed2c3f79ef5af783c479871217e4aba0dcf0fd2
PARENT_2 = 5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510
MERGE_TREE = 8b8b7657ceb6d5bdd032727c95e4536847031aa2
```

## Exact post-merge qualification

Push-triggered qualification on exact canonical merge `483857be2abf017c93fd94c210beef8080bc47fb` completed successfully:

```text
CI = 34225052523_SUCCESS
R3 = 34225052492_SUCCESS
```

CI #131 completed Ubuntu, macOS, and Windows formatting, lint, tests, and registered dependency closure successfully. Diffcipline R2, SpecGrain, provenance validation/generated closure, provenance adversarial self-test, and negative controls also succeeded. Repository-owner COMMENT review `5141620618` records the durable post-merge qualification evidence and is not an independent approval.

## Historical evidence limitation preserved

`P011` remains unchecked. Historical PR #37 expected-head request-body evidence remains not reconstructible post hoc and is not retroactively promoted to PASS.

## Scope not claimed

B202 does not implement or claim:

- B203 OS-CSPRNG nonce generation, manifest/inventory reservation, retry/regeneration, restore behavior, or duplicate/collision rejection;
- B204 recovery Argon2id/envelope execution;
- B205 aggregate adversarial qualification beyond the bounded B202 tests above;
- SQLCipher integration or encryption-active proof — B301+;
- Apple/Android/Windows/Linux native protector adapters — B401+;
- protected freshness persistence, backup/restore, full rotation, or deletion — B501+;
- Specification 005 streaming/media journaling;
- new dependency or donor adoption;
- release, FIPS, or compliance qualification.

## B203 rebound boundary

B203 may begin only after this B202/B203 reconciliation itself is exact-head qualified, reconciled against live `main`, merged with explicit expected-head protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

B203 is bounded to the reviewed nonce lifecycle: fresh 24-byte OS-CSPRNG output per encryption attempt; fail-closed randomness errors; new nonce after ambiguous/failed publication; exact preservation when copying/restoring an already authenticated envelope; fresh nonce on re-encryption; canonical `(key_generation, nonce)` reservation for bounded blobs; collision discard/regeneration before publication; duplicate canonical nonce detection as `CorruptOrTampered`; and the corresponding reviewed manifest-nonce retained-history rules. B203 must not absorb B204 recovery, B301 SQLCipher, B401 native protectors, B501 full freshness-manifest/anchor implementation, Specification 005 media behavior, new dependency adoption, donor-code adoption, or release/compliance work.