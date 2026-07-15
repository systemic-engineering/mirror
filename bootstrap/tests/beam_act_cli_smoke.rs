//! Arc-1 Tick 1.4 smoke test — `mirror beam act` CLI verb end-to-end.
//!
//! Composes over `apply_h::act` (Tick 1.3 GREEN `f747a2c`) + the CLI
//! dispatcher (Tick 1.4 `[substrate-floor:@io-boundary]`). This test IS
//! the first user-invocable substrate dispatch smoke: sbec empirically
//! lifts from 0 to > 0 the instant `mirror beam act
//! @subject/visibility/public consent_scope_universal` returns Pass
//! with exit code 0.
//!
//! Alex directive verbatim (2026-07-15, in-transcript):
//!   "Sounds perfect. Let's get that shipped and then effectively call
//!    `mirror roomba --commit`. I want the compiler itself to make the
//!    commit, you know? The whole end2end flow as an empirical CLI call
//!    proof."
//!
//! Pattern mirrors `bootstrap/tests/cmd_beam_anonymous_shard.rs`:
//! in-process dispatch via `mirror::kintsugi_main_in` for stdout/stderr
//! capture without the 200-800ms dyld + Accelerate startup tax.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_beam_act(args: &[&str]) -> Output {
    let mut argv: Vec<String> =
        vec!["mirror".to_string(), "beam".to_string(), "act".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

// ── T1: empirical target ────────────────────────────────────────────
//
// `mirror beam act @subject/visibility/public consent_scope_universal`
// must return Pass with exit code 0. This is the load-bearing anchor
// per Alex 2026-07-15 directive.

#[test]
fn t01_empirical_target_consent_scope_universal_passes() {
    let out = run_beam_act(&[
        "@subject/visibility/public",
        "consent_scope_universal",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T1: empirical target must exit 0; stdout={:?} stderr={:?}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T1: empirical target must print `Pass` on stdout; got stdout={:?}",
        stdout
    );
}

// ── T2: usage on missing args ────────────────────────────────────────
//
// `mirror beam act` with no shard-path / action must exit non-zero and
// emit a usage line naming the arg shape.

#[test]
fn t02_usage_on_missing_args() {
    let out = run_beam_act(&[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(0),
        "T2: `mirror beam act` with no args must exit non-zero"
    );
    assert!(
        stderr.contains("beam act") || stderr.contains("shard-path"),
        "T2: usage line must mention `beam act` / `shard-path`; \
         got stderr={:?}",
        stderr
    );
}

// ── T3: operator-supplied arg on Fail path ───────────────────────────
//
// When operator supplies an arg that does NOT carry the [everyone]
// sentinel, the resolver must return Fail (exit 1) with a diagnostic
// naming the missing sentinel.

#[test]
fn t03_operator_arg_without_sentinel_fails() {
    let out = run_beam_act(&[
        "@subject/visibility/public",
        "consent_scope_universal",
        "wrong-scope:no-sentinel",
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(1),
        "T3: Fail verdict must exit 1; stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("Fail"),
        "T3: stderr must name `Fail`; got stderr={:?}",
        stderr
    );
}

// ── T4: unresolved shard_action_ref returns Partial ──────────────────
//
// An action not in the Tick 1.3 MVP resolver returns Partial via the
// spec §1.4 last-arm — Transparency::opaque naming the missing
// shard_action_ref. Marshaled to exit 2 per Mara-B §1.4 semantics.

#[test]
fn t04_unresolved_action_returns_partial() {
    let out = run_beam_act(&[
        "@subject/visibility/public",
        "not_in_mvp_resolver_action",
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(2),
        "T4: Partial verdict must exit 2; stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("Partial"),
        "T4: stderr must name `Partial`; got stderr={:?}",
        stderr
    );
}
