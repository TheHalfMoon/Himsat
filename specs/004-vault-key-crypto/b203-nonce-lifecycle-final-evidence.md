# B203 Nonce Lifecycle Final Evidence

## Disposition

Specification 004 B203 is implementation-canonical and exact-post-merge qualified for the reviewed nonce lifecycle only.

```text
B203_PR = 54
B203_BASE = 96442f49a834cd88195fd934194485ca0c4fad7a
B203_INITIAL_HEAD = beaf8345c8b23fda2b56df77801c2b675ac4cfff
B203_FINAL_HEAD = b6fdde9da02b8cc92363095e26cc8971e0df7b6e
B203_CANONICAL_MERGE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
```

## Implemented boundary

B203 adds only the reviewed nonce-generation and reservation lifecycle required by Specification 004:

- exact 24-byte XChaCha20-Poly1305 nonces from the approved OS CSPRNG through `getrandom::fill`;
- typed fail-closed randomness failure with no weak fallback or downgrade;
- reservation identity bound to `VaultId`, purpose key, key generation, and nonce;
- independent bounded-blob and freshness-manifest purpose domains;
- hydration only from already authenticated canonical inventory/history supplied by the owning layer;
- exact duplicate canonical reservation rejection as `CorruptOrTampered`;
- pre-publication collision discard and regeneration;
- abandoned/ambiguous attempt reservations retained in-process so retries do not reuse a consumed candidate;
- restore/copy preserves an existing authenticated nonce exactly rather than regenerating it;
- re-encryption takes a fresh nonce through the production generation path;
- retained authenticated manifest-history reservations may be loaded before manifest nonce generation;
- cross-vault reservation and generation requests fail closed; and
- the normal bounded-blob production encryption entry point reserves a fresh nonce before invoking the already reviewed B202 envelope implementation. If B202 encryption fails after reservation, the candidate remains reserved so retry cannot reuse it.

The nonce ledger is deliberately not a persistence format. Authenticated manifest serialization/publication, protected freshness anchoring, backup/restore workflow, and canonical inventory ownership remain later leaves.

## Preserved negative lineage

Initial exact head `beaf8345c8b23fda2b56df77801c2b675ac4cfff` produced CI `34230105312` with macOS and Windows formatting failures. Downstream lint/tests/registered dependency closure on those failed jobs were skipped and remain NOT PASS.

Durable PR #54 negative-lineage comment: `5585698183`.

No rerun, force-push, rebase, or history rewrite was used. Forward-only successor commit `b6fdde9da02b8cc92363095e26cc8971e0df7b6e` applied the required rustfmt repair and bound the normal production encryption path to B203 reservation semantics. The initial failed lineage is not reclassified by later success.

## Exact-head qualification

Final accepted head:

```text
b6fdde9da02b8cc92363095e26cc8971e0df7b6e
```

Exact-head workflow evidence:

```text
CI = 34230572075 / run #135 / SUCCESS
R3 = 34230572042 / run #112 / SUCCESS
```

CI completed formatting, lint, tests, and registered dependency closure on Ubuntu, macOS, and Windows, together with Diffcipline R2, SpecGrain pinned-source validation, provenance validation/generated closure, provenance adversarial self-test, and negative controls.

Observed exact-head tests on Ubuntu:

```text
himsat-core:   56 passed; 0 failed
himsat-events: 10 passed; 0 failed
```

The exact PR diff remained limited to:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_nonce.rs
```

Compare from base to final head was ahead 2, behind 0. No Cargo manifest, lockfile, provenance registry, generated artifact, workflow, donor, Specification 005, or release-policy path changed.

## Review and reconciliation state

Before merge, PR #54 was open, non-draft, mergeable, and based on exact canonical `96442f49a834cd88195fd934194485ca0c4fad7a` with exact final head `b6fdde9da02b8cc92363095e26cc8971e0df7b6e`.

No inline review thread existed. No independent approval was available. Qodo was billing-blocked and CodeRabbit auto-skipped because of repository eligibility; those outputs are NOT PASS. Repository-owner evidence is not treated as independent security review.

Durable pre-merge repository-owner reconciliation review: `5142309480`.

## Guarded merge and parentage

The merge request used exact expected-head protection:

```text
EXPECTED_HEAD_SHA = b6fdde9da02b8cc92363095e26cc8971e0df7b6e
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
```

Durable transport-result comment: `5585875796`.

Canonical parentage is exact:

```text
PARENT_1 = 96442f49a834cd88195fd934194485ca0c4fad7a
PARENT_2 = b6fdde9da02b8cc92363095e26cc8971e0df7b6e
MERGE_TREE = 5c2b4a5ea595e2761eb8810d0203b0a0f79465ed
```

## Exact post-merge qualification

Exact push-triggered workflows on canonical merge `33cab7cb4e9dae1469e7e44ffaeeec05673f88d2` reached terminal SUCCESS:

```text
POSTMERGE_CI = 34231954517 / run #136 / SUCCESS
POSTMERGE_R3 = 34231954422 / run #113 / SUCCESS
```

Post-merge CI again completed the supported Rust matrix and all repository qualification jobs successfully, including Windows lint/tests/registered dependency closure and negative controls.

B203 is therefore canonical and closed only for the reviewed nonce lifecycle described above.

## Scope not claimed

B203 does not implement or claim:

- B204 recovery-envelope execution, Argon2id passphrase derivation, or recovery authentication;
- B205 aggregate adversarial qualification beyond tests required to prove B203 itself;
- B301+ SQLCipher integration;
- B401+ native platform protector adapters;
- B501 authenticated freshness-manifest persistence, protected genesis/anchor advancement, backup/restore workflow, full rotation, or deletion;
- Specification 005 media streaming/journal sequencing;
- new dependency or donor adoption; or
- release, FIPS, or compliance qualification.

`P011` remains historical NOT PASS and is not modified or retroactively upgraded.

## B204 rebound boundary

B204 may begin only after the B203/B204 reconciliation that carries this evidence becomes exact-head qualified, reconciled against live `main`/diff/reviews/threads/comments/mergeability, merged with explicit expected-head protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

B204 is bounded to the already reviewed canonical recovery-envelope v1 contract:

- exact 167-byte public `HIMSAT/RECOVERY/ENVELOPE/v1` representation;
- fixed v1 identifiers and `ARGON2ID_RFC9106_64M_V1` parameters: Argon2 version `0x0013`, memory `65536 KiB`, passes `3`, parallelism `4`, output `32` bytes, salt `16` bytes, nonce `24` bytes;
- XChaCha20-Poly1305 wrapping of exactly one 32-byte VRK into exactly 48 ciphertext+tag bytes;
- exact canonical `HIMSAT/RECOVERY/AAD/v1` binding of every public cryptographic context field;
- checked parser arithmetic and exact-length validation before KDF allocation or cryptographic execution;
- unsupported version/policy rejection without downgrade;
- externally uniform `RecoveryAuthenticationFailed` for wrong passphrase and AEAD authentication failure with no VRK release; and
- vault/generation/policy/context transplant rejection.

B204 must not absorb B205 aggregate qualification, SQLCipher, native protector mechanics, full freshness/backup/rotation/deletion work, Specification 005, new dependency/donor adoption, or release/compliance claims.
