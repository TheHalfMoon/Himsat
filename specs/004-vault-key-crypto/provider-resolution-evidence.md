# Specification 004P — Resolver Evidence

## Purpose

This file records isolated Cargo resolver evidence for the selected Specification 004P crypto/SQLCipher strategy. It is research evidence only until canonicalized through the normal exact-head merge gate. It does not adopt dependency bytes into Himsat's canonical workspace.

## Exact research execution

```text
CANONICAL_BASE_AT_RESEARCH = c054566e9015ac7d05c575b090a7ecfe7bd8e418
RESEARCH_BRANCH = research/004p-cargo-resolution
RESEARCH_HEAD = 81ecd875a833dfe24694e81210ea3f13e40ea108
WORKFLOW_RUN = 34062931896
JOB = 101566588043
JOB_CONCLUSION = SUCCESS
RUNNER_OS = Ubuntu 24.04.4 LTS
RUNNER_IMAGE = ubuntu-24.04 / 20260831.293.1
RUSTC = 1.98.1 (48a229cea 2026-09-01)
CARGO = 1.98.1 (797e8a9bc 2026-08-05)
OPENSSL_RUST_USE_NASM = 0
```

The isolated manifest exact-pinned the selected direct packages and used `rusqlite = 0.40.1` with `bundled-sqlcipher-vendored-openssl`. The resolver ran outside the canonical workspace dependency graph.

Cargo reported:

```text
Locking 41 packages to latest Rust 1.98.1 compatible versions
Adding rusqlite v0.40.1 (available: v0.40.2)
```

One package is the local research resolver itself. The resulting external lockfile closure contains 40 packages.

## Resolver drift discovered

The original P002 selection expected `libsqlite3-sys 0.38.1` because that is the version at the `rusqlite v0.40.1` release commit. Actual Cargo resolution on 2026-09-06 selected:

```text
libsqlite3-sys = 0.38.2
checksum = f1d20bef17f513b9b3004532233187769cd072d790971f4e4da0e346eb6401e8
```

This was treated as a blocking selection contradiction until independently reconciled. No canonical dependency bytes were adopted while the contradiction was open.

Upstream reconciliation established:

```text
libsqlite3-sys 0.38.2 release source revision = e88f112bef7899234a497baed5cc3c3d553deeb8
rusqlite tag v0.40.2 = e88f112bef7899234a497baed5cc3c3d553deeb8
SQLCipher generator version = 4.14.0
SQLCipher source files changed from 0.38.1 to 0.38.2 = NO
SQLCipher embedded SQLite version = 3.51.3
SQLCipher embedded SQLite source id = 737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1
```

GitHub compare `6d3c282dc5531a57eb4e22ece3207f00c95d0fb0...e88f112bef7899234a497baed5cc3c3d553deeb8` contains no `libsqlite3-sys/sqlcipher/*` file change. The `libsqlite3-sys/build.rs` delta is a local `cfg_select!` compatibility macro/refactor plus import ordering; the SQLCipher build/provider branch and SQLCipher compile definitions remain materially unchanged for the selected feature.

The corrected P002 strategy therefore accepts exact `libsqlite3-sys 0.38.2` and requires a direct exact version constraint at adoption time so future resolver runs cannot silently move the selected sys crate.

## Exact external lockfile closure

Every package below was present in the isolated `Cargo.lock`. P005/P006 must register and match the complete canonical lockfile closure, including packages present because Cargo records optional dependency possibilities even when they are not active in the selected normal/build feature graph.

| Package | Version | crates.io SHA-256 checksum |
| --- | --- | --- |
| `aead` | 0.6.1 | `1973cfbc1a2daf9cf550e74e1f088c28e7f7d8c1e1418fb6c9dc5184b7e84c99` |
| `argon2` | 0.6.0 | `134c52ddac6d63c576bef8168db10c83c49c26444ecbc68060fef078925a901c` |
| `base64ct` | 1.8.3 | `2af50177e190e07a26ab74f8b1efbfe2ef87da2116221318cb1c2e82baf7de06` |
| `bitflags` | 2.13.1 | `b588b76d00fde79687d7646a9b5bdf3cc0f655e0bbd080335a95d7e96f3587da` |
| `blake2` | 0.11.0 | `5b5d4d889834ee8ecfc0f8426ad30faf7cdcb10f741a8e6d7224d95325479f6f` |
| `block-buffer` | 0.12.1 | `d2f6c7dbe95a6ed67ad9f18e57daf93a2f034c524b99fd2b76d18fdfeb6660aa` |
| `cc` | 1.4.5 | `005ec2760ca554fae18df7a11195552ec576cd665632a881bc011d5bb2fd4d80` |
| `cfg-if` | 1.0.4 | `9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801` |
| `chacha20` | 0.10.2 | `65c35e4b699c7e15ccbe7ee35c005e4fc0a278d22238a2857e6ce2dadeda1b06` |
| `chacha20poly1305` | 0.11.0 | `9b89e1c441e926b9c82a8d023f6e1b7ae0adcfaa7d621814e4d60789bac751cb` |
| `cipher` | 0.5.2 | `e8cf2a2c93cd704877c0858356ed03480ff301ee950b43f1cbe4573b088bfa6c` |
| `cmov` | 0.5.4 | `0c9ea0ac24bc397ab3c98583a3c9ba74fa56b09a4449bbe172b9b1ddb016027a` |
| `cpufeatures` | 0.3.1 | `5ca28b0ae3115b884660db4118d803791fd6756b6e88f39c0f3f7859060d7566` |
| `crypto-common` | 0.2.2 | `ce6e4c961d6cd6c9a86db418387425e8bdeaf05b3c8bc1411e6dca4c252f1453` |
| `ctutils` | 0.4.2 | `7d5515a3834141de9eafb9717ad39eea8247b5674e6066c404e8c4b365d2a29e` |
| `digest` | 0.11.3 | `f1dd6dbb5841937940781866fa1281a1ff7bd3bf827091440879f9994983d5c2` |
| `fallible-iterator` | 0.3.0 | `2acce4a10f12dc2fb14a218589d4f1f62ef011b2d0cc4b3cb1bba8e94da14649` |
| `fallible-streaming-iterator` | 0.1.9 | `7360491ce676a36bf9bb3c56c1aa791658183a54d2744120f27285738d90465a` |
| `find-msvc-tools` | 0.1.12 | `3e0f1c7c3a72c66fd80abe965175f7523475c0489a87d3ff9d6e8c87d87a9d2d` |
| `getrandom` | 0.4.3 | `300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099` |
| `hkdf` | 0.13.0 | `4aaa26c720c68b866f2c96ef5c1264b3e6f473fe5d4ce61cd44bbe913e553018` |
| `hmac` | 0.13.0 | `6303bc9732ae41b04cb554b844a762b4115a61bfaa81e3e83050991eeb56863f` |
| `hybrid-array` | 0.4.14 | `707114b52a152fa7bdb290cd7cd5912d9467273b6d74e21b8d81aca1f8533f6b` |
| `inout` | 0.2.2 | `4250ce6452e92010fdf7268ccc5d14faa80bb12fc741938534c58f16804e03c7` |
| `libc` | 0.2.189 | `3eaf3ede3fee6db1a4c2ee091bf8a8b4dccdc6d17f656fb07896ee72867612f2` |
| `libsqlite3-sys` | 0.38.2 | `f1d20bef17f513b9b3004532233187769cd072d790971f4e4da0e346eb6401e8` |
| `openssl-src` | 300.6.1+3.6.3 | `46eb8fb9fb3b61ce1c0f8a026c4c1a0714d3a9e138e7fbde78753ce2babc3846` |
| `openssl-sys` | 0.9.117 | `b47e7e6bb2c38cd930d25a23b40fa52e068c10e85f3e03a7f5ba5aaca5713695` |
| `password-hash` | 0.6.1 | `aab41826031698d6ffcd9cff78ef56ef998e39dc7e5067cdfebe373842d4723b` |
| `phc` | 0.6.1 | `44dc769b75f93afdddd8c7fa12d685292ddeff1e66f7f0f3a234cf1818afe892` |
| `pkg-config` | 0.3.34 | `f6b464fbc74e149a392436b17d523f769e057cb6877f6a5c4618bc6f11800548` |
| `poly1305` | 0.9.1 | `6e2d0073b297041425c7c3df6eb4792d598a15323fe63346852b092eca02904c` |
| `r-efi` | 6.0.0 | `f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf` |
| `rusqlite` | 0.40.1 | `11438310b19e3109b6446c33d1ed5e889428cf2e278407bc7896bc4aaea43323` |
| `sha2` | 0.11.0 | `446ba717509524cb3f22f17ecc096f10f4822d76ab5c0b9822c5f9c284e825f4` |
| `shlex` | 2.0.1 | `f8fadd59c855ef2080decdef8ff161eb6661b86933c9d82e5ba29dc602a55aba` |
| `smallvec` | 1.16.0 | `b9be42f50aa861c555654aa3a37f52f4b1074bacf4e48fe0ef7fa584e80f1f0f` |
| `typenum` | 1.20.1 | `b6f5e870be6c3b371b77fe0ee0bafb859fa4964b4404c27de1d380043c4dda20` |
| `universal-hash` | 0.6.1 | `f4987bdc12753382e0bec4a65c50738ffaabc998b9cdd1f952fb5f39b0048a96` |
| `vcpkg` | 0.2.15 | `accd4ea62f7bb7a82fe23066fb0957d48ef677f6eeb8215f372f52e48bb32426` |
| `zeroize` | 1.9.0 | `e13c156562582aa81c60cb29407084cdb54c4164760106ab78e6c5b0858cf64e` |

## Active selected feature graph vs lockfile closure

Cargo metadata reported `argon2` active features exactly as:

```text
alloc
zeroize
```

The selected `cargo tree --edges normal,build` showed active Argon2 dependencies:

```text
base64ct
blake2
cpufeatures
zeroize
```

It did **not** show `password-hash` or `phc` in the active normal/build tree. Those packages were nevertheless present in the generated lockfile because Cargo lockfiles record optional dependency resolution. Therefore:

```text
LOCKFILE_PRESENT(password-hash, phc) = YES
ACTIVE_SELECTED_NORMAL_BUILD_GRAPH(password-hash, phc) = NO
PROVENANCE_REGISTRATION_REQUIRED(password-hash, phc) = YES
RUNTIME_USAGE_CLAIM(password-hash, phc) = NONE
```

This distinction must be preserved in provenance and security evidence. Himsat must not claim those packages absent from the lockfile, and also must not falsely claim they are compiled/active under the selected feature graph.

## Selected SQLCipher/OpenSSL active graph

The active normal/build tree included:

```text
rusqlite 0.40.1
  -> libsqlite3-sys 0.38.2
     -> openssl-sys 0.9.117
        -> openssl-src 300.6.1+3.6.3
```

Cargo metadata reported:

```text
rusqlite features:
  bundled
  bundled-sqlcipher
  bundled-sqlcipher-vendored-openssl
  modern_sqlite

libsqlite3-sys features:
  bundled
  bundled-sqlcipher
  bundled-sqlcipher-vendored-openssl
  bundled_bindings
  cc
  default
  min_sqlite_version_3_34_1
  openssl-sys
  pkg-config
  vcpkg

openssl-sys features:
  openssl-src
  vendored

openssl-src features:
  default
  legacy
```

The compiled OpenSSL `legacy` provider surface remains explicit evidence. It is not equivalent to proof that SQLCipher actively loads that provider.

## P003/P004 remaining work

This evidence closes exact package-version/checksum discovery for the isolated candidate graph, but P003/P004 are not complete yet.

Still required before adoption:

- immutable upstream source revision/path for every one of the 40 external lockfile packages;
- controlling license/notice evidence for every package;
- explicit native SQLCipher source identity and exact OpenSSL embedded source identity;
- proof that every selected license has `allow` disposition under Himsat policy;
- exact canonical manifest/lockfile graph must reproduce this closure after direct `libsqlite3-sys = 0.38.2` pinning;
- any resolver difference must stop adoption and reopen reconciliation.

## Disposition

```text
P001 = SELECTED_NOT_ADOPTED
P002 = CORRECTED_TO_LIBSQLITE3_SYS_0_38_2_NOT_ADOPTED
P003_PACKAGE_VERSION_CHECKSUM_DISCOVERY = COMPLETE_FOR_ISOLATED_CANDIDATE_GRAPH
P003_SOURCE_REVISION_CLOSURE = PENDING
P004_LICENSE_NOTICE_CLOSURE = PENDING
P005 = BLOCKED
P006 = BLOCKED
004B = BLOCKED
```
