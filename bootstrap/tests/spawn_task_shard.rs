//! T1 RED — `cmd_spawn --task <mission-file>` for the bare-agent collapse arc.
//!
//! Terminal-tick of the /loop `recursively collapse until we can mirror spawn
//! the @torus collapse and inspect the diff together`. Reed inline RED per
//! pair-tick discipline.
//!
//! Composes with:
//! - Mara `b62c843` / `7978f84` (@shatter as bidirectional lens): mission
//!   enters as linear at winding (0,0); `@shatter(_, mission)` pulls a graph
//!   projection out of state space.
//! - `@peer-has-a-torus` (recognition landed 2026-07-07): peer receives intent
//!   at winding (0,0); envelope IS graph → linear at that winding; the mission
//!   IS linear → graph — the reverse direction of the same lens.
//! - Recognition #58 (Fate IS optical inference): @fate.roll would dispatch on
//!   the parsed mission at Phase H; v0 carries the mission in the envelope
//!   without invoking @fate — pieces 4-7 remain named stubs per the b10f00c
//!   structural-negatives contract.
//!
//! Substrate discipline: envelope grows exactly ONE field (`mission`). The
//! `--task <path>` sigil is filesystem-path-shaped (@f). String/inline missions
//! are a follow-up tick; MVP demands the path form so missions are git-
//! versionable and reviewable in-tree.
//!
//! Structural negatives honored:
//! - No subprocess. No @io/process. cmd_spawn just reads the mission file and
//!   carries the text into the envelope; the bare-agent invocation is a
//!   separate downstream concern (Agent tool at T5, not Rust).
//! - Backward compatibility: without --task, the envelope is byte-identical to
//!   today's spawn --hello-world output. `mission` is ABSENT, not null-valued.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_spawn(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "spawn".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

const TEST_PEER: &str = "bootstrap/tests/fixtures/spawn-test-peer";
const TEST_MISSION: &str = "bootstrap/tests/fixtures/spawn-task-mission.txt";

#[test]
fn t01_task_flag_accepted_with_existing_mission_file() {
    let out = run_spawn(&[TEST_PEER, "--task", TEST_MISSION, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spawn --task <existing-mission> --hello-world must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn t02_task_missing_file_errors_nonzero_with_readable_stderr() {
    let out = run_spawn(&[
        TEST_PEER,
        "--task",
        "bootstrap/tests/fixtures/definitely-not-here.txt",
        "--hello-world",
    ]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "spawn --task <nonexistent> must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = stderr.contains("mission") || stderr.contains("task") || stderr.contains("read");
    assert!(
        ok,
        "stderr must name mission-file / task / read failure; got:\n{}",
        stderr
    );
}

#[test]
fn t03_envelope_carries_mission_field_when_task_set() {
    let out = run_spawn(&[TEST_PEER, "--task", TEST_MISSION, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("envelope must be valid JSON");
    let mission = envelope["mission"].as_str().unwrap_or_else(|| {
        panic!(
            "envelope.mission must be a string when --task is set; got:\n{}",
            stdout
        )
    });
    // Fixture mission contains the distinctive substring "collapse the
    // substrate". Match on that so the assertion is content-anchored to the
    // fixture (any drift between fixture and assertion is caught here).
    assert!(
        mission.contains("collapse the substrate"),
        "envelope.mission must carry the mission-file contents; got: {:?}",
        mission
    );
}

#[test]
fn t04_no_mission_field_without_task_flag() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("envelope must be valid JSON");
    assert!(
        envelope.get("mission").is_none(),
        "spawn without --task must NOT include a 'mission' field; got:\n{}",
        envelope
    );
}

#[test]
fn t05_mission_composes_with_peer_recall_both_present() {
    let out = run_spawn(&[TEST_PEER, "--task", TEST_MISSION, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("envelope must be valid JSON");
    assert!(
        envelope["mission"].is_string(),
        "envelope.mission must be present as a string"
    );
    assert!(
        envelope["peer_recall"].is_object(),
        "envelope.peer_recall must remain an object (mission does not displace recall)"
    );
    assert!(
        envelope["peer_recall"]["cascade"].is_array(),
        "peer_recall.cascade must still be present alongside mission"
    );
    assert!(
        envelope["peer_recall"]["pack_trail"].is_array(),
        "peer_recall.pack_trail must still be present alongside mission"
    );
}

#[test]
fn t06_envelope_mission_and_composition_pieces_coexist() {
    // The seven composition_pieces from b10f00c must remain named alongside
    // the new mission field. Recognition #58's Fate-inference piece stays a
    // named stub — v0 does NOT dispatch through @fate on the mission text.
    let out = run_spawn(&[TEST_PEER, "--task", TEST_MISSION, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("envelope must be valid JSON");
    let pieces = envelope["composition_pieces"]
        .as_object()
        .expect("composition_pieces present");
    assert!(
        pieces.len() >= 7,
        "composition_pieces must still name all 7 pieces alongside mission; got {} keys",
        pieces.len()
    );
    assert!(
        envelope["mission"].is_string(),
        "envelope.mission present alongside composition_pieces"
    );
}
