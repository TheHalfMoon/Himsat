# Specification 007 Tasks — macOS Capture

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and review evidence remain authoritative.

## Shaping — canonical

- [x] S001 Re-read canonical `main` at Specification 006 closeout merge `ea5a5e4b43dcc7e9de756a9ea3ce619179718687`.
- [x] S002 Confirm Specification 006 post-closeout CI `35275083648` and R3 `35275083661` succeeded.
- [x] S003 Re-read constitution, master-plan unit 007, Gate E platform-qualification rule, the 003/004/005/006 consumable contracts, the donor registry (Meetily/Anarlog/OpenSuperWhisper boundaries), and the Superwhisper research record without adopting material.
- [x] S004 Note upstream facts to refresh at implementation time (current supported Apple audio APIs and SDK pins, TCC permission behavior, sleep/wake and background-audio rules, binding-crate options) without adopting dependencies.
- [x] S005 Split prospective leaves 007A-007C as candidates only; exact split re-bounds after shaping qualifies.
- [x] S006 Define scope, additive-only 003/004/005/006 boundary discipline, donor-compare posture with 004P-style adoption gate, carried 006 residuals, acceptance, and R3 evidence contracts in `spec.md`/`plan.md`.
- [x] S007 Exact-head qualify shaping head: CI and R3 SUCCESS.
  - Shaping head `189974bf7826f4824696a34019f36f1d0bf5751b` (PR #183): pre-merge CI `35277118177` SUCCESS, R3 `35277118298` SUCCESS.
- [x] S008 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
  - Zero submitted reviews; comments are bot-only (Qodo billing-blocked, CodeRabbit skip), no blocking finding.
- [x] S009 Merge shaping; canonical merge contains the exact shaping head as parent.
  - Canonical shaping merge `61a300c844befe273c3a89577a485f89087812be` (PR #183, parents `ea5a5e4b43dcc7e9de756a9ea3ce619179718687` + `189974bf7826f4824696a34019f36f1d0bf5751b`; merge tree equals shaping-head tree).
- [x] S010 Require post-shaping qualification: CI and R3 SUCCESS on the exact canonical merge.
  - Post-merge CI `35278832383` SUCCESS, R3 `35278832399` SUCCESS on `61a300c844befe273c3a89577a485f89087812be`.

## 007A implementation grains (re-bound after shaping qualified)

- [x] A001 Re-bind 007A grain 1 against live main as the adapter-core Grain: discovery/selection/identity/fault-classification over an injected backend with zero new dependencies. cpal binding deferred to grain 2 with the recorded DEPEND decision; system tap stays in 007B.
