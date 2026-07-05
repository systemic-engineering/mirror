//! Phase A RED — restart-intensity-bilateral.
//!
//! Per Seam audit `f732d9c` §Next-/loop: land the property/fracture
//! bilateral pair for `restart_intensity`. Closes recognition #53's
//! fourth instance (property/fracture bilateral pattern) AND the
//! restart-intensity-shard arc's forward-promised bilateral at
//! `shards/spectral/supervisor.mirror` lines 269-272.
//!
//! Two shards to land (Mara Phase B):
//!
//! - `shards/epistemologic/property/restart_intensity_well_formed.mirror`
//!   The declarative predicate side of the bilateral.
//!   `well_formed(ri: restart_intensity) -> verdict`
//!   Fires `success` when the intensity is well-formed at supervision
//!   altitude; `failure` otherwise.
//!
//! - `shards/kintsugi/fracture/restart_storm.mirror`
//!   The operational fracture-body side of the bilateral.
//!   Takes opacity (from @glass); returns morphism (from @kintsugi/consent).
//!   Per Mara emergent-supervision §5.5 H4: when the storm rate exceeds
//!   the intensity's window, kintsugi surfaces (spawn or hold per
//!   compiler-error-surface §8's three-mode algebra) rather than
//!   silently restarting into a storm.
//!
//! **RED phase**: neither shard exists. Text-check tests fail on
//! file absence and signature absence. Discipline mirrors
//! `restart_intensity_shard.rs` (Reed `9f63730`) at the bilateral
//! altitude.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_property_shard() -> String {
    let path =
        repo_root().join("shards/epistemologic/property/restart_intensity_well_formed.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read property shard at {:?}: {}", path, e))
}

fn read_fracture_shard() -> String {
    let path = repo_root().join("shards/kintsugi/fracture/restart_storm.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fracture shard at {:?}: {}", path, e))
}

// -----------------------------------------------------------------
// Property side: @epistemologic/property/restart_intensity_well_formed
// -----------------------------------------------------------------

#[test]
fn property_shard_declares_well_formed_action() {
    let content = read_property_shard();
    assert!(
        content.contains("well_formed"),
        "property shard must declare `well_formed` action (predicate on restart_intensity) per Seam audit f732d9c + #53 bilateral pattern"
    );
}

#[test]
fn property_shard_takes_restart_intensity_carrier() {
    let content = read_property_shard();
    assert!(
        content.contains("restart_intensity"),
        "property shard must reference the `restart_intensity` carrier (landed at Mara `a3dcb94`) as its predicate subject"
    );
}

#[test]
fn property_shard_returns_verdict() {
    let content = read_property_shard();
    assert!(
        content.contains("-> verdict") || content.contains("-&gt; verdict"),
        "property shard's well_formed action must return `verdict` per @epistemologic/property bilateral discipline"
    );
}

#[test]
fn property_shard_inherits_epistemologic_property() {
    let content = read_property_shard();
    assert!(
        content.contains("in @epistemologic"),
        "property shard must inherit @epistemologic (typical: `in @epistemologic`) per landed cybernetic/property family precedent"
    );
}

// -----------------------------------------------------------------
// Fracture side: @kintsugi/fracture/restart_storm
// -----------------------------------------------------------------

#[test]
fn fracture_shard_declares_resolve_action() {
    let content = read_fracture_shard();
    assert!(
        content.contains("resolve_restart_storm") || content.contains("resolve_storm"),
        "fracture shard must declare a `resolve_*` action per @kintsugi/fracture bilateral discipline (existing pattern: angle_to_paren, symbol_lift)"
    );
}

#[test]
fn fracture_shard_takes_opacity() {
    let content = read_fracture_shard();
    assert!(
        content.contains("opacity"),
        "fracture shard's resolve action must take `opacity` from @glass per existing fracture bodies (angle_to_paren, symbol_lift, symbol_lift.mirror pattern)"
    );
}

#[test]
fn fracture_shard_returns_morphism() {
    let content = read_fracture_shard();
    assert!(
        content.contains("morphism"),
        "fracture shard's resolve action must return `morphism` from @kintsugi/consent per existing fracture bodies"
    );
}

#[test]
fn fracture_shard_inherits_kintsugi_fracture() {
    let content = read_fracture_shard();
    assert!(
        content.contains("@kintsugi/fracture") || content.contains("in @kintsugi"),
        "fracture shard must inherit @kintsugi/fracture family per existing precedent"
    );
}

// -----------------------------------------------------------------
// Bilateral composition check
// -----------------------------------------------------------------

#[test]
fn bilateral_pair_cross_references() {
    let property_content = read_property_shard();
    let fracture_content = read_fracture_shard();

    // Property should mention the fracture body it discharges to
    assert!(
        property_content.contains("restart_storm")
            || property_content.contains("@kintsugi/fracture"),
        "property shard must cross-reference the paired fracture body per #53 bilateral pattern"
    );

    // Fracture should mention the property it restores
    assert!(
        fracture_content.contains("well_formed")
            || fracture_content.contains("@epistemologic/property"),
        "fracture shard must cross-reference the paired property per #53 bilateral pattern"
    );
}
