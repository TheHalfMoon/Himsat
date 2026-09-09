# B306 Authority Transport-Recovery Evidence

## Incident disposition

PR #72 canonically recorded the already-qualified B305/B306 recovery and authored `SPEC_004_IMPLEMENTATION_AUTHORITY = B306_ONLY`, but its actual merge API request body is not reconstructible from GitHub's post-merge surfaces.

```text
RECONCILIATION_PR = 72
BASE = c22385e0245d9b372b396bbe9a2d158d30b40052
HEAD = 9d0b95b056eee74cb38565fd85079f09cf942050
TREE = d32384fe6785c65edfee140c8ef7e33d4c5e3c8e
PREMERGE_CI = 34295676449 / run #185 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34295676425 / run #162 / SUCCESS / attempt 1 / pull_request
OWNER_RECONCILIATION_REVIEW = 5148527565 / NOT_Q009
PREMERGE_TRANSPORT_BINDING_COMMENT = 5594048202
CANONICAL_MERGE = 45c23656082477354c7b96ff4e4774e549a43cab
PARENT_1 = c22385e0245d9b372b396bbe9a2d158d30b40052
PARENT_2 = 9d0b95b056eee74cb38565fd85079f09cf942050
MERGE_TREE = d32384fe6785c65edfee140c8ef7e33d4c5e3c8e
POSTMERGE_CI = 34296649094 / run #186 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34296649118 / run #163 / SUCCESS / attempt 1 / push
POSTMERGE_NEGATIVE_EVIDENCE_COMMENT = 5594168791
ACTUAL_MERGE_REQUEST_BODY = NOT_RECONSTRUCTIBLE_POST_HOC
EXPECTED_HEAD_SHA_TRANSPORT_PROOF = NOT_PROVEN
```

The pre-merge binding, exact parentage/tree, and successful post-merge workflows do not reconstruct the actual merge request body. PR #72 is therefore not retroactively credited with expected-head transport proof.

## Authority hold

Until this forward-only recovery is itself qualified, the B306 authority text already present in canonical `specs/CURRENT.md` is held as transport-unqualified and MUST NOT be used to begin B306 implementation.

This recovery does not rewrite PR #72, alter its canonical tree, or manufacture historical evidence. It repairs only the missing transport qualification by adding new forward evidence.

## Recovery acceptance

This exact recovery revision may release the existing B306-only authority only after all of the following are proven:

1. original-attempt pull-request CI and R3 reach terminal SUCCESS on the exact recovery head;
2. live reconciliation confirms canonical `main`, exact base/head/diff, reviews, review threads, comments, and mergeability;
3. the actual merge operation is observed using the exact recovery head as `expected_head_sha` with `merge_method = merge`;
4. the resulting canonical merge has exact parentage and the exact recovery tree; and
5. original-attempt push-triggered post-merge CI and R3 reach terminal SUCCESS on the exact canonical merge.

After all five conditions are proven, no further state mutation is required: this recovery solely repairs transport qualification for the already-canonical PR #72 state, and the existing `SPEC_004_IMPLEMENTATION_AUTHORITY = B306_ONLY` becomes qualified by live repository truth.

## Recovery scope

This recovery changes only this evidence document. It does not change runtime or cryptographic semantics, `specs/CURRENT.md`, `tasks.md`, Cargo manifests/lockfiles, dependency bytes, provider selection, provenance entries, generated artifacts, workflows, donor material, B306 implementation bytes, B307 migration, B401-B406 platform protectors, B501-B506 freshness/backup/rotation/deletion behavior, Specification 005 behavior, release/FIPS/compliance posture, or any historical evidence disposition.

`P011` remains unchecked / NOT PASS. `B305R002` remains unchecked / NOT PASS. Q009 remains unsatisfied and still requires a genuinely independent substantive crypto/security review of the exact implementation revision. Repository-owner review, CI/R3, billing-blocked/skipped automation, neutral automation output, or this recovery do not satisfy Q009.
