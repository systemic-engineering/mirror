//! `Mandelbrot<T>` — parent trait unifying Liquid<T> and Crystal<T> as
//! two states of a Mandelbrot-set point.
//!
//! Per Alex 2026-07-13 recognition (memory `project_fractal_mandelbrot_substrate`):
//! *"@fractal underlies @kintsugi/consent; mirror compiler IS a Mandelbrot set."*
//! Ratified at species-decl altitude via Mara Round 2 `shards/fractal/
//! mandelbrot.mirror` + `shards/fractal/crystal.mirror` (2026-07-20).
//!
//! ## The two states
//!
//! Mandelbrot iteration `z_{n+1} = z_n² + c` produces either:
//! - **bounded orbit** → point INSIDE the set → `Crystal<T>` (settled;
//!   content-addressed; SAGA-replayable; immutable)
//! - **still-iterating / diverging** → point on BOUNDARY or
//!   unresolved → `Liquid<T>` (in-flow; participates in prismqueer's
//!   pillar dispatch; not yet settled)
//!
//! The `crystallize` operation (implemented in `crystal.rs`) is the
//! Liquid → Crystal transition. In VSM terms it is the @time/now
//! collapse-point where a state settles into an OID-stamped fragment
//! that participates in the SAGA chain.
//!
//! ## Composition with landed substrate
//!
//! - `prismqueer::liquid` (Reed iter 1-10 arc) supplies the Prism trait
//!   + five-op basis + pillar primitives that Liquid<T> participates in.
//! - `Crystal<T>` composes over `Witnessed` (this crate; MARA doctrine
//!   Author≠Committer) so every Crystal carries provenance-by-construction.
//! - Session-closing recognition (Mara Round 3 `39b64b8`): mirror IS
//!   VSM at compiler altitude composing prismqueer into BEAM as Principal
//!   Bundle Tower. Mandelbrot<T> is the fibre-shape of that tower.
//!
//! ## Content-addressing invariant
//!
//! Every implementor MUST have a stable `oid()` — the content-addressed
//! identifier that survives across serialization/deserialization/network-
//! transport. This IS what makes `@peer.redirect(crystal_oid)` refuse
//! re-litigation via OID-verifiability (Round 3 recognition).

use crate::witnessed::Witnessed;

/// Content-addressed identifier for a Mandelbrot-set point.
///
/// 32-byte SHA-256-shaped OID. Same content → same OID. Different
/// content → different OID by hash collision resistance.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Oid(pub [u8; 32]);

impl Oid {
    /// The genesis OID (all-zero). Used as the initial `prev` in a
    /// SAGA chain before the first crystal has been deposited.
    pub const GENESIS: Oid = Oid([0u8; 32]);

    /// True iff this is the genesis OID.
    pub fn is_genesis(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

/// The parent trait unifying Liquid<T> and Crystal<T> as two states
/// of a Mandelbrot-set point.
///
/// Implementors:
/// - `Liquid<T>` (defined in `prismqueer::void`; re-exported at
///   `prismqueer::liquid`; still-iterating; participates in pillar
///   dispatch)
/// - `Crystal<T>` (defined in `crystal.rs`; settled interior;
///   content-addressed; SAGA-replayable; immutable)
///
/// Every implementor supplies a stable `oid()` (content-addressed
/// identity) plus predicate methods for state-classification
/// (`is_liquid` / `is_crystal`).
pub trait Mandelbrot<T> {
    /// The content-addressed identifier for this point.
    fn oid(&self) -> Oid;

    /// True iff this point is in the still-iterating / boundary state.
    fn is_liquid(&self) -> bool;

    /// True iff this point is in the settled-interior state.
    fn is_crystal(&self) -> bool;
}

/// The provenance chain any Crystal carries — the WHO of each settled
/// state. Composes with `Witnessed` (Author + Committer + Timestamp +
/// Message) via MARA doctrine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MandelbrotProvenance {
    pub witnessed: Witnessed,
    pub prev: Oid,
}

// =====================================================================
// Property tests — Oid content-addressing invariants.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_oid_is_all_zero() {
        assert_eq!(Oid::GENESIS.0, [0u8; 32]);
        assert!(Oid::GENESIS.is_genesis());
    }

    #[test]
    fn distinct_content_produces_distinct_oids() {
        // Content-addressing invariant: different bytes → different OID.
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        a[0] = 1;
        b[0] = 2;
        let oid_a = Oid(a);
        let oid_b = Oid(b);
        assert_ne!(oid_a, oid_b, "distinct content MUST produce distinct OIDs");
    }

    #[test]
    fn identical_content_produces_identical_oids() {
        let bytes = [42u8; 32];
        let oid1 = Oid(bytes);
        let oid2 = Oid(bytes);
        assert_eq!(oid1, oid2, "identical content MUST produce identical OIDs");
    }

    #[test]
    fn genesis_predicate_distinguishes_only_zero_oid() {
        assert!(Oid([0u8; 32]).is_genesis());
        assert!(!Oid([1u8; 32]).is_genesis());
        let mut single_bit = [0u8; 32];
        single_bit[15] = 1;
        assert!(!Oid(single_bit).is_genesis());
    }

    #[test]
    fn oid_is_hashable_for_use_in_hashmaps() {
        // Uses the Hash derive; verifies OID can be a HashMap key.
        use std::collections::HashMap;
        let mut map: HashMap<Oid, &str> = HashMap::new();
        map.insert(Oid([1u8; 32]), "one");
        map.insert(Oid([2u8; 32]), "two");
        assert_eq!(map.get(&Oid([1u8; 32])), Some(&"one"));
        assert_eq!(map.get(&Oid([3u8; 32])), None);
    }
}
