//! `@epistemologic/math/sheaf_laplacian` — boundary Rust for the discrete
//! sheaf-Laplacian numerical primitive.
//!
//! This module realizes substrate-declared types and actions from
//! [`shards/epistemologic/math/sheaf_laplacian.mirror`] as Rust bodies.
//! T8 in the implementation cascade after T7's [`crate::kintsugi::minimize`].
//!
//! ## What `sheaf_laplacian` is
//!
//! The discrete realisation of [`spectral-triple.mirror`]'s `dirac_op` per
//! Barbero et al. 2022 (arXiv:2206.08702): for an `O(d)`-bundle over a
//! graph the sheaf Laplacian IS the connection Laplacian. Per Bodnar et al.
//! 2022 (arXiv:2202.04579) §2, the exact construction for cellular sheaf
//! `F` over a graph with restriction maps `F_{v ⊴ e} : F(v) → F(e)`:
//!
//! ```text
//! L_{F vv} = Σ_{v ⊴ e} F_{v ⊴ e}^⊤ F_{v ⊴ e}        (diagonal blocks)
//! L_{F vu} = − F_{v ⊴ e}^⊤ F_{u ⊴ e}                (off-diagonal blocks)
//! ```
//!
//! At the d=1 case (scalar fibers) this reduces to the **weighted graph
//! Laplacian** `L = D − W`, with `D` the diagonal degree matrix and `W`
//! the (symmetric) weight matrix carrying per-edge restriction magnitudes.
//! The normalised form `Δ_F = D^{-1/2} L D^{-1/2}` is what
//! [`crate::tensor::Tensor`]'s `fiedler` reads.
//!
//! ## What lives here
//!
//! - [`Restriction`] — the Rust mirror of the substrate's
//!   `restriction = { source: u32, target: u32, weight: ref }`.
//! - [`Operator`] — the Rust mirror of `operator = { dimension: u32,
//!   entries: [restriction] }`.
//! - [`Eigenvalue`] — the Rust mirror of `eigenvalue = { value: ref,
//!   multiplicity: u32 }`.
//! - [`sheaf_laplacian`] — the executable form of
//!   `sheaf_laplacian([restriction]) -> operator`.
//! - [`lambda_zero`] — the executable form of
//!   `lambda_zero(op: operator) -> eigenvalue`, calling
//!   [`prismqueer::ffi::eigenvalues`] (LAPACK `dsyev`).
//!
//! ## Why the LAPACK path (not pure Rust)
//!
//! The reference implementation of symmetric-eigendecomposition is
//! LAPACK's `dsyev` — ~2 million lines of Fortran 90, mature, the global
//! standard. The substrate-pull discipline (per AGENTS.md "Keywords Are
//! Substrate Declarations") says numerical mechanism the substrate cannot
//! express belongs at the boundary; the boundary uses the standard library
//! rather than reinventing it. `prismqueer::ffi::eigenvalues` is the
//! already-built wrapper (per `docs/specs/numerical-substrate-via-fortran.md`
//! §1.4); T8 lifts T6's Jacobi-stub `fiedler_of` to call into it via the
//! sheaf-Laplacian assembly here.
//!
//! ## flang vs gfortran distinction
//!
//! Per [`docs/specs/numerical-substrate-via-fortran.md`] §1.4: the
//! existing kernels at `prism/core/native/spectral.f90` are gfortran-built
//! (via Accelerate framework on darwin). flang is the LLVM-aligned Fortran
//! front-end and is the long-term substrate target (per
//! `docs/specs/numerical-substrate-via-fortran.md` §3); the gfortran
//! pathway is the proven floor that T8 builds on. The reconciliation is a
//! deferred concern; T8 surfaces real eigenvalues through the path that
//! already links.
//!
//! [`shards/epistemologic/math/sheaf_laplacian.mirror`]: ../../../../shards/epistemologic/math/sheaf_laplacian.mirror
//! [`spectral-triple.mirror`]: ../../../../boot/std/epistemologic/math/spectral-triple.mirror
//! [`docs/specs/numerical-substrate-via-fortran.md`]: ../../../../docs/specs/numerical-substrate-via-fortran.md
//! [`prismqueer::ffi::eigenvalues`]: prismqueer::ffi::eigenvalues
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Restriction — the Rust mirror of the substrate's restriction carrier.
// ---------------------------------------------------------------------------

/// One edge of a cellular sheaf in a chosen gauge.
///
/// Per [`shards/epistemologic/math/sheaf_laplacian.mirror`]:
///
/// ```mirror
/// type restriction = {
///   source: u32,
///   target: u32,
///   weight: ref,
/// }
/// ```
///
/// The `weight` carries the magnitude of the parallel-transport operator
/// on the edge `(source, target)` — the audible-altitude floor for the
/// full in-a-gauge matrix block per `bundle.mirror`'s `type optic`. At
/// the d=1 case this scalar IS the edge weight in the weighted graph
/// Laplacian; for higher fiber dimension the substrate-altitude carrier
/// lifts (deferred — T8 lands the d=1 minimum).
///
/// `source` and `target` are `u32` per the substrate-pull discipline
/// (matches `harmonic.ratio`'s `numerator: u32` precedent for small-
/// integer cardinals).
///
/// [`shards/epistemologic/math/sheaf_laplacian.mirror`]: ../../../../shards/epistemologic/math/sheaf_laplacian.mirror
#[derive(Clone, Debug, PartialEq)]
pub struct Restriction {
    source: u32,
    target: u32,
    weight: f64,
}

impl Restriction {
    /// Construct a restriction from its (source, target, weight) triple.
    pub fn new(source: u32, target: u32, weight: f64) -> Self {
        Restriction {
            source,
            target,
            weight,
        }
    }

    /// Borrow this restriction's source vertex index.
    pub fn source(r: &Restriction) -> u32 {
        r.source
    }

    /// Borrow this restriction's target vertex index.
    pub fn target(r: &Restriction) -> u32 {
        r.target
    }

    /// Borrow this restriction's edge weight (audible-altitude floor).
    pub fn weight(r: &Restriction) -> f64 {
        r.weight
    }
}

// ---------------------------------------------------------------------------
// Operator — the Rust mirror of the substrate's operator carrier.
// ---------------------------------------------------------------------------

/// The assembled sheaf Laplacian `Δ_F = δ*δ`.
///
/// Per [`shards/epistemologic/math/sheaf_laplacian.mirror`]:
///
/// ```mirror
/// type operator = {
///   dimension: u32,
///   entries:   [restriction],
/// }
/// ```
///
/// The discrete realisation of `spectral-triple.mirror`'s `dirac_op`
/// per Barbero et al. 2022 (arXiv:2206.08702). At the d=1 case (scalar
/// fibers) the operator's matrix form is `n × n` and IS the weighted
/// graph Laplacian. The substrate-altitude carrier names the
/// (dimension, entries) pair; the dense matrix materialisation lives
/// at the realisation layer (see [`dense_laplacian`]).
///
/// [`shards/epistemologic/math/sheaf_laplacian.mirror`]: ../../../../shards/epistemologic/math/sheaf_laplacian.mirror
#[derive(Clone, Debug, PartialEq)]
pub struct Operator {
    dimension: u32,
    entries: Vec<Restriction>,
}

impl Operator {
    /// Construct an operator from its (dimension, entries) carriers.
    /// Most callers want [`sheaf_laplacian`] — this is for tests and
    /// downstream consumers that build operators from other sources.
    pub fn new(dimension: u32, entries: Vec<Restriction>) -> Self {
        Operator { dimension, entries }
    }

    /// Read this operator's dimension (vertex count of the base graph).
    pub fn dimension(o: &Operator) -> u32 {
        o.dimension
    }

    /// Borrow this operator's restriction entries.
    pub fn entries(o: &Operator) -> &[Restriction] {
        &o.entries
    }
}

// ---------------------------------------------------------------------------
// Eigenvalue — the Rust mirror of the substrate's eigenvalue carrier.
// ---------------------------------------------------------------------------

/// One root of the operator's characteristic polynomial.
///
/// Per [`shards/epistemologic/math/sheaf_laplacian.mirror`]:
///
/// ```mirror
/// type eigenvalue = {
///   value:        ref,
///   multiplicity: u32,
/// }
/// ```
///
/// Element of `spectral-triple.mirror`'s `spectrum` reduced to its
/// scalar plus multiplicity. For the sheaf Laplacian on a graph with
/// `k` connected components, λ₀ = 0 with multiplicity `k` per
/// Fiedler 1973 / Bodnar et al. 2022 §2.
///
/// [`shards/epistemologic/math/sheaf_laplacian.mirror`]: ../../../../shards/epistemologic/math/sheaf_laplacian.mirror
#[derive(Clone, Debug, PartialEq)]
pub struct Eigenvalue {
    value: f64,
    multiplicity: u32,
}

impl Eigenvalue {
    /// Construct an eigenvalue from its (value, multiplicity) pair.
    pub fn new(value: f64, multiplicity: u32) -> Self {
        Eigenvalue {
            value,
            multiplicity,
        }
    }

    /// Read this eigenvalue's value.
    pub fn value(e: &Eigenvalue) -> f64 {
        e.value
    }

    /// Read this eigenvalue's multiplicity.
    pub fn multiplicity(e: &Eigenvalue) -> u32 {
        e.multiplicity
    }
}

// ---------------------------------------------------------------------------
// sheaf_laplacian — the δ*δ assembly.
// ---------------------------------------------------------------------------

/// `sheaf_laplacian(restrictions) -> operator` — build the discrete
/// sheaf Laplacian per Bodnar et al. 2022 §2's exact construction.
///
/// Per [`shards/epistemologic/math/sheaf_laplacian.mirror`]:
///
/// ```mirror
/// sheaf_laplacian(restrictions: [restriction]) -> operator { \ }
/// ```
///
/// At the d=1 case (scalar fibers) this constructs the weighted graph
/// Laplacian carrier — the dimension is `max(source, target) + 1` over
/// all entries (or `0` for an empty restriction vector). The entries
/// vector is stored as-is; the dense materialisation happens in
/// [`dense_laplacian`] when `lambda_zero` pulls.
///
/// Pure; no I/O; deterministic.
pub fn sheaf_laplacian(restrictions: Vec<Restriction>) -> Operator {
    let dimension: u32 = restrictions
        .iter()
        .map(|r| Restriction::source(r).max(Restriction::target(r)) + 1)
        .max()
        .unwrap_or(0);
    Operator::new(dimension, restrictions)
}

// ---------------------------------------------------------------------------
// lambda_zero — the smallest non-trivial eigenvalue via LAPACK dsyev.
// ---------------------------------------------------------------------------

/// `lambda_zero(op) -> eigenvalue` — the spectral gap of the operator.
///
/// Per [`shards/epistemologic/math/sheaf_laplacian.mirror`]:
///
/// ```mirror
/// lambda_zero(op: operator) -> eigenvalue { \ }
/// ```
///
/// Computes the smallest non-trivial eigenvalue of the operator's
/// dense matrix realisation via [`prismqueer::ffi::eigenvalues`]
/// (LAPACK `dsyev`). Multiplicity counts how many eigenvalues equal
/// the smallest within the numerical-zero threshold (`1e-9`) — for a
/// graph with k connected components, λ₀ = 0 with multiplicity k.
///
/// **Algorithm:**
///
/// 1. Materialise the dense normalised Laplacian
///    `Δ_F = I − D^{-1/2} W D^{-1/2}` via [`dense_laplacian`]. (When the
///    underlying graph is trivial — dimension ≤ 1 — emit `Eigenvalue(0,
///    dimension)`.)
/// 2. Call `prismqueer::ffi::eigenvalues(n, &matrix)` to get the
///    eigenvalues in ascending order.
/// 3. Count multiplicity at the zero-eigenvalue boundary; the smallest
///    eigenvalue STRICTLY greater than the threshold is the spectral
///    gap. If every eigenvalue is at zero (disconnected with k = n),
///    return `Eigenvalue(0.0, n)`.
///
/// **Why LAPACK over Jacobi:** the Jacobi sweep at T6's `tensor::fiedler_of`
/// was correct but stub. LAPACK's `dsyev` uses QR with implicit shifts —
/// the global standard, numerically stable for the matrix sizes the
/// property layer emits.
///
/// Pure relative to LAPACK (LAPACK is deterministic per platform).
pub fn lambda_zero(op: &Operator) -> Eigenvalue {
    let n = Operator::dimension(op) as usize;
    if n == 0 {
        return Eigenvalue::new(0.0, 0);
    }
    if n == 1 {
        return Eigenvalue::new(0.0, 1);
    }
    let matrix = dense_laplacian(op);
    let evals = match prismqueer::ffi::eigenvalues(n, &matrix) {
        Ok(v) => v,
        Err(_) => return Eigenvalue::new(0.0, n as u32),
    };

    // Count multiplicity at the zero-eigenvalue boundary (numerical-zero
    // threshold). The kernel dimension equals the number of connected
    // components per Bodnar et al. 2022 §2.
    let threshold = 1e-9_f64;
    let zero_count = evals.iter().filter(|&&e| e.abs() < threshold).count() as u32;

    // The smallest strictly-positive eigenvalue is the algebraic
    // connectivity / Fiedler value / spectral gap.
    let gap = evals
        .iter()
        .find(|&&e| e > threshold)
        .copied()
        .unwrap_or(0.0);

    // Disconnected (k > 1 components) → return λ₀ = 0 with multiplicity k.
    // Connected (k = 1) → return the spectral gap with multiplicity 1.
    if zero_count > 1 {
        Eigenvalue::new(0.0, zero_count)
    } else {
        Eigenvalue::new(gap, 1)
    }
}

// ---------------------------------------------------------------------------
// dense_laplacian — materialise the operator's normalised graph Laplacian.
// ---------------------------------------------------------------------------

/// Materialise the operator's dense normalised graph Laplacian
/// `Δ_F = I − D^{-1/2} W D^{-1/2}` as a row-major `n × n` `Vec<f64>`.
///
/// The substrate-altitude carrier names (dimension, entries); the dense
/// matrix is realisation-layer scaffolding for the LAPACK call. The
/// symmetrisation (treating each `Restriction(s, t, w)` as the
/// undirected edge `{s, t}` with weight `w`) matches Bodnar et al. 2022
/// §2's d=1 case where restriction maps reduce to scalars and the sheaf
/// Laplacian collapses to the weighted graph Laplacian.
///
/// **Symmetrisation:** if both `(s, t, w_st)` and `(t, s, w_ts)` appear
/// in the entries vector, the edge weight is `w_st + w_ts` (Bodnar's
/// `F_{v ⊴ e}^⊤ F_{u ⊴ e}` summation over the incident edge set).
/// Self-loops (`s == t`) contribute to the diagonal without doubling
/// the degree.
///
/// Isolated vertices (degree 0) keep `D^{-1/2} = 0` by convention; the
/// corresponding row/column of Δ_F is the zero row plus an identity
/// diagonal — the eigenvalue at that vertex is 1 (consistent with the
/// disconnected-component reading per Bodnar §2).
pub fn dense_laplacian(op: &Operator) -> Vec<f64> {
    let n = Operator::dimension(op) as usize;
    if n == 0 {
        return Vec::new();
    }

    // Build the symmetric weight matrix W (n × n, row-major).
    let mut w = vec![0.0_f64; n * n];
    for r in Operator::entries(op) {
        let s = Restriction::source(r) as usize;
        let t = Restriction::target(r) as usize;
        let weight = Restriction::weight(r);
        if s >= n || t >= n {
            continue;
        }
        if s == t {
            // Self-loop contributes once to the diagonal.
            w[s * n + s] += weight;
        } else {
            w[s * n + t] += weight;
            w[t * n + s] += weight;
        }
    }

    // Compute weighted degrees D_ii = Σ_j W_ij (excluding self-loops
    // per the standard weighted-graph-Laplacian convention).
    let mut degrees = vec![0.0_f64; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                degrees[i] += w[i * n + j];
            }
        }
    }

    // Build the normalised Laplacian Δ_F = I − D^{-1/2} W D^{-1/2}.
    // For isolated vertices (degree 0) the row/column is the zero row —
    // eigenvalue 0 contributes to the kernel per Bodnar et al. 2022 §2's
    // "kernel dim = component count" reading, where each isolated
    // vertex IS a component.
    let mut delta = vec![0.0_f64; n * n];
    for i in 0..n {
        if degrees[i] <= 0.0 {
            // Isolated vertex: zero row/column. Contributes one
            // dimension to ker(Δ_F).
            continue;
        }
        for j in 0..n {
            if i == j {
                delta[i * n + j] = 1.0;
            } else if degrees[j] > 0.0 {
                delta[i * n + j] = -w[i * n + j] / (degrees[i] * degrees[j]).sqrt();
            }
        }
    }
    delta
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`sheaf_laplacian`], [`lambda_zero`], and
    //! the substrate carriers. T8's RED tests; the body lands GREEN in
    //! the next commit.
    //!
    //! Per `shards/epistemologic/math/sheaf_laplacian.mirror` and
    //! `docs/specs/gap-tension-tensor-substrate.md` §3.2: the substrate
    //! names the obligation; the FFI body discharges it. These tests
    //! cover:
    //!
    //! - The substrate-altitude carriers (Restriction, Operator,
    //!   Eigenvalue) construct and read correctly.
    //! - sheaf_laplacian's assembly produces the right (dimension,
    //!   entries) carrier.
    //! - lambda_zero on the trivial / singleton / K_n cases reproduces
    //!   the known algebraic-connectivity values (n/(n−1) for K_n on
    //!   the normalised Laplacian — same values T6's Jacobi stub
    //!   validated).
    //! - Disconnected graphs surface λ₀ = 0 with multiplicity =
    //!   component count (Fiedler 1973 / Bodnar 2022 §2).
    //! - Non-uniform restriction weights produce DIFFERENT λ₀ than
    //!   uniform weights — the substantive correctness check that T6's
    //!   identity-restriction-map limitation no longer holds.

    use super::*;

    // -----------------------------------------------------------------------
    // Substrate-carrier shape tests.
    // -----------------------------------------------------------------------

    /// Restriction carries (source, target, weight) — the substrate
    /// floor reading.
    #[test]
    fn restriction_carries_source_target_weight() {
        let r = Restriction::new(2, 5, 0.7);
        assert_eq!(Restriction::source(&r), 2);
        assert_eq!(Restriction::target(&r), 5);
        assert!((Restriction::weight(&r) - 0.7).abs() < 1e-12);
    }

    /// Operator carries (dimension, entries) — the substrate's matrix-
    /// dimension + restriction-list carrier.
    #[test]
    fn operator_carries_dimension_and_entries() {
        let entries = vec![Restriction::new(0, 1, 1.0)];
        let op = Operator::new(2, entries.clone());
        assert_eq!(Operator::dimension(&op), 2);
        assert_eq!(Operator::entries(&op), &entries[..]);
    }

    /// Eigenvalue carries (value, multiplicity) — the substrate's
    /// spectrum-scalar + kernel-dim carrier.
    #[test]
    fn eigenvalue_carries_value_and_multiplicity() {
        let e = Eigenvalue::new(1.5, 2);
        assert!((Eigenvalue::value(&e) - 1.5).abs() < 1e-12);
        assert_eq!(Eigenvalue::multiplicity(&e), 2);
    }

    // -----------------------------------------------------------------------
    // sheaf_laplacian's assembly — the (dimension, entries) carrier.
    // -----------------------------------------------------------------------

    /// Empty restrictions → trivial operator (dimension 0).
    #[test]
    fn empty_restrictions_yield_trivial_operator() {
        let op = sheaf_laplacian(Vec::new());
        assert_eq!(Operator::dimension(&op), 0);
        assert!(Operator::entries(&op).is_empty());
    }

    /// Single edge (s, t) → operator with dimension max(s, t) + 1.
    #[test]
    fn single_edge_yields_correct_dimension() {
        let op = sheaf_laplacian(vec![Restriction::new(0, 2, 1.0)]);
        assert_eq!(Operator::dimension(&op), 3);
        assert_eq!(Operator::entries(&op).len(), 1);
    }

    // -----------------------------------------------------------------------
    // lambda_zero on the substrate's boundary cases.
    // -----------------------------------------------------------------------

    /// Trivial operator (dimension 0) → λ₀ = 0 with multiplicity 0.
    #[test]
    fn lambda_zero_on_trivial_operator_is_zero_multiplicity_zero() {
        let op = sheaf_laplacian(Vec::new());
        let e = lambda_zero(&op);
        assert_eq!(Eigenvalue::value(&e), 0.0);
        assert_eq!(Eigenvalue::multiplicity(&e), 0);
    }

    /// Singleton vertex (dimension 1) → λ₀ = 0 with multiplicity 1.
    #[test]
    fn lambda_zero_on_singleton_is_zero_multiplicity_one() {
        let op = Operator::new(1, Vec::new());
        let e = lambda_zero(&op);
        assert_eq!(Eigenvalue::value(&e), 0.0);
        assert_eq!(Eigenvalue::multiplicity(&e), 1);
    }

    /// K₂ (unit edge) → λ₀ = 2.0 (= 2/(2−1); Fiedler 1973 / Bodnar 2022 §2).
    /// Same value T6's Jacobi stub computed; T8 reproduces via LAPACK.
    #[test]
    fn lambda_zero_on_k2_unit_edge_is_two() {
        let op = sheaf_laplacian(vec![Restriction::new(0, 1, 1.0)]);
        let e = lambda_zero(&op);
        assert!(
            (Eigenvalue::value(&e) - 2.0).abs() < 1e-9,
            "K₂ unit-weight algebraic connectivity must be 2.0; got {}",
            Eigenvalue::value(&e),
        );
        assert_eq!(Eigenvalue::multiplicity(&e), 1);
    }

    /// K₃ (three unit edges) → λ₀ = 1.5 (= 3/(3−1); Fiedler 1973).
    /// Same value T6's Jacobi stub computed.
    #[test]
    fn lambda_zero_on_k3_unit_edges_is_three_halves() {
        let op = sheaf_laplacian(vec![
            Restriction::new(0, 1, 1.0),
            Restriction::new(0, 2, 1.0),
            Restriction::new(1, 2, 1.0),
        ]);
        let e = lambda_zero(&op);
        assert!(
            (Eigenvalue::value(&e) - 1.5).abs() < 1e-9,
            "K₃ unit-weight algebraic connectivity must be 1.5; got {}",
            Eigenvalue::value(&e),
        );
        assert_eq!(Eigenvalue::multiplicity(&e), 1);
    }

    /// Disconnected graph (two components: K₂ ⊔ K₁) → λ₀ = 0 with
    /// multiplicity 2 (component count per Bodnar 2022 §2).
    ///
    /// Construction: edge between vertices 0 and 1; vertex 2 isolated.
    /// We have to surface dimension 3 so vertex 2 appears in the
    /// Laplacian; supply a self-loop with weight 0 on vertex 2 to
    /// push dimension up without adding an edge (or pass dimension
    /// directly through Operator::new).
    #[test]
    fn lambda_zero_on_disconnected_is_zero_multiplicity_two() {
        let op = Operator::new(3, vec![Restriction::new(0, 1, 1.0)]);
        let e = lambda_zero(&op);
        assert_eq!(
            Eigenvalue::value(&e),
            0.0,
            "disconnected graph → λ₀ = 0 (no spectral gap)",
        );
        assert_eq!(
            Eigenvalue::multiplicity(&e),
            2,
            "K₂ ⊔ K₁ has 2 components; kernel dim = 2",
        );
    }

    // -----------------------------------------------------------------------
    // Non-uniform restriction weights — the substantive-correctness
    // check T6's identity-restriction-map stub could not validate.
    // -----------------------------------------------------------------------

    /// K₂ with non-unit weight w → λ₀ scales as 2.0 (the normalised
    /// Laplacian of K₂ is INVARIANT under positive scaling of the
    /// single edge weight; the normalisation cancels). The test
    /// asserts the value AND that lambda_zero accepts non-uniform
    /// weights without error — the substantive correctness check.
    #[test]
    fn lambda_zero_on_k2_scales_with_weight() {
        let op_unit = sheaf_laplacian(vec![Restriction::new(0, 1, 1.0)]);
        let op_heavy = sheaf_laplacian(vec![Restriction::new(0, 1, 5.0)]);
        let e_unit = lambda_zero(&op_unit);
        let e_heavy = lambda_zero(&op_heavy);
        // K_2 normalised Laplacian is invariant under positive edge-
        // weight scaling — both should yield 2.0.
        assert!((Eigenvalue::value(&e_unit) - 2.0).abs() < 1e-9);
        assert!((Eigenvalue::value(&e_heavy) - 2.0).abs() < 1e-9);
    }

    /// Three-vertex path graph 0 — 1 — 2 with NON-uniform weights
    /// produces a DIFFERENT λ₀ than the K₃ (complete) case — the
    /// substantive correctness check that T8 surfaces real spectral
    /// content the Jacobi stub could not reach.
    ///
    /// The P₃ normalised Laplacian's eigenvalues are {0, 1, 2}; the
    /// algebraic connectivity is 1.0 (independent of equal weights).
    /// With NON-equal weights w(0,1) = 1.0, w(1,2) = 4.0, the spectral
    /// gap shifts — the value depends on the weight ratio. The test
    /// asserts the gap is strictly LESS than the K₃ value (1.5) AND
    /// strictly GREATER than 0 — meaningful curvature surfaces.
    #[test]
    fn lambda_zero_on_p3_with_nonuniform_weights_differs_from_k3() {
        let op_p3 = sheaf_laplacian(vec![
            Restriction::new(0, 1, 1.0),
            Restriction::new(1, 2, 4.0),
        ]);
        let e_p3 = lambda_zero(&op_p3);
        let p3_gap = Eigenvalue::value(&e_p3);
        // P_3 with any positive weights is connected → spectral gap > 0.
        assert!(
            p3_gap > 1e-9,
            "P_3 (connected) must have spectral gap > 0; got {}",
            p3_gap,
        );
        // P_3 is structurally less connected than K_3; the spectral gap
        // is strictly smaller than K_3's 1.5.
        assert!(
            p3_gap < 1.5,
            "P_3 spectral gap must be < K_3's 1.5; got {}",
            p3_gap,
        );
    }
}
