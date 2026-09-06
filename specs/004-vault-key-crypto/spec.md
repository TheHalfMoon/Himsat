# Specification 004 — Vault, Key, and Crypto Architecture

## Status

```text
LIFECYCLE = SHAPING
RISK = R3
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
INDEPENDENT_CRYPTO_SECURITY_REVIEW = REQUIRED
```

## Problem

Himsat will store recordings, transcripts, documents, derived memory, credentials, and other sensitive user-owned data locally. Local-first storage without a reviewed key hierarchy, encrypted structured/blob foundation, recovery model, and platform secure-storage boundary would create a false privacy claim and would block the crash-safe media store in Specification 005.

Specification 004 must establish the smallest reviewed cryptographic architecture that later storage/capture units can consume without inventing cryptography or binding the whole product to one operating system.

## Canonical dependency

Specification 004 begins only after Specification 003 closeout merge `1f14bbe004962dd164402e6e6c7f9c046cf5b489` and post-closeout CI `34047579266` succeed.

Specification 003 is therefore `CLOSED_CANONICAL` for this successor shaping candidate.

## Scope in

### 004A — reviewed cryptographic design packet

- explicit protected assets, adversaries, security goals, and non-goals;
- a portable vault-root/key-separation model;
- OS secret-protector boundaries for Apple, Android, Windows, and Linux;
- candidate authenticated-encryption and key-derivation constructions built only from established primitives/libraries;
- structured-store encryption requirements and SQLCipher candidate evaluation;
- opaque blob-envelope requirements;
- lock/unlock state and fail-closed error semantics;
- key generation, rotation, revocation, backup, recovery, and deletion semantics;
- versioning/migration/rollback rules;
- explicit metadata-leakage accounting;
- adversarial evidence requirements;
- an independent crypto/security review gate tied to an exact design revision.

### 004B — encrypted storage foundation

004B is **not authorized by shaping alone**. It becomes eligible only after:

1. 004A shaping is exact-head qualified and merged;
2. the exact canonical 004A design receives independent substantive crypto/security review;
3. all blocking review findings are resolved or explicitly dispositioned under repository governance;
4. any proposed third-party dependency/code adoption has exact provenance/license closure and explicit bounded adoption authority;
5. the implementation leaf is re-scoped against the reviewed design and remains within Diffcipline bounds.

When those gates are proven, 004B may implement the smallest reviewed foundation needed by Specification 005, not unrelated product behavior.

## Scope out

- custom cipher, MAC, password KDF, signature scheme, PRNG, or novel cryptographic primitive;
- crash-safe media journaling/chunk sequencing, owned by Specification 005;
- capture/background recording, owned by Specification 006+;
- sync, pairing, relay, connectors, plugins, agents, external writes, biometrics, or update signing;
- cloud key escrow or mandatory Himsat account;
- hidden plaintext fallback when an OS secret store is unavailable;
- claims of physical secure erase on SSD/flash;
- FIPS/compliance claims without separately proven configuration/evidence;
- donor/dependency adoption during shaping;
- production migration from an existing plaintext Himsat vault, because no such canonical product vault exists yet.

## Protected assets

At minimum:

- Vault Root Key (VRK) and derived purpose keys;
- OS-protected/wrapped VRK records;
- optional recovery envelope and recovery-KDF material;
- encrypted structured database and its WAL/journal/temp surfaces;
- encrypted blob content and authentication metadata;
- future media/object keys derived from or protected by the vault hierarchy;
- connector credentials once future units place them in the vault;
- deletion/rotation state that determines which keys remain valid.

## Threat model

### Adversaries and failures in scope

1. offline attacker who copies Himsat files from a locked/stolen device;
2. another local process/user with filesystem access but without the OS-protected secret;
3. corruption or malicious modification of database/blob/wrapped-key bytes;
4. rollback/replay of older encrypted manifests or key-generation metadata;
5. compromised backup/storage provider that sees stored ciphertext and allowed metadata;
6. brute-force attack against an opt-in recovery passphrase envelope;
7. OS secret store unavailable, locked, reset, revoked, or returning an unexpected error;
8. crash/power loss during rekey, rotation, restore, or deletion;
9. accidental plaintext spill to logs, crash reports, SQLite temp files, caches, or export staging;
10. implementation/configuration error that opens a database without encryption;
11. app compromise while the vault is unlocked and keys are resident in process memory.

### Security goals

- confidentiality at rest while the vault key is unavailable;
- authenticated integrity for Himsat-managed encrypted blob/wrapped-key envelopes;
- fail-closed behavior for wrong keys, corruption, unknown versions, or unavailable protectors;
- key separation between storage purposes;
- no mandatory remote trust or Himsat key escrow;
- portable recovery only when the user explicitly enables it;
- deterministic accounting of which metadata remains visible;
- safe key lifecycle and migration semantics;
- no secret material in normal logs or diagnostics.

### Explicit non-goals / residual risk

This unit cannot guarantee secrecy from a fully compromised process while the vault is unlocked, defeat arbitrary kernel/hardware compromise, guarantee physical erasure from flash media, hide all ciphertext sizes/timing/object counts, or recover a vault after all valid protector/recovery material is destroyed. Those limits must be stated rather than implied away.

## Candidate cryptographic architecture for review

The design packet proposes, but does not yet authorize implementation of:

```text
platform OS secret protector
       │
       └── protects/unlocks random 256-bit VRK
                                    │
                                    ├── reviewed domain-separated structured-store key
                                    ├── reviewed domain-separated blob/envelope key
                                    └── future reviewed purpose keys

optional recovery passphrase
       │
       └── Argon2id -> Recovery KEK -> authenticated VRK wrap
```

Candidate primitive choices requiring independent review:

- HKDF-SHA-256 for domain-separated purpose-key derivation;
- XChaCha20-Poly1305 for bounded opaque blob envelopes;
- Argon2id for recovery-passphrase KDF;
- SQLCipher for encrypted SQLite structured storage.

Exact provider/library choice, version, labels, salts, nonces, associated-data schema, KDF parameters, and database build settings are not implementation-authorized until review and provenance gates close.

## OS secret-protector contract

A portable `SecretProtector` boundary must model capabilities instead of pretending platforms are identical.

Required platform direction:

- Apple: Keychain baseline; Secure Enclave only where its actual key/operation model is applicable;
- Android: Android Keystore protector, hardware-backed/StrongBox capability detected rather than assumed;
- Windows: current-user DPAPI/CNG-class protection for the small vault-root record;
- Linux: Secret Service when available; no secret material in lookup attributes; no plaintext fallback if unavailable/locked.

The OS protector may either store a small VRK secret directly or protect/wrap it using a non-exportable platform key, depending on the platform's supported secure design. The portable layer depends on the behavior contract, not identical native mechanics.

## Recovery requirements

- recovery is opt-in and local/user-controlled;
- Himsat never sends the recovery passphrase or plaintext VRK to a Himsat service;
- recovery uses Argon2id under an independently reviewed parameter policy;
- recovery envelope stores only non-secret version/KDF parameters, random salt, and authenticated wrapped secret material;
- wrong passphrase and tampered envelope are indistinguishable from successful unlock only by typed safe error behavior, never partial plaintext output;
- if recovery is disabled and all device protectors are lost, permanent data loss is an expected and clearly communicated outcome.

## Structured-store requirements

SQLCipher is the leading candidate, not yet adopted.

If adopted, the implementation must:

- use reviewed random key material from the Himsat vault hierarchy;
- verify encryption is active after keying the handle and fail closed otherwise;
- use version-pinned cipher/provider/build settings;
- disable file-backed temporary data for sensitive operations;
- explicitly qualify WAL/rollback-journal behavior;
- run both normal database integrity and cipher/page-authentication integrity checks;
- handle wrong key/corrupt/unsupported-version cases without plaintext fallback;
- make rekey/migration crash recoverable and preserve a verified old copy until replacement is proven valid;
- never log the database key or recovery material.

## Blob-envelope requirements

A Himsat-owned encrypted blob format must be versioned and authenticated. It must bind ciphertext to non-secret context such as vault identity, object identity, object purpose, envelope version, and key generation through reviewed associated-data rules.

For bounded generic blobs, XChaCha20-Poly1305 is the leading candidate. Specification 005 must separately choose a reviewed streaming/chunk construction for long media rather than stretching a one-shot API beyond its intended use.

## Rotation and revocation

The design must distinguish:

- device-protector replacement around the same VRK;
- recovery-KEK/passphrase replacement;
- full VRK generation rotation requiring data migration;
- device revocation from future access.

No old key generation may be destroyed until all canonical data depending on it is proven migrated or intentionally retained.

## Backup and restore

Portable encrypted backup must include enough non-secret/versioned metadata and optional recovery-wrapped VRK material to restore without a Himsat service. Backup/restore acceptance requires authentication/integrity checks before declaring success and explicit accounting of provider-visible metadata.

## Deletion semantics

Application-level deletion must cover canonical rows, blobs, derived caches/indexes/temp files, wraps/protector records, and transient keys according to policy. Crypto-erasure may make remaining ciphertext inaccessible after all valid key wraps are destroyed, but Himsat must not represent that as guaranteed physical-media erasure.

## Required review gate

Before any 004B crypto/storage implementation:

- reviewer must be substantively independent from the authoring agent;
- review must identify the exact Git revision of the design packet;
- review scope must cover threat model, key hierarchy, candidate primitives/providers, OS secure-storage boundaries, recovery/KDF, associated data/nonce rules, SQLCipher configuration, rotation/rollback, backup, deletion, and implementation split;
- `APPROVE`, blocking findings, and residual risks must be recorded explicitly;
- absent/billing-blocked/skipped automated review is not independent review;
- author self-review is not independent review.

## Acceptance criteria

### Shaping acceptance

- exact canonical Spec003 closeout/post-CI evidence is recorded;
- `spec.md`, `plan.md`, `tasks.md`, and current research form one coherent bounded R3 design unit;
- no implementation/dependency/donor adoption enters the diff;
- threat model includes assets, adversaries, goals, and non-goals;
- algorithm/provider choices are labeled candidate pending independent review;
- 004B is explicitly blocked behind exact design review and provenance gates;
- exact-head SpecGrain, Diffcipline R3, provenance, existing Rust regression, and negative-control CI succeed.

### Specification 004 completion acceptance

Specification 004 is not complete until a later reviewed implementation proves:

- platform secret-protector behavior for the supported baseline targets;
- encrypted structured store and blob foundation;
- fail-closed wrong-key/corruption/version behavior;
- key rotation/recovery/backup/deletion semantics;
- plaintext-spill negative tests;
- exact dependency/provenance/SBOM closure;
- R3 adversarial evidence;
- independent substantive crypto/security review of the implemented exact revision;
- expected-head merge and post-merge verification.

## Successor rule

Specification 005 remains blocked until Specification 004 is `CLOSED_CANONICAL`. No Specification 005 media-journal implementation is authorized by this shaping work.
