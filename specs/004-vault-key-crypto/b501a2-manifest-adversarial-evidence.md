# B501A2 Manifest Adversarial Qualification Evidence

## Scope

B501A2 adds only adversarial qualification for the already-introduced B501A manifest codec. It changes no production manifest implementation, protector, provider, dependency, anchor persistence, backup, rotation, deletion, or release authority.

The regression rejects wrong public context, truncation, trailing bytes, invalid genesis hash/epoch combinations, invalid rotation target state, generic-blob full-envelope lengths below the Round 4 minimum, and duplicate retained `(object_key_generation, blob_nonce)` pairs. The existing B501A smoke test continues to prove authenticated round trip, exact-envelope hashing, nonce-reserved production encryption, zeroized plaintext serialization, and ciphertext tamper rejection.

This leaf does not claim complete Q004 closure; Q004 later mutates every reviewed public field and all required failure classes on the exact final Specification 004 implementation revision.

B501 platform freshness-anchor methods remain `UnsupportedPolicy` unless separately qualified with exact protected serialized CAS plus crash old-or-new evidence. Q009 remains UNSATISFIED.

## Authority precondition

Candidate base is exact B501A canonical merge `1827d5b35f5e75de8a4adedbb33f90424383701c`. This leaf must not be pushed or accepted until that merge's original-attempt post-merge CI and R3 are terminal SUCCESS.
