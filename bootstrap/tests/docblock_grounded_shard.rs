//! Arc 3 TICK 3 RED — `@epistemologic/property/docblock_grounded`.
//!
//! Sixth #53 property/fracture bilateral instance (first-of-trio). Declarative
//! half of the docblock grounding audit. Operational half
//! (`@kintsugi/fracture/docblock_ungrounded`) lands at TICK 4.
//!
//! Per Mara's spec `docs/specs/doc-code-seam-shards.md` §3 RED test targets (4
//! canonical) + Interpretation B discipline (6 mirroring TICK 1 T15-T20).
//!
//! Post TICK 1 (`5c0f5ba` docblock.mirror + `820a451` RATIFY) + TICK 2
//! (`32a1e2a` liquid_extraction.mirror + `3283de4` RATIFY).
//!
//! RED phase: `shards/epistemologic/property/docblock_grounded.mirror` does not
//! yet exist. Text-check tests fail on file absence.
//!
//! Closes: task #535 sub-arc A TICK 3 entry point. Mara 🟢 dispatched post-RED.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_grounded_shard() -> String {
    let path = repo_root().join("shards/epistemologic/property/docblock_grounded.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/epistemologic/property/docblock_grounded.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T4: canonical shape per Mara spec §3 RED test targets ===

#[test]
fn t01_docblock_grounded_declares_prism() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("prism @epistemologic/property/docblock_grounded"),
        "T1: must declare `prism @epistemologic/property/docblock_grounded` per Mara spec §3"
    );
}

#[test]
fn t02_docblock_grounded_declares_predicate() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("docblock_grounded(d: docblock) -> verdict"),
        "T2: must declare `docblock_grounded(d: docblock) -> verdict` per Mara spec §3"
    );
}

#[test]
fn t03_docblock_grounded_inherits_docblock() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("in @docblock"),
        "T3: must inherit `in @docblock` per Mara spec §3"
    );
}

#[test]
fn t04_docblock_grounded_inherits_property_family() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("in @epistemologic/property"),
        "T4: must inherit `in @epistemologic/property` per Mara spec §3"
    );
}

// === T5-T10: Interpretation B structural discipline ===

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
    let content = read_docblock_grounded_shard();
    let first = first_nonempty_line(&content)
        .expect("T5: must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T5: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T5: line-1 `---` is Interpretation A drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T5: `in @...` clauses live BELOW the seam per Interpretation B"
    );
}

#[test]
fn t06_exactly_one_seam_line_at_column_zero() {
    let content = read_docblock_grounded_shard();
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
    let content = read_docblock_grounded_shard();
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
fn t08_narrative_grounds_bilateral_sixth_instance() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("#53") && (content.contains("sixth") || content.contains("first")),
        "T8: narrative must ground #53 bilateral pattern sixth-instance first-of-trio per Mara spec §3"
    );
}

#[test]
fn t09_narrative_names_operational_pair() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("docblock_ungrounded") || content.contains("fracture"),
        "T9: narrative must name operational half (`@kintsugi/fracture/docblock_ungrounded`) per #53 bilateral discipline"
    );
}

#[test]
fn t10_narrative_grounds_predicate_substance() {
    let content = read_docblock_grounded_shard();
    assert!(
        content.contains("extract_claims") || content.contains("ancestor_exists_on_main") || content.contains("citation"),
        "T10: narrative must ground predicate substance per Mara spec §3 (extract_claims / citation / ancestor_exists_on_main)"
    );
}
