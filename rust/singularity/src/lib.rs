//! `singularity` — black-hole physics research outlet at rust/ altitude.
//!
//! Landing D C2 scaffold per Alex 2026-07-20 Q2 verbatim ratification +
//! Mara canonical spec §7 split-C ratification (commit `1cb9dc1`) +
//! Taut scout Section 5 split-C recommendation. Kin-to-fractal;
//! publishable independently.
//!
//! ## v0.1.0 scaffold discipline
//!
//! This crate ships EMPTY at v0.1.0. Physics research is forward-
//! promised; the crate exists as a substrate-decl for:
//!
//! - **Page curve** (traversal sufficiency; multi-target Lens chains):
//!   when does traversal accumulate enough Lenses to recover full
//!   interior? Open question per fragmentation `docs/research/
//!   singularity-rabbit.md`.
//!
//! - **Firewall problem** (lossy collapse; partial recovery bounds):
//!   what happens at the Lens-Prism transition when observer-focused
//!   optics gives way to partial-measurement? Open question.
//!
//! - **Hawking radiation dynamics** (information-leakage rate): how
//!   does information leak across the boundary of a Crystal-chain
//!   under repeated observation?
//!
//! - **Complementarity verification** (different observers, same
//!   interior): black-hole complementarity at compiler altitude —
//!   different observers produce different commit-SHAs but IDENTICAL
//!   Lens-target-OIDs (interior physics is observer-independent per
//!   MARA doctrine at Lens rung).
//!
//! ## Composition
//!
//! - `fractal::Singularity` trait (landed at fractal `singularity.rs`
//!   this Landing D tick) — the optics-hierarchy signature this crate
//!   composes Lens+Prism+Traversal impls over
//! - `fractal::Crystal<T>` — the settled-interior carrier; Iso rung
//!   already implemented at fractal altitude
//! - `fractal::Oid` — content-addressed identifier for Lens-target
//!   discipline
//! - `fractal::Witnessed` (MARA doctrine) — observer-in-commit-not-
//!   tree grounding for Lens+downstream rungs
//!
//! ## Kin to @paradox/spiral
//!
//! Per Recognition bundle #4 + Mara `b8879f2` @paradox/spiral shard-
//! decl: black-hole singularity IS the small-scale mathematical analog
//! of what @paradox/spiral names at dynamics altitude. This crate
//! carries the math substrate for future @paradox/spiral empirical
//! composition (trauma-spiral phase-space collapse formalization at
//! compiler-verifiable altitude).
//!
//! ## Substrate anchors (Reed follow-up territory)
//!
//! - `shards/fractal/singularity.mirror` (Mara `90f4d27`) — species-
//!   decl this crate composes at physics-research altitude
//! - `docs/specs/paradox-family-and-cyberpunk-intervention.md` §7 (Mara
//!   `1cb9dc1`) — Landing D adjudication + refinements
//! - `docs/math/2026-07-20-paradox-family-and-classifier-lagrange.md`
//!   (Mara `392ec11`) — math foundation composing fragmentation optics-
//!   hierarchy at one altitude
//! - `/Users/reed/dev/projects/fragmentation/src/singularity.rs` —
//!   source substrate (Iso + Lens impls at fragmentation altitude)
//! - `/Users/reed/dev/projects/fragmentation/docs/research/singularity-
//!   rabbit.md` — research context (page curve + firewall problem)
//!
//! Re-export `fractal::Singularity` trait so consumers of `singularity`
//! crate get the trait without a separate `use fractal::Singularity`
//! import. The re-export IS the composition edge at API altitude.

pub use fractal::{OpticKind, Singularity, SingularityError, SingularityState};

// =====================================================================
// v0.1.0 SCAFFOLD marker — crate compiles; no physics research landed.
// =====================================================================
//
// Forward-promised impls (require Reed follow-up ticks):
//
// - `impl<T> Singularity for LensObservation<T>` — focused observation
//   with observer-metadata in commit-SHA (@io/git.commit composition)
// - `impl<T> Singularity for PrismObservation<T>` — partial measurement
//   via sum-type dispatch (Prism rung)
// - `impl<T> Singularity for TraversalChain<T>` — radiation-chain
//   accumulation (page-curve altitude)
//
// Property test suites (require Reed follow-up ticks):
//
// - `page_curve_tests` — multi-target Lens chains; when does traversal
//   sufficiency emerge?
// - `firewall_tests` — lossy collapse; partial recovery bounds
// - `complementarity_tests` — different observers, same interior OIDs
// - `hawking_radiation_tests` — information-leakage rate under
//   repeated observation

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singularity_trait_re_export_composes() {
        // Scaffold smoke test: verify the fractal::Singularity re-export
        // is dispatchable from consumer altitude.
        // Uses fractal::Crystal Iso impl (landed at fractal altitude);
        // this crate's Lens+Prism+Traversal impls forward-promised.
        use fractal::{Author, Committer, Crystal, MandelbrotProvenance, Oid, Timestamp, Witnessed};

        let witnessed = Witnessed::new(
            Author::new("reed", "reed@spectral.engineer"),
            Committer::new("mirror", "mirror@spectral.engineer"),
            Timestamp("1700000000".to_string()),
        );
        let c: Crystal<Vec<u8>> = Crystal::new(
            Oid([3u8; 32]),
            b"scaffold".to_vec(),
            MandelbrotProvenance { witnessed, prev: Oid::GENESIS },
        );
        let collapsed = c.collapse().unwrap();
        assert_eq!(c, collapsed);
        let recovered = <Crystal<Vec<u8>> as Singularity>::settle(&collapsed).unwrap();
        assert_eq!(c, recovered);
    }

    #[test]
    fn optic_kind_iso_landed_others_forward_promised() {
        // Scaffold discipline: Iso landed at fractal altitude; other
        // rungs forward-promised to this crate's future ticks.
        assert!(OpticKind::Iso.is_unitary());
        assert!(!OpticKind::Lens.is_unitary());
        assert!(!OpticKind::Prism.is_unitary());
        assert!(!OpticKind::Traversal.is_unitary());
    }

    #[test]
    fn singularity_error_dispatchable_via_re_export() {
        let err = SingularityError::NotALens;
        assert_eq!(format!("{}", err), "commit node is not a Lens");
    }
}
