# B501A Authenticated Manifest Codec Evidence

## Scope

B501A introduces only the canonical v1 authenticated freshness-manifest codec: exact public envelope/AAD/plaintext layouts, XChaCha20-Poly1305 under the existing freshness-manifest HKDF purpose key, canonical validation, exact-envelope SHA-256 hashing, and a bounded round-trip/tamper smoke test. It does not implement OS freshness-anchor persistence, genesis installation, compare-and-advance, restore, rotation, backup, deletion, or Q009.

## Canonical authority prerequisite

B501 authority is active only because B406/B501 reconciliation merge `115ab8c71efa29789e98cf58201e1c1b86d8e949` passed push CI `34662382909` and R3 `34662382776` on attempt 1, including Windows Rust job `103467376947`. Durable transition qualification is recorded on PR #102.

## Contract boundaries

The codec rejects unsupported envelope version/suite, zero generation/epoch, invalid envelope lengths, truncation/trailing bytes, context mismatch, authentication failure, non-canonical generation/inventory ordering, invalid rotation-state shapes, inconsistent generation references, duplicate logical IDs or retained blob nonces, invalid kind metadata, and generic-blob manifest lengths outside the Round 4 canonical full-envelope range. Manifest plaintext encode/decrypt buffers are zeroized after use.

The codec does not select canonical filesystem objects by itself. Round 4 full stored-object length/hash/parser/context verification remains a later B501 qualification leaf before any inventory entry may release plaintext or control canonical object selection.

## Platform and authority limits

B501A changes no platform protector. Apple, Android, Windows, and Linux freshness-anchor methods remain `UnsupportedPolicy` unless a later exact provider-specific leaf proves protected serialized compare-and-set plus crash old-or-new semantics. No rollback resistance, atomic anchor, hardware backing, release, FIPS, or compliance claim is made here.

Canonical acceptance requires exact-head CI/R3, live review/thread reconciliation, expected-head guarded merge, exact parent/tree proof, and original-attempt post-merge CI/R3. Q009 remains UNSATISFIED until a genuinely independent substantive crypto/security review is tied to the exact final Specification 004 implementation revision.
