//! 005D kill-style fault injection for the media journal.
//!
//! A real operating-system kill (not a hand-truncated fixture) strikes a
//! writer child mid-stream; the parent then replays and reconciles. The
//! kill point is nondeterministic by nature, so every assertion below is
//! timing-independent: per-record `sync_all` means a kill can only tear
//! the single in-flight tail record, and all complete records must
//! classify exactly. Any kill timing satisfies the same bounds, which is
//! the bounded-loss property under test.
//!
//! The child is this same test binary re-spawned with an exact filter
//! onto `kill_writer_entry` (marked ignored so normal runs skip it).

use himsat_core::vault::{FreshnessEpoch, KeyGeneration, ManifestHash, VaultId};
use himsat_core::vault_manifest::{
    GenerationState, ManifestGeneration, ManifestPlaintext, RotationPhase,
};
use himsat_core::vault_media_journal::{JournalWriter, envelope_digest};
use himsat_core::vault_media_recovery::{
    inventory_envelopes, reconcile_recovery, scan_journal_dir,
};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const VAULT: [u8; 16] = [0xD1; 16];
const SESSION: [u8; 16] = [0xD2; 16];
const GENERATION: u64 = 7;
const COMMITS: u64 = 8;
const KILL_AT: u64 = 3;
const POLL_DEADLINE: Duration = Duration::from_secs(60);

static KILL_COUNTER: AtomicU64 = AtomicU64::new(0);

struct KillDir {
    path: PathBuf,
}

impl KillDir {
    fn new() -> Self {
        let id = KILL_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!("himsat-kill-{}-{id}", std::process::id()));
        fs::create_dir_all(&path).expect("kill temp dir");
        Self { path }
    }
}

impl Drop for KillDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn envelope_bytes(index: u64) -> Vec<u8> {
    let mut bytes = format!("kill-envelope-{index:04}-payload").into_bytes();
    bytes.resize(200, 0xE0);
    bytes
}

fn generation() -> KeyGeneration {
    KeyGeneration::new(GENERATION).expect("kill generation")
}

fn empty_manifest() -> ManifestPlaintext {
    ManifestPlaintext::new(
        VaultId::from_bytes(VAULT),
        FreshnessEpoch::new(7).expect("kill epoch"),
        ManifestHash::from_bytes([0x09; 32]),
        generation(),
        (RotationPhase::None, None),
        vec![ManifestGeneration::new(
            generation(),
            GenerationState::Active,
        )],
        vec![],
    )
    .expect("kill manifest valid")
}

/// Crash-writer child entry. Normal runs skip it (ignored); the harness
/// below re-spawns this binary onto it with the kill protocol armed.
#[test]
#[ignore = "spawned by kill_mid_stream_recovery_stays_bounded with HIMSAT_KILL_CHILD=1"]
fn kill_writer_entry() {
    if std::env::var("HIMSAT_KILL_CHILD").as_deref() != Ok("1") {
        return;
    }
    let journals = PathBuf::from(std::env::var("HIMSAT_KILL_JOURNALS").expect("journals dir"));
    let envelopes = PathBuf::from(std::env::var("HIMSAT_KILL_ENVELOPES").expect("envelopes dir"));
    let progress = PathBuf::from(std::env::var("HIMSAT_KILL_PROGRESS").expect("progress file"));
    let commits: u64 = std::env::var("HIMSAT_KILL_COMMITS")
        .expect("commit count")
        .parse()
        .expect("commit count parses");
    let mut writer = JournalWriter::create(
        &journals.join("journal-0001.log"),
        VaultId::from_bytes(VAULT),
        SESSION,
        generation(),
    )
    .expect("child creates journal");
    for index in 0..commits {
        // Realistic pipeline order: seal and sync the envelope bytes
        // before quoting their digest in the commit record.
        let content = envelope_bytes(index);
        let envelope_path = envelopes.join(format!("chunk-{index:04}.bin"));
        fs::write(&envelope_path, &content).expect("child writes envelope");
        File::options()
            .write(true)
            .open(&envelope_path)
            .expect("child reopens envelope")
            .sync_all()
            .expect("child syncs envelope");
        writer
            .append_commit(
                index,
                generation(),
                [index as u8; 24],
                content.len() as u64,
                envelope_digest(&content),
            )
            .expect("child appends commit");
        fs::write(&progress, (index + 1).to_string()).expect("child writes progress");
        File::options()
            .write(true)
            .open(&progress)
            .expect("child reopens progress")
            .sync_all()
            .expect("child syncs progress");
    }
}

fn spawn_writer(root: &Path) -> Child {
    Command::new(std::env::current_exe().expect("test binary path"))
        .args(["kill_writer_entry", "--exact", "--nocapture", "--ignored"])
        .env("HIMSAT_KILL_CHILD", "1")
        .env("HIMSAT_KILL_JOURNALS", root.join("journals"))
        .env("HIMSAT_KILL_ENVELOPES", root.join("envelopes"))
        .env("HIMSAT_KILL_PROGRESS", root.join("progress"))
        .env("HIMSAT_KILL_COMMITS", COMMITS.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn crash writer")
}

fn wait_for_progress(progress: &Path, kill_at: u64) {
    let deadline = Instant::now() + POLL_DEADLINE;
    loop {
        let done = fs::read_to_string(progress)
            .ok()
            .and_then(|text| text.trim().parse::<u64>().ok())
            .unwrap_or(0);
        if done >= kill_at {
            return;
        }
        assert!(Instant::now() < deadline, "crash writer stalled");
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn kill_mid_stream_recovery_stays_bounded() {
    let root = KillDir::new();
    let journals_dir = root.path.join("journals");
    let envelopes_dir = root.path.join("envelopes");
    fs::create_dir_all(&journals_dir).expect("journals dir");
    fs::create_dir_all(&envelopes_dir).expect("envelopes dir");

    let mut child = spawn_writer(&root.path);
    wait_for_progress(&root.path.join("progress"), KILL_AT);
    child.kill().expect("kill lands");
    let _ = child.wait();

    // Recovery itself must never fail on kill-torn inputs.
    let scan = scan_journal_dir(&journals_dir).expect("scan succeeds after kill");
    let envelopes = inventory_envelopes(&envelopes_dir).expect("inventory succeeds");
    let report = reconcile_recovery(&scan, &envelopes, &empty_manifest());

    // At most the single in-flight tail record can tear.
    assert!(report.torn().len() <= 1);
    // No retry path exists, so no key can duplicate.
    assert!(report.duplicates().is_empty());
    // Progress proved three commits durable; at most all eight ran.
    let classified = report.verified().len() + report.orphans().len();
    assert!((KILL_AT as usize) <= classified && classified <= COMMITS as usize);
    // Every classified commit is one of ours.
    for commit in report.verified() {
        assert!(commit.chunk_index() < COMMITS);
    }
    for orphan in report.orphans() {
        assert!(orphan.chunk_index() < COMMITS);
    }
    // Stray bytes are only ours: kills between envelope seal and commit.
    for envelope in report.unreferenced() {
        let name = envelope.file().to_str().expect("fixture name is utf8");
        let index: u64 = name
            .strip_prefix("chunk-")
            .and_then(|rest| rest.strip_suffix(".bin"))
            .and_then(|digits| digits.parse().ok())
            .expect("fixture envelope name");
        assert!(index < COMMITS);
    }
    // The journal never closed and the manifest holds nothing media.
    assert!(report.close_counts().is_empty());
    assert!(report.diverged().is_empty());
    assert!(report.unlogged().is_empty());
    assert!(report.vault_mismatches().is_empty());
    assert!(!report.is_clean());
}

/// Bounded-loss proof, executable: a kill at every commit point keeps
/// recovery inside the same bounds. Progress proving `k` commits durable
/// means at least `k` complete records must classify and at most all
/// eight ran, whatever the kill timing around them.
#[test]
fn kill_matrix_covers_every_commit_point() {
    for kill_at in 0..COMMITS {
        let root = KillDir::new();
        let journals_dir = root.path.join("journals");
        let envelopes_dir = root.path.join("envelopes");
        fs::create_dir_all(&journals_dir).expect("journals dir");
        fs::create_dir_all(&envelopes_dir).expect("envelopes dir");

        let mut child = spawn_writer(&root.path);
        wait_for_progress(&root.path.join("progress"), kill_at);
        child.kill().expect("kill lands");
        let _ = child.wait();

        let scan = scan_journal_dir(&journals_dir).expect("scan succeeds");
        let envelopes = inventory_envelopes(&envelopes_dir).expect("inventory succeeds");
        let report = reconcile_recovery(&scan, &envelopes, &empty_manifest());

        assert!(report.torn().len() <= 1, "kill point {kill_at}");
        assert!(report.duplicates().is_empty(), "kill point {kill_at}");
        let classified = report.verified().len() + report.orphans().len();
        assert!(
            (kill_at as usize) <= classified && classified <= COMMITS as usize,
            "kill point {kill_at}: {classified} classified"
        );
        for commit in report.verified() {
            assert!(commit.chunk_index() < COMMITS, "kill point {kill_at}");
        }
        for orphan in report.orphans() {
            assert!(orphan.chunk_index() < COMMITS, "kill point {kill_at}");
        }
    }
}
