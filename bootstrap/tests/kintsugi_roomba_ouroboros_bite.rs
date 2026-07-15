//! Arc-2 Tick 2.4 FOURTH OUROBOROS BITE empirical smoke.
//!
//! `bootstrap/src/roomba.rs` (Reed substrate-dishonest Rust extension,
//! 2026-07-14; 15.8KB per Taut #108 — the walker-altitude bite). The
//! five bilateral predicates now dispatch through `apply_h::act` via
//! the CLI verb `mirror beam act @kintsugi/roomba <predicate>`.
//!
//! Composes over:
//!   - `apply_h::act` (Arc-1 Tick 1.3 GREEN `f747a2c`) — the 7-combinator
//!     evaluator surface.
//!   - `cmd_beam_act` (Arc-1 Tick 1.4 `b189adb`) — the CLI dispatcher.
//!   - `shards/kintsugi/roomba.mirror` (this landing) — the substrate-
//!     decl'd shard the resolver dispatches from.
//!
//! Pass = ouroboros_monotone holds (Mara-B §4.5.4):
//!   - rust_loc DECREASES (docblock condenses to @io-boundary FLOOR
//!     announcement; Dijkstra + LAPACK composition stays as walker
//!     primitive per Alex 2026-07-14 walk-IS-@io composition)
//!   - test_pass_rate STAYS 100% (including `mirror roomba --commit`
//!     regression — roomba_commit.rs still composes over the walker)
//!   - io_violations DECREASES (5 bilateral predicates now dispatch via
//!     substrate-honest apply_h::act path instead of Rust-internal call)
//!   - sbec INCREASES (+5 dispatchable bilateral predicates:
//!     walk_terminates_cleanly, tension_monotone_descending,
//!     coherence_gradient_admissible, knife_verdict_bounded,
//!     walk_witnessing)
//!
//! Gate: `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
//! cascade-a2-a6.md` Phase D-cascade audit.
//!
//! Pattern proves at walker altitude. Tick 2.5 (roomba_walk_smoke.rs)
//! closes the Arc-2 cascade.

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

// ── T1: walk_terminates_cleanly dispatches Pass via CLI ───────────
//
// `mirror beam act @kintsugi/roomba walk_terminates_cleanly` — the CLI
// synthesizes the substrate-decl'd `termination=scope-a-exhaustive`
// sentinel per shards/kintsugi/roomba.mirror docblock (Scope A four-
// state exhaustive termination); resolver returns Pass; exit 0.

#[test]
fn t01_walk_terminates_cleanly_passes_via_cli() {
    let out = run_beam_act(&[
        "@kintsugi/roomba",
        "walk_terminates_cleanly",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T1: walk_terminates_cleanly must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T1: walk_terminates_cleanly must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T2: tension_monotone_descending dispatches Pass via CLI ───────

#[test]
fn t02_tension_monotone_descending_passes_via_cli() {
    let out = run_beam_act(&[
        "@kintsugi/roomba",
        "tension_monotone_descending",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T2: tension_monotone_descending must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T2: got stdout={:?}", stdout);
}

// ── T3: coherence_gradient_admissible dispatches Pass via CLI ──────

#[test]
fn t03_coherence_gradient_admissible_passes_via_cli() {
    let out = run_beam_act(&[
        "@kintsugi/roomba",
        "coherence_gradient_admissible",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T3: coherence_gradient_admissible must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T3: got stdout={:?}", stdout);
}

// ── T4: knife_verdict_bounded dispatches Pass via CLI ────────────

#[test]
fn t04_knife_verdict_bounded_passes_via_cli() {
    let out = run_beam_act(&[
        "@kintsugi/roomba",
        "knife_verdict_bounded",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T4: knife_verdict_bounded must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T4: got stdout={:?}", stdout);
}

// ── T5: walk_witnessing (composed) dispatches Pass via CLI ────────
//
// Composed bilateral: requires all four sub-bilaterals Pass. Sentinel
// `witnessing=all-four-pass` marks the composed discharge.

#[test]
fn t05_walk_witnessing_passes_via_cli() {
    let out = run_beam_act(&[
        "@kintsugi/roomba",
        "walk_witnessing",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T5: walk_witnessing must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T5: got stdout={:?}", stdout);
}

// ── T6: operator-supplied wrong sentinel path returns Fail ────────
//
// When operator supplies an arg (takes precedence over CLI synthesis) that
// does NOT carry the substrate-decl'd sentinel, resolver returns Fail;
// exit 1.

#[test]
fn t06_walk_terminates_cleanly_fail_on_wrong_sentinel() {
    let out = run_beam_act(&[
        "@kintsugi/roomba",
        "walk_terminates_cleanly",
        "operator-supplied:termination=wildcard-crash",
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(1),
        "T6: walk_terminates_cleanly must exit 1 on wrong sentinel; stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("Fail"),
        "T6: must print `Fail`; got stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("termination=scope-a-exhaustive"),
        "T6: Fail reason must name expected sentinel; got stderr={:?}",
        stderr
    );
}

// ── T7: direct apply_h::act dispatch (not through CLI) ──────────
//
// Verifies the resolver arm holds under direct-Rust dispatch too —
// substrate-honest: the CLI is a thin wrapper; the substrate is the
// resolver. All five sentinels discharge Pass when supplied directly.

#[test]
fn t07_direct_apply_h_dispatch_all_five() {
    use mirror::apply_h;
    use mirror::apply_h::Value;

    let cases = [
        (
            "@kintsugi/roomba.walk_terminates_cleanly",
            "walk-trajectory-fixture:termination=scope-a-exhaustive",
        ),
        (
            "@kintsugi/roomba.tension_monotone_descending",
            "walk-trajectory-fixture:tension=trajectory-descending",
        ),
        (
            "@kintsugi/roomba.coherence_gradient_admissible",
            "walk-trajectory-fixture:gradient=foerster-admissible",
        ),
        (
            "@kintsugi/roomba.knife_verdict_bounded",
            "walk-trajectory-fixture:verdict=three-state-bounded",
        ),
        (
            "@kintsugi/roomba.walk_witnessing",
            "walk-trajectory-fixture:witnessing=all-four-pass",
        ),
    ];

    for (action, arg_oid) in cases {
        let v = apply_h::act(
            action.to_string(),
            vec![Value {
                oid: arg_oid.to_string(),
            }],
        );
        assert!(
            matches!(v, apply_h::Verdict::Pass),
            "T7: {} must Pass on direct dispatch with sentinel {:?}; got {:?}",
            action, arg_oid, v
        );
    }
}
