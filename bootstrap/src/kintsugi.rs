//! `@kintsugi` — boundary Rust for the kintsugi-loop production engine.
//!
//! This module realizes substrate-declared actions from
//! [`docs/specs/gap-tension-tensor-substrate.md`] §3.2 and §11.1 as
//! Rust bodies. T7 in the implementation cascade after T6's
//! [`tensor_of`].
//!
//! ## What lives here
//!
//! - [`Fracture`] — the Rust mirror of the substrate's
//!   `fracture <= gap & { site: span }` from
//!   [`docs/specs/gap-tension-tensor-substrate.md`] §11.1. A
//!   substrate-marked gap the kintsugi loop is attempting to close.
//!   Inherits [`Gap`]'s carriers and adds the descent magnitude.
//! - [`minimize`] — the executable form of
//!   `minimize(tensor) -> [fracture]`. The **gradient-descent step on
//!   the Dirichlet energy** per
//!   [`docs/specs/gap-tension-tensor-substrate.md`] §6. Ranks tensions
//!   by descent magnitude (steepest first); emits the gap pair of each
//!   ranked tension as fractures.
//!
//! ## Why this lives here (not in `fracture.rs`, not in `tensor.rs`)
//!
//! The substrate-path-honest placement question for T7 surfaced two
//! candidates: `kintsugi.rs` (the broader composer altitude — future
//! `pulse`, `oscillate`-driver, `active_pass`, `dark_pass`, `query_phi`
//! land here too) vs `fracture.rs` (the narrower type-only altitude).
//! [`crate::tensor`] mirrors `@fate` (where `minimize` is declared per
//! [`docs/specs/gap-tension-tensor-substrate.md`] §2); [`crate::gap`]
//! mirrors `@epistemologic/property/gap`. `minimize` produces the
//! substrate's `[fracture]` — the kintsugi loop's mutation candidates
//! per [`docs/specs/gap-tension-tensor-substrate.md`] §6 — so the
//! composer-altitude home (`@kintsugi`) is the natural common
//! ancestor for the production-engine surface. Co-locating `Fracture`
//! here keeps the substrate-declared inheritance chain
//! `fracture <= gap` next to the action that emits it. When the future
//! `pulse` / `active_pass` / `dark_pass` driver lands, it composes
//! `minimize`'s output with `gaps_of` / `tensor_of` in this module.
//!
//! ## Why scalar-magnitude ranking (not SDRF curvature)
//!
//! T7 lands the **minimal first version**: rank tensions by
//! [`TensionVector::magnitude`] (steepest descent first); emit each
//! ranked tension's gap pair as a fracture. SDRF curvature ranking
//! (Topping et al. 2022, arXiv:2111.14522, Algorithm 1) requires the
//! Balanced Forman curvature on the tension graph, which in turn
//! requires the proper sheaf-Laplacian numerical primitive landing at
//! T8 (flang/mirror split). Today's tensor carries identity
//! restriction maps and uniform unit magnitudes; SDRF on this
//! structure would degenerate to vertex-degree counting — strictly
//! less informative than the magnitude reading the
//! [`TensionVector`] already carries. SDRF lifts when the conductivity
//! tensor declares its read at the boundary.
//!
//! ## What this is for
//!
//! Per [`docs/specs/gap-tension-tensor-substrate.md`] §6, the kintsugi
//! loop's job is:
//!
//! 1. `gaps_of(ast)` → `[gap]`  (T5)
//! 2. `tensor_of([gap])` → `tensor`  (T6)
//! 3. `minimize(tensor)` → `[fracture]`  (T7 — here)
//! 4. apply the fractures; settle  (future tick — composer level)
//!
//! T7 closes the floor: the gradient-descent step exists in Rust. The
//! production engine for the kintsugi loop can now be driven from the
//! boundary; the composers (pulse / oscillate-driver / active_pass /
//! dark_pass / query_phi) wrap this body in their substrate-pulled
//! shapes.
//!
//! [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
//! [`Gap`]: crate::gap::Gap
//! [`TensionVector`]: crate::tensor::TensionVector
//! [`TensionVector::magnitude`]: crate::tensor::TensionVector::magnitude
#![allow(dead_code)]

use crate::gap::Gap;
use crate::tensor::{Tension, TensionVector, Tensor};

// ---------------------------------------------------------------------------
// Fracture — the Rust mirror of `fracture <= gap & { site: span }`.
// ---------------------------------------------------------------------------

/// The substrate's fracture type at the kintsugi altitude.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §11.1:
///
/// ```mirror
/// type fracture <= gap & {
///   site:   span,                      # the `\` location
/// }
/// ```
///
/// A `fracture` is a **specific gap the substrate is attempting to
/// close** — the descent target along the Dirichlet-energy gradient
/// per [`docs/specs/gap-tension-tensor-substrate.md`] §6. The
/// `<= gap` inheritance is realised in Rust by carrying a [`Gap`]
/// value directly (composition-as-inheritance per
/// `prism-as-trait-as-everything`); the additional structure named in
/// §11.1 — the descent magnitude (audible-altitude floor) and the
/// substrate "site" (read at this altitude as the source [`Gap`]'s
/// origin) — lives as named fields on the wrapper.
///
/// ## Audible-altitude shape (T7 minimum)
///
/// - [`gap`](Fracture::gap) — the inherited [`Gap`] (substrate-pull:
///   the `<= gap` chain is direct composition; the gap's `origin` IS
///   the substrate site per §11.1's `site: span` reading at this
///   altitude).
/// - [`descent`](Fracture::descent) — the magnitude of the
///   gradient-descent step the substrate would take to close this
///   fracture. Scalar in `[0, 1]`; inherited from the emitting
///   [`Tension`]'s [`TensionVector::magnitude`]. The full directed
///   tangent-space element lands when §8.1 closes.
///
/// ## Inheritance pattern
///
/// `Fracture` does NOT re-declare [`Gap`]'s carriers (`level`,
/// `origin`, `tension_summary`) — they are reached through
/// [`gap`](Fracture::gap). The substrate's `<= gap` chain becomes Rust
/// composition; mirror's read of `<=` is "implements" (per
/// `prism-as-trait-as-everything`) which at the type-mirror altitude
/// becomes "carries". Downstream consumers that need the level / origin
/// of the underlying gap reach through the accessor.
///
/// [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
/// [`Gap`]: crate::gap::Gap
/// [`Tension`]: crate::tensor::Tension
/// [`TensionVector::magnitude`]: crate::tensor::TensionVector::magnitude
#[derive(Clone, Debug, PartialEq)]
pub struct Fracture {
    /// The inherited [`Gap`] — substrate-pull realisation of
    /// `fracture <= gap`. Carries the Bateson level, the substrate
    /// origin, and the audible-altitude tension summary.
    gap: Gap,
    /// The magnitude of the gradient-descent step the substrate would
    /// take to close this fracture. Inherited from the emitting
    /// [`Tension`]'s [`TensionVector::magnitude`] per
    /// [`docs/specs/gap-tension-tensor-substrate.md`] §6's
    /// rank-by-magnitude step. In `[0, 1]`.
    ///
    /// [`Tension`]: crate::tensor::Tension
    /// [`TensionVector::magnitude`]: crate::tensor::TensionVector::magnitude
    descent: f64,
}

impl Fracture {
    /// Construct a fracture from its inherited [`Gap`] and descent
    /// magnitude. Most callers want [`minimize`] — this constructor is
    /// for tests and downstream consumers that build fractures from
    /// non-tensor sources. `descent` is clamped to `[0, 1]`.
    pub fn new(gap: Gap, descent: f64) -> Self {
        Fracture {
            gap,
            descent: descent.clamp(0.0, 1.0),
        }
    }

    /// Borrow the inherited [`Gap`] — the substrate-pull realisation
    /// of the `<= gap` inheritance chain.
    pub fn gap(f: &Fracture) -> &Gap {
        &f.gap
    }

    /// Read this fracture's descent magnitude — how hard the gradient
    /// pulls toward closing this gap.
    pub fn descent(f: &Fracture) -> f64 {
        f.descent
    }
}

// ---------------------------------------------------------------------------
// minimize — the gradient-descent step on the Dirichlet energy.
// ---------------------------------------------------------------------------

/// `minimize(t: tensor) -> [fracture]` — the gradient-descent step.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §6 and §3.2: the
/// kintsugi loop's mutation-candidate emission. Takes a [`Tensor`]
/// (the gap-tension field built by [`tensor_of`]); reads each
/// [`Tension`]'s descent magnitude; ranks tensions steepest-first;
/// emits each ranked tension's gap pair as a [`Fracture`].
///
/// ## Algorithm (audible-altitude floor)
///
/// 1. For each [`Tension`] in the tensor, read the
///    [`TensionVector::magnitude`] as the scalar descent magnitude.
///    (The full directed tangent-space element lands when §8.1 closes.)
/// 2. Rank tensions by descent magnitude, **largest first** (steepest
///    descent per §6 step 1). Stable sort preserves source-order
///    discipline under ties.
/// 3. For each ranked tension, emit one [`Fracture`] per gap endpoint
///    (both `a` and `b` are substrate-marked descent candidates per
///    §3.2's directed-pull shape). The emission order is
///    `(t1.a, t1.b, t2.a, t2.b, …)` — tensions by rank, gaps in
///    `(a, b)` order within each tension.
/// 4. Return the [`Vec<Fracture>`].
///
/// ## Why both endpoints (not just one)
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §3.2's `vector`
/// field shape: "the vector names the direction the tension pulls when
/// minimized: which gap closes, at what cost to the other." Both gaps
/// are descent candidates; the kintsugi loop's selection between them
/// happens at the application altitude (confidence-aware dispatch per
/// `kintsugi-fracture-confidence-and-scene-dispatch.md`). The
/// substrate's `minimize` surfaces both; downstream picks.
///
/// ## Boundary cases
///
/// - **Empty tensor** (no tensions): returns empty [`Vec<Fracture>`].
///   The substrate's gradient on a trivial sheaf is the additive
///   identity; nothing to mutate.
/// - **Singleton vertex** (no tensions): returns empty
///   [`Vec<Fracture>`]. Same reason — no tensions means no descent
///   direction at this altitude.
/// - **Disconnected components**: tensions from each component
///   contribute fractures independently. The ranking is global by
///   magnitude (per-component ranking lifts when sheaf restriction
///   maps land in T8).
///
/// Pure; no I/O; allocates per the returned [`Vec`].
///
/// [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
/// [`Tension`]: crate::tensor::Tension
/// [`TensionVector::magnitude`]: crate::tensor::TensionVector::magnitude
/// [`tensor_of`]: crate::tensor::tensor_of
pub fn minimize(t: &Tensor) -> Vec<Fracture> {
    // Phase 1: rank tensions by descent magnitude (steepest first).
    // Stable sort preserves the substrate's source-order discipline
    // among ties — matches `gaps_of`'s pre-order emission so
    // downstream consumers see deterministic fracture sequences.
    let mut ranked: Vec<&Tension> = Tensor::tensions(t).iter().collect();
    ranked.sort_by(|x, y| {
        let mx = TensionVector::magnitude(Tension::vector(x));
        let my = TensionVector::magnitude(Tension::vector(y));
        // Reverse: largest magnitude first (steepest descent per §6).
        my.partial_cmp(&mx).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Phase 2: emit each ranked tension's gap pair as fractures.
    // Both endpoints surface per §3.2's directed-pull shape;
    // confidence-aware dispatch at the application altitude picks.
    let mut fractures: Vec<Fracture> = Vec::with_capacity(ranked.len() * 2);
    for tension in ranked {
        let magnitude = TensionVector::magnitude(Tension::vector(tension));
        fractures.push(Fracture::new(Tension::a(tension).clone(), magnitude));
        fractures.push(Fracture::new(Tension::b(tension).clone(), magnitude));
    }
    fractures
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`minimize`] and [`Fracture`] — T7's
    //! `minimize` body. The kintsugi loop's gradient-descent step.
    //!
    //! Per `docs/specs/gap-tension-tensor-substrate.md` §6 and §3.2:
    //! `minimize` takes a tensor, ranks tensions by descent magnitude,
    //! and emits the substrate's mutation candidates as
    //! `fracture <= gap` values. These tests RED first; the body lands
    //! GREEN in the next commit.

    use super::*;
    use crate::gap::Gap;
    use crate::tensor::{tensor_of, Tension, TensionVector, Tensor};
    use prism_core::Ref;

    fn total_origin() -> Ref {
        Ref::new("@epistemologic/property/total_classification").expect("valid ref")
    }

    fn other_origin() -> Ref {
        Ref::new("@epistemologic/property/strict_classification").expect("valid ref")
    }

    // -----------------------------------------------------------------------
    // Fracture's substrate-pull shape — the inheritance chain `fracture <= gap`.
    // -----------------------------------------------------------------------

    /// Fracture carries an inherited [`Gap`] (the `<= gap` chain) and
    /// a descent magnitude. The accessor reaches the inherited gap so
    /// downstream consumers can read its level / origin / summary.
    #[test]
    fn fracture_carries_inherited_gap_and_descent() {
        let g = Gap::new(0, total_origin(), "dark region [0, 5)");
        let f = Fracture::new(g.clone(), 0.75);
        assert_eq!(Fracture::gap(&f), &g);
        assert!((Fracture::descent(&f) - 0.75).abs() < 1e-12);
    }

    /// The descent magnitude is clamped to `[0, 1]` — the audible-
    /// altitude floor reading per [`TensionVector`]'s clamp discipline.
    #[test]
    fn fracture_descent_clamps_to_unit_interval() {
        let g = Gap::new(0, total_origin(), "test");
        let f = Fracture::new(g.clone(), 1.5);
        assert_eq!(Fracture::descent(&f), 1.0);
        let f = Fracture::new(g.clone(), -0.5);
        assert_eq!(Fracture::descent(&f), 0.0);
        let f = Fracture::new(g, 0.42);
        assert!((Fracture::descent(&f) - 0.42).abs() < 1e-12);
    }

    // -----------------------------------------------------------------------
    // minimize on the substrate's boundary cases.
    // -----------------------------------------------------------------------

    /// Empty tensor → empty fracture vector. The substrate's gradient
    /// on the trivial sheaf is the additive identity; nothing to
    /// mutate at this altitude.
    #[test]
    fn empty_tensor_yields_no_fractures() {
        let t = tensor_of(Vec::new());
        let fractures = minimize(&t);
        assert!(
            fractures.is_empty(),
            "empty tensor yields no fractures; got {} fracture(s)",
            fractures.len(),
        );
    }

    /// Singleton-vertex tensor → empty fracture vector. No tensions
    /// means no descent direction at this altitude.
    #[test]
    fn singleton_vertex_tensor_yields_no_fractures() {
        let g = Gap::new(0, total_origin(), "dark region [0, 5)");
        let t = tensor_of(vec![g]);
        let fractures = minimize(&t);
        assert!(
            fractures.is_empty(),
            "singleton-vertex tensor yields no fractures; got {} fracture(s)",
            fractures.len(),
        );
    }

    /// Disconnected tensor (two same-origin gaps + one isolated other-
    /// origin gap; K₂ ⊔ K₁) → two fractures from the single tension
    /// (one per endpoint); the isolated vertex contributes nothing.
    #[test]
    fn disconnected_tensor_yields_fractures_from_connected_components_only() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let g3 = Gap::new(0, other_origin(), "strict failure");
        let t = tensor_of(vec![g1.clone(), g2.clone(), g3]);
        let fractures = minimize(&t);
        assert_eq!(
            fractures.len(),
            2,
            "K₂ ⊔ K₁ yields one tension (two endpoint fractures); got {}",
            fractures.len(),
        );
        // Both fractures must come from the K₂ component.
        let gaps: Vec<&Gap> = fractures.iter().map(Fracture::gap).collect();
        assert!(gaps.contains(&&g1) && gaps.contains(&&g2));
    }

    // -----------------------------------------------------------------------
    // The K₂ case — one tension, two endpoint fractures.
    // -----------------------------------------------------------------------

    /// K₂ tensor (two same-origin gaps; one tension) → two fractures
    /// (one per endpoint). Both endpoints are substrate-marked descent
    /// candidates per §3.2's directed-pull shape.
    #[test]
    fn k2_tensor_yields_one_fracture_per_endpoint() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let t = tensor_of(vec![g1.clone(), g2.clone()]);
        let fractures = minimize(&t);
        assert_eq!(
            fractures.len(),
            2,
            "K₂ tension yields one fracture per endpoint",
        );
        // Both endpoints are marked as descent candidates; both
        // fractures carry the tension's audible-altitude magnitude.
        assert_eq!(Fracture::gap(&fractures[0]), &g1);
        assert_eq!(Fracture::gap(&fractures[1]), &g2);
        // Audible-altitude floor: uniform magnitude 1.0 per
        // tensor_of's construction.
        assert!((Fracture::descent(&fractures[0]) - 1.0).abs() < 1e-12);
        assert!((Fracture::descent(&fractures[1]) - 1.0).abs() < 1e-12);
    }

    /// K₃ tensor (three same-origin gaps; three tensions) → six
    /// fractures (two endpoint candidates per tension).
    #[test]
    fn k3_tensor_yields_six_fractures() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let g3 = Gap::new(0, total_origin(), "dark [20, 25)");
        let t = tensor_of(vec![g1, g2, g3]);
        let fractures = minimize(&t);
        assert_eq!(
            fractures.len(),
            6,
            "K₃ has three tensions; each yields two endpoint fractures; total 6",
        );
    }

    // -----------------------------------------------------------------------
    // Ranking — steepest descent first.
    // -----------------------------------------------------------------------

    /// Tensions are ranked by descent magnitude (largest first). When
    /// magnitudes differ, the higher-magnitude tension's fractures
    /// surface before the lower-magnitude tension's.
    #[test]
    fn fractures_are_ranked_by_descent_magnitude_largest_first() {
        let g_a = Gap::new(0, total_origin(), "high-magnitude a");
        let g_b = Gap::new(0, total_origin(), "high-magnitude b");
        let g_c = Gap::new(0, total_origin(), "low-magnitude a");
        let g_d = Gap::new(0, total_origin(), "low-magnitude b");

        let high = Tension::new(g_a.clone(), g_b.clone(), TensionVector::new(0.9));
        let low = Tension::new(g_c.clone(), g_d.clone(), TensionVector::new(0.2));
        // Construct a tensor by hand so we control the ordering and
        // magnitudes — `tensor_of` yields uniform magnitudes; this test
        // exercises the rank-by-magnitude branch directly.
        let t = Tensor::new(
            vec![g_a.clone(), g_b.clone(), g_c.clone(), g_d.clone()],
            // Deliberately list `low` before `high` to prove the sort
            // moves `high` ahead of `low`.
            vec![low.clone(), high.clone()],
            0.0,
        );
        let fractures = minimize(&t);
        assert_eq!(fractures.len(), 4);
        // High-magnitude tension's endpoints come first.
        assert_eq!(Fracture::gap(&fractures[0]), &g_a);
        assert_eq!(Fracture::gap(&fractures[1]), &g_b);
        assert!((Fracture::descent(&fractures[0]) - 0.9).abs() < 1e-12);
        assert!((Fracture::descent(&fractures[1]) - 0.9).abs() < 1e-12);
        // Low-magnitude tension's endpoints come after.
        assert_eq!(Fracture::gap(&fractures[2]), &g_c);
        assert_eq!(Fracture::gap(&fractures[3]), &g_d);
        assert!((Fracture::descent(&fractures[2]) - 0.2).abs() < 1e-12);
        assert!((Fracture::descent(&fractures[3]) - 0.2).abs() < 1e-12);
    }

    /// Tied magnitudes preserve source order (stable sort). The
    /// substrate's discipline: deterministic fracture sequences even
    /// under ties.
    #[test]
    fn tied_magnitudes_preserve_source_order() {
        let g1 = Gap::new(0, total_origin(), "first");
        let g2 = Gap::new(0, total_origin(), "second");
        let g3 = Gap::new(0, total_origin(), "third");
        let g4 = Gap::new(0, total_origin(), "fourth");
        let t1 = Tension::new(g1.clone(), g2.clone(), TensionVector::new(0.5));
        let t2 = Tension::new(g3.clone(), g4.clone(), TensionVector::new(0.5));
        let tensor = Tensor::new(
            vec![g1.clone(), g2.clone(), g3.clone(), g4.clone()],
            vec![t1, t2],
            0.0,
        );
        let fractures = minimize(&tensor);
        assert_eq!(fractures.len(), 4);
        // First tension's endpoints come first under stable sort.
        assert_eq!(Fracture::gap(&fractures[0]), &g1);
        assert_eq!(Fracture::gap(&fractures[1]), &g2);
        assert_eq!(Fracture::gap(&fractures[2]), &g3);
        assert_eq!(Fracture::gap(&fractures[3]), &g4);
    }

    // -----------------------------------------------------------------------
    // The substrate's source-tension carrier — fractures address the cocycle.
    // -----------------------------------------------------------------------

    /// Each fracture's inherited [`Gap`] addresses the substrate
    /// location where the descent step would land. The gap's `origin`
    /// IS the substrate "site" per §11.1's `site: span` reading at
    /// this altitude.
    #[test]
    fn fracture_inherits_gap_origin_for_addressability() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let t = tensor_of(vec![g1.clone(), g2.clone()]);
        let fractures = minimize(&t);
        for f in &fractures {
            assert_eq!(
                Fracture::gap(f).origin(),
                &total_origin(),
                "fracture must carry the substrate origin of its inherited gap",
            );
            assert_eq!(
                Gap::level(Fracture::gap(f)),
                0,
                "K₂ same-origin tensions are level-0 (floor-altitude) gaps",
            );
        }
    }

    /// Type-level: the substrate signature `minimize(tensor) -> [fracture]`
    /// IS realized as `minimize(&Tensor) -> Vec<Fracture>` at the
    /// boundary. The value witnesses the type.
    #[test]
    fn minimize_returns_vec_of_fracture() {
        let t = tensor_of(Vec::new());
        let _fractures: Vec<Fracture> = minimize(&t);
    }
}
