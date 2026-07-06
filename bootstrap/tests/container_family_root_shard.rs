//! Arc 5 TICK 1 RED — `@container` top-level family-root.
//!
//! Per Seam pre-review at `c9e153d` (docs/audits/2026-07-06-seam-pre-review-docker-container-substrate-decl.md):
//! RATIFY-WITH-CORRECTIONS on Shape γ from Mara's spec `ec636d3`
//! (docs/specs/docker-container-substrate-decl-v0.1.md).
//!
//! **Substrate-pull-notable outcome**: Seam confirmed #C3 as SECOND WITNESS for
//! recognition #55 form/process partition. containerd's four-layer split
//! (content store + snapshotter = form; tasks + shim v2 = process) provides
//! independent canonical evidence. #55 promotion gate SATISFIED.
//!
//! Hedges resolved in review:
//! - H1: `@container` COMPOSES with `@autopoietic` at species-body; does NOT
//!   inherit at family-root (T9 asserts this discipline)
//! - H2: TOP-LEVEL `@container` (sibling to @io/@code/@mirror/@kintsugi);
//!   Shape α would COLLAPSE #55.
//!
//! T1-T13 per Seam §4 canonical roster + T14-T15 Interpretation B discipline
//! mirroring sub-arc A pattern.
//!
//! Closes: task #541 TICK 1 entry. Mara 🟢 dispatched post-RED to land
//! `shards/container.mirror` family-root.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_container_shard() -> String {
    let path = repo_root().join("shards/container.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/container.mirror at {:?}: {}", path, e))
}

// === T1-T13: canonical shape per Seam §4 pre-review roster ===

#[test]
fn t01_container_shard_file_exists() {
    let _ = read_container_shard();
    // Passes if file readable.
}

#[test]
fn t02_container_declares_family_root() {
    let content = read_container_shard();
    assert!(
        content.contains("prism @container"),
        "T2: shards/container.mirror must declare `prism @container` per path-namespace pact"
    );
}

#[test]
fn t03_container_inherits_prism() {
    let content = read_container_shard();
    assert!(
        content.contains("in @prism"),
        "T3: must inherit `in @prism` (universal marker)"
    );
}

#[test]
fn t04_container_inherits_meta() {
    let content = read_container_shard();
    assert!(
        content.contains("in @meta"),
        "T4: must inherit `in @meta` (universal family-root marker)"
    );
}

#[test]
fn t05_container_inherits_glass() {
    let content = read_container_shard();
    assert!(
        content.contains("in @glass"),
        "T5: must inherit `in @glass` (transparency default)"
    );
}

#[test]
fn t06_container_no_autopoietic_inheritance_at_family_root() {
    let content = read_container_shard();
    let prism_block: String = content
        .lines()
        .skip_while(|l| !l.contains("prism @container"))
        .take_while(|l| !l.trim().starts_with("type") && !l.contains("prism_kind"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !prism_block.contains("<= @autopoietic") && !prism_block.contains("<=@autopoietic"),
        "T6: NO `<= @autopoietic` on family-root prism per Seam H1: composition at species-body, NOT inheritance at family-root. Prism block: {}",
        prism_block
    );
}

#[test]
fn t07_container_declares_five_op_prism_body() {
    let content = read_container_shard();
    for op in ["focus", "project", "split", "shift", "settle"] {
        assert!(
            content.contains(op),
            "T7: prism @container must declare five-operation `{}` per canonical body shape (mirroring shards/io/oci.mirror)",
            op
        );
    }
}

#[test]
fn t08_container_declares_container_spec_carrier() {
    let content = read_container_shard();
    assert!(
        content.contains("type container_spec = ref"),
        "T8: must declare `type container_spec = ref` per Seam §4 (byte-equality on OCI runtime-spec config bytes)"
    );
}

#[test]
fn t09_container_declares_container_rootfs_carrier() {
    let content = read_container_shard();
    assert!(
        content.contains("type container_rootfs = ref"),
        "T9: must declare `type container_rootfs = ref` per Seam §4"
    );
}

#[test]
fn t10_container_declares_container_caps_carrier() {
    let content = read_container_shard();
    assert!(
        content.contains("type container_caps = ref"),
        "T10: must declare `type container_caps = ref` per Seam §4"
    );
}

#[test]
fn t11_container_declares_composed_bilateral_container_runnable() {
    let content = read_container_shard();
    assert!(
        content.contains("container_runnable"),
        "T11: must declare composed-bilateral `container_runnable(spec, p) -> verdict` per Seam §4 (13th instance of #53 bilateral pattern)"
    );
}

#[test]
fn t12_narrative_cross_references_oci() {
    let content = read_container_shard();
    assert!(
        content.contains("@io/oci") || content.contains("shards/io/oci"),
        "T12: narrative must cross-reference `@io/oci` / `shards/io/oci.mirror` (load-bearing distribution-adapter dependency)"
    );
}

#[test]
fn t13_narrative_names_form_side_witness() {
    let content = read_container_shard();
    assert!(
        content.contains("#55") || content.contains("form/process") || content.contains("form-side"),
        "T13: narrative must ground #55 form/process partition placement (Shape γ preserves; Shape α collapses)"
    );
}

// === T14-T15: Interpretation B structural discipline ===

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
fn t14_first_nonempty_line_is_narrative_and_exactly_one_seam() {
    let content = read_container_shard();
    let first = first_nonempty_line(&content).expect("T14: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T14: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T14: line-1 `---` is drift");
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T14: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
}

#[test]
fn t15_in_clauses_below_seam() {
    let content = read_container_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T15 depends on T14 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T15: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}
