# 008E grain — payload accumulation into sealed chunks (candidate)

## Grain declaration

- Outcome: captured payload bytes become the `SealedChunk` values the 008D
  commit path appends to the 005 journal, in contiguous 0-based order, with
  typed refusals and no silent frame loss. This is the second half of the
  journal-sink wiring recorded as Spec 008 task `O009`.
- `scope_in`: `crates/himsat-core/src/capture_payload_accum.rs`
  (`AccumConfig`, `PayloadAccum`, `PushReport`, `CloseSummary`, `AccumError`,
  and their tests) plus its module declaration.
- `scope_out`: the real-time audio callback stays a bounded forward (no I/O,
  seal, or commit inside it); the 004 lease/B203 reservation binding, manifest
  binding (005C), codecs and resampling (010), and every runtime evidence row
  that needs real Windows hardware.
- Dependencies: none new; the module consumes only closed 005A, 005B (via the
  008D types), 006C, 008C, and 003 contracts.
- Acceptance: exact-head CI + R3 SUCCESS with all three Rust platform jobs
  green and Diffcipline policy bounds respected with no exception.
- Risk: R3 data-integrity surface. No device, no clock, no crypto, no key
  material, and no path invention; the module buffers caller bytes and quotes
  caller nonces only.
- Recovery: repair forward; the module is additive and one file deep.
- Minimality: one new module plus one module declaration. Admission and
  cadence stay in their closed 008C owners; sealing authority stays with the
  owner-held 005A sealer.
- Safety/security: no `unsafe`, no key material, no nonce generation, no
  network, and no ambient authority; every refusal is typed and keeps buffered
  bytes, stamps, and indexes unchanged.

## Contents and guards

```text
AccumConfig   binding/codec/source plus chunk, pending, budget, cadence bounds
PayloadAccum  push (real-time safe) / seal_next (drain thread) / note_committed / close
push          timestamps, 008C admission, pending bound, chunk bound, bounded copy
seal_next     owner 005A sealer called once; nonce and length checked pre-advance
close         refuses with buffered bytes or uncommitted chunks outstanding
```

1. `push` is the only real-time method: ordered checks plus one copy into the
   pre-reserved buffer; sealing, committing, I/O, and telemetry never run in
   the audio callback;
2. admission reuses closed 008C `decide_admission`: warn degrades (surfaced by
   the owner through 006B), critical storage or a full queue refuses with the
   exact `RefusalReason`;
3. the owner seals with its own 005A sealer under the 004 lease; nonces are
   owner-supplied under the B203 lifecycle and never generated here;
4. the envelope is verified (parser nonce equals the quoted nonce; length
   equals buffered bytes plus 129) before any index advances, so a lying
   sealer keeps the bytes for an honest retry instead of splitting the order;
5. chunk indexes are a contiguous 0-based prefix; `close` refuses with
   `BufferedRemainder` or `PendingDrain` rather than abandoning audio.

## Tests

```text
push_then_seal_round_trips_into_the_commit_path
admission_refusals_are_explicit_before_buffering
chunk_full_backpressure_never_drops_silently
timestamps_are_checked_and_never_regress
seal_due_follows_size_and_cadence
close_demands_an_empty_buffer_and_a_drained_queue
a_lying_sealer_is_refused_without_losing_bytes
every_sealed_envelope_decrypts_and_indexes_stay_contiguous
```

Every test seals through the real 005A `encrypt_media_chunk` path with a
caller-supplied nonce; the round-trip test commits through the real 008D path
and decrypts the exact bytes the journal quotes.

## What is not claimed

- No runtime evidence: no audio device, no OS surface, and no Windows hardware
  is touched; `SPEC_008_PLATFORM_SCOPE` stays
  `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
- Buffered-but-unsealed bytes and sealed-but-uncommitted chunks are volatile;
  durability begins at the 008D commit, which is why `close` refuses both.
- The 005A context-accessor limitation recorded in 008D carries over
  unchanged: a structurally valid envelope sealed for another context is
  authenticated at decryption and reconciled by 005C, and no reviewed 005A
  byte is changed here.
- `O009` is not complete until this grain is canonically qualified.
