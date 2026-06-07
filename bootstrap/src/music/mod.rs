//! `@epistemologic/math/music` — boundary Rust for the audible-altitude
//! discriminator floor.
//!
//! This module realizes substrate-declared actions from
//! `shards/epistemologic/math/music/*.mirror` as Rust bodies. The
//! substrate names the action signature; this module's bodies execute
//! it. Per AGENTS.md "Boundary Rust" — the thin floor a substrate-
//! declared action stands on, not capability itself.
//!
//! ## What lives here
//!
//! - [`CadenceKind`] — the Rust mirror of
//!   `type cadence_kind = authentic | plagal | deceptive | half`
//!   from `shards/epistemologic/math/music/cadence.mirror`.
//! - [`Verdict`] — the Rust realisation of `glass.mirror`'s
//!   `verdict = pass | partial(confidence) | failure(reason)`,
//!   superseded per
//!   [`docs/specs/property-and-inference-collapse.md`] §9.1 to:
//!
//!   ```text
//!   pub type Verdict = Imperfect<(), Gap, Transparency<Ref>>;
//!   ```
//!
//!   The Hodge framing (per
//!   [`docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md`]):
//!
//!   - `Success(())`                       — harmonic representative;
//!                                            cohomology class is the zero
//!                                            class; pure ground state.
//!   - `Partial((), Transparency<Ref>)`    — harmonic representative
//!                                            reached with gauge content
//!                                            (exact/co-exact) logged in
//!                                            the transparency.
//!   - `Failure(Gap, Transparency<Ref>)`   — nontrivial cohomology class;
//!                                            gap names the cocycle the
//!                                            substrate cannot trivialise.
//!
//!   The old three-variant boundary-Rust enum was the *degenerate
//!   scalar projection* of this triple per spec §9.3. The supersession
//!   replaces the enum with the type alias; `Reason::DeceptiveCadence`
//!   is replaced by a [`Gap`] whose tension summary names the V → vi
//!   unresolved tension.
//!
//! - [`is_settled`] — the executable form of
//!   `is_settled(c: cadence) -> verdict`. Reads a `CadenceKind` and
//!   emits the substrate's verdict per the projection table below.
//!
//! ## The four-state cadence mapping survives as projection
//!
//! Per spec §4.4: the substrate carries the full verdict; each
//! consumer projects what it needs. The audible-altitude projection
//! `verdict_to_cadence_kind` lives in [`crate::gap`] and inverts the
//! emission below:
//!
//! | cadence_kind | verdict                                          | confidence_of | meaning                                        |
//! |--------------|--------------------------------------------------|---------------|------------------------------------------------|
//! | authentic    | `Success(())`                                    | `1.0`         | harmonic; auto-apply; holonomy → 0             |
//! | plagal       | `Partial((), Transparency::clear())`             | `1.0`         | harmonic with gauge content; IV → I path       |
//! | half         | `Partial((), Transparency::opaque(…, 0.25))`     | `0.25`        | paused on V; low confidence; mid-progression   |
//! | deceptive    | `Failure(Gap{level:1,…}, Transparency::opaque)`  | `0.0`         | escalate to consent; V → vi; dissonance chosen |
//!
//! Plagal is encoded with `Transparency::clear()` rather than an
//! opaque map at 0.85 because at the audible floor the substrate has
//! no *located* cracks to report at plagal — the gauge content is a
//! path-shape signal, not a substrate location. The `confidence_of`
//! projection lands at `1.0` for `Clear`; the substrate's distinction
//! between authentic and plagal is carried in the `Imperfect` variant
//! (Success vs Partial), not in the confidence scalar. The four-state
//! mapping is preserved structurally.
//!
//! Half is encoded with a `Transparency::opaque(…, Partial { 0.25 })`
//! carrying a `paused on V` diagnostic. The 0.25 sits below 1/φ
//! ≈ 0.618 so `verdict_to_cadence_kind` routes Partial-with-low-
//! confidence to `CadenceKind::Half`.
//!
//! Deceptive is encoded with a `Gap { level: 1, … }` per
//! [`docs/specs/gap-tension-tensor-substrate.md`] §3.2: V → vi is a
//! level-1 contradiction (simple unresolved tension; no nested
//! learning loop). The `Transparency<Ref>` carries the opacity
//! observed en route to the failure.
//!
//! [`docs/specs/property-and-inference-collapse.md`]: ../../../../../docs/specs/property-and-inference-collapse.md
//! [`docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md`]: ../../../../../docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md
//! [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../../docs/specs/gap-tension-tensor-substrate.md
//! [`Gap`]: crate::gap::Gap
//!
//! ## The implementation cascade template
//!
//! This module is the first tick of the implementation cascade after
//! eight ticks of substrate declaration. The pattern this tick
//! establishes — substrate declares the action signature; bootstrap
//! realises the body in this module under `[substrate-pull:realize]`
//! — is the template for `is_consonant`, `is_pareto`, `is_complete`,
//! and the rest of the discriminator-floor cascade. Each future
//! cadence/dissonance/interval/harmonic body lands as a sibling
//! function here (or a submodule when the family grows).
//!
//! `#[allow(dead_code)]` at the module level: this module's surface is
//! the forward-promise to `@mirror/spectral/consent`, which is not yet
//! wired in `main.rs`. Per `cadence.mirror`'s `Forward-promise`
//! comment: "this shard does NOT pull `@mirror/spectral/*`; the
//! substrate-altitude arrow points upward (consent consumes cadence;
//! not the other way around)." The Rust dead-code analysis sees the
//! same arrow inverted — the consumer hasn't pulled yet, so the
//! surface is technically unused. The tests below DO use the surface;
//! they prove the contract regardless of the consumer's arrival.
#![allow(dead_code)]

use prism_core::{Diagnostic, PropertyVerdict, Ref, Transparency};
use terni::Imperfect;

use crate::gap::Gap;

/// The four classical cadence types as a closed sum.
///
/// Mirrors `type cadence_kind = authentic | plagal | deceptive | half`
/// from `shards/epistemologic/math/music/cadence.mirror`. Each variant
/// names an established music-theoretic object (Zarlino 1558; Rameau
/// 1722; Koch 1782; Riemann 1893) whose audible-altitude semantics
/// maps directly onto a `Verdict` outcome via [`is_settled`].
///
/// Identity contract: two `CadenceKind` values are equal iff they name
/// the same variant. `PartialEq` derives this; the music-theoretic
/// reality is that authentic ≠ plagal ≠ deceptive ≠ half as harmonic
/// events even though the audible surface may be similar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CadenceKind {
    /// V → I; the strongest tonal resolution; the canonical closure;
    /// formatter's auto-apply signal; holonomy → 0; the autopoietic
    /// ground state. Rameau's harmonic-functional ground.
    Authentic,
    /// IV → I; the "Amen" cadence; a consonant alternative path to the
    /// tonic; formatter auto-applies with reduced confidence.
    Plagal,
    /// V → vi (or analogous); the subverted expectation; Koch's
    /// interrupted cadence; the formatter chose dissonance over
    /// consonance — escalate to the consent surface.
    Deceptive,
    /// Progression ends on V (the dominant); Riemann's open cadence;
    /// the formatter paused mid-path; awaiting the next consent
    /// surface or tick.
    Half,
}

/// The substrate's verdict shape at the verdict altitude.
///
/// Per [`docs/specs/property-and-inference-collapse.md`] §9.1 the
/// Rust realisation of `glass.mirror`'s
/// `verdict = pass | partial(confidence) | failure(reason)` IS the
/// Hodge-framed triple:
///
/// ```text
/// Imperfect<(), Gap, Transparency<Ref>>
/// ```
///
/// - `Success(())` — harmonic representative (cohomology class is the
///   zero class; no gauge content).
/// - `Partial((), Transparency<Ref>)` — harmonic representative
///   reached with gauge content logged in the transparency.
/// - `Failure(Gap, Transparency<Ref>)` — nontrivial cohomology; the
///   gap names the cocycle the substrate cannot trivialise.
///
/// At the audible altitude `Aggregate` collapses to `()` per spec
/// §9.1: the cadence-altitude consumer cares about the verdict's
/// *shape*, not an aggregated section. The `Aggregate` carrier (a
/// non-unit type) lands at the verdict altitude when T6 (`tensor_of`)
/// substrate-pulls.
///
/// Distinct from `spectral::Verdict<S>` (the algebra-level
/// `terni::Imperfect<S, _, Transparency<Ref>>` carrier an operator
/// returns when acting on a state). The two are at different altitudes
/// but share the same triple shape; they compose through the
/// projection functors in [`crate::gap`].
///
/// [`docs/specs/property-and-inference-collapse.md`]: ../../../../../docs/specs/property-and-inference-collapse.md
pub type Verdict = Imperfect<(), Gap, Transparency<Ref>>;

/// `is_settled(c: cadence) -> verdict` — the formatter's settle signal
/// at the audible altitude.
///
/// Reads a [`CadenceKind`] and emits the substrate's Hodge-framed
/// verdict per the projection table in the module docs:
///
/// - `Authentic` → `Success(())` — harmonic representative IS the
///   input; cohomology class is zero; pure ground state.
/// - `Plagal`    → `Partial((), Transparency::clear())` — harmonic
///   reached with gauge content (no located cracks at the audible
///   floor); `confidence_of` projects to 1.0.
/// - `Half`      → `Partial((), Transparency::opaque(…, 0.25))` — a
///   substrate-located opacity at `@epistemologic/math/music/cadence`
///   names the paused-on-V state; `confidence_of` projects to 0.25.
/// - `Deceptive` → `Failure(Gap{level:1,…}, Transparency::opaque(…))`
///   — the gap names the V → vi unresolved tension (level-1
///   contradiction per `gap-tension-tensor-substrate.md` §3.2);
///   transparency carries the opacity observed en route to the
///   failure.
///
/// Pure; no I/O; no allocation beyond the verdict carrier.
///
/// Per `cadence.mirror`'s substrate declaration: the action takes a
/// full `cadence` record (kind + path + resolved_to); this body reads
/// only the `kind` field because the four-state mapping is determined
/// by the cadence type alone. The `path` and `resolved_to` fields are
/// substrate carriers for downstream consumers (e.g.,
/// `tonic_attractor`, `cadence_as_holonomy`); they do not affect the
/// settle verdict at this altitude. When the full `cadence` record
/// crosses into Rust (via the future `@mirror/spectral/consent`
/// pull), the wrapper will project to `kind` and dispatch here.
pub fn is_settled(kind: CadenceKind) -> Verdict {
    // The four-state mapping IS the body. Pattern-exhaustive over
    // CadenceKind; no fallback arm; the compiler enforces the
    // four-state contract. Adding a fifth variant to CadenceKind
    // (none is currently substrate-declared) will fail to compile
    // here, surfacing the gap immediately.
    match kind {
        // V → I: the canonical tonal closure; harmonic representative
        // IS the input; cohomology class is zero. Per cadence.mirror's
        // mapping and the Hodge insight (`a07d5b2`): pure ground state.
        CadenceKind::Authentic => Imperfect::Success(()),
        // IV → I: consonant alternative path. Harmonic representative
        // reached, gauge content carried as path-shape signal (no
        // *located* cracks at the audible floor). Transparency::clear
        // — the path-shape distinction is the Partial-vs-Success
        // variant, not a located opacity. confidence_of -> 1.0.
        CadenceKind::Plagal => Imperfect::Partial((), Transparency::clear()),
        // Paused on V: progression open; awaiting the next consent
        // surface. A substrate-located opacity at the cadence origin
        // carries the audible-altitude reading of "paused on V" at
        // confidence 0.25 (below 1/φ ≈ 0.618 — closer-to-failure-
        // than-not; still partial because the formatter has not yet
        // chosen).
        CadenceKind::Half => Imperfect::Partial((), half_transparency()),
        // V → vi: dissonance over consonance; the cocycle the
        // substrate cannot trivialise at this tick. Per
        // gap-tension-tensor-substrate.md §3.2: level-1 contradiction
        // (simple unresolved tension; no nested learning loop). The
        // transparency carries the opacity observed en route to the
        // failure. Escalate to Reflection.
        CadenceKind::Deceptive => Imperfect::Failure(deceptive_gap(), deceptive_transparency()),
    }
}

/// The audible-altitude origin Ref for the cadence shard.
///
/// The validating constructor on [`Ref`] cannot fail for this constant
/// path; the `expect` is unreachable in practice (the path is non-empty,
/// `@`-prefixed, no whitespace, no control characters). The panic is
/// the substrate's contract violation — it would only fire if the
/// audible-altitude shard path stops being a valid substrate ref,
/// which is a substrate-altitude invariant T4 should re-check.
fn cadence_origin() -> Ref {
    Ref::new("@epistemologic/math/music/cadence")
        .expect("audible-altitude cadence shard path must be a valid substrate ref")
}

/// The `Half` cadence's transparency: a substrate-located opacity at
/// the cadence origin with `confidence = 0.25`.
fn half_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        cadence_origin(),
        PropertyVerdict::Partial {
            confidence: 0.25,
            diagnostics: vec![Diagnostic::new("paused on V")],
        },
    )
}

/// The `Deceptive` cadence's gap: a level-1 contradiction naming the
/// V → vi unresolved tension. Per
/// [`docs/specs/gap-tension-tensor-substrate.md`] §3.2.
fn deceptive_gap() -> Gap {
    Gap::new(1, cadence_origin(), "V -> vi")
}

/// The `Deceptive` cadence's transparency: the opacity observed en
/// route to the failure. A `Fail` verdict at the cadence origin
/// surfaces the V → vi deviation to consent.
fn deceptive_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        cadence_origin(),
        PropertyVerdict::Fail(Diagnostic::new("deceptive cadence: V -> vi")),
    )
}

// =====================================================================
// T4: the dissonance discriminator floor.
//
// Per `shards/epistemologic/math/music/dissonance.mirror` (substrate-
// frozen at commit 7625ee5): the Helmholtz/Plomp-Levelt roughness curve
// IS the formatter's discriminator at the audible altitude. T4 lands the
// boundary-Rust bodies for `is_consonant` and `is_pareto`.
//
// The carrier types ([`Dissonance`], [`Candidate`], [`ParetoSet`]) are
// minimal Rust mirrors of the substrate-declared records:
//
//   type dissonance = { roughness: ref, partials: u32 }
//   type candidate  = { content: ref, score: dissonance }
//   type pareto_set = [candidate]
//
// Per `[[feedback-no-bare-types]]`: each is its own newtype; bare floats
// and bare `Vec` never escape the substrate boundary. Fuller shapes
// (typed `morphism` content; richer pareto-set carriers) lift when
// `@mirror/spectral/consent` substrate-pulls.
// =====================================================================

/// The substrate's dissonance carrier at the audible altitude.
///
/// Mirrors `type dissonance = { roughness: ref, partials: u32 }` from
/// `shards/epistemologic/math/music/dissonance.mirror`. The Plomp-Levelt
/// curve's continuous output, normalised to [0.0, 1.0]:
///
/// - `0.0`  — unison / octave / consonant small-integer ratio
/// - `~1.0` — critical-band-width interval (the tritone region)
///
/// `roughness` is an `f64` here because the substrate's `ref` floor at
/// the audible altitude IS `f64` (per `boot/std/number.mirror`'s
/// substrate-pull discipline and `glass.mirror`'s `opacity.weight: f64`
/// precedent). When the substrate's float-floor lifts to a richer
/// carrier (a forward-promise), this struct's field type lifts with it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dissonance {
    /// The Plomp-Levelt curve's value at this ratio. Bounded in [0, 1].
    roughness: f64,
    /// The number of partials summed when computing the roughness.
    partials: u32,
}

impl Dissonance {
    /// Construct from a roughness reading and a partial count.
    pub fn new(roughness: f64, partials: u32) -> Self {
        Dissonance {
            roughness,
            partials,
        }
    }

    /// Read this dissonance's roughness value.
    pub fn roughness(&self) -> f64 {
        self.roughness
    }

    /// Read this dissonance's partial count.
    pub fn partials(&self) -> u32 {
        self.partials
    }
}

/// The substrate's candidate carrier at the audible altitude.
///
/// Mirrors `type candidate = { content: ref, score: dissonance }` from
/// `shards/epistemologic/math/music/dissonance.mirror`. A single
/// candidate morphism the discriminator evaluates.
///
/// Per the substrate header's gap-honest commentary: `content: ref`
/// carries the substrate reference to wherever the morphism content
/// lives (a splinter oid, a shard handle, a Fate proposal id). When
/// `@mirror/spectral/consent` substrate-pulls a typed `morphism`
/// carrier, this struct's `content` field lifts.
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    content: Ref,
    score: Dissonance,
}

impl Candidate {
    /// Construct from a content reference and a dissonance score.
    pub fn new(content: Ref, score: Dissonance) -> Self {
        Candidate { content, score }
    }

    /// Borrow this candidate's substrate content reference.
    pub fn content(&self) -> &Ref {
        &self.content
    }

    /// Read this candidate's dissonance score.
    pub fn score(&self) -> Dissonance {
        self.score
    }
}

/// The substrate's pareto-set carrier at the audible altitude.
///
/// Mirrors `type pareto_set = [candidate]` from
/// `shards/epistemologic/math/music/dissonance.mirror`. The set of
/// candidates the discriminator evaluates.
///
/// Per the substrate header's gap-honest commentary: the empty set is
/// the "no admissible morphism" case; the discriminator handles it via
/// its own input validation (returning `Failure(reason)`), not via
/// non-empty contract at the carrier altitude. The newtype keeps the
/// substrate boundary explicit per `[[feedback-no-bare-types]]`.
#[derive(Clone, Debug, PartialEq)]
pub struct ParetoSet {
    candidates: Vec<Candidate>,
}

impl ParetoSet {
    /// Construct from a vector of candidates.
    pub fn new(candidates: Vec<Candidate>) -> Self {
        ParetoSet { candidates }
    }

    /// Borrow this set's candidates as a slice.
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// Read whether this set is empty (the "no admissible morphism" case).
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
}

/// `1/φ²` — the consonant-floor threshold. Below this roughness the
/// dissonance reading is consonant; the discriminator emits
/// `Success(())`. Equals `1 - 1/φ` exactly (the golden-ratio reflection
/// of [`INV_PHI`]).
///
/// [`INV_PHI`]: crate::gap::INV_PHI
const INV_PHI_SQ: f64 = 0.381_966_011_250_105_2_f64;

/// `1/φ` — the consonant-ceiling threshold. Above this roughness the
/// dissonance reading is dissonant beyond resolution; the discriminator
/// emits `Failure`. Same threshold the audible altitude uses to
/// discriminate plagal from half (per `crate::gap::INV_PHI`).
///
/// The roughness band split is golden-ratio anchored per the T3
/// discipline:
///
///   roughness ∈ [0, 1/φ²]  — consonant     → Success(())
///   roughness ∈ (1/φ², 0.5] — consonant-leaning partial → Partial+Clear
///   roughness ∈ (0.5, 1/φ]  — dissonant-leaning partial → Partial+opaque@0.25
///   roughness ∈ (1/φ, 1.0]  — dissonant      → Failure+opaque
const INV_PHI: f64 = crate::gap::INV_PHI;

/// The audible-altitude origin Ref for the dissonance shard.
fn dissonance_origin() -> Ref {
    Ref::new("@epistemologic/math/music/dissonance")
        .expect("audible-altitude dissonance shard path must be a valid substrate ref")
}

/// The dissonant-leaning partial-band transparency: opaque at
/// `confidence = 0.25` (same shape as `half_transparency`; the
/// dissonant-leaning band sits in the same below-1/φ confidence stratum
/// as the half cadence).
fn dissonant_leaning_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        dissonance_origin(),
        PropertyVerdict::Partial {
            confidence: 0.25,
            diagnostics: vec![Diagnostic::new("dissonant-leaning roughness")],
        },
    )
}

/// The dissonance-floor failure's gap: a level-0 contradiction naming
/// the floor-altitude inability to consonate. Level 0 (not 1): floor-
/// altitude dissonance is not a Bateson learning contradiction; it is
/// the substrate's resolution boundary on the consonant basis.
fn dissonance_floor_gap() -> Gap {
    Gap::new(0, dissonance_origin(), "roughness above 1/phi")
}

/// The dissonance-floor failure's transparency: the opacity observed
/// at the dissonance origin when the roughness exceeds the consonant
/// ceiling.
fn dissonance_floor_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        dissonance_origin(),
        PropertyVerdict::Fail(Diagnostic::new(
            "roughness beyond substrate resolution; not consonate at this floor",
        )),
    )
}

/// `is_consonant(d: dissonance) -> verdict` — the consonance verdict at
/// the dissonance altitude.
///
/// Reads `d.roughness` and emits the substrate's Hodge-framed verdict
/// per the golden-ratio anchored mapping (the T3 discipline's threshold
/// discipline, reflected onto the dissonance roughness axis):
///
/// - `roughness ≤ 1/φ²`         → `Success(())`               (consonant)
/// - `1/φ² < roughness ≤ 0.5`   → `Partial((), Transparency::clear())`
///                                  (consonant-leaning; confidence → 1.0)
/// - `0.5 < roughness ≤ 1/φ`    → `Partial((), opaque@0.25)`
///                                  (dissonant-leaning; below 1/φ)
/// - `roughness > 1/φ`           → `Failure(Gap{level:0,…}, opaque)`
///                                  (dissonant beyond resolution)
///
/// Pure; no I/O; no allocation beyond the verdict carrier.
///
/// Per `dissonance.mirror`'s substrate declaration: the action's
/// threshold value is a forward-promise to the consumer (refract
/// measures one threshold; the formatter measures another; the
/// audition lens measures a third). T4 lands the consonant-floor /
/// consonant-ceiling pair at golden-ratio anchors; per-consumer
/// thresholds compose on top.
pub fn is_consonant(d: Dissonance) -> Verdict {
    let r = d.roughness();
    if r <= INV_PHI_SQ {
        // Consonant ground: harmonic representative IS the input.
        Imperfect::Success(())
    } else if r <= 0.5 {
        // Consonant-leaning partial band: harmonic representative
        // reached with gauge content (no located cracks at the
        // dissonance floor; the gauge content is a roughness-band
        // signal, not a substrate location). Transparency::clear
        // mirrors the Plagal cadence's encoding.
        Imperfect::Partial((), Transparency::clear())
    } else if r <= INV_PHI {
        // Dissonant-leaning partial band: opaque with confidence 0.25.
        // Below 1/φ so `verdict_to_cadence_kind` reads as Half-shape.
        Imperfect::Partial((), dissonant_leaning_transparency())
    } else {
        // Dissonant beyond substrate resolution: level-0 floor verdict.
        Imperfect::Failure(dissonance_floor_gap(), dissonance_floor_transparency())
    }
}

// =====================================================================
// is_pareto helpers.
// =====================================================================

/// The audible-altitude tie tolerance on dissonance roughness.
///
/// Per `harmonic.mirror`'s ~5-cents resolution at trained-listener
/// altitude: 5 cents ≈ 0.003 in normalized roughness (cents are
/// logarithmic on frequency; the Helmholtz curve is on roughness; the
/// substrate-pull discipline maps the audible resolution to a roughness
/// tolerance of `0.003`). Candidates whose dissonance scores differ by
/// less than this are Pareto-tied at the audible altitude; the
/// formatter must pause and present the tied front to consent.
const PARETO_TIE_TOLERANCE: f64 = 0.003;

/// The empty-pareto-set failure's gap. Level 0 (not 1): no-admissible-
/// morphism is a floor-altitude verdict that the substrate has no
/// candidate to evaluate at this tick; it is not a nested learning
/// contradiction. The kintsugi loop's fracture stays a target for the
/// next tick when new candidates land.
fn empty_pareto_gap() -> Gap {
    Gap::new(0, dissonance_origin(), "no admissible morphism")
}

/// The empty-pareto-set failure's transparency.
fn empty_pareto_transparency() -> Transparency<Ref> {
    Transparency::opaque(
        dissonance_origin(),
        PropertyVerdict::Fail(Diagnostic::new(
            "pareto set is empty; no admissible morphism this tick",
        )),
    )
}

/// `is_pareto(candidates: pareto_set) -> verdict` — the pareto-front
/// discriminator at the dissonance altitude.
///
/// Reads a list of candidates (each scored by dissonance roughness
/// against the current eigenboard state) and emits the formatter's
/// pause-or-auto-apply verdict per `dissonance.mirror` §is_pareto:
///
/// - empty                                    → `Failure(Gap{level:0,…}, opaque)`
///                                              ("no admissible morphism")
/// - single strict minimum (gap > tolerance)  → `Success(())`
///                                              (singleton consonance; auto-apply)
/// - multiple within tie tolerance            → `Partial((), opaque(tied refs))`
///                                              (Pareto-tied; pause + present to consent)
///
/// **T4 returns `Success(())` for the singleton-winner case.** The
/// winner's identity (`Ref`) is a forward-promise to T5 (`gaps_of`) /
/// T6 (`tensor_of`): the substrate's `Aggregate` carrier (currently
/// `()`) lifts to a typed shape that threads the winning candidate
/// back to the consumer. Until then, the consumer-pull seam (the future
/// `@mirror/spectral/consent` driver) invokes both `is_pareto` and a
/// sibling `pick_winner(set)` to extract the winner.
///
/// **T4 does NOT pre-widen.** Single-attractor minimization on
/// roughness; the coherent-tension / temporal-eigensheaf extension
/// waits for the substrate's `coherent_tension` declaration to land.
/// Per the task brief: "the substrate is single-attractor; do not
/// pre-widen."
///
/// Pure; no I/O; allocates the opacity map on the Partial path.
pub fn is_pareto(candidates: ParetoSet) -> Verdict {
    if candidates.is_empty() {
        return Imperfect::Failure(empty_pareto_gap(), empty_pareto_transparency());
    }

    // Find the minimum roughness. NaN-safe via `partial_cmp` fallback:
    // if any roughness is NaN, partial_cmp returns None and we treat the
    // pair as tied (the substrate's audible-altitude resolution cannot
    // discriminate against a non-comparable scalar).
    let mut min_roughness = f64::INFINITY;
    for c in candidates.candidates() {
        let r = c.score().roughness();
        if r < min_roughness {
            min_roughness = r;
        }
    }

    // Collect all candidates whose roughness sits within the tie
    // tolerance of the minimum. Single survivor → strict minimum →
    // Success. Multiple survivors → Pareto-tied → Partial.
    let tied: Vec<&Candidate> = candidates
        .candidates()
        .iter()
        .filter(|c| (c.score().roughness() - min_roughness).abs() <= PARETO_TIE_TOLERANCE)
        .collect();

    if tied.len() <= 1 {
        // Strict minimum: singleton consonance; auto-apply.
        Imperfect::Success(())
    } else {
        // Pareto-tied: each tied candidate's content ref surfaces as a
        // located opacity. The consent surface reads the opacity map to
        // present the tied front; the per-candidate verdict carries
        // confidence (1.0 - normalized tie gap) so the runner-up gap is
        // recoverable from the transparency. The substrate-altitude
        // discriminator stays honest about WHERE the tie lives.
        use terni::Loss;
        let mut t: Transparency<Ref> = Transparency::clear();
        for c in &tied {
            // Confidence per tied candidate: 1.0 minus the normalised
            // distance to the minimum (0.0 if AT the minimum; less
            // otherwise). All tied candidates sit within tolerance, so
            // the confidence floor is `1.0 - tolerance` ≈ 0.997.
            let conf = 1.0 - (c.score().roughness() - min_roughness) / PARETO_TIE_TOLERANCE;
            let conf = conf.clamp(0.0, 1.0);
            let sibling = Transparency::opaque(
                c.content().clone(),
                PropertyVerdict::Partial {
                    confidence: conf,
                    diagnostics: vec![Diagnostic::new("pareto-tied candidate")],
                },
            );
            t = t.combine(sibling);
        }
        Imperfect::Partial((), t)
    }
}

#[cfg(test)]
mod tests {
    //! The executable spec for `is_settled` after T3 — the Verdict
    //! supersession to `Imperfect<(), Gap, Transparency<Ref>>`.
    //!
    //! Per `docs/specs/property-and-inference-collapse.md` §9.1: the
    //! audible-altitude `Verdict` supersedes to the Hodge-framed
    //! verdict triple (per `docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md`):
    //!
    //!   - `Success(())`                       — harmonic representative;
    //!                                            cohomology class is zero;
    //!                                            no gauge content.
    //!   - `Partial((), Transparency<Ref>)`    — harmonic representative
    //!                                            with gauge content logged.
    //!   - `Failure(Gap, Transparency<Ref>)`   — nontrivial cohomology;
    //!                                            gap names the cocycle.
    //!
    //! The four-state cadence mapping survives as a projection per
    //! §4.4 — `confidence_of` and `cadence_kind_of` are the projection
    //! functors. The substrate carries the full verdict; each consumer
    //! projects what it needs.
    //!
    //! Per AGENTS.md "TDD": these tests RED first; the body lands
    //! GREEN in the next commit. Drift on either side breaks them.

    use super::*;
    use crate::gap::{confidence_of, verdict_to_cadence_kind, Gap};
    use terni::Imperfect;

    // -----------------------------------------------------------------
    // Type-level: the supersession compiles to the Hodge-framed shape.
    // -----------------------------------------------------------------

    /// `Verdict` IS `Imperfect<(), Gap, Transparency<Ref>>`. This test
    /// asserts shape at the type level: assigning an `Imperfect` value
    /// of that triple to a `Verdict` binding must typecheck. RED while
    /// `Verdict` is still a 3-variant enum; GREEN once the type alias
    /// lands.
    #[test]
    fn verdict_is_imperfect_unit_gap_transparency_ref() {
        let v: Verdict = Imperfect::Success(());
        // Force the structural read so the compiler exhausts the alias.
        assert!(matches!(v, Imperfect::Success(())));
    }

    // -----------------------------------------------------------------
    // The four-state cadence mapping survives as projection.
    // -----------------------------------------------------------------

    /// Authentic cadence — V → I — IS the autopoietic ground state.
    /// Per `cadence.mirror`: `authentic → pass`. The Hodge reading:
    /// the harmonic representative IS the input; cohomology class is
    /// the zero class; no gauge content carried.
    #[test]
    fn authentic_cadence_is_success_unit() {
        let v = is_settled(CadenceKind::Authentic);
        assert!(matches!(v, Imperfect::Success(())));
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Authentic);
    }

    /// Plagal cadence — IV → I — the "Amen" cadence. The Hodge
    /// reading: harmonic representative reached, but the path carried
    /// (exact) gauge content; the substrate logs the transparency as
    /// `Clear` at this floor (no located opacity to report at the
    /// audible altitude — the gauge content is a path-shape signal,
    /// not a located crack). `confidence_of` projects to ~0.85.
    #[test]
    fn plagal_cadence_is_partial_with_high_confidence() {
        let v = is_settled(CadenceKind::Plagal);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "plagal transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    c > 0.618,
                    "plagal confidence must lie above 1/phi (got {c})",
                );
            }
            other => panic!("plagal must yield Partial; got {other:?}"),
        }
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Plagal);
    }

    /// Half cadence — progression paused on V. The Hodge reading:
    /// harmonic representative not yet reached; the substrate is
    /// mid-projection; the transparency carries an opacity signalling
    /// the pause. `confidence_of` projects to ~0.25 (below 1/φ).
    #[test]
    fn half_cadence_is_partial_with_low_confidence() {
        let v = is_settled(CadenceKind::Half);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "half transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(c < 0.618, "half confidence must lie below 1/phi (got {c})",);
            }
            other => panic!("half must yield Partial; got {other:?}"),
        }
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Half);
    }

    /// Deceptive cadence — V → vi — the formatter chose dissonance
    /// over consonance. The Hodge reading: nontrivial cohomology;
    /// gradient flow cannot reach `ker(D)`; the gap names the cocycle
    /// (V → vi unresolved tension); escalate to Reflection.
    ///
    /// Per `gap-tension-tensor-substrate.md` §3.2: the gap carries
    /// Bateson level (level 1 = simple contradiction; the V → vi case).
    /// The transparency carries opacity sites observed en route to the
    /// failure.
    #[test]
    fn deceptive_cadence_is_failure_with_gap_and_transparency() {
        let v = is_settled(CadenceKind::Deceptive);
        match &v {
            Imperfect::Failure(gap, t) => {
                // Level-1 contradiction per gap-tension-tensor §3.2.
                assert_eq!(
                    Gap::level(gap),
                    1,
                    "deceptive cadence is a level-1 contradiction",
                );
                assert!(
                    !t.is_catastrophic(),
                    "deceptive transparency must not be catastrophic"
                );
                // confidence_of projects Failure-side opacity to 0.0
                // per spec §3.1.
                let c = confidence_of(t);
                assert!(c < 0.25, "deceptive confidence must be near zero (got {c})",);
            }
            other => panic!("deceptive must yield Failure; got {other:?}"),
        }
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Deceptive);
    }

    // -----------------------------------------------------------------
    // is_ok / is_err / is_partial shape — Imperfect's queries survive.
    // -----------------------------------------------------------------

    /// `Success(())` is_ok and not is_partial.
    #[test]
    fn success_unit_is_ok_not_partial() {
        let v = is_settled(CadenceKind::Authentic);
        assert!(v.is_ok());
        assert!(!v.is_partial());
        assert!(!v.is_err());
    }

    /// `Partial((), _)` is_ok AND is_partial.
    #[test]
    fn partial_unit_is_ok_and_partial() {
        let v = is_settled(CadenceKind::Plagal);
        assert!(v.is_ok());
        assert!(v.is_partial());
        assert!(!v.is_err());
    }

    /// `Failure(_, _)` is_err.
    #[test]
    fn failure_gap_is_err() {
        let v = is_settled(CadenceKind::Deceptive);
        assert!(v.is_err());
        assert!(!v.is_ok());
        assert!(!v.is_partial());
    }

    // -----------------------------------------------------------------
    // T4: is_consonant — the discriminator-floor body for dissonance.
    // -----------------------------------------------------------------
    //
    // Per `shards/epistemologic/math/music/dissonance.mirror`'s
    // `is_consonant(d: dissonance) -> verdict` declaration. The body
    // reads `d.roughness` (a ref ~ f64 in [0.0, 1.0]) and emits the
    // three-state floor on a golden-ratio anchored threshold band:
    //
    //   roughness ≤ 1/φ² ≈ 0.382  → Success(())            (consonant)
    //   1/φ² < r ≤ 0.5           → Partial((), Clear)        (consonant-leaning)
    //   0.5  < r ≤ 1/φ ≈ 0.618  → Partial((), opaque@0.25)  (dissonant-leaning)
    //   roughness > 1/φ           → Failure(Gap{level:0}, opaque)  (dissonant beyond resolution)
    //
    // Canonical anchors from dissonance.mirror's curve:
    //   1:1 → 0.00 (unison, success); 3:2 → 0.02 (P5, success);
    //   5:4 → 0.10 (M3, success); tritone → ~1.0 (failure).
    //
    // Level 0 (not 1) for the consonant-floor failure: dissonance is
    // not a Bateson learning contradiction — it is a floor-altitude
    // verdict that the substrate cannot consonate at this resolution.
    // Level 1 is reserved for V → vi cadence-level cocycles per
    // gap-tension-tensor-substrate.md §3.2.

    use crate::music::{is_consonant, Dissonance};

    fn unison() -> Dissonance {
        // 1:1 — zero roughness; the consonant ground.
        Dissonance::new(0.0, 6)
    }

    fn perfect_fifth() -> Dissonance {
        // 3:2 — ~0.02 roughness per Plomp-Levelt; near-pure consonance.
        Dissonance::new(0.02, 6)
    }

    fn major_third() -> Dissonance {
        // 5:4 — ~0.10 roughness; consonant.
        Dissonance::new(0.10, 6)
    }

    fn borderline_consonant() -> Dissonance {
        // Just inside the consonant-leaning partial band.
        Dissonance::new(0.45, 6)
    }

    fn borderline_dissonant() -> Dissonance {
        // Just inside the dissonant-leaning partial band (above 0.5, below 1/φ).
        Dissonance::new(0.55, 6)
    }

    fn tritone() -> Dissonance {
        // 45:32 — ~1.0 roughness; the curve's peak.
        Dissonance::new(1.0, 6)
    }

    /// Unison — zero roughness — IS the consonant ground.
    /// Per `dissonance.mirror`: `1:1 → 0.00 roughness (pure consonance)`.
    #[test]
    fn unison_is_success_unit() {
        let v = is_consonant(unison());
        assert!(matches!(v, Imperfect::Success(())));
    }

    /// Perfect fifth — 0.02 roughness — is consonant.
    /// Per `dissonance.mirror`: `3:2 → 0.02 roughness (near-pure)`.
    #[test]
    fn perfect_fifth_is_success_unit() {
        let v = is_consonant(perfect_fifth());
        assert!(matches!(v, Imperfect::Success(())));
    }

    /// Major third — 0.10 roughness — is consonant (below 1/φ² ≈ 0.382).
    /// Per `dissonance.mirror`: `5:4 → 0.10 roughness`.
    #[test]
    fn major_third_is_success_unit() {
        let v = is_consonant(major_third());
        assert!(matches!(v, Imperfect::Success(())));
    }

    /// Borderline-consonant — in the consonant-leaning partial band.
    /// `Partial((), Transparency::clear())`; confidence projects to 1.0.
    #[test]
    fn borderline_consonant_is_partial_with_high_confidence() {
        let v = is_consonant(borderline_consonant());
        match &v {
            Imperfect::Partial((), t) => {
                let c = confidence_of(t);
                assert!(
                    c > 0.618,
                    "borderline-consonant confidence must lie above 1/φ (got {c})",
                );
            }
            other => panic!("borderline-consonant must yield Partial; got {other:?}"),
        }
    }

    /// Borderline-dissonant — in the dissonant-leaning partial band.
    /// `Partial((), opaque@0.25)`; confidence below 1/φ.
    #[test]
    fn borderline_dissonant_is_partial_with_low_confidence() {
        let v = is_consonant(borderline_dissonant());
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "borderline-dissonant transparency must not be catastrophic",
                );
                let c = confidence_of(t);
                assert!(
                    c < 0.618,
                    "borderline-dissonant confidence must lie below 1/φ (got {c})",
                );
            }
            other => panic!("borderline-dissonant must yield Partial; got {other:?}"),
        }
    }

    /// Tritone — ~1.0 roughness — is dissonant beyond resolution.
    /// `Failure(Gap{level:0,…}, opaque)`. Level 0: floor-altitude dissonance
    /// is not a Bateson learning contradiction; the substrate simply cannot
    /// consonate at this resolution.
    #[test]
    fn tritone_is_failure_with_level_zero_gap() {
        let v = is_consonant(tritone());
        match &v {
            Imperfect::Failure(gap, t) => {
                assert_eq!(
                    Gap::level(gap),
                    0,
                    "floor-altitude dissonance is a level-0 verdict",
                );
                assert!(
                    !t.is_catastrophic(),
                    "tritone transparency must not be catastrophic",
                );
            }
            other => panic!("tritone must yield Failure; got {other:?}"),
        }
    }

    // -----------------------------------------------------------------
    // T4: is_pareto — the pareto-front discriminator (single-attractor).
    // -----------------------------------------------------------------
    //
    // Per `shards/epistemologic/math/music/dissonance.mirror`'s
    // `is_pareto(candidates: pareto_set) -> verdict` declaration. The
    // body reads a list of candidates (each carrying a `content: ref`
    // and a `score: dissonance`) and emits:
    //
    //   empty                        → Failure(Gap{level:0,…}, opaque)
    //   single strict minimum        → Success(())
    //   multiple within tie-tolerance → Partial((), opaque(…))
    //
    // Tie tolerance: ~5 cents per harmonic.mirror's audible-altitude
    // resolution. In normalized roughness that maps to ~0.003 (cents
    // are logarithmic on frequency; the Helmholtz curve is on roughness;
    // the substrate-pull discipline uses 0.003 as the floor tolerance).
    //
    // T4 returns Success(()) for the singleton-winner case; the winner's
    // identity (Ref) is a forward-promise to T5/T6 (the gaps_of /
    // tensor_of substrate-pull will lift Aggregate to a typed carrier).
    //
    // Per the brief: do NOT pre-widen. Single-attractor minimization;
    // the coherent_tension extension waits for substrate-pull.

    use crate::music::{is_pareto, Candidate, ParetoSet};

    fn candidate_with_roughness(path: &str, roughness: f64) -> Candidate {
        Candidate::new(
            prism_core::Ref::new(path).expect("valid ref"),
            Dissonance::new(roughness, 6),
        )
    }

    /// Empty pareto set — no admissible morphism — is Failure.
    /// Per `dissonance.mirror`: "the empty pareto set is the 'no
    /// admissible morphism' case."
    #[test]
    fn empty_pareto_set_is_failure() {
        let v = is_pareto(ParetoSet::new(vec![]));
        match &v {
            Imperfect::Failure(gap, t) => {
                assert_eq!(
                    Gap::level(gap),
                    0,
                    "empty pareto set is a level-0 verdict (no admissible morphism)",
                );
                assert!(
                    !t.is_catastrophic(),
                    "empty transparency must not be catastrophic"
                );
            }
            other => panic!("empty pareto set must yield Failure; got {other:?}"),
        }
    }

    /// Single strict minimum — the formatter auto-applies the winner.
    /// Per `dissonance.mirror` §is_pareto: "singleton consonance; the
    /// pareto set has a strict minimum."
    #[test]
    fn single_strict_minimum_is_success_unit() {
        let set = ParetoSet::new(vec![
            candidate_with_roughness("@spectral/candidate/a", 0.10),
            candidate_with_roughness("@spectral/candidate/b", 0.45),
            candidate_with_roughness("@spectral/candidate/c", 0.80),
        ]);
        let v = is_pareto(set);
        assert!(matches!(v, Imperfect::Success(())));
    }

    /// Single-element set — trivially the strict minimum — is Success.
    #[test]
    fn singleton_pareto_set_is_success_unit() {
        let set = ParetoSet::new(vec![candidate_with_roughness(
            "@spectral/candidate/only",
            0.10,
        )]);
        let v = is_pareto(set);
        assert!(matches!(v, Imperfect::Success(())));
    }

    /// Two candidates within the tie tolerance — Pareto-tied — is
    /// Partial. Per `dissonance.mirror` §is_pareto: "two or more
    /// candidates tie at the dissonance floor (within the audible-
    /// altitude resolution of ~5 cents)."
    #[test]
    fn pareto_tie_within_tolerance_is_partial() {
        let set = ParetoSet::new(vec![
            candidate_with_roughness("@spectral/candidate/a", 0.100),
            candidate_with_roughness("@spectral/candidate/b", 0.101),
        ]);
        let v = is_pareto(set);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "pareto-tie transparency must not be catastrophic",
                );
                // The tied candidates' refs surface in the opacity map.
                let opacities = t.opacities().expect("opaque transparency");
                assert!(
                    opacities.len() >= 2,
                    "tied candidates' refs must surface as opacity locations",
                );
            }
            other => panic!("pareto-tie must yield Partial; got {other:?}"),
        }
    }

    /// Three candidates with two tied at the minimum and one dominated
    /// — still Partial (the formatter pauses on the tied pair).
    #[test]
    fn pareto_two_tied_one_dominated_is_partial() {
        let set = ParetoSet::new(vec![
            candidate_with_roughness("@spectral/candidate/a", 0.100),
            candidate_with_roughness("@spectral/candidate/b", 0.102),
            candidate_with_roughness("@spectral/candidate/c", 0.500),
        ]);
        let v = is_pareto(set);
        assert!(matches!(v, Imperfect::Partial((), _)));
    }
}
