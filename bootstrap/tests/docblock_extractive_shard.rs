//! Arc 3 TICK 8 RED — `@kintsugi/fracture/docblock_extractive`.
//!
//! Operational half of TICK 7 (`docblock_no_extraction_pattern` at `c56c3c8`).
//! Routes to THREE surface classes based on which sub-check fires per Mara §8:
//!   - Sub-check 1 (no_unmarked_superlatives)  → `ashby_mismatch` (§3.1)
//!   - Sub-check 3 (citations_content_match)    → `out_of_band` (§3.4)
//!   - Sub-check 4 (claim_size_matches_landing_size) → `contradiction` (§3.2)
//!   - Sub-check 2 (hedged_claims_marked_defer) → kind-reclassification via
//!     `@epistemologic/liquid_extraction` (deterministic; no reader-frame surfacing)
//!
//! Per Mara spec `docs/specs/doc-code-seam-shards.md` §8 RED test targets (4
//! canonical) + Interpretation B discipline (5 mirroring prior fracture ticks).
//!
//! **THIS TICK CLOSES SUB-ARC A.** All three #53 bilateral trio pairs complete
//! after Mara 🟢 + Seam 📝 RATIFY.
//!
//! Post TICK 7 close (Reed 🔴 `8246ad4` + Mara 🟢 `c56c3c8` + Seam 📝 `1f8df3f`).
//!
//! RED phase: `shards/kintsugi/fracture/docblock_extractive.mirror` does not
//! yet exist. Text-check tests fail on file absence.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_extractive_shard() -> String {
    let path = repo_root().join("shards/kintsugi/fracture/docblock_extractive.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/kintsugi/fracture/docblock_extractive.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T4: canonical shape per Mara spec §8 RED test targets ===

#[test]
fn t01_docblock_extractive_declares_glass() {
    let content = read_docblock_extractive_shard();
    assert!(
        content.contains("glass @kintsugi/fracture/docblock_extractive"),
        "T1: must declare `glass @kintsugi/fracture/docblock_extractive` per Mara spec §8"
    );
}

#[test]
fn t02_docblock_extractive_declares_body() {
    let content = read_docblock_extractive_shard();
    assert!(
        content.contains("docblock_extractive_body(d: docblock, ctx: kintsugi_context)"),
        "T2: must declare `docblock_extractive_body(d: docblock, ctx: kintsugi_context)` per Mara spec §8"
    );
}

#[test]
fn t03_docblock_extractive_requires_ashby_variety_match() {
    let content = read_docblock_extractive_shard();
    assert!(
        content.contains("requires ashby_variety_match(kintsugi_lock)"),
        "T3: body must carry `requires ashby_variety_match(kintsugi_lock)` per Mara spec §8"
    );
}

#[test]
fn t04_docblock_extractive_names_three_routing_classes() {
    let content = read_docblock_extractive_shard();
    for class in ["ashby_mismatch", "out_of_band", "contradiction"] {
        assert!(
            content.contains(class),
            "T4: docblock must list routing class `{}` explicitly per Mara spec §8",
            class
        );
    }
}

// === T5-T9: Interpretation B structural discipline ===

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
fn t05_first_nonempty_line_is_narrative_docblock() {
    let content = read_docblock_extractive_shard();
    let first = first_nonempty_line(&content).expect("T5: must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T5: first non-empty line must be `#`-narrative; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T5: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T5: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t06_exactly_one_seam_line_at_column_zero() {
    let content = read_docblock_extractive_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T6: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
}

#[test]
fn t07_in_clauses_below_seam() {
    let content = read_docblock_extractive_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T7 depends on T6 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T7: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t08_narrative_names_operational_pair() {
    let content = read_docblock_extractive_shard();
    assert!(
        content.contains("docblock_no_extraction_pattern"),
        "T8: narrative must name declarative pair `docblock_no_extraction_pattern` (TICK 7)"
    );
}

#[test]
fn t09_narrative_names_liquid_extraction_reclassification() {
    let content = read_docblock_extractive_shard();
    assert!(
        content.contains("liquid_extraction") || content.contains("kind-reclassif") || content.contains("kind_reclassif"),
        "T9: narrative must name sub-check-2 kind-reclassification path via @epistemologic/liquid_extraction per Mara spec §8"
    );
}
