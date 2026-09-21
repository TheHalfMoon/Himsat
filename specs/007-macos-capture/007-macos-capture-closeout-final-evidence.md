# Specification 007 — macOS Capture — Closeout Final Evidence

## Scope

This document records Specification 007 (macOS capture) as a
whole: reviewed shaping plus three bounded implementation leaves —
007A microphone pathway, 007B system tap, 007C system-tap streaming
and life cycle — each with reconciliation, expected-head merge, and
post-merge CI/R3 qualification. 007 adds two macOS-only dependency
edges (cpal 0.18.2, cidre 0.29.0, both adopted under the bounded
B007/B007B adoption gates) and copies no donor code. It changes no
reviewed 003/004/005/006 byte.

## Dependency state

Specifications 003, 004, 005, and 006 are `CLOSED_CANONICAL` beneath
007. Shaping started from the 006 closeout merge
`ea5a5e4b43dcc7e9de756a9ea3ce619179718687`. 007 shaping is
`CLOSED_CANONICAL` at merge `61a300c844befe273c3a89577a485f89087812be`
(PR #183, post-merge CI `35278832383` / R3 `35278832399` SUCCESS).
The 007C shaping amendment is
`8d379af2042cf23ea4da1f70e03f9a7675296fb0` (PR #195, post-merge CI
`35315392601` / R3 `35315392695` SUCCESS).

## Reviewed shaping

```text
SPEC_007_SHAPING_HEAD = 189974bf7826f4824696a34019f36f1d0bf5751b (PR #183)
SPEC_007_SHAPING_PREMERGE_CI = 35277118177_SUCCESS_PULL_REQUEST_ATTEMPT_1
SPEC_007_SHAPING_PREMERGE_R3 = 35277118298_SUCCESS_PULL_REQUEST_ATTEMPT_1
SPEC_007_SHAPING_CANONICAL_MERGE = 61a300c844befe273c3a89577a485f89087812be
SPEC_007_SHAPING_POSTMERGE_CI = 35278832383_SUCCESS_PUSH_ATTEMPT_1
SPEC_007_SHAPING_POSTMERGE_R3 = 35278832399_SUCCESS_PUSH_ATTEMPT_1
SPEC_007_SHAPING_REVIEW = ZERO_SUBMITTED_REVIEWS_BOT_ONLY_NO_BLOCKING_FINDING
SPEC_007C_SHAPING_HEAD = f8d967ded183a956efa88d7dd13005bf466cf123 (PR #195)
SPEC_007C_SHAPING_PREMERGE_CI = 35314133113_SUCCESS_PULL_REQUEST_ATTEMPT_1
SPEC_007C_SHAPING_PREMERGE_R3 = 35314133154_SUCCESS_PULL_REQUEST_ATTEMPT_1
SPEC_007C_SHAPING_CANONICAL_MERGE = 8d379af2042cf23ea4da1f70e03f9a7675296fb0
SPEC_007C_SHAPING_POSTMERGE_CI = 35315392601_SUCCESS_PUSH_ATTEMPT_1
SPEC_007C_SHAPING_POSTMERGE_R3 = 35315392695_SUCCESS_PUSH_ATTEMPT_1
```

## Leaf 007A — microphone pathway (canonical-closed at PR #190)

```text
B007A1_HEAD = 3df8e39bdca3e57f504095a2b066bf7204637d24 (PR #185, adapter core)
B007A1_TREE = 107b0ca3600520e10e7e2347187521e55ed5d1ab_EQUALS_MERGE_TREE
B007A1_PREMERGE_CI = 35284373920_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007A1_PREMERGE_R3 = 35284373899_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007A1_CANONICAL_MERGE = 0ef54bc941bdaddce35eb1d4a18554fc544d3966
B007A1_POSTMERGE_CI = 35285918928_SUCCESS_PUSH_ATTEMPT_1
B007A1_POSTMERGE_R3 = 35285918939_SUCCESS_PUSH_ATTEMPT_1
B007A2_CANDIDATE = dc379604d503cf54053f5db9d9c41f470a92da28_CLOSED_SUPERSEDED_PREDATED_GATE_NOTHING_REAUTHORED
B007A2_HEAD = 0bbc1f465aef056bbb03e3d016c24cf3f2215dbe (PR #189, cpal adoption)
B007A2_TREE = ba1ea948dd3e272f27e6ccb0b3cfc09172c96dbe_EQUALS_MERGE_TREE
B007A2_PREMERGE_CI = 35294006607_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007A2_PREMERGE_R3 = 35294006447_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007A2_CANONICAL_MERGE = 9578d65b9cd17d665617db4acb3462fc3015c76c
B007A2_POSTMERGE_CI = 35295689765_SUCCESS_PUSH_ATTEMPT_1
B007A2_POSTMERGE_R3 = 35295689738_SUCCESS_PUSH_ATTEMPT_1
B007A_AGGREGATE_EVIDENCE = 007a-microphone-final-evidence.md_CANONICAL_QUALIFIED
B007A_DISPOSITION = CANONICAL_CLOSED
```

Gate saga (PR #186 gate hardening `9041323`, PR #188 predicate
correction `323fbe4`, PR #187 `dc37960` superseded) preserved
forward-only; the cpal `=0.18.2` macOS-target DEPEND carries
Apache-2.0 selection with 48-package closure evidence.

## Leaf 007B — system tap (canonical-closed at PR #194)

```text
B007B1_HEAD = a90356545137ad1ccbaa3b07e09d951eca9f5259 (PR #191, tap core)
B007B1_TREE = b85278d20b84f0b4f0c8d932f1e914b000316b88_EQUALS_MERGE_TREE
B007B1_PREMERGE_CI = 35301029300_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007B1_PREMERGE_R3 = 35301029277_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007B1_CANONICAL_MERGE = 45e913056a26477ac3bb1f5d2f499f80e0bec295
B007B1_POSTMERGE_CI = 35302602751_SUCCESS_PUSH_ATTEMPT_1
B007B1_POSTMERGE_R3 = 35302602767_SUCCESS_PUSH_ATTEMPT_1
B007BGATE_HEAD = 0672c63d942c31a51487e03c93ccb046757212a1 (PR #193)
B007BGATE_CANONICAL_MERGE = faff02d9490c6b42850745f68e9d2e9f228654dc
B007BGATE_POSTMERGE_CI = 35308616412_SUCCESS_PUSH_ATTEMPT_1
B007BGATE_POSTMERGE_R3 = 35308616390_SUCCESS_PUSH_ATTEMPT_1
B007B2_CANDIDATE = ea483cac2947376b3e4f462eaf6cf7acbcfad26b_REBASED_VIA_GATE_BASE_MERGE_BLOBS_INTACT_RESUMED_AS_eee37d6
B007B2_HEAD = eee37d60dceafbe1f19affab079d3e087edc730e (PR #192, cidre adoption)
B007B2_TREE = 6aa890ada9ea9883c7b2b345ba59aa9941e23590_EQUALS_MERGE_TREE
B007B2_PREMERGE_CI = 35308649616_SUCCESS_PULL_REQUEST_ATTEMPT_2_AFTER_GATE_MERGE
B007B2_PREMERGE_R3 = 35308649631_SUCCESS_PULL_REQUEST_ATTEMPT_2_B007B_EXCEPTION
B007B2_FIRST_ATTEMPT_R3 = 35305754018_REVIEW_EXIT_1_DESIGNED_AUTHORIZATION_PROOF_RECORDED
B007B2_CANONICAL_MERGE = 086b59026a402a3c9fdaefea61c97e91e02b6cb9
B007B2_POSTMERGE_CI = 35309850877_SUCCESS_PUSH_ATTEMPT_1
B007B2_POSTMERGE_R3 = 35309850870_SUCCESS_PUSH_ATTEMPT_1
B007B_AGGREGATE_EVIDENCE = 007b-system-tap-final-evidence.md_CANONICAL_QUALIFIED
B007B_DISPOSITION = CANONICAL_CLOSED
```

cidre `=0.29.0` macOS-target DEPEND with full 2-crate closure
(both MIT, registry 211) adopted under the live-authorized B007B
adoption gate; sample streaming deferred to 007C as a recorded
residual.

## Leaf 007C — system-tap streaming and lifecycle (canonical-closed at PR #200)

```text
B007C1_HEAD = d6527dd6796414c9b4cd978af8425bc4538bced7 (PR #196/#197, aggregate first audio)
B007C1_TREE = 90ac76347bda08f09de3d9705689e0cb5b1d1aec_EQUALS_MERGE_TREE
B007C1_PREMERGE_CI = 35321568922_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C1_PREMERGE_R3 = 35321568928_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C1_CANONICAL_MERGE = f52ff0856b8cea0fbc28f94715a797e8bf24a6db
B007C1_POSTMERGE_CI = 35323542302_SUCCESS_PUSH_ATTEMPT_1
B007C1_POSTMERGE_R3 = 35323542345_SUCCESS_PUSH_ATTEMPT_1
B007C2_HEAD = 50603568a3185733c50ff18d2fe972dd7b08fa15 (PR #197/#198, polling lifecycle watch)
B007C2_TREE = deada7297782a7bf8e4e0987b00cdca6b6ecb374_EQUALS_MERGE_TREE
B007C2_PREMERGE_CI = 35325695878_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C2_PREMERGE_R3 = 35325695881_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C2_CANONICAL_MERGE = 8b6c619750646c454fa754671c9eb3d34114b28c
B007C2_POSTMERGE_CI = 35327258174_SUCCESS_PUSH_ATTEMPT_1
B007C2_POSTMERGE_R3 = 35327258212_SUCCESS_PUSH_ATTEMPT_1
B007C3_HEAD = 65aa2d623ab9edb4e7d1b7f7250a825ddb9a2c4d (PR #199, sustained-flow loss accounting)
B007C3_TREE = 8a5153610a0b9a17d9de0f2cc6e8eebf2503ac07_EQUALS_MERGE_TREE
B007C3_PREMERGE_CI = 35329269388_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C3_PREMERGE_R3 = 35329269420_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C3_CANONICAL_MERGE = 9b2268f5164592ee961780a5580478ab6d6f0f5e
B007C3_FIRST_POSTMERGE = CANCELLED_BY_CONCURRENCY_SUPERSEDED_BY_35618918438_35618918612
B007C3_POSTMERGE_CI = 35618918438_SUCCESS_PUSH_ATTEMPT_1
B007C3_POSTMERGE_R3 = 35618918612_SUCCESS_PUSH_ATTEMPT_1
B007C_RECONCILIATION_HEAD = 19e66a5a329c9625b9a317bd1d4a1cf11c75ea5a (PR #200)
B007C_RECONCILIATION_PREMERGE_CI = 35619979787_SUCCESS_PULL_REQUEST_AFTER_DIAGNOSTIC_JOB_RERUN
B007C_RECONCILIATION_PREMERGE_R3 = 35619979750_SUCCESS_PULL_REQUEST_ATTEMPT_1
B007C_RECONCILIATION_CANONICAL_MERGE = 2c6dd53dfd8a5b27c1d7ea4bae0475da6f3fb5b4
B007C_RECONCILIATION_MERGE_TREE = edac10f7199f17392fcefb623eed4d67ff7fedc4_EQUALS_ACCEPTED_TREE
B007C_RECONCILIATION_POSTMERGE_CI = 35622869469_SUCCESS_PUSH_ATTEMPT_1
B007C_RECONCILIATION_POSTMERGE_R3 = 35622869485_SUCCESS_PUSH_ATTEMPT_1
B007C_AGGREGATE_EVIDENCE = 007c-system-tap-streaming-final-evidence.md_CANONICAL_QUALIFIED
B007C_DISPOSITION = CANONICAL_CLOSED
```

The 007B streaming-sanction residual is closed by evidence: no safe
IOProc sample view exists in cidre 0.29.0, so streaming takes the
sanctioned tap -> private aggregate device -> cpal F32 path with
zero unsafe code. The reconciliation PR #200 is docs-only and was
additionally reviewed with alibaba/open-code-review 1.12.7 in
delegation mode (recorded on the PR); its first CI attempt hit the
pre-existing flaky `vault_backup_restore` staging test on
`ubuntu-latest` (also failing on the unrelated base `9b2268f`), and
the diagnostic re-run of the identical head passed. The failure is
preserved in run history, not erased.

## Acceptance proof

```text
FMT = cargo fmt --all -- --check clean at each accepted head
CLIPPY = cargo clippy --workspace --all-targets --locked -- -D warnings clean (CI)
TESTS = cargo test --workspace --all-targets --locked green on
  ubuntu-latest, macos-latest, windows-latest for every accepted
  grain head (25 capture_system_audio tests: 20 portable + 5
  env-gated live), plus every pre-existing suite, 0 failures
LIVE = darwin/arm64 env-gated live tests green: microphone open,
  tap probe TAP_OK, aggregate first-audio (318 callbacks /
  162,816 frames / 79,298 nonzero), lifecycle watch clean, 5 s
  sustained-flow reconciliation with zero stream errors
CLOSURE = cidre 0.29.0 + cpal 0.18.2 registered; registry 211;
  provenance_gate validate/check-generated PASS
PROVENANCE = no donor code copied (Meetily/Anarlog/OpenSuperWhisper
  and Superwhisper research compared as planning inputs only)
```

## Honesty bounds (not claims)

- Platform truth is macOS-only: microphone and system-tap behavior
  is evidenced on darwin/arm64 and claimed for macOS alone. Windows
  and Linux adapters are 008/009 and unauthorised here.
- The adapter delivers captured frames; it does not process them.
  Codecs/DSP/resampling/VAD (010), transcription and intelligence
  (011+), UI (015), sync, and connectors are out of 007 scope.
- No multi-hour endurance run is claimed; the 5 s settled
  reconciliation generalises by construction but is not run as
  multi-hour evidence.
- Journal sink wiring (adapter frame delivery driving 005 open/
  commit/close) is a 007-closeout boundary decision recorded here,
  not product code delivered by 007.
- Non-F32 tap formats refuse as create-stage faults; negotiation
  widens only with fresh Gate E evidence.

## Residual risks (carried, not blockers)

1. Pre-existing backup staging tests remain flaky on CI runners
   across the lineage (006 closeout residual 1); one `ubuntu-latest`
   occurrence is recorded on PR #200 and on base `9b2268f`, with the
   diagnostic re-run green. A dedicated flaky-test remediation is a
   separate forward node.
2. Sequence coordination across multiple monitors stays caller-owned
   under 003 order; each adapter monitor instance starts at zero.
3. iOS/iPadOS, Android, Windows, and Linux capture remain future
   008+ shaping; 007 narrows no shared 006 contract.

## Spec 007 closeout

Specification 007 satisfies its closeout rule: reviewed shaping
(`61a300c`, plus the `8d379af` 007C shaping amendment), three
bounded implementation leaves (007A/007B/007C) with adapter,
platform, adversarial, and dependency-closure evidence per leaf,
reconciliation for each leaf, expected-head merges with merge-tree
equality throughout, post-merge CI/R3 SUCCESS on every canonical
merge SHA, and this durable closeout record. Every leaf is
`CANONICAL_CLOSED`. Specification 007 is `CLOSED_CANONICAL`. Only
now may Specification 008 (platform adapters) shaping begin.
