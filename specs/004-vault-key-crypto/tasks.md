# Specification 004 Tasks — Vault, Key, and Crypto Architecture

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and independent review evidence remain authoritative. A checked historical authoring task does not override a later blocking review finding.

## Shaping — canonical

- [x] S001 Re-read canonical `main` at Specification 003 closeout merge `1f14bbe004962dd164402e6e6c7f9c046cf5b489`.
- [x] S002 Confirm Specification 003 post-closeout CI `34047579266` succeeded.
- [x] S003 Re-read constitution, architecture, security/platform plan, qualification strategy, and master-plan unit 004.
- [x] S004 Refresh upstream facts for Argon2id, XChaCha20-Poly1305/secretstream, OS secure stores, and SQLCipher without adopting dependencies.
- [x] S005 Recursively split reviewed design before implementation/provenance authority.
- [x] S006 Define initial threat model, key hierarchy, recovery, protector, structured-store, blob, rotation, backup, deletion, and R3 evidence contracts.
- [x] S007 Exact-head qualify shaping head `037295ffed1e6e065c70b90216751404d4f8ff18`: CI `34048208674` SUCCESS and R3 `34048208667` SUCCESS.
- [x] S008 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; no submitted review/thread existed, Qodo billing-blocked and CodeRabbit skipped were not counted as PASS.
- [x] S009 Merge shaping with explicit expected-head protection; canonical merge `384608c8fc13531c399f3726caa5022eb3612aa2` contains exact shaping head as parent.
- [x] S010 Require post-shaping qualification: CI `34048394581` SUCCESS and R3 `34048394550` SUCCESS on exact canonical merge.

## 004A independent design review — first and second passes

- [x] R001 Prepare review-only PR #12 with base `1f14bbe004962dd164402e6e6c7f9c046cf5b489` and head pointing directly to exact canonical design `384608c8fc13531c399f3726caa5022eb3612aa2`.
- [x] R002 Obtain substantive independent CodeRabbit review `PRR_kwDOUQPwRs8AAAABMYy3gw` on the exact canonical design; GitHub state `COMMENTED`, submitted 2026-09-06T18:47:41Z, 16 actionable findings.
- [x] R003 Classify all 16 actionable findings as blocking 004B implementation; record exact SHA, reviewer identity, state, evidence limitations, and Himsat governance disposition `CHANGES_REQUIRED` in `review-evidence.md`.
- [x] R004 Merge forward remediation D001-D016 in PR #16 with expected-head protection; canonical merge `6d1bbc9b55690939833917eebd447c361627f48a` contains exact remediation head `befb5026a778491f20dce456fdb9b855d4a6377a`.
- [x] R005 Obtain second substantive independent CodeRabbit review on review-only PR #18 for exact canonical SHA `6d1bbc9b55690939833917eebd447c361627f48a`.
- [x] R006 Record second-review `CHANGES_REQUIRED` disposition, two blockers D017-D018, non-blocking recommendations, and residual risks in `review-round2-evidence.md`; old review/self-review/CI/skipped/billing-blocked output are not substitutes.
- [x] R007 Canonicalize round-2 ledger/tooling evidence at `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121`; post-merge CI `34058307222` SUCCESS and R3 `34058307204` SUCCESS.

## 004A first-remediation findings

- [x] D001 Specify OS-CSPRNG-only production randomness, exact lengths, fail-closed errors, and prohibited fallback.
- [x] D002 Define protector ownership/access scope and user-presence policy with fail-closed unsupported-policy behavior.
- [x] D003 Require exact SQLCipher core/binding/provider/SQLite/build/transitive provenance identity before P002.
- [x] D004 Define metadata-leakage qualification across DB/filesystem/backup-provider/secret-store boundaries.
- [x] D005 Define Apple/Android/Windows/Linux application/same-user access, restart, and revocation semantics.
- [x] D006 Freeze versioned Argon2id recovery policy, validation/resource bounds, and passphrase handling.
- [x] D007 Define recovery AAD binding `VaultId`, generation, envelope type/policy/suite; later B019 review requires the complete canonical public recovery envelope and extended AAD now defined in `round3-normative-contracts.md`.
- [x] D008 Define externally uniform wrong-passphrase/tag failure with typed fail-closed result and no partial plaintext.
- [x] D009 Author dependency-free canonical AAD/schema intent; later independent review B019 proved public bounded-blob/recovery envelope serialization remained incomplete, so D009 is not gate-closed until round-3 review succeeds.
- [x] D010 Freeze XChaCha nonce generation, storage, retry/restore/generation behavior, collision handling, and duplicate-canonical detection.
- [x] D011 Define established-anchor rollback/replay state machine; later independent review B020 proved initial protected freshness genesis remained incomplete, so D011 is not gate-closed until round-3 review succeeds.
- [x] D012 Define crash-atomic VRK rotation phases `PREPARE -> STAGE -> VERIFY -> PUBLISH -> ANCHOR -> ACTIVATE -> RETIRE` and fault-injection evidence.
- [x] D013 Freeze provider-visible metadata allowlist and forbidden logical metadata surfaces.
- [x] D014 State detached portable-backup retention/revocation limits independently from physical-erasure limits.
- [x] D015 Create durable `review-evidence.md` with exact first reviewed SHA, reviewer identity, state, findings, and Himsat disposition.
- [x] D016 Define revocable keyed-handle lease, post-lock/revocation rejection, secret release/zeroization behavior, tests, and runtime residual risk.

## 004A first-remediation qualification

- [x] A001 Confirm PR #16 remediation diff changes only Specification 004 docs/state and stays within Diffcipline bounds; no product code, Cargo dependency, provenance adoption, or generated SBOM change.
- [x] A002 Exact-head qualify `befb5026a778491f20dce456fdb9b855d4a6377a`: CI `34054646684` SUCCESS and R3 `34054646643` SUCCESS.
- [x] A003 Reconcile PR #16 reviews, review threads, comments, exact diff, canonical `main`, and mergeability immediately before merge.
- [x] A004 Merge PR #16 with exact expected-head protection.
- [x] A005 Re-read canonical remediation merge `6d1bbc9b55690939833917eebd447c361627f48a`; post-merge CI `34055180993` SUCCESS and R3 `34055180810` SUCCESS.
- [x] A006 Re-review exact canonical remediation SHA on PR #18; disposition `CHANGES_REQUIRED`, with D017-D018 remaining blocking.

## 004A second-remediation findings

- [x] D017 Freeze Apple Keychain non-synchronizable/device-only/access-group/passcode/presence/migration/invalidation behavior and per-target evidence requirements.
- [x] D018 Freeze distinct manifest-purpose HKDF domain, canonical authenticated manifest envelope/plaintext/AAD/nonce/hash/parser bounds, and established-anchor fail-closed compare-and-advance contract; later B020 review requires the initial genesis transition now defined in `round3-normative-contracts.md`.

## 004A second-remediation qualification

- [x] A201 Confirm PR #21 exact diff is limited to `specs/004-vault-key-crypto/spec.md` and `review-round2-evidence.md`; no implementation, dependency, provenance-adoption, SBOM, workflow, donor, model, dataset, or asset change.
- [x] A202 Exact-head qualify `4357102d388248400116eeab45cf83de71e35051`: CI `34057694827` SUCCESS and R3 `34057694832` SUCCESS.
- [x] A203 Reconcile PR #21 exact diff, comments, reviews, review threads, canonical `main`, and mergeability immediately before merge; Qodo billing block and CodeRabbit auto-skip were not counted as PASS.
- [x] A204 Merge PR #21 with `expected_head_sha = 4357102d388248400116eeab45cf83de71e35051`; canonical merge `cb8511c1b420c58f714768c3561e74f04f026b3a` has parents `6d1bbc9b55690939833917eebd447c361627f48a` and `4357102d388248400116eeab45cf83de71e35051`.
- [x] A205 Require post-merge qualification on exact canonical `cb8511c1b420c58f714768c3561e74f04f026b3a`: CI `34057861438` SUCCESS and R3 `34057861447` SUCCESS.
- [x] A206 Exact-head qualify and merge ledger-only reconciliation without changing security semantics; canonical reconciliation `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121`, post-merge CI `34058307222` SUCCESS and R3 `34058307204` SUCCESS.

## 004A third-review evidence conflict

- [x] R301 Obtain substantive review on PR #24, comment `5561981955`, for exact SHA `3c3847add4c8f56fe418fc02e62ae735e8239058`; disposition `CHANGES_REQUIRED` with blockers B019-B020 and recommendations for rotation invariants/retained manifest history.
- [x] R302 Obtain later substantive review on PR #25 for exact SHA `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121`; initial disposition `APPROVE`, no blockers.
- [x] R303 Prove by GitHub compare that `3c3847add... -> ea7ed384...` changed only `specs/CURRENT.md` and this task ledger, not the security semantics implicated by B019/B020.
- [x] R304 Search canonical main and confirm no existing `HIMSAT/BLOB/ENVELOPE/v1` or `install_genesis_freshness_anchor` contract resolves the negative findings by exact text.
- [x] R305 Request focused contradiction reconciliation on PR #25 in comment `5562030888`; do not treat the initial approval as final while known negative evidence remains unreconciled.
- [x] R306 Obtain a final consistent exact-SHA security disposition: review-only PR #29 comment `5562296249` reviewed exact canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and returned `APPROVE`, `BLOCKING_FINDINGS = NONE`, `NON_BLOCKING_RECOMMENDATIONS = NONE`, `B019_1_STATUS = RESOLVED`, `B020_STATUS = RESOLVED`, and `D001_D018_STATUS = RESOLVED_NO_REGRESSION`.

## 004A round-3 findings and remediation

- [x] B019 Author canonical v1 recovery and bounded-blob public envelope byte layouts, fixed identifiers, exact sizes/length relations, checked arithmetic, extended AAD binding, parser rejection rules, and adversarial evidence requirements in `round3-normative-contracts.md`.
- [x] B020 Author explicit protected `UNINITIALIZED`/`PRESENT` freshness state, crash-atomic protected-state creation, `install_genesis_freshness_anchor` compare-and-set, new-vault genesis, fresh-device-restore genesis, missing-item fail-closed behavior, and platform negative evidence in `round3-normative-contracts.md`.
- [x] R3R001 Freeze complete rotation phase/generation invariants, permitted transitions, abort boundary, retry semantics, and corrupt-state rejection.
- [x] R3R002 Freeze minimum retained-manifest history lifetime and clarify retained-history nonce scanning as defense in depth rather than a global nonce ledger.
- [x] R3E001 Record contradictory review lineage, negative-evidence disposition, B019/B020 rationale, residual risks, and required re-review in `review-round3-evidence.md`.
- [x] R3E002 Make `round3-normative-contracts.md` an explicit controlling Specification 004A amendment through `specs/CURRENT.md` and this task ledger.

## 004A round-3/round-4 qualification

- [x] A301 Confirm remediation changes stayed design/evidence/state only and within Diffcipline bounds; no product code, Cargo/native dependency, provenance adoption, generated SBOM, workflow, donor, model, dataset, or asset change. The final round-4 remediation PR #28 changed only `round4-blob-inventory-contract.md` and `review-round4-evidence.md`.
- [x] A302 Exact-head qualify the final security-semantic remediation head `81587ab6af6ad88a1f82114dfd5ddd78767a6d47`: CI `34060963107` SUCCESS and R3 `34060963141` SUCCESS. Earlier round-3 head `4ee8f83975dd074003df2d151e80d56bc5f1c005` also passed CI `34059290985` and R3 `34059290977`.
- [x] A303 Reconcile PR #28 exact diff, reviews, review threads, comments, live `main`, and mergeability immediately before merge; no submitted review/thread blocked the merge, while Qodo billing-blocked and CodeRabbit auto-skip were not counted as PASS.
- [x] A304 Complete expected-head protection forward-only: post-hoc transport-level proof for earlier PR #26 is unavailable and remains unclaimed, but final security-semantic successor PR #28 was merged with explicit `expected_head_sha = 81587ab6af6ad88a1f82114dfd5ddd78767a6d47`; canonical merge `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` contains exact base and remediation head as parents.
- [x] A305 Re-read exact final canonical design merge `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98`; post-merge CI `34061056655` SUCCESS and R3 `34061056698` SUCCESS.
- [x] A306 Create review-only PR #29 with head pointing directly to exact canonical SHA `5fe8a99b0d8a623cda6a73a9ea8ed39cba957d98` and obtain substantive independent CodeRabbit response `5562296249`, explicitly covering B019-1, B020, and regression status for D001-D018.
- [x] A307 Record the final no-unresolved-blocker disposition and preserved residual risks in `review-round4-final-evidence.md`; this reconciliation opens 004P only after it becomes canonical and exact-head/post-merge qualification succeeds.

## Dependency/provenance decision — canonical closed

- [x] P001 Select exact established crypto provider/library strategy from reviewed candidates.
- [x] P002 Select exact SQLCipher core/binding/provider/build strategy, with immutable core/binding identities and exact SQLite/provider/native inputs.
- [x] P003 Inventory every direct/transitive Cargo/native dependency, immutable source/version, package source, and checksum.
- [x] P004 Verify controlling licenses/notices under `governance/provenance/policy.json`.
- [x] P005 Record adopted package/native closure in provenance registry and deterministic SBOM/notices before accepting unregistered external dependency bytes.
- [x] P006 Prove resulting lockfile/native distribution contains no unregistered, checksum-mismatched, manual-without-decision, denied, or unknown-license component.

## 004P closeout and B101 re-bound reconciliation

- [x] P007 Exact-head qualify adoption head `ee753debb23ac5a925a0736cec116994166953c5`: CI `34142484081` SUCCESS and R3 `34142484053` SUCCESS.
- [x] P008 Obtain substantive exact-head CodeRabbit review comment `5573320081`: `APPROVE`, `BLOCKING_FINDINGS = NONE`, all recorded adoption-gate blockers resolved, and 004B authority boundary preserved.
- [x] P009 Reconcile PR #36 and merge with `expected_head_sha = ee753debb23ac5a925a0736cec116994166953c5`; canonical adoption merge `a4d32ee93e0ab95af8376ba0ca09e248070c5924`.
- [x] P010 Require exact post-merge qualification on canonical adoption merge: CI `34144623816` SUCCESS and R3 `34144623829` SUCCESS.
- [ ] P011 Preserve the historical transport evidence limitation for PR #37: exact head `367348c2b11add2c15f7d25af92ca4679b2a19d1`, canonical merge `7ecba93ae0763a5165dd99ce0ec190cb106906de`, and post-merge CI `34146939013` / R3 `34146938921` are proven, but the historical merge API request body containing `expected_head_sha` is not reconstructible post hoc. Do not retroactively mark this transport proof PASS.

## 004B1 portable vault contracts

B101-B105 code and the B105/B201 reconciliation are canonical and exact-post-merge qualified. Historical expected-head transport proof for PR #38 remains not reconstructible post hoc; that evidence gap was repaired forward-only by canonical reconciliation PR #40 without rewriting history. B201-B205 implementations, prerequisite reconciliations through B205/B301, B301-B304 implementations, and prerequisite reconciliations through B303/B304 are canonical and exact-post-merge qualified. B305 remains blocked until the B304/B305 evidence-state reconciliation is itself exact-head qualified, explicitly expected-head guarded, parentage-proven, and exact-post-merge qualified.

- [x] B101 Add only reviewed `VaultId`, generation, freshness, lock/error, lease, and provider-neutral `SecretProtector` contract surface. Exact final head `ea82e1246095bb921dc9e7e40716076edbda41a6` passed CI `34148418394` and R3 `34148418341`; canonical merge `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c` passed post-merge CI `34149709088` and R3 `34149708977`.
- [x] B101R001 Canonicalize the B101 final-evidence and B102 re-bound reconciliation without security-semantic, dependency, provenance, generated-artifact, workflow, donor, or product-runtime changes; exact reconciliation head `d99ceb841d834324d06774753b661bdf108f354b` passed CI `34152839411` and R3 `34152839424`.
- [x] B101R002 Merge that exact reconciliation head with explicit `expected_head_sha = d99ceb841d834324d06774753b661bdf108f354b`; canonical merge `0252bb31764c9178e270694f4087e8ac701271a0` records successful guarded transport.
- [x] B101R003 Verify canonical reconciliation parentage and exact post-merge qualification: CI `34153492791` SUCCESS and R3 `34153492769` SUCCESS on `0252bb31764c9178e270694f4087e8ac701271a0` before the accepted B102 lineage began.
- [x] B102 Implement typed lock/unlock/protector/freshness access errors and revocable keyed-handle lease. Accepted PR #41 exact head `40b9705258a302a4e71340f4dd0a28746e86c8a7` passed CI `34154993507` and R3 `34154993537`, merged with explicit expected-head protection as `4b27ede7d9bf17caac163b7607c331056634fc95`, and passed post-merge CI `34155726527` and R3 `34155726505`. PR #39 remains preserved as pre-authority closed-unmerged history; stale/cancelled CI `34154807417` is not PASS.
- [x] B102R001 Canonicalize `b102-revocable-lease-final-evidence.md`, task-ledger reconciliation, and the B103 re-bound `specs/CURRENT.md` state through PR #42 exact head `47710251ade64e48741e94b4c23d784f04cbe809`; pre-merge CI `34156621024` and R3 `34156621129` succeeded.
- [x] B102R002 Merge exact B102/B103 reconciliation head `47710251ade64e48741e94b4c23d784f04cbe809` with explicit expected-head protection; canonical merge `9ed7ccd960cbf92a9718d424421dd40b71cfe0de` has exact prior-canonical and reconciliation parents.
- [x] B102R003 Verify exact canonical reconciliation post-merge qualification before B103 began: CI `34157271018` SUCCESS and R3 `34157271044` SUCCESS on `9ed7ccd960cbf92a9718d424421dd40b71cfe0de`.
- [x] B103 Implement reviewed portable `SecretProtector` behavior contract without pretending platform mechanics/capabilities are identical. PR #43 final head `47b8ed78705774d5e55f2eb1145385c725e325c0` passed CI `34159489786` and R3 `34159489849`; initial CI `34159354566` failed formatting and remains negative evidence. Guarded canonical merge `ead22ea8c0b248431a2f8a50264f6acdbc9f7a72` passed exact post-merge CI `34160202949` and R3 `34160202961`.
- [x] B103R001 Canonicalize `b103-secret-protector-final-evidence.md`, task-ledger reconciliation, and the B104 re-bound `specs/CURRENT.md` state through PR #44 exact head `f348330a52cec6eb46772529fbcb82fb52eb43d4`; pre-merge CI `34161105524` and R3 `34161105501` succeeded.
- [x] B103R002 Merge exact B103/B104 reconciliation head `f348330a52cec6eb46772529fbcb82fb52eb43d4` with explicit `expected_head_sha = f348330a52cec6eb46772529fbcb82fb52eb43d4`; durable transport comment `5575629798` records canonical merge `57217a6614c07ac5e8a00d85114dd06eee1a0120`.
- [x] B103R003 Verify canonical reconciliation parentage (`ead22ea8c0b248431a2f8a50264f6acdbc9f7a72` + `f348330a52cec6eb46772529fbcb82fb52eb43d4`) and exact post-merge qualification: CI `34161857768` SUCCESS and R3 `34161857755` SUCCESS on `57217a6614c07ac5e8a00d85114dd06eee1a0120` before B104 implementation began.
- [x] B104 Implement reviewed key hierarchy/domain separation and secret lifetime contract. PR #46 final head `e3c27cc2806feb2aa5ace7169e0507b9bd970a1c` passed CI `34166549041` and R3 `34166549045`; initial CI `34166444702` was cancelled after a forward head advance but contains a Windows formatting failure with lint/tests skipped and remains negative/non-PASS evidence. PR #46 merged with explicit `expected_head_sha = e3c27cc2806feb2aa5ace7169e0507b9bd970a1c` as `6578936f7051b548339f3cf95ca0d41e6629d8bc`, parentage is exact, and exact post-merge CI `34167236097` and R3 `34167236079` succeeded. Actual HKDF-SHA-256 execution and deterministic derivation vectors remain B201; concrete DB/blob post-lock I/O proof remained B105.
- [x] B105 Prove previously obtained DB/blob handles reject reads/writes after lock/revocation/failure and document residual process/runtime memory limits. PR #48 initial head `39d63bbf647f0f13aff1cefa9da0d7eb1a9fb119` produced CI `34169460825` with Ubuntu/Windows formatting failures and skipped downstream steps that remain non-PASS evidence. Forward-only formatting repair produced final head `f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf`, which passed CI `34169567748` and R3 `34169567620`. Durable final reconciliation comment `5576822047` preceded explicit expected-head transport; comment `5576824739` records merge `73b13d38ac3e34143c813bda679e6e96ce01762e`. Exact canonical parentage is `623c0ad4207ecbbd5e404a2a32ba3cdb61006778` + `f39e39e67e01ea3e145591e87ceb1ccfcdbbfeaf`; post-merge CI `34170861437` and R3 `34170861436` succeeded.
- [x] B105R001 Canonicalize `b105-post-lock-io-final-evidence.md`, this task ledger, and the B201 re-bound `specs/CURRENT.md` state through PR #49 exact head `267351bcad78434b612c929d3165bc03ad1fb08c`; pre-merge CI `34171637803` and R3 `34171637800` succeeded.
- [x] B105R002 Merge exact B105/B201 reconciliation head `267351bcad78434b612c929d3165bc03ad1fb08c` with explicit expected-head protection; durable transport comment `5577099576` records canonical merge `33fc791443d25dba0ed7a5958710cb52ebb5bfad`.
- [x] B105R003 Verify exact reconciliation parentage (`73b13d38ac3e34143c813bda679e6e96ce01762e` + `267351bcad78434b612c929d3165bc03ad1fb08c`) and exact post-merge qualification: CI `34172683259` SUCCESS and R3 `34172683357` SUCCESS on `33fc791443d25dba0ed7a5958710cb52ebb5bfad` before B201 implementation began.

## 004B2 cryptographic envelope foundation

- [x] B201 Implement reviewed HKDF-SHA-256 domain separation and deterministic test vectors. PR #50 exact head `3c1706dfd0dfc7c745e19b81d19f31e4beb39e38` passed CI `34173364034` and R3 `34173364032`, merged with explicit expected-head protection as canonical `3785561963b6eadab219b330367a9b6295755939`, and passed exact post-merge CI `34174517394` and R3 `34174517397`. Complete evidence is recorded in `b201-hkdf-final-evidence.md`.
- [x] B202 Implement reviewed versioned bounded-blob XChaCha20-Poly1305 envelope with exact public bytes, v1 AAD, checked parser bounds, and 64 MiB plaintext ceiling. PR #52 final head `5ad57a15dad1e3c1b1de78fb0fa8e720a02b7510` passed CI `34224041244` and R3 `34224041193`, merged with guarded `expected_head_sha` evidence comment `5584971324` as canonical `483857be2abf017c93fd94c210beef8080bc47fb`, and passed exact post-merge CI `34225052523` and R3 `34225052492`. Initial CI `34177306333`, successor CI `34223420708`, and successor CI/R3 `34223784461`/`34223784391` remain preserved negative/non-PASS evidence. Complete evidence is recorded in `b202-bounded-blob-final-evidence.md`.
- [x] B203 Implement fresh 24-byte OS-CSPRNG nonce per attempt, manifest reservation, retry/regeneration, restore behavior, and duplicate/collision rejection. PR #54 initial head `beaf8345c8b23fda2b56df77801c2b675ac4cfff` produced CI `34230105312` with macOS/Windows formatting failures that remain NOT PASS. Forward-only successor head `b6fdde9da02b8cc92363095e26cc8971e0df7b6e` passed CI `34230572075` and R3 `34230572042`, merged with explicit expected-head protection as canonical `33cab7cb4e9dae1469e7e44ffaeeec05673f88d2`, exact parentage is proven, and exact post-merge CI `34231954517` and R3 `34231954422` succeeded. Complete evidence is recorded in `b203-nonce-lifecycle-final-evidence.md`.
- [x] B203R001 Canonicalize `b203-nonce-lifecycle-final-evidence.md`, this task ledger, and the B204 re-bound `specs/CURRENT.md` state through PR #55 exact head `6c01a464b89521d99d42798ba4e4129ea6bc5413`; pre-merge CI `34233929258` and R3 `34233929295` succeeded.
- [x] B203R002 Merge exact B203/B204 reconciliation head `6c01a464b89521d99d42798ba4e4129ea6bc5413` with explicit `expected_head_sha` protection; durable transport comment `5586288776` records canonical merge `c453d56c00fc1fabca09987f1c11084099c6326e`.
- [x] B203R003 Verify exact reconciliation parentage (`33cab7cb4e9dae1469e7e44ffaeeec05673f88d2` + `6c01a464b89521d99d42798ba4e4129ea6bc5413`) and exact post-merge qualification: CI `34235087839` SUCCESS and R3 `34235087861` SUCCESS on `c453d56c00fc1fabca09987f1c11084099c6326e` before B204 implementation began.
- [x] B204 Implement recovery envelope with exact public bytes, Argon2id v1 policy, canonical recovery AAD, uniform authentication failure, parser bounds, and transplant rejection. PR #56 final head `2a071e21c483b239e76fb12af5cecd24eec8059b` passed CI `34241383350` and R3 `34241383280`, merged with explicit expected-head protection as canonical `ac0925051cc128580b9794837fd03200f9a7374b`, exact parentage is proven, and exact post-merge CI `34242682960` and R3 `34242683051` succeeded. Failed/cancelled heads remain preserved as NOT PASS in durable PR comments. Complete evidence is recorded in `b204-recovery-envelope-final-evidence.md`.
- [x] B204R001 Canonicalize `b204-recovery-envelope-final-evidence.md`, this task ledger, and the B205 re-bound `specs/CURRENT.md` state through PR #57 exact head `7224dbb1fd005813796938af2cb783d500fdf7ef`; pre-merge CI `34245532899` and R3 `34245532870` succeeded. Superseded head `63029aad7a2ab94f6592b3b31afd15c2df510cf6` remains cancelled/non-PASS evidence.
- [x] B204R002 Merge exact B204/B205 reconciliation head `7224dbb1fd005813796938af2cb783d500fdf7ef` with explicit `expected_head_sha` protection; durable transport comment `5588329735` records canonical merge `854373dc4fd44cc496b9e52f9f14804cdbf750aa`.
- [x] B204R003 Verify exact reconciliation parentage (`ac0925051cc128580b9794837fd03200f9a7374b` + `7224dbb1fd005813796938af2cb783d500fdf7ef`) and exact post-merge qualification: CI `34250181834` SUCCESS and R3 `34250182026` SUCCESS on `854373dc4fd44cc496b9e52f9f14804cdbf750aa` before B205 implementation began.
- [x] B205 Reject wrong key, tampered header/ciphertext/tag/AAD, unknown suite/version, stale generation, truncation/trailing data, invalid KDF policy, overflow/length mismatch, and random-source failure. PR #58 final head `bcb317b8b2e4417a8a22534f5fed375305915161` passed CI `34251969171` and R3 `34251969189`, merged with explicit expected-head protection as canonical `4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf`, exact parentage is proven, and exact post-merge CI `34253283595` and R3 `34253283658` succeeded. Failed/cancelled predecessor heads remain preserved as NOT PASS. Complete evidence is recorded in `b205-crypto-adversarial-final-evidence.md`.
- [x] B206 Keep media streaming/journal sequencing in Specification 005. B201-B205 remained bounded to Specification 004 cryptographic envelopes and aggregate evidence; no media streaming/journal implementation was absorbed, and Specification 005 remains blocked pending Specification 004 closeout.
- [x] B205R001 Canonicalize `b205-crypto-adversarial-final-evidence.md`, this task ledger, and the B301 re-bound `specs/CURRENT.md` state through PR #59 exact head `ad74fc41dbaa7353b3d3c47b33395b8681e56e82`; pre-merge CI `34254716241` and R3 `34254716268` succeeded.
- [x] B205R002 Merge exact B205/B301 reconciliation head `ad74fc41dbaa7353b3d3c47b33395b8681e56e82` with explicit `expected_head_sha` protection; durable transport comment `5589030694` records canonical merge `c002bc21706bd90622586d453cb9ed9722d53fbb`.
- [x] B205R003 Verify exact reconciliation parentage (`4b8a40fff9f42be87fdd7d00c9f9f58a511cc1cf` + `ad74fc41dbaa7353b3d3c47b33395b8681e56e82`) and exact post-merge qualification: CI `34255936220` SUCCESS and R3 `34255936283` SUCCESS on `c002bc21706bd90622586d453cb9ed9722d53fbb` before B301 implementation began.

## 004B3 encrypted structured-store foundation

- [x] B301 Integrate only the exact reviewed/provenance-registered SQLCipher strategy. PR #60 final head `630ca6ccd545a49d81efbdb08cb1efbab9cab0e4` passed CI `34258332677` and R3 `34258332704`, merged with explicit expected-head protection as canonical `6c30399da307b1b5ea23988a99edf540872cad42`, exact parentage is proven, and exact post-merge CI `34259707758` and R3 `34259707773` succeeded. Failed/cancelled predecessor heads remain preserved as NOT PASS. Complete evidence is recorded in `b301-sqlcipher-provider-final-evidence.md`.
- [x] B301R001 Canonicalize `b301-sqlcipher-provider-final-evidence.md`, this task ledger, and the B302 re-bound `specs/CURRENT.md` state through PR #61 exact head `a2121a3f886f293118705d57b25d27918043a32a`; pre-merge CI `34261574985` and R3 `34261574889` succeeded.
- [x] B301R002 Merge exact B301/B302 reconciliation head `a2121a3f886f293118705d57b25d27918043a32a` with explicit `expected_head_sha` protection; durable transport comment `5589866661` records canonical merge `8af8ddfa4084f98655e692c149e972edf7f7368b`.
- [x] B301R003 Verify exact reconciliation parentage (`6c30399da307b1b5ea23988a99edf540872cad42` + `a2121a3f886f293118705d57b25d27918043a32a`) and exact post-merge qualification: CI `34262792084` SUCCESS and R3 `34262791999` SUCCESS on `8af8ddfa4084f98655e692c149e972edf7f7368b` before B302 implementation began.
- [x] B302 Fail closed unless the opened handle proves encryption is active. PR #62 initial head `d1fd6e11ba4e53b798323a232feef448cb00ffe6` remains NOT PASS: CI `34264057007` / run #162 ended `CANCELLED_NOT_PASS` after Ubuntu/macOS test failures were observed, and R3 `34264057122` / run #139 ended `FAILURE_NOT_PASS`. Forward-only final head `c926c6656417b54e42c09925d8e0f02180bd0de9` passed CI `34264516523` and R3 `34264516517`, merged with explicit expected-head protection as canonical `001e274a321cd0f6c472ce768f1a9910165598fe`, exact parentage is proven, and exact post-merge CI `34266303513` and R3 `34266303537` succeeded. Complete evidence is recorded in `b302-encryption-active-final-evidence.md`.
- [x] B302R001 Canonicalize `b302-encryption-active-final-evidence.md`, this task ledger, and the B303 re-bound `specs/CURRENT.md` state through PR #63 exact head `188b2096163890fa3967f0d3228b1d72f9812a0c`; pre-merge CI `34269945274` and R3 `34269945248` succeeded.
- [x] B302R002 Merge exact B302/B303 reconciliation head `188b2096163890fa3967f0d3228b1d72f9812a0c` with explicit `expected_head_sha` protection; durable transport comment `5590900702` records canonical merge `407d79dc6bc872088423569e9265ea32a094e101`.
- [x] B302R003 Verify exact reconciliation parentage (`001e274a321cd0f6c472ce768f1a9910165598fe` + `188b2096163890fa3967f0d3228b1d72f9812a0c`) and exact post-merge qualification: CI `34271065550` SUCCESS and R3 `34271065553` SUCCESS on `407d79dc6bc872088423569e9265ea32a094e101` before B303 implementation began; durable reconciliation comment `5591073000`.
- [x] B303 Enforce reviewed temp/WAL/journal/provider/build settings. PR #64 final head `c129d8a0959ae68b2bf2c0986a8622cae97c4244` passed CI `34275130542` and R3 `34275130518`, merged with explicit expected-head protection as canonical `b3d4c7de77ec7a072fdcdd09d2d798519236db34`, exact parentage is proven, and exact push-triggered post-merge CI `34276275141` and R3 `34276275137` succeeded. Initial heads `22c83da98ff913443b2bc1eeaffb5ece3c07521a` and `eea61d65b0a58fe7bcac6688eb5561940532bad1` remain preserved as formatting-failure NOT PASS evidence. Complete evidence is recorded in `b303-provider-temp-journal-final-evidence.md`.
- [x] B303R001 Canonicalize `b303-provider-temp-journal-final-evidence.md`, this task ledger, and the B304 re-bound `specs/CURRENT.md` state through PR #65 exact head `798fe493e99c9524b76dfaf065de4514ac511acf`; pre-merge CI `34278147166` and R3 `34278147198` succeeded.
- [x] B303R002 Merge exact B303/B304 reconciliation head `798fe493e99c9524b76dfaf065de4514ac511acf` with explicit `expected_head_sha` protection; durable transport comment `5591956575` records canonical merge `76f6a3ccacc56d58daf4bd9f40d21234a6e50002`.
- [x] B303R003 Verify exact reconciliation parentage (`b3d4c7de77ec7a072fdcdd09d2d798519236db34` + `798fe493e99c9524b76dfaf065de4514ac511acf`) and exact post-merge qualification: CI `34279419817` SUCCESS and R3 `34279419704` SUCCESS on `76f6a3ccacc56d58daf4bd9f40d21234a6e50002` before B304 implementation began; durable reconciliation comment `5592110972`.
- [x] B304 Run normal DB and cipher/page-authentication integrity checks. PR #66 initial head `4841362fac8ea62c4de01ce18ef1465557881a9c` remains NOT PASS: CI `34280979977` / run #173 and R3 `34280979912` / run #150 both ended `FAILURE_NOT_PASS` for the observed formatting failure. Forward-only final head `751a02c13129face0fb5ca828ccf846a483e4941` passed CI `34281346220` and R3 `34281346213`, merged with explicit expected-head protection as canonical `bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7`, exact parentage is proven, and exact push-triggered post-merge CI `34283841909` and R3 `34283842070` succeeded. Complete evidence is recorded in `b304-integrity-final-evidence.md`.
- [x] B304R001 Canonicalize `b304-integrity-final-evidence.md`, this task ledger, and the B305 re-bound `specs/CURRENT.md` state through PR #67 exact head `bfc9bea223975edf4a0f71812e38c58de6ed8443`; pre-merge CI `34285446663` and R3 `34285446650` succeeded.
- [x] B304R002 Merge exact B304/B305 reconciliation head `bfc9bea223975edf4a0f71812e38c58de6ed8443` with explicit `expected_head_sha` protection; durable transport comment `5592820085` records canonical merge `5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be`.
- [x] B304R003 Verify exact reconciliation parentage (`bfba47dd43d2b1c8ca94a9a1fb0c7d0c94a142e7` + `bfc9bea223975edf4a0f71812e38c58de6ed8443`) and exact post-merge qualification: CI `34286770543` SUCCESS and R3 `34286770579` SUCCESS on `5806adb8aa7c0d4cdf8b8f74fe3c3503e204e4be` before B305 implementation began; durable reconciliation comment `5592917396`.
- [x] B305 Add wrong-key/corruption/unsupported-provider/version fixtures. PR #68 final head `55f512820d82100c1f60f6bc7bb79450a9b6181f` passed CI `34288208755` and R3 `34288208747`, merged with explicit expected-head protection as canonical `e27a0b2921af2661037b7af581f6be737c1352de`, exact parentage is proven, and exact push-triggered post-merge CI `34289111683` and R3 `34289111736` succeeded. Byte-identical duplicate PR #69 was closed without merge and is not qualification evidence. Complete evidence is recorded in `b305-negative-fixtures-final-evidence.md`.
- [x] B305R001 Canonicalize `b305-negative-fixtures-final-evidence.md`, this task ledger, and the B306 re-bound `specs/CURRENT.md` state through PR #70 exact head `954948554b371d80d9473a155a48ea7f31edcdf7`; pre-merge CI `34290128656` and R3 `34290128652` succeeded on attempt 1.
- [ ] B305R002 Merge the exact B305/B306 reconciliation head only with explicit `expected_head_sha` protection and durable transport evidence. PR #70 merged as canonical `7cdb8154ec90c59b64b2b47b37111cb491489e92`, but the actual merge API request body is not reconstructible from GitHub after the concurrent merge. Transport-intent comments `5593348515` and `5593354365` are not retroactively upgraded into proof that the request used `expected_head_sha`; this task remains NOT PASS.
- [x] B305R003 Verify exact reconciliation parentage and exact push-triggered post-merge CI/R3 SUCCESS before B306 begins. Parentage is proven as `e27a0b2921af2661037b7af581f6be737c1352de` + `954948554b371d80d9473a155a48ea7f31edcdf7` with tree `b58acfdae5d9df89833d30413623130c80b67461`; post-merge R3 `34291064716` and post-merge CI `34291064751` both succeeded on attempt 1. This evidence does not repair B305R002 and does not by itself authorize B306.
- [x] B305X001 Canonicalize the bounded transport-recovery surfaces through PR #71 exact head `b7f2886a4bbe641aeff7ea071cf261b5c7fd5c6a`; pre-merge CI `34292555783` and R3 `34292555765` succeeded on attempt 1, with repository-owner reconciliation review `5148286720` explicitly `NOT_Q009`.
- [x] B305X002 Merge the exact recovery head only through an observed `expected_head_sha`-guarded merge request and record the accepted canonical merge result. The actual merge invocation used `expected_head_sha = b7f2886a4bbe641aeff7ea071cf261b5c7fd5c6a` with `merge_method = merge`; GitHub returned `merged = true` and canonical merge `c22385e0245d9b372b396bbe9a2d158d30b40052`. Pre-merge binding comment: `5593654712`.
- [x] B305X003 Verify exact recovery parentage plus exact push-triggered post-merge CI/R3 SUCCESS. Canonical recovery merge `c22385e0245d9b372b396bbe9a2d158d30b40052` has parents `7cdb8154ec90c59b64b2b47b37111cb491489e92` + `b7f2886a4bbe641aeff7ea071cf261b5c7fd5c6a`, tree `18c69932b809998292a0e67dc8e667c34312b48e`, post-merge CI `34293423595` and R3 `34293423619` both SUCCESS on attempt 1 / push, with durable qualification comment `5593744878`. B306 may now be rebound by canonical state reconciliation.
- [x] B306 Prove semantic fixture markers/logical IDs are absent from DB/WAL/journal/file-backed-temp/public filenames under qualified configuration. PR #74 final head `35bd5ace1f72fed39ff34236c17ae46a0e73f705` changed only `crates/himsat-core/tests/b306_plaintext_spill.rs`, passed CI `34366925805` / run #191 and R3 `34366925788` / run #168 on attempt 1, merged through observed `expected_head_sha` protection as canonical `7359ca64f6679409ddc81a025fe556c7297845e2`, has exact tree/parentage, and passed push-triggered CI `34369011280` / run #192 plus R3 `34369011241` / run #169 on attempt 1. Durable post-merge qualification comment: `5604388541`. Complete evidence is recorded in `b306-plaintext-spill-final-evidence.md`.
- [x] B307 Use copy-verify-publish migration; preserve verified prior encrypted state until new state is published, anchored, reopened, and verified. PR #76 final head `bcaf38abde83a55cb1caccd81141167911d05a92` passed CI `34383104985` / run #198 and R3 `34383104987` / run #175 on attempt 1 after preserving failed predecessor heads as NOT PASS, merged through observed `expected_head_sha` protection as canonical `fdc1d17a8df49c007f5038336ce267ea5eaf28a8`, has exact tree/parentage, and passed push-triggered CI `34384441066` / run #199 plus R3 `34384441018` / run #176 on attempt 1. Durable post-merge qualification comment: `5606324660`. Complete evidence is recorded in `b307-copy-verify-publish-final-evidence.md`.

## 004B4 platform protectors

- [x] B401 Apple Keychain adapter with proven scope/presence policy; Secure Enclave only for supported reviewed operations. PR #78 accepted head `145e0db14a957760adb811e557372f4aa8f24bd5` passed CI `34407235044` / run #211 and R3 `34407235017` / run #188 on attempt 1, passed genuine Apple-authorized native macOS Data Protection Keychain qualification with `SAME_USER_ACCOUNT`, `NOT_REQUIRED`, `WHEN_PASSCODE_SET_THIS_DEVICE_ONLY`, synchronization disabled, and stronger policy rejection (comment `5610427033`), merged through observed `expected_head_sha` protection as canonical `880165a40108bdf0c27e7246c1b890e8456e9768`, has exact parentage/tree, and passed push-triggered CI `34419047465` / run #212 plus R3 `34419047471` / run #189 on attempt 1. Durable post-merge qualification comment: `5610513209`. Complete evidence is recorded in `b401-apple-keychain-final-evidence.md`.
- [ ] B402 Android Keystore adapter with actual app/UID scope, auth policy, invalidation, and hardware capability state.
- [ ] B403 Windows current-user DPAPI/CNG-class adapter reporting `SAME_USER_ACCOUNT`; stronger scope/presence fails unless separately proven.
- [ ] B404 Linux Secret Service adapter reporting actual scope, minimal non-secret attributes, and no plaintext fallback.
- [ ] B405 Mismatched app/vault/generation/policy or unavailable/locked/invalidated protector fails closed and returns no VRK.
- [ ] B406 Prove restart/revocation/unsupported-policy/platform-negative paths where automation permits.

## 004B5 freshness, backup, rotation, deletion

- [ ] B501 Implement authenticated manifest + explicit protected genesis + established OS freshness anchor with rollback/replay/freshness-gap detection.
- [ ] B502 Implement explicit older-backup restore as a new epoch; fresh-device restore uses reviewed protected genesis and records inability to prove global newest-ness.
- [ ] B503 Implement protector/recovery rotation and crash-atomic full VRK seven-phase rotation with normal writes quiesced and reviewed phase invariants.
- [ ] B504 Inject failure before/after every rotation commit point and prove canonical decryptability/recovery.
- [ ] B505 Implement opaque-name portable backup/restore and exact provider-visible metadata allowlist checks.
- [ ] B506 Implement active-vault deletion across canonical/derived/temp/wrap surfaces while preserving detached-backup and physical-erasure limitations.

## R3 qualification

- [ ] Q001 Exact dependency/license/SBOM/provenance closure.
- [ ] Q002 Exact-head fmt/lint/build/tests on supported target matrix.
- [ ] Q003 Positive vault identity/create/lock/unlock/restart, encrypted DB/blob, recovery/backup, freshness genesis/advance, and rotation evidence.
- [ ] Q004 Random-source failure, wrong-key/passphrase, malformed/truncated/trailing/overflow envelope, tamper, transplant, nonce duplicate/collision, genesis replay, rollback/replay, protector/policy, and post-lock-handle adversarial evidence.
- [ ] Q005 SQLCipher encryption-active, cipher-integrity, WAL/journal/temp plaintext-spill evidence.
- [ ] Q006 Interrupted genesis/rotation/recovery/restore phase evidence.
- [ ] Q007 Backup/filesystem/secret-store metadata allowlist and secret/log/crash-output leakage evidence.
- [ ] Q008 Platform capability evidence; no universal app-exclusive, user-presence, atomic-anchor, or hardware-backed claim.
- [ ] Q009 Independent substantive crypto/security review of exact implementation revision.
- [ ] Q010 Reconcile all blocking findings, reviews, threads, comments, diff, `main`, and mergeability.
- [ ] Q011 Merge with exact expected head and verify canonical parentage.
- [ ] Q012 Require post-merge CI and R3 SUCCESS.

## Closeout

- [ ] C001 Record durable design/review/provider/provenance/implementation/adversarial/platform/residual-risk evidence.
- [ ] C002 Restore any temporary policy allowance to conservative successor posture.
- [ ] C003 Exact-head qualify closeout.
- [ ] C004 Expected-head merge closeout and require post-closeout CI/R3.
- [ ] C005 Mark Specification 004 `CLOSED_CANONICAL` only after all prior gates are proven.

## Explicit non-tasks

- custom cryptographic primitives/protocols;
- media chunk journal/streaming implementation;
- capture/background services;
- sync/pairing/relay;
- connectors/plugins/agents/external writes;
- model/biometric/update-signing behavior;
- mandatory cloud/account/key escrow;
- physical secure-erase or universal metadata-secrecy claims;
- release/FIPS/compliance claims without separate qualification.