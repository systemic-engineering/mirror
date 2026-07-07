//! Tick 3 RED — `cmd_craft --target-kind <kind>` substrate-honest alias.
//!
//! /loop close @torus to rest, Tick 3. Substrate-decl at mirror.spec
//! `1e45c50` declares `flag target_kind: str` on `command craft`. The
//! substrate-honest name (matches mirror.spec's `emit` vocabulary and
//! disambiguates today's binary `target` positional + `--target` flag
//! same-name collision) is `--target-kind`. Today's binary parses
//! `--target <kind>` only. This RED lands the alias obligation:
//! `--target-kind <kind>` MUST parse identically to `--target <kind>`;
//! `--target` stays as backward-compat alias per two-tick discipline.
//!
//! Parse-only tests (no heavy binary/crystal builds): validates that
//! parsing errors surface, that unknown-value errors surface, and
//! that the backward-compat `--target` alias still parses cleanly.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_craft(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "craft".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

#[test]
fn t01_target_kind_without_value_errors_with_value_required_message() {
    // `mirror craft --target-kind` (no value) must error non-zero with a
    // "requires a value" message; the same shape as --target.
    let out = run_craft(&["--target-kind"]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "--target-kind without value must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = stderr.contains("requires a value") && stderr.contains("target");
    assert!(
        ok,
        "stderr must mention 'requires a value' and 'target'; got:\n{}",
        stderr
    );
}

#[test]
fn t02_target_kind_with_unknown_value_errors_with_unknown_message() {
    // `mirror craft --target-kind wat boot` must error non-zero and name
    // the unknown value.
    let out = run_craft(&["--target-kind", "wat", "boot"]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "--target-kind wat must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = stderr.contains("unknown") && stderr.contains("wat");
    assert!(
        ok,
        "stderr must mention 'unknown' and 'wat'; got:\n{}",
        stderr
    );
}

#[test]
fn t03_target_kind_equals_form_with_unknown_value_errors() {
    // `mirror craft --target-kind=wat boot` (equals form) must error the
    // same way as the separate-value form.
    let out = run_craft(&["--target-kind=wat", "boot"]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "--target-kind=wat must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = stderr.contains("unknown") && stderr.contains("wat");
    assert!(
        ok,
        "stderr must mention 'unknown' and 'wat'; got:\n{}",
        stderr
    );
}

#[test]
fn t04_target_alias_still_errors_on_unknown_value_backward_compat() {
    // `mirror craft --target wat boot` (legacy --target) must ALSO still
    // exit non-zero with "unknown" — backward compat verification.
    let out = run_craft(&["--target", "wat", "boot"]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "--target wat (backward-compat alias) must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let ok = stderr.contains("unknown") && stderr.contains("wat");
    assert!(
        ok,
        "stderr must mention 'unknown' and 'wat' for --target too; got:\n{}",
        stderr
    );
}
