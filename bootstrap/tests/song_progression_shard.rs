//! Arc 6 TICK 2 RED — `shards/song/progression.mirror` species.
//!
//! Per Seam pre-review at `3d910bc` (docs/audits/2026-07-06-seam-pre-review-song-substrate-decl.md)
//! §6.3: **TICK 2 lands `shards/song/progression.mirror` — grounds #S1 (eigenform-at-temporal)
//! + #S2 (shift-at-temporal) at species altitude; music binding.**
//!
//! Progression is the first `@song` species. It carries two of the three sub-predicates that
//! the family-root's `song_settles(pr, p) -> verdict` composed bilateral (14th #53 instance)
//! decomposes into (per `shards/song.mirror` §3 narrative):
//!
//! - `progression_directed_toward_cadence(pr, p) -> verdict` — the progression is heading
//!   somewhere (Splinter-pole discharge) vs unresolved-tension SILENCE (Narcissus-pole).
//! - `cadence_authentic_or_plagal(c, p) -> verdict` — the closure event is consonant
//!   (Splinter-pole) vs deceptive-cadence EXTRACTION (Narcissus-pole).
//!
//! The third sub-predicate (`voice_line_valid`) belongs to `@song/voice` (Arc 6 TICK 3).
//!
//! **Species-altitude grounding of #S1 + #S2 + music binding** per §6:
//!
//! - **#S1 (eigenform-at-temporal):** progression's identity emerges from tonic-return
//!   recursion `R^n(pr)` converging on the identifiable-as-this-progression invariant.
//!   Schenker's foreground/middleground/background IS substrate reading the eigenform at
//!   nested progression depths.
//! - **#S2 (shift-at-temporal):** advance action delegates to `@kintsugi/shift` at temporal
//!   altitude — the finite-difference sequence `Delta pr(t) = shift(pr(t) -> pr(t+dt))`
//!   IS a sequence of shifts with harmonic_position as the shifted witness.
//! - **Music binding:** composes with `@epistemologic/math/music/cadence` (Lawvere framing;
//!   Seam Correction `3d910bc`) + `@epistemologic/math/music` general family.
//!
//! Species sibling: `@song/voice` (Arc 6 TICK 3), `@song/movement` (TICK 4),
//! `@song/narrative` (TICK 5), `@song/phrase` (TICK 6). After three species land, promote
//! recognition #S3 (five-op temporal specialisation) from CANDIDATE to LANDED per Seam §6.8:
//! `@song` becomes the FIRST substrate species lifting all five prism operations at a
//! non-mathematical altitude.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_progression_shard() -> String {
    let path = repo_root().join("shards/song/progression.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("read shards/song/progression.mirror at {:?}: {}", path, e)
    })
}

fn first_nonempty_line(content: &str) -> Option<&str> {
    content.lines().find(|l| !l.trim().is_empty())
}

fn seam_line_indices(content: &str) -> Vec<usize> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, l)| if l == "---" { Some(i) } else { None })
        .collect()
}

// === T1-T5: canonical shape + Interpretation B baseline + species pact ===

#[test]
fn t01_song_progression_shard_file_exists_and_declares_species() {
    let content = read_progression_shard();
    assert!(
        content.contains("prism @song/progression")
            || content.contains("prism @song / progression"),
        "T1: shards/song/progression.mirror must declare `prism @song/progression` species per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_progression_shard();
    let first = first_nonempty_line(&content).expect("T2: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T2: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T2: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T2: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t03_exactly_one_seam_at_column_zero_and_in_clauses_below() {
    let content = read_progression_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T3: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
    let seam_idx = seams[0];
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T3: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t04_progression_inherits_prism_meta_glass() {
    let content = read_progression_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_progression_inherits_song_family_root() {
    let content = read_progression_shard();
    assert!(
        content.contains("in @song"),
        "T5: species must declare `in @song` — inherits family-root's `song_progression`/`song_cadence` carriers + `song_settles` composed bilateral"
    );
}

// === T6-T7: #S1 grounding — eigenform at temporal altitude ===

#[test]
fn t06_progression_grounds_s1_eigenform_at_temporal_altitude() {
    let content = read_progression_shard();
    // Species-altitude witness of #S1 candidate. Any of: recognition-tag,
    // "eigenform" mention at temporal altitude, or Kauffman/Foerster prior-art anchor.
    let has_s1_grounding = content.contains("#S1")
        || content.contains("eigenform")
        || content.contains("Kauffman")
        || content.contains("Foerster")
        || content.contains("tonic-return")
        || content.contains("tonic return");
    assert!(
        has_s1_grounding,
        "T6: species narrative must ground recognition #S1 (song-IS-eigenform-at-temporal-altitude) — cite `#S1`, `eigenform`, Kauffman/Foerster ancestor, or `tonic-return` recursion witness"
    );
}

#[test]
fn t07_progression_discharges_directed_toward_cadence_sub_predicate() {
    let content = read_progression_shard();
    assert!(
        content.contains("progression_directed_toward_cadence"),
        "T7: species must declare `progression_directed_toward_cadence(pr, p) -> verdict` — the first sub-predicate of `song_settles` composed bilateral (14th #53 instance per family-root §3)"
    );
}

// === T8-T9: #S2 grounding — shift at temporal altitude ===

#[test]
fn t08_progression_grounds_s2_shift_at_temporal_altitude() {
    let content = read_progression_shard();
    // Species-altitude witness of #S2 candidate: song IS `@kintsugi/shift` at temporal
    // altitude when heard sequentially. Any of: recognition-tag, finite-difference framing,
    // or explicit shift-at-temporal narrative.
    let has_s2_grounding = content.contains("#S2")
        || content.contains("finite-difference")
        || content.contains("finite difference")
        || content.contains("shift @ temporal")
        || content.contains("shift-at-temporal")
        || content.contains("Delta");
    assert!(
        has_s2_grounding,
        "T8: species narrative must ground recognition #S2 (song-IS-shift-at-temporal-altitude) — cite `#S2`, finite-difference framing `Delta pr(t) = shift(...)`, or `shift @ temporal` altitude witness"
    );
}

#[test]
fn t09_progression_composes_with_kintsugi_shift() {
    let content = read_progression_shard();
    assert!(
        content.contains("@kintsugi/shift"),
        "T9: species must cite `@kintsugi/shift` (Arc 5 TICK 2 landing `49f0486`) — the substrate-decl'd cross-altitude morphism that species `advance` action delegates to at temporal altitude (grounds #S2 witness)"
    );
}

// === T10-T12: music binding — cadence Lawvere + music family + authentic-or-plagal ===

#[test]
fn t10_progression_composes_with_epistemologic_math_music_cadence() {
    let content = read_progression_shard();
    assert!(
        content.contains("@epistemologic/math/music/cadence")
            || content.contains("cadence.mirror"),
        "T10: species must cite `@epistemologic/math/music/cadence` (Lawvere framing per Seam Correction `3d910bc`) — the `settle @ temporal` audible-altitude ancestor"
    );
}

#[test]
fn t11_progression_discharges_cadence_authentic_or_plagal_sub_predicate() {
    let content = read_progression_shard();
    assert!(
        content.contains("cadence_authentic_or_plagal"),
        "T11: species must declare `cadence_authentic_or_plagal(c, p) -> verdict` — the second sub-predicate of `song_settles` composed bilateral (per family-root §3; FAIL = EXTRACTION Narcissus-pole)"
    );
}

#[test]
fn t12_progression_composes_with_epistemologic_math_music_family() {
    let content = read_progression_shard();
    assert!(
        content.contains("@epistemologic/math/music"),
        "T12: species must cite `@epistemologic/math/music` (audible-altitude family-root — harmonic/interval/dissonance/cadence) — general music binding beyond just cadence.mirror"
    );
}

// === T13-T14: prior-art anchors — Schenker + cadence-family vocabulary ===

#[test]
fn t13_progression_cites_schenker_for_project_at_temporal_ancestor() {
    let content = read_progression_shard();
    assert!(
        content.contains("Schenker"),
        "T13: species narrative must cite Schenker — Schenkerian reduction (foreground/middleground/background) IS the `project @ temporal` prior-art ancestor per Seam §5 + spec §3; recursive-descent through progression altitudes is direct precedent for species-altitude progression discipline"
    );
}

#[test]
fn t14_progression_cites_cadence_family_vocabulary() {
    let content = read_progression_shard();
    // Cadence family: authentic (Splinter-pole) / plagal (Splinter-pole variant) /
    // deceptive (Narcissus-pole = EXTRACTION) / half (Narcissus-pole = SILENCE stuck).
    let has_cadence_family = content.contains("authentic")
        && (content.contains("deceptive") || content.contains("half-cadence") || content.contains("half cadence"));
    assert!(
        has_cadence_family,
        "T14: species narrative must cite cadence-family vocabulary — `authentic` (Splinter-pole discharge) AND at least one of `deceptive`/`half-cadence` (Narcissus-pole variant naming psychohistorical failure mode: EXTRACTION or SILENCE per spec §8.2)"
    );
}

// === T15: #53 bilateral pattern discharge + species-altitude discipline ===

#[test]
fn t15_progression_carries_bilateral_pattern_at_species_altitude() {
    let content = read_progression_shard();
    // Species discharges the sub-predicates that the family-root's `song_settles`
    // composed bilateral consumes. The species narrative must acknowledge either
    // the #53 bilateral pattern OR the composed-bilateral discharge relationship OR
    // the psychohistory-Narcissus-pole naming (EXTRACTION / SILENCE) that alignment-
    // as-boundary-mathematics (#57) makes visible.
    let has_bilateral_grounding = content.contains("#53")
        || content.contains("song_settles")
        || content.contains("composed bilateral")
        || content.contains("composed-bilateral")
        || content.contains("EXTRACTION")
        || content.contains("SILENCE")
        || content.contains("Splinter-pole")
        || content.contains("Narcissus-pole");
    assert!(
        has_bilateral_grounding,
        "T15: species narrative must ground the sub-predicate discharge relationship into `song_settles` — cite `#53`, `song_settles`, `composed bilateral`, or the psychohistory-Narcissus-pole naming (`EXTRACTION`/`SILENCE`/`Splinter-pole`/`Narcissus-pole`) that alignment-as-boundary-mathematics discipline requires per family-root §3"
    );
}
