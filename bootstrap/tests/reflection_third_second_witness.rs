//! Phase A RED — reflection-third-second-witness.
//!
//! Per Seam audit `017e568` §Next-/loop: land the SECOND witness for
//! candidate #141 (@third conditional-marker) at `shards/reflection.mirror`
//! at pipeline-error altitude.
//!
//! **The precedent** (candidate #141 first-instance): Mara `e910dd6` at
//! `shards/kintsugi/surface.mirror` imports `in @third` at species altitude
//! only — the family root `shards/kintsugi.mirror` stays clean (only
//! `@prism/@glass/@meta`).
//!
//! **The second witness**: `shards/reflection.mirror` currently imports 18
//! markers including `in @kintsugi/oscillate` but NOT `in @third` (Seam
//! grep-verified at `017e568`). Adding `in @third` at pipeline-error
//! altitude (per Mara compiler-error-surface §10.4 candidate #143 forward-
//! promise) fires the second-witness gate for #141 promotion.
//!
//! **RED phase**: `shards/reflection.mirror` does not yet import `in @third`.
//! Text-check test fails on absence. GREEN adds the import at species
//! altitude and updates the docblock to reference candidate #141 second-
//! witness precedent.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_reflection_shard() -> String {
    let path = repo_root().join("shards/reflection.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/reflection.mirror at {:?}: {}", path, e))
}

#[test]
fn reflection_imports_third() {
    let content = read_reflection_shard();
    assert!(
        content.contains("in @third"),
        "@reflection must import `in @third` at species altitude per candidate #141 second-witness gate (first witness: Mara `e910dd6` at kintsugi/surface)"
    );
}

#[test]
fn reflection_docblock_references_141_precedent() {
    let content = read_reflection_shard();
    assert!(
        content.contains("#141") || content.contains("candidate 141") || content.contains("candidate #141"),
        "@reflection docblock must reference candidate #141 (conditional-marker) precedent per Seam audit `017e568` recommendation"
    );
}

#[test]
fn reflection_docblock_references_kintsugi_surface_first_witness() {
    let content = read_reflection_shard();
    assert!(
        content.contains("kintsugi/surface") || content.contains("e910dd6"),
        "@reflection docblock must cite the first-witness (kintsugi/surface at `e910dd6`) per un-cite-ability discipline (citation-must-include-OID for candidate #141 promotion chain)"
    );
}
