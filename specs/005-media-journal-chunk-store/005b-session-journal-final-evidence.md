# 005B Session-Journal Final Evidence

## Scope

This document records the Specification 005 second implementation leaf
(005B): the append-only session-journal codec and fsync discipline with
session open/close markers, chunk-commit records quoting public 005A
envelope fields, per-record `sync_all`, and truncation-safe read-only
replay. It implements no manifest binding, no recovery reconciliation, and
no fault-injection harness; those remain 005C/005D candidates. It changes
no reviewed 004/005A byte, adopts no donor code, and adds no dependency,
provenance record, SBOM entry, or workflow change (`sha2` was already in
the crate closure).

## Authority

005A is canonically closed (implementation merge
`c3a371511b1bad519c2ed57817f3951e67cb6355`, post-merge CI `35189660586` /
R3 `35189660481` SUCCESS; reconciliation merge
`a97f627088ead6770af2b019ccee0e1b16e8ee80`, post-merge CI `35192652860` /
R3 `35192652887` SUCCESS). The 005 plan re-bounds one leaf at a time; this
leaf re-bounds prospective candidate 005B against live main as the second
implementation Grain. 005C/005D remain shaping-level candidates with no
implementation authority.

## Change surface

```text
BASE = a97f627088ead6770af2b019ccee0e1b16e8ee80 (origin/main at PR open)
FILES = crates/himsat-core/src/vault_media_journal.rs (new)
        crates/himsat-core/src/lib.rs (+2 module registration)
        specs/005-media-journal-chunk-store/005b-session-journal-final-evidence.md (new, this file)
        specs/005-media-journal-chunk-store/tasks.md (005B section)
        specs/CURRENT.md (005A transcription + B005B lines)
TOTAL = product code additive only; zero modifications to reviewed files
```

## Design (additive-only boundary discipline)

- Fixed 61-byte header: magic(13) + version(2) + type(2) + seq(8) +
  vault(16) + session(16) + payload_len(4); open payload 8B (generation),
  commit payload 80B (index, generation, 24B nonce quote, plaintext length,
  32B envelope digest), close payload 8B (commit count); 32B SHA-256
  trailer over magic..payload. Open/commit/close records are 101/173/101
  bytes.
- One file holds exactly one session; the caller owns placement and opaque
  naming per the 004 metadata rules. No parent directory is created.
- `JournalWriter::create` requires a missing or empty file; `resume`
  refuses empty files, torn tails, binding mismatches, and closed
  sessions. Every successful append ends with `sync_all` plus a
  post-sync length check. Resume never truncates: torn-tail mutation is a
  005C recovery decision.
- Replay is read-only and never fails on content: framing, version, exact
  sequence, constant vault/session binding, exact per-type lengths, and
  integrity are checked in order; the valid prefix plus a machine
  `TailStatus` is returned and no partial record applies.
- Nonces are quoted, never generated: B203 owns the lifecycle; the journal
  only logs the value the caller used, leaving a later reservation log
  buildable on this file.
- Donor comparison: Meetily recording-manager/journaling ideas, Anarlog
  community recorder paths, and OpenSuperWhisper file-queue patterns were
  surveyed as planning inputs. No donor journal implements the
  B202-compatible AEAD binding and truncation discipline required here,
  and donor file-queue UX belongs to the 006+ capture surfaces, so 005B
  adopts no donor code.

## Honesty bounds (not claims)

- Integrity is crash-tear detection, not a MAC: forgery resistance for
  chunk bytes stays with the 005A AEAD envelope at recovery time.
- Durability is exactly the host `sync_all` guarantee per record. No
  `F_FULLFSYNC`, directory-entry-durability, or power-loss-proof claim is
  made beyond a successful `sync_all` return.
- The journal carries only public 005A header fields plus digests. No key,
  secret, or plaintext audio ever enters a record.

## Acceptance proof (exact head PENDING_THIS_PR_CI)

```text
PREMERGE_CI = PENDING (pull_request run on the 005B head)
PREMERGE_R3 = PENDING (R3 exact diff on the 005B head)
FMT = cargo fmt --check clean (local exact-head run)
CLIPPY = cargo clippy --workspace --all-targets --all-features -- -D warnings clean (local exact-head run)
TESTS = cargo test --workspace --all-targets --locked green locally:
  257 lib passed (15 new vault_media_journal tests) plus all integration
  suites, 0 failures
COVERAGE = open/commit/close round-trip with exact byte sizes; empty file;
  exhaustive truncation matrix (every cut offset 0..=len); exhaustive
  single-bit-flip matrix over one commit; splice-built sequence gap;
  exact-reason replay arms (after-close/second-open/first-not-open/version/type/length/generation/session-transplant); truncation reason split;
  integrity-repaired vault transplant; magic corruption; create/refusal
  rules; resume clean/closed/torn/binding cases; close/double-close
  refusal; missing-file I/O error; digest stability; error display
```

## Review reconciliation

```text
OCR_DELEGATE_REVIEW = NO_BLOCKING_FINDINGS with 11 non-blocking notes
  triaged (independent subagent adversarial review under ocr delegate
  rules; LLM endpoint unconfigured, no LLM result claimed). Fixed and
  re-verified 8/8 applicable: resume torn-before-empty ordering, writer
  single-writer docs, u32::try_from hardening, checked seq arithmetic,
  record_end-derived slicing, close-count debug_assert, Drop-guarded temp
  files, exact-reason replay-arm tests. Declined with rationale: parser
  check order (all orders refuse; version-gating before hashing is
  deliberate) and uncapped replay size (carried as 005C input: streaming
  replay or byte cap if journals ever face untrusted sizes). Focused
  re-review of all fixes: 8/8 FIXED_CORRECT, zero new findings. Residual
  risks: concurrent-creator race stays caller-serialized by contract;
  commit_count verified only by debug_assert until 005C.
SUBMITTED_REVIEWS = PENDING_AT_PR_OPEN
REVIEW_RECONCILIATION = PENDING (unavailable/skipped/neutral outputs are
  NOT PASS, never approval)
```

## Merge record

```text
PR = PENDING
MERGE_METHOD = merge
EXPECTED_HEAD_SHA = PENDING
CANONICAL_MERGE = PENDING
```

## Post-merge qualification

```text
POSTMERGE_CI = PENDING
POSTMERGE_R3 = PENDING
```

Pre-merge, merge, and post-merge IDs are filled before the 005B
reconciliation merges; the reconciliation is a separate docs-only unit per
the 005A precedent. If any gate fails, repair is forward-only.
