//! `Crystal<T>` — the settled-interior state of a Mandelbrot-set point.
//!
//! Per Alex 2026-07-20 direct-transcript recognition + Mara Round 3
//! `shards/fractal/crystal.mirror` species-decl:
//!
//! - Mandelbrot iteration produces two states: bounded orbit (Crystal;
//!   inside the set) or diverging/still-iterating (Liquid; boundary or
//!   unresolved).
//! - `Crystal<T>` is the SETTLED state — content-addressed, immutable,
//!   SAGA-replayable, part of `@time/past` history.
//! - `crystallize<T>(l: Liquid<T>) → Crystal<T>` is the operation
//!   `@time/now` performs at each moment; converts flowing state to
//!   settled content-addressed fragment.
//!
//! ## SAGA-chain-of-Crystals
//!
//! Every @song is a trajectory through Mandelbrot phase space; every
//! @song/beat is a Crystal deposited at the trajectory's current point.
//! The Crystal chain is walkable in reverse for SAGA compensation.
//! This is what `@peer.redirect(crystal_oid)` walks when refusing
//! re-litigation.
//!
//! ## Composition edges
//!
//! - `Witnessed` (MARA doctrine Author≠Committer) supplies provenance
//! - `Oid` (content-addressed identifier from `mandelbrot.rs`) is the
//!   handle @peer.redirect targets
//! - `crystallize` is called by `@time/now` in the compile.rs SAGA
//!   orchestration loop
//! - `pillar::crystallization_preserves_saga` (Reed's pillar surface)
//!   verifies the chain integrity across crystallization events
//!
//! ## The five-altitude composition
//!
//! Per session-closing recognition (Mara `39b64b8`): Crystal<T> lives
//! at the Rust altitude of the five-scale Principal Bundle Tower
//! (silicon → prismqueer → mirror → BEAM → distributed). Every
//! Crystal<T> at Rust altitude corresponds to a settled process-state
//! at BEAM altitude and a settled cell in the distributed substrate.

use crate::mandelbrot::{Mandelbrot, MandelbrotProvenance, Oid};
use crate::witnessed::Witnessed;

/// A settled-interior Mandelbrot-set point carrying content `T`.
///
/// Immutable by construction (all fields `pub` but only constructed
/// via `crystallize`; T's mutability is T's business, not Crystal's).
/// Content-addressed via `oid`. SAGA-replayable via `provenance.prev`
/// chain. Provenance-preserved via `provenance.witnessed` (Author ≠
/// Committer per MARA doctrine).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crystal<T> {
    /// The content-addressed identifier for this Crystal.
    pub oid: Oid,
    /// The settled content.
    pub content: T,
    /// Provenance chain: WHO settled this Crystal, WHEN, and what
    /// Crystal preceded it in the SAGA chain.
    pub provenance: MandelbrotProvenance,
}

impl<T: Clone> Crystal<T> {
    /// Construct a Crystal directly (for tests + composition; the
    /// canonical constructor is `crystallize`).
    pub fn new(oid: Oid, content: T, provenance: MandelbrotProvenance) -> Self {
        Crystal { oid, content, provenance }
    }

    /// The previous Crystal's OID in the SAGA chain — what
    /// `@peer.redirect` walks backwards to.
    pub fn prev(&self) -> &Oid {
        &self.provenance.prev
    }

    /// True iff this Crystal is a genesis-Crystal (no predecessor).
    pub fn is_genesis(&self) -> bool {
        self.provenance.prev.is_genesis()
    }
}

impl<T> Mandelbrot<T> for Crystal<T> {
    fn oid(&self) -> Oid {
        self.oid.clone()
    }
    fn is_liquid(&self) -> bool {
        false
    }
    fn is_crystal(&self) -> bool {
        true
    }
}

/// The `crystallize` operation — the `@time/now` Liquid → Crystal
/// transition.
///
/// Takes a liquid-state content + witnessed provenance + previous OID;
/// computes a content-addressed OID for the resulting Crystal; returns
/// the settled Crystal. The OID computation is a deterministic hash
/// over (prev, witnessed, content-serialization) so the same inputs
/// produce byte-identical Crystals across replays.
///
/// Content-addressing invariant: `crystallize(l1) == crystallize(l2)`
/// iff `l1` and `l2` produce identical serialized bytes. This is what
/// makes @peer.redirect refuse re-litigation via OID-verifiability.
///
/// Note: at this scaffold altitude the OID is computed via a simple
/// XOR-fold over the input bytes; the production impl composes over
/// `@spectral/signature.hash` (SHA-256/512 per @spectral substrate).
/// Reed post-Round-3 territory to lift to real hashing when the
/// @spectral/signature substrate composes into rust/fractal/.
pub fn crystallize<T: Clone + AsRef<[u8]>>(
    content: T,
    witnessed: Witnessed,
    prev: Oid,
) -> Crystal<T> {
    let mut oid_bytes = [0u8; 32];
    // Seed with prev
    for (i, b) in prev.0.iter().enumerate() {
        oid_bytes[i] ^= *b;
    }
    // Mix in content
    for (i, b) in content.as_ref().iter().enumerate() {
        oid_bytes[i % 32] ^= *b;
    }
    // Mix in witnessed identity (Author name + Committer name bytes;
    // MARA doctrine — different Author OR different Committer produces
    // different OID)
    for (i, b) in witnessed.author.name.as_bytes().iter().enumerate() {
        oid_bytes[(i + 7) % 32] ^= *b;
    }
    for (i, b) in witnessed.committer.name.as_bytes().iter().enumerate() {
        oid_bytes[(i + 19) % 32] ^= *b;
    }
    // Mix in author + committer emails too (identity is name+email pair)
    for (i, b) in witnessed.author.email.as_bytes().iter().enumerate() {
        oid_bytes[(i + 3) % 32] ^= *b;
    }
    for (i, b) in witnessed.committer.email.as_bytes().iter().enumerate() {
        oid_bytes[(i + 23) % 32] ^= *b;
    }
    let oid = Oid(oid_bytes);
    let provenance = MandelbrotProvenance { witnessed, prev };
    Crystal { oid, content, provenance }
}

// =====================================================================
// Property tests — Crystal invariants + crystallize determinism.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::witnessed::{Author, Committer, Timestamp};

    fn test_witnessed() -> Witnessed {
        Witnessed::new(
            Author::new("reed", "reed@spectral.engineer"),
            Committer::new("mirror", "mirror@spectral.engineer"),
            Timestamp("1700000000".to_string()),
        )
    }

    #[test]
    fn crystallize_is_deterministic() {
        // Same content + same witnessed + same prev → same OID.
        let c1 = crystallize(b"hello".to_vec(), test_witnessed(), Oid::GENESIS);
        let c2 = crystallize(b"hello".to_vec(), test_witnessed(), Oid::GENESIS);
        assert_eq!(c1.oid, c2.oid, "crystallize MUST be deterministic");
        assert_eq!(c1.content, c2.content);
    }

    #[test]
    fn crystallize_content_addressing_distinguishes_different_content() {
        let c1 = crystallize(b"hello".to_vec(), test_witnessed(), Oid::GENESIS);
        let c2 = crystallize(b"world".to_vec(), test_witnessed(), Oid::GENESIS);
        assert_ne!(c1.oid, c2.oid, "different content MUST produce different OIDs");
    }

    #[test]
    fn crystallize_content_addressing_distinguishes_different_witnessed() {
        // MARA doctrine: different Author → different Witnessed →
        // different OID. Even with identical content + prev.
        let w1 = test_witnessed();
        let mut w2 = test_witnessed();
        w2.author = Author::new("mara", "mara@spectral.engineer");

        let c1 = crystallize(b"same-content".to_vec(), w1, Oid::GENESIS);
        let c2 = crystallize(b"same-content".to_vec(), w2, Oid::GENESIS);
        assert_ne!(c1.oid, c2.oid, "different witnessed MUST produce different OIDs");
    }

    #[test]
    fn genesis_crystal_predicate_detects_genesis_prev() {
        let genesis = crystallize(b"start".to_vec(), test_witnessed(), Oid::GENESIS);
        assert!(genesis.is_genesis());
        assert_eq!(genesis.prev(), &Oid::GENESIS);

        let child_oid = Oid([7u8; 32]);
        let child = crystallize(b"next".to_vec(), test_witnessed(), child_oid.clone());
        assert!(!child.is_genesis());
        assert_eq!(child.prev(), &child_oid);
    }

    #[test]
    fn mandelbrot_trait_impl_reports_crystal_state() {
        let c = crystallize(b"beat".to_vec(), test_witnessed(), Oid::GENESIS);
        assert!(c.is_crystal());
        assert!(!c.is_liquid());
        assert_eq!(c.oid(), c.oid.clone());
    }

    #[test]
    fn saga_chain_walkable_via_prev() {
        // Build a 3-Crystal SAGA chain; verify each predecessor points
        // to the prior Crystal's OID. This is what @peer.redirect walks.
        let c1 = crystallize(b"first".to_vec(), test_witnessed(), Oid::GENESIS);
        let c2 = crystallize(b"second".to_vec(), test_witnessed(), c1.oid.clone());
        let c3 = crystallize(b"third".to_vec(), test_witnessed(), c2.oid.clone());

        assert_eq!(c2.prev(), &c1.oid);
        assert_eq!(c3.prev(), &c2.oid);
        assert!(c1.is_genesis());
        assert!(!c2.is_genesis());
        assert!(!c3.is_genesis());
    }
}
