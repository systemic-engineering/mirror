//! `@mirror/spectral/oscillate` — boundary Rust for the kintsugi-loop
//! termination check.
//!
//! This module realizes a substrate-declared action from
//! `shards/mirror/spectral/oscillate.mirror` as Rust body. The substrate
//! names the full kintsugi loop primitive (the five-state
//! `oscillation_state`, the `pulse` atomic step, the `active_pass` /
//! `dark_pass` two-pass alternation, the `oscillate` driver, the
//! `read_consent` four-to-five bridge, the `is_complete` termination
//! check); T4 lands only the **termination check** — `is_complete` —
//! and the carrier it reads ([`OscillationState`]). The rest of the
//! oscillate primitives are forward-promises to ticks that compose the
//! driver (consent.morphism + consent.query_phi + uuid_spectral.dark
//! must substrate-pull into Rust first; T4 sits at the discriminator
//! floor only).
//!
//! ## What lives here
//!
//! - [`OscillationState`] — the Rust mirror of
//!   `type oscillation_state = active | dark | settled | escalated | waiting`
//!   from `shards/mirror/spectral/oscillate.mirror`.
//! - [`is_complete`] — the executable form of
//!   `is_complete(s: oscillation_state) -> verdict`. Reads an
//!   `OscillationState` and emits the substrate's three-state verdict
//!   per `oscillate.mirror`'s declared mapping table (§is_complete):
//!
//!   | oscillation_state | verdict                                              | confidence | meaning                                       |
//!   |-------------------|------------------------------------------------------|------------|-----------------------------------------------|
//!   | settled           | `Success(())`                                        | 1.0        | autopoietic ground state; holonomy → 0        |
//!   | active            | `Partial((), Transparency::opaque(…, 0.5))`          | 0.5        | mid-cycle; ACTIVE pass live; proposing        |
//!   | dark              | `Partial((), Transparency::opaque(…, 0.5))`          | 0.5        | mid-cycle; DARK pass live; anchoring identity |
//!   | waiting           | `Partial((), Transparency::opaque(…, 0.25))`         | 0.25       | half-cadence; awaiting next consent tick      |
//!   | escalated         | `Failure(Gap{level:1, "escalation surfaced"}, …)`    | 0.0        | external resolution required; pause emitted   |
//!
//! ## The active/dark mapping call (T4 design call)
//!
//! Per `oscillate.mirror`'s `is_complete` declared mapping table
//! (lines 631–649): `active → partial(confidence)` and
//! `dark → partial(confidence)` — the substrate names BOTH as the same
//! `partial` shape and reads them as "driver continues with next
//! pulse". The substrate's own framing IS the answer to the brief's
//! design question: active and dark are not "stop here" (the substrate
//! says `partial`, not `failure`) and not "settled with caveat" (the
//! substrate says `partial`, not `pass`); they are the loop's two live
//! phases, and `partial(confidence)` IS the substrate's honest reading
//! of mid-cycle.
//!
//! The body chooses `confidence = 0.5` for both (the band midpoint).
//! Rationale: `Waiting` already sits at 0.25 (below 1/φ ≈ 0.618 — the
//! "closer to failure" Half-cadence band per `music/mod.rs`); active /
//! dark sit at 0.5 (still below 1/φ; mid-cycle is not yet consonant,
//! but it's structurally distinct from waiting — waiting is paused at
//! the dominant, active/dark is in-flight). Through
//! [`crate::gap::verdict_to_cadence_kind`] both 0.5 and 0.25 read as
//! `CadenceKind::Half` (paused-on-V shape); the substrate's
//! mid-progression IS a half-cadence read at the audible altitude,
//! consonant with `oscillate.mirror`'s own framing
//! (`read_consent.partial+half → waiting`).
//!
//! `escalated → failure(reason)` uses `Gap { level: 1, … }` per
//! [`docs/specs/gap-tension-tensor-substrate.md`] §3.2: an escalation
//! surfacing is a level-1 contradiction (simple unresolved tension; no
//! nested learning loop yet — the nested loop happens when Reflection
//! reads the escalation and responds). Higher levels lift when T5
//! (`gaps_of`) substrate-pulls richer `Gap` shapes.
//!
//! [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../docs/specs/gap-tension-tensor-substrate.md
//! [`Gap`]: crate::gap::Gap
//!
//! ## The implementation cascade template
//!
//! This module is the third discriminator-floor tick in the
//! implementation cascade (after `is_settled` in T3 and `is_consonant`
//! / `is_pareto` in T4's sibling `music` body). It follows the same
//! template T3 established: substrate declares the action signature;
//! bootstrap realises the body in this module under
//! `[substrate-pull:realize]`. The rest of `oscillate.mirror`'s
//! primitives (`pulse`, `active_pass`, `dark_pass`, `oscillate`,
//! `read_consent`) land as sibling functions here when their upstream
//! consent / uuid_spectral dependencies substrate-pull.
//!
//! `#[allow(dead_code)]` at the module level: this module's surface is
//! the forward-promise to `@mirror/spectral/oscillate`'s consumer (the
//! future `mirror kintsugi` driver), which is not yet wired in
//! `main.rs`. The tests below DO use the surface; they prove the
//! contract regardless of the consumer's arrival.
#![allow(dead_code)]

use prism_core::{Diagnostic, PropertyVerdict, Ref, Transparency};
use terni::Imperfect;

use crate::gap::Gap;
use crate::music::{CadenceKind, Dissonance, Verdict};

/// The substrate's oscillation-state carrier at the kintsugi-loop
/// altitude.
///
/// Mirrors `type oscillation_state = active | dark | settled | escalated | waiting`
/// from `shards/mirror/spectral/oscillate.mirror`. The loop's live
/// position; one of five mutually-exclusive states exhausting the
/// substrate's possible loop positions (per the substrate header's
/// five-state argument: two live phases [active, dark] plus three
/// closure positions [settled, escalated, waiting]).
///
/// Identity contract: two `OscillationState` values are equal iff they
/// name the same variant. `PartialEq` derives this; the substrate-
/// theoretic reality is that each variant names a distinct loop
/// position even though two of them (active, dark) project to the same
/// verdict shape per [`is_complete`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OscillationState {
    /// ACTIVE pass live; proposing a loss-decreasing morphism. The
    /// navigable spectral coordinate (uuid_spectral's 48 ACTIVE bits)
    /// is the pulling axis.
    Active,
    /// DARK pass live; anchoring identity. The content-address signal
    /// (uuid_spectral's 80 DARK bits) is the constraint axis.
    Dark,
    /// `cadence.is_settled` returned `pass` (authentic cadence); the
    /// autopoietic ground state reached; holonomy → 0; the loop
    /// terminates with final ref.
    Settled,
    /// `consent.query_phi` returned `failure(reason)` (deceptive
    /// cadence OR Pareto-tied admissibility); the consent surface MUST
    /// resolve via external signal; oscillation halts with `pause_event`
    /// emitted to metalogue.
    Escalated,
    /// `consent.query_phi` returned `partial(low_confidence)`
    /// (half-cadence; substrate mid-progression); the next pulse may
    /// resolve OR pause again; resumes WITHOUT external resolution.
    /// Distinct from `Escalated`.
    Waiting,
}

/// The kintsugi-loop origin Ref for the oscillate shard.
fn oscillate_origin() -> Ref {
    Ref::new("@mirror/spectral/oscillate")
        .expect("kintsugi-loop oscillate shard path must be a valid substrate ref")
}

/// The mid-cycle transparency: a substrate-located opacity at the
/// oscillate origin with `confidence = 0.5`. Same shape for both
/// `Active` and `Dark` because the substrate's `is_complete` mapping
/// names both as `partial(confidence)` and reads them as "driver
/// continues with next pulse" — the difference is which void-duality
/// axis is pulling, not the loop's completeness.
fn mid_cycle_transparency(phase: &'static str) -> Transparency<Ref> {
    Transparency::opaque(
        oscillate_origin(),
        PropertyVerdict::Partial {
            confidence: 0.5,
            diagnostics: vec![Diagnostic::new(phase)],
        },
    )
}

/// The waiting transparency: a substrate-located opacity at the
/// oscillate origin with `confidence = 0.25`. Mirrors the audible-
/// altitude Half-cadence encoding in `music/mod.rs::half_transparency`
/// exactly — substrate's `read_consent.partial+half` maps to
/// `Waiting`, so the confidence reading IS the half-cadence reading at
/// the loop altitude.
fn waiting_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        oscillate_origin(),
        PropertyVerdict::Partial {
            confidence: 0.25,
            diagnostics: vec![Diagnostic::new("awaiting next consent surface tick")],
        },
    )
}

/// The escalated state's gap: a level-1 contradiction naming the
/// surfacing of an external-resolution requirement. Per
/// [`docs/specs/gap-tension-tensor-substrate.md`] §3.2: escalation
/// surfacing is a level-1 contradiction (simple unresolved tension;
/// the nested learning loop happens when Reflection reads the
/// escalation and responds, lifting to higher Bateson levels).
fn escalated_gap() -> Gap {
    Gap::new(1, oscillate_origin(), "escalation surfaced")
}

/// The escalated state's transparency: the opacity observed when the
/// consent surface returned `failure(reason)`; surfaces the failure to
/// metalogue.
fn escalated_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        oscillate_origin(),
        PropertyVerdict::Fail(Diagnostic::new(
            "kintsugi loop halted pending external resolution",
        )),
    )
}

/// `is_complete(s: oscillation_state) -> verdict` — the kintsugi-loop
/// termination check at the loop altitude.
///
/// Reads an [`OscillationState`] and emits the substrate's three-state
/// verdict per `oscillate.mirror` §is_complete (the substrate's own
/// declared mapping table, lines 631–649):
///
/// - `Settled`   → `Success(())` — autopoietic ground state; holonomy → 0
/// - `Active`    → `Partial((), opaque@0.5)` — mid-cycle ACTIVE phase
/// - `Dark`      → `Partial((), opaque@0.5)` — mid-cycle DARK phase
/// - `Waiting`   → `Partial((), opaque@0.25)` — half-cadence
/// - `Escalated` → `Failure(Gap{level:1,…}, opaque)` — external resolution required
///
/// Pure; no I/O; allocates per the verdict carrier.
///
/// Per `oscillate.mirror`'s substrate declaration: the action's verdict
/// IS the loop driver's termination discipline. The substrate has
/// reached its autopoietic fixed point when `is_complete` returns
/// `pass`; the substrate is approaching but not yet at the fixed point
/// when `is_complete` returns `partial`; the substrate has failed to
/// reach the fixed point at this tick when `is_complete` returns
/// `failure`. The three-state surface IS the substrate's honest
/// acknowledgment of its own loop boundary.
pub fn is_complete(state: OscillationState) -> Verdict {
    // The five-state mapping IS the body. Pattern-exhaustive over
    // OscillationState; no fallback arm; the compiler enforces the
    // five-state contract. Adding a sixth variant to OscillationState
    // (none is currently substrate-declared) will fail to compile here.
    match state {
        // Autopoietic ground state per cadence.is_settled(authentic);
        // holonomy → 0; loop terminates.
        OscillationState::Settled => Imperfect::Success(()),
        // Mid-cycle, ACTIVE phase live. Per oscillate.mirror
        // §is_complete: "driver continues with next pulse."
        OscillationState::Active => Imperfect::Partial((), mid_cycle_transparency("active pass")),
        // Mid-cycle, DARK phase live. Same shape as Active per the
        // substrate's own framing.
        OscillationState::Dark => Imperfect::Partial((), mid_cycle_transparency("dark pass")),
        // Half-cadence; substrate paused at the dominant; awaits next
        // consent surface tick; resumes WITHOUT external resolution.
        OscillationState::Waiting => Imperfect::Partial((), waiting_transparency()),
        // Substrate halted pending external resolution; pause_event
        // emitted to metalogue per `consent.emit_to_metalogue`. Level-1
        // contradiction per gap-tension-tensor-substrate.md §3.2.
        OscillationState::Escalated => {
            Imperfect::Failure(escalated_gap(), escalated_transparency())
        }
    }
}

// =====================================================================
// T10.5: the `pulse` body — one ACTIVE→DARK alternation.
//
// Per `shards/mirror/spectral/oscillate.mirror` (substrate-FROZEN at
// commit 02e2981/9efac39/c3802eba sweep): `pulse(o: oscillation) ->
// oscillation` is the atomic step of the kintsugi loop. T10.5 lands
// the boundary-Rust body that composes:
//
//   1. active_pass(o)         — propose a loss-decreasing morphism
//                                (T11 stub: returns a fixture morphism).
//   2. query_phi(morphism)    — gate + rank via the three glass
//                                properties (later-tick stub: returns
//                                a verdict consonant with the fixture).
//   3. dark_pass(o, m)        — anchor identity; advance ref if preserved
//                                (T12 stub: leaves anchor unchanged,
//                                returns a verdict-shaped trace).
//   4. read_consent(v, k)     — bridge the verdict + cadence_kind into
//                                the next oscillation_state.
//   5. emit next Oscillation with iteration += 1 and state updated.
//
// **The stubs are intentional.** Per the brief: pulse demonstrates the
// composition; T11 fills `active_pass` with real candidate generation
// via gaps_of → tensor_of → minimize; T12 fills `dark_pass` with the
// @uuid/spectral.dark byte-equality check; later ticks land
// `query_phi`'s full structural query. T10.5 proves the *chain shape*
// at the boundary — the wiring that those bodies will land into.
//
// Banach contraction discipline (spec §2.1): γ_oscillate ≤ γ_up · γ_down.
// The contraction map (pulse) lands GREEN first; the iteration (the
// `oscillate` driver) lands later. Per the task brief: "pulse's γ_pulse
// witnesses first."
// =====================================================================

/// The substrate's tick carrier at the oscillation altitude.
///
/// Per `shards/epistemologic/reality/time.mirror`'s `type tick =
/// monotonic`: a strictly-monotonic counter. The oscillation's
/// iteration field rides this — each `pulse` call advances `Tick` by
/// exactly one monotonic unit. Newtype per `[[feedback-no-bare-types]]`;
/// a bare `u32` would let two different counters of the same width
/// flow through the substrate boundary without typing.
///
/// Identity contract: two ticks are equal iff their counts match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(u32);

impl Tick {
    /// The zero tick: the loop's starting position.
    pub const fn zero() -> Self {
        Tick(0)
    }

    /// Construct from a raw count.
    pub const fn new(count: u32) -> Self {
        Tick(count)
    }

    /// Read this tick's raw count.
    pub const fn count(self) -> u32 {
        self.0
    }

    /// Advance by one monotonic unit. Saturating at `u32::MAX` per
    /// the substrate's monotonic discipline: the realisation layer
    /// guarantees no decrement; saturation IS the substrate boundary
    /// for the u32 representation. A future-tick lift to `u64` or a
    /// big-int carrier moves the boundary; the discipline holds.
    pub const fn advance(self) -> Self {
        Tick(self.0.saturating_add(1))
    }
}

/// The substrate's oscillation record at the kintsugi-loop altitude.
///
/// Mirrors `type oscillation = { state, iteration: tick, anchor: ref }`
/// from `shards/mirror/spectral/oscillate.mirror`. The loop's full
/// live position:
///
/// - [`state`]     — the loop's live position; one of the five
///   [`OscillationState`] variants. Read by [`pulse`] to dispatch the
///   next half-cycle; read by [`is_complete`] to emit the termination
///   verdict.
/// - [`iteration`] — the loop's tick count from initial. Each `pulse`
///   advances by one monotonic unit; the ordering on `Tick` IS the
///   loop's ordering.
/// - [`anchor`]    — the substrate ref the loop is currently anchored
///   on. The eigenboard's current shard handle; updated by `dark_pass`
///   after a morphism applies; read by `active_pass` to compose the
///   candidate morphism_set.
///
/// Identity contract: two oscillations are equal iff their state,
/// iteration, and anchor all match.
///
/// [`state`]: Oscillation::state
/// [`iteration`]: Oscillation::iteration
/// [`anchor`]: Oscillation::anchor
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Oscillation {
    state: OscillationState,
    iteration: Tick,
    anchor: Ref,
}

impl Oscillation {
    /// Construct from the three carriers.
    pub fn new(state: OscillationState, iteration: Tick, anchor: Ref) -> Self {
        Oscillation {
            state,
            iteration,
            anchor,
        }
    }

    /// Construct the initial oscillation: `active` state, tick zero,
    /// anchored at `initial`. Per `oscillate.mirror` §oscillate (the
    /// driver's step 1): `initial → oscillation { state: active,
    /// iteration: tick_zero, anchor: initial }`.
    pub fn initial(initial: Ref) -> Self {
        Oscillation {
            state: OscillationState::Active,
            iteration: Tick::zero(),
            anchor: initial,
        }
    }

    /// Read this oscillation's live position.
    pub fn state(&self) -> OscillationState {
        self.state
    }

    /// Read this oscillation's iteration count.
    pub fn iteration(&self) -> Tick {
        self.iteration
    }

    /// Borrow this oscillation's anchor ref.
    pub fn anchor(&self) -> &Ref {
        &self.anchor
    }
}

/// The substrate's morphism carrier at the consent altitude (Rust
/// boundary shape, stub).
///
/// Mirrors `type morphism = { content: ref, score: dissonance,
/// expected: cadence_kind }` from
/// `shards/mirror/spectral/consent.mirror`. T10.5 lands the minimal
/// three-field shape required to type the `active_pass` / `dark_pass`
/// signatures. Per the brief: "Stub it minimally; full shape lands
/// when active_pass becomes real" — when T11 substrate-pulls real
/// candidate generation through `gaps_of → tensor_of → minimize`,
/// this carrier may grow richer fields (a typed-content lift; a
/// per-candidate confidence trace; a holonomy reading).
///
/// Identity contract: two morphisms are equal iff their content,
/// score, and expected fields all match.
#[derive(Clone, Debug, PartialEq)]
pub struct Morphism {
    content: Ref,
    score: Dissonance,
    expected: CadenceKind,
}

impl Morphism {
    /// Construct from the three carriers.
    pub fn new(content: Ref, score: Dissonance, expected: CadenceKind) -> Self {
        Morphism {
            content,
            score,
            expected,
        }
    }

    /// Borrow this morphism's content reference.
    pub fn content(&self) -> &Ref {
        &self.content
    }

    /// Read this morphism's dissonance score.
    pub fn score(&self) -> Dissonance {
        self.score
    }

    /// Read this morphism's expected cadence_kind.
    pub fn expected(&self) -> CadenceKind {
        self.expected
    }
}

/// `active_pass(o: oscillation) -> morphism` — the proposal action
/// (T11 stub).
///
/// **STUB.** Per `oscillate.mirror` §active_pass: the realisation reads
/// the anchor ref's candidate morphism_set (the eigenboard's pending
/// imperfections), invokes `dissonance.is_pareto` for the rank,
/// projects the highest-ranked candidate into a morphism. T10.5 returns
/// a fixture morphism anchored at the input's anchor ref so the chain
/// shape is provable end-to-end; the body proper lands at T11 with
/// real candidate generation.
///
/// The fixture: a morphism whose content references the input
/// oscillation's anchor, scored at zero roughness (consonant), expected
/// to resolve authentically. This lets `pulse` thread the chain
/// without coupling to substrate machinery that hasn't pulled yet.
pub fn active_pass(o: &Oscillation) -> Morphism {
    Morphism::new(
        o.anchor().clone(),
        Dissonance::new(0.0, 1),
        CadenceKind::Authentic,
    )
}

/// `query_phi(m: morphism) -> verdict` — the structural Φ query
/// (later-tick stub).
///
/// **STUB.** Per `consent.mirror` §query_phi: the realisation runs the
/// gates (`loss_decreasing`, `identity_preserving`) over the
/// morphism_set; delegates to `admissibility_singleton` for the
/// discriminator; composes with `cadence.is_settled` per the surviving
/// morphism's `expected` field; emits the consent verdict. T10.5
/// dispatches through the substrate's existing audible-altitude
/// `is_settled` against the stub morphism's `expected` cadence —
/// enough to type the chain; the structural query lands when the
/// `morphism_set` shape and the three glass properties pull into Rust.
pub fn query_phi(m: &Morphism) -> Verdict {
    crate::music::is_settled(m.expected())
}

/// `dark_pass(o: oscillation, m: morphism) -> oscillation` — the
/// identity-anchor action (T12 stub).
///
/// **STUB.** Per `oscillate.mirror` §dark_pass: the realisation invokes
/// `consent.identity_preserving(m)`, branches on the verdict, emits
/// the next oscillation with the anchor advanced if preserved (or
/// unchanged on escalation). T10.5 returns an oscillation that
/// projects the consent verdict onto the next state via
/// [`read_consent`], leaving the anchor unchanged (the @uuid/spectral
/// dark-bits byte-equality check is the T12 substrate-pull; until then
/// the stub treats every morphism as identity-preserving and reads its
/// next state from the verdict + the morphism's expected cadence).
pub fn dark_pass(o: &Oscillation, m: &Morphism) -> Oscillation {
    let verdict = query_phi(m);
    let kind = crate::gap::verdict_to_cadence_kind(&verdict);
    let next_state = read_consent(&verdict, kind);
    Oscillation::new(next_state, o.iteration().advance(), o.anchor().clone())
}

/// `read_consent(v: verdict, k: cadence_kind) -> oscillation_state` —
/// the four-to-five bridge.
///
/// Reads a consent verdict + a cadence_kind and emits the next
/// oscillation_state per `oscillate.mirror` §read_consent's declared
/// mapping table (lines 583–622):
///
/// | verdict   | cadence_kind   | next state    | rationale                              |
/// |-----------|----------------|---------------|----------------------------------------|
/// | Success   | Authentic      | Settled       | autopoietic ground; holonomy → 0       |
/// | Success   | (other)        | Active        | consonant alt path; continue           |
/// | Partial   | Plagal         | Active        | graded auto-apply; loop advances       |
/// | Partial   | Authentic      | Active        | high-confidence partial; continue      |
/// | Partial   | Half           | Waiting       | paused on V; awaits next consent tick  |
/// | Partial   | Deceptive      | Waiting       | low-confidence partial; awaits tick    |
/// | Failure   | (any)          | Escalated     | external resolution required           |
///
/// Threshold call: the substrate's `partial+plagal → active`,
/// `partial+half → waiting` mapping rides cadence_kind's variant
/// directly. The verdict-shape carries the structural distinction;
/// the cadence_kind carries the trajectory distinction; together
/// they collapse the four-state verdict × four-state cadence cross-
/// product onto the five-state oscillation_state surface
/// exhaustively. No fallback arm; the compiler enforces the contract.
pub fn read_consent(v: &Verdict, k: CadenceKind) -> OscillationState {
    match (v, k) {
        // Authentic + Success: the canonical closure.
        (Imperfect::Success(()), CadenceKind::Authentic) => OscillationState::Settled,
        // Success on any non-authentic kind: harmonic alt path; the
        // loop keeps moving (the consonant motion is real, the
        // substrate just hasn't reached the canonical tonic yet).
        (Imperfect::Success(()), _) => OscillationState::Active,
        // Partial + Half: paused on V; awaits next consent surface
        // tick; resumes WITHOUT external resolution.
        (Imperfect::Partial((), _), CadenceKind::Half) => OscillationState::Waiting,
        // Partial + Deceptive: substrate chose dissonance over
        // consonance, but only partially; wait for next tick rather
        // than escalating (escalation requires a hard Failure verdict).
        (Imperfect::Partial((), _), CadenceKind::Deceptive) => OscillationState::Waiting,
        // Partial + Plagal: graded auto-apply at high confidence; loop
        // advances through `active` per oscillate.mirror's table.
        (Imperfect::Partial((), _), CadenceKind::Plagal) => OscillationState::Active,
        // Partial + Authentic: high-confidence partial reading of the
        // canonical resolution; continue.
        (Imperfect::Partial((), _), CadenceKind::Authentic) => OscillationState::Active,
        // Failure: pause_event MUST be emitted to metalogue; consent
        // surface MUST resolve externally. Cadence_kind discarded.
        (Imperfect::Failure(_, _), _) => OscillationState::Escalated,
    }
}

/// `pulse(o: oscillation) -> oscillation` — one ACTIVE→DARK alternation.
///
/// The atomic step of the kintsugi loop. Composes
/// `active_pass → query_phi → dark_pass → read_consent` into ONE
/// half-cycle of the oscillation:
///
/// 1. `active_pass(o)`      — propose a loss-decreasing morphism (stub).
/// 2. `query_phi(m)`        — gate + rank → consent verdict (stub).
/// 3. `dark_pass(o, m)`     — anchor identity; emit next oscillation
///    (stub; reads `read_consent` for the next state).
///
/// Emits the next [`Oscillation`] with `iteration += 1`, `state`
/// updated per `read_consent`, and `anchor` carried through (the stub
/// `dark_pass` leaves the anchor unchanged; T12's body advances on
/// identity-preserving morphisms).
///
/// Idempotency on settled / escalated: when the input oscillation is
/// already at a closure state, the chain still runs (active_pass over
/// a settled anchor produces a fixture morphism whose query_phi yields
/// the closure verdict; dark_pass's read_consent re-reads the closure).
/// The iteration advances by one regardless — the substrate's
/// monotonic-tick discipline holds whether or not the loop has settled.
/// The driver (`oscillate`, T13+) reads `is_complete` AFTER each pulse
/// to decide termination; pulse itself is the pure contraction step.
///
/// Per spec §2.1: pulse IS the loop's contraction step. Each pulse is
/// one application of the contraction map; the iteration count IS the
/// contraction step index.
pub fn pulse(o: &Oscillation) -> Oscillation {
    // Step 1: propose a loss-decreasing morphism (T11 stub).
    let morphism = active_pass(o);
    // Steps 2-4: anchor identity, composing query_phi + read_consent
    // through dark_pass (T12 stub). dark_pass emits the next
    // oscillation with state, iteration, and anchor updated.
    dark_pass(o, &morphism)
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`is_complete`] — T4's `oscillate`
    //! body. RED first; the body lands GREEN in the next commit.
    //!
    //! Per `shards/mirror/spectral/oscillate.mirror`'s `is_complete`
    //! declared mapping (§is_complete, lines 627–669): the loop's
    //! position projects onto the substrate's three-state verdict
    //! surface. The five-state oscillation_state collapses onto the
    //! three-state verdict cleanly:
    //!
    //!   - `settled`             → `Success(())`              (pass)
    //!   - `active`              → `Partial((), opaque)`      (partial; mid-cycle ACTIVE)
    //!   - `dark`                → `Partial((), opaque)`      (partial; mid-cycle DARK)
    //!   - `waiting`             → `Partial((), opaque)`      (partial; half-cadence)
    //!   - `escalated`           → `Failure(Gap, opaque)`     (failure)
    //!
    //! Per AGENTS.md "TDD": these tests RED first; the body lands
    //! GREEN in the next commit. Drift on either side breaks them.

    use super::*;
    use crate::gap::{confidence_of, Gap};
    use terni::Imperfect;

    /// `settled` is the autopoietic ground state — `Success(())`.
    /// Per `oscillate.mirror` §is_complete: "Loop completed cleanly;
    /// holonomy → 0; final ref emitted; driver returns."
    #[test]
    fn settled_is_success_unit() {
        let v = is_complete(OscillationState::Settled);
        assert!(matches!(v, Imperfect::Success(())));
        assert!(v.is_ok());
        assert!(!v.is_partial());
        assert!(!v.is_err());
    }

    /// `active` is mid-cycle, ACTIVE pass live — `Partial` at confidence 0.5.
    /// Per `oscillate.mirror` §is_complete: "Loop mid-cycle in ACTIVE phase;
    /// driver continues with next pulse."
    #[test]
    fn active_is_partial_with_mid_confidence() {
        let v = is_complete(OscillationState::Active);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "active transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    (c - 0.5).abs() < 1e-9,
                    "active confidence must be 0.5 (got {c})",
                );
            }
            other => panic!("active must yield Partial; got {other:?}"),
        }
        assert!(v.is_ok());
        assert!(v.is_partial());
        assert!(!v.is_err());
    }

    /// `dark` is mid-cycle, DARK pass live — `Partial` at confidence 0.5.
    /// Per `oscillate.mirror` §is_complete: "Loop mid-cycle in DARK phase;
    /// driver continues with next pulse." Same confidence as `active`:
    /// the substrate's distinction is which phase is live, not the
    /// loop's completeness.
    #[test]
    fn dark_is_partial_with_mid_confidence() {
        let v = is_complete(OscillationState::Dark);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "dark transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    (c - 0.5).abs() < 1e-9,
                    "dark confidence must be 0.5 (got {c})",
                );
            }
            other => panic!("dark must yield Partial; got {other:?}"),
        }
    }

    /// `waiting` is half-cadence; substrate paused at the dominant —
    /// `Partial` at confidence 0.25. Per `oscillate.mirror`
    /// §is_complete: "Loop paused mid-progression at half-cadence; will
    /// resume on next consent surface tick." The 0.25 mirrors
    /// `music/mod.rs`'s Half-cadence confidence exactly — the audible
    /// altitude and the oscillation altitude both read half-cadence as
    /// 0.25 (below 1/φ ≈ 0.618 — closer-to-failure-than-not).
    #[test]
    fn waiting_is_partial_with_half_cadence_confidence() {
        let v = is_complete(OscillationState::Waiting);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "waiting transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    (c - 0.25).abs() < 1e-9,
                    "waiting confidence must be 0.25 (got {c})",
                );
            }
            other => panic!("waiting must yield Partial; got {other:?}"),
        }
        assert!(v.is_partial());
    }

    /// `escalated` is failure — pause_event already emitted to metalogue.
    /// Per `oscillate.mirror` §is_complete: "Loop halted pending external
    /// resolution." Per `gap-tension-tensor-substrate.md` §3.2: escalation
    /// is a level-1 contradiction (simple unresolved tension; no nested
    /// learning loop).
    #[test]
    fn escalated_is_failure_with_level_one_gap() {
        let v = is_complete(OscillationState::Escalated);
        match &v {
            Imperfect::Failure(gap, t) => {
                assert_eq!(Gap::level(gap), 1, "escalation is a level-1 contradiction");
                assert!(
                    !t.is_catastrophic(),
                    "escalated transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(c < 0.25, "escalated confidence must be near zero (got {c})",);
            }
            other => panic!("escalated must yield Failure; got {other:?}"),
        }
        assert!(v.is_err());
        assert!(!v.is_ok());
    }

    /// Type-level: the substrate's three-state floor IS reachable as
    /// `crate::music::Verdict` (the same Hodge-framed triple T3 landed).
    /// Per `oscillate.mirror`'s §is_complete: "the verdict mapping IS the
    /// loop driver's termination discipline" — same verdict shape as
    /// `is_settled`, same altitude.
    #[test]
    fn is_complete_returns_music_verdict_shape() {
        let v: crate::music::Verdict = is_complete(OscillationState::Settled);
        // The type assertion is the test; the value just witnesses.
        assert!(matches!(v, Imperfect::Success(())));
    }

    // ================================================================
    // T10.5: the `pulse` body — one ACTIVE→DARK alternation.
    //
    // The chain-shape proof. These tests RED first; the body lands
    // GREEN under [substrate-pull:realize]. The stubs (active_pass,
    // dark_pass, query_phi, read_consent) are all here at the boundary
    // so the chain wires end-to-end with a fixture morphism.
    //
    // Reading the brief's pulse contract:
    //
    //   - pulse(Oscillation { state: Active, iteration: 0, anchor: r })
    //     yields Oscillation { state ∈ {settled, dark, escalated,
    //     waiting, active}, iteration: 1, anchor: r }
    //   - iteration advances by exactly one per pulse
    //   - the stub chain leaves the anchor unchanged (T12's dark_pass
    //     body will lift this when @uuid/spectral.dark substrate-pulls)
    //   - read_consent's table dispatches verdict×cadence_kind onto
    //     the five-state oscillation surface
    //   - the fixture morphism (active_pass stub) carries an authentic
    //     cadence expectation; query_phi runs it through is_settled;
    //     the result reads as Settled in read_consent
    // ================================================================

    use crate::music::{is_settled, Dissonance};
    use prism_core::Transparency;

    /// Fixture anchor ref for the pulse chain tests.
    fn fixture_anchor() -> Ref {
        Ref::new("@mirror/spectral/oscillate/fixture").expect("valid substrate ref")
    }

    // ----------------------------------------------------------------
    // Tick — the substrate's monotonic counter.
    // ----------------------------------------------------------------

    /// `Tick::zero` is the loop's starting position; count 0.
    #[test]
    fn tick_zero_is_count_zero() {
        assert_eq!(Tick::zero().count(), 0);
    }

    /// `Tick::advance` increments by exactly one monotonic unit.
    #[test]
    fn tick_advance_is_plus_one() {
        let t0 = Tick::zero();
        let t1 = t0.advance();
        assert_eq!(t1.count(), 1);
        assert_eq!(t1.advance().count(), 2);
    }

    /// `Tick::advance` saturates at u32::MAX per the monotonic
    /// discipline (no decrement; no overflow panic).
    #[test]
    fn tick_advance_saturates_at_u32_max() {
        let t = Tick::new(u32::MAX);
        assert_eq!(t.advance().count(), u32::MAX);
    }

    /// `Tick` ordering reflects the monotonic counter.
    #[test]
    fn tick_ordering_is_monotonic() {
        assert!(Tick::zero() < Tick::new(1));
        assert!(Tick::new(5) < Tick::new(10));
    }

    // ----------------------------------------------------------------
    // Oscillation — the loop's full live position.
    // ----------------------------------------------------------------

    /// `Oscillation::initial` puts the loop at Active / tick-zero /
    /// anchored on the initial ref per `oscillate.mirror` §oscillate
    /// step 1.
    #[test]
    fn oscillation_initial_is_active_zero_anchor() {
        let r = fixture_anchor();
        let o = Oscillation::initial(r.clone());
        assert_eq!(o.state(), OscillationState::Active);
        assert_eq!(o.iteration(), Tick::zero());
        assert_eq!(o.anchor(), &r);
    }

    /// `Oscillation` carries state/iteration/anchor through new().
    #[test]
    fn oscillation_new_carries_three_fields() {
        let r = fixture_anchor();
        let o = Oscillation::new(OscillationState::Dark, Tick::new(7), r.clone());
        assert_eq!(o.state(), OscillationState::Dark);
        assert_eq!(o.iteration(), Tick::new(7));
        assert_eq!(o.anchor(), &r);
    }

    // ----------------------------------------------------------------
    // Morphism — the consent-altitude carrier.
    // ----------------------------------------------------------------

    /// `Morphism::new` carries content/score/expected fields.
    #[test]
    fn morphism_new_carries_three_fields() {
        let m = Morphism::new(
            fixture_anchor(),
            Dissonance::new(0.42, 3),
            CadenceKind::Authentic,
        );
        assert_eq!(m.content(), &fixture_anchor());
        assert!((m.score().roughness() - 0.42).abs() < 1e-9);
        assert_eq!(m.score().partials(), 3);
        assert_eq!(m.expected(), CadenceKind::Authentic);
    }

    // ----------------------------------------------------------------
    // active_pass stub — fixture morphism anchored at the input.
    // ----------------------------------------------------------------

    /// The stub `active_pass` returns a morphism whose content
    /// references the input oscillation's anchor. The fixture is
    /// scored consonantly (roughness 0) and expects authentic cadence
    /// — enough to thread the chain without coupling to T11.
    #[test]
    fn active_pass_stub_returns_fixture_morphism_at_anchor() {
        let r = fixture_anchor();
        let o = Oscillation::initial(r.clone());
        let m = active_pass(&o);
        assert_eq!(m.content(), &r, "fixture morphism anchors at input ref");
        assert_eq!(m.score().roughness(), 0.0, "fixture is consonant");
        assert_eq!(m.expected(), CadenceKind::Authentic);
    }

    // ----------------------------------------------------------------
    // query_phi stub — dispatches through is_settled on m.expected().
    // ----------------------------------------------------------------

    /// `query_phi` over an authentic-expected morphism reads as
    /// `Success(())` (the canonical closure) per is_settled's mapping.
    #[test]
    fn query_phi_stub_on_authentic_morphism_is_success() {
        let m = Morphism::new(
            fixture_anchor(),
            Dissonance::new(0.0, 1),
            CadenceKind::Authentic,
        );
        let v = query_phi(&m);
        assert!(matches!(v, Imperfect::Success(())));
    }

    /// `query_phi` over a deceptive-expected morphism reads as
    /// `Failure` per is_settled's mapping (V → vi).
    #[test]
    fn query_phi_stub_on_deceptive_morphism_is_failure() {
        let m = Morphism::new(
            fixture_anchor(),
            Dissonance::new(0.9, 1),
            CadenceKind::Deceptive,
        );
        let v = query_phi(&m);
        assert!(matches!(v, Imperfect::Failure(_, _)));
    }

    // ----------------------------------------------------------------
    // read_consent — the four-to-five bridge.
    // ----------------------------------------------------------------

    /// `Success(()) + Authentic` → `Settled`. The canonical closure.
    #[test]
    fn read_consent_success_authentic_is_settled() {
        let v: Verdict = Imperfect::Success(());
        assert_eq!(
            read_consent(&v, CadenceKind::Authentic),
            OscillationState::Settled,
        );
    }

    /// `Success(()) + non-authentic` → `Active`. Harmonic alt path; loop
    /// keeps moving. Per oscillate.mirror's read_consent table: the
    /// `pass + plagal` row maps to active.
    #[test]
    fn read_consent_success_plagal_is_active() {
        let v: Verdict = Imperfect::Success(());
        assert_eq!(
            read_consent(&v, CadenceKind::Plagal),
            OscillationState::Active,
        );
    }

    /// `Partial + Plagal` → `Active`. Graded auto-apply at high
    /// confidence; loop advances.
    #[test]
    fn read_consent_partial_plagal_is_active() {
        let v: Verdict = Imperfect::Partial((), Transparency::clear());
        assert_eq!(
            read_consent(&v, CadenceKind::Plagal),
            OscillationState::Active,
        );
    }

    /// `Partial + Half` → `Waiting`. Half-cadence; substrate paused at
    /// the dominant; awaits next consent surface tick.
    #[test]
    fn read_consent_partial_half_is_waiting() {
        let v: Verdict = Imperfect::Partial((), Transparency::clear());
        assert_eq!(
            read_consent(&v, CadenceKind::Half),
            OscillationState::Waiting,
        );
    }

    /// `Partial + Authentic` → `Active`. High-confidence partial reading
    /// of the canonical resolution; loop continues.
    #[test]
    fn read_consent_partial_authentic_is_active() {
        let v: Verdict = Imperfect::Partial((), Transparency::clear());
        assert_eq!(
            read_consent(&v, CadenceKind::Authentic),
            OscillationState::Active,
        );
    }

    /// `Failure + any cadence_kind` → `Escalated`. Pause_event MUST be
    /// emitted; external resolution required.
    #[test]
    fn read_consent_failure_is_escalated_for_any_cadence() {
        let v: Verdict = Imperfect::Failure(
            Gap::new(1, oscillate_origin(), "deceptive"),
            Transparency::clear(),
        );
        for k in [
            CadenceKind::Authentic,
            CadenceKind::Plagal,
            CadenceKind::Half,
            CadenceKind::Deceptive,
        ] {
            assert_eq!(
                read_consent(&v, k),
                OscillationState::Escalated,
                "failure on {k:?} must escalate",
            );
        }
    }

    // ----------------------------------------------------------------
    // dark_pass — emits next oscillation with iteration += 1, anchor
    // unchanged (stub), state per read_consent.
    // ----------------------------------------------------------------

    /// `dark_pass` increments iteration by one.
    #[test]
    fn dark_pass_stub_advances_iteration() {
        let r = fixture_anchor();
        let o = Oscillation::new(OscillationState::Active, Tick::new(3), r.clone());
        let m = active_pass(&o);
        let next = dark_pass(&o, &m);
        assert_eq!(next.iteration(), Tick::new(4));
    }

    /// `dark_pass` (stub) leaves the anchor unchanged. T12 lifts this
    /// when @uuid/spectral.dark substrate-pulls; until then, the
    /// identity-preservation contract is the stub's default.
    #[test]
    fn dark_pass_stub_preserves_anchor() {
        let r = fixture_anchor();
        let o = Oscillation::new(OscillationState::Active, Tick::zero(), r.clone());
        let m = active_pass(&o);
        let next = dark_pass(&o, &m);
        assert_eq!(next.anchor(), &r);
    }

    /// `dark_pass` over the authentic-expected fixture morphism emits
    /// `Settled` (the consent read of Success(()) + Authentic).
    #[test]
    fn dark_pass_stub_on_authentic_fixture_is_settled() {
        let r = fixture_anchor();
        let o = Oscillation::initial(r);
        let m = active_pass(&o);
        let next = dark_pass(&o, &m);
        assert_eq!(next.state(), OscillationState::Settled);
    }

    // ----------------------------------------------------------------
    // pulse — the composer chain.
    //
    // The atomic step. T10.5's load-bearing proof: pulse threads
    // active_pass → query_phi → dark_pass → read_consent end-to-end.
    // ----------------------------------------------------------------

    /// `pulse` over the initial oscillation advances iteration by one.
    #[test]
    fn pulse_advances_iteration_by_one() {
        let o = Oscillation::initial(fixture_anchor());
        let next = pulse(&o);
        assert_eq!(next.iteration(), Tick::new(1));
    }

    /// `pulse` over an oscillation at tick 7 advances to tick 8.
    #[test]
    fn pulse_iteration_advances_from_arbitrary_tick() {
        let o = Oscillation::new(OscillationState::Active, Tick::new(7), fixture_anchor());
        let next = pulse(&o);
        assert_eq!(next.iteration(), Tick::new(8));
    }

    /// `pulse` preserves the anchor (stub `dark_pass` does not
    /// mutate; T12 lifts this when @uuid/spectral.dark pulls).
    #[test]
    fn pulse_preserves_anchor() {
        let r = fixture_anchor();
        let o = Oscillation::initial(r.clone());
        let next = pulse(&o);
        assert_eq!(next.anchor(), &r);
    }

    /// `pulse` over the initial oscillation lands at `Settled` via the
    /// stub chain (fixture morphism's authentic cadence → Success(())
    /// → read_consent picks Settled). This proves the composer chain
    /// end-to-end: active_pass → query_phi → dark_pass → read_consent.
    #[test]
    fn pulse_composer_chain_yields_settled_on_authentic_fixture() {
        let o = Oscillation::initial(fixture_anchor());
        let next = pulse(&o);
        assert_eq!(
            next.state(),
            OscillationState::Settled,
            "the composer chain must thread end-to-end and read Settled",
        );
    }

    /// `pulse` emits a state in the five-element oscillation_state
    /// surface — any oscillation in, an oscillation in the closed set
    /// of five states out. The substrate's exhaustive surface holds.
    #[test]
    fn pulse_output_state_is_in_five_state_surface() {
        let o = Oscillation::initial(fixture_anchor());
        let next = pulse(&o);
        match next.state() {
            OscillationState::Active
            | OscillationState::Dark
            | OscillationState::Settled
            | OscillationState::Escalated
            | OscillationState::Waiting => {}
        }
    }

    /// `pulse` is idempotent on the anchor (the stub chain does not
    /// mutate the ref; running pulse repeatedly only advances iteration
    /// and updates state per the verdict).
    #[test]
    fn pulse_is_idempotent_on_anchor_across_iterations() {
        let r = fixture_anchor();
        let o0 = Oscillation::initial(r.clone());
        let o1 = pulse(&o0);
        let o2 = pulse(&o1);
        let o3 = pulse(&o2);
        assert_eq!(o1.anchor(), &r);
        assert_eq!(o2.anchor(), &r);
        assert_eq!(o3.anchor(), &r);
        assert_eq!(o1.iteration(), Tick::new(1));
        assert_eq!(o2.iteration(), Tick::new(2));
        assert_eq!(o3.iteration(), Tick::new(3));
    }

    /// Type-level: `pulse` returns `Oscillation`. The substrate's
    /// carrier shape holds at the Rust boundary.
    #[test]
    fn pulse_returns_oscillation_shape() {
        let o = Oscillation::initial(fixture_anchor());
        let next: Oscillation = pulse(&o);
        assert_eq!(next.iteration().count(), 1);
    }

    /// The full chain proof: pulse composes the four stubs in order.
    /// active_pass returns a morphism; query_phi turns it into a
    /// verdict via is_settled; dark_pass reads the verdict + cadence
    /// through read_consent; the next oscillation carries the result.
    /// This test asserts the composition is observable end-to-end by
    /// reconstructing it from the four boundary stubs.
    #[test]
    fn pulse_chain_matches_explicit_stub_composition() {
        let o = Oscillation::initial(fixture_anchor());
        // Run the chain explicitly:
        let m = active_pass(&o);
        let v = query_phi(&m);
        let k = crate::gap::verdict_to_cadence_kind(&v);
        let expected_state = read_consent(&v, k);
        let expected_iter = o.iteration().advance();
        // Run pulse:
        let next = pulse(&o);
        // The two must match (the chain shape).
        assert_eq!(next.state(), expected_state);
        assert_eq!(next.iteration(), expected_iter);
        assert_eq!(next.anchor(), o.anchor());
        // And the chain witness: an authentic-expected fixture yields
        // is_settled(Authentic) = Success(()); read_consent reads that
        // with the Authentic cadence_kind to Settled.
        let witness = is_settled(CadenceKind::Authentic);
        assert!(matches!(witness, Imperfect::Success(())));
        assert_eq!(expected_state, OscillationState::Settled);
    }
}
