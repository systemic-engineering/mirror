//! Arc 3 TICK 2 RED — `@epistemologic/liquid_extraction` sibling family-root.
//!
//! Per Seam Phase D pre-review at `43aaadd` §5 canonical execution: TICK 2
//! follows TICK 1 (`5c0f5ba` docblock.mirror RATIFIED at `820a451`) as the
//! sibling family-root at the logical altitude. Interpretation B canonical shape
//! (narrative above `---`; `in`+declarations below) enforced.
//!
//! T1-T8 canonical per Mara's spec `docs/specs/doc-code-seam-shards.md` §2
//! RED test targets. T9-T14 Interpretation B structural discipline mirroring
//! TICK 1 T15-T20.
//!
//! RED phase: `shards/epistemologic/liquid_extraction.mirror` does not yet
//! exist. Text-check tests fail on file absence. Pattern mirrors
//! `bootstrap/tests/docblock_shard.rs` (Reed `18db8b7`).
//!
//! Closes: task #535 sub-arc A TICK 2 entry point. Mara 🟢 dispatched post-RED.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_liquid_extraction_shard() -> String {
    let path = repo_root().join("shards/epistemologic/liquid_extraction.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/epistemologic/liquid_extraction.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T8: canonical shape per Mara spec §2 RED test targets ===

#[test]
fn t01_liquid_extraction_declares_prism() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("prism @epistemologic/liquid_extraction"),
        "T1: liquid_extraction.mirror must declare `prism @epistemologic/liquid_extraction` per Mara spec §2"
    );
}

#[test]
fn t02_liquid_extraction_declares_extractor_input() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("type extractor_input = doc_claim"),
        "T2: liquid_extraction.mirror must declare `type extractor_input = doc_claim` per Mara spec §2"
    );
}

#[test]
fn t03_liquid_extraction_declares_predicate_shape() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("type predicate_shape = ref"),
        "T3: liquid_extraction.mirror must declare `type predicate_shape = ref` per Mara spec §2"
    );
}

#[test]
fn t04_liquid_extraction_declares_verdict_variants() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("type extraction_verdict"),
        "T4: liquid_extraction.mirror must declare `type extraction_verdict` per Mara spec §2"
    );
    for variant in ["satisfiable", "unsatisfiable", "partial", "unextractable"] {
        assert!(
            content.contains(variant),
            "T4: extraction_verdict must include variant `{}` per Mara spec §2",
            variant
        );
    }
}

#[test]
fn t05_liquid_extraction_declares_extract_predicate_action() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("extract_predicate(i: extractor_input) -> ref"),
        "T5: liquid_extraction.mirror must declare `extract_predicate(i: extractor_input) -> ref` per Mara spec §2"
    );
}

#[test]
fn t06_liquid_extraction_declares_bilateral_predicate() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("liquid_extraction_sound(i: extractor_input, v: extraction_verdict) -> verdict"),
        "T6: liquid_extraction.mirror must declare `liquid_extraction_sound(i: extractor_input, v: extraction_verdict) -> verdict` per Mara spec §2"
    );
}

#[test]
fn t07_liquid_extraction_inherits_epistemologic() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("in @epistemologic"),
        "T7: liquid_extraction.mirror must inherit `in @epistemologic` per Mara spec §2"
    );
}

#[test]
fn t08_liquid_extraction_inherits_docblock() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("in @docblock"),
        "T8: liquid_extraction.mirror must inherit `in @docblock` per Mara spec §2"
    );
}

// === T9-T14: Interpretation B structural discipline (mirrors TICK 1 T15-T20) ===

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

#[test]
fn t09_first_nonempty_line_is_narrative_docblock() {
    let content = read_liquid_extraction_shard();
    let first = first_nonempty_line(&content)
        .expect("T9: liquid_extraction.mirror must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T9: first non-empty line must be `#`-narrative per Interpretation B; got `{}`. Line-1 `---` is drift.",
        first
    );
    assert_ne!(first.trim(), "---", "T9: line-1 `---` marker is Interpretation A drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T9: `in @...` clauses live BELOW the seam per Interpretation B"
    );
}

#[test]
fn t10_exactly_one_seam_line_at_column_zero() {
    let content = read_liquid_extraction_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T10: exactly one `---` at column 0 required (Mara spec §6.1 stateful predicate); found {}",
        seams.len()
    );
}

#[test]
fn t11_in_clauses_below_seam() {
    let content = read_liquid_extraction_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T11 depends on T10 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T11: `in @...` at line {} appears ABOVE seam at line {}. Interpretation B places imports BELOW. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t12_narrative_names_sibling_relationship() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("sibling") || content.contains("@docblock"),
        "T12: narrative must position this shard as sibling family-root to @docblock at logical altitude per Mara spec §2"
    );
}

#[test]
fn t13_narrative_names_logical_altitude() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("logical"),
        "T13: narrative must name `logical` altitude per Mara §3 four-altitude structure (linguistic/logical/temporal/publishable)"
    );
}

#[test]
fn t14_narrative_grounds_bilateral_pattern() {
    let content = read_liquid_extraction_shard();
    assert!(
        content.contains("#53") || content.contains("bilateral") || content.contains("property"),
        "T14: narrative must ground the property/fracture bilateral pattern per #53 (Mara spec §2 references sixth-instance bilateral in §3)"
    );
}
