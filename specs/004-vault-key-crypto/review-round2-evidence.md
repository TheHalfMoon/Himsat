# Specification 004A Independent Security Review Evidence — Round 2

## Review identity

```text
REVIEW_ONLY_PR = 18
REVIEW_BASE_SHA = 384608c8fc13531c399f3726caa5022eb3612aa2
REVIEWED_CANONICAL_SHA = 6d1bbc9b55690939833917eebd447c361627f48a
REVIEWER = coderabbitai
GITHUB_EVIDENCE_COMMENT = 5561662451
REVIEW_DISPOSITION = CHANGES_REQUIRED
BLOCKING_FINDINGS = 2
IMPLEMENTATION_AUTHORITY = BLOCKED
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED
```

PR #18 is review-only. Its head points directly at exact canonical remediation merge `6d1bbc9b55690939833917eebd447c361627f48a`; it must not be merged.

The substantive CodeRabbit review explicitly checked out and reviewed exact `HEAD = 6d1bbc9b55690939833917eebd447c361627f48a`, reviewed the first remediation lineage, and returned `DISPOSITION = CHANGES_REQUIRED`.

Qodo billing-blocked output, skipped CodeRabbit automatic review, CI/R3 success, author self-review, and historical PR #12 evidence are not substitutes for this exact-revision independent review.

## Round-2 blocking findings

### D017 — Apple Keychain protector policy remains underspecified

The reviewed design did not normatively freeze the Apple Keychain accessibility class, device-only versus migratable behavior, synchronization prohibition, passcode-removal/invalidation behavior, cross-device restore behavior, or typed failure outcomes strongly enough to prevent an implementation from creating a recovery path outside the explicit opt-in recovery envelope.

Required remediation:

- freeze a non-synchronizable `ThisDeviceOnly` baseline;
- prohibit iCloud Keychain synchronization and backup-migratable VRK material;
- define user-presence/access-control behavior when requested;
- define passcode removal, Keychain loss/reset, access-group mismatch, migration, and restore failure behavior;
- require per-target platform evidence before claiming a stronger Apple scope/presence capability.

### D018 — Freshness manifest format remains underspecified

The reviewed design had a freshness state machine but did not freeze the authenticated manifest construction sufficiently. It lacked a distinct manifest-purpose key/domain, canonical envelope/plaintext encoding, exact AAD, nonce rules, field bounds, exact `manifest_hash` bytes, parser rejection rules, and complete atomic compare-and-advance failure semantics.

Required remediation:

- add a distinct HKDF manifest-purpose domain;
- freeze a versioned bounded canonical manifest envelope and plaintext representation;
- freeze exact AAD and XChaCha20-Poly1305 nonce lifecycle;
- define `manifest_hash = SHA-256(exact canonical manifest envelope bytes)`;
- reject duplicate, missing, unknown, reordered, non-canonical, truncated, or trailing fields before state use;
- require parse + authentication + consistency validation before manifest-controlled state becomes authoritative;
- define compare-and-swap freshness-anchor mutation, reread verification, typed conflicts/failures, and fail-closed behavior where a platform cannot provide the required atomic primitive.

## Prior findings disposition

The independent round-2 review reported:

```text
D001-D004 = SUBSTANTIVELY_RESOLVED
D005 = PARTIALLY_RESOLVED_BY_D017
D006-D010 = SUBSTANTIVELY_RESOLVED
D011 = PARTIALLY_RESOLVED_BY_D018
D012-D016 = SUBSTANTIVELY_RESOLVED
```

No previously resolved finding is reopened unless a later exact-revision review identifies a concrete regression.

## Non-blocking recommendations retained

- keep `ARGON2ID_RFC9106_64M_V1` fixed for v1; any future calibrated profile needs a new identifier and review;
- keep a fresh 24-byte OS-CSPRNG nonce for every XChaCha20-Poly1305 encryption attempt and bound future inventory parsing before allocation;
- preserve the SQLCipher exact-provider/build/provenance gate;
- preserve explicit residual-risk limits for detached backups, OS secret-store scope, unlocked-process compromise, physical erasure, and fresh-device rollback detection.

## Residual risks retained

- a compromised unlocked process can access plaintext and resident secrets;
- kernel, firmware, hardware, and physical-memory compromise remain outside the portable vault boundary;
- ciphertext sizes, counts, filesystem state, operation timing, upload timing, and rotation cadence can leak metadata;
- offline recovery-passphrase attacks remain possible;
- a fresh device can authenticate a backup but cannot prove global newest-ness without prior trusted freshness state;
- crypto-erasure does not prove physical-media, snapshot, provider-copy, or user-duplicate erasure;
- detached backups with a valid recovery-wrapped VRK remain usable while the holder retains the recovery passphrase.

## Round-2 remediation rule

The review above applies only to exact SHA `6d1bbc9b55690939833917eebd447c361627f48a` and is `CHANGES_REQUIRED` evidence, not approval.

After D017-D018 are repaired:

1. exact-head CI and Diffcipline R3 must succeed on the remediation candidate;
2. the remediation must merge with expected-head protection after live diff/review/thread/comment/main reconciliation;
3. post-merge CI and R3 must succeed on the new canonical merge;
4. a successor review-only PR must point directly at the new exact canonical design SHA with no review-content commit layered on top;
5. a new substantive independent crypto/security review must disposition that exact SHA;
6. any new blocking finding must be resolved forward and re-reviewed;
7. dependency/provider selection and 004B implementation remain blocked until an exact canonical design revision has a qualifying independent disposition with no unresolved blocking findings.
