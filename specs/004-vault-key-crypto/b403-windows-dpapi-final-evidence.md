# B403 Windows DPAPI Final Evidence

## Scope

B403 closes only the bounded Windows current-user DPAPI protector adapter and its exact Windows qualification path. It does not qualify Linux Secret Service, freshness anchors, backup/restore, full VRK rotation, deletion, Specification 005, release/FIPS/compliance posture, or Q009 independent final security review.

The accepted implementation changes only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_windows_dpapi.rs
```

## Canonical B402/B403 authority prerequisite

B403 began only after PR #83 resolved the prior conditional authority through exact expected-head merge and post-merge qualification:

```text
RECONCILIATION_PR = 83
ACCEPTED_HEAD = cb34c69c430fc268991c2a0bd203e11c8d862348
ACCEPTED_TREE = 0948d22e3f7a9f57ae4e5da7bf16dfc26261bc30
PREMERGE_CI = 34499410847 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34499410854 / SUCCESS / attempt 1 / pull_request
PREMERGE_WINDOWS_JOB = 102946085980 / SUCCESS
OWNER_RECONCILIATION_REVIEW = 5169541104 / NOT_Q009
EXPECTED_HEAD_BINDING = 5621799767
CANONICAL_MERGE = b2d62613b192c54cc501513ee6bb40059c90817c
POSTMERGE_CI = 34500686685 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34500686751 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 102950412516 / SUCCESS
POSTMERGE_QUALIFICATION_COMMENT = 5622030550
```

That qualification canonically closed B402 and resolved authority to B403 only. It did not authorize B404 or satisfy Q009.

## Dependency and trusted-gate chain

B403 required multiple forward-only dependency leaves because rejected implementation candidates exposed missing safe Windows primitives. The repository did not bypass the dependency/provenance boundary.

- PR #84 hardened the trusted B403A dependency-adoption gate and merged as `5861f65966b76a8a132a4a5a8591a2ff96c74843`.
- PR #85 adopted the initial pinned Windows dependency closure and merged as `37e0ef305ef379aed7b588efd6878608569766ec`. Its first push-triggered CI `34516031395` and R3 `34516031278` failed and remain immutable NOT PASS evidence.
- PR #86 recovered B403A forward-only. Accepted head `f3fed920c0ebedd81d9ed6fed7a9f75e8d4ebd18` passed pre-merge CI `34518876884` and R3 `34518876837`, merged with expected-head protection as `c642029ff8c338b46bb1585ebff58e168e5a8653`, and passed push-triggered CI `34519856675` plus R3 `34519856643` on attempt 1. Durable qualification comment: `5624320027`.
- PR #88 hardened the B403C trusted adoption gate and merged as `05c28faa126953b7fdbbc048fb9247580b8b1c9d`.
- PR #89 adopted the WinSafe token-identity dependency and merged as `27459079f8925cbcf513aaab0e456ed091573bb5`; push-triggered CI `34540368803` and R3 `34540368791` succeeded on attempt 1. Durable qualification comment: `5626715374`.
- PR #91 hardened the B403D file-security dependency gate and merged as `198d0ebe5b8f8c97217fa8c1032fdd0db4ec1137`.
- PR #92 adopted the reviewed Windows file-security dependency closure and merged as `c1d2dfd2b6151e9ee7e71b7693c98ace2e3cdb50`; push-triggered CI `34556698977`, Windows job `103130778710`, and R3 `34556698943` succeeded on attempt 1. Durable qualification comment: `5628855690`.

Detailed dependency/provenance evidence remains in the existing B403A/B403C/B403D evidence documents. No dependency result is promoted beyond the claim supported by its own evidence.

## Preserved rejected implementation evidence

Rejected and failed predecessors remain part of the audit trail and are not reclassified:

```text
PR #87 / b713f30c0318ba26948f547e9ec8ad446175848c = CLOSED_UNMERGED_CHANGES_REQUIRED
PR #90 / 19db9de8749c0d6c3ca906f8b954ed0674addb18 = CLOSED_UNMERGED_CHANGES_REQUIRED
PR #93 / 597577a3b8d6cc8ad6b3b011de7b3fd0bccb9a29 = GREEN_AUTOMATION_NOT_ACCEPTED / path-substitution TOCTOU finding
PR #93 / 67a9770002688b5e66916d6af1dee8dd97109160 = FAILURE_NOT_PASS / 971 added lines exceeded canonical Diffcipline maximum 900
```

The PR #90 findings required delete-by-validated-handle behavior, temporary VRK zeroization, and owner-SID verification. The PR #93 predecessor finding required record I/O to stay bound to the already verified root identity rather than returning to replaceable absolute paths. All repairs were forward-only; no failed head was amended, rebased, force-pushed, rerun into qualification, or silently promoted.

## Accepted implementation revision

```text
IMPLEMENTATION_PR = 93
BASE = c1d2dfd2b6151e9ee7e71b7693c98ace2e3cdb50
HEAD = 43c9e702102ec35ca3bfbd64bc3ec70cb87e6e13
TREE = 7b74367463b3d8bc52163ff985db2d6189605f72
CHANGED_FILES = 2
ADDITIONS = 900
DELETIONS = 0
```

The accepted Windows adapter:

```text
ACCESS_SCOPE = SAME_USER_ACCOUNT
USER_PRESENCE = NOT_REQUIRED
HARDWARE_BACKING = UNKNOWN
APP_EXCLUSIVE = UNSUPPORTED_POLICY
SAME_USER_SESSION = UNSUPPORTED_POLICY
REQUIRED_EACH_HIMSAT_UNLOCK = UNSUPPORTED_POLICY
DPAPI_SCOPE = USER
PLAINTEXT_FALLBACK = NONE
```

The provider-visible filename is derived only from an opaque random protector identifier. Vault ID, key generation, policy, owner tag, and VRK remain inside the DPAPI-protected record. DPAPI additional entropy is domain-separated by the opaque protector identifier. Existing records are binding-validated before use or removal, same-generation replacement with a different VRK fails closed, temporary plaintext record material is zeroized, and removal is performed through the validated file handle.

Storage-root and record ACLs are restricted to the current process-token SID and verified by owner SID plus exact DACL shape. Filesystem objects are opened with reparse-point protections. Record opens, creates, cleanup, and removal remain handle-relative to the verified root identity. The adversarial root-substitution test proves that replacing the configured pathname after verification does not redirect record I/O to the replacement path.

Freshness-anchor and protector-replacement operations remain `UnsupportedPolicy` because they belong to later reviewed leaves. B403 makes no CNG/TPM hardware-backed claim.

## Exact-head pre-merge qualification and review reconciliation

```text
PREMERGE_CI = 34624044015 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34624043503 / SUCCESS / attempt 1 / pull_request
PREMERGE_WINDOWS_JOB = 103344703932 / SUCCESS
LOCAL_PREFLIGHT = PASS
B403_FOCUSED_TESTS = 7 passed / 0 failed
OPEN_REVIEW_THREADS = 0
OWNER_RECONCILIATION_REVIEW = 5181372120 / NOT_Q009 / READY_FOR_B403_LEAF_MERGE
Q009 = UNSATISFIED
```

CodeRabbit's final-head status did not provide an independent substantive final-head approval; the service reported manual review required for this OSS repository. Cubic skipped output and Qodo billing-blocked output are not PASS. Repository-owner reconciliation is governance evidence only and does not satisfy Q009.

## Guarded merge and canonical parentage

Durable pre-merge transport binding is PR #93 comment `5637855910`.

The actual observed merge invocation used:

```text
EXPECTED_HEAD_SHA = 43c9e702102ec35ca3bfbd64bc3ec70cb87e6e13
MERGE_METHOD = merge
```

GitHub returned:

```text
MERGED = true
CANONICAL_MERGE = caa815bdcc33262487a7e7587771d9e7c2e9a2cd
PARENT_1 = c1d2dfd2b6151e9ee7e71b7693c98ace2e3cdb50
PARENT_2 = 43c9e702102ec35ca3bfbd64bc3ec70cb87e6e13
MERGE_TREE = 7b74367463b3d8bc52163ff985db2d6189605f72
```

The merge tree exactly equals the accepted implementation tree.

## Post-merge qualification

```text
POSTMERGE_CI = 34625185689 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34625185699 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 103348466854 / SUCCESS
POSTMERGE_R3_JOB = 103348466889 / SUCCESS
POSTMERGE_QUALIFICATION_COMMENT = 5637993261
```

Canonical merge `caa815bdcc33262487a7e7587771d9e7c2e9a2cd` is therefore exact-post-merge qualified for the bounded B403 Windows DPAPI implementation.

## Preserved limitations

`P011` remains unchecked / NOT PASS. `B305R002` remains unchecked / NOT PASS. Neither historical transport gap is repaired by B403.

Q009 remains unsatisfied. Repository-owner reconciliation, implementation tests, native Windows execution, CI/R3 automation, or review automation do not substitute for the required genuinely independent substantive crypto/security review of the eventual exact Specification 004 implementation revision.

B403 does not authorize release, FIPS/compliance claims, app-exclusive Windows protection, per-Himsat-unlock user-presence enforcement, hardware-backed CNG/TPM claims, Linux behavior, or later freshness/backup/rotation/deletion behavior.

## B404 rebound

B404 may begin only after the B403/B404 evidence-state reconciliation itself becomes exact-head qualified, reconciled against live `main`/base/head/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection and `merge_method = merge`, has exact canonical parentage/tree proven, and then passes original-attempt push-triggered post-merge CI and R3.

After that qualification, B404 is the only authorized implementation leaf. B404 is bounded to a Linux Secret Service adapter reporting `SAME_USER_SESSION` unless the exact provider proves stronger semantics, using only fixed application/service identifiers plus an opaque random protector ID as non-secret lookup attributes, rejecting stronger unproven policy, and providing no plaintext fallback. Any new native dependency requires exact provenance/license/notices/SBOM/closure evidence before canonical adoption. B405-B406, B501-B506, Q009, Specification 005, release/FIPS/compliance claims, and unrelated donor adoption remain out of scope until separately authorized.
