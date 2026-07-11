//! Blocker 2 v1 autopoietic closure RED — `mirror peer beam <home>
//! --integrate-diff` persists received bytes as new moment in
//! peer_home/.bauchladen/, closing the operator-feedback loop.
//!
//! Substrate authority:
//! - Mara `7e5c298` iter-26 @optics/lens/diff put action +
//!   autopoietic_closure bilateral
//! - Mara `2c26537` iter-31 psychohistory_sheaf substrate-decl'd
//!   at @song/narrative
//! - Mara `ce9745f` iter-32 bounded_by(sheaf) at @fate/tournament
//! - Reed v1 empirical spawn (fate-bounded-from-psychohistory, most
//!   recent commit) — different peer content produces different
//!   decisions
//!
//! The loop that closes:
//!   1. peer emits diff via --fate-select --from-psychohistory (D1)
//!   2. operator edits diff → E1
//!   3. --integrate-diff receives E1, PERSISTS as new moment file in
//!      peer_home/.bauchladen/moment_<delta_oid>.mirror
//!   4. next --fate-select --from-psychohistory reads updated
//!      psychohistory (C1 + E1) → different root_oid → different
//!      bounded_by weights → different decision (D2 ≠ D1 in general)
//!
//! This is the autopoietic closure Mara iter-26 declared:
//! operator-edit integration back into @bauchladen.tray updates
//! peer state at the storage layer. The next inference reads through
//! the updated content-addressed graph without any explicit "update
//! weights" step — the sheaf IS the weights source.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn mirror_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mirror"))
}

fn make_peer_home() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!(
        "mirror-peer-beam-autopoietic-{}-{}",
        std::process::id(),
        ts
    ));
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    std::fs::write(
        dir.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");
    std::fs::write(dir.join("initial_moment.txt"), "initial observation\n").expect("write initial");
    dir
}

fn fate_select_from_psychohistory(dir: &std::path::Path) -> Output {
    Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("peer")
        .arg("beam")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .arg("--fate-select")
        .arg("--from-psychohistory")
        .output()
        .expect("mirror peer beam --fate-select --from-psychohistory")
}

fn integrate_diff(dir: &std::path::Path, edit_bytes: &[u8]) -> Output {
    let mut child = Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("peer")
        .arg("beam")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .arg("--integrate-diff")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn integrate-diff");
    {
        let stdin = child.stdin.as_mut().expect("child stdin");
        stdin.write_all(edit_bytes).expect("write edit bytes");
    }
    child.wait_with_output().expect("child output")
}

fn extract_root(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        let t = line.trim_start_matches("+").trim();
        if let Some(rest) = t.strip_prefix("psychohistory_root_oid:") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

fn extract_decision(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        let t = line.trim_start_matches("+").trim();
        if let Some(rest) = t.strip_prefix("fate_decision:") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

// === T1: --integrate-diff persists received bytes to .bauchladen/ =====
//
// Post-GREEN: envelope names bauchladen_moment_path (or similar), and
// a file MUST exist at peer_home/.bauchladen/ containing the received
// edit bytes. Pre-GREEN: --integrate-diff acknowledges but does not
// persist — no file created.

#[test]
fn t01_integrate_diff_persists_moment_to_bauchladen() {
    let dir = make_peer_home();
    let edit_bytes = b"+ operator edit: prefer smaller diffs\n";
    let out = integrate_diff(&dir, edit_bytes);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    assert!(
        out.status.success(),
        "T1: --integrate-diff must exit 0; stdout=<{stdout}>"
    );

    let bauchladen_dir = dir.join(".bauchladen");
    let dir_exists = bauchladen_dir.is_dir();

    let file_count = std::fs::read_dir(&bauchladen_dir)
        .map(|r| r.filter_map(|e| e.ok()).count())
        .unwrap_or(0);

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        dir_exists,
        "T1: peer_home/.bauchladen/ dir MUST exist after --integrate-diff persists a moment"
    );
    assert!(
        file_count >= 1,
        "T1: peer_home/.bauchladen/ MUST contain at least one moment file after \
         --integrate-diff persists received bytes; found {file_count} files"
    );
}

// === T2: envelope names the bauchladen_moment persistence ============

#[test]
fn t02_integrate_diff_envelope_names_bauchladen_moment() {
    let dir = make_peer_home();
    let out = integrate_diff(&dir, b"+ another edit\n");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        stdout.contains("bauchladen_moment") || stdout.contains(".bauchladen"),
        "T2: stdout must name bauchladen_moment persistence path; got: <{stdout}>"
    );
}

// === T3: autopoietic closure — integrate-diff changes next decision ===
//
// The load-bearing test: persist edit → next --fate-select reads
// updated psychohistory → different root_oid → different decision.
// This IS the autopoietic loop closure.

#[test]
fn t03_autopoietic_closure_changes_next_decision() {
    let dir = make_peer_home();

    // Iteration 1: capture decision D1 + root R1.
    let out1 = fate_select_from_psychohistory(&dir);
    let stdout1 = String::from_utf8_lossy(&out1.stdout).to_string();
    let root1 = extract_root(&stdout1).unwrap_or_default();
    let decision1 = extract_decision(&stdout1).unwrap_or_default();

    // Operator edit persists via --integrate-diff.
    let _out_integrate = integrate_diff(&dir, b"+ substantive operator feedback\n");

    // Iteration 2: capture decision D2 + root R2.
    let out2 = fate_select_from_psychohistory(&dir);
    let stdout2 = String::from_utf8_lossy(&out2.stdout).to_string();
    let root2 = extract_root(&stdout2).unwrap_or_default();
    let decision2 = extract_decision(&stdout2).unwrap_or_default();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !root1.is_empty() && !root2.is_empty(),
        "T3: both iterations must emit psychohistory_root_oid; \
         r1=<{root1}> r2=<{root2}>"
    );

    assert_ne!(
        root1, root2,
        "T3: AUTOPOIETIC CLOSURE — after --integrate-diff persists a new moment, \
         the next --fate-select --from-psychohistory MUST see a different \
         psychohistory_root_oid (peer's history changed). r1=<{root1}> r2=<{root2}> \
         d1=<{decision1}> d2=<{decision2}>"
    );
}
