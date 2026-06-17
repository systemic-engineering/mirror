//! `@epistemologic/pact/substrate_source_in_shards` substrate-decl RED —
//! names the rule that drives the boot/std/* → shards/* migration via
//! kintsugi.
//!
//! Recognition shape: per [[architecture-shards-as-substrate-source]],
//! "Mirror source lives in shards/ (not boot/, not glass/). Substrate
//! source IS substrate data; the recursive proof is literal." The 17
//! remaining `boot/std/epistemologic/property/*` shards (Taut flagged
//! 2026-06-17) violate this rule. Every other boot/std/*.mirror does
//! too. The kintsugi loop already surfaces them as darks under
//! `@epistemologic/pact/keyword_matches_path_root` (those that use the
//! `property` segment); this property generalizes that to ALL boot/std
//! shards.
//!
//! Per Alex's 2026-06-17 question: "How can we use kintsugi to migrate
//! the old bootstrap into the new shape?" — by naming the rule at the
//! @epistemologic altitude, then letting the kintsugi loop carry the
//! darks to a fracture body that proposes the migration morphisms.
//!
//! Operational discharge: today, Mara's cross-shard resolver at mirror
//! `128e0d2` walks `shards/` + `boot/std/` and builds the declared-
//! namespace index. Extending it to surface boot/std/* presence as an
//! additional dark axis closes the @io floor for this property
//! (parallel to how composition_closed's discharge lives in the same
//! resolver). The fracture body that AUTO-APPLIES the migration is
//! either (a) blocked on #272 (fracture predicate fixpoint) for the
//! substrate-clean path, or (b) implementable today in Rust @io floor
//! for the practical path. Either way, the property declaration is
//! the entry point.
//!
//! Self-witnessing: this property's own shard MUST live at
//! `shards/epistemologic/pact/substrate_source_in_shards.mirror` and
//! declare with `pact`. It satisfies its own rule on the first frame.
//!
//! Sub-types are Mara's to pick. The RED specifies the SHAPE:
//! - `pact @epistemologic/pact/substrate_source_in_shards { ... }`
//! - At least 2 typed predicates: a helper enumerating the offenders
//!   (boot/std declarations or similar) + the combined predicate
//!   `substrate_source_in_shards(corpus: ref) -> transparency`
//! - Exports for namespace + combined predicate
//! - Preamble citing [[architecture-shards-as-substrate-source]],
//!   Mara's keyword_matches_path_root rename closure (d498fc0),
//!   and Taut's 2026-06-17 report identifying the 17 + 22 + 5
//!   surfaces still pending migration.

use std::path::{Path, PathBuf};

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

fn shard_path() -> PathBuf {
    repo_root().join("shards/epistemologic/pact/substrate_source_in_shards.mirror")
}

fn shard_src() -> String {
    let p = shard_path();
    std::fs::read_to_string(&p).unwrap_or_else(|_| {
        panic!(
            "expected substrate file at {} (substrate_source_in_shards — names the boot/std/* → shards/* migration rule)",
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

// ── Existence at the right path ──────────────────────────────────────────

#[test]
fn substrate_source_in_shards_lives_under_epistemologic_pact() {
    assert!(
        shard_path().exists(),
        "expected {} to be declared (the property naming the boot/std/* → shards/* migration rule must itself live at shards/, satisfying its own rule on the first frame)",
        shard_path().display()
    );
}

#[test]
fn substrate_source_in_shards_does_NOT_live_under_boot_std() {
    let boot_path =
        repo_root().join("boot/std/epistemologic/property/substrate_source_in_shards.mirror");
    assert!(
        !boot_path.exists(),
        "expected {} to NOT exist (landing the property declaring `source in shards/` at the OLD path boot/std/ would violate its own rule immediately)",
        boot_path.display()
    );
}

// ── Imports @glass for ref + transparency vocabulary ───────────────────

#[test]
fn substrate_source_in_shards_imports_glass() {
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
fn substrate_source_in_shards_uses_pact_keyword() {
    let src = shard_src();
    let want = "pact @epistemologic/pact/substrate_source_in_shards";
    assert!(
        src.contains(want),
        "expected declaration line containing `{}` (keyword=pact lined up with first-segment=pact — self-witnessing under keyword_matches_path_root) in {}; src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Combined predicate returns transparency ─────────────────────────────

#[test]
fn substrate_source_in_shards_predicate_returns_transparency() {
    let src = shard_src();
    let want_substr = "substrate_source_in_shards(";
    assert!(
        src.contains(want_substr),
        "expected predicate `substrate_source_in_shards(...)` declared inside the pact block in {}; src:\n{}",
        shard_path().display(),
        src
    );
    assert!(
        src.contains("-> transparency"),
        "expected `substrate_source_in_shards(...) -> transparency` (the kintsugi loop reads the opacity_map to drive migration; each opacity names ONE offending boot/std/*.mirror file) in {}; src:\n{}",
        shard_path().display(),
        src
    );
}

// ── Helper predicate(s) declared inside the pact block ──────────────────
//
// At least one helper names the operational shape — e.g. the set of
// boot/std/* declarations, or the count of offending paths. Name is
// Mara's choice; the shape must surface so the property is replayable
// independent of the @io discharge.

#[test]
fn substrate_source_in_shards_declares_helper_predicates() {
    let src = shard_src();
    let total_actions = src.matches(") -> ").count();
    assert!(
        total_actions >= 2,
        "expected at least 2 typed predicates inside the pact block (one combined + at least one helper naming the boot/std/* offender set) in {}; total `) -> ` count = {}",
        shard_path().display(),
        total_actions
    );
}

// ── Exports the namespace + combined predicate ─────────────────────────

#[test]
fn substrate_source_in_shards_exports_namespace() {
    let src = shard_src();
    let want = "out @epistemologic/pact/substrate_source_in_shards";
    assert!(
        src.contains(want),
        "expected `{}` export in {}; src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

#[test]
fn substrate_source_in_shards_exports_combined_predicate() {
    let src = shard_src();
    let want = "out substrate_source_in_shards";
    assert!(
        src.contains(want),
        "expected `{}` export in {} (downstream consumers — the kintsugi loop, the future fracture body, the @apply mode — import the predicate directly); src:\n{}",
        want,
        shard_path().display(),
        src
    );
}

// ── Preamble cites the architecture memory + the migration lineage ──────

#[test]
fn substrate_source_in_shards_preamble_cites_architecture_memory() {
    let src = shard_src();
    let cites = src.contains("architecture-shards-as-substrate-source")
        || src.contains("shards-as-substrate-source");
    assert!(
        cites,
        "expected preamble to cite [[architecture-shards-as-substrate-source]] (the load-bearing memory: 'Mirror source lives in shards/, not boot/, not glass/') in {}",
        shard_path().display()
    );
}

#[test]
fn substrate_source_in_shards_preamble_cites_rename_lineage() {
    let src = shard_src();
    // Either Mara's keyword_matches_path_root rename (d498fc0) or the
    // fact that boot/std/* currently darks under that property.
    let cites = src.contains("d498fc0")
        || src.contains("keyword_matches_path_root")
        || src.contains("128e0d2");
    assert!(
        cites,
        "expected preamble to cite the rename-driver lineage (d498fc0 closure or keyword_matches_path_root or 128e0d2 cross-shard resolver) so the cascade context is preserved in {}",
        shard_path().display()
    );
}
