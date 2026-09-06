# Specification 004 — Vault, Key, and Crypto Architecture

## Status

```text
LIFECYCLE = DESIGN_REVIEW_REMEDIATION
RISK = R3
SHAPING_MERGE = 384608c8fc13531c399f3726caa5022eb3612aa2
INDEPENDENT_REVIEW_PR = 12
INDEPENDENT_REVIEW_FINDINGS = 16_ACTIONABLE
HIMSAT_REVIEW_DISPOSITION = CHANGES_REQUIRED
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
FINAL_INDEPENDENT_CRYPTO_SECURITY_REVIEW = REQUIRED_AFTER_REMEDIATION
```

## Problem

Himsat will store recordings, transcripts, documents, derived memory, credentials, and other sensitive user-owned data locally. Local-first storage without a reviewed key hierarchy, encrypted structured/blob foundation, recovery model, freshness model, and platform secure-storage boundary would create a false privacy claim and would block the crash-safe media store in Specification 005.

Specification 004 establishes the smallest reviewed cryptographic architecture that later storage/capture units can consume without inventing cryptographic primitives or pretending operating-system security boundaries are identical.

## Canonical dependency and review state

Specification 003 is `CLOSED_CANONICAL` at closeout merge `1f14bbe004962dd164402e6e6c7f9c046cf5b489` with post-closeout CI `34047579266` SUCCESS.

Specification 004 shaping merged at `384608c8fc13531c399f3726caa5022eb3612aa2` after exact-head CI `34048208674` and R3 run `34048208667` succeeded; post-merge CI `34048394581` and R3 run `34048394550` also succeeded.

Independent CodeRabbit review on review-only PR #12 examined exact canonical SHA `384608c8fc13531c399f3726caa5022eb3612aa2` and posted 16 actionable findings. The GitHub review state was `COMMENTED`; Himsat's governance disposition is `CHANGES_REQUIRED`. Exact review evidence and finding classification are in `review-evidence.md`.

## Scope in

### 004A — reviewed cryptographic design packet

- protected assets, adversaries, security goals, non-goals, and residual risks;
- portable vault identity, vault-root, key-generation, and key-separation model;
- fail-closed cryptographic randomness contract;
- OS secret-protector boundaries for Apple, Android, Windows, and Linux;
- normative recovery KDF/envelope policy;
- normative bounded-blob AEAD envelope, associated-data, nonce, and size policy;
- structured-store encryption requirements and exact-provider provenance gate;
- lock/unlock and key/handle lifecycle semantics;
- rollback/replay freshness anchor;
- crash-atomic key rotation;
- backup, restore, metadata-leakage, revocation, and deletion semantics;
- adversarial evidence requirements;
- independent crypto/security review tied to an exact canonical revision.

### 004B — encrypted storage foundation

004B remains **blocked** until all of the following are proven:

1. D001-D016 from the first independent design review are remediated canonically;
2. the remediated exact canonical 004A design passes CI and Diffcipline R3;
3. the remediated exact canonical design receives a new independent substantive crypto/security review;
4. all blocking findings from that re-review are resolved or explicitly dispositioned under repository governance;
5. every proposed third-party dependency/native library has exact provenance/license/package/checksum/build closure and bounded adoption authority;
6. the first implementation leaf is re-bounded against then-live canonical truth.

A design merge or green CI alone does not authorize cryptographic implementation.

## Scope out

- custom cipher, MAC, password KDF, signature scheme, PRNG, or novel cryptographic primitive;
- crash-safe media journaling/chunk sequencing, owned by Specification 005;
- capture/background recording, owned by Specification 006+;
- sync, pairing, relay, connectors, plugins, agents, external writes, biometrics, or update signing;
- cloud key escrow or mandatory Himsat account;
- hidden plaintext fallback when an OS secret store is unavailable;
- claims of physical secure erase on SSD/flash;
- claims that an unlocked fully compromised process can keep secrets from its attacker;
- FIPS/compliance claims without separately proven configuration/evidence;
- dependency adoption during this design-remediation unit;
- production migration from an existing plaintext Himsat vault, because no such canonical product vault exists yet.

## Protected assets

At minimum:

- `VaultId`, Vault Root Key (VRK), key generation, and derived purpose keys;
- OS-protected/wrapped VRK and freshness-anchor records;
- optional recovery envelope, recovery KDF salt/parameters, and Recovery KEK;
- encrypted structured database and its WAL/journal/temp surfaces;
- encrypted blob content, nonce, public format header, and authentication tag;
- authenticated vault manifest, inventory, and rotation state;
- future media/object keys derived from or protected by the vault hierarchy;
- deletion/rotation state that determines which key generations remain valid.

## Threat model

### Adversaries and failures in scope

1. offline attacker who copies Himsat files from a locked/stolen device;
2. another local process/user with filesystem access but without the OS-protected secret;
3. corruption or malicious modification of database/blob/wrapped-key/manifest bytes;
4. rollback/replay of older encrypted manifests, backups, or key-generation metadata;
5. compromised backup/storage provider that sees stored ciphertext and the explicitly allowed metadata surface;
6. brute-force attack against an opt-in recovery passphrase envelope;
7. OS secret store unavailable, locked, reset, revoked, invalidated, or returning an unexpected error;
8. crash/power loss during rekey, VRK rotation, restore, deletion, or manifest publication;
9. accidental plaintext spill to logs, crash reports, SQLite temp files, caches, or export staging;
10. implementation/configuration error that opens a database without encryption;
11. stale keyed handles used after lock/revocation/failure;
12. app compromise while the vault is unlocked and keys are resident in process memory.

### Security goals

- confidentiality at rest while a valid vault key is unavailable;
- authenticated integrity for Himsat-managed encrypted blob/recovery/manifest envelopes;
- fail-closed behavior for wrong keys, corruption, rollback, unknown versions, or unavailable protectors;
- explicit key separation between storage purposes and generations;
- no mandatory remote trust or Himsat key escrow;
- portable recovery only when the user explicitly enables it;
- exact accounting and qualification of metadata visible outside encrypted content;
- recoverable key rotation with no destruction of the only known-good decryptable state;
- no secret material in normal logs or diagnostics.

### Explicit non-goals / residual risk

This unit cannot guarantee secrecy from a fully compromised process while the vault is unlocked, defeat arbitrary kernel/hardware compromise, guarantee physical erasure from flash media, hide ciphertext sizes/counts/timing completely, remotely revoke already exported portable backups, or prove that a backup restored on a fresh device was globally the newest backup when no prior trusted freshness anchor exists. Those limits must be preserved as residual risk rather than implied away.

## Normative v1 identity, generation, and randomness contract

### Vault identity

`VaultId` is a non-secret immutable 128-bit value generated once from the approved cryptographic random source when the vault is created. It is encoded as exactly 16 bytes in cryptographic context. It is not derived from an account, device ID, filesystem path, or user data.

`key_generation` and `freshness_epoch` are unsigned 64-bit integers encoded big-endian in Himsat cryptographic context. Generation `1` is the first VRK generation; value `0` is invalid. Epoch `1` is the first authenticated manifest epoch; value `0` is invalid.

### Approved cryptographic random source

All v1 randomness must come from the target operating system's cryptographically secure RNG, either directly or through a separately reviewed provider that is proven to route to the OS source:

- Apple platforms: `SecRandomCopyBytes` or a reviewed equivalent backed by the OS CSPRNG;
- Windows: `BCryptGenRandom` using the system-preferred RNG or a reviewed equivalent;
- Linux and Android: `getrandom(2)` / the platform cryptographic random facility or a reviewed equivalent.

Normative lengths:

- VRK: exactly 32 random bytes;
- `VaultId`: exactly 16 random bytes;
- Argon2id recovery salt: exactly 16 random bytes;
- XChaCha20-Poly1305 nonce: exactly 24 random bytes per encryption attempt.

A randomness API failure aborts the operation before any new protector, envelope, database key state, manifest, or rotation state becomes canonical. No time-based seed, process/device identifier, ordinary pseudo-random generator, deterministic test RNG, UUID source of unknown cryptographic quality, zero-filled value, counter-only fallback, or best-effort downgrade is permitted in production.

Deterministic RNGs are permitted only inside explicitly marked test fixtures that cannot be selected by production code.

## Normative v1 key hierarchy and domain separation

Each VRK is a random 32-byte secret and belongs to one `VaultId` and one non-zero `key_generation`.

Candidate provider selection remains provenance-gated, but the v1 construction to be implemented after final review is:

- HKDF-SHA-256;
- HKDF salt = the 16 raw bytes of `VaultId`;
- output length = 32 bytes;
- structured-store `info` = ASCII bytes `HIMSAT/004/STRUCTURED/v1` followed by `key_generation` as `u64be`;
- bounded-blob `info` = ASCII bytes `HIMSAT/004/BLOB/v1` followed by `key_generation` as `u64be`.

Purpose keys are never reused across purposes. A future purpose requires a new unique `info` domain and review; it must not reuse either v1 label.

## OS secret-protector contract

### Portable contract

A `SecretProtector` reports its actual enforcement capabilities and supports behavior equivalent to:

```text
create_protector(requested_scope, user_presence_policy)
protect_or_store_vrk(vault_id, key_generation, vrk)
unlock_vrk(vault_id, key_generation)
read_freshness_anchor(vault_id)
advance_freshness_anchor(vault_id, expected_old, new_anchor)
replace_protector(vault_id)
remove_protector(vault_id)
actual_access_scope
requires_user_presence
hardware_backed_state
```

Access-scope classes are:

```text
APP_EXCLUSIVE
SAME_USER_ACCOUNT
SAME_USER_SESSION
```

User-presence policy is:

```text
NOT_REQUIRED
REQUIRED_EACH_HIMSAT_UNLOCK
```

The adapter must report the scope it can actually enforce. If the caller requests a stronger scope or user-presence policy than the platform adapter can prove, creation/unlock fails with `UnsupportedPolicy`; it must not silently weaken the policy.

Every protector record is bound to the Himsat application identity plus `VaultId` and key generation. A record whose owner/application metadata, vault identity, generation, or policy does not match the requested vault fails closed and never returns a VRK.

A process restart never restores a plaintext VRK from Himsat-managed disk. The new process must perform `unlock_vrk` again according to the protector policy. A system restart follows the native secure-store lifecycle but does not change this Himsat rule.

Revoking/removing a protector prevents future successful Himsat unlock through that protector. It cannot erase a VRK already copied from an unlocked process by a process compromise; that remains residual risk and may require full VRK rotation.

Typed protector failures include: `Unavailable`, `Locked`, `Denied`, `ItemMissing`, `Invalidated`, `OwnerMismatch`, `PolicyMismatch`, `CorruptOrTampered`, and `UnsupportedPolicy`.

### Platform-specific baseline

- **Apple:** Keychain is the baseline. The item is application/access-group scoped and may use Keychain access-control flags to require user presence when requested. Secure Enclave is used only for native operations it actually supports; Himsat does not claim arbitrary symmetric VRK bytes universally remain inside the enclave. Baseline reported scope is `APP_EXCLUSIVE` when the configured access group proves it.
- **Android:** Android Keystore uses an application-owned alias/non-exportable protector key. User authentication is required when the requested policy is `REQUIRED_EACH_HIMSAT_UNLOCK`; invalidation fails closed. TEE/StrongBox state is reported as capability evidence, not assumed. Baseline scope is `APP_EXCLUSIVE` when package/UID isolation is in force.
- **Windows:** current-user DPAPI/CNG-class protection is `SAME_USER_ACCOUNT`, not app-exclusive isolation from every process running as that user. Protector ciphertext must remain in Himsat-owned ACL-restricted storage. If `APP_EXCLUSIVE` or `REQUIRED_EACH_HIMSAT_UNLOCK` is requested and no separately reviewed Windows Hello/CNG mechanism proves it, return `UnsupportedPolicy`.
- **Linux:** Secret Service is treated as `SAME_USER_SESSION` unless the exact service proves stronger semantics. Lookup attributes contain only fixed application/service identifiers and an opaque random protector ID; they contain no `VaultId`, logical object ID, filename, key generation, content, or secret. If per-Himsat-unlock user presence or app-exclusive scope cannot be proven, return `UnsupportedPolicy`. There is no plaintext fallback.

## Normative v1 recovery policy

Recovery is opt-in, local/user-controlled, and independent of normal device unlock. Himsat never sends the recovery passphrase, Recovery KEK, or plaintext VRK to a Himsat service.

### Argon2id policy

V1 policy identifier: `ARGON2ID_RFC9106_64M_V1`.

Normative parameters:

```text
algorithm = Argon2id
version = 0x13
memory_kib = 65536
passes = 3
parallelism = 4
salt_bytes = 16
output_bytes = 32
passphrase_min_unicode_scalars = 16
passphrase_max_utf8_bytes = 1024
```

Passphrase bytes are UTF-8 exactly as entered. Himsat must not trim, case-fold, truncate, or silently normalize the passphrase. The UI may provide strength guidance but must not claim a minimum entropy solely from length.

The envelope parser validates the policy identifier and exact allowed parameters **before** allocating KDF resources. Parameters below the v1 minimum are rejected; attacker-supplied larger arbitrary parameters are not executed merely because they appear in an envelope. If the device cannot execute the v1 profile within available memory/resource policy, recovery fails with `ResourceLimit`; Himsat does not silently downgrade to weaker KDF parameters.

A future high-memory or calibrated profile requires a new versioned policy identifier, benchmark evidence, and independent review. V1 intentionally has no runtime auto-tuning that could weaken the stored policy.

### Recovery wrap

The 32-byte Argon2id output is the Recovery KEK. The reviewed v1 recovery-wrap candidate is XChaCha20-Poly1305 using a fresh 24-byte OS-CSPRNG nonce and the canonical recovery AAD below. The envelope stores only non-secret format/policy parameters, random salt, nonce, ciphertext/tag, and key-generation information required to select context.

Wrong passphrases and authentication/tag failures both return the same externally visible `RecoveryAuthenticationFailed` result, release no VRK or partial plaintext, and do not log which authentication path failed. Structurally invalid lengths, unsupported versions, invalid KDF policy, or resource-limit errors may return distinct safe typed errors before authentication because those fields are non-secret.

If recovery is disabled and all valid device protectors are lost, permanent data loss is expected and must be communicated clearly.

## Canonical v1 associated-data encoding

Himsat cryptographic AAD uses only the following primitive encodings:

- `domain(s)`: `u16be(byte_length(s)) || ASCII(s)`;
- `u16`: unsigned 16-bit big-endian;
- `u64`: unsigned 64-bit big-endian;
- `id128`: exactly 16 raw bytes.

No JSON, locale-dependent string, platform-native integer encoding, filesystem path encoding, or unspecified serialization is permitted in v1 AAD.

### Bounded blob AAD v1

```text
domain("HIMSAT/BLOB/AAD/v1")
u16(1)                         # AAD schema
id128(VaultId)
id128(ArtifactId)
u64(key_generation)
u16(1)                         # envelope version
u16(1)                         # cipher suite: XChaCha20-Poly1305
u16(1)                         # object purpose: GENERIC_ARTIFACT_BLOB
u64(plaintext_length_bytes)
```

`ArtifactId` is the 16-byte identity from Specification 003. Any non-artifact encrypted-object type requires a separately versioned purpose/AAD contract instead of overloading purpose code `1`.

### Recovery AAD v1

```text
domain("HIMSAT/RECOVERY/AAD/v1")
u16(1)                         # AAD schema
id128(VaultId)
u64(key_generation)
u16(1)                         # envelope type: VRK_WRAP
u16(1)                         # recovery policy: ARGON2ID_RFC9106_64M_V1
u16(1)                         # cipher suite: XChaCha20-Poly1305
```

A recovery envelope transplanted to another `VaultId` or key generation must fail authentication. This transplant-negative case is mandatory evidence.

### Structured-store context

Specification 004B does **not** add Himsat application-level per-row AEAD. The SQLCipher database key is the HKDF structured-store purpose key defined above; SQLCipher's reviewed page encryption/authentication supplies the database integrity construction. If a future design adds application-level encrypted records, it requires a separate versioned AAD contract and review.

## Structured-store requirements

SQLCipher remains the leading candidate but is not adopted by this remediation.

Before P002 may complete, the selected integration record must identify all of:

```text
upstream repository
immutable upstream revision/tag
exact SQLCipher core version
exact SQLite baseline
binding/Cargo package and exact version/revision
source and binding licenses
crypto provider and exact version/license
whether core is linked, built, or bundled
native compiler/toolchain inputs
security-relevant compile definitions/features
target platform matrix
Cargo/native transitive closure and checksums
```

The current research statement that upstream SQLCipher 4.18.0 exists is not an adoption identity and must never be treated as one.

If SQLCipher is later adopted, implementation must:

- use the reviewed random structured-store key derived from the Himsat hierarchy;
- prove encryption is active after keying the handle and fail closed otherwise;
- use version-pinned cipher/provider/build settings;
- disable file-backed temporary data for sensitive operations;
- intentionally qualify WAL and rollback-journal behavior;
- run normal SQLite integrity and SQLCipher cipher/page-authentication integrity checks;
- handle wrong key/corruption/unsupported versions without plaintext fallback;
- make rotation/migration copy-verify-publish recoverable rather than destroying the last verified encrypted copy;
- never log database keys or recovery material.

## Bounded blob envelope and nonce protocol

The v1 generic blob envelope is limited to a maximum plaintext length of **64 MiB (67,108,864 bytes)**. Larger or streaming media objects belong to a separately reviewed streaming design, beginning with Specification 005.

The v1 bounded-blob cipher suite is the reviewed candidate XChaCha20-Poly1305 with a 32-byte derived blob-purpose key and a 24-byte nonce.

Nonce rules:

1. generate a fresh 24-byte nonce from the approved OS CSPRNG for **every encryption attempt**;
2. store the nonce in the public envelope header;
3. uniqueness domain is `(VaultId, key_generation, blob-purpose key, nonce)` and nonce reuse with the same key is prohibited;
4. a retry after any ambiguous/failed publish generates a new nonce; it never reuses the abandoned attempt's nonce;
5. copying/restoring an existing authenticated envelope preserves its nonce and ciphertext exactly; re-encryption always generates a fresh nonce;
6. key-generation rotation creates a new purpose key but still follows the same fresh-nonce rule;
7. canonical publication reserves `(key_generation, nonce)` in the authenticated vault manifest/inventory; a collision detected before publication discards the candidate and generates a new nonce;
8. a duplicate nonce discovered in canonical inventory for the same generation is `CorruptOrTampered`; new writes stop until recovery reconciles the duplicate rather than choosing one silently.

Ciphertext is authenticated with the exact bounded-blob AAD v1 bytes above before plaintext is released.

## Freshness, rollback, and replay protection

Each vault has a `FreshnessAnchor` stored through the OS protector boundary and outside rollbackable Himsat vault files:

```text
VaultId
highest_epoch: u64
manifest_hash: 32 bytes SHA-256
```

Each authenticated encrypted vault manifest contains:

```text
VaultId
freshness_epoch
previous_manifest_hash
active_key_generation
key-generation state
opaque object inventory and authentication metadata
rotation state when active
```

Normal open rules:

- manifest epoch `< anchor.highest_epoch` => `RollbackDetected`, fail closed;
- equal epoch + hash mismatch => `CorruptOrTampered`, fail closed;
- equal epoch + matching hash => normal open;
- exactly `anchor.highest_epoch + 1` with `previous_manifest_hash == anchor.manifest_hash` is a recoverable interrupted-publication state: verify every referenced canonical object, then atomically advance the anchor;
- larger unexplained gaps => `FreshnessGap`, fail closed and require explicit recovery.

Normal commit ordering is: write/flush new encrypted data → write and fsync authenticated manifest `N+1` → verify the manifest/object set → atomically compare-and-advance the OS freshness anchor from `N` to `N+1` → retire superseded data only when no valid recovery path needs it.

An intentional restore of an older authenticated backup on a device with a higher anchor must never decrement the anchor. After explicit user-confirmed recovery, Himsat restores verified backup content into a **new epoch greater than the existing anchor** and republishes it as current state.

On a fresh device with no trusted prior anchor, Himsat can authenticate a backup but cannot prove that it was globally the newest backup. The first accepted authenticated restore establishes the local anchor; this limitation is residual risk and must be user-visible in restore evidence.

## Crash-atomic VRK rotation

Full VRK rotation is a copy-verify-publish migration. V1 freezes normal vault writes during root rotation to minimize recovery ambiguity.

Durable phases:

1. **PREPARE:** generate VRK generation `G+1`; create new protector/recovery wraps; persist authenticated rotation state while retaining all `G` material.
2. **STAGE:** produce separately named structured-store/blob copies under `G+1`; never mutate the only `G` canonical copy in place.
3. **VERIFY:** authenticate/integrity-check the complete staged database/blob inventory and verify that every required logical object is represented.
4. **PUBLISH:** write/fsync authenticated manifest epoch `E+1` that references only the verified `G+1` canonical set and links to the previous manifest hash.
5. **ANCHOR:** compare-and-advance the OS freshness anchor to `E+1`.
6. **ACTIVATE:** reopen the published state using `G+1`, rerun required integrity checks, then mark rotation stable.
7. **RETIRE:** only after successful activation remove old `G` protector/recovery wraps and encrypted copies according to retention policy.

A crash before `ANCHOR` recovers from the old anchor/manifest or verified interrupted-publication path; a crash after `ANCHOR` rolls forward from the new manifest while still retaining old material until activation completes. Old key material is never destroyed before the new canonical state has been published, anchored, reopened, and verified.

Power-loss/fault-injection evidence is required immediately before and after every phase commit point, including manifest publication, anchor advancement, activation, and old-generation destruction.

## Backup, provider-visible metadata, and restore

### Portable backup contents

A portable backup contains encrypted structured data, encrypted opaque blobs, an encrypted authenticated manifest/inventory, and—only when the user opts into recovery—a recovery-wrapped VRK plus the public KDF/envelope fields required to open it.

Provider-visible metadata is restricted to the following v1 allowlist:

- backup container/version identifier;
- opaque random backup-set identifier unrelated to `VaultId`;
- opaque random storage object names unrelated to logical object IDs or user filenames;
- ciphertext object count and ciphertext byte sizes;
- public cryptographic envelope fields required to parse/decrypt: envelope version, cipher-suite ID, key generation, nonce, recovery policy identifier, recovery salt, and Argon2id public parameters;
- provider-generated upload/modify timing and transport/account metadata outside Himsat's control.

The following must **not** be deliberately exposed in backup filenames/keys or unencrypted provider metadata:

- `VaultId`;
- `ArtifactId`, session/source/event IDs, or logical object IDs;
- user/source filenames or titles;
- object semantic kind beyond unavoidable public envelope format identifiers;
- plaintext session/document/transcript/note content;
- user-created timestamps or evidence ranges.

The encrypted manifest maps opaque backup object names to logical identities. Backup qualification must capture the provider-view surface and fail if a forbidden field/fixture marker appears outside encrypted payloads.

### Other visible metadata surfaces

- local filesystem: fixed vault database filenames, opaque blob filenames, ciphertext sizes/counts, file timestamps, and directory existence may be visible; logical IDs/titles/content are not permitted in filenames;
- locked SQLCipher store: database/WAL/journal existence, sizes, timestamps, and unavoidable public database-format bytes may be visible; semantic fixture markers must not be visible;
- Linux Secret Service lookup attributes: only fixed Himsat service/application identity and opaque protector ID; no vault/logical IDs, key generation, path, filename, or content.

Residual metadata risk includes ciphertext size distributions, object counts, operation timing, backup timing, and key-rotation cadence. Closeout evidence must record measured provider/filesystem/secret-store visibility rather than claim perfect metadata secrecy.

## Deletion and portable-backup limits

Application-level active-vault deletion must cover canonical rows, encrypted blobs, derived indexes/caches/temp files, local protector/recovery wraps, Himsat-managed backup staging files, and in-memory key material according to policy.

Destroying all active local wraps may make remaining local ciphertext cryptographically inaccessible, but this is not guaranteed physical-media erasure on SSD/mobile flash.

A detached/exported portable backup containing ciphertext and its own recovery-wrapped VRK can remain decryptable after the active local vault is deleted if the user still possesses the recovery passphrase. Deleting the active vault cannot remotely revoke, erase, or invalidate detached copies, provider snapshots, or user-made duplicates. Himsat must state that retention limit at export/deletion time. Future provider deletion/retention controls belong to the connector/sync capability that actually controls that provider.

## In-process key and handle lifecycle

An unlocked vault owns a revocable `VaultLease`/generation token. Every keyed database/blob handle is bound to the live lease and must check it before operations.

On lock, protector revocation, rotation transition requiring quiescence, authentication failure, or fatal integrity/freshness failure:

1. revoke the current lease first so existing handles reject new reads/writes with a typed locked/revoked error;
2. close keyed database/blob handles;
3. release VRK, Recovery KEK, and purpose-key objects;
4. zeroize owned secret buffers where the selected runtime/provider can provide a reviewed zeroization guarantee;
5. discard plaintext caches owned by the vault session.

Tests must attempt post-lock/post-revocation reads and writes through previously obtained handles and prove rejection. Himsat does not claim that language/runtime zeroization erases copies made by a compromised process, CPU registers, swap, crash/core dumps, allocator copies, or arbitrary OS/kernel memory; those are residual/platform-hardening concerns.

## Required independent review gate

Before any 004B implementation:

- reviewer must be substantively independent from the authoring agent;
- review must identify the exact canonical Git revision of the design packet;
- scope must cover threat model, key hierarchy/domain separation, randomness, candidate providers, OS secure-storage access boundaries, recovery/KDF, AAD/nonce rules, freshness/rollback, SQLCipher selection/configuration, rotation, backup, metadata leakage, deletion, key lifecycle, and implementation split;
- review evidence must enumerate blocking findings, non-blocking recommendations, and residual risks and record a clear disposition;
- absent/billing-blocked/skipped/neutral automated review is not independent review;
- author self-review and green CI are not substitutes;
- any security-semantic remediation invalidates older review as final approval evidence and requires re-review of the new exact canonical SHA.

## Acceptance criteria

### 004A remediation acceptance

- all D001-D016 findings are addressed in normative design/tasks/evidence;
- no implementation or dependency adoption enters the remediation diff;
- exact-head CI and Diffcipline R3 succeed;
- remediation is merged with expected-head protection and post-merge CI/R3 succeed;
- a new independent substantive review is obtained on that exact remediated canonical SHA;
- no unresolved blocking design finding remains before dependency selection begins.

### Specification 004 completion acceptance

Specification 004 is not complete until later implementation proves:

- platform secret-protector behavior for every claimed baseline target;
- encrypted structured-store and bounded-blob foundation;
- fail-closed wrong-key/corruption/version/rollback/protector behavior;
- nonce/AAD/transplant/metadata-leakage negative tests;
- crash-atomic key rotation/recovery/backup/deletion semantics;
- post-lock handle invalidation and secret/log leakage tests;
- exact dependency/license/SBOM/provenance closure;
- R3 adversarial/platform evidence;
- independent substantive crypto/security review of the exact implementation revision;
- expected-head merge and post-merge verification;
- durable closeout evidence with residual risks.

## Successor rule

Specification 005 remains blocked until Specification 004 is `CLOSED_CANONICAL`. No Specification 005 media-journal implementation is authorized by this design/remediation work.
