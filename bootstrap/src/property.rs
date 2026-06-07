//! `@epistemologic/property` — boundary Rust for the property layer.
//!
//! This module realizes the substrate-declared action
//!
//! ```mirror
//! action gaps_of(ast: ast) -> [gap]
//! ```
//!
//! from `docs/specs/gap-tension-tensor-substrate.md` §3.1 / `docs/specs/property-and-inference-collapse.md` §9.2.
//!
//! ## What `gaps_of` is
//!
//! The **first body that touches the actual property layer.** Given a
//! parsed [`AstNode`], iterate the substrate's known-evaluable
//! properties and extract every failure as a typed [`Gap`].
//!
//! Per the spec: `gaps_of` returns `[gap]` directly (NOT a `Verdict`).
//! It is the gap-extraction primitive. Downstream consumers
//! (T6 `tensor_of`, T7 `minimize`) compose the gaps into Verdict-
//! shaped outputs.
//!
//! ## The property registry (substrate-pull question)
//!
//! Today: **implicit**. Properties are scattered across `shards/**/*.mirror`
//! as `property X(arg) -> verdict { \ }` declarations. There is no
//! substrate-declared `[property]` map or `properties` collection that
//! `gaps_of` can iterate.
//!
//! The minimal first version evaluates the **one property whose body the
//! AST altitude has already realized** — `@epistemologic/property/total_classification`.
//! Every [`AstKind::Dark`] node in the AST is a failure of that property
//! (per `docs/specs/strict-and-total-classification.md`; per the existing
//! `enforce_strict` / `count_dark` pipeline in `bootstrap/src/main.rs`
//! that already names this property in its comments).
//!
//! As more properties get boundary-realizable verifiers, they get added
//! here as additional gap-source helpers. The substrate-pull question
//! flagged for the substrate altitude: would a substrate-declared
//! property registry (a `[property]` map, iterable at compile time) make
//! `gaps_of` substrate-driven rather than Rust-driven? Flagged for the
//! next substrate-pull tick; today this body iterates the AST altitude's
//! single known-evaluable property.
//!
//! ## Substrate-pull on `gaps_of` itself
//!
//! `gaps_of` is **not yet declared in `shards/`** — the spec at
//! `docs/specs/gap-tension-tensor-substrate.md` §3.1 names it in a
//! `grammar @epistemologic/property { ... }` block, but the file
//! `shards/epistemologic/property.mirror` does not yet exist. The
//! substrate FROZEN constraint of this tick keeps `shards/` untouched;
//! a follow-up substrate-pull tick should declare `gaps_of` proper.
//! This Rust body is the boundary-altitude realization that the
//! substrate-altitude declaration will mirror when it lands.
//!
//! ## Why this lives here (not in `gap.rs`)
//!
//! [`crate::gap`] mirrors `@epistemologic/property/gap` — the `gap`
//! *type* shard. `gaps_of` is declared at the parent altitude
//! (`@epistemologic/property`) and *produces* gaps; it is not a method
//! on `gap`. The substrate-path-honest placement is the parent
//! altitude's module, parallel to (not inside) `gap.rs`.
//!
//! [`AstNode`]: crate::ast::AstNode
//! [`AstKind::Dark`]: crate::ast::AstKind::Dark
//! [`Gap`]: crate::gap::Gap
#![allow(dead_code)]

use prism_core::Ref;

use crate::ast::{AstKind, AstNode};
use crate::gap::Gap;

// ---------------------------------------------------------------------------
// Property origins — the substrate refs each known-evaluable property lives at.
// ---------------------------------------------------------------------------

/// The substrate origin for `@epistemologic/property/total_classification`.
///
/// Per `docs/specs/strict-and-total-classification.md` and the comment
/// block on `count_dark` in `bootstrap/src/main.rs` (the existing
/// realization of this property's verdict surface): every [`AstKind::Dark`]
/// node is a failure of `total_classification` at the AST altitude.
fn total_classification_origin() -> Ref {
    Ref::new("@epistemologic/property/total_classification")
        .expect("total_classification shard path must be a valid substrate ref")
}

// ---------------------------------------------------------------------------
// Per-property gap extractors. Each extractor consumes an AST and emits
// the gaps surfaced by the substrate's verdict on that property.
// ---------------------------------------------------------------------------

/// Walk the AST in source order and collect a borrowed reference to every
/// [`AstKind::Dark`] node. Mirrors `bootstrap/src/main.rs::collect_dark`
/// in shape; lives here because the gap-extraction altitude is property,
/// not strict-mode reporting.
fn collect_dark<'a>(node: &'a AstNode, out: &mut Vec<&'a AstNode>) {
    if node.kind == AstKind::Dark {
        out.push(node);
    }
    for c in &node.children {
        collect_dark(c, out);
    }
}

/// Construct a level-0 [`Gap`] for one `total_classification` failure
/// (one [`AstKind::Dark`] region).
///
/// Per `docs/specs/gap-tension-tensor-substrate.md` §11 the Bateson
/// level reading is:
///
/// - **level 0** — floor-altitude single-claim gap (no binary opposition).
/// - **level 1** — binary opposition (a `contradiction`).
/// - **level 2+** — Bateson Learning III cocycle (gap *about gaps*).
///
/// A dark region is a floor-altitude `total_classification` failure: the
/// substrate has no rule for these bytes; there is no opposing claim, just
/// an unbounded section. Level 0.
///
/// `tension_summary` carries the source span `start..end` so downstream
/// consumers (T6 `tensor_of`'s MUS-graph construction; the kintsugi loop;
/// the scene curator) can address the specific bytes the gap names.
fn dark_gap(dark: &AstNode) -> Gap {
    let span = dark.dark_span;
    let summary = format!("dark region [{}, {})", span.start, span.end);
    Gap::new(0, total_classification_origin(), summary)
}

// ---------------------------------------------------------------------------
// gaps_of — the substrate's gap-extraction primitive at the AST altitude.
// ---------------------------------------------------------------------------

/// `gaps_of(ast: ast) -> [gap]` — extract every gap visible in an AST.
///
/// Per `docs/specs/gap-tension-tensor-substrate.md` §3.1 and
/// `docs/specs/property-and-inference-collapse.md` §9.2: the
/// compiler-side production of the gap-tensor field from a parsed AST.
/// The **first body that touches the actual property layer**.
///
/// Today's body iterates the substrate's one AST-altitude
/// known-evaluable property:
///
/// - `@epistemologic/property/total_classification` — every
///   [`AstKind::Dark`] node is a floor-altitude (level-0) gap.
///
/// As more properties get boundary-realizable verifiers, they get added
/// here as additional iteration steps. The shape stays: walk the AST per
/// property, surface each failure as a typed [`Gap`].
///
/// Pure; no I/O; allocates per the returned `Vec<Gap>`.
///
/// Returns `[]` when the AST is gap-free (every claim is verified or no
/// property failed at this altitude).
///
/// [`AstKind::Dark`]: crate::ast::AstKind::Dark
pub fn gaps_of(ast: &AstNode) -> Vec<Gap> {
    let mut gaps: Vec<Gap> = Vec::new();

    // Property 1: @epistemologic/property/total_classification.
    // Each AstKind::Dark node is one floor-altitude (level-0) failure.
    // Walk discipline matches `bootstrap/src/main.rs::collect_dark`'s
    // pre-order traversal so downstream consumers (T6's MUS-graph; the
    // kintsugi loop) see gaps in source order.
    let mut darks: Vec<&AstNode> = Vec::new();
    collect_dark(ast, &mut darks);
    for d in &darks {
        gaps.push(dark_gap(d));
    }

    // Future properties land here as additional iteration steps. When
    // a substrate-declared property registry lands (see module doc),
    // this body becomes a loop over the registry rather than a sequence
    // of per-property extractors.

    gaps
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`gaps_of`] — T5's `gaps_of` body. The
    //! first body that touches the property layer.
    //!
    //! Per `docs/specs/gap-tension-tensor-substrate.md` §3.1: `gaps_of`
    //! walks an AST, evaluates each substrate property against it, and
    //! returns a `[gap]` for each failure. Today's AST-altitude
    //! known-evaluable property is `total_classification`; tests below
    //! exercise the gap-free and gap-bearing branches plus the multi-
    //! dark-region accumulation discipline.
    //!
    //! Per AGENTS.md "TDD": these tests RED first; the body lands GREEN
    //! in the next commit.

    use super::*;
    use crate::ast::{AstKind, AstNode, DarkSpan};
    use crate::gap::Gap;

    /// A gap-free AST (no [`AstKind::Dark`] anywhere) yields the empty
    /// gap vector. The substrate's verdict at the AST altitude: every
    /// claim is verified or no property failed.
    #[test]
    fn gap_free_ast_yields_empty_vec() {
        // A bare Focus node, no children, no dark regions.
        let ast = AstNode::new(AstKind::Focus, "name");
        let gaps = gaps_of(&ast);
        assert!(
            gaps.is_empty(),
            "gap-free AST must yield [], got {} gap(s)",
            gaps.len()
        );
    }

    /// A nested gap-free AST (Project containing Focus, no Dark) also
    /// yields []. Verifies the walk is recursive; the gap-free verdict
    /// holds across the whole tree.
    #[test]
    fn nested_gap_free_ast_yields_empty_vec() {
        let mut root = AstNode::new(AstKind::Project, "outer");
        let inner = AstNode::new(AstKind::Focus, "inner");
        root.add_child(inner);
        let gaps = gaps_of(&root);
        assert!(gaps.is_empty(), "nested gap-free AST must yield []");
    }

    /// One [`AstKind::Dark`] node yields one [`Gap`] at the
    /// `@epistemologic/property/total_classification` origin, level 0,
    /// with the span recorded in `tension_summary`.
    ///
    /// Per `gap-tension-tensor-substrate.md` §11: a dark region is a
    /// floor-altitude single-claim gap; level 0.
    #[test]
    fn one_dark_region_yields_one_level_zero_gap() {
        let dark = AstNode::dark("unknown bytes", DarkSpan { start: 10, end: 23 });
        let gaps = gaps_of(&dark);
        assert_eq!(gaps.len(), 1, "one dark region yields one gap");
        let g = &gaps[0];
        assert_eq!(
            Gap::level(g),
            0,
            "dark region is a floor-altitude (level-0) gap per gap-tension-tensor-substrate.md §11"
        );
        assert_eq!(
            g.origin().as_str(),
            "@epistemologic/property/total_classification",
            "the dark gap's origin is the total_classification shard"
        );
        assert!(
            g.tension_summary().contains("10") && g.tension_summary().contains("23"),
            "tension summary must record the span; got {:?}",
            g.tension_summary()
        );
    }

    /// Multiple dark regions yield one gap each, in source order. The
    /// accumulation discipline matches `collect_dark`'s walk order so
    /// downstream consumers (T6's MUS-graph) see gaps in deterministic
    /// order.
    #[test]
    fn multiple_dark_regions_yield_one_gap_each_in_order() {
        let mut root = AstNode::new(AstKind::Project, "outer");
        let d1 = AstNode::dark("first", DarkSpan { start: 0, end: 5 });
        let d2 = AstNode::dark("second", DarkSpan { start: 6, end: 12 });
        root.add_child(d1);
        root.add_child(d2);
        let gaps = gaps_of(&root);
        assert_eq!(gaps.len(), 2, "two dark regions yield two gaps");
        // Order matches the walk.
        assert!(
            gaps[0].tension_summary().contains("0") && gaps[0].tension_summary().contains("5"),
            "first gap names first span; got {:?}",
            gaps[0].tension_summary()
        );
        assert!(
            gaps[1].tension_summary().contains("6") && gaps[1].tension_summary().contains("12"),
            "second gap names second span; got {:?}",
            gaps[1].tension_summary()
        );
    }

    /// A dark region nested inside a Project node still surfaces as a
    /// gap. The walk is structural; depth does not hide the gap.
    #[test]
    fn nested_dark_region_surfaces_as_gap() {
        let mut root = AstNode::new(AstKind::Project, "outer");
        let mut middle = AstNode::new(AstKind::Settle, "middle");
        let dark = AstNode::dark("hidden", DarkSpan { start: 42, end: 99 });
        middle.add_child(dark);
        root.add_child(middle);
        let gaps = gaps_of(&root);
        assert_eq!(gaps.len(), 1, "nested dark surfaces; depth does not hide");
        assert_eq!(Gap::level(&gaps[0]), 0);
        assert_eq!(
            gaps[0].origin().as_str(),
            "@epistemologic/property/total_classification"
        );
    }

    /// Type-level: the substrate signature `gaps_of(ast) -> [gap]` IS
    /// realized as `gaps_of(&AstNode) -> Vec<Gap>` at the boundary. This
    /// test is the type assertion; the value is the witness.
    #[test]
    fn gaps_of_returns_vec_of_gap() {
        let ast = AstNode::new(AstKind::Focus, "witness");
        let _gaps: Vec<Gap> = gaps_of(&ast);
    }
}
