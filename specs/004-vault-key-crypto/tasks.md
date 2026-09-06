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
- [ ] R004 Merge forward remediation of D001-D016 after exact-head CI/R3 and pre-merge reconciliation.
- [ ] R005 Repoint/create review-only head at the remediated exact canonical SHA and obtain new substantive independent crypto/security review.
- [ ] R006 Record final design-review disposition, blocking/non-blocking findings, recommendations, and residual risks for the remediated exact SHA. Old review of `384608c8...`, self-review, CI, billing-blocked/skipped/neutral output are not substitutes.

## 004A remediation findings

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

## 004A remediation qualification

- [ ] A001 Confirm remediation diff changes only Specification 004 docs/state and stays within Diffcipline bounds; no product code, Cargo dependency, provenance adoption, or generated SBOM change.
- [ ] A002 Exact-head run existing CI and Diffcipline R3 and require both SUCCESS.
- [ ] A003 Reconcile PR reviews, review threads, comments, exact diff, canonical `main`, and mergeability immediately before merge.
- [ ] A004 Merge with exact expected-head protection.
- [ ] A005 Re-read canonical remediation merge and require post-merge CI plus R3 SUCCESS.
- [ ] A006 Re-review the exact canonical remediation SHA; security-semantic changes make the first CodeRabbit review stale as final approval evidence.

## Dependency/provenance decision

Blocked until R005-R006/A006 close with no unresolved blocking design finding.

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
