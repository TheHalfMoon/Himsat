# 007B System Tap — Final Evidence (grains 1+2, reconciled)

## Bounded unit

B007B closes the 007B system-tap pathway: tap core over an
injected backend (grain 1, zero new dependencies) plus the live
cidre 0.29.0 tap-lifecycle binding with full dependency closure
(grain 2, one direct + one transitive package, both MIT). Donor
inputs compared live at pinned revisions; no donor code copied.
Sample streaming stays in 007C as the recorded residual (IOProc
sample extraction needs an unsafe block the workspace forbids);
aggregate-device assembly and the evidence matrix are 007C.

## Grain 1 — tap core (PR191)

- Head `a903565`, 4 files, 453 added / 0 removed:
  `capture_system_audio.rs` (backend trait, route discovery/
  selection/identity, fault classification, mock backend) +
  `lib.rs` line + core evidence + tasks binding.
- Pre-merge CI/R3 SUCCESS (exact-head, per merge record).
- Canonical merge `45e913056a26477ac3bb1f5d2f499f80e0bec295`;
  tree equals head tree.
- Post-merge CI `35302602751` / R3 `35302602767` SUCCESS.
- Reviews: none submitted; no blocking finding.

## Gate (PR193)

- B007B gate hardening `0672c63` (1 file, 119 added /
  0 removed, tool-only): trusted-base predicate + pinned
  ten-file exception accepting REVIEW exit-1 only (B006A
  precedent: in-policy size, manifest + lockfile reasons).
  Precondition base is live main `45e9130`.
- Pre-merge CI `35307196109` / R3 `35307196084` SUCCESS direct
  (no exception needed, gate-only diff).
- Canonical merge `faff02d9490c6b42850745f68e9d2e9f228654dc`
  (`45e9130` + `0672c63`); delta is the gate file only.
- Post-merge CI `35308616412` / R3 `35308616390` SUCCESS.
- Live R3 proof on the candidate (`35305754018`) showed the
  manifest-changing adoption exits 1/REVIEW with the exact pinned
  file/count/reason set — the shape this gate authorizes. No
  predicate correction was needed (first-attempt fail is the
  designed authorization proof, recorded not hidden).

## Grain 2 — cidre binding adoption (PR192)

- Head `eee37d6` (content `ea483ca` + gate-base merge; all ten
  pinned blobs intact across the merge), 10 files, 529 added /
  2 removed: cidre `=0.29.0` macOS-target DEPEND (features
  `av`/`core_audio`/`dispatch`/`macos_15_0`),
  `CidreSystemTapBackend`, live `probe_process_tap`
  authorization/create/format proof, F32-only ASBD mapping
  (`DeviceConfig::from_supported` widened to `pub(crate)`,
  non-F32 refused), 004P closure entries, registry 209 -> 211,
  regenerated notices/SBOM, binding evidence, tasks binding.
- Pre-merge CI `35308649616` / R3 `35308649631` SUCCESS (REVIEW
  exit-1 via the B007B exception, live-authorized).
- Canonical merge `086b59026a402a3c9fdaefea61c97e91e02b6cb9`
  (`faff02d` + `eee37d6`); tree equals adopted head tree.
- Post-merge CI `35309850877` / R3 `35309850870` SUCCESS.
- Reviews: none submitted (2 bot comments, zero threads);
  cubic skipping, CodeRabbit review-skipped; no blocking finding.

## Live-hardware proof

MacBook Pro Speakers enumerated with default flag, 2ch/48kHz/
F32; `probe_process_tap` returned `TAP_OK` with a 36-char uid
and 1ch/48kHz snapshot; guard dropped clean with no stream
persisting. Env-gated live tests pass; default CI runs
hardware-tolerant enumeration plus pure ASBD-mapping tests
(13 `capture_system_audio` tests green).

## Residual risks (carried, not blockers)

1. Sample streaming is unauthorised until a sanctioned
   sample-extraction path exists under `forbid(unsafe_code)`
   (007C input, shaping must say how).
2. Aggregate-device assembly is unproven (007C input).
3. Non-F32 tap formats refuse as build faults; negotiation
   widens only with Gate E evidence (007C input).

## Disposition

B007B is CANONICAL_CLOSED: shaped (007 shaping qualified),
bounded (grains 1+2 + gate), exact-head qualified with
merge-tree equality throughout, post-merge CI/R3 SUCCESS on
every merge SHA, reconciled here. 007C (evidence matrix +
streaming sanction) may now be shaped.
