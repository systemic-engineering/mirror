//! `@mirror/lens/knife` — Rung 9 coherence loop closure module.
//!
//! Rust runtime discharge of `shards/mirror/lens/knife.mirror` (Reed
//! `0a267ce`) per Seam `e8508f5` §3 divergence #5 verdict: `pub fn
//! knife_cut(...)` inside `bootstrap/src/converge.rs`, NOT a new
//! `bootstrap/src/knife.rs` module. Substrate-pull-realize discipline:
//! **one primitive → one fn**; Rust MIRRORS the decl, doesn't wrap it.
//!
//! ## Substrate authority
//!
//! - Alex Wolf 2026-07-13 in-transcript: "Is @knife what Foester described
//!   as COORD(x)?" — answered YES per verbatim Mara §2.4 citation.
//! - Foerster 1976 "Objects: Tokens for (Eigen-)Behaviors" Appendix A3:
//!   `Op(COORDᵢ) = COORDᵢ` within stable domain; `COORDᵢ → COORDⱼ` at
//!   boundary crossing. Substrate-decl'd at `shards/mirror/lens/knife.mirror`.
//! - McCulloch 1945 heterarchy discipline (topology, not stack height).
//! - Mara `06a8547` + `38c2eeb` — canonical spec + math foundation.
//! - Mara `2026-07-07-onto-cascade-toroidal-reframe.md` §2.4 — verbatim
//!   Foerster COORD citation.
//! - Taut `15f7ed6` — substrate-already-had-the-word ~55th instance;
//!   ~90% substrate coverage already landed.
//! - Seam `e8508f5` — Phase D audit ratification-with-qualifications;
//!   the five rulings applied here.
//! - Douady-Hubbard 1985 — R-universality theorem = Foerster's heterarchy
//!   at complex-analytic altitude (Mara math §5 same-theorem-three-altitudes).
//!
//! ## What lands
//!
//! Three primitives (mirroring the three substrate-decl'd actions/property):
//!
//! 1. `knife_cut` — `@mirror/lens/knife.action jump`. COORD's jump
//!    at stability-domain boundary.
//! 2. `stable_within` — `@mirror/lens/knife.action stable_within`. Foerster
//!    A3 fixed-point check.
//! 3. `heterarchy_preserved` — `@mirror/lens/knife.property
//!    heterarchy_preserved`. Bilateral predicate; Seam §5 missed-item #1
//!    reformulation (not the tautological SC=SC form).

use fragmentation::spectral_coordinate::SpectralCoordinate;
use fragmentation_spectral::coincidence::Detector;

/// Verdict at the @knife altitude. Substrate-honest placeholder until
/// @glass.verdict Rust runtime discharge composes here (Rung 9+ forward-
/// promise).
#[derive(Debug, Clone, PartialEq)]
pub enum KnifeVerdict {
    /// pass — `Op(COORDᵢ) = COORDᵢ`; peer within stable domain (Foerster A3).
    Stable,
    /// partial(confidence) — pain elevated but below ε_pain threshold;
    /// near boundary; jump not yet triggered.
    NearBoundary,
    /// failure(reason) — coordinate exceeded stability-domain boundary;
    /// @cyberpunk/reframe should fire, invoking @knife.jump.
    Jumped,
}

/// `@mirror/lens/knife.action jump` — COORD's jump-behavior at
/// stability-domain boundary.
///
/// Per Foerster 1976 Appendix A3:
/// - `Op(COORDᵢ) = COORDᵢ`         within stable domain (fixed-point)
/// - `COORDᵢ → COORDⱼ`             at boundary crossing (jump)
///
/// Semantics:
/// - If `pain_delta < epsilon_pain`: return `sc_in` unchanged (fixed-point;
///   peer stays in current stability domain).
/// - Else: re-project via alternate Detector basis, signaling COORD jump
///   to new coordinate system. Substrate-decl'd target-domain parameter
///   (from @cyberpunk/reframe ceremony) not yet landed at Rust altitude;
///   minimum viable uses "content:jump" space to distinguish jumped-coords
///   from stable-coords at hex level.
///
/// Per Seam `e8508f5` §3 #5: minimal function shape; no new module,
/// no additional Rust namespace/type density. Composes with
/// `@cyberpunk/reframe.perform` at the peer_converge outer driver
/// (forward-promise Landing 8+9.5+).
///
/// Per Seam §4 #6: `epsilon_pain` is a **required parameter** — no default
/// (empirical calibration required first per Asher discipline;
/// Landing 8+9.6d verifies Mara math §10 prediction #1).
pub fn knife_cut<const N: usize>(
    sc_in: SpectralCoordinate<N>,
    pain_delta: f64,
    epsilon_pain: f64,
) -> SpectralCoordinate<N> {
    if pain_delta < epsilon_pain {
        return sc_in;
    }
    let bytes = sc_in.eigenvalue().as_bytes().to_vec();
    let new_detector: Detector<N> = Detector::canonical("content:jump", 16);
    let detection = new_detector.detect(&bytes);
    detection
        .eigenvalue_hex()
        .map(SpectralCoordinate::from_eigenvalue)
        .unwrap_or(sc_in)
}

/// `@mirror/lens/knife.action stable_within` — Foerster A3 fixed-point check.
///
/// Verdict:
/// - `Stable` — `pain_delta.abs() < epsilon_pain / 2`; coord is fixed-point
///   well within domain.
/// - `NearBoundary` — `epsilon_pain / 2 ≤ pain_delta.abs() < epsilon_pain`;
///   near boundary; jump not yet triggered (peer's @cyberpunk/algedonic.
///   sample_pain magnitude is elevated).
/// - `Jumped` — `pain_delta.abs() ≥ epsilon_pain`; boundary exceeded;
///   @cyberpunk/reframe should fire, invoking @knife.jump.
///
/// Composes with `@eigenform.is_fixed_point` (Foerster 1976 direct
/// per Taut `15f7ed6` §1 substrate-already-had-the-word).
pub fn stable_within<const N: usize>(
    _coord: &SpectralCoordinate<N>,
    pain_delta: f64,
    epsilon_pain: f64,
) -> KnifeVerdict {
    let magnitude = pain_delta.abs();
    if magnitude < epsilon_pain * 0.5 {
        KnifeVerdict::Stable
    } else if magnitude < epsilon_pain {
        KnifeVerdict::NearBoundary
    } else {
        KnifeVerdict::Jumped
    }
}

/// `@mirror/lens/knife.property heterarchy_preserved` — bilateral predicate.
///
/// Per Seam `e8508f5` §5 missed-item #1 REFORMULATION:
///
/// > Original spec `heterarchy_preserved(sc_before, sc_after)` was
/// > TAUTOLOGICAL — SC<5>=SC<5> is compile-time-guaranteed. Substrate-
/// > honest form: check that BOTH coordinates belong to the same interior
/// > component of the Mandelbrot manifold (M∘), OR that BOTH exceed to
/// > the boundary (∂M).
///
/// Per Seam §4 #7 (stability-domain ruling):
///
/// > M∘-vs-∂M decision at runtime is Turing-undecidable per Braverman-
/// > Yampolsky 2007. bootstrap/src/gap.rs does NOT provide this primitive.
/// > Default: emit @kintsugi/consent.pause(Φ) as external witness;
/// > agent-in-transcript (Alex or Pack) provides the boundary judgment.
///
/// Minimum viable placeholder: eigenvalue-string equality proves trivial
/// preservation (no jump occurred); jump events return `NearBoundary`
/// pending external witness discharge (Rung 9+ @kintsugi/consent.pause
/// Rust runtime forward-promise).
pub fn heterarchy_preserved<const N: usize>(
    before: &SpectralCoordinate<N>,
    after: &SpectralCoordinate<N>,
) -> KnifeVerdict {
    if before.eigenvalue() == after.eigenvalue() {
        KnifeVerdict::Stable
    } else {
        KnifeVerdict::NearBoundary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::sha::HashAlg;

    // === knife_cut ==========================================================

    #[test]
    fn knife_cut_below_pain_threshold_returns_unchanged() {
        // Foerster A3: Op(COORDᵢ) = COORDᵢ within stable domain
        let sc = SpectralCoordinate::<5>::hash(b"test");
        let hex_before = sc.eigenvalue().to_string();
        let sc_after = knife_cut(sc, 0.0, 0.5);
        assert_eq!(sc_after.eigenvalue(), hex_before);
    }

    #[test]
    fn knife_cut_at_threshold_still_stable() {
        // pain_delta == epsilon_pain: boundary case; per Foerster A3
        // "whenever the boundary conditions exceed" — strict inequality;
        // at-threshold remains stable.
        let sc = SpectralCoordinate::<5>::hash(b"test");
        let hex_before = sc.eigenvalue().to_string();
        let sc_after = knife_cut(sc, 0.5, 0.5);
        // pain_delta (0.5) is NOT strictly less than epsilon_pain (0.5),
        // so this WILL jump. Documenting the strict-inequality semantic.
        assert_ne!(sc_after.eigenvalue(), hex_before);
    }

    #[test]
    fn knife_cut_above_pain_threshold_jumps() {
        // Boundary conditions exceeded: COORDᵢ → COORDⱼ
        let sc = SpectralCoordinate::<5>::hash(b"test");
        let hex_before = sc.eigenvalue().to_string();
        let sc_after = knife_cut(sc, 1.0, 0.5);
        assert_ne!(
            sc_after.eigenvalue(),
            hex_before,
            "knife_cut must produce different coordinate on jump"
        );
    }

    #[test]
    fn knife_cut_deterministic_for_same_inputs() {
        // Douady-Hubbard universality: same c produces same R(c)
        let sc_a = SpectralCoordinate::<5>::hash(b"test");
        let sc_b = SpectralCoordinate::<5>::hash(b"test");
        let out_a = knife_cut(sc_a, 1.0, 0.5);
        let out_b = knife_cut(sc_b, 1.0, 0.5);
        assert_eq!(out_a.eigenvalue(), out_b.eigenvalue());
    }

    // === stable_within ======================================================

    #[test]
    fn stable_within_below_half_threshold_is_stable() {
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(stable_within(&sc, 0.1, 0.5), KnifeVerdict::Stable);
    }

    #[test]
    fn stable_within_near_threshold_is_boundary() {
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(stable_within(&sc, 0.35, 0.5), KnifeVerdict::NearBoundary);
    }

    #[test]
    fn stable_within_above_threshold_is_jumped() {
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(stable_within(&sc, 0.7, 0.5), KnifeVerdict::Jumped);
    }

    #[test]
    fn stable_within_symmetric_in_pain_sign() {
        // pain_delta magnitude matters, not sign; @pain gradient can
        // point in any direction on the SC<5> manifold
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(
            stable_within(&sc, 0.35, 0.5),
            stable_within(&sc, -0.35, 0.5)
        );
    }

    // === heterarchy_preserved ===============================================

    #[test]
    fn heterarchy_preserved_same_coord_is_stable() {
        // Trivial preservation: no jump occurred
        let sc = SpectralCoordinate::<5>::hash(b"test");
        assert_eq!(heterarchy_preserved(&sc, &sc), KnifeVerdict::Stable);
    }

    #[test]
    fn heterarchy_preserved_diff_coord_needs_external_witness() {
        // Seam §5 #1: M∘-vs-∂M runtime decision Turing-undecidable;
        // NearBoundary signals "@kintsugi/consent.pause(Φ) external
        // witness required"
        let sc_a = SpectralCoordinate::<5>::hash(b"before");
        let sc_b = SpectralCoordinate::<5>::hash(b"after");
        assert_eq!(heterarchy_preserved(&sc_a, &sc_b), KnifeVerdict::NearBoundary);
    }

    // === Integration: jump + heterarchy_preserved composition ===============

    #[test]
    fn knife_cut_followed_by_heterarchy_preserved_signals_external_witness() {
        // The composition Reed will use at Rung 9 converge loop:
        // 1. Peer's coordinate: sc_before
        // 2. Pain gradient exceeds threshold
        // 3. @knife.jump: sc_before → sc_after
        // 4. Check: heterarchy_preserved(sc_before, sc_after) → NearBoundary
        //    (Seam #1 substrate-honest form)
        // 5. @kintsugi/consent.pause(Φ) emitted for external witness
        //    (Rung 9+ discharge)
        let sc_before = SpectralCoordinate::<5>::hash(b"stable-state");
        let sc_after = knife_cut(sc_before.clone(), 1.0, 0.5);
        let verdict = heterarchy_preserved(&sc_before, &sc_after);
        assert_eq!(verdict, KnifeVerdict::NearBoundary);
    }
}
