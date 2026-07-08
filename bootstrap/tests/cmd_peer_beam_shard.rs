//! Tick 3 Landing 3 RED — `cmd_peer_beam` dispatch for
//! `mirror peer beam ~peer'<home>'` (persistent-identity variant).
//!
//! Substrate-decl at mirror.spec Landing 1 (OID
//! `b1ebb881414458f81bfc6ec463aaafba005679144bc9693d63aee8869019d119`)
//! declares nested `command peer { command beam { ... } }` under the
//! recursive-command grammar landed at @mirror/lens/cli Tick 1
//! (`fe82500`). The dispatch surface at bootstrap/src/lib.rs must
//! recognize `mirror peer beam <peer-home>` as the persistent-identity
//! variant of `@mirror/peer/beam.beam` (shards/mirror/peer/beam.mirror:310).
//!
//! Composition:
//! - Tick 1 grammar (recursive-command depth-2, `fe82500`): `command
//!   peer { command beam { ... } }` admissible.
//! - Tick 2 substrate-decl rename (`9de2226`): substrate action lives
//!   at @mirror/peer/beam, not @mirror/spawn.
//! - Fault-plane #1 (Taut scout `bd837cd`): @pack.spawn at pack altitude
//!   (shards/pack.mirror:263) UNCHANGED. Divergence preserved.
//!
//! Backward-compat: `mirror spawn ~peer'<home>'` continues to dispatch
//! to `cmd_peer_beam` via the "spawn" alias arm; the alias emits a
//! deprecation warning to stderr per two-tick discipline. Byte-equal
//! envelope on stdout preserves round-trip guarantees.
//!
//! Pattern mirrors `bootstrap/tests/spawn_mission_shard.rs` and
//! `bootstrap/tests/shatter_target_shard.rs`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_mirror(subcommand_path: &[&str], args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string()];
    argv.extend(subcommand_path.iter().map(|s| s.to_string()));
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

const TEST_PEER: &str = "bootstrap/tests/fixtures/spawn-test-peer";
const TEST_MISSION: &str = "bootstrap/tests/fixtures/spawn-task-mission.txt";

// ── T1: subcommand nesting recognized ─────────────────────────────────
//
// The dispatch must accept `mirror peer beam <peer-home>` as a
// recognized nested subcommand. Pre-Landing-2 the binary rejects
// `mirror peer` with `unknown: peer` (only compile/craft/kintsugi/
// init/recall/spawn/shatter arms exist).

#[test]
fn t01_peer_beam_subcommand_nesting_is_recognized() {
    let out = run_mirror(&["peer", "beam"], &[TEST_PEER]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("unknown: peer"),
        "T1: `mirror peer beam` must be a recognized nested subcommand \
         (not `unknown: peer`); stderr:\n{}",
        stderr
    );
    assert!(
        !stderr.contains("unknown: beam"),
        "T1: `mirror peer beam` must not be rejected at `beam` sub-arm \
         (would indicate incomplete nesting dispatch); stderr:\n{}",
        stderr
    );
}

// ── T2: exit 0 on valid peer-home ─────────────────────────────────────
//
// The persistent-identity dispatch must succeed on the same fixture
// `mirror spawn` accepts today. The envelope shape is `@song` per
// shards/mirror/peer/beam.mirror:310; --hello-world opts into the
// structured JSON form (mirrors spawn.rs pattern).

#[test]
fn t02_peer_beam_exits_zero_on_valid_peer_home() {
    let out = run_mirror(&["peer", "beam"], &[TEST_PEER, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T2: `mirror peer beam <peer-home> --hello-world` must exit 0; \
         stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// ── T3: envelope carries @song / spawn identity ───────────────────────
//
// The dispatch routes to the same substrate action `beam` at
// @mirror/peer/beam; the envelope shape must be byte-compatible with
// today's `mirror spawn --hello-world` output. Envelope carries
// `spawn: "hello_world"` and `peer: "test-peer"` (regression against
// spawn.rs assertions).

#[test]
fn t03_peer_beam_envelope_carries_song_identity() {
    let out = run_mirror(&["peer", "beam"], &[TEST_PEER, "--hello-world"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "T3: --hello-world stdout must be valid JSON @song envelope; \
             got:\n{}\nparse error: {}",
            stdout, e
        )
    });
    assert_eq!(
        envelope["spawn"].as_str().unwrap_or(""),
        "hello_world",
        "T3: envelope.spawn must equal 'hello_world' (round-trip identity \
         with mirror spawn); got: {}",
        stdout
    );
    assert_eq!(
        envelope["peer"].as_str().unwrap_or(""),
        "test-peer",
        "T3: envelope.peer must carry declared project name; got: {}",
        stdout
    );
}

// ── T4: spawn alias emits deprecation warning ─────────────────────────
//
// Per two-tick discipline: `mirror spawn` continues to dispatch to
// cmd_peer_beam but MUST emit a deprecation warning naming the
// substrate-honest cli surface `mirror peer beam`. The warning goes
// to stderr so stdout envelope stays byte-equal.

#[test]
fn t04_spawn_alias_emits_deprecation_warning() {
    let out = run_mirror(&["spawn"], &[TEST_PEER, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "T4: `mirror spawn` (alias) must still exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    // Substrate-honest: name the new surface + mark the alias status.
    let mentions_alias = stderr.contains("deprecat")
        || stderr.contains("alias")
        || stderr.contains("mirror peer beam");
    assert!(
        mentions_alias,
        "T4: `mirror spawn` must emit a deprecation notice on stderr \
         naming `mirror peer beam` (deprecation / alias / substrate-\
         honest surface); got stderr:\n{}",
        stderr
    );
}

// ── T5: spawn alias envelope byte-equal to peer beam envelope ─────────
//
// Backward-compat contract: the stdout envelope from `mirror spawn` and
// `mirror peer beam` on the same fixture must be byte-identical.
// Deprecation warning is stderr-scoped; stdout stays canonical.

#[test]
fn t05_spawn_alias_stdout_byte_equal_to_peer_beam() {
    let out_spawn = run_mirror(&["spawn"], &[TEST_PEER, "--hello-world"]);
    let out_peer_beam = run_mirror(&["peer", "beam"], &[TEST_PEER, "--hello-world"]);
    assert_eq!(
        out_spawn.status.code(),
        Some(0),
        "T5: `mirror spawn` alias must exit 0"
    );
    assert_eq!(
        out_peer_beam.status.code(),
        Some(0),
        "T5: `mirror peer beam` must exit 0"
    );
    assert_eq!(
        out_spawn.stdout, out_peer_beam.stdout,
        "T5: `mirror spawn` and `mirror peer beam` stdout envelopes must be \
         byte-identical (deprecation warning is stderr-scoped; stdout stays \
         canonical @song envelope)"
    );
}

// ── T6: --mission flag composes at peer beam ──────────────────────────
//
// The substrate-honest --mission flag (landed at `8d6e9af`) must
// compose with the persistent-identity `mirror peer beam` variant.
// Envelope carries mission field; round-trip with existing --mission
// contract preserved.

#[test]
fn t06_peer_beam_mission_flag_composes() {
    let out = run_mirror(
        &["peer", "beam"],
        &[TEST_PEER, "--mission", TEST_MISSION, "--hello-world"],
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "T6: `mirror peer beam <peer> --mission <path> --hello-world` must \
         exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("T6: envelope must be valid JSON");
    let mission = envelope["mission"].as_str().unwrap_or_else(|| {
        panic!(
            "T6: envelope.mission must be a string when --mission set at \
             peer beam altitude; got:\n{}",
            stdout
        )
    });
    assert!(
        mission.contains("collapse the substrate"),
        "T6: envelope.mission must carry mission-file contents; got: {:?}",
        mission
    );
}
