#!/usr/bin/env python3
"""Composite Himsat provenance gate with embedded native-component closure."""

from __future__ import annotations

import argparse
import ast
import copy
import json
from pathlib import Path
import re
import shutil
import sys
import tempfile
import tomllib
from typing import Any

import provenance as legacy

REGISTRY_SCHEMA = "himsat.provenance-registry/v2"
NATIVE_FIXTURE_SCHEMA = "himsat.native-provenance-fixtures/v1"
NATIVE_KEYS = {
    "id",
    "name",
    "source_repository",
    "source_revision",
    "source_paths",
    "source_license",
    "embedded_paths",
    "notice",
    "evidence_reference",
}
ENTRY_KEYS = legacy.ENTRY_KEYS | {"native_components"}
LOCAL_IMPORT_ROOTS = {"provenance"}


class GateError(legacy.GateError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    return legacy.load_json(path)


def validate_native_component(
    policy: dict[str, Any],
    component: Any,
    *,
    parent_id: str,
    seen_ids: set[str],
) -> dict[str, Any]:
    label = f"entry {parent_id} native component"
    if not isinstance(component, dict) or set(component) != NATIVE_KEYS:
        raise GateError(f"{label}: expected exactly {sorted(NATIVE_KEYS)}")

    component_id = component["id"]
    if not isinstance(component_id, str) or not re.fullmatch(r"[a-z0-9][a-z0-9._-]*", component_id):
        raise GateError(f"{label}: invalid id")
    if component_id in seen_ids:
        raise GateError(f"native components: duplicate id {component_id}")
    seen_ids.add(component_id)
    label = f"native component {component_id}"

    name = component["name"]
    if not isinstance(name, str) or not name.strip():
        raise GateError(f"{label}: name required")

    repository = component["source_repository"]
    if not isinstance(repository, str) or not repository.startswith("https://") or any(c.isspace() for c in repository):
        raise GateError(f"{label}: source_repository must be an https URL")

    revision = component["source_revision"]
    if not isinstance(revision, str) or not legacy.HEX_REVISION.fullmatch(revision):
        raise GateError(f"{label}: immutable source_revision required")

    source_paths = legacy.string_list(component["source_paths"], f"{label}.source_paths", nonempty=True)
    source_paths = [legacy.safe_repo_path(path, f"{label}.source_paths") for path in source_paths]
    embedded_paths = legacy.string_list(component["embedded_paths"], f"{label}.embedded_paths", nonempty=True)
    embedded_paths = [legacy.safe_repo_path(path, f"{label}.embedded_paths") for path in embedded_paths]

    license_id = component["source_license"]
    if not isinstance(license_id, str) or not license_id:
        raise GateError(f"{label}: source_license required")
    disposition = legacy.license_disposition(policy, license_id)
    if disposition != "allow":
        raise GateError(f"{label}: source license disposition is {disposition}")

    notice = component["notice"]
    if not isinstance(notice, str) or not notice.strip():
        raise GateError(f"{label}: notice text required")

    evidence_reference = legacy.safe_repo_path(component["evidence_reference"], f"{label}.evidence_reference")

    normalized = dict(component)
    normalized["source_paths"] = source_paths
    normalized["embedded_paths"] = embedded_paths
    normalized["evidence_reference"] = evidence_reference
    return normalized


def strip_for_legacy(registry: dict[str, Any]) -> dict[str, Any]:
    entries = []
    for entry in registry["entries"]:
        if not isinstance(entry, dict):
            entries.append(entry)
            continue
        stripped = {key: value for key, value in entry.items() if key != "native_components"}
        entries.append(stripped)
    return {"schema": legacy.REGISTRY_SCHEMA, "entries": entries}


def validate_registry(root: Path, policy: dict[str, Any], registry: dict[str, Any]) -> list[dict[str, Any]]:
    if set(registry) != {"schema", "entries"} or registry.get("schema") != REGISTRY_SCHEMA:
        raise GateError("registry: unsupported schema or fields")
    if not isinstance(registry["entries"], list):
        raise GateError("registry: entries must be a list")
    for entry in registry["entries"]:
        if not isinstance(entry, dict) or not set(entry).issubset(ENTRY_KEYS):
            raise GateError("registry entry: unsupported field")

    legacy_registry = strip_for_legacy(registry)
    try:
        normalized_legacy = legacy.validate_registry(root, policy, legacy_registry)
    except legacy.GateError as exc:
        raise GateError(str(exc)) from exc

    original_by_id = {
        entry.get("id"): entry
        for entry in registry["entries"]
        if isinstance(entry, dict) and isinstance(entry.get("id"), str)
    }
    seen_native_ids: set[str] = set()
    normalized: list[dict[str, Any]] = []
    for entry in normalized_legacy:
        original = original_by_id[entry["id"]]
        native_values = original.get("native_components", [])
        if not isinstance(native_values, list):
            raise GateError(f"entry {entry['id']}: native_components must be a list")
        if native_values and not (
            entry.get("adopted")
            and entry.get("adoption_mode") == "depend"
            and entry.get("kind") == "dependency"
        ):
            raise GateError(f"entry {entry['id']}: native_components require an adopted dependency")
        natives = [
            validate_native_component(
                policy,
                component,
                parent_id=entry["id"],
                seen_ids=seen_native_ids,
            )
            for component in native_values
        ]
        merged = dict(entry)
        if natives:
            merged["native_components"] = natives
        normalized.append(merged)
    return normalized


def validated_state(root: Path) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    policy = load_json(root / "governance/provenance/policy.json")
    registry = load_json(root / "governance/provenance/registry.json")
    try:
        legacy.validate_policy(policy)
    except legacy.GateError as exc:
        raise GateError(str(exc)) from exc
    return policy, validate_registry(root, policy, registry)


def render_notices(entries: list[dict[str, Any]]) -> str:
    lines = [
        "# Third-Party Notices",
        "",
        "This file is generated by `python tools/provenance_gate.py generate`. Do not edit manually.",
        "",
    ]
    adopted = sorted((entry for entry in entries if entry.get("adopted")), key=lambda item: item["id"])
    if not adopted:
        lines += ["No third-party adopted components are registered.", ""]
    for entry in adopted:
        lines += [
            f"## {entry['name']} (`{entry['id']}`)",
            "",
            f"- Kind: `{entry['kind']}`",
            f"- Adoption mode: `{entry['adoption_mode']}`",
            f"- Source: {entry['source_repository']}",
            f"- Revision: `{entry['source_revision']}`",
            f"- Source license: `{entry['source_license']}`",
        ]
        if entry.get("artifact_license"):
            lines.append(f"- Artifact license: `{entry['artifact_license']}`")
        if entry.get("package"):
            package = entry["package"]
            lines += [
                f"- Cargo package: `{package['name']} {package['version']}`",
                f"- Cargo checksum: `{package['checksum']}`",
            ]
        lines += ["", entry["notice"].strip(), ""]
        for component in sorted(entry.get("native_components", []), key=lambda item: item["id"]):
            embedded = ", ".join(f"`{path}`" for path in component["embedded_paths"])
            lines += [
                f"### Embedded native component: {component['name']} (`{component['id']}`)",
                "",
                f"- Source: {component['source_repository']}",
                f"- Revision: `{component['source_revision']}`",
                f"- Source license: `{component['source_license']}`",
                f"- Embedded path(s) in parent source: {embedded}",
                f"- Evidence: `{component['evidence_reference']}`",
                "",
                component["notice"].strip(),
                "",
            ]
    return "\n".join(lines)


def render_sbom(entries: list[dict[str, Any]]) -> str:
    components: list[dict[str, Any]] = []
    for entry in sorted((item for item in entries if item.get("adopted")), key=lambda item: item["id"]):
        component = {
            key: entry[key]
            for key in (
                "id",
                "name",
                "kind",
                "adoption_mode",
                "source_repository",
                "source_revision",
                "source_paths",
                "source_license",
            )
        }
        for key in ("artifact_license", "destination_paths", "artifacts", "package"):
            if entry.get(key):
                component[key] = entry[key]
        if entry.get("native_components"):
            component["native_components"] = [
                {
                    key: native[key]
                    for key in (
                        "id",
                        "name",
                        "source_repository",
                        "source_revision",
                        "source_paths",
                        "source_license",
                        "embedded_paths",
                        "evidence_reference",
                    )
                }
                for native in sorted(entry["native_components"], key=lambda item: item["id"])
            ]
        components.append(component)
    payload = {
        "schema": legacy.SBOM_SCHEMA,
        "generated_from": "governance/provenance/registry.json",
        "components": components,
    }
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


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
        roots: set[str]
        if isinstance(node, ast.Import):
            roots = {alias.name.split(".", 1)[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            roots = {node.module.split(".", 1)[0]} if node.module else set()
        else:
            roots = set()
        for root in roots:
            if root in LOCAL_IMPORT_ROOTS:
                continue
            if root not in sys.stdlib_module_names:
                raise GateError(f"tool audit: non-stdlib import {root}")
            if root in legacy.BANNED_IMPORT_ROOTS:
                raise GateError(f"tool audit: forbidden execution/network import {root}")
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id in legacy.BANNED_CALLS:
            raise GateError(f"tool audit: forbidden dynamic execution call {node.func.id}")


def run_legacy_self_test(root: Path) -> None:
    """Run v1 fixtures against an isolated dependency-free canonical sandbox.

    The real v2 registry and 41-package lock are validated separately before
    this function. Legacy fixtures exercise v1 policy behavior and must not
    inherit unrelated canonical dependencies from the current workspace.
    """
    with tempfile.TemporaryDirectory() as temp:
        legacy_root = Path(temp)
        paths = [
            "tools/provenance.py",
            "governance/provenance/policy.json",
            "governance/provenance/fixtures/cases.json",
            "Cargo.toml",
        ]
        workspace = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]
        paths.extend(f"{member}/Cargo.toml" for member in workspace.get("members", []))
        for relative in paths:
            source = root / relative
            target = legacy_root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, target)

        internal = sorted(legacy.workspace_identities(legacy_root))
        lock_lines = [
            "# This file is automatically @generated by Cargo.",
            "# It is not intended for manual editing.",
            "version = 4",
            "",
        ]
        for name, version in internal:
            lock_lines += [
                "[[package]]",
                f'name = "{name}"',
                f'version = "{version}"',
                "",
            ]
        (legacy_root / "Cargo.lock").write_text("\n".join(lock_lines), encoding="utf-8")

        empty_registry = {"schema": legacy.REGISTRY_SCHEMA, "entries": []}
        target = legacy_root / "governance/provenance/registry.json"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(empty_registry, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        try:
            legacy.self_test(legacy_root)
        except legacy.GateError as exc:
            raise GateError(f"legacy self-test failed: {exc}") from exc


def run_native_fixture_matrix(root: Path, policy: dict[str, Any]) -> None:
    payload = load_json(root / "governance/provenance/fixtures/native-cases.json")
    if set(payload) != {"schema", "cases"} or payload["schema"] != NATIVE_FIXTURE_SCHEMA:
        raise GateError("native fixtures: unsupported schema")
    cases = payload["cases"]
    if not isinstance(cases, list) or not cases:
        raise GateError("native fixtures: cases must be a non-empty list")
    for case in cases:
        if not isinstance(case, dict) or not {"name", "expected", "component"}.issubset(case):
            raise GateError("native fixtures: malformed case")
        error = None
        try:
            validate_native_component(
                policy,
                copy.deepcopy(case["component"]),
                parent_id="fixture-parent",
                seen_ids=set(),
            )
        except (GateError, legacy.GateError) as exc:
            error = str(exc)
        if case["expected"] == "pass":
            if error is not None:
                raise GateError(f"native fixture {case['name']} unexpectedly failed: {error}")
        elif case["expected"] == "fail":
            if error is None:
                raise GateError(f"native fixture {case['name']} unexpectedly passed")
            fragment = case.get("error_contains")
            if fragment and fragment not in error:
                raise GateError(f"native fixture {case['name']} wrong error: {error}")
        else:
            raise GateError(f"native fixture {case['name']}: expected must be pass/fail")


def self_test(root: Path) -> None:
    audit_tool_source(root / "tools/provenance_gate.py")
    legacy.audit_tool_source(root / "tools/provenance.py")
    policy = load_json(root / "governance/provenance/policy.json")
    registry = load_json(root / "governance/provenance/registry.json")
    legacy.validate_policy(policy)
    validate_registry(root, policy, registry)
    run_legacy_self_test(root)
    run_native_fixture_matrix(root, policy)
    first = generated_bytes(root)
    second = generated_bytes(root)
    if first != second:
        raise GateError("generation is not deterministic")
    with tempfile.TemporaryDirectory() as temp:
        scratch = Path(temp)
        notices, sbom = first
        (scratch / "governance/generated").mkdir(parents=True)
        (scratch / "THIRD_PARTY_NOTICES.md").write_text(notices + "drift\n", encoding="utf-8")
        (scratch / "governance/generated/sbom.json").write_text(sbom, encoding="utf-8")
        if (scratch / "THIRD_PARTY_NOTICES.md").read_text(encoding="utf-8") == notices:
            raise GateError("generated drift negative control unexpectedly passed")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "command",
        choices=("validate", "render-notices", "render-sbom", "generate", "check-generated", "self-test"),
    )
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = args.root.resolve()
    try:
        if args.command == "validate":
            audit_tool_source(root / "tools/provenance_gate.py")
            legacy.audit_tool_source(root / "tools/provenance.py")
            validated_state(root)
            print("PROVENANCE V2 PASS")
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
    except (GateError, legacy.GateError, OSError, tomllib.TOMLDecodeError, json.JSONDecodeError) as exc:
        print(f"PROVENANCE V2 FAIL: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
