# Specification 005 Tasks — Crash-Safe Media Journal and Chunk Store

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and review evidence remain authoritative.

## Shaping — canonical

- [x] S001 Re-read canonical `main` at Specification 004 closeout merge `c5255f6a60044edbdd7562fe1f295ce59e14c5a0`.
- [x] S002 Confirm Specification 004 post-closeout CI `35091125997` and R3 `35091125863` succeeded.
- [x] S003 Re-read constitution, master-plan unit 005, the 004 media-adjacent contracts (B202/B203/B501/B503/B505/B506), and the donor registry without adopting material.
- [x] S004 Note upstream facts to refresh at implementation time (container framing, fsync/durability semantics, fault-injection harnesses) without adopting dependencies.
- [x] S005 Split prospective leaves 005A-005D as candidates only; exact split re-bounds after shaping qualifies.
- [x] S006 Define scope, additive-only 004 boundary discipline, donor-compare posture, acceptance, and R3 evidence contracts in `spec.md`/`plan.md`.
- [x] S007 Exact-head qualify shaping head: CI and R3 SUCCESS.
  - Shaping head `feb92dd4b13b9982c1fd5404b99a000138172fd7` (PR #159): pre-merge CI `35094188292` SUCCESS, R3 `35094188252` SUCCESS.
- [x] S008 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
- [x] S009 Merge shaping; canonical merge contains the exact shaping head as parent.
  - Canonical shaping merge `2d7ec3e8e714d493f865ecf445d93d503351e390` (PR #159, parents `c5255f6a60044edbdd7562fe1f295ce59e14c5a0` + `feb92dd4b13b9982c1fd5404b99a000138172fd7`; merge tree equals shaping-head tree). The historical merge API transport argument is not reconstructible post hoc; parentage and tree are verified live. Zero submitted reviews; comments are bot-only (Qodo billing-blocked, CodeRabbit skip), no blocking finding.
- [x] S010 Require post-shaping qualification: CI and R3 SUCCESS on the exact canonical merge.
  - Post-merge CI `35096310420` SUCCESS, R3 `35096310425` SUCCESS on `2d7ec3e8e714d493f865ecf445d93d503351e390`.

## 005A media-chunk envelope — implementation leaf (re-bounded after shaping qualified)

- [x] A001 Re-bound 005A against live main `2d7ec3e8e714d493f865ecf445d93d503351e390` as the first implementation Grain; 005B-005D remain candidates.
- [x] A002 Implement additive-only envelope (`vault_media_chunk.rs`), `KeyPurpose::MediaChunk` domain, teardown-owned key slot, 15 adversarial tests; no B202 byte changed, no donor adoption, no dependency change.
- [x] A003 Exact-head qualify `f338b3c264103a64a74dcc27248f3badd930ca34`: CI `35099474113` SUCCESS, R3 `35099474219` SUCCESS; local fmt/clippy/tests green.
- [x] A004 OCR delegate adversarial review: NO_BLOCKING_FINDINGS; reconcile zero submitted reviews/threads with unavailable/skipped/neutral outputs as NOT PASS.
- [x] A005 Expected-head merge PR #160 to canonical merge `c3a371511b1bad519c2ed57817f3951e67cb6355` (parents `2d7ec3e8e714d493f865ecf445d93d503351e390` + `f338b3c264103a64a74dcc27248f3badd930ca34`; merge tree equals accepted tree).
- [x] A006 Post-merge qualification: CI and R3 SUCCESS on `c3a371511b1bad519c2ed57817f3951e67cb6355` (push CI `35189660586` SUCCESS 17m59s / push R3 `35189660481` SUCCESS 10m23s, both head `c3a3715`).
- [x] A007 Reconcile `specs/CURRENT.md` and this evidence in the 005A reconciliation PR before authorizing 005B.
  - Reconciliation merged as `a97f627088ead6770af2b019ccee0e1b16e8ee80` (PR #161, parents `c3a371511b1bad519c2ed57817f3951e67cb6355` + `517fdf38a11559b52da0269d0f0842785c333409`; merge tree equals accepted tree). Post-merge CI `35192652860` SUCCESS and R3 `35192652887` SUCCESS on exact merge SHA `a97f627` (transcribed by the 005B leaf PR per the next-unit-records-prior-postmerge precedent).

## 005B session journal codec and fsync discipline — implementation leaf (re-bounded after 005A closed)

- [x] B001 Re-bound 005B against live main `a97f627088ead6770af2b019ccee0e1b16e8ee80` as the second implementation Grain; 005C/005D remain candidates.
- [x] B002 Implement additive-only journal (`vault_media_journal.rs`): fixed 61B header, open/commit/close records, per-record `sync_all` with length check, read-only replay with `TailStatus`, 15 focused tests incl. exhaustive truncation/bit-flip matrices and exact-reason replay arms; no reviewed file changed, no donor adoption, no dependency change.
- [x] B003 Local exact-head proof: `cargo fmt --check` clean; `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean; `cargo test --workspace --all-targets --locked` green (257 lib incl. 15 new, 0 failures).
- [ ] B004 OCR delegate adversarial review with NO_BLOCKING_FINDINGS; reconcile submitted reviews/threads with unavailable/skipped/neutral outputs as NOT PASS.
- [ ] B005 Exact-head qualify 005B head: CI and R3 SUCCESS (IDs filled before merge).
- [ ] B006 Expected-head merge; canonical merge contains the exact 005B head as parent with merge tree equal to the accepted tree.
- [ ] B007 Post-merge qualification and follow-up reconciliation (records B005B merge/post-merge IDs, closes 005B) before authorizing 005C.
