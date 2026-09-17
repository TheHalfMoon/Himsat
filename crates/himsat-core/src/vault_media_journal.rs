//! Session-journal append path and replay for Specification 005B.
//!
//! This module executes the 005B record layout, file-append discipline, and
//! machine replay: fixed session open/close markers, chunk-commit records
//! quoting public 005A envelope fields, a `JournalWriter` that appends one
//! caller-owned file per session with a `sync_all` plus length check after
//! every record, truncation-safe replay returning the exact valid
//! prefix plus a machine `TailStatus`, and crash-resume that refuses torn
//! tails without mutating them. Recovery reconciliation stays with 005C.
//!
//! Additive-only 004/005A boundary discipline:
//!
//! - no reviewed 004 byte and no 005A byte is changed;
//! - chunk keys, nonces, and AEAD behavior stay exactly where 005A/B203 put
//!   them; commit records only quote public 005A header fields (vault,
//!   session, index, generation, nonce, plaintext length) plus the SHA-256
//!   digest of the exact envelope bytes;
//! - no new KDF, primitive, key domain, or dependency (`sha2` is already in
//!   the crate closure; no adoption gate is triggered).
//!
//! Deliberately out of scope here:
//!
//! - recovery reconciliation against the manifest inventory (005C);
//! - fault-injection harnesses and bounded-loss quantification (005D);
//! - nonce reservation: records quote the 24-byte nonce so a later log can
//!   be built on this codec, but nothing here generates nonces.
//!
//! Integrity model, stated honestly: the per-record SHA-256 is crash-tear
//! detection, not a security boundary. Forgery resistance for chunk bytes
//! comes from the 005A AEAD envelope at recovery time. No secret, key, or
//! plaintext audio ever enters a journal record.
//!
//! Durability model, stated honestly: durability is exactly the host
//! `sync_all` guarantee per record. No `F_FULLFSYNC`,
//! directory-entry-durability, or power-loss-proof claim is made beyond a
//! successful `sync_all` return. One file holds exactly one session; the
//! caller owns placement and opaque naming per the 004 metadata rules.
//!
//! Donor posture: Meetily/Anarlog/OpenSuperWhisper journaling and
//! file-queue patterns are planning inputs only; no donor journal fits the
//! AEAD-bound truncation discipline, so this codec is Himsat-native and
//! adopts no donor code.

use crate::vault::{KeyGeneration, VaultId};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

/// Journal magic: `00 0B` length-prefixed `HIMSAT/JRN1` (13 bytes total).
pub const JOURNAL_MAGIC: &[u8; 13] = b"\x00\x0BHIMSAT/JRN1";
/// Exact journal codec version reviewed in 005B.
pub const JOURNAL_VERSION: u16 = 1;
/// Record type: session open marker.
pub const JOURNAL_TYPE_OPEN: u16 = 1;
/// Record type: chunk-commit record.
pub const JOURNAL_TYPE_COMMIT: u16 = 2;
/// Record type: session close marker.
pub const JOURNAL_TYPE_CLOSE: u16 = 3;
/// Exact fixed header size: magic(13) + version(2) + type(2) + seq(8) +
/// vault(16) + session(16) + payload_len(4) = 61 bytes.
pub const JOURNAL_HEADER_BYTES: usize = 61;
/// Exact trailing integrity size: SHA-256 over magic..payload.
pub const JOURNAL_INTEGRITY_BYTES: usize = 32;
/// Exact open payload size: generation u64.
pub const JOURNAL_OPEN_PAYLOAD_BYTES: usize = 8;
/// Exact commit payload size: index(8) + generation(8) + nonce(24) +
/// plaintext_len(8) + envelope digest(32) = 80 bytes.
pub const JOURNAL_COMMIT_PAYLOAD_BYTES: usize = 80;
/// Exact close payload size: commit_count u64.
pub const JOURNAL_CLOSE_PAYLOAD_BYTES: usize = 8;
const VERSION_OFFSET: usize = 13;
const TYPE_OFFSET: usize = 15;
const SEQ_OFFSET: usize = 17;
const VAULT_OFFSET: usize = 25;
const SESSION_OFFSET: usize = 41;
const PAYLOAD_LEN_OFFSET: usize = 57;

/// Computes the SHA-256 digest of exact 005A envelope bytes for commit records.
///
/// Single-sourced recipe so capture, journal, and 005C recovery agree on the
/// binding without reimplementing it at each site.
#[must_use]
pub fn envelope_digest(envelope: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(envelope);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// One parsed journal record with its sequence number and session binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JournalRecord {
    seq: u64,
    vault_id: VaultId,
    session_id: [u8; 16],
    kind: JournalRecordKind,
}

/// Typed journal record payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalRecordKind {
    /// Session open marker carrying the vault key generation.
    Open { generation: KeyGeneration },
    /// Chunk-commit record quoting public 005A envelope fields.
    Commit {
        chunk_index: u64,
        generation: KeyGeneration,
        nonce: [u8; 24],
        plaintext_len: u64,
        envelope_digest: [u8; 32],
    },
    /// Session close marker carrying the writer's commit count.
    Close { commit_count: u64 },
}

/// Public view of a chunk-commit payload for cross-module consumers
/// (recovery reconciliation). All fields are public 005A header values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommitView {
    /// Session-scoped chunk index.
    pub chunk_index: u64,
    /// Vault key generation active at commit time.
    pub generation: KeyGeneration,
    /// Quoted 005A nonce (B203-owned, never generated here).
    pub nonce: [u8; 24],
    /// Quoted plaintext length.
    pub plaintext_len: u64,
    /// SHA-256 of the exact 005A envelope bytes.
    pub envelope_digest: [u8; 32],
}

impl JournalRecordKind {
    /// Returns the commit view for commit records, `None` otherwise.
    #[must_use]
    pub const fn as_commit(self) -> Option<CommitView> {
        match self {
            Self::Commit {
                chunk_index,
                generation,
                nonce,
                plaintext_len,
                envelope_digest,
            } => Some(CommitView {
                chunk_index,
                generation,
                nonce,
                plaintext_len,
                envelope_digest,
            }),
            _ => None,
        }
    }

    /// Returns the generation for open markers, `None` otherwise.
    #[must_use]
    pub const fn as_open_generation(self) -> Option<KeyGeneration> {
        match self {
            Self::Open { generation } => Some(generation),
            _ => None,
        }
    }

    /// Returns the commit count for close markers, `None` otherwise.
    #[must_use]
    pub const fn as_close_count(self) -> Option<u64> {
        match self {
            Self::Close { commit_count } => Some(commit_count),
            _ => None,
        }
    }
}

impl JournalRecord {
    /// Returns the 1-based monotonic sequence number of this record.
    #[must_use]
    pub const fn seq(self) -> u64 {
        self.seq
    }

    /// Returns the vault identity bound into every record of this file.
    #[must_use]
    pub const fn vault_id(self) -> VaultId {
        self.vault_id
    }

    /// Returns the session identity bound into every record of this file.
    #[must_use]
    pub const fn session_id(self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the typed payload of this record.
    #[must_use]
    pub const fn kind(self) -> JournalRecordKind {
        self.kind
    }
}

/// Machine reason a journal tail stopped the replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TailReason {
    /// Header bytes missing before the fixed 61-byte header completed.
    TruncatedHeader,
    /// Magic prefix is not the exact journal v1 magic.
    InvalidMagic,
    /// Codec version is not the reviewed 005B value.
    UnsupportedVersion,
    /// Record type is not open/commit/close.
    UnsupportedType,
    /// Declared payload length differs from the exact type length.
    LengthMismatch,
    /// Key generation field is zero.
    InvalidGeneration,
    /// File ends inside the declared payload.
    TruncatedPayload,
    /// SHA-256 over magic..payload does not match the stored integrity.
    IntegrityMismatch,
    /// File ends inside the 32-byte integrity trailer.
    TruncatedIntegrity,
    /// Sequence number is not exactly one past the previous record.
    SequenceGap,
    /// Vault/session differs from the file's first record binding.
    ContextMismatch,
    /// First record is not a session open marker.
    FirstRecordNotOpen,
    /// Open marker appears after the first record.
    UnexpectedOpen,
    /// A record follows a session close marker in a single-session file.
    RecordAfterClose,
}

/// Tail state of a journal replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TailStatus {
    /// Every byte parsed into valid records; `valid_bytes` equals file length.
    CleanEof,
    /// Replay stopped at `valid_bytes`; the valid prefix is returned and no
    /// partial record is applied.
    TornTail {
        /// Exact valid prefix length in bytes.
        valid_bytes: usize,
        /// Machine reason the replay stopped.
        reason: TailReason,
    },
}

/// Read-only result of a journal replay: valid prefix plus tail state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalReplay {
    records: Vec<JournalRecord>,
    tail: TailStatus,
}

impl JournalReplay {
    /// Returns the valid records in file order.
    #[must_use]
    pub fn records(&self) -> &[JournalRecord] {
        &self.records
    }

    /// Returns the tail state (`CleanEof` or the torn-tail stop reason).
    #[must_use]
    pub fn tail(&self) -> TailStatus {
        // TailStatus is Copy; return by value for call-site ergonomics.
        self.tail
    }
}

/// Fail-closed journal errors for append operations. Only filesystem
/// failures and protocol refusals surface here; content anomalies are
/// reported by replay. The `EmptyJournal`, `TornTail`, and `ContextMismatch`
/// variants are constructed only by resume; the append path never emits
/// them.
#[derive(Debug)]
pub enum JournalError {
    /// Underlying filesystem operation failed.
    Io(io::Error),
    /// Create refused: the file already holds bytes (crash-safe single open).
    JournalNotEmpty,
    /// Resume refused: the file holds no records yet; create it first.
    EmptyJournal,
    /// Resume refused: the tail is not clean; 005C owns the recovery decision.
    TornTail {
        valid_records: usize,
        valid_bytes: usize,
        reason: TailReason,
    },
    /// Resume refused: the file binding differs from the requested session.
    ContextMismatch,
    /// Append refused: the session already closed.
    SessionClosed,
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(_) => f.write_str("media journal filesystem operation failed"),
            Self::JournalNotEmpty => {
                f.write_str("media journal file already holds session records")
            }
            Self::EmptyJournal => f.write_str("media journal file holds no records"),
            Self::TornTail { .. } => {
                f.write_str("media journal tail is torn; recovery owns this file")
            }
            Self::ContextMismatch => {
                f.write_str("media journal binding does not match the requested session")
            }
            Self::SessionClosed => f.write_str("media journal session already closed"),
        }
    }
}

impl Error for JournalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<io::Error> for JournalError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

fn parse_u16(input: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(
        input[offset..offset + 2]
            .try_into()
            .expect("fixed 005B header range"),
    )
}

fn parse_u32(input: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(
        input[offset..offset + 4]
            .try_into()
            .expect("fixed 005B header range"),
    )
}

fn parse_u64(input: &[u8], offset: usize) -> u64 {
    u64::from_be_bytes(
        input[offset..offset + 8]
            .try_into()
            .expect("fixed 005B header range"),
    )
}

fn parse_array<const N: usize>(input: &[u8], offset: usize) -> [u8; N] {
    input[offset..offset + N]
        .try_into()
        .expect("fixed 005B header range")
}

fn record_kind(record_type: u16, payload: &[u8]) -> Result<JournalRecordKind, TailReason> {
    match record_type {
        JOURNAL_TYPE_OPEN => {
            if payload.len() != JOURNAL_OPEN_PAYLOAD_BYTES {
                return Err(TailReason::LengthMismatch);
            }
            let generation = KeyGeneration::new(parse_u64(payload, 0))
                .map_err(|_| TailReason::InvalidGeneration)?;
            Ok(JournalRecordKind::Open { generation })
        }
        JOURNAL_TYPE_COMMIT => {
            if payload.len() != JOURNAL_COMMIT_PAYLOAD_BYTES {
                return Err(TailReason::LengthMismatch);
            }
            let chunk_index = parse_u64(payload, 0);
            let generation = KeyGeneration::new(parse_u64(payload, 8))
                .map_err(|_| TailReason::InvalidGeneration)?;
            let nonce = parse_array(payload, 16);
            let plaintext_len = parse_u64(payload, 40);
            let envelope_digest = parse_array(payload, 48);
            Ok(JournalRecordKind::Commit {
                chunk_index,
                generation,
                nonce,
                plaintext_len,
                envelope_digest,
            })
        }
        JOURNAL_TYPE_CLOSE => {
            if payload.len() != JOURNAL_CLOSE_PAYLOAD_BYTES {
                return Err(TailReason::LengthMismatch);
            }
            Ok(JournalRecordKind::Close {
                commit_count: parse_u64(payload, 0),
            })
        }
        _ => Err(TailReason::UnsupportedType),
    }
}

fn parse_records(bytes: &[u8]) -> (Vec<JournalRecord>, TailStatus) {
    let mut records = Vec::new();
    let mut offset = 0usize;
    let mut closed = false;
    loop {
        let remaining = bytes.len() - offset;
        if remaining == 0 {
            return (records, TailStatus::CleanEof);
        }
        if remaining < JOURNAL_HEADER_BYTES {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::TruncatedHeader,
                },
            );
        }
        let header = &bytes[offset..offset + JOURNAL_HEADER_BYTES];
        if &header[..JOURNAL_MAGIC.len()] != JOURNAL_MAGIC {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::InvalidMagic,
                },
            );
        }
        if parse_u16(header, VERSION_OFFSET) != JOURNAL_VERSION {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::UnsupportedVersion,
                },
            );
        }
        let record_type = parse_u16(header, TYPE_OFFSET);
        let seq = parse_u64(header, SEQ_OFFSET);
        let expected_seq = records.len() as u64 + 1;
        if seq != expected_seq {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::SequenceGap,
                },
            );
        }
        let vault_id = VaultId::from_bytes(parse_array(header, VAULT_OFFSET));
        let session_id = parse_array(header, SESSION_OFFSET);
        let payload_len = parse_u32(header, PAYLOAD_LEN_OFFSET) as usize;
        let Some(record_end) = JOURNAL_HEADER_BYTES
            .checked_add(payload_len)
            .and_then(|body| body.checked_add(JOURNAL_INTEGRITY_BYTES))
            .and_then(|total| offset.checked_add(total))
        else {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::LengthMismatch,
                },
            );
        };
        // Derive all bounds from the checked record_end: record_end is at
        // least offset + HEADER + INTEGRITY, so body_end cannot underflow and
        // no second unchecked copy of the length arithmetic exists to drift.
        let body_end = record_end - JOURNAL_INTEGRITY_BYTES;
        if bytes.len() < body_end {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::TruncatedPayload,
                },
            );
        }
        if bytes.len() < record_end {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::TruncatedIntegrity,
                },
            );
        }
        let payload = &bytes[offset + JOURNAL_HEADER_BYTES..body_end];
        let stored = &bytes[body_end..record_end];
        let mut hasher = Sha256::new();
        hasher.update(&bytes[offset..body_end]);
        if hasher.finalize().as_slice() != stored {
            return (
                records,
                TailStatus::TornTail {
                    valid_bytes: offset,
                    reason: TailReason::IntegrityMismatch,
                },
            );
        }
        let kind = match record_kind(record_type, payload) {
            Ok(kind) => kind,
            Err(reason) => {
                return (
                    records,
                    TailStatus::TornTail {
                        valid_bytes: offset,
                        reason,
                    },
                );
            }
        };
        if records.is_empty() {
            if !matches!(kind, JournalRecordKind::Open { .. }) {
                return (
                    records,
                    TailStatus::TornTail {
                        valid_bytes: offset,
                        reason: TailReason::FirstRecordNotOpen,
                    },
                );
            }
        } else {
            let first = records[0];
            if vault_id != first.vault_id || session_id != first.session_id {
                return (
                    records,
                    TailStatus::TornTail {
                        valid_bytes: offset,
                        reason: TailReason::ContextMismatch,
                    },
                );
            }
            if matches!(kind, JournalRecordKind::Open { .. }) {
                return (
                    records,
                    TailStatus::TornTail {
                        valid_bytes: offset,
                        reason: TailReason::UnexpectedOpen,
                    },
                );
            }
            if closed {
                return (
                    records,
                    TailStatus::TornTail {
                        valid_bytes: offset,
                        reason: TailReason::RecordAfterClose,
                    },
                );
            }
        }
        if matches!(kind, JournalRecordKind::Close { .. }) {
            closed = true;
        }
        records.push(JournalRecord {
            seq,
            vault_id,
            session_id,
            kind,
        });
        offset = record_end;
    }
}

fn build_record(
    seq: u64,
    vault_id: VaultId,
    session_id: [u8; 16],
    record_type: u16,
    payload: &[u8],
) -> Vec<u8> {
    let mut out =
        Vec::with_capacity(JOURNAL_HEADER_BYTES + payload.len() + JOURNAL_INTEGRITY_BYTES);
    out.extend_from_slice(JOURNAL_MAGIC);
    out.extend_from_slice(&JOURNAL_VERSION.to_be_bytes());
    out.extend_from_slice(&record_type.to_be_bytes());
    out.extend_from_slice(&seq.to_be_bytes());
    out.extend_from_slice(vault_id.as_bytes());
    out.extend_from_slice(&session_id);
    let payload_len = u32::try_from(payload.len()).expect("005B payloads are fixed 8/80 bytes");
    out.extend_from_slice(&payload_len.to_be_bytes());
    out.extend_from_slice(payload);
    let digest = Sha256::digest(&out);
    out.extend_from_slice(&digest);
    debug_assert_eq!(
        out.len(),
        JOURNAL_HEADER_BYTES + payload.len() + JOURNAL_INTEGRITY_BYTES
    );
    out
}

fn open_payload(generation: KeyGeneration) -> [u8; JOURNAL_OPEN_PAYLOAD_BYTES] {
    generation.get().to_be_bytes()
}

fn commit_payload(
    chunk_index: u64,
    generation: KeyGeneration,
    nonce: [u8; 24],
    plaintext_len: u64,
    digest: [u8; 32],
) -> [u8; JOURNAL_COMMIT_PAYLOAD_BYTES] {
    let mut payload = [0u8; JOURNAL_COMMIT_PAYLOAD_BYTES];
    payload[0..8].copy_from_slice(&chunk_index.to_be_bytes());
    payload[8..16].copy_from_slice(&generation.get().to_be_bytes());
    payload[16..40].copy_from_slice(&nonce);
    payload[40..48].copy_from_slice(&plaintext_len.to_be_bytes());
    payload[48..80].copy_from_slice(&digest);
    payload
}

fn close_payload(commit_count: u64) -> [u8; JOURNAL_CLOSE_PAYLOAD_BYTES] {
    commit_count.to_be_bytes()
}

/// Replays one journal file without mutating it.
///
/// Returns the valid record prefix plus the tail state. Content anomalies
/// never surface as errors; only filesystem failures do.
///
/// # Errors
///
/// Returns `JournalError::Io` when the file cannot be read.
pub fn replay_journal(path: &Path) -> Result<JournalReplay, JournalError> {
    let mut bytes = Vec::new();
    File::open(path)?.read_to_end(&mut bytes)?;
    let (records, tail) = parse_records(&bytes);
    Ok(JournalReplay { records, tail })
}

/// Append handle for one session journal file.
///
/// The handle owns the file, the vault/session binding, and the next
/// sequence number. Every successful append ends with `sync_all` and a
/// length check before the sequence number is returned.
///
/// Single-writer contract: at most one live writer (or creator) per file;
/// the caller serializes create/resume across processes and threads. Two
/// concurrent creators can both observe an empty file and both append a
/// seq-1 open, and two interleaved writers corrupt sequencing (replay then
/// stops torn, typically at `RecordAfterClose`). The crash-safe single-open
/// guarantee covers crash-restart, not races.
#[derive(Debug)]
pub struct JournalWriter {
    file: File,
    vault_id: VaultId,
    session_id: [u8; 16],
    next_seq: u64,
    closed: bool,
}

impl JournalWriter {
    fn write_synced(&mut self, bytes: &[u8]) -> io::Result<()> {
        let before = self.file.metadata()?.len();
        self.file.write_all(bytes)?;
        self.file.sync_all()?;
        let after = self.file.metadata()?.len();
        let expected = before
            .checked_add(bytes.len() as u64)
            .ok_or_else(|| io::Error::other("journal length overflow"))?;
        if after != expected {
            return Err(io::Error::other("journal length mismatch after sync"));
        }
        Ok(())
    }

    /// Creates a session journal and appends the open marker as seq 1.
    ///
    /// The file must not exist or must be empty; a non-empty file is refused
    /// with `JournalError::JournalNotEmpty` so a live session is never
    /// silently extended. No parent directory is created.
    ///
    /// # Errors
    ///
    /// Returns `JournalError::Io` on filesystem failure or
    /// `JournalError::JournalNotEmpty` when the file already holds bytes.
    pub fn create(
        path: &Path,
        vault_id: VaultId,
        session_id: [u8; 16],
        generation: KeyGeneration,
    ) -> Result<Self, JournalError> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        if file.metadata()?.len() != 0 {
            return Err(JournalError::JournalNotEmpty);
        }
        let mut writer = Self {
            file,
            vault_id,
            session_id,
            next_seq: 1,
            closed: false,
        };
        let bytes = build_record(
            1,
            vault_id,
            session_id,
            JOURNAL_TYPE_OPEN,
            &open_payload(generation),
        );
        writer.write_synced(&bytes)?;
        writer.next_seq = 2;
        Ok(writer)
    }

    /// Returns the vault identity this writer is bound to.
    #[must_use]
    pub const fn vault_id(&self) -> VaultId {
        self.vault_id
    }

    /// Returns the session identity this writer is bound to.
    #[must_use]
    pub const fn session_id(&self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the sequence number the next append will carry.
    #[must_use]
    pub const fn next_seq(&self) -> u64 {
        self.next_seq
    }

    /// Resumes a cleanly closed-tailed session journal for further appends.
    ///
    /// Refuses an empty file (`EmptyJournal`), a torn tail (`TornTail`,
    /// owned by 005C recovery), a binding mismatch (`ContextMismatch`), and
    /// an already-closed session (`SessionClosed`).
    ///
    /// # Errors
    ///
    /// Returns the refusal above or `JournalError::Io` on filesystem failure.
    pub fn resume(
        path: &Path,
        vault_id: VaultId,
        session_id: [u8; 16],
    ) -> Result<Self, JournalError> {
        let replay = replay_journal(path)?;
        // Torn tail first: a garbage file has zero valid records but must
        // report TornTail, not EmptyJournal (whose "create it first" advice
        // would then refuse with JournalNotEmpty).
        if let TailStatus::TornTail {
            valid_bytes,
            reason,
        } = replay.tail
        {
            return Err(JournalError::TornTail {
                valid_records: replay.records.len(),
                valid_bytes,
                reason,
            });
        }
        if replay.records.is_empty() {
            return Err(JournalError::EmptyJournal);
        }
        let first = replay.records[0];
        if first.vault_id != vault_id || first.session_id != session_id {
            return Err(JournalError::ContextMismatch);
        }
        if matches!(
            replay.records.last().map(|record| record.kind()),
            Some(JournalRecordKind::Close { .. })
        ) {
            return Err(JournalError::SessionClosed);
        }
        let file = OpenOptions::new().append(true).open(path)?;
        let next_seq = replay.records.len() as u64 + 1;
        // Bind from the replayed file, not the caller: the equality check
        // above makes them identical on this path, and reading from `first`
        // stays correct even if a future edit reorders the check.
        Ok(Self {
            file,
            vault_id: first.vault_id,
            session_id: first.session_id,
            next_seq,
            closed: false,
        })
    }

    /// Appends one chunk-commit record quoting public 005A envelope fields.
    ///
    /// The nonce is quoted, never generated: B203 owns the nonce lifecycle
    /// and the 005B journal only logs the value the caller used.
    ///
    /// A failed append must not be retried with the same handle: `next_seq`
    /// is not advanced on `Io` failure, so a retry would duplicate a
    /// sequence number this grain cannot diagnose. Quarantine the file for
    /// 005C recovery instead.
    ///
    /// # Errors
    ///
    /// Returns `JournalError::Io` on filesystem failure or
    /// `JournalError::SessionClosed` after a close marker.
    pub fn append_commit(
        &mut self,
        chunk_index: u64,
        generation: KeyGeneration,
        nonce: [u8; 24],
        plaintext_len: u64,
        digest: [u8; 32],
    ) -> Result<u64, JournalError> {
        if self.closed {
            return Err(JournalError::SessionClosed);
        }
        let seq = self.next_seq;
        let payload = commit_payload(chunk_index, generation, nonce, plaintext_len, digest);
        let bytes = build_record(
            seq,
            self.vault_id,
            self.session_id,
            JOURNAL_TYPE_COMMIT,
            &payload,
        );
        self.write_synced(&bytes)?;
        self.next_seq = seq
            .checked_add(1)
            .ok_or_else(|| io::Error::other("journal sequence exhausted"))?;
        Ok(seq)
    }

    /// Appends the session close marker with the writer's commit count.
    ///
    /// # Errors
    ///
    /// Returns `JournalError::Io` on filesystem failure or
    /// `JournalError::SessionClosed` when called twice.
    pub fn append_close(&mut self, commit_count: u64) -> Result<u64, JournalError> {
        if self.closed {
            return Err(JournalError::SessionClosed);
        }
        let seq = self.next_seq;
        // Early warning only: 005C verifies the count against the manifest
        // at recovery. seq - 2 cannot underflow: next_seq is at least 2 on
        // every path that reaches an append.
        debug_assert_eq!(commit_count, seq - 2);
        let bytes = build_record(
            seq,
            self.vault_id,
            self.session_id,
            JOURNAL_TYPE_CLOSE,
            &close_payload(commit_count),
        );
        self.write_synced(&bytes)?;
        self.next_seq = seq
            .checked_add(1)
            .ok_or_else(|| io::Error::other("journal sequence exhausted"))?;
        self.closed = true;
        Ok(seq)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        JournalError, JournalRecordKind, JournalWriter, TailReason, TailStatus, envelope_digest,
        replay_journal,
    };
    use crate::vault::{KeyGeneration, VaultId};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    const TEST_VAULT: [u8; 16] = [0x51; 16];
    const TEST_SESSION: [u8; 16] = [0x53; 16];
    const TEST_NONCE: [u8; 24] = [0x54; 24];
    const TEST_DIGEST: [u8; 32] = [0x55; 32];
    const OTHER_SESSION: [u8; 16] = [0x56; 16];

    /// Unique journal path removed on drop. Declared before any writer
    /// handle so reverse-drop order closes handles before deletion
    /// (required on Windows, where removing an open file fails).
    struct TestFile {
        path: PathBuf,
    }

    impl TestFile {
        fn new() -> Self {
            let id = COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = std::env::temp_dir().join(format!(
                "himsat-005b1-{}-{}-{}.jrn",
                std::process::id(),
                id,
                // Uniqueness within one process run without a clock dependency.
                id.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ));
            Self { path }
        }

        fn path(&self) -> &std::path::Path {
            &self.path
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn vault() -> VaultId {
        VaultId::from_bytes(TEST_VAULT)
    }

    fn generation() -> KeyGeneration {
        KeyGeneration::new(9).expect("test generation is non-zero")
    }

    #[test]
    fn append_path_writes_exact_sized_synced_records() {
        use super::JournalWriter;

        let file = TestFile::new();
        let mut writer = JournalWriter::create(file.path(), vault(), TEST_SESSION, generation())
            .expect("create");
        assert_eq!(writer.next_seq(), 2);
        // open(101) + commit(173) + commit(173) + close(101) = 548 bytes.
        assert_eq!(
            writer
                .append_commit(0, generation(), TEST_NONCE, 128, TEST_DIGEST)
                .expect("commit 0"),
            2
        );
        assert_eq!(
            writer
                .append_commit(1, generation(), TEST_NONCE, 64, TEST_DIGEST)
                .expect("commit 1"),
            3
        );
        assert_eq!(writer.append_close(2).expect("close"), 4);
        drop(writer);
        let bytes = fs::read(file.path()).expect("journal must be readable");
        assert_eq!(bytes.len(), 101 + 173 + 173 + 101);
        // Framing magic opens the file; the open marker carries seq 1.
        assert_eq!(&bytes[0..13], super::JOURNAL_MAGIC);
        assert_eq!(
            u64::from_be_bytes(bytes[17..25].try_into().expect("seq")),
            1
        );
    }

    #[test]
    fn create_refuses_nonempty_file() {
        use super::JournalWriter;

        let file = TestFile::new();
        let writer = JournalWriter::create(file.path(), vault(), TEST_SESSION, generation())
            .expect("create");
        let err = JournalWriter::create(file.path(), vault(), TEST_SESSION, generation())
            .expect_err("second create must fail");
        assert!(matches!(err, JournalError::JournalNotEmpty));
        drop(writer);
    }

    #[test]
    fn append_after_close_is_refused() {
        use super::JournalWriter;

        let file = TestFile::new();
        let mut writer = JournalWriter::create(file.path(), vault(), TEST_SESSION, generation())
            .expect("create");
        writer.append_close(0).expect("close");
        let err = writer
            .append_commit(0, generation(), TEST_NONCE, 1, TEST_DIGEST)
            .expect_err("commit after close must fail");
        assert!(matches!(err, JournalError::SessionClosed));
        let err = writer.append_close(0).expect_err("double close must fail");
        assert!(matches!(err, JournalError::SessionClosed));
        drop(writer);
    }

    #[test]
    fn envelope_digest_is_stable_and_input_sensitive() {
        let a = envelope_digest(b"chunk-a");
        assert_eq!(a, envelope_digest(b"chunk-a"));
        assert_ne!(a, envelope_digest(b"chunk-b"));
        assert_eq!(a.len(), 32);
    }

    #[test]
    fn error_display_names_each_failure_class() {
        let cases: Vec<JournalError> = vec![
            JournalError::Io(std::io::Error::other("probe")),
            JournalError::JournalNotEmpty,
            JournalError::EmptyJournal,
            super::JournalError::TornTail {
                valid_records: 1,
                valid_bytes: 101,
                reason: super::TailReason::IntegrityMismatch,
            },
            JournalError::ContextMismatch,
            JournalError::SessionClosed,
        ];
        for case in &cases {
            assert!(!format!("{case}").is_empty());
        }
    }

    fn write_session(path: &std::path::Path, commits: u64, close: bool) -> Vec<u8> {
        let mut writer =
            JournalWriter::create(path, vault(), TEST_SESSION, generation()).expect("create");
        for index in 0..commits {
            writer
                .append_commit(index, generation(), TEST_NONCE, 128, TEST_DIGEST)
                .expect("commit");
        }
        if close {
            writer.append_close(commits).expect("close");
        }
        drop(writer);
        fs::read(path).expect("journal must be readable")
    }

    #[test]
    fn replay_returns_valid_prefix_with_clean_tail() {
        let file = TestFile::new();
        let bytes = write_session(file.path(), 3, true);
        assert_eq!(bytes.len(), 101 + 3 * 173 + 101);
        let replay = replay_journal(file.path()).expect("replay");
        assert_eq!(replay.records().len(), 5);
        assert_eq!(replay.tail(), TailStatus::CleanEof);
        assert_eq!(replay.records()[0].seq(), 1);
        assert!(matches!(
            replay.records()[0].kind(),
            JournalRecordKind::Open { .. }
        ));
        assert!(matches!(
            replay.records()[4].kind(),
            JournalRecordKind::Close { commit_count: 3 }
        ));
        if let JournalRecordKind::Commit {
            chunk_index,
            nonce,
            plaintext_len,
            envelope_digest,
            ..
        } = replay.records()[2].kind()
        {
            assert_eq!(chunk_index, 1);
            assert_eq!(nonce, TEST_NONCE);
            assert_eq!(plaintext_len, 128);
            assert_eq!(envelope_digest, TEST_DIGEST);
        } else {
            panic!("record 3 must be a commit");
        }
    }

    #[test]
    fn empty_file_replays_clean_with_zero_records() {
        let file = TestFile::new();
        fs::write(file.path(), []).expect("empty file");
        let replay = replay_journal(file.path()).expect("replay");
        assert!(replay.records().is_empty());
        assert_eq!(replay.tail(), TailStatus::CleanEof);
    }

    #[test]
    fn every_truncation_offset_returns_the_exact_valid_prefix() {
        let file = TestFile::new();
        let bytes = write_session(file.path(), 2, true);
        // Boundaries: open 101, commits 274/447, close 548.
        let boundaries = [0usize, 101, 274, 447, 548];
        for cut in 0..=bytes.len() {
            fs::write(file.path(), &bytes[..cut]).expect("truncate fixture");
            let replay = replay_journal(file.path()).expect("replay");
            let contained = [101, 274, 447, 548].iter().filter(|b| **b <= cut).count();
            assert_eq!(replay.records().len(), contained, "cut at {cut}");
            if boundaries.contains(&cut) {
                // A cut on a record boundary is a clean crash point.
                assert_eq!(replay.tail(), TailStatus::CleanEof, "cut at {cut}");
            } else {
                let valid = boundaries
                    .iter()
                    .copied()
                    .filter(|b| *b <= cut)
                    .max()
                    .unwrap_or(0);
                assert!(
                    matches!(replay.tail(), TailStatus::TornTail { valid_bytes, .. } if valid_bytes == valid),
                    "cut at {cut} must stop at the last complete record, got {:?}",
                    replay.tail()
                );
            }
        }
    }

    #[test]
    fn single_bit_flips_stop_at_the_damaged_record() {
        let file = TestFile::new();
        let bytes = write_session(file.path(), 2, false);
        // Flip every bit of the second commit (274..447): the valid prefix
        // is preserved and replay stops at 274. Payload/trailer flips
        // (at/after 335) break exactly the integrity; header-region reasons
        // follow parser order.
        for byte in 274..447 {
            for bit in 0..8 {
                let mut damaged = bytes.clone();
                damaged[byte] ^= 1 << bit;
                fs::write(file.path(), &damaged).expect("damage fixture");
                let replay = replay_journal(file.path()).expect("replay");
                assert_eq!(replay.records().len(), 2, "flip at byte {byte} bit {bit}");
                if byte < 335 {
                    assert!(
                        matches!(
                            replay.tail(),
                            TailStatus::TornTail {
                                valid_bytes: 274,
                                ..
                            }
                        ),
                        "flip at byte {byte} bit {bit} must stop at 274, got {:?}",
                        replay.tail()
                    );
                } else {
                    assert_eq!(
                        replay.tail(),
                        TailStatus::TornTail {
                            valid_bytes: 274,
                            reason: TailReason::IntegrityMismatch,
                        },
                        "flip at byte {byte} bit {bit}"
                    );
                }
            }
        }
    }

    #[test]
    fn magic_corruption_at_offset_zero_yields_empty_prefix() {
        let file = TestFile::new();
        let mut bytes = write_session(file.path(), 1, false);
        bytes[0] ^= 0xFF;
        fs::write(file.path(), &bytes).expect("damage fixture");
        let replay = replay_journal(file.path()).expect("replay");
        assert!(replay.records().is_empty());
        assert_eq!(
            replay.tail(),
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::InvalidMagic,
            }
        );
    }

    #[test]
    fn sequence_gap_stops_replay_without_applying_later_records() {
        let file = TestFile::new();
        let bytes = write_session(file.path(), 2, false);
        // Drop the middle commit: open (seq 1) then commit (seq 3), both
        // integrity-valid, so the sequence rule stops the replay.
        let mut gapped = bytes[0..101].to_vec();
        gapped.extend_from_slice(&bytes[274..447]);
        fs::write(file.path(), &gapped).expect("gap fixture");
        let replay = replay_journal(file.path()).expect("replay");
        assert_eq!(replay.records().len(), 1);
        assert_eq!(
            replay.tail(),
            TailStatus::TornTail {
                valid_bytes: 101,
                reason: TailReason::SequenceGap,
            }
        );
    }

    #[test]
    fn transplanted_vault_binding_stops_replay() {
        use sha2::{Digest, Sha256};

        const OTHER_VAULT: [u8; 16] = [0x52; 16];
        let file = TestFile::new();
        let bytes = write_session(file.path(), 2, false);
        // Transplant the second commit under a foreign vault id and repair
        // its integrity trailer, so the parser reaches the binding rule.
        let mut transplant = bytes[274..447].to_vec();
        transplant[25..41].copy_from_slice(&OTHER_VAULT);
        let digest = Sha256::digest(&transplant[..141]);
        transplant[141..173].copy_from_slice(&digest);
        let mut spliced = bytes[0..274].to_vec();
        spliced.extend_from_slice(&transplant);
        fs::write(file.path(), &spliced).expect("transplant fixture");
        let replay = replay_journal(file.path()).expect("replay");
        assert_eq!(replay.records().len(), 2);
        assert_eq!(
            replay.tail(),
            TailStatus::TornTail {
                valid_bytes: 274,
                reason: TailReason::ContextMismatch,
            }
        );
    }

    #[test]
    fn missing_file_replay_is_an_io_error_not_a_tail() {
        let file = TestFile::new();
        let err = replay_journal(file.path()).expect_err("missing file must fail");
        assert!(matches!(err, JournalError::Io(_)));
    }

    #[test]
    fn resume_refuses_empty_and_missing_files() {
        let file = TestFile::new();
        fs::write(file.path(), []).expect("empty file");
        let err = JournalWriter::resume(file.path(), vault(), TEST_SESSION)
            .expect_err("resume on empty file must fail");
        assert!(matches!(err, super::JournalError::EmptyJournal));
        let missing = TestFile::new();
        let err = JournalWriter::resume(missing.path(), vault(), TEST_SESSION)
            .expect_err("resume on missing file must fail");
        assert!(matches!(err, super::JournalError::Io(_)));
    }

    #[test]
    fn resume_continues_a_clean_tail_and_refuses_closed_or_torn() {
        let file = TestFile::new();
        let path = file.path().to_path_buf();
        let mut writer =
            JournalWriter::create(&path, vault(), TEST_SESSION, generation()).expect("create");
        writer
            .append_commit(0, generation(), TEST_NONCE, 64, TEST_DIGEST)
            .expect("commit");
        drop(writer);
        let mut resumed =
            JournalWriter::resume(&path, vault(), TEST_SESSION).expect("resume clean tail");
        assert_eq!(resumed.next_seq(), 3);
        resumed.append_close(1).expect("close");
        drop(resumed);
        // Refused resumes must not mutate the file: snapshot before each.
        let before = fs::read(&path).expect("read");
        let err = JournalWriter::resume(&path, vault(), TEST_SESSION)
            .expect_err("resume after close must fail");
        assert!(matches!(err, super::JournalError::SessionClosed));
        assert_eq!(fs::read(&path).expect("read"), before);
        // Torn tail: truncate mid-record, resume must refuse with the replay.
        let mut bytes = fs::read(&path).expect("read");
        bytes.truncate(bytes.len() - 7);
        fs::write(&path, &bytes).expect("tear fixture");
        let before = fs::read(&path).expect("read");
        let err = JournalWriter::resume(&path, vault(), TEST_SESSION)
            .expect_err("resume on torn tail must fail");
        assert!(
            matches!(err, super::JournalError::TornTail { valid_bytes, .. } if valid_bytes == 274),
            "torn tail must report the valid prefix, got {err:?}"
        );
        assert_eq!(fs::read(&path).expect("read"), before);
        // Wrong session binding on a clean file is refused as a mismatch.
        let clean_file = TestFile::new();
        let clean = clean_file.path().to_path_buf();
        let writer =
            JournalWriter::create(&clean, vault(), TEST_SESSION, generation()).expect("create");
        drop(writer);
        let before = fs::read(&clean).expect("read");
        let err = JournalWriter::resume(&clean, vault(), [0x99; 16])
            .expect_err("wrong session must fail");
        assert!(matches!(err, super::JournalError::ContextMismatch));
        assert_eq!(fs::read(&clean).expect("read"), before);
    }

    #[test]
    fn replay_arms_report_exact_reasons() {
        use sha2::{Digest, Sha256};

        // Recompute the integrity trailer after crafting a record mutation.
        fn repair(record: &mut [u8]) {
            let body = record.len() - 32;
            let digest = Sha256::digest(&record[..body]);
            record[body..].copy_from_slice(&digest);
        }

        // One open-only journal (101B) and one open+commit journal (274B).
        let fa = TestFile::new();
        write_session(fa.path(), 0, false);
        let a = fs::read(fa.path()).expect("read open journal");
        let fb = TestFile::new();
        write_session(fb.path(), 1, false);
        let b = fs::read(fb.path()).expect("read commit journal");

        // Each case: crafted file bytes, expected record count, exact tail.
        let mut cases: Vec<(&str, Vec<u8>, usize, TailStatus)> = Vec::new();

        // Record after close: open + close + a commit carrying the correct
        // next seq (3) with repaired integrity, so the parser reaches the
        // close rule instead of stopping at a sequence gap.
        let mut after_close = write_session(TestFile::new().path(), 0, true);
        let mut post_close = b[101..274].to_vec();
        post_close[17..25].copy_from_slice(&3u64.to_be_bytes());
        repair(&mut post_close);
        after_close.extend_from_slice(&post_close);
        cases.push((
            "record-after-close",
            after_close,
            2,
            TailStatus::TornTail {
                valid_bytes: 202,
                reason: TailReason::RecordAfterClose,
            },
        ));

        // Second open: open + open(seq patched to 2, integrity repaired).
        let mut second_open = a.clone();
        let mut reopen = a.clone();
        reopen[17..25].copy_from_slice(&2u64.to_be_bytes());
        repair(&mut reopen);
        second_open.extend_from_slice(&reopen);
        cases.push((
            "second-open",
            second_open,
            1,
            TailStatus::TornTail {
                valid_bytes: 101,
                reason: TailReason::UnexpectedOpen,
            },
        ));

        // First record not open: lone commit re-sequenced to 1.
        let mut lone_commit = b[101..274].to_vec();
        lone_commit[17..25].copy_from_slice(&1u64.to_be_bytes());
        repair(&mut lone_commit);
        cases.push((
            "first-not-open",
            lone_commit,
            0,
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::FirstRecordNotOpen,
            },
        ));

        // Unsupported version: open with version 2, integrity repaired.
        let mut bad_version = a.clone();
        bad_version[13..15].copy_from_slice(&2u16.to_be_bytes());
        repair(&mut bad_version);
        cases.push((
            "bad-version",
            bad_version,
            0,
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::UnsupportedVersion,
            },
        ));

        // Unsupported type: open with type 9, integrity repaired.
        let mut bad_type = a.clone();
        bad_type[15..17].copy_from_slice(&9u16.to_be_bytes());
        repair(&mut bad_type);
        cases.push((
            "bad-type",
            bad_type,
            0,
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::UnsupportedType,
            },
        ));

        // Length mismatch: open header declaring 16 payload bytes with a
        // matching 16-byte payload and valid integrity.
        let mut bad_len = a[..61].to_vec();
        bad_len[57..61].copy_from_slice(&16u32.to_be_bytes());
        bad_len.extend_from_slice(&[0xABu8; 16]);
        bad_len.extend_from_slice(&[0u8; 32]);
        repair(&mut bad_len);
        cases.push((
            "bad-length",
            bad_len,
            0,
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::LengthMismatch,
            },
        ));

        // Invalid generation: open with zeroed generation, repaired.
        let mut bad_gen = a.clone();
        bad_gen[61..69].copy_from_slice(&0u64.to_be_bytes());
        repair(&mut bad_gen);
        cases.push((
            "bad-generation",
            bad_gen,
            0,
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::InvalidGeneration,
            },
        ));

        // Session transplant: second commit under a foreign session id with
        // repaired integrity (mirrors the vault transplant proof).
        let mut session_swap = b[101..274].to_vec();
        session_swap[41..57].copy_from_slice(&OTHER_SESSION);
        repair(&mut session_swap);
        let mut spliced = b[..101].to_vec();
        spliced.extend_from_slice(&session_swap);
        cases.push((
            "session-transplant",
            spliced,
            1,
            TailStatus::TornTail {
                valid_bytes: 101,
                reason: TailReason::ContextMismatch,
            },
        ));

        for (name, bytes, expected_records, expected_tail) in &cases {
            let file = TestFile::new();
            fs::write(file.path(), bytes).expect("arm fixture");
            let replay = replay_journal(file.path()).expect("replay");
            assert_eq!(
                replay.records().len(),
                *expected_records,
                "arm {name} must preserve the valid prefix"
            );
            assert_eq!(
                replay.tail(),
                *expected_tail,
                "arm {name} must report its exact reason"
            );
        }
    }

    #[test]
    fn truncation_reason_split_reports_header_payload_integrity() {
        // One-commit journal: open 0..101, commit header 101..162, commit
        // payload 162..242, commit integrity 242..274.
        let file = TestFile::new();
        let bytes = write_session(file.path(), 1, false);
        // Cut inside the open record tears it: empty prefix at offset 0.
        fs::write(file.path(), &bytes[..50]).expect("cut fixture");
        let replay = replay_journal(file.path()).expect("replay");
        assert!(replay.records().is_empty());
        assert_eq!(
            replay.tail(),
            TailStatus::TornTail {
                valid_bytes: 0,
                reason: TailReason::TruncatedHeader,
            }
        );
        // Later cuts keep the open and name the torn commit region.
        for (cut, reason) in [
            (200, TailReason::TruncatedPayload),
            (260, TailReason::TruncatedIntegrity),
        ] {
            fs::write(file.path(), &bytes[..cut]).expect("cut fixture");
            let replay = replay_journal(file.path()).expect("replay");
            assert_eq!(replay.records().len(), 1, "cut at {cut} keeps the open");
            assert_eq!(
                replay.tail(),
                TailStatus::TornTail {
                    valid_bytes: 101,
                    reason,
                },
                "cut at {cut} must report {reason:?}"
            );
        }
    }
}
