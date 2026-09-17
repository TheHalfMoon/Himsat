//! Session-journal append path for Specification 005B, first grain.
//!
//! This module executes the 005B record layout and file-append discipline:
//! fixed session open/close markers, chunk-commit records quoting public
//! 005A envelope fields, and a `JournalWriter` that appends one
//! caller-owned file per session with a `sync_all` plus length check after
//! every record. Machine replay of the file (truncation-safe tail
//! handling) and crash-resume follow in the second 005B grain; recovery
//! reconciliation stays with 005C.
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
//! - file replay and resume (second grain);
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
use std::io::{self, Write};
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

/// Machine reason a journal tail is torn (reported by replay in the second grain).
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

/// Fail-closed journal errors for append operations. Only filesystem
/// failures and protocol refusals surface here; content anomalies are
/// reported by replay in the second grain. The `EmptyJournal`, `TornTail`,
/// and `ContextMismatch` variants are never constructed by this grain;
/// they are reserved for the second grain's resume API.
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

/// Append handle for one session journal file.
///
/// The handle owns the file, the vault/session binding, and the next
/// sequence number. Every successful append ends with `sync_all` and a
/// length check before the sequence number is returned.
///
/// Single-writer contract: at most one live writer (or creator) per file;
/// the caller serializes creation across processes and threads. Two
/// concurrent creators can both observe an empty file and both append a
/// seq-1 open, and two interleaved writers corrupt sequencing (replay then
/// stops torn once replay exists). The crash-safe single-open
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
    use super::{JournalError, envelope_digest};
    use crate::vault::{KeyGeneration, VaultId};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    const TEST_VAULT: [u8; 16] = [0x51; 16];
    const TEST_SESSION: [u8; 16] = [0x53; 16];
    const TEST_NONCE: [u8; 24] = [0x54; 24];
    const TEST_DIGEST: [u8; 32] = [0x55; 32];

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
}
