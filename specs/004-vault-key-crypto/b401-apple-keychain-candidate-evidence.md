# B401 Apple Keychain Candidate Evidence

## Status

```text
BASE_CANONICAL_MAIN = ba32dfc21d025a189cddfdd9f46c48fdcd327e1e
LEAF = B401_APPLE_KEYCHAIN_ADAPTER_ONLY
IMPLEMENTATION_STATUS = CANDIDATE_NOT_CANONICAL
B401_TASK_DISPOSITION = UNCHECKED_NOT_PASS
SIGNED_KEYCHAIN_RUNTIME_EVIDENCE = NOT_PROVEN
MACOS_PORTABLE_BUILD_TEST_EVIDENCE = PASS
IOS_TARGET_RUNTIME_EVIDENCE = NOT_PROVEN
Q009 = UNSATISFIED
```

This document records only the current B401 implementation candidate and the evidence that genuinely exists. It does not mark B401 complete, does not open B402, and does not weaken the per-target Apple evidence requirements in the reviewed Specification 004 contract.

## Candidate boundary

The candidate adds a target-specific Apple dependency and a bounded `SecretProtector` implementation that:

- uses a fixed Himsat service identifier supplied by the caller, a signed-target access group supplied by the caller, and one opaque random protector identifier as provider-visible lookup metadata;
- sets `kSecAttrSynchronizable = false` and selects the data-protection Keychain path;
- creates `SecAccessControl` with `AccessibleWhenPasscodeSetThisDeviceOnly` and adds `USER_PRESENCE` when `REQUIRED_EACH_HIMSAT_UNLOCK` is requested;
- keeps `VaultId`, key generation, requested policy, and VRK bytes inside the protected Keychain value;
- probes the configured access-group entitlement before reporting `APP_EXCLUSIVE`;
- reports hardware backing as `Unknown`; no Secure Enclave claim is made;
- returns `UnsupportedPolicy` for freshness-anchor and full protector-replacement behavior owned by B501/B503;
- has no plaintext fallback.

B402-B406, B501-B506, Specification 005, release/FIPS/compliance, and independent Q009 review remain outside this candidate.

## Dependency and provenance candidate

The target-specific dependency is exact-pinned:

```text
security-framework = 3.7.0
features = OSX_10_15
default_features = false
```

The canonical lockfile delta is bounded to four new registry packages and no removals or unrelated version drift:

```text
core-foundation 0.10.1
core-foundation-sys 0.8.7
security-framework 3.7.0
security-framework-sys 2.17.0
```

Exact crates.io checksums, immutable VCS revisions/source paths, MIT notices, and Cargo package identities are represented in `governance/provenance/registry.json`. `THIRD_PARTY_NOTICES.md` and `governance/generated/sbom.json` were regenerated through the canonical provenance gate.

Local provenance evidence:

```text
python3 tools/provenance_gate.py validate = PASS
python3 tools/provenance_gate.py check-generated = PASS
python3 tools/provenance_gate.py self-test = PASS
LOCKFILE_UNRELATED_DRIFT = NONE
```

## Local implementation evidence

On the connected macOS development host, exact working-tree candidate bytes passed:

```text
cargo +1.98.1 fmt --all -- --check = PASS
cargo +1.98.1 clippy --workspace --locked --all-targets -- -D warnings = PASS
cargo +1.98.1 test --workspace --locked --all-targets = PASS
HIMSAT_CORE_UNIT_TESTS = 88 passed / 0 failed
B401_APPLE_UNIT_TESTS = 5 passed / 0 failed
```

The B401 unit tests prove opaque identifier formatting/redaction, empty application metadata rejection, protected record vault/generation/policy binding, corruption rejection, and typed fail-closed native error mapping. These are Himsat-owned tests, but they do not substitute for signed native Keychain runtime qualification.

## Signed native evidence gap

The reviewed Specification 004 contract requires actual Apple implementation evidence for the stored Keychain attributes and relevant target behavior. That evidence is not yet proven for this candidate.

Observed local facts:

- a usable Apple Development signing identity is present on the connected development host;
- non-interactive signing access to its private key requires SecurityAgent interaction and was not bypassed;
- an ad-hoc signature carrying sandbox/application/keychain-access-group entitlements was rejected by macOS and is NOT PASS;
- a separate ephemeral signing route was not executed because the available automation safety boundary rejected that credential/key-generation operation;
- therefore no signed B401 Keychain store/read result is promoted to PASS.

The exact native acceptance evidence still required before B401 may close includes, at minimum, a signed Himsat target proving the configured access-group boundary, non-synchronizable data-protection Keychain lookup, `WhenPasscodeSetThisDeviceOnly`, and the requested user-presence policy without secret fallback.

## iOS evidence gap

A cross-target check was attempted for `aarch64-apple-ios`. The Rust target could be selected, but the host does not provide the iPhoneOS SDK. The existing vendored OpenSSL/SQLCipher graph failed in `openssl-sys` while resolving the missing `iphoneos` SDK before a complete Himsat iOS target build could finish.

This is recorded as `NOT_PROVEN`, not as a B401 PASS or a B401 code defect. No iOS runtime qualification claim is made.

## Required next gate

This candidate may be pushed for exact-head CI/R3 and review, but it MUST NOT be merged or marked complete until the missing signed native Apple evidence is genuinely supplied and reconciled against the exact final candidate head. Failed, unavailable, interactive, or tool-blocked evidence must remain NOT PASS.
