# Specification 004A — Round 3 Normative Security Contracts

## Normative status

This document is a **normative amendment to Specification 004A** and MUST be read together with `spec.md`, `plan.md`, and `tasks.md` for the current design-review lineage. Where this document supplies an exact byte layout, parser rule, genesis transition, rotation invariant, or retained-manifest rule that `spec.md` previously left unspecified, this document is controlling for Specification 004A v1.

It does not authorize dependency adoption, provider selection, or 004B implementation. Any semantic change to these contracts invalidates earlier final-approval evidence and requires a new exact-SHA independent security review.

Independent review on PR #24 of canonical SHA `3c3847add4c8f56fe418fc02e62ae735e8239058`, comment `5561981955`, identified B019 and B020 as blocking. A later review on PR #25 initially returned `APPROVE` for `ea7ed384f3cadf68e6a3e82f4a3e4372c8727121`, but GitHub comparison proves the intervening changes affected only `specs/CURRENT.md` and `tasks.md`, not the security semantics that caused B019/B020. The contradictory approval therefore does not erase the earlier negative evidence. B019/B020 remain conservatively blocking unless explicitly reconciled against exact canonical text or remediated and re-reviewed.

```text
NEGATIVE_REVIEW_PR = 24
NEGATIVE_REVIEW_COMMENT = 5561981955
NEGATIVE_REVIEWED_SHA = 3c3847add4c8f56fe418fc02e62ae735e8239058
NEGATIVE_DISPOSITION = CHANGES_REQUIRED
BLOCKING_FINDINGS = B019,B020
CONFLICTING_APPROVAL_PR = 25
CONFLICTING_APPROVAL_REVIEWED_SHA = ea7ed384f3cadf68e6a3e82f4a3e4372c8727121
CONFLICT_RECONCILIATION_REQUEST = 5562030888
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
```

## Shared canonical encoding extension

The primitive encoding rules in `spec.md` remain normative. This amendment additionally defines:

```text
bytes16(x) = exactly 16 raw bytes
bytes48(x) = exactly 48 raw bytes at the recovery-wrap ciphertext/tag site
```

All integer arithmetic used to validate public envelope lengths MUST be checked for overflow before allocation, KDF execution, AEAD invocation, or plaintext release. Positional v1 envelopes have no optional or duplicate fields. A parser MUST reject an invalid domain label, unsupported identifier, impossible length relation, truncation, non-canonical value, or trailing byte before using attacker-controlled values to allocate unbounded memory or select cryptographic policy.

## B019 — canonical recovery envelope v1

### Fixed policy

The recovery envelope wraps exactly one 32-byte VRK using `ARGON2ID_RFC9106_64M_V1` and XChaCha20-Poly1305.

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

No field may request weaker, larger, auto-tuned, or implementation-selected parameters under v1. A future policy requires a new policy identifier and independent review.

### Exact public envelope bytes

The canonical recovery envelope is exactly:

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

Because `domain(s)` is `u16be(length) || ASCII(s)`, the complete v1 recovery envelope is exactly **167 bytes**. Any other total length is malformed.

The parser validates the exact domain, total length, fixed identifiers, non-zero generation, exact Argon2 parameters, and fixed ciphertext/tag length **before** allocating KDF memory or invoking Argon2id. Unsupported versions/policies fail without downgrade. Malformed, truncated, duplicate-by-representation, or trailing input never reaches KDF or AEAD authentication.

### Exact recovery AAD v1

The prior recovery AAD is extended and superseded by:

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

The salt is additionally bound through KDF derivation and the nonce is an AEAD input; including both in AAD freezes one canonical authenticated representation.

Wrong passphrase or authentication failure returns the same externally visible `RecoveryAuthenticationFailed` and releases no VRK. Structurally invalid version/policy/length fields and resource-limit failures may use the safe pre-authentication typed errors already allowed by `spec.md`.

### Recovery negative evidence required later

004B qualification MUST include fixtures for truncation, trailing data, every fixed identifier changed independently, zero/stale generation, salt/nonce/AAD tamper, ciphertext/tag corruption, declared-length mismatch, weaker/larger KDF parameters, vault/generation transplant, and wrong-passphrase versus AEAD-failure result uniformity.

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

The fixed public header before ciphertext/tag is **107 bytes**. `ciphertext_and_tag_length` MUST equal `plaintext_length_bytes + 16`, MUST be between **16** and **67,108,880** inclusive, and MUST be representable without overflow. The complete envelope therefore MUST be between **123 bytes** and **67,108,987 bytes** inclusive.

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

The prior bounded-blob AAD is extended and superseded by:

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

Every public field that selects cryptographic context or controls allocation is explicitly authenticated. The nonce remains a fresh OS-CSPRNG 24-byte value for every encryption attempt. No plaintext is released before full XChaCha20-Poly1305 authentication succeeds.

### Bounded-blob negative evidence required later

004B qualification MUST independently mutate every public field and test zero length, exact maximum, maximum-plus-one, checked-add overflow, declared-length mismatch, truncation, trailing bytes, unknown version/suite/purpose, nonce/ciphertext/tag tamper, vault/artifact/generation transplant, and duplicate/collision behavior.

## B020 — protected freshness-anchor genesis state

### Explicit protected slot state

Genesis MUST NOT infer freshness from a missing secret-store item. The local protector state for a newly provisioned vault contains an explicit protected freshness field with exactly one logical state:

```text
UNINITIALIZED
PRESENT(FreshnessAnchor)
```

`UNINITIALIZED` is an explicit value inside the protected protector-state record created for a new vault or explicit fresh-device restore. It is **not** equivalent to an absent/missing/reset item. Creation of the protected protector-state record itself MUST be old-or-new atomic; if the target secure store cannot prove complete-record creation semantics, the target returns `UnsupportedPolicy` before vault creation/restore is qualified.

A crash before the initial protected protector-state record exists leaves no canonical vault. Any unanchored candidate files from that failed creation attempt are non-canonical and MUST be discarded or quarantined before a new creation attempt. A missing/reset/lost protected state record for a vault that has local vault files or previously initialized state is `ItemMissing` or `Invalidated`; it MUST NOT be interpreted as `UNINITIALIZED` and MUST NOT silently bootstrap from rollbackable files.

Once successfully advanced to `PRESENT`, the protected state MUST NOT transition back to `UNINITIALIZED`. Additional device protectors for an already initialized local vault attach to the established protected freshness state and MUST NOT recreate `UNINITIALIZED`.

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

- it succeeds only when the complete protected protector-state record exists and its freshness field is exactly `UNINITIALIZED`;
- attempting it against `PRESENT` returns `AnchorAlreadyInitialized` and performs no mutation;
- the protected-record update MUST be crash-atomic: after power loss the freshness field is either exactly `UNINITIALIZED` or exactly `PRESENT(new_anchor)`, never torn/mixed;
- after reported success, an immediate protected reread MUST return exactly `PRESENT(new_anchor)` or return `AnchorGenesisFailed` and keep the vault closed;
- provider/storage failure returns `AnchorGenesisFailed` and MUST NOT be reported as success;
- a platform/provider that cannot prove serialized compare-and-set plus durable old-or-new semantics returns `UnsupportedPolicy`.

The existing `advance_freshness_anchor` operation remains valid only for `PRESENT -> PRESENT` epoch increments and can never perform genesis.

### New-vault genesis protocol

For a newly created vault:

1. create a new random `VaultId`, VRK, and protected protector-state record whose explicit freshness field is `UNINITIALIZED`;
2. create encrypted initial objects as applicable;
3. create canonical authenticated manifest epoch `1` with all-zero `previous_manifest_hash`;
4. write/fsync and reread/authenticate/canonically parse that exact epoch-1 manifest and verify every referenced object;
5. compute `manifest_hash = SHA-256(exact canonical epoch-1 manifest envelope bytes)`;
6. call `install_genesis_freshness_anchor(..., PRESENT(VaultId, 1, manifest_hash))`;
7. reread the protected state and require exact equality;
8. only then expose the vault as successfully created/unlocked canonical state.

A crash after step 1 but before step 6 leaves explicit protected `UNINITIALIZED` plus zero or more non-canonical candidate files. Recovery may resume genesis only after authenticating one exact epoch-1 candidate and proving its complete object set; multiple distinct valid epoch-1 candidates return `FreshnessGenesisConflict` and require explicit recovery. A crash during step 6 observes only the complete old or complete new protected state. A crash after step 6 rolls forward from the exact anchored manifest.

Normal open of existing local vault files with a missing protected state record is never a genesis path.

### Fresh-device restore genesis protocol

A genuinely fresh device with no prior Himsat protector state can authenticate a portable backup but cannot prove global newest-ness. After explicit user acceptance of that residual risk:

1. create a new local protected protector-state record with explicit freshness field `UNINITIALIZED`;
2. authenticate the recovery envelope and complete backup manifest/object set under the reviewed contracts;
3. write/fsync the accepted exact manifest/object set locally and reread/verify it;
4. install `PRESENT(VaultId, accepted_manifest.freshness_epoch, SHA-256(exact accepted manifest envelope bytes))` with the same genesis compare-and-set operation;
5. reread and require exact equality before opening the restored vault.

The accepted restore epoch MUST be non-zero and its manifest fully authenticated. Genesis does not claim that the accepted backup is globally newest. If the protected freshness state is already `PRESENT`, this protocol is forbidden; the existing restore-as-a-new-epoch-greater-than-current-anchor rule applies.

A crash before installation leaves the restore incomplete and closed; a crash during installation has old-or-new protected-state semantics; a crash after successful installation rolls forward from the accepted anchored manifest.

### Genesis negative evidence required later

Every claimed platform MUST prove first-create success only from explicit protected `UNINITIALIZED`, second-genesis rejection, missing-item-not-genesis behavior, crash before/during/after installation, reread mismatch fail-closed behavior, ambiguous epoch-1 candidate rejection, fresh-device newestness residual-risk recording, and `UnsupportedPolicy` without rollbackable-file fallback when atomicity cannot be proven.

## R001 — rotation-state invariants

The manifest rotation fields have these additional normative invariants:

- `rotation_phase = NONE` requires `rotation_target_generation = 0`;
- every non-`NONE` phase requires a non-zero `rotation_target_generation` present exactly once in the generation table;
- `PREPARE`, `STAGE`, and `VERIFY` require the source generation to remain the sole `ACTIVE` generation and the target generation to be `STAGED`;
- `PUBLISH`, `ANCHOR`, `ACTIVATE`, and `RETIRE` require the target generation to be the sole `ACTIVE` generation and the source generation to remain `RETAINED` until retirement is qualified;
- target generation is greater than source generation and cannot alias an earlier generation;
- normal transitions are only `NONE -> PREPARE -> STAGE -> VERIFY -> PUBLISH -> ANCHOR -> ACTIVATE -> RETIRE -> NONE`;
- repeating the same phase after crash recovery is permitted only after idempotent re-verification;
- `PREPARE`, `STAGE`, or `VERIFY` may abort to `NONE` only while the previously anchored source remains canonical and unanchored target material is removed/quarantined; `PUBLISH` and later phases never abort backward and must roll forward or fail closed;
- any manifest violating these invariants is `CorruptOrTampered` before it controls key retirement or object selection.

## R002 — retained manifest history set

For nonce-collision checking and crash recovery, the **retained manifest history set** means authenticated canonical/candidate manifest envelopes Himsat intentionally keeps locally for recovery.

Minimum retention:

1. retain the currently anchored manifest;
2. after publishing candidate `N+1`, retain anchored manifest `N` until anchor advancement succeeds and the new state completes at least one successful canonical reopen/integrity verification;
3. during interrupted publication, restore, or rotation, retain every manifest needed to resolve the current phase and both source/target generations until the operation reaches its stable retirement point;
4. during full VRK rotation, do not prune source-generation manifests needed to recover `G` until `RETIRE` completes;
5. older stable history may be pruned after these minimums, but pruning never relaxes the fresh OS-CSPRNG nonce requirement.

Before publishing a new manifest, compare its nonce against every locally retained envelope that used the same manifest-purpose key generation. A collision discards the candidate and generates a fresh nonce. Discovery of a duplicate among retained canonical history is `CorruptOrTampered`.

This retained-set scan is defense in depth, not a permanent global nonce ledger. Pruned history, detached backups, or copies outside Himsat control may be unavailable to the local scan; the 192-bit fresh OS-CSPRNG nonce rule remains the primary uniqueness mechanism.

## Round 3 acceptance

B019/B020 are not closed merely because this amendment exists. The remediation must:

1. be exact-head qualified by CI and Diffcipline R3;
2. merge with expected-head protection after exact live reconciliation;
3. pass post-merge CI and R3 on the exact canonical SHA;
4. receive a new substantive independent crypto/security review tied to that exact canonical SHA;
5. have no unresolved blocking design finding before 004P provider/dependency selection begins.
