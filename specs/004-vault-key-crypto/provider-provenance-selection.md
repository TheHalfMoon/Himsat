# Specification 004P — Provider and Provenance Selection

## Status

```text
BASE_CANONICAL_SHA = 0ad55fc1c457403bb9454fbf69f22525cfabb317
004P_AUTHORITY = BOUNDED_PROVIDER_PROVENANCE_SELECTION
P001 = SELECTED_NOT_ADOPTED
P002 = SELECTED_NOT_ADOPTED
P003 = PENDING_EXACT_RESOLVED_CLOSURE
P004 = PENDING_LICENSE_NOTICE_CLOSURE
P005 = PENDING_REGISTRY_SBOM_NOTICES_ADOPTION
P006 = PENDING_LOCKFILE_NATIVE_CLOSURE_PROOF
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```

This document selects the exact provider/library strategy for Specification 004. It does **not** adopt package bytes, modify `Cargo.toml` or `Cargo.lock`, register dependency entries, or authorize 004B implementation.

The Specification 002 provenance registry remains the machine adoption boundary. Exact crates.io package checksums, resolved transitive versions, source revisions, notices, and the final native build closure MUST be proven in P003-P006 before any dependency graph is accepted canonically.

## P001 — established crypto provider/library strategy

Himsat selects established RustCrypto primitives plus direct operating-system randomness. No custom cipher, MAC, KDF, PRNG, protocol, or general-purpose `rand` facade is selected.

### Selected direct packages

| Purpose | Package | Exact version | Immutable upstream release revision | Selected feature posture |
| --- | --- | --- | --- | --- |
| XChaCha20-Poly1305 | `chacha20poly1305` | `0.11.0` | `e37a978ccf0992d9053fbc039470d6527108e393` | `default-features = false`, features `alloc`, `zeroize`; nonce generation is external/direct OS randomness |
| HKDF | `hkdf` | `0.13.0` | `bfb3b209abeeaa02277935b167e06bac320b2773` | no optional `kdf` feature; generic HKDF instantiated only with selected SHA-256 |
| SHA-256 | `sha2` | `0.11.0` | `ffe093984c004769747e998f77da8ff7c0e7a765` | `default-features = false`; SHA-256 only for reviewed HKDF/hash uses |
| Argon2id | `argon2` | `0.6.0` | `b1e0ad6fe229b1ba74e4696c7359ab45d7e931f0` | `default-features = false`, features `alloc`, `zeroize`; do not enable PHC/password-string formatting or parallelism unless separately reviewed |
| secret zeroization | `zeroize` | `1.9.0` | `0b715735a660a8566ccd240bf42489fe2ed98efb` | default `alloc` only; no derive/serde feature unless separately justified and provenance-closed |
| OS CSPRNG access | `getrandom` | `0.4.3` | `5e7cd5733536844a9856dc7259bd4696bbe5e3ae` | direct `fill`/supported OS backend only; no unsupported/custom fallback |

### P001 normative constraints

1. Production randomness MUST come from the selected `getrandom` OS-backed implementation or a separately reviewed platform mechanism that is proven to route to a cryptographically secure OS source.
2. Random-source failure MUST propagate as a typed fail-closed error. It MUST NOT fall back to timestamps, deterministic seeds, weak PRNGs, UUID randomness assumptions, user data, or reused bytes.
3. `chacha20poly1305` is selected specifically for its XChaCha20-Poly1305 implementation and 24-byte nonce support. Himsat owns nonce lifecycle policy exactly as frozen in Specification 004A.
4. `hkdf` MUST be instantiated with `sha2::Sha256`; an implementation MUST NOT silently substitute another digest.
5. `argon2` MUST execute the versioned Argon2id recovery policy frozen by Specification 004A. The package's password-string/PHC convenience surface is not required for the v1 recovery envelope and remains disabled by this selection.
6. `zeroize` is best-effort in-process secret-lifetime hygiene and MUST NOT be represented as proof against kernel, crash-dump, swap, physical-memory, firmware, or unlocked-process compromise.
7. Direct package versions MUST be exact-pinned when the dependency graph is adopted. SemVer ranges are not sufficient provenance evidence.
8. P003 MUST record every transitive package actually selected by Cargo and prove its crates.io checksum/source/revision/license before P005.

### P001 rejected alternatives

- **Custom crypto primitives/protocols:** prohibited by governance and unnecessary.
- **General `rand` facade for security-critical bytes:** rejected for this leaf because direct `getrandom` gives a narrower and auditable OS-randomness boundary.
- **Implicit `chacha20poly1305` random-generation defaults:** rejected; Himsat must own nonce generation/failure semantics explicitly.
- **Argon2 default feature set:** rejected because PHC formatting/random helper features are not required by the reviewed raw recovery-envelope KDF contract and widen the closure.
- **Unpinned compatible-version ranges:** rejected as final adoption identity; exact resolved package versions/checksums are required.

## P002 — SQLCipher core/binding/provider/build strategy

Himsat retains SQLCipher for encrypted structured storage and selects a bundled, deterministic provider path rather than target-dependent system crypto discovery.

### Binding and SQLCipher identities

```text
RUSQLITE_PACKAGE = 0.40.1
RUSQLITE_RELEASE_REVISION = 6d3c282dc5531a57eb4e22ece3207f00c95d0fb0
RUSQLITE_LICENSE = MIT

LIBSQLITE3_SYS_PACKAGE = 0.38.1
LIBSQLITE3_SYS_REPOSITORY = https://github.com/rusqlite/rusqlite
LIBSQLITE3_SYS_RELEASE_REVISION = 6d3c282dc5531a57eb4e22ece3207f00c95d0fb0

RUSQLITE_FEATURE = bundled-sqlcipher-vendored-openssl
SQLCIPHER_VERSION = 4.14.0
SQLCIPHER_TAG_OBJECT = 46bb08ec73b2caa84b6945a19c9e435fff446dcd
SQLCIPHER_SOURCE_REVISION = 778ab890cfc30c3631212dcceb0295498abdcd3e
SQLCIPHER_LICENSE = BSD-3-Clause
SQLCIPHER_EMBEDDED_SQLITE_VERSION = 3.51.3
SQLCIPHER_EMBEDDED_SQLITE_SOURCE_ID = 737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1
```

The `rusqlite` v0.40.1 `upgrade_sqlcipher.sh` explicitly freezes the vendored SQLCipher source at `4.14.0`; the vendored SQLCipher `sqlite3.h` at the same revision reports SQLite `3.51.3`. The separate ordinary `bundled` SQLite baseline is not the SQLCipher baseline and MUST NOT be substituted in evidence.

### Crypto provider identity

The selected SQLCipher feature is:

```text
rusqlite = { version = "=0.40.1", default-features = false, features = ["bundled-sqlcipher-vendored-openssl"] }
```

This selects `libsqlite3-sys` bundled SQLCipher and its vendored OpenSSL provider path.

The expected exact provider resolution to be proven by P003 is:

```text
OPENSSL_SYS_PACKAGE = 0.9.117
OPENSSL_SYS_RELEASE_REVISION = db9c9e2f5db2ad7b45fd894e8d297ee15bfd0c7c
OPENSSL_SYS_LICENSE = MIT
OPENSSL_SRC_PACKAGE = 300.6.1+3.6.3
OPENSSL_SRC_RELEASE_REVISION = 64c38cc48205400720199476aff8a780f98d167d
OPENSSL_UPSTREAM_VERSION = 3.6.3
OPENSSL_UPSTREAM_SUBMODULE_REVISION = aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f
OPENSSL_LICENSE = Apache-2.0
```

These are selected identities, not yet adopted registry entries. If Cargo resolves any different `openssl-sys`, `openssl-src`, OpenSSL source, or native provider identity, P003 MUST stop and reconcile the difference before adoption.

### Build-mode freeze

For the selected `bundled-sqlcipher-vendored-openssl` path:

- SQLCipher MUST be compiled from the `libsqlite3-sys` vendored amalgamation, not discovered from a system SQLCipher installation.
- OpenSSL MUST be built from the exact P003-qualified `openssl-src` package/submodule, not discovered from `/usr`, Homebrew, vcpkg, arbitrary `OPENSSL_DIR`, or another host installation.
- `OPENSSL_NO_VENDOR` MUST NOT be set for qualified builds.
- Himsat MUST NOT set `OPENSSL_LIB_DIR`, `OPENSSL_INCLUDE_DIR`, or `OPENSSL_DIR` to redirect the selected vendored-provider build.
- `LIBSQLITE3_SYS_USE_PKG_CONFIG` MUST NOT be set to force a linked system library.
- no unreviewed `LIBSQLITE3_FLAGS` additions/removals are allowed.
- Windows qualification MUST set `OPENSSL_RUST_USE_NASM=0` unless a separately reviewed deterministic NASM toolchain identity is added to P003; this prevents host-PATH NASM presence from silently changing the native build.
- the qualified compiler/toolchain identity and target triples MUST be recorded with implementation evidence before 004B closeout.

### Security-relevant SQLCipher compile posture inherited from the selected binding

The selected `libsqlite3-sys` bundled SQLCipher build path enables its checked-in build-script definitions, including:

```text
SQLITE_ENABLE_MEMORY_MANAGEMENT
SQLITE_ENABLE_RTREE
SQLITE_ENABLE_STAT4
SQLITE_SOUNDEX
SQLITE_THREADSAFE=1
SQLITE_USE_URI
HAVE_USLEEP=1
HAVE_ISNAN
_POSIX_THREAD_SAFE_FUNCTIONS
SQLITE_HAS_CODEC
SQLITE_TEMP_STORE=2
SQLITE_EXTRA_INIT=sqlcipher_extra_init
SQLITE_EXTRA_SHUTDOWN=sqlcipher_extra_shutdown
HAVE_STDINT_H=1
```

Android builds additionally use the binding's Android `SQLITE_TEMP_STORE=3` path. Any compile-definition change that affects storage, crypto-provider selection, journaling/temp behavior, or authentication semantics requires explicit re-review before qualification.

### P002 rationale and rejected alternatives

**Selected: `bundled-sqlcipher-vendored-openssl`.** It provides one auditable SQLCipher source identity and one explicit vendored crypto-provider lineage across the supported Rust core targets, subject to exact target qualification.

Rejected for the canonical v1 strategy:

- `sqlcipher` linked against a host-installed SQLCipher: rejects immutable core/provider identity.
- plain `bundled-sqlcipher`: rejected because the provider selection differs by host/target and environment; Apple may use Security/CommonCrypto while Linux and Windows use/discover `libcrypto`, making the canonical provider identity environment-dependent.
- ordinary `bundled`: rejected because it builds unencrypted SQLite rather than SQLCipher.
- dynamically discovered OpenSSL/LibreSSL: rejected because provider version/build flags become host state rather than repository-qualified inputs.
- OpenSSL 4.x: not selected by the `openssl-src` 300.x compatibility line used by the chosen `openssl-sys` vendored dependency and not part of this P002 decision.

## Target/platform qualification boundary

This provider decision covers the shared Rust encrypted-storage core. It does not claim identical secure-store behavior across Apple, Android, Windows, and Linux; the platform protector adapters remain separately qualified under 004B4.

P003-P006 and 004B qualification MUST prove the actual supported target matrix. A target on which the selected SQLCipher/OpenSSL build cannot be reproduced, provenance-closed, and tested MUST fail qualification rather than silently switch to a system provider.

## P003-P006 entry conditions

Before dependency adoption, the next provenance leaf MUST:

1. resolve the exact Cargo graph from exact-pinned direct dependencies and selected features;
2. capture every external package name/version/source/checksum in the resulting `Cargo.lock`;
3. map every package to an immutable upstream repository revision and controlling license/notice;
4. inventory the SQLCipher amalgamation and OpenSSL native source identity explicitly;
5. reject any manual/deny/unknown license disposition under `governance/provenance/policy.json`;
6. write registry entries for the **complete** external lockfile closure in the same bounded adoption change that introduces that lockfile closure;
7. regenerate deterministic `THIRD_PARTY_NOTICES.md` and Himsat SBOM with `python tools/provenance.py generate` semantics;
8. pass `python tools/provenance.py check` and adversarial provenance tests on every qualified CI platform;
9. prove there are no registry extras, unregistered lockfile dependencies, source mismatches, checksum mismatches, or unresolved native components.

Until P003-P006 close canonically:

```text
DEPENDENCY_BYTES_ADOPTED = NO
CARGO_MANIFEST_CHANGED = NO
CARGO_LOCK_CHANGED = NO
PROVENANCE_REGISTRY_CHANGED = NO
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```
