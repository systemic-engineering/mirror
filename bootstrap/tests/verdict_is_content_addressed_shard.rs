//! N1 TICK 1 RED — `shards/epistemologic/property/verdict_is_content_addressed.mirror`.
//!
//! Per Taut scout report on incremental compilation (this session): the ONE
//! substrate-decl that lifts `crystal.derived_predicates` from "audit trail
//! of what was verified" to "memoizable cache by construction."
//!
//! **The claim**: `verdict(spec_oid, target_oid, inputs_oid) -> verdict` IS a
//! TOTAL FUNCTION of its input OIDs. Same inputs → same verdict, always,
//! trivially. Memoization is valid by construction (no invalidation logic
//! required beyond checking whether the input OID triple has changed).
//!
//! **Root cause of the 13-minute pre-commit hook** (per Taut scout):
//! `cmd_kintsugi_spec` at `bootstrap/src/lib.rs:~1189` walks every target on
//! every invocation, spawns `cargo <check>` cold each time. Zero cross-
//! invocation cache; zero per-target result memoization. This shard is the
//! substrate-decl that authorizes memoization to become the standard path.
//!
//! **Existing substrate already has most of the machinery**:
//! - `crystal.derived_predicates: [property_verdict]` at
//!   `shards/mirror/store/crystal.mirror:344-368` — the carrier field
//! - `walk(root) -> splinter_graph` at `@mirror/store` — forward closure
//! - Blake3 content-addressing throughout `bootstrap/src/**/*.rs`
//! - `Crystallizations<H>` dispatch table at `bootstrap/src/crystallize.rs:520+`
//!   — the Ref→body pattern that will accept a `(Ref, inputs_hash) → verdict`
//!   variant
//! - `docs/specs/mosaic-store-cache-invariants.md` (Mara, 2016 lines,
//!   2026-06-28) §9.2 forward-promises 8 bilateral shard pairs; none landed
//!
//! **N-cascade forward-promises** downstream of this predicate:
//! - **N2**: `cache_read` + `cache_write` at `@mirror/store` keyed on
//!   `(spec_oid, target_oid, inputs_oid) → verdict`
//! - **N3**: Rust wiring — `cmd_kintsugi_spec` consults verdict cache before
//!   dispatching cargo
//! - **N4**: reverse-closure `impacted_by(oid) -> [oid]` at `@mirror/store`
//! - **N5**: `@kintsugi/store/git commit-as-fold` — `mirror kintsugi --commit`
//!   IS `git commit` at git-projection altitude
//!
//! **Interpretation B canonical DOES apply** (green-field shard).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_verdict_shard() -> String {
    let path =
        repo_root().join("shards/epistemologic/property/verdict_is_content_addressed.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/epistemologic/property/verdict_is_content_addressed.mirror at {:?}: {}",
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

// === T1-T4: canonical shape + Interpretation B baseline + property pact ===

#[test]
fn t01_verdict_shard_declares_property_per_path_pact() {
    let content = read_verdict_shard();
    assert!(
        content.contains("@epistemologic/property/verdict_is_content_addressed"),
        "T1: shard MUST declare `@epistemologic/property/verdict_is_content_addressed` per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_verdict_shard();
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
    let content = read_verdict_shard();
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
fn t04_verdict_shard_inherits_universal_transparency_and_property_ancestry() {
    let content = read_verdict_shard();
    for req in ["in @prism", "in @meta", "in @epistemologic"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + property family root)",
            req
        );
    }
}

// === T5-T7: predicate signature + content-addressed invariant ===

#[test]
fn t05_verdict_predicate_signature_declared() {
    let content = read_verdict_shard();
    // Accept either full signature or per-parameter naming. The predicate
    // takes three OID arguments and returns verdict.
    let has_signature = content.contains("verdict_is_content_addressed(")
        && (content.contains("spec_oid") || content.contains("spec_ref"))
        && (content.contains("target_oid") || content.contains("target_ref"))
        && (content.contains("inputs_oid")
            || content.contains("input_oid")
            || content.contains("inputs_ref"))
        && content.contains("-> verdict");
    assert!(
        has_signature,
        "T5: shard MUST declare `verdict_is_content_addressed(spec_oid, target_oid, inputs_oid) -> verdict` predicate signature (or equivalent naming with `ref` suffix). Per Taut scout Reed lean."
    );
}

#[test]
fn t06_verdict_narrative_names_total_function_invariant() {
    let content = read_verdict_shard();
    // The CORE claim: verdict is a TOTAL FUNCTION of input OIDs.
    // Same inputs → same verdict, always. Cacheable by construction.
    let has_total_function = content.contains("total function")
        || content.contains("total-function")
        || content.contains("purely-functional")
        || content.contains("purely functional")
        || content.contains("referentially transparent")
        || content.contains("deterministic function");
    assert!(
        has_total_function,
        "T6: shard narrative MUST name the total-function invariant. `verdict(spec_oid, target_oid, inputs_oid)` IS a total function of its inputs by content-address discipline. Same inputs → same verdict, always."
    );
}

#[test]
fn t07_verdict_narrative_names_memoization_by_construction() {
    let content = read_verdict_shard();
    // The consequence: memoization is trivially valid.
    let has_memoization = content.contains("memoiz")
        || content.contains("cache by construction")
        || content.contains("cached by construction")
        || content.contains("trivially cacheable")
        || content.contains("cache lookup");
    assert!(
        has_memoization,
        "T7: shard narrative MUST name the memoization consequence. When verdict is a total function of input OIDs, memoization is VALID BY CONSTRUCTION — no invalidation logic beyond checking whether input OID triple changed."
    );
}

// === T8-T10: ancestor citations ===

#[test]
fn t08_verdict_cites_recognition_43_content_addressed_build_system() {
    let content = read_verdict_shard();
    let has_43 = content.contains("#43")
        || content.contains("content-addressed build system")
        || content.contains("architecture-mirror-as-content-addressed");
    assert!(
        has_43,
        "T8: shard MUST cite Recognition #43 (mirror IS content-addressed build system; LANDED at M6 TICK 1 `884f433`). This predicate IS the operational consequence of #43 at verdict altitude."
    );
}

#[test]
fn t09_verdict_cites_crystal_derived_predicates_carrier() {
    let content = read_verdict_shard();
    let has_crystal = (content.contains("crystal") || content.contains("@mirror/store/crystal"))
        && (content.contains("derived_predicates") || content.contains("derived predicates"));
    assert!(
        has_crystal,
        "T9: shard MUST cite `crystal.derived_predicates` at `shards/mirror/store/crystal.mirror` as the carrier field this predicate operationalizes. `derived_predicates: [property_verdict]` was landed as an audit-trail field; this predicate lifts it to memoizable-cache-by-construction."
    );
}

#[test]
fn t10_verdict_cites_mosaic_store_cache_invariants_spec() {
    let content = read_verdict_shard();
    let has_spec = content.contains("mosaic-store-cache-invariants")
        || content.contains("cache-invariants")
        || content.contains("cache invariants");
    assert!(
        has_spec,
        "T10: shard MUST cite `docs/specs/mosaic-store-cache-invariants.md` (Mara 2026-06-28, 2016 lines). §9.2 forward-promises 8 bilateral shard pairs; this predicate is one of them landing."
    );
}

// === T11-T12: composition points (@mirror/store + hook overhead framing) ===

#[test]
fn t11_verdict_composes_with_mirror_store_oid_discipline() {
    let content = read_verdict_shard();
    let has_store = content.contains("@mirror/store") || content.contains("@mirror / store");
    assert!(
        has_store,
        "T11: shard MUST cite `@mirror/store` — the OID discipline that provides `oid`, `walk`, `read`, `write`, `verify` primitives this predicate composes with."
    );
}

#[test]
fn t12_verdict_names_the_hook_overhead_target() {
    let content = read_verdict_shard();
    // Business context: this predicate exists to enable incremental
    // verdict caching, which fixes the 13-min pre-commit hook overhead.
    let has_target = content.contains("pre-commit")
        || content.contains("cmd_kintsugi_spec")
        || content.contains("incremental")
        || content.contains("13-min")
        || content.contains("13 min")
        || content.contains("hook overhead");
    assert!(
        has_target,
        "T12: shard narrative MUST name the operational target — `pre-commit hook`, `cmd_kintsugi_spec`, `incremental`, or `hook overhead`. Grounds the substrate-decl in its consumer discipline: fixing the 13-min pre-commit hook via verdict cache."
    );
}

// === T13-T15: N-cascade context + forward-promises ===

#[test]
fn t13_verdict_names_n_cascade_forward_promises() {
    let content = read_verdict_shard();
    // Forward-promised downstream: N2 cache_read/write, N3 Rust wiring,
    // N4 impacted_by (reverse-closure), N5 commit-as-fold.
    let has_forward = content.contains("cache_read")
        || content.contains("cache_write")
        || content.contains("impacted_by")
        || content.contains("commit-as-fold")
        || content.contains("reverse closure")
        || content.contains("reverse-closure")
        || content.contains("N-cascade")
        || content.contains("N2")
        || content.contains("N3");
    assert!(
        has_forward,
        "T13: shard narrative MUST forward-promise N-cascade downstream ticks — `cache_read`/`cache_write` (N2), Rust wiring (N3), `impacted_by` reverse-closure (N4), `commit-as-fold` (N5). Names the substrate-decl as the anchor point for the incremental verdict cascade."
    );
}

#[test]
fn t14_verdict_cites_fate_hinge_or_optical_inference_ancestry() {
    let content = read_verdict_shard();
    // Verdicts emerge from the runtime substrate (Fate optical inference).
    // The predicate ANCHORS the compiletime/runtime hinge for verdicts.
    let has_fate = content.contains("@fate")
        || content.contains("#58")
        || content.contains("optical inference")
        || content.contains("hinge between runtime")
        || content.contains("runtime and compiletime");
    assert!(
        has_fate,
        "T14: shard MUST cite the @fate substrate-decl OR Recognition #58 (Fate IS optical inference) OR the runtime/compiletime hinge discipline (per M3 TICK 1 correction). Verdicts emerge from the runtime substrate; this predicate declares that emergence to be content-addressed."
    );
}

#[test]
fn t15_verdict_has_predicate_body_obligation_block() {
    let content = read_verdict_shard();
    // Every substrate-decl'd predicate body is an obligation block `{ \ }`.
    let has_obligation = content.contains("{ \\ }")
        || content.contains("{ \\}")
        || content.contains("{\\ }")
        || content.contains("{ \\ ")
        || content.contains("\\ }");
    assert!(
        has_obligation,
        "T15: predicate body MUST be an obligation block `{{ \\ }}` per substrate-decl discipline. Realisation dispatches through the substrate; the shard declares the contract only."
    );
}
