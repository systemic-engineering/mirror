//! Arc-2 Tick 2.2 empirical smoke — SECOND OUROBOROS BITE.
//!
//! The pattern proven at Tick 2.1 (f211ee48 — @spectral/signature) applied
//! again to `bootstrap/src/coherence.rs` (Reed substrate-dishonest Rust
//! extension, 2026-07-14). The four bilateral predicates now dispatch
//! through `apply_h::act` via the CLI verb `mirror beam act
//! @epistemologic/cybernetic/coherence <predicate>`.
//!
//! Composes over:
//!   - `apply_h::act` (Arc-1 Tick 1.3 GREEN `f747a2c`) — the 7-combinator
//!     evaluator surface.
//!   - `cmd_beam_act` (Arc-1 Tick 1.4 `b189adb`) — the CLI dispatcher.
//!   - `shards/epistemologic/cybernetic/coherence.mirror` (Mara `e0a3e48`) —
//!     the substrate-decl'd shard the resolver dispatches from.
//!
//! Pass = ouroboros_monotone holds (Mara-B §4.5.4):
//!   - rust_loc DECREASES (coherence.rs shrinks by removed bilateral +
//!     obsolete tests)
//!   - test_pass_rate STAYS 100%
//!   - io_violations DECREASES (4 bilateral predicates now dispatch via
//!     substrate-honest apply_h::act path instead of Rust-internal call)
//!   - sbec INCREASES (+4 dispatchable bilateral predicates:
//!     coherence_increases, is_narcissus_pole, is_splinter_pole,
//!     coherence_witnessing)
//!
//! Gate: `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
//! cascade-a2-a6.md` Phase D-cascade audit.
//!
//! Pattern proves REPEATABLE. Ticks 2.3-2.5 follow (peer_persistence,
//! roomba, roomba_walk_smoke).

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

// ── T1: coherence_increases dispatches Pass via CLI ────────────────
//
// `mirror beam act @epistemologic/cybernetic/coherence coherence_increases`
// — the CLI synthesizes the substrate-decl'd `axis=splinter-ward` sentinel
// per shards/epistemologic/cybernetic/coherence.mirror docblock (Foerster-
// admissible transition); resolver returns Pass; exit 0.

#[test]
fn t01_coherence_increases_passes_via_cli() {
    let out = run_beam_act(&[
        "@epistemologic/cybernetic/coherence",
        "coherence_increases",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T1: coherence_increases must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T1: coherence_increases must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T2: is_narcissus_pole dispatches Pass via CLI ──────────────────

#[test]
fn t02_is_narcissus_pole_passes_via_cli() {
    let out = run_beam_act(&[
        "@epistemologic/cybernetic/coherence",
        "is_narcissus_pole",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T2: is_narcissus_pole must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T2: is_narcissus_pole must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T3: is_splinter_pole dispatches Pass via CLI ───────────────────

#[test]
fn t03_is_splinter_pole_passes_via_cli() {
    let out = run_beam_act(&[
        "@epistemologic/cybernetic/coherence",
        "is_splinter_pole",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T3: is_splinter_pole must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T3: is_splinter_pole must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T4: coherence_witnessing dispatches Pass via CLI ───────────────

#[test]
fn t04_coherence_witnessing_passes_via_cli() {
    let out = run_beam_act(&[
        "@epistemologic/cybernetic/coherence",
        "coherence_witnessing",
    ]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T4: coherence_witnessing must exit 0; stdout={:?} stderr={:?}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Pass"),
        "T4: coherence_witnessing must print `Pass`; got stdout={:?}",
        stdout
    );
}

// ── T5: operator-supplied wrong sentinel path returns Fail ─────────
//
// When operator supplies an arg (takes precedence over CLI synthesis) that
// does NOT carry the substrate-decl'd sentinel, resolver returns Fail;
// exit 1.

#[test]
fn t05_coherence_increases_fail_on_wrong_sentinel() {
    let out = run_beam_act(&[
        "@epistemologic/cybernetic/coherence",
        "coherence_increases",
        "operator-supplied:axis=narcissus-ward",
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(1),
        "T5: coherence_increases must exit 1 on wrong sentinel; stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("Fail"),
        "T5: coherence_increases must print `Fail`; got stderr={:?}",
        stderr
    );
    assert!(
        stderr.contains("axis=splinter-ward"),
        "T5: Fail reason must name expected sentinel; got stderr={:?}",
        stderr
    );
}

// ── T6: direct apply_h::act dispatch (not through CLI) ─────────────
//
// The four bilateral predicates dispatch identically when called directly
// through the substrate's `apply_h::act` combinator surface. Composes over
// the Arc-1 Tick 1.3 evaluator floor without the CLI wrapper.

#[test]
fn t06_direct_apply_h_dispatch_all_four() {
    use mirror::apply_h;

    // coherence_increases
    let v = apply_h::act(
        "@epistemologic/cybernetic/coherence.coherence_increases".to_string(),
        vec![apply_h::Value {
            oid: "direct-dispatch:axis=splinter-ward".to_string(),
        }],
    );
    assert!(
        matches!(v, apply_h::Verdict::Pass),
        "T6a: coherence_increases direct dispatch must Pass; got {:?}",
        v
    );

    // is_narcissus_pole
    let v = apply_h::act(
        "@epistemologic/cybernetic/coherence.is_narcissus_pole".to_string(),
        vec![apply_h::Value {
            oid: "direct-dispatch:structure=star-K1n".to_string(),
        }],
    );
    assert!(
        matches!(v, apply_h::Verdict::Pass),
        "T6b: is_narcissus_pole direct dispatch must Pass; got {:?}",
        v
    );

    // is_splinter_pole
    let v = apply_h::act(
        "@epistemologic/cybernetic/coherence.is_splinter_pole".to_string(),
        vec![apply_h::Value {
            oid: "direct-dispatch:structure=complete-Kn".to_string(),
        }],
    );
    assert!(
        matches!(v, apply_h::Verdict::Pass),
        "T6c: is_splinter_pole direct dispatch must Pass; got {:?}",
        v
    );

    // coherence_witnessing
    let v = apply_h::act(
        "@epistemologic/cybernetic/coherence.coherence_witnessing".to_string(),
        vec![apply_h::Value {
            oid: "direct-dispatch:witness=coherence-preserving".to_string(),
        }],
    );
    assert!(
        matches!(v, apply_h::Verdict::Pass),
        "T6d: coherence_witnessing direct dispatch must Pass; got {:?}",
        v
    );
}
