//! Crash-recovery scan for Specification 005C, first grain.
//!
//! This module executes the read-only recovery scan: it enumerates one
//! caller-owned journal directory and one caller-owned envelope directory,
//! replays every magic-gated journal file, and digests every envelope
//! file. Association between commits and envelopes is purely by content
//! hash (SHA-256 of the exact 005A envelope bytes): filenames are never
//! parsed, preserving opaque naming per the 004 metadata rules.
//!
//! Additive-only 004/005A/005B boundary discipline:
//!
//! - no reviewed byte is changed; this module only reads through the 005B
//!   public API (`replay_journal`, record accessors) and the 005A digest
//!   recipe (`envelope_digest`);
//! - no new KDF, primitive, key domain, or dependency;
//! - scan never mutates: no truncation, no repair, no deletion. Every
//!   anomaly is reported; nothing is silently skipped except foreign files
//!   (see below).
//!
//! Deliberately out of scope here:
//!
//! - reconciliation against the manifest inventory and the deterministic
//!   `RecoveryReport` (second grain);
//! - fault-injection harnesses and bounded-loss quantification (005D);
//! - envelope authenticity: digest equality detects missing and swapped
//!   bytes; forgery resistance stays with the 005A AEAD envelope, which
//!   needs keys this module never touches.
//!
//! Scan contracts, stated honestly:
//!
//! - Dedicated directories: each scanned directory holds exactly one file
//!   class (journals or envelopes). A journal file is recognized solely by
//!   the 13-byte journal magic; any other regular file is foreign and
//!   skipped without a report. Plain subdirectories are skipped; symlinks
//!   resolving to regular files are followed, and any other non-regular
//!   entry (fifo, socket, device, dangling link) fails the scan.
//! - Size cap: files larger than `MAX_SCANNED_FILE_BYTES` (8 MiB) fail the
//!   scan with an I/O error rather than being read unbounded. A 005A
//!   envelope is at most ~1 MiB and a journal byte is small; anything
//!   larger in a dedicated directory is an anomaly worth refusing.
//! - Failure closure: an unreadable regular file fails the whole scan. A
//!   file that cannot be read cannot be recovered, and silently skipping it
//!   would hide data loss.
//! - Quiescence: recovery scans a settled directory. Concurrent writers
//!   during a scan are outside this contract (the single-writer rule
//!   already forbids them at runtime).
//!
//! Donor posture: Meetily/Anarlog/OpenSuperWhisper file-queue and
//! recorder-path patterns are planning inputs only. No donor implements a
//! magic-gated, content-hash-associated, read-only recovery scan over
//! AEAD-bound journals, so this scan is Himsat-native and adopts no donor
//! code.

use crate::vault_media_journal::{
    JOURNAL_MAGIC, JournalError, JournalRecord, TailStatus, envelope_digest, replay_journal,
};
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

/// Maximum scanned file size: 8 MiB. 005A envelopes are at most
/// `MAX_ENVELOPE_BYTES` (~1 MiB); journals are small records. Anything
/// larger in a dedicated scan directory is refused rather than read
/// unbounded.
pub const MAX_SCANNED_FILE_BYTES: u64 = 8 << 20;

/// Fail-closed scan errors. Content anomalies never surface here: torn
/// journals replay to a valid prefix plus `TailStatus` inside their
/// `ScannedJournal`, and only filesystem failures fail the scan.
#[derive(Debug)]
pub enum ScanError {
    /// Underlying filesystem operation failed.
    Io(io::Error),
    /// Journal replay failed (only on filesystem failure per contract).
    Journal(JournalError),
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(_) => f.write_str("recovery scan filesystem operation failed"),
            Self::Journal(_) => f.write_str("recovery scan journal replay failed"),
        }
    }
}

impl Error for ScanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            Self::Journal(inner) => Some(inner),
        }
    }
}

impl From<io::Error> for ScanError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<JournalError> for ScanError {
    fn from(value: JournalError) -> Self {
        Self::Journal(value)
    }
}

/// One replayed journal file: its opaque name plus the valid prefix and
/// the tail state. The tail is reported, never repaired.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScannedJournal {
    file_name: OsString,
    records: Vec<JournalRecord>,
    tail: TailStatus,
}

impl ScannedJournal {
    /// Returns the opaque file name this journal was read from.
    #[must_use]
    pub fn file_name(&self) -> &OsString {
        &self.file_name
    }

    /// Returns the valid records in file order.
    #[must_use]
    pub fn records(&self) -> &[JournalRecord] {
        &self.records
    }

    /// Returns the tail state (`CleanEof` or the torn-tail stop reason).
    #[must_use]
    pub fn tail(&self) -> TailStatus {
        self.tail
    }
}

/// Deterministic (name-sorted) scan of one journal directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalScan {
    journals: Vec<ScannedJournal>,
}

impl JournalScan {
    /// Returns the scanned journals sorted by file name (see scan docs).
    #[must_use]
    pub fn journals(&self) -> &[ScannedJournal] {
        &self.journals
    }
}

/// One envelope file: its opaque name, exact length, and the SHA-256 of
/// its exact bytes (computed with the single-sourced 005A digest recipe).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvelopeFile {
    file_name: OsString,
    length: u64,
    digest: [u8; 32],
}

impl EnvelopeFile {
    /// Returns the opaque file name this envelope was read from.
    #[must_use]
    pub fn file_name(&self) -> &OsString {
        &self.file_name
    }

    /// Returns the exact file length in bytes.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// Returns the SHA-256 digest of the exact file bytes.
    #[must_use]
    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

/// Deterministic (name-sorted) inventory of one envelope directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvelopeInventory {
    envelopes: Vec<EnvelopeFile>,
}

impl EnvelopeInventory {
    /// Returns the inventoried envelopes sorted by file name (see scan docs).
    #[must_use]
    pub fn envelopes(&self) -> &[EnvelopeFile] {
        &self.envelopes
    }
}

fn has_journal_magic(path: &Path) -> io::Result<bool> {
    // Prefix-only read: the decision needs 13 bytes, not the whole file.
    let mut file = fs::File::open(path)?;
    let mut prefix = [0u8; JOURNAL_MAGIC.len()];
    match file.read_exact(&mut prefix) {
        Ok(()) => Ok(prefix == *JOURNAL_MAGIC),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => Ok(false),
        Err(error) => Err(error),
    }
}

/// Classifies one directory entry for scanning: plain subdirectories are
/// skipped; symlinks resolving to regular files are followed; anything
/// else non-regular (fifo, socket, device, symlink to non-file) fails
/// closed so recoverable content is never silently skipped.
fn entry_is_scannable(entry: &fs::DirEntry) -> io::Result<bool> {
    let file_type = entry.file_type()?;
    if file_type.is_dir() {
        return Ok(false);
    }
    if file_type.is_symlink() {
        if fs::metadata(entry.path())?.is_file() {
            return Ok(true);
        }
        return Err(io::Error::other("scan refused non-regular file"));
    }
    if file_type.is_file() {
        return Ok(true);
    }
    Err(io::Error::other("scan refused non-regular file"))
}

fn check_size(path: &Path) -> io::Result<u64> {
    let length = fs::metadata(path)?.len();
    if length > MAX_SCANNED_FILE_BYTES {
        return Err(io::Error::other("scan file exceeds size cap"));
    }
    Ok(length)
}

/// Scans one dedicated journal directory without mutating it.
///
/// Every regular file carrying the journal magic is replayed; foreign
/// files and plain subdirectories are skipped while symlinks to regular
/// files are followed and other non-regular entries fail the scan.
/// Results sort by file name (byte order on Unix, deterministic platform
/// order elsewhere).
///
/// # Errors
///
/// Returns `ScanError::Io` when the directory cannot be listed, when a
/// regular file cannot be read, or when a file exceeds the size cap.
pub fn scan_journal_dir(dir: &Path) -> Result<JournalScan, ScanError> {
    let mut journals = Vec::new();
    let mut names: Vec<OsString> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry_is_scannable(&entry)? {
            continue;
        }
        names.push(entry.file_name());
    }
    names.sort();
    for name in names {
        let path = dir.join(&name);
        check_size(&path)?;
        if !has_journal_magic(&path)? {
            continue;
        }
        let replay = replay_journal(&path)?;
        journals.push(ScannedJournal {
            file_name: name,
            records: replay.records().to_vec(),
            tail: replay.tail(),
        });
    }
    Ok(JournalScan { journals })
}

/// Inventories one dedicated envelope directory without mutating it.
///
/// Every regular file (following symlinks to regular files) is hashed
/// with the single-sourced 005A digest recipe; results sort by file name
/// (byte order on Unix, deterministic platform order elsewhere).
/// Oversized files fail the scan (see
/// `MAX_SCANNED_FILE_BYTES`).
///
/// # Errors
///
/// Returns `ScanError::Io` when the directory cannot be listed, when a file
/// cannot be read, or when a file exceeds the size cap.
pub fn inventory_envelopes(dir: &Path) -> Result<EnvelopeInventory, ScanError> {
    let mut names: Vec<OsString> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry_is_scannable(&entry)? {
            continue;
        }
        names.push(entry.file_name());
    }
    names.sort();
    let mut envelopes = Vec::new();
    for name in names {
        let path = dir.join(&name);
        let length = check_size(&path)?;
        let bytes = fs::read(&path)?;
        envelopes.push(EnvelopeFile {
            file_name: name,
            length,
            digest: envelope_digest(&bytes),
        });
    }
    Ok(EnvelopeInventory { envelopes })
}

#[cfg(test)]
mod tests {
    use super::{inventory_envelopes, scan_journal_dir};
    use crate::vault::{KeyGeneration, VaultId};
    use crate::vault_media_journal::{JournalWriter, TailStatus};
    use std::error::Error as _;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let id = COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = std::env::temp_dir().join(format!(
                "himsat-005c1-{}-{}",
                std::process::id(),
                id.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ));
            fs::create_dir(&path).expect("test dir");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    const VAULT: [u8; 16] = [0x61; 16];
    const SESSION_A: [u8; 16] = [0x62; 16];
    const SESSION_B: [u8; 16] = [0x63; 16];
    const NONCE: [u8; 24] = [0x64; 24];
    const DIGEST: [u8; 32] = [0x65; 32];

    fn generation() -> KeyGeneration {
        KeyGeneration::new(4).expect("test generation is non-zero")
    }

    fn write_journal(dir: &Path, name: &str, session: [u8; 16], commits: u64) {
        let mut writer = JournalWriter::create(
            &dir.join(name),
            VaultId::from_bytes(VAULT),
            session,
            generation(),
        )
        .expect("create journal");
        for index in 0..commits {
            writer
                .append_commit(index, generation(), NONCE, 64, DIGEST)
                .expect("commit");
        }
        writer.append_close(commits).expect("close");
    }

    #[test]
    fn scan_finds_journals_skips_foreign_and_sorts() {
        let dir = TestDir::new();
        write_journal(dir.path(), "b.jrn", SESSION_B, 1);
        write_journal(dir.path(), "a.jrn", SESSION_A, 2);
        fs::write(dir.path().join("notes.txt"), b"not a journal").expect("foreign");
        fs::create_dir(dir.path().join("subdir")).expect("subdir");
        let scan = scan_journal_dir(dir.path()).expect("scan");
        assert_eq!(scan.journals().len(), 2);
        assert_eq!(scan.journals()[0].file_name(), "a.jrn");
        assert_eq!(scan.journals()[1].file_name(), "b.jrn");
        assert_eq!(scan.journals()[0].records().len(), 4);
        assert_eq!(scan.journals()[0].tail(), TailStatus::CleanEof);
        assert_eq!(scan.journals()[1].records().len(), 3);
        assert_eq!(scan.journals()[1].tail(), TailStatus::CleanEof);
    }

    #[test]
    fn sub_magic_short_files_are_foreign() {
        let dir = TestDir::new();
        fs::write(dir.path().join("short.bin"), b"12345").expect("write");
        write_journal(dir.path(), "s.jrn", SESSION_A, 0);
        let scan = scan_journal_dir(dir.path()).expect("scan");
        assert_eq!(scan.journals().len(), 1);
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_journals_are_followed_not_skipped() {
        use std::os::unix::fs::symlink;

        let dir = TestDir::new();
        write_journal(dir.path(), "real.jrn", SESSION_A, 1);
        symlink(dir.path().join("real.jrn"), dir.path().join("link.jrn")).expect("symlink");
        let scan = scan_journal_dir(dir.path()).expect("scan");
        assert_eq!(scan.journals().len(), 2);
        assert_eq!(scan.journals()[1].file_name(), "real.jrn");
        assert_eq!(scan.journals()[0].file_name(), "link.jrn");
        assert_eq!(
            scan.journals()[0].records().len(),
            scan.journals()[1].records().len()
        );
    }

    #[test]
    fn scan_reports_torn_tails_without_failing() {
        let dir = TestDir::new();
        write_journal(dir.path(), "t.jrn", SESSION_A, 2);
        let bytes = fs::read(dir.path().join("t.jrn")).expect("read");
        fs::write(dir.path().join("t.jrn"), &bytes[..200]).expect("tear");
        let scan = scan_journal_dir(dir.path()).expect("scan");
        assert_eq!(scan.journals().len(), 1);
        assert_eq!(scan.journals()[0].records().len(), 1);
        assert!(
            matches!(
                scan.journals()[0].tail(),
                TailStatus::TornTail {
                    valid_bytes: 101,
                    ..
                }
            ),
            "torn tail must report the valid prefix, got {:?}",
            scan.journals()[0].tail()
        );
    }

    #[test]
    fn scan_empty_and_missing_dirs() {
        let dir = TestDir::new();
        let scan = scan_journal_dir(dir.path()).expect("empty scan");
        assert!(scan.journals().is_empty());
        let missing = dir.path().join("no-such-dir");
        assert!(scan_journal_dir(&missing).is_err());
        assert!(inventory_envelopes(&missing).is_err());
    }

    #[test]
    fn inventory_hashes_lengths_and_sorts() {
        let dir = TestDir::new();
        fs::write(dir.path().join("z.bin"), b"envelope-z-bytes").expect("write");
        fs::write(dir.path().join("a.bin"), b"envelope-a-bytes").expect("write");
        fs::create_dir(dir.path().join("subdir")).expect("subdir");
        let inv = inventory_envelopes(dir.path()).expect("inventory");
        assert_eq!(inv.envelopes().len(), 2);
        assert_eq!(inv.envelopes()[0].file_name(), "a.bin");
        assert_eq!(inv.envelopes()[0].length(), 16);
        let expected = crate::vault_media_journal::envelope_digest(b"envelope-a-bytes");
        assert_eq!(inv.envelopes()[0].digest(), expected);
        assert_eq!(inv.envelopes()[1].file_name(), "z.bin");
        assert_eq!(inv.envelopes()[1].length(), 16);
        let expected_z = crate::vault_media_journal::envelope_digest(b"envelope-z-bytes");
        assert_eq!(inv.envelopes()[1].digest(), expected_z);
    }

    #[test]
    fn scan_error_display_names_each_failure_class() {
        let io = super::ScanError::Io(std::io::Error::other("probe"));
        assert!(!format!("{io}").is_empty());
        assert!(io.source().is_some());
        let dir = TestDir::new();
        let missing = dir.path().join("no-such-dir");
        let err = scan_journal_dir(&missing).expect_err("missing dir must fail");
        assert!(matches!(err, super::ScanError::Io(_)));
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn oversize_files_fail_the_scan() {
        let dir = TestDir::new();
        let big = vec![0x77u8; (8 << 20) + 1];
        fs::write(dir.path().join("big.jrn"), &big).expect("write big");
        assert!(scan_journal_dir(dir.path()).is_err());
        assert!(inventory_envelopes(dir.path()).is_err());
    }

    #[test]
    fn exact_cap_boundary_passes() {
        let dir = TestDir::new();
        let capped = vec![0x78u8; 8 << 20];
        fs::write(dir.path().join("capped.bin"), &capped).expect("write capped");
        // Exactly 8 MiB passes the cap: foreign to the journal scan, hashed
        // by the envelope inventory.
        let scan = scan_journal_dir(dir.path()).expect("scan");
        assert!(scan.journals().is_empty());
        let inv = inventory_envelopes(dir.path()).expect("inventory");
        assert_eq!(inv.envelopes().len(), 1);
        assert_eq!(inv.envelopes()[0].length(), 8 << 20);
    }

    #[test]
    fn scan_error_journal_variant_reports_source() {
        use crate::vault_media_journal::JournalError;

        let err = super::ScanError::Journal(JournalError::EmptyJournal);
        assert!(!format!("{err}").is_empty());
        assert!(err.source().is_some());
    }
}
// 005C grain 2: read-only reconcile of journal commits against envelope
// files and the manifest into a `RecoveryReport`. Binding reuses
// `GenericArtifactBlob`; media ids carry `MEDIA_LOGICAL_TAG`. Matching is
// by content hash only; lists are sorted; duplicates resolve
// first-by-(file, seq), everything else is reported, never merged.

use crate::vault::{KeyGeneration, VaultId};
use crate::vault_manifest::{ManifestAuthMetadata, ManifestPlaintext};
use crate::vault_media_journal::TailReason;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Tag prefix marking media-managed manifest logical ids: `HJM-0001`.
pub const MEDIA_LOGICAL_TAG: [u8; 8] = *b"HJM-0001";

/// Domain separating media logical-id derivation (identifier only, never
/// a key and never secret).
const LOGICAL_ID_DOMAIN: &[u8] = b"HIMSAT/005/MANIFEST-LOGICAL/v1";

/// Derives the manifest logical id for one committed chunk: the tag plus
/// the first 8 digest bytes over domain, session, and index.
#[must_use]
pub fn derive_media_logical_id(session_id: [u8; 16], chunk_index: u64) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(LOGICAL_ID_DOMAIN);
    hasher.update(session_id);
    hasher.update(chunk_index.to_be_bytes());
    let digest = hasher.finalize();
    let mut id = [0u8; 16];
    id[0..8].copy_from_slice(&MEDIA_LOGICAL_TAG);
    id[8..16].copy_from_slice(&digest[..8]);
    id
}

/// Reports whether a manifest logical id is media-managed (tag prefix).
#[must_use]
pub fn is_media_logical_id(logical_id: &[u8; 16]) -> bool {
    logical_id[0..8] == MEDIA_LOGICAL_TAG
}

/// One journal occurrence of a committed chunk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitOccurrence {
    file: OsString,
    seq: u64,
    generation: KeyGeneration,
    nonce: [u8; 24],
    plaintext_len: u64,
    digest: [u8; 32],
}

impl CommitOccurrence {
    /// Returns the opaque journal file holding this occurrence.
    #[must_use]
    pub fn file(&self) -> &OsString {
        &self.file
    }

    /// Returns the sequence number of this occurrence.
    #[must_use]
    pub const fn seq(&self) -> u64 {
        self.seq
    }

    /// Returns the quoted envelope digest of this occurrence.
    #[must_use]
    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

/// A commit whose envelope bytes were found by digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCommit {
    session_id: [u8; 16],
    chunk_index: u64,
    generation: KeyGeneration,
    nonce: [u8; 24],
    plaintext_len: u64,
    digest: [u8; 32],
    envelope_files: Vec<OsString>,
    manifest_bound: bool,
}

impl VerifiedCommit {
    /// Returns the session identity of this commit.
    #[must_use]
    pub const fn session_id(&self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the chunk index of this commit.
    #[must_use]
    pub const fn chunk_index(&self) -> u64 {
        self.chunk_index
    }

    /// Returns the quoted envelope digest of this commit.
    #[must_use]
    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    /// Returns the envelope files matching this commit by digest.
    #[must_use]
    pub fn envelope_files(&self) -> &[OsString] {
        &self.envelope_files
    }

    /// Reports whether a matching manifest object was found.
    #[must_use]
    pub const fn manifest_bound(&self) -> bool {
        self.manifest_bound
    }
}

/// A commit with no envelope file matching its digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrphanCommit {
    session_id: [u8; 16],
    chunk_index: u64,
    generation: KeyGeneration,
    digest: [u8; 32],
}

impl OrphanCommit {
    /// Returns the session identity of this commit.
    #[must_use]
    pub const fn session_id(&self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the chunk index of this commit.
    #[must_use]
    pub const fn chunk_index(&self) -> u64 {
        self.chunk_index
    }

    /// Returns the quoted envelope digest no file matched.
    #[must_use]
    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

/// One (session, index) key committed more than once. The canonical
/// occurrence is the first by (file, seq); every occurrence is reported.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DuplicateCommit {
    session_id: [u8; 16],
    chunk_index: u64,
    occurrences: Vec<CommitOccurrence>,
    canonical_digest: [u8; 32],
}

impl DuplicateCommit {
    /// Returns the session identity of this commit key.
    #[must_use]
    pub const fn session_id(&self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the chunk index of this commit key.
    #[must_use]
    pub const fn chunk_index(&self) -> u64 {
        self.chunk_index
    }

    /// Returns every occurrence in (file, seq) order.
    #[must_use]
    pub fn occurrences(&self) -> &[CommitOccurrence] {
        &self.occurrences
    }

    /// Returns the digest of the first occurrence by (file, seq).
    #[must_use]
    pub const fn canonical_digest(&self) -> [u8; 32] {
        self.canonical_digest
    }
}

/// A manifest binding proposed for a verified but unlogged commit. The
/// caller assigns `storage_id` per its opaque scheme and inserts through
/// the authenticated manifest writer; proposals arrive sorted by
/// logical id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProposedBinding {
    logical_id: [u8; 16],
    key_generation: KeyGeneration,
    ciphertext_length: u64,
    ciphertext_sha256: [u8; 32],
    nonce: [u8; 24],
}

impl ProposedBinding {
    /// Returns the derived media logical id to insert.
    #[must_use]
    pub const fn logical_id(&self) -> [u8; 16] {
        self.logical_id
    }

    /// Returns the key generation to record.
    #[must_use]
    pub const fn key_generation(self) -> KeyGeneration {
        self.key_generation
    }

    /// Returns the exact envelope length to record.
    #[must_use]
    pub const fn ciphertext_length(&self) -> u64 {
        self.ciphertext_length
    }

    /// Returns the envelope digest to record.
    #[must_use]
    pub const fn ciphertext_sha256(&self) -> [u8; 32] {
        self.ciphertext_sha256
    }

    /// Returns the quoted nonce to record.
    #[must_use]
    pub const fn nonce(&self) -> [u8; 24] {
        self.nonce
    }
}

/// A verified commit with no manifest counterpart, plus its proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnboundCommit {
    session_id: [u8; 16],
    chunk_index: u64,
    proposed: ProposedBinding,
}

impl UnboundCommit {
    /// Returns the session identity of this commit.
    #[must_use]
    pub const fn session_id(&self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the chunk index of this commit.
    #[must_use]
    pub const fn chunk_index(&self) -> u64 {
        self.chunk_index
    }

    /// Returns the proposed manifest binding.
    #[must_use]
    pub const fn proposed(&self) -> ProposedBinding {
        self.proposed
    }
}

/// Authenticated manifest facts found for a media logical id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FoundManifestFacts {
    key_generation: KeyGeneration,
    ciphertext_length: u64,
    ciphertext_sha256: [u8; 32],
    nonce: [u8; 24],
}

/// A verified commit whose manifest object disagrees on generation,
/// length, digest, or nonce.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DivergedManifest {
    session_id: [u8; 16],
    chunk_index: u64,
    logical_id: [u8; 16],
    expected_generation: KeyGeneration,
    expected_length: u64,
    expected_digest: [u8; 32],
    expected_nonce: [u8; 24],
    found: FoundManifestFacts,
}

/// A media-tagged manifest object with no journal commit at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnloggedEntry {
    logical_id: [u8; 16],
    key_generation: KeyGeneration,
    ciphertext_length: u64,
    ciphertext_sha256: [u8; 32],
}

/// An envelope file matching no commit digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnreferencedEnvelope {
    file: OsString,
    digest: [u8; 32],
    length: u64,
}

/// A scanned journal whose tail is torn (valid prefix still reconciled).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TornJournalFile {
    file: OsString,
    valid_records: usize,
    valid_bytes: usize,
    reason: TailReason,
}

/// A journal file bound to a different vault than the manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultMismatchFile {
    file: OsString,
}

/// Deterministic crash-recovery reconciliation report. Every list is
/// sorted; every anomaly is reported; nothing is mutated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryReport {
    verified: Vec<VerifiedCommit>,
    orphans: Vec<OrphanCommit>,
    duplicates: Vec<DuplicateCommit>,
    unbound: Vec<UnboundCommit>,
    diverged: Vec<DivergedManifest>,
    unlogged: Vec<UnloggedEntry>,
    unreferenced: Vec<UnreferencedEnvelope>,
    torn: Vec<TornJournalFile>,
    vault_mismatches: Vec<VaultMismatchFile>,
}

impl RecoveryReport {
    /// Reports whether every commit verified and bound with no anomaly.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.orphans.is_empty()
            && self.duplicates.is_empty()
            && self.unbound.is_empty()
            && self.diverged.is_empty()
            && self.unlogged.is_empty()
            && self.unreferenced.is_empty()
            && self.torn.is_empty()
            && self.vault_mismatches.is_empty()
    }

    /// Returns verified commits in (session, index) order.
    #[must_use]
    pub fn verified(&self) -> &[VerifiedCommit] {
        &self.verified
    }

    /// Returns orphan commits in (session, index) order.
    #[must_use]
    pub fn orphans(&self) -> &[OrphanCommit] {
        &self.orphans
    }

    /// Returns duplicate commit keys in (session, index) order.
    #[must_use]
    pub fn duplicates(&self) -> &[DuplicateCommit] {
        &self.duplicates
    }

    /// Returns unbound commits with proposals, in logical-id order.
    #[must_use]
    pub fn unbound(&self) -> &[UnboundCommit] {
        &self.unbound
    }

    /// Returns diverged manifest entries in (session, index) order.
    #[must_use]
    pub fn diverged(&self) -> &[DivergedManifest] {
        &self.diverged
    }

    /// Returns unlogged media manifest entries in logical-id order.
    #[must_use]
    pub fn unlogged(&self) -> &[UnloggedEntry] {
        &self.unlogged
    }

    /// Returns unreferenced envelope files in file-name order.
    #[must_use]
    pub fn unreferenced(&self) -> &[UnreferencedEnvelope] {
        &self.unreferenced
    }

    /// Returns torn journal files in file-name order.
    #[must_use]
    pub fn torn(&self) -> &[TornJournalFile] {
        &self.torn
    }

    /// Returns vault-mismatched journal files in file-name order.
    #[must_use]
    pub fn vault_mismatches(&self) -> &[VaultMismatchFile] {
        &self.vault_mismatches
    }
}

// Anomaly field accessors (005C grain 2b): read API for every anomaly
// struct the reconcile pass reports. Each mirrors the construction site;
// lists stay sorted per the report contract.

impl FoundManifestFacts {
    /// Returns the manifest-recorded key generation.
    #[must_use]
    pub const fn key_generation(&self) -> KeyGeneration {
        self.key_generation
    }

    /// Returns the manifest-recorded ciphertext length.
    #[must_use]
    pub const fn ciphertext_length(&self) -> u64 {
        self.ciphertext_length
    }

    /// Returns the manifest-recorded ciphertext digest.
    #[must_use]
    pub const fn ciphertext_sha256(&self) -> [u8; 32] {
        self.ciphertext_sha256
    }

    /// Returns the manifest-recorded nonce.
    #[must_use]
    pub const fn nonce(&self) -> [u8; 24] {
        self.nonce
    }
}

impl DivergedManifest {
    /// Returns the session identity of this commit.
    #[must_use]
    pub const fn session_id(&self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the chunk index of this commit.
    #[must_use]
    pub const fn chunk_index(&self) -> u64 {
        self.chunk_index
    }

    /// Returns the media logical id that diverged.
    #[must_use]
    pub const fn logical_id(&self) -> [u8; 16] {
        self.logical_id
    }

    /// Returns the journal-quoted key generation.
    #[must_use]
    pub const fn expected_generation(&self) -> KeyGeneration {
        self.expected_generation
    }

    /// Returns the envelope length the manifest should record.
    #[must_use]
    pub const fn expected_length(&self) -> u64 {
        self.expected_length
    }

    /// Returns the journal-quoted envelope digest.
    #[must_use]
    pub const fn expected_digest(&self) -> [u8; 32] {
        self.expected_digest
    }

    /// Returns the journal-quoted nonce.
    #[must_use]
    pub const fn expected_nonce(&self) -> [u8; 24] {
        self.expected_nonce
    }

    /// Returns the authenticated manifest facts found.
    #[must_use]
    pub const fn found(&self) -> FoundManifestFacts {
        self.found
    }
}

impl UnloggedEntry {
    /// Returns the media logical id with no journal commit.
    #[must_use]
    pub const fn logical_id(&self) -> [u8; 16] {
        self.logical_id
    }

    /// Returns the manifest-recorded key generation.
    #[must_use]
    pub const fn key_generation(&self) -> KeyGeneration {
        self.key_generation
    }

    /// Returns the manifest-recorded ciphertext length.
    #[must_use]
    pub const fn ciphertext_length(&self) -> u64 {
        self.ciphertext_length
    }

    /// Returns the manifest-recorded ciphertext digest.
    #[must_use]
    pub const fn ciphertext_sha256(&self) -> [u8; 32] {
        self.ciphertext_sha256
    }
}

impl UnreferencedEnvelope {
    /// Returns the opaque envelope file with no commit.
    #[must_use]
    pub fn file(&self) -> &OsString {
        &self.file
    }

    /// Returns the digest of the unreferenced bytes.
    #[must_use]
    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    /// Returns the exact length of the unreferenced bytes.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }
}

impl TornJournalFile {
    /// Returns the opaque journal file with a torn tail.
    #[must_use]
    pub fn file(&self) -> &OsString {
        &self.file
    }

    /// Returns the valid record count of the reconciled prefix.
    #[must_use]
    pub const fn valid_records(&self) -> usize {
        self.valid_records
    }

    /// Returns the valid prefix length in bytes.
    #[must_use]
    pub const fn valid_bytes(&self) -> usize {
        self.valid_bytes
    }

    /// Returns the machine reason the replay stopped.
    #[must_use]
    pub const fn reason(&self) -> TailReason {
        self.reason
    }
}

impl VaultMismatchFile {
    /// Returns the opaque journal file bound to another vault.
    #[must_use]
    pub fn file(&self) -> &OsString {
        &self.file
    }
}

/// Reconciles journal commits against envelope files and the manifest
/// inventory into a deterministic, read-only `RecoveryReport`.
///
/// Commit occurrences group by (session, index); repeats become
/// `DuplicateCommit` with first-by-(file, seq) canonical digest. Canonical
/// commits match envelope files by digest (missing files become orphans)
/// and manifest `GenericArtifactBlob` objects by derived logical id
/// (missing objects become `UnboundCommit` with a sorted proposal;
/// field disagreements become `DivergedManifest`). Media-tagged manifest
/// objects with no commit become `UnloggedEntry`; envelopes matching no
/// commit become `UnreferencedEnvelope`; torn journals and
/// vault-mismatched files are listed and otherwise skipped.
#[must_use]
pub fn reconcile_recovery(
    scan: &JournalScan,
    envelopes: &EnvelopeInventory,
    manifest: &ManifestPlaintext,
) -> RecoveryReport {
    let manifest_vault: VaultId = manifest.vault_id();
    let mut occurrences: BTreeMap<([u8; 16], u64), Vec<CommitOccurrence>> = BTreeMap::new();
    let mut torn = Vec::new();
    let mut vault_mismatches = Vec::new();

    for journal in scan.journals() {
        let records = journal.records();
        if let TailStatus::TornTail {
            valid_bytes,
            reason,
        } = journal.tail()
        {
            torn.push(TornJournalFile {
                file: journal.file_name().clone(),
                valid_records: records.len(),
                valid_bytes,
                reason,
            });
        }
        let Some(first) = records.first() else {
            continue;
        };
        if first.vault_id() != manifest_vault {
            vault_mismatches.push(VaultMismatchFile {
                file: journal.file_name().clone(),
            });
            continue;
        }
        for record in records {
            let Some(commit) = record.kind().as_commit() else {
                continue;
            };
            occurrences
                .entry((record.session_id(), commit.chunk_index))
                .or_default()
                .push(CommitOccurrence {
                    file: journal.file_name().clone(),
                    seq: record.seq(),
                    generation: commit.generation,
                    nonce: commit.nonce,
                    plaintext_len: commit.plaintext_len,
                    digest: commit.envelope_digest,
                });
        }
    }

    let mut digest_index: BTreeMap<[u8; 32], Vec<(OsString, u64)>> = BTreeMap::new();
    for envelope in envelopes.envelopes() {
        digest_index
            .entry(envelope.digest())
            .or_default()
            .push((envelope.file_name().clone(), envelope.length()));
    }
    let envelope_length = |digest: &[u8; 32]| -> u64 {
        digest_index
            .get(digest)
            .and_then(|files| files.first().map(|(_, length)| *length))
            .unwrap_or(0)
    };

    let mut verified = Vec::new();
    let mut orphans = Vec::new();
    let mut duplicates = Vec::new();
    // Every committed key counts as logged, even orphaned or duplicated.
    let mut logged_ids: Vec<[u8; 16]> = Vec::new();

    for ((session_id, chunk_index), mut occurrences) in occurrences {
        let logical_id = derive_media_logical_id(session_id, chunk_index);
        logged_ids.push(logical_id);
        occurrences.sort_by(|a, b| (&a.file, a.seq).cmp(&(&b.file, b.seq)));
        if occurrences.len() > 1 {
            duplicates.push(DuplicateCommit {
                session_id,
                chunk_index,
                canonical_digest: occurrences[0].digest,
                occurrences,
            });
            continue;
        }
        let occurrence = &occurrences[0];
        let Some(files) = digest_index.get(&occurrence.digest) else {
            orphans.push(OrphanCommit {
                session_id,
                chunk_index,
                generation: occurrence.generation,
                digest: occurrence.digest,
            });
            continue;
        };
        verified.push(VerifiedCommit {
            session_id,
            chunk_index,
            generation: occurrence.generation,
            nonce: occurrence.nonce,
            plaintext_len: occurrence.plaintext_len,
            digest: occurrence.digest,
            envelope_files: files.iter().map(|(name, _)| name.clone()).collect(),
            manifest_bound: false,
        });
    }

    let mut unbound = Vec::new();
    let mut diverged = Vec::new();
    for commit in verified.iter_mut() {
        let logical_id = derive_media_logical_id(commit.session_id, commit.chunk_index);
        let found = manifest.objects().iter().find(|object| {
            object.logical_id() == logical_id
                && matches!(
                    object.auth_metadata(),
                    ManifestAuthMetadata::GenericArtifactBlob { .. }
                )
        });
        let Some(found) = found else {
            unbound.push(UnboundCommit {
                session_id: commit.session_id,
                chunk_index: commit.chunk_index,
                proposed: ProposedBinding {
                    logical_id,
                    key_generation: commit.generation,
                    ciphertext_length: envelope_length(&commit.digest),
                    ciphertext_sha256: commit.digest,
                    nonce: commit.nonce,
                },
            });
            continue;
        };
        let ManifestAuthMetadata::GenericArtifactBlob { nonce: found_nonce } =
            found.auth_metadata()
        else {
            continue;
        };
        if found.key_generation() == commit.generation
            && found.ciphertext_length() == envelope_length(&commit.digest)
            && found.ciphertext_sha256() == commit.digest
            && found_nonce == commit.nonce
        {
            commit.manifest_bound = true;
        } else {
            diverged.push(DivergedManifest {
                session_id: commit.session_id,
                chunk_index: commit.chunk_index,
                logical_id,
                expected_generation: commit.generation,
                expected_length: envelope_length(&commit.digest),
                expected_digest: commit.digest,
                expected_nonce: commit.nonce,
                found: FoundManifestFacts {
                    key_generation: found.key_generation(),
                    ciphertext_length: found.ciphertext_length(),
                    ciphertext_sha256: found.ciphertext_sha256(),
                    nonce: found_nonce,
                },
            });
        }
    }

    let mut unlogged = Vec::new();
    for object in manifest.objects() {
        let ManifestAuthMetadata::GenericArtifactBlob { .. } = object.auth_metadata() else {
            continue;
        };
        if !is_media_logical_id(&object.logical_id()) {
            continue;
        }
        if !logged_ids.contains(&object.logical_id()) {
            unlogged.push(UnloggedEntry {
                logical_id: object.logical_id(),
                key_generation: object.key_generation(),
                ciphertext_length: object.ciphertext_length(),
                ciphertext_sha256: object.ciphertext_sha256(),
            });
        }
    }

    let mut committed_digests: Vec<[u8; 32]> = Vec::new();
    for digest in verified
        .iter()
        .map(|commit| commit.digest)
        .chain(orphans.iter().map(|orphan| orphan.digest))
        .chain(
            duplicates
                .iter()
                .flat_map(|duplicate| duplicate.occurrences.iter().map(|o| o.digest)),
        )
    {
        if !committed_digests.contains(&digest) {
            committed_digests.push(digest);
        }
    }
    let mut unreferenced = Vec::new();
    for envelope in envelopes.envelopes() {
        if !committed_digests.contains(&envelope.digest()) {
            unreferenced.push(UnreferencedEnvelope {
                file: envelope.file_name().clone(),
                digest: envelope.digest(),
                length: envelope.length(),
            });
        }
    }

    unbound.sort_by_key(|a| a.proposed.logical_id);
    unlogged.sort_by_key(|a| a.logical_id);

    RecoveryReport {
        verified,
        orphans,
        duplicates,
        unbound,
        diverged,
        unlogged,
        unreferenced,
        torn,
        vault_mismatches,
    }
}

#[cfg(test)]
mod reconcile_tests {
    use super::{
        derive_media_logical_id, inventory_envelopes, reconcile_recovery, scan_journal_dir,
    };
    use crate::vault::{FreshnessEpoch, KeyGeneration, ManifestHash, VaultId};
    use crate::vault_manifest::{
        GenerationState, ManifestAuthMetadata, ManifestGeneration, ManifestObject,
        ManifestPlaintext, RotationPhase,
    };
    use crate::vault_media_journal::{JournalWriter, envelope_digest};
    use std::ffi::OsString;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static RECONCILE_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct ReconcileDir {
        path: PathBuf,
    }

    impl ReconcileDir {
        fn new() -> Self {
            let id = RECONCILE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let path =
                std::env::temp_dir().join(format!("himsat-reconcile-{}-{id}", std::process::id()));
            fs::create_dir_all(&path).expect("reconcile temp dir");
            Self { path }
        }
    }

    impl Drop for ReconcileDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    const VAULT: [u8; 16] = [0xA5; 16];
    const SESSION: [u8; 16] = [0x0C; 16];
    const N0: [u8; 24] = [0xC0; 24];
    const N1: [u8; 24] = [0xC1; 24];

    /// One fixture commit: chunk index, generation, nonce, envelope seed.
    type TestCommit = (u64, u64, [u8; 24], u64);

    // Padded past the manifest blob minimum (123 bytes) so fixtures validate.
    fn envelope_bytes(seed: u64) -> Vec<u8> {
        let mut bytes = format!("reconcile-envelope-{seed}-payload").into_bytes();
        bytes.resize(200, 0xE0);
        bytes
    }

    fn digest_of(seed: u64) -> [u8; 32] {
        envelope_digest(&envelope_bytes(seed))
    }

    fn generation(value: u64) -> KeyGeneration {
        KeyGeneration::new(value).expect("test generation")
    }

    fn write_journal(dir: &Path, name: &str, commits: &[TestCommit]) {
        let mut writer = JournalWriter::create(
            &dir.join(name),
            VaultId::from_bytes(VAULT),
            SESSION,
            generation(4),
        )
        .expect("create journal");
        for (index, gen_value, nonce, seed) in commits {
            let content = envelope_bytes(*seed);
            writer
                .append_commit(
                    *index,
                    generation(*gen_value),
                    *nonce,
                    content.len() as u64,
                    digest_of(*seed),
                )
                .expect("append commit");
        }
    }

    fn test_manifest(objects: Vec<ManifestObject>) -> ManifestPlaintext {
        let mut objects = objects;
        objects.sort_by(|a, b| {
            (a.kind() as u16, a.logical_id()).cmp(&(b.kind() as u16, b.logical_id()))
        });
        ManifestPlaintext::new(
            VaultId::from_bytes(VAULT),
            FreshnessEpoch::new(7).expect("test epoch"),
            ManifestHash::from_bytes([0x09; 32]),
            generation(4),
            (RotationPhase::None, None),
            vec![ManifestGeneration::new(
                generation(4),
                GenerationState::Active,
            )],
            objects,
        )
        .expect("test manifest valid")
    }

    fn blob(seed: u64, gen_value: u64, nonce: [u8; 24]) -> ManifestObject {
        ManifestObject::new(
            derive_media_logical_id(SESSION, seed),
            [0xB0 + seed as u8; 16],
            generation(gen_value),
            envelope_bytes(seed).len() as u64,
            digest_of(seed),
            ManifestAuthMetadata::GenericArtifactBlob { nonce },
        )
    }

    struct Fixture {
        _root: ReconcileDir,
        journals: PathBuf,
        envelopes: PathBuf,
    }

    fn build_fixture(journals: &[(&str, Vec<TestCommit>)], envelope_seeds: &[u64]) -> Fixture {
        let root = ReconcileDir::new();
        let journals_dir = root.path.join("journals");
        let envelopes_dir = root.path.join("envelopes");
        fs::create_dir_all(&journals_dir).expect("journals dir");
        fs::create_dir_all(&envelopes_dir).expect("envelopes dir");
        for (name, commits) in journals {
            write_journal(&journals_dir, name, commits);
        }
        for seed in envelope_seeds {
            fs::write(
                envelopes_dir.join(format!("chunk-{seed}.bin")),
                envelope_bytes(*seed),
            )
            .expect("envelope");
        }
        Fixture {
            _root: root,
            journals: journals_dir,
            envelopes: envelopes_dir,
        }
    }

    fn reconcile(fixture: &Fixture, manifest: &ManifestPlaintext) -> super::RecoveryReport {
        let scan = scan_journal_dir(&fixture.journals).expect("scan");
        let envelopes = inventory_envelopes(&fixture.envelopes).expect("envelopes");
        reconcile_recovery(&scan, &envelopes, manifest)
    }

    #[test]
    fn clean_recovery_binds_every_commit() {
        let fixture = build_fixture(
            &[("journal-0001.log", vec![(0, 4, N0, 0), (1, 4, N1, 1)])],
            &[0, 1],
        );
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let report = reconcile(&fixture, &manifest);
        assert!(report.is_clean());
        assert_eq!(report.verified().len(), 2);
        assert!(
            report
                .verified()
                .iter()
                .all(|commit| commit.manifest_bound())
        );
        assert!(
            report.verified()[0]
                .envelope_files()
                .contains(&OsString::from("chunk-0.bin"))
        );
    }

    #[test]
    fn manifest_gap_proposes_exact_binding() {
        let fixture = build_fixture(&[("journal-0001.log", vec![(0, 4, N0, 0)])], &[0]);
        let manifest = test_manifest(vec![]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert_eq!(report.verified().len(), 1);
        assert!(!report.verified()[0].manifest_bound());
        assert_eq!(report.unbound().len(), 1);
        let proposed = report.unbound()[0].proposed();
        assert_eq!(proposed.logical_id(), derive_media_logical_id(SESSION, 0));
        assert_eq!(proposed.key_generation(), generation(4));
        assert_eq!(proposed.ciphertext_length(), envelope_bytes(0).len() as u64);
        assert_eq!(proposed.ciphertext_sha256(), digest_of(0));
        assert_eq!(proposed.nonce(), N0);
    }

    #[test]
    fn missing_envelope_bytes_yield_orphan() {
        let fixture = build_fixture(
            &[("journal-0001.log", vec![(0, 4, N0, 0), (1, 4, N1, 1)])],
            &[0],
        );
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert_eq!(report.verified().len(), 1);
        assert_eq!(report.orphans().len(), 1);
        assert_eq!(report.orphans()[0].chunk_index(), 1);
        assert_eq!(report.orphans()[0].digest(), digest_of(1));
        assert!(report.unreferenced().is_empty());
        assert!(report.unlogged().is_empty());
    }

    #[test]
    fn duplicate_commits_report_first_by_file_seq() {
        let fixture = build_fixture(
            &[
                ("journal-0001.log", vec![(0, 4, N0, 0)]),
                ("journal-0002.log", vec![(0, 4, N1, 1)]),
            ],
            &[0, 1],
        );
        let manifest = test_manifest(vec![blob(0, 4, N0)]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert!(report.verified().is_empty());
        assert_eq!(report.duplicates().len(), 1);
        let duplicate = &report.duplicates()[0];
        assert_eq!(duplicate.chunk_index(), 0);
        assert_eq!(duplicate.canonical_digest(), digest_of(0));
        // Both duplicate-quoted digests count as committed: nothing is
        // unreferenced even though no commit verified.
        assert!(report.unreferenced().is_empty());
        let occurrences = duplicate.occurrences();
        assert_eq!(occurrences.len(), 2);
        assert_eq!(occurrences[0].file(), &OsString::from("journal-0001.log"));
        assert_eq!(occurrences[1].file(), &OsString::from("journal-0002.log"));
    }

    #[test]
    fn diverged_manifest_reports_expected_and_found() {
        let fixture = build_fixture(&[("journal-0001.log", vec![(0, 4, N0, 0)])], &[0]);
        let wrong = ManifestObject::new(
            derive_media_logical_id(SESSION, 0),
            [0xD0; 16],
            generation(4),
            envelope_bytes(0).len() as u64,
            [0xFF; 32],
            ManifestAuthMetadata::GenericArtifactBlob { nonce: N0 },
        );
        let manifest = test_manifest(vec![wrong]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert_eq!(report.verified().len(), 1);
        assert!(!report.verified()[0].manifest_bound());
        assert!(report.unbound().is_empty());
        assert_eq!(report.diverged().len(), 1);
        let diverged = &report.diverged()[0];
        assert_eq!(diverged.logical_id(), derive_media_logical_id(SESSION, 0));
        assert_eq!(diverged.expected_digest(), digest_of(0));
        assert_eq!(diverged.expected_nonce(), N0);
        assert_eq!(diverged.found().ciphertext_sha256(), [0xFF; 32]);
        assert_eq!(diverged.found().nonce(), N0);
    }

    #[test]
    fn unlogged_media_entry_reported() {
        const N2: [u8; 24] = [0xC2; 24];
        let fixture = build_fixture(&[("journal-0001.log", vec![(0, 4, N0, 0)])], &[0]);
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(5, 4, N2)]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert_eq!(report.verified().len(), 1);
        assert!(report.verified()[0].manifest_bound());
        assert_eq!(report.unlogged().len(), 1);
        assert_eq!(
            report.unlogged()[0].logical_id(),
            derive_media_logical_id(SESSION, 5)
        );
        assert_eq!(report.unlogged()[0].ciphertext_sha256(), digest_of(5));
    }

    #[test]
    fn unreferenced_envelope_reported() {
        let fixture = build_fixture(&[("journal-0001.log", vec![(0, 4, N0, 0)])], &[0, 9]);
        let manifest = test_manifest(vec![blob(0, 4, N0)]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert!(report.verified()[0].manifest_bound());
        assert_eq!(report.unreferenced().len(), 1);
        assert_eq!(
            report.unreferenced()[0].file(),
            &OsString::from("chunk-9.bin")
        );
        assert_eq!(report.unreferenced()[0].digest(), digest_of(9));
        assert_eq!(
            report.unreferenced()[0].length(),
            envelope_bytes(9).len() as u64
        );
    }

    #[test]
    fn torn_journal_prefix_still_reconciles() {
        let fixture = build_fixture(&[("journal-0001.log", vec![(0, 4, N0, 0)])], &[0]);
        let path = fixture.journals.join("journal-0001.log");
        let mut bytes = fs::read(&path).expect("read journal");
        bytes.extend_from_slice(b"TORN-TAIL-GARBAGE!!");
        fs::write(&path, &bytes).expect("tear tail");
        let manifest = test_manifest(vec![]);
        let report = reconcile(&fixture, &manifest);
        assert!(!report.is_clean());
        assert_eq!(report.torn().len(), 1);
        assert_eq!(report.torn()[0].file(), &OsString::from("journal-0001.log"));
        assert_eq!(report.torn()[0].valid_records(), 2);
        assert_eq!(report.verified().len(), 1);
        assert_eq!(report.unbound().len(), 1);
    }

    #[test]
    fn vault_mismatch_file_skipped() {
        let root = ReconcileDir::new();
        let journals_dir = root.path.join("journals");
        let envelopes_dir = root.path.join("envelopes");
        fs::create_dir_all(&journals_dir).expect("journals dir");
        fs::create_dir_all(&envelopes_dir).expect("envelopes dir");
        let mut writer = JournalWriter::create(
            &journals_dir.join("journal-0001.log"),
            VaultId::from_bytes([0x5A; 16]),
            SESSION,
            generation(4),
        )
        .expect("create journal");
        writer
            .append_commit(
                0,
                generation(4),
                N0,
                envelope_bytes(0).len() as u64,
                digest_of(0),
            )
            .expect("append commit");
        fs::write(envelopes_dir.join("chunk-0.bin"), envelope_bytes(0)).expect("envelope");
        let scan = scan_journal_dir(&journals_dir).expect("scan");
        let envelopes = inventory_envelopes(&envelopes_dir).expect("envelopes");
        let manifest = test_manifest(vec![]);
        let report = reconcile_recovery(&scan, &envelopes, &manifest);
        assert!(!report.is_clean());
        assert_eq!(report.vault_mismatches().len(), 1);
        assert_eq!(
            report.vault_mismatches()[0].file(),
            &OsString::from("journal-0001.log")
        );
        assert!(report.verified().is_empty());
        assert!(report.orphans().is_empty());
        assert!(report.torn().is_empty());
        assert_eq!(report.unreferenced().len(), 1);
    }

    #[test]
    fn empty_inputs_report_clean() {
        let fixture = build_fixture(&[], &[]);
        let manifest = test_manifest(vec![]);
        let report = reconcile(&fixture, &manifest);
        assert!(report.is_clean());
    }

    #[test]
    fn reconcile_is_deterministic() {
        let fixture = build_fixture(
            &[
                ("journal-0001.log", vec![(0, 4, N0, 0)]),
                ("journal-0002.log", vec![(0, 4, N1, 1)]),
            ],
            &[0, 1, 9],
        );
        let manifest = test_manifest(vec![blob(0, 4, N0)]);
        let first = reconcile(&fixture, &manifest);
        let second = reconcile(&fixture, &manifest);
        assert_eq!(first, second);
        assert!(!first.is_clean());
    }
}
