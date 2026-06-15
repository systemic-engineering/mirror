//! `@epistemologic/property/gap` — boundary Rust for the substrate's
//! gap algebra and the verdict-altitude projections.
//!
//! This module realizes substrate-declared types from
//! [`shards/epistemologic/property.mirror`] and the spec at
//! [`docs/specs/gap-tension-tensor-substrate.md`] as Rust bodies.
//! It also lands the projection functors named in
//! [`docs/specs/property-and-inference-collapse.md`] §3.1 / §4.4:
//!
//! - [`Gap`] — the Rust mirror of `gap = { claim, verifier, state }`.
//!   T3 lands a *minimal* shape (just enough to type the [`Failure`]
//!   variant of [`crate::music::Verdict`]); fuller shape lands when
//!   T5 (`gaps_of`) and T6 (`tensor_of`) substrate-pull.
//! - [`confidence_of`] — the scalar projection functor
//!   `transparency → [0, 1]` per the spec §3.1.
//! - [`verdict_to_cadence_kind`] — the audible-altitude projection
//!   `verdict → cadence_kind` per the spec §4.4. Survives as a
//!   *projection* from the substrate's full verdict, not as the
//!   primary surface.
//!
//! ## Why this lives here (not in `music/`, not in `prism/imperfect/`)
//!
//! The audible altitude is *one* consumer of the verdict; the
//! substrate uses `Gap` at the verdict altitude broadly (T4 will pull
//! it into the discriminator floor; T5/T6 will pull it into
//! `gaps_of`/`tensor_of`; T7 will pull it into `minimize`). A
//! verdict-altitude home is the natural common ancestor. Co-locating
//! `confidence_of` here keeps the projection functor next to the type
//! it projects; substrate-pull may later lift `confidence_of` onto
//! [`Transparency<P>`] in `prism/imperfect` (a sibling to
//! [`Transparency::opacities`]). When that lands, this module re-
//! exports.
//!
//! [`shards/epistemologic/property.mirror`]: ../../../../shards/epistemologic/property.mirror
//! [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
//! [`docs/specs/property-and-inference-collapse.md`]: ../../../../docs/specs/property-and-inference-collapse.md
//! [`Failure`]: terni::Imperfect::Failure
//! [`Transparency<P>`]: prismqueer::Transparency
//! [`Transparency::opacities`]: prismqueer::Transparency::opacities
#![allow(dead_code)]

use prismqueer::{PropertyVerdict, Ref, Transparency};

use crate::music::CadenceKind;

// ---------------------------------------------------------------------------
// Gap — the Failure carrier of `Imperfect<(), Gap, Transparency<Ref>>`.
// ---------------------------------------------------------------------------

/// The substrate's gap type at the verdict altitude.
///
/// Per [`docs/specs/gap-tension-tensor-substrate.md`] §3.1 the full
/// substrate shape is
///
/// ```mirror
/// type gap = { claim: claim, verifier: verifier, state: gap_state }
/// ```
///
/// and per §11 the failure-level extension is
///
/// ```mirror
/// type contradiction <= gap & { level: u32, … }
/// ```
///
/// At this tick (T3) the Rust shape is **minimal** — only the fields
/// the audible-altitude consumer ([`crate::music::is_settled`]) needs
/// to type the [`Failure`] variant. Specifically:
///
/// - [`level`] carries Bateson's logical level (per the substrate's
///   `contradiction <= gap & { level }`). The deceptive-cadence case
///   maps to level 1 (a simple unresolved tension; V → vi).
/// - [`origin`] names the substrate location where the gap surfaced,
///   so downstream consumers can address the cocycle. The
///   audible-altitude origin is `@epistemologic/math/music/cadence`.
/// - [`tension_summary`] carries a short human-readable name of the
///   tension direction (e.g. `"V → vi"`). The full `tension_vector`
///   shape is a [`gap-tension-tensor-substrate.md`] §8.1 design call
///   still parked at the substrate altitude; this field is the
///   audible-altitude reading until that design call lands.
///
/// Fuller shape (`claim`, `verifier`, `state`, structured
/// `tension_vector`) lands when T5 (`gaps_of`) and T6 (`tensor_of`)
/// substrate-pull. The minimal shape here is exactly what the
/// implementation cascade can stand on without inventing structure
/// the substrate hasn't yet declared.
///
/// [`Failure`]: terni::Imperfect::Failure
/// [`level`]: Gap::level
/// [`origin`]: Gap::origin
/// [`tension_summary`]: Gap::tension_summary
/// [`gap-tension-tensor-substrate.md`]: ../../../../docs/specs/gap-tension-tensor-substrate.md
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gap {
    /// Bateson's logical level of the contradiction (per
    /// `contradiction <= gap & { level: u32 }`). Level 1 = simple
    /// unresolved tension; higher levels = nested learning loops.
    level: u32,
    /// The substrate location where this gap surfaced. Names *where*
    /// the cocycle lives so downstream consumers (Reflection, the
    /// kintsugi loop, the scene curator) can address it.
    origin: Ref,
    /// A short human-readable name of the tension direction. The full
    /// `tension_vector` shape is parked at the substrate altitude;
    /// this is the audible-altitude floor.
    tension_summary: String,
}

impl Gap {
    /// Construct a gap from its three carriers. Level is the Bateson
    /// logical level; `origin` is the substrate location; `summary` is
    /// the audible-altitude tension reading.
    pub fn new(level: u32, origin: Ref, summary: impl Into<String>) -> Self {
        Gap {
            level,
            origin,
            tension_summary: summary.into(),
        }
    }

    /// Read this gap's Bateson logical level.
    pub fn level(g: &Gap) -> u32 {
        g.level
    }

    /// Borrow this gap's substrate origin.
    pub fn origin(&self) -> &Ref {
        &self.origin
    }

    /// Borrow this gap's tension summary.
    pub fn tension_summary(&self) -> &str {
        &self.tension_summary
    }
}

// ---------------------------------------------------------------------------
// confidence_of — the scalar projection functor `transparency → [0, 1]`.
// ---------------------------------------------------------------------------

/// The forgetful functor from the [`Transparency<Ref>`] lattice to
/// `[0, 1]`.
///
/// Per [`docs/specs/property-and-inference-collapse.md`] §3.1:
///
/// ```mirror
/// action confidence_of(t: transparency(_)) -> f64 {
///   match t {
///     success      -> 1.0,
///     partial(m)   -> 1.0 - normalize(sum(m, |op| op.weight)),
///     failure(_)   -> 0.0,
///   }
/// }
/// ```
///
/// At the bootstrap altitude [`Transparency<Ref>`] does not carry an
/// explicit success/partial/failure tag at the value level — it carries
/// `Clear` (the identity) or `Opaque(OpacityMap)` with per-location
/// [`PropertyVerdict`]s. This projection reads the opaque map:
///
/// - `Clear` → `1.0` (no opacity = full confidence).
/// - `Opaque(empty)` → `0.0` (catastrophic sentinel = zero confidence).
/// - `Opaque(m)` where any verdict is [`PropertyVerdict::Fail`] → `0.0`.
/// - `Opaque(m)` of [`PropertyVerdict::Partial`] verdicts → the minimum
///   confidence carried in the map (matches the `merge_with`
///   discipline: confidence only goes down through accumulation).
/// - `Opaque(m)` of [`PropertyVerdict::Pass`] only → `1.0`.
///
/// This is the audible-altitude floor projection. When the substrate
/// declares a richer `normalize(…)` (per §8.1 design call) the body
/// updates; the *signature* stays.
///
/// [`docs/specs/property-and-inference-collapse.md`]: ../../../../docs/specs/property-and-inference-collapse.md
pub fn confidence_of(t: &Transparency<Ref>) -> f64 {
    if t.is_catastrophic() {
        return 0.0;
    }
    match t.opacities() {
        // `Clear` — the identity of the Loss monoid; no opacity.
        None => 1.0,
        Some(map) => {
            // Reduce per-location verdicts. `Fail` dominates (matches
            // the `merge_with` discipline). `Partial`s take the min
            // confidence. `Pass`-only is 1.0.
            let mut min_confidence: f64 = 1.0;
            for verdict in map.values() {
                match verdict {
                    PropertyVerdict::Fail(_) => return 0.0,
                    PropertyVerdict::Partial { confidence, .. } => {
                        if *confidence < min_confidence {
                            min_confidence = *confidence;
                        }
                    }
                    PropertyVerdict::Pass => { /* neutral */ }
                }
            }
            min_confidence
        }
    }
}

// ---------------------------------------------------------------------------
// verdict_to_cadence_kind — the audible-altitude projection.
// ---------------------------------------------------------------------------

/// The audible-altitude projection `verdict → cadence_kind`.
///
/// Per [`docs/specs/property-and-inference-collapse.md`] §4.4 the
/// four-state cadence mapping survives as a *projection* from the
/// substrate's full verdict, not as the primary surface. The Hodge
/// reading (per `docs/insights/2026-06-07-hodge-duality-three-readings-of-H.md`):
/// the four-state cadence dispatch IS the audible-altitude
/// discretization of the Hodge projection path the verdict altitude
/// tracks at full resolution.
///
/// Mapping (the inverse of [`crate::music::is_settled`]'s emission):
///
/// - `Success(())` → [`CadenceKind::Authentic`] (harmonic
///   representative IS the input; the canonical closure).
/// - `Partial((), t)` with `confidence_of(t) > 1/φ` → [`CadenceKind::Plagal`]
///   (consonant alternative path).
/// - `Partial((), t)` with `confidence_of(t) ≤ 1/φ` → [`CadenceKind::Half`]
///   (paused on V).
/// - `Failure(_, _)` → [`CadenceKind::Deceptive`] (V → vi; the
///   cocycle the substrate cannot trivialise).
///
/// `1/φ ≈ 0.618` is the golden-ratio threshold the audible altitude
/// already uses per `bootstrap/src/music/mod.rs`'s 0.85 / 0.25
/// commitments.
pub fn verdict_to_cadence_kind(v: &crate::music::Verdict) -> CadenceKind {
    use terni::Imperfect;
    match v {
        Imperfect::Success(()) => CadenceKind::Authentic,
        Imperfect::Partial((), t) => {
            if confidence_of(t) > INV_PHI {
                CadenceKind::Plagal
            } else {
                CadenceKind::Half
            }
        }
        Imperfect::Failure(_, _) => CadenceKind::Deceptive,
    }
}

/// `1/φ` — the golden-ratio threshold the audible altitude uses to
/// discriminate plagal (closer-to-pass-than-not) from half
/// (closer-to-failure-than-not). Per `bootstrap/src/music/mod.rs`'s
/// 0.85 / 0.25 confidence commitments sitting on opposite sides.
pub(crate) const INV_PHI: f64 = 0.618_033_988_749_894_8_f64;

#[cfg(test)]
mod tests {
    //! The executable spec for [`Gap`], [`confidence_of`], and
    //! [`verdict_to_cadence_kind`].

    use super::*;
    use prismqueer::{Diagnostic, PropertyVerdict};
    use terni::Imperfect;

    fn audible_origin() -> Ref {
        Ref::new("@epistemologic/math/music/cadence").expect("valid ref")
    }

    #[test]
    fn gap_minimal_shape_constructs_and_reads() {
        let g = Gap::new(1, audible_origin(), "V -> vi");
        assert_eq!(Gap::level(&g), 1);
        assert_eq!(g.origin().as_str(), "@epistemologic/math/music/cadence");
        assert_eq!(g.tension_summary(), "V -> vi");
    }

    #[test]
    fn confidence_of_clear_is_one() {
        let t: Transparency<Ref> = Transparency::clear();
        assert_eq!(confidence_of(&t), 1.0);
    }

    #[test]
    fn confidence_of_catastrophic_is_zero() {
        let t: Transparency<Ref> = Transparency::catastrophic();
        assert_eq!(confidence_of(&t), 0.0);
    }

    #[test]
    fn confidence_of_opaque_partial_returns_min_confidence() {
        let t = Transparency::opaque(
            audible_origin(),
            PropertyVerdict::Partial {
                confidence: 0.42,
                diagnostics: vec![Diagnostic::new("half-state")],
            },
        );
        assert_eq!(confidence_of(&t), 0.42);
    }

    #[test]
    fn confidence_of_opaque_with_fail_returns_zero() {
        let t = Transparency::opaque(
            audible_origin(),
            PropertyVerdict::Fail(Diagnostic::new("V -> vi")),
        );
        assert_eq!(confidence_of(&t), 0.0);
    }

    #[test]
    fn verdict_to_cadence_kind_success_is_authentic() {
        let v: crate::music::Verdict = Imperfect::Success(());
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Authentic);
    }

    #[test]
    fn verdict_to_cadence_kind_partial_high_confidence_is_plagal() {
        // Clear (confidence = 1.0) projects to Plagal (above 1/phi).
        let v: crate::music::Verdict = Imperfect::Partial((), Transparency::clear());
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Plagal);
    }

    #[test]
    fn verdict_to_cadence_kind_partial_low_confidence_is_half() {
        let t = Transparency::opaque(
            audible_origin(),
            PropertyVerdict::Partial {
                confidence: 0.25,
                diagnostics: vec![Diagnostic::new("paused on V")],
            },
        );
        let v: crate::music::Verdict = Imperfect::Partial((), t);
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Half);
    }

    #[test]
    fn verdict_to_cadence_kind_failure_is_deceptive() {
        let v: crate::music::Verdict = Imperfect::Failure(
            Gap::new(1, audible_origin(), "V -> vi"),
            Transparency::clear(),
        );
        assert_eq!(verdict_to_cadence_kind(&v), CadenceKind::Deceptive);
    }
}
