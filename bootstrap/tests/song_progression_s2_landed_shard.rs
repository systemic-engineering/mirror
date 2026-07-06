//! M3 TICK 1 RED — `shards/song/progression.mirror` #S2 status update to LANDED.
//!
//! **Substrate reality** (Reed's substrate-pull check):
//! - The shard already grounds #S2 extensively via `advance → @kintsugi/shift`
//!   composition at species altitude (as designed).
//! - The shard does NOT currently cite `@fate` — correctly, per Taut's
//!   Phase F anti-pattern correction 2026-06-24: `@fate` is runtime substrate,
//!   NOT substrate-decl prism. `in @fate` at species altitude is anti-pattern.
//! - The shard's #S2 status is CANDIDATE (per pre-collapse-spec labeling).
//!
//! **The collapse spec** (Mara `2cfd2a7` §5, §9.2) PROMOTED #S2 CANDIDATE → LANDED
//! via Fate multi-frequency tournament grounding through #58 (Fate IS optical
//! inference; LANDED). The empirical mechanism: at each temporal step, Fate
//! decomposes state at multiple harmonic frequencies, tournament picks the
//! winning frequency — that winning-frequency's move IS the `shift` at temporal
//! altitude. The wiring composes transitively: progression `advance` →
//! `@kintsugi/shift.shift` → Fate multi-frequency (runtime altitude, #58).
//!
//! **This tick**: narrative-only enrichment updating progression.mirror's #S2
//! labeling from CANDIDATE to LANDED. Cite collapse spec + Fate multi-frequency
//! composition path transitively. Do NOT add `in @fate` (respects Phase F).
//!
//! **Enrichment discipline** (matches M6/M1/M2 pattern):
//! - Regression guards: existing progression.mirror substrate preserved
//!   (species declaration, ancestry, actions, sub-predicates, prior-art anchors)
//! - Enrichment delta: #S2 LANDED status; Fate multi-frequency citation;
//!   Recognition #58 composition path; collapse spec `2cfd2a7` citation
//!
//! **Interpretation B applies** (green-field species per Arc 6; DIFFERENT
//! from M6/M1/M2 whose shards predate Interpretation B).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_progression_shard() -> String {
    let path = repo_root().join("shards/song/progression.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/song/progression.mirror at {:?}: {}", path, e))
}

// === T1-T5: regression guards (existing substrate preserved) ===

#[test]
fn t01_progression_species_declaration_preserved() {
    let content = read_progression_shard();
    assert!(
        content.contains("prism @song/progression"),
        "T1: `prism @song/progression` species declaration must remain load-bearing (regression guard from Arc 6 TICK 2 `54ff1e8`)"
    );
}

#[test]
fn t02_progression_kintsugi_shift_composition_preserved() {
    let content = read_progression_shard();
    assert!(
        content.contains("@kintsugi/shift"),
        "T2: `@kintsugi/shift` composition MUST remain (regression guard; the shift-at-temporal witness ancestor at cross-altitude morphism altitude)"
    );
}

#[test]
fn t03_progression_advance_action_preserved() {
    let content = read_progression_shard();
    assert!(
        content.contains("advance("),
        "T3: `advance` action MUST remain declared (regression guard; the #S2 species-altitude witness action)"
    );
}

#[test]
fn t04_progression_sub_predicates_preserved() {
    let content = read_progression_shard();
    for pred in [
        "progression_directed_toward_cadence",
        "cadence_authentic_or_plagal",
    ] {
        assert!(
            content.contains(pred),
            "T4: `{}` sub-predicate MUST remain (regression guard; 2 of 3 song_settles composed bilateral)",
            pred
        );
    }
}

#[test]
fn t05_progression_declares_in_fate_hinge_composition() {
    let content = read_progression_shard();
    // **Alex correction 2026-07-06**: the Phase F correction Reed inherited
    // from `shards/mirror/spawn.mirror`'s docblock (Taut 2026-06-24: "@fate
    // is runtime substrate, not substrate-decl prism") is STALE. Substrate
    // reality: `shards/fate.mirror` (42.1KB, LANDED 2026-06-30) declares
    // `prism @fate`; `shards/fate/tournament.mirror` (41.5KB, LANDED
    // 2026-06-30) declares `prism @fate/tournament`. Both landed SIX DAYS
    // AFTER Taut's Phase F correction and SUPERSEDE it.
    //
    // @fate IS the hinge between compiletime (@song, @kintsugi/shift,
    // @mirror/store — typed declarations) and runtime (peer processes,
    // tournament, spawn output). `in @fate` at species altitude COMPOSES
    // with that substrate-decl — that's the hinge binding, not an
    // anti-pattern. Alex's exact words: *"If @fate is not defined it
    // needs to be defined. It's the GLUE between runtime and compiletime.
    // You're omitting the hinge right now."*
    //
    // Recorded in [[feedback-verify-substrate-freshness-of-flagged-
    // corrections]]: docblock decays; substrate reality authoritative.
    let has_in_fate_ancestry = content.lines().any(|l| l.trim() == "in @fate");
    assert!(
        has_in_fate_ancestry,
        "T5 (REVERSED per Alex correction 2026-07-06): shard MUST declare `in @fate` at species altitude — the hinge composition binding. @fate is substrate-decl'd at shards/fate.mirror (LANDED 2026-06-30) which SUPERSEDES Taut's stale Phase F correction (2026-06-24). Progression composes with @fate at temporal altitude by declaring `in @fate`; this IS the hinge between compiletime and runtime."
    );
}

#[test]
fn t05b_progression_cites_fate_substrate_decl_landing() {
    let content = read_progression_shard();
    // Explicit citation of the @fate substrate-decl landings that supersede
    // Taut's Phase F correction. Either shard reference works.
    let has_fate_shard_citation = content.contains("shards/fate.mirror")
        || content.contains("shards/fate/tournament.mirror")
        || content.contains("@fate/tournament")
        || content.contains("fate.mirror")
        || content.contains("prism @fate");
    assert!(
        has_fate_shard_citation,
        "T5b (new per Alex correction 2026-07-06): shard MUST cite the @fate substrate-decl LANDING — either `shards/fate.mirror` (LANDED 2026-06-30, 42.1KB `prism @fate`), `shards/fate/tournament.mirror` (LANDED 2026-06-30, 41.5KB `prism @fate/tournament`), `@fate/tournament`, or `prism @fate`. Grounds `in @fate` composition (T5) in the actual substrate-decl'd ancestor, not just runtime narrative."
    );
}

// === T6-T10: #S2 LANDED status + Fate multi-frequency citation (enrichment delta) ===

#[test]
fn t06_progression_narrative_marks_s2_landed() {
    let content = read_progression_shard();
    // Per collapse spec `2cfd2a7` §5, §9.2: #S2 PROMOTED to LANDED via Fate
    // multi-frequency tournament grounding through #58. Shard narrative
    // should reflect the LANDED status (was CANDIDATE pre-collapse-spec).
    let has_s2_landed = content.contains("#S2 LANDED")
        || content.contains("#S2 (LANDED")
        || content.contains("#S2 promoted")
        || content.contains("#S2 PROMOTED")
        || content.contains("S2 LANDED");
    assert!(
        has_s2_landed,
        "T6: shard narrative MUST mark #S2 status as LANDED (per collapse spec `2cfd2a7` §5 + §9.2 promotion). Currently shard cites #S2 as CANDIDATE; enrichment updates to LANDED."
    );
}

#[test]
fn t07_progression_cites_fate_multi_frequency() {
    let content = read_progression_shard();
    let has_fate =
        (content.contains("@fate") || content.contains("Fate") || content.contains("fate"))
            && (content.contains("multi-frequency")
                || content.contains("multi frequency")
                || content.contains("tournament")
                || content.contains("optical inference"));
    assert!(
        has_fate,
        "T7: shard narrative MUST cite Fate multi-frequency tournament (OR `tournament` OR `optical inference` composition). Per collapse spec §5: Fate's multi-frequency decomposition IS the mechanism grounding #S2 at temporal altitude — the winning-frequency move IS the shift."
    );
}

#[test]
fn t08_progression_cites_recognition_58() {
    let content = read_progression_shard();
    let has_58 = content.contains("#58")
        || content.contains("Recognition #58")
        || content.contains("Fate IS optical inference")
        || content.contains("architecture-fate-is-optical-inference");
    assert!(
        has_58,
        "T8: shard narrative MUST cite Recognition #58 (Fate IS optical inference; LANDED). This IS the ancestor recognition that grounds #S2 through the composition path: progression `advance` → @kintsugi/shift → Fate optical inference (#58)."
    );
}

#[test]
fn t09_progression_cites_collapse_spec() {
    let content = read_progression_shard();
    let has_collapse =
        content.contains("mcp-spec-song-collapse") || content.contains("collapse spec");
    assert!(
        has_collapse,
        "T9: shard MUST cite collapse spec `docs/specs/mcp-spec-song-collapse.md` — the canonical spec that promoted #S2 CANDIDATE → LANDED at Mara `2cfd2a7`. Links substrate-decl to promotion authority."
    );
}

#[test]
fn t10_progression_narrative_names_transitive_composition_path() {
    let content = read_progression_shard();
    // The composition chain: progression `advance` → `@kintsugi/shift` →
    // Fate multi-frequency (runtime substrate, #58). Narrative should
    // name the transitive composition explicitly to make the substrate-
    // honest wiring visible without violating Phase F.
    let has_transitive = (content.contains("transitive")
        || content.contains("composition path")
        || content.contains("composition chain")
        || content.contains("runtime substrate")
        || content.contains("runtime altitude"))
        && (content.contains("advance") || content.contains("@kintsugi/shift"));
    assert!(
        has_transitive,
        "T10: shard narrative MUST name the transitive composition path (progression advance → @kintsugi/shift → Fate optical inference) OR `runtime substrate` framing that makes the @fate composition visible without `in @fate` anti-pattern."
    );
}
