//! Phase G v0 — empirical-path-traversal proof for `mirror spawn`.
//!
//! Per Mara's spawn-semantics insight (2026-06-26, commit b10f00c at
//! `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md`):
//! spawn IS the substrate's controlled excitation above λ₀. The seven-
//! piece composition (insight §2.1–§2.7) must compose. v0 implements
//! pieces 1–3 (cli surface + @peer resolution via filesystem + pack{}.
//! lead extraction) as real reads; pieces 4–7 (lead-at-N+1 obligation,
//! @spectral/supervisor kick, @fate runtime, λ₀-excitation) are logged
//! stubs that prove the path traversed all seven. Phase H wires real
//! @fate.
//!
//! Tests dispatch through `mirror::kintsugi_main_in` in-process per
//! Taut #286 Win 2 (the established pattern across this directory).

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_spawn(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "spawn".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

const TEST_PEER: &str = "bootstrap/tests/fixtures/spawn-test-peer";

#[test]
fn spawn_exits_zero_on_valid_peer_home() {
    let out = run_spawn(&[TEST_PEER]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spawn must exit 0 on valid peer home; got {:?}\nstderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn spawn_emits_peer_name_from_spec() {
    let out = run_spawn(&[TEST_PEER]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Insight §2.2 piece: @peer resolution reads <home>/mirror.spec and
    // extracts the project name. The fixture's project name is
    // `test-peer`.
    assert!(
        stdout.contains("test-peer"),
        "spawn output must mention the spawned peer name (test-peer); got:\n{}",
        stdout
    );
}

#[test]
fn spawn_emits_lead_from_pack_block() {
    let out = run_spawn(&[TEST_PEER]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Insight §2.3 piece: contextual pack read from the spec's pack{}
    // block. The fixture's pack{}.lead is ~peer'~/.test-lead'; output
    // must mention `test-lead` (the lead identifier) to prove the pack
    // block was traversed.
    assert!(
        stdout.contains("test-lead"),
        "spawn output must mention lead from pack{{}} block (test-lead); got:\n{}",
        stdout
    );
}

#[test]
fn spawn_emits_envelope_naming_seven_pieces() {
    let out = run_spawn(&[TEST_PEER]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // The envelope names all seven composition pieces by their substrate-
    // pull anchors so the path-traversal is empirically checkable.
    for anchor in [
        "peer=",
        "home=",
        "lead=",
        "excitation=",
        "supervisor=",
        "fate=",
        "probe_channel=",
    ] {
        assert!(
            stdout.contains(anchor),
            "envelope must name composition piece anchor `{}`; got:\n{}",
            anchor,
            stdout
        );
    }
}

#[test]
fn spawn_exits_non_zero_on_missing_home() {
    let out = run_spawn(&["bootstrap/tests/fixtures/does-not-exist"]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "spawn must exit non-zero when peer home is absent; got {:?}\nstdout: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout)
    );
}
