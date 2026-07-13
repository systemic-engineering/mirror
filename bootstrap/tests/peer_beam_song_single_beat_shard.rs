//! Rung 1 RED — `mirror peer beam <home> --song <file>` fires a hardcoded
//! single-beat @song through @kintsugi/oscillate ACTIVE/DARK pulse and
//! emits a beat-envelope naming @song/beat + @kintsugi/oscillate as
//! substrate authorities.
//!
//! Substrate authority:
//! - Mara `94e55eb` — `shards/song/beat.mirror` sixth species of @song
//!   (Rung 0 landing); `type song_beat = ref` + `strike(b, ctx)` +
//!   `hold(b, dt)` operational surface
//! - Taut `c54740c` §5.2 — testable-increment ladder Rung 1 spec
//! - Family-root `shards/song.mirror:181` verbatim: "oscillate's
//!   ACTIVE/DARK alternation IS the beat"
//! - Alex 2026-07-13 in-transcript: "climb the ladder until unresolvable
//!   ambiguity that cannot be postponed further"
//! - Reed `130084e` — CURRENT.md Rung 1 tracking
//!
//! Rung 1 minimum viable delivers:
//!   1. Grammar `flag song: ~f` in `mirror.spec` for `command peer
//!      { command beam { ... } }` (following the `flag mission: ~f`
//!      pattern verbatim).
//!   2. New `bootstrap/src/song.rs` module with `single_beat_song`
//!      hardcoded execution path firing @kintsugi/oscillate ACTIVE/DARK
//!      pulse.
//!   3. `--song <path>` parsing in `cmd_peer_beam` dispatch cascade
//!      (branch fires before other flags per Taut ladder Rung 1 spec).
//!   4. Beat-envelope emission naming @song/beat + @kintsugi/oscillate
//!      substrate authorities + `beat_index:` + `beat_state:` fields.
//!
//! Byte-equality preserved for non-`--song` paths via same
//! `if let Some(...)` discipline as existing `--mission` at
//! `bootstrap/src/lib.rs:5254-5262`.
//!
//! Four RED tests — T1 exit code + flag acceptance; T2 @song/beat
//! authority naming; T3 @kintsugi/oscillate authority naming; T4
//! beat_index field emission. All 4 must GREEN in the Rung 1 landing.

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_peer_home() -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-song-single-beat-{}",
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

fn make_song_file(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("single_beat.song");
    // Rung 1 hardcodes a single beat in the runtime; the song-file
    // presence triggers the dispatch. Content doesn't matter for Rung 1
    // (Rung 2+ parses multi-beat phrases per Taut c54740c §5.3).
    std::fs::write(&path, "beat 1\n").expect("write song file");
    path
}

fn run_song_beat(
    dir: &std::path::PathBuf,
    song: &std::path::PathBuf,
) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("beam")
        .arg(dir)
        .arg("--song")
        .arg(song)
        .output()
        .expect("execute mirror peer beam --song")
}

// === T1: --song flag accepted; process exits 0 ==========================
//
// Substrate provenance: Rung 1 minimum viable per Taut `c54740c` §5.2:
// `mirror peer beam <home> --song <file>` must parse the flag + dispatch
// without failing. Prior to Rung 1 landing, `--song` was not admitted
// by mirror.spec grammar nor parsed by cmd_peer_beam; the run should
// fail (usage message on stderr, non-zero exit) — that's the RED lock.
// Rung 1 GREEN flips exit to 0.

#[test]
fn t01_song_flag_accepted_exit_zero() {
    let dir = make_peer_home();
    let song = make_song_file(&dir);
    let out = run_song_beat(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: exit 0 required with --song flag; stdout=<{stdout}> stderr=<{stderr}>"
    );
}

// === T2: envelope names @song/beat substrate authority ==================
//
// Substrate provenance: Mara `94e55eb` `shards/song/beat.mirror` declares
// the sixth species of @song at path-namespace `@song/beat`. Rung 1
// runtime must cite this authority in the beat-envelope so consumers
// (and future Rung 2-6 tests) can verify the substrate binding by string
// match. Same discipline as `--with-shadow` naming `@song/narrative`
// (per peer_beam_shadow_casting_shard.rs T3).

#[test]
fn t02_envelope_names_song_beat_authority() {
    let dir = make_peer_home();
    let song = make_song_file(&dir);
    let out = run_song_beat(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("@song/beat"),
        "T2: envelope must name @song/beat as substrate authority \
         (per Mara `94e55eb` sixth-species mint); got: <{stdout}>"
    );
}

// === T3: envelope names @kintsugi/oscillate authority ==================
//
// Substrate provenance: `shards/song.mirror:181` verbatim: "oscillate's
// ACTIVE/DARK alternation IS the beat". Mara `94e55eb` beat.mirror §
// "@kintsugi/oscillate binding" makes this identity load-bearing at
// species altitude. Rung 1 runtime IS this binding at operational
// altitude — the beat's `strike` fires ONE @kintsugi/oscillate ACTIVE/
// DARK pulse. The envelope must cite @kintsugi/oscillate to make the
// binding empirically verifiable.

#[test]
fn t03_envelope_names_oscillate_authority() {
    let dir = make_peer_home();
    let song = make_song_file(&dir);
    let out = run_song_beat(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("@kintsugi/oscillate"),
        "T3: envelope must name @kintsugi/oscillate as substrate authority \
         (per shards/song.mirror:181 verbatim binding); got: <{stdout}>"
    );
}

// === T4: envelope emits beat_index field ================================
//
// Substrate provenance: Rung 1 hardcodes a single-beat @song. The
// envelope must emit `beat_index:` naming the beat's ordinal position
// in the (hardcoded) sequence — for Rung 1 this is always 0 or 1, but
// the FIELD's presence is the load-bearing structural claim: the
// runtime observes beats as ORDERED discrete events, not as unstructured
// output. Rung 2+ leverages this field for phrase-level assertions.

#[test]
fn t04_envelope_emits_beat_index() {
    let dir = make_peer_home();
    let song = make_song_file(&dir);
    let out = run_song_beat(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("beat_index:"),
        "T4: envelope must emit `beat_index:` field for hardcoded single \
         beat (Rung 1 minimum viable structural claim); got: <{stdout}>"
    );
}
