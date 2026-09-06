#!/usr/bin/env python3
"""Offline provenance, license, notice, and Himsat SBOM gate."""

from __future__ import annotations

import argparse
import ast
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import sys
import tempfile
import tomllib
from typing import Any

POLICY_SCHEMA = "himsat.provenance-policy/v1"
REGISTRY_SCHEMA = "himsat.provenance-registry/v1"
SBOM_SCHEMA = "himsat.sbom/v1"
HEX_REVISION = re.compile(r"^(?:[0-9a-f]{40}|[0-9a-f]{64})$")
HEX_SHA256 = re.compile(r"^[0-9a-f]{64}$")
ENTRY_KEYS = {
    "id", "name", "kind", "adoption_mode", "adopted", "source_repository",
    "source_revision", "source_paths", "source_license", "artifact_license",
    "destination_paths", "artifacts", "package", "notice",
}
BANNED_IMPORT_ROOTS = {"ftplib", "http", "os", "socket", "ssl", "subprocess", "urllib"}
BANNED_CALLS = {"__import__", "compile", "eval", "exec"}


class GateError(ValueError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise GateError(f"{path}: invalid JSON: {exc}") from exc
    if not isinstance(value, dict):
        raise GateError(f"{path}: top level must be an object")
    return value


def safe_repo_path(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value or "\\" in value or "\x00" in value:
        raise GateError(f"{label}: unsafe repository path")
    path = PurePosixPath(value)
    if path.is_absolute() or value.startswith("~") or any(part in {"", ".", ".."} for part in path.parts):
        raise GateError(f"{label}: unsafe repository path")
    return value


def string_list(value: Any, label: str, *, nonempty: bool = False) -> list[str]:
    if not isinstance(value, list) or (nonempty and not value):
        raise GateError(f"{label}: expected {'non-empty ' if nonempty else ''}list")
    if not all(isinstance(item, str) and item for item in value):
        raise GateError(f"{label}: every item must be a non-empty string")
    return value


def validate_policy(policy: dict[str, Any]) -> None:
    if set(policy) != {"schema", "license_dispositions", "kinds", "adoption_modes"}:
        raise GateError("policy: unsupported or missing fields")
    if policy["schema"] != POLICY_SCHEMA:
        raise GateError("policy: unsupported schema")
    dispositions = policy["license_dispositions"]
    if not isinstance(dispositions, dict) or set(dispositions) != {"allow", "manual", "deny"}:
        raise GateError("policy: license_dispositions must contain allow/manual/deny")
    seen: set[str] = set()
    for name in ("allow", "manual", "deny"):
        values = string_list(dispositions[name], f"policy.{name}")
        if values != sorted(values):
            raise GateError(f"policy.{name}: values must be sorted")
        overlap = seen.intersection(values)
        if overlap:
            raise GateError(f"policy: duplicate license classification: {sorted(overlap)[0]}")
        seen.update(values)
    if policy["kinds"] != ["code", "dependency", "model", "dataset", "font", "asset"]:
        raise GateError("policy: unexpected kinds")
    if policy["adoption_modes"] != ["copy", "depend", "vendor", "reference"]:
        raise GateError("policy: unexpected adoption_modes")


def license_disposition(policy: dict[str, Any], license_id: str) -> str:
    for disposition in ("allow", "manual", "deny"):
        if license_id in policy["license_dispositions"][disposition]:
            return disposition
    return "unknown"


def validate_artifacts(root: Path, artifacts: Any, label: str) -> list[dict[str, str]]:
    if not isinstance(artifacts, list):
        raise GateError(f"{label}: artifacts must be a list")
    result: list[dict[str, str]] = []
    seen: set[str] = set()
    for index, item in enumerate(artifacts):
        if not isinstance(item, dict) or set(item) != {"path", "sha256"}:
            raise GateError(f"{label}.artifacts[{index}]: expected path and sha256 only")
        path = safe_repo_path(item["path"], f"{label}.artifacts[{index}].path")
        digest = item["sha256"]
        if not isinstance(digest, str) or not HEX_SHA256.fullmatch(digest):
            raise GateError(f"{label}.artifacts[{index}].sha256: invalid SHA-256")
        if path in seen:
            raise GateError(f"{label}: duplicate artifact path {path}")
        seen.add(path)
        candidate = (root / path).resolve()
        try:
            candidate.relative_to(root.resolve())
        except ValueError as exc:
            raise GateError(f"{label}: artifact escapes repository root") from exc
        if not candidate.is_file():
            raise GateError(f"{label}: adopted artifact missing: {path}")
        observed = hashlib.sha256(candidate.read_bytes()).hexdigest()
        if observed != digest:
            raise GateError(f"{label}: digest mismatch for {path}")
        result.append({"path": path, "sha256": digest})
    return result


def validate_package(value: Any, label: str) -> dict[str, Any]:
    required = {"ecosystem", "name", "version", "source", "checksum"}
    if not isinstance(value, dict) or set(value) != required:
        raise GateError(f"{label}.package: expected {sorted(required)}")
    if value["ecosystem"] != "cargo":
        raise GateError(f"{label}.package: only cargo is supported")
    for key in ("name", "version", "source"):
        if not isinstance(value[key], str) or not value[key]:
            raise GateError(f"{label}.package.{key}: non-empty string required")
    checksum = value["checksum"]
    if checksum is not None and (not isinstance(checksum, str) or not HEX_SHA256.fullmatch(checksum)):
        raise GateError(f"{label}.package.checksum: invalid SHA-256")
    return dict(value)


def validate_entry(root: Path, policy: dict[str, Any], entry: Any, seen_ids: set[str]) -> dict[str, Any]:
    if not isinstance(entry, dict) or not set(entry).issubset(ENTRY_KEYS):
        raise GateError("registry entry: unsupported field")
    required = {
        "id", "name", "kind", "adoption_mode", "adopted", "source_repository",
        "source_license", "source_paths", "destination_paths", "artifacts",
    }
    missing = required.difference(entry)
    if missing:
        raise GateError(f"registry entry: missing field {sorted(missing)[0]}")
    entry_id = entry["id"]
    if not isinstance(entry_id, str) or not re.fullmatch(r"[a-z0-9][a-z0-9._-]*", entry_id):
        raise GateError("registry entry: invalid id")
    if entry_id in seen_ids:
        raise GateError(f"registry: duplicate id {entry_id}")
    seen_ids.add(entry_id)
    label = f"entry {entry_id}"
    if not isinstance(entry["name"], str) or not entry["name"]:
        raise GateError(f"{label}: name required")
    kind = entry["kind"]
    mode = entry["adoption_mode"]
    if kind not in policy["kinds"] or mode not in policy["adoption_modes"]:
        raise GateError(f"{label}: unknown kind or adoption mode")
    if not isinstance(entry["adopted"], bool):
        raise GateError(f"{label}: adopted must be boolean")
    repository = entry["source_repository"]
    if not isinstance(repository, str) or not repository.startswith("https://") or any(c.isspace() for c in repository):
        raise GateError(f"{label}: source_repository must be an https URL")
    source_paths = string_list(entry["source_paths"], f"{label}.source_paths", nonempty=mode != "reference")
    source_paths = [safe_repo_path(path, f"{label}.source_paths") for path in source_paths]
    destinations = string_list(entry["destination_paths"], f"{label}.destination_paths")
    destinations = [safe_repo_path(path, f"{label}.destination_paths") for path in destinations]
    license_id = entry["source_license"]
    if not isinstance(license_id, str) or not license_id:
        raise GateError(f"{label}: source_license required")

    if mode == "reference":
        if entry["adopted"] or destinations or entry["artifacts"] or entry.get("package") is not None:
            raise GateError(f"{label}: reference entries cannot adopt or distribute bytes")
        return dict(entry)

    if not entry["adopted"]:
        raise GateError(f"{label}: non-reference entries must be explicitly adopted")
    revision = entry.get("source_revision")
    if not isinstance(revision, str) or not HEX_REVISION.fullmatch(revision):
        raise GateError(f"{label}: immutable source_revision required")
    disposition = license_disposition(policy, license_id)
    if disposition != "allow":
        raise GateError(f"{label}: source license disposition is {disposition}")
    notice = entry.get("notice")
    if not isinstance(notice, str) or not notice.strip():
        raise GateError(f"{label}: notice text required for adopted entry")

    if mode == "depend":
        if kind != "dependency":
            raise GateError(f"{label}: depend mode requires dependency kind")
        if destinations or entry["artifacts"]:
            raise GateError(f"{label}: dependency entry cannot claim copied artifacts")
        package = validate_package(entry.get("package"), label)
    else:
        if kind == "dependency":
            raise GateError(f"{label}: dependency kind requires depend mode")
        if not destinations:
            raise GateError(f"{label}: copy/vendor requires destination_paths")
        artifacts = validate_artifacts(root, entry["artifacts"], label)
        if sorted(destinations) != sorted(item["path"] for item in artifacts):
            raise GateError(f"{label}: destination_paths must exactly match artifact paths")
        package = None

    if kind in {"model", "dataset", "font", "asset"}:
        artifact_license = entry.get("artifact_license")
        if not isinstance(artifact_license, str) or not artifact_license:
            raise GateError(f"{label}: independent artifact_license required")
        artifact_disposition = license_disposition(policy, artifact_license)
        if artifact_disposition != "allow":
            raise GateError(f"{label}: artifact license disposition is {artifact_disposition}")
        if mode in {"copy", "vendor"} and not entry["artifacts"]:
            raise GateError(f"{label}: artifact digest evidence required")

    normalized = dict(entry)
    if package is not None:
        normalized["package"] = package
    return normalized


def workspace_identities(root: Path) -> set[tuple[str, str]]:
    manifest = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    workspace = manifest.get("workspace")
    if not isinstance(workspace, dict):
        raise GateError("Cargo.toml: workspace table required")
    inherited_version = workspace.get("package", {}).get("version")
    result: set[tuple[str, str]] = set()
    for member in workspace.get("members", []):
        path = safe_repo_path(member, "Cargo.toml workspace member")
        package = tomllib.loads((root / path / "Cargo.toml").read_text(encoding="utf-8")).get("package", {})
        name = package.get("name")
        version = package.get("version")
        if isinstance(version, dict) and version.get("workspace") is True:
            version = inherited_version
        if not isinstance(name, str) or not isinstance(version, str):
            raise GateError(f"{path}/Cargo.toml: package name/version unavailable")
        result.add((name, version))
    return result


def validate_cargo_closure(root: Path, entries: list[dict[str, Any]]) -> None:
    lock = tomllib.loads((root / "Cargo.lock").read_text(encoding="utf-8"))
    packages = lock.get("package", [])
    if not isinstance(packages, list):
        raise GateError("Cargo.lock: package array unavailable")
    internal = workspace_identities(root)
    external = [pkg for pkg in packages if (pkg.get("name"), pkg.get("version")) not in internal]
    registered = {
        (entry["package"]["name"], entry["package"]["version"]): entry["package"]
        for entry in entries
        if entry.get("adopted") and entry.get("adoption_mode") == "depend"
    }
    for pkg in external:
        identity = (pkg.get("name"), pkg.get("version"))
        if identity not in registered:
            raise GateError(f"Cargo.lock: unregistered external dependency {identity[0]} {identity[1]}")
        expected = registered[identity]
        if pkg.get("source") != expected["source"]:
            raise GateError(f"Cargo.lock: source mismatch for {identity[0]} {identity[1]}")
        if pkg.get("checksum") != expected["checksum"]:
            raise GateError(f"Cargo.lock: checksum mismatch for {identity[0]} {identity[1]}")
    lock_ids = {(pkg.get("name"), pkg.get("version")) for pkg in external}
    extras = set(registered).difference(lock_ids)
    if extras:
        name, version = sorted(extras)[0]
        raise GateError(f"registry: dependency not present in Cargo.lock: {name} {version}")


def validate_registry(root: Path, policy: dict[str, Any], registry: dict[str, Any]) -> list[dict[str, Any]]:
    if set(registry) != {"schema", "entries"} or registry["schema"] != REGISTRY_SCHEMA:
        raise GateError("registry: unsupported schema or fields")
    if not isinstance(registry["entries"], list):
        raise GateError("registry: entries must be a list")
    seen: set[str] = set()
    entries = [validate_entry(root, policy, entry, seen) for entry in registry["entries"]]
    validate_cargo_closure(root, entries)
    return entries


def render_notices(entries: list[dict[str, Any]]) -> str:
    lines = [
        "# Third-Party Notices", "",
        "This file is generated by `python tools/provenance.py generate`. Do not edit manually.", "",
    ]
    adopted = sorted((entry for entry in entries if entry.get("adopted")), key=lambda item: item["id"])
    if not adopted:
        lines += ["No third-party adopted components are registered.", ""]
    for entry in adopted:
        lines += [
            f"## {entry['name']} (`{entry['id']}`)", "",
            f"- Kind: `{entry['kind']}`",
            f"- Adoption mode: `{entry['adoption_mode']}`",
            f"- Source: {entry['source_repository']}",
            f"- Revision: `{entry['source_revision']}`",
            f"- Source license: `{entry['source_license']}`",
        ]
        if entry.get("artifact_license"):
            lines.append(f"- Artifact license: `{entry['artifact_license']}`")
        lines += ["", entry["notice"].strip(), ""]
    return "\n".join(lines)


def render_sbom(entries: list[dict[str, Any]]) -> str:
    components: list[dict[str, Any]] = []
    for entry in sorted((item for item in entries if item.get("adopted")), key=lambda item: item["id"]):
        component = {
            key: entry[key]
            for key in (
                "id", "name", "kind", "adoption_mode", "source_repository",
                "source_revision", "source_paths", "source_license",
            )
        }
        for key in ("artifact_license", "destination_paths", "artifacts", "package"):
            if entry.get(key):
                component[key] = entry[key]
        components.append(component)
    payload = {
        "schema": SBOM_SCHEMA,
        "generated_from": "governance/provenance/registry.json",
        "components": components,
    }
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def validated_state(root: Path, policy_path: Path | None = None, registry_path: Path | None = None) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    policy = load_json(policy_path or root / "governance/provenance/policy.json")
    registry = load_json(registry_path or root / "governance/provenance/registry.json")
    validate_policy(policy)
    return policy, validate_registry(root, policy, registry)


def generated_bytes(root: Path) -> tuple[str, str]:
    _, entries = validated_state(root)
    return render_notices(entries), render_sbom(entries)


def check_generated(root: Path) -> None:
    notices, sbom = generated_bytes(root)
    expected = {
        root / "THIRD_PARTY_NOTICES.md": notices,
        root / "governance/generated/sbom.json": sbom,
    }
    for path, content in expected.items():
        if not path.is_file() or path.read_text(encoding="utf-8") != content:
            raise GateError(f"generated output drift: {path.relative_to(root)}")


def audit_tool_source(path: Path) -> None:
    tree = ast.parse(path.read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            roots = {alias.name.split(".", 1)[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            roots = {node.module.split(".", 1)[0]} if node.module else set()
        else:
            roots = set()
        for root in roots:
            if root not in sys.stdlib_module_names:
                raise GateError(f"tool audit: non-stdlib import {root}")
            if root in BANNED_IMPORT_ROOTS:
                raise GateError(f"tool audit: forbidden execution/network import {root}")
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id in BANNED_CALLS:
            raise GateError(f"tool audit: forbidden dynamic execution call {node.func.id}")


def self_test(root: Path) -> None:
    audit_tool_source(root / "tools/provenance.py")
    cases = load_json(root / "governance/provenance/fixtures/cases.json").get("cases")
    if not isinstance(cases, list) or not cases:
        raise GateError("fixtures: cases must be a non-empty list")
    canonical_policy = (root / "governance/provenance/policy.json").read_text(encoding="utf-8")
    canonical_workspace = (root / "Cargo.toml").read_text(encoding="utf-8")
    workspace = tomllib.loads(canonical_workspace).get("workspace", {})
    member_manifests: dict[str, str] = {}
    for member in workspace.get("members", []):
        member = safe_repo_path(member, "Cargo.toml workspace member")
        relative = f"{member}/Cargo.toml"
        member_manifests[relative] = (root / relative).read_text(encoding="utf-8")
    canonical_lock = (root / "Cargo.lock").read_text(encoding="utf-8")
    for case in cases:
        if not isinstance(case, dict) or not {"name", "expected", "registry"}.issubset(case):
            raise GateError("fixtures: malformed case")
        with tempfile.TemporaryDirectory() as temp:
            fixture_root = Path(temp)
            (fixture_root / "governance/provenance").mkdir(parents=True)
            (fixture_root / "governance/generated").mkdir(parents=True)
            (fixture_root / "governance/provenance/policy.json").write_text(canonical_policy, encoding="utf-8")
            (fixture_root / "governance/provenance/registry.json").write_text(
                json.dumps(case["registry"], indent=2, sort_keys=True) + "\n", encoding="utf-8"
            )
            (fixture_root / "Cargo.toml").write_text(canonical_workspace, encoding="utf-8")
            for relative, content in member_manifests.items():
                path = fixture_root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
            (fixture_root / "Cargo.lock").write_text(case.get("cargo_lock", canonical_lock), encoding="utf-8")
            for relative, content in case.get("files", {}).items():
                path = fixture_root / safe_repo_path(relative, "fixture file")
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
            error = None
            try:
                _, entries = validated_state(fixture_root)
                first = (render_notices(entries), render_sbom(entries))
                second = (render_notices(entries), render_sbom(entries))
                if first != second:
                    raise GateError("generation is not deterministic")
            except GateError as exc:
                error = str(exc)
            if case["expected"] == "pass":
                if error is not None:
                    raise GateError(f"fixture {case['name']} unexpectedly failed: {error}")
            elif case["expected"] == "fail":
                if error is None:
                    raise GateError(f"fixture {case['name']} unexpectedly passed")
                fragment = case.get("error_contains")
                if fragment and fragment not in error:
                    raise GateError(f"fixture {case['name']} wrong error: {error}")
            else:
                raise GateError(f"fixture {case['name']}: expected must be pass/fail")

    notices, sbom = generated_bytes(root)
    with tempfile.TemporaryDirectory() as temp:
        copy_root = Path(temp)
        (copy_root / "governance/generated").mkdir(parents=True)
        (copy_root / "governance/provenance").mkdir(parents=True)
        canonical_files = {
            "governance/provenance/policy.json": canonical_policy,
            "governance/provenance/registry.json": (root / "governance/provenance/registry.json").read_text(encoding="utf-8"),
            "Cargo.toml": canonical_workspace,
            "Cargo.lock": canonical_lock,
            **member_manifests,
        }
        for relative, content in canonical_files.items():
            target = copy_root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(content, encoding="utf-8")
        (copy_root / "THIRD_PARTY_NOTICES.md").write_text(notices + "drift\n", encoding="utf-8")
        (copy_root / "governance/generated/sbom.json").write_text(sbom, encoding="utf-8")
        try:
            check_generated(copy_root)
        except GateError as exc:
            if "generated output drift" not in str(exc):
                raise
        else:
            raise GateError("generated drift negative control unexpectedly passed")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("validate", "render-notices", "render-sbom", "generate", "check-generated", "self-test"))
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = args.root.resolve()
    try:
        if args.command == "validate":
            audit_tool_source(root / "tools/provenance.py")
            validated_state(root)
            print("PROVENANCE PASS")
        elif args.command == "render-notices":
            _, entries = validated_state(root)
            sys.stdout.write(render_notices(entries))
        elif args.command == "render-sbom":
            _, entries = validated_state(root)
            sys.stdout.write(render_sbom(entries))
        elif args.command == "generate":
            notices, sbom = generated_bytes(root)
            (root / "governance/generated").mkdir(parents=True, exist_ok=True)
            (root / "THIRD_PARTY_NOTICES.md").write_text(notices, encoding="utf-8")
            (root / "governance/generated/sbom.json").write_text(sbom, encoding="utf-8")
            print("GENERATED")
        elif args.command == "check-generated":
            check_generated(root)
            print("GENERATED OUTPUTS PASS")
        else:
            self_test(root)
            print("SELF-TEST PASS")
    except (GateError, OSError, tomllib.TOMLDecodeError) as exc:
        print(f"PROVENANCE FAIL: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())