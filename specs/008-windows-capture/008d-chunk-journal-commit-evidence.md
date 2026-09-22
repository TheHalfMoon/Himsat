# 008D grain 1 — durable commit path for sealed capture chunks (candidate)

## Grain declaration

- Outcome: one sealed 005A chunk envelope plus its adapter facts become exactly
  one 005B commit record and one 006C metadata entry, in order, with typed
  refusals and fail-closed failure handling. This is the first half of the
  journal-sink wiring recorded as Spec 008 task `O009`.
- `scope_in`: `crates/himsat-core/src/capture_journal_commit.rs`
  (`SinkBinding`, `SealedChunk`, `ChunkFacts`, `CommitError`, `CommitReport`,
  `ClosedJournal`, `ChunkJournal`, and their tests) plus its module
  declaration.
- `scope_out`: the second half of `O009` — accumulating captured payloads into
  sealed chunks under the owner's sealer, the 008C admission policy, and the
  checkpoint cadence — which stays a separate bounded grain; also the
  real-time handoff, the real 005A-under-004 sealer binding, manifest binding
  (005C), codecs and resampling (010), and every runtime evidence row that
  needs real Windows hardware.
- Dependencies: none new; the module consumes only closed 005A, 005B, 006C,
  003, and 004 identity contracts.
- Acceptance: exact-head CI + R3 SUCCESS with all three Rust platform jobs
  green and Diffcipline policy bounds respected with no exception.
- Risk: R3 data-integrity surface. No device, no clock, no crypto, and no path
  invention; the module writes only to the caller's journal path.
- Recovery: repair forward; the module is additive and one file deep.
- Minimality: one new module plus one module declaration. The payload
  accumulator, the admission policy, and the cadence window stay in their
  existing owners.
- Safety/security: no `unsafe`, no key material, no nonce generation, no
  network, and no ambient authority; every refusal is typed and a 005B failure
  poisons the handle instead of retrying the append.

## Why this grain is split

A single-grain `O009` (commit path plus payload accumulator plus their tests)
exceeded the Diffcipline `max_added_lines = 900` bound. The Spec 008 plan's
Diffcipline scope control says to split rather than weaken bounds, so `O009`
is delivered as two bounded grains: this commit path first, then the
payload-accumulation grain that produces the `SealedChunk` values it commits.

## Contents and guards

```text
SinkBinding     vault/session/generation binding plus the 005A chunk context
SealedChunk     quoted nonce plus the exact 005A envelope bytes the owner stored
ChunkFacts      plaintext_len, codec, source, and start/end stamps
ChunkJournal    journal-backed handle: create, resume, commit, close
commit          validates the envelope, then appends exactly one commit
close           appends the close marker with the exact commit count
```

1. the envelope must parse under the closed 005A public parser, so a malformed
   or truncated envelope never reaches the journal;
2. the quoted nonce must equal the nonce the envelope itself carries;
3. the quoted plaintext length must equal the length the envelope's own size
   implies (`113 + plaintext + 16`), so a commit can never disagree with the
   bytes it points at about chunk size;
4. commit order is a contiguous 0-based index prefix that advances only after a
   successful append, and a resumed journal whose indexes are gapped is refused
   instead of re-committing an index;
5. a 005B failure poisons the handle, because the closed 005B contract forbids
   retrying a failed append with the same handle.

## Tests

```text
a_commit_quotes_the_exact_envelope_and_records_metadata
envelope_guards_refuse_before_the_journal_is_touched
close_marks_the_exact_commit_count_and_replays_clean
every_committed_envelope_still_decrypts_to_its_plaintext
journal_refusals_surface_as_typed_errors
resume_continues_the_index_and_refuses_a_gapped_prefix
a_poisoned_handle_refuses_further_work_instead_of_retrying
```

Every test seals through the real 005A `encrypt_media_chunk` path with a
caller-supplied nonce, so each commit is bound to a real envelope and the
decrypt test authenticates the exact bytes the journal quotes.

## What is not claimed

- No runtime evidence: no audio device, no OS surface, and no Windows hardware
  is touched; `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- No real sealer binding: the 004 vault lease and the B203 nonce lifecycle are
  not wired here, and this module generates no nonce.
- The 005A parser proves envelope structure, not the vault/session/index
  binding inside the ciphertext; that binding is authenticated at decryption,
  which is 005C reconciliation's job.
- `O009` is not complete: payload accumulation into sealed chunks remains the
  next bounded grain.
