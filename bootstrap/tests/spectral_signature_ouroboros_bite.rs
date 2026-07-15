//! Arc-2 Tick 2.1 empirical smoke — FIRST OUROBOROS BITE.
//!
//! The substrate mends its own Rust with mirror. This test IS the
//! empirical proof: `bootstrap/src/spectral_signature.rs` (Reed
//! substrate-dishonest Rust extension, 2026-07-14) now dispatches its
//! four bilateral predicates through `apply_h::act` via the CLI verb
//! `mirror beam act @spectral/signature <predicate>`.
//!
//! Composes over:
//!   - `apply_h::act` (Arc-1 Tick 1.3 GREEN `f747a2c`) — the 7-combinator
//!     evaluator surface.
//!   - `cmd_beam_act` (Arc-1 Tick 1.4 `b189adb`) — the CLI dispatcher.
//!   - `shards/spectral/signature.mirror` (Arc-2 Tick 2.1) — the
//!     substrate-decl'd shard the resolver dispatches from.
//!
//! Pass = ouroboros_monotone holds:
//!   - rust_loc DECREASES (176 → 143)
//!   - test_pass_rate STAYS 100%
//!   - io_violations DECREASES (Rust module bypassed for predicates)
//!   - sbec INCREASES (+4 dispatchable bilateral predicates)
//!
//! Gate: `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
//! cascade-a2-a6.md` Phase D-cascade audit.

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

// ── T1: signature_integrity dispatches Pass via CLI ───────────────
//
// `mirror beam act @spectral/signature signature_integrity` — the CLI
// synthesizes the substrate-decl'd `chain=merkle-linked` sentinel per
// shards/spectral/signature.mirror docblock; resolver returns Pass; exit 0.

#[test]
fn t01_signature_integrity_passes_via_cli() {
    let out = run_beam_act(&[
        "@spectral/signature",
        "signature_integrity",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T1: signature_integrity must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T1: signature_integrity must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T2: signature_authorship dispatches Pass via CLI ──────────────

#[test]
fn t02_signature_authorship_passes_via_cli() {
    let out = run_beam_act(&[
        "@spectral/signature",
        "signature_authorship",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T2: signature_authorship must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T2: got stdout={:?}", stdout);
}

// ── T3: signature_monotone dispatches Pass via CLI ───────────────

#[test]
fn t03_signature_monotone_passes_via_cli() {
    let out = run_beam_act(&[
        "@spectral/signature",
        "signature_monotone",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T3: signature_monotone must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T3: got stdout={:?}", stdout);
}

// ── T4: signature_composition_honest dispatches Pass via CLI ────────

#[test]
fn t04_signature_composition_honest_passes_via_cli() {
    let out = run_beam_act(&[
        "@spectral/signature",
        "signature_composition_honest",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T4: signature_composition_honest must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T4: got stdout={:?}", stdout);
}

// ── T5: operator-supplied arg without sentinel fails ──────────────
//
// Same discipline as `@subject/visibility/public.consent_scope_universal`
// per beam_act_cli_smoke.rs T3: operator arg overrides synth; missing
// sentinel → Fail (exit 1).

#[test]
fn t05_operator_arg_without_sentinel_fails() {
    let out = run_beam_act(&[
        "@spectral/signature",
        "signature_integrity",
        "wrong-sig:no-sentinel",
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(1),
        "T5: Fail verdict must exit 1; stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("Fail"),
        "T5: stderr must name `Fail`; got stderr={:?}",
        stderr
    );
}

// ── T6: apply_h::act direct dispatch (no CLI) ───────────────────
//
// The Rust surface must also accept the shard_action_ref directly (per
// evaluator_shard_body_dispatch_smoke.rs pattern for consent_scope_universal).

#[test]
fn t06_apply_h_act_direct_dispatch_all_four() {
    use mirror::apply_h::{self, Value, Verdict};

    let cases = [
        (
            "@spectral/signature.signature_integrity",
            "rolling-signature-fixture:chain=merkle-linked",
        ),
        (
            "@spectral/signature.signature_authorship",
            "rolling-signature-fixture:authorship=ssh-matched",
        ),
        (
            "@spectral/signature.signature_monotone",
            "rolling-signature-fixture:ordering=timestamp-monotone",
        ),
        (
            "@spectral/signature.signature_composition_honest",
            "rolling-signature-fixture:composition=song-emission",
        ),
    ];

    for (action, arg_oid) in cases {
        let v = apply_h::act(action.to_string(), vec![Value { oid: arg_oid.to_string() }]);
        assert!(
            matches!(v, Verdict::Pass),
            "T6: {} on sentinel fixture must Pass; got {:?}",
            action, v
        );
    }
}
