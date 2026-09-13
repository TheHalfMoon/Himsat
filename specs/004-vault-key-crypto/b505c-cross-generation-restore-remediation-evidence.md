# B505C Cross-Generation Existing-Device Restore Remediation Evidence

## Status

This record captures a forward-only Round 5 security-semantic remediation discovered after B505B became canonical at `840e45ee24b59cab4d8f5d244195b035590194dd` and before B505 implementation authority opened.

```text
B505B_CANONICAL_SHA = 840e45ee24b59cab4d8f5d244195b035590194dd
B505B_ACCEPTED_HEAD = dda3f8199c20808c3f7ad741276ec23b6fe97e8e
B505B_ACCEPTED_TREE = 9f96acce324f86d1b5f615a683e882c9b9ad9877
B505B_PREMERGE_CI = 34745347276_SUCCESS_ATTEMPT_1
B505B_PREMERGE_R3 = 34745347210_SUCCESS_ATTEMPT_1
B505B_POSTMERGE_CI = 34745974030_SUCCESS_ATTEMPT_1
B505B_POSTMERGE_R3 = 34745974034_SUCCESS_ATTEMPT_1
SUPERSEDED_REVIEW_ONLY_PR = 127
ADVISORY_REVIEW_COMMENT = 5652106027
ADVISORY_REVIEWED_SHA = 840e45ee24b59cab4d8f5d244195b035590194dd
ADVISORY_DISPOSITION = CHANGES_REQUIRED
ADVISORY_BLOCKER = B505-R5-001
B505_IMPLEMENTATION_AUTHORITY = NONE
```

The first automatic post-merge CI/R3 pair (`34745934498` / `34745934479`) was cancelled by the workflows' `cancel-in-progress: true` concurrency rule when a second independent `push`, attempt-1 pair for the same exact canonical SHA started. Those cancellations remain preserved and are not represented as PASS.
## Finding B505-R5-001 -- existing-device restore did not compose across VRK generations

Advisory exact-SHA CodeRabbit review on review-only PR #127 comment `5652106027` identified a valid blocker against `840e45ee24b59cab4d8f5d244195b035590194dd` and returned `CHANGES_REQUIRED` with `BLOCKING_FINDINGS = B505-R5-001`. This automation result is useful defect evidence but is not the required genuinely independent human review. PR #127 was closed as `SUPERSEDED / CHANGES_REQUIRED` before any qualifying human review and MUST NOT reopen B505 authority.

Round 5 required a fully verified recovered backup to enter B502 restore-state logic and also required an intentional older backup on an existing device to become a new epoch above the trusted anchor. Canonical B502 intentionally accepts that path only when both the authenticated B501 envelope generation and manifest `active_key_generation` equal the device's current active generation.

Therefore a normal sequence was undefined:

```text
portable backup source generation = G
existing device later active generation = H
G != H
backup epoch < trusted current anchor epoch
```

The portable backup can correctly recover VRK G and authenticate its stable B501/B202/SQLCipher state, but B502 correctly returns `KeyGenerationMismatch` because it must not pretend that old-generation ciphertext becomes current merely by advancing freshness.

Canonical B502 evidence explicitly leaves this case at the B503 integration boundary. Canonical B503F proves copy/re-encryption of SQLCipher and bounded blobs across distinct generations while retaining source state until target verification.
## Remediation

B505C defines an existing-device **restore rebase** before B502. The detached backup remains read-only evidence; the current active generation and protector remain authoritative.

For a current active generation `H`:

- authenticate the complete portable backup under recovered source generation `G` before local mutation;
- quiesce current writes and authenticate the exact current anchor/current manifest under the existing active VRK H;
- require both source and current manifests to be stable (`rotation_phase == NONE`) and require the source epoch to be strictly less than the current anchor epoch;
- stage restored content under fresh, non-colliding local storage IDs;
- when `G == H`, require the recovered backup state also authenticates under the current H VRK; otherwise fail `KeyGenerationIdentityMismatch`; verified ciphertext may then be copied to fresh storage locations without creating a second VRK identity for H;
- when `G != H`, decrypt/authenticate source content under G and re-encrypt/export it under the already-current H VRK, with fresh B203 blob nonces and full target verification; this is restore migration, not a root-key rotation and does not change H;
- never overwrite the current canonical object set in place.
The rebased inventory is represented by an in-memory authenticated staging manifest under H. It keeps the source backup epoch and source previous-manifest hash, uses the current stable generation table, references only the fresh target storage IDs, and is never published or anchored as canonical state.

Its manifest nonce is generated from a separate ephemeral B203 ledger seeded from the same retained authenticated canonical manifest reservations as the real restore ledger. The staging reservation is not inserted into canonical history. B502 receives the in-memory staging envelope as its authenticated older-backup source and explicitly excludes that exact staging reservation while generating the final canonical fresh nonce.

B502 then remains the sole owner of existing-anchor freshness republication:

```text
final_epoch = trusted_anchor.highest_epoch + 1
final_previous_manifest_hash = trusted_anchor.manifest_hash
final_active_generation = H
```

The storage coordinator durably writes the verified rebased objects and B502 final manifest, rereads/authenticates the complete target set, then invokes the existing protected compare-and-advance operation with B502's exact `expected_old_anchor` and `new_anchor`.

## Canonical composition proof

The remediation is constrained by already-canonical implementation behavior rather than by a new freshness algorithm.

`crates/himsat-core/src/vault_restore.rs::prepare_older_backup_restore` authenticates the supplied B501 envelope and intentionally requires both the envelope generation and authenticated manifest `active_key_generation` to equal the caller-provided current generation. It then requires the source epoch to be strictly older than the protected anchor, computes exactly `anchor + 1`, republishes against the old anchor hash, generates a fresh manifest nonce while excluding the source-envelope reservation, rereads/authenticates the result, and returns exact `expected_old_anchor` / `new_anchor` values without mutating the protected anchor itself. B505C preserves those semantics and transforms only the non-canonical restore input so B502 receives a verified source already expressed under current generation `H`.

Canonical B503 rotation is strictly monotonic: `FullRotationIdentity::for_next_generation` chooses `source_generation + 1`, and stable RETIRE state is `rotation_phase == NONE` with the target generation active and every canonical object under that target generation. Therefore an authenticated older-epoch portable backup claiming `G > H` cannot be established as an ancestor of the current protected state under the canonical single-lineage rotation rules. B505C rejects that divergent/unproven case as `RestoreGenerationAhead` instead of silently rewriting it as history.

Canonical B503F already proves real SQLCipher and bounded-blob copy/re-encryption across distinct generations, target authentication under the target VRK, and source retention through target verification. Canonical B307 separately proves source-retaining copy -> verify -> publish ordering. B505C reuses those qualified mechanics under the already-current VRK `H`; it does not invoke the B503 rotation state machine, mint a new generation, replace the current protector, or weaken B502 freshness checks.

The current stable generation table is preserved as authenticated current state. B505C does not assume that only one generation-table record exists; it requires one active `H`, no active rotation, and every current/source inventory object to use its respective active generation. The synthetic staging manifest preserves the full current stable generation table while rebinding restored inventory to `H`.

## Same-generation identity rule

Generation numbers are not sufficient key identity. If the portable backup says `G == H`, successful recovery MUST yield VRK bytes equal to the already-unlocked current VRK. A mismatch is `KeyGenerationIdentityMismatch` and aborts before staging. This prevents a divergent vault lineage from presenting different secret material under the same `(VaultId, generation)` label.

When equality is proven, already-authenticated B202 envelopes may be copied byte-for-byte to fresh target storage IDs under B203's copy/restore nonce-preservation rule. Before acceptance, each copied `(VaultId, BoundedBlob, H, nonce)` reservation is reconciled against authenticated current/retained `H` reservations. If absent, it is admitted as the source encryption instance. If already present, the source logical identity, canonical envelope length/hash, and exact authenticated envelope bytes must prove that both references are the same encryption instance; otherwise restore fails `NonceReservationConflict`. This distinguishes safe physical copying of one authenticated ciphertext from dangerous acceptance of two different ciphertexts under one nonce reservation. Cross-generation `G < H` always performs fresh encryption under `H` with fresh B203 blob nonces.

## Staging-manifest and nonce-ledger composition

The synthetic staging manifest is authenticated B501-format evidence, not canonical state. It exists only to satisfy B502's intentional same-generation input boundary after content has been verified/rebased under `H`. It preserves the authenticated source epoch but uses current generation semantics and verified target inventory. It is never published and never written into the protected anchor.

Its manifest nonce is drawn from a separate ephemeral ledger seeded with every retained authenticated canonical manifest reservation. For `G == H`, staging nonce generation also excludes the exact authenticated portable source-manifest reservation, even when that reservation is not present in retained local history; `reserve_fresh_manifest_nonce_avoiding` already provides the required candidate-space exclusion without falsely persisting the detached source as canonical history. For `G < H`, the portable source manifest is under a different generation key and therefore a different B203 uniqueness domain. The real B502 ledger is independently seeded from the same canonical history and does not persist the staging reservation. The exact staging reservation is passed to B502 as the source-envelope reservation, and B502's existing `encrypt_fresh_manifest_avoiding_reservation` behavior prevents the final canonical manifest from reusing it. This preserves nonce uniqueness across the source -> staging -> final chain without falsely adding a non-canonical envelope to canonical retained history.

## Final local design challenge

Before commit, B505C was challenged directly against canonical B203/B501/B502/B503 behavior. That challenge found one ambiguity in the first local draft: same-generation `G == H` restore did not explicitly prevent the synthetic staging manifest from reusing the detached source manifest nonce when that older manifest reservation was absent from retained local history, and it did not explicitly distinguish safe physical copying of one B202 encryption instance from a conflicting same-reservation ciphertext.

The final candidate repairs both points without changing product code: same-generation staging-manifest generation must exclude the exact authenticated source-manifest reservation in addition to retained `H` history, and copied B202 reservations must reconcile against current/retained `H` reservations with exact logical/envelope identity required for any duplicate reservation. This is a pre-commit design repair, not post-merge evidence, and therefore invalidates earlier local qualification bytes; all required exact-byte gates must be rerun on this final candidate.

## Pre-commit local qualification lineage

After the final local design challenge repaired the same-generation nonce ambiguity, the candidate was qualified again before this evidence/task-state update. Observed results were:

- `git diff --check` -> PASS;
- explicit changed-file control-byte scan (rejecting bytes `< 0x20` except tab/LF/CR and rejecting `0x7f`) -> PASS across all six changed/untracked candidate files;
- `python tools/provenance_gate.py validate` -> `PROVENANCE V2 PASS`;
- `python tools/provenance_gate.py check-generated` -> `GENERATED OUTPUTS PASS`;
- the first `python tools/004p_dependency_closure.py` invocation failed before repository closure evaluation because required build environment `OPENSSL_RUST_USE_NASM=0` was absent; after applying the exact build-environment contract encoded by the script and leaving all forbidden provider redirects unset, the gate returned `004P P005/P006 CLOSURE PASS`;
- `cargo fmt --all -- --check` -> PASS;
- local Rust toolchain identity matched CI: `rustc 1.98.1`, `cargo 1.98.1`, `clippy 0.1.98`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings` -> PASS;
- `cargo test --workspace --all-targets --locked` -> PASS with no failed tests; the main `himsat_core` unit suite reported `131 passed`, followed by all integration/example/event suites with zero failures;
- the first generic WSL invocation did not execute the Linux self-test because it selected the wrong default distribution and returned `bash: Permission denied`; explicit `wsl.exe -d Ubuntu -- ... python3 tools/provenance_gate.py self-test` then executed the intended Linux gate and returned `SELF-TEST PASS`;
- the previously observed Windows provenance composite self-test fixture failure remains preserved as **NOT PASS** and is not rewritten as success or attributed to B505C without evidence.

Because this evidence/task-state update itself changes repository bytes, the results above do not by themselves qualify the bytes containing this section. The same required exact-byte gates must be rerun after this update and before commit; that final disposition is recorded in commit/PR evidence rather than by another self-referential evidence-file edit.
## Required B505 implementation qualification for this remediation

Future B505 implementation evidence MUST include, at minimum:

- same-generation older restore where recovered VRK equals current VRK and exact B202 envelopes are copied to fresh storage IDs;
- same-generation generation-number match with different VRK bytes -> `KeyGenerationIdentityMismatch`, no local canonical mutation;
- completed-rotation restore with `G < H`: SQLCipher and every B202 object authenticate under `G`, are copied/re-encrypted under `H`, reread/verify under `H`, and source/provider bytes remain unchanged;
- source `G > H` -> `RestoreGenerationAhead`, no staging/publication/anchor mutation;
- source/current `VaultId` mismatch -> fail closed;
- source epoch equal to or newer than current anchor -> existing B502 older-backup rule remains fail closed;
- current active rotation, freshness mismatch, interrupted publication, or current inventory outside `H` -> `RestoreStateNotStable`;
- target storage-ID collision with current/source/attempt target identity -> reject and regenerate/fail before publication;
- target SQLCipher/B202 authentication failure under `H` -> no final publication;
- same-generation copied B202 reservation collision against current/retained `H` history -> allow only the exact same authenticated logical object/envelope instance; otherwise `NonceReservationConflict`;
- same-generation source-manifest nonce injected as the first staging candidate -> discard/regenerate without persisting the detached source as canonical history;
- staging-manifest nonce collision injection against retained `H` history -> regenerate/fail; final B502 nonce MUST differ from staging reservation;
- crash before final publication -> old anchor/current manifest authoritative;
- crash after final publication before anchor advancement -> only the canonical B501 interrupted-publication rule may advance after complete target verification;
- crash after anchor advancement -> exact anchored rebased set must reopen/verify before writes resume;
- retry after an unaccepted attempt uses fresh target IDs and fresh encryption nonces where re-encryption occurs;
- pre-restore current object set is retained through successful anchor/reopen verification and B505 makes no B506/physical-erasure claim.

## Crash and retention disposition

The current canonical state and trusted anchor remain untouched through source authentication, rebase staging, target verification, and final-manifest verification.

- crash/failure before compare-and-advance: the old anchor remains authoritative; staged target objects/final candidate are unaccepted and MUST NOT replace current state;
- compare-and-advance retains the existing old-or-new protected-state semantics;
- if the new anchor is observed after restart, the exact anchored manifest and complete rebased object set must reopen and verify under H before ordinary writes resume;
- current pre-restore canonical data is retained through successful anchor/reopen verification; B505 does not claim physical erasure or B506 deletion authority;
- provider backup bytes are never mutated by local restore rebase;
- retry after an unaccepted attempt uses fresh target storage IDs and fresh encryption nonces where re-encryption occurs.

Current-vault rotation in progress is not composed with restore. If the authenticated current manifest is not stable or any current inventory object is outside active generation H, existing-device restore fails `RestoreStateNotStable` until canonical B503 recovery reaches a stable state.