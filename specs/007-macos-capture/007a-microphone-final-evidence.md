# 007A Microphone Pathway — Final Evidence (grains 1+2, reconciled)

## Bounded unit

B007A closes the 007A microphone pathway: adapter core over an
injected backend (grain 1, zero new dependencies) plus the live
cpal 0.18.2 OS binding with full dependency closure (grain 2, one
direct + 47 transitive packages). Donor inputs compared live at
pinned revisions; no donor code copied. System tap stays in 007B;
evidence matrix in 007C.

## Grain 1 — adapter core (PR185)

- Head `3df8e39bdca3e57f504095a2b066bf7204637d24`, 4 files,
  504 added / 0 removed: `capture_macos.rs` (backend trait,
  selection, stable FNV-1a source ids, bounded labels, fault
  classification, mock backend, 10 tests) + `lib.rs` line +
  decision evidence + tasks binding.
- Pre-merge CI `35284373920` / R3 `35284373899` SUCCESS.
- Canonical merge `0ef54bc941bdaddce35eb1d4a18554fc544d3966`
  (`bf9d43b` + `3df8e39`); tree equals head tree
  (`107b0ca3600520e10e7e2347187521e55ed5d1ab`).
- Post-merge CI `35285918928` / R3 `35285918939` SUCCESS.
- Reviews: none submitted, zero threads; no blocking finding.

## Gate saga (PR186/PR188, PR187 superseded)

- B007 gate hardening `2d339a8` (PR186, 117-line tool-only):
  trusted-base predicate + pinned eight-file exception. Pre-merge
  CI `35288509258` / R3 `35288509273` SUCCESS; merge `9041323`
  (`0ef54bc` + `2d339a8`, tree-equal); post-merge CI
  `35290282633` / R3 `35290282960` SUCCESS.
- Live R3 proof on the candidate showed the oversized adoption
  exits 2/FAIL (not 1/REVIEW) with the exact pinned file/count/
  reason set otherwise. Predicate corrected forward in `2821d7d`
  (PR188, 4a/3d tool-only, B404 exit-2 shape, precondition advanced
  to `9041323`): pre-merge CI `35292624902` / R3 `35292625001`
  SUCCESS; merge `323fbe4` (`9041323` + `2821d7d`, tree-equal);
  post-merge R3 `35293977339` SUCCESS; post-merge CI
  `35293977326` superseded-cancelled by the newer push (no
  failure; tree coverage subsumed by the adoption merge CI).
- `dc37960` (PR187) predated the gate merge so its tree delta
  reverted the gate tool; closed superseded with no re-authoring,
  reshipped byte-identical as `0bbc1f4` atop `323fbe4`.

## Grain 2 — cpal binding adoption (PR189)

- Head `0bbc1f465aef056bbb03e3d016c24cf3f2215dbe`, 8 files,
  5551 added / 297 removed: cpal `=0.18.2` macOS-target DEPEND,
  `CpalMicrophoneBackend`, F32 open/play/pause/resume,
  `PermissionDenied` classification, 004P closure entries,
  registry (+48), regenerated notices/SBOM, binding evidence.
- Pre-merge CI `35294006607` / R3 `35294006447` SUCCESS (FAIL
  exit-2 via the corrected B007 exception, live-proven).
- Canonical merge `9578d65b9cd17d665617db4acb3462fc3015c76c`
  (`323fbe4` + `0bbc1f4`); tree equals head tree
  (`ba1ea948dd3e272f27e6ccb0b3cfc09172c96dbe`).
- Post-merge CI `35295689765` / R3 `35295689738` SUCCESS.
- Reviews: none submitted, zero threads; no blocking finding.

## License basis correction (recorded, not hidden)

Grain-1 evidence assumed cpal MIT-dual. Tag-level verification
(`v0.15.3`–`v0.18.2`) proves Apache-2.0-only; `cpal 0.18.2` is
selected `Apache-2.0` through `EXPECTED_PLATFORM_PACKAGES`
(policy-allowed, fully evidenced), the sole non-MIT selection of
the 48. All other new packages MIT-selected with MIT notice text.

## Live-hardware proof

MacBook Pro Microphone enumerated with default flag, 1ch/48kHz/
F32; `open_f32_input_stream` returned `OPEN_OK` with matching
config; immediate drop clean. Env-gated live tests pass;
default CI runs hardware-tolerant enumeration/selection tests.

## Residual risks (carried, not blockers)

1. `tools/provenance.py` still expects registry schema v1 while
   the registry is v2, so its `generate`/`validate` fail on main
   too; regeneration used the v2 `provenance_gate.py` renderer
   that CI checks. Repairing the stale tool is deferred.
2. Non-F32 default configurations refuse as build faults; format
   negotiation widens only with Gate E evidence.
3. The ~300 removed diff lines in the adoption are regeneration
   reordering, not content loss (161 -> 209 entries, zero lost,
   per-entry rendering byte-identical).

## Disposition

B007A is CANONICAL_CLOSED: shaped (007 shaping qualified),
bounded (grains 1+2 + gate saga), exact-head qualified with
merge-tree equality throughout, post-merge CI/R3 SUCCESS on every
merge SHA (one superseded-cancelled CI covered by its superset),
reconciled here. 007B (system tap) may now begin; 007C owns the
permission/sleep-wake/long-session matrix.
