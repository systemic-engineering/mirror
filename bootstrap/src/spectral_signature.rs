//! `@spectral/signature` — @io-boundary FLOOR (post-ouroboros-bite).
//!
//! # Arc-2 Tick 2.1 — FIRST OUROBOROS BITE (2026-07-15)
//!
//! This file was authored 2026-07-14 by Reed as a substrate-dishonest
//! Rust extension per the (now-renamed) `[substrate-pull:realize]`
//! marker; see `docs/audits/2026-07-15-reed-substrate-dishonest-rust-
//! extensions-during-gift-arc.md`. Arc-2 Tick 2.1 lifts the substrate-
//! decl into `shards/spectral/signature.mirror` per Mara-B canonical
//! spec `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md`. The
//! four bilateral predicates (`signature_integrity`,
//! `signature_authorship`, `signature_monotone`,
//! `signature_composition_honest`) now dispatch via `apply_h::act`; sbec
//! lifts by four.
//!
//! What remains in this file is the @io-boundary primitive the shard-
//! decl's `compute`/`verify`/`current` action bodies compose over —
//! LAPACK eigenvalue reduction of the peer's home-directory corpus into
//! `SpectralCoordinate<5>` bytes. Called by
//! `bootstrap/src/peer_persistence.rs` (materialize/refresh) as the
//! @io-boundary primitive.
//!
//! ## Substrate authority
//!
//! - Canonical shard-decl: `shards/spectral/signature.mirror` (this landing).
//! - Spec: `docs/specs/gift-and-mirror-reflection.md` §12.
//! - Ouroboros spec: `docs/specs/kintsugi-ouroboros-compiler-self-
//!   collapse.md` Arc-2 Tick 2.1.
//! - Audit: `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
//!   cascade-a2-a6.md`.

use fragmentation::sha::HashAlg;
use fragmentation::spectral_coordinate::SpectralCoordinate;
use std::path::Path;
use std::time::SystemTime;

/// One beat in the peer's rolling signature — a single contribution moment.
/// Full ancestry via git-log walk forward-promised; this landing carries
/// the shape only.
#[derive(Debug, Clone)]
pub struct SignatureBeat {
    pub contribution_oid: String,
    pub sc_at_beat: SpectralCoordinate<5>,
    pub rung: u8,
    pub previous_beat: Option<String>,
    pub timestamp: SystemTime,
    /// SSH fingerprint placeholder; adjudicated via `@sheaf`.
    pub ssh_fingerprint: String,
}

/// A peer's rolling signature at a point in time.
///
/// Per `shards/spectral/signature.mirror` (this landing) `rolling_signature`
/// substrate-decl. The signature IS the eigenvalue-derived SC<5> over
/// the peer's home directory.
#[derive(Debug, Clone)]
pub struct RollingSignature {
    pub author_name: String,
    pub beats: Vec<SignatureBeat>,
    pub current_sc: SpectralCoordinate<5>,
    pub song_oid: String,
    pub garden_endpoint: Option<String>,
}

/// `@spectral/signature.compute(peer, garden_root, at)` @io-boundary FLOOR.
///
/// Minimum viable: SC<5> derived from the `EigenvalueProfile` bytes of
/// `@mirror/index::index(garden_root)`. Called by peer_persistence's
/// materialize/refresh. Full eigensheaf projection + contribution
/// ancestry forward-promised.
pub fn compute(
    author_name: &str,
    garden_root: &Path,
    _at: SystemTime,
) -> RollingSignature {
    let profile = crate::index::index(garden_root);
    let bytes: Vec<u8> = profile
        .values
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let sc: SpectralCoordinate<5> = SpectralCoordinate::<5>::hash(&bytes);
    RollingSignature {
        author_name: author_name.to_string(),
        beats: Vec::new(), // beats forward-promised
        current_sc: sc,
        song_oid: format!("song:{}", author_name), // placeholder
        garden_endpoint: None,
    }
}

/// Verify a signature by re-computing from garden state. Substrate-
/// honest re-derivation: same corpus → same profile → same SC<5>.
pub fn verify(sig: &RollingSignature, garden_root: &Path, at: SystemTime) -> bool {
    let recomputed = compute(&sig.author_name, garden_root, at);
    recomputed.current_sc.eigenvalue() == sig.current_sc.eigenvalue()
}

/// Get the current signature for an author (fetches from live garden).
pub fn current(author_name: &str, garden_root: &Path) -> RollingSignature {
    compute(author_name, garden_root, SystemTime::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mara_home() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("mirror root")
            .join("peers")
            .join("mara")
    }

    #[test]
    fn compute_returns_signature() {
        let path = mara_home();
        if !path.exists() {
            return; // fixture-guarded
        }
        let sig = compute("Mara", &path, SystemTime::now());
        assert_eq!(sig.author_name, "Mara");
        assert!(!sig.current_sc.eigenvalue().is_empty());
    }

    #[test]
    fn compute_is_deterministic_across_calls() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let now = SystemTime::now();
        let sig_a = compute("Mara", &path, now);
        let sig_b = compute("Mara", &path, now);
        assert_eq!(sig_a.current_sc.eigenvalue(), sig_b.current_sc.eigenvalue());
    }

    #[test]
    fn verify_succeeds_on_unchanged_garden() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let sig = compute("Mara", &path, SystemTime::now());
        assert!(verify(&sig, &path, SystemTime::now()));
    }
}
