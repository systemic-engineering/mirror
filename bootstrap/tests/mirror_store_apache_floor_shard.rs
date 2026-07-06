//! M6 TICK 1 RED — `shards/mirror/store.mirror` Apache-2.0 rock-solid floor enrichment.
//!
//! **Substrate-pull-honest context**: `@mirror/store` is NOT green-field. The family-root shard
//! at `shards/mirror/store.mirror` (10.7KB, 2026-06-30) already declares:
//! - `prism @mirror/store` five-op family-root
//! - `type oid = ref` content-addressed identity
//! - `type splinter_graph = { root: oid, children: [oid] }` (composite; canonical name for the
//!   git-tree-analog trichotomy element; DOES NOT collide with `@mirror` family root)
//! - `type bytes = ref(scalar/bytes)` raw payload
//! - `type shard_ref = uuid_spectral` typed handle
//! - Six-op canonical surface: `read / write / exists / diff / walk / verify`
//! - Species roster: git (LANDED), mem/s3/oci (forward-promised)
//! - `@mirror/store/crystal` species LANDED at `shards/mirror/store/crystal.mirror` (2026-06-16)
//!
//! **The M6 enrichment (this RED)** adds Apache-2.0 floor discipline per collapse spec
//! `docs/specs/mcp-spec-song-collapse.md` (Mara `a6bb25c` + `2cfd2a7`) §11:
//! - Immutable-under-hash invariant declared EXPLICITLY (not just implicit in oid semantics)
//! - Purely-functional composition invariant declared
//! - Bazel REAPI floor decomposition (CAS + action-cache split) named
//! - Projection API `oid ↔ working-tree` formalized (git-object-model analog)
//! - Recognition #43 explicit citation (mirror IS content-addressed build system)
//! - Prior-art anchors: Dolstra 2006 PhD; Mokhov/Peyton Jones "Build Systems à la Carte" JFP 2020;
//!   IPFS Merkle DAG canon
//! - §11.6 agentic value-adds framing at substrate altitude (rock-solid floor emphasis;
//!   "not a consolation prize")
//! - `@spectral/db` business model witness (OPTIONAL closed navigation on top of open floor)
//! - **Vocabulary reconciliation**: `splinter_graph` is the substrate-canonical composite name;
//!   Kagi-derived "mirror" name in Taut's initial framing is superseded per
//!   `feedback-substrate-already-had-the-word` (55+ instances) and the `@mirror` collision risk.
//!
//! **Interpretation B canonical NOT enforced here** — the existing shard predates Interpretation B
//! ratification (Arc 4 sub-arc A close 2026-07-06); enrichment preserves existing structure.
//! New species (M6 sub-ticks) must honour Interpretation B.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_store_shard() -> String {
    let path = repo_root().join("shards/mirror/store.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/mirror/store.mirror at {:?}: {}", path, e))
}

// === T1: family-root already exists (baseline / regression guard) ===

#[test]
fn t01_store_family_root_declaration_present() {
    let content = read_store_shard();
    assert!(
        content.contains("prism @mirror/store"),
        "T1: `prism @mirror/store` family-root declaration must remain load-bearing"
    );
    for req in ["in @prism", "in @glass", "in @meta"] {
        assert!(
            content.contains(req),
            "T1: ancestry `{}` must remain declared (regression guard)",
            req
        );
    }
}

// === T2: six-op canonical surface preserved ===

#[test]
fn t02_store_six_op_surface_preserved() {
    let content = read_store_shard();
    for op_signature in [
        "read(o: oid)",
        "write(content: bytes)",
        "exists(o: oid)",
        "diff(a: oid, b: oid)",
        "walk(root: oid)",
        "verify(o: oid, content: bytes)",
    ] {
        assert!(
            content.contains(op_signature),
            "T2: six-op surface signature `{}` must remain load-bearing (Apache-2.0 floor contract)",
            op_signature
        );
    }
}

// === T3: canonical vocabulary preserved (substrate-already-had-the-word) ===

#[test]
fn t03_store_canonical_vocabulary_preserved() {
    let content = read_store_shard();
    // `splinter_graph` is the substrate-canonical composite (git-tree analog).
    // Do NOT rename to "mirror" (would collide with @mirror family-root).
    assert!(
        content.contains("splinter_graph"),
        "T3: substrate-canonical `splinter_graph` MUST remain load-bearing (composite/tree-analog trichotomy element). Kagi-derived alternative names (`mirror`) would collide with @mirror family-root per feedback-substrate-already-had-the-word"
    );
    assert!(
        content.contains("type oid = ref") || content.contains("type oid=ref"),
        "T3: `type oid = ref` MUST remain (content-addressed identity primitive)"
    );
}

// === T4: Recognition #43 explicit citation (mirror IS content-addressed build system) ===

#[test]
fn t04_store_cites_recognition_43_content_addressed_build_system() {
    let content = read_store_shard();
    let has_43_citation = content.contains("#43")
        || content.contains("Recognition #43")
        || content.contains("architecture-mirror-as-content-addressed-build-system")
        || content.contains("content-addressed build system");
    assert!(
        has_43_citation,
        "T4: shard MUST cite Recognition #43 (mirror IS content-addressed build system; LANDED per collapse spec §11 promotion). Existing shard references `[[architecture-fragmentation-is-the-rust-substrate]]` but not #43 direct — promote to explicit witness."
    );
}

// === T5: immutable-under-hash invariant declared explicitly ===

#[test]
fn t05_store_declares_immutable_under_hash_invariant() {
    let content = read_store_shard();
    let has_immutability = content.contains("immutable-under-hash")
        || content.contains("immutable under hash")
        || content.contains("content-address is immutable")
        || content.contains("immutable by construction")
        || content.contains("immutability invariant");
    assert!(
        has_immutability,
        "T5: shard MUST declare `immutable-under-hash` invariant EXPLICITLY. Currently implicit in oid semantics but not named. Per collapse spec §11.3: non-negotiable at v0 — Nix's ca-derivations arc shows what happens when bolted on later."
    );
}

// === T6: purely-functional composition invariant declared ===

#[test]
fn t06_store_declares_purely_functional_composition() {
    let content = read_store_shard();
    let has_pfc = content.contains("purely-functional")
        || content.contains("purely functional")
        || content.contains("referentially transparent")
        || content.contains("referential transparency")
        || content.contains("deterministic composition");
    assert!(
        has_pfc,
        "T6: shard MUST declare purely-functional composition invariant. Per collapse spec §11.3: `any Prism<A,B> on stored objects is a total function OID_A → OID_B; deterministic`. Grounds Dolstra 2006 lineage."
    );
}

// === T7: Bazel REAPI floor decomposition (CAS + action-cache split) ===

#[test]
fn t07_store_cites_bazel_reapi_floor() {
    let content = read_store_shard();
    let has_reapi = content.contains("REAPI")
        || content.contains("Bazel Remote")
        || content.contains("Remote Execution API")
        || content.contains("action-cache")
        || content.contains("action cache")
        || content.contains("CAS");
    assert!(
        has_reapi,
        "T7: shard MUST cite Bazel REAPI floor decomposition (CAS + action-cache split). Per collapse spec §11.3: `(1) OID → object map + (2) action-hash → OID map. Both open.` This IS the industrial minimum floor."
    );
}

// === T8: Dolstra 2006 PhD ancestor cited ===

#[test]
fn t08_store_cites_dolstra_2006_ancestor() {
    let content = read_store_shard();
    let has_dolstra = content.contains("Dolstra")
        || content.contains("Nix")
        || content.contains("Purely Functional Software Deployment");
    assert!(
        has_dolstra,
        "T8: shard MUST cite Dolstra 2006 PhD `Purely Functional Software Deployment Model` OR reference `Nix` as prior-art ancestor. The canonical treatment of derivation-store mathematics."
    );
}

// === T9: Mokhov et al. "Build Systems à la Carte" JFP 2020 ancestor ===

#[test]
fn t09_store_cites_build_systems_a_la_carte() {
    let content = read_store_shard();
    let has_mokhov = content.contains("Mokhov")
        || content.contains("Build Systems à la Carte")
        || content.contains("Build Systems a la Carte")
        || content.contains("Peyton Jones")
        || content.contains("topological")
        || content.contains("verifying-trace");
    assert!(
        has_mokhov,
        "T9: shard MUST cite Mokhov/Mitchell/Peyton Jones `Build Systems à la Carte` (JFP 2020) OR reference the Rebuilder × Scheduler factoring. Per collapse spec §11.1: `Nix=topological×verifying-trace; Bazel=restarting×constructive-trace; Shake=suspending`. Names @mirror/store's own placement in the taxonomy."
    );
}

// === T10: IPFS Merkle DAG ancestor ===

#[test]
fn t10_store_cites_merkle_dag_ancestor() {
    let content = read_store_shard();
    let has_merkle = content.contains("Merkle")
        || content.contains("IPFS")
        || content.contains("MERKLE_DAG");
    assert!(
        has_merkle,
        "T10: shard MUST cite Merkle DAG ancestor (Merkle 1979 OR IPFS MERKLE_DAG.md OR generic `Merkle` reference). The content-addressing invariant substrate the store realises."
    );
}

// === T11: projection API `oid ↔ working-tree` formalized ===

#[test]
fn t11_store_formalizes_projection_api() {
    let content = read_store_shard();
    let has_projection = content.contains("projection API")
        || content.contains("projection api")
        || content.contains("working-tree")
        || content.contains("working tree")
        || (content.contains("projection") && content.contains("disk"));
    assert!(
        has_projection,
        "T11: shard MUST formalize projection API `oid ↔ working-tree` (git object model analog). Per collapse spec §3.5 correction: files on disk ARE projections of OIDs in @mirror/store, not source of truth. Existing shard says `store IS canonical, .shatter is one projection format` narratively — promote to formal API surface."
    );
}

// === T12: agentic value-adds framing at substrate altitude (rock-solid floor) ===

#[test]
fn t12_store_names_agentic_value_adds_at_substrate_altitude() {
    let content = read_store_shard();
    // Per collapse spec §11.6: nine agentic value-adds. At least 3 must be substrate-decl'd
    // as first-order capabilities of the open floor. This is Alex's "rock solid + useful
    // in agentic workflows standalone" directive made substrate-fact.
    let value_adds: Vec<&str> = [
        "deterministic compilation",
        "reproducible builds",
        "session persistence",
        "cross-agent",
        "provenance chains",
        "provenance",
        "verifiable computation",
        "verifiable",
        "immutable rollback",
        "time-travel",
        "ecosystem interop",
        "deterministic replay",
        "federated substrate",
        "federation",
        "agent memory",
        "agentic",
    ]
    .iter()
    .filter(|s| content.contains(*s))
    .copied()
    .collect();
    assert!(
        value_adds.len() >= 3,
        "T12: shard narrative MUST cite at least 3 agentic-workflow value-adds from collapse spec §11.6 substrate-altitude. Found: {:?}. Per Alex directive: `rock solid + useful in agentic workflows even without @spectral/db magic`. The open floor IS a first-order product, not a consolation prize.",
        value_adds
    );
}

// === T13: collapse spec §11 citation ===

#[test]
fn t13_store_cites_collapse_spec_apache_floor_section() {
    let content = read_store_shard();
    let has_citation = content.contains("mcp-spec-song-collapse.md")
        || content.contains("mcp-spec-song-collapse")
        || content.contains("collapse spec")
        || content.contains("Apache-2.0 floor")
        || content.contains("Apache-2.0 rock-solid floor");
    assert!(
        has_citation,
        "T13: shard MUST cite `docs/specs/mcp-spec-song-collapse.md` §11 OR reference `Apache-2.0 floor` framing. Links substrate-decl to the canonical spec that promoted #S2 + #S4 + declared the floor."
    );
}

// === T14: business model witness — @spectral/db as OPTIONAL closed navigation on top ===

#[test]
fn t14_store_names_spectral_db_optional_closed_layer() {
    let content = read_store_shard();
    let has_spectral_db = content.contains("@spectral/db")
        || content.contains("spectral-db")
        || content.contains("spectral/db");
    let has_optional_framing = content.contains("OPTIONAL")
        || content.contains("optional")
        || content.contains("standalone")
        || content.contains("without @spectral/db")
        || content.contains("closed navigation")
        || content.contains("closed engine");
    assert!(
        has_spectral_db && has_optional_framing,
        "T14: shard MUST name `@spectral/db` AND frame it as OPTIONAL / closed navigation on top of the open floor. Per collapse spec §11.4: `@spectral/db` adds typed edges + spectral navigation + sub-Turing queries + cellular sheaf; open store deliberately doesn't carry these. Business model + technical differentiation both live on this fault plane."
    );
}

// === T15: substrate-canonical splinter_graph naming defended (anti-collision guard) ===

#[test]
fn t15_store_defends_splinter_graph_naming_over_mirror_collision() {
    let content = read_store_shard();
    // The composite trichotomy element MUST NOT be named `mirror` (collides with @mirror
    // family root). The substrate's canonical name `splinter_graph` MUST be preserved AND
    // the shard should acknowledge the naming choice (either in narrative or by structural
    // presence of splinter_graph declaration adjacent to trichotomy vocabulary).
    let has_trichotomy_awareness = content.contains("trichotomy")
        || content.contains("three-layer")
        || content.contains("git-object")
        || content.contains("blob/tree/commit")
        || content.contains("leaf")
        || (content.contains("splinter") && content.contains("crystal"));
    assert!(
        has_trichotomy_awareness,
        "T15: shard narrative MUST demonstrate awareness of the git-object-model trichotomy (blob/tree/commit analog) using substrate-canonical names (`splinter` leaf + `splinter_graph` composite + `crystal` settled root). Explicit trichotomy narrative preserves the collision-avoidance discipline (substrate-canonical `splinter_graph` NOT Kagi-alternative `mirror`)."
    );
}
