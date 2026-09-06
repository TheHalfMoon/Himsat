# Specification 004A — Round 4 Bounded-Blob Inventory Contract

## Normative status

This document is a **normative amendment to Specification 004A v1** and MUST be read together with `spec.md` and `round3-normative-contracts.md`.

For `GENERIC_ARTIFACT_BLOB` manifest inventory semantics, this document is controlling and supersedes any more general or ambiguous interpretation of `ciphertext_length` or `ciphertext_sha256` in `spec.md`.

This amendment closes only blocking finding B019-1 from the independent review of canonical SHA `c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6` on review-only PR #27, comment `5562237044`.

It does **not** authorize dependency adoption, provider selection, 004B implementation, product features, donor-code adoption, or release work. Any security-semantic change to this contract invalidates earlier final-review evidence and requires a new exact-SHA independent security review.

```text
BASE_CANONICAL_SHA = c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6
REVIEW_ONLY_PR = 27
REVIEW_COMMENT = 5562237044
REVIEWED_SHA = c69684df26d7c9b5c7416ad2f0ecb8fe8114fbc6
REVIEW_DISPOSITION = CHANGES_REQUIRED
BLOCKING_FINDING = B019-1
B020_STATUS = RESOLVED
D001_D018_STATUS = RESOLVED_NO_REGRESSION
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
```

## Canonical `GENERIC_ARTIFACT_BLOB` manifest inventory semantics

For every authenticated manifest inventory record whose `object_kind == GENERIC_ARTIFACT_BLOB`, the following requirements are normative.

### Canonical stored-object boundary

The stored object is exactly one complete canonical v1 bounded-blob envelope defined by `round3-normative-contracts.md`.

Its canonical byte range begins with the **first byte** of:

```text
domain("HIMSAT/BLOB/ENVELOPE/v1")
```

and ends with the **final byte of the XChaCha20-Poly1305 AEAD tag**.

No envelope byte may be excluded from the canonical stored object. No external or separately stored public header, trailer, transport framing, filename, filesystem metadata, provider metadata, allocation padding, or container metadata is part of the canonical blob-envelope byte range.

### `ciphertext_length`

For `GENERIC_ARTIFACT_BLOB`:

```text
manifest.ciphertext_length
    = len(exact_complete_canonical_blob_envelope_bytes)
    = 107 + envelope.ciphertext_and_tag_length
```

Therefore `manifest.ciphertext_length` MUST be in the inclusive range:

```text
123 .. 67_108_987
```

and MUST equal the actual stored byte length of the complete canonical bounded-blob envelope.

It MUST NOT mean or be derived from any of the following alternate quantities:

- plaintext length;
- ciphertext-plus-tag length alone;
- payload-only length;
- filesystem allocation length;
- transport/container framing length;
- provider object size including external metadata;
- any subset or transformed representation of the canonical envelope.

### `ciphertext_sha256`

For `GENERIC_ARTIFACT_BLOB`:

```text
manifest.ciphertext_sha256
    = SHA-256(exact_complete_canonical_blob_envelope_bytes)
```

The hash input is exactly the same `manifest.ciphertext_length` bytes, starting at the first byte of `domain("HIMSAT/BLOB/ENVELOPE/v1")` and ending at the final AEAD tag byte.

The hash MUST NOT omit the public envelope header, nonce, authenticated public fields, ciphertext, or tag. It MUST NOT include a filename, filesystem metadata, provider metadata, transport framing, padding outside the canonical envelope, or any decoded plaintext representation.

Hashing only ciphertext-plus-tag, payload-only bytes, a parsed/re-serialized representation, or any other subset/alternate representation is non-conforming.

## Manifest inventory verification order

Before a `GENERIC_ARTIFACT_BLOB` inventory entry is accepted as consistent with the authenticated manifest, the verifier MUST perform all of the following against the **same exact stored byte sequence**:

1. resolve the object using only the already authenticated manifest storage identifier;
2. read the exact stored object bytes without interpreting external metadata as part of the canonical envelope;
3. require the actual stored byte count to equal `manifest.ciphertext_length`;
4. compute SHA-256 over exactly those stored bytes and require equality with `manifest.ciphertext_sha256`;
5. parse those same bytes as exactly one canonical v1 bounded-blob envelope under `round3-normative-contracts.md`, including exact domain, fixed identifiers, checked length relations, no truncation, and no trailing bytes;
6. require the parsed envelope `VaultId` to equal the authenticated manifest `VaultId`;
7. require the parsed envelope `ArtifactId` to equal the inventory record `logical_id`;
8. require the parsed envelope `key_generation` to equal the inventory record `object_key_generation`;
9. require the parsed envelope total byte length to equal both the actual stored byte count and `manifest.ciphertext_length`;
10. only after the inventory, parser, context, and AEAD checks succeed may plaintext be released.

A failure of any required equality, hash, canonical parse, authenticated-context binding, or AEAD authentication is `CorruptOrTampered`, releases no plaintext, and MUST NOT be repaired by hashing, parsing, or authenticating a subset or alternate representation.

The manifest verifier MUST NOT accept an entry merely because the ciphertext-plus-tag authenticates if the full canonical-envelope length/hash checks fail.

## Structured-store boundary

This amendment changes no `STRUCTURED_STORE` inventory semantics. SQLCipher/provider/build/provenance selection remains a later 004P decision and no SQLCipher dependency is adopted by this document.

## Required 004B negative evidence

Later 004B qualification MUST include deterministic fixtures that prove rejection when:

- the public bounded-blob header is changed while ciphertext-plus-tag is unchanged;
- only ciphertext-plus-tag is hashed and stored in `ciphertext_sha256`;
- only ciphertext-plus-tag length is stored in `ciphertext_length`;
- a canonical header byte is omitted from the hash input;
- bytes are prepended or appended outside the canonical envelope;
- `VaultId`, `ArtifactId`, or key generation differs between envelope and authenticated inventory entry;
- an identical ciphertext-plus-tag is paired with a different authenticated public envelope header;
- the stored-object byte count, manifest length, parsed envelope length, and manifest hash do not all agree;
- a verifier attempts to accept an authenticated payload after a full-envelope inventory mismatch.

Every such mismatch MUST fail before manifest inventory acceptance and release no plaintext.

## Acceptance

B019-1 is not closed merely because this amendment exists. This remediation must:

1. pass exact-head CI and Diffcipline R3 without policy relaxation;
2. be reconciled against live `main`, exact diff, reviews, review threads, and comments immediately before merge;
3. merge only with explicit `expected_head_sha` protection;
4. pass fresh post-merge CI and R3 on the exact canonical merge SHA;
5. receive a new substantive independent security review tied to that exact canonical SHA;
6. have no unresolved blocking design finding before 004P begins.
