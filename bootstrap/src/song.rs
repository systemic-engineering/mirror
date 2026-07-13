//! @song runtime — Rung 3 GREEN per Mara `d29d45e` Path B.
//!
//! Substrate authority:
//! - Mara `d29d45e` — `docs/specs/song-file-is-mirror-native-grammar.md`
//!   Path B verdict: `.song` files are mirror-native; grammar via
//!   `shards/song/keywords.mirror` companion-keyword file; parser walks
//!   tokenize+AST rather than hand-parsing lines.
//! - Mara `94e55eb` — `shards/song/beat.mirror` sixth species (Rung 0)
//! - Family-root `shards/song.mirror:181`: "oscillate's ACTIVE/DARK
//!   alternation IS the beat"
//! - Taut `c54740c` §5.2-§5.4 (Rungs 1-3 ladder spec)
//! - Reed `c36fbf5` (Rung 1) + `70766c3` (Rung 2) + `7b7fb0b` (Rung 3 RED)
//!   — song-runtime discharge chain
//! - Alex 2026-07-13 in-transcript ladder-climb mandate
//!
//! Rung 3 discipline: walk the tokenized AST emitting per-block-altitude
//! envelopes. Backward-compat with Rung 2's line-per-beat via BC-3
//! auto-wrap (implicit phrase when parse produces zero recognized blocks
//! and non-blank lines exist). Byte-equality preserved for the
//! non-`--song` code path.

use crate::ast::{AstKind, AstNode};
use crate::grammar::{parse_grammar, Grammar};
use crate::tokenize::tokenize;
use crate::Ctx;

// Rung 3 (2026-07-13) per Mara `d29d45e` Path B: bundle the song grammar
// at compile time via `include_str!` so grammar loading is CWD-independent.
// The substrate-decl direction is honored (grammar lives in `shards/song/
// keywords.mirror` at repo altitude; parser reads it verbatim from that
// canonical source; `grammar @song("song") { ... }` block is parsed by
// `parse_grammar` and merged into the tokenize dispatch). No divergence.
const SONG_KEYWORDS_SRC: &str = include_str!("../../shards/song/keywords.mirror");

fn load_song_grammar() -> Grammar {
    // Companion-only grammar: `shards/song.mirror` is the family-root
    // (prism-decl) but does not contain a `grammar { ... }` block; the
    // companion `shards/song/keywords.mirror` carries the tokenizer
    // vocabulary per Mara `d29d45e` Path B.
    let mut g = parse_grammar(SONG_KEYWORDS_SRC);
    g.r#ref = "@song".to_string();
    g
}

/// Backward-compat wrapper preserving the Rung 1 dispatch name; delegates
/// to `execute_song` which Rung 3 lifts to tokenize+AST walk.
pub fn single_beat_peer_beam(
    peer_home: &str,
    spec_path: &std::path::Path,
    song_path: &str,
    ctx: &Ctx,
) -> i32 {
    execute_song(peer_home, spec_path, song_path, ctx)
}

/// Fire a @song at the peer's shard graph. Rung 3: parse via tokenize+
/// AST walk per Mara `d29d45e` Path B; emit per-block-altitude envelopes
/// (song, movement, voice, progression, narrative, phrase, beat) each
/// naming its substrate authority. Rung 2 line-per-beat fixture BC-
/// preserved via auto-wrap discipline.
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

    let source = match std::fs::read_to_string(song_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("song: failed to read {}: {}", song_path, e);
            return 1;
        }
    };

    // Rung 3: parse via compile-time-embedded @song grammar (per
    // `shards/song/keywords.mirror`). CWD-independent load; test
    // contexts + production peer_home dispatch both work.
    let grammar = load_song_grammar();
    let ast = tokenize(source.as_bytes(), &grammar);

    // Auto-wrap discipline per Mara `d29d45e` §7 BC-2: if the source
    // parses to zero `song`-headed blocks but has non-blank lines,
    // treat as Rung 2 line-per-beat legacy and fall through to that
    // discharge path.
    let has_song_block = ast
        .children
        .iter()
        .any(|c| c.kind == AstKind::Focus && c.keyword == "song");
    if !has_song_block {
        return execute_legacy_line_per_beat(peer_home, song_path, &source);
    }

    // Rung 3 tokenize+AST walk. Emit per-block envelopes.
    for song_node in &ast.children {
        if song_node.kind == AstKind::Focus && song_node.keyword == "song" {
            emit_song_envelope(peer_home, song_path, song_node);
        }
    }

    0
}

/// Rung 2 backward-compat: parse non-blank lines as beats + emit per-
/// beat envelope. Preserved verbatim from Reed `70766c3` for BC-2
/// implicit-wrap discipline.
fn execute_legacy_line_per_beat(peer_home: &str, song_path: &str, source: &str) -> i32 {
    let beats: Vec<&str> = source
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let phrase_beat_count = beats.len();

    println!(
        "@@ song phrase @song/phrase via {} × @song/beat × @kintsugi/oscillate ACTIVE/DARK pulses (Rung 2 legacy line-per-beat auto-wrap) @@",
        phrase_beat_count
    );
    println!("+ peer_home: {}", peer_home);
    println!("+ song_path: {}", song_path);
    println!("+ phrase_beat_count: {}", phrase_beat_count);

    for (i, beat_content) in beats.iter().enumerate() {
        println!("+ beat_index: {}", i);
        println!("+ beat_content: {}", beat_content);
        println!("+ beat_state: settled (Rung 2 stub; no morphism proposal)");
        println!("+ oscillate_phase: ACTIVE→DARK (single pulse per shards/song.mirror:181)");
    }

    println!("+ song_authority: @song/beat (Mara `94e55eb` sixth species; shards/song/beat.mirror)");
    println!("+ phrase_authority: @song/phrase (Mara `6b9bc5c` fifth species; shards/song/phrase.mirror)");
    println!("+ oscillate_authority: @kintsugi/oscillate (shards/song.mirror:181 verbatim binding)");
    println!("+ ladder_rung: 2 (Reed GREEN discharging Taut `c54740c` §5.3 via Rung 3 BC auto-wrap)");
    println!(
        "+ substrate_authority: @song/beat + @song/phrase + @kintsugi/oscillate (Rung 2 minimum viable)"
    );

    0
}

/// Rung 3 song-envelope emitter. Walks `song X { movement { ... } voice
/// { ... } progression { ... } phrase { ... } narrative { ... } }` nested
/// blocks per Mara `d29d45e` §3.2 formal grammar.
fn emit_song_envelope(peer_home: &str, song_path: &str, song_node: &AstNode) {
    println!(
        "@@ song @song/{} via @song/beat × @kintsugi/oscillate ACTIVE/DARK pulses (Rung 3) @@",
        song_node.name
    );
    println!("+ peer_home: {}", peer_home);
    println!("+ song_path: {}", song_path);
    println!("+ song: {}", song_node.name);

    let mut has_movement = false;
    let mut has_voice = false;
    let mut has_progression = false;
    let mut has_phrase = false;

    for child in &song_node.children {
        match (child.kind, child.keyword.as_str()) {
            (AstKind::Focus, "movement") => {
                has_movement = true;
                println!("+ movement: {}", child.name);
                emit_nested_blocks(child, &mut has_voice, &mut has_progression, &mut has_phrase);
            }
            (AstKind::Focus, "voice") => {
                has_voice = true;
                println!("+ voice: {}", child.name);
                emit_voice_fields(child);
            }
            (AstKind::Focus, "progression") => {
                has_progression = true;
                println!("+ progression: {}", child.name);
                emit_progression_fields(child);
            }
            (AstKind::Focus, "phrase") => {
                has_phrase = true;
                println!("+ phrase: {}", child.name);
            }
            (AstKind::Focus, "narrative") => {
                println!("+ narrative: {}", child.name);
            }
            (AstKind::Focus, "beat") => {
                println!("+ beat: {}", child.name);
            }
            _ => {}
        }
    }

    // Absent-block markers ensure T1 asserts on all four keys land even
    // when the fixture omits one. For the minimum-viable fixture (Mara
    // `d29d45e` §3.4), all four are present via the movement's nested
    // voice/progression/phrase blocks.
    if !has_movement {
        println!("+ movement: <none>");
    }
    if !has_voice {
        println!("+ voice: <none>");
    }
    if !has_progression {
        println!("+ progression: <none>");
    }
    if !has_phrase {
        println!("+ phrase: <none>");
    }

    println!("+ song_authority: @song (Reed `f01cf9f` family-root)");
    println!("+ movement_authority: @song/movement (Mara `4efbf16`)");
    println!("+ voice_authority: @song/voice (Mara `cc5a440`)");
    println!("+ progression_authority: @song/progression (Mara `54ff1e8`)");
    println!("+ phrase_authority: @song/phrase (Mara `6b9bc5c`)");
    println!("+ narrative_authority: @song/narrative (Mara `0434a39`)");
    println!("+ beat_authority: @song/beat (Mara `94e55eb`)");
    println!("+ oscillate_authority: @kintsugi/oscillate (shards/song.mirror:181)");
    println!("+ ladder_rung: 3 (Reed GREEN discharging Taut `c54740c` §5.4 per Mara `d29d45e` Path B)");
    println!(
        "+ substrate_authority: @song + species + @kintsugi/oscillate (Rung 3 nested-block parsing)"
    );
}

/// Walk the movement's nested children, propagating presence flags for
/// the caller's absent-block detection.
fn emit_nested_blocks(
    movement_node: &AstNode,
    has_voice: &mut bool,
    has_progression: &mut bool,
    has_phrase: &mut bool,
) {
    for child in &movement_node.children {
        match (child.kind, child.keyword.as_str()) {
            (AstKind::Focus, "voice") => {
                *has_voice = true;
                println!("+ voice: {}", child.name);
                emit_voice_fields(child);
            }
            (AstKind::Focus, "progression") => {
                *has_progression = true;
                println!("+ progression: {}", child.name);
                emit_progression_fields(child);
            }
            (AstKind::Focus, "phrase") => {
                *has_phrase = true;
                println!("+ phrase: {}", child.name);
            }
            (AstKind::Focus, "beat") => {
                println!("+ beat: {}", child.name);
            }
            (AstKind::Focus, "narrative") => {
                println!("+ narrative: {}", child.name);
            }
            _ => {}
        }
    }
}

/// Emit voice-block field lines per Mara `d29d45e` §3.2 voice_field
/// grammar.
fn emit_voice_fields(voice_node: &AstNode) {
    for field in &voice_node.children {
        if field.kind == AstKind::Project {
            match field.keyword.as_str() {
                "scope" | "lines" | "stepwise_or_leap" | "attribution_discipline"
                | "narcissus_watch" => {
                    println!("+ {}: {}", field.keyword, field.name);
                }
                _ => {}
            }
        }
    }
}

/// Emit progression-block field lines per Mara `d29d45e` §3.2
/// progression_field grammar. Cadence type (authentic/plagal/deceptive/
/// half) is the Rung 3 T2 asserted field.
fn emit_progression_fields(progression_node: &AstNode) {
    for field in &progression_node.children {
        if field.kind == AstKind::Project {
            match field.keyword.as_str() {
                "cadence_type" | "phase" | "from" | "to"
                | "directed_toward_cadence" | "coupling" | "convergence"
                | "narcissus_watch" | "voice" => {
                    println!("+ {}: {}", field.keyword, field.name);
                }
                _ => {}
            }
        }
    }
}
