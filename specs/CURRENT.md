# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_004_ROUND3_DESIGN_REMEDIATION
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
SPEC_004_LEDGER_RECONCILIATION_MERGE = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
SPEC_004_LEDGER_POSTMERGE_CI = 34058307222_SUCCESS
SPEC_004_LEDGER_POSTMERGE_R3 = 34058307204_SUCCESS
SPEC_004_NEGATIVE_REVIEW_PR = 24
SPEC_004_NEGATIVE_REVIEWED_SHA = 3c3847add4c8f56fe418fc02e62ae735e8239058
SPEC_004_NEGATIVE_REVIEW_COMMENT = 5561981955
SPEC_004_NEGATIVE_REVIEW_DISPOSITION = CHANGES_REQUIRED_B019_B020
SPEC_004_CONFLICTING_REVIEW_PR = 25
SPEC_004_CONFLICTING_REVIEWED_SHA = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
SPEC_004_CONFLICTING_INITIAL_DISPOSITION = APPROVE
SPEC_004_REVIEW_CONFLICT_RECONCILIATION = REQUIRED
SPEC_004_B019_B020 = ROUND3_REMEDIATION_AUTHORED_UNMERGED
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_VALIDATED
SPEC_004_DESIGN_AUTHORITY = ROUND3_REMEDIATION_ONLY
SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED_PENDING_CONSISTENT_NO_BLOCKER_REVIEW_AND_PROVENANCE
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED_PENDING_CONSISTENT_NO_BLOCKER_REVIEW
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Live GitHub truth proves Specification 003 is `CLOSED_CANONICAL` at `1f14bbe004962dd164402e6e6c7f9c046cf5b489`.

Specification 004 shaping merged at `384608c8fc13531c399f3726caa5022eb3612aa2`. The first independent review on PR #12 produced 16 actionable findings. D001-D016 were remediated and merged canonically at `6d1bbc9b55690939833917eebd447c361627f48a`.

The second independent review on PR #18 reviewed exact SHA `6d1bbc9b55690939833917eebd447c361627f48a` and returned `CHANGES_REQUIRED` with D017 Apple Keychain policy completeness and D018 freshness-manifest construction/atomic-anchor completeness blocking. PR #21 remediated those findings at exact head `4357102d388248400116eeab45cf83de71e35051`, which passed CI `34057694827` and R3 `34057694832`, merged with expected-head protection at `cb8511c1b420c58f714768c3561e74f04f026b3a`, and passed post-merge CI `34057861438` plus R3 `34057861447`.

A ledger-only reconciliation later merged canonically at `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121`; exact post-merge CI `34058307222` and R3 `34058307204` succeeded. GitHub comparison proves the reconciliation from `3c3847add4c8f56fe418fc02e62ae735e8239058` to `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121` changed only this state file and `specs/004-vault-key-crypto/tasks.md`, not Specification 004 security semantics.

Independent evidence is currently contradictory and MUST be reconciled conservatively. PR #24 comment `5561981955` reviewed exact canonical SHA `3c3847add4c8f56fe418fc02e62ae735e8239058` and returned `CHANGES_REQUIRED` with two concrete blockers:

- B019: bounded-blob and recovery envelopes lacked normative canonical public-byte layouts, exact total-size/length relations, and complete safe parser rules;
- B020: the freshness-anchor contract lacked an explicit crash-atomic genesis transition for new-vault creation and first accepted restore on a fresh device.

PR #25 later initially returned `APPROVE` for exact SHA `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121`, but the intervening commits did not change the security text implicated by B019/B020. The approval discussed bounded-blob AAD/nonce and established-anchor compare-and-advance behavior but did not identify a canonical public blob/recovery envelope layout or a genesis transition. A focused reconciliation request was posted as PR #25 comment `5562030888`.

Diffcipline negative-evidence discipline therefore requires B019/B020 to remain blocking unless a substantive exact-text reconciliation proves the contracts already existed or the contracts are remediated and a new exact-canonical review returns no blocker. Provider/dependency selection and 004B implementation remain blocked.

## Active objective

Complete the smallest round-3 Specification 004A design-remediation leaf:

- preserve the contradictory review evidence rather than selecting the more convenient disposition;
- freeze canonical public-byte layouts, bounds, authenticated public fields, and safe parser behavior for v1 recovery and bounded-blob envelopes;
- define explicit protected freshness state and crash-atomic genesis for new vaults and first accepted fresh-device restore;
- prohibit interpreting missing/reset protector state as a fresh genesis opportunity;
- freeze complete rotation state invariants and minimum retained-manifest history rules requested as non-blocking review recommendations;
- exact-head qualify the remediation under CI and Diffcipline R3;
- merge only with expected-head protection after live reconciliation;
- require post-merge CI/R3 on the exact new canonical SHA;
- obtain a new substantive independent crypto/security review tied to that exact SHA;
- only after a consistent no-unresolved-blocker disposition, shape 004P dependency/provenance selection.

Active artifacts:

```text
specs/004-vault-key-crypto/spec.md
specs/004-vault-key-crypto/plan.md
specs/004-vault-key-crypto/tasks.md
specs/004-vault-key-crypto/review-evidence.md
specs/004-vault-key-crypto/review-round2-evidence.md
specs/004-vault-key-crypto/review-round3-evidence.md
specs/004-vault-key-crypto/round3-normative-contracts.md
specs/004-vault-key-crypto/tooling-incident-evidence.md
docs/research/2026-09-06-vault-crypto-foundation.md
```

`round3-normative-contracts.md` is a normative Specification 004A amendment. Where it supplies byte layouts, parser rules, genesis transitions, rotation invariants, or retained-manifest rules that `spec.md` left unspecified, it is controlling for the current v1 design-review lineage. It does not grant implementation or dependency authority.

## Authority boundary

Until the new exact-canonical design review has no unresolved blocking finding:

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
  -> reconcile contradictory security evidence
  -> round-3 exact design remediation
  -> independent exact-revision crypto/security review with no unresolved blocker
  -> exact dependency/provenance decision
  -> 004B encrypted storage foundation implementation
  -> exact implementation review/qualification
  -> closeout
```

Green CI/R3, author self-review, summaries, skipped review, billing-blocked review, old review evidence, or an approval that does not reconcile a known contradictory blocking finding are not substitutes for the final consistent independent exact-revision review.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       CLOSED_CANONICAL
      -> 002 Provenance/license/SBOM        CLOSED_CANONICAL
      -> 003 Core event/schema foundation  CLOSED_CANONICAL
          -> 004 Vault/key architecture    ROUND3_DESIGN_REMEDIATION_R3
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
- Negative evidence remains evidence and must not be hidden by a later contradictory summary or approval.
- `NOT RUN`, unavailable, manual-review, unknown, absent, skipped, billing-blocked, and self-review are never equivalent to independent PASS evidence.
- Native SpecGrain lifecycle state comes only from validated tool state.
