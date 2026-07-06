//! Arc 3 TICK 5 RED — `@epistemologic/property/docblock_coherent`.
//!
//! Second #53 property/fracture bilateral of the trio. Declarative half; operational
//! half (`@kintsugi/fracture/docblock_incoherent` via `contradiction` routing) at TICK 6.
//!
//! Per Mara spec `docs/specs/doc-code-seam-shards.md` §5 RED test targets (3 canonical)
//! + Interpretation B discipline (6 mirroring TICK 1 T15-T20).
//!
//! Post TICK 4 close (Reed 🔴 `d454088` + Mara 🟢 `6cb6054` + Seam 📝 `9b0aeb5`
//! RATIFY). First #53 bilateral trio pair complete; second trio pair starting.
//!
//! RED phase: `shards/epistemologic/property/docblock_coherent.mirror` does not
//! yet exist. Text-check tests fail on file absence.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_coherent_shard() -> String {
    let path = repo_root().join("shards/epistemologic/property/docblock_coherent.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/epistemologic/property/docblock_coherent.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T3: canonical shape per Mara spec §5 RED test targets ===

#[test]
fn t01_docblock_coherent_declares_prism() {
    let content = read_docblock_coherent_shard();
    assert!(
        content.contains("prism @epistemologic/property/docblock_coherent"),
        "T1: must declare `prism @epistemologic/property/docblock_coherent` per Mara spec §5"
    );
}

#[test]
fn t02_docblock_coherent_declares_predicate() {
    let content = read_docblock_coherent_shard();
    assert!(
        content.contains("docblock_coherent(d: docblock) -> verdict"),
        "T2: must declare `docblock_coherent(d: docblock) -> verdict` per Mara spec §5"
    );
}

#[test]
fn t03_docblock_coherent_inherits_liquid_extraction() {
    let content = read_docblock_coherent_shard();
    assert!(
        content.contains("in @epistemologic/liquid_extraction"),
        "T3: must inherit `in @epistemologic/liquid_extraction` per Mara spec §5 (extract_predicates dependency)"
    );
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
    let content = read_docblock_coherent_shard();
    let first = first_nonempty_line(&content).expect("T4: must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T4: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
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
    let content = read_docblock_coherent_shard();
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
    let content = read_docblock_coherent_shard();
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
fn t07_narrative_names_second_bilateral_of_trio() {
    let content = read_docblock_coherent_shard();
    assert!(
        content.contains("#53") && (content.contains("second") || content.contains("trio")),
        "T7: narrative must ground #53 second-of-trio bilateral per Mara spec §5"
    );
}

#[test]
fn t08_narrative_names_operational_pair() {
    let content = read_docblock_coherent_shard();
    assert!(
        content.contains("docblock_incoherent") || content.contains("contradiction"),
        "T8: narrative must name operational half (docblock_incoherent) or its routing (contradiction class per compiler-error-surface §3.2)"
    );
}

#[test]
fn t09_narrative_grounds_predicate_substance() {
    let content = read_docblock_coherent_shard();
    assert!(
        content.contains("contradicts") || content.contains("extract_predicates") || content.contains("below_seam"),
        "T9: narrative must ground predicate substance per Mara spec §5 (contradicts / extract_predicates / below_seam)"
    );
}
