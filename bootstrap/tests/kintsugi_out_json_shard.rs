//! Tick 7 RED — `mirror kintsugi --out @data/json` @shatter × @io fold.
//!
//! @shatter × @io = the linearization operator per Recognition #58 dual
//! (Mara `583b939` canonical spec at
//! `docs/specs/shatter-is-the-io-linearization-operator.md` §4.1). The
//! MCP wrapper's job IS this linearization — envelope-shaped output
//! collapses to JSON transport at the @io boundary.
//!
//! Per Taut LRM scout `1658b95` β-shallow path:
//!
//! > In `bin/mirror-mcp` mirror_kintsugi dispatch, pass `--out @data/json`
//! > so the binary itself emits JSON via kintsugi's own output-format
//! > switch. The MCP wrapper doesn't re-shatter; the substrate does.
//!
//! **Prerequisite verified**: `parse_substrate_ref_to_format` at
//! `bootstrap/src/lib.rs:1851-1857` maps `@data/json` → `CiFormat::Json`.
//! Combined with `--ci`, `cmd_kintsugi_ci_single` routes through
//! `emit_ci_verdict_json`. The wrapper composes both flags to hit the
//! JSON emission path today (Landing 1 discharge: existing wiring).
//!
//! **Scope**: this shard tests the CLI-altitude fold surface that
//! `bin/mirror-mcp` will invoke. The MCP wrapper roundtrip tests live
//! separately; this file locks the binary contract the wrapper depends on.
//!
//! Composes with:
//! - Recognition #58 dual — @shatter × @io = linearization operator
//! - Mara `583b939` — @shatter × @io canonical spec
//! - Taut `1658b95` — LRM scout β-shallow path (this file discharges
//!   the prerequisite verification named in that scout)
//! - Reed 2026-06-16 substrate-pull — `--out` accepts substrate refs
//! - Tick 0 Landing 3 (`05bac44`) — `parse_substrate_ref_to_format` lift

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

fn run_kintsugi(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let mut cmd = Command::new(exe);
    cmd.current_dir(repo_root());
    cmd.arg("kintsugi");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("binary did not run")
}

// Same fixture used by kintsugi_ci.rs / kintsugi_out_substrate_ref.rs —
// small settled shard reaching the fixed point in ≤1 tick.
const FIXTURE: &str = "boot/std/nl.mirror";

// ── T1: `--out @data/json` accepts the flag ─────────────────────────────
//
// The flag surface exists today via `parse_substrate_ref_to_format`
// (`bootstrap/src/lib.rs:1851`) + `dispatch_out_substrate_ref`
// (`:1896`). No "unknown flag" error should surface at arg-parse.

#[test]
fn t1_kintsugi_out_at_data_json_accepts_flag() {
    let out = run_kintsugi(&["--ci", "--out", "@data/json", FIXTURE]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown flag") && !stderr.contains("unrecognized"),
        "T1: `--out @data/json` must be recognized at arg-parse; got stderr:\n{}",
        stderr
    );
    assert!(
        out.status.success(),
        "T1: kintsugi --ci --out @data/json {} must exit 0; got status={} stderr={}",
        FIXTURE,
        out.status,
        stderr
    );
}

// ── T2: Output is valid JSON ────────────────────────────────────────────
//
// `--out @data/json` combined with `--ci` (the JSON emission path today)
// routes through `emit_ci_verdict_json`, producing a parseable JSON
// object at stdout. This IS the @shatter × @io linearization at
// kintsugi altitude.

#[test]
fn t2_kintsugi_out_at_data_json_emits_valid_json() {
    let out = run_kintsugi(&["--ci", "--out", "@data/json", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{') && trimmed.ends_with('}'),
        "T2: expected JSON envelope (starts with `{{`, ends with `}}`); got:\n{}",
        stdout
    );
    let _v: Value = serde_json::from_str(trimmed).unwrap_or_else(|e| {
        panic!(
            "T2: `--out @data/json` must produce parseable JSON; parse error: {}; stdout:\n{}",
            e, stdout
        )
    });
}

// ── T3: Envelope structure preserved (verdict/target/objective) ─────────
//
// The verdict envelope shape is locked by `kintsugi_ci.rs`
// (`ci_format_json_has_required_fields`); this test re-asserts the
// three semantically-load-bearing fields under the substrate-ref
// invocation shape (`--out @data/json`, not the legacy `--format=json`).
// The redundancy is by design: the MCP wrapper depends on these fields
// being present, and this shard names that dependency.

#[test]
fn t3_kintsugi_out_at_data_json_envelope_has_verdict_target_objective() {
    let out = run_kintsugi(&["--ci", "--out", "@data/json", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("T3: envelope must parse as JSON: {}; got:\n{}", e, stdout));
    for field in ["verdict", "target", "objective"] {
        assert!(
            v.get(field).is_some(),
            "T3: envelope must carry `{}` field; got: {}",
            field,
            stdout
        );
    }
    // `target` echoes the input path (round-trip lock).
    assert_eq!(
        v["target"].as_str(),
        Some(FIXTURE),
        "T3: `target` field must echo the input path; got: {}",
        v["target"]
    );
}

// ── T4: `--out @data/mirror` (or default) still produces text ──────────
//
// Backward-compat lock: the JSON path is one of many; the mirror-text
// path (default under `--ci`) MUST remain the default, and
// `--out @data/mirror` MUST explicitly select mirror-text emission.
// The MCP wrapper's fold to `--out @data/json` is a POSITIVE selection,
// not a hidden default flip.

#[test]
fn t4_kintsugi_default_ci_still_emits_mirror_text() {
    let out = run_kintsugi(&["--ci", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim_start().starts_with('{'),
        "T4: default --ci must NOT emit JSON (mirror-text default preserved); got:\n{}",
        stdout
    );
    assert!(
        stdout.trim_start().starts_with("verdict"),
        "T4: default --ci must start with `verdict <...>` (mirror-text record); got:\n{}",
        stdout
    );
}

#[test]
fn t4b_kintsugi_out_at_data_mirror_emits_mirror_text() {
    let out = run_kintsugi(&["--ci", "--out", "@data/mirror", FIXTURE]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim_start().starts_with('{'),
        "T4b: `--out @data/mirror` must emit mirror-text (not JSON); got:\n{}",
        stdout
    );
    assert!(
        stdout.trim_start().starts_with("verdict"),
        "T4b: `--out @data/mirror` must start with `verdict <...>`; got:\n{}",
        stdout
    );
}
