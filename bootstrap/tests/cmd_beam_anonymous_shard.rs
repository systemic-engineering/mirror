//! Tick 3 Landing 3 RED — `cmd_beam` dispatch for
//! `mirror beam <mission>` (anonymous variant).
//!
//! Substrate-decl at mirror.spec Landing 1 (OID
//! `b1ebb881414458f81bfc6ec463aaafba005679144bc9693d63aee8869019d119`)
//! declares top-level `command beam { arg mission: ~f }` alongside the
//! nested `command peer { command beam { ... } }`. The anonymous variant
//! is the primitive form per docs/specs/beam-as-substrate-primitive.md
//! §3 composition table: beam-without-persistent-identity — substrate
//! accepts a mission and returns @song without binding trajectory to a
//! peer-home.
//!
//! Both variants dispatch to the same substrate action
//! @mirror/peer/beam.beam (shards/mirror/peer/beam.mirror:310); runtime
//! differentiation is on positional-arg shape:
//! - `mirror beam <mission-file>`      — anonymous
//! - `mirror peer beam ~peer'<home>'`  — persistent-identity
//!
//! Pattern mirrors `bootstrap/tests/spawn_mission_shard.rs` +
//! `bootstrap/tests/cmd_peer_beam_shard.rs`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_beam(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "beam".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

const TEST_MISSION: &str = "bootstrap/tests/fixtures/spawn-task-mission.txt";

// ── T1: top-level `beam` subcommand recognized ────────────────────────
//
// Pre-Landing-2 the binary rejects `mirror beam` with `unknown: beam`.
// Post-Landing-2 the dispatch must accept `mirror beam <mission>` as
// the anonymous variant of `@mirror/peer/beam.beam`.

#[test]
fn t01_beam_subcommand_is_recognized() {
    let out = run_beam(&[TEST_MISSION]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown: beam"),
        "T1: `mirror beam` must be a recognized top-level subcommand \
         (not `unknown: beam`); stderr:\n{}",
        stderr
    );
}

// ── T2: usage line names --hello-world / mission ──────────────────────
//
// Per Landing 1 mirror.spec: `command beam { arg mission: ~f, flag
// hello_world: bool = false }`. The dispatcher's usage() line must
// expose the flag surface so operators see it on missing-args.

#[test]
fn t02_usage_line_names_beam_surface() {
    let out = run_beam(&[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(0),
        "T2: `mirror beam` with no positional must exit non-zero (mission \
         is required arg)"
    );
    assert!(
        stderr.contains("beam") || stderr.contains("mission"),
        "T2: usage line for `mirror beam` must mention beam / mission; \
         got stderr:\n{}",
        stderr
    );
}

// ── T3: mission read as positional arg ────────────────────────────────
//
// Per Landing 1 mirror.spec: `arg mission: ~f`. The dispatcher must
// accept the mission-file path as a positional argument (not a flag).

#[test]
fn t03_beam_mission_positional_accepted() {
    let out = run_beam(&[TEST_MISSION, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T3: `mirror beam <mission> --hello-world` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
