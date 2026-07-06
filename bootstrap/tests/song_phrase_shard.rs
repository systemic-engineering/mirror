//! Arc 6 TICK 6 RED — `shards/song/phrase.mirror` species. **Arc 6 CLOSING TICK.**
//!
//! Per Seam pre-review at `3d910bc` §6.7: **TICK 6 lands `shards/song/phrase.mirror`
//! — atomic unit; OBC binding closure.**
//!
//! Fifth `@song` species (after progression `54ff1e8`, voice `cc5a440`, movement
//! `4efbf16`, narrative `0434a39`). Per family-root `shards/song.mirror` species-
//! roster:
//!
//! > `@song/phrase` — the atomic unit — bounded, self-contained, composable;
//! > OBC binding closure at temporal altitude (a phrase IS the substrate-decl of
//! > one OBC-bounded interaction's ambiguity-load budget). Actions:
//! > `song_phrase(ph: ref) -> ref` with `join`/`split` at species altitude.
//!
//! **OBC binding** (Alex's `systemic.engineering` corpus):
//! - OBC = One Boundary Condition per `blog/pieces/3published/Piece - Constraints
//!   (OBC).md`.
//! - A phrase carries ONE OBC's ambiguity-load budget at temporal altitude.
//! - Splinter-pole: phrase discharges within budget; interaction closes on-
//!   boundary. Narcissus-pole: interaction exceeds budget; phrase runs into
//!   ADO-refusal / forced-progression / boundary violation.
//!
//! **Species actions**: `join` (compose two phrases) + `split` (decompose a
//! phrase into finer phrases).
//!
//! **#S5 promotion decision** (per TICK 5 audit `750cb19` §3.3): if phrase is
//! consolidation-shape (no new sub-predicates), #S5 (consolidation-vs-
//! decomposition species pattern) promotes CANDIDATE → LANDED at Arc 6 close.
//! If phrase is decomposition-shape (adds a species-altitude admissibility
//! gate), #S5 stays CANDIDATE.
//!
//! **Recognition #S3 LANDED** at `10c34cf`; phrase inherits + consolidates.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_phrase_shard() -> String {
    let path = repo_root().join("shards/song/phrase.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/song/phrase.mirror at {:?}: {}", path, e))
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
fn t01_song_phrase_shard_file_exists_and_declares_species() {
    let content = read_phrase_shard();
    assert!(
        content.contains("prism @song/phrase") || content.contains("prism @song / phrase"),
        "T1: shards/song/phrase.mirror must declare `prism @song/phrase` species per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_phrase_shard();
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
    let content = read_phrase_shard();
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
fn t04_phrase_inherits_prism_meta_glass() {
    let content = read_phrase_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_phrase_inherits_song_family_root() {
    let content = read_phrase_shard();
    assert!(
        content.contains("in @song"),
        "T5: species must declare `in @song` — inherits family-root's carriers + #S3 LANDED five-op temporal specialisation"
    );
}

// === T6-T7: species actions (join/split per family-root roster) ===

#[test]
fn t06_phrase_declares_join_action() {
    let content = read_phrase_shard();
    let has_join = content.contains("join(ph") || content.contains("join(phrase");
    assert!(
        has_join,
        "T6: species MUST declare `join(ph: song_phrase, ...)` or `join(phrase: ..., ...)` action. Per family-root song.mirror species-roster: phrase actions are `join` and `split` at species altitude. Join composes two phrases into one."
    );
}

#[test]
fn t07_phrase_declares_split_action() {
    let content = read_phrase_shard();
    let has_split = content.contains("split(ph") || content.contains("split(phrase");
    assert!(
        has_split,
        "T7: species MUST declare `split(ph: song_phrase, ...)` or `split(phrase: ..., ...)` action. Split decomposes a phrase into finer phrases; specialises `split @ temporal` per family-root prism block."
    );
}

// === T8: OBC binding (Piece - Constraints per SE corpus) ===

#[test]
fn t08_phrase_cites_obc_binding() {
    let content = read_phrase_shard();
    let has_obc = content.contains("OBC")
        || content.contains("One Boundary Condition")
        || content.contains("one-boundary-condition")
        || content.contains("one boundary condition")
        || content.contains("Piece - Constraints");
    assert!(
        has_obc,
        "T8: species narrative MUST cite OBC binding — `OBC`, `One Boundary Condition`, or `Piece - Constraints`. Per family-root song.mirror species-roster + spec §8: phrase IS the substrate-decl of one OBC-bounded interaction's ambiguity-load budget."
    );
}

// === T9: ambiguity-load budget vocabulary ===

#[test]
fn t09_phrase_cites_ambiguity_load_vocabulary() {
    let content = read_phrase_shard();
    let has_load_budget = content.contains("ambiguity-load")
        || content.contains("ambiguity load")
        || content.contains("load budget")
        || content.contains("load-budget")
        || content.contains("budget");
    assert!(
        has_load_budget,
        "T9: species narrative MUST cite ambiguity-load budget vocabulary — `ambiguity-load`, `load budget`, or `budget`. Per family-root: `a phrase IS the substrate-decl of one OBC-bounded interaction's ambiguity-load budget`."
    );
}

// === T10: bounded / self-contained / composable ===

#[test]
fn t10_phrase_narrative_grounds_atomic_unit_discipline() {
    let content = read_phrase_shard();
    let has_atomic_discipline = (content.contains("bounded") || content.contains("self-contained"))
        && (content.contains("composable") || content.contains("atomic"));
    assert!(
        has_atomic_discipline,
        "T10: species narrative MUST ground the atomic-unit discipline — cite `bounded` + `composable` (or `self-contained` + `atomic`). Per family-root: phrase IS `the atomic unit — bounded, self-contained, composable`."
    );
}

// === T11: musical-phrase prior-art anchor ===

#[test]
fn t11_phrase_cites_musical_phrase_prior_art() {
    let content = read_phrase_shard();
    let has_musical_anchor = content.contains("Riemann")
        || content.contains("Schoenberg")
        || content.contains("Koch")
        || content.contains("phrasing")
        || content.contains("period")
        || content.contains("antecedent")
        || content.contains("consequent")
        || content.contains("Grundgestalt");
    assert!(
        has_musical_anchor,
        "T11: species narrative MUST cite musical-phrase prior-art — Riemann (phrasing / period / motive) or Schoenberg (Grundgestalt) or Koch (Versuch) or general phrasing/period/antecedent/consequent vocabulary. Classical-music phrase discipline traces to 18-19th c. theorists."
    );
}

// === T12: sibling species awareness (Arc 6 species landscape) ===

#[test]
fn t12_phrase_acknowledges_sibling_species() {
    let content = read_phrase_shard();
    // Arc 6 sibling species: progression, voice, movement, narrative.
    // Phrase should acknowledge at least 2 siblings in narrative context.
    let siblings: Vec<&str> = [
        "@song/progression",
        "@song/voice",
        "@song/movement",
        "@song/narrative",
    ]
    .iter()
    .filter(|s| content.contains(*s))
    .copied()
    .collect();
    assert!(
        siblings.len() >= 2,
        "T12: species narrative MUST acknowledge at least 2 sibling @song species (@song/progression / @song/voice / @song/movement / @song/narrative). Found: {:?}. Arc 6 is closing; phrase completes the family; the species landscape should be visible.",
        siblings
    );
}

// === T13: composed-bilateral closure acknowledgement (either decomposition OR consolidation shape) ===

#[test]
fn t13_phrase_acknowledges_song_settles_and_species_pattern() {
    let content = read_phrase_shard();
    let has_pattern_ack = content.contains("song_settles")
        || content.contains("composed bilateral")
        || content.contains("composed-bilateral")
        || content.contains("decomposition-species")
        || content.contains("consolidation-species")
        || content.contains("consolidation")
        || content.contains("decomposition")
        || content.contains("#53");
    assert!(
        has_pattern_ack,
        "T13: species narrative MUST acknowledge the composed-bilateral discipline OR the consolidation-vs-decomposition species pattern (#S5 CANDIDATE per Seam Phase D TICK 5 `750cb19`). Cite `song_settles`, `composed bilateral`, `consolidation`, `decomposition`, or `#53`."
    );
}

// === T14: #S3 LANDED marker ===

#[test]
fn t14_phrase_acknowledges_s3_landed() {
    let content = read_phrase_shard();
    let has_s3 = content.contains("#S3")
        || content.contains("five-op temporal")
        || content.contains("five-operation temporal")
        || content.contains("five operations");
    assert!(
        has_s3,
        "T14: species narrative MUST cite `#S3` (LANDED at `10c34cf`) or five-op temporal specialisation. Phrase inherits the five-op prism block from family-root."
    );
}

// === T15: Arc 6 closing marker ===

#[test]
fn t15_phrase_marks_arc_6_close() {
    let content = read_phrase_shard();
    let has_arc_6_close = content.contains("Arc 6")
        || content.contains("arc 6")
        || content.contains("TICK 6")
        || content.contains("tick 6")
        || content.contains("family close")
        || content.contains("closes @song")
        || content.contains("completes @song")
        || content.contains("final species")
        || content.contains("fifth species")
        || content.contains("closing tick");
    assert!(
        has_arc_6_close,
        "T15: species narrative MUST mark this as Arc 6 closing tick — cite `Arc 6`, `TICK 6`, `closes @song`, `completes @song`, `final species`, `fifth species`, or `closing tick`. Phrase completes the @song species roster (progression + voice + movement + narrative + phrase)."
    );
}
