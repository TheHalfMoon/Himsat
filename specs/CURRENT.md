# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_FINAL_DESIGN_REVIEW_PENDING
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
SPEC_004_SHAPING_MERGE = 384608c8fc13531c399f3726caa5022eb3612aa2
SPEC_004_FIRST_REMEDIATION_MERGE = 6d1bbc9b55690939833917eebd447c361627f48a
SPEC_004_ROUND2_REMEDIATION_HEAD = 4357102d388248400116eeab45cf83de71e35051
SPEC_004_ROUND2_PREMERGE_CI = 34057694827_SUCCESS
SPEC_004_ROUND2_PREMERGE_R3 = 34057694832_SUCCESS
SPEC_004_ROUND2_REMEDIATION_MERGE = cb8511c1b420c58f714768c3561e74f04f026b3a
SPEC_004_ROUND2_POSTMERGE_CI = 34057861438_SUCCESS
SPEC_004_ROUND2_POSTMERGE_R3 = 34057861447_SUCCESS
SPEC_004_LAST_REVIEWED_SHA = 6d1bbc9b55690939833917eebd447c361627f48a
SPEC_004_LAST_REVIEW_DISPOSITION = CHANGES_REQUIRED_2_BLOCKERS
SPEC_004_D017_D018 = REMEDIATED_CANONICAL_UNREVIEWED
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
SPEC_004_DESIGN_AUTHORITY = FINAL_REVIEW_ONLY
SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED_PENDING_FINAL_INDEPENDENT_REVIEW_AND_PROVENANCE
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED_PENDING_FINAL_INDEPENDENT_REVIEW
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Live GitHub truth proves Specification 003 is `CLOSED_CANONICAL` at `1f14bbe004962dd164402e6e6c7f9c046cf5b489`.

Specification 004 shaping merged at `384608c8fc13531c399f3726caa5022eb3612aa2`. The first independent review on review-only PR #12 produced 16 actionable findings. D001-D016 were remediated and merged canonically at `6d1bbc9b55690939833917eebd447c361627f48a`, with post-merge CI `34055180993` and R3 `34055180810` successful.

The second independent review on review-only PR #18 reviewed exact SHA `6d1bbc9b55690939833917eebd447c361627f48a` and returned `CHANGES_REQUIRED` with two remaining blockers: D017 Apple Keychain policy completeness and D018 freshness-manifest construction/atomic-anchor completeness. Those findings were remediated in PR #21. Exact head `4357102d388248400116eeab45cf83de71e35051` passed CI `34057694827` and R3 `34057694832`, merged with expected-head protection at `cb8511c1b420c58f714768c3561e74f04f026b3a`, and the canonical merge passed post-merge CI `34057861438` plus R3 `34057861447`.

No independent reviewer has yet dispositioned the resulting canonical D017-D018 remediation revision. Therefore the design is not approved, dependency/provider selection remains blocked, and no 004B implementation authority exists.

## Active objective

Complete the final exact-revision Specification 004A design-review gate without changing the reviewed security semantics:

- reconcile the durable task/state/evidence ledgers with exact live GitHub truth;
- exact-head qualify and canonicalize that ledger-only reconciliation;
- create a review-only head pointing directly at the resulting exact canonical SHA with no review-content commit layered on top;
- obtain a substantive independent crypto/security review of that exact canonical SHA;
- resolve any new blocking finding forward and repeat exact-revision review as required;
- only after a no-unresolved-blocker design disposition, shape the exact dependency/provenance decision leaf before any dependency bytes or 004B implementation enter Himsat.

Active artifacts:

```text
specs/004-vault-key-crypto/spec.md
specs/004-vault-key-crypto/plan.md
specs/004-vault-key-crypto/tasks.md
specs/004-vault-key-crypto/review-evidence.md
specs/004-vault-key-crypto/review-round2-evidence.md
docs/research/2026-09-06-vault-crypto-foundation.md
```

## Authority boundary

Until the final exact-canonical design review has no unresolved blocking finding:

- no crypto, key-management, encrypted-database, blob-encryption, recovery, freshness-anchor, or OS secret-store implementation is authorized;
- no Cargo/native dependency, SQLCipher, libsodium, RustCrypto, or other candidate is adopted merely because it appears in research or planning;
- no database/event-log schema, media journal/chunk store, capture, transcription, document, search, memory, sync, plugin, agent, connector, UI, or release implementation is authorized by Specification 004A;
- no custom cipher/MAC/KDF/PRNG/protocol is authorized;
- no plaintext key-file fallback is authorized;
- the Specification 002 provenance registry remains the machine adoption boundary;
- no donor code is authorized by the founder permission record alone;
- no security/FIPS/compliance/public-superiority claim is authorized.

## Specification 004 review gate

Specification 004 remains recursively split:

```text
004A reviewed cryptographic design
  -> independent exact-revision crypto/security review with no unresolved blocker
  -> exact dependency/provenance decision
  -> 004B encrypted storage foundation implementation
  -> exact implementation review/qualification
  -> closeout
```

Green CI/R3, author self-review, summaries, skipped review, billing-blocked review, old review evidence, or a review of a different SHA are not substitutes for the final independent exact-revision review.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        CLOSED_CANONICAL
      -> 003 Core event/schema foundation  CLOSED_CANONICAL
          -> 004 Vault/key architecture    FINAL_DESIGN_REVIEW_PENDING_R3
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
