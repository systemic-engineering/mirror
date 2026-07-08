//! M-CLEAN TICK 1 RED — `shards/mirror/peer/beam.mirror` (renamed 2026-07-08 Tick 2 from shards/mirror/spawn.mirror) @fate hinge composition
//! (removes stale Phase F quote; adds `in @fate` per M3 TICK 1 correction).
//!
//! **Substrate-pull correction cascade** (M3 TICK 1 close `21241f6`):
//!
//! Alex correction 2026-07-06: *"If @fate is not defined it needs to be
//! defined. It's the GLUE between runtime and compiletime. You're omitting
//! the hinge right now."*
//!
//! Substrate reality: `shards/fate.mirror` (42.1KB, LANDED 2026-06-30)
//! declares `prism @fate` with full five-op body; `shards/fate/tournament.mirror`
//! (41.5KB, LANDED 2026-06-30) declares `prism @fate/tournament`. Both landed
//! SIX DAYS AFTER Taut's Phase F correction (2026-06-24) that
//! `shards/mirror/peer/beam.mirror`'s docblock quotes as authority. **The quote
//! is stale.**
//!
//! **Cascade delta**: spawn.mirror should:
//! 1. Remove or reformulate the stale Phase F quote ("@fate is runtime
//!    substrate, not a substrate-decl prism") — substrate reality contradicts.
//! 2. Cite the @fate substrate-decl landing (2026-06-30) as current authority.
//! 3. Add `in @fate` ancestry per hinge discipline — spawn IS the runtime
//!    spawn action that fires Fate optical inference; the runtime-compiletime
//!    hinge composition must be structurally visible.
//!
//! **Enrichment discipline** (matches M6/M1/M2/M3 pattern):
//! - Regression guards: existing prism, ancestry, action signatures preserved
//! - Enrichment delta: `in @fate` added; stale Phase F quote removed/reformulated;
//!   @fate substrate-decl landing cited; M3 TICK 1 correction referenced
//!
//! **Interpretation B NOT enforced** — existing shard predates Interpretation B.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_peer_beam_shard() -> String {
    let path = repo_root().join("shards/mirror/peer/beam.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/mirror/peer/beam.mirror at {:?}: {}", path, e))
}

// === T1-T4: regression guards (existing substrate preserved) ===

#[test]
fn t01_spawn_family_root_declaration_preserved() {
    let content = read_peer_beam_shard();
    assert!(
        content.contains("prism @mirror/peer/beam"),
        "T1: `prism @mirror/peer/beam` family-root declaration must remain load-bearing (regression guard; renamed 2026-07-08 Tick 2 from `prism @mirror/spawn`)"
    );
}

#[test]
fn t02_spawn_existing_ancestry_preserved() {
    let content = read_peer_beam_shard();
    for req in [
        "in @prism",
        "in @mirror/cli",
        "in @mirror/mosaic",
        "in @mirror/pack",
        "in @pack",
        "in @peer",
        "in @spectral/supervisor",
    ] {
        assert!(
            content.contains(req),
            "T2: ancestry `{}` must remain declared (regression guard)",
            req
        );
    }
}

#[test]
fn t03_spawn_song_return_type_preserved() {
    let content = read_peer_beam_shard();
    // M2 TICK 1 landed spawn's return type as @song (or a @song species
    // carrier). Preserve.
    let has_song_return = content.contains("-> @song")
        || content.contains("-> song_voice")
        || content.contains("-> song_progression")
        || content.contains("-> song_movement")
        || content.contains("-> song_narrative")
        || content.contains("-> song_phrase");
    assert!(
        has_song_return,
        "T3: `spawn` action MUST retain @song return type (or species carrier) per M2 TICK 1 `63ea934` (regression guard)"
    );
}

#[test]
fn t04_spawn_in_song_ancestry_preserved() {
    let content = read_peer_beam_shard();
    assert!(
        content.contains("in @song"),
        "T4: `in @song` ancestry must remain (M2 TICK 1 regression guard; hinge to @song family)"
    );
}

// === T5-T8: @fate hinge composition (enrichment delta per M3 TICK 1 correction) ===

#[test]
fn t05_spawn_declares_in_fate_hinge_ancestry() {
    let content = read_peer_beam_shard();
    // Per M3 TICK 1 correction: species that touch the runtime hinge MUST
    // declare `in @fate`. Spawn IS the runtime spawn action that fires
    // Fate optical inference — quintessentially a hinge composer.
    let has_in_fate_ancestry = content.lines().any(|l| l.trim() == "in @fate");
    assert!(
        has_in_fate_ancestry,
        "T5: shard MUST declare `in @fate` at species altitude — the hinge composition binding. Spawn IS the runtime spawn action that fires Fate optical inference; the runtime-compiletime hinge must be structurally visible in the ancestry per M3 TICK 1 correction (Alex 2026-07-06: `@fate is the GLUE between runtime and compiletime`)."
    );
}

#[test]
fn t06_spawn_cites_fate_substrate_decl_landing() {
    let content = read_peer_beam_shard();
    // Cite the @fate substrate-decl LANDING that supersedes the stale
    // Phase F correction.
    let has_fate_citation = content.contains("shards/fate.mirror")
        || content.contains("shards/fate/tournament.mirror")
        || content.contains("@fate/tournament")
        || content.contains("fate.mirror")
        || content.contains("2026-06-30");
    assert!(
        has_fate_citation,
        "T6: shard MUST cite the @fate substrate-decl LANDING — `shards/fate.mirror` (LANDED 2026-06-30), `shards/fate/tournament.mirror` (LANDED 2026-06-30), `@fate/tournament`, or the 2026-06-30 landing date. Grounds `in @fate` composition in the actual substrate-decl'd ancestors."
    );
}

#[test]
fn t07_spawn_no_stale_phase_f_anti_pattern_claim() {
    let content = read_peer_beam_shard();
    // The stale Phase F quote ("@fate is runtime substrate, not a
    // substrate-decl prism") should be REMOVED or REFORMULATED. If the
    // shard still asserts this claim as current authority, that's the
    // stale-authority-quote-drift-recognition pattern.
    //
    // Accept: quote is removed OR reformulated with "stale" / "superseded"
    // / historical marker. Reject: quote asserted as current authority.
    let has_stale_pattern_asserted = content
        .contains("@fate is the runtime substrate, not a substrate-decl prism")
        || content.contains("@fate is runtime substrate, not a substrate-decl prism");
    // If the quote is present, it should ALSO have a stale/superseded marker
    let has_supersession_marker = content.contains("stale")
        || content.contains("STALE")
        || content.contains("superseded")
        || content.contains("SUPERSEDED")
        || content.contains("historical")
        || content.contains("pre-2026-06-30")
        || content.contains("outdated");
    assert!(
        !has_stale_pattern_asserted || has_supersession_marker,
        "T7: shard MUST NOT assert the stale Phase F claim (`@fate is the runtime substrate, not a substrate-decl prism`) as current authority. Either remove the quote OR mark it with `stale`/`superseded`/`historical` context per M3 TICK 1 correction. The @fate substrate-decl LANDED 2026-06-30 supersedes the Phase F correction."
    );
}

#[test]
fn t08_spawn_cites_m3_tick_1_correction_or_hinge_discipline() {
    let content = read_peer_beam_shard();
    // Cite the M3 TICK 1 correction context OR the hinge discipline
    // narrative that grounds `in @fate` composition.
    let has_hinge_narrative = content.contains("hinge")
        || content.contains("GLUE between runtime and compiletime")
        || content.contains("runtime and compiletime")
        || content.contains("M3 TICK 1")
        || content.contains("runtime-compiletime");
    assert!(
        has_hinge_narrative,
        "T8: shard MUST name the hinge discipline narrative — `hinge`, `GLUE between runtime and compiletime`, `runtime-compiletime`, or `M3 TICK 1` reference. Grounds the `in @fate` composition in the substrate-decl'd hinge discipline per Alex 2026-07-06 correction."
    );
}

// === T9-T10: preservation guards (M2 TICK 1 + peer-ACL) ===

#[test]
fn t09_spawn_preserves_recognition_ancestry_84_58_99() {
    let content = read_peer_beam_shard();
    let has_84 = content.contains("#84") || content.contains("@pack multi-repo");
    let has_58 = content.contains("#58")
        || content.contains("Fate IS optical inference")
        || content.contains("optical inference");
    let has_99 = content.contains("#99")
        || content.contains("lambda_0")
        || content.contains("λ₀")
        || content.contains("λ_0")
        || content.contains("mirror.spec IS");
    assert!(
        has_84 && has_58 && has_99,
        "T9: recognition ancestry (#84 @pack multi-repo; #58 Fate IS optical inference; #99 mirror.spec IS λ₀) must remain preserved (regression guard)"
    );
}

#[test]
fn t10_spawn_preserves_peer_acl_10_lead_semantics_and_spectral_supervisor() {
    let content = read_peer_beam_shard();
    let has_lead = content.contains("peer-ACL")
        || content.contains("lead")
        || content.contains("N+1 observer");
    let has_supervisor = content.contains("@spectral/supervisor")
        || content.contains("start_child")
        || content.contains("lifecycle");
    assert!(
        has_lead && has_supervisor,
        "T10: peer-ACL §10 lead semantics + @spectral/supervisor lifecycle composition must remain preserved (regression guard)"
    );
}
