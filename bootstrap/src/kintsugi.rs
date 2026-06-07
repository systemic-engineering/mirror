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
//! ## SDRF curvature ranking (Topping 2022)
//!
//! T9 supersedes T7's scalar-magnitude ranking with **Balanced Forman
//! curvature** ranking per Topping et al. 2022 (arXiv:2111.14522)
//! Algorithm 1. The MUS-graph underlying [`Tensor`]'s tension set is
//! treated as the substrate's `operator` (per
//! [`shards/epistemologic/math/sheaf_laplacian.mirror`]'s declaration);
//! [`crate::curvature::balanced_forman`] reads the per-edge Balanced
//! Forman curvature `Ric(i, j)` (per
//! [`shards/epistemologic/math/curvature.mirror`]'s declaration);
//! `minimize` ranks tensions by **most-negative curvature first** —
//! the bottleneck-y edges SDRF Algorithm 1's outer selection step
//! identifies as the rewrite targets.
//!
//! ### `Fracture.descent` semantics
//!
//! `Fracture.descent` carries the SDRF-derived "how hard the substrate
//! pulls toward closing this fracture" reading:
//!
//! ```text
//! descent = ((−Ric + 2.0) / 4.0).clamp(0, 1)
//! ```
//!
//! Mapping (Topping 2022 §4 Proposition 4 bounds `Ric > −2` on every
//! edge):
//!
//! - `Ric = −2` (deep bottleneck)        → `descent = 1.0` (max pull)
//! - `Ric = 0`  (neutral)                 → `descent = 0.5`
//! - `Ric = +2` (well-connected K_n edge) → `descent = 0.0` (no pull)
//!
//! Monotone in `−Ric`: ranking by `descent` (largest first) is
//! identical to ranking by `Ric` (smallest first / most negative
//! first). The descent field surfaces the SDRF signal in a unit-
//! interval shape; the raw curvature is readable on the operator via
//! [`crate::curvature::balanced_forman`].
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

use crate::curvature::{balanced_forman, Curvature};
use crate::gap::Gap;
use crate::sheaf_laplacian::{Operator, Restriction};
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
///   fracture. Scalar in `[0, 1]`; SDRF-derived from the emitting
///   tension edge's Balanced Forman curvature per
///   `descent = ((−Ric + 2.0) / 4.0).clamp(0, 1)`. The full directed
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
    /// take to close this fracture. SDRF-derived from the emitting
    /// tension edge's Balanced Forman curvature per Topping et al.
    /// 2022 (arXiv:2111.14522) §4 Algorithm 1:
    /// `descent = ((−Ric + 2.0) / 4.0).clamp(0, 1)`. In `[0, 1]`.
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
/// ## Algorithm (SDRF curvature ranking per Topping 2022 Algorithm 1)
///
/// 1. Reconstruct the MUS-graph as an [`Operator`] from the tensor's
///    vertex basis and tensions (re-indexing each [`Tension`]'s
///    cloned `a` / `b` gaps against the vertex order yields a
///    [`Restriction`] per tension).
/// 2. For each tension, read the per-edge Balanced Forman curvature
///    `Ric(i, j)` via [`balanced_forman`] on the constructed operator
///    — Topping et al. 2022 §4 Definition 1 equation (3).
/// 3. Rank tensions by **most-negative curvature first** (SDRF
///    Algorithm 1's outer selection: the edge with minimal
///    `Ric(i, j)` is the rewrite target). Stable sort preserves
///    source-order discipline under ties.
/// 4. For each ranked tension, emit one [`Fracture`] per gap endpoint
///    (both `a` and `b` are substrate-marked descent candidates per
///    §3.2's directed-pull shape). `Fracture.descent` carries the
///    SDRF-derived `((−Ric + 2.0) / 4.0).clamp(0, 1)` reading so the
///    descent field surfaces the curvature signal in a unit-interval
///    shape. Emission order is `(t1.a, t1.b, t2.a, t2.b, …)`.
/// 5. Return the [`Vec<Fracture>`].
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
    // RED phase: stub body. The GREEN commit lands the SDRF curvature
    // ranking that consumes balanced_forman per tension and ranks by
    // most-negative curvature first per Topping 2022 Algorithm 1.
    let _ = (t, balanced_forman, Curvature::value, Operator::new, Restriction::new, TensionVector::magnitude, Tension::a, Tension::b, Tension::vector, Tensor::vertices, Tensor::tensions);
    unimplemented!("T9 RED: SDRF curvature-ranked minimize lands in the GREEN commit")
}

/// Map a Balanced Forman curvature value to a `[0, 1]` descent reading
/// per the SDRF semantics in [`minimize`]: monotone in `−Ric`, so
/// largest descent corresponds to most negative curvature.
///
/// ```text
/// descent = ((−Ric + 2.0) / 4.0).clamp(0, 1)
/// ```
///
/// `Ric = −2` (Topping 2022 §4 Proposition 4 lower bound) → `1.0`;
/// `Ric = 0` (neutral) → `0.5`; `Ric = +2` (K_n edge) → `0.0`.
/// Curvature at or above `+2` saturates to `0.0`; the bound `Ric > −2`
/// keeps the unsaturated upper tail away from `1.0` for realisable
/// edges, leaving headroom that distinguishes deeper-bottleneck cases
/// the substrate might encounter in the future.
fn descent_from_curvature(_ric: f64) -> f64 {
    // RED phase: stub. The GREEN commit lands the
    // ((-Ric + 2.0) / 4.0).clamp(0, 1) mapping.
    unimplemented!("T9 RED: descent_from_curvature lands in the GREEN commit")
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`minimize`] and [`Fracture`].
    //!
    //! Per `docs/specs/gap-tension-tensor-substrate.md` §6 and §3.2:
    //! `minimize` takes a tensor and emits the substrate's mutation
    //! candidates as `fracture <= gap` values. T9 supersedes T7's scalar-
    //! magnitude ranking with SDRF Balanced Forman curvature ranking
    //! (Topping et al. 2022, arXiv:2111.14522 §4 Algorithm 1): most-
    //! negative curvature first. `Fracture.descent` carries the SDRF-
    //! derived `((−Ric + 2.0) / 4.0).clamp(0, 1)` reading.
    //!
    //! Canonical descent values for common-fixture edges:
    //!
    //! - K₂ (one tension, d=1 each)        → Ric = 0,    descent = 0.5
    //! - K₃ (three tensions, d=2 + triangle) → Ric = 2,   descent = 0.0
    //! - K₄ (six tensions, d=3 + triangles)  → Ric = 2,   descent = 0.0
    //! - Barbell bridge (d=3, no triangle)   → Ric = -2/3, descent ≈ 0.667

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
    /// candidates per §3.2's directed-pull shape. SDRF: K₂ edge has
    /// min degree 1 → Ric = 0 (Topping 2022 Definition 1 degenerate)
    /// → descent = ((-0 + 2.0) / 4.0) = 0.5.
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
        // fractures carry the SDRF-derived descent.
        assert_eq!(Fracture::gap(&fractures[0]), &g1);
        assert_eq!(Fracture::gap(&fractures[1]), &g2);
        // SDRF: K₂ edge degenerate (min degree 1) → Ric = 0 →
        // descent = 0.5 (neutral curvature reading).
        assert!(
            (Fracture::descent(&fractures[0]) - 0.5).abs() < 1e-9,
            "K₂ fracture descent must be 0.5 (Ric=0 → neutral); got {}",
            Fracture::descent(&fractures[0]),
        );
        assert!((Fracture::descent(&fractures[1]) - 0.5).abs() < 1e-9);
    }

    /// K₃ tensor (three same-origin gaps; three tensions) → six
    /// fractures (two endpoint candidates per tension). SDRF: K₃ edge
    /// has d=2, one triangle → Ric = 2 → descent = 0 (well-connected,
    /// no SDRF pull). All three tensions tied at Ric=2; source order
    /// preserved.
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
        // K₃ well-connected → descent = 0 (no pull on any edge).
        for f in &fractures {
            assert!(
                Fracture::descent(f) < 1e-9,
                "K₃ fracture descent must be 0 (Ric=2, well-connected); got {}",
                Fracture::descent(f),
            );
        }
    }

    // -----------------------------------------------------------------------
    // SDRF curvature ranking — most-negative curvature first.
    // -----------------------------------------------------------------------

    /// Tensions with tied curvature (e.g., two disjoint K₂ edges, both
    /// with Ric=0) preserve source order under the stable sort. The
    /// substrate's discipline: deterministic fracture sequences even
    /// under ties.
    #[test]
    fn tied_curvature_preserves_source_order() {
        let g1 = Gap::new(0, total_origin(), "first");
        let g2 = Gap::new(0, total_origin(), "second");
        let g3 = Gap::new(0, total_origin(), "third");
        let g4 = Gap::new(0, total_origin(), "fourth");
        let t1 = Tension::new(g1.clone(), g2.clone(), TensionVector::new(1.0));
        let t2 = Tension::new(g3.clone(), g4.clone(), TensionVector::new(1.0));
        let tensor = Tensor::new(
            vec![g1.clone(), g2.clone(), g3.clone(), g4.clone()],
            vec![t1, t2],
            0.0,
        );
        let fractures = minimize(&tensor);
        assert_eq!(fractures.len(), 4);
        // Both edges have min degree 1 → Ric=0 → tied; source order
        // preserves: t1's endpoints (g1, g2) before t2's (g3, g4).
        assert_eq!(Fracture::gap(&fractures[0]), &g1);
        assert_eq!(Fracture::gap(&fractures[1]), &g2);
        assert_eq!(Fracture::gap(&fractures[2]), &g3);
        assert_eq!(Fracture::gap(&fractures[3]), &g4);
    }

    /// SDRF ranking moves the most-negatively-curved edge ahead of
    /// well-connected edges. Construction: a barbell — two K₃ triangles
    /// {0,1,2} and {3,4,5} joined by a single bridge tension (2,3).
    /// The bridge edge has Ric = -2/3 (bottleneck); the K₃ edges have
    /// Ric = 2 (well-connected). SDRF: bridge fractures surface FIRST.
    #[test]
    fn sdrf_ranks_bridge_edge_first_in_barbell() {
        // Six same-origin gaps form the barbell vertex basis.
        let gs: Vec<Gap> = (0..6)
            .map(|i| Gap::new(0, total_origin(), format!("v{}", i)))
            .collect();
        let mk = |a: usize, b: usize| {
            Tension::new(gs[a].clone(), gs[b].clone(), TensionVector::new(1.0))
        };
        // K₃ {0,1,2} + K₃ {3,4,5} + bridge (2,3).
        let tensions = vec![
            mk(0, 1),
            mk(0, 2),
            mk(1, 2),
            mk(3, 4),
            mk(3, 5),
            mk(4, 5),
            mk(2, 3), // bridge — listed LAST in source order
        ];
        let tensor = Tensor::new(gs.clone(), tensions, 0.0);
        let fractures = minimize(&tensor);
        // 7 tensions → 14 fractures.
        assert_eq!(fractures.len(), 14);
        // Bridge tension (gs[2], gs[3]) must surface FIRST despite
        // being listed last in source order — SDRF ranks most-negative
        // curvature first.
        assert_eq!(Fracture::gap(&fractures[0]), &gs[2]);
        assert_eq!(Fracture::gap(&fractures[1]), &gs[3]);
        // Bridge edge has Ric = -2/3 → descent = ((2/3 + 2)/4) = 2/3.
        assert!(
            (Fracture::descent(&fractures[0]) - 2.0 / 3.0).abs() < 1e-9,
            "bridge fracture descent must be 2/3 (Ric=-2/3); got {}",
            Fracture::descent(&fractures[0]),
        );
        // Remaining fractures are K₃ edges: those NOT adjacent to the
        // bridge endpoint (e.g., (0,1) and (3,4)) have Ric=2 →
        // descent=0. K₃ edges adjacent to the bridge endpoint (e.g.,
        // (0,2), (1,2), (3,5), (4,5)) have Ric = 4/3 → descent ≈ 1/6
        // because the bridge inflates the degree of vertex 2 (or 3)
        // to 3, changing the formula. Both descents are STRICTLY LESS
        // than the bridge's 2/3 — SDRF ordering invariant holds.
        for f in &fractures[2..] {
            assert!(
                Fracture::descent(f) < 2.0 / 3.0 - 1e-9,
                "K₃ leg fractures must rank below bridge; got descent {}",
                Fracture::descent(f),
            );
        }
    }

    /// SDRF descent reading is monotone in -Ric: a P₄ middle edge
    /// (Ric=0, descent=0.5) ranks between a barbell bridge (Ric=-2/3,
    /// descent=2/3) and a K₃ edge (Ric=2, descent=0).
    #[test]
    fn sdrf_descent_is_monotone_in_negative_curvature() {
        // Mixed graph: barbell bridge (Ric=-2/3) + isolated P₄ middle
        // edge (Ric=0) + K₃ well-connected edge (Ric=2).
        // Build the three sub-graphs disjointly on a shared basis.
        // K₃ {0,1,2} + barbell bridge {3,4,5,6} (two K₂ triangles via
        // {3,4,5} would clash; simpler: explicit barbell {3,4,5}∈K₃ +
        // {7,8,9}∈K₃ + bridge (5,7); + isolated P₄ {10,11,12,13}).
        // We focus the test on the descent ORDERING property.
        let gs: Vec<Gap> = (0..14)
            .map(|i| Gap::new(0, total_origin(), format!("v{}", i)))
            .collect();
        let mk = |a: usize, b: usize| {
            Tension::new(gs[a].clone(), gs[b].clone(), TensionVector::new(1.0))
        };
        let tensions = vec![
            // K₃ on {0,1,2}: all edges Ric=2
            mk(0, 1), mk(0, 2), mk(1, 2),
            // Barbell K₃{3,4,5} + K₃{7,8,9} + bridge (5,7):
            mk(3, 4), mk(3, 5), mk(4, 5),
            mk(7, 8), mk(7, 9), mk(8, 9),
            mk(5, 7),  // bridge — Ric=-2/3
            // P₄ {10,11,12,13}: middle edge (11,12) has Ric=0
            mk(10, 11), mk(11, 12), mk(12, 13),
        ];
        let tensor = Tensor::new(gs.clone(), tensions, 0.0);
        let fractures = minimize(&tensor);

        // The first fracture pair is the barbell bridge (descent=2/3).
        assert!(
            Fracture::descent(&fractures[0]) > 0.6,
            "highest descent (bridge) must exceed 0.6; got {}",
            Fracture::descent(&fractures[0]),
        );
        // The last fracture pair is a K₃ edge (descent=0).
        let last = Fracture::descent(&fractures[fractures.len() - 1]);
        assert!(
            last < 1e-9,
            "lowest descent (K₃) must be 0; got {}",
            last,
        );
        // Descent sequence is monotonically non-increasing.
        for w in fractures.windows(2) {
            assert!(
                Fracture::descent(&w[0]) + 1e-12 >= Fracture::descent(&w[1]),
                "descent must be non-increasing; got {} then {}",
                Fracture::descent(&w[0]),
                Fracture::descent(&w[1]),
            );
        }
    }

    /// descent_from_curvature mapping: Ric = -2 → 1.0; Ric = 0 → 0.5;
    /// Ric = +2 → 0.0; Ric outside the (-2, +2) interval clamps.
    #[test]
    fn descent_from_curvature_maps_canonical_values() {
        assert!((descent_from_curvature(-2.0) - 1.0).abs() < 1e-12);
        assert!((descent_from_curvature(0.0) - 0.5).abs() < 1e-12);
        assert!((descent_from_curvature(2.0) - 0.0).abs() < 1e-12);
        // Clamp on extremes.
        assert_eq!(descent_from_curvature(-10.0), 1.0);
        assert_eq!(descent_from_curvature(10.0), 0.0);
        // Infinity (defensive: unindexed tension): descent = 0.
        assert_eq!(descent_from_curvature(f64::INFINITY), 0.0);
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

    // -----------------------------------------------------------------------
    // T8.5 bridge composes with SDRF: weighted-tensor minimize still
    // produces the SDRF curvature ranking because curvature is
    // topological (per Topping 2022 Definition 1 the formula reads
    // degrees and triangle / 4-cycle counts — not edge weights).
    //
    // The bridge composition: weights flow through tensor_of_with_
    // restrictions into the tension graph's CONNECTIVITY; SDRF reads
    // the connectivity's curvature; minimize ranks by it. The bridge
    // remains load-bearing for future work that adds weighted forms
    // (Ollivier-Ricci with edge-weighted Wasserstein, etc.).
    // -----------------------------------------------------------------------

    use crate::sheaf_laplacian::Restriction;
    use crate::tensor::tensor_of_with_restrictions;

    /// `minimize` on a weighted-bridge tensor (K₂ ⊔ K₂) ranks the two
    /// edges identically because both edges are degenerate (min degree
    /// 1 → Ric = 0). The post-T9 contract: ranking is by curvature, NOT
    /// by consumer weight — weights flow into connectivity, curvature
    /// reads the connectivity. Descent on tied curvature is uniform
    /// (0.5).
    #[test]
    fn minimize_on_weighted_disjoint_edges_ties_on_curvature() {
        let g0 = Gap::new(0, total_origin(), "low pair a");
        let g1 = Gap::new(0, total_origin(), "low pair b");
        let g2 = Gap::new(0, total_origin(), "high pair a");
        let g3 = Gap::new(0, total_origin(), "high pair b");
        let t = tensor_of_with_restrictions(
            vec![g0.clone(), g1.clone(), g2.clone(), g3.clone()],
            vec![Restriction::new(0, 1, 0.2), Restriction::new(2, 3, 0.9)],
        );
        let fractures = minimize(&t);
        assert_eq!(fractures.len(), 4);
        // Both edges are K₂ (min degree 1 → Ric = 0); tied; source
        // order preserves the (g0, g1) pair before the (g2, g3) pair.
        assert_eq!(Fracture::gap(&fractures[0]), &g0);
        assert_eq!(Fracture::gap(&fractures[1]), &g1);
        assert_eq!(Fracture::gap(&fractures[2]), &g2);
        assert_eq!(Fracture::gap(&fractures[3]), &g3);
        // All descents = 0.5 (SDRF neutral on K₂ edges).
        for f in &fractures {
            assert!(
                (Fracture::descent(f) - 0.5).abs() < 1e-9,
                "K₂ ⊔ K₂ fracture descent must be 0.5; got {}",
                Fracture::descent(f),
            );
        }
    }

    /// Weighted-tensor minimize reproduces SDRF ranking on a barbell:
    /// the bridge edge (regardless of its weight) is identified as the
    /// bottleneck and surfaces first. Topological curvature reading is
    /// weight-independent per Topping 2022 §4 Definition 1.
    #[test]
    fn minimize_weighted_barbell_still_ranks_bridge_first() {
        let gs: Vec<Gap> = (0..6)
            .map(|i| Gap::new(0, total_origin(), format!("v{}", i)))
            .collect();
        // Barbell: K₃ {0,1,2} + K₃ {3,4,5} + bridge (2,3). Bridge
        // gets a SMALL consumer weight (0.1); K₃ edges get LARGE
        // weights (0.9). SDRF must still rank the bridge first.
        let restrictions = vec![
            Restriction::new(0, 1, 0.9),
            Restriction::new(0, 2, 0.9),
            Restriction::new(1, 2, 0.9),
            Restriction::new(3, 4, 0.9),
            Restriction::new(3, 5, 0.9),
            Restriction::new(4, 5, 0.9),
            Restriction::new(2, 3, 0.1), // bridge — small weight
        ];
        let t = tensor_of_with_restrictions(gs.clone(), restrictions);
        let fractures = minimize(&t);
        assert_eq!(fractures.len(), 14);
        // Bridge endpoints come first despite the small consumer
        // weight — curvature is topological.
        assert_eq!(Fracture::gap(&fractures[0]), &gs[2]);
        assert_eq!(Fracture::gap(&fractures[1]), &gs[3]);
        assert!(
            (Fracture::descent(&fractures[0]) - 2.0 / 3.0).abs() < 1e-9,
            "bridge descent (Ric=-2/3 → 0.667) must dominate; got {}",
            Fracture::descent(&fractures[0]),
        );
    }
}
