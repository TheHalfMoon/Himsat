#!/usr/bin/env python3
"""Fail-closed bridge for the one-time Specification 004P adoption diff.

Repository Diffcipline policy remains conservative. The historical 004P
P005/P006 adoption diff is exceptional because it intentionally introduces the
reviewed dependency graph and generated provenance closure in one bounded leaf.
This bridge accepts Diffcipline's otherwise-blocking REVIEW/size findings only
for explicitly bounded historical/provider, B401, B403A, B403C, or B403D candidate transitions. The B401, B403A, B403C, and B403D exceptions are available only when this gate is executed
from the corresponding trusted immutable PR base and every permitted candidate
artifact matches its pinned Git blob. Every configured verification still has to pass.

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
B401_PRECONDITION_BASE = "ba32dfc21d025a189cddfdd9f46c48fdcd327e1e"
B401_MAX_ADDED_LINES = 1600
B401_TRUSTED_BASE_DELTA = {
    ".github/workflows/ci.yml",
    ".github/workflows/r3-security.yml",
    "tools/diffcipline_adoption_gate.py",
}
B401_EXPECTED_BLOBS = {
    "Cargo.lock": "d9c5aac949ec0b0b7ec12d7baa4311a46ee79f41",
    "THIRD_PARTY_NOTICES.md": "721a7022939eafdf6e5203c4ec1c57676bacc409",
    "crates/himsat-core/Cargo.toml": "4aefdc25644c7b54b55ac9536cbf8823a6b683d7",
    "crates/himsat-core/examples/b401_apple_keychain_probe.rs": "164c97e1e2af5d788de53ce147b580b5b17c409f",
    "crates/himsat-core/src/lib.rs": "c506c30aeef391363bcdc076a694660b2deb5332",
    "crates/himsat-core/src/vault_apple_keychain.rs": "b9f2e1429a15284abf8ae06e0a633facaba72eca",
    "governance/generated/sbom.json": "dfb107acb6d647979903098518db357a1ffd1741",
    "governance/provenance/registry.json": "ebe0264d0b318e757d89d214138cb1c6ae4c2c66",
    "specs/004-vault-key-crypto/b401-apple-keychain-candidate-evidence.md": "b4c4cac48962fb4685b473bbd75751bf03a52139",
    "tools/004p_dependency_closure.py": "203f5c351154aa1de9734f47a997059a997eebe3",
}
B401_EXPECTED_FILES = set(B401_EXPECTED_BLOBS)

B403_PRECONDITION_BASE = "b2d62613b192c54cc501513ee6bb40059c90817c"
B403_MAX_ADDED_LINES = 1400
B403_TRUSTED_BASE_DELTA = {"tools/diffcipline_adoption_gate.py"}
B403_EXPECTED_BLOBS = {
    "Cargo.lock": "8b4b0e96828bc781395fc6a880f62480839b228e",
    "THIRD_PARTY_NOTICES.md": "38cebb18c59b8a334d0011ff7c5121f4e16d7239",
    "crates/himsat-core/Cargo.toml": "77872a29fa32c32d893e438c757b6e7161fe0583",
    "governance/generated/sbom.json": "64542dda6d2a7cb33a907ee36ac5478f731f62dc",
    "governance/provenance/registry.json": "32c3853af124f670d2207f81af312df47eb3f23a",
    "specs/004-vault-key-crypto/b403a-windows-dependency-adoption-evidence.md": "95104aa22baa1c71b7588fd072af63c25378cb88",
    "tools/004p_dependency_closure.py": "1b3b3ed59972a00cd0ced0c52fa0d05540f9ee3c",
}
B403_EXPECTED_FILES = set(B403_EXPECTED_BLOBS)

B403C_PRECONDITION_BASE = "c642029ff8c338b46bb1585ebff58e168e5a8653"
B403C_TRUSTED_BASE_DELTA = {"tools/diffcipline_adoption_gate.py"}
B403C_EXPECTED_ADDED_LINES = 150
B403C_EXPECTED_DELETED_LINES = 1
B403C_EXPECTED_BLOBS = {
    "Cargo.lock": "54e44197e15e47178be838a90c3f5136e68a88d9",
    "THIRD_PARTY_NOTICES.md": "2a161e6c7d85435a4ff1b0436e44973912482fff",
    "crates/himsat-core/Cargo.toml": "022639774fe4c667ee743f29e91758171edf6edd",
    "governance/generated/sbom.json": "9694b55de442b8512788eb08f5b7e2c30e084975",
    "governance/provenance/registry.json": "1a5a4482159b670209302d39a8bed53a1f388138",
    "specs/004-vault-key-crypto/b403c-windows-token-identity-dependency-adoption-evidence.md": "016f684df960a3491193568413ae57cde1f9d57a",
    "tools/004p_dependency_closure.py": "5ef3432b2e97d5e01bcaf15936a32d4f183d4592",
}
B403C_EXPECTED_FILES = set(B403C_EXPECTED_BLOBS)

B403D_PRECONDITION_BASE = "27459079f8925cbcf513aaab0e456ed091573bb5"
B403D_TRUSTED_BASE_DELTA = {"tools/diffcipline_adoption_gate.py"}
B403D_MAX_ADDED_LINES = 2100
B403D_EXPECTED_ADDED_LINES = 2028
B403D_EXPECTED_DELETED_LINES = 34
B403D_EXPECTED_BLOBS = {
    "Cargo.lock": "8774e77c1a3ba36c23e739f365fc3a01cf337e88",
    "THIRD_PARTY_NOTICES.md": "91ff3141802ccccd01b17e65d0e6b0a7e9c827fa",
    "crates/himsat-core/Cargo.toml": "4ac5e91e76c0d323a2ef8ef19faf1097b4ac0059",
    "governance/generated/sbom.json": "0c848d903272dc244fd125fccc3299a854773e92",
    "governance/provenance/registry.json": "034e5d3ee20396c6d74c30382ef39b98e45de195",
    "specs/004-vault-key-crypto/b403d-windows-file-security-dependency-adoption-evidence.md": "e9551d89f479c274289411f979c683b19529e343",
    "tools/004p_dependency_closure.py": "cc624a7913a73adee42b4af00852b478a7f0f544",
}
B403D_EXPECTED_FILES = set(B403D_EXPECTED_BLOBS)

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


def b401_trusted_base(base: str) -> bool:
    """Return whether base is the exact canonical B401 gate-hardening successor.

    The secure B401 exception is available only after the precondition base has
    changed by exactly the trusted workflow and gate-hardening paths. The workflow
    then executes this gate from that immutable base rather than from PR HEAD.
    Any unrelated base drift disables the exception until separately reconciled.
    """

    try:
        raw = git("diff", "--no-renames", "--name-only", B401_PRECONDITION_BASE, base)
    except subprocess.CalledProcessError:
        return False
    paths = [line for line in raw.splitlines() if line]
    return set(paths) == B401_TRUSTED_BASE_DELTA and len(paths) == len(B401_TRUSTED_BASE_DELTA)


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
                return fail(f"B401 candidate artifact blob drift: {path} expected {expected} got {actual}")
    except subprocess.CalledProcessError as exc:
        return fail(f"cannot resolve B401 candidate artifact blob from HEAD: {exc}")

    print("DIFFCIPLINE B401 DEPENDENCY EXCEPTION PASS")
    print(f"base={args.base}")
    print(f"risk={args.risk}")
    print(f"changed_files={len(actual_files)}")
    print(f"added_lines={actual_added}")
    print(f"deleted_lines={actual_deleted}")
    return 0


def b403_trusted_base(base: str) -> bool:
    """Return whether base is the exact canonical B403 gate-hardening successor.

    The secure B403 exception is available only when the comparison base is the
    exact fetched canonical main revision, has the precondition base as first
    parent, and changes exactly the trusted gate-hardening path. The workflow
    then executes this gate from that immutable base rather than from PR HEAD.
    Any unrelated base drift disables the exception until separately reconciled.
    """

    try:
        canonical_main = git("rev-parse", "refs/remotes/origin/main^{commit}")
        if base != canonical_main:
            return False
        parent_row = git("rev-list", "--parents", "-n", "1", base).split()
        if len(parent_row) != 3 or parent_row[1] != B403_PRECONDITION_BASE:
            return False
        raw = git("diff", "--no-renames", "--name-only", B403_PRECONDITION_BASE, base)
    except subprocess.CalledProcessError:
        return False
    paths = [line for line in raw.splitlines() if line]
    return set(paths) == B403_TRUSTED_BASE_DELTA and len(paths) == len(B403_TRUSTED_BASE_DELTA)


def check_b403_exception(args: argparse.Namespace, proof: dict[str, Any]) -> int:
    """Accept only the exact B403 candidate dependency-adoption diff.

    This bridge qualifies repository review mechanics only. The exact path set
    deliberately excludes `tasks.md` and `specs/CURRENT.md`, so this exception
    cannot mark B403 complete or advance B404 authority. Native Windows
    runtime qualification remains a separate controlling requirement.
    """

    if args.exit_code != 2 or proof.get("verdict") != "FAIL":
        return fail(
            f"unexpected B403 non-PASS result: exit={args.exit_code} verdict={proof.get('verdict')}"
        )

    try:
        actual_files, actual_added, actual_deleted = actual_git_diff(args.base)
    except (subprocess.CalledProcessError, ValueError) as exc:
        return fail(f"cannot reconcile actual B403 Git diff: {exc}")

    if set(actual_files) != B403_EXPECTED_FILES or len(actual_files) != len(B403_EXPECTED_FILES):
        return fail("actual B403 Git changed-path set is not exact")

    files = proof.get("files")
    if not isinstance(files, list) or files != actual_files:
        return fail("B403 Diffcipline proof path list does not equal the actual Git diff")
    if set(files) != B403_EXPECTED_FILES or len(files) != len(B403_EXPECTED_FILES):
        return fail("B403 exception changed-path set is not exact")

    changed_files = proof.get("changed_files")
    if changed_files != len(actual_files) or changed_files != len(B403_EXPECTED_FILES):
        return fail("B403 Diffcipline changed-file count is not exact")

    added = proof.get("added_lines")
    if added != actual_added:
        return fail("B403 Diffcipline added-line count does not equal the actual Git diff")
    if not isinstance(added, int) or not (POLICY_MAX_ADDED_LINES < added <= B403_MAX_ADDED_LINES):
        return fail("B403 exception added-line count is outside the bounded window")

    deleted = proof.get("deleted_lines")
    if deleted != actual_deleted:
        return fail("B403 Diffcipline deleted-line count does not equal the actual Git diff")

    expected_reasons = {
        f"added lines {added} exceed maximum {POLICY_MAX_ADDED_LINES}",
        "dependency manifest changed: crates/himsat-core/Cargo.toml",
        "lockfile changed: Cargo.lock",
    }
    reasons = proof.get("reasons")
    if not isinstance(reasons, list) or set(reasons) != expected_reasons or len(reasons) != len(expected_reasons):
        return fail(f"unexpected B403 Diffcipline reason set: {reasons!r}")

    if proof.get("scope_violations") != []:
        return fail("B403 scope violations cannot be excepted")

    verification = proof.get("verification")
    if not isinstance(verification, list) or not verification:
        return fail("B403 verification evidence is missing")
    for result in verification:
        if not isinstance(result, dict) or result.get("state") != "PASS":
            return fail(f"B403 verification is not PASS: {result!r}")

    try:
        for path, expected in B403_EXPECTED_BLOBS.items():
            actual = git("rev-parse", f"HEAD:{path}")
            if actual != expected:
                return fail(f"B403 candidate artifact blob drift: {path} expected {expected} got {actual}")
    except subprocess.CalledProcessError as exc:
        return fail(f"cannot resolve B403 candidate artifact blob from HEAD: {exc}")

    print("DIFFCIPLINE B403 DEPENDENCY EXCEPTION PASS")
    print(f"base={args.base}")
    print(f"risk={args.risk}")
    print(f"changed_files={len(actual_files)}")
    print(f"added_lines={actual_added}")
    print(f"deleted_lines={actual_deleted}")
    return 0


def b403d_trusted_base(base: str) -> bool:
    """Accept only the canonical B403D gate successor or its immediate merge parent.

    Pull-request qualification requires the comparison base to be current
    canonical main. Push-triggered qualification after the guarded adoption
    merge requires that same trusted base to be the first parent of current
    canonical main. The trusted base itself must be a two-parent merge whose
    first parent is the exact B403D precondition and whose only tree delta is
    this adoption-gate path.
    """

    try:
        canonical_main = git("rev-parse", "refs/remotes/origin/main^{commit}")
        if base != canonical_main:
            canonical_parent_row = git("rev-list", "--parents", "-n", "1", canonical_main).split()
            if len(canonical_parent_row) != 3 or canonical_parent_row[1] != base:
                return False
        base_parent_row = git("rev-list", "--parents", "-n", "1", base).split()
        if len(base_parent_row) != 3 or base_parent_row[1] != B403D_PRECONDITION_BASE:
            return False
        raw = git("diff", "--no-renames", "--name-only", B403D_PRECONDITION_BASE, base)
    except subprocess.CalledProcessError:
        return False
    paths = [line for line in raw.splitlines() if line]
    return set(paths) == B403D_TRUSTED_BASE_DELTA and len(paths) == len(B403D_TRUSTED_BASE_DELTA)


def check_b403d_exception(args: argparse.Namespace, proof: dict[str, Any]) -> int:
    """Accept only the pinned B403D Windows file-security dependency adoption."""

    if args.exit_code != 2 or proof.get("verdict") != "FAIL":
        return fail(
            f"unexpected B403D non-PASS result: exit={args.exit_code} verdict={proof.get('verdict')}"
        )

    try:
        actual_files, actual_added, actual_deleted = actual_git_diff(args.base)
    except (subprocess.CalledProcessError, ValueError) as exc:
        return fail(f"cannot reconcile actual B403D Git diff: {exc}")

    if set(actual_files) != B403D_EXPECTED_FILES or len(actual_files) != len(B403D_EXPECTED_FILES):
        return fail("actual B403D Git changed-path set is not exact")

    files = proof.get("files")
    if not isinstance(files, list) or files != actual_files:
        return fail("B403D Diffcipline proof path list does not equal the actual Git diff")
    if set(files) != B403D_EXPECTED_FILES or len(files) != len(B403D_EXPECTED_FILES):
        return fail("B403D exception changed-path set is not exact")

    if proof.get("changed_files") != len(B403D_EXPECTED_FILES):
        return fail("B403D Diffcipline changed-file count is not exact")
    if actual_added != B403D_EXPECTED_ADDED_LINES or proof.get("added_lines") != actual_added:
        return fail("B403D added-line count is not exact")
    if actual_deleted != B403D_EXPECTED_DELETED_LINES or proof.get("deleted_lines") != actual_deleted:
        return fail("B403D deleted-line count is not exact")
    if not (POLICY_MAX_ADDED_LINES < actual_added <= B403D_MAX_ADDED_LINES):
        return fail("B403D dependency leaf is outside the bounded adoption line window")

    expected_reasons = {
        f"added lines {actual_added} exceed maximum {POLICY_MAX_ADDED_LINES}",
        "dependency manifest changed: crates/himsat-core/Cargo.toml",
        "lockfile changed: Cargo.lock",
    }
    reasons = proof.get("reasons")
    if not isinstance(reasons, list) or set(reasons) != expected_reasons or len(reasons) != 3:
        return fail(f"unexpected B403D Diffcipline reason set: {reasons!r}")
    if proof.get("scope_violations") != []:
        return fail("B403D scope violations cannot be excepted")

    verification = proof.get("verification")
    if not isinstance(verification, list) or not verification:
        return fail("B403D verification evidence is missing")
    for result in verification:
        if not isinstance(result, dict) or result.get("state") != "PASS":
            return fail(f"B403D verification is not PASS: {result!r}")

    try:
        for path, expected in B403D_EXPECTED_BLOBS.items():
            actual = git("rev-parse", f"HEAD:{path}")
            if actual != expected:
                return fail(
                    f"B403D candidate artifact blob drift: {path} expected {expected} got {actual}"
                )
    except subprocess.CalledProcessError as exc:
        return fail(f"cannot resolve B403D candidate artifact blob from HEAD: {exc}")

    print("DIFFCIPLINE B403D DEPENDENCY EXCEPTION PASS")
    print(f"base={args.base}")
    print(f"risk={args.risk}")
    print(f"changed_files={len(actual_files)}")
    print(f"added_lines={actual_added}")
    print(f"deleted_lines={actual_deleted}")
    return 0



def b403c_trusted_base(base: str) -> bool:
    """Accept only the canonical B403C gate successor or its immediate merge parent.

    Pull-request qualification requires the comparison base to be current
    canonical main. Push-triggered qualification after the guarded adoption
    merge requires that same trusted base to be the first parent of current
    canonical main. The trusted base itself must be a two-parent merge whose
    first parent is the exact B403C precondition and whose only tree delta is
    this adoption-gate path.
    """

    try:
        canonical_main = git("rev-parse", "refs/remotes/origin/main^{commit}")
        if base != canonical_main:
            canonical_parent_row = git("rev-list", "--parents", "-n", "1", canonical_main).split()
            if len(canonical_parent_row) != 3 or canonical_parent_row[1] != base:
                return False
        base_parent_row = git("rev-list", "--parents", "-n", "1", base).split()
        if len(base_parent_row) != 3 or base_parent_row[1] != B403C_PRECONDITION_BASE:
            return False
        raw = git("diff", "--no-renames", "--name-only", B403C_PRECONDITION_BASE, base)
    except subprocess.CalledProcessError:
        return False
    paths = [line for line in raw.splitlines() if line]
    return set(paths) == B403C_TRUSTED_BASE_DELTA and len(paths) == len(B403C_TRUSTED_BASE_DELTA)


def check_b403c_exception(args: argparse.Namespace, proof: dict[str, Any]) -> int:
    """Accept only the pinned B403C safe token-identity dependency adoption."""

    if args.exit_code != 1 or proof.get("verdict") != "REVIEW":
        return fail(
            f"unexpected B403C non-PASS result: exit={args.exit_code} verdict={proof.get('verdict')}"
        )

    try:
        actual_files, actual_added, actual_deleted = actual_git_diff(args.base)
    except (subprocess.CalledProcessError, ValueError) as exc:
        return fail(f"cannot reconcile actual B403C Git diff: {exc}")

    if set(actual_files) != B403C_EXPECTED_FILES or len(actual_files) != len(B403C_EXPECTED_FILES):
        return fail("actual B403C Git changed-path set is not exact")

    files = proof.get("files")
    if not isinstance(files, list) or files != actual_files:
        return fail("B403C Diffcipline proof path list does not equal the actual Git diff")
    if set(files) != B403C_EXPECTED_FILES or len(files) != len(B403C_EXPECTED_FILES):
        return fail("B403C exception changed-path set is not exact")

    if proof.get("changed_files") != len(B403C_EXPECTED_FILES):
        return fail("B403C Diffcipline changed-file count is not exact")
    if actual_added != B403C_EXPECTED_ADDED_LINES or proof.get("added_lines") != actual_added:
        return fail("B403C added-line count is not exact")
    if actual_deleted != B403C_EXPECTED_DELETED_LINES or proof.get("deleted_lines") != actual_deleted:
        return fail("B403C deleted-line count is not exact")
    if actual_added > POLICY_MAX_ADDED_LINES:
        return fail("B403C dependency leaf unexpectedly exceeds ordinary line bounds")

    expected_reasons = {
        "dependency manifest changed: crates/himsat-core/Cargo.toml",
        "lockfile changed: Cargo.lock",
    }
    reasons = proof.get("reasons")
    if not isinstance(reasons, list) or set(reasons) != expected_reasons or len(reasons) != 2:
        return fail(f"unexpected B403C Diffcipline reason set: {reasons!r}")
    if proof.get("scope_violations") != []:
        return fail("B403C scope violations cannot be excepted")

    verification = proof.get("verification")
    if not isinstance(verification, list) or not verification:
        return fail("B403C verification evidence is missing")
    for result in verification:
        if not isinstance(result, dict) or result.get("state") != "PASS":
            return fail(f"B403C verification is not PASS: {result!r}")

    try:
        for path, expected in B403C_EXPECTED_BLOBS.items():
            actual = git("rev-parse", f"HEAD:{path}")
            if actual != expected:
                return fail(
                    f"B403C candidate artifact blob drift: {path} expected {expected} got {actual}"
                )
    except subprocess.CalledProcessError as exc:
        return fail(f"cannot resolve B403C candidate artifact blob from HEAD: {exc}")

    print("DIFFCIPLINE B403C DEPENDENCY EXCEPTION PASS")
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

    # B403D is the exact file-security dependency adoption required by the
    # substantive B403B successor review. The gate is trusted only from its
    # separate canonical hardening merge and pins every candidate artifact.
    if b403d_trusted_base(args.base):
        return check_b403d_exception(args, proof)

    # B403C is a bounded follow-up dependency adoption required by substantive
    # review of the first B403B implementation candidate. Its trusted base is
    # canonical main during PR qualification or the immediate first parent of
    # canonical main during post-merge push qualification.
    if b403c_trusted_base(args.base):
        return check_b403c_exception(args, proof)

    # B403A has one exact dependency-adoption exception, enabled only from
    # its canonical trusted-base hardening successor and pinned candidate blobs.
    if b403_trusted_base(args.base):
        return check_b403_exception(args, proof)

    # B401 has one later exact dependency-adoption exception. It is enabled
    # only from the canonical trusted-base hardening successor and pins every
    # candidate artifact. It cannot close B401 because its exact path set
    # excludes task/current-state mutations.
    if b401_trusted_base(args.base):
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
