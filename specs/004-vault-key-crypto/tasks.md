# Specification 004 Tasks — Vault, Key, and Crypto Architecture

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and independent review evidence remain authoritative.

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

## 004A independent design review — first pass

- [x] R001 Prepare review-only PR #12 with base `1f14bbe004962dd164402e6e6c7f9c046cf5b489` and head pointing directly to exact canonical design `384608c8fc13531c399f3726caa5022eb3612aa2`.
- [x] R002 Obtain substantive independent CodeRabbit review `PRR_kwDOUQPwRs8AAAABMYy3gw` on the exact canonical design; GitHub state `COMMENTED`, submitted 2026-09-06T18:47:41Z, 16 actionable findings.
- [x] R003 Classify all 16 actionable findings as blocking 004B implementation; record exact SHA, reviewer identity, review state, evidence limitations, and Himsat governance disposition `CHANGES_REQUIRED` in `review-evidence.md`.
- [x] R004 Merge forward remediation D001-D016 in PR #16 with expected-head protection; canonical merge `6d1bbc9b55690939833917eebd447c361627f48a` contains exact remediation head `befb5026a778491f20dce456fdb9b855d4a6377a`.
- [x] R005 Obtain second substantive independent CodeRabbit review on review-only PR #18 for exact canonical SHA `6d1bbc9b55690939833917eebd447c361627f48a`.
- [x] R006 Record second-review `CHANGES_REQUIRED` disposition, two blockers D017-D018, non-blocking recommendations, and residual risks in `review-round2-evidence.md`; old review/self-review/CI/skipped/billing-blocked output are not substitutes.
- [ ] R007 Reconcile/canonicalize the round-2 ledger-only state and point a successor review-only PR directly at the resulting exact canonical SHA.
- [ ] R008 Obtain a new substantive independent crypto/security disposition on that exact canonical SHA with no unresolved blocking finding before provider/dependency selection.

## 004A first-remediation findings

- [x] D001 Specify OS-CSPRNG-only production randomness, exact lengths, fail-closed errors, and prohibited fallback.
- [x] D002 Define protector ownership/access scope and user-presence policy with fail-closed unsupported-policy behavior.
- [x] D003 Require exact SQLCipher core/binding/provider/SQLite/build/transitive provenance identity before P002.
- [x] D004 Define metadata-leakage qualification across DB/filesystem/backup-provider/secret-store boundaries.
- [x] D005 Define Apple/Android/Windows/Linux application/same-user access, restart, and revocation semantics.
- [x] D006 Freeze versioned Argon2id recovery policy, validation/resource bounds, and passphrase handling.
- [x] D007 Define canonical recovery AAD binding `VaultId`, generation, envelope type/policy/suite; require transplant-negative test.
- [x] D008 Define externally uniform wrong-passphrase/tag failure with typed fail-closed result and no partial plaintext.
- [x] D009 Freeze dependency-free canonical AAD encoding and exact bounded-blob/recovery schemas; keep SQLCipher record-level AEAD out of scope.
- [x] D010 Freeze XChaCha nonce generation, storage, retry/restore/generation behavior, collision handling, and duplicate-canonical detection.
- [x] D011 Define OS-protected freshness anchor, authenticated manifest chain, rollback/replay detection, recovery, and fresh-device residual risk.
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
- [x] D018 Freeze distinct manifest-purpose HKDF domain, canonical authenticated manifest envelope/plaintext/AAD/nonce/hash/parser bounds, and fail-closed atomic freshness-anchor contract.

## 004A second-remediation qualification

- [x] A201 Confirm PR #21 exact diff is limited to `specs/004-vault-key-crypto/spec.md` and `review-round2-evidence.md`; no implementation, dependency, provenance-adoption, SBOM, workflow, donor, model, dataset, or asset change.
- [x] A202 Exact-head qualify `4357102d388248400116eeab45cf83de71e35051`: CI `34057694827` SUCCESS and R3 `34057694832` SUCCESS.
- [x] A203 Reconcile PR #21 exact diff, comments, reviews, review threads, canonical `main`, and mergeability immediately before merge; Qodo billing block and CodeRabbit auto-skip were not counted as PASS.
- [x] A204 Merge PR #21 with `expected_head_sha = 4357102d388248400116eeab45cf83de71e35051`; canonical merge `cb8511c1b420c58f714768c3561e74f04f026b3a` has parents `6d1bbc9b55690939833917eebd447c361627f48a` and `4357102d388248400116eeab45cf83de71e35051`.
- [x] A205 Require post-merge qualification on exact canonical `cb8511c1b420c58f714768c3561e74f04f026b3a`: CI `34057861438` SUCCESS and R3 `34057861447` SUCCESS.
- [ ] A206 Exact-head qualify and merge the ledger-only reconciliation without changing security semantics.
- [ ] A207 Obtain a substantive independent crypto/security review on the resulting exact canonical SHA; any blocking finding requires forward remediation and another exact-SHA review.

## Dependency/provenance decision

Blocked until R008/A207 close with no unresolved blocking design finding.

- [ ] P001 Select exact established crypto provider/library strategy from reviewed candidates.
- [ ] P002 Select exact SQLCipher core/binding/provider/build strategy, if SQLCipher remains selected, with immutable core/binding identities and exact SQLite/provider/native inputs.
- [ ] P003 Inventory every direct/transitive Cargo/native dependency, immutable source/version, package source, and checksum.
- [ ] P004 Verify controlling licenses/notices under `governance/provenance/policy.json`.
- [ ] P005 Record adopted package/native closure in provenance registry and deterministic SBOM/notices before accepting unregistered external dependency bytes.
- [ ] P006 Prove resulting lockfile/native distribution contains no unregistered, checksum-mismatched, manual-without-decision, denied, or unknown-license component.

## 004B1 portable vault contracts

Blocked until review and provenance gates close and this leaf is re-bounded from live canonical truth.

- [ ] B101 Add only reviewed `VaultId`, generation, freshness, lock/error, lease, and provider-neutral `SecretProtector` contract surface.
- [ ] B102 Implement typed lock/unlock/protector/freshness errors and revocable keyed-handle lease.
- [ ] B103 Implement reviewed `SecretProtector` behavior contract without pretending platform mechanics/capabilities are identical.
- [ ] B104 Implement reviewed key hierarchy/domain separation; on lock/rotation/revocation/fatal failure revoke lease first, close handles, release VRK/Recovery KEK/purpose keys, and zeroize owned buffers where reviewed runtime support exists.
- [ ] B105 Prove previously obtained DB/blob handles reject reads/writes after lock/revocation/failure; document process-teardown/swap/crash/runtime memory limits as residual risk.

## 004B2 cryptographic envelope foundation

- [ ] B201 Implement reviewed HKDF-SHA-256 domain separation and deterministic test vectors.
- [ ] B202 Implement reviewed versioned bounded-blob XChaCha20-Poly1305 envelope with exact v1 AAD and 64 MiB plaintext ceiling.
- [ ] B203 Implement fresh 24-byte OS-CSPRNG nonce per attempt, manifest reservation, retry/regeneration, restore behavior, and duplicate/collision rejection.
- [ ] B204 Implement recovery envelope with exact Argon2id v1 policy, canonical recovery AAD, uniform authentication failure, and transplant rejection.
- [ ] B205 Reject wrong key, tampered header/ciphertext/tag/AAD, unknown suite/version, stale generation, truncation, invalid KDF policy, and random-source failure.
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

- [ ] B501 Implement authenticated manifest + OS freshness anchor with rollback/replay/freshness-gap detection.
- [ ] B502 Implement explicit older-backup restore as a new epoch; fresh-device restore records inability to prove global newest-ness.
- [ ] B503 Implement protector/recovery rotation and crash-atomic full VRK seven-phase rotation with normal writes quiesced.
- [ ] B504 Inject failure before/after every rotation commit point and prove canonical decryptability/recovery.
- [ ] B505 Implement opaque-name portable backup/restore and exact provider-visible metadata allowlist checks.
- [ ] B506 Implement active-vault deletion across canonical/derived/temp/wrap surfaces while preserving detached-backup and physical-erasure limitations.

## R3 qualification

- [ ] Q001 Exact dependency/license/SBOM/provenance closure.
- [ ] Q002 Exact-head fmt/lint/build/tests on supported target matrix.
- [ ] Q003 Positive vault identity/create/lock/unlock/restart, encrypted DB/blob, recovery/backup, freshness, and rotation evidence.
- [ ] Q004 Random-source failure, wrong-key/passphrase, tamper, transplant, nonce duplicate/collision, rollback/replay, protector/policy, and post-lock-handle adversarial evidence.
- [ ] Q005 SQLCipher encryption-active, cipher-integrity, WAL/journal/temp plaintext-spill evidence.
- [ ] Q006 Interrupted rotation/recovery/restore phase evidence.
- [ ] Q007 Backup/filesystem/secret-store metadata allowlist and secret/log/crash-output leakage evidence.
- [ ] Q008 Platform capability evidence; no universal app-exclusive, user-presence, or hardware-backed claim.
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
