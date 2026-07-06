//! M1 TICK 1 RED — `shards/spectral/gen_prism/mcp_session.mirror` species.
//!
//! Per collapse spec `docs/specs/mcp-spec-song-collapse.md` §10.1 (Mara `2cfd2a7`):
//! **M1 wires `bootstrap/src/mcp.rs`'s stateless request/reply into a
//! session-state-machine.** State lives in `@mirror/store`, NOT in-process.
//!
//! Per spec §3.6: MCP is stateless AT THE PROCESS LAYER because it is stateful
//! AT THE STORE LAYER. The session ref (`refs/gen_prism/mcp/<session-uuid>`) is
//! a content-addressed pointer into the store; each request reads the head
//! crystal, applies the tick, writes the new crystal, and CAS-advances the ref.
//! The Rust MCP handler holds only the session identifier; the accumulated
//! `@spec`, the query trajectory, and the Hilbert dimension all live as
//! content-addressed DAG nodes in `@mirror/store`.
//!
//! **Substrate-canonical placement**: @spectral/gen_prism/mcp_session (species
//! under @spectral/gen_prism, canonical per Taut scout finding). NOT
//! `@mirror/runtime/mcp_session` (collapse spec §3.5 grammar name is superseded
//! by substrate-canonical location per `feedback-substrate-already-had-the-word`).
//! Interpretation B canonical DOES apply (green-field species).
//!
//! **Species contract**:
//! - `glass @spectral/gen_prism/mcp_session` (specialization of gen_prism per
//!   glass discipline; NOT a new keyword)
//! - Inherits @prism, @glass, @meta, @spectral, @uuid/spectral, @mirror/store
//! - State surface: `shard_ref` at accumulated @spec crystal (per gen_prism
//!   state-surface discipline)
//! - Tool surface: mq five-op query surface specialized for MCP JSON-RPC wire
//! - Session ref path: `refs/gen_prism/mcp/<session-uuid>` per spec §3.6
//! - Tick semantics: read head crystal → apply mq query → write new crystal →
//!   CAS-advance ref
//! - Ancestor-chain IS the query trajectory (replay + audit)
//! - Session state survives daemon restart via ref persistence in @mirror/store

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_mcp_session_shard() -> String {
    let path = repo_root().join("shards/spectral/gen_prism/mcp_session.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/spectral/gen_prism/mcp_session.mirror at {:?}: {}",
            path, e
        )
    })
}

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

// === T1-T5: canonical shape + Interpretation B + species pact ===

#[test]
fn t01_mcp_session_shard_declares_glass_species_per_path_pact() {
    let content = read_mcp_session_shard();
    assert!(
        content.contains("glass @spectral/gen_prism/mcp_session")
            || content.contains("glass @spectral / gen_prism / mcp_session"),
        "T1: shards/spectral/gen_prism/mcp_session.mirror MUST declare `glass @spectral/gen_prism/mcp_session` per path-namespace pact + gen_prism species discipline (glass NOT new keyword; per recognition #46 + shards/spectral.mirror discipline)."
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_mcp_session_shard();
    let first = first_nonempty_line(&content).expect("T2: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T2: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T2: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T2: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t03_exactly_one_seam_at_column_zero_and_in_clauses_below() {
    let content = read_mcp_session_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T3: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
    let seam_idx = seams[0];
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T3: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t04_mcp_session_inherits_universal_transparency_ancestry() {
    let content = read_mcp_session_shard();
    for req in ["in @prism", "in @glass", "in @meta"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_mcp_session_inherits_spectral_gen_prism_and_mirror_store() {
    let content = read_mcp_session_shard();
    for req in ["in @spectral", "in @mirror/store", "in @uuid/spectral"] {
        assert!(
            content.contains(req),
            "T5: must inherit `{}` — gen_prism state surface (@mirror/store), identity carrier (@uuid/spectral), family root (@spectral)",
            req
        );
    }
}

// === T6-T8: session ref + state discipline (state lives in @mirror/store) ===

#[test]
fn t06_mcp_session_declares_session_ref_pattern() {
    let content = read_mcp_session_shard();
    let has_ref_pattern = content.contains("refs/gen_prism/mcp")
        || content.contains("session-uuid")
        || content.contains("session_uuid")
        || content.contains("session ref")
        || content.contains("session_ref");
    assert!(
        has_ref_pattern,
        "T6: species narrative MUST declare the session ref pattern (`refs/gen_prism/mcp/<session-uuid>` or equivalent). Per collapse spec §3.6: content-addressed pointer into @mirror/store; the Rust MCP handler holds only the session identifier."
    );
}

#[test]
fn t07_mcp_session_grounds_state_lives_in_store_discipline() {
    let content = read_mcp_session_shard();
    let has_store_state = (content.contains("state")
        || content.contains("State"))
        && (content.contains("@mirror/store")
            || content.contains("in the store")
            || content.contains("content-addressed")
            || content.contains("crystal"));
    assert!(
        has_store_state,
        "T7: species narrative MUST ground `state lives in @mirror/store` discipline. Per collapse spec §3.6: MCP is stateless at process layer BECAUSE stateful at store layer. Session state = crystals in @mirror/store; NOT in-process Rust."
    );
}

#[test]
fn t08_mcp_session_declares_read_apply_write_cas_tick_semantics() {
    let content = read_mcp_session_shard();
    // The tick discipline: read head crystal → apply tick → write new crystal
    // → CAS-advance ref. Various narrative forms acceptable.
    let has_tick = (content.contains("read") || content.contains("Read"))
        && (content.contains("tick") || content.contains("apply"))
        && (content.contains("CAS") || content.contains("advance") || content.contains("advances"));
    assert!(
        has_tick,
        "T8: species narrative MUST declare tick semantics (read head crystal → apply tick → write new crystal → CAS-advance ref) per collapse spec §3.6 + §10.1. The advance-by-CAS pattern IS the substrate's session-mutation primitive."
    );
}

// === T9-T10: mq surface + accumulator discipline (session builds @spec) ===

#[test]
fn t09_mcp_session_composes_with_mq_query_surface() {
    let content = read_mcp_session_shard();
    let has_mq = content.contains("mq")
        || content.contains("@code/mq")
        || content.contains("query")
        || content.contains("queries");
    assert!(
        has_mq,
        "T9: species narrative MUST cite mq (mirror query language) as tool-surface composition. Per collapse spec §3.2: each mq query = one substrate-pull dimension expansion; session accumulates @spec via mq trajectory."
    );
}

#[test]
fn t10_mcp_session_grounds_accumulator_builds_spec() {
    let content = read_mcp_session_shard();
    let has_accumulator = (content.contains("accumulat") || content.contains("builds"))
        && (content.contains("@spec")
            || content.contains("spec")
            || content.contains("@mirror/spec"));
    assert!(
        has_accumulator,
        "T10: species narrative MUST ground the accumulator discipline (session accumulates an @spec across queries). Per collapse spec §3.3: after N queries, session state IS a partial @spec; when settle_on ratifies, mirror kintsugi @spec spawns @song."
    );
}

// === T11-T12: agentic value-adds at MCP session altitude ===

#[test]
fn t11_mcp_session_grounds_session_persistence_across_restart() {
    let content = read_mcp_session_shard();
    let has_persistence = (content.contains("restart") || content.contains("survive"))
        || content.contains("persistence")
        || content.contains("process boundary");
    assert!(
        has_persistence,
        "T11: species narrative MUST ground `session persistence` agentic value-add per collapse spec §11.6.3. Session state SURVIVES daemon restart because it lives in @mirror/store, not in-process Rust. This IS the M6 floor's first-order capability specialized at MCP altitude."
    );
}

#[test]
fn t12_mcp_session_grounds_replay_via_ancestor_chain() {
    let content = read_mcp_session_shard();
    let has_replay = (content.contains("replay")
        || content.contains("trajectory")
        || content.contains("ancestor")
        || content.contains("audit"))
        && (content.contains("chain") || content.contains("history") || content.contains("query"));
    assert!(
        has_replay,
        "T12: species narrative MUST ground the query trajectory / ancestor chain discipline. Per collapse spec §6.4: session ancestor chain IS the query trajectory for replay + audit. Composes with M6 floor's provenance-chain value-add at MCP altitude."
    );
}

// === T13-T14: prior-art + business model witnesses ===

#[test]
fn t13_mcp_session_cites_beam_gen_server_or_nix_daemon_prior_art() {
    let content = read_mcp_session_shard();
    let has_prior_art = content.contains("gen_server")
        || content.contains("BEAM")
        || content.contains("OTP")
        || content.contains("Erlang")
        || content.contains("nix-daemon")
        || content.contains("Nix daemon");
    assert!(
        has_prior_art,
        "T13: species narrative MUST cite prior-art ancestor — BEAM/OTP gen_server (analogue at @spectral/gen_prism family root) or nix-daemon (persistent session via socket + store). Grounds the session-state-machine discipline in existing canonical patterns."
    );
}

#[test]
fn t14_mcp_session_cites_recognition_or_collapse_spec_binding() {
    let content = read_mcp_session_shard();
    let has_binding = content.contains("mcp-spec-song-collapse")
        || content.contains("#99")
        || content.contains("#S3")
        || content.contains("#S4")
        || content.contains("lambda_0")
        || content.contains("λ₀")
        || content.contains("MCP session IS gen_prism");
    assert!(
        has_binding,
        "T14: species narrative MUST cite collapse spec `docs/specs/mcp-spec-song-collapse.md` OR relevant recognition (#99 λ₀ ground state / MCP session IS gen_prism candidate per §9.3). Links substrate-decl to the canonical spec that framed M1."
    );
}

// === T15: gen_prism family-root inheritance + tool-surface discipline ===

#[test]
fn t15_mcp_session_inherits_five_op_tool_surface_from_gen_prism() {
    let content = read_mcp_session_shard();
    // Gen_prism species inherit the five-op tool surface as the wire protocol.
    // Per @spectral/gen_prism docblock: "The five-op block IS the tool surface
    // (the wire protocol); external callers speak to a gen_prism through its
    // five-op surface."
    let has_tool_surface = (content.contains("tool surface")
        || content.contains("tool-surface")
        || content.contains("five-op")
        || content.contains("five op"))
        || (content.contains("focus")
            && content.contains("project")
            && content.contains("split"));
    assert!(
        has_tool_surface,
        "T15: species narrative MUST cite the five-op tool surface discipline inherited from @spectral/gen_prism. Per @spectral/gen_prism docblock: `The five-op block IS the tool surface; external callers speak through it`. MCP session's tool surface IS the mq wire specialized for JSON-RPC."
    );
}
