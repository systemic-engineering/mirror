//! `@epistemologic/pact/composition_closed` substrate-decl RED — the
//! property altitude declaration that names what Mara's cross-shard
//! resolver (mirror `128e0d2`) operationalized at the @io floor.
//!
//! Per [[architecture-property-fracture-bilateral]]: declarative property
//! at `@epistemologic/pact/<predicate>` + operational discharge via the
//! kintsugi loop bridge. The Rust resolver in `bootstrap/src/lib.rs`
//! (`collect_declared_namespaces`, `count_unresolved_imports`) is the
//! current @io discharge — it builds the declared-namespace index over
//! `shards/` + `boot/std/` and surfaces each unresolved `in @X` as a
//! per-file dark region, downgrading the corpus verdict to `failure`.
//!
//! Composition is closed under a corpus iff every `in @X` statement
//! across the corpus resolves to a `glass`/`prism`/`pact` declaration
//! somewhere in the same corpus. "Closed" is dependency-graph language:
//! the directed graph of imports has no edges escaping the declared
//! namespace set. A dangling `in @bogus` is a dependency-graph break;
//! the kintsugi loop sees the break as a dark region and mutates against
//! it.
//!
//! Lands at the new path — `@epistemologic/pact/composition_closed`,
//! not `@epistemologic/property/composition_closed` — per the rename
//! Mara just landed at `d498fc0`. The keyword_matches_path_root
//! property guards this: the file MUST live at @epistemologic/pact/.
//!
//! Sub-types are Mara's to pick. The RED specifies the SHAPE:
//! - `pact @epistemologic/pact/composition_closed { ... }`
//! - At least 2 typed predicates: helper(s) (corpus-altitude index +
//!   unresolved-imports tally) + combined `composition_closed(...)` ->
//!   transparency
//! - Exports for the namespace + the combined predicate
//! - Preamble citing 128e0d2 (the @io floor discharging the property)
//!   and d498fc0 (the @epistemologic/pact rename closure that made the
//!   path correct)

use std::path::{Path, PathBuf};

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn shard_path() -> PathBuf {
    repo_root().join("shards/epistemologic/pact/composition_closed.mirror")
}

fn shard_src() -> String {
    let p = shard_path();
    std::fs::read_to_string(&p).unwrap_or_else(|_| {
        panic!(
            "expected substrate file at {} (composition_closed property — names what Mara's cross-shard resolver at 128e0d2 discharges at the @io floor)",
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

// ── Existence at the NEW path ───────────────────────────────────────

#[test]
fn composition_closed_shard_lives_under_epistemologic_pact() {
    assert!(
        shard_path().exists(),
        "expected {} to be declared (post-rename: properties live at @epistemologic/pact/, not @epistemologic/property/, per `keyword_matches_path_root` landed at d498fc0)",
        shard_path().display()
    );
}

#[test]
fn composition_closed_does_NOT_live_under_epistemologic_property() {
    let old_path = repo_root().join("shards/epistemologic/property/composition_closed.mirror");
    assert!(
        !old_path.exists(),
        "expected {} to NOT exist (landing at the OLD path would violate `keyword_matches_path_root` immediately on the first kintsugi pass)",
        old_path.display()
    );
}

// ── Imports `@glass` for ref + transparency vocabulary ──────────────────

#[test]
fn composition_closed_imports_glass_substrate() {
    let src = shard_src();
    let lines = nonblank_lines(&src);
    assert!(
        lines.iter().any(|l| *l == "in @glass"),
        "expected `in @glass` (substrate vocabulary: ref, transparency, opacity_map) in {}; got non-blank lines: {:?}",
        shard_path().display(),
        lines
    );
}

// ── `pact` declaration at the right path ───────────────────────────────

#[test]
fn composition_closed_uses_pact_keyword_at_pact_path() {
    let src = shard_src();
    let want = "pact @epistemologic/pact/composition_closed";
    assert!(
        src.contains(want),
        "expected declaration line containing `{}` (keyword=pact lined up with first-segment=pact — satisfies `keyword_matches_path_root`) in {}; src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Combined predicate returns transparency ─────────────────────────────

#[test]
fn composition_closed_predicate_returns_transparency() {
    let src = shard_src();
    let want_substr = "composition_closed(";
    assert!(
        src.contains(want_substr),
        "expected predicate `composition_closed(...)` declared inside the pact block in {}; src:\n{}",
        shard_path().display(),
        src
    );
    assert!(
        src.contains("-> transparency"),
        "expected `composition_closed(...) -> transparency` (verdict surface the kintsugi loop consumes via opacity_map naming each unresolved import; per `keyword_matches_depth.mirror` convention) in {}; src:\n{}",
        shard_path().display(),
        src
    );
}

// ── Helper predicate(s) declared inside the pact block ──────────────────
//
// The combined property decomposes into helpers naming the resolver's
// operational form: a declared-namespace index across the corpus + an
// unresolved-imports tally. Names are Mara's choice; both shapes must
// surface as separate typed predicates so the property can be replayed
// independent of the @io discharge.

#[test]
fn composition_closed_declares_helper_predicates() {
    let src = shard_src();
    let total_actions = src.matches(") -> ").count();
    assert!(
        total_actions >= 3,
        "expected at least 3 typed predicates inside the pact block (one combined + at least two helpers: declared-namespace index + unresolved-imports tally) in {}; total `) -> ` count = {}",
        shard_path().display(),
        total_actions
    );
}

// ── Exports the namespace ───────────────────────────────────────────────

#[test]
fn composition_closed_exports_namespace() {
    let src = shard_src();
    let want = "out @epistemologic/pact/composition_closed";
    assert!(
        src.contains(want),
        "expected `{}` export in {} (substrate composition discipline); src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

#[test]
fn composition_closed_exports_combined_predicate() {
    let src = shard_src();
    let want = "out composition_closed";
    assert!(
        src.contains(want),
        "expected `{}` export in {} (downstream consumers import the predicate directly); src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Preamble cites the operational discharge + the rename closure ──────

#[test]
fn composition_closed_preamble_cites_operational_discharge() {
    let src = shard_src();
    assert!(
        src.contains("128e0d2"),
        "expected preamble to cite mirror commit `128e0d2` (Mara's cross-shard resolver GREEN; the @io floor that builds the declared-namespace index and surfaces unresolved imports as dark regions — this property's discharge today) in {}",
        shard_path().display()
    );
}

#[test]
fn composition_closed_preamble_cites_bilateral_pattern() {
    let src = shard_src();
    let cites_bilateral =
        src.contains("property-fracture-bilateral") || src.contains("property/fracture");
    assert!(
        cites_bilateral,
        "expected preamble to cite the [[architecture-property-fracture-bilateral]] pattern (declarative property + operational discharge; this shard is the declarative half, the Rust resolver is the operational half) in {}",
        shard_path().display()
    );
}
