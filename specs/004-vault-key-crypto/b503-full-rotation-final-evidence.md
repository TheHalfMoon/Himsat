# B503 Full VRK Rotation Final Evidence

## Scope

B503 implements and qualifies the reviewed crash-atomic full Vault Root Key rotation sequence:

`NONE -> PREPARE -> STAGE -> VERIFY -> PUBLISH -> ANCHOR -> ACTIVATE -> RETIRE -> NONE`.

Normal writes are quiesced before rotation begins. The source generation remains the canonical active generation through PREPARE, STAGE, and VERIFY. PUBLISH and later phases select the target generation as the sole active generation while retaining source material until the target has been anchored, reopened, verified, activated, and made stable. Post-PUBLISH recovery rolls forward; pre-publication abort is permitted only while the source remains canonical.

This evidence closes only B503. It does not claim B504 exhaustive commit-point fault injection, B505 portable-backup metadata qualification, B506 deletion, Q009 independent implementation review, Specification 004 closeout, Specification 005 authority, release authority, FIPS validation, or compliance certification.

## Authority and canonical lineage

B503 authority was opened by the canonical B502/B503 reconciliation in PR #112. Canonical B503 implementation and qualification then advanced forward-only through these bounded leaves:

- PR #113 / B503A — accepted head `697e4938e93ab28462488eef03b0d3749ea59782`; canonical merge `a00878c7b9c4beac7f40b4946297c006abfa3877`.
- PR #114 / B503B — accepted head `ac794575faba37c906bb735b99533006381800e9`; canonical merge `b0b462d437d497d28a3f454c11cc1dc1b753f388`.
- PR #115 / B503C — accepted head `5b16d59b83775619b9d73d15e5594dba4fada1f6`; canonical merge `0b7ba3eb9573199694f051804c57b234ba60d4ff`.
- PR #116 / B503D — accepted head `77a85a2bb95874f41ee46993ee8d5cc588ae6e95`; canonical merge `dc6d0a8db536c3867fc7873cd66ba8bbe67466d2`.
- PR #117 / B503E — accepted head `e5b5a8a1f0ae5a981327ee3c228aaa443de2b597`; canonical merge `6f94b5b95a20ef0bb83021ddfba0a9f3991aa114`.
- PR #118 / B503F — accepted head `860efc523986f863268b0aecbc2517a53f4d8e61`; canonical merge `c1f75a6039feca369bee2b8e886ff0bb0418baa0`.
- PR #119 / B503G — accepted final head `0ec503a148ef88bf8a5d91627124bc529a8def49`; canonical merge `47e42631a49bc2a26ec02a30f03cab17e1df6036`.

The canonical B502/B503 reconciliation is merge `2d6eda2b0eaece6c129bdd88eabc112122d1f28c`.

## Implemented rotation behavior

The accepted implementation provides:

- explicit next-generation rotation identity and distinct source/target protector binding;
- quiescence proof derived from the live source session before rotation begins;
- PREPARE creation of a distinct target protector and target VRK while retaining all source material;
- optional target recovery-wrap creation bound to the target generation;
- encrypted authenticated rotation checkpoints with exact phase and generation state;
- STAGE re-encryption of source inventory under the target VRK without source retirement;
- VERIFY authentication and target-inventory verification before publication;
- PUBLISH of the target-generation manifest with source material retained;
- ANCHOR compare-and-advance on the target protected freshness state;
- ACTIVATE reopen-and-verify of the published target before retirement eligibility;
- RETIRE creation/verification of stable `RotationPhase::None` target state, source inventory/recovery/protector retirement, and checkpoint/binding cleanup;
- stable-target recovery when the final checkpoint has already been cleared;
- pre-publication abort that quarantines the target while preserving the source anchor and source generation;
- fail-closed rejection of invalid phase/generation/binding/inventory/anchor state.

## Real encrypted data qualification

B503F qualifies real cross-generation encrypted rotation rather than only in-memory state transitions. It exercises the production rotation coordinator with the qualified SQLCipher path and bounded encrypted blob path, proving source data is copied/re-encrypted to the target generation, the target decrypts and verifies under the target VRK, source ciphertext is preserved until retirement eligibility, and the source key cannot authenticate target-generation ciphertext.

PR #118 exact-head qualification:

- CI `34718754760` — SUCCESS / pull_request / attempt 1.
- R3 `34718754767` — SUCCESS / pull_request / attempt 1.

Canonical B503F merge `c1f75a6039feca369bee2b8e886ff0bb0418baa0` then passed:

- CI `34719845225` — SUCCESS / push / attempt 1.
- R3 `34719845184` — SUCCESS / push / attempt 1.

Durable B503F qualification is recorded on PR #118 comment `5648862055`.

## Native Apple qualification

B503G adds a native macOS qualifier that executes the production full-rotation coordinator through real Apple Data Protection Keychain protectors. The accepted exact head is `0ec503a148ef88bf8a5d91627124bc529a8def49`, tree `35af44d50517bf7df76e9df8de0ec37bdbf6a419`.

The final probe was rebuilt from that exact revision and executed inside the matching signed/provisioned application context. Signing evidence:

- signing authority: `Apple Development: <redacted> (5VU39N6XKG)`;
- TeamIdentifier: `5QJ8QL886F`;
- application identifier: `5QJ8QL886F.com.thehalfmoon.himsat.b401.qualifier`;
- unsigned candidate binary SHA-256: `71bb3c2a2284d438fed00086c716cb43491a18cf3cd2ae0e69b5143acf2fa5fd`;
- signed executable SHA-256: `5bad9fd560c342dc69aa79dafc055873045dd98140d1f769abfcdb0287c505b9`;
- CDHash: `4a4630f53b97fbc594e2cdc1644d1f07d55b460f`;
- `codesign --verify --deep --strict`: PASS.

Native attempt 1 on the accepted exact head exited `0` with empty stderr and the complete required marker set:

```text
B503_NATIVE_APPLE_CLEANUP=PASS
B503_NATIVE_APPLE_SOURCE_SCOPE=SAME_USER_ACCOUNT
B503_NATIVE_APPLE_TARGET_SCOPE=SAME_USER_ACCOUNT
B503_NATIVE_APPLE_PRESENCE=NOT_REQUIRED
B503_NATIVE_APPLE_ACCESSIBILITY=WHEN_PASSCODE_SET_THIS_DEVICE_ONLY
B503_NATIVE_APPLE_SYNCHRONIZABLE=FALSE
B503_NATIVE_APPLE_DISTINCT_PROTECTORS=PASS
B503_NATIVE_APPLE_DISTINCT_VRK=PASS
B503_NATIVE_APPLE_SOURCE_RETAINED_THROUGH_RETIRE=PASS
B503_NATIVE_APPLE_TARGET_REOPEN=PASS
B503_NATIVE_APPLE_STABLE_RECOVERY=PASS
B503_NATIVE_APPLE_ROTATION_QUALIFICATION=PASS
```

The qualifier proves two distinct opaque Keychain protector records, target generation G+1, source retention through RETIRE eligibility, target reopen, stable-target restart recovery, device-only/passcode-set accessibility, synchronization disabled, and explicit cleanup success. Durable native evidence is PR #119 comment `5649155969`.

## Preserved negative and review evidence

The native predecessor `7c25f81a2e40a311e2d18d6e2ed342337b7a9d59` is permanently preserved as NOT PASS. It was correctly signed but exited `1` with `Protector(UnsupportedPolicy)` and no B503 native PASS markers. Investigation showed the qualifier reused a retired source protector object after completion rather than reconstructing/reconfiguring protectors as a real restart does. The defect was repaired forward-only; the failed head was not rerun or reclassified.

Intermediate `32305caab72843c325794c5d3ddbf10fbf66a23a` repaired restart simulation and produced native success, but a substantive CodeRabbit finding correctly identified that best-effort `Drop` cleanup discarded Keychain cleanup failures. Final head `0ec503a148ef88bf8a5d91627124bc529a8def49` repaired that finding forward-only by making cleanup a fail-closed success gate before final qualification markers and reporting fallback `Drop` failures. The sole inline review thread was resolved.

Repository-owner reconciliation review `5188403687` is governance evidence only and is explicitly `NOT_Q009`.

## Exact final qualification and guarded merge

Final PR #119 exact-head qualification on `0ec503a148ef88bf8a5d91627124bc529a8def49`:

- CI `34723156586` — SUCCESS / pull_request / attempt 1;
- R3 `34723156531` — SUCCESS / pull_request / attempt 1;
- Windows Rust job `103632551046` — SUCCESS.

PR #119 was merged only through an observed guarded request using `expected_head_sha = 0ec503a148ef88bf8a5d91627124bc529a8def49` and `merge_method = merge`.

Canonical merge `47e42631a49bc2a26ec02a30f03cab17e1df6036` has exact parents:

- `c1f75a6039feca369bee2b8e886ff0bb0418baa0`;
- `0ec503a148ef88bf8a5d91627124bc529a8def49`.

Its tree is `35af44d50517bf7df76e9df8de0ec37bdbf6a419`, exactly matching the accepted head tree.

Original-attempt push-triggered qualification on the canonical merge succeeded:

- CI `34723648055` — SUCCESS / push / attempt 1;
- R3 `34723648064` — SUCCESS / push / attempt 1;
- Windows Rust job `103633868719` — SUCCESS on the original CI attempt.

Durable canonical B503G post-merge qualification is PR #119 comment `5649245286`.

## Residual limits and B504 boundary

B503 proves the normal seven-phase coordinator, bounded real encrypted cross-generation rotation, retry/restart behavior already covered by the accepted implementation tests, and native Apple target/source protector behavior. It does not claim exhaustive crash injection immediately before and after every durable rotation commit point.

B504 owns that next boundary. B504 must inject failures before and after every rotation commit point and prove that every resulting durable state is recoverable without selecting unverifiable data: before publication the source must remain canonical/decryptable or abort-safe; at and after publication recovery must fail closed or roll forward to the authenticated target, never destructively roll backward.

B504 is not authorized merely by this evidence file. Authority opens only if the separate B503/B504 state reconciliation becomes exact-head qualified, explicitly expected-head merged, exact parent/tree verified, and exact-post-merge CI/R3 qualified.

Q009 remains `UNSATISFIED`. Repository-owner review, ChatGPT work, CI/R3, CodeRabbit, and other automation do not substitute for the required genuinely independent substantive crypto/security review of the exact final Specification 004 implementation revision.
