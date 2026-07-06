//! Arc 3 TICK 4 RED — `@kintsugi/fracture/docblock_ungrounded`.
//!
//! Operational half of TICK 3 (`docblock_grounded` at `98664a7`). Routes via
//! `@kintsugi/surface`'s `ashby_mismatch` class per compiler-error-surface §3.1.
//!
//! Per Mara's spec `docs/specs/doc-code-seam-shards.md` §4 RED test targets (4
//! canonical) + Interpretation B discipline (6 mirroring TICK 1 T15-T20).
//!
//! Post TICK 3 close (Reed 🔴 8466a05-drift + Mara ♻️ 98664a7 GREEN +
//! Seam-inline `ff53d46` RATIFY-WITH-CORRECTIONS).
//!
//! RED phase: `shards/kintsugi/fracture/docblock_ungrounded.mirror` does not
//! yet exist. Text-check tests fail on file absence.
//!
//! Closes: task #535 sub-arc A TICK 4 entry point. Mara 🟢 dispatched post-RED.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_ungrounded_shard() -> String {
    let path = repo_root().join("shards/kintsugi/fracture/docblock_ungrounded.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/kintsugi/fracture/docblock_ungrounded.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T4: canonical shape per Mara spec §4 RED test targets ===

#[test]
fn t01_docblock_ungrounded_declares_glass() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("glass @kintsugi/fracture/docblock_ungrounded"),
        "T1: must declare `glass @kintsugi/fracture/docblock_ungrounded` per Mara spec §4"
    );
}

#[test]
fn t02_docblock_ungrounded_declares_body() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("docblock_ungrounded_body(c: doc_claim, ctx: kintsugi_context)"),
        "T2: must declare `docblock_ungrounded_body(c: doc_claim, ctx: kintsugi_context)` per Mara spec §4"
    );
}

#[test]
fn t03_docblock_ungrounded_requires_ashby_variety_match() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("requires ashby_variety_match(kintsugi_lock)"),
        "T3: body must carry `requires ashby_variety_match(kintsugi_lock)` per Mara spec §4"
    );
}

#[test]
fn t04_docblock_ungrounded_inherits_kintsugi_surface() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("in @kintsugi/surface"),
        "T4: must inherit `in @kintsugi/surface` per Mara spec §4 (routes via ashby_mismatch class)"
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
    let content = read_docblock_ungrounded_shard();
    let first = first_nonempty_line(&content).expect("T5: must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T5: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
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
    let content = read_docblock_ungrounded_shard();
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
    let content = read_docblock_ungrounded_shard();
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
fn t08_narrative_names_operational_role() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("operational") || content.contains("fracture body") || content.contains("docblock_grounded"),
        "T8: narrative must position this shard as operational half of docblock_grounded bilateral (Mara spec §4)"
    );
}

#[test]
fn t09_narrative_names_ashby_mismatch_routing() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("ashby_mismatch"),
        "T9: narrative must name `ashby_mismatch` routing class per compiler-error-surface §3.1"
    );
}

#[test]
fn t10_narrative_grounds_bilateral_pair() {
    let content = read_docblock_ungrounded_shard();
    assert!(
        content.contains("#53") && content.contains("docblock_grounded"),
        "T10: narrative must ground #53 bilateral pair with docblock_grounded declarative half"
    );
}
