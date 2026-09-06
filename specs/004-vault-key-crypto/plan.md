# Specification 004 Plan — Vault, Key, and Crypto Architecture

## Objective

Deliver a reviewed R3 cryptographic design first, then only after an independent exact-revision security review and provenance closure implement the smallest encrypted structured/blob foundation required by Specification 005.

## Why this unit is recursively split

The canonical master plan names one broad outcome: reviewed vault/key hierarchy plus encrypted structured/blob storage foundation. Treating that as one implementation leap would mix security design, third-party selection, platform secure storage, recovery policy, database encryption, blob format, and migration semantics in one high-blast-radius change.

Specification 004 therefore contains two dependency-ordered leaves:

```text
004A Reviewed cryptographic design
  -> independent crypto/security review gate
  -> dependency/provenance decision gate
  -> 004B Encrypted storage foundation implementation
  -> closeout
```

Shaping authorizes only 004A design/review preparation. It does not authorize 004B code.

## Canonical base

```text
SPEC_003_CLOSEOUT_MERGE = 1f14bbe004962dd164402e6e6c7f9c046cf5b489
SPEC_003_POST_CLOSEOUT_CI = 34047579266_SUCCESS
```

All implementation/review branches after shaping must start from the exact canonical merge current at that stage rather than from stale shaping heads.

## 004A — design and review preparation

### A1. Threat-model freeze

Define and reconcile:

- protected assets;
- attacker/failure classes;
- confidentiality/integrity/availability goals;
- explicit non-goals and residual risk;
- locked-vs-unlocked process boundary;
- metadata leakage;
- backup-provider compromise assumptions;
- deletion/physical-erasure limits.

Evidence: exact design text plus reviewer disposition on the exact revision.

### A2. Key hierarchy freeze

Review candidate:

```text
OS SecretProtector
  -> random 256-bit VRK
      -> reviewed structured-store purpose key
      -> reviewed blob/envelope purpose key
      -> future domain-separated keys

optional recovery passphrase
  -> Argon2id
  -> Recovery KEK
  -> authenticated VRK wrap
```

The reviewer must explicitly disposition:

- HKDF-SHA-256 suitability;
- exact domain-separation/context strategy;
- salts and versioning;
- key-generation identifiers;
- whether key wrapping should use the same selected AEAD provider or a platform/provider-specific reviewed primitive;
- transient-key zeroization expectations.

### A3. OS secret-protector contract

Specify behavior before native mechanics:

```text
create_protector / capability
protect_or_store_vrk
unlock_vrk
replace_protector
remove_protector
requires_user_presence
hardware_backed_state
```

Platform plans:

- Apple Keychain baseline; Secure Enclave only for supported key operations;
- Android Keystore with TEE/StrongBox capability surfaced, not assumed;
- Windows current-user DPAPI/CNG-class protection;
- Linux Secret Service with no plaintext fallback.

Failures must be typed: unavailable, locked, denied, item missing, invalidated, corrupt/tampered, unsupported capability.

### A4. Structured-store design

Leading candidate: SQLCipher, current research baseline 4.18.0 on 2026-09-06.

Before adoption the implementation plan must pin exact source/version/license/build strategy and review transitive/native provider implications.

Required runtime proof design:

- encryption-active check after keying;
- SQLCipher cipher-integrity check;
- normal SQLite integrity check;
- temp-store configuration proving no sensitive file-backed temp pages;
- intentional WAL/rollback-journal tests;
- wrong-key and malformed database failure;
- crash/recovery behavior for rekey/migration;
- no secret-bearing diagnostics.

### A5. Blob-envelope design

Leading bounded-blob candidate: XChaCha20-Poly1305.

Freeze only after review:

- provider implementation;
- key size;
- nonce generation and uniqueness strategy;
- associated-data schema;
- envelope magic/version/cipher-suite ID/key generation;
- maximum bounded blob size;
- streaming handoff boundary to Specification 005;
- corruption/unknown-suite behavior.

### A6. Recovery and backup design

Review and freeze:

- opt-in recovery lifecycle;
- Argon2id parameter/calibration policy against RFC 9106;
- recovery envelope format/version;
- restore authentication order;
- device-protector vs recovery portability;
- provider-visible metadata;
- loss semantics when recovery is disabled;
- backup inventory/digest consistency.

### A7. Rotation/revocation/deletion design

Define distinct state transitions for:

- protector rotation;
- recovery KEK/passphrase rotation;
- VRK generation rotation;
- device revocation;
- deletion of canonical/derived/temp/wrap surfaces.

Full VRK rotation must be modeled as a recoverable migration, not an in-place assumption.

## Independent review gate

004B is blocked until an independent reviewer provides substantive review tied to the exact canonical 004A design revision.

Valid review evidence must:

- identify reviewer and exact Git SHA;
- discuss the R3 security-sensitive design rather than merely formatting/style;
- cover the required review scope in `spec.md`;
- enumerate blocking findings, non-blocking recommendations, and residual risks;
- state a clear disposition such as APPROVE / CHANGES_REQUIRED.

Invalid substitutes:

- authoring-agent self-review;
- checklist completion without analysis;
- absent review;
- Qodo billing-blocked notification;
- CodeRabbit skipped/summary-only output;
- CI green status alone.

If no independent reviewer is available, record `BLOCKED_EXTERNAL_REVIEW`; do not implement cryptography to manufacture progress.

## Dependency/provenance gate

After design review, evaluate exact provider choices. For every adopted code/dependency/native library:

1. source repository/package and immutable revision/version;
2. exact controlling license;
3. adoption mode and destination/package identity;
4. transitive Cargo/native closure;
5. required notices;
6. source/build checksum evidence where applicable;
7. why the adoption is safer/smaller than alternative native implementation;
8. registry/SBOM updates before bytes enter the implementation candidate.

No candidate mentioned in research is automatically authorized.

## 004B — prospective implementation leaves

The exact split must be reconciled after review. Current intended order is:

### B1. Portable vault contracts

Potential crate: `himsat-vault`.

Bounded behavior:

- vault/key generation identifiers;
- lock state and typed errors;
- provider-neutral `SecretProtector` contract;
- reviewed key-derivation/envelope interfaces;
- no persistence adapter beyond deterministic test fixtures unless separately authorized.

### B2. Encrypted blob foundation

- reviewed bounded-blob envelope;
- atomic write/replace semantics appropriate to generic blobs;
- authentication before plaintext release;
- corruption/version/associated-data negative tests;
- no media journal/chunk sequencing.

### B3. Encrypted structured-store foundation

Potential crate: `himsat-db` or a bounded module selected after dependency review.

- SQLCipher integration if adopted;
- encrypted-state assertion;
- integrity checks;
- safe temp/WAL configuration;
- schema bootstrap for vault metadata only;
- wrong-key/corruption tests;
- no domain tables unrelated to the foundation.

### B4. Platform secret-protector adapters

Implement only the platform set justified by the current product baseline and repository structure. Platform-specific code remains native where required by the constitution. Unsupported platforms fail explicitly; they do not fall back to plaintext key files.

### B5. Recovery/rotation foundation

- opt-in recovery wrap/open path;
- versioned key-generation metadata;
- protector/recovery rotation;
- narrowly scoped VRK rotation orchestration sufficient to prove migration safety;
- no sync/device-pairing semantics.

## Verification strategy

Specification 004 is R3. Applicable implementation evidence includes:

### Static/build

- pinned toolchains/providers;
- fmt/lint/build on supported target matrix;
- dependency/license/SBOM closure;
- no untracked/generated drift.

### Positive

- fresh vault create/lock/unlock;
- structured DB encrypted and readable only after valid unlock;
- blob encrypt/decrypt round trip;
- backup/recovery round trip;
- protector and recovery rotation;
- reopen after process restart.

### Adversarial

- wrong key/passphrase;
- tampered key wrap/blob/database;
- unknown envelope/key generation;
- OS protector unavailable/invalidated;
- SQLCipher opened without encryption;
- WAL/journal/temp plaintext marker scan;
- interrupted rekey/rotation/restore;
- stale/rollback metadata;
- deletion residue in Himsat-managed plaintext caches/temp/logs;
- secret scanner over logs/test outputs/crash artifacts.

### Platform

- Apple/Android/Windows/Linux behavior only where adapter exists;
- capability detection for hardware-backed/user-presence support;
- no universal hardware-security claim.

### Independent review

A second substantive review of the exact implementation revision is required before Specification 004 completion. Design review does not automatically approve the later code.

## Diffcipline and scope control

### Shaping candidate

Risk profile: R3 because the documents establish security semantics.

Expected changed surfaces:

```text
.diffcipline.toml
docs/research/2026-09-06-vault-crypto-foundation.md
specs/004-vault-key-crypto/**
specs/CURRENT.md
```

No Cargo manifests, lockfile, source code, generated SBOM, or provenance registry should change during shaping.

### Implementation candidates

Each 004B leaf must be independently bounded after review. Do not expand `max_added_lines` merely to fit a large crypto implementation; split the leaf instead.

## Recovery plan

- shaping failure: forward-fix only the design/governance candidate;
- review finds design flaw: revise design forward, invalidate prior review for the changed security semantics, and obtain review on the new exact revision;
- dependency/provenance failure: do not adopt the candidate; select another reviewed provider or keep implementation blocked;
- implementation CI/security failure: preserve failed evidence, repair forward, re-run exact-head R3 qualification;
- rotation/migration failure: preserve last verified encrypted copy/key generation; never destroy the only known-good state during recovery testing.

## Closeout rule

Specification 004 can become `CLOSED_CANONICAL` only after:

1. reviewed design is canonical;
2. implementation leaves are complete within reviewed authority;
3. provenance/dependency closure is exact;
4. R3 positive/adversarial/platform evidence passes on exact implementation heads;
5. independent substantive review of exact implementation is complete;
6. reviews/threads/comments and residual risks are reconciled;
7. expected-head merge succeeds;
8. post-merge CI succeeds;
9. durable closeout evidence is canonical.

Only then may Specification 005 shaping begin.
