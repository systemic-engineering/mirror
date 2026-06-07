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
//! - [`Verdict`] — the Rust mirror of `glass.mirror`'s
//!   `verdict = pass | partial(confidence) | failure(reason)`. This is
//!   the three-state substrate-floor verdict surface, distinct from the
//!   algebra-level `Verdict<S>` in `spectral.rs` (which is
//!   `terni::Imperfect<S, _, Transparency<Ref>>`). The two are at
//!   different altitudes: this one is the consent-surface verdict the
//!   formatter reads; that one is the operator-action verdict an
//!   algebra element returns.
//! - [`Reason`] — substrate-aligned reason carrier for the failure
//!   case. `glass.mirror` types `reason` as `@nl` (natural language);
//!   the realised carrier is a small variant set the formatter knows
//!   how to escalate. `Reason::DeceptiveCadence` is the first variant;
//!   future bodies extend.
//! - [`is_settled`] — the executable form of
//!   `is_settled(c: cadence) -> verdict`. Reads a `CadenceKind` and
//!   emits the formatter's settle verdict per the substrate-declared
//!   four-state mapping.
//!
//! ## The four-state mapping (per `cadence.mirror`)
//!
//! | cadence_kind | verdict                  | meaning                                          |
//! |--------------|--------------------------|--------------------------------------------------|
//! | authentic    | `Pass`                   | auto-apply; full resolution; holonomy → 0        |
//! | plagal       | `Partial(0.85)`          | auto-apply, reduced confidence; IV → I path      |
//! | half         | `Partial(0.25)`          | wait; paused on V; low confidence; mid-progression |
//! | deceptive    | `Failure(DeceptiveCadence)` | escalate to consent; V → vi; dissonance chosen   |
//!
//! Confidence values: golden-ratio-anchored. `plagal = 0.85` and
//! `half = 0.25` sit on opposite sides of 1/φ ≈ 0.618 — plagal is
//! "closer-to-pass-than-not" (auto-apply at reduced confidence) and
//! half is "closer-to-failure-than-not" (wait, but not yet escalate).
//! These are the substrate's first surfaced confidence-tier values;
//! when `glass.mirror` later declares confidence-tier constants, lift.
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

/// The reason carrier for a `Verdict::Failure`.
///
/// `glass.mirror` types `reason = @nl` (natural language); the realised
/// carrier is a small variant set the formatter knows how to escalate.
/// Each variant names *why* the substrate yielded to consent; new
/// variants land as future bodies (e.g., `is_consonant`, `is_pareto`)
/// surface new failure modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    /// V → vi (or analogous); the formatter chose dissonance over
    /// consonance. Per `cadence.mirror`: "the deviation must be named
    /// to the consent surface." Per `docs/specs/mirror-spectral.md`
    /// §4.7: "the substrate emits pause(Φ)."
    DeceptiveCadence,
}

/// The substrate-floor verdict surface.
///
/// Mirrors `glass.mirror`'s
/// `verdict = pass | partial(confidence) | failure(reason)`. This is
/// the formatter's settle signal — the substrate's honest three-state
/// acknowledgement of its own settle boundary.
///
/// Distinct from `spectral::Verdict<S>` (the algebra-level
/// `terni::Imperfect<S, _, Transparency<Ref>>` carrier an operator
/// returns when acting on a state). The two are at different altitudes:
/// this is the consent-surface verdict the formatter reads; that is
/// the operator-action verdict the algebra returns. They compose, but
/// they do not coincide.
///
/// `confidence` is the substrate's `f64` carrier per
/// `glass.mirror`: `type confidence = f64`. The realisation discipline
/// is that confidence lies in `[0.0, 1.0]`; values outside that range
/// are realisation bugs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Verdict {
    /// Auto-apply; the spectrum has closed on its autopoietic ground
    /// state. The formatter commits; the tick advances. Per
    /// `docs/specs/mirror-spectral.md` §2.2: "loss-decreasing = auto."
    Pass,
    /// Auto-apply with reduced confidence (or wait, with low
    /// confidence). The `f64` is the confidence in `[0.0, 1.0]`.
    /// `cadence.mirror` distinguishes the auto-apply (plagal, high
    /// confidence) and wait (half, low confidence) variants by the
    /// magnitude of the confidence; the verdict surface itself is the
    /// same `Partial`.
    Partial(f64),
    /// Escalate to the consent surface. The substrate yields; the
    /// consent query MUST resolve before the next tick.
    Failure(Reason),
}

/// `is_settled(c: cadence) -> verdict` — the formatter's settle signal
/// at the audible altitude.
///
/// Reads a [`CadenceKind`] and emits the substrate's three-state
/// verdict per the four-state mapping declared in
/// `shards/epistemologic/math/music/cadence.mirror`:
///
/// - `Authentic` → `Verdict::Pass`
/// - `Plagal`    → `Verdict::Partial(0.85)`
/// - `Half`      → `Verdict::Partial(0.25)`
/// - `Deceptive` → `Verdict::Failure(Reason::DeceptiveCadence)`
///
/// Pure; no I/O; no allocation beyond the verdict.
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
        // V → I: the canonical tonal closure; holonomy → 0; the
        // autopoietic ground state. Per cadence.mirror's mapping.
        CadenceKind::Authentic => Verdict::Pass,
        // IV → I: consonant alternative path; auto-apply with reduced
        // confidence. 0.85 > 1/φ ≈ 0.618 — closer-to-pass-than-not.
        CadenceKind::Plagal => Verdict::Partial(0.85),
        // Paused on V: progression open; awaiting the next consent
        // surface. 0.25 < 1/φ — closer-to-failure-than-not; still
        // partial because the formatter has not yet chosen.
        CadenceKind::Half => Verdict::Partial(0.25),
        // V → vi: dissonance over consonance; escalate to consent.
        // Per cadence.mirror: "the deviation must be named to the
        // consent surface." The substrate yields.
        CadenceKind::Deceptive => Verdict::Failure(Reason::DeceptiveCadence),
    }
}

#[cfg(test)]
mod tests {
    //! The executable spec for `is_settled`.
    //!
    //! Each test asserts one row of the four-state mapping declared in
    //! `shards/epistemologic/math/music/cadence.mirror`. RED before the
    //! body lands (panics via `todo!()`); GREEN after.
    //!
    //! Per AGENTS.md "TDD": red first as a separate commit; green
    //! follows immediately. These four tests ARE the executable
    //! substrate-altitude contract — drift on either side breaks them.

    use super::*;

    /// Authentic cadence — V → I — IS the autopoietic ground state.
    /// Per `cadence.mirror`: `authentic → pass`. Holonomy → 0.
    #[test]
    fn authentic_cadence_passes() {
        assert_eq!(is_settled(CadenceKind::Authentic), Verdict::Pass);
    }

    /// Plagal cadence — IV → I — IS the "Amen" cadence; consonant but
    /// not the strongest signal. Per `cadence.mirror`: auto-apply with
    /// reduced confidence. Confidence committed at 0.85 (above 1/φ).
    #[test]
    fn plagal_cadence_partial_high_confidence() {
        assert_eq!(is_settled(CadenceKind::Plagal), Verdict::Partial(0.85),);
    }

    /// Half cadence — progression paused on V. Per `cadence.mirror`:
    /// awaiting the next consent surface; low confidence in the
    /// current candidate being final. Confidence committed at 0.25
    /// (below 1/φ).
    #[test]
    fn half_cadence_partial_low_confidence() {
        assert_eq!(is_settled(CadenceKind::Half), Verdict::Partial(0.25),);
    }

    /// Deceptive cadence — V → vi — the formatter chose dissonance
    /// over consonance. Per `cadence.mirror`: consent query NEEDED;
    /// the substrate yields. Failure variant: `DeceptiveCadence`.
    #[test]
    fn deceptive_cadence_fails_to_consent() {
        assert_eq!(
            is_settled(CadenceKind::Deceptive),
            Verdict::Failure(Reason::DeceptiveCadence),
        );
    }
}
