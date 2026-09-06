# Specification 004 Research — Vault, Keys, and Encrypted Storage Foundation

Date: 2026-09-06

```text
PURPOSE = SHAPING_RESEARCH
IMPLEMENTATION_AUTHORITY = NONE
DEPENDENCY_ADOPTION_AUTHORITY = NONE
CRYPTO_DESIGN_REVIEW = REQUIRED_BEFORE_IMPLEMENTATION
```

This note records current upstream facts and candidate directions. It is not a security audit, dependency-adoption record, or permission to implement cryptography. Exact dependency/source/license provenance remains mandatory before third-party bytes enter Himsat.

## Canonical constraints

Himsat requires local/offline operation, user-controlled keys, OS secure storage where available, encrypted structured/blob data, optional recovery without Himsat escrow, explicit rotation/backup/recovery/deletion/corruption behavior, no custom cryptographic primitive, and R3 adversarial evidence plus independent substantive review. Specification 005 owns crash-safe media journal/chunk semantics.

## Current upstream facts

### Argon2id

RFC 9106 recommends Argon2id for realistic side-channel-aware environments. Its uniform recommendations are `t=1, p=4, m=2 GiB` and, for memory-constrained environments, `t=3, p=4, m=64 MiB`, with 128-bit salt guidance. Himsat should store salt/parameters in a versioned recovery envelope and calibrate only inside an independently reviewed policy; it should not silently weaken recovery KDF settings for convenience.

Source: https://www.rfc-editor.org/rfc/rfc9106.html

### Authenticated blob encryption

Libsodium documents XChaCha20-Poly1305 with a 192-bit nonce and recommends it when interoperability with libraries lacking XChaCha is not required. Its `crypto_secretstream_xchacha20poly1305` API manages nonces/rekeying for ordered streams and arbitrarily large files. Himsat therefore treats XChaCha20-Poly1305 as a bounded-blob candidate and secretstream as a later Specification 005 streaming candidate; neither libsodium nor a Rust implementation is adopted here.

Sources: https://doc.libsodium.org/secret-key_cryptography/aead/chacha20-poly1305 · https://doc.libsodium.org/secret-key_cryptography/secretstream

### OS secure storage

Apple Keychain is intended for small secrets including cryptographic keys; Secure Enclave adds hardware-backed protection for supported key operations, and Keychain items can require user authentication. Android Keystore can keep key material non-exportable, optionally hardware-backed by TEE/StrongBox, and enforce key-use restrictions outside the app process. Windows DPAPI protects static per-user/per-machine data and CNG is the current Windows cryptography family. The freedesktop Secret Service API provides session secret storage on Linux and explicitly warns that lookup attributes are not secret.

Sources:

- Apple: https://developer.apple.com/documentation/security/keychain-services · https://developer.apple.com/documentation/security/protecting-keys-with-the-secure-enclave
- Android: https://developer.android.com/privacy-and-security/keystore · https://developer.android.com/privacy-and-security/security-key-attestation
- Windows: https://learn.microsoft.com/en-us/windows/win32/seccng/cng-dpapi · https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal
- Linux: https://specifications.freedesktop.org/secret-service/latest-single/

Himsat implication: expose one behavior contract while preserving platform truth. Apple uses Keychain as baseline; Android prefers a non-exportable Keystore protector with hardware state detected rather than assumed; Windows uses current-user DPAPI/CNG-class protection; Linux uses Secret Service where available with no secret lookup attributes. No platform gets a silent plaintext fallback.

### SQLCipher

Himsat architecture already names SQLCipher as the leading structured-store candidate. Zetetic reports SQLCipher 4.18.0 (2026-08-18) as current on this research date. SQLCipher supports raw key material, `PRAGMA cipher_status`, and `PRAGMA cipher_integrity_check`. Its design documentation states that database, rollback-journal, and WAL page data are encrypted, but file-backed temporary stores must be disabled because other transient files are not necessarily encrypted.

Sources: https://www.zetetic.net/blog/2026/08/18/sqlcipher-4.18.0-release/ · https://www.zetetic.net/sqlcipher/sqlcipher-api/ · https://www.zetetic.net/sqlcipher/design/

Himsat implication: if separately adopted, use random binary key material from the reviewed vault hierarchy, fail closed unless encryption is active, exercise cipher integrity, intentionally qualify WAL/journal modes, and prove sensitive temporary pages cannot spill to plaintext files.

## Candidate key model for independent review

```text
OS SecretProtector
  -> protects/unlocks random 256-bit Vault Root Key (VRK)
      -> reviewed domain-separated structured-store key
      -> reviewed domain-separated blob/envelope key
      -> future reviewed purpose keys

optional recovery passphrase
  -> Argon2id -> Recovery KEK -> authenticated wrap of same VRK
```

Candidate design rules: generate VRK from an OS CSPRNG; never derive VRK from identity/device/passphrase; keep OS-specific protector mechanics outside the portable format; consider HKDF-SHA-256 with explicit versioned domain separation only after review; keep recovery independent from device unlock; bind ciphertext to vault/object/purpose/version/key-generation context via reviewed AEAD associated data; version key generations; zeroize transient key buffers where practical without claiming protection from a fully compromised unlocked process.

## Structured-store proof requirements

If SQLCipher is adopted, implementation must verify encrypted state immediately after keying; disable file-backed temp storage; qualify WAL and rollback journals; run SQLite plus cipher-integrity checks; fail closed on wrong key/corrupt/unsupported settings; record provider/build identity without secrets; and make rekey/migration recoverable while preserving a verified previous encrypted copy until replacement succeeds.

## Blob-envelope proof requirements

A Himsat envelope must be versioned/authenticated and contain only the minimum format/cipher-suite/key-generation/nonce-or-header/ciphertext/tag metadata. Reviewed associated data should bind vault ID, object ID, purpose, envelope version, and key generation. Do not embed plaintext filenames/content for convenience. XChaCha20-Poly1305 is the leading bounded-blob candidate; large media remains Specification 005.

## Backup, rotation, and deletion

Portable backup must contain encrypted structured data/blobs, non-secret versioned manifest/inventory, and only when enabled an authenticated recovery-KEK-wrapped VRK plus KDF salt/parameters. Device-specific protectors alone are insufficient for cross-device recovery. If recovery is disabled and all device protectors are lost, permanent loss is expected and must be communicated.

Differentiate device-protector rotation, recovery-KEK rotation, full VRK generation rotation, and device revocation. VRK rotation is a recoverable data migration; never destroy an old generation while canonical data still requires it. Deletion covers canonical rows, blobs, derived indexes/caches/temp files, wraps/protector records, and transient keys. Destroying all wraps may provide crypto-erasure, but Himsat must not claim guaranteed physical secure erase on SSD/mobile flash or remote provider copies.

## Lock state

```text
LOCKED -> UNLOCKING -> UNLOCKED -> LOCKING
                     -> ROTATING
                     -> RECOVERY_REQUIRED
                     -> CORRUPT_OR_TAMPERED
```

No decrypt while locked; errors are typed; UI/account state is not cryptographic unlock state; locking closes keyed handles and zeroizes transient keys where practical; restart behavior follows platform protector policy instead of assuming universal auto-unlock.

## Minimum adversarial evidence

Wrong VRK/passphrase; tampered key wrap/blob header/ciphertext/tag/AAD; nonce/header corruption; stale/unknown key generation or envelope version; protector unavailable/locked/invalidated; recovery envelope copied across vault identity; accidental unencrypted database handle; WAL/journal/temp plaintext-marker scan; SQLCipher cipher-integrity corruption; interrupted rekey/rotation/restore; partial blob inventory; deletion residue; secret leakage to logs/crash output; backup surface audit.

## Decisions reserved for independent review

The reviewer must disposition: HKDF-SHA-256 and exact domain separation; XChaCha20-Poly1305 nonce/AAD rules; libsodium vs RustCrypto/other established provider; Argon2id minimum/calibration policy; SQLCipher version/build/provider/temp/WAL strategy; Apple/Android/Windows/Linux protector behavior and user-presence policy; rotation/recovery/rollback/deletion semantics; and the exact implementation split needed safely by Specification 005.

No implementation branch is authorized merely because this research exists. Review must identify the exact Git revision, and all blocking findings must be resolved or explicitly dispositioned before crypto implementation begins.
