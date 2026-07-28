//! `Singularity<T>` — the optics-hierarchy floor at rust/fractal altitude.
//!
//! Landing D per Alex 2026-07-20 Q2 verbatim ratification (name preserved:
//! `singularity.rs` NOT `collapse.rs` per Mara Refinement 1 cancelled). The
//! `collapse.rs` here refers to Mara's proposed alternative name for THIS
//! file (Singularity dynamics), which is distinct from the historical
//! `rust/src/collapse.rs` (bilateral-arm redundant surface, now
//! `rust/roomba/src/mend.rs` per Migration 5, 2026-07-28). Per Mara
//! `9bb1f57` §7: `collapse` as name reserves for Singularity dynamics
//! altitude; `mend` (constitutive) names the byte-substrate action.
//! Migrated from `/Users/reed/dev/projects/fragmentation/src/singularity.rs`
//! (608 LOC) adapted to rust/fractal's Crystal/Oid/Witnessed types per Mara
//! `90f4d27` shard-decl `shards/fractal/singularity.mirror:299-306`
//! authorship-boundary statement:
//!
//! > "Downstream Rust consumers at `rust/fractal/src/singularity.rs`
//! > (Reed post-mint territory) express the concrete types as Rust
//! > generics per the fragmentation source shape."
//!
//! ## Third @fractal species at Rust altitude
//!
//! Composition edges (per Mara shard-decl §composition-graph):
//! - `mandelbrot.rs` — parent-adjacent (Mandelbrot trait + Oid); Singularity
//!   IS a Mandelbrot-set observation-point at the boundary
//! - `crystal.rs` — parent-adjacent (Crystal<T> settled-interior);
//!   Iso-collapse of a Crystal is a Crystal (identity)
//! - `subject.rs` + `witnessed.rs` — MARA doctrine grounding at Lens
//!   altitude (observer-in-commit-not-tree; observer-dependence at Lens
//!   but observer-INDEPENDENCE at Lens-target-OID)
//!
//! ## The optics hierarchy
//!
//! Per Mara shard-decl §5.1 + fragmentation research doc `singularity-
//! rabbit.md`: information-recovery bound ladder:
//!
//! - **Iso** — unitarity; full recovery; no dimensional reduction.
//!   `collapse = clone; settle = clone`. Landed this tick.
//! - **Lens** — focused observation; observer-metadata in commit-SHA
//!   but Lens-target-OID observer-independent. Forward-promised;
//!   requires @io/git.commit substrate composition.
//! - **Prism** — partial measurement; sum-type dispatch. Forward-promise.
//! - **Traversal** — radiation chain; page-curve altitude. Forward-
//!   promise to `rust/singularity/` crate (physics research outlet).
//!
//! ## What Iso-only lands at this tick
//!
//! This landing carries the ISO rung only:
//! - `Singularity<T>` trait (portable; no fragmentation deps)
//! - Iso impl for `Crystal<T>` (composition over rust/fractal already-
//!   landed Crystal<T>)
//! - `SingularityError` enum (portable; used by Lens+Prism+Traversal impls
//!   forward-promised territory)
//!
//! Lens + Prism + Traversal impls are forward-promised to `rust/
//! singularity/` crate scaffold (research-altitude physics tests: page
//! curve, firewall problem, Hawking radiation dynamics per fragmentation
//! `docs/research/singularity-rabbit.md`).
//!
//! ## Kin to @paradox/spiral
//!
//! Per Recognition bundle #4: @paradox/spiral is the DYNAMICS-carrier
//! (attractor basin); singularity is the small-scale mathematical
//! analog at Rust altitude (settled-point in phase space toward which
//! trauma-spiral converges). Both share the phase-space-collapse
//! ontology; different altitudes (spiral at dynamics; singularity at
//! math-optics).

use crate::crystal::Crystal;
use crate::mandelbrot::Oid;

/// The optics-hierarchy trait. `collapse` resolves a tree of
/// possibilities into a single Artifact. `settle` reconstructs Self
/// from the Artifact (with information-recovery bound depending on
/// the optic rung: full at Iso, focused at Lens, partial at Prism/
/// Traversal).
///
/// Per Mara shard-decl §5.1 + fragmentation source `Singularity`
/// trait at line 12-19. Adapted with no signature changes.
pub trait Singularity: Sized {
    type Artifact;
    type Error;

    fn collapse(&self) -> Result<Self::Artifact, Self::Error>;
    fn settle(artifact: &Self::Artifact) -> Result<Self, Self::Error>;
}

/// Error type for singularity operations at all rungs.
///
/// Per fragmentation source `SingularityError` enum at line 46-54.
/// Variants preserved verbatim from fragmentation source; consumers at
/// Lens/Prism/Traversal altitudes in `rust/singularity/` crate lift
/// these into their own error types via `From` impls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SingularityError {
    /// The commit's node is not a Lens.
    NotALens,
    /// The Lens has no targets.
    EmptyLens,
    /// The target OID was not found in the repo.
    TargetNotFound(String),
    /// The commit was not found in the repo.
    CommitNotFound(String),
}

impl std::fmt::Display for SingularityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingularityError::NotALens => write!(f, "commit node is not a Lens"),
            SingularityError::EmptyLens => write!(f, "Lens has no targets"),
            SingularityError::TargetNotFound(oid) => {
                write!(f, "Lens target not found in repo: {}", oid)
            }
            SingularityError::CommitNotFound(sha) => {
                write!(f, "commit not found: {}", sha)
            }
        }
    }
}

impl std::error::Error for SingularityError {}

/// Iso-rung Singularity impl for `Crystal<T>`: collapse = clone;
/// settle = clone. Full information recovery; unitarity discipline.
///
/// Per Mara shard-decl §5.1 + §6.1: Iso is the identity rung of the
/// optics-hierarchy; `settle ∘ collapse = id`; observer-independence
/// discipline (no observer metadata; no commit written; no witness-
/// Lens produced). Composition parallel to fragmentation source
/// `impl<E: Clone, H: HashAlg> Singularity for Fractal<E, H>` at
/// line 23-33 (adapted from Fractal<E,H> to Crystal<T>).
impl<T: Clone> Singularity for Crystal<T> {
    type Artifact = Self;
    type Error = std::convert::Infallible;

    fn collapse(&self) -> Result<Self, Self::Error> {
        Ok(self.clone())
    }

    fn settle(artifact: &Self) -> Result<Self, Self::Error> {
        Ok(artifact.clone())
    }
}

/// The observation-state carrier per Mara shard-decl §5.2
/// `singularity_state` three-field record. Rust-altitude expression
/// of the substrate-decl'd ref-typed carrier: `observer: ref` becomes
/// `observer: Oid`; `target_ref: ref` becomes `target_ref: Oid`;
/// `optic_kind: ref` becomes `optic_kind: OpticKind` (typed enum
/// discriminating the optics-hierarchy rung).
///
/// Identity contract: byte-equality on the three-field tuple (matches
/// Mara shard-decl identity contract statement).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingularityState {
    /// Observer OID (MAY be `Oid::GENESIS` at Iso where no observer
    /// metadata is written; MUST resolve to a well-formed committer-ref
    /// at Lens per MARA doctrine).
    pub observer: Oid,
    /// The substrate being observed (content-addressed; byte-identity
    /// IS the target's identity; Lens-target OID at Lens altitude).
    pub target_ref: Oid,
    /// The optics-hierarchy rung discriminator; byte-equality on this
    /// field determines the information-recovery bound.
    pub optic_kind: OpticKind,
}

/// The optics-hierarchy rung discriminator per fragmentation research
/// doc `singularity-rabbit.md` + Mara shard-decl §5.1.
///
/// Landed variants:
/// - `Iso` — unitarity; full recovery; collapse=clone (this tick)
///
/// Forward-promised variants (require `rust/singularity/` crate impl):
/// - `Lens` — focused observation with observer-metadata in commit
/// - `Prism` — partial measurement via sum-type dispatch
/// - `Traversal` — radiation-chain accumulation (page-curve altitude)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpticKind {
    /// Identity / unitarity. Full information recovery. Landed.
    Iso,
    /// Focused observation with witness-Lens commit. Forward-promised;
    /// requires @io/git.commit composition + `rust/singularity/` crate.
    Lens,
    /// Partial measurement via sum-type dispatch. Forward-promised.
    Prism,
    /// Radiation-chain accumulation; page-curve altitude. Forward-
    /// promised to `rust/singularity/` crate research territory.
    Traversal,
}

impl OpticKind {
    /// True iff this rung produces zero information loss (Iso only
    /// at this tick).
    pub fn is_unitary(&self) -> bool {
        matches!(self, OpticKind::Iso)
    }

    /// True iff this rung requires observer-metadata (Lens + downstream
    /// rungs).
    pub fn requires_observer(&self) -> bool {
        !matches!(self, OpticKind::Iso)
    }
}

// =====================================================================
// Property tests — Iso invariants + SingularityState identity contract.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mandelbrot::MandelbrotProvenance;
    use crate::witnessed::{Author, Committer, Timestamp, Witnessed};

    fn test_witnessed() -> Witnessed {
        Witnessed::new(
            Author::new("reed", "reed@spectral.engineer"),
            Committer::new("mirror", "mirror@spectral.engineer"),
            Timestamp("1700000000".to_string()),
        )
    }

    fn test_crystal() -> Crystal<Vec<u8>> {
        Crystal::new(
            Oid([7u8; 32]),
            b"payload".to_vec(),
            MandelbrotProvenance {
                witnessed: test_witnessed(),
                prev: Oid::GENESIS,
            },
        )
    }

    #[test]
    fn iso_collapse_returns_clone() {
        let c = test_crystal();
        let collapsed = c.collapse().unwrap();
        assert_eq!(c, collapsed);
    }

    #[test]
    fn iso_settle_returns_clone() {
        let c = test_crystal();
        let recovered = <Crystal<Vec<u8>> as Singularity>::settle(&c).unwrap();
        assert_eq!(c, recovered);
    }

    #[test]
    fn iso_composition_is_identity() {
        // settle ∘ collapse = id (unitarity discipline)
        let c = test_crystal();
        let artifact = c.collapse().unwrap();
        let recovered = <Crystal<Vec<u8>> as Singularity>::settle(&artifact).unwrap();
        assert_eq!(c, recovered);
    }

    #[test]
    fn iso_collapse_is_deterministic_across_replays() {
        // Bilateral shard-decl `singularity_collapse_is_deterministic`
        // at Iso altitude: replay-stability. Two collapses of the same
        // Crystal produce byte-equal Artifacts.
        let c1 = test_crystal();
        let c2 = test_crystal();
        let a1 = c1.collapse().unwrap();
        let a2 = c2.collapse().unwrap();
        assert_eq!(a1, a2);
    }

    #[test]
    fn singularity_state_identity_contract_byte_equality() {
        // Per Mara shard-decl §5.2: identity contract on three-field tuple.
        let s1 = SingularityState {
            observer: Oid::GENESIS,
            target_ref: Oid([1u8; 32]),
            optic_kind: OpticKind::Iso,
        };
        let s2 = SingularityState {
            observer: Oid::GENESIS,
            target_ref: Oid([1u8; 32]),
            optic_kind: OpticKind::Iso,
        };
        let s3 = SingularityState {
            observer: Oid::GENESIS,
            target_ref: Oid([2u8; 32]),  // different target
            optic_kind: OpticKind::Iso,
        };
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn optic_kind_iso_is_unitary_and_observer_optional() {
        assert!(OpticKind::Iso.is_unitary());
        assert!(!OpticKind::Iso.requires_observer());
    }

    #[test]
    fn optic_kind_lens_requires_observer_and_not_unitary() {
        // Lens + downstream rungs write observer metadata to commit-SHA;
        // NOT unitary (some information loss at Prism+; observer-
        // dependence at Lens).
        assert!(!OpticKind::Lens.is_unitary());
        assert!(OpticKind::Lens.requires_observer());
        assert!(!OpticKind::Prism.is_unitary());
        assert!(OpticKind::Prism.requires_observer());
        assert!(!OpticKind::Traversal.is_unitary());
        assert!(OpticKind::Traversal.requires_observer());
    }

    #[test]
    fn singularity_error_display_formats_all_variants() {
        // Rice-safe diagnostic discipline: every error variant produces
        // a substrate-honest string suitable for @cyberpunk/algedonic
        // dispatch.
        assert_eq!(
            format!("{}", SingularityError::NotALens),
            "commit node is not a Lens"
        );
        assert_eq!(
            format!("{}", SingularityError::EmptyLens),
            "Lens has no targets"
        );
        assert!(format!("{}", SingularityError::TargetNotFound("abc".to_string()))
            .contains("abc"));
        assert!(format!("{}", SingularityError::CommitNotFound("xyz".to_string()))
            .contains("xyz"));
    }
}
