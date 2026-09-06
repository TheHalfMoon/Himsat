# Specification 004A — Third-Review Evidence and Conflict Reconciliation

## Exact evidence lineage

```text
NEGATIVE_REVIEW_ONLY_PR = 24
NEGATIVE_REVIEWED_SHA = 3c3847add4c8f56fe418fc02e62ae735e8239058
NEGATIVE_REVIEWER = coderabbitai[bot]
NEGATIVE_REVIEW_COMMENT = 5561981955
NEGATIVE_DISPOSITION = CHANGES_REQUIRED
NEGATIVE_BLOCKING_FINDINGS = B019,B020

LEDGER_RECONCILIATION_MERGE = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
INTERVENING_SECURITY_SEMANTICS_CHANGE = NONE

CONFLICTING_REVIEW_ONLY_PR = 25
CONFLICTING_REVIEWED_SHA = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
CONFLICTING_INITIAL_DISPOSITION = APPROVE
CONFLICTING_INITIAL_BLOCKERS = NONE
FOCUSED_RECONCILIATION_REQUEST = 5562030888
FOCUSED_RECONCILIATION_RESPONSE = 5562034608
FOCUSED_RECONCILIATION_DISPOSITION = CHANGES_REQUIRED
FOCUSED_RECONCILIATION_B019 = BLOCKING
FOCUSED_RECONCILIATION_B020 = BLOCKING
PR25_INITIAL_APPROVAL_STATUS = WITHDRAWN_BY_FOCUSED_RECONCILIATION

IMPLEMENTATION_AUTHORITY = BLOCKED
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED
```

The review-only PRs were not merge authority. Qodo billing-blocked output and CodeRabbit auto-review skip output are not independent PASS evidence.

## Conflict reconciliation result

The focused CodeRabbit reconciliation on PR #25 explicitly re-examined B019 and B020 against exact SHA `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121` after being shown the contradictory PR #24 evidence and the exact GitHub comparison.

The reviewer stated that the previous `APPROVE` disposition was not valid after reconciliation and returned:

```text
REVIEWED_SHA = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
DISPOSITION = CHANGES_REQUIRED
B019 = BLOCKING
B020 = BLOCKING
```

The reviewer confirmed that D017 and D018 remain substantively resolved and classified B019/B020 as separate unresolved design gaps. Provider selection, dependency adoption, and 004B implementation therefore remain blocked.

## Why the initial PR #25 approval did not erase B019/B020

GitHub comparison from `3c3847add4c8f56fe418fc02e62ae735e8239058` to `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121` changes only:

```text
specs/CURRENT.md
specs/004-vault-key-crypto/tasks.md
```

It does not change `specs/004-vault-key-crypto/spec.md` or any security construction. Therefore a concrete security ambiguity reported against `3c3847add...` remained applicable to `ea7ed384...`.

The initial PR #25 approval discussed bounded-blob AAD/nonce and established-anchor compare-and-advance behavior, but did not identify a canonical public bounded-blob/recovery envelope byte layout and did not identify a freshness-anchor genesis transition. The focused reconciliation confirmed both omissions as blocking.

This is now reconciled evidence, not an unresolved reviewer disagreement: the later focused disposition supersedes the initial incomplete approval for the same exact SHA.

## B019 — canonical public recovery/blob envelopes

The focused reconciliation confirmed that recovery/blob AAD, nonce rules, and plaintext limits existed, but complete public envelope byte serialization was absent.

The blocking ambiguity included:

- no exact public recovery envelope field order/widths;
- no fixed recovery envelope/version/suite/policy serialization and total-size contract;
- no exact bounded-blob public header/ciphertext-tag serialization;
- no exact bounded-blob total-size and declared-length relation;
- no complete pre-allocation rejection contract for malformed/truncated/trailing attacker input;
- no explicit authenticated binding for every public field that selects context or controls allocation;
- no exact statement tying manifest blob inventory hashes to the complete canonical stored blob-envelope bytes.

`round3-normative-contracts.md` supplies the missing recovery/blob v1 byte layouts, exact bounds, authenticated AAD bindings, checked length arithmetic, parser rejection rules, and required negative evidence. The existing manifest contract already defines `ciphertext_sha256` as SHA-256 of exact canonical stored ciphertext/envelope bytes; the round-3 review must verify that this means the complete canonical bounded-blob envelope from the first domain byte through the final AEAD tag byte and that `ciphertext_length` is the exact complete envelope length.

## B020 — protected freshness genesis

The focused reconciliation confirmed that the existing contract defined only established-anchor compare-and-advance behavior. It did not specify how a new vault or first accepted fresh-device restore creates the initial protected anchor without an ambiguous crash window.

The reviewer requested an explicit absent-anchor expected state. The remediation deliberately uses a stricter protected-state model:

```text
UNINITIALIZED
PRESENT(FreshnessAnchor)
```

`UNINITIALIZED` is an explicit value inside a successfully created protected protector-state record. It is not inferred from a missing/reset/lost secure-store item. This preserves the reviewer's required compare-and-set genesis property while preventing loss of protected state from being misclassified as a fresh-device bootstrap opportunity.

`round3-normative-contracts.md` supplies:

- explicit protected `UNINITIALIZED` versus `PRESENT(FreshnessAnchor)` state;
- a rule that `UNINITIALIZED` is a protected state value and never inferred from a missing item;
- crash-atomic creation of the initial protected protector-state record;
- compare-and-set `install_genesis_freshness_anchor` semantics;
- `AnchorAlreadyInitialized`, `AnchorGenesisFailed`, `FreshnessGenesisConflict`, and `UnsupportedPolicy` behavior;
- exact new-vault and fresh-device-restore genesis order;
- crash before/during/after semantics and reread verification;
- fail-closed missing/reset protector handling;
- preservation of the user-visible residual risk that a fresh device cannot prove the accepted authenticated backup is globally newest.

## Non-blocking recommendations proactively resolved

The negative PR #24 review also recommended:

- complete rotation phase/generation invariants;
- explicit retained-manifest history-set lifetime for nonce collision defense and crash recovery.

Both are frozen normatively in `round3-normative-contracts.md` to reduce another ambiguity cycle.

## Residual risks retained

Round-3 remediation does not weaken or relabel the established residual risks:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel/firmware/hardware/physical-memory compromise remains outside the portable vault boundary;
- ciphertext size/count/filesystem/timing/upload/rotation metadata can leak;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove an authenticated backup is globally newest without prior trusted freshness state;
- crypto-erasure does not prove physical/provider/snapshot/user-copy deletion;
- detached recovery-enabled backups remain usable by a holder of the recovery passphrase.

## Required successor evidence

This record is not final approval. After the round-3 security semantics are merged and exact canonical CI/R3 succeed, all reviews of older SHAs are stale as final approval evidence. A new substantive review must examine the exact new canonical SHA, explicitly assess B019/B020 and the protected `UNINITIALIZED` genesis model, and report no unresolved blocking finding before 004P may begin.
