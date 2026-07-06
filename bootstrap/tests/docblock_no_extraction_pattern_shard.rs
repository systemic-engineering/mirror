//! Arc 3 TICK 7 RED — `@epistemologic/property/docblock_no_extraction_pattern`.
//!
//! Third #53 property/fracture bilateral of the trio. Declarative half;
//! operational half (`@kintsugi/fracture/docblock_extractive` with three-way
//! routing) at TICK 8.
//!
//! Four sub-checks (any one unbounded ⇒ predicate unbounded ⇒ audit_docblock
//! routes to `overreach`):
//!   1. no_unmarked_superlatives
//!   2. hedged_claims_marked_defer
//!   3. citations_content_match
//!   4. claim_size_matches_landing_size
//!
//! Per Mara spec `docs/specs/doc-code-seam-shards.md` §7 RED test targets (3
//! canonical) + Interpretation B discipline (6 mirroring TICK 5).
//!
//! Post TICK 6 close (Reed 🔴 `2d66909` + Mara 🟢 `019976d` + Seam 📝 `caab92f`).
//! Second #53 bilateral trio pair complete; third pair starting.
//!
//! RED phase: `shards/epistemologic/property/docblock_no_extraction_pattern.mirror`
//! does not yet exist. Text-check tests fail on file absence.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_no_extraction_pattern_shard() -> String {
    let path =
        repo_root().join("shards/epistemologic/property/docblock_no_extraction_pattern.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/epistemologic/property/docblock_no_extraction_pattern.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T3: canonical shape per Mara spec §7 RED test targets ===

#[test]
fn t01_docblock_no_extraction_pattern_declares_prism() {
    let content = read_docblock_no_extraction_pattern_shard();
    assert!(
        content.contains("prism @epistemologic/property/docblock_no_extraction_pattern"),
        "T1: must declare `prism @epistemologic/property/docblock_no_extraction_pattern` per Mara spec §7"
    );
}

#[test]
fn t02_docblock_no_extraction_pattern_declares_predicate() {
    let content = read_docblock_no_extraction_pattern_shard();
    assert!(
        content.contains("docblock_no_extraction_pattern(d: docblock) -> verdict"),
        "T2: must declare `docblock_no_extraction_pattern(d: docblock) -> verdict` per Mara spec §7"
    );
}

#[test]
fn t03_docblock_no_extraction_pattern_names_four_subchecks() {
    let content = read_docblock_no_extraction_pattern_shard();
    for subcheck in [
        "no_unmarked_superlatives",
        "hedged_claims_marked_defer",
        "citations_content_match",
        "claim_size_matches_landing_size",
    ] {
        assert!(
            content.contains(subcheck),
            "T3: docblock must LIST sub-check `{}` per Mara spec §7",
            subcheck
        );
    }
}

// === T4-T9: Interpretation B structural discipline ===

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
fn t04_first_nonempty_line_is_narrative_docblock() {
    let content = read_docblock_no_extraction_pattern_shard();
    let first = first_nonempty_line(&content).expect("T4: must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T4: first non-empty line must be `#`-narrative; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T4: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T4: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t05_exactly_one_seam_line_at_column_zero() {
    let content = read_docblock_no_extraction_pattern_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T5: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
}

#[test]
fn t06_in_clauses_below_seam() {
    let content = read_docblock_no_extraction_pattern_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T6 depends on T5 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T6: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t07_narrative_names_third_bilateral_of_trio() {
    let content = read_docblock_no_extraction_pattern_shard();
    assert!(
        content.contains("#53") && (content.contains("third") || content.contains("trio")),
        "T7: narrative must ground #53 third-of-trio bilateral per Mara spec §7"
    );
}

#[test]
fn t08_narrative_names_operational_pair() {
    let content = read_docblock_no_extraction_pattern_shard();
    assert!(
        content.contains("docblock_extractive") || content.contains("overreach"),
        "T8: narrative must name operational pair (docblock_extractive) or unbounded routing (overreach)"
    );
}

#[test]
fn t09_narrative_grounds_three_way_routing() {
    let content = read_docblock_no_extraction_pattern_shard();
    let has_two_of_three = ["ashby_mismatch", "contradiction", "out_of_band"]
        .iter()
        .filter(|c| content.contains(*c))
        .count()
        >= 2;
    assert!(
        has_two_of_three,
        "T9: narrative must name at least two of the three surface classes the sub-checks route to (ashby_mismatch / contradiction / out_of_band per Mara spec §8)"
    );
}
