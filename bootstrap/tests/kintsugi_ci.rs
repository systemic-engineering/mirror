//! T11.2.5 — `mirror kintsugi --ci` emits stringified mirror AST by default.
//!
//! The original T11.2 shipped JSON as the default. That was wrong:
//! `@io` crossings are decoherence events; the verdict belongs IN the
//! substrate as long as possible. The default wire format is the
//! mirror-text record shape — a blank-line-separated sequence of
//! `<key> <value>` lines, lossless and human-readable.
//!
//! JSON appears only at the `@io` boundary — the action's `run.sh`
//! invokes `--out=@data/json` when it needs to set `$GITHUB_OUTPUT`.
//!
//! ## Default (mirror-text) shape
//!
//! Single-file (T11.2.5 / T11.2):
//!
//! ```text
//! verdict      success
//! target       "boot/std/nl.mirror"
//! objective    0.0
//! iterations   1
//! dark_count   0
//! ```
//!
//! Corpus (T11.2.5 / T11.3): aggregate record first, then one
//! blank-line-separated record per file, sorted by path:
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
//!
//! ## JSON shape (behind `--out=@data/json`)
//!
//! Same fields. T11.2/T11.3 contract preserved: the JSON envelope keeps
//! `verdict | target | objective | iterations | dark_count`, plus
//! `files_processed | per_file` for the corpus mode.

use serde_json::Value;
use std::collections::HashMap;
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

/// Parse one blank-line-separated record into a key→value map.
///
/// Each non-blank line is `<key>[ws]+<value>`. The value is the
/// remainder of the line, trimmed. Keys repeat across records (each
/// record has its own `verdict` field); within a record, keys are
/// unique. Returns `None` if the input is empty or contains no
/// well-formed key/value lines.
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

/// Split the stdout into blank-line-separated records.
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
            "--ci --out=@data/json output is not valid JSON: {e}\n---stdout---\n{stdout}\n---stderr---\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

// Use a known-stable existing .mirror file. nl.mirror is the smallest in
// boot/std/ (161 bytes), so the kintsugi loop reaches the fixed point
// quickly and the test stays fast.
const FIXTURE: &str = "boot/std/nl.mirror";

// ── Default (mirror-text) shape ──────────────────────────────────────────────

#[test]
fn ci_emits_mirror_text_by_default() {
    let out = run_ci(&["--ci", FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--ci exit code must be 0 when the record emits cleanly; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Substrate-pull: default emission must NOT be JSON.
    assert!(
        !stdout.trim_start().starts_with('{'),
        "default --ci must not emit JSON; got: {stdout}"
    );
    // Each record begins with `verdict <discrimination>`.
    assert!(
        stdout.trim_start().starts_with("verdict"),
        "default --ci must start with `verdict <...>`; got: {stdout}"
    );
}

#[test]
fn ci_has_required_fields() {
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    assert!(!records.is_empty(), "must emit at least one record");
    let r = &records[0];
    for field in ["verdict", "target", "objective", "iterations", "dark_count"] {
        assert!(
            r.contains_key(field),
            "missing field `{field}`; got: {stdout}"
        );
    }
}

#[test]
fn ci_verdict_is_one_of_three_strings() {
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    let verdict = r.get("verdict").expect("verdict present");
    assert!(
        matches!(verdict.as_str(), "success" | "partial" | "failure"),
        "verdict must be success|partial|failure; got: {verdict:?}"
    );
}

#[test]
fn ci_target_echoes_input_path_quoted() {
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    // Paths are emitted quoted so containment of `/`, `.`, and other
    // mirror-significant bytes round-trips losslessly.
    let expected = format!("\"{FIXTURE}\"");
    assert_eq!(r.get("target"), Some(&expected));
}

#[test]
fn ci_objective_is_non_negative_number() {
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    let obj: f64 = r
        .get("objective")
        .expect("objective present")
        .parse()
        .expect("objective parses as f64");
    assert!(obj >= 0.0, "objective must be non-negative; got: {obj}");
}

#[test]
fn ci_iterations_is_at_least_one() {
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    let iter: u64 = r
        .get("iterations")
        .expect("iterations present")
        .parse()
        .expect("iterations parses as u64");
    assert!(iter >= 1, "iterations must be ≥ 1; got: {iter}");
}

#[test]
fn ci_dark_count_is_non_negative_integer() {
    let out = run_ci(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    let _dark: u64 = r
        .get("dark_count")
        .expect("dark_count present")
        .parse()
        .expect("dark_count parses as u64");
}

// ── Determinism ───────────────────────────────────────────────────────────────

#[test]
fn ci_default_output_is_byte_identical_across_runs() {
    let a = run_ci(&["--ci", FIXTURE]);
    let b = run_ci(&["--ci", FIXTURE]);
    assert_eq!(
        a.stdout, b.stdout,
        "default --ci output must be byte-identical across runs"
    );
}

// ── Composition with existing flags ──────────────────────────────────────────────

#[test]
fn ci_composes_with_shatter() {
    let out = run_ci(&["--ci", "--shatter", "1", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let r = &split_records(&stdout)[0];
    assert!(r.contains_key("verdict"));
    let iter: u64 = r.get("iterations").unwrap().parse().unwrap();
    assert!(iter >= 1, "--ci --shatter 1 should iterate at least once");
}

// ── Without --ci, behavior unchanged ─────────────────────────────────────────────

#[test]
fn without_ci_no_verdict_record_on_stdout() {
    let out = run_ci(&[FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Default rendering is the canonical text form of the input file.
    // It must NOT be a verdict record.
    let trimmed = stdout.trim_start();
    assert!(
        !trimmed.starts_with("verdict "),
        "without --ci, kintsugi must NOT emit a verdict record; got: {stdout}"
    );
}

// ── JSON path (behind --out=@data/json) — the @io boundary ────────────────────────

#[test]
fn ci_format_json_emits_valid_json() {
    let out = run_ci(&["--ci", "--out=@data/json", FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--ci --out=@data/json exit code must be 0 when JSON emits cleanly"
    );
    let v = parse_ci_json(&out);
    assert!(
        v.is_object(),
        "--out=@data/json output must be a JSON object"
    );
}

#[test]
fn ci_format_json_has_required_fields() {
    let out = run_ci(&["--ci", "--out=@data/json", FIXTURE]);
    let v = parse_ci_json(&out);
    for field in ["verdict", "target", "objective", "iterations", "dark_count"] {
        assert!(
            v.get(field).is_some(),
            "--out=@data/json output missing field `{field}`; got: {v}"
        );
    }
}

#[test]
fn ci_format_json_target_echoes_input_path() {
    let out = run_ci(&["--ci", "--out=@data/json", FIXTURE]);
    let v = parse_ci_json(&out);
    assert_eq!(v["target"].as_str(), Some(FIXTURE));
}

#[test]
fn ci_format_json_is_deterministic_across_runs() {
    let a = run_ci(&["--ci", "--out=@data/json", FIXTURE]);
    let b = run_ci(&["--ci", "--out=@data/json", FIXTURE]);
    let va = parse_ci_json(&a);
    let vb = parse_ci_json(&b);
    assert_eq!(va["objective"], vb["objective"]);
    assert_eq!(va["verdict"], vb["verdict"]);
    assert_eq!(va["dark_count"], vb["dark_count"]);
}
