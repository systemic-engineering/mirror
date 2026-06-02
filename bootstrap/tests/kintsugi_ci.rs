//! T11.2 — RED: `mirror kintsugi --ci` doesn't exist yet.
//!
//! First impl tick of the v0.1 CI chain per
//! `docs/specs/kintsugi-ci-v0.1.md`. Adds `--ci` flag + JSON serialiser
//! to the kintsugi subcommand so the GitHub Action's composite shell
//! step can parse the verdict and set `outputs.verdict` /
//! `outputs.objective` deterministically.
//!
//! Verdict shape per the spec:
//! ```json
//! {
//!   "verdict":    "success" | "partial" | "failure",
//!   "target":     "<path>",
//!   "objective":  <non-negative scalar>,
//!   "iterations": <non-negative integer>,
//!   "dark_count": <non-negative integer>
//! }
//! ```
//!
//! `verdict` discrimination:
//! - `success` — kintsugi converged (eⁿ⁺¹ == eⁿ) at objective == 0,
//!   dark_count == 0.
//! - `partial` — kintsugi converged at non-zero (harmonic remainder
//!   present, OR dark_count > 0).
//! - `failure` — the loop did not complete (file unreadable, parse
//!   error, grammar load failure). JSON is still emitted; the verdict
//!   field carries the discrimination.
//!
//! Exit code under `--ci` is **0 iff JSON was successfully emitted**.
//! The workflow YAML decides what verdict counts as pass; the binary
//! does not.

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

// Use a known-stable existing .mirror file. nl.mirror is the smallest in
// boot/std/ (161 bytes), so the kintsugi loop reaches the fixed point
// quickly and the test stays fast.
const FIXTURE: &str = "boot/std/nl.mirror";

// ── Shape ───────────────────────────────────────────────────────────────────────────

#[test]
fn ci_emits_valid_json() {
    let out = run_ci(&["--ci", FIXTURE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "--ci exit code must be 0 when JSON emits cleanly"
    );
    let v = parse_ci_json(&out);
    assert!(v.is_object(), "--ci output must be a JSON object");
}

#[test]
fn ci_has_required_fields() {
    let out = run_ci(&["--ci", FIXTURE]);
    let v = parse_ci_json(&out);
    for field in ["verdict", "target", "objective", "iterations", "dark_count"] {
        assert!(
            v.get(field).is_some(),
            "--ci output missing field `{field}`; got: {v}"
        );
    }
}

#[test]
fn ci_verdict_is_one_of_three_strings() {
    let out = run_ci(&["--ci", FIXTURE]);
    let v = parse_ci_json(&out);
    let verdict = v["verdict"].as_str().expect("verdict must be a string");
    assert!(
        matches!(verdict, "success" | "partial" | "failure"),
        "verdict must be success|partial|failure; got: {verdict:?}"
    );
}

#[test]
fn ci_target_echoes_input_path() {
    let out = run_ci(&["--ci", FIXTURE]);
    let v = parse_ci_json(&out);
    assert_eq!(v["target"].as_str(), Some(FIXTURE));
}

#[test]
fn ci_objective_is_non_negative_number() {
    let out = run_ci(&["--ci", FIXTURE]);
    let v = parse_ci_json(&out);
    let obj = v["objective"]
        .as_f64()
        .expect("objective must be a JSON number");
    assert!(obj >= 0.0, "objective must be non-negative; got: {obj}");
}

#[test]
fn ci_iterations_is_at_least_one() {
    let out = run_ci(&["--ci", FIXTURE]);
    let v = parse_ci_json(&out);
    let iter = v["iterations"]
        .as_u64()
        .expect("iterations must be a non-negative integer");
    assert!(
        iter >= 1,
        "iterations must be ≥ 1 (the loop ran at least once); got: {iter}"
    );
}

#[test]
fn ci_dark_count_is_non_negative_integer() {
    let out = run_ci(&["--ci", FIXTURE]);
    let v = parse_ci_json(&out);
    let dark = v["dark_count"]
        .as_u64()
        .expect("dark_count must be a non-negative integer");
    let _ = dark; // just shape; semantics elsewhere
}

// ── Determinism ───────────────────────────────────────────────────────────────

#[test]
fn ci_objective_is_deterministic_across_runs() {
    let a = run_ci(&["--ci", FIXTURE]);
    let b = run_ci(&["--ci", FIXTURE]);
    let va = parse_ci_json(&a);
    let vb = parse_ci_json(&b);
    // Per the v0.1 spec's `outputs.objective` byte-identical guarantee
    // across three runs on ubuntu-latest. Two runs locally are enough
    // for the unit-test contract.
    assert_eq!(
        va["objective"], vb["objective"],
        "objective must be deterministic; got {} then {}",
        va["objective"], vb["objective"]
    );
    assert_eq!(
        va["verdict"], vb["verdict"],
        "verdict must be deterministic"
    );
    assert_eq!(
        va["dark_count"], vb["dark_count"],
        "dark_count must be deterministic"
    );
}

// ── Composition with existing flags ──────────────────────────────────────────────

#[test]
fn ci_composes_with_shatter() {
    let out = run_ci(&["--ci", "--shatter", "1", FIXTURE]);
    let v = parse_ci_json(&out);
    assert!(v.get("verdict").is_some());
    // --shatter > 0 should record the iteration count from the actual loop.
    let iter = v["iterations"].as_u64().unwrap();
    assert!(iter >= 1, "--ci --shatter 1 should iterate at least once");
}

// ── Without --ci, behavior unchanged ─────────────────────────────────────────────

#[test]
fn without_ci_no_json_envelope_on_stdout() {
    let out = run_ci(&[FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Default rendering is the canonical text form. It should NOT be a
    // JSON object with the verdict shape.
    let looks_like_ci_json = stdout.trim_start().starts_with('{')
        && stdout.contains("\"verdict\"")
        && stdout.contains("\"objective\"");
    assert!(
        !looks_like_ci_json,
        "without --ci, kintsugi must NOT emit the CI JSON envelope; got: {stdout}"
    );
}
