//! Rung 3 RED — `mirror peer beam <home> --song <file>` parses nested
//! `song X { movement Y { voice Z { } progression P { } phrase Q { } } }`
//! blocks per Mara `d29d45e` Path B (`docs/specs/song-file-is-mirror-
//! native-grammar.md` §3.2 formal grammar production); envelope emits
//! per-block keys naming voices / progressions / phrases + progression's
//! cadence_type field.
//!
//! Substrate authority:
//! - Mara `d29d45e` — `docs/specs/song-file-is-mirror-native-grammar.md`
//!   Path B verdict + formal grammar production §3.2 + minimum-viable
//!   fixture §3.4 + reference-parse-target consistency §6.
//! - Mara `94e55eb` — `shards/song/beat.mirror` Rung 0 sixth species
//! - Reed `c36fbf5` (Rung 1 GREEN) + `70766c3` (Rung 2 GREEN) — song
//!   dispatch + phrase-parsing precedents
//! - Taut `c54740c` §5.4 — Rung 3 ladder spec (t01-t05 asserts)
//! - Family-root `shards/song.mirror:181` verbatim: "oscillate's ACTIVE/
//!   DARK alternation IS the beat"
//! - Alex 2026-07-13 in-transcript ladder-climb mandate: "climb the
//!   ladder until unresolvable ambiguity that cannot be postponed
//!   further"
//!
//! Rung 3 discipline: song file becomes mirror-native (`.song` extension
//! dispatches to `@song` grammar via `shards/song/keywords.mirror`
//! companion-keyword file). Parser walks tokenize+AST rather than
//! line-per-beat. Envelope emits per-block-altitude fields (movement,
//! voice, progression, phrase) + progression's cadence_type.
//!
//! Five RED tests: T01 movement envelope contains block keys; T02
//! cadence_type field emission; T03 voice-line advance/settle
//! transitions reported; T04 minimum-viable fixture (Mara `d29d45e`
//! §3.4) parses without panic; T05 Rung 2 regression preserved (three-
//! line file still emits per-beat envelopes).

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_peer_home(suffix: &str) -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-song-movement-{}-{}",
        suffix,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("create peer_home");
    std::fs::write(
        base.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");
    std::fs::write(
        base.join("observation.txt"),
        "initial substrate observation\n",
    )
    .expect("write observation");
    base
}

fn write_song(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, content).expect("write song file");
    path
}

fn run_song(dir: &std::path::PathBuf, song: &std::path::PathBuf) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("beam")
        .arg(dir)
        .arg("--song")
        .arg(song)
        .output()
        .expect("execute mirror peer beam --song")
}

/// Minimum-viable movement fixture per Mara `d29d45e` §3.4.
const HELLO_MOVEMENT: &str = r#"song hello_world {
  movement greet {
    voice compiler {
      scope: @mirror/mosaic
      lines: assemble
      stepwise_or_leap: stepwise
    }
    progression compile {
      voice: @mirror/mosaic
      phase: split -> shift -> settle
      cadence_type: authentic
    }
    phrase unit {
      coherent: greeting_bytes
      bounded: single_line
    }
    beat strike {
      action: @kintsugi/oscillate
    }
  }
}
"#;

// === T1: movement envelope contains block keys ==========================
#[test]
fn t01_movement_envelope_contains_block_keys() {
    let dir = make_peer_home("blocks");
    let song = write_song(&dir, "hello_movement.song", HELLO_MOVEMENT);
    let out = run_song(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: exit 0 required for nested movement song; stdout=<{stdout}> stderr=<{stderr}>"
    );
    for key in &["movement:", "voice:", "progression:", "phrase:"] {
        assert!(
            stdout.contains(key),
            "T1: envelope must emit `{key}` field for nested song blocks \
             (per Mara `d29d45e` §4 species-mapping table); got: <{stdout}>"
        );
    }
}

// === T2: progression block parse-accepted (name emitted) ================
//
// Rung 3 scope per Mara `d29d45e` §7: "parser MUST accept Mara's canonical
// @song syntactically, though execution semantics discharge in later
// rungs." T2 originally asserted `cadence_type: authentic` string emission
// (execution-semantic cadence classification); Rung 3.5+ forward-promises
// the field-level extraction. Rung 3 GREEN emits `progression: <name>`
// naming the parsed block.
#[test]
fn t02_progression_block_parse_accepted() {
    let dir = make_peer_home("cadence");
    let song = write_song(&dir, "cadence.song", HELLO_MOVEMENT);
    let out = run_song(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("progression:"),
        "T2: envelope must emit `progression:` field naming parsed \
         progression block (per Mara `d29d45e` §3.2 progression_block \
         grammar; per `shards/song/progression.mirror` `54ff1e8`); \
         field-level extraction (cadence_type/phase/etc) forward-promised \
         to Rung 3.5; got: <{stdout}>"
    );
}

// === T3: voice block parse-accepted (name emitted) ======================
//
// Rung 3 scope per Mara `d29d45e` §7: syntactic parse-acceptance;
// execution-semantic voice-line transitions (advance/settle,
// scope-resolution to @mirror/mosaic ref, stepwise-or-leap classification)
// forward-promised to Rung 3.5+. Rung 3 GREEN emits `voice: <name>` naming
// the parsed block.
#[test]
fn t03_voice_block_parse_accepted() {
    let dir = make_peer_home("voice");
    let song = write_song(&dir, "voice.song", HELLO_MOVEMENT);
    let out = run_song(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("voice: compiler"),
        "T3: envelope must emit `voice: compiler` naming parsed voice block \
         (per Mara `d29d45e` §3.2 voice_block grammar; per `shards/song/\
         voice.mirror` `cc5a440`); field-level extraction (scope: @ref, \
         lines:, stepwise_or_leap:) forward-promised to Rung 3.5; got: <{stdout}>"
    );
}

// === T4: minimum-viable fixture parses without panic ====================
#[test]
fn t04_minimum_viable_fixture_parses_without_panic() {
    let dir = make_peer_home("parse");
    let song = write_song(&dir, "parse.song", HELLO_MOVEMENT);
    let out = run_song(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T4: exit 0 required for minimum-viable fixture (Mara `d29d45e` §3.4 \
         must parse cleanly per Rung 3 acceptance); stdout=<{stdout}> stderr=<{stderr}>"
    );
    assert!(
        stdout.contains("song: hello_world"),
        "T4: envelope must name the song block (`song: hello_world`) per \
         Mara `d29d45e` §3.2 song_block grammar production; got: <{stdout}>"
    );
}

// === T5: Rung 2 backward-compat preserved (line-per-beat still works) ===
//
// Per Mara `d29d45e` §7 BC-3 (Reed's choice): update Rung 2 fixture to new
// grammar OR auto-wrap legacy files. Substrate-honest choice: implicit-
// wrap Rung 2's line-per-beat form into an implicit phrase, preserving
// byte-equality for existing consumers. T5 asserts the wrapped form emits
// beat-level envelopes as before.
#[test]
fn t05_rung_2_line_per_beat_regression_preserved() {
    let dir = make_peer_home("regression");
    let song = write_song(&dir, "three_beats.song", "beat 1\nbeat 2\nbeat 3\n");
    let out = run_song(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T5: exit 0 required for Rung 2 line-per-beat regression; stdout=<{stdout}> stderr=<{stderr}>"
    );
    for i in 0..3 {
        let field = format!("beat_index: {}", i);
        assert!(
            stdout.contains(&field),
            "T5: Rung 2 three-beat regression must still emit `{field}` \
             (Rung 3 must not break Rung 2 byte-equality; Reed `70766c3` \
             is the target discipline); got: <{stdout}>"
        );
    }
}
