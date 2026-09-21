# Specification 008 Tasks — Windows Capture

> Checkboxes track authored/reconciled work. Exact repository state, CI, provenance, and review evidence remain authoritative.

## Shaping — canonical

- [x] S001 Re-read canonical `main` at Specification 007 closeout merge `c96abba6ed5cd760611d0f407e7c3a8df9fb1879`.
- [x] S002 Confirm Specification 007 post-closeout CI `35627161924` and R3 `35627161891` succeeded.
- [x] S003 Re-read the constitution, master-plan unit 008, Gate E platform-qualification rule, the 003/004/005/006/007 consumable contracts, the donor registry, and the Windows-relevant research records without adopting material.
- [x] S004 Note upstream facts to refresh at implementation time (current supported WASAPI capture APIs, the cpal `wasapi` backend path, microphone privacy-control behavior, exclusive/shared-mode rules, Bluetooth/USB endpoint behavior, loopback restrictions) without adopting dependencies.
- [x] S005 Split prospective leaves 008A-008C as candidates only; exact split re-bounds after shaping qualifies.
- [x] S006 Define scope, additive-only 003/004/005/006/007 boundary discipline, donor-compare posture with 004P-style adoption gate, carried 006 residuals, acceptance, and R3 evidence contracts in `spec.md`/`plan.md`.
- [x] S007 Exact-head qualify shaping head: CI and R3 SUCCESS.
  - Shaping head `3338fd0df0e15f42725b4b11e2ce8a92cde176cf` (PR #202): pre-merge CI `35629526180` SUCCESS, R3 `35629526000` SUCCESS.
- [x] S008 Reconcile shaping diff/reviews/threads/comments/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
  - Docs-only diff; Open Code Review 1.12.7 delegation mode reported 0 reviewable files (markdown/TOML only); zero submitted reviews, bot-only comments, no blocking finding.
- [x] S009 Merge shaping; canonical merge contains the exact shaping head as parent.
  - Canonical shaping merge `fa797a8288f26e98c5460e8f766f2acb44957101` (PR #202, parents `c96abba` + `3338fd0`; merge tree equals shaping-head tree).
- [x] S010 Require post-shaping qualification: CI and R3 SUCCESS on the exact canonical merge; only then may 008 implementation grains be bound.
  - Post-merge CI `35631397479` SUCCESS, R3 `35631396992` SUCCESS on `fa797a8288f26e98c5460e8f766f2acb44957101`.

## Implementation — 008A dependency adoption (Windows `cpal`)

- [x] I001 Verify registry entry for `cpal 0.18.2` (checksum, source repository/revision, Apache-2.0 license, registry count 211) before any product change.
- [x] I002 Bind the Windows-only target edge `cpal = { version = "=0.18.2", default-features = false }` in `crates/himsat-core/Cargo.toml`; no global dependency, no feature widening, no other target touched.
- [x] I003 Extend `tools/004p_dependency_closure.py` `EXPECTED_WINDOWS_DIRECT` so the closure gate fails closed on any drift of the new edge.
- [x] I004 Prove the WASAPI backend subset from immutable crates.io index metadata (`default = []`; non-optional `windows ^0.62` with the Win32_Media* features and `windows-core ^0.62`; ASIO/JACK/`audio_thread_priority`/`num-traits` disabled) and from `cargo tree --target x86_64-pc-windows-msvc -p cpal`.
- [x] I005 Record that no `Cargo.lock`, registry, notice, SBOM, or workflow byte changes: 211 external packages, 41 provider packages, unchanged.
- [x] I006 Run the local gates: `004p_dependency_closure.py`, `provenance_gate.py validate`, `provenance_gate.py check-generated`, `cargo metadata --locked`, `cargo fmt --check`.
- [ ] I007 Exact-head qualify the adoption head: pull-request CI and R3 SUCCESS with the `windows-latest` Rust job green.
- [ ] I008 Reconcile diff/reviews/threads/`main`/mergeability; billing-blocked/skipped/absent review output is not PASS.
- [ ] I009 Merge under explicit expected-head protection; verify parents, tree equality, and signature.
- [ ] I010 Require post-merge CI and R3 SUCCESS on the exact canonical merge, then record the adoption merge truth and promote the Windows microphone implementation grain.
