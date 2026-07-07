//! N4 TICK 1 RED — `shards/mirror/store.mirror` family-root enrichment:
//! `impacted_by(oid: oid) -> [oid]` reverse-closure action.
//!
//! **Sibling to `walk(root: oid) -> splinter_graph`** — the existing forward
//! closure at @mirror/store family-root. `walk` answers "what does this OID
//! reach?"; `impacted_by` answers "what reaches this OID?". Together they
//! close the substrate's reachability algebra at family-root altitude.
//!
//! **Business-observable outcome**: on git diff, `impacted_by` names the
//! downstream OIDs whose cached verdict entries at
//! `@mirror/store/action_cache` (N2, 0a72c42) may be affected. Composes with
//! N2 `cache_read` / `cache_write` (N3 wired 756f2f7) for surgical
//! invalidation — only recompute verdicts whose input closure crosses the
//! touched OIDs.
//!
//! **Bazel REAPI ancestor**: `bazel query 'rdeps(//..., //target:foo)'` — the
//! reverse-dependency query at build-system altitude. `impacted_by` IS the
//! substrate's declared surface for that query.
//!
//! **N-cascade positioning**:
//! - N1 (`2857fb1`): `@epistemologic/property/verdict_is_content_addressed`
//!   predicate — memoization valid by construction
//! - N2 (`0a72c42`): `@mirror/store/action_cache` species — the surface
//! - N3 (`756f2f7`): Rust wiring in `cmd_kintsugi_spec` — the consumer
//! - **N4 (this tick)**: family-root enrichment with `impacted_by` — the
//!   REVERSE closure that makes the cache surgically invalidatable
//! - N5 (forward): `@kintsugi/store/git commit-as-fold`
//!
//! **Target shape** — enrichment of family-root, not a new species:
//! - New action declared alongside existing `walk` in the six-op region:
//!   `impacted_by(oid: oid) -> [oid] { \ }`
//! - Narrative preamble adds reverse-closure symmetry rationale
//! - `out impacted_by` in the exports block
//! - Composition narrative cites N2 action_cache invalidation surface

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

// === T1-T3: impacted_by action declaration ===

#[test]
fn t01_family_root_declares_impacted_by_action() {
    let content = read_store_shard();
    assert!(
        content.contains("impacted_by("),
        "T1: family-root MUST declare `impacted_by(...)` action — the reverse-closure at @mirror/store complementing existing `walk`"
    );
}

#[test]
fn t02_impacted_by_takes_oid_argument() {
    let content = read_store_shard();
    let has_oid_arg = content.contains("impacted_by(oid:")
        || content.contains("impacted_by(root:")
        || content.contains("impacted_by(o:");
    assert!(
        has_oid_arg,
        "T2: impacted_by MUST take an `oid` argument — the root of the reverse closure. Simplest form: `impacted_by(oid: oid) -> [oid]`."
    );
}

#[test]
fn t03_impacted_by_returns_list_of_oids() {
    let content = read_store_shard();
    let has_return = content.contains("impacted_by(oid: oid) -> [oid]")
        || content.contains("impacted_by(root: oid) -> [oid]")
        || content.contains("impacted_by(o: oid) -> [oid]")
        || (content.contains("impacted_by") && content.contains("-> [oid]"));
    assert!(
        has_return,
        "T3: impacted_by MUST return `[oid]` — the flat list of OIDs whose forward `walk` closure includes the argument."
    );
}

// === T4-T5: obligation-block body + narrative ===

#[test]
fn t04_impacted_by_body_is_obligation_block() {
    let content = read_store_shard();
    let has_body = content
        .lines()
        .filter(|l| l.contains("impacted_by("))
        .any(|l| {
            l.contains("{ \\ }") || l.trim_end().ends_with("{ \\") || l.trim_end().ends_with("\\ }")
        });
    let has_body_multiline = content.contains("impacted_by(") && content.contains("{ \\ }");
    assert!(
        has_body || has_body_multiline,
        "T4: impacted_by body MUST be an obligation block `{{ \\\\ }}` per substrate-decl discipline. Realisation via @io + fragmentation; family-root declares the contract only."
    );
}

#[test]
fn t05_family_root_narrates_reverse_closure_complement_to_walk() {
    let content = read_store_shard();
    let has_reverse_pair = (content.contains("reverse") || content.contains("reverse-closure"))
        && content.contains("walk");
    assert!(
        has_reverse_pair,
        "T5: family-root narrative MUST document `impacted_by` as the reverse-closure complement of `walk`. The pair closes the substrate's reachability algebra at family-root altitude."
    );
}

// === T6-T7: composition with N2 action_cache + REAPI rdeps ===

#[test]
fn t06_family_root_cites_n2_action_cache_invalidation_composition() {
    let content = read_store_shard();
    let has_composition = (content.contains("action_cache")
        || content.contains("@mirror/store/action_cache"))
        && (content.contains("invalidat")
            || content.contains("cache")
            || content.contains("verdict"));
    assert!(
        has_composition,
        "T6: family-root MUST cite N2 `@mirror/store/action_cache` invalidation composition — impacted_by names the OIDs whose cached verdicts may need re-verdicting after a git diff."
    );
}

#[test]
fn t07_family_root_cites_bazel_rdeps_ancestor() {
    let content = read_store_shard();
    let has_rdeps = content.contains("rdeps")
        || content.contains("reverse-dep")
        || content.contains("reverse dep")
        || content.contains("reverse dependency")
        || content.contains("reverse-dependency");
    assert!(
        has_rdeps,
        "T7: family-root MUST cite Bazel `rdeps` (or equivalent reverse-dependency-query vocabulary) as prior-art ancestor. impacted_by IS the substrate's rdeps at store altitude."
    );
}

// === T8: exports ===

#[test]
fn t08_impacted_by_appears_in_out_block() {
    let content = read_store_shard();
    assert!(
        content.contains("out impacted_by"),
        "T8: `impacted_by` MUST appear in the `out` block at file tail — action is not consumable downstream otherwise."
    );
}

// === T9: N-cascade positioning ===

#[test]
fn t09_family_root_cites_n_cascade_positioning() {
    let content = read_store_shard();
    let has_n_cascade = content.contains("N4")
        || content.contains("N-cascade")
        || (content.contains("impacted_by")
            && (content.contains("N1")
                || content.contains("N2")
                || content.contains("N3")
                || content.contains("N5")));
    assert!(
        has_n_cascade,
        "T9: family-root narrative MUST position impacted_by within the N-cascade (cites N1/N2/N3/N4/N5 or `N-cascade`)."
    );
}

// === T10-T12: regression guards — existing six-op surface preserved ===

#[test]
fn t10_regression_existing_walk_action_preserved() {
    let content = read_store_shard();
    assert!(
        content.contains("walk(root: oid) -> splinter_graph"),
        "T10 (regression): existing `walk(root: oid) -> splinter_graph` action MUST remain declared. Forward closure sibling of impacted_by."
    );
}

#[test]
fn t11_regression_existing_six_op_surface_preserved() {
    let content = read_store_shard();
    for action_name in ["read(", "write(", "exists(", "diff(", "walk(", "verify("] {
        assert!(
            content.contains(action_name),
            "T11 (regression): canonical six-op action `{}` MUST remain declared at family-root. impacted_by is a seventh action complementing walk, not a replacement of any op.",
            action_name
        );
    }
}

#[test]
fn t12_regression_existing_out_exports_preserved() {
    let content = read_store_shard();
    for export in [
        "out oid",
        "out splinter_graph",
        "out read",
        "out write",
        "out walk",
        "out verify",
    ] {
        assert!(
            content.contains(export),
            "T12 (regression): existing export `{}` MUST remain in the `out` block. impacted_by joins as an additional export, not a replacement.",
            export
        );
    }
}
