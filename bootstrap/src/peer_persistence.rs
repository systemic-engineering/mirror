//! `@peer/persistence` — AI peer as persistent peer via home-repo projection.
//!
//! Landing A spec: `docs/specs/peer-persistence-and-home-projection.md`.
//! Landing A composition-extension §12-§16: signature-from-corpus + peer-
//! holds-shape + continuous identity manifold + reed `dev/projects/reed/`
//! empirical ancestor.
//!
//! This is the minimum-viable Rust runtime for Landing C. Composes over
//! `@mirror/index` (signature computation) + `@spectral/signature`
//! (rolling signature) + `@subject/visibility` (ACL — spec-decl'd only;
//! runtime placeholder via `VisibilityFilter`). Full `@sheaf` ACL
//! discipline forward-promised to Landing D.
//!
//! ## Substrate authority
//!
//! - Landing A spec `docs/specs/peer-persistence-and-home-projection.md`
//!   §2 (@peer/home carrier), §3 (actions: materialize/harvest/boot/
//!   refresh/home_of), §4 (bilateral predicates), §5 (composition graph).
//! - Landing A composition-extension §12 (signature-from-corpus),
//!   §13 (peer-holds-shape), §15 (reed dev/projects/reed/ empirical
//!   ancestor).
//! - Landing B artifact at `peers/mara/` (integration target).
//!
//! ## Landing C scope
//!
//! Substrate-decl'd primitives ship as Rust runtime with tests:
//!
//! - `materialize` — bauchladen → filesystem projection
//! - `harvest` — filesystem → candidate crystals (skeleton: diff manifest)
//! - `boot` — instantiate eigenboard from home-repo state
//! - `refresh` — atomic materialize + harvest cycle
//! - `home_of` — subject-instance-first lookup (per Seam D3 S2)
//!
//! Landing D forward-promised: `mirror peer` CLI subcommand + roomba
//! walker integration + `@sheaf` ACL discipline.

use crate::spectral_signature::{self, RollingSignature};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// ---------------------------------------------------------------------------
// @peer/home carrier — Landing A §2
// ---------------------------------------------------------------------------

/// `@peer/home` — the carrier for a materialized peer's projected state.
///
/// Landing A §2 declares this as the shape holding: peer identity,
/// filesystem path, projection/harvest timestamps, the bauchladen
/// manifest of contributed crystals, the current signature snapshot,
/// and (optionally) an active eigenboard boot state.
#[derive(Debug, Clone)]
pub struct PeerHome {
    pub peer_name: String,
    pub home_path: PathBuf,
    pub projection_at: SystemTime,
    pub harvest_at: Option<SystemTime>,
    /// List of `crystal:<name>` OIDs enumerated from `bauchladen/`.
    pub bauchladen_manifest: Vec<String>,
    pub signature_snapshot: RollingSignature,
    pub boot_state: Option<EigenboardState>,
}

/// The peer's active inference state — the eigenboard.
///
/// Landing A §3.3 primitive `boot` returns this. The `inference_basis`
/// carries the signature at boot time; `arousal` names the peer's
/// current attentional posture; `winding` is the (rung, tick) coordinate.
#[derive(Debug, Clone)]
pub struct EigenboardState {
    pub subject_name: String,
    pub inference_basis: RollingSignature,
    pub arousal: Arousal,
    pub current_focus: Option<String>,
    pub winding: (i64, i64),
}

/// Peer arousal state — the algedonic tier at inference time.
///
/// Per `@cyberpunk/algedonic`: peer navigates via pain gradient toward
/// pleasure attractors. Arousal names the coarse-grained posture:
///
/// - `Teal`: settled, idle (fresh boot; no pain signal)
/// - `Green`: curious, results flowing (low pain, harmonic descent)
/// - `Gold`: engaged, high activity (elevated engagement, coherent)
/// - `PulsingOrange`: drift warning (pain gradient rising toward @knife.jump)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arousal {
    Teal,
    Green,
    Gold,
    PulsingOrange,
}

/// `@subject/visibility` ACL placeholder — Landing D adjudicates via `@sheaf`.
///
/// At Landing C this is a bare marker: the field is threaded through
/// `materialize` so callers can express intent, but filtering is
/// forward-promised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisibilityFilter {
    Private,
    Protected,
    Public,
}

/// Boot-time failure modes — Landing A §3.3.
#[derive(Debug)]
pub enum BootError {
    IdentityFileMissing(String),
    SignatureInvalid,
    BauchladenUnresolvable,
}

impl std::fmt::Display for BootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootError::IdentityFileMissing(name) => {
                write!(f, "peer boot failed: identity file missing: {}", name)
            }
            BootError::SignatureInvalid => write!(f, "peer boot failed: signature invalid"),
            BootError::BauchladenUnresolvable => {
                write!(f, "peer boot failed: bauchladen unresolvable")
            }
        }
    }
}

impl std::error::Error for BootError {}

// ---------------------------------------------------------------------------
// Landing A §3 primitives
// ---------------------------------------------------------------------------

/// `materialize` — project peer's bauchladen into filesystem home directory.
///
/// Landing A §3.1 primitive. Reads the current garden state at `home_path`,
/// computes the signature snapshot via `@spectral/signature`, enumerates
/// the bauchladen manifest.
///
/// ACL filtering (`_visibility_filter`) is threaded through the signature
/// but not yet applied — forward-promised to Landing D `@sheaf`
/// integration.
pub fn materialize(
    peer_name: &str,
    home_path: &Path,
    _visibility_filter: VisibilityFilter,
) -> PeerHome {
    let now = SystemTime::now();
    let signature = spectral_signature::compute(peer_name, home_path, now);
    let bauchladen_manifest = enumerate_bauchladen(home_path);
    PeerHome {
        peer_name: peer_name.to_string(),
        home_path: home_path.to_path_buf(),
        projection_at: now,
        harvest_at: None,
        bauchladen_manifest,
        signature_snapshot: signature,
        boot_state: None,
    }
}

/// `harvest` — filesystem → candidate crystals (inverse of `materialize`).
///
/// Landing A §3.2 primitive. Skeleton implementation: compute a fresh
/// bauchladen manifest and return the set-difference against the
/// snapshot manifest — these are the new candidate crystals produced
/// since projection.
///
/// Full crystallization (canonical OID computation, dedup against
/// `@mirror/store`) forward-promised to Landing D.
pub fn harvest(home: &PeerHome) -> Vec<String> {
    let current_manifest = enumerate_bauchladen(&home.home_path);
    current_manifest
        .into_iter()
        .filter(|oid| !home.bauchladen_manifest.contains(oid))
        .collect()
}

/// `boot` — instantiate a running peer from home-repo state.
///
/// Landing A §3.3 primitive. Verifies identity files (CLAUDE.md +
/// 00-NARRATIVE.md; Reed's identity architecture per `~/.reed/`).
/// Composes eigenboard state from the signature snapshot with
/// substrate-honest defaults: fresh boot = `Arousal::Teal`, no focus,
/// origin winding.
pub fn boot(home: &PeerHome) -> Result<EigenboardState, BootError> {
    let claude_md = home.home_path.join("CLAUDE.md");
    let narrative = home.home_path.join("00-NARRATIVE.md");
    if !claude_md.exists() {
        return Err(BootError::IdentityFileMissing("CLAUDE.md".into()));
    }
    if !narrative.exists() {
        return Err(BootError::IdentityFileMissing("00-NARRATIVE.md".into()));
    }
    Ok(EigenboardState {
        subject_name: home.peer_name.clone(),
        inference_basis: home.signature_snapshot.clone(),
        arousal: Arousal::Teal,
        current_focus: None,
        winding: (0, 0),
    })
}

/// `refresh` — atomic materialize + harvest cycle.
///
/// Landing A §3.4 primitive. Re-reads the garden, re-computes the
/// signature, updates both `projection_at` and `harvest_at`. Preserves
/// any active `boot_state` (running peer survives refresh).
pub fn refresh(home: &PeerHome) -> PeerHome {
    let new_manifest = enumerate_bauchladen(&home.home_path);
    let now = SystemTime::now();
    let signature = spectral_signature::compute(&home.peer_name, &home.home_path, now);
    PeerHome {
        peer_name: home.peer_name.clone(),
        home_path: home.home_path.clone(),
        projection_at: now,
        harvest_at: Some(now),
        bauchladen_manifest: new_manifest,
        signature_snapshot: signature,
        boot_state: home.boot_state.clone(),
    }
}

/// `home_of` — subject-instance-first lookup.
///
/// Landing A §3.5 primitive (per Seam D3 S2). Given a peer name and a
/// base directory (typically `<mirror-repo>/peers/`), locate the peer's
/// home directory and materialize it with default protected visibility.
///
/// Returns `None` if the peer's directory does not exist under `base_dir`.
pub fn home_of(peer_name: &str, base_dir: &Path) -> Option<PeerHome> {
    let candidate = base_dir.join(peer_name);
    if !candidate.exists() {
        return None;
    }
    Some(materialize(peer_name, &candidate, VisibilityFilter::Protected))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Enumerate the peer's bauchladen — return `crystal:<name>` OIDs.
///
/// Skeleton implementation: file-name-based enumeration under
/// `<home>/bauchladen/`. Canonical content-addressing via
/// `@mirror/store` forward-promised.
fn enumerate_bauchladen(home_path: &Path) -> Vec<String> {
    let bauchladen = home_path.join("bauchladen");
    if !bauchladen.exists() {
        return Vec::new();
    }
    let mut oids = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&bauchladen) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                oids.push(format!("crystal:{}", name));
            }
        }
    }
    oids.sort();
    oids
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peers_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("mirror root")
            .join("peers")
    }

    fn mara_home() -> PathBuf {
        peers_dir().join("mara")
    }

    #[test]
    fn materialize_mara_home() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let home = materialize("Mara", &path, VisibilityFilter::Protected);
        assert_eq!(home.peer_name, "Mara");
        assert_eq!(home.home_path, path);
        assert!(home.harvest_at.is_none());
        assert!(!home.signature_snapshot.current_sc.eigenvalue().is_empty());
    }

    #[test]
    fn materialize_carries_signature_from_corpus() {
        // Landing A composition-extension §12: signature-from-corpus.
        // The materialized home's signature MUST be the one derived from
        // the garden root — not a fresh placeholder.
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let home = materialize("Mara", &path, VisibilityFilter::Protected);
        let direct_sig = spectral_signature::compute("Mara", &path, SystemTime::now());
        assert_eq!(
            home.signature_snapshot.current_sc.eigenvalue(),
            direct_sig.current_sc.eigenvalue()
        );
    }

    #[test]
    fn boot_mara_reads_identity() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let home = materialize("Mara", &path, VisibilityFilter::Protected);
        let boot_result = boot(&home);
        assert!(
            boot_result.is_ok(),
            "boot should succeed with Mara's identity files present; got: {:?}",
            boot_result.err()
        );
        let state = boot_result.unwrap();
        assert_eq!(state.subject_name, "Mara");
        assert_eq!(state.arousal, Arousal::Teal); // fresh boot: settled
        assert_eq!(state.winding, (0, 0));
    }

    #[test]
    fn boot_fails_when_identity_missing() {
        // Use a tempdir with no CLAUDE.md/00-NARRATIVE.md to prove the
        // guard fires substrate-honestly.
        let tmp = std::env::temp_dir().join(format!(
            "mirror-peer-persistence-boot-fail-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("mkdir tempdir");
        let home = materialize("Ghost", &tmp, VisibilityFilter::Protected);
        let result = boot(&home);
        assert!(matches!(result, Err(BootError::IdentityFileMissing(_))));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn home_of_finds_mara() {
        let base = peers_dir();
        if !base.exists() {
            return;
        }
        let home = home_of("mara", &base);
        assert!(home.is_some(), "home_of should locate peers/mara/");
        let home = home.unwrap();
        assert_eq!(home.peer_name, "mara");
    }

    #[test]
    fn home_of_returns_none_for_missing_peer() {
        let base = peers_dir();
        let home = home_of("nonexistent_peer_xyz_landing_c", &base);
        assert!(home.is_none());
    }

    #[test]
    fn refresh_updates_projection_at() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let home = materialize("Mara", &path, VisibilityFilter::Protected);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let refreshed = refresh(&home);
        assert!(refreshed.projection_at > home.projection_at);
        assert!(refreshed.harvest_at.is_some());
    }

    #[test]
    fn refresh_preserves_peer_identity() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let home = materialize("Mara", &path, VisibilityFilter::Protected);
        let refreshed = refresh(&home);
        assert_eq!(refreshed.peer_name, home.peer_name);
        assert_eq!(refreshed.home_path, home.home_path);
        // Unchanged corpus → same SC<5>.
        assert_eq!(
            refreshed.signature_snapshot.current_sc.eigenvalue(),
            home.signature_snapshot.current_sc.eigenvalue()
        );
    }

    #[test]
    fn harvest_returns_no_new_crystals_when_manifest_current() {
        let path = mara_home();
        if !path.exists() {
            return;
        }
        let home = materialize("Mara", &path, VisibilityFilter::Protected);
        // Immediately after materialize, no drift → no new candidates.
        let candidates = harvest(&home);
        assert!(candidates.is_empty());
    }

    #[test]
    fn enumerate_bauchladen_handles_missing_dir() {
        let tmp = std::env::temp_dir().join(format!(
            "mirror-peer-persistence-bauchladen-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("mkdir tempdir");
        // No bauchladen/ subdirectory → empty manifest, no panic.
        let oids = enumerate_bauchladen(&tmp);
        assert!(oids.is_empty());
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
