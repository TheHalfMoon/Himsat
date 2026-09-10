# B403C Windows Token Identity Dependency Adoption Evidence

## Status

```text
BASE_CANONICAL = c642029ff8c338b46bb1585ebff58e168e5a8653
SUBGRAIN = B403C_WINDOWS_TOKEN_IDENTITY_DEPENDENCY_ADOPTION_ONLY
IMPLEMENTATION_STATUS = CANDIDATE_NOT_CANONICAL
PR87_DISPOSITION = CHANGES_REQUIRED_PRESERVED
B403_TASK_DISPOSITION = UNCHECKED_NOT_PASS
B404_AUTHORITY = BLOCKED
Q009 = UNSATISFIED
```

## Purpose

The first B403B implementation candidate was not accepted after exact-head review identified that filesystem ACL ownership was derived from an account name rather than the current process token `TOKEN_USER`. Himsat core forbids unsafe code, and the already-adopted `windows-acl 0.3.0` public API does not expose current-token SID retrieval.

This bounded dependency-only candidate adopts `winsafe 0.0.29` with only the `advapi` feature so a later B403B implementation revision can use safe `OpenProcessToken` and `GetTokenInformation(TokenUser)` wrappers without weakening `#![forbid(unsafe_code)]`. It contains no B403B runtime implementation change.

## Exact package identity

```text
PACKAGE = winsafe 0.0.29
CRATES_IO_SHA256 = 9ef0ffc427f045c0cc9ebffd6f4f91153dcd2a1547b5c3c29ee3f541e16e95c6
SOURCE_REPOSITORY = https://github.com/rodrigocfd/winsafe
SOURCE_REVISION = 71ed88c2a0d18b03ee452f6d4261f4c22f443483
LICENSE = MIT
RUST_VERSION = 1.87
FEATURES = advapi -> kernel
TRANSITIVE_PACKAGE_DELTA = 0
```

The published crate `.cargo_vcs_info.json` binds the package to the recorded source revision. The published `LICENSE.md` contains the MIT license text recorded in Himsat provenance.

## Native API qualification probe

A preparatory Windows-only scratch probe outside the Himsat repository compiled with `#![forbid(unsafe_code)]` and the exact dependency configuration. It called the safe WinSafe `OpenProcessToken(TOKEN::QUERY)` and `GetTokenInformation(TOKEN_INFORMATION_CLASS::User)` wrappers and returned the same current-user SID as the native `whoami /user` observation. The probe is qualification research only and is not adopted source.

## Scope and non-claims

This candidate changes dependency/provenance closure only. It does not accept PR #87, fix its implementation findings, claim B403 completion, satisfy Q009, change Windows capability claims, add user-presence/hardware claims, or authorize B404+. The rejected B403B candidate and its review remain preserved evidence.
## Required acceptance sequence

```text
1. freeze exact dependency-only candidate bytes from canonical main
2. canonicalize a separate trusted gate that pins every candidate blob
3. recreate the candidate from the canonical trusted-gate successor
4. prove provenance/generated/004P and supported-target build/test gates
5. pass exact-head pull-request CI and R3 on attempt 1
6. reconcile substantive review findings and mergeability
7. merge only with explicit expected_head_sha and merge_method=merge
8. prove exact parentage/tree and attempt-1 push-triggered CI/R3
9. rebuild B403B forward-only from the new canonical dependency base
```

The original PR #87 review findings remain controlling for the B403B successor. This dependency leaf cannot close B403 or advance B404 by itself.
