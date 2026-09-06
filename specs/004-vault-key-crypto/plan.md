# Specification 004 Plan — Vault, Key, and Crypto Architecture

## Objective

Deliver a reviewed R3 cryptographic design first, then only after exact-revision independent re-review and provenance closure implement the smallest encrypted structured/blob foundation required by Specification 005.

## Current exact frontier

```text
SPEC_003_CLOSEOUT_MERGE = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
SPEC_003_POST_CLOSEOUT_CI = 34047579266_SUCCESS
SPEC_004_SHAPING_HEAD = 037295ffed1e6e065c70b90216751404d4f8ff18
SPEC_004_EXACT_SHAPING_CI = 34048208674_SUCCESS
SPEC_004_EXACT_SHAPING_R3 = 34048208667_SUCCESS
SPEC_004_SHAPING_MERGE = 384608c8fc13531c399f3726caa5022eb3612aa2
SPEC_004_POST_SHAPING_CI = 34048394581_SUCCESS
SPEC_004_POST_SHAPING_R3 = 34048394550_SUCCESS
SPEC_004_REVIEW_ONLY_PR = 12
SPEC_004_REVIEWER = coderabbitai
SPEC_004_REVIEW_STATE = COMMENTED
SPEC_004_ACTIONABLE_FINDINGS = 16
HIMSAT_GOVERNANCE_DISPOSITION = CHANGES_REQUIRED
IMPLEMENTATION_AUTHORITY = NONE
```

The 16 findings are classified in `review-evidence.md`. This remediation changes security semantics, so the review of `384608c8...` cannot become final approval evidence for the remediated design. A new exact canonical SHA must be independently reviewed before dependency selection or implementation.

## Recursive delivery split

```text
004A1 shaping                         MERGED
  -> 004A2 independent review         CHANGES_REQUIRED
  -> 004A3 design remediation         ACTIVE
  -> 004A4 exact canonical re-review  BLOCKED_PENDING_REMEDIATION
  -> 004P provider/provenance choice  BLOCKED_PENDING_REVIEW
  -> 004B encrypted foundation        BLOCKED_PENDING_REVIEW_AND_PROVENANCE
  -> closeout
```

No task below 004P can pull authority forward from a candidate design, green CI, a stale review, or preparatory research.

## 004A3 — remediation work

### A1. Cryptographic randomness — D001

The design uses only OS cryptographic randomness or a separately reviewed provider proven to route to it.

Required production sources:

- Apple: `SecRandomCopyBytes` class;
- Windows: `BCryptGenRandom` system-preferred RNG class;
- Linux/Android: `getrandom`/platform cryptographic RNG class.

Required random lengths are frozen in `spec.md`: VRK 32 bytes, `VaultId` 16 bytes, recovery salt 16 bytes, XChaCha nonce 24 bytes. Entropy-source failure aborts before publication. No deterministic, time/device seeded, unknown-quality UUID, ordinary PRNG, zero-value, or best-effort fallback is allowed outside deterministic tests.

Implementation evidence later must inject random-source failure and prove no protector/envelope/manifest/state becomes canonical.

### A2. Key hierarchy and domain separation

The remediated design freezes:

- random 32-byte VRK per non-zero key generation;
- 16-byte immutable non-secret `VaultId`;
- HKDF-SHA-256 purpose derivation;
- salt = raw `VaultId` bytes;
- exact structured-store and blob `info` labels plus `u64be(key_generation)`;
- 32-byte purpose-key outputs.

No additional purpose may reuse an existing domain label.

### A3. OS secret-protector access policy — D002/D005

The portable contract exposes actual enforcement rather than pretending all systems provide the same isolation.

Access scopes:

```text
APP_EXCLUSIVE
SAME_USER_ACCOUNT
SAME_USER_SESSION
```

Presence policies:

```text
NOT_REQUIRED
REQUIRED_EACH_HIMSAT_UNLOCK
```

The adapter must fail `UnsupportedPolicy` when it cannot prove the requested scope/presence policy; no silent weakening is allowed.

Platform baseline:

- Apple Keychain: app/access-group scoped; user-presence controls where requested; Secure Enclave only where the native operation model applies.
- Android Keystore: app-owned alias/non-exportable protector; optional per-unlock authentication; TEE/StrongBox reported, not assumed.
- Windows DPAPI/CNG baseline: `SAME_USER_ACCOUNT`; never claim app-exclusive isolation from every same-user process. Stronger/presence policy requires a separately reviewed native mechanism or fails closed.
- Linux Secret Service baseline: `SAME_USER_SESSION` unless the exact service proves stronger. Lookup attributes remain non-secret and minimal; stronger/presence policy fails if not enforceable.

Every record is bound to Himsat app identity, `VaultId`, generation, and policy. Process restart requires unlock again. Revocation blocks future unlock but cannot erase a VRK already stolen from an unlocked compromised process; full rotation addresses future data under a new generation.

### A4. Recovery KDF and envelope — D006/D007/D008

V1 recovery is fixed, not runtime-auto-tuned:

```text
policy = ARGON2ID_RFC9106_64M_V1
version = 0x13
memory_kib = 65536
passes = 3
parallelism = 4
salt_bytes = 16
output_bytes = 32
passphrase_min_unicode_scalars = 16
passphrase_max_utf8_bytes = 1024
```

Passphrase bytes are exact UTF-8 with no trim/case-fold/truncation/normalization. Envelope parameters are validated before KDF allocation; below-policy or attacker-selected arbitrary profiles are rejected. Resource inability returns `ResourceLimit`, never a weaker fallback.

The Recovery KEK authenticates/wraps the VRK using the reviewed XChaCha20-Poly1305 candidate and canonical recovery AAD from `spec.md`. Wrong passphrase and tag failure share `RecoveryAuthenticationFailed`, release no plaintext, and do not log secret-sensitive distinctions. Cross-vault/generation transplant is a mandatory negative test.

### A5. Canonical context binding and bounded-blob nonce — D009/D010

`spec.md` freezes a dependency-free binary AAD encoding using ASCII domain strings with explicit lengths, big-endian integers, and fixed 16-byte IDs.

Bounded blob v1 is limited to 64 MiB plaintext. Each encryption attempt obtains a fresh random 24-byte nonce, stored in the envelope. Retries after ambiguous publication use a new nonce. Existing authenticated backup copies preserve their nonce; re-encryption uses a new nonce.

Canonical publication reserves `(key_generation, nonce)` in authenticated inventory. A detected candidate collision is discarded/regenerated. A duplicate found in canonical inventory is `CorruptOrTampered` and blocks writes pending recovery.

Structured-store page authentication remains SQLCipher/provider-native in 004B; adding application-level record AEAD later requires a distinct reviewed AAD contract.

### A6. Rollback/replay freshness — D011

The OS protector stores a small `FreshnessAnchor` outside rollbackable vault files:

```text
VaultId
highest_epoch
manifest_hash
```

The encrypted authenticated manifest carries epoch, previous hash, active generation, inventory, and rotation state. Normal state transitions and recovery rules are frozen in `spec.md`.

A normal commit writes/flushes encrypted objects, writes/fsyncs manifest `N+1`, verifies it, compare-and-advances the OS anchor, then retires superseded material. Stale epochs and hash mismatch fail closed.

An intentional restore never decrements the anchor: verified older data is republished into a new epoch greater than the trusted local anchor. A fresh device without a prior anchor can authenticate a backup but cannot prove global newest-ness; that limitation remains explicit residual risk.

### A7. Crash-atomic full VRK rotation — D012

V1 freezes ordinary writes during full root rotation and uses copy-verify-publish rather than destroying/rekeying the only canonical copy in place.

Phases:

```text
PREPARE -> STAGE -> VERIFY -> PUBLISH -> ANCHOR -> ACTIVATE -> RETIRE
```

Old-generation material remains until the new encrypted set is complete, authenticated, published, anchored, reopened, and integrity-verified. Power-loss tests are required before/after every durable phase boundary, including manifest publication, anchor advancement, activation, and old-generation destruction.

### A8. Metadata leakage and portable backup — D004/D013/D014

The allowed provider-visible metadata set is frozen in `spec.md`. Backup objects use opaque random storage names; logical vault/object IDs and user filenames stay in the encrypted manifest. Qualification must inspect four boundaries independently:

1. local database/WAL/journal/temp files;
2. local blob/filesystem names and public envelope bytes;
3. portable-backup provider view;
4. OS secret-store labels/attributes.

Fixtures must contain distinctive semantic markers and logical IDs; the test fails if they appear outside the explicitly allowed encrypted/public surfaces.

Closeout records unavoidable residual leakage: ciphertext size distributions, counts, operation/upload timing, and rotation cadence.

Detached exported backups are independently retained copies. Active-vault deletion cannot remotely revoke a recovery-wrapped backup, provider snapshot, or user-made duplicate. The product must state this at export/deletion time; physical erase and provider deletion are separate concerns.

### A9. Locked key/handle lifecycle — D016

An unlocked vault owns a revocable lease. Keyed handles validate the lease before reads/writes. Lock, revocation, fatal integrity/freshness failure, and relevant rotation transitions revoke the lease before handle closure, release key objects, zeroize owned secret buffers where the reviewed runtime/provider supports it, and discard owned plaintext caches.

Mandatory negative evidence later:

- use a previously obtained DB/blob handle after lock;
- use it after protector revocation;
- inject unlock/integrity failure after partial setup;
- verify all such operations reject without plaintext read/write.

Memory copies outside Himsat/runtime control remain residual risk; zeroization is not represented as protection from a fully compromised unlocked process.

## Independent re-review gate — D015 plus final disposition

The durable first-review record is `review-evidence.md` and includes exact SHA `384608c8fc13531c399f3726caa5022eb3612aa2`, reviewer identity, GitHub review ID/state, finding classification, and Himsat `CHANGES_REQUIRED` disposition.

After remediation merges:

1. re-read canonical `main` and freeze the new exact design SHA;
2. require CI and R3 success on that exact canonical SHA;
3. move/create a review-only head that points directly to the canonical SHA with no review-content commit;
4. request a substantive independent crypto/security re-review;
5. record reviewer, exact SHA, review state, findings, recommendations, residual risks, and Himsat disposition;
6. if semantics change again, invalidate the stale review and repeat.

`APPROVE`/no-blocking-findings evidence must be substantive. Author self-review, Qodo billing block, Cubic limit/neutral output, skipped CodeRabbit output, CI green, or an old review of `384608c8...` cannot satisfy this gate.

## 004P — dependency/provenance decision

Blocked until the remediated canonical design passes independent re-review with no unresolved blocking finding.

### P1. Crypto provider candidates

Preparatory research may record candidates but grants no adoption authority. Current immutable upstream candidate tags observed during remediation include:

```text
chacha20poly1305 0.11.0 -> RustCrypto/AEADs e37a978ccf0992d9053fbc039470d6527108e393
hkdf 0.13.0             -> RustCrypto/KDFs bfb3b209abeeaa02277935b167e06bac320b2773
argon2 0.6.0            -> RustCrypto/password-hashes b1e0ad6fe229b1ba74e4696c7359ab45d7e931f0
zeroize 1.9.0           -> RustCrypto/utils 0b715735a660a8566ccd240bf42489fe2ed98efb
rusqlite 0.40.2          -> rusqlite/rusqlite e88f112bef7899234a497baed5cc3c3d553deeb8
```

These are research identities only. Exact crates.io package checksums, full transitive closure, feature graph, licenses/notices, and provider behavior must be verified before registry adoption.

### P2. SQLCipher exact integration record — D003

The existence of upstream SQLCipher 4.18.0 is research only and is not a provider identity.

Before selecting SQLCipher integration, one record must freeze:

```text
upstream repository + immutable revision/tag
SQLCipher core version
SQLite baseline
binding/Cargo package + immutable version/revision
core and binding licenses
crypto provider + exact version/license
linked vs bundled build mode
native compiler/toolchain inputs
security-relevant compile definitions/features
target matrix
Cargo and native transitive closure
package/source checksums
required notices
```

If a Rust binding vendors an SQLCipher amalgamation, the actual vendored source revision/version must be proven from that binding; it must not be assumed equal to the newest upstream SQLCipher release.

### P3. Machine adoption gate

For every adopted external package/native component:

1. exact immutable source/version and controlling license;
2. exact package source/checksum identity;
3. transitive Cargo/native closure;
4. notices and build/provider identity;
5. Himsat provenance registry/SBOM entry **before** unregistered external lockfile/native bytes are accepted;
6. proof the resulting dependency graph contains no unregistered or policy-incompatible component.

## 004B prospective implementation leaves

The exact implementation split is re-bounded only after 004P. Current dependency order is:

```text
B1 portable vault/key/lease contracts
B2 reviewed cryptographic derivation + recovery/blob envelope primitives
B3 encrypted structured-store foundation
B4 platform SecretProtector adapters
B5 freshness/backup/rotation/deletion foundation
```

Specification 005 media streaming/journaling remains excluded.

## R3 implementation verification strategy

### Static/build/provenance

- pinned provider/toolchains;
- fmt/lint/build/tests on claimed targets;
- exact Cargo/native dependency/license/SBOM closure;
- generated provenance closure and no untracked drift.

### Positive

- fresh vault identity/VRK create;
- protector lock/unlock/restart;
- purpose-key derivation vectors;
- bounded blob encrypt/decrypt;
- encrypted SQLCipher open/integrity;
- recovery/backup restore;
- protector/recovery rotation;
- full VRK copy-verify-publish rotation.

### Adversarial

- OS CSPRNG failure leaves no new canonical state;
- wrong key/passphrase and tampered wrap/blob/database/manifest;
- cross-vault/generation recovery transplant;
- nonce collision injection, retry, duplicate canonical nonce detection;
- unknown/invalid envelope, KDF policy, generation, and freshness epoch;
- stale manifest/backup replay and freshness gaps;
- OS protector unavailable/invalidated/owner/policy mismatch;
- post-lock/post-revocation handle use;
- SQLCipher accidentally unencrypted handle;
- database/WAL/journal/temp semantic-marker scan;
- interrupted rotation/restore at every durable phase;
- backup/filesystem/secret-store metadata allowlist inspection;
- deletion residue in Himsat-managed plaintext caches/temp/logs;
- secret scanner over logs/test outputs/crash artifacts.

### Independent implementation review

Design approval does not approve code. Exact implementation revision requires another independent substantive crypto/security review before Specification 004 completion.

## Diffcipline scope control

This remediation is R3 but docs/governance only. It must not modify Cargo manifests, lockfile, product Rust/native code, registry adoption entries, or generated SBOM/notices. If the remediation exceeds current diff bounds, split it rather than weakening bounds.

Each future 004B leaf must be independently bounded. Do not increase limits merely to fit a large crypto change.

## Recovery plan

- review remediation finding remains valid: fix forward and preserve its evidence;
- review finds new design flaw: revise forward and re-review exact new canonical SHA;
- provenance/provider mismatch: reject that provider candidate rather than weaken the gate;
- implementation failure: preserve failed evidence and repair forward;
- rotation/migration failure: preserve last verified encrypted copy/key generation; never destroy the only known-good state during recovery testing.

## Closeout rule

Specification 004 becomes `CLOSED_CANONICAL` only after reviewed design, exact provider/provenance closure, bounded implementation leaves, R3 adversarial/platform evidence, exact implementation review, reconciliation, expected-head merges, post-merge verification, and durable closeout evidence all close. Only then may Specification 005 shaping begin.
