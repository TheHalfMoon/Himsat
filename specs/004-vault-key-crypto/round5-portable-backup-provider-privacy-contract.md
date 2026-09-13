# Specification 004A — Round 5 Portable Backup Provider-Privacy Contract

## Normative status

This document is a **normative security-semantic amendment to Specification 004A v1** and MUST be read with `spec.md`, `round3-normative-contracts.md`, and `round4-blob-inventory-contract.md`.

It exists because direct provider upload of the already-canonical B202, B204, and B501 envelopes would expose identifiers that the D013 portable-backup provider-view contract forbids. This amendment preserves those inner formats byte-for-byte and adds only the portable-backup privacy boundary needed for B505.

This document does **not** authorize B505 implementation merely by existing. The exact canonical revision containing this amendment MUST complete docs/design qualification, expected-head guarded merge, exact post-merge qualification, and reconciliation of every known blocking finding before B505 implementation authority reopens. Independent human review may be requested as additional assurance but is not a mandatory authority gate.

```text
DISCOVERY_BASE_SHA = 1f6cf11a0df427521adfd1d728c78483e09285d1
B504_B505_RECONCILIATION = CANONICAL_QUALIFIED
B505_IMPLEMENTATION_AUTHORITY = NONE_PENDING_CANONICAL_GOVERNANCE_AMENDMENT_QUALIFICATION
NEW_PRODUCT_CODE = NONE
NEW_DEPENDENCY_ADOPTION = NONE
NEW_CRYPTO_PRIMITIVE = NONE
```

## Conflict being remediated

D013 forbids deliberate provider exposure of `VaultId`, `ArtifactId`, logical object IDs, user/source names or filenames, semantic titles/kinds beyond unavoidable public format identifiers, plaintext markers/content, and user timestamps/evidence ranges.

The existing canonical inner formats are correct for their original local authenticated-storage boundaries but are not safe to upload directly as provider objects:

- B202 public bytes contain `VaultId`, `ArtifactId`, and `key_generation`;
- B204 public bytes contain `VaultId` and `key_generation`;
- B501 public envelope bytes contain `VaultId`, `key_generation`, and `freshness_epoch`.
- Round 4 requires every backed-up B202 object to preserve the exact complete canonical inner envelope bytes for `ciphertext_length` and `ciphertext_sha256`; those bytes therefore cannot be stripped or rewritten to satisfy provider privacy.

Direct upload is non-conforming. Provider privacy MUST be achieved outside the canonical inner formats.

## Provider-visible v1 allowlist after this amendment

Himsat-controlled provider-visible material is limited to:

- the portable-backup set format/version identifier;
- a fresh random `BackupSetId` unrelated to `VaultId` or user identity;
- fresh random `BackupObjectId` values unrelated to logical IDs, filenames, or semantic roles;
- ciphertext/object counts and byte sizes;
- one non-zero backup `key_generation`, which remains an allowed public cryptographic selector and may reveal rotation cadence;
- XChaCha20-Poly1305 nonces and ciphertext lengths required by the backup privacy format;
- the fixed recovery-bootstrap Argon2id policy identifier, salt, parameters, nonce, and fixed-size bootstrap slot;
- provider-generated timing, transport, account, retention, and similar metadata outside Himsat control.

Himsat MUST NOT deliberately expose `VaultId`, `ArtifactId`, other logical IDs, logical storage IDs, `freshness_epoch`, user/source names, user filenames, titles, semantic object kinds, plaintext markers/content, user timestamps, or evidence ranges in provider keys, filenames, unencrypted descriptor fields, or unencrypted object bytes.

Residual leakage remains explicit: ciphertext sizes, counts, upload/operation timing, backup timing, provider account/transport metadata, and rotation cadence where inferable. B505 MUST NOT claim these are hidden.

## Opaque identifiers and provider keys

`BackupSetId` and every data-object `BackupObjectId` are independently generated as exactly 16 bytes from the approved OS CSPRNG. Generation failure aborts before any provider-visible candidate becomes canonical. All-zero values are rejected and regenerated. Duplicate object identifiers inside one backup set are rejected and regenerated before publication.

The canonical provider key encoding is lowercase hexadecimal of the raw opaque identifier bytes. A provider hierarchy, where available, is:

The all-zero 16-byte object identifier is reserved exclusively for the set-descriptor provider-key leaf and MUST NOT be generated or accepted as a data-object `BackupObjectId`. This fixed reserved leaf permits the same canonical hierarchy to work on filesystem-style providers without requiring one path to be both a file and a directory.

On read, the set-descriptor provider key MUST decode to the exact embedded `BackupSetId` plus the reserved all-zero leaf. Every data-object provider key MUST decode to the exact embedded `(BackupSetId, BackupObjectId)`, and the data-object ID MUST be non-zero. Mismatch is `CorruptOrTampered` before payload release.

```text
set descriptor key = hex_lower(BackupSetId) / 00000000000000000000000000000000
data object key     = hex_lower(BackupSetId) / hex_lower(BackupObjectId)
```

No semantic prefix, suffix, extension, logical identifier, user filename, title, or object-kind token may be added by Himsat. A future connector that cannot preserve this property requires its own review before it may claim D013 compliance.

## Backup privacy keys

Round 5 adds distinct reviewed HKDF domains so backup index and backup object encryption never share a key:

```text
index_key = HKDF-SHA-256(VRK, salt = raw VaultId,
    info = ASCII("HIMSAT/004/BACKUP-INDEX/v1") || u64be(key_generation) || raw BackupSetId, L = 32)
object_key = HKDF-SHA-256(VRK, salt = raw VaultId,
    info = ASCII("HIMSAT/004/BACKUP-OBJECT/v1") || u64be(key_generation) || raw BackupSetId, L = 32)
```

The labels are unique and MUST NOT reuse `StructuredStore`, `BoundedBlob`, or `FreshnessManifest` key material. Index and object encryption use different derived keys, and the raw `BackupSetId` in each HKDF info string makes both keys backup-set-specific even when the same vault generation is exported more than once. Cross-set nonce equality therefore cannot create same-key nonce reuse, while fresh OS-CSPRNG nonces remain mandatory for every set. These purposes are design-only until the exact Round 5 canonical revision receives the required independent security approval. Product code MUST NOT silently add them before that gate.

The only cryptographic primitive families used by this amendment are already-reviewed families: HKDF-SHA-256, XChaCha20-Poly1305, Argon2id, SHA-256, and the OS CSPRNG.

## Set descriptor v1

The provider-visible set descriptor is exactly:

```text
domain("HIMSAT/BACKUP/SET/ENVELOPE/v1")
u16(1)                         # set format version
opaque16(BackupSetId)
u64(key_generation)              # non-zero
u32(data_object_count)          # 2..=1_048_576
u16(1)                         # bootstrap cipher: XChaCha20-Poly1305
u16(1)                         # recovery policy: ARGON2ID_RFC9106_64M_V1
u16(0x0013)                    # Argon2 version
u32(65536)                     # memory_kib
u32(3)                         # passes
u16(4)                         # parallelism
u16(32)                        # KDF output bytes
bytes16(bootstrap_salt)
bytes24(bootstrap_nonce)
u32(183)                       # bootstrap_slot_length
bytes183(bootstrap_slot)
bytes24(index_nonce)
u32(index_ciphertext_and_tag_length)
bytes(index_ciphertext_and_tag)
```

## Recovery bootstrap v1

B505 v1 is a genuinely portable fresh-device restore format only when the user has explicitly enabled the canonical B204 recovery envelope. Attempting to create a B505 portable backup without a current authenticated recovery envelope fails with `RecoveryRequired`; Himsat MUST NOT silently create escrow, weaken recovery policy, or publish a device-bound artifact while calling it portable.

The set descriptor `bootstrap_salt` MUST equal the salt encoded in the exact inner B204 recovery envelope. The passphrase is processed once under the fixed B204 Argon2id policy to obtain the canonical 32-byte Recovery KEK. A distinct bootstrap key is then derived:

```text
bootstrap_key = HKDF-SHA-256(
    IKM = RecoveryKEK,
    salt = raw 16-byte BackupSetId,
    info = ASCII("HIMSAT/004/BACKUP-BOOTSTRAP/v1") || u64be(key_generation),
    L = 32
)
```

The bootstrap plaintext is exactly one complete 167-byte canonical B204 recovery envelope. The bootstrap XChaCha20-Poly1305 ciphertext/tag is therefore exactly 183 bytes. Its nonce is a fresh 24-byte OS-CSPRNG value for every encryption attempt.

The bootstrap AAD is exactly:

```text
domain("HIMSAT/BACKUP/BOOTSTRAP/AAD/v1")
u16(1)                         # set format version
opaque16(BackupSetId)
u64(key_generation)
u16(1)                         # XChaCha20-Poly1305
u16(1)                         # ARGON2ID_RFC9106_64M_V1
u16(0x0013)
u32(65536)
u32(3)
u16(4)
u16(32)
bytes16(bootstrap_salt)
bytes24(bootstrap_nonce)
u32(183)
```

After bootstrap authentication succeeds, the decrypted 167 bytes MUST parse as exactly one canonical B204 recovery envelope. Its `key_generation`, recovery salt, fixed KDF policy, and suite MUST equal the corresponding set-descriptor values. Any mismatch is `CorruptOrTampered` and releases no VRK.

The already-derived Recovery KEK may be reused only as the exact B204 Recovery KEK object to authenticate/decrypt that inner recovery envelope; implementations MAY avoid a second Argon2id execution but MUST preserve B204's uniform `RecoveryAuthenticationFailed` behavior and secret lifetime/zeroization rules.

A bootstrap from another set, generation, salt, policy, or nonce context MUST fail authentication. The bootstrap key and Recovery KEK MUST NOT encrypt backup data objects or the encrypted index.

Wrong passphrase, outer-bootstrap authentication failure, and inner B204 authentication failure MUST all surface the same `RecoveryAuthenticationFailed` result and release no VRK. Public structural errors that can be established before passphrase authentication may return `CorruptOrTampered`; implementations MUST NOT create a passphrase-validity oracle by distinguishing secret-authentication failures.

## Opaque backup object envelope v1

Every provider data object, including SQLCipher bytes, B501 manifest bytes, and B202 bounded-blob bytes, is carried only inside this outer provider-privacy envelope. B505 does not rely on SQLCipher's public-header posture or on B306 marker absence as a provider-privacy boundary.

Large payloads are split into independently authenticated chunks. `BACKUP_CHUNK_PLAINTEXT_MAX = 33_554_432` bytes (32 MiB). Every non-final chunk MUST have exactly that plaintext length; a final chunk MUST contain `1..=33_554_432` bytes. Zero-length chunks are invalid.

The canonical provider object envelope is exactly:

```text
domain("HIMSAT/BACKUP/OBJECT/ENVELOPE/v1")
u16(1)                         # object envelope version
opaque16(BackupSetId)
opaque16(BackupObjectId)
u64(key_generation)              # non-zero
u32(plaintext_chunk_length)     # 1..=33_554_432
bytes24(nonce)
u32(ciphertext_and_tag_length) # plaintext_chunk_length + 16
bytes(ciphertext_and_tag)
```

Before allocation or AEAD invocation the parser MUST validate the exact domain/version, non-zero generation, chunk bound, checked `+16`, exact total input length, and no trailing bytes.

The object AAD is exactly:

```text
domain("HIMSAT/BACKUP/OBJECT/AAD/v1")
u16(1)
opaque16(BackupSetId)
opaque16(BackupObjectId)
u64(key_generation)
u32(plaintext_chunk_length)
bytes24(nonce)
u32(ciphertext_and_tag_length)
```

The object key is the Round 5 `object_key`. Every object encryption attempt uses a fresh OS-CSPRNG nonce. Retry after any ambiguous or failed publication generates a new nonce and a new candidate ciphertext. Within one set, duplicate `BackupObjectId` values are rejected before encryption and duplicate object nonces are rejected before publication.

Discovery of a repeated object nonce in the same retained backup set under the same generation is `CorruptOrTampered`. Detached/pruned sets may be unavailable for scanning; the fresh 192-bit OS-CSPRNG nonce rule remains the primary uniqueness mechanism.

Provider keys and the object envelope reveal no payload role. Manifest, structured-store, and bounded-blob chunks use the same outer format and differ only inside authenticated ciphertext and the encrypted index.

## Encrypted backup index v1

The index plaintext is encrypted with the distinct Round 5 `index_key`. Its XChaCha20-Poly1305 nonce is fresh per attempt and MUST NOT be reused with the same index key. The index plaintext maximum is 64 MiB. `payload_count`, per-payload `chunk_count`, and their checked global sum are validated before allocation. V1 permits at most 262,145 payloads (the complete B501 inventory maximum of 262,144 objects plus the manifest payload) and at most 1,048,576 provider data objects/chunks. With the fixed 82-byte payload record and 24-byte chunk record defined below, even the simultaneous maxima consume 46,661,714 record bytes before the small fixed index header, remaining below the 64 MiB plaintext bound.

The canonical plaintext begins:

```text
domain("HIMSAT/BACKUP/INDEX/PLAINTEXT/v1")
u16(1)                         # index schema
id128(VaultId)
u64(key_generation)
u64(source_freshness_epoch)
bytes32(source_manifest_hash)   # exact canonical B501 envelope hash
u32(payload_count)              # 2..=262145
```

Each payload record then encodes exactly:

```text
u32(payload_ordinal)            # contiguous 0..payload_count-1
u16(payload_kind)               # 1=MANIFEST, 2=STRUCTURED_STORE, 3=GENERIC_ARTIFACT_BLOB
id128(logical_id)               # zero for kinds 1/2; ArtifactId for kind 3
opaque16(source_storage_id)      # zero for MANIFEST; exact B501 storage_id for kinds 2/3
u64(exact_payload_length)
bytes32(exact_payload_sha256)
u32(chunk_count)                # 1..=1_048_576
repeat chunk_count times:
    u32(chunk_ordinal)          # contiguous 0..chunk_count-1
    opaque16(BackupObjectId)
    u32(plaintext_chunk_length)
```

The complete index MUST contain exactly one `MANIFEST` payload at ordinal 0 and exactly one `STRUCTURED_STORE` payload at ordinal 1. Remaining records are `GENERIC_ARTIFACT_BLOB` records sorted lexicographically by `logical_id`. Blob logical IDs are unique. All `BackupObjectId` values are unique across the set. The `MANIFEST` record MUST use all-zero `source_storage_id`; every `STRUCTURED_STORE` and `GENERIC_ARTIFACT_BLOB` record MUST copy the exact authenticated B501 inventory `storage_id` for that source object. Source storage identifiers are inside the encrypted index and therefore are not provider-visible. The sum of every payload `chunk_count` MUST equal the set descriptor `data_object_count` and MUST NOT exceed 1,048,576.

For every payload, `exact_payload_length` MUST equal the checked sum of its chunk plaintext lengths, and `exact_payload_sha256` is SHA-256 of the exact reconstructed payload bytes in chunk order.

Payload semantics are fixed:

- `MANIFEST`: exact complete canonical B501 authenticated manifest envelope bytes; its SHA-256 MUST equal `source_manifest_hash` and its authenticated `VaultId`, generation, and freshness epoch MUST match the index.
- `STRUCTURED_STORE`: exact bytes of the quiesced, integrity-verified SQLCipher snapshot selected by the authenticated manifest/state. Its `source_storage_id` MUST equal the authenticated B501 `STRUCTURED_STORE` inventory record selected for the snapshot. Restore MUST verify the reconstructed SQLCipher file using the canonical B301-B305 provider/runtime/integrity path before accepting it.
- `GENERIC_ARTIFACT_BLOB`: exact complete canonical B202 envelope bytes. Its `source_storage_id` MUST equal the authenticated B501 inventory `storage_id` for the same `ArtifactId`. After reconstruction, Round 4 full-envelope length/hash, storage-ID resolution, and B501 inventory binding MUST succeed before plaintext can be released.

No logical identifier, payload kind, epoch, hash, or semantic role from the index is provider-visible before successful index authentication.

The index AAD is exactly:

```text
domain("HIMSAT/BACKUP/INDEX/AAD/v1")
u16(1)                         # set format version
opaque16(BackupSetId)
u64(key_generation)
u32(data_object_count)
bytes24(index_nonce)
u32(index_ciphertext_and_tag_length)
bytes32(bootstrap_slot_sha256)
```

`bootstrap_slot_sha256` is SHA-256 of the exact 183 bootstrap ciphertext/tag bytes. This binds the encrypted index to the recovery bootstrap selected for the same backup set without exposing the inner recovery envelope.

`index_ciphertext_and_tag_length` MUST be between 16 and `67_108_880` bytes inclusive (64 MiB plaintext plus the 16-byte AEAD tag), and the complete descriptor MUST contain exactly that many index ciphertext/tag bytes with no trailing bytes.

After index decryption, the parser rejects unknown payload kinds, duplicate `(payload_kind, logical_id)` records, duplicate blob logical IDs, duplicate or non-contiguous ordinals, duplicate `BackupObjectId` values, zero or oversized chunks, payload/chunk/global-count overflow, non-canonical ordering, zero generation/epoch, a non-zero manifest `source_storage_id`, and trailing plaintext bytes as `CorruptOrTampered`. The required `MANIFEST` and `STRUCTURED_STORE` records may both use all-zero `logical_id` only because their `payload_kind` values are distinct. After the B501 manifest authenticates, every non-manifest `source_storage_id` and logical identity MUST match exactly one authenticated inventory record; missing, duplicate, extra, or mismatched mappings are `CorruptOrTampered`.

## Backup snapshot and publication protocol

B505 backup creation MUST operate on one verified logical vault state. It MUST NOT combine a manifest from one state with SQLCipher/blob bytes from another concurrent mutation.

A portable set is self-contained around exactly one recovered active VRK generation. Therefore backup creation MUST require the authenticated source manifest to be in stable state before any provider-visible publication: `rotation_phase == NONE`, `rotation_target_generation == 0`, and every authenticated B501 inventory record MUST have `object_key_generation == active_key_generation`. If any inventory object requires another generation, or rotation is in progress, creation fails `BackupStateNotStable`; B505 MUST NOT export a set that requires a second VRK not carried by the recovery bootstrap.

The minimum protocol is:

1. obtain the vault's normal coordination boundary and quiesce commits that could change the authenticated manifest/object set;
2. authenticate the currently accepted B501 manifest against the trusted freshness state, require the stable single-active-generation backup condition above, and verify its complete referenced inventory using each authenticated `storage_id`;
3. require an authenticated B204 recovery envelope for the same active generation, otherwise fail `RecoveryRequired`; obtain the user's recovery passphrase locally, derive the canonical Recovery KEK under the fixed B204 policy, authenticate/decrypt that exact B204 envelope, and require the recovered VRK to equal the currently active unlocked VRK before deriving the bootstrap key or performing any provider-visible publication. Wrong-passphrase/tag failures remain `RecoveryAuthenticationFailed`; a successfully authenticated recovery envelope that yields a different VRK than the active vault is `CorruptOrTampered`. The passphrase/Recovery KEK MUST NOT be logged, transmitted to the provider, or persisted as backup metadata;
4. while writes remain quiesced, use the canonical SQLCipher provider to checkpoint/truncate any WAL so every committed page is represented in the main database, verify no committed state remains only in a sidecar, and close the source handles required for a stable copy;
5. copy the exact stable main-database bytes to a separately named staging file and run the canonical SQLCipher provider/runtime/integrity checks against that staged copy;
6. read and Round-4-verify every exact B202 envelope referenced by the authenticated manifest;
7. generate a fresh `BackupSetId`, fresh `BackupObjectId` values, bootstrap nonce, index nonce, and per-object nonces from the OS CSPRNG;
8. chunk and outer-encrypt every manifest/database/blob payload, require `payload_count <= 262_145` and the checked global chunk/data-object count `<= 1_048_576`, construct the encrypted canonical index within the 64 MiB plaintext bound, then construct the set descriptor;
9. publish only to the new opaque set namespace; never overwrite a previously accepted portable backup in place;
10. reread every uploaded byte through the same provider abstraction and perform full bootstrap/index/object/reconstruction/inner-format verification;
11. mark the backup complete locally only after reread verification succeeds, then release the quiescence boundary.

An upload failure, ambiguous provider acknowledgement, descriptor/object mismatch, or reread-verification failure leaves the candidate set unaccepted. Retry uses a new `BackupSetId` and fresh nonces/object identifiers; it MUST NOT mutate an accepted set or reuse an abandoned encryption attempt's nonce.

Publication does not advance the vault freshness anchor and does not make provider storage canonical vault state. The local authenticated manifest/freshness system remains authoritative.

## Restore protocol

Provider transport and vault-state restoration remain separate security boundaries. B505 retrieves and authenticates a portable backup; B502 remains the sole owner of the existing-anchor freshness republication transition. Cross-generation existing-device restore is a B505/B503 composition boundary and MUST be rebased into the already-current local generation before B502 is invoked.

Restore MUST first:

1. select one opaque set descriptor, parse all provider-visible fields under the strict bounds above, and require the descriptor's Argon2id version, memory, passes, parallelism, and output length to equal the fixed canonical policy before any Argon2id invocation; any mismatch is `CorruptOrTampered`;
2. obtain the user's recovery passphrase without logging or provider transmission;
3. execute only the validated fixed Argon2id policy from the descriptor, derive the bootstrap key, authenticate/decrypt the 183-byte slot, and parse the exact inner B204 envelope;
4. require inner/outer generation, salt, policy, and suite equality; authenticate the inner B204 recovery envelope and release the source VRK only on success;
5. derive the distinct backup-set-specific index/object keys from the recovered source VRK, authenticated `VaultId`, descriptor generation, and parsed `BackupSetId`;
6. authenticate/decrypt the index before interpreting any logical ID, role, epoch, or object relationship;
7. fetch objects only by authenticated `BackupObjectId`, parse the outer object envelope, authenticate each chunk, and reconstruct every payload with checked lengths/counts;
8. require every reconstructed payload hash/length to match the authenticated index;
9. authenticate and canonically parse the recovered B501 manifest; require `rotation_phase == NONE`, `rotation_target_generation == 0`, every inventory `object_key_generation == active_key_generation`, and an exact one-to-one match from every non-manifest index `source_storage_id`/logical identity to the authenticated B501 inventory; then Round-4-verify every reconstructed B202 payload and run canonical SQLCipher provider/integrity checks on the staged structured store.

### Fresh-device restore

On a genuinely fresh device, B502's explicit protected `UNINITIALIZED` genesis and user-visible inability-to-prove-global-newestness rule remains mandatory. The recovered source generation becomes the local generation selected by the authenticated backup. B505 MUST NOT claim that the selected backup was globally newest.

### Existing-device restore rebase

For an existing device with a trusted anchor, let `G` be the recovered backup generation and `H` the already-current active local generation. Before any canonical local mutation, B505 MUST acquire the vault coordination boundary, quiesce ordinary writes, unlock the current vault normally, and authenticate the exact current manifest/anchor under the current VRK `H`.

The current local state MUST be a stable normal-open state: current manifest epoch equals the protected anchor epoch, current manifest hash equals the protected anchor hash, `rotation_phase == NONE`, `rotation_target_generation == 0`, the generation table identifies `H` as the current active generation, and every current inventory object is under `H`. Any interrupted publication, freshness gap, active rotation, retained transition requiring recovery, or inventory-generation mismatch fails `RestoreStateNotStable` and MUST be resolved by its owning recovery path before B505 restore continues.

The recovered backup `VaultId` MUST equal the current vault/anchor `VaultId`, and its authenticated source epoch MUST be strictly less than the current protected anchor epoch. B505 v1 also rejects a source generation greater than the current active generation as `RestoreGenerationAhead`; this fail-closed rule prevents an unproven divergent/future generation from being silently rewritten as an ancestor of the current trusted state.

B505 then creates a non-canonical restore target using fresh, non-colliding local storage IDs. Target IDs MUST NOT alias any current canonical storage ID, any source backup storage ID, or another target ID in the same attempt.

When `G == H`, the recovered source VRK MUST equal the already-unlocked current VRK byte-for-byte. A generation-number match with different key material fails `KeyGenerationIdentityMismatch`. After equality is proven, verified SQLCipher bytes may be copied to a fresh staging location and exact authenticated B202 envelopes may be copied byte-for-byte to fresh target storage locations. This is preservation of an already-authenticated envelope, not a new encryption attempt; B203 copy/restore nonce-preservation rules remain controlling. Before accepting a copied B202 envelope, B505 MUST reconcile its `(VaultId, BoundedBlob, H, nonce)` reservation against the authenticated current/retained `H` reservation set. An unseen reservation is admitted as the existing source encryption instance. A reservation already present is allowed only when the source logical identity, canonical envelope length/hash, and exact authenticated envelope bytes identify the same encryption instance; otherwise restore fails `NonceReservationConflict`. Physical copying of one already-authenticated encryption instance MUST NOT be misclassified as a second encryption attempt, and a conflicting ciphertext under the same reservation MUST NOT be accepted.

When `G < H`, B505 performs a restore rebase using the already-current VRK `H`; it does not rotate or replace the current root key. The complete source state is first authenticated under `G`. Structured-store content is then copied/exported through the canonical SQLCipher path into a separately named database encrypted under the current `H` structured-store key. Every B202 payload is authenticated/decrypted under `G`, re-encrypted under `H` with a fresh B203 nonce, written under a fresh target storage ID, and reread/authenticated under `H`. The source backup bytes and current canonical local state remain retained and unmodified throughout this staging operation. B503F's qualified copy/re-encrypt/verify semantics and B307's source-retaining copy-verify-publish ordering are the implementation precedent; B505 MUST NOT invoke the B503 root-rotation state machine or create a new generation.

After target verification, B505 constructs one synthetic **restore-staging manifest** under `H`. It is an authenticated B501-format envelope used only as B502 input and is never itself published or anchored. Its plaintext:

- keeps the authenticated source backup freshness epoch and source `previous_manifest_hash`;
- uses the current stable generation table with `H` active and no restore-created generation;
- references only the fresh verified target storage IDs;
- records `object_key_generation == H` for every target inventory object;
- records the exact target envelope lengths/hashes and unchanged logical identities/roles required by B501/Round 4.

The staging manifest nonce is generated from an ephemeral B203 manifest-nonce ledger seeded with every retained authenticated canonical manifest reservation known to the current vault. When `G == H`, the authenticated portable source manifest reservation is additionally passed as the exact forbidden reservation for staging-manifest nonce generation; it may already be present in retained local history, but it MUST NOT be reused for the synthetic staging encryption. When `G < H`, the source manifest belongs to a different generation/purpose-key uniqueness domain and is not inserted into the `H` ledger. This proves the staging nonce does not collide with retained canonical `H` history or, for same-generation restore, the detached source manifest encryption. The real B502 restore ledger is independently seeded from the same retained canonical reservations but does not contain the synthetic staging reservation. B505 then calls canonical B502 `prepare_older_backup_restore` with the current VRK `H`, current protected anchor, current generation `H`, the authenticated staging envelope, and explicit user-confirmed older-backup recovery. B502's existing `encrypt_fresh_manifest_avoiding_reservation` rule MUST exclude the staging reservation from the final canonical manifest nonce.

B502 remains the sole owner of the final existing-anchor freshness transition:

```text
final_epoch = trusted_anchor.highest_epoch + 1
final_previous_manifest_hash = trusted_anchor.manifest_hash
final_active_generation = H
```

B505 MUST write/fsync the verified rebased objects and the B502 final manifest candidate, reread/authenticate the complete final object set, and only then publish the final manifest through the canonical copy-verify-publish boundary. After publication it invokes the already-qualified protected compare-and-advance operation with B502's exact `expected_old_anchor` and `new_anchor`. B505 MUST NOT construct an alternate anchor transition, decrement the anchor, or rewrite B502's returned final manifest semantics.

Crash/restart rules are fail closed:

- before final-manifest publication, the pre-restore anchor/current manifest remain authoritative and all restore targets are unaccepted staging material;
- after final-manifest publication but before anchor advancement, canonical B501 interrupted-publication recovery applies only if the published candidate is exactly `old_anchor + 1`, links to the old anchor hash, and its complete rebased object set verifies under `H`;
- after a successful anchor advancement, restart MUST reopen and verify the exact anchored manifest and complete rebased object set under `H` before ordinary writes resume;
- the pre-restore current object set is retained through successful anchor/reopen verification, and B505 makes no B506 deletion or physical-erasure claim;
- retry after an unaccepted attempt uses fresh target storage IDs and fresh encryption nonces for every re-encrypted object and never mutates the detached provider backup.

## Provider-view qualification

B505 qualification MUST inspect the provider-visible boundary as data, not merely scan filenames. The qualifier records every Himsat-controlled provider key and every unencrypted byte field, parses them under this allowlist, and fails on any field outside the exact v1 grammar.

Fixtures MUST include distinctive values for `VaultId`, multiple `ArtifactId` values, user/source filenames, titles, semantic markers, user-created timestamps, and evidence-range text. Qualification MUST search provider keys and visible bytes for their raw/textual encodings where meaningful, including canonical raw ID bytes and lowercase/uppercase hexadecimal ID forms.

Required negative cases include:

- substitute `VaultId`, `ArtifactId`, logical storage ID, filename, title, semantic kind token, or fixture marker into a provider key;
- expose `freshness_epoch` or logical ID in the set descriptor;
- upload raw B202, B204, or B501 canonical bytes directly;
- upload raw SQLCipher database bytes as a B505 provider object instead of the outer object envelope;
- transplant a bootstrap/index/object across set IDs or generations;
- duplicate `BackupObjectId` or object nonce inside a retained set;
- mutate any outer public length/version/suite/nonce/identifier field;
- truncate or append descriptor, bootstrap, index, or object bytes;
- reorder/duplicate/omit index payloads or chunks;
- alter, duplicate, omit, or mis-map an encrypted `source_storage_id`;
- present a source manifest with `rotation_phase != NONE`, a non-zero rotation target, or any inventory object outside the active key generation;
- exceed the 262,145 payload, 1,048,576 data-object/chunk, or 64 MiB index-plaintext bounds;
- corrupt reconstructed SQLCipher, manifest, or B202 payload bytes;
- return a set descriptor whose provider key has a non-zero descriptor leaf or whose set component and embedded `BackupSetId` differ;
- return a data object with the reserved all-zero `BackupObjectId`, or whose provider key and embedded `(BackupSetId, BackupObjectId)` differ;
- derive index/object keys without the exact parsed `BackupSetId`, or transplant ciphertext between backup sets;
- authenticate a B204 envelope during creation that yields a VRK different from the active unlocked VRK;
- attempt existing-device restore while the current manifest/anchor is not a stable exact normal-open pair;
- restore a backup whose `VaultId` differs from the current trusted vault, whose source generation is greater than current `H`, or whose source epoch is not strictly older than the current anchor;
- claim `G == H` while the recovered backup VRK differs from the current unlocked VRK;
- for `G == H`, present a copied B202 nonce reservation that collides with current/retained `H` history but does not identify the exact same authenticated logical object/envelope;
- for `G == H`, force the synthetic staging-manifest nonce candidate to equal the authenticated portable source-manifest reservation;
- collide a restore target storage ID with current/source/staged storage identity;
- accept a cross-generation target whose SQLCipher/B202 re-encryption cannot be reread and authenticated under `H`;
- generate a synthetic staging-manifest nonce that collides with retained canonical history, or allow B502's final manifest to reuse the staging reservation.

Every failure MUST be fail-closed, release no unauthenticated plaintext, and leave the candidate set or restore unaccepted.

Provider-generated modification/upload times and account/transport metadata remain outside Himsat's control and are recorded as residual leakage rather than copied into Himsat-controlled semantic metadata.

## Crash, retry, and retention rules

A B505 portable backup is immutable after acceptance. Failed or ambiguous publication never upgrades a partial set into accepted state. Local cleanup may remove abandoned staging material only after proving it is not the sole copy required by the active vault or an accepted backup.

Provider deletion is not part of B505 cryptographic revocation. Deleting an active vault or rotating its current VRK cannot invalidate a detached portable set that contains a recovery bootstrap for an older generation. This preserves D014: detached copies, provider snapshots, retention systems, and user duplicates may remain decryptable to a holder of the recovery passphrase.

B505 MUST NOT claim physical secure erase, remote revocation, globally newest backup discovery, hidden object counts/sizes/timing, or provider snapshot deletion.

## Scope boundaries

Round 5 changes no canonical B202, B204, B501, Round 3, or Round 4 inner byte format. It does not adopt a dependency, change SQLCipher configuration, add a connector SDK, implement provider-specific transport, change platform protector/freshness semantics, implement B506 deletion, satisfy Q009, authorize Specification 005, or make release/FIPS/compliance claims.

The B505 implementation leaf following this amendment is bounded to provider-neutral portable backup packaging/restore, an in-memory/filesystem test provider sufficient to capture exact provider view, and exact allowlist/adversarial qualification. A real Drive/Box/cloud connector remains Specification 034 or another separately authorized connector leaf.

## Round 5 acceptance and authority gate

This amendment is not self-authorizing. Before B505 product code begins, all of the following MUST be true for the exact canonical Round 5 security-semantic revision:

1. the diff is docs/design/evidence/state only and contains no product code, dependency, provenance-adoption, generated SBOM/notices, workflow, donor, model, dataset, or release change;
2. exact-head CI and Diffcipline R3 succeed without relaxing policy;
3. live base/head/diff/checks/reviews/threads/comments/mergeability are reconciled immediately before merge;
4. merge uses explicit `expected_head_sha` protection and exact parent/tree are verified;
5. push-triggered post-merge CI and R3 succeed on the exact canonical merge;
6. every known blocking finding from historical human, automated, owner, agent, CI, R3, or adversarial evidence is explicitly reconciled against the exact candidate;
7. no unresolved blocking finding remains.

Only then may canonical state reopen authority to the bounded B505 implementation. Independent human review is optional additional assurance rather than a mandatory authority gate. Any later security-semantic change still requires fresh exact-head qualification and blocking-finding reconciliation.
