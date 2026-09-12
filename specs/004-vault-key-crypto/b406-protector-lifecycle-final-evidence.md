# B406 Protector Lifecycle and Platform-Negative Qualification Evidence

## Scope

B406 qualifies restart, protector removal/revocation, unsupported-policy, and platform-negative paths already implemented in the canonical B401-B405 lineage. This leaf changes no production runtime, dependency, cryptography, protector capability claim, freshness-anchor behavior, or Q009 disposition.

## Authority prerequisite

B406 is authorized only after the B405/B406 reconciliation becomes canonical and post-merge qualified. This draft is based on reconciliation candidate `c3784ce7f15712369e6432566b3bba370d080038` and must not be committed or pushed until that transition is canonical-qualified.

## Restart evidence

- Android: `process_restart_reconfigures_policy_then_unlocks_existing_record` requires policy reconfiguration before a restarted protector can unlock the existing native record.
- Windows: `native_round_trip_restart_acl_and_removal_are_fail_closed` rejects pre-configuration restart unlock, then reopens the existing DPAPI record after policy configuration.
- Linux: the genuine GNOME Keyring phase-1/phase-2 qualifier runs separate Himsat test processes in one private D-Bus/provider session; phase 2 reconnects and unlocks the existing VRK without weakening policy.
- Apple: canonical macOS Keychain qualification covers persisted-item lookup and native attribute verification. No stronger cross-device or unsupported restart claim is inferred.

## Revocation and removal evidence

- Android wrong-vault removal preserves the valid protector; correct removal prevents later unlock. Partial native delete failures remain retryable and fail closed.
- Windows exact-record removal makes later unlock return `ItemMissing`; the native qualifier also preserves ACL/root identity checks.
- Linux exact opaque-item deletion is followed by lookup verification; the genuine provider qualifier proves the record is unavailable after removal.
- Portable `VaultLease`, keyed-handle, DB, and blob tests prove non-lock revocation and lock revocation reject stale handles and publish terminal state before protected cleanup proceeds.

Revocation does not claim erasure of VRK bytes already copied by a compromised unlocked process. Full future-generation invalidation requires the later reviewed VRK-rotation flow.

## Unsupported-policy and platform-negative evidence

Every adapter reports only proven capabilities and rejects stronger unproven policy. Apple rejects stronger unproven scope/presence semantics; Android rejects unsupported per-unlock presence in the qualified baseline; Windows rejects app-exclusive/per-unlock/freshness/replacement claims; Linux rejects app-exclusive/same-user-account/per-unlock/freshness/replacement claims. No adapter silently downgrades requested policy or falls back to plaintext.

Native/provider states unavailable on a target are not fabricated as PASS. Billing-blocked, skipped, or neutral review automation is not platform qualification evidence.

## Qualification and limits

B406 becomes canonical only after its exact head passes original-attempt CI/R3 on the supported target matrix, review reconciliation, expected-head guarded merge, exact parent/tree verification, and original-attempt post-merge CI/R3. B406 closes only the remaining protector lifecycle qualification. B501-B506 remain separate later leaves.

Q009 remains UNSATISFIED and requires genuinely independent substantive crypto/security review of the exact final Specification 004 implementation revision.
