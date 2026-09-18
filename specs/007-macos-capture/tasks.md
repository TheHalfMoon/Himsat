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
  - Grain-1 head `3df8e39bdca3e57f504095a2b066bf7204637d24` (PR #185, 4 files, 504a/0d): pre-merge CI `35284373920` / R3 `35284373899` SUCCESS; canonical merge `0ef54bc941bdaddce35eb1d4a18554fc544d3966` (parents `bf9d43b` + `3df8e39`, tree-equal); post-merge CI `35285918928` / R3 `35285918939` SUCCESS. No submitted reviews, no blocking finding.

- [x] A002 Ship 007A grain 2 (cpal binding + closure) through the B007 gate saga: gate hardening `2d339a8` (PR #186, tool-only 117a/0d; pre CI `35288509258` / R3 `35288509273`; merge `9041323`; post CI `35290282633` / R3 `35290282960` SUCCESS), predicate correction `2821d7d` (PR #188, 4a/3d, exit-2 shape from live R3 evidence; pre CI `35292624902` / R3 `35292625001`; merge `323fbe4`; post R3 `35293977339` SUCCESS, post CI superseded-cancelled with superset coverage), candidate `dc37960` closed superseded (PR #187, predated gate), byte-identical reship `0bbc1f4` (PR #189, 8 files, 5551a/297d; pre CI `35294006607` / R3 `35294006447` SUCCESS via the exception; merge `9578d65`; post CI `35295689765` / R3 `35295689738` SUCCESS). No submitted reviews anywhere, no blocking finding.
- [x] A003 Record this aggregate evidence in `007a-microphone-final-evidence.md`, reconcile `specs/CURRENT.md` with B007A entries, and close B007A as CANONICAL_CLOSED in this reconciliation; only then may 007B shaping begin.

## 007B implementation grains

- [x] B001 Re-bind 007B grain 1 against live main as the tap-core Grain: discovery/selection/identity/fault-classification over an injected backend with zero new dependencies (`SourceKind::SystemAudio` already closed, no contract change). cidre binding deferred to grain 2 with the recorded DEPEND decision.
- [x] B002 Bind 007B grain 2 against live main as the tap-lifecycle Grain: `CidreSystemTapBackend` over cidre 0.29.0 (macOS-only DEPEND, features `av`/`core_audio`/`dispatch`/`macos_15_0`) implementing route enumeration plus a live `probe_process_tap` authorization/create/format proof (TAP_OK 1ch/48kHz); full closure for the 2-crate subtree (both MIT, registry 211); sample streaming deferred to 007C as the recorded residual (unsafe extraction forbidden by workspace policy). Evidence: `b007b-tap-binding-evidence.md`.
- [x] B003 Record this aggregate evidence in `007b-system-tap-final-evidence.md`, reconcile `specs/CURRENT.md` with B007B entries, and close B007B as CANONICAL_CLOSED in this reconciliation; only then may 007C shaping begin.

## 007C shaping — canonical

- [x] C001 Re-read canonical `main` at the 007B reconciliation merge `5f64f04` (post-merge CI `35312683706` / R3 `35312683709` SUCCESS) and confirm the 007B residuals: streaming sanction open, aggregate device unproven, non-F32 refused.
- [x] C002 Declare the 007C leaf in `plan.md` with outcome, `scope_in`/`scope_out`, dependencies, acceptance, risk, recovery path, context budget, change surface, evidence requirements, minimality rationale, and safety/security implications; no code, manifest, closure, workflow, or donor change in shaping.
- [x] C003 Exact-head qualify shaping head: CI and R3 SUCCESS.
  - Shaping head `f8d967ded183a956efa88d7dd13005bf466cf123` (PR #195): pre-merge CI `35314133113` SUCCESS, R3 `35314133154` SUCCESS.
- [x] C004 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
  - Zero submitted reviews; 2 bot comments, no blocking finding.
- [x] C005 Merge shaping; canonical merge contains the exact shaping head as parent.
  - Canonical shaping merge `8d379af2042cf23ea4da1f70e03f9a7675296fb0` (PR #195, parents `5f64f04` + `f8d967d`; merge tree equals shaping-head tree).
- [x] C006 Require post-shaping qualification: CI and R3 SUCCESS on the exact canonical merge; only then may 007C implementation grains be bound.
  - Post-merge CI `35315392601` SUCCESS, R3 `35315392695` SUCCESS on `8d379af2042cf23ea4da1f70e03f9a7675296fb0`.
