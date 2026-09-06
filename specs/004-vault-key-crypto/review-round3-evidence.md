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

IMPLEMENTATION_AUTHORITY = BLOCKED
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED
```

The review-only PRs were not merge authority. Qodo billing-blocked output and CodeRabbit auto-review skip output are not independent PASS evidence.

## Why the initial PR #25 approval does not erase B019/B020

GitHub comparison from `3c3847add4c8f56fe418fc02e62ae735e8239058` to `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121` changes only:

```text
specs/CURRENT.md
specs/004-vault-key-crypto/tasks.md
```

It does not change `specs/004-vault-key-crypto/spec.md` or any security construction. Therefore a concrete security ambiguity reported against `3c3847add...` remains applicable to `ea7ed384...` unless a later substantive review explicitly demonstrates that the allegedly missing contract already exists in canonical text or the contract is remediated.

The initial PR #25 approval discusses bounded-blob AAD/nonce and freshness compare-and-advance behavior, but does not identify a canonical public bounded-blob/recovery envelope byte layout and does not discuss a freshness-anchor genesis transition. Repository search at the time of conflict reconciliation found no canonical `HIMSAT/BLOB/ENVELOPE/v1` or `install_genesis_freshness_anchor` contract.

Under Diffcipline's negative-evidence rule, the conservative Himsat disposition is therefore `CHANGES_REQUIRED` until B019/B020 are explicitly reconciled or remediated and re-reviewed.

## B019 — canonical public recovery/blob envelopes

PR #24 found that recovery/blob AAD, nonce rules, and plaintext limits existed, but complete public envelope byte serialization was absent.

The blocking ambiguity includes:

- no exact public recovery envelope field order/widths;
- no fixed recovery envelope/version/suite/policy serialization and total-size contract;
- no exact bounded-blob public header/ciphertext-tag serialization;
- no exact bounded-blob total-size and declared-length relation;
- no complete pre-allocation rejection contract for malformed/truncated/trailing attacker input;
- no explicit authenticated binding for every public field that selects context or controls allocation.

`round3-normative-contracts.md` supplies the missing recovery/blob v1 byte layouts, exact bounds, authenticated AAD bindings, checked length arithmetic, parser rejection rules, and required negative evidence.

## B020 — protected freshness genesis

PR #24 found that the existing contract defined only established-anchor compare-and-advance behavior. It did not specify how a new vault or first accepted fresh-device restore creates the initial protected anchor without an ambiguous crash window.

`round3-normative-contracts.md` supplies:

- explicit protected `UNINITIALIZED` versus `PRESENT(FreshnessAnchor)` state;
- a rule that `UNINITIALIZED` is a protected state value and never inferred from a missing item;
- crash-atomic creation of the initial protected protector-state record;
- compare-and-set `install_genesis_freshness_anchor` semantics;
- `AnchorAlreadyInitialized`, `AnchorGenesisFailed`, `FreshnessGenesisConflict`, and `UnsupportedPolicy` behavior;
- exact new-vault and fresh-device-restore genesis order;
- crash before/during/after semantics and reread verification;
- fail-closed missing/reset protector handling.

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

This record is not final approval. After the round-3 security semantics are merged and exact canonical CI/R3 succeed, all earlier reviews — including the initial PR #25 `APPROVE` — are stale as final approval evidence. A new substantive review must examine the exact new canonical SHA and explicitly report no unresolved blocking finding before 004P may begin.
