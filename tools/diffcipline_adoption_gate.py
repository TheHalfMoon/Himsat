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
MAX_ADOPTION_ADDED_LINES = 5400
POLICY_MAX_ADDED_LINES = 900
EXPECTED_BLOBS = {
    "Cargo.lock": "070dea51697a6b6159627f7ac0140b43ae9c636a",
    "crates/himsat-core/Cargo.toml": "2e5efcd43b82c5ed88b6246ae62014a40a55f145",
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
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise ValueError("proof must be a JSON object")
    return payload


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

    # The only non-PASS result that can be accepted is the one-time 004P
    # adoption diff from its exact pre-adoption canonical base.
    if args.base != ADOPTION_BASE:
        return fail("non-PASS proof is not on the exact 004P adoption base")
    if args.exit_code != 2 or verdict != "FAIL":
        return fail(f"unexpected non-PASS result: exit={args.exit_code} verdict={verdict}")

    files = proof.get("files")
    if not isinstance(files, list) or set(files) != EXPECTED_ADOPTION_FILES or len(files) != len(EXPECTED_ADOPTION_FILES):
        return fail("004P exception changed-path set is not exact")
    if proof.get("changed_files") != len(EXPECTED_ADOPTION_FILES):
        return fail("004P exception changed-file count is not exact")

    added = proof.get("added_lines")
    if not isinstance(added, int) or not (POLICY_MAX_ADDED_LINES < added <= MAX_ADOPTION_ADDED_LINES):
        return fail("004P exception added-line count is outside the bounded adoption window")

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

    for path, expected in EXPECTED_BLOBS.items():
        actual = git("hash-object", path)
        if actual != expected:
            return fail(f"dependency adoption blob drift: {path} expected {expected} got {actual}")

    print("DIFFCIPLINE 004P ADOPTION EXCEPTION PASS")
    print(f"base={args.base}")
    print(f"risk={args.risk}")
    print(f"changed_files={len(files)}")
    print(f"added_lines={added}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
