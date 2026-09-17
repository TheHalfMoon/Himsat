//! 006C checkpoint derivation and chunk metadata binding.
//!
//! Read-only contract over the closed 005 substrate: journals plus a
//! [`RecoveryReport`](crate::vault_media_recovery::RecoveryReport) digest
//! into per-file session checkpoints, loss is accounted per session and
//! never silent, and chunk metadata maps adapter facts onto 005 fields.
//! A checkpoint seals only when the journal closed cleanly, its close
//! marker agrees, and every session commit verified bound; anything else
//! stays [`CheckpointStatus::Incomplete`] with the loss itemized. The
//! manifest writer that applies bindings stays caller-owned (005C).
//! Manifest-side unlogged entries carry no session and stay reported by
//! reconcile itself rather than by per-session loss.

use super::vault_media_recovery::{JournalScan, RecoveryReport};
use crate::vault::{KeyGeneration, VaultId};
use crate::vault_media_journal::TailStatus;
use himsat_events::{SessionId, SourceId};
use std::ffi::OsString;

/// Loss accounting for one session journal. Counts come from the
/// recovery report; flags come from the journal tail and close check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LossAccount {
    /// Commits with no envelope bytes.
    pub orphaned: u64,
    /// Chunk keys committed more than once.
    pub duplicated_keys: u64,
    /// Verified commits with no manifest binding.
    pub unbound: u64,
    /// Commits whose manifest entry disagrees.
    pub diverged: u64,
    /// The journal tail tore; the valid prefix still classified.
    pub torn: bool,
    /// The close marker disagrees with the replayed count.
    pub close_mismatch: bool,
}

impl LossAccount {
    /// Reports whether nothing was lost or left unbound.
    #[must_use]
    pub const fn is_clean(self) -> bool {
        self.orphaned == 0
            && self.duplicated_keys == 0
            && self.unbound == 0
            && self.diverged == 0
            && !self.torn
            && !self.close_mismatch
    }
}

/// Checkpoint status for one session journal file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointStatus {
    /// Closed cleanly with every commit verified bound.
    Sealed {
        /// Replayable commit records in the file.
        commits: u64,
        /// Of those, manifest-bound.
        bound: u64,
    },
    /// Anything else: open, torn, mismatched, orphaned, duplicated,
    /// unbound, or diverged. See the loss account.
    Incomplete,
}

/// Per-file session checkpoint with itemized loss.
#[derive(Clone, Debug, PartialEq)]
pub struct SessionCheckpoint {
    session: SessionId,
    journal_file: OsString,
    status: CheckpointStatus,
    loss: LossAccount,
}

impl SessionCheckpoint {
    /// Returns the owning session identity.
    #[must_use]
    pub const fn session(&self) -> SessionId {
        self.session
    }

    /// Returns the opaque journal file (never parsed).
    #[must_use]
    pub fn journal_file(&self) -> &OsString {
        &self.journal_file
    }

    /// Returns the checkpoint status.
    #[must_use]
    pub const fn status(&self) -> CheckpointStatus {
        self.status
    }

    /// Returns the itemized loss account.
    #[must_use]
    pub const fn loss(&self) -> LossAccount {
        self.loss
    }
}

/// Portable chunk codec tag. Adapters own the bytes; the core only
/// labels them for routing and health display.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChunkCodec {
    /// 16-bit PCM frames.
    Pcm16,
    /// Opus packets.
    Opus,
    /// AAC frames.
    Aac,
    /// Anything else, adapter-defined.
    Unknown,
}

/// Chunk metadata binding adapter facts onto 005 fields.
///
/// Field mapping (no new 005 format is introduced):
///
/// - session, chunk index, generation, nonce, plaintext length, digest
///   mirror the 005A commit record;
/// - digest and byte length identify the 005A envelope file;
/// - session, generation, digest, and length address the manifest blob
///   inventory through the 005C derived logical id;
/// - timestamps, codec, and source are adapter facts carried alongside
///   for 006B consumers and 007+ adapters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkMetadata {
    /// Owning session identity.
    pub session: SessionId,
    /// Chunk index within the session.
    pub chunk_index: u64,
    /// Key generation quoted by the commit.
    pub generation: KeyGeneration,
    /// Commit nonce.
    pub nonce: [u8; 24],
    /// Plaintext length quoted by the commit.
    pub plaintext_len: u64,
    /// Envelope digest quoted by the commit.
    pub digest: [u8; 32],
    /// Adapter timestamp range start, session-relative milliseconds.
    pub timestamp_start_ms: u64,
    /// Adapter timestamp range end, session-relative milliseconds.
    pub timestamp_end_ms: u64,
    /// Portable codec tag.
    pub codec: ChunkCodec,
    /// Source identity from the 006A descriptor.
    pub source: SourceId,
}

/// Derives per-file session checkpoints from a scan plus its recovery
/// report. Journals with no records are skipped (nothing to seal);
/// journals bound to another vault are skipped (the report already
/// lists them as mismatches). Output follows scan (file-name) order.
#[must_use]
pub fn derive_checkpoints(
    scan: &JournalScan,
    report: &RecoveryReport,
    manifest_vault: VaultId,
) -> Vec<SessionCheckpoint> {
    let mut checkpoints = Vec::new();
    for journal in scan.journals() {
        let records = journal.records();
        let Some(first) = records.first() else {
            continue;
        };
        if first.vault_id() != manifest_vault {
            continue;
        }
        let session_raw = first.session_id();
        let session = SessionId::new(u128::from_be_bytes(session_raw));
        let mut commits = Vec::new();
        let mut close_claim = None;
        for record in records {
            if let Some(claimed) = record.kind().as_close_count() {
                close_claim = Some(claimed);
                continue;
            }
            if let Some(commit) = record.kind().as_commit() {
                commits.push(commit.chunk_index);
            }
        }
        if commits.is_empty() && close_claim.is_none() {
            continue;
        }
        let close_agrees = matches!(journal.tail(), TailStatus::CleanEof)
            && matches!(close_claim, Some(claimed) if claimed == commits.len() as u64);
        let bound = report
            .verified()
            .iter()
            .filter(|commit| {
                commit.session_id() == session_raw
                    && commit.manifest_bound()
                    && commits.contains(&commit.chunk_index())
            })
            .count() as u64;
        let loss = LossAccount {
            orphaned: report
                .orphans()
                .iter()
                .filter(|orphan| orphan.session_id() == session_raw)
                .count() as u64,
            duplicated_keys: report
                .duplicates()
                .iter()
                .filter(|duplicate| duplicate.session_id() == session_raw)
                .count() as u64,
            unbound: report
                .verified()
                .iter()
                .filter(|commit| commit.session_id() == session_raw && !commit.manifest_bound())
                .count() as u64,
            diverged: report
                .diverged()
                .iter()
                .filter(|diverged| diverged.session_id() == session_raw)
                .count() as u64,
            torn: report
                .torn()
                .iter()
                .any(|torn| torn.file() == journal.file_name()),
            close_mismatch: report
                .close_counts()
                .iter()
                .any(|mismatch| mismatch.file() == journal.file_name()),
        };
        let status = if close_agrees
            && bound == commits.len() as u64
            && loss.orphaned == 0
            && loss.duplicated_keys == 0
            && loss.unbound == 0
            && loss.diverged == 0
            && !loss.torn
            && !loss.close_mismatch
        {
            CheckpointStatus::Sealed {
                commits: commits.len() as u64,
                bound,
            }
        } else {
            CheckpointStatus::Incomplete
        };
        checkpoints.push(SessionCheckpoint {
            session,
            journal_file: journal.file_name().clone(),
            status,
            loss,
        });
    }
    checkpoints
}

/// Returns the latest sealed checkpoint for a session, if any. Input
/// order is scan (file-name) order, so the last sealed entry wins.
#[must_use]
pub fn latest_sealed_for(
    session: SessionId,
    checkpoints: &[SessionCheckpoint],
) -> Option<&SessionCheckpoint> {
    checkpoints.iter().rfind(|checkpoint| {
        checkpoint.session() == session
            && matches!(checkpoint.status(), CheckpointStatus::Sealed { .. })
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CheckpointStatus, ChunkCodec, ChunkMetadata, derive_checkpoints, latest_sealed_for,
    };
    use crate::vault::{FreshnessEpoch, KeyGeneration, ManifestHash, VaultId};
    use crate::vault_manifest::{
        GenerationState, ManifestAuthMetadata, ManifestGeneration, ManifestObject,
        ManifestPlaintext, RotationPhase,
    };
    use crate::vault_media_journal::{JournalWriter, envelope_digest};
    use crate::vault_media_recovery::{
        derive_media_logical_id, inventory_envelopes, reconcile_recovery, scan_journal_dir,
    };
    use himsat_events::{SessionId, SourceId};
    use sha2::{Digest, Sha256};
    use std::ffi::OsString;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static CHECKPOINT_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct CheckpointDir {
        path: PathBuf,
    }

    impl CheckpointDir {
        fn new() -> Self {
            let id = CHECKPOINT_COUNTER.fetch_add(1, Ordering::SeqCst);
            let path =
                std::env::temp_dir().join(format!("himsat-checkpoint-{}-{id}", std::process::id()));
            fs::create_dir_all(&path).expect("checkpoint temp dir");
            Self { path }
        }
    }

    impl Drop for CheckpointDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    const VAULT: [u8; 16] = [0xA5; 16];
    const SESSION: [u8; 16] = [0x0A; 16];
    const SESSION_ID: SessionId = SessionId::new(u128::from_be_bytes(SESSION));
    const N0: [u8; 24] = [0xC0; 24];
    const N1: [u8; 24] = [0xC1; 24];

    type TestCommit = (u64, u64, [u8; 24], u64);

    fn envelope_bytes(seed: u64) -> Vec<u8> {
        let mut bytes = format!("checkpoint-envelope-{seed}-payload").into_bytes();
        bytes.resize(200, 0xE0);
        bytes
    }

    fn digest_of(seed: u64) -> [u8; 32] {
        envelope_digest(&envelope_bytes(seed))
    }

    fn generation(value: u64) -> KeyGeneration {
        KeyGeneration::new(value).expect("test generation")
    }

    fn open_writer(dir: &Path, name: &str, vault: [u8; 16]) -> JournalWriter {
        JournalWriter::create(
            &dir.join(name),
            VaultId::from_bytes(vault),
            SESSION,
            generation(4),
        )
        .expect("create journal")
    }

    fn append_commits(writer: &mut JournalWriter, commits: &[TestCommit]) {
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

    fn write_open(dir: &Path, name: &str, commits: &[TestCommit]) {
        let mut writer = open_writer(dir, name, VAULT);
        append_commits(&mut writer, commits);
    }

    fn write_closed(dir: &Path, name: &str, commits: &[TestCommit], claimed: u64) {
        let mut writer = open_writer(dir, name, VAULT);
        append_commits(&mut writer, commits);
        writer.append_close(claimed).expect("close journal");
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
        _root: CheckpointDir,
        journals: PathBuf,
        envelopes: PathBuf,
    }

    fn fixture() -> Fixture {
        let root = CheckpointDir::new();
        let journals = root.path.join("journals");
        let envelopes = root.path.join("envelopes");
        fs::create_dir_all(&journals).expect("journals dir");
        fs::create_dir_all(&envelopes).expect("envelopes dir");
        Fixture {
            _root: root,
            journals,
            envelopes,
        }
    }

    fn seal_envelope(dir: &Path, seed: u64) {
        fs::write(dir.join(format!("chunk-{seed}.bin")), envelope_bytes(seed)).expect("envelope");
    }

    fn derive(fixture: &Fixture, manifest: &ManifestPlaintext) -> Vec<super::SessionCheckpoint> {
        let scan = scan_journal_dir(&fixture.journals).expect("scan");
        let envelopes = inventory_envelopes(&fixture.envelopes).expect("envelopes");
        let report = reconcile_recovery(&scan, &envelopes, manifest);
        derive_checkpoints(&scan, &report, VaultId::from_bytes(VAULT))
    }

    #[test]
    fn sealed_checkpoint_happy_path() {
        let fixture = fixture();
        write_closed(
            &fixture.journals,
            "journal-0001.log",
            &[(0, 4, N0, 0), (1, 4, N1, 1)],
            2,
        );
        seal_envelope(&fixture.envelopes, 0);
        seal_envelope(&fixture.envelopes, 1);
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let checkpoints = derive(&fixture, &manifest);
        assert_eq!(checkpoints.len(), 1);
        assert_eq!(checkpoints[0].session(), SESSION_ID);
        assert_eq!(
            checkpoints[0].journal_file(),
            &OsString::from("journal-0001.log")
        );
        assert_eq!(
            checkpoints[0].status(),
            CheckpointStatus::Sealed {
                commits: 2,
                bound: 2
            }
        );
        assert!(checkpoints[0].loss().is_clean());
        assert!(latest_sealed_for(checkpoints[0].session(), &checkpoints).is_some());
    }

    #[test]
    fn open_journal_is_incomplete_but_lossless() {
        let fixture = fixture();
        write_open(&fixture.journals, "journal-0001.log", &[(0, 4, N0, 0)]);
        seal_envelope(&fixture.envelopes, 0);
        let manifest = test_manifest(vec![blob(0, 4, N0)]);
        let checkpoints = derive(&fixture, &manifest);
        assert_eq!(checkpoints.len(), 1);
        assert_eq!(checkpoints[0].status(), CheckpointStatus::Incomplete);
        assert!(checkpoints[0].loss().is_clean());
        assert!(latest_sealed_for(checkpoints[0].session(), &checkpoints).is_none());
    }

    #[test]
    fn torn_journal_marks_loss() {
        let fixture = fixture();
        write_open(
            &fixture.journals,
            "journal-0001.log",
            &[(0, 4, N0, 0), (1, 4, N1, 1)],
        );
        let path = fixture.journals.join("journal-0001.log");
        let mut bytes = fs::read(&path).expect("read journal");
        bytes.extend_from_slice(b"TORN-TAIL-GARBAGE!!");
        fs::write(&path, &bytes).expect("tear tail");
        seal_envelope(&fixture.envelopes, 0);
        seal_envelope(&fixture.envelopes, 1);
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let checkpoints = derive(&fixture, &manifest);
        assert_eq!(checkpoints.len(), 1);
        assert_eq!(checkpoints[0].status(), CheckpointStatus::Incomplete);
        assert!(checkpoints[0].loss().torn);
        assert_eq!(checkpoints[0].loss().orphaned, 0);
    }

    #[test]
    fn orphan_taints_checkpoint() {
        let fixture = fixture();
        write_closed(
            &fixture.journals,
            "journal-0001.log",
            &[(0, 4, N0, 0), (1, 4, N1, 1)],
            2,
        );
        seal_envelope(&fixture.envelopes, 0);
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let checkpoints = derive(&fixture, &manifest);
        assert_eq!(checkpoints.len(), 1);
        assert_eq!(checkpoints[0].status(), CheckpointStatus::Incomplete);
        assert_eq!(checkpoints[0].loss().orphaned, 1);
        assert!(!checkpoints[0].loss().is_clean());
    }

    #[test]
    fn close_mismatch_taints_checkpoint() {
        use crate::vault_media_journal::{
            JOURNAL_CLOSE_PAYLOAD_BYTES, JOURNAL_HEADER_BYTES, JOURNAL_INTEGRITY_BYTES,
        };
        let fixture = fixture();
        write_closed(
            &fixture.journals,
            "journal-0001.log",
            &[(0, 4, N0, 0), (1, 4, N1, 1)],
            2,
        );
        seal_envelope(&fixture.envelopes, 0);
        seal_envelope(&fixture.envelopes, 1);
        let path = fixture.journals.join("journal-0001.log");
        let mut bytes = fs::read(&path).expect("read journal");
        let record_len =
            JOURNAL_HEADER_BYTES + JOURNAL_CLOSE_PAYLOAD_BYTES + JOURNAL_INTEGRITY_BYTES;
        let record_at = bytes.len() - record_len;
        let payload_at = bytes.len() - JOURNAL_INTEGRITY_BYTES - JOURNAL_CLOSE_PAYLOAD_BYTES;
        bytes[payload_at..payload_at + JOURNAL_CLOSE_PAYLOAD_BYTES]
            .copy_from_slice(&9u64.to_be_bytes());
        let digest = Sha256::digest(&bytes[record_at..bytes.len() - JOURNAL_INTEGRITY_BYTES]);
        let trailer_at = bytes.len() - JOURNAL_INTEGRITY_BYTES;
        bytes[trailer_at..].copy_from_slice(&digest);
        fs::write(&path, &bytes).expect("rewrite journal");
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let checkpoints = derive(&fixture, &manifest);
        assert_eq!(checkpoints.len(), 1);
        assert_eq!(checkpoints[0].status(), CheckpointStatus::Incomplete);
        assert!(checkpoints[0].loss().close_mismatch);
    }

    #[test]
    fn open_only_and_foreign_journals_are_skipped() {
        let fixture = fixture();
        write_open(&fixture.journals, "journal-0001.log", &[]);
        let mut writer = open_writer(&fixture.journals, "journal-0002.log", [0x5A; 16]);
        append_commits(&mut writer, &[(0, 4, N0, 0)]);
        let manifest = test_manifest(vec![]);
        let checkpoints = derive(&fixture, &manifest);
        assert!(checkpoints.is_empty());
    }

    #[test]
    fn metadata_mapping_round_trip() {
        let metadata = ChunkMetadata {
            session: SESSION_ID,
            chunk_index: 3,
            generation: generation(4),
            nonce: N0,
            plaintext_len: 200,
            digest: digest_of(3),
            timestamp_start_ms: 1_000,
            timestamp_end_ms: 2_000,
            codec: ChunkCodec::Opus,
            source: SourceId::new(10),
        };
        assert_eq!(metadata.chunk_index, 3);
        assert_eq!(
            metadata.timestamp_end_ms - metadata.timestamp_start_ms,
            1_000
        );
        assert_eq!(metadata.codec, ChunkCodec::Opus);
        assert_eq!(metadata.digest, digest_of(3));
    }

    #[test]
    fn latest_sealed_picks_last_file() {
        let fixture = fixture();
        write_closed(&fixture.journals, "journal-0001.log", &[(0, 4, N0, 0)], 1);
        write_closed(&fixture.journals, "journal-0002.log", &[(1, 4, N1, 1)], 1);
        seal_envelope(&fixture.envelopes, 0);
        seal_envelope(&fixture.envelopes, 1);
        let manifest = test_manifest(vec![blob(0, 4, N0), blob(1, 4, N1)]);
        let checkpoints = derive(&fixture, &manifest);
        assert_eq!(checkpoints.len(), 2);
        let latest =
            latest_sealed_for(checkpoints[0].session(), &checkpoints).expect("latest sealed");
        assert_eq!(latest.journal_file(), &OsString::from("journal-0002.log"));
    }
}
