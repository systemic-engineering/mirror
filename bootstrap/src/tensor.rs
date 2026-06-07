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
//! `fiedler_of` routes through [`crate::sheaf_laplacian::lambda_zero`] —
//! the LAPACK `dsyev` path via `prism_core::ffi::eigenvalues` (T8). The
//! adjacency matrix on the gap basis becomes a `Vec<Restriction>`; the
//! sheaf-Laplacian assembly produces an `Operator`; `lambda_zero`
//! returns the spectral gap. T6's K₂ = 2.0 / K₃ = 1.5 / disconnected =
//! 0.0 baseline numbers are preserved — LAPACK reproduces Jacobi's
//! values on identity-weight sheaves and additionally handles the
//! non-uniform case the Jacobi stub could not reach.
//!
//! Boundary cases:
//!
//! - `n = 0` (empty gap vector): trivial sheaf; `fiedler = 0.0`. The
//!   substrate’s λ₀ on a zero-dimensional space is the additive identity.
//! - `n = 1` (singleton vertex set): no edges; `Δ_0 = [0]`; `fiedler = 0.0`.
//! - `n ≥ 2`: build a `Restriction` per tension (weight = magnitude);
//!   compose with `sheaf_laplacian`; read `lambda_zero`. Disconnected
//!   sheaves return `0.0` (multiplicity of λ₀ > 1) per Bodnar 2022 §2.
//!
//! ## T8.5 bridge
//!
//! [`tensor_of_with_restrictions`] is the sibling constructor for
//! consumers that have real conductivity-tensor weights (per
//! `eigenboard-representation.md`). `tensor_of` stays simple (uniform
//! 1.0 from same-origin pairing); the weighted path becomes the
//! cleanest call site for downstream composers (pulse / oscillate-
//! driver / active_pass / dark_pass / query_phi).
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
//! [`crate::sheaf_laplacian::lambda_zero`]: crate::sheaf_laplacian::lambda_zero
#![allow(dead_code)]

use crate::gap::Gap;
use crate::sheaf_laplacian::{lambda_zero, sheaf_laplacian, Eigenvalue, Restriction};

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
    // Phase 1: build the tension (edge) set. Two gaps are adjacent when
    // their substrate origins compare equal — the minimal Klein-MUS
    // reading of “two claims about the same substrate location can be
    // in opposition.” Each edge carries the audible-altitude floor
    // TensionVector (uniform magnitude 1.0).
    let mut tensions: Vec<Tension> = Vec::new();
    for i in 0..gaps.len() {
        for j in (i + 1)..gaps.len() {
            if gaps[i].origin() == gaps[j].origin() {
                tensions.push(Tension::new(
                    gaps[i].clone(),
                    gaps[j].clone(),
                    TensionVector::new(1.0),
                ));
            }
        }
    }

    // Phase 2: λ₀ of the normalised sheaf Laplacian. With identity
    // restriction maps (the minimal first version) the sheaf Laplacian
    // reduces to the standard normalised graph Laplacian per
    // Bodnar et al. 2022 §2. Trivial / singleton / disconnected cases
    // emit 0.0.
    let fiedler = fiedler_of(&gaps, &tensions);

    Tensor {
        vertices: gaps,
        tensions,
        fiedler,
    }
}

// ---------------------------------------------------------------------------
// tensor_of_with_restrictions — the bridge constructor (T8.5).
// ---------------------------------------------------------------------------

/// `tensor_of_with_restrictions(gaps, restrictions) -> tensor` — the
/// **bridge constructor** that accepts explicit per-edge restriction
/// weights from the consumer.
///
/// Where [`tensor_of`] hard-codes uniform `TensionVector::magnitude =
/// 1.0` from a same-origin pairing rule, this sibling lets the
/// consumer supply real conductivity data: each [`Restriction`] names
/// a `(source_index, target_index, weight)` triple keyed against the
/// gap basis (the `gaps` argument). The weight flows into the
/// resulting [`Tension`]'s [`TensionVector::magnitude`]; downstream
/// `minimize` rankings (T7) become meaningful because the magnitudes
/// now carry real signal.
///
/// ## Why a sibling (not extending `tensor_of`)
///
/// The bridge tick discipline (per Reed's lean): add the parameter;
/// default to T6's uniform behavior; let consumers explicitly supply
/// real weights. `tensor_of` stays as the same-origin floor-altitude
/// constructor; `tensor_of_with_restrictions` is the consumer-driven
/// path that lets composers (pulse / oscillate-driver / active_pass /
/// dark_pass / query_phi) wire real conductivity data without the
/// origin-pairing rule getting in the way.
///
/// ## Substrate-pull discipline
///
/// The [`Restriction`] carrier IS the substrate-declared
/// `restriction` type from
/// [`shards/epistemologic/math/sheaf_laplacian.mirror`] (T8). The
/// substrate names it; the boundary reuses it directly — no parallel
/// type. Indices are `u32` per the substrate's `source: u32, target:
/// u32` declaration; this constructor accepts `Restriction` values
/// directly.
///
/// ## Algorithm
///
/// 1. For each [`Restriction`] in the input vector, build a
///    [`Tension`] connecting the gap at `Restriction::source` to the
///    gap at `Restriction::target` with `TensionVector::magnitude =
///    Restriction::weight`. Out-of-range indices (≥ `gaps.len()`) are
///    silently dropped (defensive boundary read).
/// 2. Read `fiedler` via [`fiedler_of`] on the constructed tensions
///    — routes through [`lambda_zero`] under the hood. Disconnected /
///    trivial / singleton cases emit `0.0`.
///
/// Pure; no I/O; allocates per the returned [`Tensor`].
///
/// [`Restriction`]: crate::sheaf_laplacian::Restriction
/// [`shards/epistemologic/math/sheaf_laplacian.mirror`]: ../../../../shards/epistemologic/math/sheaf_laplacian.mirror
pub fn tensor_of_with_restrictions(gaps: Vec<Gap>, restrictions: Vec<Restriction>) -> Tensor {
    // Phase 1: build tensions from the consumer-supplied restrictions.
    // Out-of-range indices are silently dropped.
    let n = gaps.len();
    let mut tensions: Vec<Tension> = Vec::with_capacity(restrictions.len());
    for r in &restrictions {
        let s = Restriction::source(r) as usize;
        let t = Restriction::target(r) as usize;
        if s >= n || t >= n {
            continue;
        }
        let weight = Restriction::weight(r);
        tensions.push(Tension::new(
            gaps[s].clone(),
            gaps[t].clone(),
            TensionVector::new(weight),
        ));
    }

    // Phase 2: λ₀ of the normalised sheaf Laplacian via the LAPACK
    // path. fiedler_of routes through sheaf_laplacian::lambda_zero —
    // the same primitive whether the construction is uniform-weight
    // (tensor_of) or weighted (this constructor).
    let fiedler = fiedler_of(&gaps, &tensions);

    Tensor {
        vertices: gaps,
        tensions,
        fiedler,
    }
}

// ---------------------------------------------------------------------------
// fiedler_of — the algebraic-connectivity reading on the gap basis.
// ---------------------------------------------------------------------------

/// Compute λ₀(Δ_F) on the gap-tension graph by routing through
/// [`lambda_zero`].
///
/// **T8.5 bridge:** this function previously called a pure-Rust Jacobi
/// eigenvalue routine (T6); it now constructs a [`Restriction`] per
/// tension (with `source` / `target` as gap-basis indices and
/// `weight` = `TensionVector::magnitude`) and reads the spectral gap
/// from [`lambda_zero`] — the LAPACK `dsyev` path via
/// `prism_core::ffi::eigenvalues` (T8). LAPACK reproduces Jacobi's
/// values on identity-weight sheaves (K₂ = 2.0, K₃ = 1.5, disconnected
/// = 0.0) and additionally handles the non-uniform case the Jacobi
/// stub could not reach.
///
/// `Δ_F = I − D^{-1/2} W D^{-1/2}` is the normalised graph Laplacian
/// on the vertex set (the gaps) with edge weights from the tension
/// vectors. On disconnected graphs the multiplicity of eigenvalue
/// `0` equals the number of components; the substrate's convention
/// (per [`lambda_zero`]) emits `value = 0.0` with multiplicity > 1
/// in that case. `fiedler_of` projects that to a scalar `0.0` so the
/// trivial / singleton / disconnected paths all surface as zero
/// algebraic connectivity per Fiedler 1973 / Bodnar 2022 §2.
///
/// Trivial / singleton: `0.0`.
fn fiedler_of(gaps: &[Gap], tensions: &[Tension]) -> f64 {
    let n = gaps.len();
    if n < 2 {
        return 0.0;
    }

    // Build Restrictions by re-indexing tensions against the gap basis.
    // Tensions carry cloned gaps (not indices); linear scan recovers
    // the indices. n is bounded by the gap count (small for the
    // corpora the property layer emits today).
    let mut restrictions: Vec<Restriction> = Vec::with_capacity(tensions.len());
    for t in tensions {
        let mut ia: Option<u32> = None;
        let mut ib: Option<u32> = None;
        for (k, g) in gaps.iter().enumerate() {
            if ia.is_none() && g == Tension::a(t) {
                ia = Some(k as u32);
                continue;
            }
            if ib.is_none() && g == Tension::b(t) {
                ib = Some(k as u32);
            }
        }
        if let (Some(a), Some(b)) = (ia, ib) {
            let weight = TensionVector::magnitude(Tension::vector(t));
            restrictions.push(Restriction::new(a, b, weight));
        }
    }

    // The substrate-altitude carrier names the (dimension, entries)
    // pair: `sheaf_laplacian` assembles. But the gap basis fixes the
    // dimension at `n` (not `max(source, target) + 1`), so we
    // construct the Operator directly to preserve isolated vertices
    // beyond the highest index referenced by a restriction. (Without
    // this, an isolated trailing gap would shrink the dimension and
    // hide a component.)
    let assembled = sheaf_laplacian(restrictions);
    let op = if (crate::sheaf_laplacian::Operator::dimension(&assembled) as usize) < n {
        // Wrap with the gap-basis dimension so isolated trailing
        // vertices contribute their own zero-eigenvalue components.
        crate::sheaf_laplacian::Operator::new(
            n as u32,
            crate::sheaf_laplacian::Operator::entries(&assembled).to_vec(),
        )
    } else {
        assembled
    };

    let eigenvalue = lambda_zero(&op);
    // Disconnected (multiplicity > 1) → algebraic connectivity vanishes
    // per Fiedler 1973 / Bodnar 2022 §2.
    if Eigenvalue::multiplicity(&eigenvalue) > 1 {
        return 0.0;
    }
    Eigenvalue::value(&eigenvalue)
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
    /// Connected K₂ has normalised algebraic connectivity λ₀(Δ_0) = 2.0:
    /// adjacency A = [[0,1],[1,0]], degrees [1,1], so D^{-1/2} = I; the
    /// normalised Laplacian Δ_0 = I − A = [[1,-1],[-1,1]] with eigenvalues
    /// {0, 2}. The smallest non-trivial eigenvalue is 2.0 — in general,
    /// the complete graph K_n has λ₀(Δ_0) = n/(n−1).
    #[test]
    fn two_gaps_same_origin_yield_one_tension_fiedler_two() {
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
        // K₂ normalised Laplacian: λ₀ = 2/(2−1) = 2.0.
        assert!(
            (Tensor::fiedler(&t) - 2.0).abs() < 1e-9,
            "K₂ normalised algebraic connectivity must be 2.0; got {}",
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

    // -----------------------------------------------------------------------
    // T8.5 bridge: tensor_of_with_restrictions + LAPACK-routed fiedler_of.
    //
    // The bridge tick lifts T6's identity-restriction-map limitation: a
    // sibling constructor accepts per-edge `Restriction`s (the substrate-
    // declared `restriction` carrier from `sheaf_laplacian.mirror`) and
    // the resulting Fiedler value is computed by the same numerical
    // primitive `lambda_zero` uses — LAPACK `dsyev` via
    // `prism_core::ffi::eigenvalues`. The T6 tests above continue to pass
    // through the new LAPACK path (substantive correctness check).
    //
    // The brief: cleanest scope is a sibling constructor, not extending
    // `tensor_of`. `tensor_of` stays simple (uniform 1.0 from same-origin
    // pairing); `tensor_of_with_restrictions` enables non-uniform weights
    // for downstream consumers that have real conductivity data.
    // -----------------------------------------------------------------------

    use crate::sheaf_laplacian::Restriction;

    /// `tensor_of_with_restrictions(gaps, [])` on a single-gap basis
    /// yields the same trivial tensor as `tensor_of(gaps)`: one vertex,
    /// no tensions, fiedler = 0.
    #[test]
    fn tensor_of_with_restrictions_empty_yields_trivial_tensor() {
        let g = Gap::new(0, total_origin(), "dark region [0, 5)");
        let t = tensor_of_with_restrictions(vec![g.clone()], Vec::new());
        assert_eq!(Tensor::vertices(&t).len(), 1);
        assert!(Tensor::tensions(&t).is_empty());
        assert_eq!(Tensor::fiedler(&t), 0.0);
    }

    /// `tensor_of_with_restrictions` with one unit-weight restriction on
    /// a 2-gap basis reproduces K₂'s fiedler = 2.0 — the substantive
    /// correctness check that the LAPACK path matches the Jacobi path on
    /// the identity-weights baseline.
    #[test]
    fn tensor_of_with_restrictions_k2_unit_weight_matches_jacobi_baseline() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let t = tensor_of_with_restrictions(
            vec![g1.clone(), g2.clone()],
            vec![Restriction::new(0, 1, 1.0)],
        );
        assert_eq!(Tensor::vertices(&t).len(), 2);
        assert_eq!(Tensor::tensions(&t).len(), 1);
        // K₂ normalised algebraic connectivity = 2/(2−1) = 2.0; LAPACK
        // path must reproduce the Jacobi-path value.
        assert!(
            (Tensor::fiedler(&t) - 2.0).abs() < 1e-9,
            "K₂ unit-weight via LAPACK must yield 2.0; got {}",
            Tensor::fiedler(&t),
        );
    }

    /// `tensor_of_with_restrictions` carries the consumer-supplied
    /// weight into the constructed Tension's `TensionVector::magnitude`.
    /// This is what makes T7's `minimize` rankings meaningful when the
    /// consumer has real weights.
    #[test]
    fn tensor_of_with_restrictions_propagates_weight_into_tension_vector() {
        let g1 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g2 = Gap::new(0, total_origin(), "dark [10, 15)");
        let t = tensor_of_with_restrictions(vec![g1, g2], vec![Restriction::new(0, 1, 0.42)]);
        assert_eq!(Tensor::tensions(&t).len(), 1);
        let tension = &Tensor::tensions(&t)[0];
        assert!(
            (TensionVector::magnitude(Tension::vector(tension)) - 0.42).abs() < 1e-12,
            "consumer-supplied weight 0.42 must flow into TensionVector::magnitude; got {}",
            TensionVector::magnitude(Tension::vector(tension)),
        );
    }

    /// `tensor_of_with_restrictions` on a 3-vertex path graph P₃ with
    /// NON-uniform weights produces a Fiedler value strictly less than
    /// the K₃ value (1.5) — the substantive correctness check that
    /// non-uniform restriction weights surface meaningful curvature
    /// signal. P₃ (path) is structurally less connected than K₃
    /// (complete); the spectral gap shrinks. With weights w(0,1)=1.0,
    /// w(1,2)=4.0 the gap is non-zero and strictly less than 1.5.
    #[test]
    fn tensor_of_with_restrictions_p3_nonuniform_yields_meaningful_fiedler() {
        let g0 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g1 = Gap::new(0, total_origin(), "dark [10, 15)");
        let g2 = Gap::new(0, total_origin(), "dark [20, 25)");
        let t = tensor_of_with_restrictions(
            vec![g0, g1, g2],
            vec![Restriction::new(0, 1, 1.0), Restriction::new(1, 2, 4.0)],
        );
        let f = Tensor::fiedler(&t);
        assert!(f > 1e-9, "P₃ connected → fiedler > 0; got {}", f,);
        assert!(
            f < 1.5,
            "P₃ less connected than K₃ → fiedler < 1.5; got {}",
            f,
        );
    }

    /// `tensor_of_with_restrictions` on disconnected vertices yields
    /// fiedler = 0.0. (Two isolated gaps with no restriction between
    /// them; the graph has two components; algebraic connectivity
    /// vanishes per Bodnar 2022 §2.)
    #[test]
    fn tensor_of_with_restrictions_disconnected_yields_zero_fiedler() {
        let g0 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g1 = Gap::new(0, total_origin(), "dark [10, 15)");
        let t = tensor_of_with_restrictions(vec![g0, g1], Vec::new());
        assert_eq!(Tensor::vertices(&t).len(), 2);
        assert!(Tensor::tensions(&t).is_empty());
        assert_eq!(Tensor::fiedler(&t), 0.0);
    }

    /// Out-of-range restriction indices are silently ignored: the gap
    /// basis has `n` vertices, indices ≥ n have no corresponding gap.
    /// (Consumers are expected to keep indices in range; this is the
    /// defensive boundary read.)
    #[test]
    fn tensor_of_with_restrictions_ignores_out_of_range_indices() {
        let g0 = Gap::new(0, total_origin(), "dark [0, 5)");
        let g1 = Gap::new(0, total_origin(), "dark [10, 15)");
        // Restriction references index 5 which has no corresponding
        // gap; it must be dropped without panicking.
        let t = tensor_of_with_restrictions(
            vec![g0, g1],
            vec![Restriction::new(0, 1, 1.0), Restriction::new(0, 5, 0.5)],
        );
        // Only the in-range restriction becomes a tension.
        assert_eq!(Tensor::tensions(&t).len(), 1);
    }
}
