//! Arc 3 TICK 6 RED — `@kintsugi/fracture/docblock_incoherent`.
//!
//! Operational half of TICK 5 (`docblock_coherent` at `d158ca0`). Routes via
//! `@kintsugi/surface`'s `contradiction` class per compiler-error-surface §3.2
//! (RIGOROUS via `[ω,ω]` Bateson-bind).
//!
//! Per Mara spec `docs/specs/doc-code-seam-shards.md` §6 RED test targets (3
//! canonical) + Interpretation B discipline (6 mirroring TICK 4).
//!
//! Post TICK 5 close (Reed 🔴 `be4f27a` + Mara 🟢 `d158ca0` + Seam 📝 `83bbdc8`
//! RATIFY). Second #53 bilateral trio pair closing.
//!
//! RED phase: `shards/kintsugi/fracture/docblock_incoherent.mirror` does not
//! yet exist. Text-check tests fail on file absence.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_incoherent_shard() -> String {
    let path = repo_root().join("shards/kintsugi/fracture/docblock_incoherent.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/kintsugi/fracture/docblock_incoherent.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T3: canonical shape per Mara spec §6 RED test targets ===

#[test]
fn t01_docblock_incoherent_declares_glass() {
    let content = read_docblock_incoherent_shard();
    assert!(
        content.contains("glass @kintsugi/fracture/docblock_incoherent"),
        "T1: must declare `glass @kintsugi/fracture/docblock_incoherent` per Mara spec §6"
    );
}

#[test]
fn t02_docblock_incoherent_declares_body() {
    let content = read_docblock_incoherent_shard();
    assert!(
        content.contains("docblock_incoherent_body(d: docblock, ctx: kintsugi_context)"),
        "T2: must declare `docblock_incoherent_body(d: docblock, ctx: kintsugi_context)` per Mara spec §6"
    );
}

#[test]
fn t03_docblock_incoherent_requires_ashby_variety_match() {
    let content = read_docblock_incoherent_shard();
    assert!(
        content.contains("requires ashby_variety_match(kintsugi_lock)"),
        "T3: body must carry `requires ashby_variety_match(kintsugi_lock)` per Mara spec §6"
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
    let content = read_docblock_incoherent_shard();
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
    let content = read_docblock_incoherent_shard();
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
    let content = read_docblock_incoherent_shard();
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
fn t07_narrative_names_contradiction_routing() {
    let content = read_docblock_incoherent_shard();
    assert!(
        content.contains("contradiction"),
        "T7: narrative must name `contradiction` routing class per compiler-error-surface §3.2"
    );
}

#[test]
fn t08_narrative_names_operational_pair() {
    let content = read_docblock_incoherent_shard();
    assert!(
        content.contains("docblock_coherent"),
        "T8: narrative must name declarative pair (docblock_coherent at TICK 5)"
    );
}

#[test]
fn t09_narrative_grounds_bateson_bind() {
    let content = read_docblock_incoherent_shard();
    assert!(
        content.contains("[ω,ω]")
            || content.contains("Bateson")
            || content.contains("[omega")
            || content.contains("§3.2"),
        "T9: narrative must ground `[ω,ω]` Bateson-bind rigor per compiler-error-surface §3.2"
    );
}
