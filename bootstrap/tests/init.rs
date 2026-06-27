//! P3 RED — `mirror init <path>` contract per Mara spec §4.7.
//!
//! Per Mara's mirror-init spec (`docs/specs/mirror-init.md`, commits
//! `fe215bd` → `14dd043`, ~1208 lines) + Seam's audit (commit `8392ab5`;
//! 0C/3S/8L/12✓) + Reed's Cargo-edge GREEN (commit `6b36808`; bridge
//! verified; R2 = 0 bytes Δ empirically): mirror init is the bridge
//! command that makes the declared substrate operational.
//!
//! These tests assert the envelope shape contract per spec §4.7. They
//! fail against the cmd_init stub in `lib.rs` until the GREEN tick wires
//! the real composition (NamespacedGitStore + project::project +
//! per-file Splinter + set_ref("HEAD", root)).
//!
//! Pattern mirrors `bootstrap/tests/spawn.rs` + `bootstrap/tests/recall.rs`:
//! in-process dispatch via `mirror::kintsugi_main_in`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_init(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "init".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

fn parse_envelope(stdout: &[u8]) -> serde_json::Value {
    let s = String::from_utf8_lossy(stdout);
    serde_json::from_str(s.trim()).unwrap_or_else(|e| {
        panic!(
            "mirror init stdout must be valid JSON; got:\n{}\nerr: {}",
            s, e
        )
    })
}

#[test]
fn init_exits_zero_on_valid_repo() {
    let out = run_init(&["."]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "init must exit 0 on valid repo; got {:?}\nstderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// P3 RED: envelope must carry the 9 contract keys per Mara spec §4.7.
#[test]
fn init_envelope_carries_contract_keys() {
    let out = run_init(&["."]);
    let envelope = parse_envelope(&out.stdout);
    for key in [
        "spec_version",
        "operation",
        "repo",
        "store",
        "indexed",
        "bytes_total",
        "root_oid",
        "hooks_installed",
        "verdict",
    ] {
        assert!(
            envelope.get(key).is_some(),
            "init envelope must carry '{}' per spec §4.7; got: {}",
            key,
            envelope
        );
    }
}

/// P3 RED: envelope.operation must equal "init".
#[test]
fn init_operation_equals_init() {
    let out = run_init(&["."]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["operation"].as_str().unwrap_or(""),
        "init",
        "envelope.operation must == 'init'; got: {}",
        envelope["operation"]
    );
}

/// P3 RED: envelope.spec_version must == "v0.1.0" (matches spawn/recall envelope vocabulary).
#[test]
fn init_spec_version_v0_1_0() {
    let out = run_init(&["."]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["spec_version"].as_str().unwrap_or(""),
        "v0.1.0",
        "envelope.spec_version must match round-trip vocabulary; got: {}",
        envelope["spec_version"]
    );
}

/// P3 RED: hooks_installed defaults to false (no --install-hooks flag).
#[test]
fn init_hooks_default_false() {
    let out = run_init(&["."]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["hooks_installed"].as_bool(),
        Some(false),
        "envelope.hooks_installed must default false without --install-hooks; got: {}",
        envelope["hooks_installed"]
    );
}

#[test]
fn init_exits_non_zero_on_missing_repo() {
    let out = run_init(&["/nonexistent/mirror-init-test"]);
    // RED: stub does not validate the repo path. GREEN must reject
    // non-existent paths with a typed error.
    assert_ne!(
        out.status.code(),
        Some(0),
        "init must exit non-zero when repo path is missing"
    );
}

#[test]
fn init_exits_non_zero_on_missing_arg() {
    let argv = vec!["mirror".to_string(), "init".to_string()];
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    assert_ne!(
        out.exit_code, 0,
        "init without args must exit non-zero (usage message)"
    );
}
