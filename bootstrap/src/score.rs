//! `@mirror/spectral/score` — boundary Rust for the orchestra's
//! shared-state carrier.
//!
//! This module realizes substrate-declared types and actions from
//! [`shards/mirror/spectral/score.mirror`] (substrate-FROZEN at commit
//! 5c6bda2) as Rust bodies. T11 lands the carrier plus the two read
//! surfaces ([`score_of`], [`pending`]) the kintsugi loop's
//! `active_pass` composes through.
//!
//! ## What lives here
//!
//! - [`MetalogueSession`] — a minimal Rust mirror of the substrate's
//!   `metalogue_session` (per `metalogue.mirror`'s `type
//!   metalogue_session = { turns: [turn], opacity: transparency(turn) }`).
//!   The full session carrier lands when `@metalogue` pulls into
//!   bootstrap; T11 carries the iteration tick (the substrate's
//!   `tick` discipline per `@epistemologic/reality/time`) as the
//!   minimal substrate-honest reading of the session at the loop
//!   altitude.
//! - [`Score`] — the Rust mirror of `type score = { anchor: ref,
//!   session: metalogue_session, pending: morphism_set }` from
//!   `score.mirror` lines 348–352. Three typed carriers; identity
//!   contract per the substrate's record-equality discipline.
//! - [`score_of`] — the executable form of `score_of(o: oscillation)
//!   -> score`. The realisation-layer projection from an
//!   [`crate::oscillate::Oscillation`] to a [`Score`]; threads the
//!   anchor through directly, lifts the iteration tick into the
//!   minimal `MetalogueSession`, and derives the pending morphism set
//!   from the AST the realisation layer reads at the glue-bus surface
//!   (per `score.mirror`'s honestly-surfaced gap on the "session from
//!   ref" projection — the realisation layer reads the running AST
//!   and projects).
//! - [`pending`] — the executable form of `pending(s: score) ->
//!   morphism_set`. Direct field projection; the substrate names the
//!   surface, the realisation returns the slice.
//!
//! ## Substrate-pull discipline
//!
//! Per `score.mirror`'s honestly-surfaced gaps (lines 152–192):
//!
//! - `score_of` reads an oscillation, but the substrate has no
//!   "read the active metalogue session from a shard ref" action
//!   declared yet. The realisation layer reads the active session
//!   from its running context — here, T11 surfaces this as an
//!   explicit `ast: &AstNode` parameter (the AST IS the substrate the
//!   formatter is acting on; the session IS implicit in the
//!   formatter's current scope). A future
//!   `@mirror/spectral/voice` sub-shard MAY declare the
//!   session-from-ref projection; until then the realisation reads
//!   the AST directly.
//!
//! - `score_of` reads the oscillation's implicit pending morphism
//!   set, but the substrate has no "read the kintsugi loop's
//!   in-flight morphisms from an anchor ref" action declared at the
//!   consumer altitude. The realisation derives pending from the AST
//!   via [`crate::property::gaps_of`] — each gap surfaces as one
//!   candidate Morphism the formatter is mid-evaluation on. A future
//!   `@mirror/spectral/audition` sub-shard MAY declare the
//!   pending-from-anchor projection.
//!
//! - The score is READ-ONLY to voices. This module declares only the
//!   read surface ([`score_of`] as constructor, [`pending`] as read);
//!   no mutators. Mutations land through the kintsugi loop (the
//!   `dark_pass` body advances the anchor; the metalogue bus
//!   accumulates turns).
//!
//! ## How active_pass (T11) composes through this module
//!
//! Per `score.mirror` lines 387–395 the substrate-altitude chain is:
//!
//! ```text
//! active_pass(o, ast) =
//!   let s        = score_of(o, ast)             -- this module
//!   let m_set    = pending(&s)                  -- this module
//!   let gaps     = m_set.flat_map(|m| gaps_for_candidate(ast))
//!   let tensor   = tensor_of(gaps)              -- T6/T8.5
//!   let fractures = minimize(&tensor)            -- T7/T9 (SDRF)
//!   project fractures.head into Morphism
//! ```
//!
//! T11 lives in [`crate::oscillate::active_pass_with_ast`]; this
//! module surfaces only the score primitives.
//!
//! [`shards/mirror/spectral/score.mirror`]: ../../../../shards/mirror/spectral/score.mirror
#![allow(dead_code)]

use prism_core::Ref;

use crate::ast::AstNode;
use crate::gap::Gap;
use crate::music::{CadenceKind, Dissonance};
use crate::oscillate::{Morphism, Oscillation, Tick};
use crate::property::gaps_of;

// ---------------------------------------------------------------------------
// MetalogueSession — the minimal session carrier.
// ---------------------------------------------------------------------------

/// The substrate's `metalogue_session` carrier at the score altitude
/// (T11 minimal Rust shape).
///
/// Per `shards/metalogue.mirror`: `type metalogue_session = { turns:
/// [turn], opacity: transparency(turn) }`. The full `turn` carrier and
/// the per-turn opacity reading have not pulled into Rust yet; T11
/// lands the minimum the Score needs — the substrate-time tick at
/// which the session was projected. This is consonant with
/// `score.mirror`'s honestly-surfaced gap (lines 162–167): the
/// realisation reads the running session from its context; the
/// substrate-altitude session-from-ref projection is forward-promised.
///
/// When `@metalogue` substrate-pulls into Rust, this carrier lifts to
/// the full `[turn]` + `opacity` shape; the Score's `session` field
/// type updates with it. Until then: a single typed handle naming the
/// session's anchor-in-time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetalogueSession {
    /// The substrate clock at which this session was projected from
    /// the oscillation. Per `score.mirror`'s gap: the realisation
    /// reads the running session; here, the anchor IS the tick the
    /// session was observed at. Future lift: the full `[turn]` + per-
    /// turn `transparency` shape.
    observed_at: Tick,
}

impl MetalogueSession {
    /// Construct a session anchored at a substrate tick.
    pub fn at(observed_at: Tick) -> Self {
        MetalogueSession { observed_at }
    }

    /// Read the substrate tick this session was observed at.
    pub fn observed_at(s: &MetalogueSession) -> Tick {
        s.observed_at
    }
}

// ---------------------------------------------------------------------------
// Score — the orchestra's shared state.
// ---------------------------------------------------------------------------

/// The substrate's score carrier at the orchestra altitude.
///
/// Mirrors `type score = { anchor: ref, session: metalogue_session,
/// pending: morphism_set }` from `shards/mirror/spectral/score.mirror`
/// lines 348–352. The shared state the orchestra reads:
///
/// - [`anchor`](Score::anchor) — the eigenboard envelope's current
///   shard handle (a [`Ref`] into `@mirror/store`'s content-addressed
///   Merkle chain). The position the oscillation is anchored on; what
///   `dark_pass` advances when a morphism applies under identity
///   preservation; what voices read to see "where the substrate is now."
/// - [`session`](Score::session) — the metalogue bus (per
///   `metalogue.mirror`'s `metalogue_session` carrier). The orchestra's
///   conversation: who's spoken, what's still opaque, the ordered turns
///   that frame the consent surface's reads.
/// - [`pending`](Score::pending) — the kintsugi loop's pending morphism
///   set (per `consent.mirror`'s `morphism_set` carrier). The morphisms
///   the formatter has proposed but the consent surface has not yet
///   resolved.
///
/// Identity contract: two scores are equal iff their anchor, session,
/// and pending fields all match.
///
/// Read-only contract: voices read the score; the score is not mutated
/// directly. Mutations land through the kintsugi loop (`dark_pass`
/// advances the anchor; the metalogue bus accumulates turns; the
/// consent surface drains pending).
#[derive(Clone, Debug, PartialEq)]
pub struct Score {
    anchor: Ref,
    session: MetalogueSession,
    pending: Vec<Morphism>,
}

impl Score {
    /// Construct a score from its three substrate carriers. Most
    /// callers want [`score_of`] — this constructor is for tests and
    /// downstream consumers that build scores from non-oscillation
    /// sources.
    pub fn new(anchor: Ref, session: MetalogueSession, pending: Vec<Morphism>) -> Self {
        Score {
            anchor,
            session,
            pending,
        }
    }

    /// Borrow this score's anchor ref.
    pub fn anchor(s: &Score) -> &Ref {
        &s.anchor
    }

    /// Borrow this score's metalogue session.
    pub fn session(s: &Score) -> &MetalogueSession {
        &s.session
    }

    /// Borrow this score's pending morphism set.
    pub fn pending(s: &Score) -> &[Morphism] {
        &s.pending
    }
}

// ---------------------------------------------------------------------------
// score_of — the projection from oscillation.
// ---------------------------------------------------------------------------

/// `score_of(o: oscillation, ast: ast) -> score` — the
/// realisation-layer projection from an oscillation to a score.
///
/// Per `shards/mirror/spectral/score.mirror` lines 354–402 the
/// substrate signature is `score_of(o: oscillation) -> score`; the
/// realisation reads the active metalogue session from the running
/// glue-bus state (per CLAUDE.md's glue-bus discipline) and projects.
/// At the bootstrap altitude the realisation has no `@metalogue`
/// surface to read; the AST the formatter is acting on IS the
/// running context. T11 surfaces this honestly: the realisation
/// accepts the AST as an explicit parameter and derives the pending
/// morphism set from `gaps_of(ast)`.
///
/// Composes through:
///
/// 1. `o.anchor()` → `score.anchor` — direct field forward.
/// 2. `MetalogueSession::at(o.iteration())` — the minimal session
///    anchored at the substrate tick the oscillation is positioned
///    on. A future `@mirror/spectral/voice` substrate-pull replaces
///    this with a real session-from-ref projection.
/// 3. `gaps_of(ast)` → `Vec<Gap>` → each gap projects to one
///    candidate [`Morphism`] (anchored at the gap's substrate origin,
///    scored by the gap's level, expected to resolve authentically).
///    The pending set IS the orchestra's mid-evaluation candidate
///    set; substrate-honest reading: every gap is a morphism the
///    formatter is proposing to close.
///
/// Pure; no I/O; allocates per the returned [`Score`].
pub fn score_of(o: &Oscillation, ast: &AstNode) -> Score {
    // 🟢 [substrate-pull:realize] T11 GREEN body. Composes the three
    // substrate carriers per score.mirror lines 354–402:
    //   1. anchor    — direct field forward from the oscillation
    //   2. session   — minimal MetalogueSession anchored at the
    //                  oscillation's iteration tick
    //   3. pending   — lifted from gaps_of(ast) via per-gap
    //                  candidate Morphism projection
    let anchor = o.anchor().clone();
    let session = MetalogueSession::at(o.iteration());
    let pending = pending_from_ast(ast);
    Score::new(anchor, session, pending)
}

/// Project a [`Gap`] to a candidate [`Morphism`].
///
/// Each gap surfaces as one mid-evaluation morphism:
///
/// - `content` — the gap's substrate origin (where the cocycle lives).
/// - `score` — a dissonance reading derived from the gap's Bateson
///   level (level 0 = floor-altitude failure, low partial count; level
///   1+ = nested contradictions, more partials). Roughness is bounded
///   in `[0, 1]`; a gap is dissonant by definition (the substrate has
///   not yet closed it), so the floor reading is non-zero — `0.5` at
///   level 0 (neutral mid-roughness), saturating toward 1.0 as the
///   level grows.
/// - `expected` — [`CadenceKind::Authentic`] (the formatter proposes
///   each candidate with the expectation that resolution will reach
///   the canonical tonic; the SDRF ranking in `active_pass` revises
///   this expectation per the fracture's descent magnitude).
fn morphism_from_gap(g: &Gap) -> Morphism {
    // 🟢 [substrate-pull:realize] T11 GREEN body. Roughness grows with
    // Bateson level via the substrate-pull-honest reading:
    //   level 0 → 0.5 (neutral mid-roughness; floor-altitude failure)
    //   level 1 → ~0.667
    //   level 2 → ~0.75
    //   asymptotic to 1.0 as level → ∞
    // The partial count rides the level directly so consumers can
    // distinguish floor-altitude gaps from nested learning loops.
    let level = Gap::level(g);
    let roughness = (level as f64 + 1.0) / (level as f64 + 2.0);
    let partials = level.saturating_add(1);
    Morphism::new(
        g.origin().clone(),
        Dissonance::new(roughness, partials),
        CadenceKind::Authentic,
    )
}

/// Derive the pending morphism set from an AST by lifting every gap
/// to a candidate morphism.
fn pending_from_ast(ast: &AstNode) -> Vec<Morphism> {
    // 🟢 [substrate-pull:realize] T11 GREEN body. The realisation lifts
    // every gap to a candidate Morphism; the substrate's pending IS
    // the orchestra's mid-evaluation candidate set.
    gaps_of(ast).iter().map(morphism_from_gap).collect()
}

// ---------------------------------------------------------------------------
// pending — the read action voices use.
// ---------------------------------------------------------------------------

/// `pending(s: score) -> morphism_set` — the substrate's
/// orchestra-altitude read surface for the pending morphism set.
///
/// Per `score.mirror` lines 404–433: direct field projection. The
/// substrate names the surface; the arithmetic IS the field read.
/// Returns a slice into the score's pending vector.
pub fn pending(s: &Score) -> &[Morphism] {
    Score::pending(s)
}

// ---------------------------------------------------------------------------
// gaps_for_pending — expand a pending morphism back into its gap basis.
// ---------------------------------------------------------------------------

/// Re-derive the gap basis for a pending morphism set by reading the
/// AST the score was projected from.
///
/// Per `score.mirror` lines 110–117 the substrate's chain has
/// `pending(score) -> morphism_set` then `morphism_set.content_refs ->
/// gaps via gaps_of`. At the realisation altitude the AST IS the
/// substrate the morphisms address; re-running `gaps_of(ast)` gives
/// the gap basis the tensor reads.
///
/// Returns the same gap vector `score_of`'s `pending_from_ast` lifted
/// to morphisms — the realisation closes the loop by reading the AST
/// again rather than maintaining a Gap ↔ Morphism cross-reference.
/// (The cross-reference would land when a substrate-pull declares a
/// typed `content: gap_ref` carrier for Morphism's content field; T11
/// keeps the realisation minimal.)
pub fn gaps_for_pending(ast: &AstNode) -> Vec<Gap> {
    // 🟢 [substrate-pull:realize] T11 GREEN body. Direct
    // pass-through to gaps_of(ast); the realisation closes the loop
    // by re-reading the AST.
    gaps_of(ast)
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`Score`], [`score_of`], [`pending`],
    //! and [`gaps_for_pending`] — T11's score body.
    //!
    //! Per `shards/mirror/spectral/score.mirror`: the score carries
    //! three typed fields (anchor, session, pending); `score_of`
    //! projects an oscillation + AST into a score; `pending` reads
    //! the pending morphism set. These tests RED first; the body
    //! lands GREEN under [substrate-pull:realize].

    use super::*;
    use crate::ast::{AstKind, AstNode, DarkSpan};
    use crate::oscillate::{Oscillation, OscillationState, Tick};

    fn fixture_anchor() -> Ref {
        Ref::new("@mirror/spectral/score/fixture").expect("valid substrate ref")
    }

    /// A score carries the three typed fields named in the substrate.
    #[test]
    fn score_carries_anchor_session_and_pending() {
        let anchor = fixture_anchor();
        let session = MetalogueSession::at(Tick::new(3));
        let s = Score::new(anchor.clone(), session.clone(), Vec::new());
        assert_eq!(Score::anchor(&s), &anchor);
        assert_eq!(Score::session(&s), &session);
        assert!(Score::pending(&s).is_empty());
    }

    /// A MetalogueSession records the substrate tick it was observed
    /// at — the minimal session carrier per the score-shard gap.
    #[test]
    fn metalogue_session_carries_observed_tick() {
        let s = MetalogueSession::at(Tick::new(7));
        assert_eq!(MetalogueSession::observed_at(&s), Tick::new(7));
    }

    /// `score_of(o, ast)` threads the oscillation's anchor through
    /// directly — the score's anchor IS the oscillation's anchor.
    #[test]
    fn score_of_projects_oscillation_anchor_directly() {
        let anchor = fixture_anchor();
        let o = Oscillation::initial(anchor.clone());
        let ast = AstNode::new(AstKind::Focus, "witness");
        let s = score_of(&o, &ast);
        assert_eq!(Score::anchor(&s), &anchor);
    }

    /// `score_of(o, ast)` lifts the oscillation's iteration tick into
    /// the score's session as the observed-at anchor.
    #[test]
    fn score_of_session_anchors_at_oscillation_iteration() {
        let o = Oscillation::new(OscillationState::Active, Tick::new(5), fixture_anchor());
        let ast = AstNode::new(AstKind::Focus, "witness");
        let s = score_of(&o, &ast);
        assert_eq!(
            MetalogueSession::observed_at(Score::session(&s)),
            Tick::new(5),
        );
    }

    /// `score_of(o, ast)` on a gap-free AST yields an empty pending
    /// morphism set — nothing for the formatter to evaluate.
    #[test]
    fn score_of_on_gap_free_ast_yields_empty_pending() {
        let o = Oscillation::initial(fixture_anchor());
        let ast = AstNode::new(AstKind::Focus, "witness");
        let s = score_of(&o, &ast);
        assert!(Score::pending(&s).is_empty());
    }

    /// `score_of(o, ast)` on a single-dark-region AST yields one
    /// pending morphism, anchored at the substrate origin of the gap.
    #[test]
    fn score_of_on_one_gap_yields_one_pending_morphism() {
        let o = Oscillation::initial(fixture_anchor());
        let ast = AstNode::dark("unknown bytes", DarkSpan { start: 0, end: 5 });
        let s = score_of(&o, &ast);
        assert_eq!(Score::pending(&s).len(), 1);
        let m = &Score::pending(&s)[0];
        assert_eq!(
            m.content().as_str(),
            "@epistemologic/property/total_classification",
            "pending morphism's content IS the gap's substrate origin",
        );
        assert_eq!(
            m.expected(),
            CadenceKind::Authentic,
            "the formatter proposes each candidate expecting authentic resolution",
        );
    }

    /// `score_of(o, ast)` on a multi-dark-region AST yields one pending
    /// morphism per gap, in source order (matches gaps_of's walk).
    #[test]
    fn score_of_on_multi_gap_ast_yields_one_morphism_per_gap_in_order() {
        let o = Oscillation::initial(fixture_anchor());
        let mut root = AstNode::new(AstKind::Project, "outer");
        let d1 = AstNode::dark("first", DarkSpan { start: 0, end: 5 });
        let d2 = AstNode::dark("second", DarkSpan { start: 6, end: 12 });
        root.add_child(d1);
        root.add_child(d2);
        let s = score_of(&o, &root);
        assert_eq!(Score::pending(&s).len(), 2);
    }

    /// `pending(&score)` is direct field projection — the substrate
    /// names the surface; the read IS the field forward.
    #[test]
    fn pending_is_direct_field_projection() {
        let o = Oscillation::initial(fixture_anchor());
        let ast = AstNode::dark("unknown", DarkSpan { start: 0, end: 5 });
        let s = score_of(&o, &ast);
        let read = pending(&s);
        let direct = Score::pending(&s);
        assert_eq!(read.len(), direct.len());
        assert_eq!(read[0].content(), direct[0].content());
    }

    /// `gaps_for_pending(ast)` returns the same gap basis `score_of`
    /// derived the pending set from — the realisation closes the loop
    /// by reading the AST.
    #[test]
    fn gaps_for_pending_returns_gaps_of_the_ast() {
        let ast = AstNode::dark("unknown", DarkSpan { start: 10, end: 23 });
        let gaps = gaps_for_pending(&ast);
        assert_eq!(gaps.len(), 1);
        assert_eq!(
            gaps[0].origin().as_str(),
            "@epistemologic/property/total_classification",
        );
    }

    /// A morphism derived from a level-0 gap carries the floor reading:
    /// roughness 0.5 (neutral), partials = 1 (one Bateson logical level),
    /// expected Authentic.
    #[test]
    fn morphism_from_level_zero_gap_has_floor_dissonance() {
        let g = Gap::new(
            0,
            Ref::new("@epistemologic/property/total_classification").expect("valid"),
            "dark [0, 5)",
        );
        let m = morphism_from_gap(&g);
        assert!(
            (m.score().roughness() - 0.5).abs() < 1e-12,
            "level-0 gap maps to roughness 0.5; got {}",
            m.score().roughness(),
        );
        assert_eq!(m.score().partials(), 1);
        assert_eq!(m.expected(), CadenceKind::Authentic);
    }

    /// A morphism derived from a level-1 gap carries higher roughness
    /// than a level-0 gap (the Bateson level surfaces in dissonance).
    #[test]
    fn morphism_from_higher_level_gap_has_higher_dissonance() {
        let g0 = Gap::new(
            0,
            Ref::new("@epistemologic/property/total_classification").expect("valid"),
            "floor",
        );
        let g1 = Gap::new(
            1,
            Ref::new("@epistemologic/property/total_classification").expect("valid"),
            "nested",
        );
        let m0 = morphism_from_gap(&g0);
        let m1 = morphism_from_gap(&g1);
        assert!(
            m1.score().roughness() > m0.score().roughness(),
            "higher-level gap must yield higher roughness; level-0 {} vs level-1 {}",
            m0.score().roughness(),
            m1.score().roughness(),
        );
        assert!(m1.score().partials() > m0.score().partials());
    }
}
