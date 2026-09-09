# B401 Apple Keychain Candidate Evidence

## Status

```text
BASE_CANONICAL_MAIN = ba32dfc21d025a189cddfdd9f46c48fdcd327e1e
LEAF = B401_APPLE_KEYCHAIN_ADAPTER_ONLY
IMPLEMENTATION_STATUS = CANDIDATE_NOT_CANONICAL
B401_TASK_DISPOSITION = UNCHECKED_NOT_PASS
CURRENT_APPLE_IMPLEMENTATION_TARGET = MACOS_ONLY
QUALIFIED_SCOPE_CANDIDATE = SAME_USER_ACCOUNT
QUALIFIED_PRESENCE_CANDIDATE = NOT_REQUIRED
SIGNED_DATA_PROTECTION_KEYCHAIN_RUNTIME_EVIDENCE = NOT_PROVEN
IOS_IMPLEMENTATION_SURFACE = NOT_ADOPTED_IN_B401
Q009 = UNSATISFIED
```

This document records only evidence that actually exists. It does not mark B401 complete, authorize B402, or convert unavailable Apple platform evidence into PASS.

## Conservative candidate boundary

The current forward-only candidate deliberately narrows Apple behavior to the macOS policy that Himsat can describe without an unproven stronger claim:

- `security-framework = 3.7.0` is target-specific to `cfg(target_os = "macos")` only;
- Keychain lookup metadata is a fixed Himsat service plus one opaque random protector identifier;
- no explicit sharing access group is configured; the target's default application Keychain group is used;
- `kSecAttrSynchronizable = false` and the data-protection Keychain path are requested;
- VRK records use `AccessibleWhenPasscodeSetThisDeviceOnly`;
- the adapter reports `SAME_USER_ACCOUNT`, `NOT_REQUIRED`, and hardware backing `Unknown`;
- `APP_EXCLUSIVE`, `SAME_USER_SESSION`, and `REQUIRED_EACH_HIMSAT_UNLOCK` requests return `UnsupportedPolicy`;
- existing-item operations re-check service, account, `ThisDeviceOnly` accessibility, and absence of a synchronized twin before releasing or mutating VRK state;
- cross-vault/generation/policy protector reuse and wrong-vault removal fail closed;
- temporary Keychain value buffers are zeroized after reads/writes;
- freshness/full-rotation behavior remains `UnsupportedPolicy` for B501/B503;
- there is no plaintext fallback.

The candidate intentionally removes the earlier iOS compilation surface. Himsat currently lacks an iPhoneOS SDK/runtime qualification path, so B401 does not ship an unqualified iOS adapter merely because the Rust dependency can compile conditionally in theory.

## Dependency and provenance closure

The exact Apple Cargo closure remains four packages and no unrelated lockfile drift:

```text
core-foundation 0.10.1
core-foundation-sys 0.8.7
security-framework 3.7.0
security-framework-sys 2.17.0
```

The direct dependency remains exact-pinned with `default-features = false` and `features = ["OSX_10_15"]`. The target selector is now exactly `cfg(target_os = "macos")`.

Exact crates.io checksums, immutable VCS revisions/source paths, selected MIT license evidence, notices, registry entries, and generated SBOM remain represented in the repository provenance closure.

## Exact-head automation history

The first pushed candidate remains NOT PASS and was never rerun or upgraded:

```text
HEAD = 2e6ea72306e3ec76b4bee4062d3a07f92efa73d1
CI = 34390554794 / run #202 / FAILURE / attempt 1 / pull_request
R3 = 34390554771 / run #179 / FAILURE / attempt 1 / pull_request
CAUSE = pre-B401 dependency-closure guard expected 41 external packages instead of the exact 45-package provider-plus-Apple closure
DISPOSITION = NOT_PASS
```

The next head passed automation but was rejected by self-review:

```text
HEAD = 83397af010817dd4dc4f83d6ddc304f7caa53f78
CI = 34392435440 / run #203 / SUCCESS / attempt 1 / pull_request
R3 = 34392435528 / run #180 / SUCCESS / attempt 1 / pull_request
SELF_REVIEW_FINDING = remove_protector(vault_id) ignored vault_id before deleting the opaque Keychain item
DISPOSITION = GREEN_AUTOMATION_NOT_ACCEPTED
```

The next forward-only head repaired vault/policy removal binding, protector-ID reuse binding, and temporary-record zeroization. Its automation also passed, but later self-review found that membership in an arbitrary configured access group was insufficient evidence for `APP_EXCLUSIVE`, and the same head still exposed an unqualified iOS compilation surface:

```text
HEAD = 27e864970ac86710732adf372699825b529e22ba
CI = 34395001464 / run #204 / SUCCESS / attempt 1 / pull_request
R3 = 34395001338 / run #181 / SUCCESS / attempt 1 / pull_request
DISPOSITION = GREEN_AUTOMATION_NOT_ACCEPTED
```

No green predecessor is merged or promoted merely because automation succeeded.

## Local conservative-successor evidence

The current working candidate passes the complete local portable gate on the connected macOS development host:

```text
OPENSSL_RUST_USE_NASM=0 python3 tools/004p_dependency_closure.py = PASS
python3 tools/provenance_gate.py validate = PASS
python3 tools/provenance_gate.py check-generated = PASS
python3 tools/provenance_gate.py self-test = PASS
cargo +1.98.1 fmt --all -- --check = PASS
cargo +1.98.1 clippy --workspace --locked --all-targets -- -D warnings = PASS
cargo +1.98.1 test --workspace --locked --all-targets = PASS
HIMSAT_CORE_UNIT_TESTS = 90 passed / 0 failed
```

The added conservative-policy test proves that unqualified `APP_EXCLUSIVE` and per-unlock presence requests return `UnsupportedPolicy` before any Keychain mutation.

## Preserved conservative-head CI evidence

The conservative macOS-only tree was pushed through a forward-only cleanup head after an accidental transient-file tool incident. The final cleanup head remained byte-identical to the intended conservative tree, but its original GitHub qualification failed on Linux-only example imports that were not cfg-gated after the macOS-only refactor:

```text
HEAD = 7c759c75f9c1ed98551cd41a0b4ef3f147ce9466
TREE = 377508a4567cd61675221b03f0eb5d931fe34d0e
CI = 34398682744 / run #207 / FAILURE / attempt 1 / pull_request
R3 = 34398682756 / run #184 / FAILURE / attempt 1 / pull_request
CAUSE = Linux clippy detected macOS-only qualifier imports outside cfg(target_os = "macos")
DISPOSITION = FAILURE_NOT_PASS
```

The failure is not rerun or upgraded. The forward-only successor cfg-gates only those imports; no protector semantics are weakened.

## Native macOS evidence attempts

Positive data-protection Keychain runtime qualification remains NOT PROVEN. The following attempts are preserved as negative/unavailable evidence and are not PASS:

```text
AD_HOC_STANDALONE_SANDBOX = NOT_PASS / libsecinit rejected missing bundle identity
AD_HOC_SANDBOX_APP_DEFAULT_GROUP = NOT_PASS / OwnerMismatch
AD_HOC_SANDBOX_APP_APPLICATION_IDENTIFIER = NOT_PASS / OwnerMismatch
APPLE_DEVELOPMENT_SIGNING_IDENTITY = PRESENT
APPLE_DEVELOPMENT_PRIVATE_KEY_USE = USER_AUTHORIZED_THROUGH_SECURITYAGENT
APPLE_DEVELOPMENT_SIGNED_SANDBOX_APP_TEAM_IDENTIFIER = PRESENT
APPLE_DEVELOPMENT_DEFAULT_GROUP_WITHOUT_APPLICATION_IDENTIFIER = NOT_PASS / OwnerMismatch
APPLE_DEVELOPMENT_APPLICATION_IDENTIFIER_ONLY = NOT_PASS / OwnerMismatch
APPLE_DEVELOPMENT_EXPLICIT_DEFAULT_KEYCHAIN_GROUP = NOT_PASS / runtime terminated before qualifier output
INSTALLED_MACOS_PROVISIONING_PROFILE = PRESENT_BUT_EXPIRED_NOT_PASS
XCODE_APP = NOT_INSTALLED
ACTIVE_DEVELOPER_TOOLS = COMMAND_LINE_TOOLS_ONLY
SIGNED_DATA_PROTECTION_KEYCHAIN_RUNTIME_EVIDENCE = NOT_PROVEN
```

The candidate probe is ready to verify, on a genuinely authorized signed macOS target, all of the following without exposing a VRK:

```text
scope = SAME_USER_ACCOUNT
presence = NOT_REQUIRED
accessibility = WhenPasscodeSetThisDeviceOnly
synchronizable = false
stronger APP_EXCLUSIVE request = UnsupportedPolicy
stronger REQUIRED_EACH_HIMSAT_UNLOCK request = UnsupportedPolicy
wrong-vault removal = OwnerMismatch and original item remains usable
item removal = future unlock unavailable
```

A valid Apple-authorized runtime still must demonstrate that `kSecUseDataProtectionKeychain` actually selects the protected Keychain and that the stored item has the expected `ThisDeviceOnly` and non-synchronizable properties. Ad-hoc signatures, expired profiles, process termination, missing entitlements, and unavailable SDKs remain NOT PASS.

## Required next gate

Before B401 can be checked or merged as canonical, the exact final candidate revision still requires genuine macOS native runtime evidence for the conservative policy above. The repository must then reconcile exact head, CI/R3, reviews/threads/comments, mergeability, guarded transport, canonical parentage, and post-merge CI/R3.

B402-B406, B501-B506, Specification 005, release authority, and final Q009 independent review remain outside this candidate until dependency-ordered governance authorizes them.
