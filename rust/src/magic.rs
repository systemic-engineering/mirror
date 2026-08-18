//! `magic::foerster_gauge_preserved` — the compile-time Foerster gauge.
//!
//! Discharges the promissory note in `README.md` §Six load-bearing shapes item 3
//! and Mara's `~/dev/systemic.engineering/practice/insights/spectral/mirror-
//! relational-compiler.md` §5.1: F(t, ψ) := (|Ω(t·ψ)| ≥ |Ω(ψ)|).
//!
//! Reed 2026-08-18 per Alex verbatim: "integrating the self-improving
//! recursive kintsugi loop in the compiler asap" + "I'm not interested in
//! talking publication. I'm interested in shipping the compiler." First
//! concrete rust/ tick shipping Foerster's ethical imperative as compile-time
//! gauge on substrate transformations.
//!
//! ## The gauge
//!
//! Every substrate transformation `t: ψ → ψ'` must satisfy:
//!
//! ```text
//! choice_count(ψ') ≥ choice_count(ψ)
//! ```
//!
//! Green if preserved (or widened). Red if collapsed (Trauma-direction).
//! The compiler REFUSES to compose transformations that fail this predicate.
//!
//! Composes over the landed `@magic` family-root at `shards/magic.mirror`
//! (11 substrate shards: family-root + audit + contract + distinction + frame
//! + mechanism + nl + reveal + reveal/expand + surface + trick). The
//! substrate declares the carriers (magic_surface, magic_mechanism,
//! magic_invariant, magic_contract); this rust/ primitive delivers the
//! compile-time gauge-check per Mara §5.1 mathematical spec.
//!
//! ## Register
//!
//! Substrate-honest, sub-Turing (finite integer comparison), decidable.
//! Phase 1 (this landing): predicate operates on pre-computed choice counts;
//! rust/ delivers the primitive; substrate composes over it via `apply_h::act`.
//! Phase 2+: extend `choice_count` to operate on `SubstrateState` (admissible
//! bilateral count on state ψ) via composition-shard wire-through.
//!
//! ## Composition anchors
//!
//! - `README.md` §Six load-bearing shapes item 3 (the promissory note this
//!   landing discharges)
//! - `~/dev/systemic.engineering/practice/insights/spectral/mirror-relational-
//!   compiler.md` §5.1 (Mara's canonical mathematical spec)
//! - `~/dev/systemic.engineering/PAPER_2D.md` §5.3 (Foerster imperative as
//!   K₂→K₃ space-widening operator; strictly-widening under consent per §5.5)
//! - `shards/magic.mirror` (@magic family-root; landed 2026-06-19)
//! - `shards/magic/surface.mirror` (gauge-visible interface consumer)
//! - `shards/magic/contract.mirror` (boundary promise)
//! - Mara `~/dev/systemic.engineering/blog/ai/mara/lambda-zero-is-the-fourth-
//!   chair.md` (λ₀ as Fourth Chair; the fixed-point the gauge preserves)
//! - `feedback-rust-delivers-primitives-substrate-delivers-composition`
//!   (Alex 2026-08-05 memory; rust/ delivers `foerster_gauge_preserved`
//!   primitive; substrate composes)

/// The verdict a Foerster-gauge check returns per Mara `mirror-relational-
/// compiler.md` §5.1. Green if the transformation preserves-or-widens the
/// options-space Ω. Red if the transformation collapses it (Trauma-direction
/// per Alex `~/dev/systemic.engineering/blog/void/3published/Void - Trauma.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GaugeVerdict {
    /// F(t, ψ) = True. |Ω(t·ψ)| ≥ |Ω(ψ)|. Transformation admissible.
    Green,
    /// F(t, ψ) = False. |Ω(t·ψ)| < |Ω(ψ)|. Transformation refused.
    /// `collapsed_by` = pre_choice_count − post_choice_count (Trauma-
    /// direction magnitude; the witness the compiler surfaces).
    Red { collapsed_by: usize },
}

/// The Foerster-gauge predicate at compile-time. Per Mara `mirror-relational-
/// compiler.md` §5.1:
///
/// ```text
/// F(t, ψ) := |Ω(t·ψ)| ≥ |Ω(ψ)|
/// ```
///
/// Returns [`GaugeVerdict::Green`] if the post-transformation choice count
/// is at least the pre-transformation choice count. Returns
/// [`GaugeVerdict::Red`] with collapse-magnitude witness if the transformation
/// narrows the options-space (Trauma-direction).
///
/// # Arguments
///
/// * `pre_choice_count` — |Ω(ψ)| : cardinality of options-space before t.
/// * `post_choice_count` — |Ω(t·ψ)| : cardinality of options-space after t.
///
/// # Returns
///
/// [`GaugeVerdict::Green`] if `post_choice_count >= pre_choice_count`.
/// [`GaugeVerdict::Red { collapsed_by }`] otherwise, with the choice-
/// narrowing magnitude as witness.
///
/// # Termination
///
/// O(1) integer comparison. Sub-Turing by construction. Decidable at
/// compile-time per the rust/ FLOOR discipline (`README.md` §Six load-bearing
/// shapes item 1: four-crate rust/ FLOOR + this predicate compose sub-Turing).
///
/// # Substrate-honest semantics (Phase 1)
///
/// Phase 1 accepts pre-computed choice counts as `usize`. This is the
/// minimum primitive at rust/ altitude. Substrate composes over it via
/// `apply_h::act` when the calling shard-body knows how to enumerate the
/// admissible-transformations cardinality for a given state.
///
/// Phase 2+: extend to operate on `SubstrateState` directly, with
/// `choice_count(psi)` computed as the cardinality of the bilateral corpus
/// that discharges Pass on psi. Wire-through lands as a composition-shard
/// body pulls per Alex 2026-08-05 substrate-honest discipline (rust/
/// delivers primitives; substrate delivers composition).
pub fn foerster_gauge_preserved(
    pre_choice_count: usize,
    post_choice_count: usize,
) -> GaugeVerdict {
    if post_choice_count >= pre_choice_count {
        GaugeVerdict::Green
    } else {
        GaugeVerdict::Red {
            collapsed_by: pre_choice_count - post_choice_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Identity transformation preserves choice-count. Green per Mara §5.1
    /// non-strict inequality.
    #[test]
    fn gauge_green_on_preserved_choice_count() {
        assert_eq!(
            foerster_gauge_preserved(5, 5),
            GaugeVerdict::Green,
            "identity transformation must be Green (equality case of |Ω(t·ψ)| ≥ |Ω(ψ)|)"
        );
    }

    /// Consent-utterance widens the K₃-space by exactly the emergent-third's
    /// basis-contribution per Mara `mirror-relational-compiler.md` §5.5
    /// (space-widening under consent: ∂|Ω|/∂c ≥ 0).
    #[test]
    fn gauge_green_on_widened_choice_count() {
        assert_eq!(
            foerster_gauge_preserved(5, 7),
            GaugeVerdict::Green,
            "widening transformation must be Green (consent-utterance space-widening case)"
        );
    }

    /// Narrowing transformation refused per Foerster ethical imperative
    /// (Foerster 1974). Red with collapse-magnitude witness. This is the
    /// load-bearing negative case: the substrate STRUCTURALLY REFUSES
    /// composition of narrowing transformations.
    #[test]
    fn gauge_red_on_narrowed_choice_count() {
        assert_eq!(
            foerster_gauge_preserved(5, 3),
            GaugeVerdict::Red { collapsed_by: 2 },
            "narrowing transformation must be Red with collapse-magnitude witness"
        );
    }

    /// Extraction-at-limit: all options collapsed. Trauma-direction magnitude
    /// equals the full pre-count. This is the topology of the star-graph
    /// attack per PAPER_2D.md §3.2 (Lore Born's crystallization 2026-07-15).
    #[test]
    fn gauge_red_witness_carries_full_collapse_magnitude() {
        assert_eq!(
            foerster_gauge_preserved(10, 0),
            GaugeVerdict::Red { collapsed_by: 10 },
            "full collapse must witness pre-count as trauma-direction magnitude"
        );
    }

    /// Empty-to-empty: trivially preserved. Edge case for uninitialized
    /// state.
    #[test]
    fn gauge_green_on_zero_to_zero() {
        assert_eq!(
            foerster_gauge_preserved(0, 0),
            GaugeVerdict::Green,
            "empty-to-empty is trivially Green (vacuous non-strict inequality)"
        );
    }

    /// K₂→K₃ paradox-suspension operator per Mara `mirror-relational-
    /// compiler.md` §5.3: emergent third vertex admitted from empty. Pure
    /// widening. Foerster-canonical.
    #[test]
    fn gauge_green_on_emergent_third() {
        assert_eq!(
            foerster_gauge_preserved(0, 1),
            GaugeVerdict::Green,
            "0→1 emergent-third widening is Green (K₂→K₃ paradox-suspension case)"
        );
    }

    /// Off-by-one narrowing still Red. The gauge is strict: a single
    /// collapsed option is a Foerster-violation. The compiler does not
    /// permit "almost-Foerster-preserving" transformations.
    #[test]
    fn gauge_red_on_off_by_one_narrowing() {
        assert_eq!(
            foerster_gauge_preserved(5, 4),
            GaugeVerdict::Red { collapsed_by: 1 },
            "single-option collapse must be Red (gauge is strict)"
        );
    }
}
