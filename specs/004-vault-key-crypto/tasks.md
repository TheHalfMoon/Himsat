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
- [x] R003 Classify all 16 actionable findings as blocking 004B implementation; record exact SHA, reviewer identity, review state, evidence limitations, and Himsat governance disposition `CHANGES_REQUIRED` in `review-evidence.md`.
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
- [ ] P011 Canonicalize this state-only reconciliation with expected-head protection and require exact post-merge CI/R3 before B101 implementation starts.

## 004B1 portable vault contracts

B101 is the only implementation leaf conditionally opened after P011 completes. B102-B105 remain blocked by dependency order.

- [ ] B101 Add only reviewed `VaultId`, generation, freshness, lock/error, lease, and provider-neutral `SecretProtector` contract surface.
- [ ] B102 Implement typed lock/unlock/protector/freshness errors and revocable keyed-handle lease.
- [ ] B103 Implement reviewed `SecretProtector` behavior contract without pretending platform mechanics/capabilities are identical.
- [ ] B104 Implement reviewed key hierarchy/domain separation; on lock/rotation/revocation/fatal failure revoke lease first, close handles, release VRK/Recovery KEK/purpose keys, and zeroize owned buffers where reviewed runtime support exists.
- [ ] B105 Prove previously obtained DB/blob handles reject reads/writes after lock/revocation/failure; document process-teardown/swap/crash/runtime memory limits as residual risk.

## 004B2 cryptographic envelope foundation

- [ ] B201 Implement reviewed HKDF-SHA-256 domain separation and deterministic test vectors.
- [ ] B202 Implement reviewed versioned bounded-blob XChaCha20-Poly1305 envelope with exact public bytes, v1 AAD, checked parser bounds, and 64 MiB plaintext ceiling.
- [ ] B203 Implement fresh 24-byte OS-CSPRNG nonce per attempt, manifest reservation, retry/regeneration, restore behavior, and duplicate/collision rejection.
- [ ] B204 Implement recovery envelope with exact public bytes, Argon2id v1 policy, canonical recovery AAD, uniform authentication failure, parser bounds, and transplant rejection.
- [ ] B205 Reject wrong key, tampered header/ciphertext/tag/AAD, unknown suite/version, stale generation, truncation/trailing data, invalid KDF policy, overflow/length mismatch, and random-source failure.
- [ ] B206 Keep media streaming/journal sequencing in Specification 005.

## 004B3 encrypted structured-store foundation

- [ ] B301 Integrate only the exact reviewed/provenance-registered SQLCipher strategy.
- [ ] B302 Fail closed unless the opened handle proves encryption is active.
- [ ] B303 Enforce reviewed temp/WAL/journal/provider/build settings.
- [ ] B304 Run normal DB and cipher/page-authentication integrity checks.
- [ ] B305 Add wrong-key/corruption/unsupported-provider/version fixtures.
- [ ] B306 Prove semantic fixture markers/logical IDs are absent from DB/WAL/journal/file-backed-temp/public filenames under qualified configuration.
- [ ] B307 Use copy-verify-publish migration; preserve verified prior encrypted state until new state is published, anchored, reopened, and verified.

## 004B4 platform protectors

- [ ] B401 Apple Keychain adapter with proven scope/presence policy; Secure Enclave only for supported reviewed operations.
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