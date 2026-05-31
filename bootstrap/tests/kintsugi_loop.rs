//! Tests for the kintsugi formatter loop scaffold.
//!
//! Per `docs/specs/kintsugi-formatter.md`, `mirror kintsugi --shatter N` runs
//! the five-stage iteration for at most N ticks. Today every stage is no-op,
//! so the Banach contraction's Δ is vacuously zero and the loop terminates
//! on tick 1. These tests pin that scaffold behavior.

use std::path::Path;
use std::process::Command;

fn repo_root() -> &'static Path {
    // CARGO_MANIFEST_DIR points at bootstrap/; its parent is the repo root.
    // Grammar lookups in the binary use paths relative to its CWD
    // (e.g. "boot/std/mirror/grammar.mirror"), so we run from there.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    // Leak the owned path to satisfy the &'static signature — we read it once
    // per test and the process is short-lived.
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

#[test]
/// `--shatter 0` is the identity flag value: stdout matches the no-flag form
/// (canonical render), stderr is empty, exit 0. This is the historical
/// behavior pinned.
fn shatter_zero_matches_default() {
    let path = "boot/std/mirror/reload.mirror";
    let default = run_kintsugi(&[path]);
    let zero = run_kintsugi(&["--shatter", "0", path]);
    assert!(default.status.success(), "default kintsugi failed");
    assert!(zero.status.success(), "--shatter 0 failed");
    assert_eq!(
        default.stdout, zero.stdout,
        "--shatter 0 stdout must match default"
    );
    assert!(
        zero.stderr.is_empty(),
        "--shatter 0 must emit no tick lines, got: {}",
        String::from_utf8_lossy(&zero.stderr)
    );
}

#[test]
/// `--shatter 5` emits exactly one tick line. The Banach contraction's Δ
/// is vacuously zero on tick 1 (every stage is no-op ⇒ prior == current),
/// so stage 5 returns the Lawvere fixed point and the loop breaks before
/// tick 2.
fn shatter_five_terminates_on_tick_one() {
    let path = "boot/std/mirror/reload.mirror";
    let out = run_kintsugi(&["--shatter", "5", path]);
    assert!(out.status.success(), "--shatter 5 exit non-zero");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let tick_lines: Vec<&str> = stderr.lines().filter(|l| l.starts_with("tick ")).collect();
    assert_eq!(
        tick_lines.len(),
        1,
        "expected exactly 1 tick line, got {}: {:?}",
        tick_lines.len(),
        tick_lines
    );
    let line = tick_lines[0];
    assert!(line.contains("tick 1"), "first tick must be numbered 1: {}", line);
    assert!(
        line.contains("\u{0394}: 0.0"),
        "\u{0394} must be 0.0 on the vacuous fixed point: {}",
        line
    );
    assert!(
        line.contains("Lawvere fixed-point"),
        "line must name the Lawvere stopping criterion: {}",
        line
    );
}

#[test]
/// Negative shatter values are rejected at the parser. The flag accepts
/// only u64, so `-1` fails to parse and the binary exits non-zero before
/// touching the loop.
fn shatter_negative_rejected() {
    let path = "boot/std/mirror/reload.mirror";
    let out = run_kintsugi(&["--shatter", "-1", path]);
    assert!(
        !out.status.success(),
        "--shatter -1 must fail to parse, got success"
    );
}

#[test]
/// Per Seam C2 (pre-merge adversarial review, 2026-05-30):
/// `kintsugi_tick` must actually dispatch through the
/// `Crystallizations<H>` table, not consume it with `let _ =`. The
/// floor is empty in Tick A, so the dispatch should return
/// `CrystallizeError::Uncrystallized` for whatever Ref the tick
/// chooses (e.g. `@kintsugi/tick`). That failure must surface in the
/// tick output — either in the tick stderr line or via a dedicated
/// line — so the integration is exercised end-to-end. The empty-floor
/// fact does NOT justify omitting the dispatch; every invocation
/// should demonstrate one dispatch attempt against the empty registry.
///
/// This test asserts: with `--shatter 1`, stderr mentions the
/// Uncrystallized dispatch outcome at the substrate Ref the tick
/// resolves through.
fn shatter_one_dispatches_through_crystallizations() {
    let path = "boot/std/mirror/reload.mirror";
    let out = run_kintsugi(&["--shatter", "1", path]);
    assert!(
        out.status.success(),
        "--shatter 1 must exit 0, stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Uncrystallized"),
        "shatter-1 stderr must surface the empty-floor dispatch outcome \
         (CrystallizeError::Uncrystallized for the tick's resolved Ref); got:\n{}",
        stderr
    );
    // The Ref the tick resolves through should appear in the dispatch
    // diagnostic so the operator sees WHICH path was attempted.
    assert!(
        stderr.contains("@kintsugi"),
        "shatter-1 stderr must name the resolved Ref (e.g. @kintsugi/tick); got:\n{}",
        stderr
    );
}
