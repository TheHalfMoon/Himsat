# Specification 004A — Round 3 Normative Security Contracts

## Normative status

This document is a **normative amendment to Specification 004A** and MUST be read together with `spec.md`, `plan.md`, and `tasks.md` for the current design-review lineage.

It closes the exact omissions identified by independent review of canonical SHA `3c3847add4c8f56fe418fc02e62ae735e8239058`. Where this document supplies a byte layout, parser rule, genesis transition, rotation invariant, or retained-manifest rule that `spec.md` previously left unspecified, this document is controlling for Specification 004A v1.

It does not authorize dependency adoption, provider selection, or 004B implementation. Any semantic change to these contracts requires a new exact-SHA independent security review.

```text
REVIEWED_SHA = 3c3847add4c8f56fe418fc02e62ae735e8239058
REVIEW_PR = 24
REVIEW_COMMENT = 5561981955
REVIEW_DISPOSITION = CHANGES_REQUIRED
BLOCKING_FINDINGS = B019,B020
NON_BLOCKING_RECOMMENDATIONS_ADDRESSED = R001,R002
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
```

## Shared canonical encoding extension

The primitive encoding rules in `spec.md` remain normative. This amendment additionally defines:

```text
bytes16(x) = exactly 16 raw bytes
```

All integer arithmetic used to validate public envelope lengths MUST be checked for overflow before allocation, KDF execution, AEAD invocation, or plaintext release. Positional v1 envelopes have no optional or duplicate fields. A parser MUST reject an invalid domain label, unsupported identifier, impossible length relation, truncation, non-canonical value, or trailing byte before using attacker-controlled values to allocate unbounded memory or select a weaker cryptographic policy.

## B019 — canonical recovery envelope v1

### Fixed policy

The recovery envelope wraps exactly one 32-byte VRK using the already reviewed `ARGON2ID_RFC9106_64M_V1` policy and XChaCha20-Poly1305.

The only permitted v1 public policy values are:

```text
envelope_version = 1
envelope_type = 1                  # VRK_WRAP
cipher_suite = 1                   # XChaCha20-Poly1305
recovery_policy = 1                # ARGON2ID_RFC9106_64M_V1
argon2_version = 0x0013
memory_kib = 65536
passes = 3
parallelism = 4
kdf_output_bytes = 32
salt_bytes = 16
nonce_bytes = 24
plaintext_vrk_bytes = 32
ciphertext_and_tag_bytes = 48
```

No field may request weaker, larger, auto-tuned, or implementation-selected parameters under v1. A future policy requires a new policy identifier and review.

### Exact public envelope bytes

The canonical recovery envelope is exactly the following concatenation:

```text
domain("HIMSAT/RECOVERY/ENVELOPE/v1")
u16(1)                         # envelope_version
u16(1)                         # envelope_type = VRK_WRAP
u16(1)                         # cipher_suite = XChaCha20-Poly1305
u16(1)                         # recovery_policy = ARGON2ID_RFC9106_64M_V1
id128(VaultId)
u64(key_generation)              # non-zero
u16(0x0013)                    # Argon2 version
u32(65536)                     # memory_kib
u32(3)                         # passes
u16(4)                         # parallelism
u16(32)                        # KDF output bytes
bytes16(salt)
bytes24(nonce)
u32(48)                        # ciphertext_and_tag_length
bytes48(ciphertext_and_tag)    # 32-byte VRK ciphertext + 16-byte tag
```

`bytes48` means exactly 48 raw bytes at this sole use site. The complete v1 recovery envelope is therefore exactly **167 bytes**. Any other total length is malformed.

The parser validates the domain, exact total length, fixed identifiers, non-zero generation, exact Argon2 parameters, and fixed ciphertext/tag length **before** allocating KDF memory or invoking Argon2id. Unsupported versions/policies fail without downgrade. A malformed/truncated/trailing envelope never reaches KDF or AEAD authentication.

### Exact recovery AAD v1

The recovery AAD in `spec.md` is extended and superseded by the exact bytes below so every security-relevant public field is explicitly authenticated:

```text
domain("HIMSAT/RECOVERY/AAD/v1")
u16(1)                         # AAD schema
u16(1)                         # envelope_version
u16(1)                         # envelope_type
u16(1)                         # cipher_suite
u16(1)                         # recovery_policy
id128(VaultId)
u64(key_generation)
u16(0x0013)                    # Argon2 version
u32(65536)                     # memory_kib
u32(3)                         # passes
u16(4)                         # parallelism
u16(32)                        # KDF output bytes
bytes16(salt)
bytes24(nonce)
u32(48)                        # ciphertext_and_tag_length
```

The salt is also cryptographically bound through KDF key derivation and the nonce is an AEAD input; including both in AAD makes the canonical authentication contract explicit and prevents alternate public representations.

Wrong passphrase, changed salt/nonce/AAD, or tag/ciphertext authentication failure returns the same externally visible `RecoveryAuthenticationFailed` and releases no VRK. Structural/version/policy/resource-limit failures may retain the safe pre-authentication typed errors already allowed by `spec.md`.

### Recovery negative evidence required later

004B qualification MUST include fixtures for:

- one-byte truncation and one-byte trailing data;
- every fixed identifier changed independently;
- zero and stale key generation;
- changed salt, nonce, KDF parameter, length, `VaultId`, and generation;
- ciphertext/tag corruption;
- declared-length mismatch;
- attempts to encode weaker or larger KDF parameters;
- transplant to another vault/generation;
- wrong passphrase indistinguishable from AEAD authentication failure at the external result boundary.

## B019 — canonical bounded-blob envelope v1

### Exact public envelope bytes

The generic bounded-blob v1 envelope is exactly:

```text
domain("HIMSAT/BLOB/ENVELOPE/v1")
u16(1)                         # envelope_version
u16(1)                         # cipher_suite = XChaCha20-Poly1305
u16(1)                         # object_purpose = GENERIC_ARTIFACT_BLOB
id128(VaultId)
id128(ArtifactId)
u64(key_generation)              # non-zero
u64(plaintext_length_bytes)     # 0..67_108_864
bytes24(nonce)
u32(ciphertext_and_tag_length) # exactly plaintext_length_bytes + 16
bytes(ciphertext_and_tag)
```

The fixed public header before ciphertext/tag is **107 bytes**. `ciphertext_and_tag_length` MUST equal `plaintext_length_bytes + 16`, MUST be between 16 and **67,108,880** inclusive, and MUST be representable without overflow. The complete envelope therefore MUST be between **123 bytes** and **67,108,987 bytes** inclusive.

Before allocating ciphertext/plaintext storage or invoking AEAD, the parser MUST verify:

1. exact domain and envelope version;
2. exact suite and object-purpose identifiers;
3. non-zero generation;
4. plaintext length at or below 64 MiB;
5. checked equality `ciphertext_and_tag_length == plaintext_length_bytes + 16`;
6. total input length equals `107 + ciphertext_and_tag_length` exactly;
7. no truncation or trailing bytes.

There are no optional, duplicate, or unknown fields in v1. A future purpose, suite, or envelope version requires a separately reviewed versioned contract.

### Exact bounded-blob AAD v1

The bounded-blob AAD in `spec.md` is extended and superseded by:

```text
domain("HIMSAT/BLOB/AAD/v1")
u16(1)                         # AAD schema
u16(1)                         # envelope_version
u16(1)                         # cipher_suite
u16(1)                         # object_purpose
id128(VaultId)
id128(ArtifactId)
u64(key_generation)
u64(plaintext_length_bytes)
bytes24(nonce)
u32(ciphertext_and_tag_length)
```

Every public field that selects cryptographic context or controls allocation is therefore explicitly authenticated. The nonce remains generated fresh from the OS CSPRNG for every encryption attempt as required by `spec.md`.

No plaintext is released before full XChaCha20-Poly1305 authentication succeeds. A header/AAD/nonce/ciphertext/tag transplant or tamper fails authentication after safe structural parsing.

### Bounded-blob negative evidence required later

004B qualification MUST independently mutate every public field, test maximum and maximum-plus-one lengths, zero-length plaintext, checked-add overflow fixtures, declared-length mismatch, truncation, trailing bytes, unknown version/suite/purpose, nonce/ciphertext/tag tamper, vault/artifact/generation transplant, and duplicate/collision behavior.

## B020 — protected freshness-anchor genesis state

### Protected slot state machine

The OS-protected freshness mechanism is not modeled as “file missing means fresh.” Each locally initialized vault has a protected anchor slot with exactly one of these logical states:

```text
UNINITIALIZED
PRESENT(FreshnessAnchor)
```

`UNINITIALIZED` is created only as part of explicit new-vault or explicit fresh-device-restore protector setup. Once successfully advanced to `PRESENT`, it MUST NOT transition back to `UNINITIALIZED`.

A missing/reset/lost protected item for a vault that has local vault files or previously initialized protector state is `ItemMissing` or `Invalidated`; it MUST NOT be interpreted as a fresh device and MUST NOT silently bootstrap from rollbackable files.

Additional device protectors for an already initialized local vault attach to the existing protected freshness state; they MUST NOT recreate an `UNINITIALIZED` slot.

### Genesis operation

The portable protector contract is extended with:

```text
install_genesis_freshness_anchor(
    vault_id,
    expected_state = UNINITIALIZED,
    new_anchor
)
```

It has compare-and-set semantics:

- it succeeds only when the protected slot is exactly `UNINITIALIZED`;
- attempting it against `PRESENT` returns `AnchorAlreadyInitialized` and performs no mutation;
- the transition MUST be crash-atomic: after power loss the protected slot is either exactly `UNINITIALIZED` or exactly `PRESENT(new_anchor)`, never torn/mixed;
- after reported success, an immediate protected reread MUST return exactly `PRESENT(new_anchor)` or return `AnchorGenesisFailed` and keep the vault closed;
- provider/storage failure returns `AnchorGenesisFailed` and MUST NOT be reported as success;
- a platform/provider that cannot prove serialized compare-and-set plus old-or-new durable semantics returns `UnsupportedPolicy` before freshness-protected vault creation/restore is qualified.

The existing `advance_freshness_anchor` operation remains valid only for `PRESENT -> PRESENT` epoch increments and can never perform genesis.

### New-vault genesis protocol

For a newly created vault:

1. create the VRK and local protector plus a protected `UNINITIALIZED` anchor slot;
2. create encrypted initial objects as applicable;
3. create an authenticated canonical manifest at `freshness_epoch = 1` with all-zero `previous_manifest_hash`;
4. write/fsync and reread/authenticate/canonically parse that exact epoch-1 manifest and verify every referenced object;
5. compute `manifest_hash = SHA-256(exact canonical epoch-1 manifest envelope bytes)`;
6. call `install_genesis_freshness_anchor(..., PRESENT(VaultId, 1, manifest_hash))`;
7. reread the protected slot and require exact equality;
8. only then expose the vault as successfully created/unlocked canonical state.

A crash before step 6 leaves `UNINITIALIZED` plus zero or more non-canonical candidate files. On recovery, Himsat may resume genesis only after authenticating one exact epoch-1 candidate and proving its complete object set; ambiguous multiple valid candidates return `FreshnessGenesisConflict` and require explicit recovery. A crash during step 6 observes only the old `UNINITIALIZED` state or the complete new anchor. A crash after step 6 rolls forward from the exact anchored manifest.

Normal open of existing local vault files with no matching protected slot is never a genesis path.

### Fresh-device restore genesis protocol

A fresh device with no prior trusted Himsat freshness state can authenticate a portable backup but cannot prove global newest-ness. After explicit user acceptance of that residual risk:

1. establish a new local protector with protected slot `UNINITIALIZED`;
2. authenticate the recovery envelope and complete backup manifest/object set under the reviewed contracts;
3. write/fsync the accepted exact manifest/object set locally and reread/verify it;
4. install `PRESENT(VaultId, accepted_manifest.freshness_epoch, SHA-256(exact accepted manifest envelope bytes))` using the same genesis compare-and-set operation;
5. reread and require exact equality before opening the restored vault.

The accepted restore epoch MUST be non-zero and its manifest must be fully authenticated. Genesis does not claim that the accepted backup is globally newest. If a protected slot is already `PRESENT`, this fresh-device protocol is forbidden; the existing “restore as a new epoch greater than the current anchor” rule applies instead.

A crash before anchor installation leaves the restore incomplete and closed; a crash during installation has old-or-new protected-slot semantics; a crash after successful installation rolls forward from the accepted anchored manifest.

### Genesis negative evidence required later

Every claimed platform MUST prove:

- first create succeeds only from protected `UNINITIALIZED`;
- second genesis attempt returns `AnchorAlreadyInitialized` with no mutation;
- missing protected item for existing local files does not become genesis;
- crash before/during/after genesis installation yields only allowed states;
- reread mismatch fails closed;
- ambiguous epoch-1 candidates fail closed;
- fresh-device restore records the “cannot prove globally newest” residual risk;
- unsupported atomicity returns `UnsupportedPolicy` without fallback to a rollbackable file.

## R001 — rotation-state invariants

The manifest rotation fields have these additional normative invariants:

- `rotation_phase = NONE` requires `rotation_target_generation = 0`;
- every non-`NONE` phase requires a non-zero `rotation_target_generation` present exactly once in the generation table;
- `PREPARE`, `STAGE`, and `VERIFY` require the source generation to remain the sole `ACTIVE` generation and the target generation to be `STAGED`;
- `PUBLISH`, `ANCHOR`, `ACTIVATE`, and `RETIRE` require the target generation to be the sole `ACTIVE` generation and the source generation to remain `RETAINED` until retirement is qualified;
- the target generation must be greater than the source generation and must not alias any earlier generation;
- normal forward transitions are only `NONE -> PREPARE -> STAGE -> VERIFY -> PUBLISH -> ANCHOR -> ACTIVATE -> RETIRE -> NONE`;
- repeating the same phase after crash recovery is permitted only when the phase operation is idempotently reverified;
- `PREPARE`, `STAGE`, or `VERIFY` may abort back to `NONE` only while the previously anchored source generation remains canonical and after all unanchored/staged target material is removed or quarantined; `PUBLISH` and later phases never abort backward and must roll forward or fail closed;
- a manifest violating any phase/generation invariant is `CorruptOrTampered` before its state controls key retirement or object selection.

## R002 — retained manifest history set

For nonce-collision checking and crash recovery, the **retained manifest history set** means authenticated canonical/candidate manifest envelopes that Himsat intentionally keeps locally for recovery.

Minimum retention rules:

1. retain the currently anchored manifest;
2. after publishing a candidate `N+1`, retain the previously anchored manifest `N` until anchor advancement succeeds and the new state has completed at least one successful canonical reopen/integrity verification;
3. during interrupted publication, restore, or rotation, retain every manifest needed to resolve the current phase and both source/target generations until that operation reaches its stable retirement point;
4. during full VRK rotation, do not prune source-generation manifests needed to recover `G` until `RETIRE` completes;
5. pruning older stable history after these minimums is permitted, but it never relaxes the primary fresh-OS-CSPRNG nonce requirement.

Before publishing a new manifest, compare its nonce against every envelope in the locally retained history set that used the same manifest-purpose key generation. A collision discards the candidate and generates a fresh nonce. Discovery of a duplicate among retained canonical history is `CorruptOrTampered`.

This retained-set check is defense in depth, not a claim of a permanent global nonce ledger. Pruned historical envelopes, detached backups, or copies outside Himsat control may no longer be available to the local collision scan; the 192-bit OS-CSPRNG nonce rule remains the primary uniqueness mechanism.

## Round 3 acceptance

B019 and B020 are not closed merely because this amendment exists. The remediation must:

1. be exact-head qualified by CI and Diffcipline R3;
2. merge with expected-head protection after reconciliation;
3. pass post-merge CI and R3 on the exact canonical SHA;
4. receive a new substantive independent crypto/security review tied to that exact canonical SHA;
5. have no unresolved blocking design finding before 004P provider/dependency selection begins.
