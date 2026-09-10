# B402A Android Keystore Native Bridge Evidence

## Status

```text
BASE_CANONICAL = 6a572d0e7bfe62d5ca2c46346fed0b29f8a4060d
SUBGRAIN = B402A_ANDROID_KEYSTORE_NATIVE_BRIDGE_AND_QUALIFIER
IMPLEMENTATION_STATUS = CANDIDATE_NOT_CANONICAL
QUALIFIED_IMPLEMENTATION_COMMIT = b2543674a7f7e50752dbedd344a39e9c47941259
QUALIFIED_IMPLEMENTATION_TREE = e170adb41b1ea8455aae1639b514a7fa8ac6b9c3
BRIDGE_BLOB = cd3cccfe7a5724f5447d843aa47da71510e75f30
QUALIFIER_BLOB = a60469443bb21f1b1389d64d96502a76fcca23e9
MANIFEST_BLOB = 4298d2efcab381f319896e3755173c1802120f5a
B402_TASK_DISPOSITION = UNCHECKED_NOT_PASS
AUTHORIZED_PARENT_LEAF = B402_ANDROID_KEYSTORE_ADAPTER_ONLY
RUST_ADAPTER = NOT_INCLUDED_IN_B402A
KOTLIN_RUST_FFI_MECHANISM = NOT_SELECTED
Q009 = UNSATISFIED
```

B402A is an internal bounded subgrain of the already-authorized B402 leaf. It does not close B402, authorize B403, satisfy Q009, or select the repository-wide Kotlin↔Rust FFI mechanism.

## Native boundary

The Java bridge uses only Android platform APIs and adds no repository dependency. It:

- requires Android API 31 or newer for the qualified security-level reporting path;
- verifies the exact expected package, the calling application UID, non-isolated execution, and one-package ownership of that UID;
- accepts only fixed Himsat names plus a 16-byte opaque identifier encoded as 32 lowercase hexadecimal characters;
- creates a non-exportable 256-bit AES key in `AndroidKeyStore` with encrypt/decrypt purposes, GCM mode, no padding, randomized encryption, and no user-authentication requirement;
- re-inspects readable `KeyInfo` properties before reporting key capability;
- reports `SOFTWARE_BACKED`, `HARDWARE_BACKED`, or `UNKNOWN` from the Android security-level API without upgrading unknown state;
- stores only bounded ciphertext in `Context.getNoBackupFilesDir()`;
- uses an atomic filesystem move with replace semantics and has no non-atomic fallback;
- maps missing keys, authentication state, permanent invalidation, denied access, corruption, policy mismatch, and owner mismatch to typed status values;
- has no plaintext fallback and logs no key material or user content.

## Genuine Android runtime qualification

The native qualifier was built from this candidate source with Android SDK 37.0 command-line tools, installed on the connected ARM64 Android 17 emulator, and executed as its own application package.

```text
ANDROID_RELEASE = 17
ANDROID_API = 37
EMULATOR_PRODUCT = sdk_gphone64_arm64
EMULATOR_FINGERPRINT = google/sdk_gphone64_arm64/emu64a:17/CE2A.260420.019/15611780:userdebug/dev-keys
QUALIFIER_PACKAGE = com.thehalfmoon.himsat.b402.qualifier
QUALIFIER_APK_SHA256 = 577797b71b4a0269a18fb539607b2a69178e71e69afadb210f2b117450be6ed0
APK_SIGNATURE_VERIFICATION = PASS / v3
JAVA_BRIDGE_COMPILE = PASS / javac --release 17 -Xlint:all / android-37.0 android.jar
NATIVE_RUNTIME_RECHECK = PASS / qualified implementation commit b2543674a7f7e50752dbedd344a39e9c47941259
```

Exact runtime markers:

```text
B402_NATIVE_SCOPE=APP_EXCLUSIVE
B402_NATIVE_UID_ISOLATION=PASS
B402_NATIVE_OWNER_MISMATCH=PASS
B402_NATIVE_NAME_BOUNDARY=PASS
B402_NATIVE_HARDWARE=SOFTWARE_BACKED
B402_NATIVE_KEY_POLICY=PASS
B402_NATIVE_PRESENCE=NOT_REQUIRED
B402_NATIVE_SIZE_BOUNDARY=PASS
B402_NATIVE_NON_EXPORTABLE_SEAL=PASS
B402_NATIVE_NO_BACKUP_STORAGE=PASS
B402_NATIVE_ROUND_TRIP=PASS
B402_NATIVE_TAMPER=CORRUPT_OR_TAMPERED
B402_NATIVE_MISSING_KEY=ITEM_MISSING
B402_NATIVE_REVOCATION=ITEM_MISSING
B402_NATIVE_RECORD_REMOVAL=PASS
B402_NATIVE_KEYSTORE_QUALIFICATION=PASS
```

The emulator reported software backing. This evidence therefore does not claim TEE, StrongBox, physical hardware backing, or behavior on any production Android device.

## Exact PR-head source mapping

The qualified implementation commit above is intentionally followed only by evidence/reconciliation commits. Any final PR head is accepted as referring to this native run only when the following command is empty and exits successfully:

```text
git diff --exit-code b2543674a7f7e50752dbedd344a39e9c47941259..HEAD -- \
  crates/himsat-core/android/com/thehalfmoon/himsat/crypto/HimsatAndroidKeystoreBridge.java \
  crates/himsat-core/android/qualifier/AndroidManifest.xml \
  crates/himsat-core/android/qualifier/com/thehalfmoon/himsat/b402/qualifier/B402QualificationActivity.java
```

The three pinned Git blobs above are the controlling source identity. A final-head GitHub reconciliation comment must record the actual empty mapping result; otherwise the native evidence is not transferable to that head.

## CodeRabbit remediation

The exact-head review on predecessor `a3d9c341b336b88360564eab739f7f8de20b673a` produced four actionable comments. Forward-only implementation commit `b2543674a7f7e50752dbedd344a39e9c47941259` addresses the three implementation findings by:

- enforcing API 31 before `createKey` or `inspectKey` can reach `KeyInfo.getSecurityLevel()`, while the checked-in qualifier manifest also declares `minSdkVersion=31`;
- rejecting any AES-GCM serialized result that would exceed the bounded ciphertext record limit, with a native 1024-byte boundary assertion;
- using a unique `File.createTempFile` path in `getNoBackupFilesDir()` for every write before atomic replacement, so separate bridge instances do not share one temporary pathname.

The fourth finding is addressed by the pinned source blobs and the reproducible final-head mapping above. Review threads still require live GitHub reconciliation; this document does not self-resolve them.

## Deliberate limitations

B402A does not claim per-unlock user presence. The current qualified key is explicitly `NOT_REQUIRED`. A future implementation may support `REQUIRED_EACH_HIMSAT_UNLOCK` only when the Android authentication transaction is implemented and proven; until then the Rust B402 policy layer must reject that stronger request with `UnsupportedPolicy`.

A native `KeyPermanentlyInvalidatedException` was not physically induced on this no-authentication qualifier key. The bridge contains an explicit fail-closed mapping for that Android exception, while the B402 Rust behavior tests exercise `Invalidated` propagation. Later B406 remains responsible for broader platform-negative qualification where automation permits.

The qualifier uses a non-secret deterministic plaintext marker solely to prove encrypt/decrypt and tamper behavior. It is not a user secret or production VRK.

## Acceptance boundary

B402A may become canonical only if its exact final revision passes repository CI/R3, review reconciliation, mergeability checks, expected-head guarded merge, exact canonical parentage/tree verification, and original-attempt push-triggered post-merge CI/R3.

Even after B402A is canonical-qualified, B402 remains unchecked. The next B402 subgrain is the bounded Rust `SecretProtector` behavior layer over an injected Android-native backend contract. B403 and all later leaves remain unauthorized until B402 is separately reconciled and closed.
