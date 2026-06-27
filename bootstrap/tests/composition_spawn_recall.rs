//! P5 (2026-06-27) composition test: spawn outbound + recall inbound
//! round-trip on fixture.
//!
//! Per substrate round-trip loop endpoint: spawn (outbound, `--hello-world`
//! flag) and recall (inbound, 4-payload envelope) must compose without
//! conflict. An agent invoking both in sequence — or via separate MCP
//! `tools/call` requests — must get coherent, version-coordinated
//! envelopes.
//!
//! Both envelopes must declare a `spec_version` field so the agent can
//! verify version coordination between the two surfaces it composes.
//! This is the RED contract for P5 (neither envelope carries it yet
//! at the start of P5; both add it in GREEN).
//!
//! Pattern mirrors `bootstrap/tests/spawn.rs` and `recall.rs`:
//! in-process dispatch via `mirror::kintsugi_main_in`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_mirror(subcommand: &str, args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), subcommand.to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

const TEST_PEER: &str = "bootstrap/tests/fixtures/spawn-test-peer";

fn parse_envelope(stdout: &[u8], context: &str) -> serde_json::Value {
    let s = String::from_utf8_lossy(stdout);
    serde_json::from_str(s.trim()).unwrap_or_else(|e| {
        panic!(
            "{} envelope must be valid JSON; got:\n{}\nparse error: {}",
            context, s, e
        )
    })
}

#[test]
fn round_trip_spawn_then_recall_both_succeed() {
    let spawn_out = run_mirror("spawn", &[TEST_PEER, "--hello-world"]);
    assert_eq!(
        spawn_out.status.code(),
        Some(0),
        "spawn --hello-world must succeed; stderr: {}",
        String::from_utf8_lossy(&spawn_out.stderr)
    );
    let spawn_env = parse_envelope(&spawn_out.stdout, "spawn --hello-world");
    assert_eq!(spawn_env["spawn"].as_str().unwrap_or(""), "hello_world");

    let recall_out = run_mirror("recall", &["."]);
    assert_eq!(
        recall_out.status.code(),
        Some(0),
        "recall must succeed; stderr: {}",
        String::from_utf8_lossy(&recall_out.stderr)
    );
    let recall_env = parse_envelope(&recall_out.stdout, "recall");
    assert!(recall_env["cascade"].is_array());
    assert!(recall_env["pack_trail"].is_array());
    assert!(recall_env["pull_frontier"].is_array());
    assert!(recall_env["dogfood"].is_object());
}

#[test]
fn round_trip_envelope_shapes_are_distinct_and_typed() {
    let spawn_out = run_mirror("spawn", &[TEST_PEER, "--hello-world"]);
    let spawn_env = parse_envelope(&spawn_out.stdout, "spawn");

    let recall_out = run_mirror("recall", &["."]);
    let recall_env = parse_envelope(&recall_out.stdout, "recall");

    // Spawn envelope does NOT carry recall payload keys.
    assert!(
        spawn_env.get("cascade").is_none(),
        "spawn envelope must not carry 'cascade'; got: {}",
        spawn_env
    );
    assert!(
        spawn_env.get("pack_trail").is_none(),
        "spawn envelope must not carry 'pack_trail'"
    );

    // Recall envelope does NOT carry spawn-specific keys.
    assert!(
        recall_env.get("spawn").is_none(),
        "recall envelope must not carry 'spawn'"
    );
    assert!(
        recall_env.get("composition_pieces").is_none(),
        "recall envelope must not carry 'composition_pieces'"
    );
}

#[test]
fn round_trip_invocation_order_does_not_deadlock() {
    // Recall first, then spawn. Verifies the cwd mutex doesn't trap
    // state and that back-to-back invocations against the same repo
    // succeed in either order (sanity guard against the architecture
    // issue Reed mis-diagnosed earlier today).
    let recall_out = run_mirror("recall", &["."]);
    assert_eq!(recall_out.status.code(), Some(0));

    let spawn_out = run_mirror("spawn", &[TEST_PEER, "--hello-world"]);
    assert_eq!(spawn_out.status.code(), Some(0));

    // Both produce valid JSON envelopes.
    let _ = parse_envelope(&recall_out.stdout, "recall (after spawn)");
    let _ = parse_envelope(&spawn_out.stdout, "spawn (after recall)");
}

/// P5 RED contract: both envelopes must declare a `spec_version` field
/// so an agent composing the round-trip can verify version coordination.
/// Fails RED until cmd_spawn (hello-world branch) and cmd_recall both
/// emit `spec_version`.
#[test]
fn round_trip_envelopes_declare_spec_version() {
    let spawn_out = run_mirror("spawn", &[TEST_PEER, "--hello-world"]);
    let spawn_env = parse_envelope(&spawn_out.stdout, "spawn");
    let spawn_version = spawn_env["spec_version"].as_str().unwrap_or_else(|| {
        panic!(
            "spawn --hello-world envelope must declare 'spec_version'; got: {}",
            spawn_env
        )
    });

    let recall_out = run_mirror("recall", &["."]);
    let recall_env = parse_envelope(&recall_out.stdout, "recall");
    let recall_version = recall_env["spec_version"].as_str().unwrap_or_else(|| {
        panic!(
            "recall envelope must declare 'spec_version'; got: {}",
            recall_env
        )
    });

    assert!(
        !spawn_version.is_empty(),
        "spec_version must be non-empty (got: {:?})",
        spawn_version
    );
    assert_eq!(
        spawn_version, recall_version,
        "spawn ({}) and recall ({}) must declare the same spec_version for composition",
        spawn_version, recall_version
    );
}
