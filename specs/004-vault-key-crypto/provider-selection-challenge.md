# Specification 004P — Provider Selection Challenge Record

## Purpose

This file records an adversarial challenge pass over `provider-provenance-selection.md` before P001/P002 are treated as canonical selections.

It is not dependency-adoption evidence. P003-P006 remain mandatory.

## P001 challenge

### Direct OS randomness

`getrandom 0.4.3` is retained as the narrow OS-CSPRNG boundary. The selected Himsat implementation must call the direct fallible API and propagate failure. No `rand` facade, deterministic fallback, or implicit AEAD nonce generation is required by the selection.

### XChaCha20-Poly1305

The exact `chacha20poly1305 0.11.0` release manifest confirms XChaCha20-Poly1305 support and shows that its default features include `getrandom`. The selection deliberately disables defaults and enables only `alloc` plus `zeroize`, so nonce generation remains an explicit Himsat responsibility rather than an implicit package helper.

### HKDF-SHA-256

The exact `hkdf 0.13.0` manifest depends on generic `hmac` and does not itself freeze a digest. Therefore `sha2 0.11.0` is an explicit direct selection and the implementation contract requires `Hkdf::<Sha256>` semantics. This closes an otherwise implicit algorithm-selection gap.

### Argon2id feature minimization

The exact `argon2 0.6.0` manifest shows that default features would enable `alloc`, `getrandom`, and `password-hash`. The selected posture disables defaults and enables only `alloc` and `zeroize`. This intentionally excludes PHC/password-string formatting, salt-generation helpers, and parallel execution from the reviewed recovery KDF path.

P003 must still inventory mandatory Argon2 transitive dependencies including `base64ct`, `blake2`, and architecture-specific `cpufeatures` where resolved.

## P002 challenge

### SQLCipher source identity

The `rusqlite v0.40.1` release commit is `6d3c282dc5531a57eb4e22ece3207f00c95d0fb0`. At that exact revision:

- `libsqlite3-sys` is `0.38.1`;
- `upgrade_sqlcipher.sh` sets `SQLCIPHER_VERSION="4.14.0"`;
- SQLCipher annotated tag `v4.14.0` resolves to upstream commit `778ab890cfc30c3631212dcceb0295498abdcd3e`;
- the vendored SQLCipher `sqlite3.h` reports embedded SQLite `3.51.3`, source id `737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1`.

This confirms that the SQLCipher path must not use the ordinary bundled-SQLite version as its evidence identity.

### Provider determinism

The `libsqlite3-sys` build logic proves that plain `bundled-sqlcipher` is environment/target dependent:

- explicit OpenSSL environment paths can select a host library;
- Apple may fall back to Security/CommonCrypto;
- non-Apple Unix links `crypto` when no explicit OpenSSL path is selected;
- Windows without the vendored feature requires an external OpenSSL installation.

Therefore plain `bundled-sqlcipher` fails the P002 deterministic-provider objective.

`bundled-sqlcipher-vendored-openssl` remains selected because it routes the SQLCipher crypto provider through the Cargo-resolved `openssl-sys` vendored source path instead of silently selecting target-installed crypto libraries.

### Known compiled legacy-provider feature

The exact `openssl-sys 0.9.117` manifest selects:

```text
openssl-src = { version = "300.2.0", optional = true, features = ["legacy"] }
```

Under the selected current 300.x line, P002 expects `openssl-src 300.6.1+3.6.3` at source revision `64c38cc48205400720199476aff8a780f98d167d`, which vendors upstream OpenSSL submodule commit `aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f`.

The `legacy` feature means the OpenSSL legacy provider is compiled into the vendored source build. Himsat MUST NOT hide this fact or claim that the provider is absent.

This does **not** by itself prove that SQLCipher loads or uses the legacy provider. P003-P006 and 004B qualification must distinguish:

```text
COMPILED_PROVIDER_SURFACE != ACTIVE_SQLCIPHER_PROVIDER
```

The implementation evidence must prove the actual SQLCipher crypto provider/version for an opened encrypted handle and must not claim a narrower OpenSSL build than the adopted `openssl-src` feature graph provides.

If qualification proves that the compiled legacy-provider surface cannot be bounded acceptably, the selected SQLCipher/OpenSSL strategy must be reopened before 004B rather than patched around silently.

### OpenSSL configuration/environment influence

Vendoring source removes host-library version discovery but does not justify ignoring runtime/configuration influence. 004B qualification must fail if environment/configuration can make the SQLCipher provider identity differ from the canonical selected provider without detection.

At minimum, encrypted-handle qualification must record the SQLCipher provider identity/version exposed by the selected SQLCipher build and fail closed on an unexpected provider/version.

No FIPS claim is authorized. The selected OpenSSL build is not a Himsat FIPS qualification.

### Native build prerequisites

The selected vendored OpenSSL path introduces native build prerequisites such as a C compiler, Perl, make/build tooling, and platform-specific linker behavior. These are build inputs, not automatically provenance-free host assumptions.

P003-P006 must record the qualified toolchain/target matrix and the Cargo-native source closure. Windows keeps `OPENSSL_RUST_USE_NASM=0` unless NASM itself is explicitly qualified and added to the build-evidence boundary.

## License challenge

The repository policy allows `Apache-2.0`, `BSD-3-Clause`, `MIT`, and related permissive licenses. The selected top-level sources are compatible with that policy:

- RustCrypto selected crates: MIT OR Apache-2.0; Himsat may record one permitted option only after exact package/source verification;
- `getrandom`: MIT OR Apache-2.0;
- `rusqlite` / `libsqlite3-sys`: MIT;
- SQLCipher: BSD-3-Clause-style three-clause license at exact upstream revision;
- `openssl-sys`: MIT;
- `openssl-src`: MIT/Apache-2.0 source wrapper;
- upstream OpenSSL 3.6.3: Apache-2.0.

This top-level license check is not P004 completion. Every resolved transitive Cargo/native component still requires its own controlling license and notice disposition.

## Challenge disposition

```text
P001_SELECTION = ACCEPT
P002_SELECTION = ACCEPT_WITH_EXPLICIT_COMPILED_LEGACY_PROVIDER_EVIDENCE
P003_P006 = REQUIRED_BEFORE_ADOPTION
KNOWN_UNRESOLVED_BLOCKER = NONE_AT_SELECTION_LAYER
DEPENDENCY_BYTES_ADOPTED = NO
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```

The next leaf must resolve and prove the complete graph. Any resolver drift from the exact identities selected here reopens the affected selection instead of being silently accepted.
