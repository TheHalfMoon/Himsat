# B404 Linux Secret Service Runtime Evidence

## Scope

This leaf implements only the B404 Linux Secret Service VRK protector and its exact Ubuntu native qualification path. It does not implement freshness anchors, protector replacement/rotation, B405-B406, B501-B506, release/compliance claims, or Q009.

## Conservative provider contract

- Reported access scope is `SAME_USER_SESSION` only.
- Per-Himsat-unlock user presence is not claimed and `REQUIRED_EACH_HIMSAT_UNLOCK` is rejected.
- `APP_EXCLUSIVE` and `SAME_USER_ACCOUNT` are rejected because the exact provider path does not prove them.
- Hardware backing remains `UNKNOWN`.
- Secret Service transport is always `EncryptionType::Dh`; plaintext Secret Service sessions are not used.
- The adapter uses only the provider default collection and never auto-unlocks a locked collection or item.

## Provider-visible metadata

Lookup attributes are exactly the fixed application identifier `com.thehalfmoon.himsat`, the fixed Himsat `xdg:schema` identifier `com.thehalfmoon.himsat`, and one opaque random 16-byte protector identifier encoded as lowercase hex. The fixed item label contains no vault-specific data. Himsat requests `application/octet-stream` for the protected value, but content type is not treated as a security binding because Secret Service providers may normalize that property; the exact GNOME qualifier accepts only the requested value or GNOME's fixed `text/plain` normalization. `VaultId`, key generation, portable policy, and VRK bytes are encoded only inside the protected secret value.

## Fail-closed behavior

Duplicate matches, malformed records, metadata drift, cross-vault/generation reuse, policy mismatch, locked state, missing items, provider errors, and secret-session cryptographic errors never return a VRK. Existing records are never silently overwritten with a different VRK. Freshness and protector replacement return `UnsupportedPolicy` in this leaf.

## Native qualification

The CI job `B404 Secret Service / ubuntu-24.04` installs the Ubuntu GNOME Keyring Secret Service provider, starts it inside a private D-Bus session, and runs two ignored exact tests in separate Himsat test processes. Phase 1 stores and reopens a VRK and verifies exact provider-visible metadata. Phase 2 reconnects after process restart, unlocks the same VRK, deletes it, locks the collection, and proves the adapter returns `Locked` instead of auto-unlocking.

Canonical qualification requires exact-head CI/R3 success, review reconciliation, expected-head guarded merge, exact parentage/tree verification, and original-attempt push-triggered post-merge CI/R3 success. Q009 remains independently unsatisfied.

## Preserved predecessor evidence

PR #97 head `b612db14dbfb76443303aca9e98c4dc2a16603b6` native job `103431436852` in CI run `34650550723` failed on attempt 1 with `OwnerMismatch` immediately after the first GNOME Keyring store. The provider path itself started and compiled successfully; the failure exposed that an exact two-attribute assumption did not account for Secret Service schema metadata. That head remains NOT PASS and is not rerun. This successor sends and verifies an explicit fixed Himsat `xdg:schema` attribute instead of accepting arbitrary provider-added metadata.

PR #97 successor head `9f9db21f5c14cde40d89f2a563fe8b5b45076659` native job `103433347406` in CI run `34651128186` also failed on attempt 1, now with `PolicyMismatch` after schema binding succeeded. This isolated the remaining mismatch to provider item properties. Secret Service permits a service to change item properties, so this successor removes content type from the security-binding decision while retaining exact label and lookup-attribute verification; that failed head remains NOT PASS and is not rerun.
