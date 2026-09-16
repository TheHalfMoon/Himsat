# B506 Active-Vault Deletion Final Evidence

## Scope

B506 closes only the bounded active-vault deletion coordinator (`vault_deletion.rs`) and its adversarial tests. It does not implement concrete durable backends (filesystem/SQLCipher/protector removals remain backend-owned behind the new trait boundary), detached/provider-side deletion, physical secure erase, Specification 005, release/FIPS/compliance posture, or independent human security review.

The accepted implementation changes only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_deletion.rs
```

Delivery was split into two heads to respect the 900-line R2 size bound: B506a coordinator core (PR #155) and B506b adversarial hardening matrix (PR #156).

## Accepted implementation revisions

```text
B506A_PR = 155
B506A_BASE = 2cfc6871765a517f802aa5415cc311ea8d0bff04
B506A_HEAD = 9d3b95d5c19633bcad55e2dbb9f077a7655d83e9
B506A_TREE = 606f90906ede6d9317fa97cad837def9b458dd00
B506A_ADDED_LINES = 822

B506B_PR = 156
B506B_BASE = 0d5579a2fd4919c4bdfad101127ea801c7fa80eb
B506B_HEAD = e5d1e21f26b22a7d876591317d3556070b7a0416
B506B_TREE = cc35c62a73b933c793883aeddc8e8c02c760498d
B506B_ADDED_LINES = 218
```

The coordinator exposes ordered boundaries:

```text
QUIESCE -> WRAPPED_KEY_MATERIAL -> FRESHNESS_STATE -> PROTECTOR_REFERENCES
-> CANONICAL_STATE -> DERIVED_STATE -> MANIFEST_STATE -> STRUCTURED_STORE
-> BLOB_STORE -> BACKUP_STAGING -> MIGRATION_REMNANTS -> RESTORE_REMNANTS
-> PUBLICATION_TEMP -> TEMPORARY_STATE -> VERIFICATION
```

Crypto-shredding runs first: wrapped-key material is removed before every ciphertext-bearing surface, and freshness records are removed before protector references so freshness enumeration stays available through its own stage. The terminal lease is enforced as revocation evidence (no new keyed operation authorizable, in-flight operations drained), not as teardown proof; session teardown through `VaultSessionLifetime::teardown` remains a documented caller obligation. Verification requires per-surface native enumeration and rejects hidden residue on any of the 13 surfaces.

## Preserved negative evidence

```text
HEAD = cf3f2736bd4596f4d02c7674aae1dcf7cad8f982
CI = 35075778800 / FAILURE / pull_request
R3 = 35075778776 / FAILURE / pull_request
CAUSE = 1040 added lines exceed the 900-line R2/R3 exact-diff size bound
DISPOSITION = NOT_PASS
```

The oversized head was repaired forward-only by relocating the hardening matrix to the B506b successor; no production line changed in the repair. No security finding was raised against the code content of the oversized head. It was not rerun, rebased, rewritten, force-pushed, or retroactively upgraded.

## Qualification behavior

The 12 committed tests prove:

- every surface is removed in the exact coordinator order with crypto-shredding first (key material before structured store and blob store);
- a live lease refuses deletion before any durable surface is touched;
- a mismatched vault identity refuses deletion before any durable surface is touched;
- lock-path and revocation-path terminal leases both authorize deletion at the gate (documented as intentional; closure/release/discard stay caller obligations);
- deletion is idempotent across retries;
- an interrupted deletion leaves a keyless-but-incomplete mid-state and a retry converges;
- injected failure at every stage (quiesce, all 13 removals, verification) reports its exact stage, stops forward progress, and leaves exactly the expected residual surfaces;
- hidden residue on each of the 13 surfaces fails verification after all removals ran;
- reopening after deletion fails while the detached backup still opens (detached copies survive by design);
- the limitations notice states detached-backup survival, provider-snapshot survival, and the absence of any physical-erasure guarantee, and states no overclaim;
- session teardown closes handles, releases keys, discards caches, and leaves the lease terminal before deletion;
- backend failures preserve their typed source and the `B506` display tag names the failing stage.

## Exact-head pre-merge qualification

```text
B506A_PREMERGE_CI = 35077848624 / run #384 / SUCCESS / attempt 1 / pull_request
B506A_PREMERGE_R3 = 35077848641 / run #361 / SUCCESS / attempt 1 / pull_request
B506B_PREMERGE_CI = 35081734604 / run #386 / SUCCESS / attempt 1 / pull_request
B506B_PREMERGE_R3 = 35081734598 / run #363 / SUCCESS / attempt 1 / pull_request
INLINE_REVIEW_THREADS = 0
QODO = BILLING_BLOCKED / NOT_PASS
CODERABBIT = AUTO_SKIPPED / NOT_PASS
```

Adversarial review was performed through `ocr delegate` rules for `**/*.rs` plus independent subagent review: one blocking docs-overclaim and six non-blocking findings were raised, all fixed and re-verified (9/9 FIXED, no new confident defects). LLM-backed `ocr review` has no configured endpoint in this environment; no LLM OCR result is claimed. Owner governance amendment: independent human review is not a mandatory gate; historical review evidence is preserved without reclassification.

## Guarded merges and canonical parentage

Both merges used explicit `expected_head_sha` protection with `merge_method = merge`:

```text
B506A_EXPECTED_HEAD_SHA = 9d3b95d5c19633bcad55e2dbb9f077a7655d83e9
B506A_CANONICAL_MERGE = 0d5579a2fd4919c4bdfad101127ea801c7fa80eb
B506A_PARENT_1 = 2cfc6871765a517f802aa5415cc311ea8d0bff04
B506A_PARENT_2 = 9d3b95d5c19633bcad55e2dbb9f077a7655d83e9
B506A_MERGE_TREE = 606f90906ede6d9317fa97cad837def9b458dd00_EQUALS_ACCEPTED_TREE

B506B_EXPECTED_HEAD_SHA = e5d1e21f26b22a7d876591317d3556070b7a0416
B506B_CANONICAL_MERGE = a3622b817cced082391b93f1abcd932b00ac56cf
B506B_PARENT_1 = 0d5579a2fd4919c4bdfad101127ea801c7fa80eb
B506B_PARENT_2 = e5d1e21f26b22a7d876591317d3556070b7a0416
B506B_MERGE_TREE = cc35c62a73b933c793883aeddc8e8c02c760498d_EQUALS_ACCEPTED_TREE
```

## Post-merge qualification

```text
B506A_POSTMERGE_CI = 35079691161 / run #385 / SUCCESS / attempt 1 / push
B506A_POSTMERGE_R3 = 35079691332 / run #362 / SUCCESS / attempt 1 / push
B506B_POSTMERGE_CI = 35083815866 / run #387 / SUCCESS / attempt 1 / push
B506B_POSTMERGE_R3 = 35083815925 / run #364 / SUCCESS / attempt 1 / push
```

Both canonical merges are therefore exact-post-merge qualified for B506 only.

## B505Y post-merge qualification record

The B505 canonical-closure merge `2cfc6871765a517f802aa5415cc311ea8d0bff04` (PR #154, parents `8a6d8b4593df8b78177a4ad2c516c4a336736894` + `829dd040538b6012e2daeab01a99c87077e5b7c6`) completed post-merge qualification after its merge:

```text
B505Y_POSTMERGE_CI = 35072933755 / run #382 / SUCCESS / attempt 1 / push
B505Y_POSTMERGE_R3 = 35072933853 / run #359 / SUCCESS / attempt 1 / push
```

## Preserved limitations

`P011` remains unchecked / NOT PASS. `B305R002` remains unchecked / NOT PASS. Neither historical transport gap is repaired by B506.

B506 claims no concrete durable backend, no detached/provider-side deletion, no physical-media erasure, and no runtime zeroization beyond the reviewed `VaultSessionLifetime` teardown contract. Q009 is retired as a mandatory gate by the forward-only owner governance amendment; historical review evidence remains preserved and every known blocking finding was explicitly resolved.

## B506 disposition and closeout rebound

`B506_DISPOSITION = CANONICAL_CLOSED` for the bounded coordinator-plus-hardening boundary only.

Specification 004 closeout may begin only after this reconciliation itself becomes exact-head qualified, reconciled against live `main`/PR head/base/diff/reviews/threads/comments/mergeability, merged with explicit `expected_head_sha` protection, parentage-proven, and exact push-triggered post-merge CI/R3 qualified. After that qualification, the Specification 004 closeout unit (Q001-Q012, C001-C005) is the only authorized leaf. Specification 005 shaping, product features, donor adoption, and release activity remain out of scope until closeout completes.
