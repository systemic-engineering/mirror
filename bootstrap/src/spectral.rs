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
//! IS the evaluator of a spectral triple `(A, H, D)` over
//! `prism_core`'s verified spectral-triple substrate. The trait chain in
//! `prism_core::bundle` (Fiber → Connection → Gauge → Transport →
//! Closure) realizes the structure; this module names what the
//! bootstrap evaluator does over it.
//!
//! - **A** — the involutive algebra of operations on H. Realized by
//!   `prism_core::Prism`: the Tambara-composable optic. In mirror's
//!   bootstrap A's generators are the five Prism operations
//!   (focus / project / split / zoom / refract) plus their compositions.
//!   The identity element is [`prism_core::IdentityPrism`] (the unit of
//!   A as a monoid).
//! - **H** — the Hilbert space the algebra acts on. The state type S
//!   carried through a beam pipeline. In the bootstrap H is the space
//!   of AST nodes — each node a state vector, the recursive descent
//!   the action of an algebra element.
//! - **D** — the Dirac operator. Realized by `Transport::transport`,
//!   whose `terni::Imperfect<State, _, Holonomy>` signature is precisely
//!   the shape of D's partially-defined action: Success when the
//!   operator sends a state in its domain, Partial-with-holonomy when
//!   transport carries the state off the manifold by a bounded
//!   residual. The holonomy is a [`terni::Metric`] (non-negative,
//!   symmetric, triangle inequality) — Connes' bounded-commutator
//!   condition `‖[D, a]‖ < ∞` is the type-level constraint. Domain
//!   rejection is encoded as a Partial verdict carrying the absorbing
//!   `ScalarLoss::total()`, not as a typed Failure (the algebra has
//!   `Error = Infallible` so it remains closed under identity
//!   composition). In the bootstrap D's concrete matrix form is
//!   `CoincidenceHash<5,5>` (see `bootstrap/src/hash.rs`); its scalar
//!   action on AST states is [`apply_h_content`] (dispatched through
//!   [`ContentOidPrism`] — the retirement of the standalone
//!   `content.rs` module, per `docs/specs/bootstrap-retirement-plan.md`
//!   Tick 1).
//!
//! ## The three primitive operations
//!
//! Everything mirror does decomposes into a finite composition of three
//! primitives over this triple:
//!
//! 1. [`compose_a`] — algebra composition. Run two algebra elements
//!    sequentially on a state, accumulating loss via the `Metric` monoid
//!    on the holonomy carrier. The composition is associative because
//!    `terni::Loss::combine` is, and the [`IdentityPrism`] is its unit.
//! 2. [`apply_h`] — operator action on a state vector. Wraps a single
//!    `prism_core::Prism`'s focus / project / refract sweep and returns
//!    the resulting [`Verdict`] in H. The function is intentionally a
//!    thin wrapper around `prism_core::apply`: naming it `apply_h` ties
//!    it to the spectral-triple framework and creates the call-site that
//!    downstream retirements (tokenize.rs, render.rs) will dispatch
//!    through.
//! 3. [`eigen_d`] — Dirac eigendecomposition. Given an operator
//!    represented in the canonical 5-d basis (one axis per Prism
//!    operation), return its spectrum. For the 5×5 case this is power
//!    iteration with deflation — no external linalg dependency, fitting
//!    the bootstrap's intentionally minimal floor (the only added
//!    crates are `prism-core` and `terni`, themselves zero-IO).
//!
//! [`prism_core::IdentityPrism`]: prism_core::IdentityPrism
//! [`terni::Imperfect`]: terni::Imperfect
//! [`terni::Metric`]: terni::Metric

use prism_core::{apply as prism_apply, Beam, Optic, Prism, ScalarLoss};
use terni::Imperfect;

use crate::ast::{AstKind, AstNode};
use crate::hash::hash_tagged;

// ---------------------------------------------------------------------------
// State carriers — the bootstrap's seed and verdict types.
// ---------------------------------------------------------------------------

/// The seed beam type for an algebra element acting on a state of type
/// `S`. We use `()` as the input position because the state vector is
/// produced by the seed itself — there is no prior input. The error
/// position is [`Infallible`] (matching `prism_core::IdentityPrism`'s
/// shape): the evaluator's algebra elements model "domain rejection" as
/// a Partial verdict with the absorbing `ScalarLoss::total()` (= ∞),
/// not as a typed Failure. This keeps the algebra closed under
/// composition with the identity element. Loss is carried as
/// [`ScalarLoss`] (the bootstrap's [`terni::Metric`] carrier).
///
/// [`Infallible`]: std::convert::Infallible
pub type Seed<S> = Optic<(), S>;

/// The verdict an algebra element returns when acting on a state.
/// Mirrors `prism_core::Transport::transport`'s signature exactly:
/// `terni::Imperfect<S, Infallible, Holonomy>` where the Holonomy is a
/// [`Metric`]. The `Infallible` error position witnesses that algebra
/// elements are total on the algebra's *closure* (every operator is
/// defined; the residual at the boundary of the domain is carried in
/// `ScalarLoss`, which can be `total()` for the absorbing case).
///
/// [`Metric`]: terni::Metric
pub type Verdict<S> = Imperfect<S, std::convert::Infallible, ScalarLoss>;

/// Seed a beam with a starting state value. The source position is `()`
/// (no prior input); the value position carries the state.
pub fn seed<S>(state: S) -> Seed<S> {
    Optic::ok((), state)
}

// ---------------------------------------------------------------------------
// 1. apply_h — operator action on H
// ---------------------------------------------------------------------------

/// Apply an algebra element to a state vector in H.
///
/// The element is any `prism_core::Prism` whose pipeline starts at
/// `Seed<S>` and ends at an `Optic<_, S>` (i.e. the refracted beam
/// still carries a state of type `S`, possibly with accumulated loss).
/// The function runs `prism_core::apply` end-to-end and returns the
/// resulting [`Verdict<S>`] — Success or Partial-with-`ScalarLoss`.
///
/// This wraps `prism_core::apply` rather than reinventing it: the
/// bootstrap stands on prism-core's verified spectral-triple substrate
/// rather than mirroring its shape.
///
/// The `Refracted` type is bound to `Optic<_, S>` (with default `Error`
/// = Infallible and `Loss` = `ScalarLoss`) because the bootstrap
/// evaluator's algebra elements operate uniformly on `H` (state in,
/// state out) and carry their residual in `ScalarLoss` (the `Metric`
/// carrier). The `In` position of the refracted beam holds the
/// previous-stage output type, which is irrelevant after the last
/// stage and dropped by `into_focus`.
pub fn apply_h<S, In, P>(p: &P, state: S) -> Verdict<S>
where
    P: Prism<Input = Seed<S>, Refracted = Optic<In, S>>,
{
    prism_apply(p, seed(state)).into_focus()
}

/// Specialisation of [`apply_h`] for AST states: apply the discrete
/// Dirac operator to an AST node, returning the OID. The canonical
/// path is `prism_apply(&ContentOidPrism, seed(node.clone()))` —
/// the Dirac operator's scalar action on the AST state vector,
/// expressed as a Prism dispatched through `apply_h`.
///
/// Per `docs/specs/bootstrap-retirement-plan.md` (Tick 1), this is
/// where the recursive walk that used to live in `content.rs` now
/// dispatches. The walk is preserved verbatim — the Prism's `focus`
/// phase carries the same per-AstKind dispatch the C original and
/// the prior Rust port used; only the call-site idiom moves from
/// hand-written recursion into `apply_h`.
pub fn apply_h_content(node: &AstNode) -> Verdict<String> {
    prism_apply(&ContentOidPrism, seed(node.clone())).into_focus()
}

/// Convenience wrapper for the common case: take an `AstNode` by
/// reference and produce the OID `String` directly. Every call-site
/// that used to read `content_oid(&ast)` now reads
/// `compute_content_oid(&ast)`. The function unwraps the `Verdict`
/// to its `Success` payload — `ContentOidPrism` is total over
/// well-formed ASTs (`AstKind::Dark` nodes are hashed under the
/// `"dark"` tag, not produced as Partial), so the unwrap is safe.
pub fn compute_content_oid(node: &AstNode) -> String {
    match apply_h_content(node) {
        Imperfect::Success(oid) => oid,
        // Defensive — `ContentOidPrism` never produces Partial today.
        // If a future combinator does, treat it as a loss-bearing OID
        // and surface the payload; callers concerned with strictness
        // should call `apply_h_content` directly.
        Imperfect::Partial(oid, _) => oid,
        // `Verdict<String>` has `Error = Infallible`; this arm is
        // structurally unreachable but the type system can't see it
        // without `unreachable!()`.
        Imperfect::Failure(_, _) => unreachable!(
            "ContentOidPrism has Error = Infallible; Failure is uninhabited"
        ),
    }
}

// ---------------------------------------------------------------------------
// ContentOidPrism — the discrete Dirac operator's scalar action on H.
//
// Parametric form (one Prism, internal dispatch on `AstKind`) rather than
// per-kind composition (ten Prisms composed via `compose_a`). Rationale:
//
// - The current `content::content_oid` is one `match` over `AstKind`.
//   A single Prism whose `focus` phase carries the same match is the
//   smallest change with the same observable behaviour — the hash bytes
//   are byte-stable by construction.
// - The per-kind alternative would surface the algebra structure more
//   honestly (ten elements of A, composed) but would multiply the
//   trait-bound surface and triple the LOC for zero observable change.
//   Parametric keeps the floor minimal; per-kind specialisation is a
//   profile-driven decision once `cargo bloat` shows a hot path.
// - Per `docs/specs/bootstrap-retirement-plan.md` §Tick 1 open question:
//   Alex's recommendation is parametric for v1.
// ---------------------------------------------------------------------------

/// The discrete Dirac operator's scalar action on AST states. Its
/// `focus` walks the node, dispatches on `AstKind`, recurses into
/// children via [`apply_h_content`], and emits the OID string. The
/// `project` and `refract` phases pass the OID through unchanged —
/// the work happens in `focus`, matching the shape of the existing
/// `Scale` / `Quantize` test fixtures in this module.
pub struct ContentOidPrism;

impl Prism for ContentOidPrism {
    type Input = Seed<AstNode>;
    type Focused = Optic<AstNode, String>;
    type Projected = Optic<String, String>;
    type Refracted = Optic<String, String>;

    fn focus(&self, beam: Self::Input) -> Self::Focused {
        let node = beam
            .value()
            .expect("ContentOidPrism::focus on dark beam")
            .clone();
        let oid = compute_oid_inner(&node);
        beam.next(oid)
    }

    fn project(&self, beam: Self::Focused) -> Self::Projected {
        let oid = beam
            .value()
            .expect("ContentOidPrism::project on dark beam")
            .clone();
        beam.next(oid)
    }

    fn refract(&self, beam: Self::Projected) -> Self::Refracted {
        let oid = beam
            .value()
            .expect("ContentOidPrism::refract on dark beam")
            .clone();
        beam.next(oid)
    }
}

/// The per-kind dispatch — byte-exact equivalent of the recursive walk
/// that used to live in `bootstrap/src/content.rs`. Each arm reproduces
/// the prior buffer construction verbatim so the resulting OIDs are
/// byte-stable across the retirement.
///
/// Recursion into children goes through [`apply_h_content`] so the
/// `apply_h` call-site is exercised at every level of the AST.
fn compute_oid_inner(node: &AstNode) -> String {
    let mut buf: Vec<u8> = Vec::new();
    match node.kind {
        AstKind::Focus => {
            buf.extend_from_slice(node.name.as_bytes());
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("focus", &buf)
        }
        AstKind::Project => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("project", &buf)
        }
        AstKind::Split => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("split", &buf)
        }
        AstKind::Zoom => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("zoom", &buf)
        }
        AstKind::Refract => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("refract", &buf)
        }
        AstKind::In => hash_tagged("in", node.name.as_bytes()),
        AstKind::Out => hash_tagged("out", node.name.as_bytes()),
        AstKind::IoBinding => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("io_binding", &buf)
        }
        AstKind::MatchExpr => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("match_expr", &buf)
        }
        AstKind::SelectExpr => {
            buf.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
            for c in &node.children {
                let child = compute_content_oid(c);
                buf.push(b':');
                buf.extend_from_slice(child.as_bytes());
            }
            hash_tagged("select_expr", &buf)
        }
        AstKind::Dark => {
            // Per `docs/specs/strict-and-total-classification.md`: hash
            // the verbatim bytes under a `"dark"` tag. Changes to a dark
            // region produce a different OID rather than silently folding
            // into the parent's body.
            let bytes: &[u8] = node
                .body
                .as_deref()
                .map(str::as_bytes)
                .unwrap_or(&[]);
            hash_tagged("dark", bytes)
        }
    }
}

// ---------------------------------------------------------------------------
// Fold5 — the AST catamorphism. One reducer per bundle level.
//
// Per `docs/specs/ast-as-bundle.md`: a `.mirror` file's AST is a Bundle
// morphism written as data. The five operation `AstKind`s
// (Focus / Project / Split / Zoom / Refract) map to the five trait-chain
// levels (Fiber / Connection / Gauge / Transport / Closure); the two IO
// `AstKind`s (In / Out) are the bundle's typed terminals. Any AST-walking
// operation is a fold over that bundle — five reducers, one per level.
//
// **Deviation from spec literal.** The spec shape is `Fold5<Ff, Fp, Fs,
// Fz, Fr, In, Out>` with five `FnMut` closures and `In`/`Out` as
// PhantomData type parameters. The realised shape here drops the `In`
// type parameter (the walker always takes `&AstNode` — the AST is the
// bundle's domain, not a separate `In` type at the Rust level) and adds
// one extra `on_other` reducer to handle the four non-canonical kinds
// (In, Out, Dark, IoBinding, MatchExpr, SelectExpr) — Dark is the
// strict-classification marker; IoBinding/MatchExpr/SelectExpr are Spec
// A/B extensions to the canonical five operations. Closures are `Fn`
// rather than `FnMut` so the `Prism::focus(&self, …)` signature
// typechecks without interior mutability — the bootstrap's two folds
// (content OID, render) are both pure functions.
//
// The walker is post-order: children fold first, then the level
// reducer combines them into the parent's `Out`. Terminal kinds
// (In/Out/Dark) have no children — their `Vec<Out>` is empty.
// ---------------------------------------------------------------------------

/// Catamorphism over the AST. One reducer per bundle trait-chain level
/// (Focus → Fiber, Project → Connection, Split → Gauge, Zoom →
/// Transport, Refract → Closure) plus an `on_other` reducer for the
/// non-canonical AST kinds (In, Out, Dark, IoBinding, MatchExpr,
/// SelectExpr).
///
/// Each reducer takes the current `AstNode` and the `Vec<Out>` of
/// already-folded child results, and returns the `Out` for this node.
/// The walker recurses post-order: a parent's reducer sees its
/// children's folded values, never the children themselves.
///
/// `Fold5` is the second-order shape; specific operations
/// (`content_oid`, `render`, …) become instances. Uniform folds — same
/// reducer at every level — are the degenerate `Fold1` case (see
/// `Fold5::uniform`).
pub struct Fold5<Ff, Fp, Fs, Fz, Fr, Fo, Out>
where
    Ff: Fn(&AstNode, Vec<Out>) -> Out,
    Fp: Fn(&AstNode, Vec<Out>) -> Out,
    Fs: Fn(&AstNode, Vec<Out>) -> Out,
    Fz: Fn(&AstNode, Vec<Out>) -> Out,
    Fr: Fn(&AstNode, Vec<Out>) -> Out,
    Fo: Fn(&AstNode, Vec<Out>) -> Out,
{
    pub on_focus: Ff,
    pub on_project: Fp,
    pub on_split: Fs,
    pub on_zoom: Fz,
    pub on_refract: Fr,
    /// Reducer for non-canonical kinds: In, Out, Dark, IoBinding,
    /// MatchExpr, SelectExpr. The bundle terminals plus the Spec A/B
    /// extensions all route through this one. Per
    /// `ast-as-bundle.md` §"Why this matters" — these are not peers of
    /// the five operations in the bundle algebra; they're the typed
    /// boundary plus the dispatch-extensions.
    pub on_other: Fo,
    _out: core::marker::PhantomData<Out>,
}

impl<Ff, Fp, Fs, Fz, Fr, Fo, Out> Fold5<Ff, Fp, Fs, Fz, Fr, Fo, Out>
where
    Ff: Fn(&AstNode, Vec<Out>) -> Out,
    Fp: Fn(&AstNode, Vec<Out>) -> Out,
    Fs: Fn(&AstNode, Vec<Out>) -> Out,
    Fz: Fn(&AstNode, Vec<Out>) -> Out,
    Fr: Fn(&AstNode, Vec<Out>) -> Out,
    Fo: Fn(&AstNode, Vec<Out>) -> Out,
{
    /// Construct a Fold5 from the six reducers.
    pub fn new(
        on_focus: Ff,
        on_project: Fp,
        on_split: Fs,
        on_zoom: Fz,
        on_refract: Fr,
        on_other: Fo,
    ) -> Self {
        Fold5 {
            on_focus,
            on_project,
            on_split,
            on_zoom,
            on_refract,
            on_other,
            _out: core::marker::PhantomData,
        }
    }

    /// Post-order fold over an AST node. Children fold first; their
    /// `Vec<Out>` then feeds the parent's level reducer.
    pub fn run(&self, node: &AstNode) -> Out {
        let child_outs: Vec<Out> = node.children.iter().map(|c| self.run(c)).collect();
        match node.kind {
            AstKind::Focus => (self.on_focus)(node, child_outs),
            AstKind::Project => (self.on_project)(node, child_outs),
            AstKind::Split => (self.on_split)(node, child_outs),
            AstKind::Zoom => (self.on_zoom)(node, child_outs),
            AstKind::Refract => (self.on_refract)(node, child_outs),
            AstKind::In
            | AstKind::Out
            | AstKind::Dark
            | AstKind::IoBinding
            | AstKind::MatchExpr
            | AstKind::SelectExpr => (self.on_other)(node, child_outs),
        }
    }
}

/// `Fold1<F, Out>` — the uniform-reducer Fold5. A single function is
/// applied at every AST kind. This is the degenerate case the
/// `content_oid` Merkle hash uses: the dispatch on `AstKind` happens
/// *inside* the reducer (via a kind-tag string), so all six "slots" are
/// the same closure.
///
/// Mechanically: builds a `Fold5` whose six reducers are all the same
/// `F`. Rust's `Fold5` is generic over six distinct closure types, so
/// for uniform folds we need a single closure type — that's what
/// `Fold1` (a thin alias-like wrapper) gives us. The closure must be
/// `Clone` so it can be installed in all six slots without moving;
/// closures over captured references typically are.
pub fn fold1<F, Out>(reducer: F) -> Fold5<F, F, F, F, F, F, Out>
where
    F: Fn(&AstNode, Vec<Out>) -> Out + Clone,
{
    Fold5::new(
        reducer.clone(),
        reducer.clone(),
        reducer.clone(),
        reducer.clone(),
        reducer.clone(),
        reducer,
    )
}

// ---------------------------------------------------------------------------
// 2. compose_a — algebra composition
// ---------------------------------------------------------------------------

/// Algebra composition. Run two algebra elements sequentially on a
/// state, accumulating loss via the [`Metric`] monoid on the residual.
///
/// On the prism-core side this is what the `Prism` trait already does
/// internally via its associated-type chain (focus → project → refract);
/// the bootstrap names the call-site for *cross-prism* composition and
/// proves associativity / identity directly against the prism-core
/// substrate (see the property tests below). The composition is
/// equivalent to `q.act(p.act(state))` with `ScalarLoss::combine`
/// accumulating residuals — i.e. the [`Imperfect::eh`] bind on the
/// `Verdict<S>` monad.
pub fn compose_a<S, InP, InQ, P, Q>(p: &P, q: &Q, state: S) -> Verdict<S>
where
    P: Prism<Input = Seed<S>, Refracted = Optic<InP, S>>,
    Q: Prism<Input = Seed<S>, Refracted = Optic<InQ, S>>,
{
    apply_h(p, state).eh(|s| apply_h(q, s))
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
/// bootstrap's minimal floor.
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
    use prism_core::{Beam, IdentityPrism, Optic};
    use terni::{Loss, Metric};

    // -----------------------------------------------------------------------
    // Test fixtures — small concrete `Prism` impls over the scalar state
    // type `f64`. Each fixture performs its real work in `focus` and
    // passes through unchanged in `project` and `refract` — the bootstrap
    // doesn't distinguish phases at the algebra level; the three-phase
    // chain of `Prism` is the spectral-triple's *internal* composition,
    // and we only need a uniform-state algebra element here.
    // -----------------------------------------------------------------------

    /// Multiplies a scalar state by `factor`. Models a focus-like
    /// operator: pure success, no residual.
    struct Scale {
        factor: f64,
    }

    impl Prism for Scale {
        type Input = Seed<f64>;
        type Focused = Optic<f64, f64>;
        type Projected = Optic<f64, f64>;
        type Refracted = Optic<f64, f64>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let v = *beam.value().expect("Scale::focus on dark beam");
            beam.next(v * self.factor)
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("Scale::project on dark beam");
            beam.next(v)
        }
        fn refract(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("Scale::refract on dark beam");
            beam.next(v)
        }
    }

    /// Quantizes a scalar to the nearest integer. Models a project-like
    /// operator: produces a value but carries a residual = the rounding
    /// error. Mirrors the precision-cut semantics of project().
    struct Quantize;

    impl Prism for Quantize {
        type Input = Seed<f64>;
        type Focused = Optic<f64, f64>;
        type Projected = Optic<f64, f64>;
        type Refracted = Optic<f64, f64>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("Quantize::focus on dark beam");
            let rounded = state.round();
            let r = (state - rounded).abs();
            if r == 0.0 {
                beam.tick(Imperfect::success(rounded))
            } else {
                beam.tick(Imperfect::partial(rounded, ScalarLoss::new(r)))
            }
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("Quantize::project on dark beam");
            beam.next(v)
        }
        fn refract(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("Quantize::refract on dark beam");
            beam.next(v)
        }
    }

    /// Rejects negative inputs. Models an operator whose domain is the
    /// non-negative reals; transporting a negative state produces a
    /// Partial verdict carrying the *boundary residual* (= |state|).
    ///
    /// Domain rejection is modelled as a Partial-with-finite-loss rather
    /// than as a typed Failure: the bootstrap evaluator's algebra has
    /// `Error = Infallible` (matching `prism_core::IdentityPrism`'s
    /// signature so identity composition typechecks). The residual
    /// witnesses the rejection — large residuals mark domain boundaries,
    /// `ScalarLoss::total()` (= ∞) marks total rejection.
    struct Positive;

    impl Prism for Positive {
        type Input = Seed<f64>;
        type Focused = Optic<f64, f64>;
        type Projected = Optic<f64, f64>;
        type Refracted = Optic<f64, f64>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("Positive::focus on dark beam");
            if state >= 0.0 {
                beam.tick(Imperfect::success(state))
            } else {
                // Domain rejection: produce a Partial carrying the
                // boundary residual. The state value carried forward
                // is the projection onto the domain (clamp to 0.0).
                beam.tick(Imperfect::partial(0.0, ScalarLoss::new(-state)))
            }
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("Positive::project on dark beam");
            beam.next(v)
        }
        fn refract(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("Positive::refract on dark beam");
            beam.next(v)
        }
    }

    // -----------------------------------------------------------------------
    // compose_a — algebra-composition tests
    // -----------------------------------------------------------------------

    #[test]
    fn compose_a_associates() {
        // (a ∘ b) ∘ c agrees with a ∘ (b ∘ c) on every input.
        //
        // Grouped left:  apply (a then b), then c.
        // Grouped right: apply a, then (b then c).
        // `compose_a` is value-level eager composition; associativity
        // follows because `ScalarLoss::combine` is associative (the Loss
        // monoid law) and the underlying state operation is function
        // composition.
        let a = Scale { factor: 2.0 };
        let b = Scale { factor: 3.0 };
        let c = Scale { factor: 5.0 };
        for s in [-1.0_f64, 0.0, 0.5, 1.0, 7.25] {
            let lhs = compose_a(&a, &b, s).eh(|v| apply_h(&c, v));
            let rhs = apply_h(&a, s).eh(|v| compose_a(&b, &c, v));
            assert_eq!(lhs, rhs, "associativity at {}", s);
        }
    }

    #[test]
    fn compose_a_with_identity_is_identity() {
        // IdentityPrism is the unit element of A: id ∘ p and p ∘ id
        // behave like p. Witnesses that prism_core::IdentityPrism is
        // the A-monoid identity at the evaluator level.
        let id: IdentityPrism<f64> = IdentityPrism::new();
        let p = Scale { factor: 4.0 };
        for s in [-2.0_f64, 0.0, 1.5, 9.0] {
            let direct = apply_h(&p, s);
            let left = compose_a(&id, &p, s);
            let right = compose_a(&p, &id, s);
            assert_eq!(left, direct, "id ∘ p at {}", s);
            assert_eq!(right, direct, "p ∘ id at {}", s);
        }
    }

    #[test]
    fn compose_a_propagates_residuals() {
        // Composing two Partial-producing operators accumulates residuals
        // via ScalarLoss::combine — the Loss monoid law at the bootstrap.
        // Quantize(0.3) = 0 with residual 0.3; Quantize(0) = 0 with no
        // extra residual.
        let v = compose_a(&Quantize, &Quantize, 0.3);
        match v {
            Imperfect::Partial(value, r) => {
                assert_eq!(value, 0.0);
                assert!((r.as_f64() - 0.3).abs() < 1e-12);
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_propagates_domain_residual() {
        // Domain rejection in the first operator carries its residual
        // forward via `Imperfect::eh`'s propagate_loss law: Partial(0, r1)
        // followed by Success(s) becomes Partial(s, r1). With Scale's
        // pure-success action, the boundary residual from Positive
        // accumulates as the only loss in the final verdict.
        //
        // (In the previous mirror-type framing this was framed as
        // "Failure short-circuits"; under prism-core's algebra closed
        // with `Error = Infallible`, the same semantics emerge via the
        // Loss-monoid combine, with the chain remaining defined.)
        let v = compose_a(&Positive, &Scale { factor: 10.0 }, -3.0);
        match v {
            Imperfect::Partial(value, r) => {
                assert_eq!(value, 0.0, "clamped state at domain boundary");
                assert!(
                    (r.as_f64() - 3.0).abs() < 1e-12,
                    "boundary residual carried, got {}",
                    r.as_f64()
                );
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // apply_h — operator-action tests
    // -----------------------------------------------------------------------

    #[test]
    fn apply_h_consistent_with_prism_apply() {
        // apply_h is the named wrapper around prism_core::apply.
        // The equivalence test: both routes produce the same Verdict.
        let p = Scale { factor: 7.0 };
        for s in [-1.0_f64, 0.0, 2.5] {
            let via_apply_h = apply_h(&p, s);
            let via_prism = prism_apply(&p, seed(s)).into_focus();
            assert_eq!(via_apply_h, via_prism);
        }
    }

    #[test]
    fn apply_h_content_matches_compute_oid_inner() {
        // apply_h_content over an AST state agrees with the direct
        // `compute_oid_inner` call (the inner recursion). The evaluator
        // route through `ContentOidPrism` doesn't change the OID.
        let mut node = AstNode::new(AstKind::Focus, "root");
        node.add_child(AstNode::new(AstKind::In, "@prism"));
        let oid_direct = compute_oid_inner(&node);
        let v = apply_h_content(&node);
        match v {
            Imperfect::Success(oid) => assert_eq!(oid, oid_direct),
            other => panic!("expected Success, got {:?}", other),
        }
    }

    #[test]
    fn compute_content_oid_matches_apply_h_content() {
        // The thin-wrapper invariant: `compute_content_oid` IS
        // `apply_h_content` unwrapped to its Success payload.
        let mut node = AstNode::new(AstKind::Project, "project");
        node.set_body("hello");
        node.add_child(AstNode::new(AstKind::In, "@prism"));
        let direct = compute_content_oid(&node);
        match apply_h_content(&node) {
            Imperfect::Success(oid) => assert_eq!(oid, direct),
            other => panic!("expected Success, got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // Fold5 — catamorphism tests on tiny hand-built ASTs.
    // -----------------------------------------------------------------------

    /// Fold5 visits every node exactly once and feeds children to parents
    /// post-order. A counting fold returns the total node count.
    #[test]
    fn fold5_counts_nodes_post_order() {
        // Tree:  Focus(root) { In(@prism), Project(p) { In(@a) } }
        // 4 nodes total.
        let mut root = AstNode::new(AstKind::Focus, "root");
        root.add_child(AstNode::new(AstKind::In, "@prism"));
        let mut p = AstNode::new(AstKind::Project, "p");
        p.add_child(AstNode::new(AstKind::In, "@a"));
        root.add_child(p);

        let count_node = |_: &AstNode, child_counts: Vec<usize>| -> usize {
            1 + child_counts.into_iter().sum::<usize>()
        };
        let fold = fold1::<_, usize>(count_node);
        assert_eq!(fold.run(&root), 4);
    }

    /// Fold5 dispatches on `AstKind` — each level reducer fires only for
    /// its own kind. Witness: tag every node by which reducer saw it,
    /// then check the trail.
    #[test]
    fn fold5_dispatches_per_level() {
        let mut root = AstNode::new(AstKind::Focus, "root");
        root.add_child(AstNode::new(AstKind::Project, "p"));
        root.add_child(AstNode::new(AstKind::Split, "s"));
        root.add_child(AstNode::new(AstKind::Zoom, "z"));
        root.add_child(AstNode::new(AstKind::Refract, "r"));
        root.add_child(AstNode::new(AstKind::In, "@in"));

        // Each reducer returns a tag identifying which fired; the root's
        // child_outs is then a sorted concat of those tags, prefixed by
        // "F:".
        let fold = Fold5::new(
            |_n: &AstNode, mut c: Vec<String>| {
                c.sort();
                let mut s = String::from("F:");
                s.push_str(&c.join(","));
                s
            },
            |_n: &AstNode, _c: Vec<String>| String::from("P"),
            |_n: &AstNode, _c: Vec<String>| String::from("S"),
            |_n: &AstNode, _c: Vec<String>| String::from("Z"),
            |_n: &AstNode, _c: Vec<String>| String::from("R"),
            |n: &AstNode, _c: Vec<String>| format!("O({:?})", n.kind),
        );
        assert_eq!(fold.run(&root), "F:O(In),P,R,S,Z");
    }

    /// `fold1` (the uniform-reducer case) installs the same reducer at
    /// every kind. Witness: a uniform reducer that concatenates child
    /// strings sees both operation kinds and terminal kinds going
    /// through the same path.
    #[test]
    fn fold1_uniform_at_every_kind() {
        let mut root = AstNode::new(AstKind::Focus, "focus");
        root.add_child(AstNode::new(AstKind::In, "in"));
        root.add_child(AstNode::new(AstKind::Project, "project"));

        let fold = fold1::<_, String>(|n: &AstNode, c: Vec<String>| {
            let mut s = n.name.clone();
            for child in c {
                s.push('|');
                s.push_str(&child);
            }
            s
        });
        assert_eq!(fold.run(&root), "focus|in|project");
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
        // rotations or QR; that's deferred to a richer linalg substrate.
        // Power iteration is correct for the well-separated case and
        // that's what the bootstrap floor needs today.
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
    // ScalarLoss / Metric — laws inherited from prism-core, smoke-checked
    // at the evaluator boundary.
    // -----------------------------------------------------------------------

    #[test]
    fn metric_non_negative_on_well_formed_inputs() {
        // ScalarLoss::is_non_negative is the Metric-trait analog of the
        // bootstrap-local `is_finite` check on the previous Residual
        // type. The constructor itself panics on negatives; everything
        // it admits is non-negative.
        assert!(ScalarLoss::zero().is_non_negative());
        assert!(ScalarLoss::new(0.5).is_non_negative());
        assert!(ScalarLoss::new(1e9).is_non_negative());
    }

    #[test]
    fn metric_distance_is_symmetric() {
        // Metric::distance_to is symmetric — D(a, b) = D(b, a).
        let a = ScalarLoss::new(0.3);
        let b = ScalarLoss::new(0.8);
        assert_eq!(a.distance_to(&b).as_f64(), b.distance_to(&a).as_f64());
    }

    #[test]
    fn metric_combine_associates() {
        // Loss::combine is associative — the Loss monoid law.
        let a = ScalarLoss::new(0.1);
        let b = ScalarLoss::new(0.2);
        let c = ScalarLoss::new(0.3);
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));
        assert!((left.as_f64() - right.as_f64()).abs() < 1e-12);
    }
}
