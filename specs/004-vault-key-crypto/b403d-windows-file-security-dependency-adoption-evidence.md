# B403D Windows File-Security Dependency Adoption Evidence

## Status

```text
BASE_CANONICAL = 27459079f8925cbcf513aaab0e456ed091573bb5
SUBGRAIN = B403D_WINDOWS_FILE_SECURITY_DEPENDENCY_ADOPTION_ONLY
PR90_DISPOSITION = CLOSED_UNMERGED_CHANGES_REQUIRED_PRESERVED
B403_TASK_DISPOSITION = UNCHECKED_NOT_PASS
B404_AUTHORITY = BLOCKED
Q009 = UNSATISFIED
```

## Purpose

Exact-head review of B403B successor PR #90 identified two Windows filesystem guarantees that the currently adopted safe APIs do not expose: security-descriptor owner-SID verification and deletion through the already validated file handle. Himsat core continues to forbid unsafe code.

This dependency-only leaf adopts `windows-permissions 0.2.4` for safe security-descriptor owner inspection and `fs_at 0.2.1` for its safe Windows handle-relative/open-handle filesystem API, including `FileExt::delete_by_handle`. It contains no B403 runtime implementation byte.

## Exact newly locked closure

The canonical pre-B403D lock contains 59 external Cargo packages. This leaf adds exactly 19 and removes none, for 78 total external packages.

| Package | Version | crates.io SHA-256 | Source revision | Selected license |
| --- | --- | --- | --- | --- |
| `aligned` | `0.4.3` | `ee4508988c62edf04abd8d92897fca0c2995d907ce1dfeaf369dac3716a40685` | `f13f8e56db5ab99257209758d14d3096a851ff04` | `MIT` |
| `as-slice` | `0.2.1` | `516b6b4f0e40d50dcda9365d53964ec74560ad4284da2e7fc97122cd83174516` | `c3687342bc9f3c9c202676a5d2e17288ff979c90` | `MIT` |
| `bitflags` | `1.3.2` | `bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a` | `ed185cfb1c447c1b4bd6ac021c9ec3bb02c9e2f2` | `MIT` |
| `cfg_aliases` | `0.2.2` | `f079e83a288787bcd14a6aea84cee5c87a67c5a3e660c30f557a3d24761b3527` | `e069ce61e00fee423ac1dbe6a897f228f1774716` | `MIT` |
| `cvt` | `0.1.2` | `d2ae9bf77fbf2d39ef573205d554d87e86c12f1994e9ea335b0651b9b278bcf1` | `ae6de53753f9e83aac06aa6638e2eac3786975bc` | `Apache-2.0` |
| `fs_at` | `0.2.1` | `14af6c9694ea25db25baa2a1788703b9e7c6648dcaeeebeb98f7561b5384c036` | `e8b58a0682496a0c6ddc9eae80942a2f29a5a7e4` | `Apache-2.0` |
| `nix` | `0.29.0` | `71e2746dc3a24dd78b3cfcb7be93368c6de9963d30f43a6a73998a9cf4b17b46` | `1dad4d8d04a2cd187fae87cb91c4f4e95ff0decd` | `MIT` |
| `stable_deref_trait` | `1.2.1` | `6ce2be8dc25455e1f91df71bfa12ad37d7af1092ae736f3a6cd0e37bc7810596` | `30002b4228f7cdee309217b6bf6eee099dda0b00` | `MIT` |
| `windows-permissions` | `0.2.4` | `9e2ccdc3c6bf4d4a094e031b63fadd08d8e42abd259940eb8aa5fdc09d4bf9be` | `8740e4efbd88dd01046ad9c169894f3a52eb6e2c` | `MIT` |
| `windows-sys` | `0.52.0` | `282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d` | `3a605cba064b26f2a198ac58085f8c8836f47c38` | `MIT` |
| `windows-targets` | `0.52.6` | `9b724f72796e036ab90c1021d4780d4d3d648aca59e491e6b98e725b84e99973` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_aarch64_gnullvm` | `0.52.6` | `32a4622180e7a0ec044bb555404c800bc9fd9ec262ec147edd5989ccd0c02cd3` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_aarch64_msvc` | `0.52.6` | `09ec2a7bb152e2252b53fa7803150007879548bc709c039df7627cabbd05d469` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_i686_gnu` | `0.52.6` | `8e9b5ad5ab802e97eb8e295ac6720e509ee4c243f69d781394014ebfe8bbfa0b` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_i686_gnullvm` | `0.52.6` | `0eee52d38c090b3caa76c563b86c3a4bd71ef1a819287c19d586d7334ae8ed66` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_i686_msvc` | `0.52.6` | `240948bc05c5e7c6dabba28bf89d89ffce3e303022809e73deaefe4f6ec56c66` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_x86_64_gnu` | `0.52.6` | `147a5c80aabfbf0c7d901cb5895d1de30ef2907eb21fbbab29ca94c5b08b1a78` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_x86_64_gnullvm` | `0.52.6` | `24d5b23dc417412679681396f2b49f3de8c1473deb516bd34410872eff51ed0d` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |
| `windows_x86_64_msvc` | `0.52.6` | `589f6da84c646204747d1270a2a5661ea66ed1cced2631d546fdfb155959f9ec` | `db06b51c2ebb743efb544d40e3064efa49f28d38` | `MIT` |

All selected licenses are in `governance/provenance/policy.json` allowlist. `fs_at` and `cvt` are selected under Apache-2.0 because their published manifests do not offer MIT; the remaining new packages are selected under MIT. No license is reclassified to obtain adoption.

## Boundary and non-claims

`fs_at` documents a safe Windows `*_at` interface intended to avoid path-substitution races and exposes safe `delete_by_handle`; `windows-permissions` documents safe wrappers for Windows SID/ACL/security-descriptor APIs. Dependency inspection is not runtime proof: the later B403 implementation must still bind owner SID to the current process token, validate the opened object, delete the validated object through its handle, and pass native adversarial tests.

This leaf does not accept PR #90, close B403, authorize B404+, satisfy Q009, claim cross-user denial, claim app-exclusive/user-presence/hardware-backed protection, or alter Specification 005/release authority.

## Required acceptance sequence

```text
1. freeze these exact dependency/provenance/generated candidate bytes
2. canonicalize a separate trusted gate that pins every candidate blob
3. recreate the dependency candidate from that canonical trusted-gate successor
4. pass exact provenance/generated/004P/fmt/clippy/test qualification
5. pass exact-head pull-request CI and R3 on attempt 1
6. reconcile reviews/threads/mergeability and bind expected_head_sha
7. merge with merge_method=merge and exact expected-head protection
8. prove exact parentage/tree and attempt-1 post-merge CI/R3
9. rebuild B403B forward-only from the qualified dependency base
```

PR #90 and its review findings remain preserved evidence and are not retroactively upgraded.
