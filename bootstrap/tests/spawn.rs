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

// ─────────────────────────────────────────────────────────────────────────────
// P4 RED (2026-06-27) — --hello-world flag emits structured JSON envelope.
//
// Per the substrate round-trip loop endpoint: `mirror spawn
// ~peer'~/.reed' --hello-world` reads peer's mirror.spec and returns
// structured envelope identifying peer by declared content. The text
// envelope (existing tests above) stays the default; --hello-world
// opts in to JSON.
//
// b10f00c §4 fences still apply: no @fate, no @io/llm, no
// identity-mint. Hello-world is bounded to reading the peer's
// mirror.spec content; no live state queries.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn spawn_hello_world_emits_json_envelope() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "--hello-world stdout must be valid JSON; got:\n{}\nparse error: {}",
            stdout, e
        )
    });
    assert_eq!(
        envelope["spawn"].as_str().unwrap_or(""),
        "hello_world",
        "envelope.spawn must equal 'hello_world'; got: {}",
        stdout
    );
}

#[test]
fn spawn_hello_world_envelope_carries_peer_identity() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("valid JSON envelope");
    assert_eq!(
        envelope["peer"].as_str().unwrap_or(""),
        "test-peer",
        "envelope.peer must carry declared project name; got: {}",
        stdout
    );
    let lead = envelope["lead"].as_str().unwrap_or("");
    assert!(
        lead.contains("test-lead"),
        "envelope.lead must carry declared pack lead; got: {}",
        lead
    );
}

#[test]
fn spawn_hello_world_envelope_names_seven_composition_pieces() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("valid JSON envelope");
    let pieces = envelope["composition_pieces"]
        .as_object()
        .expect("composition_pieces must be a JSON object");
    // Per insight b10f00c §2.1–§2.7: seven composition pieces.
    assert!(
        pieces.len() >= 7,
        "envelope.composition_pieces must name all 7 pieces; got {} keys: {:?}",
        pieces.len(),
        pieces.keys().collect::<Vec<_>>()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// P4.5 RED (2026-06-27) — peer_recall composition: the peer's hello
// IS their own recall.
//
// Per Mara's circular-reflexive observation (349bce7 §3.6): the recall
// envelope's payloads correspond to sheaf-section-with-temporal-axis
// at coherence altitude. spawn --hello-world should compose its lead-
// side envelope WITH the peer-side recall envelope, so the lead
// observes the peer's psychohistory_vector in one breath.
//
// In-process composition: cmd_spawn invokes the same recall_*
// helpers against peer_home. NO subprocess (yet); piece 5
// supervisor.start_child stays stub. This proves piece-6-via-recall
// (structured observation without @fate inference; b10f00c §2.6
// substitution form).
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn spawn_hello_world_envelope_carries_peer_recall() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("valid JSON envelope");
    assert!(
        envelope.get("peer_recall").is_some(),
        "spawn --hello-world envelope must carry 'peer_recall' (the peer's own \
         recall payloads composed in); got:\n{}",
        stdout
    );
    assert!(
        envelope["peer_recall"].is_object(),
        "peer_recall must be a JSON object; got: {}",
        envelope["peer_recall"]
    );
}

#[test]
fn spawn_hello_world_peer_recall_has_four_payloads() {
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let envelope: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim())
            .expect("valid JSON envelope");
    let peer_recall = envelope["peer_recall"]
        .as_object()
        .expect("peer_recall must be an object");
    // Per Mara b034a60 §3: the four payloads are cascade /
    // pack_trail / pull_frontier / dogfood. Each anchors at a
    // typed content-address per §3.6 sheaf-section-with-temporal-axis.
    for key in ["cascade", "pack_trail", "pull_frontier", "dogfood"] {
        assert!(
            peer_recall.get(key).is_some(),
            "peer_recall must carry '{}' payload (the peer's own \
             psychohistory_vector slot per d00f553); got keys: {:?}",
            key,
            peer_recall.keys().collect::<Vec<_>>()
        );
    }
}

#[test]
fn spawn_hello_world_peer_recall_declares_spec_version() {
    // P5 contract extension: the nested peer_recall envelope also
    // carries spec_version so composition versioning flows through
    // the round-trip transitively (lead envelope + peer_recall both
    // declare the same spec_version).
    let out = run_spawn(&[TEST_PEER, "--hello-world"]);
    let envelope: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim())
            .expect("valid JSON envelope");
    let peer_recall = &envelope["peer_recall"];
    let outer_version = envelope["spec_version"]
        .as_str()
        .expect("outer envelope spec_version");
    let peer_version = peer_recall["spec_version"].as_str().unwrap_or_else(|| {
        panic!(
            "peer_recall must declare spec_version (composition versioning \
             flows through nested envelope); got peer_recall: {}",
            peer_recall
        )
    });
    assert_eq!(
        outer_version, peer_version,
        "outer envelope ({}) and peer_recall ({}) must declare the \
         same spec_version",
        outer_version, peer_version
    );
}

#[test]
fn spawn_no_flag_still_emits_text_envelope() {
    // Regression: the 5 tests above assert the text-envelope shape.
    // --hello-world is opt-in; default behavior unchanged.
    let out = run_spawn(&[TEST_PEER]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("peer="),
        "default (no flag) envelope must remain text shape; got:\n{}",
        stdout
    );
    assert!(
        !stdout.trim_start().starts_with('{'),
        "default envelope must NOT be JSON (that's --hello-world only); got:\n{}",
        stdout
    );
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
