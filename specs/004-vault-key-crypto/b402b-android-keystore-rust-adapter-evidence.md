# B402B Android Keystore Rust Adapter Evidence

## Scope

B402B adds only the portable Rust `SecretProtector` behavior that sits over the already canonical-qualified B402A Android native bridge boundary. It does not select Kotlin↔Rust FFI, add a Cargo dependency, change Android native bridge bytes, claim physical-device qualification, close B402, satisfy Q009, authorize B403, or implement B405-B406/B501-B506 behavior.

```text
BASE_CANONICAL = d8d73df728d24994e9dfe9677d61e3c158c5db3e
B402A_POSTMERGE_CI = 34428219440 / SUCCESS / attempt 1 / push
B402A_POSTMERGE_R3 = 34428219449 / SUCCESS / attempt 1 / push
B402A_POSTMERGE_COMMENT = 5611691258
IMPLEMENTATION_COMMIT = c45188694076dd4c76cfc7abf5786717fac2f7ad
IMPLEMENTATION_TREE = b76cc1e6aed2c4537471fdc5be15e4f55c87b5da
ANDROID_RUST_BLOB = 9d4cfbecc027c0d4d86878e704c44b088be3ffb3
LIB_BLOB = bda050559f9efc43196b8a42eb21f94416c7a553
IMPLEMENTATION_STATUS = CANDIDATE_NOT_CANONICAL
B402_TASK_DISPOSITION = UNCHECKED_NOT_PASS
Q009 = UNSATISFIED
```

## Implemented boundary

The adapter derives provider-visible names only from one opaque random 16-byte protector identifier. The alias and record filename do not embed `VaultId`, key generation, user content, or secret material.
The Rust layer validates the expected package boundary through the injected Android backend before use. Baseline capability is `APP_EXCLUSIVE` with `NOT_REQUIRED`; `REQUIRED_EACH_HIMSAT_UNLOCK` is rejected as `UnsupportedPolicy` before key creation because B402 does not yet prove a per-operation authentication transaction.

Protected plaintext records bind the exact vault, non-zero key generation, and requested policy before the VRK can be returned. Wrong vault/generation, stored-policy mismatch, malformed record data, missing native keys, backend invalidation, and corruption remain typed fail-closed outcomes.

The VRK-bearing serialized record is held in `Zeroizing` memory around native seal/open calls. Himsat-owned persistence receives only backend-produced ciphertext. Freshness anchors and protector replacement remain `UnsupportedPolicy` because they are owned by later B501+ behavior.

## Restart and revocation semantics

A fresh Rust adapter instance starts without plaintext VRK state. The caller must configure the same reviewed policy again, causing the backend to re-inspect/create the existing native alias, before `unlock_vrk` can open the persisted encrypted record. The restart test proves this flow without restoring plaintext from Himsat-managed disk.

Wrong-vault removal validates the protected record first and preserves the protector. Correct removal deletes the native key before deleting the ciphertext record, so a later record-cleanup failure cannot leave future VRK unlock possible through that key. Successful removal resets in-memory policy/hardware state.

Hardware state is taken from the backend result and preserved without upgrade across `Unknown`, `SoftwareBacked`, and `HardwareBacked`. B402B does not infer TEE or StrongBox.
## Local qualification

The exact implementation bytes above passed the following local checks before this evidence-only successor was created:

```text
PROVENANCE_VALIDATE = PASS
GENERATED_OUTPUT_CHECK = PASS
PROVENANCE_SELF_TEST = PASS
004P_DEPENDENCY_CLOSURE = PASS
CARGO_FMT = PASS
CARGO_CLIPPY_WORKSPACE_ALL_TARGETS_ALL_FEATURES = PASS
CARGO_TEST_WORKSPACE_ALL_FEATURES = PASS
B402B_FOCUSED_TESTS = 11 passed / 0 failed
ANDROID_AARCH64_CARGO_CHECK = PASS
DIFF_CHECK = PASS
NEW_CARGO_DEPENDENCIES = NONE
```

The Android cross-check used the locally installed Android NDK 30.0.16248370 / Android clang 21 to compile `himsat-core` for `aarch64-linux-android`. The NDK/toolchain setup is local qualification infrastructure only and is not adopted into repository dependencies or workflow policy.

## B402A native dependency

B402B relies on the B402A native bridge contract already merged as canonical `d8d73df728d24994e9dfe9677d61e3c158c5db3e`. B402A proved the Android Keystore behavior on Android 17/API 37 emulator with app/UID isolation, non-exportable AES-GCM key policy, bounded no-backup ciphertext storage, tamper/missing-key/revocation handling, and `SOFTWARE_BACKED` hardware reporting.

This Rust subgrain does not transfer that emulator result into a physical-device, TEE, StrongBox, per-unlock-authentication, or selected-FFI claim.
## Required next gates

B402B remains a candidate until its exact final PR head passes original-attempt CI and R3, live reviews/threads are reconciled, and the merge is performed with explicit expected-head protection followed by exact push-triggered post-merge CI/R3 qualification.

After B402B is canonical-qualified, a separate evidence/state reconciliation must decide whether B402 itself is complete and may open B403. This document does not make that decision.

`Q009` remains unsatisfied. Repository-owner review, CI/R3, Android emulator qualification, and automated reviewers are not substitutes for the required genuinely independent substantive crypto/security review of the eventual exact implementation revision.

No release, FIPS/compliance, universal hardware-backing, universal user-presence, physical-device, or Specification 005 claim is authorized by B402B.
## Exact PR-head source mapping

The final PR head may follow the qualified implementation commit only with evidence/reconciliation changes. Transfer of the local implementation results requires this command to be empty and successful:

```text
git diff --exit-code c45188694076dd4c76cfc7abf5786717fac2f7ad..HEAD -- \
  crates/himsat-core/src/lib.rs \
  crates/himsat-core/src/vault_android_keystore.rs
```

The pinned Git blobs above are the controlling Rust source identity. Final GitHub reconciliation must record the actual empty mapping result before merge.