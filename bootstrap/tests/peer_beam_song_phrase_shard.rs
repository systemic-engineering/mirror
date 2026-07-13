//! Rung 2 RED — `mirror peer beam <home> --song <file>` executes a
//! multi-beat phrase; each non-empty line of the song file fires ONE
//! @kintsugi/oscillate ACTIVE/DARK pulse; envelope names @song/phrase
//! authority + emits phrase_beat_count field.
//!
//! Substrate authority:
//! - Mara `94e55eb` — shards/song/beat.mirror sixth species (Rung 0)
//! - Mara `6b9bc5c` — shards/song/phrase.mirror (fifth species; phrase
//!   as sequence-of-beats-under-OBC-boundary; Arc 6 TICK 6 landing)
//! - Reed `c36fbf5` — Rung 1 GREEN (single-beat dispatch); this Rung
//!   extends the same `song` module to parse phrase = sequence-of-beats
//! - Taut `c54740c` §5.3 — Rung 2 spec: multi-beat phrase execution
//! - Family-root shards/song.mirror §phrase.mirror `6b9bc5c` §"OBC
//!   binding closure": phrase IS a sequence of beats bounded by the
//!   ambiguity-load budget
//! - Alex 2026-07-13 in-transcript: "climb the ladder"
//!
//! Rung 2 discipline: song file parsed as N non-empty lines = N beats;
//! each beat fires @kintsugi/oscillate ACTIVE/DARK pulse; phrase-
//! envelope emitted at the end naming phrase_beat_count. Byte-equality
//! preserved for single-line song files (Rung 1 discharge path).
//!
//! Four RED tests: T1 multi-line beat_index enumeration (0, 1, 2);
//! T2 phrase_beat_count field emission; T3 @song/phrase authority
//! naming; T4 single-line backward-compat (Rung 1 envelope shape
//! preserved).

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_peer_home(suffix: &str) -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-song-phrase-{}-{}",
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

fn run_song_phrase(
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

// === T1: multi-line song emits beat_index enumeration ==================
//
// Substrate provenance: Taut `c54740c` §5.3 Rung 2: song file's non-empty
// lines are beats. A 3-beat song must emit `beat_index: 0`, `beat_index:
// 1`, `beat_index: 2` in the envelope stdout, verifying that the runtime
// PARSES the phrase content (not just detects file presence per Rung 1).

#[test]
fn t01_multi_line_emits_beat_index_enumeration() {
    let dir = make_peer_home("multi");
    let song = write_song(&dir, "three_beats.song", "beat 1\nbeat 2\nbeat 3\n");
    let out = run_song_phrase(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: exit 0 required for multi-beat song; stdout=<{stdout}> stderr=<{stderr}>"
    );
    for i in 0..3 {
        let field = format!("beat_index: {}", i);
        assert!(
            stdout.contains(&field),
            "T1: envelope must emit `{field}` for 3-beat phrase (per Taut \
             `c54740c` §5.3 Rung 2 parse-not-hardcode); got: <{stdout}>"
        );
    }
}

// === T2: envelope emits phrase_beat_count field =========================
//
// Substrate provenance: Mara `6b9bc5c` shards/song/phrase.mirror declares
// phrase as sequence-of-beats-under-OBC-boundary. Rung 2 envelope must
// name the beat count to make the phrase's cardinality empirically
// verifiable at CLI altitude.

#[test]
fn t02_envelope_emits_phrase_beat_count() {
    let dir = make_peer_home("count");
    let song = write_song(&dir, "three_beats.song", "beat 1\nbeat 2\nbeat 3\n");
    let out = run_song_phrase(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("phrase_beat_count: 3"),
        "T2: envelope must emit `phrase_beat_count: 3` for 3-beat phrase \
         (per Mara `6b9bc5c` phrase.mirror OBC binding); got: <{stdout}>"
    );
}

// === T3: envelope names @song/phrase substrate authority ================
//
// Substrate provenance: Mara `6b9bc5c` shards/song/phrase.mirror fifth
// species mint. Multi-beat execution IS phrase-altitude discipline;
// envelope must name @song/phrase alongside @song/beat + @kintsugi/
// oscillate to preserve the substrate-authority citation chain.

#[test]
fn t03_envelope_names_song_phrase_authority() {
    let dir = make_peer_home("authority");
    let song = write_song(&dir, "three_beats.song", "beat 1\nbeat 2\nbeat 3\n");
    let out = run_song_phrase(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("@song/phrase"),
        "T3: envelope must name @song/phrase authority for multi-beat \
         execution (per Mara `6b9bc5c` fifth species mint); got: <{stdout}>"
    );
}

// === T4: single-line song preserves Rung 1 backward-compat ==============
//
// Substrate provenance: byte-equality discipline. Rung 1 (Reed
// `c36fbf5`) envelope shape for single-beat --song dispatch must remain
// intact when Rung 2 parses the song file. The specific structural
// claim: a single-non-empty-line song emits the same @song/beat +
// @kintsugi/oscillate + beat_index: 0 fields as Rung 1's hardcoded
// single-beat dispatch.

#[test]
fn t04_single_line_preserves_rung_1_shape() {
    let dir = make_peer_home("single");
    let song = write_song(&dir, "one_beat.song", "beat 1\n");
    let out = run_song_phrase(&dir, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        out.status.success(),
        "T4: exit 0 required for single-line song (Rung 1 backward-compat); got: <{stdout}>"
    );
    assert!(
        stdout.contains("@song/beat"),
        "T4: single-line song must preserve @song/beat authority (Rung 1); got: <{stdout}>"
    );
    assert!(
        stdout.contains("@kintsugi/oscillate"),
        "T4: single-line song must preserve @kintsugi/oscillate authority (Rung 1); got: <{stdout}>"
    );
    assert!(
        stdout.contains("beat_index: 0"),
        "T4: single-line song must emit beat_index: 0 (Rung 1); got: <{stdout}>"
    );
    assert!(
        stdout.contains("phrase_beat_count: 1"),
        "T4: single-line song must emit phrase_beat_count: 1 (Rung 2 unifies \
         single/multi via phrase-parses-song discipline); got: <{stdout}>"
    );
}
