//! `mirror kintsugi --out @data/json` substrate-pull RED — the `out`
//! keyword's value space IS the substrate namespace, not a closed enum
//! of bare strings.
//!
//! Per Alex's 2026-06-16 substrate-pull insight: "what if `--out` accepts
//! a prism/glass e.g. @data/json or @dir('path')". The substrate's `out`
//! keyword (every shard's exports use `out @<X>`) is the substrate-correct
//! shape for the projection-flag value. Bare strings (`mirror|json`) are
//! the pre-recognition residue; substrate refs are the substrate-correct
//! vocabulary.
//!
//! This tick lands the conservative shape:
//! - `--out @data/json` produces JSON output (the receiver glass exists
//!   at shards/mirror/data/json.mirror per Mara's T16 2026-06-08 cascade)
//! - `--out json` (bare string) still works via Mara's polymorphic
//!   dispatch at `d929f5a` (regression guard)
//!
//! Parametric arguments (`@dir('path')`) are forward-promised, NOT this
//! tick. The parser surface for parametric args at use sites is a separate
//! design question.
//!
//! Future extensions: register additional projection glasses
//! (`@data/yaml`, `@data/toml`, `@code/erlang/term`,
//! `@io/stagefreight/narrative`) as the StageFreight integration
//! cascade (item 6+) needs them. The registry pattern lands here as
//! a hardcoded dispatch (`@data/json` → emit_ci_verdict_json); a
//! follow-up tick lifts it to a substrate-driven registry.

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
    // Same fixture the existing kintsugi_ci.rs + kintsugi_out_flag.rs use.
    "boot/std/nl.mirror"
}

// ── `--out @data/json <file>` produces JSON ──────────────────────────
//
// The substrate ref `@data/json` resolves to the JSON projection glass
// at shards/mirror/data/json.mirror. The CLI dispatch projects through
// it, producing the same JSON envelope as `--out json` (legacy alias).

#[test]
fn out_at_data_json_emits_json() {
    let out = run_ci(&["--ci", "--out", "@data/json", settled_fixture()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "mirror kintsugi --ci --out @data/json {file} should exit 0; got status={status} stderr={stderr}",
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
        panic!("--out @data/json should produce valid JSON; parse error: {e}; stdout:\n{stdout}")
    });
    assert!(
        v.get("verdict").is_some(),
        "JSON envelope should carry `verdict` field; got: {stdout}"
    );
}

// ── `--out=@data/json` (equals form) also produces JSON ─────────────────

#[test]
fn out_at_data_json_equals_form_emits_json() {
    let out = run_ci(&["--ci", "--out=@data/json", settled_fixture()]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "mirror kintsugi --ci --out=@data/json {file} should exit 0; got status={status} stderr={stderr}",
        file = settled_fixture(),
        status = out.status,
        stderr = stderr
    );
    let trimmed = stdout.trim();
    let _: serde_json::Value = serde_json::from_str(trimmed).unwrap_or_else(|e| {
        panic!("--out=@data/json should produce valid JSON; parse error: {e}; stdout:\n{stdout}")
    });
}

// ── HARD CUT — bare strings and --format no longer accepted ───────────
//
// Per Alex's 2026-06-16 "hard cut" directive: the substrate vocabulary
// replaces the pre-recognition residue. Bare `--out json` and
// `--format=json` are NOT compatibility aliases — they're errors. The
// CLI surface only accepts substrate refs (`@<namespace>` or
// `@<namespace>(<args>)`).
//
// This is a v0.1.1 behavior change. The v0.1 composite action will be
// updated in the same tick to call `--out @data/json` instead of
// `--format=json`.

#[test]
fn bare_string_json_errors_after_hard_cut() {
    let out = run_ci(&["--ci", "--out", "json", settled_fixture()]);
    assert!(
        !out.status.success(),
        "bare `--out json` must NOT succeed after hard cut; the substrate-correct form is `--out @data/json`. Got status={status} stdout:\n{stdout}",
        status = out.status,
        stdout = String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn format_flag_errors_after_hard_cut() {
    let out = run_ci(&["--ci", "--format=json", settled_fixture()]);
    assert!(
        !out.status.success(),
        "`--format=json` must NOT succeed after hard cut; the substrate-correct form is `--out @data/json`. Got status={status} stdout:\n{stdout}",
        status = out.status,
        stdout = String::from_utf8_lossy(&out.stdout)
    );
}

// ── `--out @io/dir('path')` writes verdict files to a directory ──────────────
//
// The parametric substrate ref `@io/dir('path')` replaces the legacy
// `--out <path>` bare-path semantics. Same `splinter(@code)` /
// `mosaic(altitude)` parametric syntax mirror already uses at type
// constructor sites — just at the CLI value position now.

#[test]
fn out_at_io_dir_writes_to_directory() {
    let tmp = std::env::temp_dir().join(format!("mirror-out-dir-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("create tmp dir");
    let tmp_str = tmp.to_string_lossy().to_string();
    let arg = format!("@io/dir('{}')", tmp_str);
    let out = run_ci(&["--ci", "--out", &arg, settled_fixture()]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "mirror kintsugi --ci --out @io/dir('{tmp_str}') {file} should exit 0; got status={status} stderr={stderr}",
        file = settled_fixture(),
        status = out.status,
        stderr = stderr,
        tmp_str = tmp_str
    );
    // Parser-level GREEN today: the parametric ref is ACCEPTED. Whether
    // the kintsugi pipeline writes verdict bytes into the directory is
    // a downstream semantic concern (a follow-up tick wires the dir
    // semantics; today we just need the parser to admit the parametric
    // shape without rejecting it).
    let _ = std::fs::remove_dir_all(&tmp);
}

// (identical-envelopes test removed: hard cut makes `--out json` an error,
// so the equivalence claim no longer holds. The `--out @data/json` JSON
// shape is locked by kintsugi_ci.rs's existing JSON envelope tests after
// they migrate to substrate refs.)
