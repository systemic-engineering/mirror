//! D4 — `mirror kintsugi <spec>` parses a `.spec` file, walks its
//! `target` blocks, and dispatches `cargo` for `emit cargo` targets.
//!
//! Per D3 (`shards/io/cargo.mirror`), cargo's exit codes lift into the
//! substrate's `transparency<p>` carrier via `cargo_exit_to_transparency`:
//!
//!   0   → success                  (clean build)
//!   101 → failure(opacity_map)     (compile error)
//!   1   → failure(opacity_map)     (generic failure)
//!   *   → partial(opacity_map)     (any other non-zero)
//!
//! This Rust glue coarsens that contract: exit 0 → `verdict: "success"`;
//! any non-zero → `verdict: "failure"` with the first 200 chars of
//! stderr in an opacity field. The rich opacity-map parsing
//! (file:line:col extraction from cargo stderr) is deferred to the
//! @mirror/mosaic substrate.
//!
//! Per T11.2.5 the default emission is mirror-text; `--format=json`
//! routes through the @io boundary.
//!
//! Per [[feedback-no-new-rust]]: this Rust function is dispatch glue
//! only. The substrate (`shards/mirror/spec.mirror`,
//! `shards/io/cargo.mirror`) carries the verdict types and the lift
//! contract. The Rust function reads the spec, finds `target` blocks,
//! invokes cargo, and aggregates into the existing
//! `CiVerdict`/`CorpusVerdict` envelope.

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

fn run_ki(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let mut cmd = Command::new(exe);
    cmd.current_dir(repo_root());
    cmd.arg("kintsugi");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("binary did not run")
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

const PASS_SPEC: &str = "bootstrap/tests/fixtures/spec-cargo-passes/mirror.spec";

// ── Spec routing ───────────────────────────────────────────────────────────────

#[test]
fn spec_extension_routes_to_spec_walker() {
    // The `.spec` extension is the trigger. Existing `<file>.mirror` /
    // `<dir>` paths still go through their original walkers
    // (backwards-compat). A `.spec` argument dispatches to the new
    // cargo-walking path.
    let out = run_ki(&[PASS_SPEC]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spec walker exit code must be 0 when the envelope emits cleanly; \
         stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Spec walker always emits a verdict envelope.
    assert!(
        stdout.trim_start().starts_with("verdict"),
        "spec walker must emit a verdict record; got: {stdout}"
    );
}

#[test]
fn spec_walker_required_fields_present() {
    let out = run_ki(&[PASS_SPEC]);
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

// ── Pass spec: cargo check succeeds → verdict success ─────────────────────────

#[test]
fn pass_spec_aggregate_verdict_is_success() {
    // The fixture's mirror.spec declares one `target binary { altitude
    // @code/rust, emit cargo }` whose Cargo.toml `cargo check` succeeds.
    // Per shards/io/cargo.mirror line 116 (exit 0 → success), the
    // aggregate verdict MUST be "success".
    let out = run_ki(&[PASS_SPEC]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let records = split_records(&stdout);
    let agg = &records[0];
    assert_eq!(
        agg.get("verdict").map(String::as_str),
        Some("success"),
        "passing cargo-check fixture must aggregate to success; got: {stdout}"
    );
}

#[test]
fn pass_spec_target_field_echoes_spec_path() {
    let out = run_ki(&[PASS_SPEC]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let agg = &split_records(&stdout)[0];
    let expected = format!("\"{PASS_SPEC}\"");
    assert_eq!(agg.get("target"), Some(&expected));
}

// ── JSON path (the @io boundary, behind --format=json) ────────────────────────

#[test]
fn spec_format_json_emits_valid_json() {
    let out = run_ki(&["--format=json", PASS_SPEC]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--format=json spec walker exit code must be 0"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("not valid JSON: {e}\n---stdout---\n{stdout}");
    });
    assert!(v.is_object());
    assert_eq!(v["verdict"].as_str(), Some("success"));
}

// ── Backwards-compat — existing modes preserved ───────────────────────────────

#[test]
fn existing_dot_mirror_corpus_walker_unchanged() {
    // The `.spec` routing must NOT affect existing `<directory>` and
    // `<file>.mirror` invocations. Re-run the kintsugi-pass fixture
    // through the corpus walker as a sanity check.
    let out = run_ki(&["--ci", "bootstrap/tests/fixtures/kintsugi-pass"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "existing --ci corpus walker must still succeed; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let agg = &split_records(&stdout)[0];
    assert_eq!(agg.get("verdict").map(String::as_str), Some("success"));
}
