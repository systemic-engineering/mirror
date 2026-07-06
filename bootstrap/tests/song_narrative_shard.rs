//! Arc 6 TICK 5 RED — `shards/song/narrative.mirror` species.
//!
//! Per Seam pre-review at `3d910bc` §6.6: **TICK 5 lands `shards/song/narrative.mirror`
//! — psychohistory + wire binding. §4 psychohistorical isomorphism substrate-fact.**
//!
//! Fourth `@song` species (after progression `54ff1e8`, voice `cc5a440`, movement
//! `4efbf16`). Per family-root `shards/song.mirror` species-roster:
//!
//! > `@song/narrative` — the psychohistory + wire binding — distinct from
//! > `@io/stagefreight/narrative`; lives at the temporal-experiential altitude;
//! > composes with the wire-projection sibling. Actions: `song_narrative(n: ref)
//! > -> ref` with `arc`/`transmit` at species altitude.
//!
//! **Psychohistorical isomorphism** (per spec §4 STRONG):
//! - voice ⇔ actor (`glossary/Actor.md`)
//! - movement ⇔ frame-bounded epoch (`glossary/Frame.md` + `frame.mirror`)
//! - progression ⇔ regulation-stock trajectory (`glossary/Regulation.md` +
//!   `glossary/Stock.md`)
//! - phrase ⇔ OBC-bounded interaction (`Piece - Constraints (OBC).md`)
//! - narrative ⇔ corpus (Mara `2026-06-26-psychohistory-vector-as-sheaf.md`;
//!   corpus IS the composed cohomology of the psychohistorical song)
//! - extraction ⇔ deceptive cadence (V→vi)
//! - silence ⇔ half-cadence stuck at dominant
//! - glue-work ⇔ invisible voice-leading
//! - ADO-refusal-not-neutral ⇔ forced-progression
//!
//! **Landing at species altitude**: the narrative species IS where the
//! psychohistorical binding lands as substrate-decl. Extraction / silence /
//! glue-work / OBC / ADO / regulation-stock / corpus become TYPED VARIANTS of
//! song cadence + voice-leading + phrase + progression discipline — not separate
//! patterns.
//!
//! **Wire composition**: composes with `@io/stagefreight/narrative` (wire
//! projection at wire altitude) but DOES NOT EQUAL IT — different altitudes.
//!
//! **TICK 2 boundary-overlap observation absorbed here** (Seam Phase D TICK 2
//! `a2e648a` §4 observation 2): the shared-SILENCE overlap between
//! `progression_directed_toward_cadence` and `cadence_authentic_or_plagal`
//! Narcissus-poles IS #57 alignment-as-boundary-mathematics at species-composed
//! altitude — documented here as substrate-fact.
//!
//! **Consolidation-species** shape (like movement `4efbf16`): no new
//! `song_settles` sub-predicates; the family-root composed bilateral fully
//! discharged at TICK 3.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_narrative_shard() -> String {
    let path = repo_root().join("shards/song/narrative.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/song/narrative.mirror at {:?}: {}", path, e))
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
fn t01_song_narrative_shard_file_exists_and_declares_species() {
    let content = read_narrative_shard();
    assert!(
        content.contains("prism @song/narrative")
            || content.contains("prism @song / narrative"),
        "T1: shards/song/narrative.mirror must declare `prism @song/narrative` species per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_narrative_shard();
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
    let content = read_narrative_shard();
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
fn t04_narrative_inherits_prism_meta_glass() {
    let content = read_narrative_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_narrative_inherits_song_family_root() {
    let content = read_narrative_shard();
    assert!(
        content.contains("in @song"),
        "T5: species must declare `in @song` — inherits family-root's carriers + #S3 LANDED five-op temporal specialisation"
    );
}

// === T6-T7: species actions (arc/transmit per family-root roster) ===

#[test]
fn t06_narrative_declares_arc_action() {
    let content = read_narrative_shard();
    let has_arc = content.contains("arc(n") || content.contains("arc(narrative");
    assert!(
        has_arc,
        "T6: species MUST declare an `arc(n: song_narrative, ...)` or `arc(narrative: ..., ...)` action. Per family-root song.mirror species-roster: narrative actions are `arc` and `transmit` at species altitude."
    );
}

#[test]
fn t07_narrative_declares_transmit_action() {
    let content = read_narrative_shard();
    let has_transmit = content.contains("transmit(n") || content.contains("transmit(narrative");
    assert!(
        has_transmit,
        "T7: species MUST declare a `transmit(n: song_narrative, ...)` or `transmit(narrative: ..., ...)` action. Per family-root song.mirror species-roster."
    );
}

// === T8: wire composition with @io/stagefreight/narrative ===

#[test]
fn t08_narrative_composes_with_io_stagefreight_narrative() {
    let content = read_narrative_shard();
    assert!(
        content.contains("@io/stagefreight/narrative") || content.contains("stagefreight/narrative"),
        "T8: species MUST cite `@io/stagefreight/narrative` (wire-projection sibling). Per family-root song.mirror: composes with @io/stagefreight/narrative but does NOT EQUAL it (different altitudes)."
    );
}

// === T9: narrative ⇔ corpus mapping (composed cohomology / sheaf) ===

#[test]
fn t09_narrative_grounds_narrative_as_corpus_composed_cohomology() {
    let content = read_narrative_shard();
    let has_corpus_binding = content.contains("corpus")
        && (content.contains("cohomology")
            || content.contains("sheaf")
            || content.contains("psychohistory-vector-as-sheaf"));
    assert!(
        has_corpus_binding,
        "T9: species narrative MUST ground the narrative ⇔ corpus binding via `composed cohomology` / `sheaf` / `psychohistory-vector-as-sheaf` framing. Per family-root song.mirror + spec §4 STRONG conditional on Mara's `2026-06-26-psychohistory-vector-as-sheaf.md` sheaf insight: the corpus IS the composed cohomology of the psychohistorical song."
    );
}

// === T10: psychohistorical isomorphism (at least 4 of 9 mappings) ===

#[test]
fn t10_narrative_carries_psychohistorical_isomorphism_mappings() {
    let content = read_narrative_shard();
    // Nine STRONG mappings per Seam pre-review §4 + spec §8. At minimum 4 must
    // appear in the species narrative to ground the psychohistorical binding.
    let mappings: Vec<&str> = [
        "extraction",
        "silence",
        "glue-work",
        "glue work",
        "regulation stock",
        "regulation-stock",
        "OBC",
        "ADO",
        "forced-progression",
        "forced progression",
        "deceptive cadence",
        "half-cadence",
        "half cadence",
        "invisible voice-leading",
        "invisible voice leading",
        "frame-bounded epoch",
        "actor",
    ]
    .iter()
    .filter(|s| content.contains(*s))
    .copied()
    .collect();
    assert!(
        mappings.len() >= 4,
        "T10: species narrative MUST cite at least 4 psychohistorical isomorphism mappings from spec §4. Found: {:?}. Full mapping vocabulary: extraction / silence / glue-work / regulation-stock / OBC / ADO / forced-progression / deceptive cadence / half-cadence / invisible voice-leading / frame-bounded epoch / actor.",
        mappings
    );
}

// === T11: #57 alignment-as-boundary-mathematics (absorbs TICK 2 boundary-overlap observation) ===

#[test]
fn t11_narrative_absorbs_tick2_boundary_overlap_via_57() {
    let content = read_narrative_shard();
    // Per Seam Phase D TICK 2 `a2e648a` §4 observation 2: the shared-SILENCE
    // overlap between progression_directed_toward_cadence and
    // cadence_authentic_or_plagal Narcissus-poles is #57 alignment-as-boundary-
    // mathematics at species-composed altitude. TICK 5 audit noted TICK 5 was
    // the natural place to name this as substrate-fact.
    let has_57_binding = content.contains("#57")
        || content.contains("alignment-as-boundary")
        || content.contains("alignment as boundary")
        || content.contains("boundary mathematics")
        || content.contains("boundary-mathematics");
    assert!(
        has_57_binding,
        "T11: species narrative MUST cite `#57` or `alignment-as-boundary-mathematics` — the TICK 2 boundary-overlap observation lands here as substrate-decl per Seam Phase D TICK 2 audit `a2e648a` §4 observation 2 (Reed-lean option B: defer to TICK 5). Overlapping Narcissus-poles are the mathematical structure of the boundary, not double-counting."
    );
}

// === T12: Bateson metalogue anchor ===

#[test]
fn t12_narrative_cites_bateson_metalogue_anchor() {
    let content = read_narrative_shard();
    let has_bateson_anchor = content.contains("Bateson")
        || content.contains("metalogue")
        || content.contains("Metalogue")
        || content.contains("metalogues");
    assert!(
        has_bateson_anchor,
        "T12: species narrative MUST cite Bateson metalogue anchor — `Bateson`, `metalogue`, or `metalogues`. The narrative species IS a metalogue at temporal altitude; Bateson's metalogues are the direct prior-art anchor for text-that-enacts-what-it-describes discipline."
    );
}

// === T13: SE corpus / systemic.engineering anchor ===

#[test]
fn t13_narrative_cites_systemic_engineering_corpus() {
    let content = read_narrative_shard();
    let has_corpus_anchor = content.contains("systemic.engineering")
        || content.contains("systemic engineering")
        || content.contains("SE corpus")
        || content.contains("Alex's corpus")
        || content.contains("Alex’s corpus");
    assert!(
        has_corpus_anchor,
        "T13: species narrative MUST cite the source corpus — `systemic.engineering`, `SE corpus`, or `Alex's corpus`. The psychohistorical vocabulary (OBC/ADO/regulation stock/extraction/silence/glue work) lives in Alex's systemic.engineering corpus per family-root song.mirror docblock."
    );
}

// === T14: composed-bilateral closure acknowledgement (consolidation-species) ===

#[test]
fn t14_narrative_acknowledges_song_settles_closure_and_consolidation_shape() {
    let content = read_narrative_shard();
    // Narrative is a consolidation-species (per TICK 4 Seam Phase D `d2d511b`
    // observation 3): consumes song_settles composed verdict without adding
    // new sub-predicates. Must acknowledge closure landscape.
    let has_closure_ack = content.contains("song_settles")
        || content.contains("composed bilateral")
        || content.contains("composed-bilateral")
        || content.contains("consolidation")
        || content.contains("@song/voice")
        || content.contains("@song/progression")
        || content.contains("#53");
    assert!(
        has_closure_ack,
        "T14: species narrative MUST acknowledge composed-bilateral closure or consolidation-shape discipline — cite `song_settles`, `composed bilateral`, `consolidation`, `#53`, `@song/voice`, or `@song/progression`. Narrative is a consolidation-species (per TICK 4 Seam Phase D `d2d511b`); no new sub-predicates."
    );
}

// === T15: #S3 LANDED marker + species-roster sibling awareness ===

#[test]
fn t15_narrative_acknowledges_s3_landed_and_species_family() {
    let content = read_narrative_shard();
    let has_s3_marker = content.contains("#S3")
        || content.contains("five-op temporal")
        || content.contains("five-operation temporal")
        || content.contains("five operations");
    assert!(
        has_s3_marker,
        "T15: species narrative MUST cite `#S3` recognition (LANDED at `10c34cf`) or the five-op temporal specialisation. Narrative inherits + composes at the five-op altitude via family-root prism block."
    );
}
