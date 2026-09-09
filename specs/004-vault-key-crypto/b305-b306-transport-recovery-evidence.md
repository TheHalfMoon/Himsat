# B305/B306 Transport-Recovery Evidence

## Disposition entering recovery

Specification 004 B305 remains canonical-qualified. PR #70 canonicalized the B305 final evidence and authored the conditional B306 frontier, but the actual concurrent merge request body is not reconstructible from GitHub's post-merge surfaces.

```text
B305_B306_RECONCILIATION_PR = 70
BASE = e27a0b2921af2661037b7af581f6be737c1352de
HEAD = 954948554b371d80d9473a155a48ea7f31edcdf7
TREE = b58acfdae5d9df89833d30413623130c80b67461
PREMERGE_CI = 34290128656 / run #181 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34290128652 / run #158 / SUCCESS / attempt 1 / pull_request
CANONICAL_MERGE = 7cdb8154ec90c59b64b2b47b37111cb491489e92
PARENT_1 = e27a0b2921af2661037b7af581f6be737c1352de
PARENT_2 = 954948554b371d80d9473a155a48ea7f31edcdf7
MERGE_TREE = b58acfdae5d9df89833d30413623130c80b67461
POSTMERGE_R3 = 34291064716 / run #159 / SUCCESS / attempt 1 / push
POSTMERGE_CI = 34291064751 / run #182 / SUCCESS / attempt 1 / push
POSTMERGE_DISPOSITION_COMMENT = 5593473838
```
## Missing transport proof

Before merge, transport-intent comments `5593348515` and `5593354365` stated that the merge would use exact `expected_head_sha = 954948554b371d80d9473a155a48ea7f31edcdf7`. Exact parentage later proved that this head became parent 2.

Those facts do not reconstruct the actual merge API request body. The repository therefore does not claim that PR #70 itself satisfied the expected-head transport gate. B305R002 remains unchecked and NOT PASS for that missing proof, by the same conservative evidence rule applied to historical `P011`.

The duplicate repository-owner COMMENT submissions created around this transition are also not independent security review and do not satisfy Q009.

## Recovery rule

The recovery is forward-only. It does not amend PR #70 history, rewrite Git history, or retroactively manufacture transport evidence.

A recovery revision may open B306 only after it independently proves all of:

1. exact-head CI and R3 SUCCESS;
2. live reconciliation of `main`, head/base, exact diff, reviews, threads, comments, and mergeability;
3. an observed merge invocation using the exact recovery head as `expected_head_sha`;
4. exact canonical parentage; and
5. exact push-triggered post-merge CI and R3 SUCCESS.
## Recovery scope

This recovery may change only Specification 004 evidence/state surfaces needed to record the observed transport gap and re-bind B306 conditionally. It must not change runtime code, cryptographic semantics, Cargo manifest/lockfile, dependency bytes, provider selection, provenance entries, generated artifacts, workflows, donor material, B307 migration, B401-B406 platform behavior, B501-B506 behavior, Specification 005 behavior, or release/FIPS/compliance posture.

`P011` remains historical unchecked NOT PASS. Q009 remains unsatisfied and still requires genuinely independent substantive crypto/security review of the exact implementation revision.

## B306 boundary after recovery qualification

Only after this recovery itself becomes exact-head qualified, observed expected-head guarded, parentage-proven, and exact-post-merge qualified may B306 begin.

B306 remains bounded to SQLCipher plaintext-spill qualification: genuine semantic markers and logical IDs must be absent from encrypted DB/WAL/rollback-journal/file-backed-temp bytes and public filenames under the already qualified configuration. `temp_store = MEMORY` must remain positively proven. Unavailable evidence is NOT PROVEN, not PASS. B306 does not absorb B307 or later platform/freshness work.

## Canonical recovery qualification

The forward-only recovery completed on PR #71 without changing runtime/security semantics, dependencies, provider/provenance records, workflows, donor material, Specification 005 behavior, P011, or Q009.

```text
RECOVERY_PR = 71
RECOVERY_BASE = 7cdb8154ec90c59b64b2b47b37111cb491489e92
RECOVERY_HEAD = b7f2886a4bbe641aeff7ea071cf261b5c7fd5c6a
RECOVERY_TREE = 18c69932b809998292a0e67dc8e667c34312b48e
PREMERGE_CI = 34292555783 / run #183 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34292555765 / run #160 / SUCCESS / attempt 1 / pull_request
OWNER_RECONCILIATION_REVIEW = 5148286720 / COMMENTED / NOT_Q009
PREMERGE_TRANSPORT_BINDING_COMMENT = 5593654712
EXPECTED_HEAD_SHA = b7f2886a4bbe641aeff7ea071cf261b5c7fd5c6a
MERGE_METHOD = merge
ACTUAL_MERGE_RESULT = merged=true
CANONICAL_MERGE = c22385e0245d9b372b396bbe9a2d158d30b40052
PARENT_1 = 7cdb8154ec90c59b64b2b47b37111cb491489e92
PARENT_2 = b7f2886a4bbe641aeff7ea071cf261b5c7fd5c6a
MERGE_TREE = 18c69932b809998292a0e67dc8e667c34312b48e
POSTMERGE_CI = 34293423595 / run #184 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34293423619 / run #161 / SUCCESS / attempt 1 / push
POSTMERGE_QUALIFICATION_COMMENT = 5593744878
```

The actual merge operation was observed in the execution context that performed PR #71's merge and returned the canonical merge SHA above while using the exact recovery head as `expected_head_sha`. Exact canonical parentage and tree independently match that accepted head.

Qodo remained billing-blocked and CodeRabbit remained auto-skipped; neither is PASS. The repository-owner reconciliation remains governance evidence only and is explicitly `NOT_Q009`. `P011` remains unchecked / NOT PASS. B305R002 remains unchecked / NOT PASS because PR #70's historical merge request body remains unavailable. B305R003 is independently proven by exact parentage and post-merge CI/R3 and may be checked without changing B305R002.

With B305X001-B305X003 now proven, the transport recovery is canonical-qualified. B306 may proceed only after canonical state reconciliation records this completed recovery and rebinds implementation authority to the bounded B306 plaintext-spill qualification leaf.
