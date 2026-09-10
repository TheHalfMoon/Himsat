# B402 Android Keystore Final Evidence

## Status

```text
BASE_CANONICAL = e8aeb1e09591358850b109fdf423a6bfd0b60003
LEAF = B402_ANDROID_KEYSTORE_ADAPTER
B402A_NATIVE_BRIDGE = CANONICAL_QUALIFIED
B402A_CANONICAL_MERGE = d8d73df728d24994e9dfe9677d61e3c158c5db3e
B402B_RUST_ADAPTER = CANONICAL_QUALIFIED
B402B_CANONICAL_MERGE = e8aeb1e09591358850b109fdf423a6bfd0b60003
B402_DISPOSITION = CANONICAL_CLOSED_IF_THIS_RECONCILIATION_QUALIFIES
NEXT_LEAF = B403_WINDOWS_DPAPI_CNG_ADAPTER_ONLY_IF_RECONCILIATION_QUALIFIED
KOTLIN_RUST_FFI_MECHANISM = NOT_SELECTED
Q009 = UNSATISFIED
```

This evidence reconciles the two bounded B402 subgrains already merged and post-merge qualified. It does not itself become canonical until this reconciliation passes exact-head CI/R3, expected-head merge, exact parentage/tree verification, and original-attempt post-merge CI/R3.

## B402A Android-native boundary

PR #81 implemented the Android-native bridge and native qualifier without adding a repository dependency. Final accepted head `bbd8a95924cb998f1651deb984fced3157239ff9` mapped exactly to qualified implementation commit `b2543674a7f7e50752dbedd344a39e9c47941259` for the bridge, manifest, and qualifier source blobs.

The exact native Android qualification ran on Android 17 / API 37 ARM64 emulator `sdk_gphone64_arm64` and produced qualifier APK SHA-256 `10d4420b51f31ca6e63b68dff7b33336cbb55bdc0aff51afba298703c57b0a1e`.

```text
B402_NATIVE_SCOPE = APP_EXCLUSIVE
B402_NATIVE_UID_ISOLATION = PASS
B402_NATIVE_OWNER_MISMATCH = PASS
B402_NATIVE_NAME_BOUNDARY = PASS
B402_NATIVE_HARDWARE = SOFTWARE_BACKED
B402_NATIVE_KEY_POLICY = PASS
B402_NATIVE_PRESENCE = NOT_REQUIRED
B402_NATIVE_SIZE_BOUNDARY = PASS
B402_NATIVE_NON_EXPORTABLE_SEAL = PASS
B402_NATIVE_NO_BACKUP_STORAGE = PASS
B402_NATIVE_ROUND_TRIP = PASS
B402_NATIVE_TAMPER = CORRUPT_OR_TAMPERED
B402_NATIVE_MISSING_KEY = ITEM_MISSING
B402_NATIVE_REVOCATION = ITEM_MISSING
B402_NATIVE_RECORD_REMOVAL = PASS
B402_NATIVE_KEYSTORE_QUALIFICATION = PASS
```

Durable exact-head native evidence is PR #81 comment `5611500574`. The qualifier proved only the emulator configuration actually observed. It reported `SOFTWARE_BACKED`; therefore B402 does not claim TEE, StrongBox, physical-device hardware backing, or any hardware-backed baseline.

CodeRabbit predecessor review `5161816963` produced four actionable findings. They were repaired forward-only before the accepted final head. The final source mapping remained empty for the qualified native source files.

PR #81 exact-head gates and guarded merge:

```text
HEAD = bbd8a95924cb998f1651deb984fced3157239ff9
TREE = 9dd68ecbc7e0b219e538ba0e216bb703bed22646
PREMERGE_CI = 34427423369 / SUCCESS / attempt 1
PREMERGE_R3 = 34427423363 / SUCCESS / attempt 1
OWNER_RECONCILIATION_REVIEW = 5161911286 / NOT_Q009
EXPECTED_HEAD_BINDING = 5611588102
CANONICAL_MERGE = d8d73df728d24994e9dfe9677d61e3c158c5db3e
PARENT_1 = 6a572d0e7bfe62d5ca2c46346fed0b29f8a4060d
PARENT_2 = bbd8a95924cb998f1651deb984fced3157239ff9
MERGE_TREE = 9dd68ecbc7e0b219e538ba0e216bb703bed22646
POSTMERGE_CI = 34428219440 / SUCCESS / attempt 1
POSTMERGE_R3 = 34428219449 / SUCCESS / attempt 1
POSTMERGE_QUALIFICATION_COMMENT = 5611691258
```

## B402B portable Rust adapter

PR #82 implemented the portable `SecretProtector` behavior layer over an injected Android-native backend contract. It selected no Kotlin↔Rust FFI mechanism and introduced no new Cargo dependency.

The final accepted B402B diff was limited to `crates/himsat-core/src/lib.rs` and `crates/himsat-core/src/vault_android_keystore.rs`, with 893 additions and zero deletions relative to the B402A canonical base.

The final adapter behavior includes:

- package/application ownership verification through the backend boundary;
- fresh protector IDs generated from the approved OS CSPRNG path and an explicit persisted-ID restoration path;
- fixed opaque provider-visible alias/record names that do not embed `VaultId`, generation, user content, or secret material;
- conservative `APP_EXCLUSIVE` reporting only for the Android package/UID isolation boundary already proven by B402A;
- `NOT_REQUIRED` presence baseline and fail-closed `UnsupportedPolicy` for unproven per-unlock presence;
- exact vault/generation/policy binding before VRK release;
- rejection of a different VRK for an already populated same generation;
- restart behavior that never restores plaintext VRK from Himsat-managed disk and requires protector unlock again;
- hardware capability state preserved as reported by the native backend without upgrade;
- orphan-record detection that does not silently create a replacement native key;
- retry-safe cleanup when either native-key deletion or ciphertext-record deletion fails independently;
- freshness-anchor and protector-replacement operations left `UnsupportedPolicy` because they belong to later separately governed leaves.

The final local qualification before push passed provenance validation, generated-output closure, provenance self-test, 004P dependency closure, formatting, Clippy, full workspace tests, 15 focused B402B tests, Android `aarch64-linux-android` cross-check, and diff check.

CodeRabbit substantive review `5168407521` on predecessor head `4d7e57e6507ac7db8066fc5ff7bd123cb8ef7dff` identified three lifecycle/data-integrity findings: same-generation VRK replacement, partial-delete retry behavior, and protector-ID lifecycle ambiguity. All three were repaired forward-only in final accepted head `b88c3aee7e674f7981457b2891b356ef23e8b1bb`; each review thread was replied to with evidence and resolved/outdated before merge. Empty CodeRabbit review objects on the final head are not represented as substantive approval.

PR #82 exact-head gates and guarded merge:

```text
HEAD = b88c3aee7e674f7981457b2891b356ef23e8b1bb
TREE = c7788a4a5eda8f7eddfc3f2722cc08afaa346e12
PREMERGE_CI = 34491693211 / SUCCESS / attempt 1
PREMERGE_R3 = 34491693214 / SUCCESS / attempt 1
WINDOWS_JOB = 102919775559 / SUCCESS
OWNER_RECONCILIATION_REVIEW = 5168770520 / NOT_Q009
EXPECTED_HEAD_BINDING = 5620798891
UNRESOLVED_REVIEW_THREADS = 0
CANONICAL_MERGE = e8aeb1e09591358850b109fdf423a6bfd0b60003
PARENT_1 = d8d73df728d24994e9dfe9677d61e3c158c5db3e
PARENT_2 = b88c3aee7e674f7981457b2891b356ef23e8b1bb
MERGE_TREE = c7788a4a5eda8f7eddfc3f2722cc08afaa346e12
POSTMERGE_CI = 34493112106 / SUCCESS / attempt 1
POSTMERGE_R3 = 34493112084 / SUCCESS / attempt 1
POSTMERGE_WINDOWS_JOB = 102924612214 / SUCCESS
POSTMERGE_QUALIFICATION_COMMENT = 5620945182
```

## B402 capability disposition

B402 qualifies only this conservative Android baseline:

```text
ACCESS_SCOPE = APP_EXCLUSIVE_WHEN_PACKAGE_UID_ISOLATION_IS_IN_FORCE
USER_PRESENCE = NOT_REQUIRED
REQUIRED_EACH_HIMSAT_UNLOCK = UNSUPPORTED_POLICY
HARDWARE_BACKING = REPORTED_NOT_ASSUMED
QUALIFIED_RUNTIME_HARDWARE = SOFTWARE_BACKED_EMULATOR_ONLY
PLAINTEXT_FALLBACK = NONE
FRESHNESS_ANCHOR = UNSUPPORTED_POLICY_PENDING_B501
PROTECTOR_REPLACEMENT = UNSUPPORTED_POLICY_PENDING_LATER_ROTATION_WORK
```

B402 does not claim physical key erasure, physical flash erasure, universal Android-version/device behavior, TEE/StrongBox, per-unlock biometric/device-credential presence, a selected Kotlin↔Rust FFI mechanism, rollback-resistant freshness storage, crash-atomic active-vault deletion, release readiness, FIPS, or compliance posture.

B506 remains responsible for active-vault deletion across canonical/derived/temp/wrap surfaces. B402 only makes its own protector-component cleanup retryable; it does not absorb the vault-wide crash-atomic deletion contract.

## Review and historical limitations

Q009 remains unsatisfied. Repository-owner reconciliation, local tests, native qualification, CI/R3 automation, resolved bot findings, empty exact-head bot review objects, and Cubic neutral output do not substitute for the required genuinely independent substantive crypto/security review of the exact final Specification 004 implementation revision.

Historical `P011` and `B305R002` remain unchecked / NOT PASS because their exact expected-head merge request transport evidence is not reconstructible post hoc. B402 evidence does not alter those historical limitations.

## Reconciliation transition

This reconciliation changes only Specification 004 evidence/state surfaces. It adopts no dependency, donor code, workflow, provider, SBOM, generated artifact, Android runtime implementation byte, or B403 implementation byte.

Only after this reconciliation exact head passes original-attempt pull-request CI/R3, live diff/review/thread/mergeability reconciliation, explicit `expected_head_sha` merge, exact parentage/tree proof, and original-attempt push-triggered post-merge CI/R3 does B402 become `CANONICAL_CLOSED` and conditional authority resolve to `B403_ONLY`.

B403 is bounded to the Windows current-user DPAPI/CNG-class adapter with accurately reported `SAME_USER_ACCOUNT` scope. `APP_EXCLUSIVE` and `REQUIRED_EACH_HIMSAT_UNLOCK` remain unsupported unless a separately reviewed mechanism proves them. B404-B406 and B501-B506 remain separate later leaves.
