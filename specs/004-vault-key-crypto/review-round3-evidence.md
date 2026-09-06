# Specification 004A — Third Independent Review Evidence

## Exact reviewed revision

```text
REVIEW_ONLY_PR = 24
REVIEWED_SHA = 3c3847add4c8f56fe418fc02e62ae735e8239058
REVIEWER = coderabbitai[bot]
REVIEW_EVIDENCE_COMMENT = 5561981955
DISPOSITION = CHANGES_REQUIRED
BLOCKING_FINDINGS = B019,B020
IMPLEMENTATION_AUTHORITY = BLOCKED
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED
```

The review-only PR head pointed directly to exact canonical SHA `3c3847add4c8f56fe418fc02e62ae735e8239058` with no review-content commit. The canonical SHA had already passed post-canonical CI `34058138372` and R3 Security Gate `34058138362`.

The automated “review skipped” status for the non-default review base is not evidence. The substantive manual `@coderabbitai review` response in comment `5561981955` is the independent evidence used here.

## Blocking finding B019 — public recovery/blob envelope serialization

The reviewer found that Specification 004A defined recovery/blob AAD, nonce rules, and plaintext limits but did not freeze complete public envelope byte layouts comparable to the manifest format.

Required remediation:

- separate canonical recovery and bounded-blob v1 layouts;
- exact field order and widths;
- exact version/suite/policy/purpose identifiers;
- exact nonce and ciphertext/tag length relations;
- total-size bounds;
- safe rejection of invalid/truncated/unknown/trailing input before unbounded allocation or plaintext release;
- authentication binding for every security-relevant public field.

Round-3 remediation is defined normatively in `round3-normative-contracts.md`.

## Blocking finding B020 — freshness genesis transition

The reviewer found that the existing freshness contract had no normative transition from a fresh protected state to the first anchor. The design rejected zero-epoch anchors and defined only `PRESENT -> PRESENT` compare-and-advance behavior, leaving new-vault creation and first fresh-device restore ambiguous.

Required remediation:

- explicit protected genesis state;
- atomic install of the first anchor;
- bind the verified manifest hash;
- reject replacement of an established anchor;
- old-or-new crash semantics;
- reread verification;
- typed failures;
- equivalent explicit handling of the first accepted restore on a fresh device;
- `UnsupportedPolicy` when the platform cannot prove the required protected-state semantics.

Round-3 remediation is defined normatively in `round3-normative-contracts.md`.

## Non-blocking recommendation R001 — rotation invariants

The reviewer recommended freezing complete `rotation_phase`, `rotation_target_generation`, generation-table state, and phase-transition invariants to reduce interrupted-rotation ambiguity.

Round-3 remediation addresses this recommendation normatively rather than deferring it.

## Non-blocking recommendation R002 — retained manifest set

The reviewer recommended defining which manifest envelopes are retained and for how long because manifest nonce collision checks depend on the retained set.

Round-3 remediation defines the minimum retained history set, retirement points, and the fact that this scan is defense in depth rather than a permanent global nonce ledger.

## D001-D018 status reported by the reviewer

```text
D001-D008 = SUBSTANTIVELY_RESOLVED, subject to B019 for recovery serialization
D009 = NOT_SUBSTANTIVELY_RESOLVED because exact public recovery/blob envelopes were absent
D010 = SUBSTANTIVELY_RESOLVED, with B019 still blocking safe envelope implementation
D011 = PARTIALLY_RESOLVED because genesis was undefined
D012-D016 = SUBSTANTIVELY_RESOLVED
D017 = SUBSTANTIVELY_RESOLVED
D018 = PARTIALLY_RESOLVED because genesis was undefined
```

## Residual risks retained

The review retained these non-blocking residual risks:

- a compromised unlocked process can access plaintext and resident secrets;
- kernel/firmware/hardware/physical-memory compromise remains outside the portable boundary;
- ciphertext size/count/filesystem/timing/upload/rotation metadata can leak;
- offline recovery-passphrase attacks remain possible;
- a fresh device cannot prove an authenticated backup is globally newest without prior trusted state;
- crypto-erasure does not prove physical/provider/snapshot/user-copy deletion;
- detached backups with a recovery-wrapped VRK remain usable by a holder of the recovery passphrase.

## Governance disposition

The exact reviewed SHA remains `CHANGES_REQUIRED`. It cannot authorize 004P or 004B.

The round-3 security semantics invalidate this review as final approval evidence. After round-3 remediation merges and the resulting canonical SHA passes CI/R3, a new exact-SHA substantive independent review is mandatory. Only a no-blocking-finding disposition on that new exact canonical SHA may open 004P.
