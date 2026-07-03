//! Phase A RED — kintsugi-surface-shard.
//!
//! Per Mara's canonical spec at `docs/math/kintsugi/compiler-error-surface.md`
//! (`920fe86` + amendment `9f4211d`): land `shards/kintsugi/surface.mirror` as
//! the fifth #53 property/fracture bilateral instance, composing against
//! landed `ashby_variety_match(kintsugi_lock)` per Alex 2026-07-02 #113
//! twelfth-instance discipline (name the lineage).
//!
//! Signature to be realized:
//!
//!   surface(t: tension, class: surface_class, ctx: kintsugi_context)
//!     -> observation_depth
//!     requires ashby_variety_match(kintsugi_lock)
//!
//! Four surface class carriers per Mara §3:
//!   - ashby_mismatch    (Tomm shape: circular)
//!   - contradiction     (Tomm shape: linear then reflexive)
//!   - conundrum         (Tomm shape: reflexive)
//!   - out_of_band       (Tomm shape: strategic)
//!
//! Composes with landed substrate:
//!   - `ashby_variety_match(lock)` at `@epistemologic/cybernetic/coherence-parametric`
//!     (LANDED 2026-06-11, verbatim ancestor per Taut scout)
//!   - `@spawn <= @loop` monad + budget (spawn-parent-family-lift 2026-07-02)
//!   - `@epistemologic/property/restart_intensity_well_formed` +
//!     `@kintsugi/fracture/restart_storm` (bilateral pair landed today `e7bd6ec`)
//!
//! **RED phase**: `shards/kintsugi/surface.mirror` does not yet exist. Text-
//! check tests fail on file absence and signature absence. Discipline mirrors
//! `restart_intensity_bilateral.rs` (Reed `342d63f`) pattern.
//!
//! Closes: kintsugi arc's shard forward-promise (`9f4211d` landing-order
//! step 5) + operationalizes yesterday's cybernetic-kintsugi architecture at
//! substrate altitude.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_surface_shard() -> String {
    let path = repo_root().join("shards/kintsugi/surface.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/kintsugi/surface.mirror at {:?}: {}", path, e))
}

#[test]
fn surface_shard_declares_surface_action() {
    let content = read_surface_shard();
    assert!(
        content.contains("surface("),
        "kintsugi/surface must declare `surface` action per Mara §7.2 canonical signature"
    );
}

#[test]
fn surface_shard_takes_tension_carrier() {
    let content = read_surface_shard();
    assert!(
        content.contains("tension"),
        "surface action must take `tension` parameter per Mara §2 (tension = failed property verdict + located opacity)"
    );
}

#[test]
fn surface_shard_takes_surface_class() {
    let content = read_surface_shard();
    assert!(
        content.contains("surface_class"),
        "surface action must take `surface_class` parameter per Mara §3 four-class disjunction"
    );
}

#[test]
fn surface_shard_returns_observation_depth() {
    let content = read_surface_shard();
    assert!(
        content.contains("observation_depth"),
        "surface action must return `observation_depth` (a @third carrier per Mara §7.2 signature; conditional-marker @third per candidate #141 discipline)"
    );
}

#[test]
fn surface_shard_requires_ashby_variety_match() {
    let content = read_surface_shard();
    assert!(
        content.contains("ashby_variety_match"),
        "surface action must `requires ashby_variety_match(kintsugi_lock)` per Alex 2026-07-02 #113 twelfth-instance discipline (name the lineage; compose against landed `@epistemologic/cybernetic/coherence-parametric.ashby_variety_match`)"
    );
}

#[test]
fn surface_shard_declares_ashby_mismatch_class() {
    let content = read_surface_shard();
    assert!(
        content.contains("ashby_mismatch"),
        "surface shard must declare/reference `ashby_mismatch` surface class per Mara §3.1"
    );
}

#[test]
fn surface_shard_declares_contradiction_class() {
    let content = read_surface_shard();
    assert!(
        content.contains("contradiction"),
        "surface shard must declare/reference `contradiction` surface class per Mara §3.2"
    );
}

#[test]
fn surface_shard_declares_conundrum_class() {
    let content = read_surface_shard();
    assert!(
        content.contains("conundrum"),
        "surface shard must declare/reference `conundrum` surface class per Mara §3.3"
    );
}

#[test]
fn surface_shard_declares_out_of_band_class() {
    let content = read_surface_shard();
    assert!(
        content.contains("out_of_band"),
        "surface shard must declare/reference `out_of_band` surface class per Mara §3.4"
    );
}

#[test]
fn surface_shard_inherits_kintsugi() {
    let content = read_surface_shard();
    assert!(
        content.contains("in @kintsugi") || content.contains("<= @kintsugi"),
        "surface shard must inherit @kintsugi per family-root discipline (parent of angle_to_paren, symbol_lift, restart_storm)"
    );
}
