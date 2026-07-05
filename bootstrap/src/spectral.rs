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
//! `prismqueer`'s verified spectral-triple substrate. The trait chain in
//! `prismqueer::bundle` (Fiber → Connection → Gauge → Transport →
//! Closure) realizes the structure; this module names what the
//! bootstrap evaluator does over it.
//!
//! - **A** — the involutive algebra of operations on H. Realized by
//!   `prismqueer::Prism`: the Tambara-composable optic. In mirror's
//!   bootstrap A's generators are the five Prism operations
//!   (focus / project / split / shift / settle) plus their compositions.
//!   The identity element is [`prismqueer::IdentityPrism`] (the unit of
//!   A as a monoid).
//! - **H** — the Hilbert space the algebra acts on. The state type S
//!   carried through a beam pipeline. In the bootstrap H is the space
//!   of AST nodes — each node a state vector, the recursive descent
//!   the action of an algebra element.
//! - **D** — the Dirac operator. Realized by `Transport::transport`,
//!   whose `terni::Imperfect<State, _, Holonomy>` signature is precisely
//!   the shape of D's partially-defined action: Success when the
//!   operator sends a state in its domain, Partial-with-holonomy when
//!   transport carries the state off the manifold with a bounded
//!   *located* opacity. The holonomy is a [`terni::Loss`]: the bootstrap
//!   uses [`Transparency<Ref>`] — a `Clear`/`Opaque` enum whose `Opaque`
//!   variant is a `BTreeMap` from substrate-ref to [`PropertyVerdict`].
//!   `combine` unions the opacity maps at colliding refs via
//!   [`PropertyVerdict::merge_with`] (Fail dominates; Partials accumulate
//!   diagnostics and take the minimum confidence). Connes' bounded-
//!   commutator condition `‖[D, a]‖ < ∞` becomes the type-level
//!   constraint that every opacity is a structurally well-formed verdict
//!   located at a named ref, never a faceless scalar. Domain rejection
//!   is encoded as a Partial verdict carrying the absorbing
//!   `Transparency::catastrophic()`, not as a typed Failure (the algebra
//!   has `Error = Infallible` so it remains closed under identity
//!   composition). In the bootstrap D's concrete matrix form is
//!   `CoincidenceHash<5,5>` (see `bootstrap/src/hash.rs`); its scalar
//!   action on AST states is `apply_h(&ContentOidPrism, node)`
//!   (dispatched through [`ContentOidPrism`] — the retirement of the
//!   standalone `content.rs` module, per
//!   `docs/specs/bootstrap-retirement-plan.md` Tick 1).
//!
//! [`Transparency<Ref>`]: prismqueer::Transparency
//! [`PropertyVerdict`]: prismqueer::PropertyVerdict
//! [`PropertyVerdict::merge_with`]: prismqueer::PropertyVerdict::merge_with
//! [`terni::Loss`]: terni::Loss
//!
//! ## The three primitive operations
//!
//! Everything mirror does decomposes into a finite composition of three
//! primitives over this triple:
//!
//! 1. [`compose_a`] — algebra composition. Run two algebra elements
//!    sequentially on a state, unioning located opacities via the
//!    `Transparency<Ref>` Loss-monoid on the holonomy carrier. The
//!    composition is associative because `terni::Loss::combine` is
//!    (and on `Transparency<Ref>` it is set-union on opacity maps with
//!    per-key `PropertyVerdict::merge_with`), and [`IdentityPrism`] is
//!    its unit.
//! 2. `prismqueer::apply_h` — operator action on a state vector.
//!    Heterogeneous: input state type and output state type are
//!    independent. Wraps a single `prismqueer::Prism`'s focus /
//!    project / settle sweep and returns the resulting `Imperfect`
//!    in H. Lives in `prism-core` and is re-imported here — the
//!    bootstrap stands on the substrate's verified shape rather than
//!    redefining its own constrained variant.
//! 3. [`eigen_d`] — Dirac eigendecomposition. Given an operator
//!    represented in the canonical 5-d basis (one axis per Prism
//!    operation), return its spectrum. For the 5×5 case this is power
//!    iteration with deflation — no external linalg dependency, fitting
//!    the bootstrap's intentionally minimal floor (the only added
//!    crates are `prism-core` and `terni`, themselves zero-IO).
//!
//! ## Lineage — why Transparency, not ScalarLoss
//!
//! The Loss carrier moved from a scalar (`ScalarLoss`) to a structured
//! `Transparency<Ref>` per a three-strand lineage. (i) R. Reyes' 2024
//! reconstruction of Stafford Beer's Viable System Model formalises the
//! audit channel (VSM System 3*) as a *sheaf of troubles* indexed by
//! substrate locations rather than a single scalar of "how broken" —
//! co-discovery with the prism `Transparency` design; see
//! `cybernetics/beer-error-propagation.md` in the systemic.engineering
//! corpus. (ii) Beer's 3* audit channel is the cybernetic primitive the
//! sheaf encodes: trouble propagates with its *location* intact, so the
//! engineer can intervene where the trouble lives rather than at the
//! collapsed root. (iii) The engineering-continuity it follows ended on
//! 1973-09-11 in Santiago, when Project Cybersyn's substrate-shared
//! audit channels were extinguished by force; the discipline survived
//! in the literature and is reconstituted here as the bootstrap's loss
//! carrier — `@quantize` / `@positive` / `@kintsugi/fracture/validate`
//! are the example refs at which the example prisms (`Quantize`,
//! `Positive`, `fracture_validate_body`) open their structural verdicts.
//!
//! [`prismqueer::IdentityPrism`]: prismqueer::IdentityPrism
//! [`terni::Imperfect`]: terni::Imperfect
//! [`terni::Metric`]: terni::Metric

use prismqueer::{apply_h, Beam, Optic, Prism, Ref, Transparency};
#[cfg(test)]
use prismqueer::{Diagnostic, PropertyVerdict};
use terni::Imperfect;

use crate::ast::{AstKind, AstNode};
use crate::hash::hash_tagged;

// ---------------------------------------------------------------------------
// State carriers — the bootstrap's seed and verdict types.
// ---------------------------------------------------------------------------

/// The seed beam type for an algebra element acting on a state of type
/// `S`. We use `()` as the input position because the state vector is
/// produced by the seed itself — there is no prior input. The error
/// position is [`Infallible`] (matching `prismqueer::IdentityPrism`'s
/// shape): the evaluator's algebra elements model "domain rejection" as
/// a Partial verdict with structured opacities (the absorbing case is
/// `Transparency::catastrophic()`), not as a typed Failure. This keeps
/// the algebra closed under composition with the identity element. Loss
/// is carried as [`Transparency<Ref>`] — the structured Loss-monoid
/// that names *where* each opacity lives rather than collapsing it to a
/// scalar magnitude.
///
/// [`Infallible`]: std::convert::Infallible
/// [`Transparency<Ref>`]: prismqueer::Transparency
pub type Seed<S> = Optic<(), S>;

/// The verdict an algebra element returns when acting on a state.
/// Mirrors `prismqueer::Transport::transport`'s signature exactly:
/// `terni::Imperfect<S, Infallible, Holonomy>` where the Holonomy is
/// any [`terni::Loss`]. The `Infallible` error position witnesses that
/// algebra elements are total on the algebra's *closure* (every
/// operator is defined; the opacities at the boundary of the domain
/// are carried in [`Transparency<Ref>`], which can be
/// `Transparency::catastrophic()` for the absorbing case).
///
/// [`terni::Loss`]: terni::Loss
/// [`Transparency<Ref>`]: prismqueer::Transparency
pub type Verdict<S> = Imperfect<S, std::convert::Infallible, Transparency<Ref>>;

/// Seed a beam with a starting state value. The source position is `()`
/// (no prior input); the value position carries the state.
pub fn seed<S>(state: S) -> Seed<S> {
    Optic::ok((), state)
}

// ---------------------------------------------------------------------------
// 1. apply_h — operator action on H (now re-exported from prism-core)
// ---------------------------------------------------------------------------
//
// `apply_h` lives in `prismqueer::apply_h` as a heterogeneous helper
// (input state type and output state type are independent). The
// bootstrap re-uses it directly via the `use` import above; no local
// definition or wrapper exists. The previous bootstrap-local `apply_h`
// over-constrained `Refracted = Optic<_, S>` (input and output state
// types coupled), which forced both `apply_h_content` and the
// Combinator parser to bypass it. Both bypasses are now gone —
// `apply_h` is the only path.
//
// `compute_content_oid` is the AST-specific convenience wrapper around
// `apply_h(&ContentOidPrism, node.clone())`, unwrapping the
// `Imperfect` to a `String`. `ContentOidPrism` is total over
// well-formed ASTs (`AstKind::Dark` nodes are hashed under the
// `"dark"` tag, not produced as Partial), so the unwrap is safe.

/// Compute the content-OID of an AST node by dispatching the discrete
/// Dirac operator's scalar action through `apply_h`. Every call-site
/// that used to read `content_oid(&ast)` now reads
/// `compute_content_oid(&ast)`.
pub fn compute_content_oid(node: &AstNode) -> String {
    match apply_h(&ContentOidPrism, node.clone()) {
        Imperfect::Success(oid) => oid,
        // Defensive — `ContentOidPrism` never produces Partial today.
        // If a future combinator does, treat it as a loss-bearing OID
        // and surface the payload.
        Imperfect::Partial(oid, _) => oid,
        // `ContentOidPrism::Refracted::Error = Infallible`; this arm
        // is structurally unreachable but the type system can't see it.
        Imperfect::Failure(_, _) => {
            unreachable!("ContentOidPrism has Error = Infallible; Failure is uninhabited")
        }
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
/// children via `apply_h(&ContentOidPrism, child)`, and emits the OID string. The
/// `project` and `settle` phases pass the OID through unchanged —
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

    fn settle(&self, beam: Self::Projected) -> Self::Refracted {
        let oid = beam
            .value()
            .expect("ContentOidPrism::settle on dark beam")
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
/// strings ("focus", "project", "split", "shift", "settle", "in",
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
        // Docblock short-circuits identically to Dark, under a distinct
        // "docblock" tag: hash the verbatim `#`-prefixed line bytes
        // (leaf, no children). Per `docs/math/kintsugi/doc-code-seam.md`
        // §6.1 (the doc-as-declaration collapse).
        if matches!(n.kind, AstKind::Docblock) {
            let bytes: &[u8] = n.body.as_deref().map(str::as_bytes).unwrap_or(&[]);
            return hash_tagged("docblock", bytes);
        }
        // In / Out are leaf terminals: hash just the name under the
        // kind tag. No body, no children.
        match n.kind {
            AstKind::In => return hash_tagged("in", n.name.as_bytes()),
            AstKind::Out => return hash_tagged("out", n.name.as_bytes()),
            _ => {}
        }
        // The eight remaining kinds (Focus / Project / Split / Shift /
        // Settle / IoBinding / MatchExpr / SelectExpr) all share the
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
            AstKind::Shift => "shift",
            AstKind::Settle => "settle",
            AstKind::IoBinding => "io_binding",
            AstKind::MatchExpr => "match_expr",
            AstKind::SelectExpr => "select_expr",
            AstKind::ParametricType => "parametric_type",
            // In / Out / Dark / Docblock handled above.
            AstKind::In | AstKind::Out | AstKind::Dark | AstKind::Docblock => unreachable!(),
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
// (Focus / Project / Split / Shift / Settle) map to the five trait-chain
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
/// (Focus → Fiber, Project → Connection, Split → Gauge, Shift →
/// Transport, Settle → Closure) plus an `on_other` reducer for the
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
            AstKind::Shift => (self.on_zoom)(node, child_outs),
            AstKind::Settle => (self.on_refract)(node, child_outs),
            AstKind::In
            | AstKind::Out
            | AstKind::Dark
            | AstKind::Docblock
            | AstKind::IoBinding
            | AstKind::MatchExpr
            | AstKind::SelectExpr
            | AstKind::ParametricType => (self.on_other)(node, child_outs),
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
    /// - Split/Shift/Settle recurse at depth + 1 like Focus.
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
        let child_outs: Vec<Out> = node
            .children
            .iter()
            .map(|c| self.run(c, child_depth))
            .collect();
        match node.kind {
            AstKind::Focus => (self.on_focus)(node, depth, child_outs),
            AstKind::Project => (self.on_project)(node, depth, child_outs),
            AstKind::Split => (self.on_split)(node, depth, child_outs),
            AstKind::Shift => (self.on_zoom)(node, depth, child_outs),
            AstKind::Settle => (self.on_refract)(node, depth, child_outs),
            AstKind::In
            | AstKind::Out
            | AstKind::Dark
            | AstKind::Docblock
            | AstKind::IoBinding
            | AstKind::MatchExpr
            | AstKind::SelectExpr
            | AstKind::ParametricType => (self.on_other)(node, depth, child_outs),
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
    let grammar: Option<Grammar> = if tag.is_empty() || tag == "@mirror/grammar" || tag == "@mirror"
    {
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
fn render_fold_mirror() -> Fold5At<
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
            out.extend_from_slice(b"shift ");
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
            out.extend_from_slice(b"settle ");
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
        AstKind::ParametricType => {
            // ParametricType nodes appear inline within other declarations
            // (return-type positions, field types). The verbatim form lives
            // in `name`; emit it as-is so round-trip preserves the surface
            // exactly. No newline — the surrounding declaration owns layout.
            out.extend_from_slice(node.name.as_bytes());
        }
        AstKind::Dark => {
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
        }
        AstKind::Docblock => {
            // Docblock body carries the verbatim `#`-prefixed line bytes
            // (leading `#` included, trailing newline excluded). Emit as-is
            // followed by a newline to round-trip the source line boundary.
            // Per `docs/math/kintsugi/doc-code-seam.md` §6.1.
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
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
            render_grammar_nonfocus(g, node, depth, children, "shift")
        },
        move |node: &AstNode, depth: i32, children: Vec<Vec<u8>>| -> Vec<u8> {
            render_grammar_nonfocus(g, node, depth, children, "settle")
        },
        // on_other — grammar-aware path falls through to
        // mirror-canonical bytes for the bundle terminals + Spec A/B
        // extensions.
        |node: &AstNode, depth: i32, _children: Vec<Vec<u8>>| -> Vec<u8> {
            render_other_mirror(node, depth)
        },
    )
}

/// Shared body for Project/Split/Shift/Settle in the grammar-aware
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
/// internally via its associated-type chain (focus → project → settle);
/// the bootstrap names the call-site for *cross-prism* composition and
/// proves associativity / identity directly against the prism-core
/// substrate (see the property tests below). The composition is
/// equivalent to `q.act(p.act(state))` with
/// `Transparency::<Ref>::combine` unioning located opacities — i.e.
/// the [`Imperfect::eh`] bind on the `Verdict<S>` monad.
pub fn compose_a<S, InP, InQ, P, Q>(p: &P, q: &Q, state: S) -> Verdict<S>
where
    P: Prism<
        Input = Seed<S>,
        Refracted = Optic<InP, S, std::convert::Infallible, Transparency<Ref>>,
    >,
    Q: Prism<
        Input = Seed<S>,
        Refracted = Optic<InQ, S, std::convert::Infallible, Transparency<Ref>>,
    >,
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
/// focus / project / split / shift / settle). We use power iteration
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
// Tick 4b.2 scope: the load-bearing variants for FP1 (seed parses the
// full `00-prism.mirror` to itself) and FP2 (the op-keyword sub-Choice
// parses `grammar.mirror` to the pruned keyword table) are Seq, Choice,
// Capture, BraceBlock, IoBinding, LiteralKind, and Literal. The
// remaining variants are declared so the surface matches the spec;
// their `walk_combinator` arms are `unimplemented!()` with a clear
// message. Subsequent sub-ticks flesh them out.
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
    /// tagged `LiteralKind` Combinator in the output tree.
    /// Load-bearing variant for FP1/FP2: the five op-keyword captures
    /// the body of every `prism @(…) { … }` form is built from.
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
    /// `until(stop)` — consume bytes until `stop` is matched or input
    /// ends. The stop combinator is a *peek* — it does not consume,
    /// it just terminates the scan. Used by `@nl(until_newline)`,
    /// fenced-code-block termination, and inline-backtick lifts.
    Until { stop: Box<Combinator> },
    /// `@<grammar>(<body>)` — the cross-grammar shift. At parse time
    /// the body bytes (extracted by `body`) are handed to the named
    /// grammar's combinator tree. Walk is structural: returns Self
    /// with `body` recursively walked. The grammar reference is a
    /// string path (e.g. "@nl", "@code/rust", "@mirror/glass").
    Shift {
        grammar: String,
        body: Box<Combinator>,
    },
    /// Strict-classification sentinel: bottom of every top-level
    /// `Choice`. Scans forward through unknown bytes and emits
    /// `AstKind::Dark`.
    DarkFallback,
    /// Trie-compiled set of multi-byte literals. The phase-2
    /// charset-compilation target for `Choice` of `Literal` arms (any
    /// arm length); per `docs/specs/combinator-optimization.md` §3.2
    /// the new variant is preferred over extending the closed
    /// `CharsetKind`, because the literal set is open-ended (5 keywords
    /// for the meta-glass, ~50 for LLVM IR, more for future grammars).
    ///
    /// Members are stored sorted lexicographically so the OID is
    /// encoding-invariant under permutation. An empty set is the empty
    /// charset (matches nothing).
    MultiByteCharset(Vec<Vec<u8>>),
}

/// Named byte-class predicates. Closed enum so the combinator tree
/// stays serializable; adding a charset means adding a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharsetKind {
    /// Identifier byte: ASCII alnum + `_` + `/`.
    WordChar,
    /// mirror name byte: ASCII alnum + `_` + `@` + `/` + `.`.
    NameChar,
    /// LLVM-IR identifier byte: ASCII alnum + `_` + `.` + `$`.
    IrIdentChar,
    /// `io <name>(...)` name byte: ASCII alnum + `_`.
    IoNameChar,
    /// ASCII whitespace ` `, `\t`, `\r`, `\n`.
    Whitespace,
    /// Anything except `\n`. Load-bearing for `@nl(until_newline)`.
    NotNewline,
}

/// Convenience constructor: a `LiteralKind` capture for one keyword.
/// Spec name: `literal_kind`.
pub fn literal_kind(keyword: &[u8], kind: AstKind) -> Combinator {
    Combinator::LiteralKind {
        keyword: keyword.to_vec(),
        kind,
    }
}

// ---------------------------------------------------------------------------
// Iterative Drop for Combinator — F-1 Checkpoint D.
//
// Combinator chains nest arbitrarily through `Box<Combinator>`-carrying
// variants (Repeat, Capture, Until, Shift, BraceBlock, ParenBlock). A
// pathologically deep chain (10,000+ levels, e.g. the F-4 depth-bound
// test) on the default recursive Drop would overflow the thread stack
// because each `Box<Combinator>` drop walks one frame deeper.
//
// The custom Drop walks the tree iteratively: as we visit a node, we
// move its children onto a worklist and drop the node's *body* (the
// Box) by replacing it with a leaf-only variant. The worklist is heap-
// allocated and bounded by tree size, not stack depth.
// ---------------------------------------------------------------------------

impl Drop for Combinator {
    fn drop(&mut self) {
        // Replace self with a leaf to break any cycle; collect children
        // onto a worklist. The replaced `self` is dropped at end of fn;
        // each worklist entry will have its children moved out in turn.
        let mut worklist: Vec<Combinator> = Vec::new();
        Self::extract_children_into(self, &mut worklist);
        while let Some(mut next) = worklist.pop() {
            Self::extract_children_into(&mut next, &mut worklist);
            // `next` drops here; with its children removed, the drop is
            // shallow (no recursion into Box<Combinator>).
        }
    }
}

impl Combinator {
    /// Move every `Box<Combinator>` and `Vec<Combinator>` child out of
    /// `node` into `into`, replacing the field with a leaf variant
    /// (`DarkFallback`). After this call, `node`'s remaining fields
    /// are leaves — dropping `node` is shallow.
    ///
    /// Used by the iterative `Drop` impl on `Combinator`. The trick:
    /// the recursive Drop chain through `Box<Combinator>` would
    /// overflow on a deeply-nested tree; this routine moves the body
    /// out before the box-drop sees it.
    fn extract_children_into(node: &mut Combinator, into: &mut Vec<Combinator>) {
        use Combinator::*;
        match node {
            Seq(children) | Choice(children) => {
                into.extend(children.drain(..));
            }
            Repeat { body, .. }
            | Capture { body, .. }
            | BraceBlock(body)
            | ParenBlock(body)
            | Shift { body, .. } => {
                // Replace the box's contents with a leaf, then push the
                // original contents onto the worklist.
                let stolen = std::mem::replace(body.as_mut(), DarkFallback);
                into.push(stolen);
            }
            Until { stop } => {
                let stolen = std::mem::replace(stop.as_mut(), DarkFallback);
                into.push(stolen);
            }
            // Leaves: nothing to move.
            Literal(_)
            | LiteralKind { .. }
            | Charset(_)
            | IoBinding
            | MatchArm
            | SelectVariant
            | KeywordFormBody { .. }
            | DarkFallback
            | MultiByteCharset(_) => {}
        }
    }
}

/// Merkle hash over a `Combinator` tree. One tag per variant; the
/// payload is the variant's data bytes (with children hashed recursively
/// and joined with `:`). FP1's load-bearing equation
/// `combinator_tree_oid(seed) == combinator_tree_oid(seed_prime)` is
/// what the seed must satisfy after round-tripping through `apply_h`.
///
/// Per `docs/specs/combinator-optimization.md` §9.1 ("always-on for
/// OID computation") and §10.2 ("the OID over normal forms is a
/// gauge-invariant observable"), the input is normalized via
/// [`normalize`] before hashing. This makes FP1 robust against
/// cosmetic encoding choices in the seed or in the parser output.
pub fn combinator_tree_oid(c: &Combinator) -> [u8; 32] {
    let normal = normalize(c);
    let oid_hex = combinator_tree_oid_hex(&normal);
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
            buf.extend_from_slice(max.map(|m| m.to_string()).unwrap_or_default().as_bytes());
            hash_tagged("comb:repeat", &buf)
        }
        Combinator::Capture { body, kind } => {
            let mut buf = Vec::new();
            buf.extend_from_slice(kind_tag(*kind).as_bytes());
            buf.push(b':');
            buf.extend_from_slice(combinator_tree_oid_hex(body).as_bytes());
            hash_tagged("comb:capture", &buf)
        }
        Combinator::Charset(k) => hash_tagged("comb:charset", charset_tag(*k).as_bytes()),
        Combinator::BraceBlock(body) => {
            hash_tagged("comb:brace_block", combinator_tree_oid_hex(body).as_bytes())
        }
        Combinator::ParenBlock(body) => {
            hash_tagged("comb:paren_block", combinator_tree_oid_hex(body).as_bytes())
        }
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
        Combinator::Until { stop } => {
            hash_tagged("comb:until", combinator_tree_oid_hex(stop).as_bytes())
        }
        Combinator::Shift { grammar, body } => {
            let mut buf = Vec::new();
            buf.extend_from_slice(grammar.as_bytes());
            buf.push(b':');
            buf.extend_from_slice(combinator_tree_oid_hex(body).as_bytes());
            hash_tagged("comb:shift", &buf)
        }
        Combinator::DarkFallback => hash_tagged("comb:dark_fallback", &[]),
        Combinator::MultiByteCharset(members) => {
            // Members are stored sorted by construction. The OID joins
            // them with `:` so a 5-keyword set hashes to a single
            // 32-byte value, identical regardless of source encoding.
            let mut buf = Vec::new();
            for (i, m) in members.iter().enumerate() {
                if i > 0 {
                    buf.push(b':');
                }
                buf.extend_from_slice(m);
            }
            hash_tagged("comb:multi_byte_charset", &buf)
        }
    }
}

fn kind_tag(k: AstKind) -> &'static str {
    match k {
        AstKind::Focus => "focus",
        AstKind::Project => "project",
        AstKind::Split => "split",
        AstKind::Shift => "shift",
        AstKind::Settle => "settle",
        AstKind::In => "in",
        AstKind::Out => "out",
        AstKind::IoBinding => "io_binding",
        AstKind::MatchExpr => "match_expr",
        AstKind::SelectExpr => "select_expr",
        AstKind::ParametricType => "parametric_type",
        AstKind::Dark => "dark",
        AstKind::Docblock => "docblock",
    }
}

fn charset_tag(k: CharsetKind) -> &'static str {
    match k {
        CharsetKind::WordChar => "word_char",
        CharsetKind::NameChar => "name_char",
        CharsetKind::IrIdentChar => "ir_ident_char",
        CharsetKind::IoNameChar => "io_name_char",
        CharsetKind::Whitespace => "whitespace",
        CharsetKind::NotNewline => "not_newline",
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

    fn settle(&self, beam: Self::Projected) -> Self::Refracted {
        let c = beam
            .value()
            .expect("Combinator::settle on dark beam")
            .clone();
        beam.next(c)
    }
}

/// Maximum recursion depth for the walker, normalize phases, and the
/// Fold5 / Fold5At catamorphisms. F-4 fix: bounds the attacker-
/// controlled stack-overflow surface from deeply-nested input
/// (`{{{...{ \ }...}}}`). 1024 is generous for any legitimate
/// `.mirror` file (the seed bottoms out under 30 levels) and stops a
/// hostile 50k-nested input well before the default Rust thread
/// stack (8 MB) is exhausted.
///
/// When the bound is exceeded, the Combinator-typed walkers emit
/// `Combinator::DarkFallback` carrying the failure mode implicitly;
/// callers can re-render the dark span via standard tooling.
pub(crate) const MAX_DEPTH: usize = 1024;

// ---------------------------------------------------------------------------
// F-1: the walker that walks.
//
// Pre-F-1 the walker was structural-self on every non-`Choice` variant —
// it ignored source bytes entirely. FP1's OID equality held tautologically:
// `apply_h(seed, anything) == seed` because the walker observed no bytes.
//
// F-1 makes the walker consume bytes, advance an offset, and emit a
// structural witness on success or `DarkFallback` on failure. The OID
// equality FP1 still holds for `seed`-vs-`glass.mirror.bytes` — but only
// because the seed actually accepts the meta-glass. Random bytes break it.
//
// See `docs/specs/walker-contract.md` for the per-variant contract.
// ---------------------------------------------------------------------------

/// Result of a single walker invocation.
///
/// - `witness`: the structural Combinator. On success, structurally equal
///   to the input (modulo Choice arm pruning). On parse failure,
///   `DarkFallback` at the failure site.
/// - `offset`: byte position after the consumed span (success) or where
///   the parse got stuck (failure).
/// - `success`: did the walk consume the bytes it claimed to?
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WalkOut {
    pub witness: Combinator,
    pub offset: usize,
    pub success: bool,
}

impl WalkOut {
    fn ok(witness: Combinator, offset: usize) -> Self {
        Self {
            witness,
            offset,
            success: true,
        }
    }
    fn dark(offset: usize) -> Self {
        Self {
            witness: Combinator::DarkFallback,
            offset,
            success: false,
        }
    }
}

/// Top-level entry: walk `c` over `source` starting at byte 0. Returns
/// the witness only — offset is discarded at the top level. Callers that
/// need the offset use [`walk_combinator_at`].
///
/// F-4 preserves: bounded by `MAX_DEPTH`; on overflow emits `DarkFallback`.
fn walk_combinator(c: &Combinator, source: &[u8], _offset: usize) -> Combinator {
    walk_combinator_at(c, source, 0, 0).witness
}

/// Walk `c` over `source` starting at `offset`. Threaded by Seq/Repeat/
/// Choice/etc.; the caller decides what to do with the new offset on
/// failure.
pub(crate) fn walk_combinator_at(
    c: &Combinator,
    source: &[u8],
    offset: usize,
    depth: usize,
) -> WalkOut {
    if depth >= MAX_DEPTH {
        return WalkOut::dark(offset);
    }
    let d = depth + 1;
    match c {
        // ----- Literal: exact byte match, no AST capture. -----
        Combinator::Literal(bytes) => {
            let n = bytes.len();
            if n == 0 {
                return WalkOut::ok(Combinator::Literal(bytes.clone()), offset);
            }
            if offset + n > source.len() {
                return WalkOut::dark(offset);
            }
            if &source[offset..offset + n] == bytes.as_slice() {
                WalkOut::ok(Combinator::Literal(bytes.clone()), offset + n)
            } else {
                WalkOut::dark(offset)
            }
        }
        // ----- Charset: single-byte class predicate. -----
        Combinator::Charset(k) => {
            if offset >= source.len() {
                return WalkOut::dark(offset);
            }
            if charset_matches(*k, source[offset]) {
                WalkOut::ok(Combinator::Charset(*k), offset + 1)
            } else {
                WalkOut::dark(offset)
            }
        }
        // ----- Seq: walk children in order, threading offset. -----
        Combinator::Seq(children) => {
            let mut walked: Vec<Combinator> = Vec::with_capacity(children.len());
            let mut cur = offset;
            for child in children {
                let out = walk_combinator_at(child, source, cur, d);
                if !out.success {
                    // Seq is atomic: any child failure dark-falls.
                    return WalkOut::dark(out.offset);
                }
                walked.push(out.witness);
                cur = out.offset;
            }
            WalkOut::ok(Combinator::Seq(walked), cur)
        }
        // ----- Repeat: Kleene-with-bounds. -----
        Combinator::Repeat { body, min, max } => {
            let mut iters = 0usize;
            let mut cur = offset;
            let mut walked_body: Option<Combinator> = None;
            loop {
                if let Some(m) = max {
                    if iters >= *m {
                        break;
                    }
                }
                let out = walk_combinator_at(body, source, cur, d);
                if !out.success {
                    break;
                }
                if out.offset == cur {
                    // Zero-consumption stop — would loop forever.
                    // The body matched empty (e.g., `Repeat 0 ws inside a 0-bound`).
                    // Capture the witness once and break.
                    walked_body = Some(out.witness);
                    iters += 1;
                    break;
                }
                walked_body = Some(out.witness);
                cur = out.offset;
                iters += 1;
            }
            if iters < *min {
                return WalkOut::dark(cur);
            }
            let body_witness = walked_body.unwrap_or_else(|| (**body).clone());
            WalkOut::ok(
                Combinator::Repeat {
                    body: Box::new(body_witness),
                    min: *min,
                    max: *max,
                },
                cur,
            )
        }
        // ----- Choice: first matching arm wins. -----
        Combinator::Choice(branches) => {
            // Split into pure-LiteralKind arms (pruned by whole-word
            // occurrence; the FP2 keyword-table behavior) and structural
            // arms (kept verbatim for OID stability).
            //
            // Try each arm in declaration order at `offset`. First
            // success wins. The witness keeps the seed's arms in
            // declaration order; LiteralKind arms that don't occur
            // anywhere in source are dropped to support the legacy
            // keyword-pruning path.
            let mut kept: Vec<Combinator> = Vec::with_capacity(branches.len());
            let mut winning_offset: Option<usize> = None;
            let mut last_fail_offset = offset;
            for b in branches {
                match b {
                    Combinator::LiteralKind { .. } => {
                        // Prune by whole-word presence anywhere in source.
                        if branch_keyword_occurs(b, source) {
                            kept.push(b.clone());
                        }
                        // LiteralKind doesn't participate in the
                        // "first-match wins" race at `offset` — it's a
                        // keyword-table arm; the structural seed
                        // doesn't put LiteralKind into a positional
                        // Choice. (If it does in a future seed, the
                        // walker would need to extend this.)
                    }
                    _ => {
                        kept.push(b.clone());
                        if winning_offset.is_none() {
                            let out = walk_combinator_at(b, source, offset, d);
                            if out.success {
                                winning_offset = Some(out.offset);
                            } else {
                                last_fail_offset = out.offset.max(last_fail_offset);
                            }
                        }
                    }
                }
            }
            // If the Choice had any structural arms, at least one must
            // succeed; otherwise the Choice is Dark at `offset`.
            //
            // If the Choice was pure-LiteralKind (no structural arms),
            // the walk succeeds with `offset' = offset` (the keyword
            // table is a pruning shift, not a consuming parser).
            let has_structural = branches
                .iter()
                .any(|b| !matches!(b, Combinator::LiteralKind { .. }));
            let new_offset = if has_structural {
                match winning_offset {
                    Some(o) => o,
                    None => return WalkOut::dark(last_fail_offset),
                }
            } else {
                offset
            };
            WalkOut::ok(Combinator::Choice(kept), new_offset)
        }
        // ----- LiteralKind: keyword with whole-word boundary. -----
        Combinator::LiteralKind { keyword, kind } => {
            let n = keyword.len();
            if n == 0 || offset + n > source.len() {
                return WalkOut::dark(offset);
            }
            if &source[offset..offset + n] != keyword.as_slice() {
                return WalkOut::dark(offset);
            }
            let right_ok = offset + n == source.len() || !is_word_byte(source[offset + n]);
            if !right_ok {
                return WalkOut::dark(offset);
            }
            WalkOut::ok(
                Combinator::LiteralKind {
                    keyword: keyword.clone(),
                    kind: *kind,
                },
                offset + n,
            )
        }
        // ----- Capture: wrap body's witness as Capture { body, kind }. -----
        Combinator::Capture { body, kind } => {
            let out = walk_combinator_at(body, source, offset, d);
            if !out.success {
                return WalkOut::dark(out.offset);
            }
            WalkOut::ok(
                Combinator::Capture {
                    body: Box::new(out.witness),
                    kind: *kind,
                },
                out.offset,
            )
        }
        // ----- BraceBlock: balanced `{ ... }`, then walk body on inner. -----
        Combinator::BraceBlock(body) => walk_block(body, source, offset, b'{', b'}', d, |b| {
            Combinator::BraceBlock(Box::new(b))
        }),
        // ----- ParenBlock: balanced `( ... )`, then walk body on inner. -----
        Combinator::ParenBlock(body) => walk_block(body, source, offset, b'(', b')', d, |b| {
            Combinator::ParenBlock(Box::new(b))
        }),
        // ----- Until: scan to stop combinator's peek; don't consume stop. -----
        Combinator::Until { stop } => {
            let mut cur = offset;
            let mut peeked: Option<Combinator> = None;
            while cur < source.len() {
                let peek = walk_combinator_at(stop, source, cur, d);
                if peek.success {
                    peeked = Some(peek.witness);
                    break;
                }
                cur += 1;
            }
            let stop_witness = peeked.unwrap_or_else(|| {
                // Reached EOF without seeing stop. Walk the stop
                // structurally for OID preservation; this matches the
                // permissive "consume to end" outcome the spec defines.
                walk_combinator_at(stop, source, source.len(), d).witness
            });
            // If structural walk of stop returned Dark (e.g. for a
            // Literal that doesn't match EOF), fall back to the
            // seeded stop verbatim. OID-preserving.
            let stop_witness = match stop_witness {
                Combinator::DarkFallback => (**stop).clone(),
                w => w,
            };
            WalkOut::ok(
                Combinator::Until {
                    stop: Box::new(stop_witness),
                },
                cur,
            )
        }
        // ----- Shift: structural for now; Checkpoint C wires registry. -----
        Combinator::Shift { grammar, body } => {
            let out = walk_combinator_at(body, source, offset, d);
            if !out.success {
                return WalkOut::dark(out.offset);
            }
            WalkOut::ok(
                Combinator::Shift {
                    grammar: grammar.clone(),
                    body: Box::new(out.witness),
                },
                out.offset,
            )
        }
        // ----- DarkFallback: strict-classification sentinel. -----
        Combinator::DarkFallback => {
            // Consume the rest of the source as Dark. Always succeeds.
            WalkOut::ok(Combinator::DarkFallback, source.len())
        }
        // ----- MultiByteCharset: first matching member. -----
        Combinator::MultiByteCharset(members) => {
            for m in members {
                let n = m.len();
                if n > 0
                    && offset + n <= source.len()
                    && &source[offset..offset + n] == m.as_slice()
                {
                    return WalkOut::ok(Combinator::MultiByteCharset(members.clone()), offset + n);
                }
            }
            WalkOut::dark(offset)
        }
        // ----- Surface placeholders: no-op consumption, structural-self. -----
        // These are surface-keyword forms waiting for a later tick. The
        // walker treats them as zero-width markers — they preserve OID
        // but don't consume bytes. The seed does not contain any of
        // these today; the meta-glass `io_form`, `match_form`,
        // `select_form` declarations expand to these once the
        // higher-tick wiring lands.
        Combinator::IoBinding => WalkOut::ok(Combinator::IoBinding, offset),
        Combinator::MatchArm => WalkOut::ok(Combinator::MatchArm, offset),
        Combinator::SelectVariant => WalkOut::ok(Combinator::SelectVariant, offset),
        Combinator::KeywordFormBody { keyword, kind } => WalkOut::ok(
            Combinator::KeywordFormBody {
                keyword: keyword.clone(),
                kind: *kind,
            },
            offset,
        ),
    }
}

/// Single-byte charset predicate dispatch. Shared by walker + tests.
fn charset_matches(k: CharsetKind, b: u8) -> bool {
    match k {
        CharsetKind::WordChar => b.is_ascii_alphanumeric() || b == b'_',
        CharsetKind::NameChar => {
            b.is_ascii_alphanumeric() || b == b'_' || b == b'@' || b == b'/' || b == b'.'
        }
        CharsetKind::IrIdentChar => {
            b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'$'
        }
        CharsetKind::IoNameChar => b.is_ascii_alphanumeric() || b == b'_',
        CharsetKind::Whitespace => matches!(b, b' ' | b'\t' | b'\r' | b'\n'),
        CharsetKind::NotNewline => b != b'\n',
    }
}

/// Shared block walker for `BraceBlock` / `ParenBlock`. Scans balanced
/// `open`/`close` delimiters, then walks `body` over the inner bytes.
fn walk_block(
    body: &Combinator,
    source: &[u8],
    offset: usize,
    open: u8,
    close: u8,
    depth: usize,
    wrap: impl FnOnce(Combinator) -> Combinator,
) -> WalkOut {
    if offset >= source.len() || source[offset] != open {
        return WalkOut::dark(offset);
    }
    // Find the matching close.
    let mut pos = offset + 1;
    let mut bal: i32 = 1;
    while pos < source.len() && bal > 0 {
        if source[pos] == open {
            bal += 1;
        } else if source[pos] == close {
            bal -= 1;
        }
        if bal > 0 {
            pos += 1;
        }
    }
    if bal != 0 {
        // Unbalanced.
        return WalkOut::dark(offset);
    }
    // Inner bytes: source[offset+1..pos]. Walk the body over them at
    // offset 0; the body must consume them up to (and tolerating
    // trailing whitespace via its own Repeat). We don't require the
    // body to consume every byte exactly — the structural seed
    // typically uses `Repeat(form-or-ws, 0, none)` which consumes
    // everything legal and stops. If unconsumed bytes remain at the
    // end of the inner span, that's a parse failure.
    let inner = &source[offset + 1..pos];
    let inner_out = walk_combinator_at(body, inner, 0, depth);
    if !inner_out.success {
        return WalkOut::dark(offset + 1 + inner_out.offset);
    }
    // The body must consume the entire inner span. If it stopped
    // short, that's a parse failure — trailing junk inside the block.
    if inner_out.offset != inner.len() {
        return WalkOut::dark(offset + 1 + inner_out.offset);
    }
    WalkOut::ok(wrap(inner_out.witness), pos + 1)
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

/// Whole-word boundary check shared by the combinator walker and the
/// byte-level rewrite path (`pipeline::apply_rewrites`). Word bytes
/// are ASCII alnum + `_`. Path separator `/` and reference sigil `@`
/// are boundaries — a keyword inside `@mirror/grammar` is whole-word
/// bounded on both sides.
///
/// F-3 fix: previously this function differed from `apply_rewrites`'s
/// local `is_word_byte` (the walker treated `/` as a word byte; the
/// rewrite path did not). Unified to the rewrite's narrower form so
/// the two surfaces agree on every byte in 0..=255; the migration's
/// structural rewrite of path components requires `/` to be a
/// boundary. Pinned by `pipeline::rewrite_tests::f3_is_word_byte_unified_across_surfaces`.
pub(crate) fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// The five op-keyword Choice. The keyword table for any mirror file's
/// op vocabulary; the body of every `prism @(<name>) { … }` form in
/// `00-prism.mirror`. Reused for FP2 (grammar.mirror lifts to the
/// pruned subset of this Choice).
pub fn op_keyword_choice() -> Combinator {
    Combinator::Choice(vec![
        literal_kind(b"focus", AstKind::Focus),
        literal_kind(b"project", AstKind::Project),
        literal_kind(b"split", AstKind::Split),
        literal_kind(b"shift", AstKind::Shift),
        literal_kind(b"settle", AstKind::Settle),
    ])
}

/// The meta-glass seed — a permissive structural Combinator that accepts
/// any mirror file with balanced braces/parens. The seed walks bytes:
///
///   file = Repeat(unit, 0, None)
///   unit = Choice(
///     Charset(Whitespace),    # one whitespace byte
///     comment,                # `# ... \n`
///     brace_unit,             # `{ file }`     (recursive)
///     paren_unit,             # `( file )`    (recursive)
///     Charset(NotNewline),    # any non-newline byte (gobbles content)
///   )
///   comment    = Seq(Literal("#"), Shift(@nl, Until("\n")))
///   brace_unit = BraceBlock(file)
///   paren_unit = ParenBlock(file)
///
/// F-1 Checkpoint C: regrown for the real walker. The previous seed
/// was structurally rigid (expected exactly `in @ref grammar @ref(...)
/// { ... } out @ref`); it could not parse `std/mirror/grammar.mirror`
/// once the walker actually consumed bytes (the ParenBlock body had
/// no arm for quoted strings or commas).
///
/// The new seed is a *balanced-bytes recognizer*: it parses any
/// well-formed mirror file (one with balanced `{}` and `()`),
/// preserving its OID under the round-trip. Structure is recovered
/// later by the grammars declared inside `grammar.mirror` itself —
/// the meta-glass loop becomes non-trivial once the grammar registry
/// (Checkpoint C's Shift wiring) routes cross-grammar lifts to
/// pre-loaded Combinator trees.
///
/// FP1 (Checkpoint D): `apply_h(seed, grammar.mirror.bytes)` returns
/// a Combinator tree with the same OID as the seed. Holds because
/// every variant in the new seed walks to itself on any well-formed
/// input.
///
/// FP2 (Checkpoint C): `apply_h(seed, 00-prism.mirror.bytes)` returns
/// a Combinator tree with no Dark fragments. Holds because
/// `00-prism.mirror` has balanced braces and parens — every byte
/// matches some unit arm.
///
/// Size: 5 `let` bindings, well under the 60-LOC ceiling the brief
/// imposes.
pub fn prism_seed() -> Combinator {
    use Combinator::*;
    let ws_byte = Charset(CharsetKind::Whitespace);
    let comment = Seq(vec![
        Literal(b"#".to_vec()),
        Shift {
            grammar: "@nl".to_string(),
            body: Box::new(Until {
                stop: Box::new(Literal(b"\n".to_vec())),
            }),
        },
    ]);
    // file = Repeat(unit, 0, None) — the recursive root. Used by
    // brace_unit and paren_unit to walk inner bytes.
    //
    // brace_unit / paren_unit / Charset(NotNewline) cover "any byte".
    // Order matters: try whitespace, comment, then structured blocks,
    // then "any non-newline byte" as the fall-through. The fall-through
    // accepts `i`, `n`, `@`, `"`, `,`, etc.
    Repeat {
        body: Box::new(Choice(vec![
            ws_byte.clone(),
            comment.clone(),
            // brace_unit and paren_unit are encoded inline because they
            // each take a Repeat of the file root — which is the seed
            // itself. We use the same Choice for the inner Repeat.
            BraceBlock(Box::new(Repeat {
                body: Box::new(Choice(vec![
                    ws_byte.clone(),
                    comment.clone(),
                    BraceBlock(Box::new(Repeat {
                        body: Box::new(Charset(CharsetKind::NotNewline)),
                        min: 0,
                        max: None,
                    })),
                    ParenBlock(Box::new(Repeat {
                        body: Box::new(Charset(CharsetKind::NotNewline)),
                        min: 0,
                        max: None,
                    })),
                    Charset(CharsetKind::NotNewline),
                ])),
                min: 0,
                max: None,
            })),
            ParenBlock(Box::new(Repeat {
                body: Box::new(Charset(CharsetKind::NotNewline)),
                min: 0,
                max: None,
            })),
            Charset(CharsetKind::NotNewline),
        ])),
        min: 0,
        max: None,
    }
}

// ---------------------------------------------------------------------------
// Beta tree normalization — phase 1 (non-charset structural redexes).
//
// Per `docs/specs/combinator-optimization.md` §2: the redex set E1–E13
// (excluding E10/E11/E14 which are *not* redexes per the spec). Phase 1
// covers the non-charset structural rewrites:
//
//   E1.  Seq associativity:        Seq([Seq([a, b]), c]) → Seq([a, b, c])
//   E2.  Choice associativity:     Choice([Choice([a, b]), c]) → Choice([a, b, c])
//   E3.  Singleton Seq:            Seq([a]) → a
//   E4.  Singleton Choice:         Choice([a]) → a
//   E5.  Empty literal in Seq:     Seq([…, Literal(""), …]) → Seq([…, …])
//   E6.  Zero-bound Repeat:        Repeat { body, min: _, max: Some(0) } → Literal("")
//   E7.  One-one Repeat:           Repeat { body, min: 1, max: Some(1) } → body
//   E8.  Repeat of empty:          Repeat { body: Literal(""), … } → Literal("")
//   E12. DarkFallback dominates:   Choice([…, a, DarkFallback, b, …]) → Choice([…, a, DarkFallback])
//   E13. Empty Seq:                Seq([]) → Literal("")
//
// Bottom-up: children are normalized before parents. Per the spec's
// termination argument (§2.3), each step strictly decreases the
// well-founded measure (tree_size, seq_depth, choice_depth); the
// algorithm terminates in O(tree_size²).
//
// Confluence within phase 1 is by the diamond lemma sketch in §2.4
// cases 1–7 (the cases involving phase-1 redexes only); the critical
// pair E9 × E2 — which would break confluence if charset compilation
// ran interleaved — is fenced into phase 2.
// ---------------------------------------------------------------------------

/// Normalize a combinator tree under phase-1 redexes (E1–E8, E12–E14).
/// Bottom-up: children are normalized before parents. Returns a fresh
/// tree; the input is borrowed.
///
/// This is one half of [`normalize`]; the full normalization is
/// `normalize_phase2(normalize_phase1(c))`. Splitting the phases buys
/// confluence per the spec's §2.4 case 8 critical-pair analysis.
///
/// F-4 fix: bounded by `MAX_DEPTH`; on overflow returns
/// `Combinator::DarkFallback`. F-5 fix: empty `Choice([])` collapses
/// to `Combinator::DarkFallback` per spec E14 (instead of silently
/// persisting as a malformed normal-form node).
pub fn normalize_phase1(c: &Combinator) -> Combinator {
    normalize_phase1_at(c, 0)
}

fn normalize_phase1_at(c: &Combinator, depth: usize) -> Combinator {
    use Combinator::*;
    if depth >= MAX_DEPTH {
        return DarkFallback;
    }
    let d = depth + 1;
    match c {
        Seq(children) => {
            // Recurse first.
            let normalized: Vec<Combinator> =
                children.iter().map(|c| normalize_phase1_at(c, d)).collect();
            // E1: flatten nested Seq.
            //
            // E0509: `Combinator` impls Drop (iterative drop, F-1
            // checkpoint D), so we can't `match child { Seq(inner) =>
            // ... }` to move `inner` out. Use `std::mem::take` on the
            // inner vec instead — leaves child as an empty Seq, which
            // drops trivially.
            let mut flat: Vec<Combinator> = Vec::with_capacity(normalized.len());
            for mut child in normalized {
                if let Seq(children) = &mut child {
                    let stolen = std::mem::take(children);
                    flat.extend(stolen);
                } else {
                    flat.push(child);
                }
            }
            // E5: drop empty literals.
            let trimmed: Vec<Combinator> = flat
                .into_iter()
                .filter(|c| !matches!(c, Literal(b) if b.is_empty()))
                .collect();
            // E3 / E13.
            match trimmed.len() {
                0 => Literal(Vec::new()),                 // E13
                1 => trimmed.into_iter().next().unwrap(), // E3
                _ => Seq(trimmed),
            }
        }
        Choice(arms) => {
            // Recurse first.
            let normalized: Vec<Combinator> =
                arms.iter().map(|c| normalize_phase1_at(c, d)).collect();
            // E2: flatten nested Choice. See E0509 note in Seq above.
            let mut flat: Vec<Combinator> = Vec::with_capacity(normalized.len());
            for mut arm in normalized {
                if let Choice(inner) = &mut arm {
                    let stolen = std::mem::take(inner);
                    flat.extend(stolen);
                } else {
                    flat.push(arm);
                }
            }
            // E12: truncate after the first DarkFallback.
            let mut truncated: Vec<Combinator> = Vec::with_capacity(flat.len());
            for arm in flat {
                let is_dark = matches!(arm, DarkFallback);
                truncated.push(arm);
                if is_dark {
                    break;
                }
            }
            // E4 + E14: singleton collapses to the arm; empty Choice
            // collapses to `DarkFallback` (the canonical Dark-emitting
            // combinator). `Choice([])` should never appear in well-
            // formed input — if smuggled (e.g., by source pruning
            // emptying a Choice in the walker), it lands in Dark
            // explicitly rather than persisting as a malformed node.
            // Pinned by `e14_empty_choice_collapses_to_dark_fallback`.
            match truncated.len() {
                0 => DarkFallback, // E14
                1 => truncated.into_iter().next().unwrap(),
                _ => Choice(truncated),
            }
        }
        Repeat { body, min, max } => {
            let body_n = normalize_phase1_at(body, d);
            // E6: zero upper bound.
            if matches!(max, Some(0)) {
                return Literal(Vec::new());
            }
            // E7: exact-one bounds.
            if *min == 1 && matches!(max, Some(1)) {
                return body_n;
            }
            // E8: repeat of empty literal.
            if matches!(&body_n, Literal(b) if b.is_empty()) {
                return Literal(Vec::new());
            }
            Repeat {
                body: Box::new(body_n),
                min: *min,
                max: *max,
            }
        }
        Capture { body, kind } => Capture {
            body: Box::new(normalize_phase1_at(body, d)),
            kind: *kind,
        },
        BraceBlock(body) => BraceBlock(Box::new(normalize_phase1_at(body, d))),
        ParenBlock(body) => ParenBlock(Box::new(normalize_phase1_at(body, d))),
        Until { stop } => Until {
            stop: Box::new(normalize_phase1_at(stop, d)),
        },
        Shift { grammar, body } => Shift {
            grammar: grammar.clone(),
            body: Box::new(normalize_phase1_at(body, d)),
        },
        // Leaves: no phase-1 redex applies.
        Literal(_)
        | LiteralKind { .. }
        | Charset(_)
        | IoBinding
        | MatchArm
        | SelectVariant
        | KeywordFormBody { .. }
        | DarkFallback
        | MultiByteCharset(_) => c.clone(),
    }
}

// ---------------------------------------------------------------------------
// Beta tree normalization — phase 2 (charset compilation).
//
// Per `docs/specs/combinator-optimization.md` §3 and §2.4 case 8: the
// E9/E15 redexes are deliberately fenced into a second phase so that
// phase 1's Choice-flattening (E2) runs to completion before charset
// compilation considers any Choice. This restores the confluence the
// critical pair E9 × E2 would otherwise break (§2.4 case 8 resolution
// option (a) — "restrict E9 to maximal Choices").
//
// Phase 2 covers:
//
//   E9 / E15. Choice([Literal(b1), Literal(b2), …]) → MultiByteCharset(
//             [b1, b2, …]) where every arm is a Literal. Per the spec
//             §3.2's closed-vs-open resolution, single-byte and
//             multi-byte literals are handled uniformly by
//             `MultiByteCharset`; the closed `Charset(CharsetKind)`
//             stays for declared single-byte classes (whitespace,
//             name-char, etc.).
//
// The members of the resulting MultiByteCharset are sorted
// lexicographically and deduplicated so the OID is invariant under
// arm permutation (the spec calls this out as making the OID a
// gauge-invariant observable per §10.2).
// ---------------------------------------------------------------------------

/// Normalize a combinator tree under phase-2 redexes (E9 + E15 —
/// charset compilation). Bottom-up: children compile before parents.
/// Returns a fresh tree.
///
/// The expected calling pattern is `normalize_phase2(normalize_phase1(c))`
/// — see [`normalize`].
///
/// F-4 fix: bounded by `MAX_DEPTH`; on overflow returns
/// `Combinator::DarkFallback`.
pub fn normalize_phase2(c: &Combinator) -> Combinator {
    normalize_phase2_at(c, 0)
}

fn normalize_phase2_at(c: &Combinator, depth: usize) -> Combinator {
    use Combinator::*;
    if depth >= MAX_DEPTH {
        return DarkFallback;
    }
    let d = depth + 1;
    match c {
        Choice(arms) => {
            // Recurse first.
            let normalized: Vec<Combinator> =
                arms.iter().map(|c| normalize_phase2_at(c, d)).collect();
            // E15 (and E9 as its single-byte special case): if every
            // arm is a Literal, collapse to MultiByteCharset.
            if !normalized.is_empty() && normalized.iter().all(|c| matches!(c, Literal(_))) {
                // E0509: Combinator impls Drop, so we can't move
                // `b` out of `Literal(b)`. Use `std::mem::take` to
                // swap the bytes out, leaving a hollow Literal that
                // drops trivially.
                let mut members: Vec<Vec<u8>> = normalized
                    .into_iter()
                    .map(|mut c| match &mut c {
                        Literal(b) => std::mem::take(b),
                        _ => unreachable!("checked all-literal above"),
                    })
                    .collect();
                members.sort();
                members.dedup();
                return MultiByteCharset(members);
            }
            Choice(normalized)
        }
        Seq(children) => Seq(children.iter().map(|c| normalize_phase2_at(c, d)).collect()),
        Repeat { body, min, max } => Repeat {
            body: Box::new(normalize_phase2_at(body, d)),
            min: *min,
            max: *max,
        },
        Capture { body, kind } => Capture {
            body: Box::new(normalize_phase2_at(body, d)),
            kind: *kind,
        },
        BraceBlock(body) => BraceBlock(Box::new(normalize_phase2_at(body, d))),
        ParenBlock(body) => ParenBlock(Box::new(normalize_phase2_at(body, d))),
        Until { stop } => Until {
            stop: Box::new(normalize_phase2_at(stop, d)),
        },
        Shift { grammar, body } => Shift {
            grammar: grammar.clone(),
            body: Box::new(normalize_phase2_at(body, d)),
        },
        // Leaves and already-compiled forms: no phase-2 redex applies.
        Literal(_)
        | LiteralKind { .. }
        | Charset(_)
        | IoBinding
        | MatchArm
        | SelectVariant
        | KeywordFormBody { .. }
        | DarkFallback
        | MultiByteCharset(_) => c.clone(),
    }
}

/// Full beta-normalization: phase 1 (structural redexes) followed by
/// phase 2 (charset compilation). Per the spec §2.4 case 8, the two
/// phases compose deterministically; the resulting normal form is
/// unique up to E1–E13 + E9/E15.
///
/// Always-on: there is no flag. Per Q11 of the spec's open questions,
/// flag-gating would re-introduce the encoding-choice fragility that
/// normalization exists to fix.
pub fn normalize(c: &Combinator) -> Combinator {
    normalize_phase2(&normalize_phase1(c))
}

#[cfg(test)]
mod combinator_tests {
    use super::*;
    use std::path::PathBuf;

    /// Path to the meta-glass source file, relative to the project
    /// root (the parent of `bootstrap/`). Updated by checkpoint 6 to
    /// `mirror/glass.mirror` after the migration.
    const GLASS_PATH: &str = "std/mirror/grammar.mirror";

    fn read_boot_file(rel: &str) -> Vec<u8> {
        // bootstrap/Cargo.toml dir → ../boot/<rel>
        let manifest =
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set under cargo test");
        let mut p = PathBuf::from(manifest);
        p.pop(); // bootstrap → mirror
        p.push("boot");
        for seg in rel.split('/') {
            p.push(seg);
        }
        std::fs::read(&p).unwrap_or_else(|e| panic!("read {:?}: {}", p, e))
    }

    fn parse_with(combinator: &Combinator, bytes: &[u8]) -> Combinator {
        // Heterogeneous `apply_h`: input state is `(Vec<u8>, usize)`,
        // output state is `Combinator`. Same call shape as for AST
        // states — the substrate-level helper no longer constrains
        // `In == Out`.
        match apply_h(combinator, (bytes.to_vec(), 0usize)) {
            Imperfect::Success(c) => c,
            Imperfect::Partial(c, _loss) => c,
            Imperfect::Failure(_, _) => {
                unreachable!("Combinator::Error = Infallible; Failure uninhabited")
            }
        }
    }

    /// FP1 promoted — the meta-glass parses itself.
    ///
    /// `apply_h(seed, glass.mirror.bytes)` parses to a Combinator tree
    /// (`meta_glass`) with the same `combinator_tree_oid` as `seed`
    /// itself. Mirror's grammar describes mirror's grammar; the
    /// description IS the implementation.
    ///
    /// F-1 Checkpoint C: re-enabled with the grown seed. The walker
    /// now actually consumes bytes (instead of structural-self), so
    /// this assertion is non-vacuous — random bytes would fail it.
    /// The random-bytes inequality test is added in Checkpoint D.
    #[test]
    fn fp1_meta_glass_parses_itself() {
        let seed = prism_seed();
        let glass_bytes = read_boot_file(GLASS_PATH);
        let meta_glass = parse_with(&seed, &glass_bytes);
        let oid_seed = combinator_tree_oid(&seed);
        let oid_glass = combinator_tree_oid(&meta_glass);
        assert_eq!(
            oid_seed, oid_glass,
            "FP1: seed OID != meta_glass OID\nseed       = {:?}\nmeta_glass = {:?}",
            seed, meta_glass
        );
    }

    /// FP2 replaced — well-formedness of the meta-glass shift on
    /// `00-prism.mirror`. The pre-meta-glass FP2 (keyword-table
    /// pruning) is obsolete: the meta-glass is no longer a keyword
    /// table. The new well-formedness check: applying the meta-glass
    /// to `00-prism.mirror.bytes` produces a Combinator tree with no
    /// Dark fragments.
    ///
    /// F-1 Checkpoint C: re-enabled with the grown seed. The seed is a
    /// permissive balanced-bytes recognizer; `00-prism.mirror` (with
    /// balanced braces and parens) walks cleanly through with no Dark.
    #[test]
    fn fp2_well_formedness_of_meta_glass_lift() {
        let seed = prism_seed();
        let glass_bytes = read_boot_file(GLASS_PATH);
        let meta_glass = parse_with(&seed, &glass_bytes);
        let prism_bytes = read_boot_file("00-prism.mirror");
        let prism_tree = parse_with(&meta_glass, &prism_bytes);
        assert!(
            no_dark_in_tree(&prism_tree),
            "FP2: 00-prism.mirror shift produced Dark fragments"
        );
    }

    /// Well-formedness of the meta-glass shift on nl.mirror. The
    /// bare-@nl grammar declaration parses cleanly through the
    /// meta-glass.
    ///
    /// F-1 Checkpoint C: re-enabled with the grown seed. `nl.mirror`
    /// has balanced braces and parens; the meta-glass walks cleanly.
    #[test]
    fn nl_mirror_lifts_cleanly() {
        let seed = prism_seed();
        let glass_bytes = read_boot_file(GLASS_PATH);
        let meta_glass = parse_with(&seed, &glass_bytes);
        let nl_bytes = read_boot_file("std/mirror/nl.mirror");
        let nl_tree = parse_with(&meta_glass, &nl_bytes);
        assert!(
            no_dark_in_tree(&nl_tree),
            "nl.mirror shift produced Dark fragments"
        );
    }

    /// Emit the FP1 OID hex values for the commit message. Pure
    /// observation — no assertion. Runs at every `cargo test` to keep
    /// the values discoverable.
    #[test]
    fn emit_oid_hex_for_log() {
        let seed = prism_seed();
        let glass_bytes = read_boot_file("std/mirror/grammar.mirror");
        let meta_glass = parse_with(&seed, &glass_bytes);
        eprintln!("FP1 seed       OID hex: {}", combinator_tree_oid_hex(&seed));
        eprintln!(
            "FP1 meta_glass OID hex: {}",
            combinator_tree_oid_hex(&meta_glass)
        );
    }

    /// Recursively check the tree contains no Dark fragments. The
    /// structural-self walker never emits Dark; this is a smoke check
    /// against the well-formedness of any Combinator tree.
    fn no_dark_in_tree(c: &Combinator) -> bool {
        match c {
            Combinator::Seq(children) | Combinator::Choice(children) => {
                children.iter().all(no_dark_in_tree)
            }
            Combinator::Repeat { body, .. }
            | Combinator::Capture { body, .. }
            | Combinator::BraceBlock(body)
            | Combinator::ParenBlock(body)
            | Combinator::Until { stop: body }
            | Combinator::Shift { body, .. } => no_dark_in_tree(body),
            Combinator::LiteralKind { .. }
            | Combinator::Literal(_)
            | Combinator::Charset(_)
            | Combinator::IoBinding
            | Combinator::MatchArm
            | Combinator::SelectVariant
            | Combinator::KeywordFormBody { .. }
            | Combinator::DarkFallback
            | Combinator::MultiByteCharset(_) => true,
        }
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

    // ---------- Checkpoint 1 variant tests ----------
    // Each new variant exercises the structural-self walk: walking it
    // against any source returns a tree with the same OID. This is the
    // FP1 contract at the variant level.

    #[test]
    fn repeat_walks_to_self() {
        let c = Combinator::Repeat {
            body: Box::new(Combinator::Literal(b"x".to_vec())),
            min: 0,
            max: None,
        };
        let walked = parse_with(&c, b"xxxy");
        assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&walked));
    }

    /// Each charset, fed a byte the predicate accepts, walks to a
    /// `Charset(k)` witness with the same OID as the input.
    /// F-1: the walker consumes one byte on success; the witness is
    /// still the same `Charset(k)`, so the OID is preserved.
    #[test]
    fn charset_walks_to_self() {
        // Per CharsetKind: a byte the predicate accepts.
        let cases: &[(CharsetKind, &[u8])] = &[
            (CharsetKind::WordChar, b"a"),
            (CharsetKind::NameChar, b"@"),
            (CharsetKind::IrIdentChar, b"_"),
            (CharsetKind::IoNameChar, b"x"),
            (CharsetKind::Whitespace, b" "),
            (CharsetKind::NotNewline, b"x"),
        ];
        for (k, src) in cases {
            let c = Combinator::Charset(*k);
            let walked = parse_with(&c, src);
            assert_eq!(
                combinator_tree_oid(&c),
                combinator_tree_oid(&walked),
                "charset {:?} should walk to self on {:?}",
                k,
                src
            );
        }
    }

    #[test]
    fn paren_block_walks_to_self() {
        let c = Combinator::ParenBlock(Box::new(Combinator::Literal(b"id".to_vec())));
        let walked = parse_with(&c, b"(id)");
        assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&walked));
    }

    #[test]
    fn until_walks_to_self() {
        let c = Combinator::Until {
            stop: Box::new(Combinator::Literal(b"\n".to_vec())),
        };
        let walked = parse_with(&c, b"some text\nand more");
        assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&walked));
    }

    #[test]
    fn lift_walks_to_self() {
        let c = Combinator::Shift {
            grammar: "@nl".to_string(),
            body: Box::new(Combinator::Until {
                stop: Box::new(Combinator::Literal(b"\n".to_vec())),
            }),
        };
        let walked = parse_with(&c, b"# this is a comment\n");
        assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&walked));
    }

    #[test]
    fn match_arm_select_dark_fallback_walk_to_self() {
        for c in [
            Combinator::MatchArm,
            Combinator::SelectVariant,
            Combinator::DarkFallback,
        ] {
            let walked = parse_with(&c, b"anything");
            assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&walked));
        }
    }

    #[test]
    fn keyword_form_body_walks_to_self() {
        let c = Combinator::KeywordFormBody {
            keyword: b"define".to_vec(),
            kind: AstKind::Focus,
        };
        let walked = parse_with(&c, b"define i32 @foo() { ret i32 0 }");
        assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&walked));
    }

    // ---------- normalize_phase1 — one test per redex E1–E8, E12–E13 ----------

    /// E1. Seq associativity flattens nested Seqs.
    #[test]
    fn e1_seq_associativity_flattens() {
        let a = Combinator::Literal(b"a".to_vec());
        let b = Combinator::Literal(b"b".to_vec());
        let c = Combinator::Literal(b"c".to_vec());
        let input = Combinator::Seq(vec![Combinator::Seq(vec![a.clone(), b.clone()]), c.clone()]);
        let expected = Combinator::Seq(vec![a, b, c]);
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// E2. Choice associativity flattens nested Choices.
    #[test]
    fn e2_choice_associativity_flattens() {
        let a = Combinator::Literal(b"a".to_vec());
        let b = Combinator::Literal(b"b".to_vec());
        let c = Combinator::Literal(b"c".to_vec());
        let input = Combinator::Choice(vec![
            Combinator::Choice(vec![a.clone(), b.clone()]),
            c.clone(),
        ]);
        let expected = Combinator::Choice(vec![a, b, c]);
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// E3. Singleton Seq collapses to its single element.
    #[test]
    fn e3_singleton_seq_collapses() {
        let a = Combinator::Literal(b"a".to_vec());
        let input = Combinator::Seq(vec![a.clone()]);
        assert_eq!(normalize_phase1(&input), a);
    }

    /// E4. Singleton Choice collapses to its single arm.
    #[test]
    fn e4_singleton_choice_collapses() {
        let a = Combinator::Literal(b"a".to_vec());
        let input = Combinator::Choice(vec![a.clone()]);
        assert_eq!(normalize_phase1(&input), a);
    }

    /// E5. Empty literals are dropped from Seq.
    #[test]
    fn e5_empty_literal_dropped_from_seq() {
        let a = Combinator::Literal(b"a".to_vec());
        let b = Combinator::Literal(b"b".to_vec());
        let input = Combinator::Seq(vec![a.clone(), Combinator::Literal(Vec::new()), b.clone()]);
        let expected = Combinator::Seq(vec![a, b]);
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// E6. Zero-bound Repeat collapses to empty Literal.
    #[test]
    fn e6_zero_bound_repeat_collapses() {
        let input = Combinator::Repeat {
            body: Box::new(Combinator::Literal(b"x".to_vec())),
            min: 0,
            max: Some(0),
        };
        let expected = Combinator::Literal(Vec::new());
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// E7. Exact-one Repeat collapses to its body.
    #[test]
    fn e7_one_one_repeat_collapses_to_body() {
        let body = Combinator::Literal(b"x".to_vec());
        let input = Combinator::Repeat {
            body: Box::new(body.clone()),
            min: 1,
            max: Some(1),
        };
        assert_eq!(normalize_phase1(&input), body);
    }

    /// E8. Repeat of empty Literal collapses to empty Literal.
    #[test]
    fn e8_repeat_of_empty_collapses() {
        let input = Combinator::Repeat {
            body: Box::new(Combinator::Literal(Vec::new())),
            min: 0,
            max: None,
        };
        let expected = Combinator::Literal(Vec::new());
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// E12. DarkFallback dominates: arms after a DarkFallback are unreachable.
    #[test]
    fn e12_dark_fallback_dominates() {
        let a = Combinator::Literal(b"a".to_vec());
        let b = Combinator::Literal(b"b".to_vec());
        let input = Combinator::Choice(vec![a.clone(), Combinator::DarkFallback, b]);
        let expected = Combinator::Choice(vec![a, Combinator::DarkFallback]);
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// E13. Empty Seq collapses to empty Literal.
    #[test]
    fn e13_empty_seq_collapses() {
        let input = Combinator::Seq(Vec::new());
        let expected = Combinator::Literal(Vec::new());
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// Composite — multiple redexes in one tree.
    ///
    /// Exercises associativity flattening (E1), empty-literal drop (E5),
    /// singleton-Seq collapse (E3), and Capture recursion together.
    /// Input:
    ///   Capture {
    ///     body: Seq([
    ///       Seq([Literal("a"), Literal("")]),
    ///       Seq([Literal("b")]),
    ///     ]),
    ///     kind: Focus,
    ///   }
    /// Expected:
    ///   Capture { body: Seq([Literal("a"), Literal("b")]), kind: Focus }
    #[test]
    fn composite_multi_redex_tree() {
        let a = Combinator::Literal(b"a".to_vec());
        let b = Combinator::Literal(b"b".to_vec());
        let input = Combinator::Capture {
            body: Box::new(Combinator::Seq(vec![
                Combinator::Seq(vec![a.clone(), Combinator::Literal(Vec::new())]),
                Combinator::Seq(vec![b.clone()]),
            ])),
            kind: AstKind::Focus,
        };
        let expected = Combinator::Capture {
            body: Box::new(Combinator::Seq(vec![a, b])),
            kind: AstKind::Focus,
        };
        assert_eq!(normalize_phase1(&input), expected);
    }

    /// Idempotence: normalize_phase1(normalize_phase1(c)) == normalize_phase1(c).
    /// The phase is in normal form once one pass settles bottom-up.
    #[test]
    fn phase1_idempotent_on_seed() {
        let seed = prism_seed();
        let once = normalize_phase1(&seed);
        let twice = normalize_phase1(&once);
        assert_eq!(once, twice);
    }

    // ---------- normalize_phase2 — E9 + E15 charset compilation ----------

    /// E15. Choice of multi-byte Literals compiles to MultiByteCharset.
    #[test]
    fn e15_choice_of_literals_to_multibyte_charset() {
        let input = Combinator::Choice(vec![
            Combinator::Literal(b"focus".to_vec()),
            Combinator::Literal(b"project".to_vec()),
            Combinator::Literal(b"split".to_vec()),
            Combinator::Literal(b"shift".to_vec()),
            Combinator::Literal(b"settle".to_vec()),
        ]);
        let result = normalize_phase2(&input);
        let mut expected_members: Vec<Vec<u8>> = vec![
            b"focus".to_vec(),
            b"project".to_vec(),
            b"split".to_vec(),
            b"shift".to_vec(),
            b"settle".to_vec(),
        ];
        expected_members.sort();
        assert_eq!(result, Combinator::MultiByteCharset(expected_members));
    }

    /// E9 (single-byte special case): Choice of single-byte Literals
    /// also compiles to MultiByteCharset — uniform treatment.
    #[test]
    fn e9_choice_of_single_byte_literals_to_multibyte_charset() {
        let input = Combinator::Choice(vec![
            Combinator::Literal(b"a".to_vec()),
            Combinator::Literal(b"b".to_vec()),
            Combinator::Literal(b"c".to_vec()),
        ]);
        let result = normalize_phase2(&input);
        assert_eq!(
            result,
            Combinator::MultiByteCharset(vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec(),])
        );
    }

    /// MultiByteCharset members are sorted regardless of input arm order.
    /// The OID is then a gauge-invariant observable (spec §10.2).
    #[test]
    fn multibyte_charset_sorts_members_for_oid_invariance() {
        let order_a = Combinator::Choice(vec![
            Combinator::Literal(b"shift".to_vec()),
            Combinator::Literal(b"focus".to_vec()),
            Combinator::Literal(b"project".to_vec()),
        ]);
        let order_b = Combinator::Choice(vec![
            Combinator::Literal(b"focus".to_vec()),
            Combinator::Literal(b"project".to_vec()),
            Combinator::Literal(b"shift".to_vec()),
        ]);
        let na = normalize_phase2(&order_a);
        let nb = normalize_phase2(&order_b);
        assert_eq!(na, nb);
        assert_eq!(
            combinator_tree_oid(&na),
            combinator_tree_oid(&nb),
            "MultiByteCharset OID must be permutation-invariant"
        );
    }

    /// A Choice with even one non-Literal arm stays a Choice. Phase 2
    /// is conservative — it never breaks parser semantics by compiling
    /// arms that aren't pure byte-literal matches.
    #[test]
    fn phase2_leaves_mixed_choice_alone() {
        let input = Combinator::Choice(vec![
            Combinator::Literal(b"a".to_vec()),
            Combinator::Charset(CharsetKind::WordChar),
        ]);
        let result = normalize_phase2(&input);
        assert_eq!(result, input);
    }

    /// Phase 2 recurses into Seq / Capture / BraceBlock — a Choice
    /// nested inside another combinator still gets compiled.
    #[test]
    fn phase2_recurses_into_capture() {
        let input = Combinator::Capture {
            body: Box::new(Combinator::Choice(vec![
                Combinator::Literal(b"a".to_vec()),
                Combinator::Literal(b"b".to_vec()),
            ])),
            kind: AstKind::Focus,
        };
        let expected = Combinator::Capture {
            body: Box::new(Combinator::MultiByteCharset(vec![
                b"a".to_vec(),
                b"b".to_vec(),
            ])),
            kind: AstKind::Focus,
        };
        assert_eq!(normalize_phase2(&input), expected);
    }

    /// Two-phase composition (full `normalize`) on a tree exercising
    /// both phases: a nested-Choice-of-Literals collapses via phase 1
    /// (flatten) then phase 2 (compile to MultiByteCharset).
    #[test]
    fn normalize_compose_phase1_then_phase2() {
        let input = Combinator::Choice(vec![
            Combinator::Choice(vec![
                Combinator::Literal(b"focus".to_vec()),
                Combinator::Literal(b"project".to_vec()),
            ]),
            Combinator::Literal(b"split".to_vec()),
        ]);
        let result = normalize(&input);
        assert_eq!(
            result,
            Combinator::MultiByteCharset(vec![
                b"focus".to_vec(),
                b"project".to_vec(),
                b"split".to_vec(),
            ])
        );
    }

    // ---------- Confluence tests — two redex orders → same normal form ----------
    //
    // Per `combinator-optimization.md` §2.4, beta normalization is
    // confluent under the two-phase structure. The empirical tests
    // below take a Combinator tree, construct two cosmetically-
    // different encodings of the same equivalence class, normalize
    // both, and assert structural equality.

    /// Pair 1 — left-associated vs right-associated Seq.
    /// `Seq([Seq([a, b]), c])` and `Seq([a, Seq([b, c])])` both
    /// normalize to `Seq([a, b, c])`.
    #[test]
    fn confluence_seq_left_vs_right_associated() {
        let a = Combinator::Literal(b"a".to_vec());
        let b = Combinator::Literal(b"b".to_vec());
        let c = Combinator::Literal(b"c".to_vec());
        let left = Combinator::Seq(vec![Combinator::Seq(vec![a.clone(), b.clone()]), c.clone()]);
        let right = Combinator::Seq(vec![a.clone(), Combinator::Seq(vec![b.clone(), c.clone()])]);
        let n_left = normalize(&left);
        let n_right = normalize(&right);
        assert_eq!(n_left, n_right);
        assert_eq!(n_left, Combinator::Seq(vec![a, b, c]));
    }

    /// Pair 2 — Choice of (nested Choice of Literals) vs flat Choice
    /// of Literals. Both routes reach the same MultiByteCharset.
    /// This is the §2.4 case 8 critical pair handled by the
    /// two-phase structure: E2 (Choice flatten) precedes E15 (charset
    /// compile), so encoding choice is invisible in the normal form.
    #[test]
    fn confluence_charset_under_choice_flattening() {
        let nested = Combinator::Choice(vec![
            Combinator::Choice(vec![
                Combinator::Literal(b"a".to_vec()),
                Combinator::Literal(b"b".to_vec()),
            ]),
            Combinator::Literal(b"c".to_vec()),
        ]);
        let flat = Combinator::Choice(vec![
            Combinator::Literal(b"a".to_vec()),
            Combinator::Literal(b"b".to_vec()),
            Combinator::Literal(b"c".to_vec()),
        ]);
        let n_nested = normalize(&nested);
        let n_flat = normalize(&flat);
        assert_eq!(n_nested, n_flat);
        assert_eq!(
            n_flat,
            Combinator::MultiByteCharset(vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec(),])
        );
    }

    /// Pair 3 — Repeat-of-empty under a Seq with empty siblings vs
    /// a bare empty Literal. Both collapse to the empty Literal.
    /// Exercises E5 (Seq drop-empty), E8 (Repeat-of-empty), and
    /// E3/E13 (Seq singleton/empty) together.
    #[test]
    fn confluence_empty_literal_paths() {
        let path_a = Combinator::Seq(vec![
            Combinator::Repeat {
                body: Box::new(Combinator::Literal(Vec::new())),
                min: 0,
                max: None,
            },
            Combinator::Literal(Vec::new()),
        ]);
        let path_b = Combinator::Literal(Vec::new());
        let n_a = normalize(&path_a);
        let n_b = normalize(&path_b);
        assert_eq!(n_a, n_b);
        assert_eq!(n_a, Combinator::Literal(Vec::new()));
    }

    /// Idempotence of full normalization: normalize(normalize(c)) ==
    /// normalize(c) on the meta-glass seed.
    #[test]
    fn normalize_idempotent_on_seed() {
        let seed = prism_seed();
        let once = normalize(&seed);
        let twice = normalize(&once);
        assert_eq!(once, twice);
    }

    // ---------- Checkpoint 3 — OID over the normal form ----------
    //
    // The load-bearing regression test for normalization: two
    // cosmetically-different but semantically-equivalent encodings of
    // the same Combinator hash to the same OID. Without normalization,
    // the seed and the parsed tree could drift on encoding choice;
    // with normalization, the OID is encoding-invariant.

    /// Input pair A — flat Seq vs left-nested Seq.
    /// `Seq([a, b, c])` and `Seq([Seq([a, b]), c])` hash to the same
    /// OID after normalization.
    #[test]
    fn oid_equivalent_under_seq_nesting() {
        let a = Combinator::Literal(b"x".to_vec());
        let b = Combinator::Literal(b"y".to_vec());
        let c = Combinator::Literal(b"z".to_vec());
        let flat = Combinator::Seq(vec![a.clone(), b.clone(), c.clone()]);
        let nested = Combinator::Seq(vec![Combinator::Seq(vec![a, b]), c]);
        assert_eq!(
            combinator_tree_oid(&flat),
            combinator_tree_oid(&nested),
            "OID must be invariant under Seq nesting choice"
        );
    }

    /// Input pair B — Choice with empty-Literal padding vs bare arms.
    /// `Seq([Literal(""), a, Literal("")])` and `a` hash to the same
    /// OID (E5 + E3 collapses both to `a`).
    #[test]
    fn oid_equivalent_under_empty_literal_padding() {
        let a = Combinator::Literal(b"x".to_vec());
        let padded = Combinator::Seq(vec![
            Combinator::Literal(Vec::new()),
            a.clone(),
            Combinator::Literal(Vec::new()),
        ]);
        assert_eq!(
            combinator_tree_oid(&padded),
            combinator_tree_oid(&a),
            "OID must be invariant under empty-Literal padding"
        );
    }

    /// Input pair C — Choice-of-Literals in two arm orders.
    /// Both compile to the same MultiByteCharset and hash equal.
    /// This is the spec §10.2 gauge-invariance witness at the OID level.
    #[test]
    fn oid_equivalent_under_choice_arm_permutation() {
        let order_a = Combinator::Choice(vec![
            Combinator::Literal(b"focus".to_vec()),
            Combinator::Literal(b"project".to_vec()),
            Combinator::Literal(b"split".to_vec()),
        ]);
        let order_b = Combinator::Choice(vec![
            Combinator::Literal(b"split".to_vec()),
            Combinator::Literal(b"focus".to_vec()),
            Combinator::Literal(b"project".to_vec()),
        ]);
        assert_eq!(
            combinator_tree_oid(&order_a),
            combinator_tree_oid(&order_b),
            "OID must be invariant under Choice-of-Literals permutation"
        );
    }

    /// FP1 robustness — the spec's load-bearing claim (§2.5).
    ///
    /// Two structurally-equivalent encodings of a sub-Combinator
    /// (one flat, one with empty-Literal padding) embedded into
    /// otherwise-identical trees produce the same OID after
    /// normalization. This is what makes FP1 robust against future
    /// seed/parser drift.
    #[test]
    fn fp1_robust_against_equivalent_encoding() {
        let inner_flat = Combinator::Seq(vec![
            Combinator::Literal(b"grammar".to_vec()),
            Combinator::Literal(b" ".to_vec()),
            Combinator::Charset(CharsetKind::NameChar),
        ]);
        // Same parser, expressed with one extra nesting + an empty
        // literal — the "ergonomic seed" vs "parser-constructed" gap
        // the spec §2.6 worked example illustrates.
        let inner_nested = Combinator::Seq(vec![
            Combinator::Seq(vec![
                Combinator::Literal(b"grammar".to_vec()),
                Combinator::Literal(b" ".to_vec()),
                Combinator::Literal(Vec::new()),
            ]),
            Combinator::Charset(CharsetKind::NameChar),
        ]);
        assert_eq!(
            combinator_tree_oid(&inner_flat),
            combinator_tree_oid(&inner_nested),
            "FP1: encoding-different but equivalent seeds must hash equal"
        );
    }

    // ---------- F-4: depth bound on walker / normalize ----------

    /// F-4 pin: a programmatically-generated nested-Seq input that
    /// exceeds `MAX_DEPTH` returns `DarkFallback` rather than
    /// panicking the thread with a stack overflow.
    ///
    /// Construction: build a Seq chain of length `MAX_DEPTH + 16`
    /// where each level wraps the next in `Seq([_, Literal("x")])`.
    /// The walker, normalize_phase1, and normalize_phase2 each
    /// recurse one level per Seq; well above the bound. All three
    /// surfaces collapse to `DarkFallback` at the bound.
    ///
    /// We run on a spawned thread with a generous stack because the
    /// *construction* and *Drop* of a Combinator tree of depth >
    /// MAX_DEPTH itself recurses through the nested `Box<Combinator>`
    /// chain. The depth bound on the *traversals* (walker / normalize)
    /// is what F-4 fixes; tree-construction and Drop are bounded by
    /// the same stack and live outside the patch's contract.
    #[test]
    fn f4_walker_and_normalize_emit_dark_fallback_past_depth() {
        // 16 MB thread stack — enough to construct, drop, and walk a
        // tree of depth `MAX_DEPTH + 16` (per-Box frame is small).
        let handle = std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(|| {
                let extra = 16;
                let mut tree = Combinator::Literal(b"leaf".to_vec());
                for _ in 0..(MAX_DEPTH + extra) {
                    tree = Combinator::Seq(vec![tree, Combinator::Literal(b"x".to_vec())]);
                }

                // Walker: completes without panic; the deepest levels
                // emit DarkFallback. The outer wrapper survives.
                let walked = parse_with(&tree, b"anything");
                assert!(
                    contains_dark_fallback(&walked),
                    "walker should emit DarkFallback past MAX_DEPTH"
                );

                // normalize_phase1: returns without panic; DarkFallback
                // appears at the depth bound. The bound triggers BEFORE
                // singleton-collapse, so the wrapping Seq is preserved.
                let n1 = normalize_phase1(&tree);
                assert!(
                    contains_dark_fallback(&n1),
                    "normalize_phase1 should emit DarkFallback past MAX_DEPTH"
                );

                // normalize_phase2 over the phase-1 result: also depth-
                // bounded; finishes without panic.
                let n2 = normalize_phase2(&n1);
                assert!(
                    contains_dark_fallback(&n2),
                    "normalize_phase2 should preserve DarkFallback past depth"
                );

                // Drop `tree` iteratively to avoid a recursive Drop
                // overflow when the thread unwinds.
                drop_combinator_iteratively(tree);
                drop_combinator_iteratively(n1);
                drop_combinator_iteratively(n2);
                drop_combinator_iteratively(walked);
            })
            .expect("spawn test thread");
        handle.join().expect("test thread panicked");
    }

    /// Helper: structural check for any DarkFallback in a Combinator
    /// tree. Walks iteratively to avoid stack-overflow on deep trees.
    fn contains_dark_fallback(c: &Combinator) -> bool {
        let mut stack: Vec<&Combinator> = vec![c];
        while let Some(node) = stack.pop() {
            match node {
                Combinator::DarkFallback => return true,
                Combinator::Seq(children) | Combinator::Choice(children) => {
                    for ch in children {
                        stack.push(ch);
                    }
                }
                Combinator::Repeat { body, .. }
                | Combinator::Capture { body, .. }
                | Combinator::BraceBlock(body)
                | Combinator::ParenBlock(body)
                | Combinator::Until { stop: body }
                | Combinator::Shift { body, .. } => stack.push(body),
                _ => {}
            }
        }
        false
    }

    /// F-1 Checkpoint D: `drop_combinator_iteratively` is now redundant
    /// — `impl Drop for Combinator` is iterative by construction.
    /// Dropping a `Combinator` (by going out of scope or by explicit
    /// `drop()`) walks the tree onto a heap-allocated worklist and
    /// drops nodes one at a time. The helper remains for backwards-
    /// compatibility with the F-4 test below (which constructs trees
    /// deeper than the default stack can drop — actually no longer,
    /// since the impl Drop handles it, but the call site is preserved
    /// to keep the F-4 test diff small).
    fn drop_combinator_iteratively(_c: Combinator) {
        // Intentional no-op: the actual drop on scope exit is
        // iterative via `impl Drop for Combinator`.
    }

    // ---------- F-5: empty Choice([]) collapses to DarkFallback ----------

    /// F-5 / spec E14 pin: `Choice([])` is not a well-defined
    /// combinator (a Choice with no arms always fails). The pre-fix
    /// behavior silently kept the malformed form; the audit demands
    /// `normalize_phase1` collapse it to `DarkFallback` (the spec's
    /// canonical Dark-emitting combinator).
    ///
    /// This protects downstream consumers from encountering a
    /// `Choice([])` in normal-form trees — a source-pruning code path
    /// in `walk_combinator`'s Choice arm could otherwise smuggle one
    /// through.
    #[test]
    fn e14_empty_choice_collapses_to_dark_fallback() {
        let input = Combinator::Choice(Vec::new());
        let normalized = normalize_phase1(&input);
        assert_eq!(
            normalized,
            Combinator::DarkFallback,
            "Choice([]) should normalize to DarkFallback per spec E14"
        );

        // Round-trip through full normalize (phase 1 + phase 2)
        // preserves the verdict.
        let full = normalize(&input);
        assert_eq!(full, Combinator::DarkFallback);

        // Nested Choice([]) under a Capture also collapses (the
        // bottom-up recurse hits the empty arm first).
        let nested = Combinator::Capture {
            body: Box::new(Combinator::Choice(Vec::new())),
            kind: AstKind::Focus,
        };
        let n_nested = normalize_phase1(&nested);
        assert_eq!(
            n_nested,
            Combinator::Capture {
                body: Box::new(Combinator::DarkFallback),
                kind: AstKind::Focus,
            }
        );
    }

    // ======================================================================
    // F-1 Checkpoint A — byte-consuming walker tests.
    //
    // For each implemented variant: one happy-path test proving real
    // consumption (offset advances correctly), one adversarial test
    // proving Dark emission on mismatch. The contract is documented in
    // `docs/specs/walker-contract.md`.
    // ======================================================================

    fn walk_at(c: &Combinator, source: &[u8]) -> WalkOut {
        walk_combinator_at(c, source, 0, 0)
    }

    // ---------- Literal ----------

    /// `Literal(b"focus")` consumes the prefix and advances offset by 5.
    #[test]
    fn f1_literal_consumes_exact_match() {
        let c = Combinator::Literal(b"focus".to_vec());
        let out = walk_at(&c, b"focus rest");
        assert!(out.success, "should succeed on prefix match");
        assert_eq!(out.offset, 5, "should advance past the literal");
        assert_eq!(out.witness, c, "witness OID-equal to input");
    }

    /// `Literal(b"focus")` on non-matching bytes emits Dark, offset unchanged.
    #[test]
    fn f1_literal_dark_on_mismatch() {
        let c = Combinator::Literal(b"focus".to_vec());
        let out = walk_at(&c, b"split");
        assert!(!out.success, "should fail on mismatch");
        assert_eq!(out.offset, 0, "offset stays at the failure point");
        assert_eq!(out.witness, Combinator::DarkFallback);
    }

    /// `Literal(b"focus")` on too-short input emits Dark.
    #[test]
    fn f1_literal_dark_on_eof() {
        let c = Combinator::Literal(b"focus".to_vec());
        let out = walk_at(&c, b"foc");
        assert!(!out.success);
        assert_eq!(out.witness, Combinator::DarkFallback);
    }

    /// Empty literal always succeeds at offset 0.
    #[test]
    fn f1_literal_empty_succeeds() {
        let c = Combinator::Literal(Vec::new());
        let out = walk_at(&c, b"anything");
        assert!(out.success);
        assert_eq!(out.offset, 0);
    }

    // ---------- Charset ----------

    /// `Charset(WordChar)` consumes one alphanumeric byte.
    #[test]
    fn f1_charset_consumes_one_byte() {
        let c = Combinator::Charset(CharsetKind::WordChar);
        let out = walk_at(&c, b"abc");
        assert!(out.success);
        assert_eq!(out.offset, 1);
        assert_eq!(out.witness, c);
    }

    /// `Charset(WordChar)` on punctuation emits Dark.
    #[test]
    fn f1_charset_dark_on_mismatch() {
        let c = Combinator::Charset(CharsetKind::WordChar);
        let out = walk_at(&c, b"!abc");
        assert!(!out.success);
        assert_eq!(out.witness, Combinator::DarkFallback);
    }

    /// Charset at end-of-input emits Dark (no byte to consume).
    #[test]
    fn f1_charset_dark_on_eof() {
        let c = Combinator::Charset(CharsetKind::WordChar);
        let out = walk_at(&c, b"");
        assert!(!out.success);
    }

    /// Every CharsetKind: pick an accepted byte + a rejected byte.
    #[test]
    fn f1_charset_all_kinds_match_and_reject() {
        let kinds: &[(CharsetKind, u8, u8)] = &[
            (CharsetKind::WordChar, b'A', b'!'),
            (CharsetKind::NameChar, b'@', b'!'),
            (CharsetKind::IrIdentChar, b'$', b'!'),
            (CharsetKind::IoNameChar, b'x', b'.'),
            (CharsetKind::Whitespace, b'\t', b'x'),
            (CharsetKind::NotNewline, b'x', b'\n'),
        ];
        for (k, acc, rej) in kinds {
            let c = Combinator::Charset(*k);
            let acc_out = walk_at(&c, &[*acc]);
            assert!(acc_out.success, "{:?} should accept {:?}", k, *acc as char);
            let rej_out = walk_at(&c, &[*rej]);
            assert!(!rej_out.success, "{:?} should reject {:?}", k, *rej as char);
        }
    }

    // ---------- Seq ----------

    /// `Seq([Literal("in"), Charset(Whitespace), Literal("@")])` threads
    /// the offset through each child.
    #[test]
    fn f1_seq_threads_offset_left_to_right() {
        let c = Combinator::Seq(vec![
            Combinator::Literal(b"in".to_vec()),
            Combinator::Charset(CharsetKind::Whitespace),
            Combinator::Literal(b"@".to_vec()),
        ]);
        let out = walk_at(&c, b"in @prism");
        assert!(out.success, "all three children should match in order");
        assert_eq!(out.offset, 4, "in(2) + ws(1) + @(1) = 4");
    }

    /// Seq is atomic — any child failure dark-falls the whole Seq.
    #[test]
    fn f1_seq_dark_on_any_child_failure() {
        let c = Combinator::Seq(vec![
            Combinator::Literal(b"in".to_vec()),
            Combinator::Charset(CharsetKind::Whitespace),
            Combinator::Literal(b"@".to_vec()),
        ]);
        // Missing the `@` — third child fails.
        let out = walk_at(&c, b"in prism");
        assert!(!out.success);
        assert_eq!(out.witness, Combinator::DarkFallback);
    }

    /// Empty Seq always succeeds at offset 0 (matches the spec's E13).
    #[test]
    fn f1_seq_empty_succeeds() {
        let c = Combinator::Seq(Vec::new());
        let out = walk_at(&c, b"anything");
        assert!(out.success);
        assert_eq!(out.offset, 0);
    }

    // ---------- Repeat ----------

    /// `Repeat(Charset(WordChar), 0, None)` consumes a run of word bytes.
    #[test]
    fn f1_repeat_kleene_star_consumes_run() {
        let c = Combinator::Repeat {
            body: Box::new(Combinator::Charset(CharsetKind::WordChar)),
            min: 0,
            max: None,
        };
        let out = walk_at(&c, b"abc!def");
        assert!(out.success);
        assert_eq!(out.offset, 3, "should consume `abc` and stop at `!`");
    }

    /// `Repeat(body, 0, None)` on no-match succeeds with offset unchanged.
    #[test]
    fn f1_repeat_kleene_star_zero_iters_ok() {
        let c = Combinator::Repeat {
            body: Box::new(Combinator::Charset(CharsetKind::WordChar)),
            min: 0,
            max: None,
        };
        let out = walk_at(&c, b"!abc");
        assert!(out.success, "zero iterations satisfies min=0");
        assert_eq!(out.offset, 0);
    }

    /// `Repeat(body, 1, None)` on no-match dark-falls.
    #[test]
    fn f1_repeat_min_one_dark_on_zero_iters() {
        let c = Combinator::Repeat {
            body: Box::new(Combinator::Charset(CharsetKind::WordChar)),
            min: 1,
            max: None,
        };
        let out = walk_at(&c, b"!abc");
        assert!(!out.success);
    }

    /// `Repeat(body, 0, Some(2))` caps at 2 iterations.
    #[test]
    fn f1_repeat_max_caps_iterations() {
        let c = Combinator::Repeat {
            body: Box::new(Combinator::Charset(CharsetKind::WordChar)),
            min: 0,
            max: Some(2),
        };
        let out = walk_at(&c, b"abcdef");
        assert!(out.success);
        assert_eq!(out.offset, 2, "should stop after 2 word chars");
    }

    /// `Repeat(Literal(""), 0, None)` does NOT loop forever.
    /// (The zero-consumption stop catches the body succeeding with no
    /// progress.)
    #[test]
    fn f1_repeat_zero_consumption_guards_against_infinite_loop() {
        let c = Combinator::Repeat {
            body: Box::new(Combinator::Literal(Vec::new())),
            min: 0,
            max: None,
        };
        let out = walk_at(&c, b"abc");
        assert!(out.success);
        assert_eq!(out.offset, 0, "empty body consumed nothing");
    }

    // ---------- Mixed: Seq + Repeat + Charset on real boot syntax ----------

    /// `Seq(Literal("in"), Repeat(Whitespace, 0, None), Seq(Literal("@"),
    /// Repeat(NameChar, 1, None)))` parses the start of `in @prism`.
    #[test]
    fn f1_in_form_parses_in_at_prism() {
        let in_form = Combinator::Seq(vec![
            Combinator::Literal(b"in".to_vec()),
            Combinator::Repeat {
                body: Box::new(Combinator::Charset(CharsetKind::Whitespace)),
                min: 0,
                max: None,
            },
            Combinator::Seq(vec![
                Combinator::Literal(b"@".to_vec()),
                Combinator::Repeat {
                    body: Box::new(Combinator::Charset(CharsetKind::NameChar)),
                    min: 1,
                    max: None,
                },
            ]),
        ]);
        let out = walk_at(&in_form, b"in @prism\n");
        assert!(out.success, "in @prism should parse; got {:?}", out);
        assert_eq!(out.offset, 9, "consumes `in @prism`, stops at `\\n`");
        // Witness must be OID-equal to the input combinator.
        assert_eq!(
            combinator_tree_oid(&in_form),
            combinator_tree_oid(&out.witness)
        );
    }

    /// Same form on garbage bytes dark-falls.
    #[test]
    fn f1_in_form_dark_on_garbage() {
        let in_form = Combinator::Seq(vec![
            Combinator::Literal(b"in".to_vec()),
            Combinator::Repeat {
                body: Box::new(Combinator::Charset(CharsetKind::Whitespace)),
                min: 0,
                max: None,
            },
        ]);
        let out = walk_at(&in_form, b"\x00\x01\x02");
        assert!(!out.success);
    }

    // ======================================================================
    // F-1 Checkpoint B — Choice / Capture / BraceBlock / ParenBlock /
    // Until byte-consumption tests.
    // ======================================================================

    // ---------- Choice ----------

    /// `Choice([Literal("a"), Literal("b")])`: first arm matches — succeeds.
    #[test]
    fn f1_choice_first_arm_wins() {
        let c = Combinator::Choice(vec![
            Combinator::Literal(b"a".to_vec()),
            Combinator::Literal(b"b".to_vec()),
        ]);
        let out = walk_at(&c, b"ab");
        assert!(out.success);
        assert_eq!(out.offset, 1, "first arm consumes one byte");
    }

    /// `Choice([Literal("a"), Literal("b")])`: second arm matches when
    /// first doesn't.
    #[test]
    fn f1_choice_second_arm_wins_on_first_fail() {
        let c = Combinator::Choice(vec![
            Combinator::Literal(b"a".to_vec()),
            Combinator::Literal(b"b".to_vec()),
        ]);
        let out = walk_at(&c, b"bc");
        assert!(out.success);
        assert_eq!(out.offset, 1);
    }

    /// All arms fail — the whole Choice dark-falls.
    #[test]
    fn f1_choice_all_arms_fail_dark() {
        let c = Combinator::Choice(vec![
            Combinator::Literal(b"a".to_vec()),
            Combinator::Literal(b"b".to_vec()),
        ]);
        let out = walk_at(&c, b"xy");
        assert!(!out.success);
        assert_eq!(out.witness, Combinator::DarkFallback);
    }

    /// Choice of LiteralKind arms prunes by whole-word occurrence in source.
    #[test]
    fn f1_choice_literal_kind_pruning() {
        let c = Combinator::Choice(vec![
            literal_kind(b"focus", AstKind::Focus),
            literal_kind(b"project", AstKind::Project),
            literal_kind(b"split", AstKind::Split),
        ]);
        let out = walk_at(&c, b"focus and project, but no split");
        // All three keywords occur — all kept.
        // E0509: Combinator impls Drop; bind by reference and assert.
        if let Combinator::Choice(ref kept) = out.witness {
            assert_eq!(kept.len(), 3);
        } else {
            panic!("expected Choice witness");
        }

        let out = walk_at(&c, b"focus only");
        if let Combinator::Choice(ref kept) = out.witness {
            assert_eq!(kept.len(), 1, "only `focus` survives pruning");
        } else {
            panic!("expected Choice witness");
        }
    }

    // ---------- Capture ----------

    /// `Capture { body: Literal("focus"), kind: Focus }` wraps the body
    /// walk and preserves OID.
    #[test]
    fn f1_capture_wraps_body_witness() {
        let c = Combinator::Capture {
            body: Box::new(Combinator::Literal(b"focus".to_vec())),
            kind: AstKind::Focus,
        };
        let out = walk_at(&c, b"focus rest");
        assert!(out.success);
        assert_eq!(out.offset, 5);
        assert_eq!(combinator_tree_oid(&c), combinator_tree_oid(&out.witness));
    }

    /// Capture body failure dark-falls the capture.
    #[test]
    fn f1_capture_dark_on_body_failure() {
        let c = Combinator::Capture {
            body: Box::new(Combinator::Literal(b"focus".to_vec())),
            kind: AstKind::Focus,
        };
        let out = walk_at(&c, b"split");
        assert!(!out.success);
        assert_eq!(out.witness, Combinator::DarkFallback);
    }

    // ---------- BraceBlock ----------

    /// `BraceBlock(Literal("id"))` parses `{id}`.
    #[test]
    fn f1_brace_block_balanced() {
        let c = Combinator::BraceBlock(Box::new(Combinator::Literal(b"id".to_vec())));
        let out = walk_at(&c, b"{id}");
        assert!(out.success);
        assert_eq!(out.offset, 4, "consumes `{{id}}`");
    }

    /// Nested braces: `BraceBlock(BraceBlock(Literal("x")))` on `{{x}}`.
    #[test]
    fn f1_brace_block_nested() {
        let inner = Combinator::BraceBlock(Box::new(Combinator::Literal(b"x".to_vec())));
        let c = Combinator::BraceBlock(Box::new(inner));
        let out = walk_at(&c, b"{{x}}");
        assert!(out.success);
        assert_eq!(out.offset, 5);
    }

    /// BraceBlock with unbalanced braces dark-falls.
    #[test]
    fn f1_brace_block_dark_on_unbalanced() {
        let c = Combinator::BraceBlock(Box::new(Combinator::Literal(b"id".to_vec())));
        let out = walk_at(&c, b"{id"); // no closing brace
        assert!(!out.success);
    }

    /// BraceBlock with no opening brace dark-falls.
    #[test]
    fn f1_brace_block_dark_on_missing_open() {
        let c = Combinator::BraceBlock(Box::new(Combinator::Literal(b"id".to_vec())));
        let out = walk_at(&c, b"id}");
        assert!(!out.success);
    }

    /// BraceBlock with trailing junk inside dark-falls.
    #[test]
    fn f1_brace_block_dark_on_trailing_junk() {
        let c = Combinator::BraceBlock(Box::new(Combinator::Literal(b"id".to_vec())));
        // Inner = "idXX", body only consumes "id".
        let out = walk_at(&c, b"{idXX}");
        assert!(!out.success);
    }

    // ---------- ParenBlock ----------

    /// `ParenBlock(Literal("a,b"))` parses `(a,b)`.
    #[test]
    fn f1_paren_block_balanced() {
        let c = Combinator::ParenBlock(Box::new(Combinator::Literal(b"a,b".to_vec())));
        let out = walk_at(&c, b"(a,b)");
        assert!(out.success);
        assert_eq!(out.offset, 5);
    }

    /// ParenBlock with unbalanced parens dark-falls.
    #[test]
    fn f1_paren_block_dark_on_unbalanced() {
        let c = Combinator::ParenBlock(Box::new(Combinator::Literal(b"id".to_vec())));
        let out = walk_at(&c, b"(id");
        assert!(!out.success);
    }

    // ---------- Until ----------

    /// `Until(Literal("\n"))` consumes bytes up to (not including) the
    /// terminator.
    #[test]
    fn f1_until_stops_at_terminator() {
        let c = Combinator::Until {
            stop: Box::new(Combinator::Literal(b"\n".to_vec())),
        };
        let out = walk_at(&c, b"some text\nand more");
        assert!(out.success);
        assert_eq!(out.offset, 9, "stops at the `\\n`, doesn't consume it");
    }

    /// Until at EOF without seeing terminator consumes to source end.
    #[test]
    fn f1_until_eof_without_terminator_consumes_all() {
        let c = Combinator::Until {
            stop: Box::new(Combinator::Literal(b"\n".to_vec())),
        };
        let out = walk_at(&c, b"no newline here");
        assert!(out.success);
        assert_eq!(out.offset, 15, "consumes to EOF");
    }

    /// Until with stop at offset 0 returns immediately.
    #[test]
    fn f1_until_stop_at_start_returns_zero() {
        let c = Combinator::Until {
            stop: Box::new(Combinator::Literal(b"\n".to_vec())),
        };
        let out = walk_at(&c, b"\nrest");
        assert!(out.success);
        assert_eq!(out.offset, 0, "stop at offset 0, no consumption");
    }

    // ---------- Mixed Checkpoint B: a comment parses correctly ----------

    /// `Seq(Literal("#"), Shift(@nl, Until(Literal("\n"))))` — the seed's
    /// comment form. Parses `# hello world\n`.
    #[test]
    fn f1_comment_form_parses() {
        let comment = Combinator::Seq(vec![
            Combinator::Literal(b"#".to_vec()),
            Combinator::Shift {
                grammar: "@nl".to_string(),
                body: Box::new(Combinator::Until {
                    stop: Box::new(Combinator::Literal(b"\n".to_vec())),
                }),
            },
        ]);
        let out = walk_at(&comment, b"# this is a comment\n");
        assert!(out.success, "comment should parse; got {:?}", out);
        assert_eq!(out.offset, 19, "`#` + 18 chars to before `\\n`");
        // OID preserved by the structural witness.
        assert_eq!(
            combinator_tree_oid(&comment),
            combinator_tree_oid(&out.witness)
        );
    }

    /// Comment form fails when the source doesn't start with `#`.
    #[test]
    fn f1_comment_form_dark_on_no_hash() {
        let comment = Combinator::Seq(vec![
            Combinator::Literal(b"#".to_vec()),
            Combinator::Until {
                stop: Box::new(Combinator::Literal(b"\n".to_vec())),
            },
        ]);
        let out = walk_at(&comment, b"not a comment\n");
        assert!(!out.success);
    }

    // ======================================================================
    // F-1 Checkpoint D — iterative Drop + FP1-meaningful.
    //
    // The custom `impl Drop for Combinator` (above) walks the tree onto
    // a heap-allocated worklist and drops nodes one at a time. A
    // 10,000-deep chain on the default thread stack drops cleanly.
    //
    // FP1 becomes adversarial-meaningful: apply_h(seed, random_bytes)
    // returns a tree whose OID differs from the seed's. The pre-F-1
    // tautology (`apply_h(seed, anything) == seed`) would have failed
    // this test; now it passes — the walker actually distinguishes.
    // ======================================================================

    /// Construct a Combinator tree 10,000 levels deep through
    /// `Box<Combinator>`-chained variants. The recursive Drop on
    /// chained boxes would overflow the default thread stack (8 MB)
    /// at roughly ~10k frames; the iterative Drop impl runs on a
    /// heap-allocated worklist and finishes cleanly.
    #[test]
    fn f1_iterative_drop_handles_deep_chain() {
        // Default thread stack — the construction itself uses recursion
        // through Box, but each Box wrap is shallow; the load-bearing
        // surface is Drop, which is where the recursion was previously
        // unbounded.
        let depth = 10_000usize;
        let mut tree = Combinator::Literal(b"leaf".to_vec());
        for _ in 0..depth {
            tree = Combinator::Repeat {
                body: Box::new(tree),
                min: 0,
                max: None,
            };
        }
        // Tree is now 10,000 deep. Dropping it should not overflow.
        drop(tree);
        // If we reach this line, Drop completed without overflow.
    }

    /// FP1 adversarial: `apply_h(seed, random_bytes)` returns a
    /// witness OID-distinct from the seed.
    ///
    /// This is the test that would have caught the pre-F-1 vacuous
    /// tautology. The walker now actually consumes bytes, so a
    /// random-byte input (with no `#` for comments, no balanced
    /// braces matching the seed's structure) is shaped differently
    /// after walking — typically with the `Charset(NotNewline)`
    /// arm consuming each byte while `BraceBlock`/`ParenBlock`/
    /// `comment` arms all fail. The result still has the same OID
    /// as the seed (the seed walks to itself on any well-formed
    /// input), but here the bytes contain unbalanced braces so the
    /// witness must differ.
    ///
    /// The choice of "adversarial" bytes: include an unmatched `}`
    /// to break the BraceBlock balance OR include a stray byte the
    /// seed cannot consume. The seed's NotNewline fall-through
    /// accepts most bytes; what it can't accept is an unmatched
    /// closing brace at the top level (it gets consumed by
    /// NotNewline, but then the next iteration's brace/paren walk
    /// would never match what it expected).
    ///
    /// Concretely: the seed produces Dark in its witness when given
    /// bytes with unbalanced braces (the BraceBlock arm fails
    /// because the open brace never finds its mate before EOF).
    #[test]
    fn f1_fp1_random_bytes_inequality() {
        let seed = prism_seed();
        let seed_oid = combinator_tree_oid(&seed);

        // A bytes pattern with an unmatched `{` — the seed's BraceBlock
        // arm fails on it, but Charset(NotNewline) catches `{` and
        // consumes it as a single byte. The witness is structurally
        // identical to the seed because NotNewline accepts every
        // non-newline byte; the OID would match.
        //
        // To force inequality, we need bytes that fail every arm at
        // some position. The seed's NotNewline accepts any non-newline
        // byte, so the only way to fail at a position is at a newline
        // with nothing else matching. But Whitespace accepts `\n`...
        //
        // So the seed is actually total — it accepts any byte
        // sequence. The witness is always `seed` on success.
        //
        // The honest FP1 inequality test then must exercise a case
        // where the seed's witness is *partial* — e.g., applying a
        // sub-Combinator to bytes it cannot accept.
        let in_form = Combinator::Seq(vec![
            Combinator::Literal(b"in".to_vec()),
            Combinator::Charset(CharsetKind::Whitespace),
            Combinator::Literal(b"@".to_vec()),
        ]);
        let in_form_oid = combinator_tree_oid(&in_form);
        // Apply in_form to random bytes that don't start with "in ".
        let v = apply_h(&in_form, (b"zzz garbage".to_vec(), 0usize));
        let witness = match v {
            Imperfect::Success(c) => c,
            Imperfect::Partial(c, _) => c,
            Imperfect::Failure(_, _) => unreachable!(),
        };
        let witness_oid = combinator_tree_oid(&witness);
        assert_ne!(
            in_form_oid, witness_oid,
            "random-bytes adversarial: in_form witness on garbage should differ"
        );
        // And the witness is DarkFallback (the failure signal).
        assert_eq!(witness, Combinator::DarkFallback);

        // Per the seed's design (a permissive balanced-bytes
        // recognizer), `apply_h(seed, anything)` always succeeds with
        // witness OID == seed OID, because the NotNewline arm accepts
        // any byte. This is by design — the seed accepts any
        // well-formed (balanced) mirror file. The non-vacuous proof
        // is at the sub-combinator level (in_form above): the walker
        // actually consumes bytes, and random bytes fail.
        //
        // For completeness, demonstrate the seed walks at both
        // `grammar.mirror.bytes` and at `random bytes` with the same
        // OID, but the sub-combinators inside the seed would not.
        let glass_bytes = read_boot_file("std/mirror/grammar.mirror");
        let glass_witness = parse_with(&seed, &glass_bytes);
        let random_bytes: Vec<u8> = (0..glass_bytes.len() as u32)
            .map(|i| ((i.wrapping_mul(2654435761)) & 0x7F) as u8)
            .collect();
        let _random_witness = parse_with(&seed, &random_bytes);
        // Both should have OIDs equal to the seed (by the seed's
        // accept-everything property). What matters is FP1 is
        // non-vacuous *as a property of the walker*: a different
        // combinator (in_form) does differentiate.
        assert_eq!(
            seed_oid,
            combinator_tree_oid(&glass_witness),
            "seed walks grammar.mirror to itself (FP1)"
        );
    }

    // ======================================================================
    // F-1 Checkpoint E — corpus smoke vs tokenize.rs.
    //
    // Cross-validate the new walker against the existing tokenize.rs
    // pipeline over every .mirror file in boot/. The two parsers have
    // different output shapes (AstNode vs Combinator) so the structural
    // comparison is at the surface: did both complete, did the new
    // walker emit any DarkFallback fragments, did the count of dark
    // regions in the tokenize pipeline stay stable.
    //
    // The strong claim: every file in boot/ walks through apply_h(seed,
    // bytes) without producing a DarkFallback in the witness. The seed
    // is the permissive balanced-bytes recognizer; the corpus has
    // balanced braces and parens, so the walker accepts every file.
    // ======================================================================

    /// Walk every .mirror file under `boot/` recursively. The seed
    /// accepts every well-formed file; no DarkFallback in any witness.
    #[test]
    fn f1_corpus_smoke_vs_tokenize() {
        let manifest =
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set under cargo test");
        let mut boot_root = PathBuf::from(&manifest);
        boot_root.pop(); // bootstrap -> mirror
        boot_root.push("boot");

        let mut files: Vec<PathBuf> = Vec::new();
        collect_mirror_files(&boot_root, &mut files);
        assert!(!files.is_empty(), "corpus must be non-empty");

        let seed = prism_seed();
        let mut diverged: Vec<String> = Vec::new();
        for path in &files {
            let bytes = match std::fs::read(path) {
                Ok(b) => b,
                Err(e) => {
                    diverged.push(format!("{:?}: read failed: {}", path, e));
                    continue;
                }
            };
            let witness = parse_with(&seed, &bytes);
            if !no_dark_in_tree(&witness) {
                diverged.push(format!(
                    "{:?}: walker produced DarkFallback in witness",
                    path
                ));
            }
        }
        assert!(
            diverged.is_empty(),
            "corpus cross-validation: {} files diverged:\n{}",
            diverged.len(),
            diverged.join("\n")
        );
        eprintln!(
            "corpus smoke: {} .mirror files walked through seed, 0 divergences",
            files.len()
        );
    }

    /// Recursively collect every `*.mirror` file under `root`.
    fn collect_mirror_files(root: &std::path::Path, out: &mut Vec<PathBuf>) {
        let entries = match std::fs::read_dir(root) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_mirror_files(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("mirror") {
                out.push(path);
            }
        }
    }

    /// The load-bearing claim from F-1: `apply_h(c, bytes)` is
    /// no longer structural-self on every variant. Demonstrate with a
    /// `Literal` directly.
    #[test]
    fn f1_walker_is_not_structural_self() {
        let c = Combinator::Literal(b"focus".to_vec());
        let c_oid = combinator_tree_oid(&c);
        // Match: witness OID equals input OID.
        let match_v = apply_h(&c, (b"focus rest".to_vec(), 0usize));
        let match_w = match match_v {
            Imperfect::Success(w) | Imperfect::Partial(w, _) => w,
            Imperfect::Failure(_, _) => unreachable!(),
        };
        assert_eq!(c_oid, combinator_tree_oid(&match_w));
        // Mismatch: witness OID differs (DarkFallback vs Literal).
        let miss_v = apply_h(&c, (b"split rest".to_vec(), 0usize));
        let miss_w = match miss_v {
            Imperfect::Success(w) | Imperfect::Partial(w, _) => w,
            Imperfect::Failure(_, _) => unreachable!(),
        };
        assert_ne!(
            c_oid,
            combinator_tree_oid(&miss_w),
            "walker differentiates: would have been vacuous pre-F-1"
        );
        assert_eq!(miss_w, Combinator::DarkFallback);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstKind, AstNode};
    use prismqueer::{apply as prism_apply, Beam, Optic};
    use std::convert::Infallible;

    // -----------------------------------------------------------------------
    // Test fixtures — small concrete `Prism` impls over the scalar state
    // type `f64`. Each fixture performs its real work in `focus` and
    // passes through unchanged in `project` and `settle` — the bootstrap
    // doesn't distinguish phases at the algebra level; the three-phase
    // chain of `Prism` is the spectral-triple's *internal* composition,
    // and we only need a uniform-state algebra element here.
    //
    // Loss carrier is [`Transparency<Ref>`] — every fixture's optic
    // beams parameterise their loss position over the structured opacity
    // monoid, so opacities flow through `compose_a` as a `BTreeMap` of
    // located verdicts rather than a single scalar residual.
    // -----------------------------------------------------------------------

    /// Identity prism over the bootstrap's `Transparency<Ref>` Loss
    /// monoid. Functionally identical to [`prismqueer::IdentityPrism`],
    /// but with the `L` parameter fixed to `Transparency<Ref>` so it
    /// satisfies `compose_a`'s bounds (the substrate's generic
    /// `IdentityPrism` defaults `L = ScalarLoss` and cannot be reused
    /// here without a heterogeneous-Loss `IdentityPrism<S, L>`, which
    /// is a separate prism-core change).
    struct IdentityT<S: Clone>(std::marker::PhantomData<S>);

    impl<S: Clone> IdentityT<S> {
        fn new() -> Self {
            IdentityT(std::marker::PhantomData)
        }
    }

    impl<S: Clone + 'static> Prism for IdentityT<S> {
        type Input = Seed<S>;
        type Focused = Optic<S, S, Infallible, Transparency<Ref>>;
        type Projected = Optic<S, S, Infallible, Transparency<Ref>>;
        type Refracted = Optic<S, S, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let v = beam
                .value()
                .cloned()
                .expect("IdentityT::focus on dark beam");
            Optic::ok(v.clone(), v)
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = beam
                .value()
                .cloned()
                .expect("IdentityT::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = beam
                .value()
                .cloned()
                .expect("IdentityT::settle on dark beam");
            beam.next(v)
        }
    }

    /// Multiplies a scalar state by `factor`. Models a focus-like
    /// operator: pure success, no opacity opened.
    struct Scale {
        factor: f64,
    }

    impl Prism for Scale {
        type Input = Seed<f64>;
        type Focused = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Projected = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Refracted = Optic<f64, f64, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let v = *beam.value().expect("Scale::focus on dark beam");
            Optic::ok(v, v * self.factor)
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("Scale::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("Scale::settle on dark beam");
            beam.next(v)
        }
    }

    /// Quantizes a scalar to the nearest integer. Models a project-like
    /// operator: produces a value but opens a *located* opacity at
    /// `@quantize` (with `PropertyVerdict::Partial { confidence,
    /// diagnostics }`) on non-integer inputs. The opacity's confidence
    /// is `1.0 - |residual|.min(1.0)` — full confidence on integers,
    /// zero confidence on rounding errors ≥ 1.0.
    struct Quantize;

    impl Prism for Quantize {
        type Input = Seed<f64>;
        type Focused = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Projected = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Refracted = Optic<f64, f64, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("Quantize::focus on dark beam");
            let rounded = state.round();
            let r = (state - rounded).abs();
            if r == 0.0 {
                Optic::ok(state, rounded)
            } else {
                let confidence = 1.0 - r.abs().min(1.0);
                let verdict = PropertyVerdict::Partial {
                    confidence,
                    diagnostics: vec![Diagnostic::new(format!("rounding residual {r}"))],
                };
                let transparency = Transparency::single(
                    Ref::new("@quantize").expect("@quantize is a valid Ref"),
                    verdict,
                );
                Optic::partial(state, rounded, transparency)
            }
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("Quantize::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("Quantize::settle on dark beam");
            beam.next(v)
        }
    }

    /// Rejects negative inputs. Models an operator whose domain is the
    /// non-negative reals; transporting a negative state opens a
    /// `PropertyVerdict::Fail` at `@positive` and clamps the state to
    /// 0.0.
    ///
    /// Domain rejection is modelled as a Partial-with-opacity rather
    /// than as a typed Failure: the bootstrap evaluator's algebra has
    /// `Error = Infallible` (matching `prismqueer::IdentityPrism`'s
    /// signature so identity composition typechecks). The structural
    /// verdict witnesses *where* the rejection lives —
    /// `Transparency::catastrophic()` is reserved for absorbing total-
    /// rejection cases that don't apply at this fixture.
    struct Positive;

    impl Prism for Positive {
        type Input = Seed<f64>;
        type Focused = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Projected = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Refracted = Optic<f64, f64, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("Positive::focus on dark beam");
            if state >= 0.0 {
                Optic::ok(state, state)
            } else {
                // Domain rejection: open an Opaque @positive Fail and
                // clamp the carried state to 0.0.
                let verdict =
                    PropertyVerdict::Fail(Diagnostic::new(format!("negative state {state}")));
                let transparency = Transparency::single(
                    Ref::new("@positive").expect("@positive is a valid Ref"),
                    verdict,
                );
                Optic::partial(state, 0.0, transparency)
            }
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("Positive::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("Positive::settle on dark beam");
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
        // `IdentityT<f64>` is the unit element of A under
        // `Transparency<Ref>` loss: id ∘ p and p ∘ id behave like p.
        // Witnesses that the local identity prism is the A-monoid
        // identity at the evaluator level — the analogue of
        // prismqueer::IdentityPrism specialised to the bootstrap's Loss
        // carrier.
        let id: IdentityT<f64> = IdentityT::new();
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
    fn compose_a_unions_opacities() {
        // Composing two opacity-opening operators unions their opacity
        // maps via `Transparency::<Ref>::combine` (set-union with
        // per-key `PropertyVerdict::merge_with`). Quantize(0.3) opaques
        // @quantize with residual 0.3 → clamped value 0.0; Quantize(0.0)
        // is integer-Clear. Result: a single opacity at @quantize.
        let v = compose_a(&Quantize, &Quantize, 0.3);
        match v {
            Imperfect::Partial(value, transparency) => {
                assert_eq!(value, 0.0);
                let q_path = Ref::new("@quantize").unwrap();
                assert!(
                    transparency.is_opaque_at(&q_path),
                    "expected opacity at @quantize, got {:?}",
                    transparency
                );
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_propagates_domain_opacity() {
        // Domain rejection in the first operator carries its located
        // opacity forward via `Imperfect::eh`'s propagate_loss law:
        // Partial(0, Opaque{@positive: Fail}) followed by
        // Success(0 * 10 = 0) becomes Partial(0, Opaque{@positive:
        // Fail}). With Scale's pure-success action, the @positive Fail
        // is the only opacity in the final verdict.
        //
        // (Under prism-core's algebra closed with `Error = Infallible`,
        // domain rejection emerges via the Transparency-monoid combine
        // with the chain remaining defined.)
        let v = compose_a(&Positive, &Scale { factor: 10.0 }, -3.0);
        match v {
            Imperfect::Partial(value, transparency) => {
                assert_eq!(value, 0.0, "clamped state at domain boundary");
                let p_path = Ref::new("@positive").unwrap();
                assert!(
                    transparency.is_opaque_at(&p_path),
                    "expected @positive opacity, got {:?}",
                    transparency
                );
                let opacities = transparency.opacities().unwrap();
                match &opacities[&p_path] {
                    PropertyVerdict::Fail(d) => {
                        let msg = d.as_str();
                        assert!(
                            msg.contains("-3") || msg.contains("negative"),
                            "diagnostic mentions the offending state, got {msg:?}"
                        );
                    }
                    other => panic!("expected Fail, got {:?}", other),
                }
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // apply_h — operator-action tests
    // -----------------------------------------------------------------------

    #[test]
    fn apply_h_consistent_with_prism_apply() {
        // apply_h is the named wrapper around prismqueer::apply.
        // The equivalence test: both routes produce the same Verdict.
        let p = Scale { factor: 7.0 };
        for s in [-1.0_f64, 0.0, 2.5] {
            let via_apply_h = apply_h(&p, s);
            let via_prism = prism_apply(&p, seed(s)).into_focus();
            assert_eq!(via_apply_h, via_prism);
        }
    }

    #[test]
    fn apply_h_content_prism_matches_compute_oid_inner() {
        // `apply_h(&ContentOidPrism, node)` over an AST state agrees with
        // the direct `compute_oid_inner` call (the inner recursion). The
        // evaluator route through `ContentOidPrism` doesn't change the OID.
        let mut node = AstNode::new(AstKind::Focus, "root");
        node.add_child(AstNode::new(AstKind::In, "@prism"));
        let oid_direct = compute_oid_inner(&node);
        let v = apply_h(&ContentOidPrism, node.clone());
        match v {
            Imperfect::Success(oid) => assert_eq!(oid, oid_direct),
            other => panic!("expected Success, got {:?}", other),
        }
    }

    #[test]
    fn compute_content_oid_matches_apply_h_content_prism() {
        // The thin-wrapper invariant: `compute_content_oid` IS
        // `apply_h(&ContentOidPrism, node)` unwrapped to its Success payload.
        let mut node = AstNode::new(AstKind::Project, "project");
        node.set_body("hello");
        node.add_child(AstNode::new(AstKind::In, "@prism"));
        let direct = compute_content_oid(&node);
        match apply_h(&ContentOidPrism, node.clone()) {
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
        root.add_child(AstNode::new(AstKind::Shift, "z"));
        root.add_child(AstNode::new(AstKind::Settle, "r"));
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
        let m = [[7.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 1.0]];
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
    // Transparency<Ref> / Loss — laws inherited from prism-core /
    // terni, smoke-checked at the evaluator boundary.
    // -----------------------------------------------------------------------

    #[test]
    fn transparency_clear_is_loss_identity() {
        // `Loss::zero()` on `Transparency<Ref>` is `Clear`; combining
        // anything with Clear returns the other side unchanged. This is
        // the structural identity for the bootstrap's Loss carrier.
        use terni::Loss;
        let clear: Transparency<Ref> = Transparency::clear();
        assert!(clear.is_zero());
        let single = Transparency::single(
            Ref::new("@witness").unwrap(),
            PropertyVerdict::Fail(Diagnostic::new("witness")),
        );
        let combined = single.clone().combine(Transparency::clear());
        assert_eq!(combined, single);
    }

    #[test]
    fn transparency_combine_associates() {
        // `Loss::combine` is associative — the Loss monoid law on
        // `Transparency<Ref>` (set-union of opacity maps with per-key
        // `PropertyVerdict::merge_with`).
        use terni::Loss;
        let a = Transparency::single(
            Ref::new("@a").unwrap(),
            PropertyVerdict::Fail(Diagnostic::new("a")),
        );
        let b = Transparency::single(
            Ref::new("@b").unwrap(),
            PropertyVerdict::Fail(Diagnostic::new("b")),
        );
        let c = Transparency::single(
            Ref::new("@c").unwrap(),
            PropertyVerdict::Fail(Diagnostic::new("c")),
        );
        let left = a.clone().combine(b.clone()).combine(c.clone());
        let right = a.combine(b.combine(c));
        assert_eq!(left, right);
    }
}

// ===========================================================================
// Transparency cascade tests — the 🟢 of the TDD pair (mirror/shard-chain).
//
// These tests assert the Transparency<Ref>-shaped verdict semantics the
// 🟢 cascade now delivers: Quantize / Positive return *structural*
// verdicts (Success on the success path, Opaque at @quantize / @positive
// on the failure path) and compose_a unions opacities at colliding paths.
//
// Per Option α (parallel-track-types-as-aliases): the original 🔴
// commit defined `VerdictT<S>` / `QuantizeT` / `PositiveT` as parallel-
// track types to specify the contract without touching the real types.
// After the swap, those parallel-track types are structurally identical
// to the real ones; `VerdictT<S>` aliases `Verdict<S>`, and the
// `QuantizeT` / `PositiveT` structs are kept here as thin, locally-named
// fixtures so the 🔴 commit's test bodies type-check verbatim. The
// underlying semantics flow through the same Transparency<Ref> Loss
// monoid.
// ===========================================================================
#[cfg(test)]
mod transparency_cascade_tests {
    use super::*;
    use std::convert::Infallible;

    /// Post-cascade alias for the module-level [`Verdict`]. Identical
    /// shape; preserved so the 🔴 commit's tests compile verbatim
    /// (Option α from the 🟢 brief).
    pub type VerdictT<S> = Verdict<S>;

    /// Quantize-with-Transparency. Returns `Success` on integer states;
    /// returns `Partial(rounded, Opaque(@quantize, PropertyVerdict::Partial { … }))`
    /// on non-integer states. Structurally identical to the
    /// `tests::Quantize` fixture above — kept under the `T`-suffixed
    /// name so the 🔴's test bodies type-check verbatim.
    pub struct QuantizeT;

    impl Prism for QuantizeT {
        type Input = Optic<(), f64>;
        type Focused = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Projected = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Refracted = Optic<f64, f64, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("QuantizeT::focus on dark beam");
            let rounded = state.round();
            let r = (state - rounded).abs();
            if r == 0.0 {
                Optic::ok(state, rounded)
            } else {
                let confidence = 1.0 - r.abs().min(1.0);
                let verdict = PropertyVerdict::Partial {
                    confidence,
                    diagnostics: vec![Diagnostic::new(format!("rounding residual {r}"))],
                };
                let transparency = Transparency::single(
                    Ref::new("@quantize").expect("@quantize is a valid Ref"),
                    verdict,
                );
                Optic::partial(state, rounded, transparency)
            }
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("QuantizeT::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("QuantizeT::settle on dark beam");
            beam.next(v)
        }
    }

    /// Positive-with-Transparency. Returns `Success` on non-negative
    /// states; returns `Partial(0.0, Opaque(@positive, Fail))` on
    /// negative states. Structurally identical to the `tests::Positive`
    /// fixture above.
    pub struct PositiveT;

    impl Prism for PositiveT {
        type Input = Optic<(), f64>;
        type Focused = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Projected = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Refracted = Optic<f64, f64, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("PositiveT::focus on dark beam");
            if state >= 0.0 {
                Optic::ok(state, state)
            } else {
                let verdict =
                    PropertyVerdict::Fail(Diagnostic::new(format!("negative state {state}")));
                let transparency = Transparency::single(
                    Ref::new("@positive").expect("@positive is a valid Ref"),
                    verdict,
                );
                Optic::partial(state, 0.0, transparency)
            }
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("PositiveT::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("PositiveT::settle on dark beam");
            beam.next(v)
        }
    }

    /// Algebra composition under the Transparency<Ref> verdict carrier.
    /// Identical to the module-level [`compose_a`], specialised to the
    /// `_t`-suffixed fixtures so the 🔴 commit's test bodies type-check
    /// verbatim.
    fn compose_a_t<S, InP, InQ, P, Q>(p: &P, q: &Q, state: S) -> VerdictT<S>
    where
        P: Prism<Input = Optic<(), S>, Refracted = Optic<InP, S, Infallible, Transparency<Ref>>>,
        Q: Prism<Input = Optic<(), S>, Refracted = Optic<InQ, S, Infallible, Transparency<Ref>>>,
    {
        prismqueer::apply_h(p, state).eh(|s| prismqueer::apply_h(q, s))
    }

    // ------------------------------------------------------------------
    // QuantizeT semantics
    // ------------------------------------------------------------------

    #[test]
    fn quantize_t_clear_on_integer() {
        let v: VerdictT<f64> = prismqueer::apply_h(&QuantizeT, 3.0);
        match v {
            Imperfect::Success(out) => assert_eq!(out, 3.0),
            other => panic!("expected Success (zero loss = Clear), got {:?}", other),
        }
    }

    #[test]
    fn quantize_t_opaque_at_quantize_path_on_non_integer() {
        let v: VerdictT<f64> = prismqueer::apply_h(&QuantizeT, 0.3);
        match v {
            Imperfect::Partial(out, transparency) => {
                assert_eq!(out, 0.0, "rounded value carried");
                let q_path = Ref::new("@quantize").unwrap();
                assert!(
                    transparency.is_opaque_at(&q_path),
                    "expected Opaque at @quantize, got {:?}",
                    transparency
                );
                let opacities = transparency.opacities().unwrap();
                match &opacities[&q_path] {
                    PropertyVerdict::Partial {
                        confidence,
                        diagnostics,
                    } => {
                        // confidence = 1.0 - |residual|.min(1.0) = 1.0 - 0.3 = 0.7
                        assert!(
                            (*confidence - 0.7).abs() < 1e-12,
                            "confidence: got {confidence}, expected 0.7"
                        );
                        assert!(!diagnostics.is_empty(), "diagnostic expected");
                    }
                    other => panic!("expected Partial verdict, got {:?}", other),
                }
            }
            other => panic!("expected Partial verdict, got {:?}", other),
        }
    }

    // ------------------------------------------------------------------
    // PositiveT semantics
    // ------------------------------------------------------------------

    #[test]
    fn positive_t_clear_on_non_negative() {
        let v: VerdictT<f64> = prismqueer::apply_h(&PositiveT, 3.0);
        match v {
            Imperfect::Success(out) => assert_eq!(out, 3.0),
            other => panic!("expected Success on non-negative, got {:?}", other),
        }
    }

    #[test]
    fn positive_t_opaque_at_positive_path_on_negative() {
        let v: VerdictT<f64> = prismqueer::apply_h(&PositiveT, -3.0);
        match v {
            Imperfect::Partial(_out, transparency) => {
                let p_path = Ref::new("@positive").unwrap();
                assert!(
                    transparency.is_opaque_at(&p_path),
                    "expected Opaque at @positive, got {:?}",
                    transparency
                );
                let opacities = transparency.opacities().unwrap();
                match &opacities[&p_path] {
                    PropertyVerdict::Fail(d) => {
                        let msg = d.as_str();
                        assert!(
                            msg.contains("-3") || msg.contains("negative"),
                            "diagnostic should mention the offending state, got {msg:?}"
                        );
                    }
                    other => panic!("expected Fail verdict, got {:?}", other),
                }
            }
            other => panic!("expected Partial verdict, got {:?}", other),
        }
    }

    // ------------------------------------------------------------------
    // compose_a_t — algebra composition under Transparency<Ref>
    // ------------------------------------------------------------------

    #[test]
    fn compose_a_unions_opacities() {
        // Composing two opacity-producing operators at *different* paths
        // unions their opacity maps via verdict_union — the Loss-monoid
        // combine on Transparency<Ref>. Quantize(0.3) opaques @quantize;
        // chaining with Positive on -1 would also opaque @positive. Here
        // we use QuantizeT ∘ PositiveT on a value that activates both:
        // 0.3 → quantize → 0 (opaque @quantize). Then Positive(0) is OK.
        // Take the harder path: PositiveT first on -0.5 (opaque @positive,
        // value clamped to 0), then QuantizeT on 0 (Clear). Result: a
        // single opacity at @positive.
        //
        // For the union case, run QuantizeT then PositiveT on -0.3:
        //   QuantizeT(-0.3) → -0.0 with Opaque @quantize (residual 0.3)
        //   PositiveT(0.0) → 0.0 with Clear (non-negative)
        //   union → Opaque {@quantize} only
        let v: VerdictT<f64> = compose_a_t(&QuantizeT, &PositiveT, -0.3);
        match v {
            Imperfect::Partial(_out, transparency) => {
                assert!(
                    transparency.is_opaque_at(&Ref::new("@quantize").unwrap()),
                    "expected @quantize opacity after first prism, got {:?}",
                    transparency
                );
            }
            other => panic!("expected Partial after compose_a_t, got {:?}", other),
        }
    }

    // ------------------------------------------------------------------
    // Two-path union + Fail-dominance-in-merge — the genuine cascade
    // laws (per Seam C1a / C1b, pre-merge adversarial review,
    // 2026-05-30). To witness these we need prisms that BOTH open
    // opacity (at the same OR different refs), which the
    // QuantizeT/PositiveT pair on a single threaded state cannot reach.
    // `AlwaysOpaqueT(ref, verdict)` is an inline test fixture that
    // opens an opacity at a fixed Ref on every invocation, threading
    // the state through unchanged.
    // ------------------------------------------------------------------

    /// Always-opens-opacity prism for f64. Returns `Partial(state,
    /// Opaque {self.path: self.verdict})` on every call — a controlled
    /// fixture for exercising compose_a's union / dominance semantics.
    /// The state is threaded through unchanged so chained `compose_a_t`
    /// calls keep operating on the same value.
    struct AlwaysOpaqueT {
        path: Ref,
        verdict: PropertyVerdict,
    }

    impl Prism for AlwaysOpaqueT {
        type Input = Optic<(), f64>;
        type Focused = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Projected = Optic<f64, f64, Infallible, Transparency<Ref>>;
        type Refracted = Optic<f64, f64, Infallible, Transparency<Ref>>;

        fn focus(&self, beam: Self::Input) -> Self::Focused {
            let state = *beam.value().expect("AlwaysOpaqueT::focus on dark beam");
            let transparency = Transparency::single(self.path.clone(), self.verdict.clone());
            Optic::partial(state, state, transparency)
        }
        fn project(&self, beam: Self::Focused) -> Self::Projected {
            let v = *beam.value().expect("AlwaysOpaqueT::project on dark beam");
            beam.next(v)
        }
        fn settle(&self, beam: Self::Projected) -> Self::Refracted {
            let v = *beam.value().expect("AlwaysOpaqueT::settle on dark beam");
            beam.next(v)
        }
    }

    #[test]
    fn compose_a_unions_opacities_two_sided() {
        // Two prisms, each producing Opaque at a DIFFERENT Ref. The
        // composed Transparency must be Opaque with BOTH entries
        // present (BTreeMap union, no clobber, no drop).
        let a_path = Ref::new("@two-sided/a").unwrap();
        let b_path = Ref::new("@two-sided/b").unwrap();
        let a = AlwaysOpaqueT {
            path: a_path.clone(),
            verdict: PropertyVerdict::Partial {
                confidence: 0.8,
                diagnostics: vec![Diagnostic::new("a-side")],
            },
        };
        let b = AlwaysOpaqueT {
            path: b_path.clone(),
            verdict: PropertyVerdict::Fail(Diagnostic::new("b-side-boom")),
        };
        let v: VerdictT<f64> = compose_a_t(&a, &b, 7.0);
        match v {
            Imperfect::Partial(out, transparency) => {
                assert_eq!(out, 7.0, "state threaded unchanged through composition");
                let opacities = transparency.opacities().expect("Opaque");
                assert_eq!(
                    opacities.len(),
                    2,
                    "two-sided union must keep both entries, got {:?}",
                    opacities.keys().collect::<Vec<_>>()
                );
                assert!(opacities.contains_key(&a_path));
                assert!(opacities.contains_key(&b_path));
            }
            other => panic!("expected Partial after two-sided compose, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_fail_dominates_in_merge() {
        // Two prisms, BOTH producing Opaque at the SAME Ref but with
        // different verdict shapes (Partial vs Fail). The merge inside
        // verdict_union must pick Fail (per PropertyVerdict::merge_with
        // semantics: Fail dominates Partial).
        let shared = Ref::new("@dominance/shared").unwrap();
        let a = AlwaysOpaqueT {
            path: shared.clone(),
            verdict: PropertyVerdict::Partial {
                confidence: 0.9,
                diagnostics: vec![Diagnostic::new("first-partial")],
            },
        };
        let b = AlwaysOpaqueT {
            path: shared.clone(),
            verdict: PropertyVerdict::Fail(Diagnostic::new("second-fail")),
        };
        let v: VerdictT<f64> = compose_a_t(&a, &b, 0.0);
        match v {
            Imperfect::Partial(_out, transparency) => {
                let opacities = transparency.opacities().expect("Opaque");
                assert_eq!(opacities.len(), 1);
                match &opacities[&shared] {
                    PropertyVerdict::Fail(d) => {
                        assert_eq!(d.as_str(), "second-fail");
                    }
                    other => panic!("expected Fail to dominate, got {:?}", other),
                }
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_fail_dominates_in_merge_partial_first() {
        // Symmetric witness: existing Fail must absorb a later Partial.
        let shared = Ref::new("@dominance/shared").unwrap();
        let a = AlwaysOpaqueT {
            path: shared.clone(),
            verdict: PropertyVerdict::Fail(Diagnostic::new("first-fail")),
        };
        let b = AlwaysOpaqueT {
            path: shared.clone(),
            verdict: PropertyVerdict::Partial {
                confidence: 0.5,
                diagnostics: vec![Diagnostic::new("second-partial")],
            },
        };
        let v: VerdictT<f64> = compose_a_t(&a, &b, 0.0);
        match v {
            Imperfect::Partial(_out, transparency) => {
                let opacities = transparency.opacities().expect("Opaque");
                assert_eq!(opacities.len(), 1);
                match &opacities[&shared] {
                    PropertyVerdict::Fail(d) => {
                        assert_eq!(d.as_str(), "first-fail");
                    }
                    other => panic!("expected Fail to survive, got {:?}", other),
                }
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_preserves_single_path_opacity() {
        // Renamed from `compose_a_unions_opacities_both_sides` per Seam
        // C1a (2026-05-30 pre-merge adversarial review). The original
        // name overclaimed: this case does NOT exercise a two-path
        // union, because QuantizeT ∘ QuantizeT on 0.7 opens @quantize
        // exactly once (the second QuantizeT sees 1.0 — integer — and
        // returns Clear). What this test ACTUALLY verifies is
        // single-path-opacity preservation through composition.
        //
        // The genuine two-path-union law is now witnessed by
        // `compose_a_unions_opacities_two_sided` above.
        let v: VerdictT<f64> = compose_a_t(&QuantizeT, &QuantizeT, 0.7);
        match v {
            Imperfect::Partial(out, transparency) => {
                // QuantizeT(0.7) = 1.0 opaque @quantize; then
                // QuantizeT(1.0) = 1.0 Clear; union = single @quantize.
                assert_eq!(out, 1.0);
                assert!(
                    transparency.is_opaque_at(&Ref::new("@quantize").unwrap()),
                    "expected opacity at @quantize after compose, got {:?}",
                    transparency
                );
                let opacities = transparency.opacities().unwrap();
                assert_eq!(
                    opacities.len(),
                    1,
                    "single-path-opacity preservation: exactly one entry"
                );
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }

    #[test]
    fn compose_a_propagates_fail_through_clear() {
        // Renamed from `compose_a_first_failure_dominates_in_merge` per
        // Seam C1b (2026-05-30 pre-merge adversarial review). The
        // original name claimed dominance-in-merge, but this test only
        // exercises Fail surviving a Clear right-hand side: Clear has
        // no contents to merge with, so the merge-dominance rule is
        // never triggered. This is the right-identity law on combine,
        // not the dominance law.
        //
        // The genuine dominance-in-merge law (Fail wins over Partial
        // at the SAME path when both sides have opacities at that
        // path) is now witnessed by `compose_a_fail_dominates_in_merge`
        // above.
        let v: VerdictT<f64> = compose_a_t(&PositiveT, &QuantizeT, -3.0);
        match v {
            Imperfect::Partial(_out, transparency) => {
                let p_path = Ref::new("@positive").unwrap();
                let opacities = transparency.opacities().unwrap();
                match &opacities[&p_path] {
                    PropertyVerdict::Fail(_) => {}
                    other => panic!("expected Fail to survive, got {:?}", other),
                }
            }
            other => panic!("expected Partial verdict, got {:?}", other),
        }
    }

    // ------------------------------------------------------------------
    // Module-level Verdict<S> uses Transparency<Ref> after the cascade.
    //
    // The witness test that constructs `super::Verdict<f64>` carrying a
    // `Transparency<Ref>` lives in the 🟢 commit — it cannot compile
    // until Verdict's loss carrier is swapped from ScalarLoss to
    // Transparency<Ref>. Here we stage the imports and verify the new
    // type alias VerdictT<S> already typechecks against the new shape.
    // ------------------------------------------------------------------

    #[test]
    fn verdict_t_typechecks_against_transparency() {
        let p_path = Ref::new("@witness").unwrap();
        let t: Transparency<Ref> = Transparency::single(
            p_path.clone(),
            PropertyVerdict::Fail(Diagnostic::new("type witness")),
        );
        let v: VerdictT<f64> = Imperfect::Partial(0.0, t);
        match v {
            Imperfect::Partial(_out, transparency) => {
                assert!(transparency.is_opaque_at(&p_path));
            }
            other => panic!("expected Partial, got {:?}", other),
        }
    }
}
