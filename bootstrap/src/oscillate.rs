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

use crate::ast::AstNode;
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

/// `active_pass(o: oscillation) -> morphism` — the proposal action.
///
/// **Graceful-default surface.** When the realisation layer has no
/// running AST context (the `pulse` driver path before the kintsugi
/// `<file>` driver pulls), this surface returns a settle-Authentic
/// morphism anchored at the input's anchor ref — the substrate-
/// honest "nothing to do; settle" reading per the score-shard gap on
/// the session-from-ref projection. The real chain lives at
/// [`active_pass_with_ast`]; downstream consumers with an AST in hand
/// (the kintsugi formatter; the future `mirror kintsugi <file>`
/// driver) call the AST-bearing form to get real fracture derivation.
///
/// Per `oscillate.mirror` §active_pass: the realisation reads the
/// anchor ref's candidate morphism_set (the eigenboard's pending
/// imperfections), invokes `dissonance.is_pareto` for the rank,
/// projects the highest-ranked candidate into a morphism. The
/// realisation honours the score-shard's gap: when no session is
/// readable from the running glue-bus state, the settle-Authentic
/// fixture surfaces (the substrate's "nothing pending" reading).
pub fn active_pass(o: &Oscillation) -> Morphism {
    // No AST in scope → no gaps → empty pending → graceful settle
    // (per the brief's "Empty pending set" design call).
    settle_morphism(o.anchor())
}

/// The settle-Authentic graceful-default morphism: anchored at the
/// input ref, scored consonantly, expected to resolve authentically.
/// Surfaces when the realisation has no fracture data to thread (empty
/// pending; no gaps in any candidate).
fn settle_morphism(anchor: &Ref) -> Morphism {
    Morphism::new(
        anchor.clone(),
        Dissonance::new(0.0, 1),
        CadenceKind::Authentic,
    )
}

/// `active_pass_with_ast(o: oscillation, ast: ast) -> morphism` — the
/// real proposal action (T11 substrate-pull realisation).
///
/// **🟢 [substrate-pull:realize] T11 GREEN body.** Composes the
/// substrate-altitude chain per `score.mirror` lines 387–395:
///
/// 1. `score_of(o, ast)` — project oscillation + AST into a Score.
/// 2. `pending(&score)` — read the candidate morphism set.
/// 3. `gaps_for_pending(ast)` — derive the gap basis the tensor reads.
/// 4. `tensor_of(gaps)` — build the gap-tensor field (T6/T8.5).
/// 5. `minimize(&tensor)` — SDRF-ranked fractures (T7/T9).
/// 6. Project the head fracture into a [`Morphism`] whose:
///    - `content` IS the head fracture's gap origin (the substrate
///      site where the descent step would land);
///    - `score` carries the descent-derived dissonance reading
///      (`roughness = 1.0 − descent`; higher descent → lower
///      roughness → closer to consonant; `partials = level + 1`);
///    - `expected` reads the cadence the formatter anticipates per
///      the descent threshold (see [`expected_cadence_for_descent`]).
///
/// ## Edge cases
///
/// - **Empty pending** (no gaps in AST) → [`settle_morphism`] anchored
///   at the oscillation's anchor; the substrate's "nothing to do;
///   settle" reading.
/// - **Single gap** → trivial MUS-graph (K₁); `minimize` emits no
///   fractures; falls back to a settle-Authentic morphism anchored at
///   the gap's substrate origin (the substrate is settled for THIS
///   gap; no contradiction edge to descend along; the substrate's
///   floor-altitude reading).
/// - **Multiple gaps** → SDRF-ranked fractures; head projects into
///   the proposed Morphism.
pub fn active_pass_with_ast(o: &Oscillation, ast: &AstNode) -> Morphism {
    // Step 1–2: score_of → pending. The substrate-altitude projection.
    let score = crate::score::score_of(o, ast);
    let pending_set = crate::score::pending(&score);

    // Edge: empty pending → graceful settle (the substrate-honest
    // "nothing to do" reading per the score-shard gap design call).
    if pending_set.is_empty() {
        return settle_morphism(o.anchor());
    }

    // Step 3–4: gaps_for_pending → tensor_of. The realisation reads
    // the gap basis from the AST again (substrate-pull discipline: no
    // Gap ↔ Morphism cross-reference; the realisation closes the
    // loop by reading the substrate).
    let gaps = crate::score::gaps_for_pending(ast);
    let tensor = crate::tensor::tensor_of(gaps);

    // Step 5: minimize — SDRF-ranked fractures by Balanced Forman
    // curvature (most-negative first; the substrate's bottleneck
    // edges surface first per Topping 2022 Algorithm 1).
    let fractures = crate::kintsugi::minimize(&tensor);

    // Edge: trivial sheaf (singleton vertex; no tensions; no
    // fractures) → settle-Authentic morphism anchored at the
    // FIRST pending morphism's content (the gap's substrate origin).
    // The substrate's floor-altitude reading: no descent gradient
    // available; the candidate IS the proposal.
    if fractures.is_empty() {
        return settle_morphism(pending_set[0].content());
    }

    // Step 6: project the head fracture into a Morphism. The head
    // IS the SDRF top-ranked fracture (steepest descent direction).
    let head = &fractures[0];
    let head_gap = crate::kintsugi::Fracture::gap(head);
    let descent = crate::kintsugi::Fracture::descent(head);
    // Descent → roughness inversion: high descent (strong gradient)
    // → low roughness (consonant; auto-apply); low descent (no
    // gradient) → high roughness (dissonant; pause). Partials carry
    // the gap's Bateson level + 1 so consumers see the substrate's
    // depth reading on the proposed morphism.
    let roughness = (1.0 - descent).clamp(0.0, 1.0);
    let partials = Gap::level(head_gap).saturating_add(1);
    let expected = expected_cadence_for_descent(descent);
    Morphism::new(
        head_gap.origin().clone(),
        Dissonance::new(roughness, partials),
        expected,
    )
}

/// Map a fracture's descent magnitude to the substrate's expected
/// resolution cadence per `consent.mirror`'s four-state cadence_kind.
///
/// Per the brief's design call (and consonant with the
/// `verdict_to_cadence_kind` 1/φ threshold per `gap.rs`):
///
/// - `descent ≥ 0.667` (strong gradient; bridge-edge SDRF reading) →
///   [`CadenceKind::Authentic`] — the formatter expects clean
///   resolution; auto-apply.
/// - `descent ≥ 0.5` (moderate gradient; neutral K₂ reading) →
///   [`CadenceKind::Plagal`] — graded auto-apply at high confidence.
/// - `descent ≥ 0.25` (mild gradient; paused-on-V reading) →
///   [`CadenceKind::Half`] — wait for the next consent surface tick.
/// - else (no gradient; well-connected K_n reading; `descent < 0.25`)
///   → [`CadenceKind::Deceptive`] — the substrate cannot resolve at
///   this altitude; escalate to the consent surface.
fn expected_cadence_for_descent(descent: f64) -> CadenceKind {
    if descent >= 0.667 {
        CadenceKind::Authentic
    } else if descent >= 0.5 {
        CadenceKind::Plagal
    } else if descent >= 0.25 {
        CadenceKind::Half
    } else {
        CadenceKind::Deceptive
    }
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

/// `dark_bits(r: &Ref) -> [u8; 10]` — substrate-pull realisation of
/// `@uuid/spectral.dark` at the loop boundary.
///
/// **🟢 [substrate-pull:realize] T12 GREEN body.** Projects a `Ref`
/// into the 80 DARK bits of its substrate identity per
/// `shards/uuid/spectral.mirror`'s 48 ACTIVE / 80 DARK golden-ratio
/// split. The realisation composes the three substrate primitives:
///
/// 1. BLAKE3-hash the ref's substrate path bytes (the substrate's
///    canonical content for an `@`-prefixed nav-ref; consonant with
///    the BLAKE3 discipline at `bootstrap/Cargo.toml` and the
///    `MerkleHash` default in `prism_core`).
/// 2. Compose into a [`SpectralUuid`] via [`SpectralUuid::from_parts`]
///    with `active = 0` (the boundary altitude has no quantized
///    `SpectralCoordinate<5>` per `@uuid/spectral` §11 OQ1 — the
///    quantized 48-bit arithmetic is forward-promised; the DARK
///    projection is independent of the ACTIVE arithmetic and lands
///    cleanly at active=0).
/// 3. Project the 80 DARK bits via [`SpectralUuid::dark`].
///
/// The check IS the substrate's identity-preservation discipline at
/// the loop altitude; two refs whose DARK bits byte-equal name the
/// same substrate identity (modulo BLAKE3 collisions on 80 bits ~ 2^-80
/// — vanishingly small at any realisation altitude).
///
/// Identity contract: pure function of the ref's substrate path; same
/// ref → same DARK bits. The realisation rides `SpectralUuid`'s
/// `from_parts` + `dark` discipline; no new substrate primitive.
///
/// [`SpectralUuid`]: prism_core::SpectralUuid
/// [`SpectralUuid::from_parts`]: prism_core::SpectralUuid::from_parts
/// [`SpectralUuid::dark`]: prism_core::SpectralUuid::dark
pub fn dark_bits(r: &Ref) -> [u8; 10] {
    let hash = blake3::hash(r.as_str().as_bytes());
    prism_core::SpectralUuid::from_parts(0, hash.as_bytes()).dark()
}

/// `dark_pass(o: oscillation, m: morphism) -> oscillation` — the
/// identity-anchor action.
///
/// **🟢 [substrate-pull:realize] T12 GREEN body.** Per
/// `oscillate.mirror` §dark_pass and `consent.mirror`
/// §identity_preserving: the realisation reads the 80 DARK bits of
/// the morphism's content and the oscillation's anchor via
/// [`dark_bits`], performs byte-equality, branches on the result:
///
/// - **Identity preserved** (DARK bits byte-equal) — the morphism does
///   not write a new substrate identity; chain through `query_phi →
///   read_consent` (per T10.5's wiring); emit the next oscillation
///   with the anchor advanced to `m.content` (per substrate's
///   §dark_pass: "next oscillation with the new anchor if preserved").
///   When `m.content == o.anchor` (the substrate's "no-op morphism"
///   case; the T11 graceful-default surface), the advance is observed
///   as anchor-unchanged.
///
/// - **Identity fractured** (DARK bits diverge) — the morphism would
///   write a new substrate identity (a different splinter set hashes
///   to a different uuid_spectral); the rug's pull would tear the
///   substrate. Emit the next oscillation with `state = Escalated`,
///   iteration advanced, anchor UNCHANGED (per substrate's §dark_pass:
///   "same anchor if violated; oscillation transitions to escalated").
///   The pause_event is forward-promised to `consent.emit_to_metalogue`
///   when the metalogue bridge lands.
///
/// Per the substrate's void duality: ACTIVE pulls (T11's
/// loss-decreasing pass) one corner of the rug; DARK pulls (T12's
/// identity-preservation check) the antipodal corner. Together they
/// realise the rough-wavy pull discipline at the Rust altitude; the
/// substrate either straightens (Settled) or surfaces a wrinkle the
/// consent surface must resolve (Escalated) or pauses mid-pull
/// awaiting more substrate (Waiting).
pub fn dark_pass(o: &Oscillation, m: &Morphism) -> Oscillation {
    let anchor_dark = dark_bits(o.anchor());
    let proposed_dark = dark_bits(m.content());
    if anchor_dark == proposed_dark {
        // Identity preserved: chain T10.5's verdict-shape composition;
        // anchor advances to m.content (per substrate's §dark_pass).
        let verdict = query_phi(m);
        let kind = crate::gap::verdict_to_cadence_kind(&verdict);
        let next_state = read_consent(&verdict, kind);
        Oscillation::new(next_state, o.iteration().advance(), m.content().clone())
    } else {
        // Identity fractured: emit Escalated; anchor unchanged.
        Oscillation::new(
            OscillationState::Escalated,
            o.iteration().advance(),
            o.anchor().clone(),
        )
    }
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

    // ================================================================
    // T11: active_pass_with_ast — the real chain.
    //
    // Per the brief: the chain composes
    //
    //   score_of → pending → gaps_for_pending → tensor_of →
    //   sheaf_laplacian → balanced_forman → minimize → head fracture
    //
    // into a Morphism whose content addresses the top-ranked fracture's
    // substrate origin, scored by the fracture's descent, expected per
    // the descent magnitude.
    //
    // RED tests cover: empty pending (graceful settle), single gap
    // (Morphism anchored at gap origin with descent-derived score),
    // multi-gap (top SDRF fracture surfaces), end-to-end chain.
    // ================================================================

    use crate::ast::{AstKind, DarkSpan};

    fn ast_with_no_gaps() -> AstNode {
        AstNode::new(AstKind::Focus, "witness")
    }

    fn ast_with_one_dark() -> AstNode {
        AstNode::dark("unknown bytes", DarkSpan { start: 0, end: 5 })
    }

    fn ast_with_three_darks() -> AstNode {
        let mut root = AstNode::new(AstKind::Project, "outer");
        root.add_child(AstNode::dark("a", DarkSpan { start: 0, end: 5 }));
        root.add_child(AstNode::dark("b", DarkSpan { start: 6, end: 12 }));
        root.add_child(AstNode::dark("c", DarkSpan { start: 13, end: 20 }));
        root
    }

    /// Empty pending (no gaps in AST) → graceful settle-Authentic
    /// morphism anchored at the input. The substrate's "nothing to
    /// do; settle" reading per the score-shard gap design call.
    #[test]
    fn active_pass_with_ast_on_empty_pending_returns_settle_morphism() {
        let r = fixture_anchor();
        let o = Oscillation::initial(r.clone());
        let ast = ast_with_no_gaps();
        let m = active_pass_with_ast(&o, &ast);
        assert_eq!(m.content(), &r, "empty pending settles at the input anchor");
        assert!(
            m.score().roughness() < 1e-9,
            "empty pending settles consonantly (roughness 0); got {}",
            m.score().roughness(),
        );
        assert_eq!(
            m.expected(),
            CadenceKind::Authentic,
            "empty pending expects authentic resolution (nothing to revise)",
        );
    }

    /// Single candidate with one gap → Morphism anchored at the gap's
    /// substrate origin, carrying fracture-derived score (SDRF
    /// descent on K₁ vertex is the singleton-baseline 0.0; expected
    /// reads as Deceptive because no descent gradient is available to
    /// auto-apply on).
    ///
    /// Per the brief: when there's only one candidate with a single
    /// gap, the MUS-graph is K₁; minimize emits no fractures (the
    /// substrate's gradient on a trivial sheaf is the additive
    /// identity). The graceful path returns a settle-Authentic
    /// morphism anchored at the gap origin (the substrate is settled
    /// for THIS gap; no contradiction edge to descend along).
    #[test]
    fn active_pass_with_ast_on_one_gap_returns_gap_anchored_morphism() {
        let o = Oscillation::initial(fixture_anchor());
        let ast = ast_with_one_dark();
        let m = active_pass_with_ast(&o, &ast);
        assert_eq!(
            m.content().as_str(),
            "@epistemologic/property/total_classification",
            "single-gap morphism anchors at the gap's substrate origin",
        );
    }

    /// Multiple candidates with multiple gaps → Morphism anchored at
    /// the TOP-ranked fracture's substrate origin. Three same-origin
    /// dark regions form K₃; SDRF curvature is uniform across edges
    /// (Ric = 2; descent = 0); source order preserved; top fracture
    /// addresses the first gap's substrate origin.
    #[test]
    fn active_pass_with_ast_on_multi_gaps_returns_top_fracture_morphism() {
        let o = Oscillation::initial(fixture_anchor());
        let ast = ast_with_three_darks();
        let m = active_pass_with_ast(&o, &ast);
        assert_eq!(
            m.content().as_str(),
            "@epistemologic/property/total_classification",
            "multi-gap morphism anchors at the top fracture's substrate origin",
        );
        // The chain ran end-to-end: a real Morphism shape (not
        // settle-fixture) carrying the SDRF reading.
    }

    /// The integration chain composes end-to-end: score_of → pending
    /// → gaps_for_pending → tensor_of → minimize → head → Morphism.
    /// The Morphism's shape matches the consent-altitude carrier; the
    /// chain is observable through the score and pending surfaces.
    #[test]
    fn active_pass_with_ast_integration_chain_composes() {
        use crate::kintsugi::minimize;
        use crate::score::{gaps_for_pending, pending, score_of};
        use crate::tensor::{tensor_of, Tensor};

        let o = Oscillation::initial(fixture_anchor());
        let ast = ast_with_three_darks();

        // Step 1–2: score_of → pending.
        let score = score_of(&o, &ast);
        let pending_set = pending(&score);
        assert_eq!(pending_set.len(), 3, "three gaps → three pending morphisms",);

        // Step 3–4: gaps_for_pending → tensor_of.
        let gaps = gaps_for_pending(&ast);
        assert_eq!(
            gaps.len(),
            3,
            "three pending morphisms expand to three gaps"
        );
        let tensor = tensor_of(gaps);
        assert_eq!(Tensor::vertices(&tensor).len(), 3);

        // Step 5: minimize.
        let fractures = minimize(&tensor);
        assert_eq!(
            fractures.len(),
            6,
            "K₃ tensor has 3 tensions; minimize emits 2 endpoints each",
        );

        // Step 6: active_pass_with_ast IS the projection of the head
        // fracture into a Morphism (chain-equivalence assertion).
        let m = active_pass_with_ast(&o, &ast);
        // The substrate origin of every gap from the same dark
        // sequence is the total_classification origin; the head
        // fracture's gap's origin IS what the morphism carries.
        assert_eq!(
            m.content().as_str(),
            "@epistemologic/property/total_classification",
        );
    }

    /// T10.5's pulse chain still works: active_pass (no-AST surface)
    /// returns the graceful settle morphism, and pulse advances the
    /// oscillation through the same authentic-cadence path. The T10.5
    /// shape contract is preserved.
    #[test]
    fn t10_5_pulse_chain_still_works_after_t11() {
        let o = Oscillation::initial(fixture_anchor());
        let next = pulse(&o);
        assert_eq!(next.iteration(), Tick::new(1));
        assert_eq!(next.anchor(), o.anchor());
        assert_eq!(
            next.state(),
            OscillationState::Settled,
            "the pulse chain still threads end-to-end and reads Settled",
        );
    }

    // ================================================================
    // T12: `dark_pass` real body — `@uuid/spectral.dark` substrate-pull.
    //
    // Per `shards/uuid/spectral.mirror` (substrate-FROZEN at the
    // 2026-06-06 hierarchical-UUID sweep): `uuid_spectral` is the
    // 128-bit ACTIVE/DARK record; `dark(u: uuid_spectral) ->
    // identity_signal` projects the 80 DARK bits — the BLAKE3-truncated
    // content-hash prefix. The Rust realisation lives in
    // `prism_core::SpectralUuid::dark() -> [u8; 10]`.
    //
    // Per `shards/mirror/spectral/consent.mirror` §identity_preserving:
    // "the morphism's resulting shard's identity_signal byte-equals the
    // pre-morphism shard's identity_signal; identity is preserved." The
    // check is byte-equality at the 80-DARK-bit boundary.
    //
    // Per `shards/mirror/spectral/oscillate.mirror` §dark_pass: "emits
    // the next oscillation with the new anchor (if preserved) or the
    // same anchor (if violated; oscillation transitions to escalated)."
    //
    // T12 lands the real check; the dark_bits derivation reads the
    // Ref's substrate path as the BLAKE3 content, anchors active=0
    // (the substrate has no quantized SpectralCoordinate<5> at the
    // boundary altitude per @uuid/spectral §11 OQ1), projects via
    // `SpectralUuid::dark()`. The check is byte-equality on [u8; 10].
    //
    //   identity-preserved   → m.content's DARK bits byte-equal o.anchor's
    //                          DARK bits → chain to query_phi → read_consent
    //                          → emit next oscillation with anchor advanced
    //                          to m.content (per substrate's §dark_pass).
    //   identity-fractured   → DARK bits diverge → emit oscillation with
    //                          state=Escalated, iteration advanced, anchor
    //                          unchanged (per substrate's §dark_pass and
    //                          consent.mirror's identity_preserving:failure).
    //
    // Edge cases per the brief:
    //   - empty content: Ref::new rejects empty refs at construction;
    //     unreachable at the dark_pass boundary (no test needed; the
    //     type-system already enforces).
    //   - identical anchor + morphism content: dark_bits trivially
    //     byte-equal; identity preserved (the substrate's "no-op
    //     morphism" reading).
    //   - divergent content: dark_bits differ (BLAKE3 collision
    //     probability is ~2^-80 ≈ 10^-24 — vanishingly small at the
    //     loop altitude; the substrate's "identity-fracturing morphism"
    //     reading).
    // ================================================================

    /// `dark_bits` over identical refs is byte-equal. The 80-DARK-bit
    /// projection is a pure function of the ref's substrate path; same
    /// path → same hash → same DARK bits.
    #[test]
    fn dark_bits_identical_refs_are_byte_equal() {
        let r = fixture_anchor();
        assert_eq!(dark_bits(&r), dark_bits(&r));
    }

    /// `dark_bits` over divergent refs diverge byte-wise. BLAKE3's
    /// collision probability on 80 bits is ~2^-80 ≈ 10^-24; substrate
    /// distinct paths produce distinct DARK bits in every observable
    /// realisation.
    #[test]
    fn dark_bits_divergent_refs_are_not_byte_equal() {
        let r1 = Ref::new("@mirror/spectral/oscillate/fixture-a").expect("valid");
        let r2 = Ref::new("@mirror/spectral/oscillate/fixture-b").expect("valid");
        assert_ne!(
            dark_bits(&r1),
            dark_bits(&r2),
            "distinct substrate refs must produce distinct DARK bits",
        );
    }

    /// `dark_bits` returns exactly 10 bytes (80 DARK bits per the
    /// substrate's 48/80 golden-ratio split in `shards/uuid/spectral.mirror`).
    #[test]
    fn dark_bits_is_eighty_bits() {
        let r = fixture_anchor();
        let bits = dark_bits(&r);
        // Type-level + value-level: [u8; 10] is 80 bits.
        let _: [u8; 10] = bits;
        assert_eq!(bits.len(), 10);
    }

    /// `dark_bits` on the substrate's well-known empty Ref-shape
    /// reads as the BLAKE3-of-the-path prefix — non-zero, since
    /// the ref carries a substrate path.
    #[test]
    fn dark_bits_on_substrate_ref_is_nonzero() {
        let r = fixture_anchor();
        let bits = dark_bits(&r);
        // The empty-shard sentinel is `SpectralUuid::EMPTY.dark()` —
        // the BLAKE3-of-empty-input prefix `af1349b9f5f9a1a6a040`.
        // Our path-derived bits MUST differ from EMPTY (non-empty
        // content cannot collide with the BLAKE3-of-empty prefix).
        assert_ne!(
            bits,
            prism_core::SpectralUuid::EMPTY.dark(),
            "non-empty substrate path must not hash to the empty-shard sentinel",
        );
    }

    /// `dark_pass` over an identity-preserving morphism (content
    /// byte-equal to anchor) advances iteration and emits the
    /// authentic-fixture's read_consent state.
    #[test]
    fn dark_pass_identity_preserved_advances_iteration() {
        let r = fixture_anchor();
        let o = Oscillation::new(OscillationState::Active, Tick::new(3), r.clone());
        // The morphism's content IS the anchor → DARK bits byte-equal
        // → identity preserved.
        let m = Morphism::new(r.clone(), Dissonance::new(0.0, 1), CadenceKind::Authentic);
        let next = dark_pass(&o, &m);
        assert_eq!(next.iteration(), Tick::new(4));
    }

    /// `dark_pass` over an identity-preserving morphism whose content
    /// IS the anchor's ref emits `Settled` (the read_consent of
    /// Success(()) + Authentic per the fixture morphism's expected
    /// cadence). Anchor is preserved because the new anchor IS
    /// m.content which equals the original anchor.
    #[test]
    fn dark_pass_identity_preserved_on_authentic_fixture_is_settled() {
        let r = fixture_anchor();
        let o = Oscillation::initial(r.clone());
        let m = Morphism::new(r.clone(), Dissonance::new(0.0, 1), CadenceKind::Authentic);
        let next = dark_pass(&o, &m);
        assert_eq!(next.state(), OscillationState::Settled);
        // Identity preserved means anchor advances to m.content; here
        // m.content IS the original anchor so the anchor is observed
        // unchanged. The substrate-altitude advance happened (per
        // oscillate.mirror §dark_pass); the substrate's "no-op
        // morphism" case bottoms out at anchor = m.content = o.anchor.
        assert_eq!(next.anchor(), &r);
    }

    /// `dark_pass` over an identity-FRACTURING morphism (content
    /// substrate-distinct from anchor) emits `Escalated` and leaves
    /// the anchor UNCHANGED. Per `oscillate.mirror` §dark_pass and
    /// `consent.mirror` §identity_preserving: a DARK-divergent
    /// morphism would write a new substrate identity; the substrate
    /// must surface the fracture to the consent surface for external
    /// resolution.
    #[test]
    fn dark_pass_identity_fractured_emits_escalated() {
        let anchor = Ref::new("@mirror/spectral/oscillate/anchor-a").expect("valid");
        let fractured_content = Ref::new("@mirror/spectral/oscillate/content-b").expect("valid");
        let o = Oscillation::new(OscillationState::Active, Tick::new(5), anchor.clone());
        let m = Morphism::new(
            fractured_content,
            Dissonance::new(0.0, 1),
            CadenceKind::Authentic,
        );
        let next = dark_pass(&o, &m);
        assert_eq!(
            next.state(),
            OscillationState::Escalated,
            "DARK-divergent morphism must escalate (identity fractured)",
        );
        assert_eq!(
            next.anchor(),
            &anchor,
            "identity-fractured dark_pass leaves the anchor unchanged",
        );
        assert_eq!(
            next.iteration(),
            Tick::new(6),
            "iteration advances by one regardless of identity-preservation verdict",
        );
    }

    /// `dark_pass` on identity-preserved + non-fixture content: the
    /// new anchor IS the morphism's content (the substrate's
    /// "morphism applies" reading per §dark_pass; the anchor advances
    /// to the new substrate position). This case is structurally
    /// rare at the boundary (a morphism whose content has the same
    /// DARK bits as the anchor but is a different Ref would need a
    /// BLAKE3 collision on 80 bits — ~2^-80) so the test directly
    /// constructs a morphism whose content IS the anchor to exercise
    /// the "anchor advances" code path without contriving a collision.
    #[test]
    fn dark_pass_identity_preserved_anchor_advances_to_content() {
        let anchor = Ref::new("@mirror/spectral/oscillate/start").expect("valid");
        let next_content = anchor.clone();
        let o = Oscillation::new(OscillationState::Active, Tick::new(7), anchor.clone());
        let m = Morphism::new(
            next_content.clone(),
            Dissonance::new(0.0, 1),
            CadenceKind::Authentic,
        );
        let next = dark_pass(&o, &m);
        assert_eq!(
            next.anchor(),
            &next_content,
            "identity-preserved dark_pass advances anchor to m.content",
        );
    }

    /// `dark_pass` identity-fractured branch escalates regardless of
    /// the morphism's expected cadence: the DARK byte-equality check
    /// short-circuits before the consent/verdict composition. A
    /// morphism whose expected cadence is Authentic STILL escalates
    /// when its content's DARK bits differ from the anchor's.
    #[test]
    fn dark_pass_identity_fractured_short_circuits_consent() {
        let anchor = Ref::new("@mirror/spectral/oscillate/anchor-x").expect("valid");
        let divergent = Ref::new("@mirror/spectral/oscillate/divergent-y").expect("valid");
        let o = Oscillation::initial(anchor.clone());
        // Authentic-expected morphism that would otherwise read as
        // Settled (per T10.5's chain on byte-equal content).
        let m = Morphism::new(divergent, Dissonance::new(0.0, 1), CadenceKind::Authentic);
        let next = dark_pass(&o, &m);
        assert_eq!(
            next.state(),
            OscillationState::Escalated,
            "identity fracture short-circuits the consent chain even on \
             authentic-expected morphisms",
        );
    }

    /// Pulse integration: when active_pass yields a morphism whose
    /// content IS the input anchor (the T11 graceful-default surface
    /// for the no-AST path), pulse runs through identity-preserved
    /// dark_pass and reads Settled. The chain composes the real
    /// dark_pass body with the rest of T10.5's logic.
    #[test]
    fn pulse_with_real_dark_pass_on_settle_morphism_reads_settled() {
        let o = Oscillation::initial(fixture_anchor());
        let next = pulse(&o);
        assert_eq!(next.state(), OscillationState::Settled);
        assert_eq!(next.iteration(), Tick::new(1));
        // Settle-morphism's content IS the anchor, so the advanced
        // anchor equals the original; same byte-shape, identity-
        // preserved branch taken.
        assert_eq!(next.anchor(), o.anchor());
    }

    /// Substrate-pull witness: `dark_bits` rides the substrate's
    /// existing `SpectralUuid::dark()` projection — the [u8; 10] read
    /// per `shards/uuid/spectral.mirror` §dark and the
    /// `prism_core::SpectralUuid` realisation. The check IS the
    /// substrate's byte-equality discipline at the loop boundary.
    #[test]
    fn dark_bits_matches_spectral_uuid_dark_projection() {
        let r = fixture_anchor();
        // The substrate's dark-bits derivation: BLAKE3 of the ref's
        // path bytes, then SpectralUuid::from_parts(0, &hash).dark().
        let hash = blake3::hash(r.as_str().as_bytes());
        let suuid = prism_core::SpectralUuid::from_parts(0, hash.as_bytes());
        let expected = suuid.dark();
        assert_eq!(
            dark_bits(&r),
            expected,
            "dark_bits MUST be SpectralUuid::dark() over BLAKE3(ref.as_str())",
        );
    }
}
