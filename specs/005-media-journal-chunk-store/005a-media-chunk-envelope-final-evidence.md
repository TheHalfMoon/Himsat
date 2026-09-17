# 005A Media-Chunk Envelope Final Evidence

## Scope

This document records the Specification 005 first implementation leaf (005A):
the v1 XChaCha20-Poly1305 media-chunk authenticated-encryption envelope with
session/index-bound context, 1 MiB plaintext ceiling, additive
`KeyPurpose::MediaChunk` HKDF domain, and teardown-owned media key slot.
It implements no journal, no manifest binding, and no fault-injection
harness; those remain 005B-005D candidates. It changes no reviewed 004 byte,
adopts no donor code, and adds no dependency, provenance record, SBOM entry,
or workflow change.

## Authority

Specification 005 shaping qualified canonically (shaping merge
`2d7ec3e8e714d493f865ecf445d93d503351e390`, PR #159, pre-merge CI
`35094188292` / R3 `35094188252` SUCCESS, post-merge CI `35096310420` / R3
`35096310425` SUCCESS). The 005 plan permits re-bounding the exact leaf split
after shaping qualifies. This leaf re-bounds prospective candidate 005A
against live main as the first implementation Grain; 005B-005D remain
shaping-level candidates with no implementation authority.

## Change surface

```text
BASE = 2d7ec3e8e714d493f865ecf445d93d503351e390 (origin/main at PR open)
HEAD = f338b3c264103a64a74dcc27248f3badd930ca34
FILES = crates/himsat-core/src/vault_media_chunk.rs (new, +801)
        crates/himsat-core/src/vault_keys.rs (+34)
        crates/himsat-core/src/lib.rs (+3)
TOTAL = 838 insertions, 0 deletions
```

`vault_blob.rs`, `Cargo.toml`, `Cargo.lock`, provenance records, SBOM,
workflows, and all reviewed 004 bytes are unchanged.

## Design (additive-only 004 boundary discipline)

- Own envelope/AAD domain prefixes (`HIMSAT/MEDIA/CHUNK/v1`,
  `HIMSAT/MEDIA/AAD/v1`) with B202-mirrored layout discipline: fixed
  113-byte public header, checked arithmetic, exact-length parser, AAD-bound
  context. A B202 parser rejects media envelopes and this parser rejects
  B202 envelopes (different domains), proven bidirectionally by test.
- Dedicated `MediaChunk` HKDF domain (`HIMSAT/005/MEDIA-CHUNK/v1`) beside the
  reviewed 004 purposes: media keys are disjoint from B202 blob keys under
  the same vault and generation, so a nonce need only be unique within its
  own protocol. No new KDF, no custom primitive.
- Nonce generation, reservation, retry, and collision behavior remain
  B203-owned: callers supply the exact 24-byte nonce, exactly as B202
  requires. The 005B journal discipline integrates reservation later.
- Media keys join `VaultKeyMaterial` teardown (`new`, `set_purpose_key`,
  `is_released`, `release_all`, `Debug` all updated; `teardown()` funnels
  through `release_all`).
- `media_chunk_nonce` parses structure only and documents that callers must
  authenticate via `decrypt_media_chunk` before trusting the nonce.
- Donor posture: transcription-oriented chunking in surveyed donors
  (seconds-scale inference windows, unencrypted session-audio layouts) is
  incompatible with the B202-compatible AEAD binding, so 005A adopts no
  donor code. Donor session-audio/file-queue patterns remain planning inputs
  for the 005B journal.

## Acceptance proof (exact head f338b3c)

```text
PREMERGE_CI = 35099474113_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35099474219_SUCCESS_PULL_REQUEST_ATTEMPT_1
FMT = cargo fmt --check clean (local exact-head run)
CLIPPY = cargo clippy --workspace --all-targets --all-features -- -D warnings clean (local exact-head run)
TESTS = cargo test --workspace --all-targets --locked green: 242 lib passed
  (15 new vault_media_chunk tests) plus all integration suites, 0 failures
COVERAGE = round-trip empty/1B/1KiB/ceiling; oversize rejection before
  encryption; truncation/trailing-data; header/domain/version/suite/purpose
  mutants; generation zero; length-field inconsistency; wrong key; wrong
  vault/session/index/generation transplant; nonce/ciphertext/tag bit flips;
  bidirectional B202 mutual rejection; HKDF vector/separation/teardown
```

## Review reconciliation

```text
OCR_DELEGATE_REVIEW = independent subagent adversarial review of the exact
  3-file diff under ocr delegate rules: NO_BLOCKING_FINDINGS; 3
  non-blocking informational notes, no code change required
LLM_OCR = endpoint unconfigured; no LLM OCR result claimed
SUBMITTED_REVIEWS = none; INLINE_THREADS = 0
QODO = billing-blocked (NOT PASS); CODERABBIT = skipped manual review
  (NOT PASS); CUBIC = neutral (NOT PASS)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

Unavailable, skipped, and neutral review outputs are recorded as NOT PASS,
never as approval, per governance.

## Residual risks (carried, not blockers)

1. `expect()` on statically-guarded infallible conversions mirrors the
   reviewed B202 idiom and is kept for consistency; a future offset edit
   must preserve the header-length guard contiguity.
2. `LengthOverflow` is unreachable defense-in-depth on 64-bit; kept.
3. `media_chunk_nonce` callers (005C inventory binding) must authenticate
   first per the documented contract.

## Merge record

```text
PR = 160
MERGE_METHOD = merge
EXPECTED_HEAD_SHA = f338b3c264103a64a74dcc27248f3badd930ca34
CANONICAL_MERGE = c3a371511b1bad519c2ed57817f3951e67cb6355
MERGE_PARENT_1 = 2d7ec3e8e714d493f865ecf445d93d503351e390
MERGE_PARENT_2 = f338b3c264103a64a74dcc27248f3badd930ca34
MERGE_TREE = 130461403ed9d8e81ef672a6469a8ee338bdc543_EQUALS_ACCEPTED_TREE
```

## Post-merge qualification

```text
POSTMERGE_CI = 35189660586_SUCCESS_PUSH_ATTEMPT_1 (head c3a3715, 17m59s)
POSTMERGE_R3 = 35189660481_SUCCESS_PUSH_ATTEMPT_1 (head c3a3715, 10m23s)
```

005A is post-merge qualified on its exact canonical merge.
