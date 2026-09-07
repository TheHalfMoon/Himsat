# Specification 004B1 B101 — Canonical Final Evidence

## Purpose

This record reconciles the exact live GitHub evidence for the Specification 004B1 B101 portable vault contract leaf and bounds the forward-only transition to B102.

It preserves both positive qualification evidence and the historical merge-transport evidence limitation discovered after B101 became canonical. It does not rewrite prior history and it does not authorize B102 until this reconciliation itself is merged with an explicit expected-head guard and passes exact post-merge CI/R3.

## P011 reconciliation evidence

```text
P011_PR = 37
P011_HEAD = 367348c2b11add2c15f7d25af92ca4679b2a19d1
P011_PREMERGE_CI = 34145714886_SUCCESS
P011_PREMERGE_R3 = 34145714971_SUCCESS
P011_CANONICAL_MERGE = 7ecba93ae0763a5165dd99ce0ec190cb106906de
P011_POSTMERGE_CI = 34146939013_SUCCESS
P011_POSTMERGE_R3 = 34146938921_SUCCESS
P011_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
```

Live GitHub proves the exact PR head, merge SHA, canonical parentage, and exact pre/post-merge qualification. No durable repository artifact exposes the historical merge API request body for PR #37, so this record does not claim transport-level proof that `expected_head_sha` was supplied for that historical merge.

That absence remains evidence. It is not converted to PASS by inference from successful parentage, CI, R3, or later work.

## B101 implementation lineage

```text
B101_PR = 38
B101_BASE = 7ecba93ae0763a5165dd99ce0ec190cb106906de
B101_INITIAL_HEAD = 327a6f2481f2905181a6fdff09a5a735d21bf314
B101_INITIAL_R3 = 34148047439_FAILURE_FORMAT_ONLY
B101_FINAL_HEAD = ea82e1246095bb921dc9e7e40716076edbda41a6
B101_PREMERGE_CI = 34148418394_SUCCESS
B101_PREMERGE_R3 = 34148418341_SUCCESS
B101_CANONICAL_MERGE = 95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c
B101_POSTMERGE_CI = 34149709088_SUCCESS
B101_POSTMERGE_R3 = 34149708977_SUCCESS
B101_EXPECTED_HEAD_TRANSPORT_PROOF = NOT_RECONSTRUCTIBLE_POST_HOC
```

The initial B101 R3 failure was a formatting-only defect. The exact final B101 head is a forward-only rustfmt repair and passed both exact-head CI and R3.

Canonical merge `95cf1de6b57f26545fd3ad03d99e18c9f9dc0a5c` has exact parents:

```text
PARENT_1 = 7ecba93ae0763a5165dd99ce0ec190cb106906de
PARENT_2 = ea82e1246095bb921dc9e7e40716076edbda41a6
```

The exact canonical merge then passed push-triggered post-merge CI and R3 listed above.

As with PR #37, live GitHub does not expose the historical merge API request body for PR #38. This record therefore preserves the expected-head transport proof as unavailable rather than reconstructing it from parentage.

## Exact B101 diff and scope reconciliation

PR #38 changed exactly two files:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault.rs
```

GitHub records:

```text
CHANGED_FILES = 2
ADDITIONS = 666
DELETIONS = 2
```

B101 remained inside the reviewed portable-contract boundary:

- exact 16-byte `VaultId` contract;
- exact 32-byte manifest-hash value;
- non-zero `KeyGeneration` and `FreshnessEpoch` values;
- provider-neutral `FreshnessAnchor` and explicit protected freshness state values;
- lock/lease identity and state values without revocation behavior;
- access-scope and user-presence policy values;
- provider-reported hardware-backing/capability values;
- typed portable protector errors;
- opaque-VRK provider-neutral `SecretProtector` operation signatures;
- Himsat-owned contract invariant tests.

B101 did not change `Cargo.toml`, `Cargo.lock`, provenance registry entries, generated SBOM/notices, workflows, native platform protector code, SQLCipher behavior, cryptographic operations, persistence, freshness compare-and-advance mechanics, lease revocation behavior, backup/restore/rotation/deletion, Specification 005 media behavior, or donor material.

## Review and comment reconciliation

Live GitHub records no submitted PR review and no inline review thread for PR #38.

Observed automated comments were not independent PASS evidence:

- Qodo reported review billing blocked;
- CodeRabbit reported automatic review skipped because of repository eligibility configuration.

No substantive blocking finding was present in the available PR review/thread/comment state. The absence of a substantive review on B101 does not satisfy the later exact implementation independent-review obligation Q009; that obligation remains open for the complete Specification 004 implementation revision.

## Forward-only repair for historical merge-transport evidence

The missing historical request-body evidence for PR #37 and PR #38 cannot be recreated honestly. The forward repair is:

1. keep the historical transport proof state explicit and negative/unknown;
2. canonicalize this B101 closeout/B102 re-bound reconciliation only through an explicit `expected_head_sha` merge call;
3. record the exact reconciliation head and merge result durably in GitHub;
4. verify exact canonical parentage;
5. require exact post-merge CI and R3 SUCCESS on that canonical merge;
6. only then activate B102 implementation authority.

This repair does not retroactively claim that the historical merges used a transport guard. It establishes a new proven guarded boundary before the next security-sensitive implementation leaf.

## Re-bounded B102 leaf

```text
B102 = REVOCABLE_KEYED_HANDLE_LEASE
RISK = R3
AUTHORITY = CONDITIONAL_ON_THIS_RECONCILIATION_CANONICAL_EXPECTED_HEAD_GUARDED_AND_POSTMERGE_QUALIFIED
```

### Scope in

B102 may implement only the portable behavior needed for:

- typed lock/unlock/protector/freshness behavior errors owned by the portable layer;
- a live in-process `VaultLease` bound to one `VaultId` and one `KeyGeneration`;
- active/revoked lease state;
- fail-closed authorization checks through the lease;
- idempotent revocation semantics;
- Himsat-owned tests for allocation, identity binding, active authorization, revocation propagation, repeated revocation, and post-revocation rejection.

### Scope out

B102 must not implement:

- entropy acquisition or random generation;
- VRK generation, wrapping, storage, unlocking, release, or zeroization behavior;
- HKDF, Argon2id, XChaCha20-Poly1305, nonce handling, or envelope serialization;
- SQLCipher/database behavior or concrete keyed DB/blob handles;
- Apple, Android, Windows, or Linux native protector mechanics;
- freshness persistence, protected genesis installation, or compare-and-advance behavior;
- B104 key hierarchy/domain-separation/secret-lifetime behavior;
- B105 concrete post-lock DB/blob I/O rejection proof;
- backup, restore, rotation, deletion, or Specification 005 media behavior;
- new dependencies or donor-code adoption.

## B102 activation gate

B102 remains blocked until all of the following are proven for this reconciliation:

1. exact reconciliation head CI SUCCESS;
2. exact reconciliation head R3 SUCCESS;
3. exact diff remains state/evidence only;
4. no unresolved blocking review/thread/comment exists;
5. canonical `main` has not moved incompatibly;
6. merge is executed with explicit `expected_head_sha` equal to the exact current reconciliation head;
7. canonical merge parentage is verified;
8. exact post-merge CI SUCCESS;
9. exact post-merge R3 SUCCESS.

## Preserved residual risks and later obligations

All reviewed Specification 004 residual risks remain unchanged. In particular, an unlocked compromised process can access resident plaintext/secrets; runtime zeroization cannot prove erasure of arbitrary copies, registers, swap, crash dumps, allocator copies, or kernel/hardware memory; and platform capability claims remain evidence-specific.

B101 closure does not close Q001-Q012 or C001-C005. Exact independent substantive crypto/security review of the final Specification 004 implementation revision remains mandatory before Specification 004 can become `CLOSED_CANONICAL`.
