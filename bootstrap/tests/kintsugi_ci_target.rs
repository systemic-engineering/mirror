//! T11.2.5 — `mirror kintsugi --ci <directory>` corpus walker emits
//! stringified mirror AST by default. JSON only under `--format=json`.
//!
//! Aggregation rules (unchanged from T11.3):
//!
//! - `verdict`: `success` iff every per-file verdict is `success`;
//!   `partial` if any per-file is partial OR has dark > 0; `failure`
//!   if any per-file failed.
//! - `objective`: sum of per-file objectives.
//! - `dark_count`: sum of per-file dark counts.
//! - `iterations`: max of per-file iterations.
//! - `files_processed`: total `.mirror` files walked.
//!
//! ## Default (mirror-text) shape
//!
//! Aggregate record first (one blank line terminator), then one
//! blank-line-separated record per file, sorted by path. Each per-file
//! record's first field is `file <quoted-path>`, not `target`.
//!
//! ```text
//! verdict          partial
//! target           "bootstrap/tests/fixtures/kintsugi-partial"
//! objective        1.0
//! iterations       1
//! dark_count       1
//! files_processed  1
//!
//! file         "bootstrap/tests/fixtures/kintsugi-partial/dark.mirror"
//! verdict      partial
//! objective    1.0
//! iterations   1
//! dark_count   1
//! ```

use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::Output;

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

/// In-process dispatch through `mirror::kintsugi_main_in` (Taut #286 Win 2).
fn run_ci(args: &[&str]) -> Output {
    use std::os::unix::process::ExitStatusExt;
    let mut argv: Vec<String> = vec!["mirror".to_string(), "kintsugi".to_string()];
    for a in args {
        argv.push((*a).to_string());
    }
    let out = mirror::kintsugi_main_in(&argv, repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

fn parse_record(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.splitn(2, char::is_whitespace);
        let key = it.next().unwrap_or("").trim().to_string();
        let value = it.next().unwrap_or("").trim().to_string();
        if !key.is_empty() {
            map.insert(key, value);
        }
    }
    map
}

fn split_records(stdout: &str) -> Vec<HashMap<String, String>> {
    stdout
        .split("\n\n")
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(parse_record)
        .collect()
}

fn parse_ci_json(out: &std::process::Output) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "--ci --format=json output is not valid JSON: {e}\n---stdout---\n{stdout}\n---stderr---\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

const PASS_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-pass";
const PARTIAL_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-partial";

// ── Default (mirror-text) shape ───────────────────────────────────────────────

#[test]
fn ci_dir_emits_mirror_text_by_default() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--ci on a directory must exit 0 when the record emits cleanly; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim_start().starts_with('{'),
        "default --ci on dir must not emit JSON; got: {stdout}"
    );
    assert!(
        stdout.trim_start().starts_with("verdict"),
        "default --ci on dir must start with `verdict <...>`; got: {stdout}"
    );
}

#[test]
fn ci_dir_aggregate_has_required_fields() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(
        !records.is_empty(),
        "must emit at least an aggregate record"
    );
    let aggregate = &records[0];
    for field in [
        "verdict",
        "target",
        "objective",
        "iterations",
        "dark_count",
        "files_processed",
    ] {
        assert!(
            aggregate.contains_key(field),
            "aggregate record missing `{field}`; got: {stdout}"
        );
    }
}

#[test]
fn ci_dir_target_echoes_directory_path_quoted() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let aggregate = &split_records(&stdout)[0];
    let expected = format!("\"{PASS_FIXTURE}\"");
    assert_eq!(aggregate.get("target"), Some(&expected));
}

#[test]
fn ci_dir_per_file_records_follow_aggregate() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    // Aggregate + 2 per-file (a.mirror, b.mirror).
    assert!(
        records.len() >= 2,
        "expected aggregate + per-file records; got {} records",
        records.len()
    );
    // Each per-file record's first key is `file` (not `target`).
    for r in records.iter().skip(1) {
        assert!(
            r.contains_key("file"),
            "per-file record missing `file` key; got: {:?}",
            r
        );
        for field in ["verdict", "objective", "iterations", "dark_count"] {
            assert!(
                r.contains_key(field),
                "per-file record missing `{field}`; got: {:?}",
                r
            );
        }
    }
}

#[test]
fn ci_dir_files_processed_matches_per_file_record_count() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    let aggregate = &records[0];
    let files_processed: u64 = aggregate
        .get("files_processed")
        .expect("files_processed present")
        .parse()
        .expect("files_processed parses as u64");
    let per_file_count = records.len().saturating_sub(1) as u64;
    assert_eq!(
        files_processed, per_file_count,
        "files_processed ({files_processed}) must equal per-file record count ({per_file_count})"
    );
}

// ── Pass corpus ─ aggregate verdict ───────────────────────────────────────────

#[test]
fn ci_pass_corpus_aggregate_is_success() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let aggregate = &split_records(&stdout)[0];
    assert_eq!(
        aggregate.get("verdict").map(String::as_str),
        Some("success"),
        "kintsugi-pass fixture must aggregate to success; got: {stdout}"
    );
    assert_eq!(
        aggregate.get("dark_count").map(String::as_str),
        Some("0"),
        "pass fixture must have zero aggregate dark_count"
    );
    let obj: f64 = aggregate.get("objective").unwrap().parse().unwrap();
    assert_eq!(obj, 0.0, "pass fixture must have zero aggregate objective");
}

#[test]
fn ci_pass_corpus_walks_two_files() {
    let out = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let aggregate = &split_records(&stdout)[0];
    assert_eq!(
        aggregate.get("files_processed").map(String::as_str),
        Some("2"),
        "kintsugi-pass fixture has exactly 2 .mirror files (a.mirror, b.mirror)"
    );
}

// ── Partial corpus ─ aggregate verdict ────────────────────────────────────────

#[test]
fn ci_partial_corpus_aggregate_is_partial() {
    let out = run_ci(&["--ci", PARTIAL_FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let aggregate = &split_records(&stdout)[0];
    assert_eq!(
        aggregate.get("verdict").map(String::as_str),
        Some("partial"),
        "kintsugi-partial fixture must aggregate to partial; got: {stdout}"
    );
    let dark: u64 = aggregate.get("dark_count").unwrap().parse().unwrap();
    assert!(
        dark > 0,
        "partial fixture must have non-zero aggregate dark_count; got: {dark}"
    );
}

// ── Determinism ───────────────────────────────────────────────────────────────

#[test]
fn ci_dir_default_output_is_byte_identical_across_runs() {
    let a = run_ci(&["--ci", PASS_FIXTURE]);
    let b = run_ci(&["--ci", PASS_FIXTURE]);
    assert_eq!(
        a.stdout, b.stdout,
        "default --ci on dir must be byte-identical across runs"
    );
}

#[test]
fn ci_dir_per_file_order_is_deterministic() {
    let a = run_ci(&["--ci", PASS_FIXTURE]);
    let b = run_ci(&["--ci", PASS_FIXTURE]);
    let stdout_a = String::from_utf8_lossy(&a.stdout);
    let stdout_b = String::from_utf8_lossy(&b.stdout);
    let ra = split_records(&stdout_a);
    let rb = split_records(&stdout_b);
    assert_eq!(ra.len(), rb.len());
    for (ea, eb) in ra.iter().zip(rb.iter()) {
        assert_eq!(ea.get("file"), eb.get("file"));
        assert_eq!(ea.get("verdict"), eb.get("verdict"));
    }
}

// ── Single-file mode (T11.2 regression in mirror-text) ────────────────────────

#[test]
fn ci_single_file_mode_unchanged() {
    // T11.2's single-file shape under mirror-text: aggregate carries
    // no `files_processed` field, no per-file records.
    let out = run_ci(&["--ci", "boot/std/nl.mirror"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert_eq!(
        records.len(),
        1,
        "single-file mode emits exactly one record"
    );
    let r = &records[0];
    assert_eq!(r.get("verdict").map(String::as_str), Some("success"));
    assert!(
        !r.contains_key("files_processed"),
        "single-file record must NOT include files_processed"
    );
    assert!(
        !r.contains_key("file"),
        "single-file record uses `target`, not `file`"
    );
}

// ── JSON path (behind --format=json) — the @io boundary ───────────────────────

#[test]
fn ci_format_json_dir_emits_valid_json() {
    let out = run_ci(&["--ci", "--format=json", PASS_FIXTURE]);
    assert_eq!(out.status.code(), Some(0));
    let v = parse_ci_json(&out);
    assert!(v.is_object(), "--format=json on dir must be a JSON object");
}

#[test]
fn ci_format_json_dir_has_aggregate_required_fields() {
    let out = run_ci(&["--ci", "--format=json", PASS_FIXTURE]);
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
            "--format=json missing `{field}`; got: {v}"
        );
    }
}

#[test]
fn ci_format_json_dir_per_file_is_array() {
    let out = run_ci(&["--ci", "--format=json", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    assert!(v["per_file"].is_array(), "per_file must be a JSON array");
}

#[test]
fn ci_format_json_pass_corpus_aggregate_is_success() {
    let out = run_ci(&["--ci", "--format=json", PASS_FIXTURE]);
    let v = parse_ci_json(&out);
    assert_eq!(v["verdict"].as_str(), Some("success"));
    assert_eq!(v["dark_count"].as_u64(), Some(0));
    assert_eq!(v["objective"].as_f64(), Some(0.0));
    assert_eq!(v["files_processed"].as_u64(), Some(2));
}

#[test]
fn ci_format_json_partial_corpus_aggregate_is_partial() {
    let out = run_ci(&["--ci", "--format=json", PARTIAL_FIXTURE]);
    let v = parse_ci_json(&out);
    assert_eq!(v["verdict"].as_str(), Some("partial"));
    let dark = v["dark_count"].as_u64().unwrap();
    assert!(dark > 0);
}

#[test]
fn ci_format_json_single_file_mode_unchanged() {
    // T11.2's JSON shape preserved under --format=json: no per_file,
    // no files_processed.
    let out = run_ci(&["--ci", "--format=json", "boot/std/nl.mirror"]);
    let v = parse_ci_json(&out);
    assert_eq!(v["verdict"].as_str(), Some("success"));
    assert!(v.get("per_file").is_none());
    assert!(v.get("files_processed").is_none());
}
