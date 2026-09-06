# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_SHAPING
ACTIVE_SPECIFICATION = 004-vault-key-crypto
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_001_DISPOSITION = CLOSED_CANONICAL
SPEC_002_DISPOSITION = CLOSED_CANONICAL
SPEC_003_DISPOSITION = CLOSED_CANONICAL
SPEC_003_IMPLEMENTATION_MERGE = 21946f7abc9247cacf784220b1932ab5946c7540
SPEC_003_POST_IMPLEMENTATION_CI = 34047078983_SUCCESS
SPEC_003_CLOSEOUT_HEAD = a098593bd48b8d69aacea2ebdc4da9ed1da2cfb7
SPEC_003_EXACT_CLOSEOUT_CI = 34047401282_SUCCESS
SPEC_003_CLOSEOUT_MERGE = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
SPEC_003_CLOSEOUT_EXPECTED_HEAD_GUARD = PROVEN
SPEC_003_POST_CLOSEOUT_CI = 34047579266_SUCCESS
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
IMPLEMENTATION_AUTHORITY = NONE_DURING_SPEC_004_SHAPING
SPEC_004_DESIGN_AUTHORITY = SHAPING_ONLY
SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED_PENDING_INDEPENDENT_REVIEW_AND_PROVENANCE
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Live GitHub truth proves Specification 003 closeout merged at `1f14bbe004962dd164402e6e6c7f9c046cf5b489` after exact-head CI run `34047401282` succeeded on `a098593bd48b8d69aacea2ebdc4da9ed1da2cfb7`. The merge request was explicitly guarded by that expected head, canonical parentage contains that exact closeout head, and post-closeout CI run `34047579266` completed successfully. Specification 003 is therefore `CLOSED_CANONICAL`.

## Active objective

Shape the smallest safe R3 path for Specification 004:

- freeze protected assets, attacker/failure classes, security goals, non-goals, and metadata/deletion limits;
- define a candidate portable vault-root/key-separation design without inventing cryptographic primitives;
- define OS secret-protector contracts for Apple, Android, Windows, and Linux while preserving platform truth;
- define recovery, rotation, revocation, backup, restore, and deletion semantics;
- define encrypted structured-store and bounded blob-envelope requirements;
- refresh current upstream crypto/platform/storage facts without adopting any dependency;
- require an independent substantive crypto/security review tied to an exact canonical design revision before any crypto/storage implementation;
- require exact provenance/license closure for every later dependency/provider before implementation bytes enter Himsat.

Active artifacts:

```text
specs/004-vault-key-crypto/spec.md
specs/004-vault-key-crypto/plan.md
specs/004-vault-key-crypto/tasks.md
docs/research/2026-09-06-vault-crypto-foundation.md
```

## Authority boundary

During Specification 004 shaping:

- no crypto, key-management, encrypted-database, blob-encryption, recovery, or OS secret-store implementation is authorized;
- no Cargo/native dependency, SQLCipher, libsodium, RustCrypto, or other candidate is adopted merely because it appears in research;
- no database/event-log schema, media journal/chunk store, capture, transcription, document, search, memory, sync, plugin, agent, connector, UI, or release implementation is authorized;
- no custom cipher/MAC/KDF/PRNG/protocol is authorized;
- no plaintext key-file fallback is authorized;
- the Specification 002 provenance registry remains the sole machine adoption boundary and currently authorizes zero third-party entries;
- no security/FIPS/compliance/public-superiority claim is authorized.

## Specification 004 review gate

Specification 004 is recursively split:

```text
004A reviewed cryptographic design
  -> independent exact-revision crypto/security review
  -> exact dependency/provenance decision
  -> 004B encrypted storage foundation implementation
  -> closeout
```

A successful shaping merge alone authorizes preparation and solicitation of the independent review. It does **not** authorize 004B implementation. If no substantively independent reviewer is available, record `BLOCKED_EXTERNAL_REVIEW`; author self-review, CI success, skipped review, or billing-blocked review is not a substitute.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        CLOSED_CANONICAL
      -> 003 Core event/schema foundation  CLOSED_CANONICAL
          -> 004 Vault/key architecture    SHAPING_R3
              -> 005 Crash-safe media journal/chunk store  BLOCKED
                  -> 006 Capture abstraction + Capture Health
                      -> 007 macOS capture
                      -> ...
```

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- No donor code may be copied without exact machine-readable provenance plus bounded adoption authority.
- Model/asset/data licensing is independent of engine/software licensing.
- `NOT RUN`, unavailable, manual-review, unknown, absent, skipped, billing-blocked, and self-review are never equivalent to independent PASS evidence.
- Native SpecGrain lifecycle state comes only from validated tool state.
