# Specification 004 Research — Vault, Keys, and Encrypted Storage Foundation

Date: 2026-09-06

## Status

```text
PURPOSE = SHAPING_RESEARCH
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
CRYPTO_DESIGN_REVIEW = REQUIRED_BEFORE_IMPLEMENTATION
```

This note narrows current upstream facts and candidate directions for Specification 004. It is not a dependency-adoption record, a security audit, or permission to implement cryptography. Exact dependency/source/license provenance must be established separately before any third-party code enters Himsat.

## Canonical Himsat constraints

The constitution and security plan require:

- local/offline operation without mandatory Himsat cloud or account;
- user-controlled cryptographic keys;
- OS secure storage where available;
- encrypted structured data and encrypted blobs;
- optional recovery without silent Himsat escrow;
- key rotation/revocation, backup, recovery, deletion, and corruption behavior;
- no plaintext vault master key in connector configuration;
- no custom cryptographic primitive;
- R3 adversarial evidence and independent substantive review.

Specification 005 owns crash-safe media journal/chunk storage. Specification 004 must therefore provide the key/envelope/storage foundation without absorbing media-journal semantics.

## Current upstream facts

### Password/recovery KDF

RFC 9106 standardizes Argon2 and recommends Argon2id for realistic environments where side-channel attacks may matter. Its two uniform recommendations are:

- Argon2id, `t=1`, `p=4`, `m=2 GiB` as the first recommended option;
- Argon2id, `t=3`, `p=4`, `m=64 MiB` as the second recommended option for memory-constrained environments.

Source:

- https://www.rfc-editor.org/rfc/rfc9106.html

Himsat implication: recovery passphrase derivation should use Argon2id, store salt and parameters in a versioned recovery envelope, and calibrate within an independently reviewed policy rather than silently choosing weak parameters for convenience. Mobile/resource evidence is required before choosing a universal runtime profile.

### Blob authenticated encryption

Libsodium documents XChaCha20-Poly1305 as an AEAD option with a 192-bit nonce and states that random nonces are safe to use; it recommends XChaCha20 where interoperability with libraries lacking XChaCha is not required. Libsodium also provides `crypto_secretstream_xchacha20poly1305`, which manages nonces/rekeying for ordered streams and can encrypt arbitrarily large files.

Sources:

- https://doc.libsodium.org/secret-key_cryptography/aead/chacha20-poly1305
- https://doc.libsodium.org/secret-key_cryptography/secretstream

Himsat implication: XChaCha20-Poly1305 is a strong candidate for Himsat-owned opaque blob envelopes; `secretstream` is a candidate for later large/streamed media handling. Neither libsodium nor a Rust implementation is adopted by this research note.

### Apple secure storage

Apple Keychain Services is designed to store small secrets including cryptographic keys in encrypted storage. Apple documents Secure Enclave as an additional hardware-backed key-management layer for supported private-key operations. Keychain access can also be gated by user authentication such as Face ID or Touch ID.

Sources:

- https://developer.apple.com/documentation/security/keychain-services
- https://developer.apple.com/documentation/security/storing-keys-in-the-keychain
- https://developer.apple.com/documentation/security/protecting-keys-with-the-secure-enclave
- https://developer.apple.com/documentation/localauthentication/accessing-keychain-items-with-face-id-or-touch-id

Himsat implication: Apple adapters should use Keychain as the baseline secret-protection boundary. Secure Enclave must be used only for operations it actually supports; Himsat must not pretend arbitrary symmetric vault keys can universally remain inside the enclave.

### Android secure storage

Android Keystore can keep key material non-exportable, can bind keys to TEE/StrongBox hardware where supported, and can enforce usage/authentication restrictions outside the application process. Hardware-backed capability is device-specific and can be inspected/attested where appropriate.

Sources:

- https://developer.android.com/privacy-and-security/keystore
- https://developer.android.com/privacy-and-security/security-key-attestation

Himsat implication: Android should prefer a non-exportable Keystore protector key for wrapping/unwrapping a Himsat vault root secret. Hardware-backed status is capability evidence, not a universal guarantee.

### Windows secure storage

Microsoft documents DPAPI (`CryptProtectData` / `CryptUnprotectData`) as a mechanism for protecting static data on one computer and CNG as the modern cryptography API family. Microsoft explicitly recommends DPAPI for protecting data at rest when the application should not manage raw protection keys itself.

Sources:

- https://learn.microsoft.com/en-us/windows/win32/seccng/cng-dpapi
- https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal

Himsat implication: Windows can use current-user DPAPI/CNG-class protection for the small wrapped vault-root record. Cross-device recovery must remain a Himsat recovery-envelope concern rather than relying on local-machine DPAPI portability.

### Linux secure storage

The freedesktop Secret Service API provides a common D-Bus interface for applications to store secrets in a service running in the user's login session. The specification warns that lookup attributes are not secret and may be stored unencrypted.

Source:

- https://specifications.freedesktop.org/secret-service/latest-single/

Himsat implication: Linux can use Secret Service where available, but labels/attributes must contain no secret material. There must be no silent plaintext-key fallback when Secret Service is unavailable or locked.

### Structured database encryption

SQLCipher remains the leading candidate identified by Himsat architecture. As of this research date, Zetetic reports SQLCipher 4.18.0 (2026-08-18) as the current release. SQLCipher provides full-database encryption for SQLite, supports raw key material, provides `PRAGMA cipher_status` to confirm that a handle is encrypted, and `PRAGMA cipher_integrity_check` to verify page HMAC integrity.

Zetetic's design documentation states:

- main database pages are encrypted;
- rollback-journal and WAL page data are encrypted;
- file-based temporary stores must be disabled because other transient files are not necessarily encrypted;
- raw binary key data can be supplied instead of a passphrase;
- SQLCipher delegates cryptographic primitives to established providers rather than inventing them.

Sources:

- https://www.zetetic.net/blog/2026/08/18/sqlcipher-4.18.0-release/
- https://www.zetetic.net/sqlcipher/sqlcipher-api/
- https://www.zetetic.net/sqlcipher/design/

Himsat implication: if adopted, Himsat should provide a random binary database key from its vault hierarchy rather than asking SQLCipher to derive the database key directly from the user's recovery passphrase. Runtime qualification must fail closed if `cipher_status` is not affirmative, exercise `cipher_integrity_check`, and prove that build/runtime temp-store settings cannot spill sensitive temporary pages to plaintext files.

## Candidate Himsat key model for review

The following is a **review candidate**, not an approved cryptographic construction.

```text
OS secret protector
       │
       └── protects/unlocks a random 256-bit Vault Root Key (VRK)
                                    │
                                    ├── domain-separated structured-store key
                                    ├── domain-separated blob-wrap/envelope key
                                    └── future domain-separated purpose keys

optional recovery passphrase
       │
       └── Argon2id -> Recovery KEK -> authenticated wrap of the same VRK
```

Design intent:

1. Generate each VRK from an OS CSPRNG; never derive it directly from account identity, device identifier, or user passphrase.
2. Keep the OS-specific protector outside the portable vault format. The portable format stores only a protected/wrapped VRK record and non-secret metadata.
3. Use explicit versioned domain separation for purpose keys. HKDF-SHA-256 is the initial candidate KDF for independent review; exact labels/salts/context must be frozen by the reviewed design before implementation.
4. Keep the recovery path independent from normal device unlock. A recovery passphrase derives a Recovery KEK with Argon2id and authenticates a wrapped VRK record; Himsat does not receive or escrow the passphrase/VRK.
5. Bind ciphertext to non-secret identity/version context through AEAD associated data so ciphertext moved across vault/object/purpose contexts is rejected.
6. Treat key generation/rotation as versioned state. New writes use the active generation while old generations remain only as long as required for migration/recovery.
7. Zeroize transient VRK/derived-key buffers where practical. Do not claim that zeroization defeats a fully compromised process or guarantees physical-memory erasure.

## Proposed storage split

### Structured store

Candidate: SQLCipher-backed SQLite.

Required behaviors before acceptance:

- open with raw random key material derived from the reviewed vault hierarchy;
- immediately verify encrypted state with `cipher_status` or an equivalent version-pinned check;
- disable file-backed temp storage in the qualified build/runtime profile;
- exercise rollback-journal and WAL modes intentionally rather than inheriting defaults;
- run normal SQLite integrity plus SQLCipher cipher-integrity checks;
- fail closed on wrong key, partial/corrupt database, encryption-disabled handle, or unsupported cipher settings;
- record exact SQLCipher/provider/build identity for diagnostics without logging secrets;
- migration/rekey must have crash/recovery tests and must never destroy the only known-good encrypted copy before replacement is verified.

### Opaque blob envelope

Candidate: versioned Himsat envelope over an established AEAD implementation.

Candidate logical fields:

```text
magic/version
cipher_suite_id
key_generation
nonce/header
associated-data schema version
ciphertext
authentication tag
```

Associated data should bind at minimum:

```text
vault_id
object_id
object_kind/purpose
envelope_version
key_generation
```

The ciphertext format must not include plaintext filenames, transcript content, or secret lookup values merely for convenience. Metadata leakage that remains necessary for filesystem operation must be documented separately.

For bounded generic blobs, XChaCha20-Poly1305 is the leading candidate. Large/streamed media encryption belongs to Specification 005, which should evaluate `secretstream` or an equivalent reviewed streaming construction rather than reuse a one-shot API unsafely.

## Backup and recovery semantics

A backup is not complete merely because encrypted files were copied.

Candidate portable backup set:

```text
vault manifest (non-secret/versioned)
encrypted structured store
encrypted blobs
optional recovery envelope containing only the authenticated, Recovery-KEK-wrapped VRK
KDF salt + Argon2id parameters + format version
integrity/digest inventory
```

Rules:

- device-specific OS protector records are not sufficient for cross-device recovery;
- Himsat-hosted services never receive plaintext VRK or recovery passphrase;
- a user who declines recovery and loses all valid device protectors may permanently lose the vault; the product must say this clearly;
- recovery must authenticate the wrapped VRK before any database/blob is accepted;
- restore must verify vault identity, key generation, database cipher status/integrity, and blob authentication before declaring success;
- backup providers may observe ciphertext sizes, timing, object counts, and any deliberately exposed names; do not claim metadata invisibility unless measured/proven.

## Rotation and revocation semantics

The design must distinguish:

- **device-protector rotation**: re-protect the same VRK with a replacement OS protector record;
- **recovery-KEK rotation**: derive a new Recovery KEK and replace the authenticated VRK wrap;
- **vault-root rotation**: create a new VRK generation and migrate structured/blob keys/content;
- **device revocation**: delete/revoke that device's protector/wrap without implying that ciphertext already copied to the device disappears remotely.

Vault-root rotation is a data migration and must be crash recoverable. Old key generations must not be deleted until all canonical data that requires them is proven migrated or intentionally retained.

## Deletion and crypto-erasure semantics

Deletion must address:

- canonical structured rows;
- encrypted source blobs;
- derived indexes/caches/temp files;
- backup manifests/copies according to user-selected policy;
- recovery wraps;
- OS protector records;
- in-memory key material.

Deleting the last valid VRK wrap can make remaining ciphertext cryptographically inaccessible, but Himsat must not call this guaranteed physical secure erase on SSD/mobile flash. File-system copies, snapshots, provider backups, wear leveling, and forensic recovery are separate threat surfaces.

## Lock/unlock semantics

Candidate state machine:

```text
LOCKED
UNLOCKING
UNLOCKED
ROTATING
RECOVERY_REQUIRED
CORRUPT_OR_TAMPERED
LOCKING
```

Rules:

- no database/blob decryption while `LOCKED`;
- unlock errors are typed and do not fall back to plaintext;
- UI/account state is not cryptographic unlock state;
- locking closes keyed database handles and zeroizes transient keys where practical;
- app restart defaults to a state consistent with the configured OS protector policy, not a hard-coded assumption that every platform auto-unlocks.

## Required adversarial evidence for implementation

At minimum:

- wrong VRK / wrong recovery passphrase;
- modified wrapped-key record;
- modified ciphertext/tag/header/associated data;
- nonce/header corruption;
- key-generation rollback and unknown future version;
- OS secret store unavailable/locked/permission failure;
- recovery envelope copied to a different vault identity;
- database opened without encryption accidentally;
- database WAL/journal/temp-file inspection for plaintext markers;
- SQLCipher cipher-integrity corruption fixture;
- interrupted database rekey/rotation;
- interrupted recovery restore;
- partial/missing blob inventory;
- deletion leaves no application-managed plaintext cache/index/temp file;
- logs/crash reports contain no key/passphrase/plaintext fixture secrets;
- backup provider receives only the documented encrypted/non-secret surfaces.

## Decisions intentionally left for independent review

The independent crypto/security reviewer must approve or request changes to:

1. whether HKDF-SHA-256 is the correct portable subkey-derivation strategy and the exact domain-separation scheme;
2. whether XChaCha20-Poly1305 is appropriate for bounded Himsat blob envelopes and the exact nonce/associated-data rules;
3. whether a libsodium-based implementation, RustCrypto-based implementation, or another established provider minimizes implementation and supply-chain risk;
4. Argon2id minimum/calibration policy for desktop and mobile recovery;
5. SQLCipher adoption/build/provider strategy and temp/WAL configuration;
6. OS protector behavior on Apple, Android, Windows, and Linux, including biometric/user-presence policy and fallback behavior;
7. key rotation, recovery, rollback, and deletion semantics;
8. the exact implementation split that allows Specification 005 to consume the vault safely.

No implementation branch should be authorized merely because this document exists. The reviewed design must be tied to an exact Git commit and all blocking findings must be resolved or explicitly dispositioned before crypto implementation starts.
