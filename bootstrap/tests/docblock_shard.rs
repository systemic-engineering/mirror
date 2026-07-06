//! Arc 3 TICK 1 RED — @docblock family-root shard.
//!
//! Per Seam Phase D pre-review at `docs/audits/2026-07-06-seam-arc-4-sub-arc-a-pre-review.md`
//! (43aaadd), Interpretation B canonical: docblock-narrative above `---`,
//! `in`+declarations below. Sub-arc A = Arc 3 TICK 1 = `shards/docblock.mirror`
//! family-root; NOT the property-family migration Taut proposed.
//!
//! T1-T14 canonical per Mara's spec `docs/specs/doc-code-seam-shards.md` §1.13
//! (RED-test-targets). T15-T20 Interpretation B structural discipline per
//! Seam audit §4 — enforce narrative-above / `in`+decl-below at landing time
//! so the shard does not silently drift into the same failure mode as the
//! four currently-landed line-1 `---` shards (drift, not precedent).
//!
//! RED phase: `shards/docblock.mirror` does not yet exist. Text-check tests
//! fail on file absence and signature absence. Discipline mirrors
//! `bootstrap/tests/kintsugi_surface_shard.rs` (Reed 2026-07-03) pattern.
//!
//! Closes: task #535 sub-arc A entry point. Mara 🟢 dispatched post-RED.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docblock_shard() -> String {
    let path = repo_root().join("shards/docblock.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/docblock.mirror at {:?}: {}", path, e))
}

// === T1-T14: canonical shape per Mara spec §1.13 ===

#[test]
fn t01_docblock_shard_declares_docblock_prism() {
    let content = read_docblock_shard();
    assert!(
        content.contains("prism @docblock"),
        "T1: docblock.mirror must declare `prism @docblock` per Mara spec §1.13.1"
    );
}

#[test]
fn t02_docblock_shard_declares_doc_claim_carrier() {
    let content = read_docblock_shard();
    assert!(
        content.contains("type doc_claim"),
        "T2: docblock.mirror must declare `type doc_claim` per Mara spec §1.13.2"
    );
}

#[test]
fn t03_docblock_shard_declares_claim_kind_variants() {
    let content = read_docblock_shard();
    assert!(
        content.contains("type claim_kind"),
        "T3: docblock.mirror must declare `type claim_kind` per Mara spec §1.13.3"
    );
    for variant in [
        "grounded_claim",
        "motivating_claim",
        "forward_promise",
        "candidate_claim",
    ] {
        assert!(
            content.contains(variant),
            "T3: claim_kind must include variant `{}` per Mara spec §1.13.3",
            variant
        );
    }
}

#[test]
fn t04_docblock_shard_declares_docblock_carrier() {
    let content = read_docblock_shard();
    assert!(
        content.contains("type docblock"),
        "T4: docblock.mirror must declare `type docblock` per Mara spec §1.13.4"
    );
}

#[test]
fn t05_docblock_shard_declares_audit_boundary() {
    let content = read_docblock_shard();
    assert!(
        content.contains("type audit_boundary = ref"),
        "T5: docblock.mirror must declare `type audit_boundary = ref` per Mara spec §1.13.5"
    );
}

#[test]
fn t06_docblock_shard_declares_verdict_variants() {
    let content = read_docblock_shard();
    assert!(
        content.contains("type docblock_verdict"),
        "T6: docblock.mirror must declare `type docblock_verdict` per Mara spec §1.13.6"
    );
    for variant in [
        "well_formed",
        "overreach",
        "incoherent",
        "underdeclares",
        "both_survive",
    ] {
        assert!(
            content.contains(variant),
            "T6: docblock_verdict must include variant `{}` per Mara spec §1.13.6",
            variant
        );
    }
}

#[test]
fn t07_docblock_shard_declares_extract_claims_action() {
    let content = read_docblock_shard();
    assert!(
        content.contains("extract_claims(d: docblock) -> ref"),
        "T7: docblock.mirror must declare `extract_claims(d: docblock) -> ref` per Mara spec §1.13.7"
    );
}

#[test]
fn t08_docblock_shard_declares_project_action() {
    let content = read_docblock_shard();
    assert!(
        content.contains("project(d: docblock) -> audit_boundary"),
        "T8: docblock.mirror must declare `project(d: docblock) -> audit_boundary` per Mara spec §1.13.8"
    );
}

#[test]
fn t09_docblock_shard_declares_audit_action() {
    let content = read_docblock_shard();
    assert!(
        content.contains("audit_docblock(d: docblock) -> docblock_verdict"),
        "T9: docblock.mirror must declare `audit_docblock(d: docblock) -> docblock_verdict` per Mara spec §1.13.9"
    );
}

#[test]
fn t10_docblock_shard_declares_bilateral_predicate() {
    let content = read_docblock_shard();
    assert!(
        content.contains("docblock_well_audited(d: docblock) -> verdict"),
        "T10: docblock.mirror must declare `docblock_well_audited(d: docblock) -> verdict` per Mara spec §1.13.10"
    );
}

#[test]
fn t11_docblock_shard_requires_bilateral_on_audit() {
    let content = read_docblock_shard();
    assert!(
        content.contains("requires docblock_well_audited(d)"),
        "T11: audit_docblock action must carry `requires docblock_well_audited(d)` obligation per Mara spec §1.13.11"
    );
}

#[test]
fn t12_docblock_shard_inherits_prism() {
    let content = read_docblock_shard();
    assert!(
        content.contains("in @prism"),
        "T12: docblock.mirror must inherit `in @prism` per Mara spec §1.13.12"
    );
}

#[test]
fn t13_docblock_shard_inherits_kintsugi() {
    let content = read_docblock_shard();
    assert!(
        content.contains("in @kintsugi"),
        "T13: docblock.mirror must inherit `in @kintsugi` per Mara spec §1.13.13"
    );
}

#[test]
fn t14_docblock_shard_inherits_epistemologic() {
    let content = read_docblock_shard();
    assert!(
        content.contains("in @epistemologic"),
        "T14: docblock.mirror must inherit `in @epistemologic` per Mara spec §1.13.14"
    );
}

// === T15-T20: Interpretation B structural discipline per Seam audit §4 ===

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
fn t15_first_nonempty_line_is_narrative_docblock() {
    let content = read_docblock_shard();
    let first = first_nonempty_line(&content)
        .expect("T15: docblock.mirror must have at least one non-empty line");
    assert!(
        first.trim_start().starts_with('#'),
        "T15: first non-empty line must be `#`-prefixed narrative-docblock (Interpretation B — narrative above), not `{}`. Line-1 `---` (Interpretation A) is drift per Seam audit §1.",
        first
    );
    assert_ne!(
        first.trim(),
        "---",
        "T15: first non-empty line MUST NOT be `---` — line-1 marker is Interpretation A drift"
    );
    assert!(
        !first.trim_start().starts_with("in "),
        "T15: first non-empty line MUST NOT be `in @...` clause — imports live BELOW the seam per Interpretation B"
    );
}

#[test]
fn t16_exactly_one_seam_line_at_column_zero() {
    let content = read_docblock_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T16: docblock.mirror must contain exactly one `---` line at column 0; found {} (per Mara spec §6.1 stateful predicate: one seam per file)",
        seams.len()
    );
}

#[test]
fn t17_in_clauses_below_seam() {
    let content = read_docblock_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams
        .first()
        .expect("T17 depends on T16 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T17: `in @...` clause at line {} appears ABOVE seam at line {} — Interpretation B places imports BELOW the seam. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t18_narrative_names_six_ancestors() {
    let content = read_docblock_shard();
    assert!(
        content.contains("six") && content.contains("ancestor"),
        "T18: narrative-docblock must name the six landed ancestors per `docs/math/kintsugi/doc-code-seam.md` §1 (canonical phrasing includes `six` + `ancestor`)"
    );
}

#[test]
fn t19_narrative_names_four_altitudes() {
    let content = read_docblock_shard();
    for altitude in ["linguistic", "logical", "temporal", "publishable"] {
        assert!(
            content.contains(altitude),
            "T19: narrative-docblock must name altitude `{}` per Mara §3 four-altitude structure",
            altitude
        );
    }
}

#[test]
fn t20_narrative_carries_both_survive_verdict() {
    let content = read_docblock_shard();
    assert!(
        content.contains("both_survive"),
        "T20: narrative-docblock must carry self-audit verdict claim `both_survive` per `63bdecc` §6 circular-reflexive discipline (not `real_survives` — promotion pending second witness)"
    );
}
