# 005B Session-Journal Final Evidence

## Scope

This document records the Specification 005 second implementation leaf
(005B): the append-only session-journal codec, file-append discipline with
per-record `sync_all`, truncation-safe machine replay, and crash-resume,
delivered as three size-fitting grains. It implements no manifest binding,
no recovery reconciliation, and no fault-injection harness; those remain
005C/005D candidates. It changes no reviewed 004/005A byte, adopts no
donor code, and adds no dependency, provenance record, SBOM entry, or
workflow change (`sha2` was already in the crate closure).

## Authority

005A is canonically closed (implementation merge
`c3a371511b1bad519c2ed57817f3951e67cb6355`, post-merge CI `35189660586` /
R3 `35189660481` SUCCESS; reconciliation merge
`a97f627088ead6770af2b019ccee0e1b16e8ee80`, post-merge CI `35192652860` /
R3 `35192652887` SUCCESS). The 005 plan re-bounds one leaf at a time; this
leaf re-bounds prospective candidate 005B against live main as the second
implementation Grain. 005C/005D remain shaping-level candidates with no
implementation authority.

## Size-gate split (forward-only)

The 005B leaf was first implemented as one PR (#162, 1550 added lines).
The R3 exact-diff gate failed it on size policy alone (1550 exceeds
maximum 900; no scope violation, all verifications PASS). PR #162 was
closed unmerged as superseded with an explicit do-not-merge record, and
the leaf shipped as three grains of 541/563/322 added lines. No content
from PR #162 merged; all three grains carry reviewed content forward. No
history was rewritten and no gate was weakened.

## Grain 1 — append path (PR #163)

```text
BASE = a97f627088ead6770af2b019ccee0e1b16e8ee80
HEAD = f13dbea3a20c0230b3e43e4eb6dea33f41ec177f (541 added lines)
CONTENT = record layout consts, envelope_digest, TailReason, JournalError,
  build_record + payload builders, JournalWriter create/append/close with
  sync_all + length check, 5 file-based tests
PREMERGE_CI = 35197258314_SUCCESS_PULL_REQUEST_ON_RERUN
PREMERGE_R3 = 35197258286_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 477eeccbd5c1b655011cec06d3b97e421800993e
MERGE_PARENT_1 = a97f627088ead6770af2b019ccee0e1b16e8ee80
MERGE_PARENT_2 = f13dbea3a20c0230b3e43e4eb6dea33f41ec177f
MERGE_TREE = e4193cc44b36102b52804df87dec700a203b1a2e_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35199911608_SUCCESS_PUSH_ATTEMPT_1 (head 477eecc)
POSTMERGE_R3 = 35199911573_SUCCESS_PUSH_ATTEMPT_1 (head 477eecc)
OCR_DELEGATE_REVIEW = PASS_NO_BLOCKING_FINDINGS with 4 non-blocking notes
  (2 fixed as doc precision, 2 accepted by design)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

macOS note: the first macOS attempt failed on pre-existing 004 test
`staging_mutation_after_b505q` (backup/restore area, untouched; all 5
journal tests passed in that run). Same-head evidence (ubuntu green, 3/3
local isolated passes, full local parallel suite 247/247 green) proved no
regression; the failed job re-ran green on the identical head. Recorded
as a runner flake; no code changed.

## Grain 2 — replay (PR #164)

```text
BASE = 477eeccbd5c1b655011cec06d3b97e421800993e
HEAD = e5337f441fa7b75a43b48c3fae114055f0833532 (563 added lines)
CONTENT = field offsets, JournalRecord/Kind, TailStatus, JournalReplay,
  parse helpers, record_kind, parse_records with record_end-derived bounds,
  replay_journal, 8 replay tests (round-trip, empty, exhaustive truncation
  and bit-flip matrices, magic, splice seq-gap, repaired vault transplant,
  missing-file)
PREMERGE_CI = 35200540127_SUCCESS_PULL_REQUEST_ATTEMPT_1
PREMERGE_R3 = 35200540194_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 889cce29af40e6d8ddf79570d68c0065b867ef0e
MERGE_PARENT_1 = 477eeccbd5c1b655011cec06d3b97e421800993e
MERGE_PARENT_2 = e5337f441fa7b75a43b48c3fae114055f0833532
MERGE_TREE = b2f827551c12c8210ced5136a0c11511cc03bd10_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = 35202239838_SUCCESS_PUSH_ATTEMPT_1 (head 889cce2)
POSTMERGE_R3 = 35202239842_SUCCESS_PUSH_ATTEMPT_1 (head 889cce2)
OCR_DELEGATE_REVIEW = PASS_NO_BLOCKING_FINDINGS with 3 non-blocking notes,
  all fixed (grain docs x2, record_end-derived truncation bounds)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

## Grain 3 — resume and arm precision (PR #165)

```text
BASE = 889cce29af40e6d8ddf79570d68c0065b867ef0e
HEAD = f0b4ea823916f47a2c5970f67967cdc83298e777 (322 added lines)
CONTENT = JournalWriter::resume (torn-first/empty/binding/closed refusals,
  file-derived binding, no mutation on refusal), empty/missing resume
  tests, 8-arm exact-reason test, truncation reason-split test
PREMERGE_CI = 35202919135_SUCCESS_PULL_REQUEST_ON_RERUN
PREMERGE_R3 = 35202918981_SUCCESS_PULL_REQUEST_ATTEMPT_1
CANONICAL_MERGE = 59ebf17806e8072302eb6f7a2ae2461d71851d2b
MERGE_PARENT_1 = 889cce29af40e6d8ddf79570d68c0065b867ef0e
MERGE_PARENT_2 = f0b4ea823916f47a2c5970f67967cdc83298e777
MERGE_TREE = ebd1699437fcef03938a41f14ca7146b6a2402f0_EQUALS_ACCEPTED_TREE
POSTMERGE_CI = PENDING (push run 35205561976 on 59ebf17)
POSTMERGE_R3 = PENDING (push run 35205561880 on 59ebf17)
OCR_DELEGATE_REVIEW = APPROVE_NO_BLOCKING_FINDINGS with 3 non-blocking
  notes, all fixed (resume no-mutation lock-in, empty/missing resume
  tests, file-derived binding)
REVIEW_RECONCILIATION = NO_SUBMITTED_REVIEWS_ZERO_THREADS_QODO_BILLING_BLOCKED_CODERABBIT_SKIPPED_CUBIC_NEUTRAL_NO_BLOCKING_FINDING
```

R2 note: the first R2 attempt failed on pre-existing 004 test
`staged_byte_identity_mismatch` (sqlcipher backup area, untouched; all 17
journal tests passed in that run). Same-head evidence (macOS+ubuntu green,
2/2 local isolated passes) proved no regression; the failed job re-ran
green on the identical head. Recorded as a runner flake; no code changed.

## Acceptance proof (final tree at 59ebf17)

```text
FMT = cargo fmt --check clean (local exact-head runs per grain)
CLIPPY = cargo clippy --workspace --all-targets --all-features -- -D warnings clean (per grain)
TESTS = cargo test --workspace --all-targets --locked green: 259 lib passed
  (17 vault_media_journal tests) plus all integration suites, 0 failures
COVERAGE = exact byte sizes (101/173/548/721); empty file; exhaustive
  truncation matrix (every cut 0..=len); exhaustive bit-flip matrix;
  splice seq-gap; integrity-repaired vault and session transplants;
  magic corruption; create/resume/append/close refusal rules with
  no-mutation lock-in; empty/missing resume paths; 8-arm exact reasons;
  truncation region split; digest stability; error display
```

Local journal suites also execute on all three CI operating systems as lib
tests (ubuntu/macos/windows Rust jobs all green per grain).

## Honesty bounds (not claims)

- Integrity is crash-tear detection, not a MAC: forgery resistance for
  chunk bytes stays with the 005A AEAD envelope at recovery time.
- Durability is exactly the host `sync_all` guarantee per record. No
  `F_FULLFSYNC`, directory-entry-durability, or power-loss-proof claim is
  made beyond a successful `sync_all` return.
- The journal carries only public 005A header fields plus digests. No key,
  secret, or plaintext audio ever enters a record.
- Single-writer contract: the caller serializes create/resume; races are
  out of scope by contract, not by mechanism.
- Uncapped `read_to_end` replay is carried as 005C input: streaming replay
  or a byte cap if journals ever face untrusted sizes.

## Residual risks (carried, not blockers)

1. Pre-existing backup staging tests flaked twice on CI runners
   (`staging_mutation_after_b505q`, `staged_byte_identity_mismatch`);
   both pass everywhere else including identical-head reruns. Watch item
   for 005D fault-injection work, not a 005B defect.
2. `commit_count` is verified only by `debug_assert_eq!` until 005C
   cross-checks it against the manifest.

## Post-merge qualification

Grain-3 post-merge results are filled before this reconciliation merges;
if either run fails, this reconciliation does not merge and the failure
is repaired forward-only.
