//! P3 RED (2026-06-26): integration tests for `mirror recall <dir>`.
//!
//! Per Mara's @mirror/recall canonical spec (`docs/specs/mirror-recall.md`,
//! commit `b034a60`) + Seam P2 adversarial review (`88f8428`) with
//! Discharge C (`last_seen_commit: content_address`, NOT `in_flight:
//! bool` which is forbidden as stateless-return-at-runtime per b10f00c
//! §4).
//!
//! These tests fail against the cmd_recall stub in `lib.rs` because the
//! stub returns a placeholder envelope without the four payload keys.
//! The GREEN tick wires the real reads.
//!
//! Pattern mirrors `bootstrap/tests/spawn.rs` (Phase G v0): in-process
//! dispatch via `mirror::kintsugi_main_in`, wrapping the returned
//! `ExitOutput` in `std::process::Output` for ergonomic assertion.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_recall(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "recall".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

#[test]
fn recall_exits_zero_on_valid_dir() {
    let out = run_recall(&["."]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "recall must exit 0 on valid dir; got {:?}\nstderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// P3 RED: cmd_recall stub emits placeholder JSON without the four
/// payload keys. The GREEN tick wires real cascade/pack_trail/
/// pull_frontier/dogfood reads. This test asserts the contract from
/// Mara spec §3 — it fails RED until GREEN lands.
#[test]
fn recall_envelope_is_json_with_four_payload_keys() {
    let out = run_recall(&["."]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("recall stdout must be JSON; got: {}\nerr: {}", stdout, e));
    for key in ["cascade", "pack_trail", "pull_frontier", "dogfood"] {
        assert!(
            envelope.get(key).is_some(),
            "recall envelope must carry '{}' payload key per Mara spec §3; got: {}",
            key,
            stdout
        );
    }
}

/// P3 RED: Seam Discharge C (88f8428) — pack_trail records use
/// `last_seen_commit: content_address`, NOT `in_flight: bool` (forbidden
/// by b10f00c §4 as stateless-return at runtime).
#[test]
fn recall_pack_trail_uses_last_seen_commit_not_in_flight() {
    let out = run_recall(&["."]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("recall stdout must be JSON; got: {}\nerr: {}", stdout, e));
    let pack_trail = envelope["pack_trail"]
        .as_array()
        .expect("pack_trail must be an array");
    if let Some(first) = pack_trail.first() {
        assert!(
            first.get("last_seen_commit").is_some(),
            "Seam Discharge C: pack_tick must carry `last_seen_commit`; got: {}",
            first
        );
        assert!(
            first.get("in_flight").is_none(),
            "Seam Discharge C: pack_tick must NOT carry `in_flight` (b10f00c §4 stateless-return); got: {}",
            first
        );
    }
}

#[test]
fn recall_exits_non_zero_on_missing_dir() {
    let out = run_recall(&["/nonexistent/mirror-recall-test"]);
    // RED: stub does not validate dir. GREEN tick must add an existence
    // check that returns non-zero.
    assert_ne!(
        out.status.code(),
        Some(0),
        "recall must exit non-zero when spec dir is missing"
    );
}

#[test]
fn recall_exits_non_zero_on_missing_arg() {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "recall".to_string()];
    argv.extend(Vec::<String>::new());
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    assert_ne!(
        out.exit_code, 0,
        "recall without args must exit non-zero (usage message)"
    );
}
