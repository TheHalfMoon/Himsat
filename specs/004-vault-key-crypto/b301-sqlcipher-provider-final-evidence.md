# B301 SQLCipher Provider Integration Final Evidence

## Disposition

Specification 004 B301 is implementation-canonical and exact-post-merge qualified for the reviewed SQLCipher provider/keying integration boundary only.

```text
B301_PR = 60
B301_BASE = c002bc21706bd90622586d453cb9ed9722d53fbb
B301_FINAL_HEAD = 630ca6ccd545a49d81efbdb08cb1efbab9cab0e4
B301_CANONICAL_MERGE = 6c30399da307b1b5ea23988a99edf540872cad42
```

The canonical base above is the exact B205/B301 reconciliation merge. That reconciliation was itself exact-head qualified, expected-head guarded, parentage-proven, and exact-post-merge qualified before B301 implementation began.

## Implemented boundary

B301 changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_sqlcipher.rs
```

The accepted implementation:

- uses the already reviewed and provenance-registered `rusqlite = 0.40.1` / `libsqlite3-sys = 0.38.2` `bundled-sqlcipher-vendored-openssl` closure;
- derives only the existing B201 `KeyPurpose::StructuredStore` key;
- validates purpose plus exact vault/generation identity before touching the database path;
- requires a live existing `KeyedHandleLease` before provider open;
- opens with explicit read-write/create/no-mutex flags and omits SQLite URI filename semantics;
- applies the exact 32-byte derived key through SQLCipher raw-key syntax without introducing a second passphrase KDF;
- zeroizes the temporary Rust raw-key SQL buffer after the provider call;
- verifies runtime SQLCipher identity `4.14.0 community` and embedded SQLite identity `3.51.3` against the reviewed source closure;
- immediately encloses the raw `rusqlite::Connection` behind the existing B105 lease-bound database handle and exposes no raw connection getter; and
- returns typed fail-closed errors without key bytes or key SQL in normal diagnostics.

B301 deliberately does not claim that encryption is active. That proof remains B302.

## B301-local execution evidence

The accepted exact head contains six B301 unit tests covering:

- reviewed provider open flags and explicit URI exclusion;
- exact lowercase 32-byte SQLCipher raw-key syntax;
- successful open of the exact reviewed SQLCipher/SQLite runtime behind the lease gate;
- non-structured purpose rejection before filesystem touch;
- vault/generation identity mismatch rejection before filesystem touch; and
- revoked lease rejection before filesystem touch.

Exact-head Ubuntu execution observed:

```text
himsat-core:                    70 passed; 0 failed
b204_recovery_negative:         3 passed; 0 failed
b205_crypto_adversarial:         6 passed; 0 failed
himsat-events:                  10 passed; 0 failed
```

Within the 70 `himsat-core` tests, all six B301 tests passed, including `exact_reviewed_sqlcipher_runtime_opens_behind_lease_gate`.

## Preserved negative and superseded lineage

No failed or cancelled B301 head was rerun, force-pushed, rebased, rewritten, or retroactively reclassified.

```text
414b7de3912914cfc88e8d56030e42c33a271588
  CI = 34257908587 / run #156 / CANCELLED_NOT_PASS
  R3 = 34257908571 / run #133 / CANCELLED_NOT_PASS
  OBSERVED_BEFORE_CANCELLATION = UBUNTU_FORMATTING_FAILURE_NOT_PASS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED

a2243bee1a4c3adbe704cf594f925ff7a947ab36
  CI = 34258148159 / run #157 / CANCELLED_NOT_PASS
  R3 = 34258148269 / run #134 / CANCELLED_NOT_PASS
  OBSERVED_BEFORE_CANCELLATION = UBUNTU_FORMATTING_FAILURE_NOT_PASS
  DISPOSITION = SUPERSEDED_NOT_QUALIFIED
```

Durable PR #60 comments preserve the failure/cancellation lineage, including `5589291564`, `5589317193`, `5589324788`, and `5589365513`. Skipped downstream steps remain NOT PASS.

## Exact-head qualification

Final accepted head:

```text
630ca6ccd545a49d81efbdb08cb1efbab9cab0e4
```

Exact-head workflow evidence:

```text
CI = 34258332677 / run #158 / SUCCESS
R3 = 34258332704 / run #135 / SUCCESS
```

CI completed formatting, Clippy, all-target tests, and registered dependency closure on Ubuntu, macOS, and Windows together with Diffcipline R2, SpecGrain pinned-source validation, provenance validation/generated closure, provenance adversarial self-test, and negative controls.

## Review and reconciliation state

Immediately before merge, PR #60 was open, non-draft, mergeable, based on exact canonical `c002bc21706bd90622586d453cb9ed9722d53fbb`, and headed by exact `630ca6ccd545a49d81efbdb08cb1efbab9cab0e4`. Compare was ahead 4 / behind 0 and changed exactly the two implementation paths listed above.

No inline review thread existed. Qodo was billing-blocked and CodeRabbit auto-skipped because of repository eligibility; those outputs are NOT PASS.

Durable repository-owner pre-merge reconciliation review: `5145171484`.

Repository-owner evidence, implementation tests, CI, R3 automation, Qodo billing block, CodeRabbit auto-skip, or neutral automation do not satisfy Q009. Q009 remains the later independent substantive crypto/security review gate for the exact implementation revision required by Specification 004 closeout.

## Guarded merge and parentage

The merge request used exact expected-head protection:

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

## Exact post-merge qualification

Exact push-triggered workflows on canonical merge `6c30399da307b1b5ea23988a99edf540872cad42` reached terminal SUCCESS on their original attempts:

```text
POSTMERGE_CI = 34259707758 / run #159 / SUCCESS
POSTMERGE_R3 = 34259707773 / run #136 / SUCCESS
```

Post-merge CI completed the supported Rust matrix, Diffcipline R2, SpecGrain, provenance validation/generated closure, provenance adversarial self-test, and negative controls successfully. Post-merge R3 completed successfully on the exact canonical merge.

B301 is therefore canonical and closed only for the provider/keying integration boundary described above.

## B205/B301 reconciliation evidence repaired forward-only

The prior task ledger still shows B205R001-B205R003 unchecked even though their exact live evidence is now proven. This reconciliation repairs only that ledger state; it does not rewrite history.

```text
B205_B301_RECONCILIATION_PR = 59
B205_B301_RECONCILIATION_HEAD = ad74fc41dbaa7353b3d3c47b33395b8681e56e82
PREMERGE_CI = 34254716241 / run #153 / SUCCESS
PREMERGE_R3 = 34254716268 / run #130 / SUCCESS
EXPECTED_HEAD_TRANSPORT = PROVEN_COMMENT_5589030694
CANONICAL_MERGE = c002bc21706bd90622586d453cb9ed9722d53fbb
PARENT_1 = 4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf
PARENT_2 = ad74fc41dbaa7353b3d3c47b33395b8681e56e82
MERGE_TREE = 270de401bf62a287a11f7c91029e866e5b47b638
POSTMERGE_CI = 34255936220 / run #154 / SUCCESS
POSTMERGE_R3 = 34255936283 / run #131 / SUCCESS
```

Durable post-merge reconciliation comment: `5589241065`.

## Scope not claimed

B301 does not implement or claim:

- B302 encryption-active proof or `cipher_status` qualification;
- B303 temp/WAL/journal/provider/build setting enforcement;
- B304 normal SQLite and SQLCipher cipher/page-authentication integrity checks;
- B305 wrong-key/corruption/unsupported-provider/version fixtures;
- B306 plaintext-spill or public-filename qualification;
- B307 copy-verify-publish migration;
- B401-B406 native platform protector adapters or platform qualification;
- B501-B506 freshness, backup, rotation, or deletion implementation;
- Q009 independent substantive crypto/security review;
- Specification 005 behavior;
- new dependency or donor adoption; or
- release, FIPS, or compliance qualification.

`P011` remains historical NOT PASS and remains unchecked.

## B302 rebound

B302 may begin only after this B301/B302 reconciliation becomes exact-head qualified, reconciled against live `main`/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified.

B302 is bounded to proving, after successful keying of the already reviewed provider, that the opened SQLCipher handle reports encryption active and failing closed otherwise. B302 must not absorb B303-B307 or any later platform/freshness/release scope.