# Specification 004 Tasks — Vault, Key, and Crypto Architecture

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and independent review evidence remain authoritative.

## Shaping

- [x] S001 Re-read canonical `main` at Specification 003 closeout merge `1f14bbe004962dd164402e6e6c7f9c046cf5b489`.
- [x] S002 Confirm post-closeout CI run `34047579266` completed successfully on the exact closeout merge.
- [x] S003 Re-read constitution, architecture, security/platform plan, qualification strategy, and master-plan unit 004.
- [x] S004 Refresh current upstream facts for Argon2id, XChaCha20-Poly1305/secretstream, Apple Keychain/Secure Enclave, Android Keystore, Windows DPAPI/CNG, Linux Secret Service, and SQLCipher.
- [x] S005 Record research without granting dependency/donor adoption authority.
- [x] S006 Recursively split Specification 004 into reviewed design (004A) before encrypted-foundation implementation (004B).
- [x] S007 Define protected assets, adversaries/failures, security goals, non-goals, and metadata/deletion limits.
- [x] S008 Define candidate key hierarchy, recovery, OS protector, structured-store, blob-envelope, rotation, backup, and deletion contracts.
- [x] S009 Define independent exact-revision crypto/security review as a hard implementation gate.
- [x] S010 Define R3 positive/adversarial/platform/provenance evidence before implementation.
- [ ] S011 Exact-head qualify the shaping candidate with existing CI plus Diffcipline R3.
- [ ] S012 Reconcile shaping diff, reviews/threads/comments, `main`, and mergeability.
- [ ] S013 Merge shaping with explicit expected-head protection.
- [ ] S014 Re-read canonical shaping merge and require post-shaping CI success.

## 004A independent design review

Authorized only after S011-S014 are proven.

- [ ] R001 Prepare exact canonical design SHA and review scope.
- [ ] R002 Obtain substantive independent crypto/security review tied to that exact SHA.
- [ ] R003 Classify every finding as blocking, non-blocking, residual risk, or accepted constraint.
- [ ] R004 Resolve blocking findings forward without rewriting shared history.
- [ ] R005 If security semantics change, invalidate stale review and obtain review on the new exact revision.
- [ ] R006 Record clear review disposition; absent/skipped/billing-blocked/self-review is not PASS.

## Dependency/provenance decision

Blocked until R001-R006 complete.

- [ ] P001 Select exact established crypto provider/library strategy from the reviewed candidates.
- [ ] P002 Select exact SQLCipher source/version/build/provider strategy if SQLCipher remains the reviewed choice.
- [ ] P003 Inventory every resulting Cargo/native dependency and immutable source/version.
- [ ] P004 Verify controlling licenses and notices.
- [ ] P005 Record adoption mode and exact package/source closure in the provenance registry/SBOM boundary before implementation bytes enter Himsat.
- [ ] P006 Prove no incompatible or unregistered dependency enters the lockfile/native distribution graph.

## 004B1 portable vault contracts

Blocked until review and provenance gates complete and the leaf is re-bounded from live canonical truth.

- [ ] B101 Add only the reviewed portable vault contract surface.
- [ ] B102 Implement typed lock/unlock/error and key-generation state.
- [ ] B103 Implement provider-neutral OS `SecretProtector` boundary.
- [ ] B104 Implement reviewed key hierarchy/domain separation with secret-zeroization behavior where practical.
- [ ] B105 Add deterministic positive and adversarial unit fixtures without real user data.

## 004B2 encrypted blob foundation

- [ ] B201 Implement reviewed versioned bounded-blob envelope.
- [ ] B202 Authenticate context before releasing plaintext.
- [ ] B203 Reject wrong key, tampered header/ciphertext/tag/AAD, unknown suite/version, stale generation, and truncation.
- [ ] B204 Keep media streaming/journal sequencing out of scope for Specification 005.

## 004B3 encrypted structured-store foundation

- [ ] B301 Integrate the reviewed SQLCipher strategy if adopted.
- [ ] B302 Fail closed unless the opened handle proves encryption is active.
- [ ] B303 Enforce reviewed temp/WAL/journal settings.
- [ ] B304 Implement normal and cipher-integrity checks.
- [ ] B305 Add wrong-key/corruption/unsupported-version fixtures.
- [ ] B306 Prove sensitive marker text is absent from database/WAL/journal/file-backed-temp surfaces under the qualified configuration.
- [ ] B307 Make rekey/migration recovery preserve a verified previous encrypted state until replacement is proven valid.

## 004B4 platform secret protectors

- [ ] B401 Implement the currently authorized Apple Keychain protector boundary; use Secure Enclave only where the reviewed native operation model supports it.
- [ ] B402 Implement Android Keystore protector with capability/hardware-backed state surfaced rather than assumed.
- [ ] B403 Implement Windows current-user DPAPI/CNG-class protector.
- [ ] B404 Implement Linux Secret Service protector with secret-free lookup attributes.
- [ ] B405 Unsupported/unavailable/locked protector paths fail explicitly; no plaintext key-file fallback.
- [ ] B406 Add platform-negative tests for missing/invalidated/denied protector state where automation permits.

## 004B5 recovery, rotation, backup, and deletion

- [ ] B501 Implement opt-in reviewed Argon2id recovery envelope.
- [ ] B502 Implement wrong-passphrase/tamper/version failure paths without partial plaintext release.
- [ ] B503 Implement protector and recovery-KEK rotation.
- [ ] B504 Implement bounded VRK generation rotation/migration proof.
- [ ] B505 Implement encrypted backup/restore inventory and authenticated restore checks.
- [ ] B506 Implement application-managed deletion across canonical/derived/temp/wrap surfaces and document physical-erasure limits.

## R3 qualification

- [ ] Q001 Exact dependency/license/SBOM/provenance closure.
- [ ] Q002 Exact-head fmt/lint/build/tests on the supported target matrix.
- [ ] Q003 Positive vault create/lock/unlock, encrypted DB, encrypted blob, backup/recovery, and rotation evidence.
- [ ] Q004 Wrong-key/passphrase/tamper/version/rollback/protector-failure adversarial evidence.
- [ ] Q005 SQLCipher encryption-active, cipher-integrity, WAL/journal/temp plaintext-spill evidence.
- [ ] Q006 Interrupted rekey/rotation/restore recovery evidence.
- [ ] Q007 Secret/log/crash-output leakage negative evidence.
- [ ] Q008 Platform capability evidence; no universal hardware-backed claim.
- [ ] Q009 Independent substantive crypto/security review of the exact implementation revision.
- [ ] Q010 Reconcile all blocking findings, reviews, threads, comments, diff, `main`, and mergeability.
- [ ] Q011 Guard merge with exact expected head and verify canonical parentage.
- [ ] Q012 Require post-merge CI success.

## Closeout

- [ ] C001 Record durable design, review, provider/provenance, implementation, negative, platform, and residual-risk evidence.
- [ ] C002 Restore any temporary bounded policy allowance to the conservative successor posture.
- [ ] C003 Exact-head qualify closeout.
- [ ] C004 Expected-head merge closeout and require post-closeout CI.
- [ ] C005 Mark Specification 004 `CLOSED_CANONICAL` only after all prior gates are proven.

## Explicit non-tasks

- custom cryptographic primitives or protocols;
- media chunk journal/streaming implementation;
- capture/background services;
- sync/pairing/relay;
- connectors/plugins/agents/external writes;
- model/biometric/update-signing behavior;
- mandatory cloud/account/key escrow;
- physical secure-erase claims;
- release/FIPS/compliance claims without separate qualification.
