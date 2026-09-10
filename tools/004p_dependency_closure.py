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
EXPECTED_TOTAL_EXTERNAL_COUNT = 59
EVIDENCE_REFERENCE = "specs/004-vault-key-crypto/provider-package-source-license-review-remediation.md"

EXPECTED_APPLE_TARGET = 'cfg(target_os = "macos")'
EXPECTED_APPLE_DIRECT: dict[str, dict[str, Any]] = {
    "security-framework": {
        "version": "=3.7.0",
        "default-features": False,
        "features": ["OSX_10_15"],
    },
}

EXPECTED_WINDOWS_TARGET = 'cfg(target_os = "windows")'
EXPECTED_WINDOWS_DIRECT: dict[str, dict[str, Any]] = {
    "windows-acl": {"version": "=0.3.0"},
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
}

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
    expected_targets = {EXPECTED_APPLE_TARGET, EXPECTED_WINDOWS_TARGET}
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
        if entry.get("source_license") != "MIT":
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
