# B403A Windows Dependency Adoption Evidence

## Status

```text
BASE_CANONICAL = b2d62613b192c54cc501513ee6bb40059c90817c
SUBGRAIN = B403A_WINDOWS_DEPENDENCY_ADOPTION_ONLY
IMPLEMENTATION_STATUS = CANDIDATE_NOT_CANONICAL
B403_TASK_DISPOSITION = UNCHECKED_NOT_PASS
B403_RUNTIME_ADAPTER = NOT_INCLUDED_IN_B403A
NATIVE_WINDOWS_QUALIFICATION = NOT_RUN
QUALIFIED_SCOPE = NOT_YET_PROVEN
Q009 = UNSATISFIED
```

B403A adopts only the exact Windows dependency closure needed by the later B403B implementation. It does not implement DPAPI storage, mark B403 complete, authorize B404, or convert dependency inspection into native runtime evidence.

## Selected boundary

The selected direct Windows dependencies are exact-pinned only under `cfg(target_os = "windows")`:

```text
windows-dpapi = 0.2.0
windows-acl = 0.3.0
```
`windows-dpapi 0.2.0` exposes a safe Rust API over `CryptProtectData` and `CryptUnprotectData`. Its `Scope::User` path passes flags `0`, so it does not request `CRYPTPROTECT_LOCAL_MACHINE`; the prompt pointer is null. Current Microsoft DPAPI documentation states that null prompt input uses the non-interactive path for new operations and that `CRYPTPROTECT_LOCAL_MACHINE` changes protection from the individual user to the current computer.

`windows-acl 0.3.0` exposes safe ACL operations while containing its Win32 unsafe code inside the dependency. Its security-descriptor apply path combines `DACL_SECURITY_INFORMATION` with `PROTECTED_DACL_SECURITY_INFORMATION` whenever it writes a DACL. B403B will still need native Windows evidence that the Himsat ciphertext file has the intended protected DACL and no unauthorized effective access.

The workspace-level `unsafe_code = "forbid"` policy remains unchanged. B403A does not create a Himsat-owned unsafe boundary crate and does not weaken the lint policy.

## Exact newly locked closure

The canonical pre-B403 lock contained 45 external Cargo packages. B403A adds exactly 13 and removes none:

| Package | Version | crates.io checksum |
| --- | --- | --- |
| `anyhow` | `1.0.104` | `330a5ed07fa54e4702c9d6c4174f74427fc0ef6e214bbd677ae50a5099946470` |
| `autocfg` | `1.5.1` | `f2032f911046de80f0a198e0901378627c33f59ea0ac00e363d481118bd70a53` |
| `field-offset` | `0.3.6` | `38e2275cc4e4fc009b0669731a1e5ab7ebf11f469eaede2bab9309a5b4d6057f` |
| `log` | `0.4.34` | `f9f8bd3e56ce4dfc153cf470fffbfa98c7620958b312ca5c3a4b8d5181fd13c6` |
| `memoffset` | `0.9.1` | `488016bfae457b036d996092f6cb448677611ce4449e970ceaf42695203f218a` |
| `rustc_version` | `0.4.1` | `cfcb3a22ef46e85b45de6ee7e79d063319ebb6594faafcf1c225ea92ab6e9b92` |
| `semver` | `1.0.28` | `8a7852d02fc848982e0c167ef163aaff9cd91dc640ba85e263cb1ce46fae51cd` |
| `widestring` | `0.4.3` | `c168940144dd21fd8046987c16a46a33d5fc84eec29ef9dcddc2ac9e31526b7c` |
| `winapi` | `0.3.9` | `5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419` |
| `winapi-i686-pc-windows-gnu` | `0.4.0` | `ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6` |
| `winapi-x86_64-pc-windows-gnu` | `0.4.0` | `712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f` |
| `windows-acl` | `0.3.0` | `177b1723986bcb4c606058e77f6e8614b51c7f9ad2face6f6fd63dd5c8b3cec3` |
| `windows-dpapi` | `0.2.0` | `2981752d6f11bdcab4db52be8ad5c0e6a6d4d6d566764b3058cc1ee473e6479e` |

The exact dependency graph contains the two direct Windows packages above plus their transitive build/runtime dependencies. `libc 0.2.189` is reused from the already canonical closure and therefore is not counted as newly adopted.

## Immutable source mapping

The registry records each new package with its exact crates.io checksum, immutable repository revision, source path, selected MIT license, and notice text. Eleven packages carry publisher-supplied `.cargo_vcs_info.json` revisions in the downloaded crates.io source.

The two architecture import-library packages do not contain their own `.cargo_vcs_info.json`. Their provenance was therefore reconstructed from the upstream `retep998/winapi-rs` history rather than guessed:

```text
UPSTREAM_REVISION = 9497609ef44cc9bcd16cd2411c0ee6ccaf5483aa
UPSTREAM_COMMIT = Publish 0.3.4
UPSTREAM_CHANGE = i686 and x86_64 package versions 0.3.2 -> 0.4.0
```
The crates.io package bytes were compared against that revision while excluding only Cargo's generated/normalized metadata files:

```text
winapi-i686-pc-windows-gnu 0.4.0 = 1389 compared files / 0 missing / 0 mismatches
winapi-x86_64-pc-windows-gnu 0.4.0 = 1418 compared files / 0 missing / 0 mismatches
```

Both are therefore bound to the exact historical `i686/` and `x86_64/` source trees at revision `9497609e...`, with the repository MIT license selected. No relationship to the later `winapi 0.3.9` revision is fabricated.

## Generated provenance closure

`governance/provenance/registry.json` is the authored machine record. `THIRD_PARTY_NOTICES.md` and `governance/generated/sbom.json` are generated only through `python3 tools/provenance_gate.py generate`.

The updated `tools/004p_dependency_closure.py` remains fail-closed. It requires exactly:

- 58 external locked packages total;
- the unchanged eight provider dependencies;
- the unchanged macOS `security-framework = 3.7.0` target dependency;
- exactly `windows-acl = 0.3.0` and `windows-dpapi = 0.2.0` under the Windows target;
- exact checksums, repositories, source revisions, and selected MIT license for every platform package; and
- the pre-existing SQLCipher, SQLite, and OpenSSL native-component identities without change.

## Explicit non-claims

B403A does not prove DPAPI round-trip behavior on Windows, protected-DACL effectiveness, cross-user denial, restart recovery, record tamper rejection, revocation, ciphertext-file deletion behavior, or any hardware-backed CNG/TPM property. It does not claim `APP_EXCLUSIVE` or `REQUIRED_EACH_HIMSAT_UNLOCK`.

B403B must remain conservative: current-user DPAPI can support only `SAME_USER_ACCOUNT` absent stronger evidence. Any stronger scope/presence request must fail closed with `UnsupportedPolicy`.
## Required acceptance sequence

B403A may become canonical only through a trusted-base dependency-adoption exception that pins every candidate artifact by exact Git blob. The candidate itself must not be able to modify the gate that authorizes its dependency/lockfile diff.

The required order is:

```text
1. build exact B403A candidate bytes locally from canonical main
2. canonicalize a separate trusted gate that pins those bytes
3. recreate the candidate from the canonical trusted-gate successor
4. prove local provenance/generated/self-test/004P/fmt/clippy/tests
5. pass exact-head CI and R3 on original attempt
6. reconcile reviews/threads/mergeability and bind expected head
7. expected-head guarded merge
8. pass push-triggered post-merge CI and R3 on original attempt
```

Only after B403A is canonical-qualified may B403B implement the Windows protector and execute genuine Windows-native qualification. B404-B406, B501-B506, Specification 005, release authority, P011, B305R002, and Q009 remain unchanged and unauthorized by this subgrain.
