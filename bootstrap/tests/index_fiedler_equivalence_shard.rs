//! Rung 8 Landing 2 RED — mirror owns its own @fractal-coherence measurement
//! per Alex 2026-07-13 ("the spectral__spectral_index is something that
//! currently lives in spectral I presume? This is something that needs to
//! be pulled into mirror") + Taut `77b8e14` migration mapping scout §6.
//!
//! ## Substrate authority
//!
//! - Alex 2026-07-13 in-transcript: pull spectral_index into mirror; "Fire"
//!   authorization for the 5-landing sequence.
//! - Taut `77b8e14` — `docs/scouts/2026-07-13-taut-spectral-to-mirror-
//!   migration-mapping-scout.md`. Landing 2 RED-lock target: FIEDLER_REF =
//!   0.0612 empirically captured from live commit-msg envelope on this repo
//!   (1138 files, 165 nodes, 6676 edges).
//! - Taut `b52b008` — Fiedler 0.0612 IS λ₀(Δ_F) = 6% residual H¹(F)
//!   obstruction; stable across 202 commits = Douady-Hubbard universality
//!   empirically confirmed.
//! - Mara `2c64060` §4 — Mandelbrot identification; substrate coherence
//!   measurement IS the parameter-space membership query.
//! - Recognition #43 (mirror IS content-addressed build system) +
//!   Recognition #55 (form/process partition; DAG is form, measurement is
//!   process; belong at same altitude).
//! - Mirror's existing `sheaf_laplacian` primitive at `bootstrap/src/
//!   sheaf_laplacian.rs` (T8-landed) provides LAPACK `dsyev` path via
//!   `prismqueer::ffi::eigenvalues`. Landing 3 GREEN routes eigenvalue
//!   computation through this primitive (substrate-pull win per Taut §6).
//!
//! ## RED contract
//!
//! T1 — `bootstrap::index::index(path)` returns a non-dark EigenvalueProfile
//!      for the mirror repo root (regression check).
//! T2 — Fiedler value on the mirror repo root is within ±5e-2 of the
//!      empirical reference 0.0612 captured by Taut `77b8e14`. Loose
//!      epsilon accommodates repo drift during this shipping session +
//!      LAPACK/Jacobi numerical difference between spectral's Jacobi and
//!      mirror's LAPACK dsyev.
//! T3 — Fiedler value is in the [0, 1] range (algebraic-connectivity
//!      invariant post-normalization; matches spectral's convention).
//! T4 — `EigenvalueProfile::values` is a `[f64; 16]` array (top-16
//!      eigenvalues per Taut §6 profile shape).

use std::path::PathBuf;

/// The mirror repo root — walks up from CARGO_MANIFEST_DIR (`bootstrap/`).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

// === T1: `index()` returns non-dark profile for connected substrate =====
#[test]
fn t01_index_returns_non_dark_profile_for_repo_root() {
    let root = repo_root();
    let profile = mirror::index::index(&root);
    assert!(
        !profile.is_dark(),
        "T1: `bootstrap::index::index(mirror_repo_root)` must return a \
         non-dark EigenvalueProfile (repo has connected substrate DAG per \
         Taut `77b8e14` empirical capture: 165 nodes, 6676 edges); got \
         all-zero profile"
    );
}

// === T2: Fiedler matches spectral's empirical 0.0612 reference ==========
#[test]
fn t02_fiedler_matches_spectral_empirical_reference() {
    let root = repo_root();
    let profile = mirror::index::index(&root);
    let fiedler = profile.fiedler_value();
    const FIEDLER_REF: f64 = 0.0612;
    const EPSILON: f64 = 5e-2;
    let delta = (fiedler - FIEDLER_REF).abs();
    assert!(
        delta < EPSILON,
        "T2: Fiedler on mirror repo root must be within ±{EPSILON} of the \
         empirical reference {FIEDLER_REF} captured by Taut `77b8e14` from \
         live commit-msg envelope (spectral MCP's spectral_index tool). \
         Loose epsilon accommodates LAPACK/Jacobi numerical difference + \
         repo drift during this shipping session. Got fiedler={fiedler}, \
         delta={delta}"
    );
}

// === T3: Fiedler in [0, 1] range (algebraic-connectivity invariant) =====
#[test]
fn t03_fiedler_in_unit_interval() {
    let root = repo_root();
    let profile = mirror::index::index(&root);
    let fiedler = profile.fiedler_value();
    assert!(
        fiedler >= 0.0 && fiedler <= 1.0,
        "T3: Fiedler value on mirror repo root must be in [0, 1] \
         (spectral's normalization convention: top-16 eigenvalues \
         divided by max); got fiedler={fiedler}"
    );
    assert!(
        !fiedler.is_nan(),
        "T3: Fiedler value must not be NaN; got {fiedler}"
    );
}

// === T4: EigenvalueProfile shape = [f64; 16] top-16 eigenvalues =========
#[test]
fn t04_profile_shape_is_top_16_eigenvalues() {
    let root = repo_root();
    let profile = mirror::index::index(&root);
    let vals: &[f64] = &profile.values;
    assert_eq!(
        vals.len(),
        16,
        "T4: EigenvalueProfile must carry top-16 eigenvalues per Taut \
         `77b8e14` §6 profile shape; got {} values",
        vals.len()
    );
    // Sorted ascending after normalization
    let mut sorted_check = vals.to_vec();
    sorted_check.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    for (i, (&raw, &srt)) in vals.iter().zip(sorted_check.iter()).enumerate() {
        assert!(
            (raw - srt).abs() < 1e-9,
            "T4: EigenvalueProfile.values must be sorted ascending; \
             mismatch at index {i}: values[{i}]={raw}, expected={srt}"
        );
    }
}
