# B505 Aggregate Reconciliation Evidence

## Authority

B505D governance amendment retired mandatory independent-human review and
resolved implementation authority to `B505_ONLY` after its own guarded merge
and post-merge qualification. B505E through B505W then implemented the bounded
Round 5 portable-backup packaging and restore composition under that authority.
B505W explicitly left aggregate reconciliation as the remaining B505 gate
before B505 can be marked complete. This document reconciles that aggregate.

Live GitHub and repository truth controls over stale state documents.
`specs/CURRENT.md` and `specs/004-vault-key-crypto/tasks.md` were materially
stale through B505W and are reconciled forward-only by the companion
state update in the same unit. No canonical history is rewritten.

## Predecessor reverification

```text
REMOTE_MAIN = 4fb5ba900760b84edce5989cedbcaebf26a06f07
B505W_PR = 152
B505W_ACCEPTED_HEAD = d5bd3a194fa984f495a63204cbc8a4494aa7b2d2
B505W_ACCEPTED_TREE = a3a0719ad1eb4914f8876bce1d9cf2312f237208
B505W_BASE = e1c4db9d965e1f66f0d457c0667f4246e9b4dbc3
B505W_CANONICAL_MERGE = 4fb5ba900760b84edce5989cedbcaebf26a06f07
B505W_MERGE_PARENT_1 = e1c4db9d965e1f66f0d457c0667f4246e9b4dbc3
B505W_MERGE_PARENT_2 = d5bd3a194fa984f495a63204cbc8a4494aa7b2d2
B505W_MERGE_TREE = a3a0719ad1eb4914f8876bce1d9cf2312f237208
B505W_POSTMERGE_CI = 34923636356 SUCCESS push attempt 1 exact merge SHA
B505W_POSTMERGE_R3 = 34923636591 SUCCESS push attempt 1 exact merge SHA
```

Parentage and tree were reverified locally with `git log --format=%H_%P_%T`.
Post-merge runs were listed live with `gh run list` and both are `completed`
with conclusion `success` on the exact canonical merge.

PR #152 is therefore not pending work and is not redone.

## Canonical B505 lineage reconciled here

Base of this lineage is the B505C reconciliation merge:

```text
BASE = ae614bf6b4c5231ddf228ab058ab4caf265e98d9
```

Each row below was verified as a canonical merge with two parents and a
recorded tree. Every post-merge CI/R3 pair listed was observed live as
`completed/success` on a `push` event for the exact merge commit.

```text
PR 133 governance retire-human-review
MERGE f848390a907f32ed41c176ce47b0d3157f0a3abd
PARENTS ae614bf6b4c5231ddf228ab058ab4caf265e98d9 fed1d72c09639731df297f86c5d06c910796c21b
TREE 78ddaa84ff486f4c9dd3f5daa1723278fbeddbc7
POSTMERGE_CI 34782291693 SUCCESS
POSTMERGE_R3 34782291681 SUCCESS

PR 134 B505E backup codec foundation
MERGE 81f50a2148f0e43e007d9dd50623fe98625df0ed
HEAD fcc4ee34961a5071bcb9370e2af9c8ba34d36aaf
POSTMERGE_CI 34804908872 SUCCESS
POSTMERGE_R3 34804908876 SUCCESS

PR 135 B505F backup index codec
MERGE 16afa236217f51930016d1d7c5cd8b05e45db258
HEAD fdcdb1ec3138c64e70b1e0760f07a6c343c57ef6
POSTMERGE_CI 34809416083 SUCCESS
POSTMERGE_R3 34809416090 SUCCESS

PR 136 B505G backup provider publication
MERGE 1c08e1898f0702a39a86a51eb434d44d6c6d1352
HEAD 0d7bb6a30a6fd436339cb0ee9fac234eeca44480
POSTMERGE_CI 34812509172 SUCCESS
POSTMERGE_R3 34812509134 SUCCESS

PR 137 B505H backup recovery bootstrap
MERGE 768168fe7e901e9ee5561a518952cda964ce8701
HEAD 78bbaf334ba5a14de14972274c29b544877975b2
POSTMERGE_CI 34815543503 SUCCESS
POSTMERGE_R3 34815543520 SUCCESS

PR 138 B505I backup semantic verification
MERGE c8d1fc74190355c88822fff4b733eca14ba1008a
HEAD 003fe7510c200fb7325901c2e3b69aaa825f87d4
POSTMERGE_CI 34820590199 SUCCESS
POSTMERGE_R3 34820590256 SUCCESS

PR 139 B505J backup SQLCipher verification
MERGE afa89aeb7986b7960ac16175b4b5d21679f4b9e9
HEAD d8596facbd4b69deb790d2139b958015e6c0d6ae
POSTMERGE_CI 34824314953 SUCCESS
POSTMERGE_R3 34824314790 SUCCESS

PR 140 B505K streaming backup publication
MERGE a077a27aed8c87529e87323f5fc9a7cdc663bc89
HEAD 1aff7c81e4b43e95c88cc47b2bc35d5828416c6e
POSTMERGE_CI 34828312510 SUCCESS
POSTMERGE_R3 34828312576 SUCCESS

PR 141 B505L backup packaging bounded memory
MERGE a24e22683dc9097a095ae45bccf666354b88fef5
HEAD bcff1ff65c42c93a3eec1348c5565754168d99c3
POSTMERGE_CI 34883309690 SUCCESS
POSTMERGE_R3 34883309672 SUCCESS

PR 142 B505M quiesced SQLCipher snapshot
MERGE ec149d7b9949d716b2ecc9574d8d2cf80814742a
HEAD f6de5aa7a91261576d62e3665aebc3b647438c14
POSTMERGE_CI 34888530206 SUCCESS
POSTMERGE_R3 34888530155 SUCCESS

PR 143 B505N WAL snapshot binding
MERGE 0969d13c25bf498b312325a26d915fbc5b4aa4f7
HEAD 9ca11836ce3558ffc02049755c37136ee1e26e9c
POSTMERGE_CI 34892505760 SUCCESS
POSTMERGE_R3 34892505871 SUCCESS

PR 144 B505O backup creation coordinator
MERGE 6b96739d47a1e44c4a8350e6fb66c9a174316a83
HEAD 5adf40a75d88dd4faeb9fb44e8fd76d48b201520
POSTMERGE_CI 34896723329 SUCCESS
POSTMERGE_R3 34896723395 SUCCESS

PR 145 B505P provider-view qualification
MERGE d44b28ef78276732400710218ec78d7325cd2a4b
HEAD afc2d7d57c0fe636f39c6927f48cf3060788fd33
POSTMERGE_CI 34901257378 SUCCESS
POSTMERGE_R3 34901257413 SUCCESS

PR 146 B505Q fresh-device restore preparation
MERGE 57dcfecd45022d68f7c4ab77b4d6c5b9c754240d
HEAD b61d4041f9367d8d0f2aabca8c97ce1215deb7b0
POSTMERGE_CI 34905074368 SUCCESS
POSTMERGE_R3 34905074384 SUCCESS

PR 147 B505R fresh-device genesis gate
MERGE b9f2a303fa3b39c4c10e82edbe373b2f005ad675
HEAD cec4be18f94af9a810dfdf9bb367241bd86c8eb0
POSTMERGE_CI 34908173915 SUCCESS
POSTMERGE_R3 34908173953 SUCCESS

PR 148 B505S fresh-device publication
MERGE bb45248844fe8e1c7361b2cad17ba9040e2370fc
HEAD 989f2efa98eb17aee9bd6a842987f9e72442bd18
POSTMERGE_CI 34910977086 SUCCESS
POSTMERGE_R3 34910977111 SUCCESS

PR 149 B505T existing-device restore gate
MERGE df28a3b5a758ef90568a547582411eaab9bbbb53
HEAD 85c3a85c2c4c52cc0a55fb0d0e79c1d55abc4219
POSTMERGE_CI 34914178330 SUCCESS
POSTMERGE_R3 34914178237 SUCCESS

PR 150 B505U existing-device target staging
MERGE 0eb7267d0d7e18ec3f35674bc476cb9a0b7429b0
HEAD 7f18904a9014fca391aa76b4b6545e31774ee5cc
POSTMERGE_CI 34917582333 SUCCESS
POSTMERGE_R3 34917582496 SUCCESS

PR 151 B505V existing-device restore preparation
MERGE e1c4db9d965e1f66f0d457c0667f4246e9b4dbc3
HEAD b2779811e9ca865a477a7c54cd8ea75566bace31
POSTMERGE_CI 34920498813 SUCCESS
POSTMERGE_R3 34920498773 SUCCESS

PR 152 B505W existing-device restore publication
MERGE 4fb5ba900760b84edce5989cedbcaebf26a06f07
HEAD d5bd3a194fa984f495a63204cbc8a4494aa7b2d2
POSTMERGE_CI 34923636356 SUCCESS
POSTMERGE_R3 34923636591 SUCCESS
```

Cancelled superseded pre-merge attempts on PRs 146 and 149 remain recorded
as `cancelled`, not `success`, and were superseded by successful
original-head attempts. No cancelled or skipped run is claimed as PASS.

## Round 5 requirement closure

The controlling contract is
`round5-portable-backup-provider-privacy-contract.md` together with the B505C
cross-generation rebase amendment. Coverage below names the implementing
module and representative tests on canonical `main`.

Provider privacy and allowlist: outer set descriptor, object envelope, and
encrypted index expose only the v1 allowlist fields. Direct B202, B204, B501,
and raw SQLCipher upload paths are rejected. Implemented in `vault_backup`,
`vault_backup_index`, `vault_backup_provider`, and `vault_backup_verify`.
Tests: `provider_keys_are_canonical_and_strict`,
`descriptor_rejects_trailing_bytes_and_provider_mismatch`,
`provider_view_qualifier_rejects_semantic_keys_and_raw_inner_uploads`,
`authenticated_index_storage_mismatch_is_rejected`,
`provider_object_tamper_fails_outer_authentication`.

Opaque naming and key semantics: fresh 16-byte OS-CSPRNG `BackupSetId` and
non-zero `BackupObjectId` values, lowercase-hex provider keys, reserved
all-zero descriptor leaf, strict embedded-identifier binding, duplicate and
transplant rejection. Implemented in `vault_backup` and
`vault_backup_provider`. Tests: `descriptor_round_trip_and_provider_binding`,
`reserved_zero_identity_is_rejected_and_generation_is_bounded`,
`preparation_rejects_count_duplicates_and_cross_set_envelopes`.

Set scoping and backup keys: distinct backup-set-specific HKDF domains for
index, object, and bootstrap keys; generation binding; cross-set transplant
failure. Implemented in `vault_backup` and `vault_backup_bootstrap`. Tests:
`backup_key_domains_sets_and_generations_are_separated`,
`encrypted_index_authenticates_context_bootstrap_and_ciphertext`,
`encryption_rejects_vault_generation_and_count_transplants`,
`cross_set_transplant_fails_outer_authentication`.

Storage-ID privacy and bounded capacity: source storage IDs remain inside the
encrypted index; provider keys carry no logical IDs; payload, chunk, object,
and 64 MiB index-plaintext bounds are checked before allocation. Implemented
in `vault_backup_index` and `vault_backup_packaging`. Tests:
`index_plaintext_round_trip_is_canonical`,
`parser_rejects_bad_ordinal_kind_zero_object_trailing_and_count`,
`model_rejects_duplicate_objects_unsorted_blobs_and_length_mismatch`.

Portable construction, active-VRK proof, and recovery wrapping: creation
requires stable single-generation state, an authenticated B204 envelope for
the active generation, passphrase-derived Recovery KEK equality with the
unlocked active VRK, and no silent escrow. Implemented in
`vault_backup_bootstrap` and `vault_backup_creation`. Tests:
`active_vrk_mismatch_fails_before_bootstrap_publication`,
`bootstrap_round_trip_preserves_canonical_recovery_identity`,
`wrong_passphrase_and_outer_tamper_are_uniform_authentication_failures`,
`authenticated_rotation_state_is_not_export_stable`.

SQLCipher snapshot, WAL, checkpoint, and generic artifacts: quiesced
checkpoint, staged-copy provider and integrity verification, snapshot binding
to the pre-checkpoint manifest identity, Round-4 B202 verification, manifest
hash binding before SQLCipher acceptance. Implemented in
`vault_backup_sqlcipher`, `vault_backup_packaging`, and
`vault_backup_verify`. Tests:
`exact_staged_sqlcipher_identity_and_integrity_pass`,
`staged_byte_identity_mismatch_fails_before_sqlcipher_acceptance`,
`wrong_vrk_cannot_upgrade_matching_staged_bytes`,
`wal_checkpoint_snapshot_is_bound_to_precheckpoint_manifest_identity`,
`post_checkpoint_structured_snapshot_still_requires_exact_storage_binding`,
`reconstructs_manifest_and_b202_before_sqlcipher`,
`blob_authentication_is_required_even_when_manifest_hash_matches`.

Publication ordering, descriptor-last, and reread verification:
immutable set namespace, no in-place overwrite, descriptor published last,
complete provider reread and reconstruction before local acceptance, new
`BackupSetId` and nonces on retry, ambiguous acknowledgements never accepted.
Implemented in `vault_backup_provider` and `vault_backup_creation`. Tests:
`publication_is_descriptor_last_and_reread_before_acceptance`,
`streaming_publication_is_descriptor_last_and_three_pass_verified`,
`streaming_publication_preflights_all_objects_before_provider_write`,
`streaming_publication_detects_cardinality_drift_before_descriptor`,
`streaming_publication_detects_source_change_before_descriptor`,
`ambiguous_descriptor_ack_is_never_accepted`,
`existing_key_prevents_in_place_overwrite`,
`failed_object_write_never_publishes_descriptor`,
`reread_mismatch_is_fail_closed`,
`wal_backed_creation_is_published_reread_and_sqlcipher_verified`.

Fresh-device restore: authenticated bootstrap and index path, staged
verification, explicit protected `UNINITIALIZED` genesis, no global-newest
claim, no VRK mutation on wrong present state. Implemented in
`vault_backup_restore` and `vault_backup_restore_publication`. Tests:
`authenticated_backup_yields_verified_fresh_device_material`,
`existing_uninitialized_matching_vrk_prepares_genesis_without_replacement`,
`existing_uninitialized_mismatched_vrk_fails_without_replacement`,
`present_anchor_rejects_fresh_device_genesis_without_vrk_mutation`,
`prepared_backup_is_published_anchored_and_reopened_in_b307_order`.

Existing-device restore, same and cross generation: stable current-state
gate, `VaultId` equality, strictly older source epoch, rejection of future
generation, same-generation VRK byte equality, fresh non-colliding target
IDs, same-generation ciphertext preservation with reservation reconciliation,
cross-generation re-encryption under current `H`, synthetic B501 staging
manifest under `H`, canonical B502 final transition, B307 copy-verify-publish
ordering, source and provider immutability. Implemented in
`vault_backup_restore` and `vault_backup_restore_publication`. Tests:
`existing_device_restore_gate_accepts_stable_same_generation_identity`,
`existing_device_restore_gate_accepts_stable_retained_history`,
`existing_device_restore_gate_rejects_generation_ahead`,
`existing_device_restore_gate_rejects_same_generation_key_mismatch`,
`existing_device_restore_gate_routes_older_generation_to_rebase`,
`existing_device_target_staging_same_generation_preserves_ciphertext`,
`existing_device_target_staging_rejects_conflicting_same_generation_nonce`,
`existing_device_target_staging_cross_generation_reencrypts_under_current_key`,
`existing_device_target_staging_fails_closed_when_no_target_id_is_available`,
`existing_device_restore_preparation_same_generation_builds_b502_candidate`,
`existing_device_restore_preparation_cross_generation_rebases_manifest_to_current`,
`existing_device_restore_preparation_requires_current_retained_manifest`,
`existing_device_publication_cross_generation_commits_exact_b502_anchor_and_reopens`.

B503 and B502 composition: rebase uses current VRK `H` without invoking the
B503 rotation state machine or creating a generation; B502 remains sole owner
of the `anchor + 1` transition with exact `expected_old_anchor`. Covered by
the preparation and publication tests above and the B502 restore-state path.

B501 interrupted-publication recovery, nonce history, retained manifests,
retries, faults, and ambiguous anchor writes: stable retained-history gate,
staging reservation seeded from retained canonical history, source-manifest
reservation exclusion for same-generation staging, B502 final-manifest nonce
exclusion, before-publication authority of the old anchor, exact
`RecoverInterruptedPublication` handling, no second advancement after an
ambiguous write, post-anchor reopen verification, retry with fresh target IDs
and nonces. Tests:
`existing_device_publication_anchor_rejection_leaves_recoverable_candidate`,
`existing_device_publication_ambiguous_anchor_error_never_rolls_back`,
`post_anchor_reopen_failure_preserves_installed_anchor_for_roll_forward`,
`staging_mutation_after_b505q_fails_before_local_copy_or_anchor`,
`protected_state_drift_is_rejected_without_overwrite`.

Source and provider immutability: throughout staging and rebase, source
backup bytes, current canonical state, and detached provider bytes remain
unmodified; publication backend exposes no delete, retire, or provider-mutation
operation. Tests: `staging_mutation_after_b505q_fails_before_local_copy_or_anchor`,
`wrong_expected_vault_is_rejected_after_complete_backup_verification`,
`provider_authentication_failure_removes_unaccepted_staging`.

## Residual risks and non-claims preserved

B505 preserves the contract limitations: ciphertext sizes, counts, operation
and backup timing, provider account and transport metadata, and inferable
rotation cadence remain observable. The implementation makes no physical-media
erasure claim, no remote revocation or provider-snapshot deletion claim, no
globally newest backup discovery claim, no hidden size/count/timing claim,
and no B506 local-deletion claim. Fresh-device genesis retains the explicit
inability-to-prove-global-newestness rule.

## Workspace qualification on the reconciled base

On exact canonical `main` `4fb5ba9`, before this docs-only unit:

```text
cargo fmt --all -- --check: PASS
git diff --check: PASS
python3 tools/provenance_gate.py validate: PROVENANCE V2 PASS
python3 tools/provenance_gate.py check-generated: GENERATED OUTPUTS PASS
python3 tools/004p_dependency_closure.py: 004P P005/P006 CLOSURE PASS
cargo +1.98.1 clippy --workspace --all-targets --locked -- -D warnings: PASS
cargo +1.98.1 test --workspace --all-targets --locked: 239 passed 0 failed
```

The full suite total is 215 lib tests plus 24 integration and event tests.
No test was disabled or weakened to obtain this result.

## Review posture

Open review-only PRs 12, 18, 24, 25, and 27 remain historical evidence and
are not merged by this unit. Automated review output, including Alibaba
OpenCodeReview where available, is advisory under current governance and does
not substitute for exact-head CI/R3, parentage and tree verification, or
post-merge qualification. Any blocking technical finding must still be
resolved; absent, skipped, billing-blocked, neutral, cancelled, or otherwise
nonqualifying automation is never recorded as PASS.

## Disposition

B505 implementation is complete through B505W. After this reconciliation unit
completes its own exact-head qualification, guarded expected-head merge,
parentage and tree verification, and exact-SHA post-merge CI/R3, canonical
state may record `B505_DISPOSITION = CANONICAL_CLOSED` and open bounded B506
active-vault deletion authority. This unit itself implements no B506 deletion,
no product behavior change, no dependency change, and no Specification 005
work.
