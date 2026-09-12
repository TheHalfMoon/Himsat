# B501D Apple freshness-anchor candidate evidence

## Scope

B501D is bounded to the macOS Data Protection Keychain freshness slot required by B501. It extends the already-qualified B401 protector record with explicit `UNINITIALIZED` / `PRESENT(FreshnessAnchor)` state, serialized Himsat writer access, update-only `SecItemUpdate` mutation, compare-before-update semantics, and mandatory protected reread after mutation.

It does not implement backup restore, protector/recovery rotation, full VRK rotation, deletion, non-Apple freshness persistence, Specification 005 behavior, release/FIPS/compliance claims, or Q009 independent final review.

## Canonical prerequisite

The candidate base is exact B501C canonical merge `d5b19849e38bec338c7a830161418ea34384cb36` with tree `11a285859165ce6a0bcc5a8b69ea79346c96ff1a`.

B501C post-merge qualification is:

- CI `34670906144` — `SUCCESS`, attempt 1;
- R3 `34670906044` — `SUCCESS`, attempt 1;
- Windows Rust job `103491954712` — `SUCCESS`;
- durable qualification comment on PR #108: `5643260105`.

No B501D branch may be accepted from a different prerequisite base without a new reconciliation.

## Protected record semantics

The new record version stores one complete protected value containing:

- fixed Himsat record magic/version;
- `VaultId`;
- non-zero key generation;
- exact protector scope/presence policy;
- explicit freshness tag;
- freshness epoch and exact 32-byte manifest hash when the tag is `PRESENT`;
- the VRK bytes.

A newly created current-format record uses explicit `UNINITIALIZED`. Zero epoch/hash bytes are required for that state. `PRESENT` requires a non-zero epoch and binds the reconstructed anchor to the record `VaultId`.

The previously qualified B401 v1 record remains readable for VRK access but has no freshness state. It is never interpreted as `UNINITIALIZED`, and freshness mutation against a legacy record returns `UnsupportedPolicy`. This prevents an upgrade from silently manufacturing genesis authority from a record that predates B501.

## Serialized compare-and-set behavior

Every Himsat Apple protector mutation that can race with freshness state takes an advisory cross-process writer lock derived from the random `VaultId` through a domain-separated SHA-256 token. The lock file is mode `0600`, contains no secret or vault content, and is used only for writer serialization; it is not freshness storage and is never a rollback fallback.

Genesis:

1. require caller expected state `UNINITIALIZED`;
2. require new anchor `VaultId` equality;
3. acquire the writer lock;
4. read and validate the complete protected record;
5. reject legacy state as `UnsupportedPolicy`;
6. reject existing `PRESENT` as `AnchorAlreadyInitialized`;
7. issue exactly one update-only Keychain value replacement;
8. reread the protected record;
9. return success only when the reread is exactly `PRESENT(new_anchor)`.

Established advance additionally requires current state byte-for-byte equality with `expected_old` and exact `new_epoch == old_epoch + 1`. Mismatch or a gap returns `AnchorConflict` without mutation.

Any `SecItemUpdate` failure after the compare maps to `AnchorGenesisFailed` or `AnchorUpdateFailed`; a failed or mismatched reread also returns the corresponding typed failure. Normal reads still preserve `ItemMissing`, `Invalidated`, `Locked`, `Denied`, owner, policy, and corruption classes.

## Native API boundary

The candidate uses only the already-adopted safe wrappers:

- `security_framework::item::update_item`;
- `ItemUpdateOptions`;
- `ItemUpdateValue::Data`;
- `core_foundation::data::CFData`.

Himsat adds no unsafe FFI. The update query is restricted to generic-password class, fixed service, opaque account, non-synchronizable Data Protection Keychain selection, and one result. Freshness mutation never uses the add-or-update helper.

Apple documentation for `SecItemUpdate` defines update of matching Keychain items, and Apple DTS states that an update call is atomic from the caller perspective. These sources support the single-update construction but do not by themselves upgrade Himsat into a universal hardware rollback-resistance or power-loss claim:

- https://developer.apple.com/documentation/security/secitemupdate(_:_:)
- https://developer.apple.com/documentation/security/updating-and-deleting-keychain-items
- https://developer.apple.com/forums/thread/724013

Exact signed native behavior and crash old-or-new evidence remain required on the final candidate revision before acceptance.

## Portable and native negative requirements

Acceptance requires evidence that:

- malformed tag/epoch/hash combinations fail closed;
- wrong-vault reads and mutations fail closed;
- legacy records cannot become implicit genesis state;
- second genesis is rejected;
- wrong expected old anchor is rejected without mutation;
- freshness gaps are rejected without mutation;
- successful advance rereads exactly the new anchor;
- re-storing the same bound VRK preserves the established anchor;
- missing protected item is `ItemMissing` and is not a genesis path;
- native item remains `WhenPasscodeSetThisDeviceOnly` and non-synchronizable;
- stronger unproven scope/presence remains unsupported;
- process restart/crash cannot expose a torn freshness record;
- all exact-head CI/R3/review/transport/post-merge requirements remain satisfied.

## Residual limits

B501D does not qualify iOS/iPadOS. Android, Windows, and Linux freshness methods remain `UnsupportedPolicy` until an exact provider-specific mechanism proves the normative protected compare-and-advance and old-or-new contract. A plain protected or ACL-restricted rollbackable file is not accepted as a substitute.

Power-loss durability is not inferred from local unit tests. Final acceptance must preserve the strongest evidence actually obtained and must return `UnsupportedPolicy` rather than weaken the contract if the selected Apple path cannot be sufficiently qualified.

Q009 remains `UNSATISFIED`; repository-owner review, ChatGPT work, native qualification, CI/R3, or automated review do not substitute for the required final independent substantive crypto/security review of the exact Specification 004 implementation revision.

## Accepted successor and canonical merge

```text
IMPLEMENTATION_PR = 109
BASE = d5b19849e38bec338c7a830161418ea34384cb36
PREDECESSOR_HEAD = a55b2dd31d6a26dc38d4f2dff06ab52e702ff501 / NOT_ACCEPTED
ACCEPTED_HEAD = 7b57ad031f389cda0e1049e1b0435ab4bbe44c40
ACCEPTED_TREE = b7dacb4974eeb65e09847c7af2e6323d64a907df
PREMERGE_CI = 34672862145 / SUCCESS / attempt 1 / pull_request
PREMERGE_R3 = 34672862065 / SUCCESS / attempt 1 / pull_request
PREMERGE_WINDOWS_JOB = 103497413995 / SUCCESS
OWNER_RECONCILIATION_REVIEW = 5185199112 / NOT_Q009
CANONICAL_MERGE = 65ef62a2d6317a4c69382a2ce3d29ddc608b7087
PARENT_1 = d5b19849e38bec338c7a830161418ea34384cb36
PARENT_2 = 7b57ad031f389cda0e1049e1b0435ab4bbe44c40
MERGE_TREE = b7dacb4974eeb65e09847c7af2e6323d64a907df
POSTMERGE_CI = 34673377314 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34673377333 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 103498804768 / SUCCESS
Q009 = UNSATISFIED
```

The predecessor marker-format finding was repaired forward-only. CodeRabbit explicitly confirmed the successor's hexadecimal marker output addresses the finding and resolved the only inline review thread. The accepted head's signed native binary SHA-256 is `fb84aafb1177c2ff574a31f3617a3c88830290385cf6a51572840ffe978078cd`; normal qualification passed, crash stress observed 40/40 allowed old-or-new states with zero bad states, and concurrent CAS stress observed 20/20 exactly-one-winner outcomes with zero bad runs.

Original-attempt push-triggered post-merge CI `34673377314` and R3 `34673377333` both completed `SUCCESS` on attempt 1 on exact canonical merge `65ef62a2d6317a4c69382a2ce3d29ddc608b7087`; Windows Rust job `103498804768` also completed `SUCCESS`. Durable post-merge qualification is recorded on PR #109 comment `5643538198`. B501D is therefore canonical-qualified for its bounded macOS freshness scope; canonical B501 task closure remains conditional only on the separate B501/B502 reconciliation becoming exact-head reviewed, expected-head merged, exact parent/tree verified, and post-merge qualified.
