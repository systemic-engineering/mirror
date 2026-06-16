//! `@epistemologic/pact/keyword_matches_path_root` substrate-decl RED —
//! the property that drives the @epistemologic/property → @epistemologic/pact
//! migration via kintsugi.
//!
//! Recognition shape: the substrate already had `pact` (recognition #37,
//! Paskian agreement; promoted 2026-06-10). The keyword `pact` is what
//! every property-declaration shard already uses internally. The path
//! segment `property` is the residue of the conversation-era vocabulary
//! that pact superseded. The substrate already had the word
//! ([[feedback-substrate-already-had-the-word]]); the path didn't move
//! with the keyword.
//!
//! The property: for every shard at depth 1 under `@epistemologic`, the
//! first path segment AFTER `@epistemologic` must equal the declaration
//! keyword used inside the file. Concretely today:
//!
//!   pact @epistemologic/pact/<name> { ... }   -- self-witnessing pact
//!   pact @epistemologic/property/<name> { ... } -- DARK (keyword=pact, path=property)
//!
//! Self-witnessing: this property's own shard MUST live at
//! `@epistemologic/pact/keyword_matches_path_root` and declare with
//! `pact`. It passes its own check the moment it lands.
//!
//! Migration target: 7 existing `@epistemologic/property/X` shards
//! (`path_matches_namespace`, `keyword_matches_depth`,
//! `parent_acyclic`, `symbol_canonical_form`, `gate_matches_diff_closure`,
//! `dissonance_partials_match_ast_breadth`,
//! `operator_matches_composition_primitive`) must move to
//! `@epistemologic/pact/X` once this property lands.
//!
//! Operational bridge: Mara's cross-shard resolver at `128e0d2` made this
//! possible — the declared-namespace index spans the whole corpus, so the
//! property predicate can be discharged at the @io floor without further
//! pipeline work. The kintsugi loop reads the opacity_map and drives
//! the migration.
//!
//! Sub-types are Mara's to pick. The RED specifies the SHAPE:
//! - `pact @epistemologic/pact/keyword_matches_path_root { ... }` (self-witnessing)
//! - At least one helper predicate (e.g. `path_root(file: ref) -> text`
//!   extracting the first segment after `@epistemologic`, or
//!   `declaration_keyword(file: ref) -> text` extracting the keyword)
//! - The combined predicate `keyword_matches_path_root(...) -> transparency`
//! - Exports for the namespace + the combined predicate
//! - Preamble citing 128e0d2 + the migration intent

use std::path::{Path, PathBuf};

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn shard_path() -> PathBuf {
    repo_root().join("shards/epistemologic/pact/keyword_matches_path_root.mirror")
}

fn shard_src() -> String {
    let p = shard_path();
    std::fs::read_to_string(&p).unwrap_or_else(|_| {
        panic!(
            "expected substrate file at {} (keyword_matches_path_root — drives @epistemologic/property → @epistemologic/pact migration)",
            p.display()
        )
    })
}

fn nonblank_lines(src: &str) -> Vec<&str> {
    src.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

// ── Existence at the NEW path ─────────────────────────────────────────
//
// The property MUST live at @epistemologic/pact/, not @epistemologic/property/.
// This is the self-witnessing constraint: the property declaring the rule
// satisfies the rule on the first frame.

#[test]
fn keyword_matches_path_root_shard_lives_under_epistemologic_pact() {
    assert!(
        shard_path().exists(),
        "expected {} to exist (self-witnessing: the property declaring `keyword=path_root` must itself satisfy the rule, so it lives at @epistemologic/pact/ not @epistemologic/property/)",
        shard_path().display()
    );
}

#[test]
fn keyword_matches_path_root_does_NOT_live_under_epistemologic_property() {
    let old_path =
        repo_root().join("shards/epistemologic/property/keyword_matches_path_root.mirror");
    assert!(
        !old_path.exists(),
        "expected {} to NOT exist (the property's own existence at the OLD path would violate the rule it declares; if Mara accidentally lands it there, the migration starts in violation)",
        old_path.display()
    );
}

// ── Imports `@glass` for ref + transparency vocabulary ──────────────────

#[test]
fn keyword_matches_path_root_imports_glass_substrate() {
    let src = shard_src();
    let lines = nonblank_lines(&src);
    assert!(
        lines.iter().any(|l| *l == "in @glass"),
        "expected `in @glass` (substrate vocabulary: ref, transparency) in {}; got non-blank lines: {:?}",
        shard_path().display(),
        lines
    );
}

// ── `pact` declaration at the right path ───────────────────────────────
//
// The keyword IS `pact` (per recognition #37). The path is
// `@epistemologic/pact/keyword_matches_path_root`. Both must line up,
// because the file declares the very rule that says they must.

#[test]
fn keyword_matches_path_root_self_witnesses_its_own_rule() {
    let src = shard_src();
    let want = "pact @epistemologic/pact/keyword_matches_path_root";
    assert!(
        src.contains(want),
        "expected declaration line containing `{}` (keyword=pact lined up with first-segment=pact — self-witnessing) in {}; src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Combined predicate returns transparency ─────────────────────────────
//
// Per `keyword_matches_depth.mirror` convention: new property predicates
// return `transparency` (located verdict surface the kintsugi loop
// consumes via opacity_map to drive migration).

#[test]
fn keyword_matches_path_root_predicate_returns_transparency() {
    let src = shard_src();
    let want_substr = "keyword_matches_path_root(";
    assert!(
        src.contains(want_substr),
        "expected predicate `keyword_matches_path_root(...)` declared inside the pact block in {}; src:\n{}",
        shard_path().display(),
        src
    );
    assert!(
        src.contains("-> transparency"),
        "expected `keyword_matches_path_root(...) -> transparency` (verdict surface the kintsugi loop reads to drive migration; opacity_map names each @epistemologic/property/X shard still at the old path) in {}; src:\n{}",
        shard_path().display(),
        src
    );
}

// ── Helper predicate(s) declared inside the pact block ──────────────────
//
// The combined property decomposes into at least one helper (e.g.
// `path_root` extracting the first segment after @epistemologic, or
// `declaration_keyword` extracting the keyword). Names are Mara's choice.

#[test]
fn keyword_matches_path_root_declares_helper_predicates() {
    let src = shard_src();
    let total_actions = src.matches(") -> ").count();
    assert!(
        total_actions >= 2,
        "expected at least 2 typed predicates inside the pact block (one combined + at least one helper naming `path_root` or `declaration_keyword`) in {}; total `) -> ` count = {}",
        shard_path().display(),
        total_actions
    );
}

// ── Exports the namespace ───────────────────────────────────────────────

#[test]
fn keyword_matches_path_root_exports_namespace() {
    let src = shard_src();
    let want = "out @epistemologic/pact/keyword_matches_path_root";
    assert!(
        src.contains(want),
        "expected `{}` export in {} (substrate composition discipline); src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Exports the combined predicate by name ──────────────────────────────

#[test]
fn keyword_matches_path_root_exports_combined_predicate() {
    let src = shard_src();
    let want = "out keyword_matches_path_root";
    assert!(
        src.contains(want),
        "expected `{}` export in {} (downstream consumers import the predicate directly, parallel to `out keyword_matches_depth`); src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Preamble cites the operational discharge + migration intent ─────────

#[test]
fn keyword_matches_path_root_preamble_cites_operational_discharge() {
    let src = shard_src();
    assert!(
        src.contains("128e0d2"),
        "expected preamble to cite mirror commit `128e0d2` (Mara's cross-shard resolver GREEN — the @io floor whose declared-namespace index spans the corpus, which is what makes the keyword/path-root check tractable to discharge today) in {}",
        shard_path().display()
    );
}

#[test]
fn keyword_matches_path_root_preamble_cites_pact_recognition() {
    let src = shard_src();
    let cites_recognition = src.contains("#37") || src.contains("Paskian");
    assert!(
        cites_recognition,
        "expected preamble to cite recognition #37 (`pact` IS Paskian agreement, promoted 2026-06-10; the keyword that the path segment must catch up to) in {}",
        shard_path().display()
    );
}
