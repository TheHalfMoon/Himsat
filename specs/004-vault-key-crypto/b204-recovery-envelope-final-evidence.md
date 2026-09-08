# B204 Recovery Envelope Final Evidence

## Disposition

Specification 004 B204 is implementation-canonical and exact-post-merge qualified for the reviewed recovery-envelope v1 execution contract only.

```text
B204_PR = 56
B204_BASE = c453d56c00fc1fabca09987f1c11084099c6326e
B204_FINAL_HEAD = 2a071e21c483b239e76fb12af5cecd24eec8059b
B204_CANONICAL_MERGE = ac0925051cc128580b9794837fd03200f9a7374b
```

The canonical base above is the exact B203/B204 reconciliation merge. That reconciliation itself was guarded and post-merge qualified before B204 implementation began.

## Implemented boundary

B204 adds only the reviewed fixed recovery-envelope v1 execution/parsing contract:

- exact 167-byte `HIMSAT/RECOVERY/ENVELOPE/v1` public representation;
- exact 116-byte canonical `HIMSAT/RECOVERY/AAD/v1` representation from the controlling round-3 normative amendment;
- fixed `ARGON2ID_RFC9106_64M_V1` policy using Argon2id version `0x0013`, memory `65536 KiB`, passes `3`, parallelism `4`, and exactly 32 output bytes;
- exact 16-byte OS-CSPRNG salt and fresh 24-byte OS-CSPRNG nonce per production creation attempt through the already adopted randomness provider;
- XChaCha20-Poly1305 wrapping of exactly one 32-byte VRK into exactly 48 ciphertext-plus-tag bytes;
- parser validation of exact total length, domain, fixed identifiers, non-zero generation, fixed Argon2 parameters, and ciphertext/tag length before KDF allocation or AEAD execution;
- rejection of unsupported/weaker/larger v1 policies without downgrade;
- fixed-profile allocation inability mapped to fail-closed `ResourceLimit`;
- exact UTF-8 passphrase consumption with no trim, case folding, truncation, or Unicode normalization;
- creation-policy bounds of at least 16 Unicode scalar values and at most 1024 exact UTF-8 bytes;
- externally uniform `RecoveryAuthenticationFailed` for wrong passphrase, AEAD/tag authentication failure, authenticated public-context tamper, and expected vault/generation transplant;
- no partial VRK release on authentication failure; and
- explicit zeroization of source Recovery KEK and recovered-VRK temporary buffers after transfer into `OwnedKeyMaterial`.

B204-local deterministic and negative tests additionally prove salt, nonce, generation, ciphertext, tag, vault/generation transplant, exact UTF-8, trailing-space, case-change, suffix/no-truncation, and composed/decomposed Unicode behavior. Those tests are implementation evidence and are not an independent substantive security review.

## Preserved negative and superseded lineage

No failed or cancelled B204 head was rerun, force-pushed, rebased, rewritten, or retroactively reclassified.

```text
ff4cb5e0e99cfb52f280b6f4c7812bf14168a82b
  CI = 34236988011 / run #139 / FAILURE_NOT_PASS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED

98ac4f61d00e0223344c114d93a94da478f99e9c
  CI = 34237525006 / run #140 / FAILURE_NOT_PASS
  R3 = 34237525043 / run #117 / FAILURE_NOT_PASS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED

aac7878570676d8b502cfa8ea2d5fe57a03a5a1e
  CI = 34238877704 / run #141 / CANCELLED_NOT_PASS
  R3 = 34238877692 / run #118 / SUCCESS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED

23f48ec2863ce17a77f37cce1b4c802160286da8
  CI = 34239753050 / run #142 / CANCELLED_NOT_PASS
  R3 = 34239753073 / run #119 / SUCCESS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED

6d02e878940f649cc330dc8970bf737f3416ebc5
  CI = 34240121726 / run #143 / FAILURE_NOT_PASS
  R3 = 34240121731 / run #120 / FAILURE_NOT_PASS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED
```

Durable PR #56 negative/superseded evidence comments include `5586570304`, `5586762590`, `5586996995`, and `5587124329`.

The `6d02e878...` failure was a pinned-rustfmt delta in `crates/himsat-core/tests/b204_recovery_negative.rs`; its B204 tests themselves passed, but formatting failure made that lineage NOT PASS. The repair was forward-only in successor commit `2a071e21c483b239e76fb12af5cecd24eec8059b`.

## Exact-head qualification

Final accepted head:

```text
2a071e21c483b239e76fb12af5cecd24eec8059b
```

Exact-head workflow evidence:

```text
CI = 34241383350 / run #144 / SUCCESS
R3 = 34241383280 / run #121 / SUCCESS
```

Exact-head CI completed formatting, lint, tests, and registered dependency closure on Ubuntu, macOS, and Windows, together with Diffcipline R2, SpecGrain pinned-source validation, provenance validation/generated closure, provenance adversarial self-test, and negative controls.

Exact-head R3 completed provenance/generated closure, registered dependency closure, rustfmt, Clippy, and all-target workspace tests. Observed exact-head test totals were:

```text
himsat-core:                    64 passed; 0 failed
b204_recovery_negative:         3 passed; 0 failed
himsat-events:                 10 passed; 0 failed
```

The exact PR diff remained limited to:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_recovery.rs
crates/himsat-core/tests/b204_recovery_negative.rs
```

Compare from exact base to final head was ahead 6, behind 0. No Cargo manifest, lockfile, provenance registry, generated artifact, workflow, donor, Specification 005, SQLCipher, native protector, freshness/backup/rotation/deletion, or release-policy path changed.

## Review and reconciliation state

Immediately before merge, PR #56 was open, non-draft, mergeable, and based on exact canonical `c453d56c00fc1fabca09987f1c11084099c6326e` with exact final head `2a071e21c483b239e76fb12af5cecd24eec8059b`.

No inline review thread existed. Qodo was billing-blocked and CodeRabbit auto-skipped because of repository eligibility; those outputs are NOT PASS. Cubic summary output is neutral automation and is not an independent security approval.

Durable pre-merge repository-owner reconciliation review: `5143469011`.

Repository-owner evidence, implementation tests, CI, R3 automation, Qodo billing-block, CodeRabbit auto-skip, or Cubic summary do not satisfy Q009. Q009 remains a later independent substantive crypto/security review gate for the exact implementation revision required by Specification 004 closeout.

## Guarded merge and parentage

The merge request used exact expected-head protection:

```text
EXPECTED_HEAD_SHA = 2a071e21c483b239e76fb12af5cecd24eec8059b
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = ac0925051cc128580b9794837fd03200f9a7374b
```

Durable transport-result comment: `5587314774`.

Canonical parentage is exact:

```text
PARENT_1 = c453d56c00fc1fabca09987f1c11084099c6326e
PARENT_2 = 2a071e21c483b239e76fb12af5cecd24eec8059b
MERGE_TREE = d63407d8a6bc50514bd8ff1df8bbf239455e9ac8
```

## Exact post-merge qualification

Exact push-triggered workflows on canonical merge `ac0925051cc128580b9794837fd03200f9a7374b` reached terminal SUCCESS:

```text
POSTMERGE_CI = 34242682960 / run #145 / SUCCESS
POSTMERGE_R3 = 34242683051 / run #122 / SUCCESS
```

Post-merge CI again completed the supported Rust matrix and all repository qualification jobs successfully, including Windows formatting/lint/tests/registered dependency closure, Diffcipline R2, SpecGrain, provenance validation/generated closure, provenance adversarial self-test, and negative controls.

B204 is therefore canonical and closed only for the reviewed recovery-envelope v1 execution contract described above.

## Scope not claimed

B204 does not implement or claim:

- B205 aggregate adversarial crypto qualification beyond tests required to prove B204 itself;
- B206 or Specification 005 media streaming/journal sequencing;
- B301+ SQLCipher integration;
- B401+ native platform protector adapters;
- B501+ authenticated freshness-manifest persistence, protected genesis/anchor advancement, backup/restore workflow, full rotation, or deletion;
- a completed Q009 independent substantive crypto/security review;
- new dependency or donor adoption; or
- release, FIPS, or compliance qualification.

`P011` remains historical NOT PASS and is not modified or retroactively upgraded.

## B205 rebound boundary

B205 may begin only after the B204/B205 reconciliation that carries this evidence becomes exact-head qualified, reconciled against live `main`/diff/reviews/threads/comments/mergeability, merged with explicit expected-head protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

B205 is bounded to aggregate adversarial evidence already required by the canonical Specification 004 contracts. It may add tests/fixtures and the minimum test-support surface necessary to prove the existing reviewed cryptographic behaviors, but it must not broaden product semantics or absorb later leaves.

Required B205 evidence includes, as applicable to the already implemented recovery and bounded-blob envelopes:

- wrong key and wrong passphrase;
- independent tamper of public header/fixed identifiers, nonce/salt, authenticated context/AAD inputs, ciphertext, and tag;
- unknown/unsupported suite, version, purpose, envelope type, or policy;
- zero/stale generation and vault/artifact/generation transplant;
- truncation and trailing data;
- declared-length mismatch, checked arithmetic/overflow paths, zero length, exact maximum, and maximum-plus-one where the contract exposes bounded lengths;
- weaker/larger/non-canonical KDF profile rejection before KDF allocation;
- wrong-passphrase versus AEAD-failure externally uniform recovery result;
- random-source failure; and
- duplicate/collision behavior already owned by the canonical nonce lifecycle.

B205 must not absorb B206/Specification 005, SQLCipher B301+, native protectors B401+, freshness/backup/rotation/deletion B501+, donor/dependency adoption, Q009 independent review, or release/compliance claims.
