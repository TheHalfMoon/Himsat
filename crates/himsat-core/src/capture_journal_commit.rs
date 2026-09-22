//! 008D durable commit path for sealed capture chunks.
//!
//! First half of the Spec 008 journal-sink wiring: a sealed 005A chunk
//! envelope plus the adapter facts that accompany it become exactly one 005B
//! commit record and one 006C metadata entry, on the owner's drain thread.
//! The second half (accumulating captured payloads into sealed chunks under
//! the owner's sealer, admission policy, and checkpoint cadence) is
//! deliberately a separate bounded grain and is not implemented here.
//!
//! Semantics this module fixes, and why:
//!
//! - ordering: chunk indexes are a contiguous 0-based prefix per session and
//!   advance only after a successful commit, so a resumed journal can never
//!   re-commit an index;
//! - no silent failure: a refusal or filesystem failure is a typed error that
//!   leaves the journal untouched and poisons the handle, because the closed
//!   005B contract forbids retrying a failed append into a duplicate sequence;
//! - binding fidelity: the commit quotes the nonce the envelope carries, the
//!   plaintext length the envelope's own length implies, and the SHA-256 of
//!   the exact envelope bytes the owner stored;
//! - no key material, no clock, no path invention: the owner seals (005A under
//!   the 004 lease and the B203 nonce lifecycle), stamps are caller facts, and
//!   the journal path stays a caller argument under the 004 placement rules.
//!
//! What this module cannot check is stated honestly: the closed 005A public
//! parser proves the envelope's structure and its quoted nonce, but the
//! vault/session/index binding and the plaintext itself are authenticated when
//! the chunk is decrypted, which is 005C reconciliation's job.
//!
//! Donor posture: no donor code is copied; the shape is Himsat-native over
//! already-closed Himsat contracts.

use crate::capture_checkpoint::{ChunkCodec, ChunkMetadata};
use crate::vault::{KeyGeneration, VaultId};
use crate::vault_media_chunk::{
    MEDIA_CHUNK_MIN_ENVELOPE_BYTES, MEDIA_CHUNK_NONCE_BYTES, MediaChunkContext, MediaChunkError,
    media_chunk_nonce,
};
use crate::vault_media_journal::{JournalError, JournalWriter, envelope_digest, replay_journal};
use himsat_events::{SessionId, SourceId};
use std::error::Error;
use std::fmt;
use std::path::Path;

/// Vault/session binding for one journal instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SinkBinding {
    /// Vault the journal file and every quoted envelope belong to.
    pub vault_id: VaultId,
    /// Session the journal file and every quoted envelope belong to.
    pub session: SessionId,
    /// Key generation every commit of this session quotes.
    pub generation: KeyGeneration,
}

impl SinkBinding {
    /// Returns the exact 16 session-identity bytes the 005 codecs bind.
    #[must_use]
    pub fn session_bytes(self) -> [u8; 16] {
        self.session.get().to_be_bytes()
    }

    /// Builds the 005A context for one chunk index of this binding.
    #[must_use]
    pub fn chunk_context(self, chunk_index: u64) -> MediaChunkContext {
        MediaChunkContext::new(
            self.vault_id,
            self.session_bytes(),
            chunk_index,
            self.generation,
        )
    }
}

/// One sealed chunk as the owner's sealer produced it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SealedChunk {
    /// Nonce the owner used; quoted into the commit, never generated here.
    pub nonce: [u8; MEDIA_CHUNK_NONCE_BYTES],
    /// Exact 005A envelope bytes the owner stored for this chunk.
    pub envelope: Vec<u8>,
}

/// Adapter facts that accompany one sealed chunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkFacts {
    /// Plaintext bytes the chunk carried, as the owner counted them.
    pub plaintext_len: u64,
    /// Portable codec tag the owner already assigned (006C vocabulary).
    pub codec: ChunkCodec,
    /// 003 source identity the chunk came from.
    pub source: SourceId,
    /// Session-relative start stamp of the chunk.
    pub timestamp_start_ms: u64,
    /// Session-relative end stamp of the chunk.
    pub timestamp_end_ms: u64,
}

/// Fail-closed commit errors.
#[derive(Debug)]
pub enum CommitError {
    /// The 005B journal refused or failed the operation.
    Journal(JournalError),
    /// The envelope is not one the closed 005A parser accepts.
    Envelope(MediaChunkError),
    /// The quoted nonce differs from the nonce the envelope carries.
    NonceMismatch,
    /// The quoted plaintext length disagrees with the envelope's own length.
    LengthMismatch {
        envelope_bytes: usize,
        plaintext_len: u64,
    },
    /// The adapter facts carry an inverted timestamp range.
    TimestampRange { start_ms: u64, end_ms: u64 },
    /// Resume refused: replayed commit indexes are not `0, 1, 2, ...`.
    IndexSequence { expected: u64, found: u64 },
    /// Chunk indexes exhausted the `u64` space.
    IndexExhausted,
    /// An earlier failure poisoned this handle; no further commit is accepted.
    Poisoned,
    /// The session is already closed.
    Closed,
}

impl fmt::Display for CommitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Journal(_) => "capture journal commit failed at the media journal",
            Self::Envelope(_) => "sealed chunk envelope is not a canonical 005A envelope",
            Self::NonceMismatch => "quoted nonce is not the nonce inside the envelope",
            Self::LengthMismatch { .. } => "quoted plaintext length disagrees with the envelope",
            Self::TimestampRange { .. } => "chunk timestamp range is inverted",
            Self::IndexSequence { .. } => "journal commit indexes are not a contiguous prefix",
            Self::IndexExhausted => "chunk index space is exhausted",
            Self::Poisoned => "capture journal commit handle is poisoned by an earlier failure",
            Self::Closed => "capture journal commit session already closed",
        };
        f.write_str(message)
    }
}

impl Error for CommitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Journal(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<JournalError> for CommitError {
    fn from(value: JournalError) -> Self {
        Self::Journal(value)
    }
}

/// What one commit did to the journal.
///
/// The 006C metadata is not repeated here: every committed chunk is appended
/// to [`ChunkJournal::metadata`] in commit order, and `chunk_index` addresses
/// the entry this report describes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommitReport {
    /// Session-scoped chunk index that was committed.
    pub chunk_index: u64,
    /// Journal sequence number the 005B writer returned.
    pub journal_seq: u64,
    /// SHA-256 of the exact envelope bytes the commit quotes.
    pub digest: [u8; 32],
}

/// Result of closing a session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosedJournal {
    /// Commit records the session appended.
    pub commits: u64,
    /// Journal sequence number of the close marker.
    pub close_seq: u64,
    /// 006C metadata of every chunk this instance committed.
    pub metadata: Vec<ChunkMetadata>,
}

/// Drain-side writer that turns sealed chunks into 005 commits.
///
/// Drive it from the owner's non-real-time thread: `commit` per sealed chunk
/// and `close` at session stop. One instance owns exactly one session journal
/// file, matching the 005B single-writer contract.
pub struct ChunkJournal {
    writer: JournalWriter,
    binding: SinkBinding,
    next_chunk_index: u64,
    commits: u64,
    metadata: Vec<ChunkMetadata>,
    poisoned: bool,
    closed: bool,
}

impl ChunkJournal {
    /// Creates a session journal file and appends its open marker.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::Journal`] when 005B refuses creation, for
    /// example because the file already holds a live session.
    pub fn create(path: &Path, binding: SinkBinding) -> Result<Self, CommitError> {
        let writer = JournalWriter::create(
            path,
            binding.vault_id,
            binding.session_bytes(),
            binding.generation,
        )?;
        Ok(Self {
            writer,
            binding,
            next_chunk_index: 0,
            commits: 0,
            metadata: Vec::new(),
            poisoned: false,
            closed: false,
        })
    }

    /// Resumes a cleanly tailed session journal for further commits.
    ///
    /// The 005B refusals (torn tail, binding mismatch, closed session, empty
    /// file) surface as [`CommitError::Journal`]. The replayed commit indexes
    /// must be the contiguous prefix `0, 1, 2, ...`, otherwise the handle
    /// refuses with [`CommitError::IndexSequence`] rather than risk a
    /// duplicate.
    ///
    /// # Errors
    ///
    /// Returns the refusals above.
    pub fn resume(path: &Path, binding: SinkBinding) -> Result<Self, CommitError> {
        let writer = JournalWriter::resume(path, binding.vault_id, binding.session_bytes())?;
        let replay = replay_journal(path)?;
        let mut commits = 0_u64;
        let mut next_chunk_index = 0_u64;
        for record in replay.records() {
            if let Some(commit) = record.kind().as_commit() {
                if commit.chunk_index != next_chunk_index {
                    return Err(CommitError::IndexSequence {
                        expected: next_chunk_index,
                        found: commit.chunk_index,
                    });
                }
                let Some(next_commits) = commits.checked_add(1) else {
                    return Err(CommitError::IndexExhausted);
                };
                let Some(next_index) = next_chunk_index.checked_add(1) else {
                    return Err(CommitError::IndexExhausted);
                };
                commits = next_commits;
                next_chunk_index = next_index;
            }
        }
        Ok(Self {
            writer,
            binding,
            next_chunk_index,
            commits,
            metadata: Vec::new(),
            poisoned: false,
            closed: false,
        })
    }

    /// Returns the binding this handle was created with.
    #[must_use]
    pub const fn binding(&self) -> SinkBinding {
        self.binding
    }

    /// Returns how many commits this session has appended.
    #[must_use]
    pub const fn commit_count(&self) -> u64 {
        self.commits
    }

    /// Returns the index the next committed chunk will carry.
    #[must_use]
    pub const fn next_chunk_index(&self) -> u64 {
        self.next_chunk_index
    }

    /// Returns the 006C metadata of every committed chunk, in commit order.
    #[must_use]
    pub fn metadata(&self) -> &[ChunkMetadata] {
        &self.metadata
    }

    /// Reports whether an earlier failure poisoned this handle.
    #[must_use]
    pub const fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    /// Commits one sealed chunk and records its 006C metadata.
    ///
    /// The envelope is validated against the closed 005A parser, its quoted
    /// nonce and its implied plaintext length are checked against the adapter
    /// facts, and only then is the commit appended. A refused chunk leaves the
    /// journal exactly as it was.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal for a malformed envelope, a nonce or length
    /// disagreement, or inverted timestamps, and [`CommitError::Journal`] when
    /// 005B refuses the append. A 005B failure poisons the handle.
    pub fn commit(
        &mut self,
        chunk: SealedChunk,
        facts: ChunkFacts,
    ) -> Result<CommitReport, CommitError> {
        self.ensure_usable()?;
        if facts.timestamp_end_ms < facts.timestamp_start_ms {
            return Err(CommitError::TimestampRange {
                start_ms: facts.timestamp_start_ms,
                end_ms: facts.timestamp_end_ms,
            });
        }
        let quoted = match media_chunk_nonce(&chunk.envelope) {
            Ok(quoted) => quoted,
            Err(error) => return Err(CommitError::Envelope(error)),
        };
        if quoted != chunk.nonce {
            return Err(CommitError::NonceMismatch);
        }
        let implied = facts
            .plaintext_len
            .checked_add(MEDIA_CHUNK_MIN_ENVELOPE_BYTES as u64);
        if implied != Some(chunk.envelope.len() as u64) {
            return Err(CommitError::LengthMismatch {
                envelope_bytes: chunk.envelope.len(),
                plaintext_len: facts.plaintext_len,
            });
        }
        let chunk_index = self.next_chunk_index;
        let Some(next_index) = chunk_index.checked_add(1) else {
            return self.poison(CommitError::IndexExhausted);
        };
        let Some(next_commits) = self.commits.checked_add(1) else {
            return self.poison(CommitError::IndexExhausted);
        };
        let digest = envelope_digest(&chunk.envelope);
        let journal_seq = match self.writer.append_commit(
            chunk_index,
            self.binding.generation,
            chunk.nonce,
            facts.plaintext_len,
            digest,
        ) {
            Ok(seq) => seq,
            Err(error) => return self.poison(CommitError::Journal(error)),
        };
        self.commits = next_commits;
        self.next_chunk_index = next_index;
        self.metadata.push(ChunkMetadata {
            session: self.binding.session,
            chunk_index,
            generation: self.binding.generation,
            nonce: chunk.nonce,
            plaintext_len: facts.plaintext_len,
            digest,
            timestamp_start_ms: facts.timestamp_start_ms,
            timestamp_end_ms: facts.timestamp_end_ms,
            codec: facts.codec,
            source: facts.source,
        });
        Ok(CommitReport {
            chunk_index,
            journal_seq,
            digest,
        })
    }

    /// Appends the close marker and returns the session summary.
    ///
    /// The close marker carries exactly the commit count this instance
    /// appended, which is what 005C checks against the replayed journal.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::Closed`] when the session already closed, and
    /// [`CommitError::Journal`] when 005B refuses the append.
    pub fn close(&mut self) -> Result<ClosedJournal, CommitError> {
        self.ensure_usable()?;
        let commits = self.commits;
        let close_seq = match self.writer.append_close(commits) {
            Ok(seq) => seq,
            Err(error) => return self.poison(CommitError::Journal(error)),
        };
        self.closed = true;
        Ok(ClosedJournal {
            commits,
            close_seq,
            metadata: std::mem::take(&mut self.metadata),
        })
    }

    fn ensure_usable(&self) -> Result<(), CommitError> {
        if self.poisoned {
            return Err(CommitError::Poisoned);
        }
        if self.closed {
            return Err(CommitError::Closed);
        }
        Ok(())
    }

    fn poison<T>(&mut self, error: CommitError) -> Result<T, CommitError> {
        self.poisoned = true;
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChunkFacts, ChunkJournal, CommitError, SealedChunk, SinkBinding};
    use crate::capture_checkpoint::ChunkCodec;
    use crate::vault::{KeyGeneration, VaultId};
    use crate::vault_keys::OwnedKeyMaterial;
    use crate::vault_media_chunk::{
        MEDIA_CHUNK_NONCE_BYTES, decrypt_media_chunk, encrypt_media_chunk,
    };
    use crate::vault_media_journal::{JournalWriter, TailStatus, envelope_digest, replay_journal};
    use himsat_events::{SessionId, SourceId};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    const TEST_VRK: [u8; 32] = [0x5A; 32];
    const VAULT_BYTES: [u8; 16] = [0x11; 16];
    const SESSION: u128 = 0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10;
    const SOURCE: u128 = 0x0f0e_0d0c_0b0a_0908_0706_0504_0302_0100;

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            let id = COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = std::env::temp_dir()
                .join(format!("himsat-chunk-journal-{}-{id}", std::process::id()));
            fs::create_dir_all(&path).expect("journal temp dir");
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

    fn facts(plaintext_len: u64) -> ChunkFacts {
        ChunkFacts {
            plaintext_len,
            codec: ChunkCodec::Pcm16,
            source: SourceId::new(SOURCE),
            timestamp_start_ms: 0,
            timestamp_end_ms: 25,
        }
    }

    /// Seals with the real 005A path, so every commit under test is bound to a
    /// real envelope; the nonce is the chunk index and is therefore never
    /// reused by a resumed handle.
    fn sealed(chunk_index: u64, plaintext: &[u8]) -> SealedChunk {
        let mut nonce = [0_u8; MEDIA_CHUNK_NONCE_BYTES];
        nonce[..8].copy_from_slice(&chunk_index.to_be_bytes());
        SealedChunk {
            nonce,
            envelope: encrypt_media_chunk(
                &OwnedKeyMaterial::from_bytes(TEST_VRK),
                binding().chunk_context(chunk_index),
                nonce,
                plaintext,
            )
            .expect("fixture chunk encrypts"),
        }
    }

    #[test]
    fn a_commit_quotes_the_exact_envelope_and_records_metadata() {
        let dir = TempDir::new();
        let mut journal = ChunkJournal::create(&dir.journal(), binding()).expect("create");
        let chunk = sealed(0, b"aaabbb");
        let report = journal.commit(chunk.clone(), facts(6)).expect("commit");
        assert_eq!(report.chunk_index, 0);
        assert_eq!(report.journal_seq, 2);
        assert_eq!(report.digest, envelope_digest(&chunk.envelope));
        assert_eq!(journal.commit_count(), 1);
        assert_eq!(journal.next_chunk_index(), 1);
        assert_eq!(journal.binding(), binding());
        let metadata = journal.metadata()[0];
        assert_eq!(metadata.chunk_index, 0);
        assert_eq!(metadata.nonce, chunk.nonce);
        assert_eq!(metadata.plaintext_len, 6);
        assert_eq!(metadata.digest, report.digest);
        assert_eq!(metadata.codec, ChunkCodec::Pcm16);
        assert_eq!(metadata.source, SourceId::new(SOURCE));
        assert_eq!(metadata.timestamp_end_ms, 25);
        let replay = replay_journal(&dir.journal()).expect("replay");
        let commit = replay.records()[1]
            .kind()
            .as_commit()
            .expect("the second record is a commit");
        assert_eq!(commit.chunk_index, 0);
        assert_eq!(commit.nonce, chunk.nonce);
        assert_eq!(commit.plaintext_len, 6);
        assert_eq!(commit.envelope_digest, report.digest);
    }

    #[test]
    fn envelope_guards_refuse_before_the_journal_is_touched() {
        let dir = TempDir::new();
        let mut journal = ChunkJournal::create(&dir.journal(), binding()).expect("create");
        let malformed = SealedChunk {
            nonce: [0x01; MEDIA_CHUNK_NONCE_BYTES],
            envelope: vec![0x00, 0x01, 0x02],
        };
        assert!(matches!(
            journal.commit(malformed, facts(6)),
            Err(CommitError::Envelope(_))
        ));
        let mut mismatched = sealed(0, b"aaabbb");
        mismatched.nonce = [0x03; MEDIA_CHUNK_NONCE_BYTES];
        assert!(matches!(
            journal.commit(mismatched, facts(6)),
            Err(CommitError::NonceMismatch)
        ));
        assert!(matches!(
            journal.commit(sealed(0, b"aaabbb"), facts(5)),
            Err(CommitError::LengthMismatch {
                envelope_bytes: 135,
                plaintext_len: 5,
            })
        ));
        let mut inverted = facts(6);
        inverted.timestamp_start_ms = 25;
        inverted.timestamp_end_ms = 24;
        assert!(matches!(
            journal.commit(sealed(0, b"aaabbb"), inverted),
            Err(CommitError::TimestampRange {
                start_ms: 25,
                end_ms: 24,
            })
        ));
        assert_eq!(journal.commit_count(), 0);
        assert!(journal.metadata().is_empty());
        assert!(!journal.is_poisoned());
        let replay = replay_journal(&dir.journal()).expect("replay");
        assert_eq!(replay.records().len(), 1);
    }

    #[test]
    fn close_marks_the_exact_commit_count_and_replays_clean() {
        let dir = TempDir::new();
        let mut journal = ChunkJournal::create(&dir.journal(), binding()).expect("create");
        journal.commit(sealed(0, b"aaa"), facts(3)).expect("first");
        journal
            .commit(sealed(1, b"bbbb"), facts(4))
            .expect("second");
        let closed = journal.close().expect("close");
        assert_eq!(closed.commits, 2);
        assert_eq!(closed.close_seq, 4);
        assert_eq!(closed.metadata.len(), 2);
        assert!(matches!(journal.close(), Err(CommitError::Closed)));
        let replay = replay_journal(&dir.journal()).expect("replay");
        assert_eq!(replay.tail(), TailStatus::CleanEof);
        assert_eq!(replay.records().len(), 4);
        let commits = replay
            .records()
            .iter()
            .filter(|record| record.kind().as_commit().is_some())
            .count();
        let claimed = replay
            .records()
            .last()
            .and_then(|record| record.kind().as_close_count());
        assert_eq!(claimed, Some(commits as u64));
    }

    #[test]
    fn every_committed_envelope_still_decrypts_to_its_plaintext() {
        let dir = TempDir::new();
        let mut journal = ChunkJournal::create(&dir.journal(), binding()).expect("create");
        let plaintexts: [&[u8]; 2] = [b"aaabbb", b"ccccc"];
        for (index, plaintext) in plaintexts.iter().enumerate() {
            let chunk = sealed(index as u64, plaintext);
            fs::write(dir.path.join(format!("chunk-{index}.bin")), &chunk.envelope)
                .expect("store envelope");
            journal
                .commit(chunk, facts(plaintext.len() as u64))
                .expect("commit");
        }
        journal.close().expect("close");
        let replay = replay_journal(&dir.journal()).expect("replay");
        assert_eq!(replay.tail(), TailStatus::CleanEof);
        for record in replay.records() {
            let Some(commit) = record.kind().as_commit() else {
                continue;
            };
            let envelope =
                fs::read(dir.path.join(format!("chunk-{}.bin", commit.chunk_index))).expect("file");
            assert_eq!(commit.envelope_digest, envelope_digest(&envelope));
            let recovered = decrypt_media_chunk(
                &OwnedKeyMaterial::from_bytes(TEST_VRK),
                binding().chunk_context(commit.chunk_index),
                &envelope,
            )
            .expect("the committed envelope decrypts");
            assert_eq!(recovered, plaintexts[commit.chunk_index as usize]);
        }
    }

    #[test]
    fn journal_refusals_surface_as_typed_errors() {
        let dir = TempDir::new();
        let journal_path = dir.journal();
        let mut journal = ChunkJournal::create(&journal_path, binding()).expect("create");
        journal.commit(sealed(0, b"aaa"), facts(3)).expect("commit");
        drop(journal);
        assert!(matches!(
            ChunkJournal::create(&journal_path, binding()),
            Err(CommitError::Journal(_))
        ));
        let empty = TempDir::new();
        assert!(matches!(
            ChunkJournal::resume(&empty.journal(), binding()),
            Err(CommitError::Journal(_))
        ));
    }

    #[test]
    fn resume_continues_the_index_and_refuses_a_gapped_prefix() {
        let dir = TempDir::new();
        let journal_path = dir.journal();
        {
            let mut journal = ChunkJournal::create(&journal_path, binding()).expect("create");
            journal.commit(sealed(0, b"aaa"), facts(3)).expect("commit");
            assert_eq!(journal.commit_count(), 1);
        }
        let mut resumed = ChunkJournal::resume(&journal_path, binding()).expect("resume");
        assert_eq!(resumed.binding(), binding());
        assert_eq!(resumed.commit_count(), 1);
        assert_eq!(resumed.next_chunk_index(), 1);
        resumed
            .commit(sealed(1, b"bbb"), facts(3))
            .expect("second chunk");
        assert_eq!(resumed.close().expect("close").commits, 2);

        let gapped = TempDir::new();
        let gapped_path = gapped.journal();
        {
            let mut writer = JournalWriter::create(
                &gapped_path,
                binding().vault_id,
                binding().session_bytes(),
                binding().generation,
            )
            .expect("writer");
            writer
                .append_commit(0, binding().generation, [0; 24], 4, [1; 32])
                .expect("first commit");
            writer
                .append_commit(2, binding().generation, [0; 24], 4, [1; 32])
                .expect("gapped commit");
        }
        assert!(matches!(
            ChunkJournal::resume(&gapped_path, binding()),
            Err(CommitError::IndexSequence {
                expected: 1,
                found: 2,
            })
        ));
    }

    #[test]
    fn a_poisoned_handle_refuses_further_work_instead_of_retrying() {
        let dir = TempDir::new();
        let mut journal = ChunkJournal::create(&dir.journal(), binding()).expect("create");
        let latched = journal.poison::<()>(CommitError::IndexExhausted);
        assert!(matches!(latched, Err(CommitError::IndexExhausted)));
        assert!(journal.is_poisoned());
        assert!(matches!(
            journal.commit(sealed(0, b"aaa"), facts(3)),
            Err(CommitError::Poisoned)
        ));
        assert!(matches!(journal.close(), Err(CommitError::Poisoned)));
        let replay = replay_journal(&dir.journal()).expect("replay");
        assert_eq!(replay.records().len(), 1);
    }
}
