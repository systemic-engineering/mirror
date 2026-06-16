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

// ── `--out @data/json` and `--out json` produce IDENTICAL envelopes ──────
//
// The substrate-correct ref and the legacy bare string should map to the
// same projection. This is the substrate-pull discipline: the new
// vocabulary doesn't introduce a new shape; it just names the old shape
// correctly. Equality on the JSON envelope guards against drift.

#[test]
fn substrate_ref_and_legacy_bare_string_produce_identical_envelope() {
    let ref_out = run_ci(&["--ci", "--out", "@data/json", settled_fixture()]);
    let bare_out = run_ci(&["--ci", "--out", "json", settled_fixture()]);
    assert!(
        ref_out.status.success() && bare_out.status.success(),
        "both runs should succeed; ref status={ref_status} bare status={bare_status}",
        ref_status = ref_out.status,
        bare_status = bare_out.status
    );
    let ref_json: serde_json::Value =
        serde_json::from_slice(&ref_out.stdout).expect("@data/json should parse");
    let bare_json: serde_json::Value =
        serde_json::from_slice(&bare_out.stdout).expect("json should parse");
    assert_eq!(
        ref_json, bare_json,
        "--out @data/json and --out json should produce byte-identical JSON envelopes\nref: {ref_json}\nbare: {bare_json}"
    );
}
