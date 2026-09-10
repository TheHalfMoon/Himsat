# B401 Apple Keychain Final Evidence

## Scope

B401 closes only the bounded macOS Apple Data Protection Keychain protector adapter and its exact native qualification path. It does not qualify iOS/iPadOS, Android, Windows, Linux, Secure Enclave backing, per-unlock user presence, freshness anchors, backup/restore, full VRK rotation, deletion, Specification 005, release/FIPS/compliance posture, or Q009 independent final security review.

The accepted implementation changed only the bounded B401 candidate surfaces authenticated by the independently canonical trusted gate:

```text
Cargo.lock
THIRD_PARTY_NOTICES.md
crates/himsat-core/Cargo.toml
crates/himsat-core/examples/b401_apple_keychain_probe.rs
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_apple_keychain.rs
governance/generated/sbom.json
governance/provenance/registry.json
specs/004-vault-key-crypto/b401-apple-keychain-candidate-evidence.md
tools/004p_dependency_closure.py
```

## Accepted implementation revision

```text
IMPLEMENTATION_PR = 78
BASE = 96b74783013f79eac39e8b942f89e6c1a32fbabc
HEAD = 145e0db14a957760adb811e557372f4aa8f24bd5
TREE = 21aff2e9260020471c9537b9d751ef4995157e48
CHANGED_FILES = 10
ADDITIONS = 1386
DELETIONS = 18
```

The accepted macOS adapter reports only the platform properties actually proven by the native qualifier:

```text
ACCESS_SCOPE = SAME_USER_ACCOUNT
USER_PRESENCE = NOT_REQUIRED
ACCESSIBILITY = WHEN_PASSCODE_SET_THIS_DEVICE_ONLY
SYNCHRONIZABLE = FALSE
HARDWARE_BACKING = UNKNOWN
APP_EXCLUSIVE = UNSUPPORTED_POLICY
SAME_USER_SESSION = UNSUPPORTED_POLICY
REQUIRED_EACH_HIMSAT_UNLOCK = UNSUPPORTED_POLICY
```

Existing-item operations verify native service/account/accessibility/non-synchronization state before release or mutation. Protected records bind vault, generation, and policy. Cross-vault/generation/policy reuse and wrong-vault removal fail closed. Keychain value buffers are zeroized on drop around fallible provider operations. Unknown Apple Security status codes map conservatively to `Unavailable`, corrupt stored discriminants fail as `CorruptOrTampered`, and there is no plaintext fallback.

## Dependency and provenance closure

B401 adds exactly four target-specific Apple packages to the already qualified provider closure:

```text
core-foundation 0.10.1
core-foundation-sys 0.8.7
security-framework 3.7.0
security-framework-sys 2.17.0
```

`security-framework = 3.7.0` is restricted to `cfg(target_os = "macos")`, exact checksums and immutable source/license evidence are present, notices and SBOM/provenance registry are updated, and the accepted exact-head dependency-closure gate passed. No iOS implementation surface is adopted by B401.

## Preserved predecessor evidence

The detailed predecessor record remains in `b401-apple-keychain-candidate-evidence.md`. No failed, cancelled, green-but-rejected, or reviewed predecessor was rerun, rewritten, force-pushed, or promoted merely because automation was green.

```text
2e6ea72306e3ec76b4bee4062d3a07f92efa73d1 = FAILURE_NOT_PASS / dependency-closure assumption
83397af010817dd4dc4f83d6ddc304f7caa53f78 = GREEN_AUTOMATION_NOT_ACCEPTED / wrong-vault removal binding
27e864970ac86710732adf372699825b529e22ba = GREEN_AUTOMATION_NOT_ACCEPTED / unproven APP_EXCLUSIVE + iOS surface
7c759c75f9c1ed98551cd41a0b4ef3f147ce9466 = FAILURE_NOT_PASS / Linux cfg defect
b9b9748401bf1c5e28ff81220d199f4515449f3a = GREEN_AUTOMATION_NOT_ACCEPTED / CodeRabbit security findings
```

Additional cancelled/transient predecessor history remains preserved in PR #78 and is not qualification evidence.

## Canonical B307/B401 authority prerequisite

Reconciliation PR #77 established the conditional B401 authority boundary:

```text
RECONCILIATION_HEAD = 1efe173b95f71e6d4b075d4e70cf6d7c1fae7734
RECONCILIATION_TREE = c6f599556934eb9601587b0abe0cd50946dd5d47
PREMERGE_CI = 34385853808 / run #200 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34385853809 / run #177 / SUCCESS / attempt 1 / pull_request
OWNER_RECONCILIATION_REVIEW = 5158089505 / NOT_Q009
EXPECTED_HEAD_BINDING = 5606508666
CANONICAL_MERGE = ba32dfc21d025a189cddfdd9f46c48fdcd327e1e
POSTMERGE_CI = 34387075554 / run #201 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34387075495 / run #178 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 102585926143 / SUCCESS
POSTMERGE_QUALIFICATION_COMMENT = 5606684163
```

The actual PR #77 merge used the expected head and `merge_method = merge`; parentage and tree were exact. Its conditional authority therefore resolved to B401 only before B401 implementation proceeded.

## Canonical trusted-gate prerequisite

PR #79 independently hardened the B401 adoption gate so the candidate could not authenticate its own mutable gate or omit candidate artifacts from the trusted blob set.

```text
GATE_HEAD = c98d4375804e519886b08731f652861189b978c9
PREMERGE_CI = 34404865951 / run #209 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34404866182 / run #186 / SUCCESS / attempt 1 / pull_request
CANONICAL_MERGE = 96b74783013f79eac39e8b942f89e6c1a32fbabc
POSTMERGE_CI = 34406084492 / run #210 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34406084590 / run #187 / SUCCESS / attempt 1 / push
```

The canonical gate materializes the trusted adoption gate from the immutable comparison base and authenticates every bounded B401 candidate artifact by Git blob identity.

## Exact-head native macOS qualification

Earlier ad-hoc, missing-entitlement, expired-profile, missing-account, and missing-device attempts remain NOT PASS. After a valid Apple Developer account was associated with team `5QJ8QL886F`, Xcode registered the explicit local `My Mac` destination and created a valid development provisioning profile for the B401 qualifier App ID.

The qualifier binary was rebuilt from exact accepted head `145e0db14a957760adb811e557372f4aa8f24bd5`; the clean worktree matched the live remote branch and produced:

```text
B401_PROBE_BINARY_SHA256 = fe6fd497331f8ff5ed3ca50ee98029dfa4f2d889b5848b1fd67a3abf6490ca1e
TARGET = arm64 macOS
PROVISIONING_PROFILE = Mac Team Provisioning Profile: com.thehalfmoon.himsat.b401.qualifier
PROFILE_TEAM = 5QJ8QL886F
PROFILE_EXPIRATION = 2026-09-16T23:52:42Z
APPLICATION_IDENTIFIER = 5QJ8QL886F.com.thehalfmoon.himsat.b401.qualifier
KEYCHAIN_ACCESS_GROUP = 5QJ8QL886F.com.thehalfmoon.himsat.b401.qualifier
CODESIGN_VERIFY = PASS
NATIVE_EXIT = 0
B401_NATIVE_SCOPE = SAME_USER_ACCOUNT
B401_NATIVE_PRESENCE = NOT_REQUIRED
B401_NATIVE_ACCESSIBILITY = WHEN_PASSCODE_SET_THIS_DEVICE_ONLY
B401_NATIVE_SYNCHRONIZABLE = FALSE
B401_NATIVE_STRONGER_POLICY = UNSUPPORTED_POLICY
B401_NATIVE_KEYCHAIN_QUALIFICATION = PASS
NATIVE_QUALIFICATION_COMMENT = 5610427033
```

This evidence proves only the conservative macOS policy above. It does not prove `APP_EXCLUSIVE`, per-Himsat-unlock user presence, Secure Enclave backing, iOS behavior, or any cross-platform property.

## Exact-head pre-merge qualification and review reconciliation

```text
PREMERGE_CI = 34407235044 / run #211 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34407235017 / run #188 / SUCCESS / attempt 1 / pull_request
CODERABBIT_STATUS = SUCCESS
OPEN_REVIEW_THREADS = 0
OWNER_RECONCILIATION_REVIEW = 5161109268 / NOT_Q009 / READY_FOR_B401_LEAF_MERGE
QODO = BILLING_BLOCKED / NOT_PASS / NOT_Q009
```

The predecessor CodeRabbit major findings were resolved forward-only. Both old inline threads are resolved and outdated on the accepted head. Repository-owner reconciliation is governance evidence only and does not satisfy Q009.

## Guarded merge and canonical parentage

Durable pre-merge transport binding is PR #78 comment `5610437045`.

The actual observed merge invocation used:

```text
EXPECTED_HEAD_SHA = 145e0db14a957760adb811e557372f4aa8f24bd5
MERGE_METHOD = merge
```

GitHub returned:

```text
MERGED = true
CANONICAL_MERGE = 880165a40108bdf0c27e7246c1b890e8456e9768
```

Canonical parentage and tree are exact:

```text
PARENT_1 = 96b74783013f79eac39e8b942f89e6c1a32fbabc
PARENT_2 = 145e0db14a957760adb811e557372f4aa8f24bd5
MERGE_TREE = 21aff2e9260020471c9537b9d751ef4995157e48
```

The merge tree exactly equals the accepted implementation tree.

## Post-merge qualification

```text
POSTMERGE_CI = 34419047465 / run #212 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34419047471 / run #189 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 102690257334 / SUCCESS
POSTMERGE_R3_JOB = 102690257246 / SUCCESS
POSTMERGE_QUALIFICATION_COMMENT = 5610513209
```

Canonical merge `880165a40108bdf0c27e7246c1b890e8456e9768` is therefore exact-post-merge qualified for the bounded B401 implementation.

## Preserved limitations

`P011` remains unchecked / NOT PASS. `B305R002` remains unchecked / NOT PASS. Neither historical transport gap is repaired by B401.

Q009 remains unsatisfied. Repository-owner review, native qualification, implementation tests, CI/R3 automation, CodeRabbit status, Qodo billing-blocked output, or other automation do not substitute for the required genuinely independent substantive crypto/security review of the eventual exact implementation revision.

B401 does not authorize release, FIPS/compliance claims, universal app-exclusive protection, universal user-presence enforcement, universal hardware backing, iOS qualification, or later freshness/backup/rotation/deletion behavior.

## B402 rebound

B402 may begin only after the B401/B402 evidence-state reconciliation itself becomes exact-head qualified, reconciled against live `main`/base/head/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection and `merge_method = merge`, has exact canonical parentage/tree proven, and then passes original-attempt push-triggered post-merge CI and R3.

After that qualification, B402 is the only authorized implementation leaf. B402 is bounded to the Android Keystore adapter with actual app/UID scope, authentication policy, invalidation, and hardware capability reporting. B403-B406, B501-B506, Q009, Specification 005, release/FIPS/compliance claims, and unrelated donor adoption remain out of scope until separately authorized.
