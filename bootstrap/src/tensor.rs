//! `@fate/tensor` — boundary Rust for the substrate's gap-tensor field
//! construction.
//!
//! This module realizes the substrate-declared action
//!
//! ```mirror
//! tensor_of([gap]) -> tensor
//! ```
//!
//! from [`docs/specs/gap-tension-tensor-substrate.md`] §3.2 and
//! [`docs/specs/property-and-inference-collapse.md`] §9.2 as a Rust
//! body. T6 in the implementation cascade after T5’s [`gaps_of`].
//!
//! ## What `tensor_of` is
//!
//! Klein-Mailly-Thimm 2020’s **MUS-graph** (Minimal Unsatisfiable
//! Subgraph; *JAIR* vol. 66, 2020) lifted to a **cellular sheaf** per
//! Hansen & Ghrist 2019 (*J. Appl. & Comput. Topology* 3:315–358,
//! arXiv:1808.01513). Each gap is a vertex in the inconsistency graph;
//! two gaps are adjacent when they share a substrate origin (the same
//! [`Ref`] — the minimal first-version reading of “structural locality”
//! before the substrate declares a richer conductivity-tensor edge
//! weight). The resulting cellular sheaf with identity restriction maps
//! reduces (per Bodnar et al. 2022 §2, arXiv:2202.04579) to the
//! standard normalised graph Laplacian; the substrate’s `fiedler` field
//! is λ₀(Δ_F) — the algebraic connectivity (also: λ₂ of the unnormalised
//! Laplacian, smallest non-trivial eigenvalue).
//!
//! ## What lives here
//!
//! - [`TensionVector`] — the Rust mirror of the substrate’s
//!   `tension_vector` (`shards`-altitude `\`-bodied; see
//!   [`docs/specs/gap-tension-tensor-substrate.md`] §8.1). Carries the
//!   audible-altitude floor reading: a scalar magnitude in `[0, 1]`
//!   naming “how hard this tension pulls.” The full Option-B
//!   tangent-space element lands when LTN grounding substrate-pulls.
//! - [`Tension`] — the Rust mirror of
//!   `tension = { a: gap, b: gap, vector: tension_vector }`.
//! - [`Tensor`] — the Rust mirror of
//!   `tensor = { tensions: [tension], fiedler: f64 }`.
//! - [`tensor_of`] — the executable form of `tensor_of([gap]) -> tensor`.
//!
//! ## Edge construction (Mara’s call, surfaced)
//!
//! Two gaps are adjacent when their `Ref` origins compare equal. This is
//! the structural reading of “two claims about the same substrate
//! location can be in opposition.” The conductivity-tensor weighting per
//! the eigenboard-representation spec is **deferred** — today’s edges
//! carry a uniform `vector.magnitude = 1.0` and identity restriction
//! maps. The richer weighting lands when the conductivity tensor declares
//! its read at the boundary (likely T8).
//!
//! ## Eigenvalue computation
//!
//! Minimal pure-Rust Jacobi rotation on the small dense normalised
//! Laplacian `Δ_0 = D^{-1/2} L D^{-1/2}`. For `n` gaps the matrix is
//! `n × n`; symmetric; bounded size. The smallest non-trivial eigenvalue
//! IS `fiedler`. Boundary cases:
//!
//! - `n = 0` (empty gap vector): trivial sheaf; `fiedler = 0.0`. The
//!   substrate’s λ₀ on a zero-dimensional space is the additive identity.
//! - `n = 1` (singleton vertex set): no edges; `Δ_0 = [0]`; `fiedler = 0.0`.
//! - `n ≥ 2`: Jacobi eigendecomposition on `Δ_0`; return the smallest
//!   eigenvalue strictly greater than a numerical-zero threshold, OR `0.0`
//!   if the graph has disconnected components (multiplicity of λ₀ > 1
//!   indicates disconnection per Bodnar et al. 2022 §2).
//!
//! T8 lands the proper sheaf-Laplacian numerical primitive in the
//! flang/mirror split; today’s Jacobi suffices for the gap-count-bounded
//! matrices the property layer produces.
//!
//! ## Why this lives here (not in `property.rs`, not in `gap.rs`)
//!
//! `tensor_of` is declared at the `@fate` altitude per
//! [`docs/specs/gap-tension-tensor-substrate.md`] §2 (“tension and tensor
//! live in `@fate`”). [`crate::property`] is the `@epistemologic/property`
//! altitude where `gaps_of` lives; [`crate::gap`] is the gap-type module.
//! The substrate-path-honest placement is a sibling top-level module
//! (matches T4’s `oscillate.rs` precedent: substrate-declared action
//! gets its own boundary file at the path the substrate names).
//!
//! [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
//! [`docs/specs/property-and-inference-collapse.md`]: ../../../../docs/specs/property-and-inference-collapse.md
//! [`Ref`]: prism_core::Ref
//! [`gaps_of`]: crate::property::gaps_of
#![allow(dead_code)]

use crate::gap::Gap;

// ---------------------------------------------------------------------------
// TensionVector — the audible-altitude floor for the §8.1 design call.
// ---------------------------------------------------------------------------

/// The substrate's `tension_vector` at the audible-altitude floor.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §8.1 the substrate
/// declares `tension_vector = \` — the choice between Option A (scalar
/// deltas), Option B (tangent-space element), and Option C (`@nl`
/// symbolic expression) is parked at the substrate altitude. T6 lands a
/// **minimal floor**: a single scalar `magnitude` in `[0, 1]` naming how
/// hard the tension pulls. This is Option A degraded to one dimension
/// — sufficient for the MUS-graph edge weighting `tensor_of` constructs
/// today, insufficient for the full directed-pull §3.2 names. The full
/// shape lifts when §8.1 closes.
///
/// [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
#[derive(Clone, Debug, PartialEq)]
pub struct TensionVector {
    /// The magnitude of the tension's pull, in `[0, 1]`. `1.0` = maximum
    /// pull (the substrate has no headroom on this gap pair); `0.0` =
    /// no pull (gaps that share an origin but do not oppose).
    magnitude: f64,
}

impl TensionVector {
    /// Construct a tension vector at the audible-altitude floor.
    /// Magnitude is clamped to `[0, 1]`.
    pub fn new(magnitude: f64) -> Self {
        TensionVector {
            magnitude: magnitude.clamp(0.0, 1.0),
        }
    }

    /// Read this tension vector's magnitude.
    pub fn magnitude(v: &TensionVector) -> f64 {
        v.magnitude
    }
}

// ---------------------------------------------------------------------------
// Tension — the Rust mirror of `tension = { a: gap, b: gap, vector }`.
// ---------------------------------------------------------------------------

/// Two gaps in structural opposition.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §3.2:
///
/// ```mirror
/// type tension = {
///   a:      gap,
///   b:      gap,
///   vector: tension_vector
/// }
/// ```
///
/// At T6's altitude two gaps are in opposition when they share a
/// substrate origin (`Ref` equality). The `vector` field carries the
/// audible-altitude floor reading (uniform `1.0` for the minimal first
/// version); the directed-pull weighting lands when the conductivity
/// tensor declares its read.
///
/// [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
#[derive(Clone, Debug, PartialEq)]
pub struct Tension {
    a: Gap,
    b: Gap,
    vector: TensionVector,
}

impl Tension {
    /// Construct a tension from two gaps and a tension vector.
    pub fn new(a: Gap, b: Gap, vector: TensionVector) -> Self {
        Tension { a, b, vector }
    }

    /// Borrow this tension's first gap.
    pub fn a(t: &Tension) -> &Gap {
        &t.a
    }

    /// Borrow this tension's second gap.
    pub fn b(t: &Tension) -> &Gap {
        &t.b
    }

    /// Borrow this tension's vector.
    pub fn vector(t: &Tension) -> &TensionVector {
        &t.vector
    }
}

// ---------------------------------------------------------------------------
// Tensor — the Rust mirror of `tensor = { tensions: [tension], fiedler }`.
// ---------------------------------------------------------------------------

/// A structured collection of tensions plus a spectral signature.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §3.2:
///
/// ```mirror
/// type tensor = {
///   tensions: [tension],
///   fiedler:  f64
/// }
/// ```
///
/// `fiedler` is λ₀(Δ_F) — the smallest non-trivial eigenvalue of the
/// normalised sheaf Laplacian. Low `fiedler` → loosely coupled tensor
/// (gaps can close independently); high `fiedler` → tightly coupled
/// (closing one gap perturbs many others). On the empty / singleton
/// gap basis `fiedler` is `0.0` (trivial sheaf).
///
/// The vertex set of the underlying MUS-graph is exposed via
/// [`vertices`](Tensor::vertices) — the per-gap basis T7 (`minimize`)
/// will walk for gradient descent.
///
/// [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    vertices: Vec<Gap>,
    tensions: Vec<Tension>,
    fiedler: f64,
}

impl Tensor {
    /// Construct a tensor from its three carriers. Most callers want
    /// [`tensor_of`] — this constructor is for tests and downstream
    /// consumers that build tensors from non-gap sources.
    pub fn new(vertices: Vec<Gap>, tensions: Vec<Tension>, fiedler: f64) -> Self {
        Tensor {
            vertices,
            tensions,
            fiedler,
        }
    }

    /// Borrow this tensor's vertex set — the per-gap MUS-graph basis.
    pub fn vertices(t: &Tensor) -> &[Gap] {
        &t.vertices
    }

    /// Borrow this tensor's tension (edge) set.
    pub fn tensions(t: &Tensor) -> &[Tension] {
        &t.tensions
    }

    /// Read this tensor's Fiedler value — the algebraic connectivity.
    pub fn fiedler(t: &Tensor) -> f64 {
        t.fiedler
    }
}

// ---------------------------------------------------------------------------
// tensor_of — the substrate's tensor-construction primitive.
// ---------------------------------------------------------------------------

/// `tensor_of(gaps: [gap]) -> tensor` — build the gap-tensor field.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §3.2 and
/// [`docs/specs/property-and-inference-collapse.md`] §9.2: the
/// construction of Klein-Mailly-Thimm 2020's inconsistency graph G_K
/// from K (the gap basis) lifted to a Hansen & Ghrist 2019 cellular
/// sheaf. Mirror's specialisation: the gaps come from a typed AST (via
/// [`crate::property::gaps_of`]), not from a free-form knowledge base.
///
/// Minimal first version (per T6 brief):
///
/// 1. Each gap becomes a vertex.
/// 2. Edges connect gaps that share a substrate origin (`Ref` equality).
/// 3. Each edge carries an audible-altitude floor `TensionVector` with
///    `magnitude = 1.0` (uniform; the conductivity-tensor weighting
///    lands later).
/// 4. Restriction maps are identity (the minimal cellular sheaf reduces
///    to the standard graph Laplacian per Bodnar 2022 §2).
/// 5. `fiedler = λ₀(Δ_0)` via Jacobi rotation on the dense normalised
///    Laplacian. `0.0` for trivial / disconnected / singleton cases.
///
/// Pure; no I/O; allocates per the returned `Tensor`.
///
/// [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
/// [`docs/specs/property-and-inference-collapse.md`]: ../../../../docs/specs/property-and-inference-collapse.md
pub fn tensor_of(gaps: Vec<Gap>) -> Tensor {
    // T6 RED — sentinel body. The GREEN commit lands the MUS-graph +
    // sheaf-lift + Jacobi-fiedler construction. This sentinel exists so
    // the test corpus compiles and the RED phase is observable.
    Tensor {
        vertices: gaps,
        tensions: Vec::new(),
        fiedler: f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`tensor_of`] — T6's `tensor_of` body.
    //!
    //! Per `docs/specs/gap-tension-tensor-substrate.md` §3.2: `tensor_of`
    //! takes a gap vector, constructs the MUS-graph (KMT 2020), lifts to
    //! a cellular sheaf (Hansen & Ghrist 2019), and returns the
    //! substrate's `tensor` with the Fiedler value λ₀(Δ_F) (Bodnar et al.
    //! 2022) — the algebraic connectivity. These tests RED first; the
    //! body lands GREEN in the next commit.

    use super::*;
    use crate::gap::Gap;
    use prism_core::Ref;

    fn total_origin() -> Ref {
        Ref::new("@epistemologic/property/total_classification").expect("valid ref")
    }

    fn other_origin() -> Ref {
        Ref::new("@epistemologic/property/strict_classification").expect("valid ref")
    }

    /// Empty gap vector → trivial sheaf. Zero vertices, zero tensions,
    /// `fiedler = 0.0` (λ₀ of the additive-identity space).
    #[test]
    fn empty_gaps_yield_trivial_tensor() {
        let t = tensor_of(Vec::new());
        assert!(Tensor::vertices(&t).is_empty());
        assert!(Tensor::tensions(&t).is_empty());
        assert_eq!(Tensor::fiedler(&t), 0.0);
    }

    /// Single gap → singleton vertex set; no edges; trivial sheaf;
    /// `fiedler = 0.0`. Δ_0 = [0] on a single vertex.
    #[test]
    fn single_gap_yields_singleton_vertex_set() {
        let g = Gap::new(0, total_origin(), "dark region [0, 5)");
        let t = tensor_of(vec![g.clone()]);
        assert_eq!(Tensor::vertices(&t).len(), 1);
        assert_eq!(Tensor::vertices(&t)[0], g);
        assert!(Tensor::tensions(&t).is_empty());
        assert_eq!(Tensor::fiedler(&t), 0.0);
    }

    /// Two gaps from the same origin → two vertices, one edge.
    /// Connected K₂ has algebraic connectivity λ₀(Δ_0) = 1.0 (the
    /// normalised Laplacian of K₂ is `[[1, -1], [-1, 1]]` with
    /// eigenvalues {0, 2}; normalised by D^{-1/2} → {0, 1}; the smallest
    /// non-trivial eigenvalue is 1.0).
    #[test]
    fn two_gaps_same_origin_yield_one_tension_fiedler_one() {
        let g1 = Gap::new(0, total_origin(), "dark region [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark region [10, 15)");
        let t = tensor_of(vec![g1.clone(), g2.clone()]);
        assert_eq!(Tensor::vertices(&t).len(), 2);
        assert_eq!(Tensor::tensions(&t).len(), 1);
        let tension = &Tensor::tensions(&t)[0];
        // The tension joins the two same-origin gaps.
        assert_eq!(Tension::a(tension), &g1);
        assert_eq!(Tension::b(tension), &g2);
        // The audible-altitude floor reading: uniform magnitude 1.0.
        assert!(
            (TensionVector::magnitude(Tension::vector(tension)) - 1.0).abs() < 1e-12,
            "vector magnitude must be 1.0 at the audible-altitude floor",
        );
        // K₂ normalised Laplacian: λ₀ = 1.0.
        assert!(
            (Tensor::fiedler(&t) - 1.0).abs() < 1e-9,
            "K₂ algebraic connectivity must be 1.0; got {}",
            Tensor::fiedler(&t),
        );
    }

    /// Two gaps from different origins → two vertices, NO edges; the
    /// graph is disconnected; `fiedler = 0.0` (Bodnar et al. 2022 §2:
    /// algebraic connectivity is zero on disconnected graphs).
    #[test]
    fn two_gaps_different_origins_yield_no_tension() {
        let g1 = Gap::new(0, total_origin(), "dark region [0, 5)");
        let g2 = Gap::new(0, other_origin(), "strict failure");
        let t = tensor_of(vec![g1, g2]);
        assert_eq!(Tensor::vertices(&t).len(), 2);
        assert!(
            Tensor::tensions(&t).is_empty(),
            "different origins → no edges at the audible-altitude floor",
        );
        // Disconnected → λ₀ = 0 (algebraic-connectivity vanishes on
        // disconnected graphs per Fiedler 1973 / Bodnar 2022 §2).
        assert_eq!(Tensor::fiedler(&t), 0.0);
    }

    /// Three gaps from the same origin → K₃ (complete graph on 3).
    /// K₃ has 3 edges (every pair joined).
    #[test]
    fn three_same_origin_gaps_yield_three_tensions() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let g3 = Gap::new(0, total_origin(), "dark [20, 25)");
        let t = tensor_of(vec![g1, g2, g3]);
        assert_eq!(Tensor::vertices(&t).len(), 3);
        assert_eq!(
            Tensor::tensions(&t).len(),
            3,
            "K₃ has 3 edges (every pair joined when they share an origin)",
        );
        // K₃ normalised Laplacian: eigenvalues are {0, 3/2, 3/2};
        // λ₀(Δ_0) = 3/2.
        assert!(
            (Tensor::fiedler(&t) - 1.5).abs() < 1e-9,
            "K₃ normalised algebraic connectivity must be 1.5; got {}",
            Tensor::fiedler(&t),
        );
    }

    /// Mixed: two same-origin gaps + one isolated gap from another
    /// origin → two components (an edge + an isolated vertex).
    /// Disconnected → fiedler = 0.0.
    #[test]
    fn clustered_plus_isolated_yields_disconnected_tensor() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let g3 = Gap::new(0, other_origin(), "strict failure");
        let t = tensor_of(vec![g1, g2, g3]);
        assert_eq!(Tensor::vertices(&t).len(), 3);
        assert_eq!(
            Tensor::tensions(&t).len(),
            1,
            "only the two same-origin gaps share an edge",
        );
        // Disconnected graph (K₂ ⊔ K₁) → λ₀ = 0.
        assert_eq!(Tensor::fiedler(&t), 0.0);
    }

    /// Type-level: the substrate signature `tensor_of([gap]) -> tensor`
    /// IS realized as `tensor_of(Vec<Gap>) -> Tensor` at the boundary.
    #[test]
    fn tensor_of_returns_tensor() {
        let _t: Tensor = tensor_of(Vec::new());
    }

    /// Tension vector magnitude is clamped to [0, 1].
    #[test]
    fn tension_vector_clamps_to_unit_interval() {
        let v = TensionVector::new(1.5);
        assert_eq!(TensionVector::magnitude(&v), 1.0);
        let v = TensionVector::new(-0.5);
        assert_eq!(TensionVector::magnitude(&v), 0.0);
        let v = TensionVector::new(0.42);
        assert_eq!(TensionVector::magnitude(&v), 0.42);
    }
}
