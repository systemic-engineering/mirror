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

// === T5: Multifractal spectrum — Rung 8 Landing 6 LOAD-BEARING ==========
#[test]
fn t05_multifractal_witness_positive_for_repo_root() {
    let root = repo_root();
    let profile = mirror::index::index(&root);
    let q_range = mirror::index::canonical_q_range();
    let spectrum = profile.multifractal_spectrum(&q_range);
    assert!(
        spectrum.multifractal_witness > 0.0,
        "T5: multifractal witness (max f(α) − min f(α)) must be positive \
         on the mirror repo DAG (Mara math §10 prediction #2: if mirror IS \
         Mandelbrot-shaped, f(α) shows non-trivial interval width). Got: \
         witness={}",
        spectrum.multifractal_witness
    );
    assert!(
        !spectrum.d_1.is_nan() && spectrum.d_1.is_finite(),
        "T5: information dimension D_1 must be finite; got {}",
        spectrum.d_1
    );
    assert!(
        !spectrum.q_values.is_empty(),
        "T5: q_values must be non-empty"
    );
    assert_eq!(
        spectrum.q_values.len(),
        spectrum.f_alpha.len(),
        "T5: q_values and f_alpha must have same length"
    );
}

// === T6: Rényi entropies consistent (H_0 ≥ H_1 ≥ H_2) =====================
#[test]
fn t06_renyi_entropies_monotone_decreasing_in_q() {
    let root = repo_root();
    let profile = mirror::index::index(&root);
    let hs = profile.renyi_entropies(&[0.0, 1.0, 2.0]);
    assert_eq!(hs.len(), 3, "T6: renyi_entropies must return one value per q");
    assert!(
        hs[0] >= hs[1] - 1e-6,
        "T6: H_0 ≥ H_1 (Rényi entropies are monotone-decreasing in q); \
         got H_0={}, H_1={}",
        hs[0],
        hs[1]
    );
    assert!(
        hs[1] >= hs[2] - 1e-6,
        "T6: H_1 ≥ H_2; got H_1={}, H_2={}",
        hs[1],
        hs[2]
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
