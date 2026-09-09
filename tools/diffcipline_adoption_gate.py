#!/usr/bin/env python3
"""Fail-closed bridge for the one-time Specification 004P adoption diff.

Repository Diffcipline policy remains conservative. The historical 004P
P005/P006 adoption diff is exceptional because it intentionally introduces the
reviewed dependency graph and generated provenance closure in one bounded leaf.
This bridge accepts Diffcipline's otherwise-blocking REVIEW/size findings only
for that exact base, exact changed-path set, and exact dependency-manifest and
lockfile blobs. Every configured verification still has to pass.

After the adoption merge the comparison base changes, so this exception cannot
approve later dependency or oversized diffs.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys
from typing import Any

ADOPTION_BASE = "0e6a3422bfe4a18a248d66027f5f538fd68542d3"
MAX_ADOPTION_ADDED_LINES = 5500
POLICY_MAX_ADDED_LINES = 900
EXPECTED_BLOBS = {
    "Cargo.lock": "070dea51697a6b6159627f7ac0140b43ae9c636a",
    "crates/himsat-core/Cargo.toml": "2e5efcd43b82c5ed88b6246ae62014a40a55f145",
}
B401_BASE = "ba32dfc21d025a189cddfdd9f46c48fdcd327e1e"
B401_MAX_ADDED_LINES = 1600
B401_EXPECTED_BLOBS = {
    "Cargo.lock": "d9c5aac949ec0b0b7ec12d7baa4311a46ee79f41",
    "crates/himsat-core/Cargo.toml": "1dc4650087687e4d41a5bf40da813e256d2f2a63",
    "governance/provenance/registry.json": "ebe0264d0b318e757d89d214138cb1c6ae4c2c66",
    "governance/generated/sbom.json": "dfb107acb6d647979903098518db357a1ffd1741",
    "THIRD_PARTY_NOTICES.md": "721a7022939eafdf6e5203c4ec1c57676bacc409",
    "tools/004p_dependency_closure.py": "a0d1e254675aa5ff2721eaa6e269670b989bfe40",
}
B401_EXPECTED_FILES = {
    "Cargo.lock",
    "THIRD_PARTY_NOTICES.md",
    "crates/himsat-core/Cargo.toml",
    "crates/himsat-core/examples/b401_apple_keychain_probe.rs",
    "crates/himsat-core/src/lib.rs",
    "crates/himsat-core/src/vault_apple_keychain.rs",
    "governance/generated/sbom.json",
    "governance/provenance/registry.json",
    "specs/004-vault-key-crypto/b401-apple-keychain-candidate-evidence.md",
    "tools/004p_dependency_closure.py",
    "tools/diffcipline_adoption_gate.py",
}

EXPECTED_ADOPTION_FILES = {
    ".diffcipline.toml",
    ".github/workflows/ci.yml",
    ".github/workflows/r3-security.yml",
    ".gitignore",
    "Cargo.lock",
    "THIRD_PARTY_NOTICES.md",
    "crates/himsat-core/Cargo.toml",
    "governance/generated/sbom.json",
    "governance/provenance/README.md",
    "governance/provenance/fixtures/native-cases.json",
    "governance/provenance/policy.json",
    "governance/provenance/registry.json",
    "specs/004-vault-key-crypto/provider-adoption-closure-evidence.md",
    "tools/004p_dependency_closure.py",
    "tools/diffcipline_adoption_gate.py",
    "tools/provenance_gate.py",
}


def fail(message: str) -> int:
    print(f"DIFFCIPLINE ADOPTION GATE FAIL: {message}", file=sys.stderr)
    return 1


def git(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    return completed.stdout.strip()


def load_proof(path: Path) -> dict[str, Any]:
    lines = [line.strip() for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]
    if not lines:
        raise ValueError("proof output is empty")
    payload = json.loads(lines[-1])
    if not isinstance(payload, dict):
        raise ValueError("final proof line must be a JSON object")
    return payload


def actual_git_diff(base: str) -> tuple[list[str], int, int]:
    raw_paths = git("diff", "--no-renames", "--name-only", base, "HEAD")
    paths = [line for line in raw_paths.splitlines() if line]

    raw_numstat = git("diff", "--no-renames", "--numstat", base, "HEAD")
    added = 0
    deleted = 0
    rows = 0
    for line in raw_numstat.splitlines():
        if not line:
            continue
        fields = line.split("\t", 2)
        if len(fields) != 3:
            raise ValueError(f"unexpected git numstat row: {line!r}")
        added_text, deleted_text, _path = fields
        if added_text == "-" or deleted_text == "-":
            raise ValueError("binary diff cannot use the 004P adoption exception")
        try:
            added += int(added_text)
            deleted += int(deleted_text)
        except ValueError as exc:
            raise ValueError(f"invalid git numstat row: {line!r}") from exc
        rows += 1

    if rows != len(paths):
        raise ValueError(
            f"git diff path/numstat count mismatch: paths={len(paths)} numstat={rows}"
        )
    return paths, added, deleted


def check_b401_exception(args: argparse.Namespace, proof: dict[str, Any]) -> int:
    """Accept only the exact B401 candidate dependency-adoption diff.

    This bridge qualifies repository review mechanics only. The exact path set
    deliberately excludes `tasks.md` and `specs/CURRENT.md`, so this exception
    cannot mark B401 complete or advance B402 authority. Signed native Apple
    runtime evidence remains a separate controlling requirement.
    """

    if args.exit_code != 2 or proof.get("verdict") != "FAIL":
        return fail(
            f"unexpected B401 non-PASS result: exit={args.exit_code} verdict={proof.get('verdict')}"
        )

    try:
        actual_files, actual_added, actual_deleted = actual_git_diff(args.base)
    except (subprocess.CalledProcessError, ValueError) as exc:
        return fail(f"cannot reconcile actual B401 Git diff: {exc}")

    if set(actual_files) != B401_EXPECTED_FILES or len(actual_files) != len(B401_EXPECTED_FILES):
        return fail("actual B401 Git changed-path set is not exact")

    files = proof.get("files")
    if not isinstance(files, list) or files != actual_files:
        return fail("B401 Diffcipline proof path list does not equal the actual Git diff")
    if set(files) != B401_EXPECTED_FILES or len(files) != len(B401_EXPECTED_FILES):
        return fail("B401 exception changed-path set is not exact")

    changed_files = proof.get("changed_files")
    if changed_files != len(actual_files) or changed_files != len(B401_EXPECTED_FILES):
        return fail("B401 Diffcipline changed-file count is not exact")

    added = proof.get("added_lines")
    if added != actual_added:
        return fail("B401 Diffcipline added-line count does not equal the actual Git diff")
    if not isinstance(added, int) or not (POLICY_MAX_ADDED_LINES < added <= B401_MAX_ADDED_LINES):
        return fail("B401 exception added-line count is outside the bounded window")

    deleted = proof.get("deleted_lines")
    if deleted != actual_deleted:
        return fail("B401 Diffcipline deleted-line count does not equal the actual Git diff")

    expected_reasons = {
        f"added lines {added} exceed maximum {POLICY_MAX_ADDED_LINES}",
        "dependency manifest changed: crates/himsat-core/Cargo.toml",
        "lockfile changed: Cargo.lock",
    }
    reasons = proof.get("reasons")
    if not isinstance(reasons, list) or set(reasons) != expected_reasons or len(reasons) != len(expected_reasons):
        return fail(f"unexpected B401 Diffcipline reason set: {reasons!r}")

    if proof.get("scope_violations") != []:
        return fail("B401 scope violations cannot be excepted")

    verification = proof.get("verification")
    if not isinstance(verification, list) or not verification:
        return fail("B401 verification evidence is missing")
    for result in verification:
        if not isinstance(result, dict) or result.get("state") != "PASS":
            return fail(f"B401 verification is not PASS: {result!r}")

    try:
        for path, expected in B401_EXPECTED_BLOBS.items():
            actual = git("rev-parse", f"HEAD:{path}")
            if actual != expected:
                return fail(f"B401 dependency blob drift: {path} expected {expected} got {actual}")
    except subprocess.CalledProcessError as exc:
        return fail(f"cannot resolve B401 dependency blob from HEAD: {exc}")

    print("DIFFCIPLINE B401 DEPENDENCY EXCEPTION PASS")
    print(f"base={args.base}")
    print(f"risk={args.risk}")
    print(f"changed_files={len(actual_files)}")
    print(f"added_lines={actual_added}")
    print(f"deleted_lines={actual_deleted}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--proof", type=Path, required=True)
    parser.add_argument("--exit-code", type=int, required=True)
    parser.add_argument("--base", required=True)
    parser.add_argument("--risk", choices=("R2", "R3"), required=True)
    args = parser.parse_args()

    try:
        proof = load_proof(args.proof)
    except (OSError, json.JSONDecodeError, ValueError) as exc:
        return fail(f"cannot read Diffcipline proof: {exc}")

    if proof.get("schema") != "diffcipline.proof/v1" or proof.get("schema_version") != "1.0":
        return fail("unexpected Diffcipline proof schema")
    if proof.get("base") != args.base:
        return fail("proof base does not match requested base")
    if proof.get("risk") != args.risk:
        return fail("proof risk does not match requested risk")
    if git("rev-parse", "HEAD") == args.base:
        return fail("HEAD unexpectedly equals comparison base")

    verdict = proof.get("verdict")
    if args.exit_code == 0 and verdict == "PASS":
        print("DIFFCIPLINE PASS")
        return 0

    # B401 has one later exact dependency-adoption exception. It cannot close
    # B401 because its exact path set excludes task/current-state mutations.
    if args.base == B401_BASE:
        return check_b401_exception(args, proof)

    # The remaining non-PASS result that can be accepted is the historical
    # one-time 004P provider adoption diff from its exact canonical base.
    if args.base != ADOPTION_BASE:
        return fail("non-PASS proof is not on an authorized exact adoption base")
    if args.exit_code != 2 or verdict != "FAIL":
        return fail(f"unexpected non-PASS result: exit={args.exit_code} verdict={verdict}")

    try:
        actual_files, actual_added, actual_deleted = actual_git_diff(args.base)
    except (subprocess.CalledProcessError, ValueError) as exc:
        return fail(f"cannot reconcile actual Git diff: {exc}")

    if (
        set(actual_files) != EXPECTED_ADOPTION_FILES
        or len(actual_files) != len(EXPECTED_ADOPTION_FILES)
    ):
        return fail("actual 004P Git changed-path set is not exact")

    files = proof.get("files")
    if not isinstance(files, list) or files != actual_files:
        return fail("Diffcipline proof path list does not equal the actual Git diff")
    if set(files) != EXPECTED_ADOPTION_FILES or len(files) != len(EXPECTED_ADOPTION_FILES):
        return fail("004P exception changed-path set is not exact")

    changed_files = proof.get("changed_files")
    if changed_files != len(actual_files):
        return fail("Diffcipline proof changed-file count does not equal the actual Git diff")
    if changed_files != len(EXPECTED_ADOPTION_FILES):
        return fail("004P exception changed-file count is not exact")

    added = proof.get("added_lines")
    if added != actual_added:
        return fail("Diffcipline proof added-line count does not equal the actual Git diff")
    if not isinstance(added, int) or not (POLICY_MAX_ADDED_LINES < added <= MAX_ADOPTION_ADDED_LINES):
        return fail("004P exception added-line count is outside the bounded adoption window")

    deleted = proof.get("deleted_lines")
    if deleted != actual_deleted:
        return fail("Diffcipline proof deleted-line count does not equal the actual Git diff")

    expected_reasons = {
        f"added lines {added} exceed maximum {POLICY_MAX_ADDED_LINES}",
        "dependency manifest changed: crates/himsat-core/Cargo.toml",
        "lockfile changed: Cargo.lock",
    }
    reasons = proof.get("reasons")
    if not isinstance(reasons, list) or set(reasons) != expected_reasons or len(reasons) != len(expected_reasons):
        return fail(f"unexpected Diffcipline reason set: {reasons!r}")

    if proof.get("scope_violations") != []:
        return fail("scope violations cannot be excepted")

    verification = proof.get("verification")
    if not isinstance(verification, list) or not verification:
        return fail("verification evidence is missing")
    for result in verification:
        if not isinstance(result, dict) or result.get("state") != "PASS":
            return fail(f"verification is not PASS: {result!r}")

    try:
        for path, expected in EXPECTED_BLOBS.items():
            actual = git("rev-parse", f"HEAD:{path}")
            if actual != expected:
                return fail(
                    f"dependency adoption blob drift: {path} expected {expected} got {actual}"
                )
    except subprocess.CalledProcessError as exc:
        return fail(f"cannot resolve dependency adoption blob from HEAD: {exc}")

    print("DIFFCIPLINE 004P ADOPTION EXCEPTION PASS")
    print(f"base={args.base}")
    print(f"risk={args.risk}")
    print(f"changed_files={len(actual_files)}")
    print(f"added_lines={actual_added}")
    print(f"deleted_lines={actual_deleted}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
