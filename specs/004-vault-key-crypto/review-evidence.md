# Specification 004A Independent Security Review Evidence

## Review identity

```text
REVIEW_ONLY_PR = 12
REVIEW_BASE_SHA = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
REVIEWED_CANONICAL_SHA = 384608c8fc13531c399f3726caa5022eb3612aa2
REVIEWER = coderabbitai
GITHUB_REVIEW_ID = PRR_kwDOUQPwRs8AAAABMYy3gw
GITHUB_REVIEW_STATE = COMMENTED
SUBMITTED_AT = 2026-09-06T18:47:41Z
ACTIONABLE_FINDINGS = 16
HIMSAT_GOVERNANCE_DISPOSITION = CHANGES_REQUIRED
IMPLEMENTATION_AUTHORITY = BLOCKED
```

PR #12 is review-only and its head points directly at canonical shaping merge `384608c8fc13531c399f3726caa5022eb3612aa2`. No review-request commit was layered on top of the reviewed design. CodeRabbit explicitly reviewed the compare from `1f14bbe004962dd164402e6e6c7f9c046cf5b489` to `384608c8fc13531c399f3726caa5022eb3612aa2` and selected all seven Specification 004 shaping paths.

The GitHub review state is `COMMENTED`; CodeRabbit did **not** submit a GitHub `REQUEST_CHANGES` or `APPROVE` state. Himsat assigns `CHANGES_REQUIRED` as its governance disposition because the substantive independent review posted 16 actionable findings, including multiple security-major findings. This document does not misrepresent the reviewer's GitHub state.

## Exact qualification context

The reviewed canonical shaping head had already passed:

```text
PRE_MERGE_CI = 34048208674_SUCCESS
PRE_MERGE_R3 = 34048208667_SUCCESS
SHAPING_MERGE = 384608c8fc13531c399f3726caa5022eb3612aa2
POST_MERGE_CI = 34048394581_SUCCESS
POST_MERGE_R3 = 34048394550_SUCCESS
REVIEW_ONLY_PR_CI = 34052147796_SUCCESS
REVIEW_ONLY_PR_R3 = 34052147791_SUCCESS
```

CI/R3 success is qualification evidence, not a substitute for the independent security review.

## Unavailable or non-qualifying review systems

- Qodo reported billing blocked; not PASS.
- Cubic reported its monthly AI review line limit was exhausted and returned a neutral check; not PASS.
- Earlier CodeRabbit automatic review on shaping PR #11 was skipped; not PASS.
- The substantive CodeRabbit review on PR #12 is the independent review evidence used here.

## Finding classification

All 16 actionable findings are blocking **004B implementation** until the design is remediated, exact-head qualified, canonical, and independently re-reviewed. Some findings were marked minor by the reviewer, but they affect the review/provenance contract and therefore remain blocking for this R3 design gate.

| ID | Surface | Finding | Classification |
| --- | --- | --- | --- |
| D001 | `plan.md` | Define fail-closed CSPRNG contract for VRK and nonces | BLOCKING |
| D002 | `plan.md` | Define OS protector ownership/access/user-presence scope | BLOCKING |
| D003 | `plan.md` | Require exact SQLCipher artifact/provider/build provenance before P002 | BLOCKING |
| D004 | `plan.md` | Add explicit metadata-leakage qualification evidence | BLOCKING |
| D005 | `spec.md` | Complete platform-specific protector access/restart/revocation contract | BLOCKING |
| D006 | `spec.md` | Make Argon2id recovery policy normative and versioned | BLOCKING |
| D007 | `spec.md` | Bind recovery wrap AAD to vault/type/generation and test transplant rejection | BLOCKING |
| D008 | `spec.md` | Rewrite recovery failure contract to fail closed without partial plaintext | BLOCKING |
| D009 | `spec.md` | Freeze normative canonical AAD schemas | BLOCKING |
| D010 | `spec.md` | Freeze XChaCha20-Poly1305 nonce lifecycle/uniqueness protocol | BLOCKING |
| D011 | `spec.md` | Define rollback/replay freshness anchor and recovery protocol | BLOCKING |
| D012 | `spec.md` | Define crash-atomic full VRK rotation phases and power-loss tests | BLOCKING |
| D013 | `spec.md` | Freeze provider-visible metadata allowlist | BLOCKING |
| D014 | `spec.md` | State detached portable-backup deletion/revocation limits | BLOCKING |
| D015 | `tasks.md` | Persist exact reviewed SHA/reviewer/disposition evidence location | BLOCKING |
| D016 | `tasks.md` | Define enforceable in-process key/handle lifecycle after lock/revocation/failure | BLOCKING |

## Remediation rule

The review above applies only to exact SHA `384608c8fc13531c399f3726caa5022eb3612aa2`. Security-semantic remediation changes invalidate it as final approval evidence. After D001-D016 are repaired and merged canonically:

1. the new exact canonical design SHA must pass CI and Diffcipline R3;
2. PR #12 or a successor review-only PR must point at that exact canonical SHA without review-only content changes;
3. an independent substantive reviewer must review the new exact SHA;
4. all new blocking findings must be resolved forward;
5. a durable final disposition with reviewer identity, exact SHA, findings, recommendations, and residual risks must be recorded before dependency adoption or 004B implementation.

Until those steps complete, `SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED`.
