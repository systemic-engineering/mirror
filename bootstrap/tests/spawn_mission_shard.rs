//! Tick 2 RED — `cmd_spawn --mission <mission-file>` substrate-honest alias.
//!
//! /loop close @torus to rest, Tick 2. Substrate-decl at mirror.spec
//! `1e45c50` declares `flag mission: ~f` on `command spawn`. The
//! substrate-honest name is `mission` (matches @song/movement's frame-
//! entry semantics); today's binary arg-parse uses `--task` as a
//! legacy carryover from T1 GREEN. This RED lands the alias
//! obligation: `--mission <path>` MUST work identically to `--task
//! <path>`. `--task` stays as a backward-compat alias per two-tick
//! discipline (both accepted for one release cycle, then --task
//! deprecates).
//!
//! Structure mirrors bootstrap/tests/spawn_task_shard.rs t01-t06 with
//! `--task` replaced by `--mission`. All six tests should fail against
//! the current binary (only --task is parsed); all six should pass
//! after the GREEN tick lands the alias.

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
fn t01_mission_flag_accepted_with_existing_mission_file() {
    let out = run_spawn(&[TEST_PEER, "--mission", TEST_MISSION, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spawn --mission <existing-mission> --hello-world must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn t02_mission_missing_file_errors_nonzero_with_readable_stderr() {
    let out = run_spawn(&[
        TEST_PEER,
        "--mission",
        "bootstrap/tests/fixtures/definitely-not-here.txt",
        "--hello-world",
    ]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "spawn --mission <nonexistent> must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = stderr.contains("mission") || stderr.contains("read");
    assert!(
        ok,
        "stderr must name mission-file / read failure; got:\n{}",
        stderr
    );
}

#[test]
fn t03_envelope_carries_mission_field_when_mission_set() {
    let out = run_spawn(&[TEST_PEER, "--mission", TEST_MISSION, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("envelope must be valid JSON");
    let mission = envelope["mission"].as_str().unwrap_or_else(|| {
        panic!(
            "envelope.mission must be a string when --mission is set; got:\n{}",
            stdout
        )
    });
    assert!(
        mission.contains("collapse the substrate"),
        "envelope.mission must carry the mission-file contents; got: {:?}",
        mission
    );
}

#[test]
fn t04_no_mission_field_without_mission_flag() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("envelope must be valid JSON");
    assert!(
        envelope.get("mission").is_none(),
        "spawn without --mission must NOT include a 'mission' field; got:\n{}",
        envelope
    );
}

#[test]
fn t05_mission_composes_with_peer_recall_both_present() {
    let out = run_spawn(&[TEST_PEER, "--mission", TEST_MISSION, "--hello-world"]);
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
fn t06_task_alias_still_accepted_backward_compat() {
    // --task remains a valid alias for --mission per two-tick discipline.
    // Both flags produce byte-identical envelopes when given the same file.
    let out_mission = run_spawn(&[TEST_PEER, "--mission", TEST_MISSION, "--hello-world"]);
    let out_task = run_spawn(&[TEST_PEER, "--task", TEST_MISSION, "--hello-world"]);
    assert_eq!(
        out_mission.status.code(),
        Some(0),
        "--mission path must exit 0"
    );
    assert_eq!(
        out_task.status.code(),
        Some(0),
        "--task path (backward-compat alias) must still exit 0"
    );
    let e_mission: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out_mission.stdout).trim())
            .expect("mission envelope valid JSON");
    let e_task: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out_task.stdout).trim())
            .expect("task envelope valid JSON");
    assert_eq!(
        e_mission["mission"], e_task["mission"],
        "envelope.mission must be byte-identical whether --mission or --task was used"
    );
}
