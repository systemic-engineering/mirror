//! M2 TICK 2 RED — `shards/mirror/lens/cli/kintsugi.mirror` @spec → @song wire.
//!
//! Per collapse spec `docs/specs/mcp-spec-song-collapse.md` §10.2 + M2 TICK 1
//! (`b50fedc`) Seam Phase D observation: this tick provides the **second
//! witness** for the candidate recognition `cli-verbs ARE species-altitude
//! actions`.
//!
//! M2 TICK 1 established the first witness: `mirror spawn` IS `@song/movement.enter`
//! at cli altitude. This tick establishes the second: `mirror kintsugi @spec` IS
//! `@song/movement.close` at cli altitude (the closure event of the temporal-
//! bounded epoch that IS the spec-construction game). If two structurally clean
//! bindings land, the candidate promotes CANDIDATE → LANDED at Seam Phase D.
//!
//! **Composition chain**:
//! - MCP session (M1 `01443b3`) accumulates @spec via mq queries
//! - `mirror kintsugi @spec` (this tick) fires tournament; `settle` action
//!   ratifies @spec's `settle_on` conditions
//! - Ratified @spec OID passes to `@mirror/peer/beam.beam` (M2 TICK 1 `63ea934`; renamed 2026-07-08 Tick 2 from `@mirror/spawn.spawn`)
//! - `spawn` returns `@song` — the running peer's time-evolution operator
//!
//! **Enrichment discipline** (matches M6/M1/M2-TICK-1 pattern):
//! 4 regression guards + 11 enrichment items on existing 7.7KB shard.
//!
//! **Interpretation B NOT enforced** — existing shard predates Interpretation B;
//! enrichment preserves structure per M6 TICK 1 + M2 TICK 1 precedent.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_cli_kintsugi_shard() -> String {
    let path = repo_root().join("shards/mirror/lens/cli/kintsugi.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/mirror/lens/cli/kintsugi.mirror at {:?}: {}",
            path, e
        )
    })
}

// === T1-T4: regression guards (existing substrate preserved) ===

#[test]
fn t01_kintsugi_stage_declaration_preserved() {
    let content = read_cli_kintsugi_shard();
    assert!(
        content.contains("stage @mirror/lens/cli/kintsugi"),
        "T1: `stage @mirror/lens/cli/kintsugi` declaration must remain load-bearing (regression guard)"
    );
}

#[test]
fn t02_kintsugi_five_op_mapping_preserved() {
    let content = read_cli_kintsugi_shard();
    // Per cli-as-prism §5.2: focus/project/split/shift/settle map to
    // target/predicate/candidate/basis/iteration.
    for op_operand in [
        ("focus", "target"),
        ("project", "predicate"),
        ("split", "candidate"),
        ("shift", "basis"),
        ("settle", "iteration"),
    ] {
        assert!(
            content.contains(op_operand.0) && content.contains(op_operand.1),
            "T2: five-op mapping `{}` → `{}` must remain preserved (regression guard per cli-as-prism §5.2)",
            op_operand.0, op_operand.1
        );
    }
    assert!(
        content.contains("default focus"),
        "T2: `default focus` (peek-not-write cli discipline) must remain (regression guard)"
    );
}

#[test]
fn t03_kintsugi_existing_actions_preserved() {
    let content = read_cli_kintsugi_shard();
    for action in [
        "kintsugi(spec",
        "kintsugi_targeted(spec",
        "kintsugi_with_shatter(spec",
    ] {
        assert!(
            content.contains(action),
            "T3: existing action `{}` must remain preserved (regression guard; matches mirror.spec command declaration)",
            action
        );
    }
}

#[test]
fn t04_kintsugi_existing_ancestry_preserved() {
    let content = read_cli_kintsugi_shard();
    for req in [
        "in @prism",
        "in @optics",
        "in @glass",
        "in @mirror/lens",
        "in @mirror/lens/cli",
    ] {
        assert!(
            content.contains(req),
            "T4: existing ancestry `{}` must remain declared (regression guard)",
            req
        );
    }
}

// === T5-T7: @song wire (enrichment delta) ===

#[test]
fn t05_kintsugi_declares_in_song_ancestry() {
    let content = read_cli_kintsugi_shard();
    assert!(
        content.contains("in @song"),
        "T5: shard MUST declare `in @song` ancestry — kintsugi's settle action feeds into @mirror/peer/beam (renamed from @mirror/spawn Tick 2) which returns @song. Composition requires the song family binding."
    );
}

#[test]
fn t06_kintsugi_narrative_binds_song_movement_close() {
    let content = read_cli_kintsugi_shard();
    // The SECOND WITNESS: `mirror kintsugi @spec` IS `@song/movement.close`
    // at cli altitude — the closure event of the temporal-bounded epoch.
    let has_close_binding = content.contains("@song/movement")
        || content.contains("song/movement")
        || content.contains("movement.close")
        || content.contains("movement close")
        || content.contains("frame-close")
        || content.contains("frame close")
        || content.contains("closure event")
        || content.contains("epoch close");
    assert!(
        has_close_binding,
        "T6: shard narrative MUST bind kintsugi to `@song/movement.close` (or equivalent: `frame-close`, `closure event`, `epoch close`). This IS the second witness for the `cli-verbs ARE species-altitude actions` candidate (first witness: M2 TICK 1 `mirror spawn` IS `@song/movement.enter`)."
    );
}

#[test]
fn t07_kintsugi_narrative_names_spawn_composition() {
    let content = read_cli_kintsugi_shard();
    // The kintsugi → spawn → @song composition chain must be named.
    let has_spawn_binding = content.contains("@mirror/peer/beam")
        || content.contains("beam.beam")
        || content.contains("@mirror/spawn")
        || content.contains("spawn.spawn")
        || content.contains("mirror spawn")
        || content.contains("spawn a @song")
        || content.contains("spawn @song")
        || content.contains("spawns @song")
        || content.contains("spawns a @song");
    assert!(
        has_spawn_binding,
        "T7: shard narrative MUST name the @mirror/peer/beam composition (renamed 2026-07-08 Tick 2 from @mirror/spawn) — kintsugi's settle action feeds a ratified @spec OID into @mirror/peer/beam.beam which returns @song (M2 TICK 1 landed the return-type upgrade at `63ea934`)."
    );
}

// === T8-T11: collapse spec + M-cascade composition points ===

#[test]
fn t08_kintsugi_cites_collapse_spec() {
    let content = read_cli_kintsugi_shard();
    let has_binding = content.contains("mcp-spec-song-collapse")
        || content.contains("collapse spec")
        || content.contains("#S2")
        || content.contains("#S3")
        || content.contains("#S4");
    assert!(
        has_binding,
        "T8: shard MUST cite collapse spec `docs/specs/mcp-spec-song-collapse.md` OR one of its promotions (#S2/#S3/#S4). Links substrate-decl to the canonical spec that motivated the kintsugi → spawn → @song wire."
    );
}

#[test]
fn t09_kintsugi_names_cli_verbs_species_actions_candidate() {
    let content = read_cli_kintsugi_shard();
    // The candidate flagged at M2 TICK 1 Seam Phase D `b50fedc`. This shard
    // provides the second witness; narrative must acknowledge the candidate
    // structurally.
    let has_candidate = content.contains("cli-verbs")
        || content.contains("cli verbs")
        || content.contains("cli-verb")
        || content.contains("species-altitude action")
        || content.contains("species-altitude actions")
        || content.contains("second witness")
        || content.contains("cli-altitude specialization")
        || content.contains("cli altitude specialization");
    assert!(
        has_candidate,
        "T9: shard MUST name the `cli-verbs ARE species-altitude actions` candidate (flagged at M2 TICK 1 Seam Phase D `b50fedc`). This shard IS the second witness; narrative should mention the second-witness role structurally."
    );
}

#[test]
fn t10_kintsugi_cites_recognition_43() {
    let content = read_cli_kintsugi_shard();
    let has_43 = content.contains("#43")
        || content.contains("content-addressed build system")
        || content.contains("architecture-mirror-as-content-addressed");
    assert!(
        has_43,
        "T10: shard MUST cite Recognition #43 (mirror IS content-addressed build system; LANDED at M6 TICK 1 `884f433`). Kintsugi IS the fourth first-order consumer of the Apache-2.0 floor after: M6 self-declaration, M1 mcp_session, M2 spawn."
    );
}

#[test]
fn t11_kintsugi_cites_m_cascade_sibling_wiring() {
    let content = read_cli_kintsugi_shard();
    // The M-cascade context: M6 store enrichment, M1 mcp_session, M2 spawn.
    let has_m_cascade = (content.contains("@mirror/store") && (content.contains("@mirror/peer/beam") || content.contains("@mirror/spawn")))
        || (content.contains("mcp_session") || content.contains("MCP session"))
        || content.contains("M-cascade")
        || content.contains("M6")
        || content.contains("M1")
        || content.contains("M2");
    assert!(
        has_m_cascade,
        "T11: shard narrative MUST reference M-cascade siblings (@mirror/store + @mirror/peer/beam OR @spectral/gen_prism/mcp_session OR direct M6/M1/M2 references; @mirror/peer/beam renamed 2026-07-08 Tick 2 from @mirror/spawn). The wiring chain: session accumulates @spec → kintsugi settles it → beam produces @song."
    );
}

// === T12-T13: @spec composition + upgrade rationale ===

#[test]
fn t12_kintsugi_narrative_names_spec_from_session() {
    let content = read_cli_kintsugi_shard();
    // The @spec that kintsugi settles IS accumulated by the MCP session
    // gen_prism across mq queries. The shard should acknowledge this
    // provenance narrative.
    let has_spec_narrative = (content.contains("@spec")
        || content.contains("accumulated")
        || content.contains("mq queries")
        || content.contains("query trajectory"))
        && (content.contains("session")
            || content.contains("@spec")
            || content.contains("ratified")
            || content.contains("settle_on"));
    assert!(
        has_spec_narrative,
        "T12: shard narrative MUST name the @spec provenance — the @spec that kintsugi settles IS accumulated by the MCP session gen_prism across mq queries. When settle_on ratifies, spawn fires."
    );
}

#[test]
fn t13_kintsugi_names_upgrade_rationale() {
    let content = read_cli_kintsugi_shard();
    // The upgrade rationale: pre-@song, kintsugi returned imperfect(settled-
    // graph-ref); post-@song, kintsugi's settle action feeds into spawn which
    // returns @song. The shard should name this rationale.
    let has_rationale = content.contains("2026-07-06")
        || content.contains("post-@song")
        || content.contains("post @song")
        || content.contains("pre-@song")
        || content.contains("pre @song")
        || content.contains("Arc 6")
        || content.contains("@song landing")
        || content.contains("@song landed")
        || content.contains("return-type");
    assert!(
        has_rationale,
        "T13: shard MUST name the upgrade rationale narratively — `2026-07-06`, `pre-@song`/`post-@song`, `Arc 6`, `@song landing`, or `return-type` transition. Substrate-honest witness that the @song wire post-dates the shard's 2026-06-12 landing."
    );
}

// === T14-T15: preservation + kintsugi.settle IS write discipline ===

#[test]
fn t14_kintsugi_preserves_ashby_variety_maintenance_narrative() {
    let content = read_cli_kintsugi_shard();
    // Per architecture-kintsugi-variety-io: @fate tournament IS variety-
    // maintenance; kintsugi.settle drives one round.
    let has_variety =
        content.contains("variety") || content.contains("Ashby") || content.contains("tournament");
    assert!(
        has_variety,
        "T14: shard narrative MUST preserve Ashby variety-maintenance framing (regression guard per architecture-kintsugi-variety-io). Existing shard names `variety-maintenance` + `tournament` extensively; this preserves it."
    );
}

#[test]
fn t15_kintsugi_settle_remains_the_one_write() {
    let content = read_cli_kintsugi_shard();
    // Per existing shard: `settle` IS the ONE write per invocation.
    // Reading (focus) is free; settle is the tournament iteration.
    let has_settle_discipline = (content.contains("the ONE write")
        || content.contains("one write")
        || content.contains("the ONE"))
        || (content.contains("settle")
            && (content.contains("iteration") || content.contains("write")));
    assert!(
        has_settle_discipline,
        "T15: shard narrative MUST preserve `settle IS the ONE write per invocation` discipline (regression guard per cli-as-prism §2.1 + existing shard §5-op mapping section)."
    );
}
