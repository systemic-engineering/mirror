//! `@cyberpunk/algedonic` — Rust runtime discharge for pain/pleasure gradient
//! primitives.
//!
//! Rung 8+9 Landing 8+9.6b per Taut `15f7ed6` §5 realization gap: substrate-
//! decl'd at `shards/cyberpunk/algedonic.mirror` but Rust runtime doesn't
//! sample. This module discharges the sampling primitive as the substrate-
//! honest form of `pain_gradient` (retires the sc_hamming ratio proxy Reed
//! used in Landing 8+9.6a).
//!
//! ## Substrate authority
//!
//! - `@cyberpunk/algedonic` — Alex Wolf substrate-decl'd
//!   (`docs/specs/peer-as-pain-driven-bounded-ontological-navigator.md`).
//!   Peer navigates via pain gradient toward pleasure attractors.
//! - Foerster 1976 A3 — boundary conditions exceed stable domain ⇒
//!   COORD jumps. Pain accumulates as peer approaches boundary; @knife
//!   jump fires when pain gradient exceeds threshold.
//! - Alex Wolf 2026-07-13 in-transcript — "pain gradient exceeds
//!   threshold" as the trigger mechanism for @cyberpunk/reframe.
//! - Mara `06a8547` §7 — pain-driven navigation as level-shift trigger.
//! - Mara `38c2eeb` §10 prediction #1 — pain gradient ∝ distance-to-
//!   nearest-boundary in SC<N> space (Landing 8+9.6d verifies empirically).
//! - Taut `15f7ed6` §5 — sample_pain / sample_pleasure Rust runtime gap.
//! - Seam `e8508f5` §4 #7 — stability-domain via bootstrap/src/gap.rs
//!   REJECTED; pain-δ IS the operational proxy per Braverman-Yampolsky
//!   Turing-undecidability.
//!
//! ## Substrate-honest interpretation
//!
//! Pain magnitude for a peer's SC<N> coordinate: Shannon entropy of the
//! hex character distribution over the coordinate's canonical byte
//! representation, normalized to `[0, 1]`. Substrate-honest reading:
//!
//! - Uniform hex distribution ⇒ maximum entropy ⇒ pain = 1.0
//!   (peer's coordinate is uniformly-distributed, i.e. no dominant
//!   structure recognizable; peer is "lost" in coordinate space)
//! - Clustered hex distribution ⇒ low entropy ⇒ pain → 0.0
//!   (peer's coordinate has dominant structure; peer is "at home" in
//!   its stable domain)
//!
//! Pleasure is the dual: `pleasure = 1.0 - pain`. Per Alex 2026-07-08
//! (peer-as-pain-driven-bounded-ontological-navigator.md): peer navigates
//! toward pleasure attractors via Rayleigh descent along Fate::bounded.
//!
//! This is a **first-tick substrate-honest proxy**; canonical
//! serialization + true `||sc||_2` formula lands per Mara `c753d5b` §10.1
//! adjudication (forward-promise). Empirical calibration at Landing 8+9.6d
//! per Mara `38c2eeb` §10 prediction #1.

use fragmentation::spectral_coordinate::SpectralCoordinate;

/// `@cyberpunk/algedonic.sample_pain` — pain magnitude at coordinate.
///
/// Returns the Shannon entropy of the coordinate's canonical hex
/// representation, normalized to `[0, 1]`. Substrate-honest interpretation:
/// higher entropy = less-recognizable structure = peer near boundary of
/// stable domain = higher pain (Foerster 1976 A3 boundary approach).
///
/// Per Taut `15f7ed6` §5: retires the sc_hamming ratio Reed used in
/// Landing 8+9.6a as substrate-decl'd pain proxy.
pub fn sample_pain<const N: usize>(sc: &SpectralCoordinate<N>) -> f64 {
    let hex = sc.eigenvalue();
    if hex.is_empty() {
        return 0.0;
    }
    let mut counts = [0u32; 16];
    for c in hex.chars() {
        if let Some(n) = c.to_digit(16) {
            counts[n as usize] += 1;
        }
    }
    let total: u32 = counts.iter().sum();
    if total == 0 {
        return 0.0;
    }
    let n = total as f64;
    let entropy: f64 = counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum();
    // Normalize by log2(16) = 4.0 to map [0, log2(16)] → [0, 1]
    entropy / 4.0
}

/// `@cyberpunk/algedonic.sample_pleasure` — dual of sample_pain.
///
/// Per Alex 2026-07-08 (peer-as-pain-driven-bounded-ontological-
/// navigator.md): peer navigates toward pleasure attractors. This
/// primitive gives the peer's current pleasure magnitude — higher =
/// more structural coherence in the coordinate.
pub fn sample_pleasure<const N: usize>(sc: &SpectralCoordinate<N>) -> f64 {
    1.0 - sample_pain(sc)
}

/// `@cyberpunk/algedonic.pain_gradient` — signed pain change across a
/// morphism.
///
/// - `> 0`: pain increased — peer's coordinate moved toward less structure
///   (Foerster A3: boundary approach signals; @knife.jump may fire)
/// - `< 0`: pain decreased — peer's coordinate moved toward more structure
///   (harmonic descent; @kintsugi/consent auto-apply candidate)
/// - `≈ 0`: no meaningful pain change (peer's coordinate unchanged
///   structurally; Landing 1 falsification pattern for docstring-append)
///
/// This IS the substrate-honest form of the pain_gradient Reed used with
/// sc_hamming ratio proxy in Landing 8+9.6a. Composes with
/// `converge::stable_within(sc, pain_gradient, epsilon_pain)` and
/// `converge::knife_cut(sc, pain_gradient, epsilon_pain)`.
pub fn pain_gradient<const N: usize>(
    sc_before: &SpectralCoordinate<N>,
    sc_after: &SpectralCoordinate<N>,
) -> f64 {
    sample_pain(sc_after) - sample_pain(sc_before)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::sha::HashAlg;

    #[test]
    fn sample_pain_deterministic() {
        // Same coordinate → same pain magnitude
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(sample_pain(&sc), sample_pain(&sc));
    }

    #[test]
    fn sample_pain_in_unit_interval() {
        // Substrate-honest: pain ∈ [0, 1]
        let sc = SpectralCoordinate::<5>::hash(b"test");
        let pain = sample_pain(&sc);
        assert!(pain >= 0.0 && pain <= 1.0);
    }

    #[test]
    fn sample_pleasure_is_dual_of_pain() {
        let sc = SpectralCoordinate::<5>::hash(b"test");
        let pain = sample_pain(&sc);
        let pleasure = sample_pleasure(&sc);
        assert!((pain + pleasure - 1.0).abs() < 1e-10);
    }

    #[test]
    fn pain_gradient_zero_for_same_coord() {
        // No morphism ⇒ no pain change
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(pain_gradient(&sc, &sc), 0.0);
    }

    #[test]
    fn pain_gradient_signed() {
        // Different coordinates → signed change (may be positive or
        // negative depending on which has higher entropy)
        let sc_a = SpectralCoordinate::<5>::hash(b"a");
        let sc_b = SpectralCoordinate::<5>::hash(b"much longer input with different entropy");
        let grad = pain_gradient(&sc_a, &sc_b);
        // Just verify the sign is well-defined (either direction is valid;
        // the magnitude and sign are the substrate-honest quantities)
        assert!(grad.is_finite());
    }

    #[test]
    fn sample_pain_empty_hex_is_zero() {
        // Defensive: empty coordinate string yields 0.0 pain
        let sc = SpectralCoordinate::<5>::from_eigenvalue(String::new());
        assert_eq!(sample_pain(&sc), 0.0);
    }
}
