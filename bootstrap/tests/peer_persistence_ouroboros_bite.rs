//! Arc-2 Tick 2.3 empirical smoke — THIRD OUROBOROS BITE.
//!
//! The pattern proven at Ticks 2.1 (f211ee48 — @spectral/signature) + 2.2
//! (2330f47 — @epistemologic/cybernetic/coherence) applied again to
//! `bootstrap/src/peer_persistence.rs` (Reed substrate-dishonest Rust
//! extension, 2026-07-14; Landing C 14.9KB — the largest bite to date).
//! The five bilateral predicates now dispatch through `apply_h::act` via
//! the CLI verb `mirror beam act @peer/persistence <predicate>`.
//!
//! Composes over:
//!   - `apply_h::act` (Arc-1 Tick 1.3 GREEN `f747a2c`) — the 7-combinator
//!     evaluator surface.
//!   - `cmd_beam_act` (Arc-1 Tick 1.4 `b189adb`) — the CLI dispatcher.
//!   - `shards/peer/persistence.mirror` (this landing) — the substrate-
//!     decl'd shard the resolver dispatches from.
//!
//! Pass = ouroboros_monotone holds (Mara-B §4.5.4):
//!   - rust_loc DECREASES (peer_persistence.rs docblock condenses to
//!     @io-boundary FLOOR announcement)
//!   - test_pass_rate STAYS 100%
//!   - io_violations DECREASES (5 bilateral predicates now dispatch via
//!     substrate-honest apply_h::act path instead of Rust-internal call)
//!   - sbec INCREASES (+5 dispatchable bilateral predicates:
//!     projection_visibility_respected, harvest_consent_verified,
//!     boot_state_coherent, home_content_addressed, home_witnessing)
//!
//! Gate: `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
//! cascade-a2-a6.md` Phase D-cascade audit.
//!
//! Pattern proves at Landing-C scale. Ticks 2.4-2.5 follow (roomba.rs,
//! roomba_walk_smoke.rs).

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

// ── T1: projection_visibility_respected dispatches Pass via CLI ────
//
// `mirror beam act @peer/persistence projection_visibility_respected` —
// the CLI synthesizes the substrate-decl'd `visibility=filter-respected`
// sentinel per shards/peer/persistence.mirror docblock (Landing A §4.1
// elevation-lattice discipline); resolver returns Pass; exit 0.

#[test]
fn t01_projection_visibility_respected_passes_via_cli() {
    let out = run_beam_act(&[
        "@peer/persistence",
        "projection_visibility_respected",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T1: projection_visibility_respected must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T1: projection_visibility_respected must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T2: harvest_consent_verified dispatches Pass via CLI ───────────

#[test]
fn t02_harvest_consent_verified_passes_via_cli() {
    let out = run_beam_act(&[
        "@peer/persistence",
        "harvest_consent_verified",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T2: harvest_consent_verified must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T2: got stdout={:?}", stdout);
}

// ── T3: boot_state_coherent dispatches Pass via CLI ────────────────

#[test]
fn t03_boot_state_coherent_passes_via_cli() {
    let out = run_beam_act(&[
        "@peer/persistence",
        "boot_state_coherent",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T3: boot_state_coherent must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T3: got stdout={:?}", stdout);
}

// ── T4: home_content_addressed dispatches Pass via CLI ─────────────

#[test]
fn t04_home_content_addressed_passes_via_cli() {
    let out = run_beam_act(&[
        "@peer/persistence",
        "home_content_addressed",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T4: home_content_addressed must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T4: got stdout={:?}", stdout);
}

// ── T5: home_witnessing (composed) dispatches Pass via CLI ─────────
//
// Landing A §4.5 composed bilateral: requires all four sub-bilaterals
// Pass. Sentinel `witnessing=all-four-pass` marks the composed discharge.

#[test]
fn t05_home_witnessing_passes_via_cli() {
    let out = run_beam_act(&[
        "@peer/persistence",
        "home_witnessing",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T5: home_witnessing must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(stdout.contains("Pass"), "T5: got stdout={:?}", stdout);
}

// ── T6: operator-supplied wrong sentinel path returns Fail ─────────
//
// When operator supplies an arg (takes precedence over CLI synthesis) that
// does NOT carry the substrate-decl'd sentinel, resolver returns Fail;
// exit 1.

#[test]
fn t06_projection_visibility_respected_fail_on_wrong_sentinel() {
    let out = run_beam_act(&[
        "@peer/persistence",
        "projection_visibility_respected",
        "operator-supplied:visibility=filter-violated",
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(1),
        "T6: projection_visibility_respected must exit 1 on wrong sentinel; stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("Fail"),
        "T6: stderr must name `Fail`; got stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("visibility=filter-respected"),
        "T6: Fail reason must name expected sentinel; got stderr={:?}",
        stderr
    );
}

// ── T7: direct apply_h::act dispatch (not through CLI) ─────────────
//
// The five bilateral predicates dispatch identically when called directly
// through the substrate's `apply_h::act` combinator surface. Composes over
// the Arc-1 Tick 1.3 evaluator floor without the CLI wrapper.

#[test]
fn t07_direct_apply_h_dispatch_all_five() {
    use mirror::apply_h::{self, Value, Verdict};

    let cases = [
        (
            "@peer/persistence.projection_visibility_respected",
            "peer-home-fixture:visibility=filter-respected",
        ),
        (
            "@peer/persistence.harvest_consent_verified",
            "peer-home-fixture:consent=chain-verified",
        ),
        (
            "@peer/persistence.boot_state_coherent",
            "peer-home-fixture:basis=snapshot-matched",
        ),
        (
            "@peer/persistence.home_content_addressed",
            "peer-home-fixture:manifest=oids-resolvable",
        ),
        (
            "@peer/persistence.home_witnessing",
            "peer-home-fixture:witnessing=all-four-pass",
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
            matches!(v, Verdict::Pass),
            "T7: {} on sentinel fixture must Pass; got {:?}",
            action,
            v
        );
    }
}
