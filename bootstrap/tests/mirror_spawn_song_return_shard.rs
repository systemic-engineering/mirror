//! M2 TICK 1 RED — `shards/mirror/spawn.mirror` return type upgrade.
//!
//! Per collapse spec `docs/specs/mcp-spec-song-collapse.md` §10 (Mara `2cfd2a7`)
//! + Taut composition-points scout (`shards/mirror/spawn.mirror` predates @song
//! by 11 days; 2026-06-25 vs @song landing 2026-07-06):
//!
//! **@mirror/spawn return type upgrade**: opaque `runtime` → typed `@song`. The
//! spawned peer IS a `@song` — the peer's temporal-progression trajectory through
//! their spec's state space (per collapse spec §5.1: `@song = spec's time-
//! evolution operator applied to a peer`).
//!
//! **The recognition**: `mirror spawn ~peer'<home>'` IS `@song/movement.enter`
//! at cli altitude — the frame-entry action of a temporal-bounded epoch at
//! runtime. Before @song, spawn returned opaque `runtime` because there was no
//! typed temporal frame to name the spawned peer's trajectory. Arc 6 close
//! (`eb50a61`, @song/movement `4efbf16`) provides the type; this tick binds
//! spawn to it.
//!
//! **Enrichment discipline** (matches M6 TICK 1 pattern):
//! - 5 regression guards: existing prism @mirror/spawn family-root, ancestry,
//!   mirror_spawn_request type, peer_well_known predicate, recognition ancestry
//! - 10 enrichment items: `in @song` ancestry; return type @song; song/movement
//!   composition; song/voice binding; collapse spec citation; @spectral/gen_prism/
//!   mcp_session (M1 LANDED); Recognition #43 first-order consumer citation;
//!   Taut-flagged type-mismatch resolution narrative; preserve peer-ACL §10;
//!   preserve @spectral/supervisor composition
//!
//! **Interpretation B NOT enforced** — existing shard predates Interpretation B
//! ratification (Arc 4 sub-arc A close 2026-07-06); enrichment preserves
//! existing structure per M6 TICK 1 precedent.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_spawn_shard() -> String {
    let path = repo_root().join("shards/mirror/spawn.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/mirror/spawn.mirror at {:?}: {}", path, e))
}

// === T1-T5: regression guards (existing substrate must remain) ===

#[test]
fn t01_spawn_family_root_declaration_present() {
    let content = read_spawn_shard();
    assert!(
        content.contains("prism @mirror/spawn"),
        "T1: `prism @mirror/spawn` family-root declaration must remain load-bearing (regression guard)"
    );
}

#[test]
fn t02_spawn_existing_ancestry_preserved() {
    let content = read_spawn_shard();
    // Existing ancestry from 2026-06-25 landing
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
fn t03_spawn_mirror_spawn_request_type_preserved() {
    let content = read_spawn_shard();
    assert!(
        content.contains("type mirror_spawn_request") || content.contains("mirror_spawn_request = {"),
        "T3: `mirror_spawn_request` type declaration must remain (regression guard). Two-field record with target: peer + options: ref per @peer.load composition."
    );
}

#[test]
fn t04_spawn_peer_well_known_predicate_preserved() {
    let content = read_spawn_shard();
    assert!(
        content.contains("peer_well_known"),
        "T4: `peer_well_known` sub-bilateral predicate must remain (regression guard; #53 family lifted to `requires` clause per 2026-06-25 landing)."
    );
}

#[test]
fn t05_spawn_recognition_ancestry_84_58_99_preserved() {
    let content = read_spawn_shard();
    let has_84 = content.contains("#84") || content.contains("@pack multi-repo");
    let has_58 = content.contains("#58") || content.contains("Fate IS optical inference") || content.contains("optical inference");
    let has_99 = content.contains("#99") || content.contains("lambda_0") || content.contains("λ₀") || content.contains("λ_0") || content.contains("mirror.spec IS");
    assert!(
        has_84 && has_58 && has_99,
        "T5: existing recognition ancestry (#84 @pack multi-repo agent runtime; #58 Fate IS optical inference; #99 mirror.spec IS λ₀ excitation) must remain preserved (regression guard)."
    );
}

// === T6-T9: @song wire (enrichment delta) ===

#[test]
fn t06_spawn_declares_in_song_ancestry() {
    let content = read_spawn_shard();
    assert!(
        content.contains("in @song"),
        "T6: shard MUST declare `in @song` ancestry — spawn's return type upgrade to @song requires the family binding. Arc 6 (`eb50a61`) landed @song family root + 5 species; spawn now composes."
    );
}

#[test]
fn t07_spawn_return_type_is_song_not_runtime() {
    let content = read_spawn_shard();
    // Return-type upgrade: opaque `runtime` → typed `@song` (or a song-
    // family carrier: song_voice / song_progression / song_movement).
    // The signature line `spawn(r: mirror_spawn_request, p: perturbation) -> ?`
    // must have @song or song_* as the return type.
    let has_song_return = content.contains("-> @song")
        || content.contains("-> song_voice")
        || content.contains("-> song_progression")
        || content.contains("-> song_movement")
        || content.contains("-> song_narrative")
        || content.contains("-> song_phrase");
    assert!(
        has_song_return,
        "T7: `spawn` action MUST return @song (or a @song species carrier: song_voice / song_progression / song_movement / song_narrative / song_phrase). Currently returns opaque `runtime` — Taut flagged this as an 11-day-old type mismatch predating @song landing."
    );
}

#[test]
fn t08_spawn_narrative_binds_song_movement_enter() {
    let content = read_spawn_shard();
    // spawn IS `@song/movement.enter` at cli altitude — the frame-entry
    // action of a temporal-bounded epoch at runtime.
    let has_movement_binding = content.contains("@song/movement")
        || content.contains("song/movement")
        || content.contains("movement.enter")
        || content.contains("movement enter")
        || content.contains("frame-entry")
        || content.contains("frame entry")
        || content.contains("bounded epoch");
    assert!(
        has_movement_binding,
        "T8: shard narrative MUST bind spawn to `@song/movement.enter` (or equivalent framing: `frame-entry`, `bounded epoch`). Per collapse spec structural claim: `mirror spawn ~peer'<home>'` IS `@song/movement.enter` at cli altitude."
    );
}

#[test]
fn t09_spawn_cites_song_voice_binding_for_spawned_peer() {
    let content = read_spawn_shard();
    // The spawned peer IS a @song/voice — the *when* on top of
    // @mirror/spectral/voice's *what/who*. This is the peer-as-actor
    // binding at temporal altitude.
    let has_voice_binding = content.contains("@song/voice")
        || content.contains("song/voice")
        || content.contains("song_voice")
        || content.contains("actor at temporal")
        || content.contains("peer as actor")
        || content.contains("time-indexed trajectory");
    assert!(
        has_voice_binding,
        "T9: shard narrative MUST cite `@song/voice` binding — the spawned peer IS a song_voice (their time-indexed trajectory through authored sections). Composes with @mirror/spectral/voice at orchestra altitude."
    );
}

// === T10-T13: collapse spec + M1/M6 composition points ===

#[test]
fn t10_spawn_cites_collapse_spec_or_song_promotions() {
    let content = read_spawn_shard();
    let has_binding = content.contains("mcp-spec-song-collapse")
        || content.contains("collapse spec")
        || content.contains("#S2")
        || content.contains("#S3")
        || content.contains("#S4");
    assert!(
        has_binding,
        "T10: shard MUST cite collapse spec `docs/specs/mcp-spec-song-collapse.md` OR one of its promotions (#S2 shift-at-temporal, #S3 five-op temporal, #S4 cascade-shape altitude-portable — all LANDED). Links substrate-decl to the canonical spec that motivated the return-type upgrade."
    );
}

#[test]
fn t11_spawn_cites_mcp_session_gen_prism_as_related_species() {
    let content = read_spawn_shard();
    let has_mcp_session = content.contains("@spectral/gen_prism/mcp_session")
        || content.contains("mcp_session")
        || content.contains("MCP session")
        || content.contains("gen_prism");
    assert!(
        has_mcp_session,
        "T11: shard narrative MUST cite `@spectral/gen_prism/mcp_session` OR `gen_prism` — M1 LANDED at `01443b3` (`shards/spectral/gen_prism/mcp_session.mirror`). The MCP session gen_prism IS the state machine that spawns @song via `kintsugi @spec`; spawn is the sibling substrate that consumes the ratified @spec into a running peer."
    );
}

#[test]
fn t12_spawn_cites_recognition_43_first_order_consumer() {
    let content = read_spawn_shard();
    let has_43 = content.contains("#43")
        || content.contains("content-addressed build system")
        || content.contains("architecture-mirror-as-content-addressed");
    assert!(
        has_43,
        "T12: shard MUST cite Recognition #43 (mirror IS content-addressed build system; LANDED at M6 TICK 1 `884f433`; empirically substrate-fact per M1 TICK 1 `8eac1de` Seam Phase D). Spawn IS the second first-order consumer of the Apache-2.0 floor."
    );
}

#[test]
fn t13_spawn_names_type_mismatch_resolution_narrative() {
    let content = read_spawn_shard();
    // Taut flagged spawn's return type `runtime` as an 11-day-old
    // mismatch predating @song. The shard should acknowledge the
    // resolution (either narratively naming the upgrade or citing
    // Taut's composition-points scout).
    let has_resolution = content.contains("return type upgrade")
        || content.contains("return-type upgrade")
        || content.contains("predated @song")
        || content.contains("predates @song")
        || content.contains("Arc 6")
        || content.contains("@song landing")
        || content.contains("typed temporal frame")
        || content.contains("temporal-progression trajectory");
    assert!(
        has_resolution,
        "T13: shard narrative MUST name the return-type upgrade rationale — `return type upgrade`, `predates @song`, `Arc 6`, `typed temporal frame`, or `temporal-progression trajectory`. This is the substrate-honest witness that the type was previously opaque because @song didn't exist yet; now it does, so the type upgrades."
    );
}

// === T14-T15: preservation guards ===

#[test]
fn t14_spawn_preserves_peer_acl_10_lead_semantics() {
    let content = read_spawn_shard();
    let has_lead = content.contains("peer-ACL")
        || content.contains("peer ACL")
        || content.contains("lead")
        || content.contains("N+1 observer");
    assert!(
        has_lead,
        "T14: peer-ACL §10 lead semantics (spawn-and-probe relation; N+1 observer) must remain preserved in narrative (regression guard). The spawn side is what @mirror/spawn discharges; the lead's ongoing spectral-Tomm-probe handling is the runtime side (unchanged)."
    );
}

#[test]
fn t15_spawn_preserves_spectral_supervisor_lifecycle_composition() {
    let content = read_spawn_shard();
    let has_supervisor = content.contains("@spectral/supervisor")
        || content.contains("spectral/supervisor")
        || content.contains("start_child")
        || content.contains("restart_strategy")
        || content.contains("lifecycle");
    assert!(
        has_supervisor,
        "T15: @spectral/supervisor lifecycle composition (spawn KICKS through start_child; restart_strategy owned by supervisor) must remain preserved (regression guard per peer-ACL §2.4 substrate landing 2026-06-25)."
    );
}
