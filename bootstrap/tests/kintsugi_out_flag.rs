//! `mirror kintsugi --out json` substrate-pull RED — the substrate's
//! `out` keyword (recognition: `out` is the export keyword every shard
//! uses for namespace + symbol exports) is the substrate-correct name
//! for the @io-boundary projection flag. The current `--format` flag
//! is the residue of the pre-recognition vocabulary; this property
//! tick lifts `--out` to the CLI surface so the StageFreight Narrative
//! wire (forward-promised) reads `mirror kintsugi --out json` instead
//! of `mirror kintsugi --emit json` or `mirror kintsugi --format json`.
//!
//! Substrate-pull discipline: `--format` keeps working for the v0.1
//! cycle (composite action is shipped). This tick ADDS the
//! substrate-correct `--out` alongside; a future tick removes
//! `--format` after the composite action updates.
//!
//! Per Alex's "hold that" 2026-06-16: `--emit json` is `--out json`
//! (the `out` keyword) — the StageFreight integration tick (item 4 on
//! the cascade stack) needs the vocabulary live before the wire format
//! lands. This tick is that vocabulary landing.

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

fn settled_fixture() -> &'static str {
    // A `.mirror` file that settles to `success`. The kintsugi_ci.rs
    // tests use boot/std/nl.mirror; reuse it for parity.
    "boot/std/nl.mirror"
}

// ── `--out json <file>` produces JSON ───────────────────────────────

#[test]
fn out_json_space_form_emits_json() {
    let out = run_ci(&["--ci", "--out", "json", settled_fixture()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "mirror kintsugi --ci --out json {file} should exit 0; got status={status} stderr={stderr}",
        file = settled_fixture(),
        status = out.status,
        stderr = stderr
    );
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{') && trimmed.ends_with('}'),
        "expected JSON envelope (starts with `{{`, ends with `}}`); got stdout:\n{}",
        stdout
    );
    let v: serde_json::Value = serde_json::from_str(trimmed).unwrap_or_else(|e| {
        panic!("--out json should produce valid JSON; parse error: {e}; stdout:\n{stdout}")
    });
    assert!(
        v.get("verdict").is_some(),
        "JSON envelope should carry `verdict` field; got: {stdout}"
    );
}

// ── `--out=json` (equals form) also produces JSON ─────────────────────

#[test]
fn out_json_equals_form_emits_json() {
    let out = run_ci(&["--ci", "--out=json", settled_fixture()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "mirror kintsugi --ci --out=json {file} should exit 0; got status={status} stderr={stderr}",
        file = settled_fixture(),
        status = out.status,
        stderr = stderr
    );
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{'),
        "expected JSON envelope (starts with `{{`); got stdout:\n{}",
        stdout
    );
    let _: serde_json::Value = serde_json::from_str(trimmed).unwrap_or_else(|e| {
        panic!("--out=json should produce valid JSON; parse error: {e}; stdout:\n{stdout}")
    });
}

// ── `--out mirror` produces mirror-text (substrate-native default) ──────

#[test]
fn out_mirror_emits_mirror_text() {
    let out = run_ci(&["--ci", "--out", "mirror", settled_fixture()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "mirror kintsugi --ci --out mirror {file} should exit 0; got status={status} stderr={stderr}",
        file = settled_fixture(),
        status = out.status,
        stderr = stderr
    );
    // mirror-text is `<key> <value>` records, NOT JSON
    let trimmed = stdout.trim();
    assert!(
        !trimmed.starts_with('{'),
        "--out mirror should NOT produce JSON; got stdout:\n{}",
        stdout
    );
    assert!(
        stdout.contains("verdict"),
        "mirror-text wire should carry `verdict` key; got stdout:\n{}",
        stdout
    );
}

// ── `--format` (legacy) still works alongside `--out` ───────────────────
//
// The v0.1 composite action still calls `--format=json`. The cycle
// strategy: ADD `--out` as the substrate-correct alias; do NOT remove
// `--format` (a separate tick after the composite action moves).

#[test]
fn format_legacy_flag_still_works() {
    let out = run_ci(&["--ci", "--format=json", settled_fixture()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "--format=json (the v0.1 wire) must keep working; got status={status} stderr={stderr}",
        status = out.status,
        stderr = stderr
    );
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{'),
        "--format=json should still produce JSON; got stdout:\n{}",
        stdout
    );
}
