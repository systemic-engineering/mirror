//! T11.3 — RED: `mirror kintsugi --ci <directory>` corpus walker.
//!
//! Second impl tick of the v0.1 CI chain per
//! `docs/specs/kintsugi-ci-v0.1.md`. Extends `--ci` to walk a directory
//! recursively, run kintsugi on each `.mirror` file, and emit an
//! aggregate JSON verdict with a `per_file` array.
//!
//! ## Aggregation rules
//!
//! - `verdict`: `success` iff every per-file verdict is `success`;
//!   `partial` if any per-file is partial OR has dark > 0; `failure`
//!   if any per-file failed.
//! - `objective`: **sum** of per-file objectives (additive, matches
//!   the [[kintsugi-variety]] objective `∑ variety_loss(op) ·
//!   crossing_cost(op)`).
//! - `dark_count`: **sum** of per-file dark counts (total residual
//!   across the corpus).
//! - `iterations`: **max** of per-file iterations (the longest-
//!   running file).
//! - `files_processed`: total `.mirror` files walked.
//! - `per_file`: array of per-file JSON verdicts, each with its own
//!   `path`, `verdict`, `objective`, `iterations`, `dark_count`.
//!
//! ## Fixture corpora
//!
//! Per the v0.1 spec's smallest-workflow-that-proves-v0.1:
//!
//! - `bootstrap/tests/fixtures/kintsugi-pass/` — every file should
//!   tokenize clean; expect `verdict=success`, `objective=0`,
//!   `dark_count=0`.
//! - `bootstrap/tests/fixtures/kintsugi-partial/` — contains a file
//!   with an unknown construct in the body, producing dark; expect
//!   `verdict=partial`, `dark_count > 0`.

use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn run_ci(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let mut cmd = Command::new(exe);
    cmd.current_dir(repo_root());
    cmd.arg("kintsugi");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("binary did not run")
}

fn parse_ci_json(out: &std::process::Output) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "--ci output is not valid JSON: {e}\n---stdout---\n{stdout}\n---stderr---\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

const PASS_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-pass";
const PARTIAL_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-partial";

// ── Directory mode ─ shape ─────────────────────────────────────────────────────

#[test]
fn ci_dir_emits_valid_json() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--ci on a directory must exit 0 when JSON emits cleanly"
    );
    let v = parse_ci_json(&out);
    assert!(v.is_object(), "--ci output must be a JSON object");
}

#[test]
fn ci_dir_has_aggregate_required_fields() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    for field in [
        "verdict",
        "target",
        "objective",
        "iterations",
        "dark_count",
        "files_processed",
        "per_file",
    ] {
        assert!(
            v.get(field).is_some(),
            "--ci dir output missing field `{field}`; got: {v}"
        );
    }
}

#[test]
fn ci_dir_target_echoes_directory_path() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    assert_eq!(v["target"].as_str(), Some(PASS_FIXTURE));
}

#[test]
fn ci_dir_per_file_is_array() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    assert!(
        v["per_file"].is_array(),
        "per_file must be a JSON array; got: {}",
        v["per_file"]
    );
}

#[test]
fn ci_dir_files_processed_matches_per_file_length() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    let files_processed = v["files_processed"]
        .as_u64()
        .expect("files_processed is integer");
    let per_file_len = v["per_file"].as_array().expect("per_file array").len() as u64;
    assert_eq!(
        files_processed, per_file_len,
        "files_processed ({files_processed}) must equal per_file length ({per_file_len})"
    );
}

#[test]
fn ci_dir_per_file_entries_have_required_shape() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    let arr = v["per_file"].as_array().expect("per_file array");
    assert!(
        !arr.is_empty(),
        "per_file must not be empty for non-empty corpus"
    );
    for entry in arr {
        for field in ["path", "verdict", "objective", "iterations", "dark_count"] {
            assert!(
                entry.get(field).is_some(),
                "per_file entry missing `{field}`; got: {entry}"
            );
        }
        let verdict = entry["verdict"].as_str().expect("verdict is string");
        assert!(
            matches!(verdict, "success" | "partial" | "failure"),
            "per_file verdict must be success|partial|failure; got: {verdict:?}"
        );
    }
}

// ── Pass corpus ─ aggregate verdict ───────────────────────────────────────────────────

#[test]
fn ci_pass_corpus_aggregate_is_success() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    let verdict = v["verdict"].as_str().expect("verdict is string");
    assert_eq!(
        verdict, "success",
        "kintsugi-pass fixture must aggregate to success; got: {v}"
    );
    assert_eq!(
        v["dark_count"].as_u64(),
        Some(0),
        "pass fixture must have zero aggregate dark_count"
    );
    assert_eq!(
        v["objective"].as_f64(),
        Some(0.0),
        "pass fixture must have zero aggregate objective"
    );
}

#[test]
fn ci_pass_corpus_walks_two_files() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    assert_eq!(
        v["files_processed"].as_u64(),
        Some(2),
        "kintsugi-pass fixture has exactly 2 .mirror files (a.mirror, b.mirror)"
    );
}

// ── Partial corpus ─ aggregate verdict ───────────────────────────────────────────────

#[test]
fn ci_partial_corpus_aggregate_is_partial() {
    let out = run_ci(&["--ci", PARTIAL_FIXTURE]);
    let v = parse_ci_json(&out);
    let verdict = v["verdict"].as_str().expect("verdict is string");
    assert_eq!(
        verdict, "partial",
        "kintsugi-partial fixture must aggregate to partial; got: {v}"
    );
    let dark = v["dark_count"].as_u64().expect("dark_count is integer");
    assert!(
        dark > 0,
        "partial fixture must have non-zero aggregate dark_count; got: {dark}"
    );
}

// ── Determinism ───────────────────────────────────────────────────────────────────────

#[test]
fn ci_dir_objective_is_deterministic_across_runs() {
    let a = run_ci(&["--ci", PASS_FIXTURE]);
    let b = run_ci(&["--ci", PASS_FIXTURE]);
    let va = parse_ci_json(&a);
    let vb = parse_ci_json(&b);
    assert_eq!(va["objective"], vb["objective"]);
    assert_eq!(va["verdict"], vb["verdict"]);
    assert_eq!(va["dark_count"], vb["dark_count"]);
    assert_eq!(va["files_processed"], vb["files_processed"]);
}

#[test]
fn ci_dir_per_file_order_is_deterministic_across_runs() {
    let a = run_ci(&["--ci", PASS_FIXTURE]);
    let b = run_ci(&["--ci", PASS_FIXTURE]);
    let va = parse_ci_json(&a);
    let vb = parse_ci_json(&b);
    let pa = va["per_file"].as_array().expect("per_file array");
    let pb = vb["per_file"].as_array().expect("per_file array");
    assert_eq!(pa.len(), pb.len());
    for (ea, eb) in pa.iter().zip(pb.iter()) {
        assert_eq!(
            ea["path"], eb["path"],
            "per_file order must be deterministic"
        );
        assert_eq!(ea["verdict"], eb["verdict"]);
        assert_eq!(ea["objective"], eb["objective"]);
        assert_eq!(ea["dark_count"], eb["dark_count"]);
    }
}

// ── Single-file mode still works (T11.2 regression) ──────────────────────────────────────

#[test]
fn ci_single_file_mode_unchanged() {
    // Asserts T11.2's single-file shape: no per_file, no files_processed.
    let out = run_ci(&["--ci", "boot/std/nl.mirror"]);
    let v = parse_ci_json(&out);
    assert_eq!(v["verdict"].as_str(), Some("success"));
    assert!(
        v.get("per_file").is_none(),
        "single-file mode must NOT include per_file (T11.2 shape unchanged)"
    );
    assert!(
        v.get("files_processed").is_none(),
        "single-file mode must NOT include files_processed (T11.2 shape unchanged)"
    );
}
