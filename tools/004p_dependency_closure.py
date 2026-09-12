#!/usr/bin/env python3
"""Fail-closed Specification 004P provider plus authorized platform dependency/native closure check."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import sys
import tomllib
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CLOSURE_DOC = ROOT / "specs/004-vault-key-crypto/provider-package-source-license-closure.md"
REMEDIATION_DOC = ROOT / "specs/004-vault-key-crypto/provider-package-source-license-review-remediation.md"
REGISTRY_PATH = ROOT / "governance/provenance/registry.json"
CORE_MANIFEST = ROOT / "crates/himsat-core/Cargo.toml"
LOCK_PATH = ROOT / "Cargo.lock"
REGISTRY_SCHEMA = "himsat.provenance-registry/v2"
REGISTRY_SOURCE = "registry+https://github.com/rust-lang/crates.io-index"
EXPECTED_PROVIDER_EXTERNAL_COUNT = 41
EXPECTED_TOTAL_EXTERNAL_COUNT = 161
EVIDENCE_REFERENCE = "specs/004-vault-key-crypto/provider-package-source-license-review-remediation.md"

EXPECTED_APPLE_TARGET = 'cfg(target_os = "macos")'
EXPECTED_APPLE_DIRECT: dict[str, dict[str, Any]] = {
    "core-foundation": {
        "version": "=0.10.1",
        "default-features": False,
    },
    "security-framework": {
        "version": "=3.7.0",
        "default-features": False,
        "features": ["OSX_10_15"],
    },
}

EXPECTED_LINUX_TARGET = 'cfg(target_os = "linux")'
EXPECTED_LINUX_DIRECT: dict[str, dict[str, Any]] = {
    "secret-service": {
        "version": "=5.2.0",
        "default-features": False,
        "features": ["rt-tokio-crypto-rust"],
    },
}

EXPECTED_WINDOWS_TARGET = 'cfg(target_os = "windows")'
EXPECTED_WINDOWS_DIRECT: dict[str, dict[str, Any]] = {
    "windows-acl": {"version": "=0.3.0"},
    "fs_at": {"version": "=0.2.1"},
    "windows-permissions": {"version": "=0.2.4"},
    "windows-dpapi": {"version": "=0.2.0"},
    "winsafe": {"version": "=0.0.29", "default-features": False, "features": ["advapi"]},
}

EXPECTED_PLATFORM_PACKAGES: dict[tuple[str, str], dict[str, str]] = {
    ("core-foundation", "0.10.1"): {
        "checksum": "b2a6cd9ae233e7f62ba4e9353e81a88df7fc8a5987b8d445b4d90c879bd156f6",
        "repository": "https://github.com/servo/core-foundation-rs",
        "revision": "548b65cd3046bb46fc5cebb9f39e7bfd56eaeaa2",
        "source_path": "core-foundation",
        "license_expression": "MIT OR Apache-2.0",
        "selected_license": "MIT",
        "license_evidence": "crate LICENSE-MIT at immutable VCS revision",
    },
    ("core-foundation-sys", "0.8.7"): {
        "checksum": "773648b94d0e5d620f64f280777445740e61fe701025087ec8b57f45c791888b",
        "repository": "https://github.com/servo/core-foundation-rs",
        "revision": "652bab0a62d87df8a974ffad2ad9f7be218a8b56",
        "source_path": "core-foundation-sys",
        "license_expression": "MIT OR Apache-2.0",
        "selected_license": "MIT",
        "license_evidence": "crate LICENSE-MIT at immutable VCS revision",
    },
    ("security-framework", "3.7.0"): {
        "checksum": "b7f4bc775c73d9a02cde8bf7b2ec4c9d12743edf609006c7facc23998404cd1d",
        "repository": "https://github.com/kornelski/rust-security-framework",
        "revision": "5f6e65114b77d5bc161d2b099cad09f2a67609d2",
        "source_path": "security-framework",
        "license_expression": "MIT OR Apache-2.0",
        "selected_license": "MIT",
        "license_evidence": "crate LICENSE-MIT at immutable VCS revision",
    },
    ("security-framework-sys", "2.17.0"): {
        "checksum": "6ce2691df843ecc5d231c0b14ece2acc3efb62c0a398c7e1d875f3983ce020e3",
        "repository": "https://github.com/kornelski/rust-security-framework",
        "revision": "5f6e65114b77d5bc161d2b099cad09f2a67609d2",
        "source_path": "security-framework-sys",
        "license_expression": "MIT OR Apache-2.0",
        "selected_license": "MIT",
        "license_evidence": "crate LICENSE-MIT at immutable VCS revision",
    },
    ('anyhow', '1.0.104'): {
        "checksum": '330a5ed07fa54e4702c9d6c4174f74427fc0ef6e214bbd677ae50a5099946470',
        "repository": 'https://github.com/dtolnay/anyhow',
        "revision": '1dbe1862aae650423e3361fbd20b7d17c5109cc3',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('autocfg', '1.5.1'): {
        "checksum": 'f2032f911046de80f0a198e0901378627c33f59ea0ac00e363d481118bd70a53',
        "repository": 'https://github.com/cuviper/autocfg',
        "revision": '2799b09c24e6632f8e653c5cd8fc303e85a906ba',
        "source_path": 'Cargo.toml',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('field-offset', '0.3.6'): {
        "checksum": '38e2275cc4e4fc009b0669731a1e5ab7ebf11f469eaede2bab9309a5b4d6057f',
        "repository": 'https://github.com/Diggsey/rust-field-offset',
        "revision": '95b242e2bd69b7dec41cdd82b780232fcbba15ca',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('log', '0.4.34'): {
        "checksum": 'f9f8bd3e56ce4dfc153cf470fffbfa98c7620958b312ca5c3a4b8d5181fd13c6',
        "repository": 'https://github.com/rust-lang/log',
        "revision": '8034743dd9d7f7583bd9a670271483d176130911',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('memoffset', '0.9.1'): {
        "checksum": '488016bfae457b036d996092f6cb448677611ce4449e970ceaf42695203f218a',
        "repository": 'https://github.com/Gilnaa/memoffset',
        "revision": '153fc3d50f03755be53148bc3eb97390f9011e45',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('rustc_version', '0.4.1'): {
        "checksum": 'cfcb3a22ef46e85b45de6ee7e79d063319ebb6594faafcf1c225ea92ab6e9b92',
        "repository": 'https://github.com/djc/rustc-version-rs',
        "revision": 'eeca449cca83e24150e46739e797aa82e9142809',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('semver', '1.0.28'): {
        "checksum": '8a7852d02fc848982e0c167ef163aaff9cd91dc640ba85e263cb1ce46fae51cd',
        "repository": 'https://github.com/dtolnay/semver',
        "revision": '7625c7aa3f0e8ba21e099d1765bcebcb72aa8816',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('widestring', '0.4.3'): {
        "checksum": 'c168940144dd21fd8046987c16a46a33d5fc84eec29ef9dcddc2ac9e31526b7c',
        "repository": 'https://github.com/starkat99/widestring-rs.git',
        "revision": 'e7236b62b9ffe8bf159644dfbf9b624060c8a843',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT/Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('winapi', '0.3.9'): {
        "checksum": '5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419',
        "repository": 'https://github.com/retep998/winapi-rs',
        "revision": '796a8e6c2971dc2ff1bcff166e6671284f9b5b6b',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT/Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('winapi-i686-pc-windows-gnu', '0.4.0'): {
        "checksum": 'ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6',
        "repository": 'https://github.com/retep998/winapi-rs',
        "revision": '9497609ef44cc9bcd16cd2411c0ee6ccaf5483aa',
        "source_path": 'i686',
        "license_expression": 'MIT/Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'root LICENSE-MIT plus byte-identical published subcrate at immutable VCS revision',
    },
    ('winapi-x86_64-pc-windows-gnu', '0.4.0'): {
        "checksum": '712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f',
        "repository": 'https://github.com/retep998/winapi-rs',
        "revision": '9497609ef44cc9bcd16cd2411c0ee6ccaf5483aa',
        "source_path": 'x86_64',
        "license_expression": 'MIT/Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'root LICENSE-MIT plus byte-identical published subcrate at immutable VCS revision',
    },
    ('aligned', '0.4.3'): {
        "checksum": 'ee4508988c62edf04abd8d92897fca0c2995d907ce1dfeaf369dac3716a40685',
        "repository": 'https://github.com/rust-embedded-community/aligned',
        "revision": 'f13f8e56db5ab99257209758d14d3096a851ff04',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('as-slice', '0.2.1'): {
        "checksum": '516b6b4f0e40d50dcda9365d53964ec74560ad4284da2e7fc97122cd83174516',
        "repository": 'https://github.com/japaric/as-slice',
        "revision": 'c3687342bc9f3c9c202676a5d2e17288ff979c90',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('bitflags', '1.3.2'): {
        "checksum": 'bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a',
        "repository": 'https://github.com/bitflags/bitflags',
        "revision": 'ed185cfb1c447c1b4bd6ac021c9ec3bb02c9e2f2',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT/Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('cfg_aliases', '0.2.2'): {
        "checksum": 'f079e83a288787bcd14a6aea84cee5c87a67c5a3e660c30f557a3d24761b3527',
        "repository": 'https://github.com/katharostech/cfg_aliases',
        "revision": 'e069ce61e00fee423ac1dbe6a897f228f1774716',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE at immutable VCS revision',
    },
    ('cvt', '0.1.2'): {
        "checksum": 'd2ae9bf77fbf2d39ef573205d554d87e86c12f1994e9ea335b0651b9b278bcf1',
        "repository": 'https://github.com/marmistrz/cvt',
        "revision": 'ae6de53753f9e83aac06aa6638e2eac3786975bc',
        "source_path": 'Cargo.toml',
        "license_expression": 'Apache-2.0',
        "selected_license": 'Apache-2.0',
        "license_evidence": 'published LICENSE at immutable VCS revision',
    },
    ('fs_at', '0.2.1'): {
        "checksum": '14af6c9694ea25db25baa2a1788703b9e7c6648dcaeeebeb98f7561b5384c036',
        "repository": 'https://github.com/rbtcollins/fs_at.git',
        "revision": 'e8b58a0682496a0c6ddc9eae80942a2f29a5a7e4',
        "source_path": 'Cargo.toml',
        "license_expression": 'Apache-2.0',
        "selected_license": 'Apache-2.0',
        "license_evidence": 'Cargo.toml Apache-2.0 declaration at immutable VCS revision',
    },
    ('nix', '0.29.0'): {
        "checksum": '71e2746dc3a24dd78b3cfcb7be93368c6de9963d30f43a6a73998a9cf4b17b46',
        "repository": 'https://github.com/nix-rust/nix',
        "revision": '1dad4d8d04a2cd187fae87cb91c4f4e95ff0decd',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE at immutable VCS revision',
    },
    ('stable_deref_trait', '1.2.1'): {
        "checksum": '6ce2be8dc25455e1f91df71bfa12ad37d7af1092ae736f3a6cd0e37bc7810596',
        "repository": 'https://github.com/storyyeller/stable_deref_trait',
        "revision": '30002b4228f7cdee309217b6bf6eee099dda0b00',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows-permissions', '0.2.4'): {
        "checksum": '9e2ccdc3c6bf4d4a094e031b63fadd08d8e42abd259940eb8aa5fdc09d4bf9be',
        "repository": 'https://github.com/danieldulaney/windows-permissions-rs',
        "revision": '8740e4efbd88dd01046ad9c169894f3a52eb6e2c',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'Cargo.toml MIT declaration at immutable VCS revision',
    },
    ('windows-sys', '0.52.0'): {
        "checksum": '282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": '3a605cba064b26f2a198ac58085f8c8836f47c38',
        "source_path": 'crates/libs/sys',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows-targets', '0.52.6'): {
        "checksum": '9b724f72796e036ab90c1021d4780d4d3d648aca59e491e6b98e725b84e99973',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/libs/targets',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_aarch64_gnullvm', '0.52.6'): {
        "checksum": '32a4622180e7a0ec044bb555404c800bc9fd9ec262ec147edd5989ccd0c02cd3',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/aarch64_gnullvm',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_aarch64_msvc', '0.52.6'): {
        "checksum": '09ec2a7bb152e2252b53fa7803150007879548bc709c039df7627cabbd05d469',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/aarch64_msvc',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_i686_gnu', '0.52.6'): {
        "checksum": '8e9b5ad5ab802e97eb8e295ac6720e509ee4c243f69d781394014ebfe8bbfa0b',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/i686_gnu',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_i686_gnullvm', '0.52.6'): {
        "checksum": '0eee52d38c090b3caa76c563b86c3a4bd71ef1a819287c19d586d7334ae8ed66',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/i686_gnullvm',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_i686_msvc', '0.52.6'): {
        "checksum": '240948bc05c5e7c6dabba28bf89d89ffce3e303022809e73deaefe4f6ec56c66',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/i686_msvc',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_x86_64_gnu', '0.52.6'): {
        "checksum": '147a5c80aabfbf0c7d901cb5895d1de30ef2907eb21fbbab29ca94c5b08b1a78',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/x86_64_gnu',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_x86_64_gnullvm', '0.52.6'): {
        "checksum": '24d5b23dc417412679681396f2b49f3de8c1473deb516bd34410872eff51ed0d',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/x86_64_gnullvm',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('windows_x86_64_msvc', '0.52.6'): {
        "checksum": '589f6da84c646204747d1270a2a5661ea66ed1cced2631d546fdfb155959f9ec',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'db06b51c2ebb743efb544d40e3064efa49f28d38',
        "source_path": 'crates/targets/x86_64_msvc',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'published LICENSE-MIT at immutable VCS revision',
    },
    ('winsafe', '0.0.29'): {
        "checksum": '9ef0ffc427f045c0cc9ebffd6f4f91153dcd2a1547b5c3c29ee3f541e16e95c6',
        "repository": 'https://github.com/rodrigocfd/winsafe',
        "revision": '71ed88c2a0d18b03ee452f6d4261f4c22f443483',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('windows-acl', '0.3.0'): {
        "checksum": '177b1723986bcb4c606058e77f6e8614b51c7f9ad2face6f6fd63dd5c8b3cec3',
        "repository": 'https://github.com/trailofbits/windows-acl',
        "revision": '09f87952649080c38b085e9759e84aa88ccbb2e2',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },
    ('windows-dpapi', '0.2.0'): {
        "checksum": '2981752d6f11bdcab4db52be8ad5c0e6a6d4d6d566764b3058cc1ee473e6479e',
        "repository": 'https://github.com/sheridans/windows-dpapi',
        "revision": '4ac261cc789ab046dd8bac3e914f49b26b8b7d77',
        "source_path": 'Cargo.toml',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": "MIT",
        "license_evidence": 'crate MIT license text at immutable VCS revision',
    },

    ('aes', '0.9.3'): {
        "checksum": '35f0f96ce78e38c3dc6d8948aa8163d06385be74000f3c7a95bf1eef35d3ea32',
        "repository": 'https://github.com/RustCrypto/block-ciphers',
        "revision": 'c1534361e7549e29a16c3505f45a04c92d26b62a',
        "source_path": 'aes',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:25f77efa393854f7c15eaf6a2589a2071db2d7337642742763adb12bdecf0445 in published crate',
    },
    ('async-broadcast', '0.7.2'): {
        "checksum": '435a87a52755b8f27fcf321ac4f04b2802e337c8c4872923137471ec39c37532',
        "repository": 'https://github.com/smol-rs/async-broadcast',
        "revision": 'f7a99132b7e12cb03945013df78b93a35ceafbc1',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:24e5860bf589d8501643e6ea51ffb3df66db2867492b09033d486183efbfa970 in published crate',
    },
    ('async-recursion', '1.1.1'): {
        "checksum": '3b43422f69d8ff38f95f1b2bb76517c91589a924d1559a0e935d7c8ce0274c11',
        "repository": 'https://github.com/dcchut/async-recursion',
        "revision": '3254b12e384f7688f9a1b95c54cfa07fe82baee7',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:30fefc3a7d6a0041541858293bcbea2dde4caa4c0a5802f996a7f7e8c0085652 in published crate',
    },
    ('async-trait', '0.1.92'): {
        "checksum": '82f6aeea286b8eb4dd3431a1be1b59d290ace00f5bfd8e2a159bc2a05e2c1667',
        "repository": 'https://github.com/dtolnay/async-trait',
        "revision": '82e7e9edd60f622294373a23c0ce9c0077ad0263',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('block-padding', '0.4.2'): {
        "checksum": '710f1dd022ef4e93f8a438b4ba958de7f64308434fa6a87104481645cc30068b',
        "repository": 'https://github.com/RustCrypto/utils',
        "revision": '6c8370f218505d3a767ace760ddef2c9b9466214',
        "source_path": 'block-padding',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:98181e7249d0c01737645ec982499ce99a0f07eb8f7d625b8840d799d10dbc01 in published crate',
    },
    ('bumpalo', '3.20.3'): {
        "checksum": '72f5acc6cb2ba439de613abc23857ec3d78374d8ed5ac84e9d11336e87da8649',
        "repository": 'https://github.com/fitzgen/bumpalo',
        "revision": '84654ace6be4444da3ff102a0a0af3b38c4df4fb',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:65f94e99ddaf4f5d1782a6dae23f35d4293a9a01444a13135a6887017d353cee in published crate',
    },
    ('bytes', '1.12.1'): {
        "checksum": 'fc652a48c352aef3ea3aed32080501cf3ef6ed5da78602a020c991775b0aff04',
        "repository": 'https://github.com/tokio-rs/bytes',
        "revision": '76c0fbb54ed4336caf9d2311658a2f4a5627c21d',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:45f522cacecb1023856e46df79ca625dfc550c94910078bd8aec6e02880b3d42 in published crate',
    },
    ('cbc', '0.2.1'): {
        "checksum": 'ce2dc9ee5f88d11e0beb842c88b33c8a5cf0d1329c4b19494af42b07dbfe8896',
        "repository": 'https://github.com/RustCrypto/block-modes',
        "revision": '2fce5053a789840aa401148b76e01f39fe37de6a',
        "source_path": 'cbc',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:63af4bea227c94d021e99427f6ca3a4b8efddadcca93ab6130f708cf6138cf68 in published crate',
    },
    ('const-oid', '0.10.2'): {
        "checksum": 'a6ef517f0926dd24a1582492c791b6a4818a4d94e789a334894aa15b0d12f55c',
        "repository": 'https://github.com/RustCrypto/formats',
        "revision": '385ef739bcec785472c7c971c57f26f87caca3b8',
        "source_path": 'const-oid',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:73b9dc2e79c7308998dd30296e073aefaefb944a68fb89aa412c23c0edcabcaa in published crate',
    },
    ('cpubits', '0.1.1'): {
        "checksum": '15b85f9c39137c3a891689859392b1bd49812121d0d61c9caf00d46ed5ce06ae',
        "repository": 'https://github.com/RustCrypto/utils',
        "revision": 'bff92a8c33629ae8e9d1407f3b7fea604992dd0f',
        "source_path": 'cpubits',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:003736bc98408b53511b9cc8ff5d1af3bf0a8f17d53841cc39d8ab1b96a195b0 in published crate',
    },
    ('endi', '1.1.1'): {
        "checksum": '66b7e2430c6dff6a955451e2cfc438f09cea1965a9d6f87f7e3b90decc014099',
        "repository": 'https://github.com/zeenix/endi',
        "revision": '226224559725a9ef0e0687b3a54eca9ad6a5a6c7',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('enumflags2', '0.7.12'): {
        "checksum": '1027f7680c853e056ebcec683615fb6fbbc07dbaa13b4d5d9442b146ded4ecef',
        "repository": 'https://github.com/meithecatte/enumflags2',
        "revision": '332c37f47577e5f6b7104419da7e761963032086',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:2e3edd2716fb15616e867bd53f724f0906732242e9ae76bb1faad1d88f7eeebd in published crate',
    },
    ('enumflags2_derive', '0.7.12'): {
        "checksum": '67c78a4d8fdf9953a5c9d458f9efe940fd97a0cab0941c075a813ac594733827',
        "repository": 'https://github.com/meithecatte/enumflags2',
        "revision": '332c37f47577e5f6b7104419da7e761963032086',
        "source_path": 'enumflags_derive',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:d13669f0eaff9b7015bff0d8637297213b53d9f239826636c07a15633acf7052 in published crate',
    },
    ('equivalent', '1.0.2'): {
        "checksum": '877a4ace8713b0bcf2a4e7eec82529c029f1d0619886d18145fea96c3ffe5c0f',
        "repository": 'https://github.com/indexmap-rs/equivalent',
        "revision": '44cdd44f8b8ebb5f9ae096c7550a5e74ffb7d6ae',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:7365cc8878a1d7ce155a58c4ca09c3d7a6be413efa5334a80ea842912b669349 in published crate',
    },
    ('errno', '0.3.14'): {
        "checksum": '39cab71617ae0d63f51a36d69f866391735b51691dbda63cf6f96d042b63efeb',
        "repository": 'https://github.com/lambda-fairy/rust-errno',
        "revision": 'ffc03bfb9eb491013567115e6eea560948cd9e52',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:8764a597675778ddfd4e25f81b08a05dbcf089ac05662df7613fe67f150e3aa2 in published crate',
    },
    ('event-listener', '5.4.2'): {
        "checksum": '5a23add41df1562121a9393cb065eab5146a1242410f23a644851e90cfd669d2',
        "repository": 'https://github.com/smol-rs/event-listener',
        "revision": '77004ac77dcef7884bf71a99c276a50f83a7fa4e',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('event-listener-strategy', '0.5.4'): {
        "checksum": '8be9f3dfaaffdae2972880079a491a1a8bb7cbed0b8dd7a347f668b4150a3b93',
        "repository": 'https://github.com/smol-rs/event-listener-strategy',
        "revision": 'f533c437e8a6561e9b625c3ee036a8ea7d19375c',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('fastrand', '2.5.0'): {
        "checksum": 'da7c62ceae207dd37ea5b845da6a0696c799f85e97da1ab5b7910be3c1c80223',
        "repository": 'https://github.com/smol-rs/fastrand',
        "revision": '7a1cc2c69ebde6cb3442809213db3fe19a2cb0ba',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('futures-core', '0.3.34'): {
        "checksum": '92d699e522242e69e3003b94ecc1f960f3a5e015aa7c5d7486e65ad01dd94f5e',
        "repository": 'https://github.com/rust-lang/futures-rs',
        "revision": '705e6b5c0f06535b1aac1cb1989a172b3d45be8c',
        "source_path": 'futures-core',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd in published crate',
    },
    ('futures-io', '0.3.34'): {
        "checksum": '53c0fa8157de1303bfffdaa1cc2a673bfffb60102f76b0ef4441659124373fed',
        "repository": 'https://github.com/rust-lang/futures-rs',
        "revision": '705e6b5c0f06535b1aac1cb1989a172b3d45be8c',
        "source_path": 'futures-io',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd in published crate',
    },
    ('futures-lite', '2.6.1'): {
        "checksum": 'f78e10609fe0e0b3f4157ffab1876319b5b0db102a2c60dc4626306dc46b44ad',
        "repository": 'https://github.com/smol-rs/futures-lite',
        "revision": '226ce18976d8714d6bd9700b61dcc81d7200bc9a',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('futures-macro', '0.3.34'): {
        "checksum": '9fb9654ba8355388abeb8dcb4fc62f511300867002afc858860463bdd9fe0c44',
        "repository": 'https://github.com/rust-lang/futures-rs',
        "revision": '705e6b5c0f06535b1aac1cb1989a172b3d45be8c',
        "source_path": 'futures-macro',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd in published crate',
    },
    ('futures-task', '0.3.34'): {
        "checksum": 'cd417de3d1d015fc3bfd2b1ea46dfc7bab72ef86f1cc7cc9c78e728b34a6d1fd',
        "repository": 'https://github.com/rust-lang/futures-rs',
        "revision": '705e6b5c0f06535b1aac1cb1989a172b3d45be8c',
        "source_path": 'futures-task',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd in published crate',
    },
    ('futures-util', '0.3.34'): {
        "checksum": '0d50a92467f8ba5dd6e3ee5d4bd04d73ab2e4e1c44474a0674821dfce14b79bc',
        "repository": 'https://github.com/rust-lang/futures-rs',
        "revision": '705e6b5c0f06535b1aac1cb1989a172b3d45be8c',
        "source_path": 'futures-util',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd in published crate',
    },
    ('hashbrown', '0.17.1'): {
        "checksum": 'ed5909b6e89a2db4456e54cd5f673791d7eca6732202bbf2a9cc504fe2f9b84a',
        "repository": 'https://github.com/rust-lang/hashbrown',
        "revision": 'c62a63a61b7caf2de8f9ecb7b06a66b0ab6bdf3d',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:ff8f68cb076caf8cefe7a6430d4ac086ce6af2ca8ce2c4e5a2004d4552ef52a2 in published crate',
    },
    ('hex', '0.4.3'): {
        "checksum": '7f24254aa9a54b5c858eaee2f5bccdb46aaf0e486a595ed5fd8f86ba55232a70',
        "repository": 'https://github.com/KokaKiwi/rust-hex',
        "revision": 'b2b4370b5bf021b98ee7adc92233e8de3f2de792',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:f7bdb3426d045cd50efd4953026e3eb5a83d0199f458a075602611b9344da5b9 in published crate',
    },
    ('indexmap', '2.14.2'): {
        "checksum": 'cc4e190f5d26ca7051642629da2c52fc03bde85a03197c99408dcd291734c855',
        "repository": 'https://github.com/indexmap-rs/indexmap',
        "revision": '41a870887c4c77adf665886e63df08f406bfe37a',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:ecc269ef87fd38a1d98e30bfac9ba964a9dbd9315c3770fed98d4d7cb5882055 in published crate',
    },
    ('js-sys', '0.3.105'): {
        "checksum": 'ce57d20d1ea864ce2ac172ab472d409214f4fd359f0b2a2775abdf522e2af99e',
        "repository": 'https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/js-sys',
        "revision": '246946fddd62163e778c3a1f6afe7264347adceb',
        "source_path": 'crates/js-sys',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397 in published crate',
    },
    ('linux-raw-sys', '0.12.1'): {
        "checksum": '32a66949e030da00e8c7d4434b251670a91556f4144941d37452769c25d58a53',
        "repository": 'https://github.com/sunfishcode/linux-raw-sys',
        "revision": '0e2918cf3e366d9c923d4ca05f169b49d826db56',
        "source_path": '.',
        "license_expression": 'Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('memchr', '2.8.3'): {
        "checksum": 'cf8baf1c55e62ffcace7a9f06f4bd9cd3f0c4beb022d3b367256b91b87513d98',
        "repository": 'https://github.com/BurntSushi/memchr',
        "revision": '5fdb40c054e1fff359a2f7bdf7f87a13b34b465d',
        "source_path": '.',
        "license_expression": 'Unlicense OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f in published crate',
    },
    ('mio', '1.2.3'): {
        "checksum": '4b18443e9c262bfe8fa82f51666e2642c53393f7e5c27b3e1aeab922cff5b9d8',
        "repository": 'https://github.com/tokio-rs/mio',
        "revision": 'da425f909dd6b86d887da9eaefcb158099b5b165',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:07919255c7e04793d8ea760d6c2ce32d19f9ff02bdbdde3ce90b1e1880929a9b in published crate',
    },
    ('num', '0.4.3'): {
        "checksum": '35bd024e8b2ff75562e5f34e7f4905839deb4b22955ef5e73d2fea1b9813cb23',
        "repository": 'https://github.com/rust-num/num',
        "revision": '1fec8524c4eaa27cbdf4bb3eb46782d2ff40b2ed',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('num-bigint', '0.4.8'): {
        "checksum": 'c89e69e7e0f03bea5ef08013795c25018e101932225a656383bd384495ecc367',
        "repository": 'https://github.com/rust-num/num-bigint',
        "revision": '65330056d6c00def4c6cda78c87453f4c2a76913',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('num-complex', '0.4.6'): {
        "checksum": '73f88a1307638156682bada9d7604135552957b7818057dcef22705b4d509495',
        "repository": 'https://github.com/rust-num/num-complex',
        "revision": '91fdc06356c0c868cb88b5a180859023c57e6e50',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('num-integer', '0.1.47'): {
        "checksum": '7ce2d95d4b3734dc35aa2f45e1aa22cd416814592a4f9d9205e11affd5b8e10b',
        "repository": 'https://github.com/rust-num/num-integer',
        "revision": '765fd9b3eb9397471d41b9f613e530f6d9d6383f',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('num-iter', '0.1.46'): {
        "checksum": 'c92800bd69a1eac91786bcfe9da64a897eb72911b8dc3095decbd07429e8048b',
        "repository": 'https://github.com/rust-num/num-iter',
        "revision": '6bfec7678bae3f4a38d1eeeab20532ac0bae7c9c',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('num-rational', '0.4.2'): {
        "checksum": 'f83d14da390562dca69fc84082e73e548e1ad308d24accdedd2720017cb37824',
        "repository": 'https://github.com/rust-num/num-rational',
        "revision": '4d55ad22ac86ebbc4cb45d79a956e4a1f7af57d1',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('num-traits', '0.2.19'): {
        "checksum": '071dfc062690e90b734c0b2273ce72ad0ffa95f0c74596bc250dcfd960262841',
        "repository": 'https://github.com/rust-num/num-traits',
        "revision": '7ec3d41d39b28190ec1d42db38021107b3951f3a',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb in published crate',
    },
    ('once_cell', '1.21.4'): {
        "checksum": '9f7c3e4beb33f85d45ae3e3a1792185706c8e16d043238c593331cc7cd313b50',
        "repository": 'https://github.com/matklad/once_cell',
        "revision": '80fe900b21f6d76c1a2ed74d3343e8a3a88c46d0',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('ordered-stream', '0.2.0'): {
        "checksum": '9aa2b01e1d916879f73a53d01d1d6cee68adbb31d6d9177a8cfce093cced1d50',
        "repository": 'https://github.com/danieldg/ordered-stream',
        "revision": '5c58ca1a3d680c2406f0c6a1520c3476c27048aa',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('parking', '2.2.1'): {
        "checksum": 'f38d5652c16fde515bb1ecef450ab0f6a219d619a7274976324d5e377f7dceba',
        "repository": 'https://github.com/smol-rs/parking',
        "revision": '0ece32dbfd6cd1bc1510ede6ed56acb772edf83f',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('pin-project-lite', '0.2.17'): {
        "checksum": 'a89322df9ebe1c1578d689c92318e070967d1042b512afbe49518723f4e6d5cd',
        "repository": 'https://github.com/taiki-e/pin-project-lite',
        "revision": '3bdf763446aa78f90e3bdac1ef583e014832ab4c',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('proc-macro-crate', '3.5.0'): {
        "checksum": 'e67ba7e9b2b56446f1d419b1d807906278ffa1a658a8a5d8a39dcb1f5a78614f',
        "repository": 'https://github.com/bkchr/proc-macro-crate',
        "revision": '8f3d8b04539f732e09c07907f138a248c8d0eed9',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('proc-macro2', '1.0.107'): {
        "checksum": '985e7ec9bb745e6ce6535b544d84d6cd6f7ad8bd711c398938ae983b91a766d9',
        "repository": 'https://github.com/dtolnay/proc-macro2',
        "revision": 'ed8a5497669cd63db33bf24646f261b012bbbc4a',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('quote', '1.0.47'): {
        "checksum": '1fbf4db142a473a8d80c26bbf18454ed458bf8d26c8219c331daecfdbd079001',
        "repository": 'https://github.com/dtolnay/quote',
        "revision": '723dcb47d3f0ddc896e17287c8a8d3f2ea2317d5',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('rustix', '1.1.4'): {
        "checksum": 'b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190',
        "repository": 'https://github.com/bytecodealliance/rustix',
        "revision": 'c4caf5caaa7e93828a2e4a4cdba1dd0171e45717',
        "source_path": '.',
        "license_expression": 'Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('rustversion', '1.0.23'): {
        "checksum": 'cf54715a573b99ac80df0bc206da022bcd442c974952c7b9720069370852e21f',
        "repository": 'https://github.com/dtolnay/rustversion',
        "revision": '3a7c76605450b9a7299c6502a421909de9126a59',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('secret-service', '5.2.0'): {
        "checksum": '5107b24b91445dd2aa449a258a1807b63240942157292354dc5bfdbeb8bc6db8',
        "repository": 'https://github.com/hwchen/secret-service-rs.git',
        "revision": '1fe4fbe405b152bc969deb5de417847e1e4e4c7b',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:074585dbaf488db540c26f7b20f36053a157d7580bb1c390dc9f6afdb017c83a in published crate',
    },
    ('serde', '1.0.229'): {
        "checksum": '4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba',
        "repository": 'https://github.com/serde-rs/serde',
        "revision": '7fc3b4c30c94f73a96ebd1553f2b090d928fc3a8',
        "source_path": 'serde',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('serde_core', '1.0.229'): {
        "checksum": '67dca2c9c51e58a4791a4b1ed58308b39c64224d349a935ab5039aa360942a48',
        "repository": 'https://github.com/serde-rs/serde',
        "revision": '7fc3b4c30c94f73a96ebd1553f2b090d928fc3a8',
        "source_path": 'serde_core',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('serde_derive', '1.0.229'): {
        "checksum": 'e7a5d71263a5a7d47b41f6b3f06ba276f10cc18b0931f1799f710578e2309348',
        "repository": 'https://github.com/serde-rs/serde',
        "revision": '7fc3b4c30c94f73a96ebd1553f2b090d928fc3a8',
        "source_path": 'serde_derive',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('serde_repr', '0.1.21'): {
        "checksum": '8d3b1629de253c70a0508c3899572da79ca359fdab27c7920ff00406df418906',
        "repository": 'https://github.com/dtolnay/serde-repr',
        "revision": '205ce23cf7c55981be8c1e02ac87695c57a13ee0',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('signal-hook-registry', '1.4.8'): {
        "checksum": 'c4db69cba1110affc0e9f7bcd48bbf87b3f4fc7c61fc9155afd4c469eb3d6c1b',
        "repository": 'https://github.com/vorner/signal-hook',
        "revision": '4d5fd6a6663be38e70774e4ac733d65916c70951',
        "source_path": 'signal-hook-registry',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:503558bfefe66ca15e4e3f7955b3cb0ec87fd52f29bf24b336af7bd00e946d5c in published crate',
    },
    ('slab', '0.4.12'): {
        "checksum": '0c790de23124f9ab44544d7ac05d60440adc586479ce501c1d6d7da3cd8c9cf5',
        "repository": 'https://github.com/tokio-rs/slab',
        "revision": 'a1e4346070a48c936d808de75191dee5d01e433c',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:8ce0830173fdac609dfb4ea603fdc002c2f4af0dc9b1a005653f5da9cf534b18 in published crate',
    },
    ('socket2', '0.6.5'): {
        "checksum": 'c3d1e2c7f27f8d4cb10542a02c49005dbd6e93095799d6f3be745fae9f8fedd4',
        "repository": 'https://github.com/rust-lang/socket2',
        "revision": '239dd83a4ced08e514d2c38942aab99791119f0d',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397 in published crate',
    },
    ('syn', '2.0.119'): {
        "checksum": '872831b642d1a07999a962a351ed35b955ea2cfc8f3862091e2a240a84f17297',
        "repository": 'https://github.com/dtolnay/syn',
        "revision": '3295f9e9841785ac88a5e558c884854d5fb7d67f',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('syn', '3.0.5'): {
        "checksum": '12df2e0110f65b775f769bb17ef989067a1d931b2eb822bd4346631eeada89f9',
        "repository": 'https://github.com/dtolnay/syn',
        "revision": 'e0ad92d68b588c964e82bb74c3bd3e1e99f97ceb',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('tempfile', '3.27.0'): {
        "checksum": '32497e9a4c7b38532efcdebeef879707aa9f794296a4f0244f6f69e9bc8574bd',
        "repository": 'https://github.com/Stebalien/tempfile',
        "revision": '5c8fa12eb584931b4f1bccfde87eb72fbfa7dc61',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:8b427f5bc501764575e52ba4f9d95673cf8f6d80a86d0d06599852e1a9a20a36 in published crate',
    },
    ('tokio', '1.53.1'): {
        "checksum": '202caea871b69668250d242070849eb495be178ed697a3e98aebce5bc81a0bed',
        "repository": 'https://github.com/tokio-rs/tokio',
        "revision": '75fef53d0a8590c2d1dbb63672aa7b7d1ef51155',
        "source_path": 'tokio',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:253cd04c6714889df2d32f3f64d669179a1c95c76ac43c40882c52eb06bc3552 in published crate',
    },
    ('toml_datetime', '1.1.1+spec-1.1.0'): {
        "checksum": '3165f65f62e28e0115a00b2ebdd37eb6f3b641855f9d636d3cd4103767159ad7',
        "repository": 'https://github.com/toml-rs/toml',
        "revision": '9db8aad6eafbc62f6b9d1950117649cc41eaf695',
        "source_path": 'crates/toml_datetime',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6 in published crate',
    },
    ('toml_edit', '0.25.15+spec-1.1.0'): {
        "checksum": '1340ea94a5856333492c9064b02c778b191dd2c853778d9609debdcdfea3a614',
        "repository": 'https://github.com/toml-rs/toml',
        "revision": '8e1d5a85c361ac012957441bb4788ae82f5dc9c8',
        "source_path": 'crates/toml_edit',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6 in published crate',
    },
    ('toml_parser', '1.1.3+spec-1.1.0'): {
        "checksum": '1d38ac1cf9b95face32296c0a3ede1fdc270627c9d9c02a7274dd6d960dc4d56',
        "repository": 'https://github.com/toml-rs/toml',
        "revision": '4ec099fed591a172f82007cb2f9d605985bbecee',
        "source_path": 'crates/toml_parser',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6 in published crate',
    },
    ('tracing', '0.1.44'): {
        "checksum": '63e71662fa4b2a2c3a26f570f037eb95bb1f85397f3cd8076caed2f026a6d100',
        "repository": 'https://github.com/tokio-rs/tracing',
        "revision": '2d55f6faf9be83e7e4634129fb96813241aac2b8',
        "source_path": 'tracing',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1 in published crate',
    },
    ('tracing-attributes', '0.1.31'): {
        "checksum": '7490cfa5ec963746568740651ac6781f701c9c5ea257c58e057f3ba8cf69e8da',
        "repository": 'https://github.com/tokio-rs/tracing',
        "revision": '55086231ec4aaeffcaab9932e696f40278f06bd1',
        "source_path": 'tracing-attributes',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1 in published crate',
    },
    ('tracing-core', '0.1.36'): {
        "checksum": 'db97caf9d906fbde555dd62fa95ddba9eecfd14cb388e4f491a66d74cd5fb79a',
        "repository": 'https://github.com/tokio-rs/tracing',
        "revision": '10a9e838a35e6ded79d66af246be2ee05417136d',
        "source_path": 'tracing-core',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1 in published crate',
    },
    ('uds_windows', '1.2.1'): {
        "checksum": 'f2f6fb2847f6742cd76af783a2a2c49e9375d0a111c7bef6f71cd9e738c72d6e',
        "repository": 'https://github.com/haraldh/rust_uds_windows',
        "revision": '0b3a6ff289a0426f96bbc0310505b8531469c896',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:27ebda9d51f0a56b7e281ccd8230a27236dcb51c05f64b07869ecf6e965d68b0 in published crate',
    },
    ('unicode-ident', '1.0.24'): {
        "checksum": 'e6e4313cd5fcd3dad5cafa179702e2b244f760991f45397d14d4ebf38247da75',
        "repository": 'https://github.com/dtolnay/unicode-ident',
        "revision": '5b54a632702b5744a1c40ea01c127c0ac0498172',
        "source_path": '.',
        "license_expression": '(MIT OR Apache-2.0) AND Unicode-3.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('uuid', '1.26.1'): {
        "checksum": '2ef6dac1e96601b4fb3acccccff2139741fcb757cb9a36089bf5be91cfb285ce',
        "repository": 'https://github.com/uuid-rs/uuid',
        "revision": '9f927126c89892ddfed6cd2f92df16852f3f9aa6',
        "source_path": '.',
        "license_expression": 'Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:436bc5a105d8e57dcd8778730f3754f7bf39c14d2f530e4cde4bd2d17a83ec3d in published crate',
    },
    ('wasi', '0.11.1+wasi-snapshot-preview1'): {
        "checksum": 'ccf3ec651a847eb01de73ccad15eb7d99f80485de043efb2f370cd654f4ea44b',
        "repository": 'https://github.com/bytecodealliance/wasi',
        "revision": 'f136a81add9706ed0b36ac3c4b0da943de8e690d',
        "source_path": '.',
        "license_expression": 'Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },
    ('wasm-bindgen', '0.2.128'): {
        "checksum": 'aecb87a33d3b0c5e3b7aa46336eaf486cffafbd281b195e4c8b80d50df2351bf',
        "repository": 'https://github.com/wasm-bindgen/wasm-bindgen',
        "revision": '246946fddd62163e778c3a1f6afe7264347adceb',
        "source_path": '.',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397 in published crate',
    },
    ('wasm-bindgen-macro', '0.2.128'): {
        "checksum": 'a690d511e3c1a8b3a55e33511e3c2c00c78415cd23650f32b808627f5696b9ed',
        "repository": 'https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro',
        "revision": '246946fddd62163e778c3a1f6afe7264347adceb',
        "source_path": 'crates/macro',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397 in published crate',
    },
    ('wasm-bindgen-macro-support', '0.2.128'): {
        "checksum": '411e4887f0071ef2d2164a9d5fdf2d20efbef78fccd3a78b0c10a1dc5295e48a',
        "repository": 'https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support',
        "revision": '246946fddd62163e778c3a1f6afe7264347adceb',
        "source_path": 'crates/macro-support',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397 in published crate',
    },
    ('wasm-bindgen-shared', '0.2.128'): {
        "checksum": '81941cd78d0c92026c33e5e01312845a4cb1e9af3407f9134b100dd03144103e',
        "repository": 'https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared',
        "revision": '246946fddd62163e778c3a1f6afe7264347adceb',
        "source_path": 'crates/shared',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397 in published crate',
    },
    ('windows-link', '0.2.1'): {
        "checksum": 'f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": 'd468916ac27a36fb8a12bafc1bf5c0ec2fe92238',
        "source_path": 'crates/libs/link',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'crates/libs/link/license-mit sha256:c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383 at immutable VCS revision',
    },
    ('windows-sys', '0.61.2'): {
        "checksum": 'ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc',
        "repository": 'https://github.com/microsoft/windows-rs',
        "revision": '32c3144490c016fe496a0aed769bce60987a2e9d',
        "source_path": 'crates/libs/sys',
        "license_expression": 'MIT OR Apache-2.0',
        "selected_license": 'MIT',
        "license_evidence": 'crates/libs/sys/license-mit sha256:c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383 at immutable VCS revision',
    },
    ('winnow', '1.0.4'): {
        "checksum": '23b97319f7b8343df12cc98938e5c3eb436064524c8d2b4e30a1d3a36eecdf81',
        "repository": 'https://github.com/winnow-rs/winnow',
        "revision": '7539ec0fc27144bfdcf9a68b0dbbec48bd0d5bae',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE-MIT sha256:cb5aedb296c5246d1f22e9099f925a65146f9f0d6b4eebba97fd27a6cdbbab2d in published crate',
    },
    ('zbus', '5.19.0'): {
        "checksum": '5db4be7c075cb421e4b7ee645541604239bd243ba7c357511f4ff3a74b555907',
        "repository": 'https://github.com/z-galaxy/zbus/',
        "revision": '7518d73db4dbf830ec1c9c43865bf8aeae9a8ffd',
        "source_path": 'zbus',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad in published crate',
    },
    ('zbus_macros', '5.19.0'): {
        "checksum": '2990635d09ade6df1868f72f8cac69a876a90981e8bd3c40b1be413f8dc88f40',
        "repository": 'https://github.com/z-galaxy/zbus/',
        "revision": '7518d73db4dbf830ec1c9c43865bf8aeae9a8ffd',
        "source_path": 'zbus_macros',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad in published crate',
    },
    ('zbus_names', '4.3.4'): {
        "checksum": 'd8bf88b4a3ff53e883001e0e0115b297a9d53c31b9c1edd2bfdd853e3428624e',
        "repository": 'https://github.com/z-galaxy/zbus/',
        "revision": '39483146d1e625a0a6da297b3aa1749ddf4bfe2a',
        "source_path": 'zbus_names',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad in published crate',
    },
    ('zcheapstr', '1.1.0'): {
        "checksum": 'd1afec51604565183aeb5c54c20aeab286120d4e4460f7f76e3e8bb8c0d99473',
        "repository": 'https://github.com/z-galaxy/zcheapstr/',
        "revision": 'a9e7595419e5e826c14e35d0a77ebc84b973ca58',
        "source_path": '.',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:59871bb1612f8a0569655c10b995636827eceab42c6578cebdc45ad2922e0142 in published crate',
    },
    ('zvariant', '5.15.0'): {
        "checksum": 'c1d34c27cc6cdd1f458427519dd6b8612f7b7e3f7b9a0b2355d041dda9869147',
        "repository": 'https://github.com/z-galaxy/zbus/',
        "revision": '0f4db6980bc1566278618a46c9ffa7f5f025c18a',
        "source_path": 'zvariant',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad in published crate',
    },
    ('zvariant_derive', '5.15.0'): {
        "checksum": '864155e69b4352db0c7f374917bf45d1e0c8d17659c8b3dbf9795f3673f8c497',
        "repository": 'https://github.com/z-galaxy/zbus/',
        "revision": '0f4db6980bc1566278618a46c9ffa7f5f025c18a',
        "source_path": 'zvariant_derive',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad in published crate',
    },
    ('zvariant_utils', '4.2.0'): {
        "checksum": 'bad0294361a320b694a328460dc73add56c306150f5cb6bfafc44446120008a3',
        "repository": 'https://github.com/z-galaxy/zbus/',
        "revision": '0f4db6980bc1566278618a46c9ffa7f5f025c18a',
        "source_path": 'zvariant_utils',
        "license_expression": 'MIT',
        "selected_license": 'MIT',
        "license_evidence": 'LICENSE sha256:23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3 in published crate',
    },}

EXPECTED_DIRECT: dict[str, dict[str, Any]] = {
    "argon2": {"version": "=0.6.0", "default-features": False, "features": ["alloc", "zeroize"]},
    "chacha20poly1305": {"version": "=0.11.0", "default-features": False, "features": ["alloc", "zeroize"]},
    "getrandom": {"version": "=0.4.3", "default-features": False},
    "hkdf": {"version": "=0.13.0", "default-features": False},
    "libsqlite3-sys": {
        "version": "=0.38.2",
        "default-features": False,
        "features": ["bundled-sqlcipher-vendored-openssl"],
    },
    "rusqlite": {
        "version": "=0.40.1",
        "default-features": False,
        "features": ["bundled-sqlcipher-vendored-openssl"],
    },
    "sha2": {"version": "=0.11.0", "default-features": False},
    "zeroize": {"version": "=1.9.0"},
}

EXPECTED_NATIVE = {
    "sqlcipher-4.14.0": {
        "parent": ("libsqlite3-sys", "0.38.2"),
        "name": "SQLCipher 4.14.0",
        "source_repository": "https://github.com/sqlcipher/sqlcipher",
        "source_revision": "778ab890cfc30c3631212dcceb0295498abdcd3e",
        "source_license": "BSD-3-Clause",
        "source_paths": ["LICENSE.md", "src"],
        "embedded_paths": ["libsqlite3-sys/sqlcipher/sqlite3.c", "libsqlite3-sys/sqlcipher/sqlite3.h"],
        "evidence_reference": EVIDENCE_REFERENCE,
    },
    "sqlite-3.51.3": {
        "parent": ("libsqlite3-sys", "0.38.2"),
        "name": "SQLite 3.51.3 embedded in SQLCipher 4.14.0",
        "source_repository": "https://github.com/rusqlite/rusqlite",
        "source_revision": "e88f112bef7899234a497baed5cc3c3d553deeb8",
        "source_license": "LicenseRef-SQLite-Public-Domain",
        "source_paths": ["libsqlite3-sys/sqlcipher/sqlite3.c", "libsqlite3-sys/sqlcipher/sqlite3.h"],
        "embedded_paths": ["libsqlite3-sys/sqlcipher/sqlite3.c", "libsqlite3-sys/sqlcipher/sqlite3.h"],
        "evidence_reference": EVIDENCE_REFERENCE,
    },
    "openssl-3.6.3": {
        "parent": ("openssl-src", "300.6.1+3.6.3"),
        "name": "OpenSSL 3.6.3",
        "source_repository": "https://github.com/openssl/openssl",
        "source_revision": "aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f",
        "source_license": "Apache-2.0",
        "source_paths": ["LICENSE.txt", "crypto", "include", "providers", "ssl"],
        "embedded_paths": ["openssl"],
        "evidence_reference": EVIDENCE_REFERENCE,
    },
}

REDIRECT_ENV = (
    "OPENSSL_NO_VENDOR",
    "OPENSSL_LIB_DIR",
    "OPENSSL_INCLUDE_DIR",
    "OPENSSL_DIR",
    "LIBSQLITE3_SYS_USE_PKG_CONFIG",
    "LIBSQLITE3_FLAGS",
)


class ClosureError(ValueError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise ClosureError(f"{path.relative_to(ROOT)}: invalid JSON: {exc}") from exc
    if not isinstance(payload, dict):
        raise ClosureError(f"{path.relative_to(ROOT)}: top level must be object")
    return payload


def parse_backtick(value: str, label: str) -> str:
    if len(value) < 2 or not value.startswith("`") or not value.endswith("`"):
        raise ClosureError(f"closure table: {label} is not backtick-delimited: {value!r}")
    return value[1:-1]


def parse_closure_table() -> dict[tuple[str, str], dict[str, str]]:
    if not CLOSURE_DOC.is_file():
        raise ClosureError("controlling package closure document missing")
    records: dict[tuple[str, str], dict[str, str]] = {}
    for raw in CLOSURE_DOC.read_text(encoding="utf-8").splitlines():
        if not raw.startswith("| `"):
            continue
        columns = [item.strip() for item in raw.strip().strip("|").split("|")]
        if len(columns) != 9:
            raise ClosureError(f"closure table: expected 9 columns, found {len(columns)}")
        name = parse_backtick(columns[0], "package")
        version = parse_backtick(columns[1], "version")
        checksum = parse_backtick(columns[2], "checksum")
        repository = parse_backtick(columns[3], "repository")
        revision = parse_backtick(columns[4], "revision")
        source_path = parse_backtick(columns[5], "source path")
        expression = parse_backtick(columns[6], "license expression")
        adoption = parse_backtick(columns[7], "adoption basis")
        evidence = parse_backtick(columns[8], "license evidence")
        identity = (name, version)
        if identity in records:
            raise ClosureError(f"closure table: duplicate package {name} {version}")
        if not re.fullmatch(r"[0-9a-f]{64}", checksum):
            raise ClosureError(f"closure table: invalid checksum for {name} {version}")
        if not re.fullmatch(r"[0-9a-f]{40}", revision):
            raise ClosureError(f"closure table: invalid revision for {name} {version}")
        if adoption != "MIT":
            raise ClosureError(f"closure table: non-MIT selected Cargo adoption basis for {name} {version}")
        records[identity] = {
            "checksum": checksum,
            "repository": repository,
            "revision": revision,
            "source_path": source_path,
            "license_expression": expression,
            "selected_license": adoption,
            "license_evidence": evidence,
        }
    if len(records) != EXPECTED_PROVIDER_EXTERNAL_COUNT:
        raise ClosureError(
            f"closure table: expected {EXPECTED_PROVIDER_EXTERNAL_COUNT} provider packages, found {len(records)}"
        )
    return records


def check_direct_manifest() -> None:
    manifest = tomllib.loads(CORE_MANIFEST.read_text(encoding="utf-8"))
    dependencies = manifest.get("dependencies")
    if not isinstance(dependencies, dict):
        raise ClosureError("himsat-core: dependencies table missing")
    if set(dependencies) != set(EXPECTED_DIRECT):
        missing = sorted(set(EXPECTED_DIRECT).difference(dependencies))
        extras = sorted(set(dependencies).difference(EXPECTED_DIRECT))
        raise ClosureError(f"himsat-core: exact direct dependency set mismatch; missing={missing} extras={extras}")
    for name, expected in EXPECTED_DIRECT.items():
        observed = dependencies[name]
        if not isinstance(observed, dict):
            raise ClosureError(f"himsat-core: {name} must use an explicit dependency table")
        if observed != expected:
            raise ClosureError(f"himsat-core: {name} exact pin/features mismatch: observed={observed!r}")

    targets = manifest.get("target")
    expected_targets = {EXPECTED_APPLE_TARGET, EXPECTED_LINUX_TARGET, EXPECTED_WINDOWS_TARGET}
    if not isinstance(targets, dict) or set(targets) != expected_targets:
        raise ClosureError(
            f"himsat-core: exact target dependency table mismatch; observed={sorted(targets) if isinstance(targets, dict) else targets!r}"
        )
    apple = targets[EXPECTED_APPLE_TARGET]
    if not isinstance(apple, dict) or set(apple) != {"dependencies"}:
        raise ClosureError("himsat-core: Apple target must contain only dependencies")
    apple_dependencies = apple["dependencies"]
    if apple_dependencies != EXPECTED_APPLE_DIRECT:
        raise ClosureError(
            f"himsat-core: Apple target dependency pin/features mismatch: observed={apple_dependencies!r}"
        )
    linux = targets[EXPECTED_LINUX_TARGET]
    if not isinstance(linux, dict) or set(linux) != {"dependencies"}:
        raise ClosureError("himsat-core: Linux target must contain only dependencies")
    linux_dependencies = linux["dependencies"]
    if linux_dependencies != EXPECTED_LINUX_DIRECT:
        raise ClosureError(
            f"himsat-core: Linux target dependency pin/features mismatch: observed={linux_dependencies!r}"
        )
    windows = targets[EXPECTED_WINDOWS_TARGET]
    if not isinstance(windows, dict) or set(windows) != {"dependencies"}:
        raise ClosureError("himsat-core: Windows target must contain only dependencies")
    windows_dependencies = windows["dependencies"]
    if windows_dependencies != EXPECTED_WINDOWS_DIRECT:
        raise ClosureError(
            f"himsat-core: Windows target dependency pin/features mismatch: observed={windows_dependencies!r}"
        )


def lock_external() -> dict[tuple[str, str], dict[str, Any]]:
    lock = tomllib.loads(LOCK_PATH.read_text(encoding="utf-8"))
    packages = lock.get("package")
    if not isinstance(packages, list):
        raise ClosureError("Cargo.lock: package list missing")
    external = [pkg for pkg in packages if isinstance(pkg, dict) and pkg.get("source") is not None]
    if len(external) != EXPECTED_TOTAL_EXTERNAL_COUNT:
        raise ClosureError(
            f"Cargo.lock: expected {EXPECTED_TOTAL_EXTERNAL_COUNT} external packages, found {len(external)}"
        )
    result: dict[tuple[str, str], dict[str, Any]] = {}
    for pkg in external:
        identity = (pkg.get("name"), pkg.get("version"))
        if not all(isinstance(item, str) and item for item in identity):
            raise ClosureError("Cargo.lock: external package identity malformed")
        if identity in result:
            raise ClosureError(f"Cargo.lock: duplicate external identity {identity[0]} {identity[1]}")
        if pkg.get("source") != REGISTRY_SOURCE:
            raise ClosureError(f"Cargo.lock: unexpected source for {identity[0]} {identity[1]}")
        result[identity] = pkg
    return result


def check_lock_against_closure(
    closure: dict[tuple[str, str], dict[str, str]],
    external: dict[tuple[str, str], dict[str, Any]],
) -> None:
    selected = closure | EXPECTED_PLATFORM_PACKAGES
    if set(selected) != set(external):
        missing = sorted(set(selected).difference(external))
        extras = sorted(set(external).difference(selected))
        raise ClosureError(f"Cargo.lock: selected closure mismatch; missing={missing} extras={extras}")
    for identity, record in selected.items():
        if external[identity].get("checksum") != record["checksum"]:
            raise ClosureError(f"Cargo.lock: checksum drift for {identity[0]} {identity[1]}")


def check_registry(
    closure: dict[tuple[str, str], dict[str, str]],
    external: dict[tuple[str, str], dict[str, Any]],
) -> None:
    registry = load_json(REGISTRY_PATH)
    if set(registry) != {"schema", "entries"} or registry.get("schema") != REGISTRY_SCHEMA:
        raise ClosureError("registry: expected himsat.provenance-registry/v2")
    entries = registry.get("entries")
    if not isinstance(entries, list):
        raise ClosureError("registry: entries must be list")
    adopted_dependencies = [
        entry
        for entry in entries
        if isinstance(entry, dict)
        and entry.get("adopted") is True
        and entry.get("adoption_mode") == "depend"
        and entry.get("kind") == "dependency"
    ]
    if len(adopted_dependencies) != EXPECTED_TOTAL_EXTERNAL_COUNT:
        raise ClosureError(
            f"registry: expected {EXPECTED_TOTAL_EXTERNAL_COUNT} adopted dependencies, found {len(adopted_dependencies)}"
        )
    if len(entries) != EXPECTED_TOTAL_EXTERNAL_COUNT:
        raise ClosureError(
            f"registry: expected only the selected {EXPECTED_TOTAL_EXTERNAL_COUNT} dependency entries, found {len(entries)}"
        )

    by_identity: dict[tuple[str, str], dict[str, Any]] = {}
    native_seen: dict[str, tuple[tuple[str, str], dict[str, Any]]] = {}
    for entry in adopted_dependencies:
        package = entry.get("package")
        if not isinstance(package, dict):
            raise ClosureError(f"registry: package object missing for {entry.get('id')}")
        identity = (package.get("name"), package.get("version"))
        if not all(isinstance(item, str) and item for item in identity):
            raise ClosureError("registry: package identity malformed")
        if identity in by_identity:
            raise ClosureError(f"registry: duplicate package identity {identity[0]} {identity[1]}")
        by_identity[identity] = entry
        for native in entry.get("native_components", []):
            if not isinstance(native, dict) or not isinstance(native.get("id"), str):
                raise ClosureError(f"registry: malformed native component under {identity[0]} {identity[1]}")
            native_id = native["id"]
            if native_id in native_seen:
                raise ClosureError(f"registry: duplicate native component {native_id}")
            native_seen[native_id] = (identity, native)

    selected_closure = closure | EXPECTED_PLATFORM_PACKAGES
    if set(by_identity) != set(selected_closure):
        raise ClosureError("registry: adopted dependency identities differ from exact provider + platform closure")
    for identity, selected in selected_closure.items():
        entry = by_identity[identity]
        package = entry["package"]
        if package != {
            "ecosystem": "cargo",
            "name": identity[0],
            "version": identity[1],
            "source": REGISTRY_SOURCE,
            "checksum": selected["checksum"],
        }:
            raise ClosureError(f"registry: package tuple drift for {identity[0]} {identity[1]}")
        if entry.get("source_repository") != selected["repository"]:
            raise ClosureError(f"registry: repository drift for {identity[0]} {identity[1]}")
        if entry.get("source_revision") != selected["revision"]:
            raise ClosureError(f"registry: revision drift for {identity[0]} {identity[1]}")
        if entry.get("source_license") != selected["selected_license"]:
            raise ClosureError(f"registry: Cargo selected-license drift for {identity[0]} {identity[1]}")
        if package.get("checksum") != external[identity].get("checksum"):
            raise ClosureError(f"registry: checksum does not match lock for {identity[0]} {identity[1]}")

    if set(native_seen) != set(EXPECTED_NATIVE):
        raise ClosureError(
            f"registry: native component set mismatch; observed={sorted(native_seen)} expected={sorted(EXPECTED_NATIVE)}"
        )
    for native_id, expected in EXPECTED_NATIVE.items():
        parent, native = native_seen[native_id]
        if parent != expected["parent"]:
            raise ClosureError(f"registry: {native_id} attached to wrong parent {parent}")
        observed = {
            "name": native.get("name"),
            "source_repository": native.get("source_repository"),
            "source_revision": native.get("source_revision"),
            "source_license": native.get("source_license"),
            "source_paths": native.get("source_paths"),
            "embedded_paths": native.get("embedded_paths"),
            "evidence_reference": native.get("evidence_reference"),
        }
        comparable = {key: value for key, value in expected.items() if key != "parent"}
        if observed != comparable:
            raise ClosureError(f"registry: exact native identity drift for {native_id}: observed={observed!r}")


def check_evidence() -> None:
    if not REMEDIATION_DOC.is_file():
        raise ClosureError("P003/P004 remediation evidence missing")
    text = REMEDIATION_DOC.read_text(encoding="utf-8")
    required = (
        "FINAL_REMEDIATION_WORKFLOW_RUN = 34067945881",
        "EXTERNAL_REGISTRY_PACKAGES = 41",
        "PROVENANCE_GAPS = 0",
        "SQLCIPHER_SOURCE_REVISION = 778ab890cfc30c3631212dcceb0295498abdcd3e",
        "SQLITE_VERSION = 3.51.3",
        "OPENSSL_UPSTREAM_REVISION = aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f",
    )
    for marker in required:
        if marker not in text:
            raise ClosureError(f"P003/P004 evidence marker missing: {marker}")


def check_build_environment() -> None:
    if os.environ.get("OPENSSL_RUST_USE_NASM") != "0":
        raise ClosureError("build environment: OPENSSL_RUST_USE_NASM must equal 0")
    for name in REDIRECT_ENV:
        if os.environ.get(name):
            raise ClosureError(f"build environment: forbidden provider redirect is set: {name}")


def main() -> int:
    try:
        check_direct_manifest()
        closure = parse_closure_table()
        external = lock_external()
        check_lock_against_closure(closure, external)
        check_registry(closure, external)
        check_evidence()
        check_build_environment()
    except (ClosureError, OSError, tomllib.TOMLDecodeError) as exc:
        print(f"004P CLOSURE FAIL: {exc}", file=sys.stderr)
        return 1
    print("004P P005/P006 CLOSURE PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
