//! 008E payload accumulation for sealed capture chunks.
//!
//! Second half of the Spec 008 journal-sink wiring (`O009`): captured payload
//! bytes become the `SealedChunk` values the 008D commit path appends to the
//! 005 journal, in order, with typed refusals and no silent frame loss.
//!
//! Threading model: the real-time callback calls only `PayloadAccum::push`
//! (bounded checks plus one bounded copy: no I/O, journal, network, clock,
//! lock, unbounded queue, retry, inference, or expensive logging), while the
//! owner drain thread seals (`seal_next` through the owner 005A sealer) and
//! commits through the 008D path.
//!
//! Authority boundaries (nothing closed is redesigned): sealing is an
//! owner-supplied 005A function under the 004 lease (no key material here);
//! nonces are owner-supplied under the B203 lifecycle (never generated here);
//! admission reuses closed 008C `decide_admission` over caller observations;
//! cadence reuses closed 008C `checkpoint_due` over caller-stamped time (the
//! owner carries the remainder); timestamps are checked caller facts.
//!
//! Crash honesty: buffered-but-unsealed bytes and sealed-but-uncommitted
//! chunks are volatile; durability begins at the 008D commit, so `close`
//! refuses both. Resume restarts at the owner-supplied `starting_index`,
//! which must equal the resumed journal commit count.
//!
//! Donor posture: no donor code is copied; the shape is Himsat-native over
//! already-closed Himsat contracts.

use crate::capture_checkpoint::ChunkCodec;
use crate::capture_journal_commit::{ChunkFacts, SealedChunk, SinkBinding};
use crate::capture_windows_pressure::{
    AdmissionDecision, RefusalReason, StorageBudgets, checkpoint_due, decide_admission,
};
use crate::vault_media_chunk::{
    MEDIA_CHUNK_MAX_PLAINTEXT_BYTES, MEDIA_CHUNK_MIN_ENVELOPE_BYTES, MEDIA_CHUNK_NONCE_BYTES,
    MediaChunkContext, MediaChunkError, media_chunk_nonce,
};
use himsat_events::SourceId;
use std::error::Error;
use std::fmt;

/// Owner-held configuration for one accumulation session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccumConfig {
    /// Vault/session/generation binding every sealed chunk quotes.
    pub binding: SinkBinding,
    /// Portable codec tag the owner already assigned (006C vocabulary).
    pub codec: ChunkCodec,
    /// 003 source identity every chunk of this session came from.
    pub source: SourceId,
    /// Largest sealed plaintext in bytes; must not exceed the 005A ceiling.
    pub max_chunk_bytes: usize,
    /// First chunk index: 0 for a new session, or the replayed commit count
    /// when resuming beside a resumed 008D journal.
    pub starting_index: u64,
    /// Largest number of sealed-but-uncommitted chunks; bounds memory.
    pub max_pending_chunks: u64,
    /// Caller-supplied storage and queue budgets for the 008C policy.
    pub budgets: StorageBudgets,
    /// Caller-supplied seal cadence in milliseconds for `seal_due`.
    pub cadence_millis: u64,
}

/// Report for one accepted `push`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PushReport {
    /// Payload bytes accepted by this call.
    pub accepted_bytes: usize,
    /// Total buffered bytes awaiting a seal after this call.
    pub buffered_bytes: usize,
}

/// Summary returned when an accumulation session closes cleanly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CloseSummary {
    /// How many chunks this instance sealed over its lifetime.
    pub chunks_sealed: u64,
    /// Index the next chunk would have carried.
    pub next_chunk_index: u64,
}

/// Fail-closed accumulation errors: every refusal names its cause and leaves
/// buffered bytes, indexes, and pending counts exactly as they were.
#[derive(Debug)]
pub enum AccumError {
    /// The owner sealer rejected the bytes.
    SealFailed(MediaChunkError),
    /// The 008C admission policy refused new audio.
    Refused(RefusalReason),
    /// One push exceeds the whole chunk bound.
    PayloadTooLarge { bytes: usize, max_bytes: usize },
    /// The push would overflow the current chunk; seal first.
    ChunkFull {
        buffered_bytes: usize,
        incoming_bytes: usize,
        max_bytes: usize,
    },
    /// Too many sealed-but-uncommitted chunks; commit or drain first.
    PendingFull { pending: u64, max_pending: u64 },
    /// The push carries an inverted timestamp range.
    TimestampRange { start_ms: u64, end_ms: u64 },
    /// The push starts before the previously observed stream time.
    TimestampRegression { start_ms: u64, stream_end_ms: u64 },
    /// A seal was requested with nothing buffered.
    EmptySeal,
    /// A commit was acknowledged with nothing pending.
    NoPendingCommit,
    /// Close was requested while bytes were still buffered; seal them first.
    BufferedRemainder { bytes: usize },
    /// Close was requested while sealed chunks were uncommitted.
    PendingDrain { pending: u64 },
    /// The sealed envelope carries a different nonce than the one supplied.
    NonceMismatch,
    /// The sealed envelope length disagrees with the buffered plaintext.
    LengthMismatch {
        envelope_bytes: usize,
        plaintext_len: u64,
    },
    /// Chunk indexes exhausted the `u64` space.
    IndexExhausted,
    /// The session already closed.
    Closed,
}

impl fmt::Display for AccumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::SealFailed(_) => "payload sealer rejected the buffered bytes",
            Self::Refused(_) => "admission policy refused new audio",
            Self::PayloadTooLarge { .. } => "payload exceeds the chunk bound",
            Self::ChunkFull { .. } => "payload would overflow the current chunk",
            Self::PendingFull { .. } => "sealed-but-uncommitted queue is full",
            Self::TimestampRange { .. } => "payload timestamp range is inverted",
            Self::TimestampRegression { .. } => "payload starts before observed stream time",
            Self::EmptySeal => "no buffered payload to seal",
            Self::NoPendingCommit => "no sealed chunk awaits commit",
            Self::BufferedRemainder { .. } => "close refused with buffered payload outstanding",
            Self::PendingDrain { .. } => "close refused with sealed chunks uncommitted",
            Self::NonceMismatch => "sealed envelope carries a different nonce",
            Self::LengthMismatch { .. } => "sealed envelope disagrees with the plaintext",
            Self::IndexExhausted => "chunk index space is exhausted",
            Self::Closed => "payload accumulation session already closed",
        };
        f.write_str(message)
    }
}

impl Error for AccumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SealFailed(inner) => Some(inner),
            _ => None,
        }
    }
}

/// Bounded accumulator that turns captured payloads into sealed chunks.
///
/// One instance owns exactly one session buffer. Call `push` from the
/// real-time callback, `seal_next` plus the 008D commit from the owner drain
/// thread, `note_committed` per commit, and `close` at session stop.
pub struct PayloadAccum {
    config: AccumConfig,
    buffer: Vec<u8>,
    chunk_start_ms: Option<u64>,
    stream_end_ms: Option<u64>,
    next_index: u64,
    pending: u64,
    sealed_total: u64,
    closed: bool,
}

impl PayloadAccum {
    /// Creates an accumulator and pre-reserves the chunk buffer, so
    /// steady-state pushes copy bytes without allocating.
    ///
    /// # Errors
    ///
    /// Returns [`AccumError::PayloadTooLarge`] when the chunk bound is zero or
    /// exceeds the 005A plaintext ceiling, and [`AccumError::PendingFull`]
    /// when the pending bound is zero (nothing could ever be sealed).
    pub fn new(config: AccumConfig) -> Result<Self, AccumError> {
        if config.max_chunk_bytes == 0 || config.max_chunk_bytes > MEDIA_CHUNK_MAX_PLAINTEXT_BYTES {
            return Err(AccumError::PayloadTooLarge {
                bytes: config.max_chunk_bytes,
                max_bytes: MEDIA_CHUNK_MAX_PLAINTEXT_BYTES,
            });
        }
        if config.max_pending_chunks == 0 {
            return Err(AccumError::PendingFull {
                pending: 0,
                max_pending: 0,
            });
        }
        let mut buffer = Vec::with_capacity(config.max_chunk_bytes);
        Ok(Self {
            config,
            buffer,
            chunk_start_ms: None,
            stream_end_ms: None,
            next_index: config.starting_index,
            pending: 0,
            sealed_total: 0,
            closed: false,
        })
    }

    /// Returns the buffered bytes awaiting a seal.
    #[must_use]
    pub fn buffered_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the sealed-but-uncommitted chunk count.
    #[must_use]
    pub const fn pending_chunks(&self) -> u64 {
        self.pending
    }

    /// Returns the index the next sealed chunk will carry.
    #[must_use]
    pub const fn next_chunk_index(&self) -> u64 {
        self.next_index
    }

    /// Accepts payload bytes into the current chunk.
    ///
    /// This is the only method that runs inside the real-time audio callback,
    /// and it stays bounded: ordered timestamp checks, one pure admission
    /// decision, two bound checks, then a single copy into the pre-reserved
    /// buffer. Sealing, committing, I/O, and telemetry stay on the drain
    /// thread; warn-level degradation is surfaced by the owner through 006B,
    /// never silently.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal for a closed session, inverted or regressed
    /// timestamps, an 008C refusal, a full pending queue, an oversized payload,
    /// or a push that would overflow the current chunk. A refused push changes
    /// nothing.
    pub fn push(
        &mut self,
        payload: &[u8],
        start_ms: u64,
        end_ms: u64,
        free_bytes: u64,
        queue_depth: u64,
    ) -> Result<PushReport, AccumError> {
        if self.closed {
            return Err(AccumError::Closed);
        }
        if end_ms < start_ms {
            return Err(AccumError::TimestampRange { start_ms, end_ms });
        }
        if let Some(stream_end) = self.stream_end_ms
            && start_ms < stream_end
        {
            return Err(AccumError::TimestampRegression {
                start_ms,
                stream_end_ms: stream_end,
            });
        }
        if let AdmissionDecision::Refuse(reason) =
            decide_admission(self.config.budgets, free_bytes, queue_depth)
        {
            return Err(AccumError::Refused(reason));
        }
        if self.pending >= self.config.max_pending_chunks {
            return Err(AccumError::PendingFull {
                pending: self.pending,
                max_pending: self.config.max_pending_chunks,
            });
        }
        if payload.len() > self.config.max_chunk_bytes {
            return Err(AccumError::PayloadTooLarge {
                bytes: payload.len(),
                max_bytes: self.config.max_chunk_bytes,
            });
        }
        let buffered = self.buffer.len();
        let Some(total) = buffered.checked_add(payload.len()) else {
            return Err(AccumError::ChunkFull {
                buffered_bytes: buffered,
                incoming_bytes: payload.len(),
                max_bytes: self.config.max_chunk_bytes,
            });
        };
        if total > self.config.max_chunk_bytes {
            return Err(AccumError::ChunkFull {
                buffered_bytes: buffered,
                incoming_bytes: payload.len(),
                max_bytes: self.config.max_chunk_bytes,
            });
        }
        self.buffer.extend_from_slice(payload);
        if self.chunk_start_ms.is_none() {
            self.chunk_start_ms = Some(start_ms);
        }
        self.stream_end_ms = Some(end_ms);
        Ok(PushReport {
            accepted_bytes: payload.len(),
            buffered_bytes: self.buffer.len(),
        })
    }

    /// Reports whether the drain thread should seal now: the buffer is nonempty
    /// and either reached the size bound or the caller-stamped cadence elapsed.
    #[must_use]
    pub fn seal_due(&self, elapsed_since_last_seal_millis: u64) -> bool {
        if self.buffer.is_empty() {
            return false;
        }
        if self.buffer.len() >= self.config.max_chunk_bytes {
            return true;
        }
        checkpoint_due(elapsed_since_last_seal_millis, self.config.cadence_millis)
    }

    /// Seals the buffered bytes into the next contiguous chunk.
    ///
    /// Runs on the owner drain thread, never in the audio callback: `seal` is
    /// the owner 005A sealer (key material and B203 nonce handling stay with
    /// the owner), called exactly once with this chunk context, the supplied
    /// nonce, and the buffered plaintext. The envelope is checked against the
    /// quoted nonce and the implied plaintext length before any index advances,
    /// so a lying sealer cannot split the sequence.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal for a closed session, an empty buffer, a full
    /// pending queue, a sealer failure, a nonce or length disagreement, or an
    /// exhausted index. A refused seal keeps the buffer, stamps, and indexes
    /// so the owner can retry with a corrected sealer without losing bytes.
    pub fn seal_next(
        &mut self,
        nonce: [u8; MEDIA_CHUNK_NONCE_BYTES],
        seal: impl FnOnce(
            MediaChunkContext,
            [u8; MEDIA_CHUNK_NONCE_BYTES],
            &[u8],
        ) -> Result<Vec<u8>, MediaChunkError>,
    ) -> Result<(SealedChunk, ChunkFacts), AccumError> {
        if self.closed {
            return Err(AccumError::Closed);
        }
        if self.buffer.is_empty() {
            return Err(AccumError::EmptySeal);
        }
        if self.pending >= self.config.max_pending_chunks {
            return Err(AccumError::PendingFull {
                pending: self.pending,
                max_pending: self.config.max_pending_chunks,
            });
        }
        let chunk_index = self.next_index;
        let context = self.config.binding.chunk_context(chunk_index);
        let plaintext_len = self.buffer.len();
        let envelope = seal(context, nonce, &self.buffer).map_err(AccumError::SealFailed)?;
        match media_chunk_nonce(&envelope) {
            Err(error) => return Err(AccumError::SealFailed(error)),
            Ok(quoted) => {
                if quoted != nonce {
                    return Err(AccumError::NonceMismatch);
                }
            }
        }
        let Some(implied) = plaintext_len.checked_add(MEDIA_CHUNK_MIN_ENVELOPE_BYTES) else {
            return Err(AccumError::LengthMismatch {
                envelope_bytes: envelope.len(),
                plaintext_len: plaintext_len as u64,
            });
        };
        if envelope.len() != implied {
            return Err(AccumError::LengthMismatch {
                envelope_bytes: envelope.len(),
                plaintext_len: plaintext_len as u64,
            });
        }
        let Some(next_index) = chunk_index.checked_add(1) else {
            return Err(AccumError::IndexExhausted);
        };
        let Some(next_pending) = self.pending.checked_add(1) else {
            return Err(AccumError::IndexExhausted);
        };
        let Some(next_total) = self.sealed_total.checked_add(1) else {
            return Err(AccumError::IndexExhausted);
        };
        let facts = ChunkFacts {
            plaintext_len: plaintext_len as u64,
            codec: self.config.codec,
            source: self.config.source,
            timestamp_start_ms: self.chunk_start_ms.unwrap_or(0),
            timestamp_end_ms: self.stream_end_ms.unwrap_or(0),
        };
        let chunk = SealedChunk { nonce, envelope };
        self.buffer.clear();
        self.chunk_start_ms = None;
        self.next_index = next_index;
        self.pending = next_pending;
        self.sealed_total = next_total;
        Ok((chunk, facts))
    }

    /// Acknowledges that one sealed chunk reached the 008D commit path.
    ///
    /// # Errors
    ///
    /// Returns [`AccumError::NoPendingCommit`] when nothing is pending and
    /// [`AccumError::Closed`] when the session already closed.
    pub fn note_committed(&mut self) -> Result<u64, AccumError> {
        if self.closed {
            return Err(AccumError::Closed);
        }
        if self.pending == 0 {
            return Err(AccumError::NoPendingCommit);
        }
        self.pending -= 1;
        Ok(self.pending)
    }

    /// Closes the session after every byte is sealed and committed.
    ///
    /// # Errors
    ///
    /// Returns [`AccumError::BufferedRemainder`] while bytes await a seal and
    /// [`AccumError::PendingDrain`] while sealed chunks await commit, so close
    /// can never silently abandon audio.
    pub fn close(&mut self) -> Result<CloseSummary, AccumError> {
        if self.closed {
            return Err(AccumError::Closed);
        }
        if !self.buffer.is_empty() {
            return Err(AccumError::BufferedRemainder {
                bytes: self.buffer.len(),
            });
        }
        if self.pending > 0 {
            return Err(AccumError::PendingDrain {
                pending: self.pending,
            });
        }
        self.closed = true;
        Ok(CloseSummary {
            chunks_sealed: self.sealed_total,
            next_chunk_index: self.next_index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AccumConfig, AccumError, PayloadAccum};
    use crate::capture_checkpoint::ChunkCodec;
    use crate::capture_journal_commit::{ChunkJournal, SinkBinding};
    use crate::capture_windows_pressure::StorageBudgets;
    use crate::vault::{KeyGeneration, VaultId};
    use crate::vault_keys::OwnedKeyMaterial;
    use crate::vault_media_chunk::{
        MEDIA_CHUNK_NONCE_BYTES, MediaChunkContext, MediaChunkError, decrypt_media_chunk,
        encrypt_media_chunk,
    };
    use himsat_events::{SessionId, SourceId};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    const TEST_VRK: [u8; 32] = [0x5A; 32];
    const VAULT_BYTES: [u8; 16] = [0x22; 16];
    const SESSION: u128 = 0x0203_0405_0607_0809_0a0b_0c0d_0e0f_1011;
    const SOURCE: u128 = 0x1f1e_1d1c_1b1a_1918_1716_1514_1312_1110;

    const BUDGETS: StorageBudgets = StorageBudgets {
        warn_bytes: 1_000,
        critical_bytes: 100,
        queue_capacity: 8,
    };

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            let id = COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = std::env::temp_dir()
                .join(format!("himsat-payload-accum-{}-{id}", std::process::id()));
            fs::create_dir_all(&path).expect("accum temp dir");
            Self { path }
        }

        fn journal(&self) -> PathBuf {
            self.path.join("session.jrn")
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn binding() -> SinkBinding {
        SinkBinding {
            vault_id: VaultId::from_bytes(VAULT_BYTES),
            session: SessionId::new(SESSION),
            generation: KeyGeneration::new(3).expect("test generation is non-zero"),
        }
    }

    fn config() -> AccumConfig {
        AccumConfig {
            binding: binding(),
            codec: ChunkCodec::Pcm16,
            source: SourceId::new(SOURCE),
            max_chunk_bytes: 64,
            max_pending_chunks: 4,
            starting_index: 0,
            budgets: BUDGETS,
            cadence_millis: 1_000,
        }
    }

    fn nonce_for(index: u64) -> [u8; MEDIA_CHUNK_NONCE_BYTES] {
        let mut nonce = [0_u8; MEDIA_CHUNK_NONCE_BYTES];
        nonce[..8].copy_from_slice(&index.to_be_bytes());
        nonce
    }

    /// Seals with the real 005A path under a test-only VRK; production owners
    /// seal under the 004 lease with B203-reserved nonces instead.
    fn seal_with_test_vrk(
        context: MediaChunkContext,
        nonce: [u8; MEDIA_CHUNK_NONCE_BYTES],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, MediaChunkError> {
        encrypt_media_chunk(
            &OwnedKeyMaterial::from_bytes(TEST_VRK),
            context,
            nonce,
            plaintext,
        )
    }

    #[test]
    fn push_then_seal_round_trips_into_the_commit_path() {
        let dir = TempDir::new();
        let mut journal = ChunkJournal::create(&dir.journal(), binding()).expect("create");
        let mut accum = PayloadAccum::new(config()).expect("accum");
        let report = accum.push(b"aaabbb", 0, 25, 10_000, 0).expect("push");
        assert_eq!(report.accepted_bytes, 6);
        assert_eq!(report.buffered_bytes, 6);
        let index = accum.next_chunk_index();
        let (chunk, facts) = accum
            .seal_next(nonce_for(index), seal_with_test_vrk)
            .expect("seal");
        assert_eq!(facts.plaintext_len, 6);
        assert_eq!(facts.timestamp_start_ms, 0);
        assert_eq!(facts.timestamp_end_ms, 25);
        assert_eq!(facts.codec, ChunkCodec::Pcm16);
        assert_eq!(facts.source, SourceId::new(SOURCE));
        let commit = journal.commit(chunk.clone(), facts).expect("commit");
        assert_eq!(commit.chunk_index, 0);
        accum.note_committed().expect("acknowledge");
        assert_eq!(accum.pending_chunks(), 0);
        let plaintext = decrypt_media_chunk(
            &OwnedKeyMaterial::from_bytes(TEST_VRK),
            binding().chunk_context(0),
            &chunk.envelope,
        )
        .expect("sealed chunk decrypts");
        assert_eq!(plaintext, b"aaabbb");
    }

    #[test]
    fn admission_refusals_are_explicit_before_buffering() {
        let mut accum = PayloadAccum::new(config()).expect("accum");
        assert!(matches!(
            accum.push(b"aa", 0, 10, 50, 0),
            Err(AccumError::Refused(_))
        ));
        assert!(matches!(
            accum.push(b"aa", 0, 10, 10_000, 8),
            Err(AccumError::Refused(_))
        ));
        assert!(matches!(
            accum.push(b"aa", 0, 10, 50, 8),
            Err(AccumError::Refused(_))
        ));
        assert_eq!(accum.buffered_bytes(), 0);
        let report = accum.push(b"aa", 0, 10, 10_000, 0).expect("healthy push");
        assert_eq!(report.buffered_bytes, 2);
    }

    #[test]
    fn chunk_full_backpressure_never_drops_silently() {
        let mut accum = PayloadAccum::new(config()).expect("accum");
        accum
            .push(&[0x11; 60], 0, 10, 10_000, 0)
            .expect("first push");
        assert!(matches!(
            accum.push(&[0x22; 8], 10, 20, 10_000, 0),
            Err(AccumError::ChunkFull { .. })
        ));
        assert_eq!(accum.buffered_bytes(), 60);
        let index = accum.next_chunk_index();
        accum
            .seal_next(nonce_for(index), seal_with_test_vrk)
            .expect("seal drains the full buffer");
        accum.note_committed().expect("acknowledge");
        accum
            .push(&[0x22; 8], 10, 20, 10_000, 0)
            .expect("fits after seal");
        assert_eq!(accum.buffered_bytes(), 8);
    }

    #[test]
    fn timestamps_are_checked_and_never_regress() {
        let mut accum = PayloadAccum::new(config()).expect("accum");
        assert!(matches!(
            accum.push(b"aa", 20, 10, 10_000, 0),
            Err(AccumError::TimestampRange { .. })
        ));
        accum.push(b"aa", 0, 10, 10_000, 0).expect("first push");
        assert!(matches!(
            accum.push(b"bb", 5, 15, 10_000, 0),
            Err(AccumError::TimestampRegression { .. })
        ));
        assert_eq!(accum.buffered_bytes(), 2);
        accum
            .push(b"bb", 10, 15, 10_000, 0)
            .expect("contiguous push");
        assert_eq!(accum.buffered_bytes(), 4);
    }

    #[test]
    fn seal_due_follows_size_and_cadence() {
        let mut accum = PayloadAccum::new(config()).expect("accum");
        assert!(!accum.seal_due(9_999));
        accum.push(b"aa", 0, 10, 10_000, 0).expect("buffered push");
        assert!(!accum.seal_due(999));
        assert!(accum.seal_due(1_000));
        accum
            .push(&[0x33; 62], 10, 20, 10_000, 0)
            .expect("size bound");
        assert!(accum.seal_due(0));
    }

    #[test]
    fn close_demands_an_empty_buffer_and_a_drained_queue() {
        let mut accum = PayloadAccum::new(config()).expect("accum");
        accum.push(b"aa", 0, 10, 10_000, 0).expect("push");
        assert!(matches!(
            accum.close(),
            Err(AccumError::BufferedRemainder { bytes: 2 })
        ));
        let index = accum.next_chunk_index();
        accum
            .seal_next(nonce_for(index), seal_with_test_vrk)
            .expect("seal");
        assert!(matches!(
            accum.close(),
            Err(AccumError::PendingDrain { pending: 1 })
        ));
        accum.note_committed().expect("acknowledge");
        let summary = accum.close().expect("clean close");
        assert_eq!(summary.chunks_sealed, 1);
        assert_eq!(summary.next_chunk_index, 1);
        assert!(matches!(
            accum.push(b"zz", 10, 20, 10_000, 0),
            Err(AccumError::Closed)
        ));
    }

    #[test]
    fn a_lying_sealer_is_refused_without_losing_bytes() {
        let mut accum = PayloadAccum::new(config()).expect("accum");
        accum.push(b"aaabbb", 0, 25, 10_000, 0).expect("push");
        let index = accum.next_chunk_index();
        assert!(matches!(
            accum.seal_next(nonce_for(index), |context, _, plaintext| {
                seal_with_test_vrk(context, nonce_for(999), plaintext)
            }),
            Err(AccumError::NonceMismatch)
        ));
        assert!(matches!(
            accum.seal_next(nonce_for(index), |_, _, _| Ok(vec![0x00, 0x01])),
            Err(AccumError::SealFailed(_))
        ));
        assert_eq!(accum.buffered_bytes(), 6);
        assert_eq!(accum.next_chunk_index(), index);
        accum
            .seal_next(nonce_for(index), seal_with_test_vrk)
            .expect("honest retry keeps the bytes");
    }

    #[test]
    fn every_sealed_envelope_decrypts_and_indexes_stay_contiguous() {
        let mut tight = PayloadAccum::new(AccumConfig {
            max_pending_chunks: 2,
            ..config()
        })
        .expect("accum");
        for chunk_index in 0..2_u64 {
            tight
                .push(
                    b"payload",
                    chunk_index * 10,
                    chunk_index * 10 + 10,
                    10_000,
                    0,
                )
                .expect("push");
            let (chunk, facts) = tight
                .seal_next(nonce_for(chunk_index), seal_with_test_vrk)
                .expect("seal");
            assert_eq!(facts.plaintext_len, 7);
            let plaintext = decrypt_media_chunk(
                &OwnedKeyMaterial::from_bytes(TEST_VRK),
                binding().chunk_context(chunk_index),
                &chunk.envelope,
            )
            .expect("chunk decrypts");
            assert_eq!(plaintext, b"payload");
        }
        assert!(matches!(
            tight.push(b"third", 20, 30, 10_000, 0),
            Err(AccumError::PendingFull { .. })
        ));
        assert!(matches!(
            tight.seal_next(nonce_for(2), seal_with_test_vrk),
            Err(AccumError::PendingFull { .. })
        ));
        tight.note_committed().expect("drain one");
        tight
            .push(b"third", 20, 30, 10_000, 0)
            .expect("fits after drain");
        tight
            .seal_next(nonce_for(2), seal_with_test_vrk)
            .expect("index continues contiguously");
        assert_eq!(tight.next_chunk_index(), 3);
        assert!(matches!(
            tight.note_committed().and(tight.note_committed()),
            Ok(0)
        ));
        assert!(matches!(
            tight.note_committed(),
            Err(AccumError::NoPendingCommit)
        ));
    }

    #[test]
    fn resume_continues_from_the_replayed_prefix() {
        let mut resumed = PayloadAccum::new(AccumConfig {
            starting_index: 41,
            ..config()
        })
        .expect("accum");
        assert_eq!(resumed.next_chunk_index(), 41);
        resumed.push(b"aa", 900, 910, 10_000, 0).expect("push");
        let (chunk, facts) = resumed
            .seal_next(nonce_for(41), seal_with_test_vrk)
            .expect("seal");
        assert_eq!(facts.plaintext_len, 2);
        let plaintext = decrypt_media_chunk(
            &OwnedKeyMaterial::from_bytes(TEST_VRK),
            binding().chunk_context(41),
            &chunk.envelope,
        )
        .expect("resumed chunk decrypts under its own context");
        assert_eq!(plaintext, b"aa");
        assert_eq!(resumed.next_chunk_index(), 42);
    }
}
