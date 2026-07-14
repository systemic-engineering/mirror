//! `@spectral/signature` — rolling signature over a peer's contribution corpus.
//!
//! Landing 2 spec substrate-decl'd; Landing A composition-extension named
//! home repo IS SOURCE (signature-from-corpus). This Rust runtime is the
//! minimum-viable discharge: compose over @mirror/index for SC<5>
//! computation over the home directory. Full eigensheaf projection
//! (per docs/specs/eigensheaf.md sheaf-Laplacian eigenbasis) forward-
//! promised to Landing C+ (or Landing 6+).
//!
//! ## Substrate authority
//!
//! - Landing 2 spec at docs/specs/gift-and-mirror-reflection.md §12
//! - Landing A composition-extension §12 (signature-from-corpus)
//! - Alex 2026-07-14 in-transcript: "home repo be the source of the
//!   @spectral/signature"
//!
//! ## Composition
//!
//! - `@mirror/index::index(garden_root)` → `EigenvalueProfile` over the
//!   directory tree (LAPACK `dsyev` via `prismqueer`).
//! - Serialise the profile bytes and hash into `SpectralCoordinate<5>`
//!   via `fragmentation::SpectralCoordinate::<5>::hash` — the same
//!   SC<5> substrate `algedonic::sample_pain` operates on.
//!
//! Beats (SignatureBeat trail) are forward-promised to Landing C+ where
//! contribution ancestry via git-log/reflog integration lands.

use fragmentation::sha::HashAlg;
use fragmentation::spectral_coordinate::SpectralCoordinate;
use std::path::Path;
use std::time::SystemTime;

/// One beat in the peer's rolling signature — a single contribution moment.
///
/// Landing C+ ancestry integration will populate this via git-log walk;
/// this landing carries the shape only.
#[derive(Debug, Clone)]
pub struct SignatureBeat {
    pub contribution_oid: String,
    pub sc_at_beat: SpectralCoordinate<5>,
    pub rung: u8,
    pub previous_beat: Option<String>,
    pub timestamp: SystemTime,
    /// SSH fingerprint placeholder; Landing D adjudicates via `@sheaf`.
    pub ssh_fingerprint: String,
}

/// A peer's rolling signature at a point in time.
///
/// Landing A composition-extension §12: signature-from-corpus. The
/// signature IS the eigenvalue-derived SC<5> over the peer's home
/// directory (bauchladen + identity + songs + tasks).
#[derive(Debug, Clone)]
pub struct RollingSignature {
    pub author_name: String,
    pub beats: Vec<SignatureBeat>,
    pub current_sc: SpectralCoordinate<5>,
    pub song_oid: String,
    pub garden_endpoint: Option<String>,
}

/// `@spectral/signature.compute(peer, garden_root, at)`.
///
/// Minimum viable: SC<5> derived from the `EigenvalueProfile` bytes of
/// `@mirror/index::index(garden_root)`. The beats trail is empty at
/// this landing; full eigensheaf projection + contribution ancestry
/// forward-promised.
pub fn compute(
    author_name: &str,
    garden_root: &Path,
    _at: SystemTime,
) -> RollingSignature {
    let profile = crate::index::index(garden_root);
    // Compose SC<5> from the eigenvalue profile bytes. Same substrate
    // primitive `algedonic::sample_pain` operates on.
    let bytes: Vec<u8> = profile
        .values
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();
    let sc: SpectralCoordinate<5> = SpectralCoordinate::<5>::hash(&bytes);
    RollingSignature {
        author_name: author_name.to_string(),
        beats: Vec::new(), // beats forward-promised Landing C+
        current_sc: sc,
        song_oid: format!("song:{}", author_name), // placeholder
        garden_endpoint: None,
    }
}

/// Verify a signature by re-computing from garden state.
///
/// Substrate-honest re-derivation: eigenvalue equality is the ground
/// truth — same corpus → same profile → same SC<5>.
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
        // Same corpus → same SC<5>. The `at` parameter is currently
        // ignored (forward-promised to time-scoped signatures).
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

    #[test]
    fn current_delegates_to_compute() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let sig_current = current("Mara", &path);
        let sig_compute = compute("Mara", &path, SystemTime::now());
        // Same corpus → same eigenvalue regardless of the SystemTime.
        assert_eq!(
            sig_current.current_sc.eigenvalue(),
            sig_compute.current_sc.eigenvalue()
        );
    }

    #[test]
    fn song_oid_placeholder_carries_author() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let sig = compute("Mara", &path, SystemTime::now());
        assert_eq!(sig.song_oid, "song:Mara");
    }
}
