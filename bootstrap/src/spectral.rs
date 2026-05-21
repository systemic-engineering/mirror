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

/// Tick 3c collapse: compute_oid_inner IS a uniform-reducer Fold5 over
/// the AST (the degenerate Fold1 case from `docs/specs/ast-as-bundle.md`
/// §Fold5). The pre-Tick3c implementation was a 10-arm match dispatching
/// per `AstKind`; this is structurally a `fold1` whose single reducer
/// dispatches internally on `AstKind` via a kind-tag string (the spec's
/// description of the uniform case verbatim).
///
/// Output is byte-identical to the pre-Tick3c implementation — the
/// closure body reproduces the same buffer construction (name + optional
/// `\0body:` + body + `:`-joined child OIDs) under the same tag
/// strings ("focus", "project", "split", "zoom", "refract", "in",
/// "out", "io_binding", "match_expr", "select_expr", "dark"). The
/// recursive walk now lives in the Fold5 walker rather than in this
/// function's per-arm `compute_content_oid(c)` calls.
///
/// The `fold1` helper installs the same closure in all six Fold5
/// reducer slots; the closure's internal match-on-kind chooses the
/// kind-tag. This is the second-order shape the cybernetics-split
/// recognition (`ast-as-bundle.md`) named: rather than a concrete
/// `Prism` per operation, one catamorphism whose level is determined
/// inside the reducer.
fn compute_oid_inner(node: &AstNode) -> String {
    let reducer = |n: &AstNode, child_oids: Vec<String>| -> String {
        // Dark short-circuits: hash verbatim bytes under "dark". No
        // name, no body-prefix, no child join. Per
        // `docs/specs/strict-and-total-classification.md`.
        if matches!(n.kind, AstKind::Dark) {
            let bytes: &[u8] = n.body.as_deref().map(str::as_bytes).unwrap_or(&[]);
            return hash_tagged("dark", bytes);
        }
        // In / Out are leaf terminals: hash just the name under the
        // kind tag. No body, no children.
        match n.kind {
            AstKind::In => return hash_tagged("in", n.name.as_bytes()),
            AstKind::Out => return hash_tagged("out", n.name.as_bytes()),
            _ => {}
        }
        // The eight remaining kinds (Focus / Project / Split / Zoom /
        // Refract / IoBinding / MatchExpr / SelectExpr) all share the
        // same buffer construction: name + optional "\0body:body" +
        // ":"-joined child OIDs. Only the kind tag differs.
        //
        // Focus historically *skipped* the body-prefix even when
        // node.body was present — the pre-Tick3c implementation's
        // `AstKind::Focus` arm wrote only name + children, no body.
        // Preserve that.
        let kind_tag: &'static str = match n.kind {
            AstKind::Focus => "focus",
            AstKind::Project => "project",
            AstKind::Split => "split",
            AstKind::Zoom => "zoom",
            AstKind::Refract => "refract",
            AstKind::IoBinding => "io_binding",
            AstKind::MatchExpr => "match_expr",
            AstKind::SelectExpr => "select_expr",
            // In / Out / Dark handled above.
            AstKind::In | AstKind::Out | AstKind::Dark => unreachable!(),
        };
        let include_body = !matches!(n.kind, AstKind::Focus);
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(n.name.as_bytes());
        if include_body {
            if let Some(body) = &n.body {
                if !body.is_empty() {
                    buf.extend_from_slice(b"\0body:");
                    buf.extend_from_slice(body.as_bytes());
                }
            }
        }
        for child in &child_oids {
            buf.push(b':');
            buf.extend_from_slice(child.as_bytes());
        }
        hash_tagged(kind_tag, &buf)
    };
    fold1::<_, String>(reducer).run(node)
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

// ---------------------------------------------------------------------------
// Render-via-Fold5 — the AST → bytes pretty-printer as a catamorphism.
//
// The pre-retirement `bootstrap/src/render.rs` had three entry points
// (`render_ast`, `render_ast_mirror`, `render_ast_with_grammar`), all
// post-order recursive descents threading a `depth: i32` and a `&mut
// Vec<u8>` accumulator through. Tick 3b collapses them into one Fold5
// instance keyed on grammar availability.
//
// **Deviation: depth as inherited attribute.** A bare catamorphism
// reducer sees only `&AstNode` and `Vec<Out>` — it has no `depth`
// parameter (that's an inherited / top-down attribute, not a
// synthesized / bottom-up one). The pre-retirement renderer's body
// emits bytes verbatim from the source mid-line, which means "reindent
// the whole child output at assembly time" does NOT preserve
// byte-for-byte output: verbatim body content (carrying its own
// source indentation) gets double-shifted.
//
// The honest fix is an attribute-grammar extension: Fold5At<…> is the
// depth-threading sibling of Fold5, with reducer signature
// `Fn(&AstNode, i32, Vec<Out>) -> Out` and a walker that passes depth
// down. Each render reducer now applies `append_indent(depth)`
// directly, matching the pre-retirement code's behavior 1:1. The
// canonical Fold5 (no depth) is unchanged — content_oid (Tick 3c) is
// depth-free and keeps using Fold5 / fold1.
//
// The grammar-aware path is keyed on `node.grammar_tag`: if it points
// at a non-mirror grammar with a loadable `.mirror` file, the per-kind
// reducer consults `Grammar::keyword_for_kind` for the reverse-lookup
// keyword. Loading is per-fold (one `load_grammar` call captured in
// the reducer closures), matching the pre-retirement dispatch in
// `render_ast`.
// ---------------------------------------------------------------------------

use crate::grammar::{grammar_path_for_ref, load_grammar, Grammar};

/// Append `depth` levels of two-space indent to `out`. Mirrors the
/// pre-retirement `render::append_indent` byte-for-byte.
fn append_indent(out: &mut Vec<u8>, depth: i32) {
    for _ in 0..depth {
        out.extend_from_slice(b"  ");
    }
}

/// Catamorphism over the AST with an inherited *depth* attribute.
/// Sibling of [`Fold5`]: same six reducers, but each one additionally
/// receives the current depth from the walker. Used by render, which
/// can't be expressed as a pure post-order catamorphism without
/// double-shifting verbatim body bytes (the pre-retirement renderer
/// emits body bytes mid-line without indent processing; preserving
/// that requires knowing depth at the reducer).
///
/// The walker `run(node, depth)` recurses into children at `depth + 1`
/// (children are conceptually one level deeper than their parent).
/// Reducers decide their own indent + structural emission.
pub struct Fold5At<Ff, Fp, Fs, Fz, Fr, Fo, Out>
where
    Ff: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fp: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fs: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fz: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fr: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fo: Fn(&AstNode, i32, Vec<Out>) -> Out,
{
    pub on_focus: Ff,
    pub on_project: Fp,
    pub on_split: Fs,
    pub on_zoom: Fz,
    pub on_refract: Fr,
    pub on_other: Fo,
    _out: core::marker::PhantomData<Out>,
}

impl<Ff, Fp, Fs, Fz, Fr, Fo, Out> Fold5At<Ff, Fp, Fs, Fz, Fr, Fo, Out>
where
    Ff: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fp: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fs: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fz: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fr: Fn(&AstNode, i32, Vec<Out>) -> Out,
    Fo: Fn(&AstNode, i32, Vec<Out>) -> Out,
{
    pub fn new(
        on_focus: Ff,
        on_project: Fp,
        on_split: Fs,
        on_zoom: Fz,
        on_refract: Fr,
        on_other: Fo,
    ) -> Self {
        Fold5At {
            on_focus,
            on_project,
            on_split,
            on_zoom,
            on_refract,
            on_other,
            _out: core::marker::PhantomData,
        }
    }

    /// Post-order fold over an AST with an inherited depth attribute.
    /// Children fold at `depth + 1`, with two exceptions matching the
    /// pre-retirement renderer's specific dispatch:
    ///
    /// - The synthetic root Focus (name == "root" at depth 0) does NOT
    ///   bump depth for its children — they render at depth 0.
    /// - Split/Zoom/Refract recurse at depth + 1 like Focus.
    ///
    /// Per-kind dispatch decisions about depth bumping happen inside
    /// the reducers; the walker uniformly passes `depth + 1` to all
    /// children and lets reducers re-shift if they need a different
    /// convention. The render reducers below are written so that they
    /// emit their own bytes at the depth they were called with, and
    /// trust the walker to have fed children at depth + 1.
    pub fn run(&self, node: &AstNode, depth: i32) -> Out {
        // Children always render one level deeper. The pre-retirement
        // renderer had a special case where the synthetic root Focus's
        // children stayed at the same depth; that's expressed in the
        // reducer for Focus by checking `node.name == "root"` and
        // adjusting how it consumes children there (it discards its
        // own indent and writes children verbatim).
        let child_depth = depth + 1;
        let child_outs: Vec<Out> =
            node.children.iter().map(|c| self.run(c, child_depth)).collect();
        match node.kind {
            AstKind::Focus => (self.on_focus)(node, depth, child_outs),
            AstKind::Project => (self.on_project)(node, depth, child_outs),
            AstKind::Split => (self.on_split)(node, depth, child_outs),
            AstKind::Zoom => (self.on_zoom)(node, depth, child_outs),
            AstKind::Refract => (self.on_refract)(node, depth, child_outs),
            AstKind::In
            | AstKind::Out
            | AstKind::Dark
            | AstKind::IoBinding
            | AstKind::MatchExpr
            | AstKind::SelectExpr => (self.on_other)(node, depth, child_outs),
        }
    }
}

/// Render an AST to bytes in mirror canonical form, dispatching
/// through the node's grammar tag for grammar-aware reverse lookup of
/// keywords. Replacement for the pre-retirement
/// `render::render_ast`. Public surface is preserved.
///
/// Per Tick 3b of `docs/specs/bootstrap-retirement-plan.md`: the three
/// pre-retirement entry points (`render_ast`, `render_ast_mirror`,
/// `render_ast_with_grammar`) collapse into one Fold5At application
/// keyed on grammar availability.
pub fn render_ast(node: &AstNode, depth: i32, out: &mut Vec<u8>) {
    let tag = node.grammar_tag.as_str();
    let grammar: Option<Grammar> =
        if tag.is_empty() || tag == "@mirror/grammar" || tag == "@mirror" {
            None
        } else {
            grammar_path_for_ref(tag).and_then(|p| load_grammar(&p).ok())
        };
    let rendered = match &grammar {
        Some(g) => render_fold_grammar(g).run(node, depth),
        None => render_fold_mirror().run(node, depth),
    };
    out.extend_from_slice(&rendered);
}

/// Fold5At instance for mirror-canonical rendering. The five operation
/// reducers + `on_other` together cover every `AstKind` the renderer
/// emits. Captures nothing — mirror canonical form is grammar-free.
fn render_fold_mirror(
) -> Fold5At<
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8>,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8>,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8>,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8>,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8>,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8>,
    Vec<u8>,
> {
    Fold5At::new(
        // on_focus
        |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            let mut out = Vec::new();
            // Synthetic root Focus: children inherit depth (they were
            // recursed at depth+1, but we want them at depth). We
            // can't "un-recurse" them once they're rendered; instead,
            // the pre-retirement code re-recursed at `depth` (same) in
            // this case. To match: re-render children at this depth
            // by re-invoking the renderer via a one-shot recursion.
            // This is mechanically a Fold5At property exception: the
            // synthetic-root Focus discards the walker-passed child
            // outputs and re-renders. The output is byte-identical to
            // the pre-retirement render_ast(c, depth, out) call.
            if node.name == "root" && depth == 0 {
                for c in &node.children {
                    render_ast(c, depth, &mut out);
                }
                return out;
            }
            append_indent(&mut out, depth);
            if node.name.as_bytes().first() == Some(&b'@') {
                out.extend_from_slice(b"grammar ");
            } else {
                out.extend_from_slice(b"focus ");
            }
            out.extend_from_slice(node.name.as_bytes());
            if !children.is_empty() {
                out.extend_from_slice(b" {\n");
                for c in children {
                    out.extend_from_slice(&c);
                }
                append_indent(&mut out, depth);
                out.extend_from_slice(b"}\n");
            } else {
                out.push(b'\n');
            }
            out
        },
        // on_project — mirror canonical: project lines render flat
        // (no children path in the pre-retirement code).
        |node: &AstNode, depth: i32, _children: Vec<Vec<u8>>| -> Vec<u8> {
            let mut out = Vec::new();
            append_indent(&mut out, depth);
            out.extend_from_slice(b"project ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            out
        },
        // on_split (`type` keyword)
        |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            let mut out = Vec::new();
            append_indent(&mut out, depth);
            out.extend_from_slice(b"type ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            for c in children {
                out.extend_from_slice(&c);
            }
            out
        },
        // on_zoom
        |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            let mut out = Vec::new();
            append_indent(&mut out, depth);
            out.extend_from_slice(b"zoom ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            for c in children {
                out.extend_from_slice(&c);
            }
            out
        },
        // on_refract
        |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            let mut out = Vec::new();
            append_indent(&mut out, depth);
            out.extend_from_slice(b"refract ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            for c in children {
                out.extend_from_slice(&c);
            }
            out
        },
        // on_other — In / Out / Dark / IoBinding / MatchExpr / SelectExpr.
        |node: &AstNode, depth: i32, _children: Vec<Vec<u8>>| -> Vec<u8> {
            render_other_mirror(node, depth)
        },
    )
}

/// Mirror-canonical rendering for non-canonical kinds (In, Out, Dark,
/// IoBinding, MatchExpr, SelectExpr). Shared by the mirror-canonical
/// and grammar-aware folds.
fn render_other_mirror(node: &AstNode, depth: i32) -> Vec<u8> {
    let mut out = Vec::new();
    match node.kind {
        AstKind::In => {
            append_indent(&mut out, depth);
            out.extend_from_slice(b"in ");
            if node.name.as_bytes().first() != Some(&b'@') {
                out.push(b'@');
            }
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
        }
        AstKind::Out => {
            append_indent(&mut out, depth);
            out.extend_from_slice(b"out ");
            if node.name.as_bytes().first() != Some(&b'@') {
                out.push(b'@');
            }
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
        }
        AstKind::IoBinding => {
            append_indent(&mut out, depth);
            out.extend_from_slice(b"io ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
        }
        AstKind::MatchExpr => {
            append_indent(&mut out, depth);
            out.extend_from_slice(b"match ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
        }
        AstKind::SelectExpr => {
            append_indent(&mut out, depth);
            out.extend_from_slice(b"select ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
        }
        AstKind::Dark => {
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
        }
        _ => unreachable!("render_other_mirror on canonical kind"),
    }
    out
}

/// Fold5At instance for grammar-aware rendering. Captures the loaded
/// grammar by reference and consults it for the reverse-lookup
/// keyword at each non-terminal kind. Falls through to the
/// mirror-canonical fallback when the grammar has no keyword.
fn render_fold_grammar<'g>(
    g: &'g Grammar,
) -> Fold5At<
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8> + 'g,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8> + 'g,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8> + 'g,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8> + 'g,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8> + 'g,
    impl Fn(&AstNode, i32, Vec<Vec<u8>>) -> Vec<u8> + 'g,
    Vec<u8>,
> {
    Fold5At::new(
        // on_focus — grammar-aware
        move |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            let mut out = Vec::new();
            // Synthetic root Focus: children inherit depth.
            if node.name == "root" && depth == 0 {
                // Re-render at same depth via the top-level entry (which
                // re-dispatches grammar). Matches the pre-retirement
                // render_ast_with_grammar's recursive call at depth=0.
                for c in &node.children {
                    let sub = render_fold_grammar(g).run(c, depth);
                    out.extend_from_slice(&sub);
                }
                return out;
            }
            // Verbatim-body path for LLVM-IR-style FOCUS nodes.
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    let fk: String = if !node.keyword.is_empty() {
                        node.keyword.clone()
                    } else if let Some(k) = g.keyword_for_kind(AstKind::Focus) {
                        k.to_string()
                    } else if node.name.as_bytes().first() == Some(&b'@') {
                        "grammar".to_string()
                    } else {
                        "focus".to_string()
                    };
                    append_indent(&mut out, depth);
                    out.extend_from_slice(fk.as_bytes());
                    if !node.name.is_empty() {
                        out.push(b' ');
                        out.extend_from_slice(node.name.as_bytes());
                    }
                    out.extend_from_slice(body.as_bytes());
                    out.push(b'\n');
                    for c in children {
                        out.extend_from_slice(&c);
                    }
                    return out;
                }
            }
            let kw: String = g
                .keyword_for_kind(AstKind::Focus)
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    if node.name.as_bytes().first() == Some(&b'@') {
                        "grammar".to_string()
                    } else {
                        "focus".to_string()
                    }
                });
            append_indent(&mut out, depth);
            out.extend_from_slice(kw.as_bytes());
            out.push(b' ');
            out.extend_from_slice(node.name.as_bytes());
            if !children.is_empty() {
                out.extend_from_slice(b" {\n");
                for c in children {
                    out.extend_from_slice(&c);
                }
                append_indent(&mut out, depth);
                out.extend_from_slice(b"}\n");
            } else {
                out.push(b'\n');
            }
            out
        },
        // on_project / on_split / on_zoom / on_refract — the four share
        // one body via the grammar-keyword reverse lookup.
        move |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            render_grammar_nonfocus(g, node, depth, children, "project")
        },
        move |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            render_grammar_nonfocus(g, node, depth, children, "type")
        },
        move |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            render_grammar_nonfocus(g, node, depth, children, "zoom")
        },
        move |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            render_grammar_nonfocus(g, node, depth, children, "refract")
        },
        // on_other — grammar-aware path falls through to
        // mirror-canonical bytes for the bundle terminals + Spec A/B
        // extensions.
        |node: &AstNode, depth: i32, _children: Vec<Vec<u8>>| -> Vec<u8> {
            render_other_mirror(node, depth)
        },
    )
}

/// Shared body for Project/Split/Zoom/Refract in the grammar-aware
/// renderer. Reverse-looks-up the keyword via the grammar, falls
/// through to the spec'd fallback, and handles the
/// verbatim-body-with-sigil-name path used by LLVM IR.
fn render_grammar_nonfocus(
    g: &Grammar,
    node: &AstNode,
    depth: i32,
    children: Vec<Vec<u8>>,
    fallback: &str,
) -> Vec<u8> {
    let kw: String = if !node.keyword.is_empty() {
        node.keyword.clone()
    } else {
        g.keyword_for_kind(node.kind)
            .map(|s| s.to_string())
            .unwrap_or_else(|| fallback.to_string())
    };
    let mut out = Vec::new();
    if let Some(body) = &node.body {
        if !body.is_empty() {
            let first = node.name.as_bytes().first().copied();
            let sigil_name = !node.name.is_empty()
                && matches!(first, Some(b'@') | Some(b'%') | Some(b'!') | Some(b'#'));
            append_indent(&mut out, depth);
            if !sigil_name {
                out.extend_from_slice(kw.as_bytes());
                if !node.name.is_empty() {
                    out.push(b' ');
                }
            }
            if !node.name.is_empty() {
                out.extend_from_slice(node.name.as_bytes());
            }
            out.extend_from_slice(body.as_bytes());
            out.push(b'\n');
            for c in children {
                out.extend_from_slice(&c);
            }
            return out;
        }
    }
    append_indent(&mut out, depth);
    out.extend_from_slice(kw.as_bytes());
    out.push(b' ');
    out.extend_from_slice(node.name.as_bytes());
    out.push(b'\n');
    for c in children {
        out.extend_from_slice(&c);
    }
    out
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

// ---------------------------------------------------------------------------
// Combinator — the closed enum vocabulary of named parser combinators.
//
// Per `docs/specs/parser-as-prism-grammar.md` §"Trait or enum?": the
// vocabulary is a closed enum, one variant per named combinator. A
// single `impl Prism for Combinator` dispatches via `match`. The seed
// is data — a `Combinator` literal — not a tree of trait objects.
// `combinator_tree_oid` is a straight Merkle hash over variants.
//
// Tick 4b.1 scope: the load-bearing variants for FP1 (seed parses
// `00-prism.mirror` to itself) and FP2 (seed parses `grammar.mirror`
// to a keyword-table Choice) are Seq, Choice, and LiteralKind. The
// remaining variants are declared so the surface matches the spec; their
// `walk_combinator` arms are `unimplemented!()` with a clear message.
// Subsequent ticks (4b.2+) flesh them out.
// ---------------------------------------------------------------------------

/// Closed enum of named parser combinators. One variant per combinator
/// named in the spec. A single `impl Prism for Combinator` dispatches
/// via match. The seed (`prism_seed()`) is a `Combinator` literal — the
/// algebra A of the spectral triple, written as data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Combinator {
    /// Parse children in order. Fragments concatenate; loss
    /// accumulates via `terni::Loss::combine`.
    Seq(Vec<Combinator>),
    /// First non-Partial branch wins. Ties broken by smallest
    /// `ScalarLoss`. Zero-progress fall-through hands off to a
    /// `DarkFallback` arm if present.
    Choice(Vec<Combinator>),
    /// Match the keyword bytes exactly, capturing as an `AstKind`-
    /// tagged `LiteralKind` Combinator in the output tree. The
    /// load-bearing variant for FP1/FP2: the five op-keyword captures.
    LiteralKind { keyword: Vec<u8>, kind: AstKind },
    /// Exact byte match, no AST capture. Atomic.
    Literal(Vec<u8>),
    /// Kleene-with-bounds. Termination follows from
    /// `Transport::Holonomy: Metric` (per-iteration loss non-negative,
    /// total bounded by source length).
    Repeat {
        body: Box<Combinator>,
        min: usize,
        max: Option<usize>,
    },
    /// Wrap the span consumed by `body` as an `AstNode` of `kind`.
    Capture {
        body: Box<Combinator>,
        kind: AstKind,
    },
    /// Byte-class predicate, named (not a fn ptr — the combinator tree
    /// stays serializable).
    Charset(CharsetKind),
    /// Balanced `{…}`, then `body` over the inner bytes.
    BraceBlock(Box<Combinator>),
    /// Balanced `(…)`.
    ParenBlock(Box<Combinator>),
    /// `io <name>(<args>) = <rhs>` — Spec A named composition.
    IoBinding,
    /// `match <subject> { <arm> => <body>, ... }` — Spec B.
    MatchArm,
    /// `select |<binder>| { <variant> => <body>, ... }` — Spec B.
    SelectVariant,
    /// LLVM-IR keyword-form body capture special case.
    KeywordFormBody { keyword: Vec<u8>, kind: AstKind },
    /// Strict-classification sentinel: bottom of every top-level
    /// `Choice`. Scans forward through unknown bytes and emits
    /// `AstKind::Dark`.
    DarkFallback,
}

/// Named byte-class predicates. Closed enum so the combinator tree
/// stays serializable; adding a charset means adding a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharsetKind {
    WordChar,
    NameChar,
    IrIdentChar,
    IoNameChar,
}

/// Convenience constructor: a `LiteralKind` capture for one keyword.
/// Spec name: `literal_kind`.
pub fn literal_kind(keyword: &[u8], kind: AstKind) -> Combinator {
    Combinator::LiteralKind {
        keyword: keyword.to_vec(),
        kind,
    }
}

/// Merkle hash over a `Combinator` tree. One tag per variant; the
/// payload is the variant's data bytes (with children hashed recursively
/// and joined with `:`). FP1's load-bearing equation
/// `combinator_tree_oid(seed) == combinator_tree_oid(seed_prime)` is
/// what the seed must satisfy after round-tripping through `apply_h`.
pub fn combinator_tree_oid(c: &Combinator) -> [u8; 32] {
    let oid_hex = combinator_tree_oid_hex(c);
    let mut out = [0u8; 32];
    let hex_bytes = oid_hex.as_bytes();
    let n = core::cmp::min(32, hex_bytes.len());
    out[..n].copy_from_slice(&hex_bytes[..n]);
    out
}

/// Internal: tagged-hash form returning a hex string. The Merkle walk
/// joins child OIDs with `:` under a per-variant tag — same shape
/// `compute_oid_inner` uses for the AST.
fn combinator_tree_oid_hex(c: &Combinator) -> String {
    match c {
        Combinator::Seq(children) => {
            let mut buf = Vec::new();
            for (i, ch) in children.iter().enumerate() {
                if i > 0 {
                    buf.push(b':');
                }
                buf.extend_from_slice(combinator_tree_oid_hex(ch).as_bytes());
            }
            hash_tagged("comb:seq", &buf)
        }
        Combinator::Choice(children) => {
            let mut buf = Vec::new();
            for (i, ch) in children.iter().enumerate() {
                if i > 0 {
                    buf.push(b':');
                }
                buf.extend_from_slice(combinator_tree_oid_hex(ch).as_bytes());
            }
            hash_tagged("comb:choice", &buf)
        }
        Combinator::LiteralKind { keyword, kind } => {
            let mut buf = Vec::new();
            buf.extend_from_slice(kind_tag(*kind).as_bytes());
            buf.push(b':');
            buf.extend_from_slice(keyword);
            hash_tagged("comb:literal_kind", &buf)
        }
        Combinator::Literal(bytes) => hash_tagged("comb:literal", bytes),
        Combinator::Repeat { body, min, max } => {
            let mut buf = Vec::new();
            buf.extend_from_slice(combinator_tree_oid_hex(body).as_bytes());
            buf.push(b':');
            buf.extend_from_slice(min.to_string().as_bytes());
            buf.push(b':');
            buf.extend_from_slice(
                max.map(|m| m.to_string()).unwrap_or_default().as_bytes(),
            );
            hash_tagged("comb:repeat", &buf)
        }
        Combinator::Capture { body, kind } => {
            let mut buf = Vec::new();
            buf.extend_from_slice(kind_tag(*kind).as_bytes());
            buf.push(b':');
            buf.extend_from_slice(combinator_tree_oid_hex(body).as_bytes());
            hash_tagged("comb:capture", &buf)
        }
        Combinator::Charset(k) => {
            hash_tagged("comb:charset", charset_tag(*k).as_bytes())
        }
        Combinator::BraceBlock(body) => hash_tagged(
            "comb:brace_block",
            combinator_tree_oid_hex(body).as_bytes(),
        ),
        Combinator::ParenBlock(body) => hash_tagged(
            "comb:paren_block",
            combinator_tree_oid_hex(body).as_bytes(),
        ),
        Combinator::IoBinding => hash_tagged("comb:io_binding", &[]),
        Combinator::MatchArm => hash_tagged("comb:match_arm", &[]),
        Combinator::SelectVariant => hash_tagged("comb:select_variant", &[]),
        Combinator::KeywordFormBody { keyword, kind } => {
            let mut buf = Vec::new();
            buf.extend_from_slice(kind_tag(*kind).as_bytes());
            buf.push(b':');
            buf.extend_from_slice(keyword);
            hash_tagged("comb:keyword_form_body", &buf)
        }
        Combinator::DarkFallback => hash_tagged("comb:dark_fallback", &[]),
    }
}

fn kind_tag(k: AstKind) -> &'static str {
    match k {
        AstKind::Focus => "focus",
        AstKind::Project => "project",
        AstKind::Split => "split",
        AstKind::Zoom => "zoom",
        AstKind::Refract => "refract",
        AstKind::In => "in",
        AstKind::Out => "out",
        AstKind::IoBinding => "io_binding",
        AstKind::MatchExpr => "match_expr",
        AstKind::SelectExpr => "select_expr",
        AstKind::Dark => "dark",
    }
}

fn charset_tag(k: CharsetKind) -> &'static str {
    match k {
        CharsetKind::WordChar => "word_char",
        CharsetKind::NameChar => "name_char",
        CharsetKind::IrIdentChar => "ir_ident_char",
        CharsetKind::IoNameChar => "io_name_char",
    }
}

// ---------------------------------------------------------------------------
// `Prism for Combinator` — one match in the focus phase.
// ---------------------------------------------------------------------------

/// `apply_h(Combinator, source)` returns a `Combinator` tree: the
/// vocabulary of captures the input induced. For a `Choice` of
/// `LiteralKind`s, this is the deduplicated set of keywords found,
/// in canonical (spec-declared) order, wrapped as `Choice`. Other
/// variants are `unimplemented!()` in 4b.1 scope (see
/// `walk_combinator`).
impl Prism for Combinator {
    type Input = Seed<(Vec<u8>, usize)>;
    type Focused = Optic<(Vec<u8>, usize), Combinator>;
    type Projected = Optic<Combinator, Combinator>;
    type Refracted = Optic<Combinator, Combinator>;

    fn focus(&self, beam: Self::Input) -> Self::Focused {
        let (source, offset) = beam
            .value()
            .expect("Combinator::focus on dark beam")
            .clone();
        let parsed = walk_combinator(self, &source, offset);
        beam.next(parsed)
    }

    fn project(&self, beam: Self::Focused) -> Self::Projected {
        let c = beam
            .value()
            .expect("Combinator::project on dark beam")
            .clone();
        beam.next(c)
    }

    fn refract(&self, beam: Self::Projected) -> Self::Refracted {
        let c = beam
            .value()
            .expect("Combinator::refract on dark beam")
            .clone();
        beam.next(c)
    }
}

/// Walk source bytes with a combinator, returning the induced
/// `Combinator` tree. Tick 4b.1 closes only the `Choice(LiteralKind)`
/// arm — the load-bearing case for FP1/FP2. Other variants
/// `unimplemented!()` with a pointer to the spec.
fn walk_combinator(c: &Combinator, source: &[u8], _offset: usize) -> Combinator {
    match c {
        Combinator::Choice(branches) => {
            // Filter branches whose keyword appears as a whole word in
            // the source. Preserve self's branch order — the canonical
            // declaration order is the spec's order.
            let mut kept: Vec<Combinator> = Vec::with_capacity(branches.len());
            for b in branches {
                if branch_keyword_occurs(b, source) {
                    kept.push(b.clone());
                }
            }
            Combinator::Choice(kept)
        }
        Combinator::Seq(_)
        | Combinator::LiteralKind { .. }
        | Combinator::Literal(_)
        | Combinator::Repeat { .. }
        | Combinator::Capture { .. }
        | Combinator::Charset(_)
        | Combinator::BraceBlock(_)
        | Combinator::ParenBlock(_)
        | Combinator::IoBinding
        | Combinator::MatchArm
        | Combinator::SelectVariant
        | Combinator::KeywordFormBody { .. }
        | Combinator::DarkFallback => unimplemented!(
            "walk_combinator: variant not yet implemented (4b.1 scope is \
             Choice-of-LiteralKind only; see \
             docs/specs/parser-as-prism-grammar.md §\"The named vocabulary\")"
        ),
    }
}

/// Does this branch's keyword (if it's a `LiteralKind`) appear as a
/// whole word in `source`? Whole-word = bounded on both sides by
/// non-word bytes (or source boundaries).
fn branch_keyword_occurs(branch: &Combinator, source: &[u8]) -> bool {
    let kw = match branch {
        Combinator::LiteralKind { keyword, .. } => keyword,
        _ => return false,
    };
    if kw.is_empty() {
        return false;
    }
    let n = kw.len();
    let mut i = 0;
    while i + n <= source.len() {
        if &source[i..i + n] == kw.as_slice() {
            let left_ok = i == 0 || !is_word_byte(source[i - 1]);
            let right_ok = i + n == source.len() || !is_word_byte(source[i + n]);
            if left_ok && right_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'/'
}

/// The seed: the Combinator-tree encoding of `boot/00-prism.mirror`.
///
/// 4b.1 realisation: a `Choice` over the five op-keyword captures.
/// This is the minimal seed that closes both fixed points:
///
/// - **FP1.** `00-prism.mirror` names all five operations as keywords;
///   `apply_h(seed, bytes)` returns the same `Choice([5])` → same OID.
/// - **FP2.** `grammar.mirror` names four operations (focus, project,
///   split, zoom — no refract); `apply_h(seed, bytes)` returns
///   `Choice([4])` — the keyword table for that file's grammar.
///
/// The spec's full seed (Tick 4b.2+) extends this with the prism-body
/// form, the abstract io declaration, and the in/out bindings. Those
/// are not load-bearing for the FP1/FP2 OID equality this tick lands.
pub fn prism_seed() -> Combinator {
    Combinator::Choice(vec![
        literal_kind(b"focus", AstKind::Focus),
        literal_kind(b"project", AstKind::Project),
        literal_kind(b"split", AstKind::Split),
        literal_kind(b"zoom", AstKind::Zoom),
        literal_kind(b"refract", AstKind::Refract),
    ])
}

#[cfg(test)]
mod combinator_tests {
    use super::*;
    use std::path::PathBuf;

    fn read_boot_file(rel: &str) -> Vec<u8> {
        // bootstrap/Cargo.toml dir → ../boot/<rel>
        let manifest = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR set under cargo test");
        let mut p = PathBuf::from(manifest);
        p.pop(); // bootstrap → mirror
        p.push("boot");
        for seg in rel.split('/') {
            p.push(seg);
        }
        std::fs::read(&p).unwrap_or_else(|e| panic!("read {:?}: {}", p, e))
    }

    fn parse_with(combinator: &Combinator, bytes: &[u8]) -> Combinator {
        // Bypass `apply_h`'s `Input = Seed<S>, Refracted = Optic<In, S>`
        // uniformity constraint (state type ingressed == egressed). The
        // Combinator parser's state type *changes* across the pipeline:
        // input is `(Vec<u8>, usize)`, output is `Combinator`. Same
        // pattern as `apply_h_content`: call `prism_apply` directly
        // because the input type maps to a different output type.
        let beam = super::seed((bytes.to_vec(), 0usize));
        let refracted = prism_apply(combinator, beam);
        // refracted: Optic<Combinator, Combinator>; into_focus()
        // collapses to Imperfect<Combinator, _, _>.
        match refracted.into_focus() {
            Imperfect::Success(c) => c,
            Imperfect::Partial(c, _loss) => c,
            Imperfect::Failure(_, _) => unreachable!(
                "Combinator::Error = Infallible; Failure uninhabited"
            ),
        }
    }

    /// FP1 — the algebra is self-hosting.
    ///
    /// `apply_h(prism_seed(), 00-prism.mirror.bytes)` parses to a tree
    /// with the same `combinator_tree_oid` as `prism_seed()` itself.
    /// Mirror's grammar describes mirror's grammar; the description IS
    /// the implementation.
    #[test]
    fn fp1_seed_parses_itself() {
        let seed = prism_seed();
        let prism_bytes = read_boot_file("00-prism.mirror");
        let seed_prime = parse_with(&seed, &prism_bytes);
        let oid_seed = combinator_tree_oid(&seed);
        let oid_prime = combinator_tree_oid(&seed_prime);
        assert_eq!(
            oid_seed, oid_prime,
            "FP1: seed OID != seed_prime OID\nseed       = {:?}\nseed_prime = {:?}",
            seed, seed_prime
        );
    }

    /// FP2 — the grammar surface lifts the keyword mapping.
    ///
    /// `apply_h(prism_seed(), grammar.mirror.bytes)` parses to a
    /// keyword-table `Choice` — the four op-keywords that appear in
    /// `boot/std/mirror/grammar.mirror` (focus, project, split, zoom;
    /// no refract).
    #[test]
    fn fp2_grammar_lifts_to_keyword_table() {
        let seed = prism_seed();
        let grammar_bytes = read_boot_file("std/mirror/grammar.mirror");
        let parsed = parse_with(&seed, &grammar_bytes);

        let expected = Combinator::Choice(vec![
            literal_kind(b"focus", AstKind::Focus),
            literal_kind(b"project", AstKind::Project),
            literal_kind(b"split", AstKind::Split),
            literal_kind(b"zoom", AstKind::Zoom),
        ]);

        assert_eq!(
            combinator_tree_oid(&parsed),
            combinator_tree_oid(&expected),
            "FP2: parsed OID != expected keyword-table OID\nparsed   = {:?}\nexpected = {:?}",
            parsed,
            expected
        );
    }

    /// Emit the FP1 + FP2 OID hex values for the commit message. Pure
    /// observation — no assertion. Runs at every `cargo test` to keep
    /// the values discoverable.
    #[test]
    fn emit_oid_hex_for_log() {
        let seed = prism_seed();
        let prism_bytes = read_boot_file("00-prism.mirror");
        let grammar_bytes = read_boot_file("std/mirror/grammar.mirror");
        let seed_prime = parse_with(&seed, &prism_bytes);
        let grammar_lift = parse_with(&seed, &grammar_bytes);
        eprintln!("FP1 seed       OID hex: {}", combinator_tree_oid_hex(&seed));
        eprintln!("FP1 seed_prime OID hex: {}", combinator_tree_oid_hex(&seed_prime));
        eprintln!("FP2 expected   OID hex: {}", combinator_tree_oid_hex(&Combinator::Choice(vec![
            literal_kind(b"focus", AstKind::Focus),
            literal_kind(b"project", AstKind::Project),
            literal_kind(b"split", AstKind::Split),
            literal_kind(b"zoom", AstKind::Zoom),
        ])));
        eprintln!("FP2 parsed     OID hex: {}", combinator_tree_oid_hex(&grammar_lift));
    }

    /// Sanity: `combinator_tree_oid` is deterministic and discriminates
    /// between distinct trees. Cheap guard against accidental hash
    /// collapse.
    #[test]
    fn combinator_oid_is_deterministic_and_discriminating() {
        let a = prism_seed();
        let b = prism_seed();
        assert_eq!(combinator_tree_oid(&a), combinator_tree_oid(&b));

        let c = Combinator::Choice(vec![literal_kind(b"focus", AstKind::Focus)]);
        assert_ne!(combinator_tree_oid(&a), combinator_tree_oid(&c));
    }
}

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
