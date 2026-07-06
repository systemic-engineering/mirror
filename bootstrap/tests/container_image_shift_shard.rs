//! Arc 5 TICK 3 RED — `@container/image` = `shift(@mirror/store/oci → @container altitude)`.
//!
//! **First empirical witness of TICK 2's promoted `@kintsugi/shift` primitive.**
//!
//! Resolves original Seam Correction #1 (drop @container/image due to overlap with
//! @mirror/store/oci) via Alex's hardlink/lens framing: same OID at two altitudes,
//! zero duplication.
//!
//! Post Arc 5:
//! - TICK 1 `aaa9a81` @container family-root
//! - TICK 2 `49f0486` @kintsugi/shift substrate-decl species (14th #53 bilateral)
//! - TICK 3 (this): @container/image species as first witness
//!
//! Shard should declare image at @container altitude as a `shift`-form of the
//! image at @mirror/store/oci altitude. NO duplicate substrate-decl of OCI
//! content. Hardlink semantic: `shift_preserves_content` invariant applies.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_container_image_shard() -> String {
    let path = repo_root().join("shards/container/image.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/container/image.mirror at {:?}: {}", path, e))
}

// === T1-T7: canonical species shape ===

#[test]
fn t01_container_image_declares_species_prism() {
    let content = read_container_image_shard();
    assert!(
        content.contains("prism @container/image"),
        "T1: must declare `prism @container/image` species per path-namespace pact"
    );
}

#[test]
fn t02_container_image_declares_five_op_body() {
    let content = read_container_image_shard();
    for op in ["focus", "project", "split", "shift", "settle"] {
        assert!(
            content.contains(op),
            "T2: prism @container/image must declare five-op `{}`",
            op
        );
    }
}

#[test]
fn t03_container_image_composes_with_kintsugi_shift() {
    let content = read_container_image_shard();
    assert!(
        content.contains("@kintsugi/shift"),
        "T3: must reference `@kintsugi/shift` primitive (this species IS a witness of the shift promoted at TICK 2 `49f0486`)"
    );
}

#[test]
fn t04_container_image_cites_mirror_store_oci_source_altitude() {
    let content = read_container_image_shard();
    assert!(
        content.contains("@mirror/store/oci") || content.contains("mirror/store/oci"),
        "T4: must cite `@mirror/store/oci` as the SOURCE altitude of the shift (image LIVES at store; @container/image is the lens/hardlink at container-altitude)"
    );
}

#[test]
fn t05_container_image_declares_image_witness_carrier() {
    let content = read_container_image_shard();
    // The shift_witness at container-altitude — same OID as @mirror/store/oci; different reader-frame
    assert!(
        content.contains("image_witness") || content.contains("shift_witness") || content.contains("witness: ref") || content.contains("= shift_witness"),
        "T5: must declare a witness carrier (image_witness OR reuse of shift_witness from @kintsugi/shift) — the OID persisting across altitude crossing"
    );
}

#[test]
fn t06_container_image_declares_composed_bilateral_shift_from_oci() {
    let content = read_container_image_shard();
    // The 15th #53 bilateral: asserts the image at container-altitude IS the shift of the image at @mirror/store/oci altitude
    assert!(
        content.contains("image_shifted_from_oci") || content.contains("image_from_store") || content.contains("image_hardlinks_oci"),
        "T6: must declare composed-bilateral (image_shifted_from_oci OR image_from_store OR image_hardlinks_oci) — 15th #53 instance; asserts hardlink to @mirror/store/oci"
    );
}

#[test]
fn t07_container_image_no_duplicate_oci_declaration() {
    let content = read_container_image_shard();
    // Must NOT redeclare OCI content shape (that lives at @mirror/store/oci)
    // This is asserted by NOT having a `type oci_manifest` or similar reinvention
    assert!(
        !content.contains("type oci_manifest") && !content.contains("type oci_content") && !content.contains("type image_bytes"),
        "T7: must NOT redeclare OCI content shape (belongs at @mirror/store/oci; this species is a lens/hardlink via @kintsugi/shift). No `type oci_manifest` / `type oci_content` / `type image_bytes`."
    );
}

// === T8-T13: Interpretation B + inheritance discipline ===

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
fn t08_first_nonempty_line_is_narrative_docblock() {
    let content = read_container_image_shard();
    let first = first_nonempty_line(&content).expect("T8: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T8: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T8: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T8: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t09_exactly_one_seam_at_column_zero() {
    let content = read_container_image_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T9: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
}

#[test]
fn t10_in_clauses_below_seam() {
    let content = read_container_image_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T10 depends on T9 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T10: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t11_container_image_inherits_prism_meta_glass() {
    let content = read_container_image_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T11: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t12_container_image_inherits_container_parent_family() {
    let content = read_container_image_shard();
    assert!(
        content.contains("in @container") && !content.contains("in @container/image"),
        "T12: must inherit `in @container` (parent family per path-namespace pact); species does not inherit itself"
    );
}

#[test]
fn t13_narrative_grounds_hardlink_lens_framing() {
    let content = read_container_image_shard();
    assert!(
        content.contains("hardlink") || content.contains("lens") || content.contains("mount") || content.contains("alias"),
        "T13: narrative must ground Alex's hardlink/lens/mount/alias framing — same OID at two altitudes, zero content duplication"
    );
}

// === T14-T15: Prior-art grounding (substrate-already-had-the-word) ===

#[test]
fn t14_narrative_names_first_witness_role() {
    let content = read_container_image_shard();
    assert!(
        content.contains("first witness") || content.contains("first empirical witness") || content.contains("first instance") || content.contains("empirical witness"),
        "T14: narrative must name this species as first empirical witness of the @kintsugi/shift primitive promoted at TICK 2 `49f0486`"
    );
}

#[test]
fn t15_narrative_cites_seam_correction_1_resolution() {
    let content = read_container_image_shard();
    // Cites either the original Seam correction or the pre-review commit c9e153d
    assert!(
        content.contains("c9e153d") || content.contains("Correction") || content.contains("correction"),
        "T15: narrative must cite Seam Correction #1 (original 'drop @container/image' at `c9e153d`) and how the shift-form resolves it without dropping the surface"
    );
}
