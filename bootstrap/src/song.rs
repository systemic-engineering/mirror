//! @song/beat runtime — Rung 1 minimum viable per Taut `c54740c` §5.2.
//!
//! Substrate authority:
//! - Mara `94e55eb` — `shards/song/beat.mirror` sixth species of @song
//!   (Rung 0 landing); atomic-execution unit binding @kintsugi/oscillate
//!   ACTIVE/DARK-pulse discipline at song altitude.
//! - Family-root `shards/song.mirror:181` verbatim: "oscillate's ACTIVE/
//!   DARK alternation IS the beat".
//! - Taut `c54740c` §5.2 (Rung 1 spec): `mirror peer beam <home> --song
//!   <file>` fires hardcoded single-beat @song; emits beat-envelope.
//! - Alex 2026-07-13 in-transcript: "climb the ladder until unresolvable
//!   ambiguity that cannot be postponed further".
//!
//! Rung 1 discipline: HARDCODED single beat. Content of the `--song`
//! file is not parsed (Rung 2 lifts to multi-beat phrase). This module's
//! job is to prove the substrate binding at runtime altitude: --song
//! path parses → cmd_peer_beam dispatches here → envelope names @song/beat
//! + @kintsugi/oscillate + beat_index. Rung 2+ (multi-beat phrase execution,
//! movement/voice/progression keywords, multi-peer @dance, spectral.engineer
//! deployment) all layer on top of this foundation.
//!
//! Byte-equality preserved for non-`--song` paths: this module is only
//! entered when `cmd_peer_beam` observes `Some(song_path)` on its `song`
//! parameter; all other dispatch paths remain byte-identical.

use crate::Ctx;

/// Fire a hardcoded single-beat @song at the peer's shard graph via
/// @kintsugi/oscillate ACTIVE/DARK pulse; emit beat-envelope naming
/// @song/beat + @kintsugi/oscillate substrate authorities.
///
/// Rung 1 minimum viable per Taut `c54740c` §5.2. The song file's
/// presence triggers the dispatch; content is not parsed until Rung 2
/// lands multi-beat phrase execution. The single beat fired here is the
/// ANCHOR beat — all subsequent Rungs' multi-beat compositions layer
/// on this dispatch shape.
///
/// Returns 0 on success (single beat settled cleanly); 1 on failure
/// (song file missing, spec_path unreadable, etc.).
pub fn single_beat_peer_beam(
    peer_home: &str,
    spec_path: &std::path::Path,
    song_path: &str,
    _ctx: &Ctx,
) -> i32 {
    // Rung 1 stub: song file's presence triggers dispatch; content is
    // not parsed until Rung 2. Just verify the file exists so the
    // envelope reports meaningful provenance.
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

    // Fire ONE @kintsugi/oscillate ACTIVE/DARK pulse — the beat.strike
    // action per Mara `94e55eb` shards/song/beat.mirror §"@kintsugi/
    // oscillate binding". Rung 1 stub: log the phases without executing
    // real morphism proposal (Rung 2+ discharge).
    //
    // The envelope shape below is the load-bearing structural claim of
    // Rung 1: consumers (subsequent Rung tests + downstream MCP tools)
    // parse this shape as the substrate-honest beat-envelope. Any change
    // to field names or ordering breaks byte-equality contracts — hold
    // this shape stable through Rung 2+ via `if let Some(...)` extension.
    println!("@@ song single-beat @song/beat via @kintsugi/oscillate ACTIVE/DARK pulse (Rung 1) @@");
    println!("+ peer_home: {}", peer_home);
    println!("+ song_path: {}", song_path);
    println!("+ beat_index: 0");
    println!("+ beat_state: settled (Rung 1 stub; ACTIVE→DARK pulse without morphism proposal)");
    println!("+ oscillate_phase: ACTIVE→DARK (single pulse per shards/song.mirror:181 verbatim binding)");
    println!("+ song_authority: @song/beat (Mara `94e55eb` sixth species; shards/song/beat.mirror)");
    println!("+ oscillate_authority: @kintsugi/oscillate (shards/song.mirror:181 verbatim binding)");
    println!("+ ladder_rung: 1 (Reed GREEN discharging Taut `c54740c` §5.2)");
    println!("+ substrate_authority: @song/beat + @kintsugi/oscillate (Rung 1 minimum viable)");

    0
}
