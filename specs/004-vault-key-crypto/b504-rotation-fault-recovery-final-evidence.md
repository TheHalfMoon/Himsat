# B504 Rotation Commit-Point Fault-Injection Final Evidence

## Scope

B504 qualifies only failure injection immediately before and immediately after the durable mutation boundaries exercised by the canonical B503 seven-phase full-VRK rotation coordinator. The accepted implementation is test-only and changes no production rotation, protector, crypto, dependency, workflow, provenance, B505 backup, B506 deletion, Specification 005, release, FIPS, or compliance semantics.

## Canonical authority

B504 authority was opened only after canonical B503/B504 reconciliation merge `96ea50a69ab0b29fdd0869bb4390df12f53ec5e9` passed exact post-merge qualification. B504 implementation PR #121 therefore starts from that exact canonical base.

## Accepted candidate and forward-only review lineage

The first B504 candidate was `ae23c1e9860843b3e1f52b0fc59a7eaab2a0ca59`. Review correctly identified that the recovery-enabled scenario did not re-authenticate the retained target recovery envelope after stable completion and an additional restart. That head is preserved as superseded and is not the accepted candidate.

Successor `dcd3b7ce72c662a72130ed99c9e8273f8520c9d2` added the post-completion/post-restart target recovery-envelope assertion. Review then correctly identified that source-retention qualification could skip source decryptability checks if the source protector disappeared prematurely. That successor is also preserved as superseded and is not the accepted candidate.

Final accepted head `5101835348c2e17793d16eed11f443e7a60f7f24`, tree `bbf2a52ff984260af48f8ad5255585398f78bbaf`, enforces source-protector presence through the exact explicit destruction boundary and preserves the target recovery-wrap assertion. No predecessor history was rewritten, rebased, force-pushed, rerun into qualification, or retroactively reclassified.

## Fault matrix and recovery proof

The qualification harness injects exactly one synthetic failure either immediately before or immediately after each selected durable mutation. `AFTER` applies the wrapped durable mutation first and then returns an error, modeling ambiguous-success crash/power-loss behavior.

The ordinary path covers 24 durable mutation positions on both sides, including protector binding, target protector creation/storage/genesis, authenticated phase checkpoints, staged inventory, manifest publications, protected target-anchor advances, target activations, source inventory/recovery retirement, source protector removal, and rotation checkpoint/binding cleanup. The recovery-enabled path covers both sides of target recovery-wrap persistence. Total scenarios: 50.

Each scenario discards transient nonce-ledger state and rehydrates it from authenticated retained manifest/checkpoint/publication reservations before restart. Pre-checkpoint recovery re-enters through `begin_full_rotation`; checkpointed recovery uses `resume_full_rotation_after_restart`. Qualification requires the injected failure to fire, source material to remain present and decryptable through its required lifetime, eventual stable target-generation `RotationPhase::None`, source retirement, checkpoint/binding cleanup, and a further idempotent restart. Recovery-enabled cases additionally authenticate and decrypt the retained target recovery envelope under the target generation after stable completion plus the additional restart.

## Exact-head qualification

Final head `5101835348c2e17793d16eed11f443e7a60f7f24` passed local provenance validation, generated closure, provenance self-test, 004P dependency closure, formatting, `git diff --check`, clippy with `-D warnings`, full locked workspace tests, and trusted Diffcipline R2/R3. `himsat-core` passed 135/135 tests.

Pull-request CI `34730429135` and R3 `34730429139` both succeeded on the exact final head. Repository-owner review `5188924919` is governance evidence only and explicitly `NOT_Q009`. CodeRabbit confirmed the final source-retention finding addressed and the inline thread is resolved.

## Guarded canonical merge

The merge transport used explicit `expected_head_sha = 5101835348c2e17793d16eed11f443e7a60f7f24` with `merge_method = merge`. GitHub returned canonical merge `1de8a70db3935fba52e1e0fea2abb77e54a7c0b1`.

Canonical parentage and tree are exact:

- parent 1: `96ea50a69ab0b29fdd0869bb4390df12f53ec5e9`
- parent 2: `5101835348c2e17793d16eed11f443e7a60f7f24`
- merge tree: `bbf2a52ff984260af48f8ad5255585398f78bbaf`

GitHub reports the merge signature as verified/valid.

## Post-merge qualification

Push-triggered R3 `34730852422` succeeded on attempt 1 on exact canonical merge `1de8a70db3935fba52e1e0fea2abb77e54a7c0b1`.

Push-triggered CI `34730852426` succeeded on attempt 1 on exact canonical merge `1de8a70db3935fba52e1e0fea2abb77e54a7c0b1`, including Windows Rust job `103653248966`.

## Residual authority

This record does not satisfy Q009. Repository-owner evidence, implementation tests, CI/R3, CodeRabbit, Cubic, or other automation do not substitute for the required genuinely independent substantive crypto/security review of the exact final Specification 004 implementation revision.

B505 may become the only authorized implementation leaf only after the B504/B505 state reconciliation itself is exact-head qualified, expected-head merged, parent/tree verified, and post-merge qualified. B506, closeout, Specification 005, release, FIPS, and compliance claims remain unauthorized until their own dependency gates are proven.
