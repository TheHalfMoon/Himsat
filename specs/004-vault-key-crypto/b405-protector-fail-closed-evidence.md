# B405 Protector Fail-Closed Qualification Evidence

## Scope

B405 qualifies the already-implemented protector failure semantics across the canonical portable contract and B401-B404 platform adapters. This leaf adds regression/evidence only; it does not add a new protector, weaken policy, change cryptography, adopt dependencies, implement freshness anchors, or satisfy Q009.

## Canonical authority prerequisite

B405 is authorized only if the B404/B405 reconciliation canonical merge `7cee4be399e3874c1cb2466d3181c932144f50b2` completes original-attempt push-triggered qualification.

```text
B404_B405_RECONCILIATION_MERGE = 7cee4be399e3874c1cb2466d3181c932144f50b2
POSTMERGE_R3 = 34655970812 / SUCCESS / attempt 1 / push
POSTMERGE_CI = 34655970804 / SUCCESS / attempt 1 / push
POSTMERGE_WINDOWS_JOB = 103448336945 / SUCCESS
```

The B404/B405 reconciliation is canonical-qualified; B405 is the only authorized implementation/qualification leaf at this frontier.

## Aggregate regression

`crates/himsat-core/tests/b405_protector_fail_closed.rs` freezes two helper-level invariants. `validate_unlock_request` rejects owner/application, vault, generation, stored-policy, and unsupported-capability mismatches before the regression test invokes any provider `unlock_vrk` operation. The regression does not claim that every future production caller is automatically forced through that helper. Separately, when the test reaches a provider boundary after successful helper validation, provider `Unavailable`, `Locked`, and `Invalidated` results remain typed errors and are not converted into VRK material.

## Platform evidence matrix

- Portable: `record_binding_fails_closed_on_owner_vault_generation_or_policy_mismatch` and the B405 integration regression prove `validate_unlock_request` rejects mismatch state before that regression explicitly calls `unlock_vrk`; this is helper-level evidence, not a claim that a separate production orchestration API enforces the helper universally.
- Apple: protected-record binding/corruption tests and native error mapping cover owner/policy mismatch, locked, unavailable, invalidated, and missing-item classes without plaintext fallback.
- Android: package/UID mismatch, wrong vault/generation, orphaned native key, injected invalidation, and partial-delete failure tests remain fail closed.
- Windows: wrong vault/generation, owner/policy/ciphertext tamper, ACL/I/O errors, restart/removal, and path-substitution tests remain fail closed.
- Linux: protected-record binding/policy tests plus the genuine GNOME Keyring qualification cover locked-provider rejection; Secret Service connection/provider failures map to `Unavailable` and no plaintext session exists.

Platforms do not fabricate an `Invalidated` claim where the native provider has no distinct invalidation semantic. Each adapter preserves the strongest typed failure it can actually prove.

## Qualification rule and limits

The exact B405 head must pass original-attempt CI/R3 on the supported matrix. Existing platform-native evidence is reused only because B405 changes no platform implementation byte; any semantic platform change would require renewed native qualification.

B405 does not close restart/revocation/unsupported-policy aggregation; B406 owns that leaf. B501-B506, Specification 005, release/FIPS/compliance claims, and Q009 remain outside this scope. Q009 remains UNSATISFIED and requires genuinely independent substantive crypto/security review of the exact final Specification 004 implementation revision.
