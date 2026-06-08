//! Tests for the kintsugi formatter loop scaffold.
//!
//! Per `docs/specs/kintsugi-formatter.md`, `mirror kintsugi --shatter N` runs
//! the five-stage iteration for at most N ticks. Today every stage is no-op,
//! so the Banach contraction's Δ is vacuously zero and the loop terminates
//! on tick 1. These tests pin that scaffold behavior.
//!
//! Per Taut #286 Win 2: these tests dispatch through `mirror::kintsugi_main`
//! in-process. The 200-800ms dyld + Accelerate startup tax that
//! `Command::new(env!("CARGO_BIN_EXE_mirror"))` paid per test is gone; the
//! whole file's wall time collapses from > 700ms to < 100ms. The
//! `[settle]` / `tick ` / `Uncrystallized` traces live on `out.stderr`
//! (captured) instead of the spawned binary's stderr (subprocess); the
//! assertions transfer 1:1.

use std::path::Path;
use std::process::Output;

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

/// In-process dispatch through `mirror::kintsugi_main` (Taut #286 Win 2).
///
/// Returns a `std::process::Output` wrapping the captured exit code,
/// stdout, and stderr — same shape the original `Command::new(...).output()`
/// call returned, so existing assertions on `.status.code()`, `.stdout`,
/// `.stderr` transfer with no per-test changes.
///
/// The kintsugi binary reads grammars via paths relative to its CWD
/// (`boot/std/mirror/grammar.mirror`), so the in-process dispatch must run
/// with the working directory set to the repo root. The cwd swap is
/// process-wide; tests that touch this run with `--test-threads=1`
/// (existing convention in the per-test invocations).
fn run_kintsugi(args: &[&str]) -> Output {
    use std::os::unix::process::ExitStatusExt;
    let mut argv: Vec<String> = vec!["mirror".to_string(), "kintsugi".to_string()];
    for a in args {
        argv.push((*a).to_string());
    }
    let out = mirror::kintsugi_main_in(&argv, repo_root());
    Output {
        // Encode exit code into the high byte of the raw status — that's
        // the form `ExitStatus::code()` decodes when there's no signal.
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

#[test]
/// `--shatter 0` is the identity flag value for the `kintsugi_tick`
/// scaffold: stdout matches the no-flag form (canonical render),
/// exit 0, and crucially — no `tick ` lines on stderr (the
/// scaffold's per-iteration diagnostic).
///
/// Per T19 (substrate-pull:realize), the single-file path threads
/// the AST through `oscillate_with_ast` and emits a `[settle]`
/// stderr trace naming the terminal oscillation state + iteration.
/// That trace is structurally distinct from the scaffold's
/// `tick ` lines (different prefix, different surface). The
/// historical pin "--shatter 0 must emit no tick lines" survives
/// in its narrow reading; the broader "stderr is empty" pin lifted
/// when T19 wired the substrate-honest settle path through the
/// boundary.
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
    let stderr = String::from_utf8_lossy(&zero.stderr);
    let tick_lines: Vec<&str> = stderr.lines().filter(|l| l.starts_with("tick ")).collect();
    assert!(
        tick_lines.is_empty(),
        "--shatter 0 must emit no `tick ` scaffold lines, got: {tick_lines:?}"
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
    assert!(
        line.contains("tick 1"),
        "first tick must be numbered 1: {}",
        line
    );
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
