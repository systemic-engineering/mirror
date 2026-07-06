//! N2 TICK 1 RED — `shards/mirror/store/action_cache.mirror` species.
//!
//! Per Taut scout report + N1 TICK 1 close: the REAPI ActionCache surface
//! that operationalizes verdict memoization. This species declares
//! `cache_read` + `cache_write` actions at `@mirror/store/action_cache`
//! keyed on `(spec_oid, target_oid, inputs_oid) → verdict`.
//!
//! **N1 TICK 1** (LANDED at `2857fb1`) declared the predicate
//! `verdict_is_content_addressed(spec_oid, target_oid, inputs_oid) -> verdict`
//! at `@epistemologic/property/`. That predicate AUTHORIZES memoization by
//! construction (verdict is a total function of input OIDs). This species
//! DECLARES THE OPERATIONAL SURFACE that implements the memoization.
//!
//! **Sibling to existing species** under `@mirror/store`:
//! - `@mirror/store/git` (LANDED 2026-06-30, 20KB) — namespaced git wire
//! - `@mirror/store/crystal` (LANDED 2026-06-16, 19KB) — the settled root
//!   trichotomy element with `derived_predicates` carrier
//! - `@mirror/store/action_cache` (this tick) — REAPI ActionCache surface
//!
//! Per M6 TICK 1 enrichment: Bazel REAPI floor decomposition = CAS +
//! action-cache split. `@mirror/store` is the CAS; this species is the
//! action-cache map — forward-promised at M6 TICK 3 (`shards/mirror/store.mirror`
//! §Bazel REAPI floor decomposition). LANDING here.
//!
//! **Interpretation B canonical DOES apply** (green-field species).
//!
//! **N-cascade downstream**:
//! - N3: Rust wiring — `cmd_kintsugi_spec` consults `cache_read` before
//!   dispatching cargo
//! - N4: reverse-closure `impacted_by`
//! - N5: commit-as-fold

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_action_cache_shard() -> String {
    let path = repo_root().join("shards/mirror/store/action_cache.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("read shards/mirror/store/action_cache.mirror at {:?}: {}", path, e)
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
fn t01_action_cache_shard_declares_species_per_path_pact() {
    let content = read_action_cache_shard();
    assert!(
        content.contains("@mirror/store/action_cache"),
        "T1: shard MUST declare `@mirror/store/action_cache` per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_action_cache_shard();
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
    let content = read_action_cache_shard();
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
fn t04_action_cache_inherits_universal_transparency() {
    let content = read_action_cache_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_action_cache_inherits_mirror_store_family_root() {
    let content = read_action_cache_shard();
    assert!(
        content.contains("in @mirror/store"),
        "T5: species must declare `in @mirror/store` — inherits family-root's OID / walk / read / write six-op surface (M6 Apache-2.0 floor)"
    );
}

// === T6-T8: cache_read + cache_write action signatures ===

#[test]
fn t06_action_cache_declares_cache_read_action() {
    let content = read_action_cache_shard();
    // The verdict-cache read: given (spec_oid, target_oid, inputs_oid),
    // return the cached verdict if present.
    let has_cache_read = content.contains("cache_read(")
        && (content.contains("spec_oid") || content.contains("spec_ref"))
        && (content.contains("target_oid") || content.contains("target_ref"))
        && (content.contains("inputs_oid") || content.contains("input_oid") || content.contains("inputs_ref"));
    assert!(
        has_cache_read,
        "T6: shard MUST declare `cache_read(spec_oid, target_oid, inputs_oid) -> imperfect` (or -> verdict OR -> imperfect(verdict, ...)). The verdict-cache read action keyed on the three-OID tuple per N1 predicate signature."
    );
}

#[test]
fn t07_action_cache_declares_cache_write_action() {
    let content = read_action_cache_shard();
    let has_cache_write = content.contains("cache_write(")
        && (content.contains("spec_oid") || content.contains("spec_ref"))
        && (content.contains("target_oid") || content.contains("target_ref"))
        && (content.contains("inputs_oid") || content.contains("input_oid") || content.contains("inputs_ref"));
    assert!(
        has_cache_write,
        "T7: shard MUST declare `cache_write(spec_oid, target_oid, inputs_oid, v: verdict) -> verdict` (or similar). Writes the verdict into the cache at the three-OID key; idempotent by content-address discipline."
    );
}

#[test]
fn t08_action_cache_declares_cache_exists_or_lookup_predicate() {
    let content = read_action_cache_shard();
    // A cache-hit check without full read — either `cache_exists` action
    // returning verdict, OR the `cache_read` result carries hit/miss.
    let has_hit_check = content.contains("cache_exists(")
        || content.contains("cache_hit(")
        || content.contains("cache_miss")
        || content.contains("hit")
        || (content.contains("partial") && content.contains("cache"))
        || (content.contains("imperfect") && content.contains("cache"));
    assert!(
        has_hit_check,
        "T8: shard narrative MUST name the cache-hit/miss discipline — either `cache_exists`/`cache_hit`/`cache_miss` action names, `hit`/`imperfect` composition in narrative, or an explicit hit/miss verdict return. The consumer needs to distinguish cached vs cold path."
    );
}

// === T9-T11: N1 predicate composition + REAPI floor lineage ===

#[test]
fn t09_action_cache_cites_n1_verdict_predicate() {
    let content = read_action_cache_shard();
    let has_n1 = content.contains("verdict_is_content_addressed")
        || content.contains("@epistemologic/property/verdict_is_content_addressed")
        || content.contains("N1");
    assert!(
        has_n1,
        "T9: shard MUST cite N1 predicate `@epistemologic/property/verdict_is_content_addressed` — the invariant that AUTHORIZES this cache by construction (verdict is a total function of input OIDs). Landed at `2857fb1`."
    );
}

#[test]
fn t10_action_cache_cites_bazel_reapi_ancestor() {
    let content = read_action_cache_shard();
    let has_reapi = content.contains("REAPI")
        || content.contains("Bazel Remote")
        || content.contains("Remote Execution API")
        || content.contains("ActionCache")
        || content.contains("action-cache")
        || content.contains("action cache");
    assert!(
        has_reapi,
        "T10: shard MUST cite Bazel REAPI ActionCache ancestor. Per M6 TICK 1 floor decomposition: `(1) OID → object map + (2) action-hash → OID map. Both open.` This species IS the (2) action-hash → verdict map at substrate altitude."
    );
}

#[test]
fn t11_action_cache_cites_crystal_derived_predicates_carrier() {
    let content = read_action_cache_shard();
    let has_crystal = (content.contains("crystal") || content.contains("@mirror/store/crystal"))
        && (content.contains("derived_predicates") || content.contains("derived predicates"));
    assert!(
        has_crystal,
        "T11: shard MUST cite `@mirror/store/crystal.derived_predicates` — the carrier field that IS the per-crystal verdict cache. This species DECLARES THE SURFACE that reads/writes into `derived_predicates`."
    );
}

// === T12-T13: composition + business context ===

#[test]
fn t12_action_cache_composes_with_family_root_six_ops() {
    let content = read_action_cache_shard();
    // The species inherits @mirror/store's six-op surface
    // (read/write/exists/diff/walk/verify) at family-root altitude.
    // Cite that composition.
    let has_six_op = content.contains("read")
        && content.contains("write")
        && (content.contains("exists") || content.contains("walk") || content.contains("six-op") || content.contains("six op"));
    assert!(
        has_six_op,
        "T12: shard narrative MUST cite family-root six-op composition — the underlying `read` + `write` (+ `exists` / `walk`) at `@mirror/store` that this species specializes at REAPI ActionCache altitude."
    );
}

#[test]
fn t13_action_cache_names_operational_target() {
    let content = read_action_cache_shard();
    let has_target = content.contains("pre-commit")
        || content.contains("cmd_kintsugi_spec")
        || content.contains("incremental")
        || content.contains("13-min")
        || content.contains("13 min")
        || content.contains("hook overhead");
    assert!(
        has_target,
        "T13: shard narrative MUST name the operational target — `pre-commit hook`, `cmd_kintsugi_spec`, `incremental`, or `hook overhead`. Names the 13-min hook overhead as the consumer discipline the cache surface exists to serve."
    );
}

// === T14: N-cascade forward + hinge ===

#[test]
fn t14_action_cache_names_n_cascade_downstream_or_kintsugi_settle() {
    let content = read_action_cache_shard();
    // Forward-promises: N3 Rust wiring, N4 impacted_by reverse-closure,
    // N5 commit-as-fold. Or citation of @kintsugi.settle as the consumer.
    let has_forward = content.contains("N3")
        || content.contains("N4")
        || content.contains("N5")
        || content.contains("N-cascade")
        || content.contains("impacted_by")
        || content.contains("commit-as-fold")
        || content.contains("@kintsugi.settle")
        || content.contains("kintsugi.settle")
        || content.contains("cmd_kintsugi_spec");
    assert!(
        has_forward,
        "T14: shard narrative MUST forward-promise N-cascade downstream OR cite `@kintsugi.settle` / `cmd_kintsugi_spec` as the consumer. Names the substrate's position in the incremental verdict cascade."
    );
}

// === T15: action bodies as obligation blocks ===

#[test]
fn t15_action_cache_action_bodies_are_obligation_blocks() {
    let content = read_action_cache_shard();
    // Every substrate-decl'd action body is an obligation block `{ \ }`.
    // The realisation dispatches through the substrate.
    let has_obligation = content.contains("{ \\ }")
        || content.contains("{ \\}")
        || content.contains("{\\ }")
        || content.contains("\\ }")
        || content.contains("{ \\");
    assert!(
        has_obligation,
        "T15: action bodies MUST be obligation blocks `{{ \\ }}` per substrate-decl discipline. Realisation via @io + fragmentation dispatchers; the shard declares the contract only."
    );
}
