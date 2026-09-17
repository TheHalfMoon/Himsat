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
