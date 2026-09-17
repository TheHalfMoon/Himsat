# Specification 006 Tasks — Capture Abstraction and Capture Health

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and review evidence remain authoritative.

## Shaping — canonical

- [x] S001 Re-read canonical `main` at Specification 005 closeout merge `85b2cb2ee5b7f8b043c66920b7ebc465edf48504`.
- [x] S002 Confirm Specification 005 post-closeout CI `35244997886` and R3 `35244997858` succeeded.
- [x] S003 Re-read constitution, master-plan unit 006, the architecture capture/health contracts, the product-plan health signals, the 003/004/005 consumable contracts, and the donor registry without adopting material.
- [x] S004 Note upstream facts to refresh at implementation time (per-OS native capture APIs, audio-session/route behavior, background-service rules) without adopting dependencies.
- [x] S005 Split prospective leaves 006A-006C as candidates only; exact split re-bounds after shaping qualifies.
- [x] S006 Define scope, additive-only 003/004/005 boundary discipline, donor-compare posture, acceptance, and R3 evidence contracts in `spec.md`/`plan.md`.
- [x] S007 Exact-head qualify shaping head: CI and R3 SUCCESS.
  - Shaping head `5785124f44cb7db5e75acee1154f20229d1dbb91` (PR #175): pre-merge CI `35247902444` SUCCESS, R3 `35247902485` SUCCESS.
- [x] S008 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
  - Zero submitted reviews; comments are bot-only (Qodo billing-blocked, CodeRabbit skip), no blocking finding.
- [x] S009 Merge shaping; canonical merge contains the exact shaping head as parent.
  - Canonical shaping merge `61414cd164a59973a0b5d4f469a16afebddc6b72` (PR #175, parents `85b2cb2ee5b7f8b043c66920b7ebc465edf48504` + `5785124f44cb7db5e75acee1154f20229d1dbb91`; merge tree equals shaping-head tree).
- [x] S010 Require post-shaping qualification: CI and R3 SUCCESS on the exact canonical merge.
  - Post-merge CI `35249873459` SUCCESS, R3 `35249873475` SUCCESS on `61414cd164a59973a0b5d4f469a16afebddc6b72`.

## 006A/006B/006C implementation leaves (re-bound after shaping qualified)

- [x] A001 Re-bound 006A/006B/006C against live main as implementation Grains; no 006 leaf remains a candidate. Shipped as gate hardening (113 added lines) plus adoption (766 added / 0 removed), 006B (532 added), and 006C (594 added), each under the R3 900-line gate.
- [x] A002 Gate hardening for the B006A workspace-events adoption (trusted-base predicate + pinned six-file exception, REVIEW-only): exact-head `88aea36` CI `35257228614` / R3 `35257228585` SUCCESS; expected-head merge PR #178 to `dc4f814`; post-merge CI `35259150477` / R3 `35259150538` SUCCESS on `dc4f814`.
- [x] A003 006A adoption reship byte-identical under the hardened gate (session machine + wiring + edge + closure + evidence; PR #177 closed superseded, nothing re-authored): exact-head `6a53ed5` CI `35261286883` / R3 `35261286917` SUCCESS via the B006A exception; expected-head merge PR #179 to `0d0a03b`; post-merge CI `35263138861` / R3 `35263138840` SUCCESS on `0d0a03b`.
- [x] A004 006B health telemetry (monitor, snapshot, classifier, 8 tests; no new dependencies): exact-head `23742e0` CI `35265775128` / R3 `35265775039` SUCCESS; expected-head merge PR #180 to `2251729`; post-merge CI `35267652412` / R3 `35267652411` SUCCESS on `2251729`.
- [x] A005 006C checkpoint and metadata (derivation, loss account, latest-wins, mapping, 8 tests; no new dependencies): exact-head `5918d4b` CI `35269367774` / R3 `35269367826` SUCCESS; expected-head merge PR #181 to `82e53d2`; post-merge CI `35271285085` / R3 `35271285074` SUCCESS on `82e53d2`.
- [x] A006 Record post-merge results in `006-capture-final-evidence.md`, reconcile `specs/CURRENT.md`, and close Specification 006 in this reconciliation; only then may Specification 007 shaping begin.
