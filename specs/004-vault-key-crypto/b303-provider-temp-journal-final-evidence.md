# B303 SQLCipher Provider, Temp-Store, and Journal Final Evidence

## Disposition

Specification 004 B303 is implementation-canonical and exact-post-merge qualified for the reviewed SQLCipher/OpenSSL provider, temp-storage, and journal-qualification boundary only.

```text
B303_PR = 64
B303_BASE = 407d79dc6bc872088423569e9265ea32a094e101
B303_FINAL_HEAD = c129d8a0959ae68b2bf2c0986a8622cae97c4244
B303_CANONICAL_MERGE = b3d4c7de77ec7a072fdcdd09d2d798519236db34
```

The canonical base is the exact B302/B303 reconciliation merge. That reconciliation was exact-head qualified, expected-head guarded, parentage-proven, and exact-post-merge qualified before B303 implementation began.

## Implemented boundary

B303 changed only:

```text
crates/himsat-core/src/vault_sqlcipher.rs
```

The keyed SQLCipher open boundary now proves, before returning a lease-bound handle:

```text
SQLCIPHER_RUNTIME = 4.14.0 community
SQLITE_RUNTIME = 3.51.3
CIPHER_PROVIDER = openssl
CIPHER_PROVIDER_VERSION = OpenSSL 3.6.3 9 Jun 2026
NON_ANDROID_TEMP_STORE_COMPILE_OPTION = TEMP_STORE=2
ANDROID_TEMP_STORE_COMPILE_OPTION = TEMP_STORE=3
TEMP_STORE_RUNTIME = MEMORY / 2
```

`PRAGMA cipher_provider` and `PRAGMA cipher_provider_version` are queried only after the existing raw-key operation, exact runtime-identity proof, and B302 encryption-active proof. Any provider name/version mismatch fails closed.

The selected target-specific SQLite compile option is positively checked with `sqlite_compileoption_used`. `PRAGMA temp_store = MEMORY` is then applied and read back as exact integer `2` before the handle is returned. Failure to prove the selected compile posture or memory-backed runtime temp-store state is fail-closed.

The exact OpenSSL runtime text is derived from the already reviewed/pinned OpenSSL 3.6.3 source revision `aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f`; SQLCipher exposes the provider version through its reviewed OpenSSL provider path.

## Journal qualification

B303 deliberately does not select or freeze one permanent journal mode. The file-backed qualification test intentionally exercises both required journal families under the same keyed reviewed provider:

1. enter WAL mode;
2. commit an encrypted transaction;
3. checkpoint/truncate WAL;
4. transition to rollback-journal `DELETE` mode;
5. commit another encrypted transaction; and
6. verify both committed rows remain readable and `temp_store` remains memory-backed.

This is B303 journal behavior qualification only. It is not B304 integrity checking, B305 corruption/wrong-key fixture expansion, or B306 plaintext-spill qualification.

## Existing build freeze reused

The canonical dependency-closure tooling and CI/R3 continue to reject unreviewed provider/build redirects. The B303 implementation does not duplicate or weaken that existing guard.

Reviewed build posture remains:

```text
OPENSSL_RUST_USE_NASM = 0
OPENSSL_NO_VENDOR = unset
OPENSSL_LIB_DIR = unset
OPENSSL_INCLUDE_DIR = unset
OPENSSL_DIR = unset
LIBSQLITE3_SYS_USE_PKG_CONFIG = unset
LIBSQLITE3_FLAGS = unset
```

B303 changed no `Cargo.toml`, `Cargo.lock`, provenance registry, generated notices/SBOM, or workflow.

## Preserved negative lineage

The first implementation head is permanently NOT PASS:

```text
HEAD = 22c83da98ff913443b2bc1eeaffb5ece3c07521a
CI = 34273708654 / run #167 / FAILURE_NOT_PASS / attempt 1
R3 = 34273708614 / run #144 / FAILURE_NOT_PASS / attempt 1
OBSERVED = UBUNTU_MACOS_WINDOWS_FORMATTING_FAILURE_NOT_PASS
OBSERVED_DIFFCIPLINE_R2 = FAILURE_NOT_PASS
OBSERVED_DIFFCIPLINE_R3 = FAILURE_NOT_PASS
DISPOSITION = SUPERSEDED_NOT_QUALIFIED
```

Durable negative-evidence comment: `5591327257`.

The second successor head is also permanently NOT PASS:

```text
HEAD = eea61d65b0a58fe7bcac6688eb5561940532bad1
CI = 34274527904 / run #168 / FAILURE_NOT_PASS / attempt 1
R3 = 34274527967 / run #145 / FAILURE_NOT_PASS / attempt 1
OBSERVED = UBUNTU_MACOS_WINDOWS_FORMATTING_FAILURE_NOT_PASS
OBSERVED_DIFFCIPLINE_R2 = FAILURE_NOT_PASS
OBSERVED_DIFFCIPLINE_R3 = FAILURE_NOT_PASS
DISPOSITION = SUPERSEDED_NOT_QUALIFIED
```

Durable negative-evidence comment: `5591419638`.

The exact Ubuntu job log for the second head proved the pinned Rust 1.98.1 / rustfmt 1.9.0-stable formatting deltas. The accepted third head applied those exact formatter-produced changes forward-only. Neither failed head was rerun, force-pushed, rebased, rewritten, or retroactively reclassified.

## Exact-head qualification

Final accepted implementation head:

```text
c129d8a0959ae68b2bf2c0986a8622cae97c4244
```

Exact-head workflow evidence:

```text
CI = 34275130542 / run #169 / SUCCESS / attempt 1
R3 = 34275130518 / run #146 / SUCCESS / attempt 1
```

All supported Rust jobs, dependency-closure checks, provenance checks, SpecGrain, negative controls, Diffcipline R2, and Diffcipline R3 succeeded on the exact final head.

## Review and live reconciliation state

Immediately before merge, PR #64 was open, non-draft, mergeable, based on exact canonical `407d79dc6bc872088423569e9265ea32a094e101`, and headed by exact `c129d8a0959ae68b2bf2c0986a8622cae97c4244`. The exact diff changed only `crates/himsat-core/src/vault_sqlcipher.rs`.

No inline review thread existed. Before the repository-owner reconciliation, no submitted review existed. Qodo was billing-blocked and CodeRabbit auto-skipped because of repository eligibility; both outputs are NOT PASS.

Durable repository-owner pre-merge reconciliation review: `5146794283`.

That repository-owner review is governance reconciliation only. Repository-owner evidence, implementation tests, CI, R3 automation, Qodo billing-blocked output, CodeRabbit skipped output, Cubic neutral output, or other automation do not satisfy Q009. Q009 remains unsatisfied and requires a genuinely independent substantive crypto/security review of the exact implementation revision at the later qualification gate.

## Guarded merge and parentage

The merge request used exact expected-head protection:

```text
EXPECTED_HEAD_SHA = c129d8a0959ae68b2bf2c0986a8622cae97c4244
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = b3d4c7de77ec7a072fdcdd09d2d798519236db34
```

Durable transport-result comment: `5591583741`.

Canonical parentage is exact:

```text
PARENT_1 = 407d79dc6bc872088423569e9265ea32a094e101
PARENT_2 = c129d8a0959ae68b2bf2c0986a8622cae97c4244
MERGE_TREE = 1d8c9a69c354990d1f82ea1c77737cc5d977d1dc
```

Durable parentage comment: `5591593465`.

## Exact post-merge qualification

Exact push-triggered workflows on canonical merge `b3d4c7de77ec7a072fdcdd09d2d798519236db34` reached terminal SUCCESS on their original attempts:

```text
POSTMERGE_CI = 34276275141 / run #170 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34276275137 / run #147 / SUCCESS / attempt 1 / push
```

Durable post-merge qualification comment: `5591699113`.

B303 is therefore canonical and closed only for the reviewed SQLCipher/OpenSSL provider, temp-store, and WAL/rollback-journal qualification boundary described above.

## B302/B303 reconciliation evidence repaired forward-only

The prior task ledger still shows B302R001-B302R003 unchecked even though their exact live evidence is proven. This reconciliation repairs only that ledger state; it does not rewrite history or upgrade unavailable evidence.

```text
B302_B303_RECONCILIATION_PR = 63
B302_B303_RECONCILIATION_BASE = 001e274a321cd0f6c472ce768f1a9910165598fe
B302_B303_RECONCILIATION_HEAD = 188b2096163890fa3967f0d3228b1d72f9812a0c
PREMERGE_CI = 34269945274 / run #165 / SUCCESS / attempt 1
PREMERGE_R3 = 34269945248 / run #142 / SUCCESS / attempt 1
OWNER_RECONCILIATION_REVIEW = 5146300086
EXPECTED_HEAD_SHA = 188b2096163890fa3967f0d3228b1d72f9812a0c
EXPECTED_HEAD_TRANSPORT_COMMENT = 5590900702
CANONICAL_MERGE = 407d79dc6bc872088423569e9265ea32a094e101
PARENT_1 = 001e274a321cd0f6c472ce768f1a9910165598fe
PARENT_2 = 188b2096163890fa3967f0d3228b1d72f9812a0c
POSTMERGE_CI = 34271065550 / run #166 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34271065553 / run #143 / SUCCESS / attempt 1 / push
POSTMERGE_RECONCILIATION_COMMENT = 5591073000
```

## Scope not claimed

B303 does not implement or claim:

- B304 normal SQLite or SQLCipher cipher/page-authentication integrity checks;
- B305 wrong-key/corruption/unsupported-provider/version fixture expansion;
- B306 plaintext-spill, logical-ID, semantic-marker, or public-filename qualification;
- B307 copy-verify-publish migration;
- B401-B406 native platform protector adapters or platform qualification;
- B501-B506 freshness, backup, rotation, or deletion implementation;
- Q009 independent substantive crypto/security review;
- Specification 005 behavior;
- new dependency or donor adoption; or
- release, FIPS, or compliance qualification.

`P011` remains historical NOT PASS and remains unchecked.

## B304 rebound

B304 may begin only after this B303/B304 evidence-state reconciliation becomes exact-head qualified, reconciled against live `main`/PR head/base/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

After that qualification, B304 is bounded to the normal SQLite integrity check and SQLCipher cipher/page-authentication integrity check on the exact adopted provider. The implementation must derive exact success/failure semantics from the pinned SQLCipher source and keep every integrity operation behind the existing B105 lease gate. B304 must not absorb corruption/wrong-key fixture expansion owned by B305.