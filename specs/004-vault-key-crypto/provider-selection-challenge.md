# Specification 004P — Provider Selection Challenge Record

## Purpose

This file records adversarial challenge passes over `provider-provenance-selection.md` before P001/P002 are treated as stable selections and before any dependency bytes are adopted.

P003-P006 remain mandatory.

## P001 challenge

### Direct OS randomness

`getrandom 0.4.3` is retained as the narrow OS-CSPRNG boundary. The selected Himsat implementation must call the direct fallible API and propagate failure. No `rand` facade, deterministic fallback, or implicit AEAD nonce generation is required by the selection.

### XChaCha20-Poly1305

The exact `chacha20poly1305 0.11.0` release manifest confirms XChaCha20-Poly1305 support and shows that its default features include `getrandom`. The selection deliberately disables defaults and enables only `alloc` plus `zeroize`, so nonce generation remains an explicit Himsat responsibility rather than an implicit package helper.

### HKDF-SHA-256

The exact `hkdf 0.13.0` manifest depends on generic `hmac` and does not itself freeze a digest. Therefore `sha2 0.11.0` is an explicit direct selection and the implementation contract requires `Hkdf::<Sha256>` semantics. This closes an otherwise implicit algorithm-selection gap.

### Argon2id feature minimization and lockfile nuance

The exact `argon2 0.6.0` manifest shows that default features would enable `alloc`, `getrandom`, and `password-hash`. The selected posture disables defaults and enables only `alloc` and `zeroize`.

The isolated Cargo resolver later proved an important distinction:

```text
ARGON2_ACTIVE_FEATURES = alloc,zeroize
ACTIVE_NORMAL_BUILD_TREE(password-hash,phc) = NO
CARGO_LOCK_PRESENT(password-hash,phc) = YES
```

Cargo recorded `password-hash 0.6.1` and `phc 0.6.1` in the candidate lockfile because optional dependency resolution is represented in `Cargo.lock`, even though those packages were not in the selected normal/build tree. Himsat therefore must provenance-register them if they remain in the canonical lockfile, while making no claim that the PHC surface is active or used.

This preserves both supply-chain truth and implementation-surface truth.

## P002 challenge

### Initial SQLCipher source identity

The `rusqlite v0.40.1` release commit is `6d3c282dc5531a57eb4e22ece3207f00c95d0fb0`. At that exact source revision:

- the workspace path version of `libsqlite3-sys` is `0.38.1`;
- `upgrade_sqlcipher.sh` sets `SQLCIPHER_VERSION="4.14.0"`;
- SQLCipher annotated tag `v4.14.0` resolves to upstream commit `778ab890cfc30c3631212dcceb0295498abdcd3e`;
- the vendored SQLCipher `sqlite3.h` reports embedded SQLite `3.51.3`, source id `737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1`.

This confirms that the SQLCipher path must not use the ordinary bundled-SQLite version as its evidence identity.

### Resolver contradiction

The isolated resolver was intentionally run before canonical dependency adoption. Exact evidence:

```text
RESEARCH_HEAD = 81ecd875a833dfe24694e81210ea3f13e40ea108
WORKFLOW = 34062931896
JOB = 101566588043
CARGO = 1.98.1 (797e8a9bc 2026-08-05)
RUSQLITE_REQUEST = =0.40.1
RESOLVED_LIBSQLITE3_SYS = 0.38.2
RESOLVED_LIBSQLITE3_SYS_CHECKSUM = f1d20bef17f513b9b3004532233187769cd072d790971f4e4da0e346eb6401e8
```

That contradicted the original `0.38.1` expectation. The contradiction was treated as blocking P002/P003 progression; Himsat did not insert the resolved graph into canonical `Cargo.lock`, did not register it after the fact, and did not silently reinterpret the original selection.

### 0.38.2 upstream reconciliation

Upstream facts for the exact resolved sys crate were then re-verified:

```text
LIBSQLITE3_SYS_VERSION = 0.38.2
LIBSQLITE3_SYS_RELEASE_SOURCE_REVISION = e88f112bef7899234a497baed5cc3c3d553deeb8
RUSQLITE_TAG_V0_40_2 = e88f112bef7899234a497baed5cc3c3d553deeb8
SQLCIPHER_VERSION_AT_0_38_2 = 4.14.0
SQLCIPHER_EMBEDDED_SQLITE_VERSION_AT_0_38_2 = 3.51.3
SQLCIPHER_EMBEDDED_SQLITE_SOURCE_ID_AT_0_38_2 = 737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1
```

GitHub compare:

```text
6d3c282dc5531a57eb4e22ece3207f00c95d0fb0
...
e88f112bef7899234a497baed5cc3c3d553deeb8
```

shows no changed file under `libsqlite3-sys/sqlcipher/*`.

The compare does change `libsqlite3-sys/build.rs` by 32 additions and 3 deletions. The relevant delta introduces a local `cfg_select!` compatibility macro and adjusts import ordering/conditional-expression syntax. The selected `bundled-sqlcipher-vendored-openssl` build/provider branch, SQLCipher compile definitions, environment controls, and SQLCipher source identity remain materially the same for this decision.

Therefore `0.38.2` is accepted as the corrected exact sys-crate selection rather than forcing `0.38.1` or blindly following future compatible versions.

### Anti-drift correction

Canonical adoption must exact-pin both:

```text
rusqlite = =0.40.1
libsqlite3-sys = =0.38.2
```

with the selected `bundled-sqlcipher-vendored-openssl` feature posture.

The direct sys-crate dependency is a resolver/provenance constraint, not authorization for application-level raw FFI use. If a later resolver does not reproduce exact source/checksum/native identities, P002 reopens.

### Provider determinism

The `libsqlite3-sys` build logic proves that plain `bundled-sqlcipher` is environment/target dependent:

- explicit OpenSSL environment paths can select a host library;
- Apple may fall back to Security/CommonCrypto;
- non-Apple Unix links `crypto` when no explicit OpenSSL path is selected;
- Windows without the vendored feature requires an external OpenSSL installation.

Therefore plain `bundled-sqlcipher` fails the P002 deterministic-provider objective.

`bundled-sqlcipher-vendored-openssl` remains selected because it routes the SQLCipher crypto provider through the Cargo-resolved `openssl-sys` vendored source path instead of silently selecting target-installed crypto libraries.

### Known compiled legacy-provider feature

The isolated resolver confirmed:

```text
openssl-sys 0.9.117 features = openssl-src,vendored
openssl-src 300.6.1+3.6.3 features = default,legacy
```

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

The repository policy allows `Apache-2.0`, `BSD-3-Clause`, `MIT`, and related permissive licenses. The selected top-level sources appear compatible with that policy:

- RustCrypto selected crates: MIT OR Apache-2.0;
- `getrandom`: MIT OR Apache-2.0;
- `rusqlite` / `libsqlite3-sys`: MIT;
- SQLCipher: BSD-3-Clause-style three-clause license at exact upstream revision;
- `openssl-sys`: MIT;
- `openssl-src`: MIT/Apache-2.0 source wrapper;
- upstream OpenSSL 3.6.3: Apache-2.0.

The isolated resolver found 40 external lockfile packages and captured their crates.io checksums and metadata license expressions. This is not P004 completion. Every package still requires immutable source revision/path and controlling license/notice evidence before adoption.

In particular, a permissive disjunct in a multi-license expression may be selected only after the exact package's license files/terms are verified; metadata strings alone are not final legal/provenance evidence.

## Challenge disposition

```text
P001_SELECTION = ACCEPT
P002_ORIGINAL_LIBSQLITE3_SYS_0_38_1_EXPECTATION = SUPERSEDED_BY_RESOLVER_TRUTH
P002_CORRECTED_LIBSQLITE3_SYS_0_38_2_SELECTION = ACCEPT
P002_EXACT_DIRECT_PIN_REQUIRED_AT_ADOPTION = YES
P003_PACKAGE_VERSION_CHECKSUM_DISCOVERY = COMPLETE_FOR_ISOLATED_CANDIDATE_GRAPH
P003_SOURCE_REVISION_CLOSURE = PENDING
P004_LICENSE_NOTICE_CLOSURE = PENDING
P005_P006 = REQUIRED_BEFORE_ADOPTION
KNOWN_UNRESOLVED_BLOCKER_AT_SELECTION_LAYER = NONE
DEPENDENCY_BYTES_ADOPTED = NO
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```

The next leaf must resolve and prove the complete immutable source/license closure. Any resolver drift from the exact identities selected here reopens the affected selection instead of being silently accepted.
