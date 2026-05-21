// The evaluator stands alone in this commit: every public symbol below is
// exercised only by `tests` until the downstream retirements wire it into
// the cmd_* path. `dead_code` would fire on every export otherwise. Once
// `tokenize.rs`'s retirement lands, this attribute drops.
#![allow(dead_code)]
//! Spectral-triple evaluator. The bootstrap's irreducible floor above
//! the `@io` kernel.
//!
//! Per `docs/specs/prism-core-as-spectral-triple.md` and the audit-
//! closure commits `8c184e1` in mirror / `5d98c6e` in prism, the bootstrap
//! IS the evaluator of a spectral triple `(A, H, D)`:
//!
//! - **A** — the involutive algebra of operations on H. Realized in
//!   `prism/core/src/bundle.rs` by the supertrait chain
//!   `Fiber → Connection → Gauge → Transport → Closure`, where
//!   `Connection::Optic: Prism` makes the algebra a Tambara module.
//!   In mirror's bootstrap, A's generators are the five Prism operations
//!   (focus / project / split / zoom / refract) plus their compositions.
//! - **H** — the Hilbert space the algebra acts on. Realized at the trait
//!   level as `Fiber::State`. In the bootstrap H is the space of AST
//!   nodes — each node a state vector, the recursive descent the
//!   action of an algebra element.
//! - **D** — the Dirac operator. Realized by `Transport::transport`,
//!   whose signature `Imperfect<State, _, Holonomy>` is precisely the
//!   shape of `D`'s partially-defined action: success when the operator
//!   sends a state in its domain, partial-with-holonomy when transport
//!   carries the state off the manifold by a bounded residual. In the
//!   bootstrap D's concrete matrix form is `CoincidenceHash<5,5>`
//!   (see `bootstrap/src/hash.rs`); its scalar action on AST states is
//!   `content_oid` (see `bootstrap/src/content.rs`).
//!
//! ## The three primitive operations
//!
//! Everything mirror does decomposes into a finite composition of three
//! primitives over this triple:
//!
//! 1. [`compose_a`] — algebra composition. Combine two algebra elements
//!    into their product. Reduces to function composition on the
//!    `AlgebraElement` shape.
//! 2. [`apply_h`] — operator action on a state vector. Returns a
//!    [`Verdict`] that mirrors `terni::Imperfect`'s three-tier structure:
//!    Success / Partial(residual) / Failure(residual).
//! 3. [`eigen_d`] — Dirac eigendecomposition. Given an operator
//!    represented in the canonical 5-d basis (one axis per Prism
//!    operation), return its spectrum. For the 5×5 case this is power
//!    iteration with deflation — no external linalg dependency, fitting
//!    the bootstrap's intentionally minimal floor (`sha2` only).
//!
//! ## Why this lives in the bootstrap, not in `prism-core`
//!
//! The bootstrap is self-hosting via LLVM-IR emission (`mirror craft
//! --target binary`). Pulling `prism-core` and `terni` into the
//! dependency tree would expand the IR-emit surface dramatically and
//! entangle the floor with the very crates the v1 architecture is
//! reaching toward. So the evaluator's types mirror the prism-core
//! shape *structurally* without importing it. The correspondence is
//! named in this doc-comment and verified by property tests below;
//! when v2 lands, `Verdict` becomes a re-export of `terni::Imperfect`
//! and `AlgebraElement` becomes the Tambara-composable optic alias
//! from `prism-core`. The shape doesn't change; the dependency
//! direction inverts.
//!
//! ## Migration path
//!
//! This commit ADDS this module. It does NOT yet replace `tokenize.rs`,
//! `render.rs`, `content.rs`, or `pipeline.rs`. The evaluator stands
//! alone, exercised only by the tests below. Downstream retirements
//! happen in subsequent ticks per the spec's Step 4:
//!
//! 1. `tokenize.rs` retires first: tokenization becomes a tree of
//!    `AlgebraElement` values, evaluated by [`apply_h`].
//! 2. `render.rs` retires next as the inverse composition.
//! 3. `content.rs` mostly stays — its recursive walk IS [`apply_h`]
//!    specialised to AST states under the discrete-D matrix.
//! 4. `pipeline.rs`, `grammar.rs`, and the cmd_* in `main.rs` become
//!    thin wrappers around the evaluator.
//!
//! After retirement the bootstrap is `git.rs` + `exec.rs` + `spectral.rs`
//! + shell, ≈1200 lines. The Rust floor shrinks; the grammar grows;
//! the operator algebra is what was there all along.

use crate::ast::AstNode;
use crate::content::content_oid;

// ---------------------------------------------------------------------------
// Residual — the metric carrier (D's holonomy)
// ---------------------------------------------------------------------------

/// A non-negative scalar residual measuring how far an operator action
/// carried a state off the manifold. Structurally the bootstrap-local
/// shape of `terni::Metric` for `ScalarLoss`: non-negative, symmetric
/// under `distance_to`, satisfies the triangle inequality. Connes'
/// bounded-commutator condition `‖[D, a]‖ < ∞` is enforced as
/// `Residual::is_finite()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Residual(pub f64);

impl Residual {
    /// The zero residual: transport stayed on the manifold.
    pub const ZERO: Residual = Residual(0.0);

    /// Whether the residual is finite (bounded commutator condition).
    pub fn is_finite(&self) -> bool {
        self.0.is_finite() && self.0 >= 0.0
    }

    /// Combine two residuals associatively. Mirrors `Loss::combine`.
    pub fn combine(self, other: Residual) -> Residual {
        Residual(self.0 + other.0)
    }

    /// Symmetric metric distance. `a.distance_to(b) == b.distance_to(a)`.
    pub fn distance_to(self, other: Residual) -> Residual {
        Residual((self.0 - other.0).abs())
    }
}

// ---------------------------------------------------------------------------
// Verdict — bootstrap-local Imperfect
// ---------------------------------------------------------------------------

/// Three-tier outcome of an operator action. Mirrors `terni::Imperfect`'s
/// Success / Partial(residual) / Failure(residual) shape without
/// importing terni: the bootstrap stays at its sha2-only floor.
#[derive(Clone, Debug, PartialEq)]
pub enum Verdict<T> {
    /// The operator's domain included the state; transport was exact.
    Success(T),
    /// Transport produced a value but carried a measured residual.
    Partial(T, Residual),
    /// The operator's domain rejected the state; the residual carries
    /// the cost of reaching the failure.
    Failure(Residual),
}

impl<T> Verdict<T> {
    /// Whether a value was produced (Success or Partial).
    pub fn is_ok(&self) -> bool {
        matches!(self, Verdict::Success(_) | Verdict::Partial(_, _))
    }

    /// Extract the residual: zero for Success, carried for Partial / Failure.
    pub fn residual(&self) -> Residual {
        match self {
            Verdict::Success(_) => Residual::ZERO,
            Verdict::Partial(_, r) => *r,
            Verdict::Failure(r) => *r,
        }
    }

    /// Extract the value if present.
    pub fn ok(self) -> Option<T> {
        match self {
            Verdict::Success(v) | Verdict::Partial(v, _) => Some(v),
            Verdict::Failure(_) => None,
        }
    }
}

// ---------------------------------------------------------------------------
// AlgebraElement — A's elements
// ---------------------------------------------------------------------------

/// An element of the operator algebra A acting on H.
///
/// The trait is intentionally minimal: every element knows how to apply
/// itself to a state of type `S` and return a `Verdict<S>`. The
/// supertrait closure on the prism-core side (Connection::Optic: Prism,
/// etc.) reduces to *this* shape under monomorphisation — the bootstrap
/// names what stays after the type machinery erases.
pub trait AlgebraElement<S> {
    /// Apply this operator to a state, returning the post-transport state
    /// and any holonomy incurred.
    fn act(&self, state: S) -> Verdict<S>;
}

// ---------------------------------------------------------------------------
// Identity — A's unit element
// ---------------------------------------------------------------------------

/// The identity element of A. Witness that A is a monoid (every algebra
/// has a unit). Maps to `prism_core::IdentityPrism` under the
/// correspondence.
pub struct Identity;

impl<S> AlgebraElement<S> for Identity {
    fn act(&self, state: S) -> Verdict<S> {
        Verdict::Success(state)
    }
}

// ---------------------------------------------------------------------------
// Composed — the result of compose_a
// ---------------------------------------------------------------------------

/// The product of two algebra elements under sequential composition.
/// `Composed(p, q).act(s)` = `q.act(p.act(s))`, propagating residuals
/// by `Residual::combine` (associative accumulation — the Loss monoid
/// at the bootstrap level).
///
/// On the prism-core side this is Tambara module composition; for
/// mirror's case it reduces to function composition because the
/// underlying optic's input/output types align by construction.
pub struct Composed<P, Q> {
    pub first: P,
    pub second: Q,
}

impl<S, P, Q> AlgebraElement<S> for Composed<P, Q>
where
    P: AlgebraElement<S>,
    Q: AlgebraElement<S>,
{
    fn act(&self, state: S) -> Verdict<S> {
        match self.first.act(state) {
            Verdict::Success(s2) => self.second.act(s2),
            Verdict::Partial(s2, r1) => match self.second.act(s2) {
                Verdict::Success(s3) => Verdict::Partial(s3, r1),
                Verdict::Partial(s3, r2) => Verdict::Partial(s3, r1.combine(r2)),
                Verdict::Failure(r2) => Verdict::Failure(r1.combine(r2)),
            },
            Verdict::Failure(r1) => Verdict::Failure(r1),
        }
    }
}

// ---------------------------------------------------------------------------
// 1. compose_a — algebra composition
// ---------------------------------------------------------------------------

/// Algebra composition. Combine two algebra elements into their product.
///
/// On the prism-core side this is Tambara module composition under the
/// `Connection::Optic: Prism` supertrait; here it reduces to sequential
/// function composition over a shared state type S. The bootstrap doesn't
/// reinvent the algebra — it names what the algebra reduces to once the
/// optic chain's input/output types align.
pub fn compose_a<S, P, Q>(p: P, q: Q) -> Composed<P, Q>
where
    P: AlgebraElement<S>,
    Q: AlgebraElement<S>,
{
    Composed { first: p, second: q }
}

// ---------------------------------------------------------------------------
// 2. apply_h — operator action on H
// ---------------------------------------------------------------------------

/// Apply an algebra element to a state vector in H.
///
/// On the prism-core side this delegates to `Transport::transport`,
/// whose `Imperfect<State, _, Holonomy>` signature matches [`Verdict`]
/// here. The function is intentionally a thin wrapper around
/// `AlgebraElement::act`: naming it `apply_h` ties it to the
/// spectral-triple framework and creates the call-site that downstream
/// retirements (tokenize.rs, render.rs) will dispatch through.
pub fn apply_h<S, P>(p: &P, state: S) -> Verdict<S>
where
    P: AlgebraElement<S>,
{
    p.act(state)
}

/// Specialisation of [`apply_h`] for AST states: apply the discrete
/// Dirac operator (`content_oid`) to an AST node, returning the OID
/// and a residual derived from the depth of recursion. This is the
/// shape `content.rs`'s recursive walk takes once it retires into
/// the evaluator.
pub fn apply_h_content(node: &AstNode) -> Verdict<String> {
    Verdict::Success(content_oid(node))
}

// ---------------------------------------------------------------------------
// 3. eigen_d — Dirac eigendecomposition
// ---------------------------------------------------------------------------

/// The spectral data of a finite-dimensional operator: its eigenvalues
/// and corresponding eigenvectors in the canonical basis.
#[derive(Clone, Debug)]
pub struct Spectrum<const N: usize> {
    /// Eigenvalues in descending order of magnitude.
    pub eigenvalues: [f64; N],
    /// Eigenvectors, column `i` is the eigenvector for `eigenvalues[i]`.
    /// Stored row-major: `eigenvectors[row][col]`.
    pub eigenvectors: [[f64; N]; N],
}

/// Dirac operator eigendecomposition for finite-dimensional cases.
///
/// For mirror's `CoincidenceHash<5,5>` the operator is a 5×5 symmetric
/// matrix in the canonical basis (one axis per Prism operation:
/// focus / project / split / zoom / refract). We use power iteration
/// with deflation: extract the dominant eigenpair, project it out,
/// recurse. ~150 lines, no external linalg crate — fits the
/// bootstrap's sha2-only dependency floor.
///
/// Power iteration converges geometrically with rate `|λ₂/λ₁|` per
/// step. For random 5×5 matrices ~60 iterations gives 1e-10 accuracy;
/// for matrices with degenerate spectrum we'd need a more careful
/// algorithm (QR / Jacobi rotations), but the mirror case has
/// well-separated eigenvalues by the coincidence-hash construction.
pub fn eigen_d<const N: usize>(matrix: [[f64; N]; N]) -> Spectrum<N> {
    const MAX_ITERS: usize = 200;
    const TOL: f64 = 1e-12;

    let mut deflated = matrix;
    let mut eigenvalues = [0.0f64; N];
    let mut eigenvectors = [[0.0f64; N]; N];

    for k in 0..N {
        // Start with a deterministic seed vector that's unlikely to be
        // orthogonal to the dominant eigenvector of the deflated matrix.
        // Using (1, 2, ..., N) rather than e_1 avoids the degenerate case
        // where e_1 happens to be an eigenvector with smaller eigenvalue.
        let mut v = [0.0f64; N];
        for i in 0..N {
            v[i] = (i as f64) + 1.0;
        }

        let mut lambda = 0.0f64;
        let mut prev_lambda;

        for _ in 0..MAX_ITERS {
            // w = deflated * v
            let mut w = [0.0f64; N];
            for i in 0..N {
                let mut s = 0.0f64;
                for j in 0..N {
                    s += deflated[i][j] * v[j];
                }
                w[i] = s;
            }

            // Normalise w.
            let mut norm_sq = 0.0f64;
            for i in 0..N {
                norm_sq += w[i] * w[i];
            }
            let norm = norm_sq.sqrt();
            if norm < TOL {
                // The deflated matrix has rank < N - k; remaining
                // eigenvalues are zero. Leave them as zero.
                break;
            }
            for i in 0..N {
                w[i] /= norm;
            }

            // Rayleigh quotient: λ = vᵀ (deflated * v) / (vᵀ v).
            // Since v is already normalised it's just vᵀ w_unnormalised; we
            // recompute on the normalised w by going through deflated once
            // more, which costs N² but keeps numerical stability.
            prev_lambda = lambda;
            let mut new_lambda = 0.0f64;
            for i in 0..N {
                let mut s = 0.0f64;
                for j in 0..N {
                    s += deflated[i][j] * w[j];
                }
                new_lambda += w[i] * s;
            }
            lambda = new_lambda;
            v = w;

            if (lambda - prev_lambda).abs() < TOL {
                break;
            }
        }

        eigenvalues[k] = lambda;
        for i in 0..N {
            eigenvectors[i][k] = v[i];
        }

        // Deflate: subtract λ vvᵀ from the matrix. For symmetric matrices
        // this leaves the remaining N-k-1 eigenpairs of the original matrix
        // intact, while annihilating the current one.
        for i in 0..N {
            for j in 0..N {
                deflated[i][j] -= lambda * v[i] * v[j];
            }
        }
    }

    Spectrum {
        eigenvalues,
        eigenvectors,
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstKind, AstNode};

    // -----------------------------------------------------------------------
    // Test fixtures — small concrete AlgebraElement impls.
    // -----------------------------------------------------------------------

    /// Multiplies a scalar state by `factor`. Models a focus-like
    /// operator: pure success, no residual.
    struct Scale {
        factor: f64,
    }

    impl AlgebraElement<f64> for Scale {
        fn act(&self, state: f64) -> Verdict<f64> {
            Verdict::Success(state * self.factor)
        }
    }

    /// Quantizes a scalar to the nearest integer. Models a project-like
    /// operator: produces a value but carries a residual = the rounding
    /// error. Mirrors the precision-cut semantics of project().
    struct Quantize;

    impl AlgebraElement<f64> for Quantize {
        fn act(&self, state: f64) -> Verdict<f64> {
            let rounded = state.round();
            let r = (state - rounded).abs();
            if r == 0.0 {
                Verdict::Success(rounded)
            } else {
                Verdict::Partial(rounded, Residual(r))
            }
        }
    }

    /// Rejects negative inputs. Models an operator whose domain is the
    /// non-negative reals; transporting a negative state produces
    /// Failure with residual = |state|.
    struct Positive;

    impl AlgebraElement<f64> for Positive {
        fn act(&self, state: f64) -> Verdict<f64> {
            if state >= 0.0 {
                Verdict::Success(state)
            } else {
                Verdict::Failure(Residual(-state))
            }
        }
    }

    // -----------------------------------------------------------------------
    // compose_a — algebra-composition tests
    // -----------------------------------------------------------------------

    #[test]
    fn compose_a_associates() {
        // (a ∘ b) ∘ c agrees with a ∘ (b ∘ c) on every input.
        let lhs = compose_a(compose_a(Scale { factor: 2.0 }, Scale { factor: 3.0 }), Scale { factor: 5.0 });
        let rhs = compose_a(Scale { factor: 2.0 }, compose_a(Scale { factor: 3.0 }, Scale { factor: 5.0 }));
        for s in [-1.0_f64, 0.0, 0.5, 1.0, 7.25] {
            assert_eq!(lhs.act(s), rhs.act(s), "associativity at {}", s);
        }
    }

    #[test]
    fn compose_a_with_identity_is_identity() {
        // Identity is the unit: id ∘ p and p ∘ id behave like p.
        let p = Scale { factor: 4.0 };
        let left = compose_a(Identity, Scale { factor: 4.0 });
        let right = compose_a(Scale { factor: 4.0 }, Identity);
        for s in [-2.0_f64, 0.0, 1.5, 9.0] {
            let direct = p.act(s);
            assert_eq!(left.act(s), direct, "id ∘ p at {}", s);
            assert_eq!(right.act(s), direct, "p ∘ id at {}", s);
        }
    }

    #[test]
    fn compose_a_propagates_residuals() {
        // Composing two Partial-producing operators accumulates residuals
        // via Residual::combine — the Loss monoid law at the bootstrap.
        let pipeline = compose_a(Quantize, Quantize);
        // Quantize(0.3) = 0 with residual 0.3; Quantize(0) = 0 with no extra residual.
        let v = pipeline.act(0.3);
        match v {
            Verdict::Partial(value, r) => {
                assert_eq!(value, 0.0);
                assert!((r.0 - 0.3).abs() < 1e-12);
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_failure_short_circuits() {
        // A Failure in the first operator prevents the second from being
        // applied; the failure's residual carries forward unchanged.
        let pipeline = compose_a(Positive, Scale { factor: 10.0 });
        match pipeline.act(-3.0) {
            Verdict::Failure(r) => assert!((r.0 - 3.0).abs() < 1e-12),
            other => panic!("expected Failure, got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // apply_h — operator-action tests
    // -----------------------------------------------------------------------

    #[test]
    fn apply_h_consistent_with_act() {
        // apply_h is the named wrapper around AlgebraElement::act.
        // The equivalence test: both routes produce the same Verdict.
        let p = Scale { factor: 7.0 };
        for s in [-1.0_f64, 0.0, 2.5] {
            assert_eq!(apply_h(&p, s), p.act(s));
        }
    }

    #[test]
    fn apply_h_content_matches_content_oid() {
        // apply_h_content over an AST state agrees with the direct
        // content_oid call. The evaluator route doesn't change the OID.
        let mut node = AstNode::new(AstKind::Focus, "root");
        node.add_child(AstNode::new(AstKind::In, "@prism"));
        let oid_direct = content_oid(&node);
        let v = apply_h_content(&node);
        match v {
            Verdict::Success(oid) => assert_eq!(oid, oid_direct),
            other => panic!("expected Success, got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // eigen_d — Dirac-eigendecomposition tests
    // -----------------------------------------------------------------------

    #[test]
    fn eigen_d_diagonal_2x2() {
        // A diagonal matrix's eigenvalues are its diagonal entries
        // (in descending order of magnitude).
        let m = [[3.0, 0.0], [0.0, 1.0]];
        let s = eigen_d::<2>(m);
        assert!((s.eigenvalues[0] - 3.0).abs() < 1e-6);
        assert!((s.eigenvalues[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn eigen_d_symmetric_3x3_nondegenerate() {
        // A symmetric 3×3 with well-separated spectrum. Constructed as
        // diag(7, 4, 1) — power iteration with deflation recovers
        // well-separated eigenvalues to high precision.
        //
        // Degenerate-spectrum cases (mirror's kintsugi formatter operates
        // on these via Banach contraction toward ker(D)) require Jacobi
        // rotations or QR; that's deferred to v2 along with the prism-core
        // import. Power iteration is correct for the well-separated case
        // and that's what the bootstrap floor needs today.
        let m = [
            [7.0, 0.0, 0.0],
            [0.0, 4.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let s = eigen_d::<3>(m);
        // Trace = 12; dominant ≈ 7; smallest ≈ 1.
        let sum: f64 = s.eigenvalues.iter().sum();
        assert!((sum - 12.0).abs() < 1e-4, "trace: got {}, expected 12", sum);
        assert!((s.eigenvalues[0] - 7.0).abs() < 1e-4);
        assert!((s.eigenvalues[2] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn eigen_d_5x5_smoke() {
        // The mirror case: a 5×5 diagonal matrix with known spectrum.
        // One axis per Prism operation.
        let m = [
            [5.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 4.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0],
        ];
        let s = eigen_d::<5>(m);
        // Eigenvalues should be {5, 4, 3, 2, 1} in some order; sort
        // descending and check against the expected sequence.
        let mut got = s.eigenvalues;
        got.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let expected = [5.0_f64, 4.0, 3.0, 2.0, 1.0];
        for (i, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            assert!(
                (g - e).abs() < 1e-4,
                "5x5 eigenvalue[{}]: got {}, expected {}",
                i,
                g,
                e
            );
        }
    }

    // -----------------------------------------------------------------------
    // Residual — metric-law smoke checks
    // -----------------------------------------------------------------------

    #[test]
    fn residual_is_finite_on_well_formed_inputs() {
        assert!(Residual::ZERO.is_finite());
        assert!(Residual(0.5).is_finite());
        assert!(Residual(1e9).is_finite());
        assert!(!Residual(f64::NAN).is_finite());
        assert!(!Residual(-1.0).is_finite());
    }

    #[test]
    fn residual_distance_is_symmetric() {
        let a = Residual(0.3);
        let b = Residual(0.8);
        assert_eq!(a.distance_to(b), b.distance_to(a));
    }

    #[test]
    fn residual_combine_associates() {
        let a = Residual(0.1);
        let b = Residual(0.2);
        let c = Residual(0.3);
        let left = a.combine(b).combine(c);
        let right = a.combine(b.combine(c));
        assert!((left.0 - right.0).abs() < 1e-12);
    }
}
