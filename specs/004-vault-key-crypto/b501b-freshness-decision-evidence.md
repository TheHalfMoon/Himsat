# B501B Portable Freshness Decision Evidence

## Scope

B501B adds only authenticated portable freshness open/genesis decision logic above the qualified manifest codec. It does not mutate any OS freshness anchor or claim provider atomicity.

The decision layer authenticates and canonically parses the manifest before comparing it with the protected anchor. Older epochs fail as rollback; equal epochs require the exact anchored manifest hash; exactly `N+1` requires `previous_manifest_hash == anchor.manifest_hash` and yields a typed interrupted-publication recovery decision; larger gaps fail closed.

Genesis is accepted only from explicit protected `UNINITIALIZED`, for the expected `VaultId`, with an authenticated epoch-1 manifest. A protected `PRESENT` state rejects second genesis. A missing provider record is not represented as genesis by this layer.

All platform freshness-anchor mutation methods remain `UnsupportedPolicy` until a separate provider-specific leaf proves protected serialized compare-and-set, immediate reread, and crash old-or-new semantics. Q009 remains UNSATISFIED.

## Authority precondition

Candidate base is exact B501A2 canonical merge `ef36bb2e76ae54918be4ca9e1f1b96966854741c`. This leaf must not be pushed or accepted until that merge's original-attempt post-merge CI and R3 are terminal SUCCESS.

## Independent review remediation

The initial exact head `5c65003b39aec09a731e16ffb5514e44b7487371` was not accepted after CodeRabbit identified two authenticated-before-decision ordering findings. The successor authenticates and canonically parses the manifest before comparing the anchor vault and before rejecting a protected `PRESENT` genesis state. A regression test tampers authenticated bytes while also supplying a wrong-vault anchor or already-present genesis state and requires `AuthenticationFailed` to win over the later freshness decision.
