# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_B301_CANONICAL_RECONCILING_B302_BOUND
ACTIVE_SPECIFICATION = 004-vault-key-crypto
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_002_DISPOSITION = CLOSED_CANONICAL
SPEC_003_DISPOSITION = CLOSED_CANONICAL
SPEC_003_CLOSEOUT_MERGE = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
SPEC_004A_DESIGN_AUTHORITY = APPROVED_EXACT_CANONICAL
SPEC_004_FINAL_REVIEWED_SHA = 5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98
SPEC_004_FINAL_REVIEW_DISPOSITION = APPROVE
SPEC_004_FINAL_BLOCKING_FINDINGS = NONE
SPEC_004P_DISPOSITION = CLOSED_CANONICAL
SPEC_004P_CANONICAL_MERGE = a4d32ee93e0ab95af8376ba0ca09e248070c5924
P011_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
B101_DISPOSITION = CANONICAL_CLOSED
B102_DISPOSITION = CANONICAL_CLOSED
B103_DISPOSITION = CANONICAL_CLOSED
B104_DISPOSITION = CANONICAL_CLOSED
B105_DISPOSITION = CANONICAL_CLOSED
B201_DISPOSITION = CANONICAL_CLOSED
B202_DISPOSITION = CANONICAL_CLOSED
B203_DISPOSITION = CANONICAL_CLOSED
B204_DISPOSITION = CANONICAL_CLOSED
B205_DISPOSITION = CANONICAL_CLOSED
B206_DISPOSITION = CLOSED_SCOPE_BOUNDARY_SPEC_005
B301_DISPOSITION = CANONICAL_CLOSED
B205_B301_RECONCILIATION_DISPOSITION = CANONICAL_CLOSED
B301_B302_RECONCILIATION_STATE = ACTIVE_NOT_YET_CANONICAL
NEXT_IMPLEMENTATION_LEAF = B302_ENCRYPTION_ACTIVE_PROOF_ONLY
SPEC_004_IMPLEMENTATION_AUTHORITY = B302_ONLY_IF_THIS_B301_B302_RECONCILIATION_IS_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
PRODUCT_FEATURE_AUTHORITY = SPEC_004_B302_SQLCIPHER_ENCRYPTION_ACTIVE_PROOF_ONLY_UNDER_THE_CONDITION_ABOVE
SPEC_005_AUTHORITY = BLOCKED_PENDING_SPEC_004_CLOSEOUT
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
```

Live GitHub/repository truth overrides this file whenever repository state changes after this reconciliation is authored.

## Canonical Specification 004 authority

Specification 004A has an exact independently reviewed design. Review-only PR #29 examined canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE` with no unresolved blocking finding. `round3-normative-contracts.md` remains controlling for recovery, bounded-blob, freshness, nonce, and rotation contracts. `round4-blob-inventory-contract.md` remains controlling for `GENERIC_ARTIFACT_BLOB` canonical stored-object and manifest-inventory semantics.

Specification 004P is closed canonically. The reviewed dependency closure already contains the exact providers required by B301-B307; this reconciliation adopts no dependency and changes no provider/provenance bytes.

`P011` remains unchecked in `tasks.md`. Historical PR #37 has exact head, canonical merge, parentage, and post-merge CI/R3 evidence, but the historical merge API request body containing the exact `expected_head_sha` argument is not reconstructible post hoc. The repository does not fabricate or retroactively upgrade that missing transport evidence.

Q009 also remains unsatisfied. Repository-owner evidence, implementation tests, CI/R3 automation, billing-blocked/skipped review bots, or neutral automation do not substitute for the required independent substantive crypto/security review of the exact implementation revision.

## Canonical implementation lineage through B301

```text
B101_CANONICAL_MERGE = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
B101_B102_RECONCILIATION = 0252bb31764c9178e270694f4087e8ac701271a0
B102_CANONICAL_MERGE = 4b27ede7d9bf17caac163b7607c331056634fc95
B102_B103_RECONCILIATION = 9ed7ccd960cbf92a9718d424421dd40b71cfe0de
B103_CANONICAL_MERGE = ead22ea8c0b248431a2f8a50264f6acdbc9f7a72
B103_B104_RECONCILIATION = 57217a6614c07ac5e8a00d85114dd06eee1a0120
B104_CANONICAL_MERGE = 6578936f7051b548339f3cf95ca0d41e6629d8bc
B104_B105_RECONCILIATION = 623c0ad4207ecbbd5e404a2a32ba3cdb61006778
B105_CANONICAL_MERGE = 73b13d38ac3e34143c813bda679e6e96ce01762e
B105_B201_RECONCILIATION = 33fc791443d25dba0ed7a5958710cb52ebb5bfad
B201_CANONICAL_MERGE = 3785561963b6eadab219b330367a9b6295755939
B201_B202_RECONCILIATION = 5ed2c3f79ef5af783c479871217e4aba0dcf0fd2
B202_CANONICAL_MERGE = 483857be2abf017c93fd94c210beef8080bc47fb
B202_B203_RECONCILIATION = 96442f49a834cd88195fd934194485ca0c4fad7a
B203_CANONICAL_MERGE = 33cab7cb4e9dae1469e7e44ffaeeec05673f88d2
B203_B204_RECONCILIATION = c453d56c00fc1fabca09987f1c11084099c6326e
B204_CANONICAL_MERGE = ac0925051cc128580b9794837fd03200f9a7374b
B204_B205_RECONCILIATION = 854373dc4fd44cc496b9e52f9f14804cdbf750aa
B205_CANONICAL_MERGE = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
B205_B301_RECONCILIATION = c002bc21706bd90622586d453cb9ed9722d53fbb
B301_CANONICAL_MERGE = 6c30399da307b1b5ea23988a99edf540872cad42
```

Complete leaf evidence remains under `specs/004-vault-key-crypto/`, including `b201-hkdf-final-evidence.md`, `b202-bounded-blob-final-evidence.md`, `b203-nonce-lifecycle-final-evidence.md`, `b204-recovery-envelope-final-evidence.md`, `b205-crypto-adversarial-final-evidence.md`, and `b301-sqlcipher-provider-final-evidence.md` once this reconciliation becomes canonical.

## Canonical B205/B301 reconciliation disposition

The B205/B301 reconciliation was exact-head qualified before B301 began:

```text
PR = 59
BASE = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
HEAD = ad74fc41dbaa7353b3d3c47b33395b8681e56e82
PREMERGE_CI = 34254716241 / run #153 / SUCCESS
PREMERGE_R3 = 34254716268 / run #130 / SUCCESS
EXPECTED_HEAD_SHA = ad74fc41dbaa7353b3d3c47b33395b8681e56e82
EXPECTED_HEAD_TRANSPORT = PROVEN_COMMENT_5589030694
MERGE_METHOD = merge
CANONICAL_MERGE = c002bc21706bd90622586d453cb9ed9722d53fbb
PARENT_1 = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
PARENT_2 = ad74fc41dbaa7353b3d3c47b33395b8681e56e82
MERGE_TREE = 270de401bf62a287a11f7c91029e866e5b47b638
POSTMERGE_CI = 34255936220 / run #154 / SUCCESS
POSTMERGE_R3 = 34255936283 / run #131 / SUCCESS
```

Durable repository-owner pre-merge reconciliation review: `5144845278`. Durable guarded-transport comment: `5589030694`. Durable post-merge reconciliation comment: `5589241065`.

The prior task ledger left B205R001-B205R003 unchecked despite this proven live evidence. This reconciliation repairs that state forward-only; it does not alter historical commits or upgrade unavailable evidence.

## Canonical B301 disposition

B301 implementation PR #60 started from exact canonical B205/B301 reconciliation merge `c002bc21706bd90622586d453cb9ed9722d53fbb` and changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_sqlcipher.rs
```

The accepted implementation integrates the exact reviewed/provenance-registered SQLCipher provider/keying strategy behind the existing B105 keyed-handle lease. It validates exact vault/generation identity and `StructuredStore` purpose before path touch, uses explicit non-URI open flags, applies the 32-byte derived key using SQLCipher raw-key syntax, zeroizes the temporary Rust key-SQL buffer, checks the reviewed runtime identities, and exposes no raw connection getter.

Canonical provider identities are:

```text
RUSQLITE_PACKAGE = 0.40.1
LIBSQLITE3_SYS_PACKAGE = 0.38.2
RUSQLITE_FEATURE = bundled-sqlcipher-vendored-openssl
SQLCIPHER_RUNTIME_VERSION = 4.14.0 community
SQLCIPHER_EMBEDDED_SQLITE_VERSION = 3.51.3
OPENSSL_UPSTREAM_VERSION = 3.6.3
```

Preserved predecessor heads remain NOT PASS:

```text
414b7de3912914cfc88e8d56030e42c33a271588
  CI = 34257908587_CANCELLED_NOT_PASS
  R3 = 34257908571_CANCELLED_NOT_PASS
  OBSERVED_BEFORE_CANCELLATION = UBUNTU_FORMATTING_FAILURE_NOT_PASS

a2243bee1a4c3adbe704cf594f925ff7a947ab36
  CI = 34258148159_CANCELLED_NOT_PASS
  R3 = 34258148269_CANCELLED_NOT_PASS
  OBSERVED_BEFORE_CANCELLATION = UBUNTU_FORMATTING_FAILURE_NOT_PASS
```

No predecessor run was rerun or retroactively reclassified.

Final exact head `630ca6ccd545a49d81efbdb08cb1efbab9cab0e4` passed CI `34258332677` and R3 `34258332704`. Exact-head Ubuntu execution observed `70 + 3 + 6 + 10` passing tests with zero failures; all six B301 unit tests passed, including the exact reviewed SQLCipher runtime-open check.

No inline review thread blocked the merge. Qodo billing-blocked and CodeRabbit auto-skipped outputs were NOT PASS. Repository-owner reconciliation review `5145171484` is durable evidence but is not an independent substantive crypto/security approval and does not satisfy Q009.

Guarded transport used:

```text
EXPECTED_HEAD_SHA = 630ca6ccd545a49d81efbdb08cb1efbab9cab0e4
MERGE_METHOD = merge
MERGED = true
CANONICAL_MERGE = 6c30399da307b1b5ea23988a99edf540872cad42
```

Durable transport-result comment: `5589503117`.

Canonical parentage is exact:

```text
PARENT_1 = c002bc21706bd90622586d453cb9ed9722d53fbb
PARENT_2 = 630ca6ccd545a49d81efbdb08cb1efbab9cab0e4
MERGE_TREE = c358119aa50b9da1fedb64355dc0c24a64ddae4b
```

Exact push-triggered post-merge CI `34259707758` and R3 `34259707773` both reached terminal SUCCESS on exact canonical merge `6c30399da307b1b5ea23988a99edf540872cad42`.

B301 is therefore canonical and closed only for the reviewed SQLCipher provider/keying integration boundary. It does not prove encryption-active state; that remains B302.

## Active reconciliation objective

Canonicalize B301 final evidence and bind the next implementation frontier to B302 without changing security semantics, production/runtime behavior, Cargo manifest/lockfile, dependency bytes, provenance entries, generated artifacts, workflows, donor material, platform protector implementation, freshness/backup/rotation/deletion behavior, Specification 005 behavior, Q009 disposition, release posture, FIPS claim, or compliance claim.

This B301/B302 reconciliation must itself:

1. pass exact-head CI and R3;
2. be reconciled against live reviews, review threads, comments, exact diff, `main`, and mergeability;
3. merge only with explicit `expected_head_sha` protection;
4. prove exact canonical parentage; and
5. pass exact push-triggered post-merge CI and R3.

Only after all five conditions are proven may B302 work begin from the resulting exact canonical `main`.

## B302 bounded scope after reconciliation qualification

B302 owns only the encryption-active proof on the already reviewed SQLCipher handle after successful B301 keying.

The canonical requirement is fail closed unless the opened handle proves encryption is active. The adopted SQLCipher provider supports the reviewed status surface required for this proof. B302 may add only the minimum lease-gated provider query/error state and tests needed to establish positive active state and fail-closed inactive/error state.

B302 must not absorb:

- B303 runtime temp/WAL/journal/provider/build setting enforcement;
- B304 normal SQLite and SQLCipher cipher/page-authentication integrity checks;
- B305 wrong-key/corruption/unsupported-provider/version fixture expansion;
- B306 plaintext-spill/file-name qualification;
- B307 copy-verify-publish migration;
- B401-B406 platform protector implementation;
- B501-B506 freshness/backup/rotation/deletion;
- Q009 independent substantive crypto/security review;
- Specification 005 behavior;
- new donor/dependency adoption; or
- release, FIPS, or compliance claims.

If encryption-active state cannot be positively proven on the exact reviewed provider, B302 must fail closed rather than continue with an unproven structured-store handle.

## Specification 004 delivery chain

```text
004A reviewed cryptographic design           APPROVED_EXACT_CANONICAL
  -> 004P provider/provenance closure         CLOSED_CANONICAL
  -> B101-B105 portable foundation            CANONICAL_QUALIFIED
  -> B201-B205 crypto envelope foundation     CANONICAL_QUALIFIED
  -> B206 Specification 005 boundary          CLOSED_SCOPE_BOUNDARY
  -> B205/B301 state reconciliation           CANONICAL_QUALIFIED
  -> B301 reviewed SQLCipher integration      CANONICAL_QUALIFIED
  -> B301/B302 state reconciliation           ACTIVE
  -> B302-B307 encrypted structured store     B302_NEXT_AFTER_RECONCILIATION_QUALIFICATION
  -> B401-B406 platform protectors            BLOCKED_PENDING_PRIOR_LEAVES
  -> B501-B506 freshness/backup/rotation      BLOCKED_PENDING_PRIOR_LEAVES
  -> Q001-Q012 exact implementation review/R3 BLOCKED_PENDING_IMPLEMENTATION
  -> C001-C005 Specification 004 closeout      BLOCKED_PENDING_QUALIFICATION
  -> Specification 005                        BLOCKED_PENDING_SPEC_004_CLOSEOUT
```

## Residual blockers and non-claims

- `P011` remains unchecked because historical expected-head transport proof is not reconstructible post hoc.
- Q009 remains unsatisfied and requires a genuine independent substantive crypto/security review of the exact implementation revision.
- Specification 005 remains blocked pending genuine Specification 004 `CLOSED_CANONICAL` closeout.
- No new donor/dependency adoption is authorized by this reconciliation.
- No release, FIPS, compliance, universal app-exclusive, universal user-presence, universal atomic-anchor, or universal hardware-backed claim is authorized.