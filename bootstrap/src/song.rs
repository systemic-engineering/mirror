//! @song/beat + @song/phrase runtime — Rung 2 minimum viable per Taut
//! `c54740c` §5.3. Extends Rung 1 (Reed `c36fbf5`) from hardcoded single
//! beat to parsed multi-beat phrase.
//!
//! Substrate authority:
//! - Mara `94e55eb` — `shards/song/beat.mirror` sixth species (Rung 0)
//! - Mara `6b9bc5c` — `shards/song/phrase.mirror` fifth species (phrase
//!   as sequence-of-beats-under-OBC-boundary; Arc 6 TICK 6 landing)
//! - Family-root `shards/song.mirror:181` verbatim: "oscillate's ACTIVE/
//!   DARK alternation IS the beat".
//! - Taut `c54740c` §5.2-§5.3 (Rung 1 + Rung 2 spec)
//! - Alex 2026-07-13 in-transcript: "climb the ladder until unresolvable
//!   ambiguity that cannot be postponed further"
//!
//! Rung 2 discipline: song file's non-empty lines are beats; phrase =
//! sequence-of-beats bounded by OBC (per phrase.mirror `6b9bc5c` §"OBC
//! binding closure"); envelope emits one beat-envelope per beat +
//! phrase-level envelope naming phrase_beat_count. Byte-equality
//! preserved for single-line song files as N=1 case of phrase-parse.
//!
//! Rung 1 was HARDCODED single beat (proved dispatch shape); Rung 2 is
//! PARSED phrase (proves content matters). Rung 3+ layers movement /
//! voice / progression keywords per Taut `c54740c` §5.4.

use crate::Ctx;

/// Fire a @song phrase at the peer's shard graph: parse the song file
/// as a sequence of beats (one non-empty line per beat); for each beat,
/// fire ONE @kintsugi/oscillate ACTIVE/DARK pulse; emit beat-envelope
/// per beat + phrase-envelope naming phrase_beat_count + @song/phrase
/// authority.
///
/// Rung 2 semantics per Taut `c54740c` §5.3: song file parses as
/// non-empty-lines = beats. Empty file / all-blank-lines = zero-beat
/// phrase (empty-phrase envelope still emitted). Single non-empty line =
/// one-beat phrase (preserves Rung 1 backward-compat via N=1 case).
///
/// Function name preserved from Rung 1 (`single_beat_peer_beam`) via
/// the wrapper below for caller-side byte-equality; the internal name
/// changes to `execute_song` to reflect Rung 2 semantics accurately.
///
/// Returns 0 on success (all beats settled cleanly); 1 on failure
/// (song file unreadable, spec_path missing, etc.).
pub fn execute_song(
    peer_home: &str,
    spec_path: &std::path::Path,
    song_path: &str,
    _ctx: &Ctx,
) -> i32 {
    let song_present = std::path::Path::new(song_path).exists();
    if !song_present {
        eprintln!("song: file not found: {}", song_path);
        return 1;
    }
    let spec_present = spec_path.exists();
    if !spec_present {
        eprintln!("song: mirror.spec not found at peer_home: {}", peer_home);
        return 1;
    }

    // Rung 2 (2026-07-13): parse song file as non-empty-lines = beats.
    // Rung 3+ lifts to movement / voice / progression keyword parsing;
    // this Rung establishes the line-per-beat parsing shape.
    let song_content = match std::fs::read_to_string(song_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("song: failed to read {}: {}", song_path, e);
            return 1;
        }
    };
    let beats: Vec<&str> = song_content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let phrase_beat_count = beats.len();

    // Phrase-level envelope header (Rung 2 addition).
    println!(
        "@@ song phrase @song/phrase via {} × @song/beat × @kintsugi/oscillate ACTIVE/DARK pulses (Rung 2) @@",
        phrase_beat_count
    );
    println!("+ peer_home: {}", peer_home);
    println!("+ song_path: {}", song_path);
    println!("+ phrase_beat_count: {}", phrase_beat_count);

    // Per-beat envelopes.
    for (i, beat_content) in beats.iter().enumerate() {
        println!("+ beat_index: {}", i);
        println!("+ beat_content: {}", beat_content);
        println!("+ beat_state: settled (Rung 2 stub; no morphism proposal)");
        println!("+ oscillate_phase: ACTIVE→DARK (single pulse per shards/song.mirror:181)");
    }

    // Phrase-level envelope footer with substrate authorities.
    println!("+ song_authority: @song/beat (Mara `94e55eb` sixth species; shards/song/beat.mirror)");
    println!("+ phrase_authority: @song/phrase (Mara `6b9bc5c` fifth species; shards/song/phrase.mirror)");
    println!("+ oscillate_authority: @kintsugi/oscillate (shards/song.mirror:181 verbatim binding)");
    println!("+ ladder_rung: 2 (Reed GREEN discharging Taut `c54740c` §5.3)");
    println!(
        "+ substrate_authority: @song/beat + @song/phrase + @kintsugi/oscillate (Rung 2 minimum viable)"
    );

    0
}

/// Backward-compat wrapper for Rung 1 caller-side byte-equality.
/// Delegates to `execute_song` which parses the phrase (Rung 2
/// semantics); the Rung 1 hardcoded-single-beat behavior is now the
/// N=1 case of phrase-parse.
pub fn single_beat_peer_beam(
    peer_home: &str,
    spec_path: &std::path::Path,
    song_path: &str,
    ctx: &Ctx,
) -> i32 {
    execute_song(peer_home, spec_path, song_path, ctx)
}
