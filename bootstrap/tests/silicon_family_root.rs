//! Phase A RED — @silicon top-level family-root per docs/specs/silicon.md §8.1.
//!
//! Load-bearing per /loop 2026-07-05 Arc 1 (Alex direction):
//! *bottom-up requires the anchor*. Currently three specialized silicons exist:
//! - `shards/reality/algebra/silicon.mirror` (27.3KB, @reality/algebra species)
//! - `shards/epistemologic/reality/silicon.mirror` (2.7KB, property altitude)
//! - `shards/glue/math_silicon.mirror` (25.5KB, glue altitude)
//!
//! ...but NO `shards/silicon.mirror` family-root. `docs/specs/silicon.md`
//! (104.8KB, 2026-06-30, Mara #527) declares the top-level @autopoietic
//! family-root per §1.1 + §8.1; landed as SPEC only, no shard on disk
//! (Taut ae063d68 Q1 grep confirms).
//!
//! Composition anchor for downstream:
//! - @mirror/bench (LANDED at shards/mirror/bench.mirror per Taut Q2) needs
//!   @silicon-typed cost carrier; currently untyped
//! - @kintsugi/knapsack (spec at 44c5db1, docs/specs/knapsack-as-kintsugi-
//!   inner-loop.md; forward-promised shard) needs @silicon/bound anchor
//! - Previous /loop's @silicon/bound framing (Alex direction 2026-07-05,
//!   BOTH-AND at f3b231d Seam Phase D Path (d)) anchors to @silicon-as-
//!   such once landed
//!
//! **RED phase**: `shards/silicon.mirror` does not exist. Text-check tests
//! fail on file absence + declaration shape.
//!
//! **GREEN phase** (Mara 🟢 next tick): land family-root shard per
//! docs/specs/silicon.md §8.1 canonical shape. Composes lifting three
//! existing species into @silicon (per Arc 1 spec: either <= @silicon
//! inheritance OR parametric altitude carrier per Mara's canonicalization).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}

#[test]
fn silicon_family_root_shard_exists() {
    let path = repo_root().join("shards/silicon.mirror");
    assert!(
        path.exists(),
        "shards/silicon.mirror must exist as top-level @autopoietic \
         family-root per docs/specs/silicon.md §8.1. Currently the substrate \
         has three specialized silicons (@reality/algebra/silicon, \
         @epistemologic/reality/silicon, @glue/math_silicon) but no \
         @silicon-as-such anchor. Bottom-up requires the root."
    );
}

#[test]
fn silicon_declares_prism_family_root() {
    let content = read_source("shards/silicon.mirror");
    assert!(
        content.contains("prism @silicon"),
        "shards/silicon.mirror must declare `prism @silicon` family-root per \
         docs/specs/silicon.md §1.1 + §8.1 canonical shape."
    );
}

#[test]
fn silicon_inherits_autopoietic() {
    let content = read_source("shards/silicon.mirror");
    assert!(
        content.contains("prism @silicon <= @autopoietic"),
        "@silicon must inherit `<= @autopoietic` per docs/specs/silicon.md \
         §1.1. The `<= @autopoietic` clause is load-bearing per §1.1: \
         declares @silicon IS-A @autopoietic prism class, inheriting the \
         fold-back permission that makes @silicon a learning loop, not a \
         one-shot probe."
    );
}

#[test]
fn silicon_imports_bauchladen_fate_glue_algebra() {
    let content = read_source("shards/silicon.mirror");
    for tag in ["in @bauchladen", "in @fate", "in @glue", "in @algebra"] {
        assert!(
            content.contains(tag),
            "@silicon must import `{}` per docs/specs/silicon.md §8.1 \
             canonical shape. Composes @bauchladen (content-addressing) + \
             @fate (constrained inference) + @glue (morphism layer to @io) \
             + @algebra (substrate-decl math being translated).",
            tag,
        );
    }
}

#[test]
fn silicon_imports_epistemologic_reality_silicon_properties() {
    let content = read_source("shards/silicon.mirror");
    for tag in [
        "in @epistemologic/reality/silicon/arch",
        "in @epistemologic/reality/silicon/compute_bound",
        "in @epistemologic/reality/silicon/memory",
        "in @epistemologic/reality/silicon/flake_ref",
    ] {
        assert!(
            content.contains(tag),
            "@silicon must import `{}` per docs/specs/silicon.md §4.2 lift \
             discipline. The property altitude at @epistemologic/reality/\
             silicon/* is the SUBSTRATE TRUTH about the silicon; @silicon \
             consumes it via `in` clauses (per §4.2).",
            tag,
        );
    }
}

#[test]
fn silicon_imports_io_algebra_and_runtime_link() {
    let content = read_source("shards/silicon.mirror");
    for tag in ["in @io/algebra", "in @io/runtime-link"] {
        assert!(
            content.contains(tag),
            "@silicon must import `{}` per docs/specs/silicon.md §8.1. \
             Emissions cross @io via @glue; @io/algebra is the Turing-\
             complete exposure surface; @io/runtime-link is the ABI link \
             mechanism.",
            tag,
        );
    }
}

#[test]
fn silicon_declares_five_operations() {
    let content = read_source("shards/silicon.mirror");
    for op in [
        "focus silicon",
        "project silicon",
        "split silicon",
        "shift silicon",
        "settle silicon",
    ] {
        assert!(
            content.contains(op),
            "@silicon must declare the five operations (`{}`) per \
             docs/specs/silicon.md §1.1 + [[architecture-prism-as-trait-as-\
             everything]]. Each operation is a typed lambda with obligation \
             discharged via @fate; bodies live in the inherited @autopoietic \
             class's @fate-resolved holes.",
            op,
        );
    }
}

#[test]
fn silicon_algebra_sub_prism_shard_exists() {
    let path = repo_root().join("shards/silicon/algebra.mirror");
    assert!(
        path.exists(),
        "shards/silicon/algebra.mirror must exist as @silicon/algebra sub-\
         prism per docs/specs/silicon.md §8.1. This is the Bauchladen where \
         crystallized executable algebra tuned to the local silicon \
         accumulates."
    );
}

#[test]
fn silicon_algebra_declares_sub_prism() {
    let content = read_source("shards/silicon/algebra.mirror");
    assert!(
        content.contains("prism @silicon/algebra"),
        "shards/silicon/algebra.mirror must declare `prism @silicon/algebra` \
         sub-prism per docs/specs/silicon.md §8.1 canonical shape."
    );
}

#[test]
fn silicon_algebra_inherits_bauchladen() {
    let content = read_source("shards/silicon/algebra.mirror");
    assert!(
        content.contains("prism @silicon/algebra <= @bauchladen"),
        "@silicon/algebra must inherit `<= @bauchladen` per docs/specs/\
         silicon.md §5.1 + §8.1. The double-inheritance (parent @silicon <= \
         @autopoietic; child @silicon/algebra <= @bauchladen) IS the \
         substrate-decl shape of a learning system per §5.1: parent \
         provides loop; child provides tray."
    );
}

#[test]
fn silicon_algebra_imports_silicon_parent() {
    let content = read_source("shards/silicon/algebra.mirror");
    assert!(
        content.contains("in @silicon"),
        "@silicon/algebra must import `in @silicon` per docs/specs/silicon.md \
         §8.1 canonical shape. Parent-child composition path."
    );
}
