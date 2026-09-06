# Specification 004P — Package Source and License Closure

## Purpose

This file canonicalizes the immutable source-revision/path and license/notice evidence required by P003/P004 for the already-selected Specification 004P candidate dependency graph. It does not adopt dependency bytes, change the canonical Cargo manifest/lockfile, write provenance registry entries, regenerate notices/SBOM, or authorize Specification 004B implementation.

## Exact research evidence

```text
CANONICAL_BASE_AT_RESEARCH = b2eed59b582fdb783613e84974ac97dba4bb14e2
RESEARCH_BRANCH = research/004p-p003-source-license-closure
FINAL_RESEARCH_HEAD = ed79501203606e7cecca5818bbd808381a448791
FINAL_WORKFLOW_RUN = 34066909709
FINAL_JOB = 101577166415
FINAL_JOB_CONCLUSION = SUCCESS
RUNNER_OS = Ubuntu 24.04.4 LTS
RUNNER_IMAGE = ubuntu-24.04 / 20260831.293.1
RUSTC = 1.98.1 (48a229cea 2026-09-01)
CARGO = 1.98.1 (797e8a9bc 2026-08-05)
OPENSSL_RUST_USE_NASM = 0
```

The research manifest exact-pinned the selected direct packages and exact-pinned both `rusqlite = 0.40.1` and `libsqlite3-sys = 0.38.2` with `bundled-sqlcipher-vendored-openssl`.

The collector fail-closed on package count, Cargo source/checksum identity, immutable upstream VCS revision/path, package repository metadata, license expression, packaged license evidence, and a concrete MIT adoption branch for each Cargo package.

## Resolver-count correction

Earlier Specification 004P prose incorrectly interpreted Cargo's:

```text
Locking 41 packages to latest Rust 1.98.1 compatible versions
```

as 40 external packages plus the local research package. The exact resolver and closure logs prove instead:

```text
EXTERNAL_REGISTRY_PACKAGES = 41
LOCAL_RESEARCH_PACKAGE = 1 additional local package
```

The existing checksum table already contained 41 external rows. Any earlier statement that the candidate closure contained 40 external packages is superseded by this evidence.

## Complete 41-package source/license closure

For each Cargo package, `Adoption basis = MIT` selects a permissive branch that is `allow` under `governance/provenance/policy.json`. The public expression remains recorded rather than being rewritten to MIT.

| Package | Version | crates.io checksum | Repository | Immutable revision | Source path | Public expression | Adoption basis | License evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `aead` | `0.6.1` | `1973cfbc1a2daf9cf550e74e1f088c28e7f7d8c1e1418fb6c9dc5184b7e84c99` | `https://github.com/RustCrypto/traits` | `3d2d90045bffc402af4edb6a5a4dbb1e217329d1` | `aead` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:33b32a251d445c5c03a634e64c53314c55540fc367fabdd45d9b6c8f260c028c` |
| `argon2` | `0.6.0` | `134c52ddac6d63c576bef8168db10c83c49c26444ecbc68060fef078925a901c` | `https://github.com/RustCrypto/password-hashes` | `b1e0ad6fe229b1ba74e4696c7359ab45d7e931f0` | `argon2` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:57a4d75c1d095bb94754a478f3cd15e60ac66a5c5627d844be01a918eea5fe88` |
| `base64ct` | `1.8.3` | `2af50177e190e07a26ab74f8b1efbfe2ef87da2116221318cb1c2e82baf7de06` | `https://github.com/RustCrypto/formats` | `9adf88fe3e6e0fb9f8cf20b54747aff67a3eca6e` | `base64ct` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:2d1c57bff28344b9e698f51063bc8509799cc4c99a4e0cf2aa3f7e7c3e1f9a9d` |
| `bitflags` | `2.13.1` | `b588b76d00fde79687d7646a9b5bdf3cc0f655e0bbd080335a95d7e96f3587da` | `https://github.com/bitflags/bitflags` | `f92a2921b41644b02ca5d50a6ace542e309e6a6f` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` |
| `blake2` | `0.11.0` | `5b5d4d889834ee8ecfc0f8426ad30faf7cdcb10f741a8e6d7224d95325479f6f` | `https://github.com/RustCrypto/hashes` | `fa3084083d946ac12436567d5a59c0935d5db1fa` | `blake2` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:3721e5764ca384059a3521d84c0fb40373894a099d8246fe76bf0d7418041846` |
| `block-buffer` | `0.12.1` | `d2f6c7dbe95a6ed67ad9f18e57daf93a2f034c524b99fd2b76d18fdfeb6660aa` | `https://github.com/RustCrypto/utils` | `cbd0963c685df025c42bed31f78c50d1bada3805` | `block-buffer` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:98181e7249d0c01737645ec982499ce99a0f07eb8f7d625b8840d799d10dbc01` |
| `cc` | `1.4.5` | `005ec2760ca554fae18df7a11195552ec576cd665632a881bc011d5bb2fd4d80` | `https://github.com/rust-lang/cc-rs` | `171f8f64698226b88c1971afc50a4b7fcdd3ff74` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` |
| `cfg-if` | `1.0.4` | `9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801` | `https://github.com/rust-lang/cfg-if` | `3510ca6abea34cbbc702509a4e50ea9709925eda` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` |
| `chacha20` | `0.10.2` | `65c35e4b699c7e15ccbe7ee35c005e4fc0a278d22238a2857e6ce2dadeda1b06` | `https://github.com/RustCrypto/stream-ciphers` | `6b236b758a0279f64d777797514813b2cb572c8b` | `chacha20` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:b8c6939380a400f53e11923d50fcc4dd2fa1ba8339fd9d04cda38a0251b6c9b0` |
| `chacha20poly1305` | `0.11.0` | `9b89e1c441e926b9c82a8d023f6e1b7ae0adcfaa7d621814e4d60789bac751cb` | `https://github.com/RustCrypto/AEADs` | `e37a978ccf0992d9053fbc039470d6527108e393` | `chacha20poly1305` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:b8c6939380a400f53e11923d50fcc4dd2fa1ba8339fd9d04cda38a0251b6c9b0` |
| `cipher` | `0.5.2` | `e8cf2a2c93cd704877c0858356ed03480ff301ee950b43f1cbe4573b088bfa6c` | `https://github.com/RustCrypto/traits` | `f836f71fadb3b975b577a293d57851d044f0a44b` | `cipher` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:950d712c518a02fcb7cff96950aee304a1cc5283361712c980ea21e8d6d669a5` |
| `cmov` | `0.5.4` | `0c9ea0ac24bc397ab3c98583a3c9ba74fa56b09a4449bbe172b9b1ddb016027a` | `https://github.com/RustCrypto/utils` | `5c7e4f9bb31af81bf766360e836b6d633b84dbff` | `cmov` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:70c9d40f1f9545c3f133b8a67206e89da850f6468eed072281bb3701514114a9` |
| `cpufeatures` | `0.3.1` | `5ca28b0ae3115b884660db4118d803791fd6756b6e88f39c0f3f7859060d7566` | `https://github.com/RustCrypto/utils` | `e3ac92bfd33051e146025baa2d0ce79891b7fc73` | `cpufeatures` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:73b9dc2e79c7308998dd30296e073aefaefb944a68fb89aa412c23c0edcabcaa` |
| `crypto-common` | `0.2.2` | `ce6e4c961d6cd6c9a86db418387425e8bdeaf05b3c8bc1411e6dca4c252f1453` | `https://github.com/RustCrypto/traits` | `93dee26c6bde3741a197f1c5f6b7baac277705f3` | `crypto-common` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:d2e7ec5355c96eeade56b09187ceb48a6a30299da3ce7531a66d3d11405ab963` |
| `ctutils` | `0.4.2` | `7d5515a3834141de9eafb9717ad39eea8247b5674e6066c404e8c4b365d2a29e` | `https://github.com/RustCrypto/utils` | `53f7fc3fa806e6d4e9650675e7b97d3621cff340` | `ctutils` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:91585c36e4fb9ab4ca0d3dfac5d66d3c0c62cc51f640a0e1196542daf2267eae` |
| `digest` | `0.11.3` | `f1dd6dbb5841937940781866fa1281a1ff7bd3bf827091440879f9994983d5c2` | `https://github.com/RustCrypto/traits` | `2fb9ed8922e244117040bb037a7d141a6a2b8228` | `digest` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:af59cea35d7f5e2777a713b8d155d65efa2c339eb43f3c14e868c6ac8506edad` |
| `fallible-iterator` | `0.3.0` | `2acce4a10f12dc2fb14a218589d4f1f62ef011b2d0cc4b3cb1bba8e94da14649` | `https://github.com/sfackler/rust-fallible-iterator` | `11cfa0558045e52bdb473c9eab6a1eb35a54e302` | `.` | `MIT/Apache-2.0` | `MIT` | `LICENSE-MIT sha256:0816e154b159ba255c563f7c8c7df5bbb8cc5fc96f5ab8cf9f4743b4f41fe7eb` |
| `fallible-streaming-iterator` | `0.1.9` | `7360491ce676a36bf9bb3c56c1aa791658183a54d2744120f27285738d90465a` | `https://github.com/sfackler/fallible-streaming-iterator` | `9217bc5e381b54b4ef4c38959488a5dc993b7b81` | `.` | `MIT/Apache-2.0` | `MIT` | `LICENSE-MIT sha256:8dcec5569a9be5b0e086c80faed6f1aefa670af0ec29cecc2f714303096887e0` |
| `find-msvc-tools` | `0.1.12` | `3e0f1c7c3a72c66fd80abe965175f7523475c0489a87d3ff9d6e8c87d87a9d2d` | `https://github.com/rust-lang/cc-rs` | `171f8f64698226b88c1971afc50a4b7fcdd3ff74` | `find-msvc-tools` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` |
| `getrandom` | `0.4.3` | `300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099` | `https://github.com/rust-random/getrandom` | `5e7cd5733536844a9856dc7259bd4696bbe5e3ae` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:523a42c25d245dde9c015f882cec7f4555aad883382a6cf19b4b7d9b2cd5419b` |
| `hkdf` | `0.13.0` | `4aaa26c720c68b866f2c96ef5c1264b3e6f473fe5d4ce61cd44bbe913e553018` | `https://github.com/RustCrypto/KDFs/` | `bfb3b209abeeaa02277935b167e06bac320b2773` | `hkdf` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:d288f9c9b4590446ec18c22ead8f8b5a12a3d4025b68f62dc9015063eb9cca69` |
| `hmac` | `0.13.0` | `6303bc9732ae41b04cb554b844a762b4115a61bfaa81e3e83050991eeb56863f` | `https://github.com/RustCrypto/MACs` | `0236c8eb50098dd7f277a71ab89caaeb1e7314df` | `hmac` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:9e0dfd2dd4173a530e238cb6adb37aa78c34c6bc7444e0e10c1ab5d8881f63ba` |
| `hybrid-array` | `0.4.14` | `707114b52a152fa7bdb290cd7cd5912d9467273b6d74e21b8d81aca1f8533f6b` | `https://github.com/RustCrypto/hybrid-array` | `1b65ce708c931d27b2a17400d0dad92500f23f30` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:70c9d40f1f9545c3f133b8a67206e89da850f6468eed072281bb3701514114a9` |
| `inout` | `0.2.2` | `4250ce6452e92010fdf7268ccc5d14faa80bb12fc741938534c58f16804e03c7` | `https://github.com/RustCrypto/utils` | `2d5eac49c253b19a064b33fc1f9ca7839732cb30` | `inout` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:a07fcacc3c60de4dc0fab10ac9d6aaba7379974e28451c99da7f7df09c25b28c` |
| `libc` | `0.2.189` | `3eaf3ede3fee6db1a4c2ee091bf8a8b4dccdc6d17f656fb07896ee72867612f2` | `https://github.com/rust-lang/libc` | `ef0906e20828777175f65caa7e681a0ce33c559a` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:123a331b5dbf04c30097fa43b8f858bc85df671fe776de498d01f3d6b7c1f69e` |
| `libsqlite3-sys` | `0.38.2` | `f1d20bef17f513b9b3004532233187769cd072d790971f4e4da0e346eb6401e8` | `https://github.com/rusqlite/rusqlite` | `e88f112bef7899234a497baed5cc3c3d553deeb8` | `libsqlite3-sys` | `MIT` | `MIT` | `LICENSE sha256:c10c1f27337546471e5f7e4e97fdd398b35b9d4e126115dcd22de8d8e65abf6f` |
| `openssl-src` | `300.6.1+3.6.3` | `46eb8fb9fb3b61ce1c0f8a026c4c1a0714d3a9e138e7fbde78753ce2babc3846` | `https://github.com/alexcrichton/openssl-src-rs` | `64c38cc48205400720199476aff8a780f98d167d` | `.` | `MIT/Apache-2.0` | `MIT` | `LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` |
| `openssl-sys` | `0.9.117` | `b47e7e6bb2c38cd930d25a23b40fa52e068c10e85f3e03a7f5ba5aaca5713695` | `https://github.com/rust-openssl/rust-openssl` | `db9c9e2f5db2ad7b45fd894e8d297ee15bfd0c7c` | `openssl-sys` | `MIT` | `MIT` | `LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` |
| `password-hash` | `0.6.1` | `aab41826031698d6ffcd9cff78ef56ef998e39dc7e5067cdfebe373842d4723b` | `https://github.com/RustCrypto/traits` | `d1954d88cbde6b6ee839d3a2f78e5b03fd4eaa1d` | `password-hash` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:d8c4ab431f20da8452c9a217084163822e5fd9d134e1343903c929f2859fc8fd` |
| `phc` | `0.6.1` | `44dc769b75f93afdddd8c7fa12d685292ddeff1e66f7f0f3a234cf1818afe892` | `https://github.com/RustCrypto/formats` | `02a43929d75c8f7a515dcc240648c26d2fe4ee5f` | `phc` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:57a4d75c1d095bb94754a478f3cd15e60ac66a5c5627d844be01a918eea5fe88` |
| `pkg-config` | `0.3.34` | `f6b464fbc74e149a392436b17d523f769e057cb6877f6a5c4618bc6f11800548` | `https://github.com/rust-lang/pkg-config-rs` | `a8852cc148f8f7d504bf3a62444c298dcd3637f0` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` |
| `poly1305` | `0.9.1` | `6e2d0073b297041425c7c3df6eb4792d598a15323fe63346852b092eca02904c` | `https://github.com/RustCrypto/universal-hashes` | `4f5691919d96ec0089ecca4492be7f1848c78fdf` | `poly1305` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:b67acfaaf787b346e1d3bf7654b4fabfd20360fdeb4351cc5e9d624147824527` |
| `r-efi` | `6.0.0` | `f8dcc9c7d52a811697d2151c701e0d08956f92b0e24136cf4cf27b57a6a0d9bf` | `https://github.com/r-efi/r-efi` | `7e1b0322d31d625f81a5656096330934f9cd835d` | `.` | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` | `MIT` | `AUTHORS sha256:d027e91dbc9cdbb2f1190068e498bd6b61cff022b6a032b191021ba658d96111` |
| `rusqlite` | `0.40.1` | `11438310b19e3109b6446c33d1ed5e889428cf2e278407bc7896bc4aaea43323` | `https://github.com/rusqlite/rusqlite` | `6d3c282dc5531a57eb4e22ece3207f00c95d0fb0` | `.` | `MIT` | `MIT` | `LICENSE sha256:c10c1f27337546471e5f7e4e97fdd398b35b9d4e126115dcd22de8d8e65abf6f` |
| `sha2` | `0.11.0` | `446ba717509524cb3f22f17ecc096f10f4822d76ab5c0b9822c5f9c284e825f4` | `https://github.com/RustCrypto/hashes` | `ffe093984c004769747e998f77da8ff7c0e7a765` | `sha2` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:831e0f43ad0bf014c1c4fec5767aae470434c1d66d6e671be2d823e729063e25` |
| `shlex` | `2.0.1` | `f8fadd59c855ef2080decdef8ff161eb6661b86933c9d82e5ba29dc602a55aba` | `https://github.com/comex/rust-shlex` | `e82b1411beb7c92871c2c078c9ab415bbcf207ef` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:4455bf75a91154108304cb283e0fea9948c14f13e20d60887cf2552449dea3b1` |
| `smallvec` | `1.16.0` | `b9be42f50aa861c555654aa3a37f52f4b1074bacf4e48fe0ef7fa584e80f1f0f` | `https://github.com/servo/rust-smallvec` | `aa22a8ff83108228b6d941b83fc91399267d72c9` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:0b28172679e0009b655da42797c03fd163a3379d5cfa67ba1f1655e974a2a1a9` |
| `typenum` | `1.20.1` | `b6f5e870be6c3b371b77fe0ee0bafb859fa4964b4404c27de1d380043c4dda20` | `https://github.com/paholg/typenum` | `0db9a0f731981f29266b63586c29fa07e4477b1a` | `.` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:a825bd853ab71619a4923d7b4311221427848070ff44d990da39b0b274c1683f` |
| `universal-hash` | `0.6.1` | `f4987bdc12753382e0bec4a65c50738ffaabc998b9cdd1f952fb5f39b0048a96` | `https://github.com/RustCrypto/traits` | `82279a5a9ff2af5f10194b9147fe60050cda1851` | `universal-hash` | `MIT OR Apache-2.0` | `MIT` | `LICENSE-MIT sha256:efa52eb70a774b62c50cf50f5e57e2625c29d375d09b91132f4f020e47b9944e` |
| `vcpkg` | `0.2.15` | `accd4ea62f7bb7a82fe23066fb0957d48ef677f6eeb8215f372f52e48bb32426` | `https://github.com/mcgoo/vcpkg-rs` | `e7d37e095ef2aa85c80242aa3850d442832b7b54` | `.` | `MIT/Apache-2.0` | `MIT` | `LICENSE-MIT sha256:016d20f335060a70e79d9fcf8dfaa6201114d65d592211f4bdfb8ae9ca2bc1dc` |
| `zeroize` | `1.9.0` | `e13c156562582aa81c60cb29407084cdb54c4164760106ab78e6c5b0858cf64e` | `https://github.com/RustCrypto/utils` | `0b715735a660a8566ccd240bf42489fe2ed98efb` | `zeroize` | `Apache-2.0 OR MIT` | `MIT` | `LICENSE-MIT sha256:8c7516d4b27b1e495be5e38b612298b63de48d05f49cdac94f70f3cd70f8864b` |

## Explicit exception closure

### `fallible-streaming-iterator 0.1.9`

The published crate lacks `.cargo_vcs_info.json`, so the collector originally failed rather than inventing a revision. Upstream annotated repository evidence closes the gap:

```text
repository = https://github.com/sfackler/fallible-streaming-iterator
version tag = v0.1.9
immutable revision = 9217bc5e381b54b4ef4c38959488a5dc993b7b81
source path = .
public license = MIT/Apache-2.0
selected adoption basis = MIT
```

The packaged crate contains `LICENSE-MIT` with SHA-256 `8dcec5569a9be5b0e086c80faed6f1aefa670af0ec29cecc2f714303096887e0`.

### `r-efi 6.0.0`

The package metadata records `MIT OR Apache-2.0 OR LGPL-2.1-or-later`, but the packaged crate does not contain a top-level file named `LICENSE*`. The collector failed until the exact packaged `AUTHORS` file was explicitly inspected. That file contains the complete MIT license text and copyright list.

```text
repository = https://github.com/r-efi/r-efi
immutable revision = 7e1b0322d31d625f81a5656096330934f9cd835d
source path = .
public license expression = MIT OR Apache-2.0 OR LGPL-2.1-or-later
selected adoption basis = MIT
MIT evidence = AUTHORS
MIT evidence sha256 = d027e91dbc9cdbb2f1190068e498bd6b61cff022b6a032b191021ba658d96111
```

This does not reclassify LGPL as allowed. It selects the independently offered MIT branch for the exact package bytes.

## Native SQLCipher / SQLite closure

The selected `libsqlite3-sys 0.38.2` package vendors the already-frozen SQLCipher amalgamation identity:

```text
SQLCIPHER_REPOSITORY = https://github.com/sqlcipher/sqlcipher
SQLCIPHER_VERSION = 4.14.0
SQLCIPHER_ANNOTATED_TAG_OBJECT = 46bb08ec73b2caa84b6945a19c9e435fff446dcd
SQLCIPHER_SOURCE_REVISION = 778ab890cfc30c3631212dcceb0295498abdcd3e
SQLCIPHER_LICENSE = BSD-3-Clause
SQLCIPHER_LICENSE_FILE = LICENSE.md
SQLCIPHER_LICENSE_GIT_BLOB = 3f71443161b6ff424ca4f335e910469f46ed4c44
SQLITE_LICENSE_FILE = SQLITE_LICENSE.md
SQLITE_LICENSE_GIT_BLOB = 4029dc9e7f25d12adfa0165542d702208276913a
SQLCIPHER_EMBEDDED_SQLITE_VERSION = 3.51.3
SQLCIPHER_EMBEDDED_SQLITE_SOURCE_ID = 737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1
```

`LICENSE.md` at the exact SQLCipher source revision contains the three-clause BSD redistribution/notice/no-endorsement terms. `BSD-3-Clause` is `allow` under Himsat provenance policy.

`SQLITE_LICENSE.md` at that same revision states that the SQLite source code and the code that becomes `sqlite3.c` / `sqlite3.h` is public domain, while build/configuration helper exceptions use permissive BSD-style terms. P005/P006 must preserve the exact SQLCipher and SQLite notices relevant to the vendored distribution rather than collapsing them into the `libsqlite3-sys` wrapper's MIT license.

## Native OpenSSL closure

The selected Cargo provider chain is:

```text
openssl-sys 0.9.117
  -> openssl-src 300.6.1+3.6.3
     -> OpenSSL 3.6.3 source
```

Exact identities:

```text
OPENSSL_SYS_REVISION = db9c9e2f5db2ad7b45fd894e8d297ee15bfd0c7c
OPENSSL_SRC_WRAPPER_REVISION = 64c38cc48205400720199476aff8a780f98d167d
OPENSSL_UPSTREAM_REPOSITORY = https://github.com/openssl/openssl
OPENSSL_UPSTREAM_REVISION = aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f
OPENSSL_UPSTREAM_VERSION = 3.6.3
OPENSSL_UPSTREAM_LICENSE = Apache-2.0
OPENSSL_UPSTREAM_LICENSE_FILE = LICENSE.txt
```

The exact OpenSSL `LICENSE.txt` is Apache-2.0. `Apache-2.0` is `allow` under Himsat provenance policy. No FIPS claim is created by this closure.

The `openssl-src` selected feature graph still compiles the `legacy` provider surface. P003/P004 closure records the source/license truth only; runtime qualification must still distinguish compiled provider surface from the provider actually used by SQLCipher.

## Policy disposition

```text
CARGO_PACKAGE_COUNT = 41
CARGO_PACKAGE_SELECTED_LICENSE = MIT for all 41
MIT_POLICY_DISPOSITION = allow
SQLCIPHER_LICENSE = BSD-3-Clause
SQLCIPHER_POLICY_DISPOSITION = allow
OPENSSL_UPSTREAM_LICENSE = Apache-2.0
OPENSSL_POLICY_DISPOSITION = allow
MANUAL_LICENSE_DECISION_REQUIRED_FOR_SELECTED_CLOSURE = NO
DENIED_LICENSE_PRESENT_IN_SELECTED_CLOSURE = NO
UNKNOWN_LICENSE_PRESENT_IN_SELECTED_CLOSURE = NO
```

Public multi-license expressions remain factual evidence. Selecting MIT for adoption is not a rewrite of upstream license metadata.

## P003/P004 disposition and remaining authority

This file closes the selected candidate graph's immutable source/version/path/checksum and controlling license/notice discovery only.

```text
P001 = SELECTED_NOT_ADOPTED
P002 = CORRECTED_SELECTED_NOT_ADOPTED
P003_PACKAGE_VERSION_CHECKSUM_DISCOVERY = COMPLETE
P003_SOURCE_REVISION_PATH_CLOSURE = COMPLETE
P004_LICENSE_NOTICE_CLOSURE = COMPLETE
P005_REGISTRY_SBOM_NOTICES_ADOPTION = PENDING
P006_CANONICAL_LOCKFILE_NATIVE_CLOSURE_PROOF = PENDING
DEPENDENCY_BYTES_ADOPTED = NO
CANONICAL_CARGO_MANIFEST_CHANGED = NO
CANONICAL_CARGO_LOCK_CHANGED = NO
PROVENANCE_REGISTRY_CHANGED = NO
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```

P005/P006 must reproduce this exact graph from the canonical manifest and lockfile. Any package/version/checksum/source-revision/native-source difference is a blocking new fact that reopens the affected P001-P004 selection/closure rather than being silently accepted.
