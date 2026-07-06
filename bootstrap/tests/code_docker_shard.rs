//! Arc 5 TICK 4 RED — `@code/docker` species (Dockerfile as declarative code).
//!
//! Per Mara spec `docs/specs/docker-container-substrate-decl-v0.1.md` (`ec636d3`) §6.
//! Sits under `@code` family-root at code altitude. Closes the three-surface
//! partition:
//! - `@code/docker` — declaration altitude (Dockerfile as content-addressable code)
//! - `@container/image` (TICK 3 `b66b280`) — runtime altitude (shift-form)
//! - `@io/oci` — distribution altitude (registry protocol)
//!
//! Composes with `@kintsugi/shift` (TICK 2 `49f0486`): docker source `build`s into
//! an image at container altitude via cross-altitude fold. The build IS a shift.
//!
//! Post cascade:
//! - TICK 1 `aaa9a81` @container family-root
//! - TICK 2 `49f0486` @kintsugi/shift substrate-decl primitive
//! - TICK 3 `b66b280` @container/image first witness
//! - TICK 4 (this): @code/docker species

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_docker_shard() -> String {
    let path = repo_root().join("shards/code/docker.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/code/docker.mirror at {:?}: {}", path, e))
}

// === T1-T7: canonical species shape ===

#[test]
fn t01_code_docker_declares_species_prism() {
    let content = read_docker_shard();
    assert!(
        content.contains("prism @code/docker"),
        "T1: must declare `prism @code/docker` species per path-namespace pact"
    );
}

#[test]
fn t02_code_docker_declares_five_op_body() {
    let content = read_docker_shard();
    for op in ["focus", "project", "split", "shift", "settle"] {
        assert!(
            content.contains(op),
            "T2: prism @code/docker must declare five-op body operation `{}`",
            op
        );
    }
}

#[test]
fn t03_code_docker_declares_dockerfile_carrier() {
    let content = read_docker_shard();
    assert!(
        content.contains("type dockerfile = ref") || content.contains("dockerfile: ref"),
        "T3: must declare `dockerfile` typed carrier (Dockerfile as content-addressable code)"
    );
}

#[test]
fn t04_code_docker_declares_composed_bilateral_docker_buildable() {
    let content = read_docker_shard();
    assert!(
        content.contains("docker_buildable"),
        "T4: must declare composed-bilateral `docker_buildable(d: dockerfile) -> verdict` per Mara spec §6 (16th #53 instance)"
    );
}

#[test]
fn t05_code_docker_declares_build_action_returning_image_witness() {
    let content = read_docker_shard();
    // The build action: dockerfile -> image at container altitude via shift
    assert!(
        content.contains("build(") && (content.contains("image_witness") || content.contains("@container/image") || content.contains("shift_witness")),
        "T5: must declare `build(d: dockerfile) -> image_witness` (or equivalent) — the docker source folds into an image at container altitude via @kintsugi/shift"
    );
}

#[test]
fn t06_code_docker_composes_with_kintsugi_shift() {
    let content = read_docker_shard();
    assert!(
        content.contains("@kintsugi/shift"),
        "T6: must compose with `@kintsugi/shift` primitive (build is the cross-altitude fold from code altitude to container altitude)"
    );
}

#[test]
fn t07_code_docker_composes_with_container_image() {
    let content = read_docker_shard();
    assert!(
        content.contains("@container/image"),
        "T7: must compose with `@container/image` (TICK 3 `b66b280`) — the fold TARGET at container altitude"
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
    let content = read_docker_shard();
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
    let content = read_docker_shard();
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
    let content = read_docker_shard();
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
fn t11_code_docker_inherits_universal_prism_meta_glass() {
    let content = read_docker_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T11: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t12_code_docker_inherits_code_parent_family() {
    let content = read_docker_shard();
    assert!(
        content.contains("in @code") && !content.contains("in @code/docker"),
        "T12: must inherit `in @code` (parent family per path-namespace pact); species does not inherit itself"
    );
}

#[test]
fn t13_narrative_grounds_three_surface_partition() {
    let content = read_docker_shard();
    // @code/docker declaration ↔ @container/image runtime ↔ @io/oci distribution
    let has_two_of_three = ["@io/oci", "@container/image", "three-surface"]
        .iter()
        .filter(|c| content.contains(*c))
        .count()
        >= 2;
    assert!(
        has_two_of_three,
        "T13: narrative must ground three-surface partition (@code/docker declaration ↔ @container/image runtime ↔ @io/oci distribution) — at least 2 of the surfaces named"
    );
}

// === T14-T15: cascade grounding ===

#[test]
fn t14_narrative_cites_kintsugi_shift_primitive() {
    let content = read_docker_shard();
    // Cites TICK 2's promoted primitive OR the recognition
    assert!(
        content.contains("49f0486") || content.contains("@kintsugi/shift") || content.contains("shift primitive"),
        "T14: narrative must cite `@kintsugi/shift` primitive (TICK 2 `49f0486`) that build composes against"
    );
}

#[test]
fn t15_narrative_names_build_as_cross_altitude_fold() {
    let content = read_docker_shard();
    assert!(
        content.contains("cross-altitude") || content.contains("fold") || content.contains("code altitude") || content.contains("container altitude"),
        "T15: narrative must name `build` as cross-altitude fold from code altitude → container altitude via @kintsugi/shift"
    );
}
