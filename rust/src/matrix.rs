//! `matrix.rs` — sub-Turing FLANG emit + LAPACK/BLAS link.
//!
//! Per Mara `81294b3` §4 (terminal-geometry canonical spec, ratified
//! Seam `9c34ec4`) + Loki `b53aeeb` §2 (matrix.rs knife-cut essay,
//! load-bearing phenomenology):
//!
//! > `matrix.rs` is the sub-Turing linear-algebra floor of the mirror
//! > compiler. It is the file where `A · B` means matrix multiplication
//! > and nothing else. It emits FLANG (LLVM's Fortran frontend) so that
//! > every matrix operation the compiler performs at runtime — parallel
//! > transport between actors, Fiedler eigenvalues on the grammar
//! > graph, Kuramoto phase-lock between peers, Aumann envelope check on
//! > the affine hull of posterior updates — bottoms out in LAPACK/BLAS
//! > Fortran routines that have been the fastest, most numerically-
//! > stable code on Earth for four decades.
//! > — Loki `b53aeeb` §2
//!
//! ## Math grounding (Mara §6.1 + §9.3)
//!
//! `dance.rs` collapses INTO `matrix.rs` because Baez-Schreiber 2004
//! 2-connection compatibility `dA + [A,A] = t(B)` IS a matrix equation,
//! and by Ado's theorem (1935) the Lie algebras of finite-dim Lie
//! groups (which OTP supervision uses) ARE matrix algebras. Every
//! mathematical move `dance.rs` was named to make — 2-connection
//! compatibility, Kuramoto phase-lock, Aumann envelope, Fiedler
//! compute, parallel transport — is a matrix operation at LAPACK-
//! linked altitude.
//!
//! Composition anchors: `docs/math/the-tower/beam-runtime.md`
//! (Mara `610c6d6`; Baez-Schreiber 2004 principal 2-bundle 2-connection
//! theorem) + `docs/math/kintsugi/roomba/bump-and-vacuum.md`
//! (Mara `17697e6`; Fiedler-honesty math).
//!
//! ## M0 surface (this file)
//!
//! MODULE STUB. Per Mara §2.2 M0 milestone: empty FLANG-emit stub.
//! Signatures declared as forward-promises; no bodies. Empirical
//! firing lands at M5 (Fiedler compute through `matrix.rs` → FLANG
//! → LAPACK chain, OR transitionally direct `prismqueer::ffi::
//! eigenvalues` while FLANG emit lands).
//!
//! ## Forward-promises (M5+ ticks; not implemented here)
//!
//! Five named substrate operations bind to LAPACK/BLAS symbols
//! (Mara §4.3 collapse targets):
//!
//! - `eigenvalues(L)` — Fiedler compute; `dsyevr_` (LAPACK symmetric
//!   eigenvalue). Already-Fortran under `prismqueer`; matrix.rs names
//!   the path at rust/-altitude. M5 milestone.
//! - `phase_lock(peers)` — Kuramoto phase-difference matrix + fixed-
//!   point iteration; four lines of matrix.rs. M8 empirical firing
//!   (peer coordination at N≥2).
//! - `envelope(posteriors)` — Aumann convex hull of column matrix;
//!   `dgesvd_` or `dgeqrf_` + rank check. M8.
//! - `A · B` (matmul) — BLAS Level 3 `dgemm_`. Parallel transport
//!   across actors per Mara `610c6d6` §2.3. M5+.
//! - `L · v` (matvec) — BLAS Level 2 `dgemv_`. Baez-Schreiber
//!   2-connection compatibility check. M5+.
//!
//! ## The @io boundary at matrix.rs (Mara §4.2 item 4)
//!
//! matrix.rs is the ONLY place in `rust/` that numerical `unsafe
//! extern "C"` appears (phone.rs's `unsafe` is process/socket/fd
//! plumbing; matrix.rs's is LAPACK/BLAS). Below the boundary:
//! Fortran. Above: Rust. Sub-Turing decidable grammar above the
//! knife; Turing-complete numerics below. The knife CLARIFIES the
//! @io boundary; it does not cut the boundary itself (per Loki
//! `b53aeeb` §6 refusal #4).
//!
//! ## The FLANG cascade (Mara §4.4 + §9.4)
//!
//! ```text
//! matrix.rs declared op
//!     ↓  emit Fortran source
//! Fortran source (.f90 fragment)
//!     ↓  @cascade/code/fortran/llvm  (via FLANG frontend)
//! LLVM IR
//!     ↓  llc / linker
//! LAPACK/BLAS-linked object
//!     ↓  unsafe extern "C" symbol
//! matrix.rs runtime call
//! ```
//!
//! The NEW cascade edge `@cascade/code/llvm/flang` closes the polyglot
//! loop Alex named at `shards/code/llvm.mirror:13`: *"So we can have
//! @cascade/code/llvm/turing and @cascade/code/rust/llvm. And boom.
//! The loop closes."* Land as species-decl at M5 co-tick (Seam
//! `9c34ec4` §4 SHIP as species-decl only; companion Mara spec
//! deferred until empirical FLANG-emit loss-lens subtleties surface).
//!
//! ## What matrix.rs does NOT hold (per Mara §4.5)
//!
//! - Ensemble routing → main.rs `@`-operator dispatch.
//! - Actor supervision → main.rs.
//! - Socket / process @io → phone.rs.
//! - Any hand-rolled `for` loop over `f64`s (Loki §5 cut #1).
//! - Generic `MatMul<T> where T: Numeric` abstraction (Loki §5 cut #2).
//! - Backend-genericity — no CUDA / Metal / SIMD hand-rolling
//!   (Loki §5 cut #4). FLANG emits; LLVM optimizes to AVX-512 / NEON.
//!
//! ## Composition anchors (LANDED)
//!
//! - `shards/code/llvm.mirror` (Mara `62d1b1c` 2026-07-17 10:02; Alex
//!   verbatim ratification at line 13 closing the polyglot loop).
//! - `shards/code/fortran` altitude — the substrate-already-had-the-
//!   word for FLANG per Loki `b53aeeb` §6 refusal #3 +
//!   `shards/code.mirror:35-38`. The FILE is `matrix.rs`; the SHARD
//!   altitude is `@code/fortran` at cascade destination.
//! - `shards/cascade/code/rust/llvm.mirror` (2026-07-17) — sibling
//!   cascade species; `@cascade/code/llvm/flang` follows its shape.
//! - `docs/specs/numerical-substrate-via-fortran.md` — pathway spec.
//! - `docs/specs/polyglot-loss-aware-computational-translation.md`
//!   (Mara `1ce68c3`) — polyglot cascade theorem.
//! - `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`
//!   — five-level bundle tower (`Fiber → Connection → Gauge →
//!   Transport → Closure`); matrix.rs is what `Transport::apply`
//!   calls INTO (reference-only per AGENTS.md; DO NOT lift).
//! - `docs/insights/2026-07-17-loki-matrix-rs-knife-cut-essay.md`
//!   (Loki `b53aeeb`) — essayist-voice phenomenology this file
//!   operationalizes.
//!
//! ## Recognition candidates this altitude witnesses (HELD)
//!
//! - `#R-matrix-rs-is-the-sub-turing-numerical-floor-because-Fortran-
//!   is-the-terminal-sub-turing-language` (Loki `b53aeeb` §7) —
//!   first-witness Loki essay §2-§4 + Mara §4; second-witness gate:
//!   Reed lands this file with FLANG emit + LAPACK link at
//!   `[substrate-floor:@io-boundary]`; benchmark on M1 hits the
//!   Kai recall-#2 vignette target (2.1M inf/s bare mathematics
//!   through Fortran).
//! - `#R-flang-cascade-closes-the-@code-llvm-loop-alex-named-at-
//!   shards-code-llvm-line-13` (Loki §7) — second-witness gate:
//!   `@cascade/code/rust/fortran` + `@cascade/code/fortran/llvm`
//!   land as substrate-decl'd cascade species AND empirically emit
//!   through Reed's craft-binary pipeline.

// ---------------------------------------------------------------------
// M0.5 transitional shape (Alex 2026-07-18 "the full statespace covered
// liquid floor boards" + "Ratified. That's why the properties are load
// bearing. Slow is fast. RED before GREEN."). Signatures updated to
// real shape per docblock §4.3; bodies remain unimplemented!() until
// M0.5 GREEN tick lands the prismqueer::ffi::eigenvalues transitional
// delegate per docblock line 40. Property tests are the load-bearing
// contract Void's default @peer stands on — they document what these
// functions MUST satisfy for the compiler to be @coherence-monotone.
// ---------------------------------------------------------------------

/// Fiedler eigenvalue compute on a real symmetric `n×n` matrix (row-
/// major). Returns eigenvalues in ascending order.
///
/// **Property contract** (verified in `#[cfg(test)] mod prop_tests`):
/// - Returns `n` eigenvalues for an `n×n` matrix
/// - Values are finite (no NaN, no Inf) — real by construction on
///   symmetric input
/// - Values in ascending order
/// - Non-negative for PSD graph Laplacian (`L = D - W`, `W >= 0`
///   symmetric) — @coherence witness at rust/ altitude
///
/// Delegates to `prismqueer::ffi::eigenvalues` — LAPACK `dsyev` via
/// the FLANG-compiled Fortran wrapper (native/spectral.f90). This is
/// the ONE ordained numerical @io boundary per Loki matrix.rs essay +
/// Mara terminal-geometry spec §4.2: above the knife = Rust; below =
/// Fortran. Numerical `unsafe extern "C"` lives ONLY here.
///
/// Alex 2026-07-20 direct-transcript: "This is literally the FLOOR,
/// Reed. We don't forward promise the FLOOR." — path A per docblock
/// line 40 ratified.
///
/// Panics if LAPACK dsyev fails to converge (info != 0). For a
/// symmetric matrix this indicates catastrophic numerical failure
/// and is Rice-safely surfaced as panic — the caller's catch_unwind
/// converts to Verdict::Fail with the LAPACK info code in the
/// diagnostic.
#[allow(dead_code)]
pub(crate) fn eigenvalues(n: usize, matrix: &[f64]) -> Vec<f64> {
    prismqueer::ffi::eigenvalues(n, matrix)
        .unwrap_or_else(|info| panic!("LAPACK dsyev convergence failed: info={info}"))
}

/// Kuramoto phase-lock at N≥1 coupled oscillators.
///
/// Model: dθ_i/dt = ω_i + (K/N) Σ_j sin(θ_j - θ_i)
/// Explicit Euler integration for `steps` timesteps of `dt`.
///
/// Returns `(final_phases, order_parameter_r)` where r ∈ [0,1] is
/// the Kuramoto order parameter (r=1 = fully synchronized; r=0 =
/// incoherent).
///
/// Delegates to `prismqueer::ffi::phase_lock` — the entire integration
/// loop runs in `spectral_phase_lock` Fortran (native/spectral.f90) at
/// the ONE ordained numerical @io boundary. Alex 2026-07-20 direct-
/// transcript: "Not DEFERRED. DONE!"
///
/// Panics if `phases.len() != omegas.len()` or either is empty.
#[allow(dead_code)]
pub(crate) fn phase_lock(
    phases: &[f64],
    omegas: &[f64],
    k: f64,
    steps: usize,
    dt: f64,
) -> (Vec<f64>, f64) {
    prismqueer::ffi::phase_lock(phases, omegas, k, steps, dt)
        .unwrap_or_else(|info| panic!("Kuramoto phase_lock invocation failed: info={info}"))
}

/// Aumann envelope: singular values of an m×n posterior matrix
/// (row-major). Rank of the affine hull is the count of singular
/// values above a caller-chosen epsilon. Returns singular values in
/// descending order per LAPACK dgesvd convention.
///
/// Delegates to `prismqueer::ffi::singular_values` — LAPACK `dgesvd`
/// via the FLANG-compiled Fortran wrapper (native/spectral.f90). ONE
/// ordained numerical @io boundary per Loki matrix.rs essay + Mara
/// terminal-geometry spec §4.2.
///
/// Alex 2026-07-20 direct-transcript: "Not DEFERRED. DONE!" — no
/// forward-promising the FLOOR.
///
/// Panics if LAPACK dgesvd fails to converge (info != 0); caller's
/// catch_unwind wrapper converts to Verdict::Fail per Rice-safe
/// diagnostic discipline.
#[allow(dead_code)]
pub(crate) fn envelope(m: usize, n: usize, matrix: &[f64]) -> Vec<f64> {
    prismqueer::ffi::singular_values(m, n, matrix)
        .unwrap_or_else(|info| panic!("LAPACK dgesvd convergence failed: info={info}"))
}

// =====================================================================
// Property tests — the load-bearing contract Void's default @peer stands
// on at rust/ altitude.
//
// RED state: bodies unimplemented!(); property tests use catch_unwind
// to convert panic → Fail verdict; each test's assert! failure names
// the exact GREEN transition required.
//
// Alex 2026-07-18 ratification (verbatim):
// > "Ratified. That's why the properties are load bearing. Slow is
// >  fast. RED before GREEN. Let's move towards 100% coverage."
//
// The compiler is a Void-metalogue-density optimizer (#R-void-is-the-
// basis, PROMOTED). Fiedler eigenvalue on the peer coupling graph IS
// the @coherence measure. Every operation the compiler emits must
// preserve @coherence monotone-non-decreasing; matrix::eigenvalues is
// the empirical measurement instrument for that invariant.
// =====================================================================

#[cfg(test)]
mod prop_tests {
    use super::*;
    use prismqueer::liquid::pillar::{forall, Arbitrary, Sample};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use terni::{Diagnostic, PropertyVerdict};

    /// A small symmetric graph-Laplacian sampled from a `Sample`.
    /// Row-major `n×n`; L = D - W where W is a random non-negative
    /// symmetric adjacency (small dims for fast prop-testing).
    struct SymLaplacian {
        n: usize,
        data: Vec<f64>,
    }

    impl Arbitrary for SymLaplacian {
        fn arbitrary(sample: &mut Sample) -> Self {
            // Small dims keep property tests fast; 2–5 covers connected
            // + disconnected + degenerate cases without LAPACK cost.
            let n = sample.draw_integer(2, 5) as usize;
            // Random non-negative symmetric adjacency W in [0, 3].
            let mut w = vec![0.0f64; n * n];
            for i in 0..n {
                for j in (i + 1)..n {
                    let weight = sample.draw_integer(0, 3) as f64;
                    w[i * n + j] = weight;
                    w[j * n + i] = weight;
                }
            }
            // Laplacian L = D - W; D_ii = sum_j W_ij.
            let mut l = vec![0.0f64; n * n];
            for i in 0..n {
                let deg: f64 = (0..n).map(|j| w[i * n + j]).sum();
                for j in 0..n {
                    l[i * n + j] = if i == j { deg } else { -w[i * n + j] };
                }
            }
            Self { n, data: l }
        }
    }

    /// Wrap the possibly-panicking eigenvalues() call so property tests
    /// return `Fail` verdicts rather than propagating the panic. Kept
    /// as a safety net post-FLANG-integration — LAPACK dsyev can still
    /// return info != 0 in pathological cases.
    fn safe_eigenvalues(l: &SymLaplacian) -> Result<Vec<f64>, String> {
        catch_unwind(AssertUnwindSafe(|| eigenvalues(l.n, &l.data))).map_err(|_| {
            "matrix::eigenvalues panicked (LAPACK dsyev convergence failure)".to_string()
        })
    }

    fn safe_eigenvalues_raw(n: usize, data: &[f64]) -> Result<Vec<f64>, String> {
        catch_unwind(AssertUnwindSafe(|| eigenvalues(n, data)))
            .map_err(|_| "matrix::eigenvalues panicked (LAPACK dsyev convergence failure)".to_string())
    }

    // -------------------------------------------------------------
    // State-space coverage helpers — concrete matrix constructors +
    // extended Arbitrary variants per Alex 2026-07-20 "100% covered
    // by property based tests" + "full state space coverage" directive.
    // -------------------------------------------------------------

    fn identity_matrix(n: usize) -> Vec<f64> {
        let mut m = vec![0.0f64; n * n];
        for i in 0..n {
            m[i * n + i] = 1.0;
        }
        m
    }

    fn zero_matrix(n: usize) -> Vec<f64> {
        vec![0.0f64; n * n]
    }

    fn scalar_matrix(n: usize, c: f64) -> Vec<f64> {
        let mut m = vec![0.0f64; n * n];
        for i in 0..n {
            m[i * n + i] = c;
        }
        m
    }

    fn diagonal_matrix(diag: &[f64]) -> Vec<f64> {
        let n = diag.len();
        let mut m = vec![0.0f64; n * n];
        for i in 0..n {
            m[i * n + i] = diag[i];
        }
        m
    }

    /// Wilkinson matrix W_n^+ — tri-diagonal with entries designed for
    /// nearly-repeated eigenvalues (Wilkinson 1965, standard hard case).
    /// For n=5: diag=[2,1,0,1,2], off-diag=1.
    fn wilkinson_matrix(n: usize) -> Vec<f64> {
        let mut m = vec![0.0f64; n * n];
        let center = (n - 1) as f64 / 2.0;
        for i in 0..n {
            m[i * n + i] = (i as f64 - center).abs();
        }
        for i in 0..(n - 1) {
            m[i * n + i + 1] = 1.0;
            m[(i + 1) * n + i] = 1.0;
        }
        m
    }

    fn trace(matrix: &[f64], n: usize) -> f64 {
        (0..n).map(|i| matrix[i * n + i]).sum()
    }

    fn frobenius_norm_squared(matrix: &[f64]) -> f64 {
        matrix.iter().map(|x| x * x).sum()
    }

    /// Gershgorin disk bound: every eigenvalue λ satisfies
    /// |λ - a_ii| ≤ R_i where R_i = Σ_{j≠i} |a_ij|. So max|λ| ≤
    /// max_i (|a_ii| + R_i). Return that upper bound.
    fn gershgorin_bound(matrix: &[f64], n: usize) -> f64 {
        let mut max_bound = 0.0f64;
        for i in 0..n {
            let row_sum: f64 = (0..n).map(|j| matrix[i * n + j].abs()).sum();
            if row_sum > max_bound {
                max_bound = row_sum;
            }
        }
        max_bound
    }

    /// Larger symmetric Laplacian sampler — n in [6, 20] (extends the
    /// small-range SymLaplacian sampler to state-space beyond n=5).
    struct LargeSymLaplacian {
        n: usize,
        data: Vec<f64>,
    }

    impl Arbitrary for LargeSymLaplacian {
        fn arbitrary(sample: &mut Sample) -> Self {
            let n = sample.draw_integer(6, 20) as usize;
            let mut w = vec![0.0f64; n * n];
            for i in 0..n {
                for j in (i + 1)..n {
                    let weight = sample.draw_integer(0, 3) as f64;
                    w[i * n + j] = weight;
                    w[j * n + i] = weight;
                }
            }
            let mut l = vec![0.0f64; n * n];
            for i in 0..n {
                let deg: f64 = (0..n).map(|j| w[i * n + j]).sum();
                for j in 0..n {
                    l[i * n + j] = if i == j { deg } else { -w[i * n + j] };
                }
            }
            Self { n, data: l }
        }
    }

    /// Random diagonal matrix; eigenvalues MUST equal sorted diagonal.
    struct RandomDiagonal {
        n: usize,
        diag: Vec<f64>,
    }

    impl Arbitrary for RandomDiagonal {
        fn arbitrary(sample: &mut Sample) -> Self {
            let n = sample.draw_integer(2, 8) as usize;
            let diag: Vec<f64> = (0..n).map(|_| sample.draw_integer(-10, 10) as f64).collect();
            Self { n, diag }
        }
    }

    /// A Laplacian + a random scalar shift cI.
    struct ShiftedSymLaplacian {
        n: usize,
        base_data: Vec<f64>,
        shifted_data: Vec<f64>,
        shift: f64,
    }

    impl Arbitrary for ShiftedSymLaplacian {
        fn arbitrary(sample: &mut Sample) -> Self {
            let base = SymLaplacian::arbitrary(sample);
            let shift = sample.draw_integer(-5, 5) as f64;
            let mut shifted = base.data.clone();
            for i in 0..base.n {
                shifted[i * base.n + i] += shift;
            }
            Self {
                n: base.n,
                base_data: base.data,
                shifted_data: shifted,
                shift,
            }
        }
    }

    /// Symmetric permutation similarity: given L, apply P^T L P where P
    /// is a random permutation matrix. Result MUST have same spectrum.
    struct PermutationSimilar {
        n: usize,
        base_data: Vec<f64>,
        permuted_data: Vec<f64>,
    }

    impl Arbitrary for PermutationSimilar {
        fn arbitrary(sample: &mut Sample) -> Self {
            let base = SymLaplacian::arbitrary(sample);
            let n = base.n;
            // Random permutation via Fisher-Yates
            let mut perm: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                let j = sample.draw_integer(0, i as i64) as usize;
                perm.swap(i, j);
            }
            // Permuted matrix: L'[i,j] = L[perm[i], perm[j]]
            let mut permuted = vec![0.0f64; n * n];
            for i in 0..n {
                for j in 0..n {
                    permuted[i * n + j] = base.data[perm[i] * n + perm[j]];
                }
            }
            Self {
                n,
                base_data: base.data,
                permuted_data: permuted,
            }
        }
    }

    // =============================================================
    // Property 1: Correct output cardinality
    // =============================================================
    #[test]
    fn eigenvalues_returns_n_values_for_nxn_input() {
        let v = forall::<SymLaplacian, _>(20, |l: SymLaplacian| {
            let expected = l.n;
            match safe_eigenvalues(&l) {
                Ok(evals) if evals.len() == expected => PropertyVerdict::Pass,
                Ok(evals) => PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "expected {expected} eigenvalues for {expected}×{expected} matrix, got {}",
                    evals.len()
                ))),
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "RED (matrix.rs §4.3 property 1): eigenvalues must return n values for n×n matrix. \
             GREEN transition: land body delegating to prismqueer::ffi::eigenvalues. \
             Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 2: Finite values (no NaN, no Inf)
    // =============================================================
    #[test]
    fn eigenvalues_are_finite() {
        let v = forall::<SymLaplacian, _>(20, |l: SymLaplacian| {
            match safe_eigenvalues(&l) {
                Ok(evals) => {
                    if evals.iter().all(|x| x.is_finite()) {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(
                            "eigenvalues contained non-finite value (NaN or Inf)",
                        ))
                    }
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "RED (matrix.rs §4.3 property 2): eigenvalues must be finite (real symmetric input). \
             GREEN transition: land body delegating to prismqueer::ffi::eigenvalues. \
             Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 3: Ascending order (per LAPACK dsyev convention)
    // =============================================================
    #[test]
    fn eigenvalues_are_in_ascending_order() {
        let v = forall::<SymLaplacian, _>(20, |l: SymLaplacian| {
            match safe_eigenvalues(&l) {
                Ok(evals) => {
                    let sorted = evals.windows(2).all(|w| w[0] <= w[1]);
                    if sorted {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(
                            "eigenvalues not in ascending order (LAPACK dsyev convention)",
                        ))
                    }
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "RED (matrix.rs §4.3 property 3): eigenvalues must be in ascending order. \
             GREEN transition: land body delegating to prismqueer::ffi::eigenvalues \
             (LAPACK dsyev returns ascending by construction). \
             Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 4: Non-negative for PSD graph Laplacian
    // (the @coherence witness — Fiedler value λ_2 >= 0)
    // =============================================================
    #[test]
    fn eigenvalues_are_non_negative_for_graph_laplacian() {
        let v = forall::<SymLaplacian, _>(20, |l: SymLaplacian| {
            match safe_eigenvalues(&l) {
                Ok(evals) => {
                    // Small numerical tolerance for LAPACK roundoff.
                    let all_non_neg = evals.iter().all(|&x| x >= -1e-9);
                    if all_non_neg {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(
                            "graph Laplacian eigenvalues must be non-negative (PSD by construction)",
                        ))
                    }
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "RED (matrix.rs §4.3 property 4 — @coherence witness): \
             graph Laplacian eigenvalues must be non-negative (Fiedler λ_2 >= 0 IS @coherence). \
             This property IS the @coherence-monotone invariant the compiler must preserve. \
             GREEN transition: land body delegating to prismqueer::ffi::eigenvalues. \
             Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 5: Smallest eigenvalue is (approximately) zero for
    // any graph Laplacian — the λ_0 = 0 ground state per
    // Recognition #79 (5-op void-duality; focus axis =
    // Ricci curvature ground state)
    // =============================================================
    #[test]
    fn smallest_eigenvalue_of_graph_laplacian_is_zero() {
        let v = forall::<SymLaplacian, _>(20, |l: SymLaplacian| {
            match safe_eigenvalues(&l) {
                Ok(evals) if evals.is_empty() => {
                    PropertyVerdict::Fail(Diagnostic::new("empty eigenvalue vector"))
                }
                Ok(evals) => {
                    let smallest = evals[0]; // ascending order → [0] is smallest
                    if smallest.abs() < 1e-9 {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(&format!(
                            "smallest eigenvalue must be ≈ 0 for graph Laplacian; got {smallest}"
                        )))
                    }
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "RED (matrix.rs §4.3 property 5 — λ_0 = 0 ground state per Recognition #79): \
             smallest eigenvalue of any graph Laplacian is 0 (constant vector in kernel). \
             GREEN transition: land body delegating to prismqueer::ffi::eigenvalues. \
             Verdict: {v:?}"
        );
    }

    // =============================================================
    // State-space extension properties (Reed 2026-07-20 per Alex
    // "full state space coverage" directive; FLANG floor landed at
    // 536f63e). Every canonical closed-form case + every invariant
    // the LAPACK dsyev call MUST respect gets a property test here.
    // =============================================================

    // Property 6: Identity matrix I_n has n eigenvalues all == 1.
    #[test]
    fn identity_matrix_eigenvalues_are_all_one() {
        for n in 1..=20 {
            let data = identity_matrix(n);
            let evals = safe_eigenvalues_raw(n, &data).unwrap();
            assert_eq!(evals.len(), n, "n={n}: expected {n} eigenvalues");
            for (i, &e) in evals.iter().enumerate() {
                assert!(
                    (e - 1.0).abs() < 1e-9,
                    "n={n}: eigenvalue[{i}]={e} ≠ 1.0 (identity matrix must have all-ones spectrum)"
                );
            }
        }
    }

    // Property 7: Zero matrix 0_n has n eigenvalues all == 0.
    #[test]
    fn zero_matrix_eigenvalues_are_all_zero() {
        for n in 1..=20 {
            let data = zero_matrix(n);
            let evals = safe_eigenvalues_raw(n, &data).unwrap();
            assert_eq!(evals.len(), n);
            for (i, &e) in evals.iter().enumerate() {
                assert!(
                    e.abs() < 1e-9,
                    "n={n}: eigenvalue[{i}]={e} ≠ 0.0 (zero matrix must have all-zero spectrum)"
                );
            }
        }
    }

    // Property 8: Scalar matrix cI has n eigenvalues all == c.
    #[test]
    fn scalar_matrix_eigenvalues_are_all_c() {
        for n in 1..=10 {
            for &c in &[-7.0, -1.5, 0.0, 1.0, 3.14, 42.0, 100.0] {
                let data = scalar_matrix(n, c);
                let evals = safe_eigenvalues_raw(n, &data).unwrap();
                for &e in &evals {
                    assert!(
                        (e - c).abs() < 1e-9,
                        "n={n}, c={c}: got eigenvalue {e}"
                    );
                }
            }
        }
    }

    // Property 9: Diagonal matrix D has eigenvalues == sorted diagonal entries.
    #[test]
    fn diagonal_matrix_eigenvalues_equal_sorted_diagonal() {
        let v = forall::<RandomDiagonal, _>(30, |d: RandomDiagonal| {
            let data = diagonal_matrix(&d.diag);
            match safe_eigenvalues_raw(d.n, &data) {
                Ok(evals) => {
                    let mut expected = d.diag.clone();
                    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    if evals.len() != expected.len() {
                        return PropertyVerdict::Fail(Diagnostic::new(&format!(
                            "length mismatch: {} vs {}",
                            evals.len(),
                            expected.len()
                        )));
                    }
                    for (i, (&got, &want)) in evals.iter().zip(expected.iter()).enumerate() {
                        if (got - want).abs() >= 1e-9 {
                            return PropertyVerdict::Fail(Diagnostic::new(&format!(
                                "diagonal_matrix: eigenvalue[{i}]={got} ≠ sorted diagonal[{i}]={want}"
                            )));
                        }
                    }
                    PropertyVerdict::Pass
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 10: Trace preservation — sum of eigenvalues == trace(A).
    #[test]
    fn trace_equals_sum_of_eigenvalues() {
        let v = forall::<SymLaplacian, _>(30, |l: SymLaplacian| {
            match safe_eigenvalues(&l) {
                Ok(evals) => {
                    let sum: f64 = evals.iter().sum();
                    let tr = trace(&l.data, l.n);
                    if (sum - tr).abs() < 1e-9 * (1.0 + tr.abs()) {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(&format!(
                            "trace preservation: Σλ_i={sum} ≠ tr(A)={tr}"
                        )))
                    }
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 11: Frobenius norm squared == sum of squared eigenvalues.
    #[test]
    fn frobenius_norm_squared_equals_sum_of_squared_eigenvalues() {
        let v = forall::<SymLaplacian, _>(30, |l: SymLaplacian| {
            match safe_eigenvalues(&l) {
                Ok(evals) => {
                    let eval_sq_sum: f64 = evals.iter().map(|x| x * x).sum();
                    let frob_sq = frobenius_norm_squared(&l.data);
                    if (eval_sq_sum - frob_sq).abs() < 1e-8 * (1.0 + frob_sq.abs()) {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(&format!(
                            "Frobenius: Σλ_i²={eval_sq_sum} ≠ ||A||_F²={frob_sq}"
                        )))
                    }
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 12: Symmetric permutation preserves spectrum — P^T A P has
    // same eigenvalues as A for any permutation matrix P.
    #[test]
    fn symmetric_permutation_preserves_spectrum() {
        let v = forall::<PermutationSimilar, _>(30, |p: PermutationSimilar| {
            let base_evals = match safe_eigenvalues_raw(p.n, &p.base_data) {
                Ok(e) => e,
                Err(msg) => return PropertyVerdict::Fail(Diagnostic::new(&msg)),
            };
            let perm_evals = match safe_eigenvalues_raw(p.n, &p.permuted_data) {
                Ok(e) => e,
                Err(msg) => return PropertyVerdict::Fail(Diagnostic::new(&msg)),
            };
            for (i, (&a, &b)) in base_evals.iter().zip(perm_evals.iter()).enumerate() {
                if (a - b).abs() >= 1e-8 {
                    return PropertyVerdict::Fail(Diagnostic::new(&format!(
                        "permutation similarity broke eigenvalue[{i}]: base={a} vs permuted={b}"
                    )));
                }
            }
            PropertyVerdict::Pass
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 13: Constant shift shifts spectrum — eigvals(A + cI) ==
    // eigvals(A) + c.
    #[test]
    fn constant_shift_shifts_spectrum() {
        let v = forall::<ShiftedSymLaplacian, _>(30, |s: ShiftedSymLaplacian| {
            let base_evals = match safe_eigenvalues_raw(s.n, &s.base_data) {
                Ok(e) => e,
                Err(msg) => return PropertyVerdict::Fail(Diagnostic::new(&msg)),
            };
            let shifted_evals = match safe_eigenvalues_raw(s.n, &s.shifted_data) {
                Ok(e) => e,
                Err(msg) => return PropertyVerdict::Fail(Diagnostic::new(&msg)),
            };
            for (i, (&a, &b)) in base_evals.iter().zip(shifted_evals.iter()).enumerate() {
                let expected = a + s.shift;
                if (b - expected).abs() >= 1e-8 * (1.0 + expected.abs()) {
                    return PropertyVerdict::Fail(Diagnostic::new(&format!(
                        "shift invariance broke eigenvalue[{i}]: base={a} shift={} shifted={b} expected={expected}",
                        s.shift
                    )));
                }
            }
            PropertyVerdict::Pass
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 14: Larger Laplacian (n=6..20) preserves all base invariants.
    #[test]
    fn larger_laplacian_n_up_to_20_holds_base_invariants() {
        let v = forall::<LargeSymLaplacian, _>(15, |l: LargeSymLaplacian| {
            let evals = match catch_unwind(AssertUnwindSafe(|| eigenvalues(l.n, &l.data))) {
                Ok(e) => e,
                Err(_) => return PropertyVerdict::Fail(Diagnostic::new("panic on larger n")),
            };
            if evals.len() != l.n {
                return PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "cardinality: expected {} got {}",
                    l.n,
                    evals.len()
                )));
            }
            if !evals.iter().all(|x| x.is_finite()) {
                return PropertyVerdict::Fail(Diagnostic::new("non-finite eigenvalue at larger n"));
            }
            if !evals.windows(2).all(|w| w[0] <= w[1]) {
                return PropertyVerdict::Fail(Diagnostic::new("not ascending at larger n"));
            }
            if !evals.iter().all(|&x| x >= -1e-8) {
                return PropertyVerdict::Fail(Diagnostic::new("negative eigenvalue for PSD Laplacian at larger n"));
            }
            if evals[0].abs() >= 1e-8 {
                return PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "smallest eigenvalue ≠ 0 for Laplacian at larger n: {}",
                    evals[0]
                )));
            }
            PropertyVerdict::Pass
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 15: Gershgorin disk bound — max|λ| ≤ max row-sum.
    #[test]
    fn eigenvalues_bounded_by_gershgorin_row_sum() {
        let v = forall::<SymLaplacian, _>(30, |l: SymLaplacian| {
            let bound = gershgorin_bound(&l.data, l.n);
            match safe_eigenvalues(&l) {
                Ok(evals) => {
                    for (i, &e) in evals.iter().enumerate() {
                        if e.abs() > bound + 1e-8 {
                            return PropertyVerdict::Fail(Diagnostic::new(&format!(
                                "Gershgorin violated: |λ_{i}|={} > row-sum bound {bound}",
                                e.abs()
                            )));
                        }
                    }
                    PropertyVerdict::Pass
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property 16: Wilkinson matrix W_5 has deterministic eigenvalues.
    // W_5^+ per Wilkinson 1965: standard hard-case check that dsyev
    // handles nearly-repeated eigenvalues. The exact spectrum is
    // symmetric around 0; we verify eigenvalues sum to trace = 6.
    #[test]
    fn wilkinson_matrix_dsyev_convergence() {
        for n in &[3, 5, 7, 9] {
            let data = wilkinson_matrix(*n);
            let evals = safe_eigenvalues_raw(*n, &data).unwrap();
            assert_eq!(evals.len(), *n);
            assert!(evals.iter().all(|x| x.is_finite()), "W_{n} produced non-finite");
            assert!(evals.windows(2).all(|w| w[0] <= w[1]), "W_{n} not ascending");
            let sum: f64 = evals.iter().sum();
            let tr = trace(&data, *n);
            assert!(
                (sum - tr).abs() < 1e-9 * (1.0 + tr.abs()),
                "Wilkinson W_{n}: Σλ={sum} ≠ tr={tr}"
            );
        }
    }

    // Property 17: N=1 degenerate case — single eigenvalue == the entry.
    #[test]
    fn n_equals_1_eigenvalue_is_the_entry() {
        for &c in &[-100.0, -1.0, 0.0, 1e-9, 1.0, 42.0, 1e9] {
            let evals = safe_eigenvalues_raw(1, &[c]).unwrap();
            assert_eq!(evals.len(), 1);
            assert!((evals[0] - c).abs() < 1e-9 * (1.0 + c.abs()));
        }
    }

    // Property 18: N=2 closed-form cross-check — for a 2×2 symmetric
    // [[a,b],[b,c]], eigenvalues = (a+c)/2 ± √(((a-c)/2)² + b²).
    #[test]
    fn n_equals_2_closed_form_cross_check() {
        for &(a, b, c) in &[
            (1.0, 0.0, 1.0),
            (2.0, 1.0, 2.0),
            (5.0, -3.0, 1.0),
            (0.0, 1.0, 0.0),
            (10.0, 2.5, -3.0),
        ] {
            let data = vec![a, b, b, c];
            let evals = safe_eigenvalues_raw(2, &data).unwrap();
            let mid = (a + c) / 2.0;
            let disc = (((a - c) / 2.0).powi(2) + b * b).sqrt();
            let lo = mid - disc;
            let hi = mid + disc;
            assert!(
                (evals[0] - lo).abs() < 1e-9 * (1.0 + lo.abs()),
                "2x2 [{a},{b},{c}]: got {} expected {lo}",
                evals[0]
            );
            assert!(
                (evals[1] - hi).abs() < 1e-9 * (1.0 + hi.abs()),
                "2x2 [{a},{b},{c}]: got {} expected {hi}",
                evals[1]
            );
        }
    }

    // =============================================================
    // envelope() property tests — Aumann envelope (singular values)
    // full state-space coverage via LAPACK dgesvd FLANG chain.
    // =============================================================

    fn safe_envelope(m: usize, n: usize, matrix: &[f64]) -> Result<Vec<f64>, String> {
        catch_unwind(AssertUnwindSafe(|| envelope(m, n, matrix)))
            .map_err(|_| "matrix::envelope panicked (LAPACK dgesvd convergence failure)".to_string())
    }

    /// Row-major matrix product A * B where A is (m×k) and B is (k×n).
    /// Deliberately naive (test-scope only); production matmul goes
    /// through BLAS dgemm at phase_lock/matmul FLANG boundary.
    fn matmul_test(a: &[f64], b: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
        let mut c = vec![0.0f64; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut s = 0.0;
                for l in 0..k {
                    s += a[i * k + l] * b[l * n + j];
                }
                c[i * n + j] = s;
            }
        }
        c
    }

    /// Rank-1 outer product uv^T (u: m, v: n) as (m×n) row-major.
    fn outer_product(u: &[f64], v: &[f64]) -> Vec<f64> {
        let m = u.len();
        let n = v.len();
        let mut out = vec![0.0f64; m * n];
        for i in 0..m {
            for j in 0..n {
                out[i * n + j] = u[i] * v[j];
            }
        }
        out
    }

    fn vec_norm(v: &[f64]) -> f64 {
        v.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    // Property E1: identity matrix I_n has n singular values all == 1.
    #[test]
    fn envelope_identity_all_singular_values_are_one() {
        for n in 1..=10 {
            let svs = safe_envelope(n, n, &identity_matrix(n)).unwrap();
            assert_eq!(svs.len(), n);
            for (i, &s) in svs.iter().enumerate() {
                assert!((s - 1.0).abs() < 1e-9, "n={n} sv[{i}]={s} ≠ 1");
            }
        }
    }

    // Property E2: zero matrix 0_{m×n} has min(m,n) singular values all == 0.
    #[test]
    fn envelope_zero_all_singular_values_are_zero() {
        for &(m, n) in &[(1, 1), (2, 3), (3, 2), (5, 5), (4, 7), (10, 3)] {
            let data = vec![0.0f64; m * n];
            let svs = safe_envelope(m, n, &data).unwrap();
            assert_eq!(svs.len(), m.min(n));
            for (i, &s) in svs.iter().enumerate() {
                assert!(s.abs() < 1e-9, "m={m} n={n} sv[{i}]={s} ≠ 0");
            }
        }
    }

    // Property E3: scalar matrix cI has singular values all == |c|.
    #[test]
    fn envelope_scalar_matrix_all_singular_values_equal_abs_c() {
        for n in 1..=6 {
            for &c in &[-7.0, -1.0, 1.0, 3.14, 42.0] {
                let data = scalar_matrix(n, c);
                let svs = safe_envelope(n, n, &data).unwrap();
                for &s in &svs {
                    assert!(
                        (s - c.abs()).abs() < 1e-9,
                        "n={n} c={c}: got sv {s} expected |c|={}", c.abs()
                    );
                }
            }
        }
    }

    // Property E4: diagonal matrix has singular values == |diag| sorted descending.
    #[test]
    fn envelope_diagonal_singular_values_equal_sorted_abs_diagonal() {
        let v = forall::<RandomDiagonal, _>(30, |d: RandomDiagonal| {
            let data = diagonal_matrix(&d.diag);
            match safe_envelope(d.n, d.n, &data) {
                Ok(svs) => {
                    let mut expected: Vec<f64> = d.diag.iter().map(|x| x.abs()).collect();
                    expected.sort_by(|a, b| b.partial_cmp(a).unwrap());
                    for (i, (&got, &want)) in svs.iter().zip(expected.iter()).enumerate() {
                        if (got - want).abs() >= 1e-9 * (1.0 + want.abs()) {
                            return PropertyVerdict::Fail(Diagnostic::new(&format!(
                                "envelope diagonal: sv[{i}]={got} ≠ sorted|diag|[{i}]={want}"
                            )));
                        }
                    }
                    PropertyVerdict::Pass
                }
                Err(msg) => PropertyVerdict::Fail(Diagnostic::new(&msg)),
            }
        });
        assert!(matches!(v, PropertyVerdict::Pass), "{v:?}");
    }

    // Property E5: rectangular matrix returns exactly min(m,n) singular values.
    #[test]
    fn envelope_returns_min_m_n_singular_values() {
        for &(m, n) in &[(1, 5), (5, 1), (2, 7), (7, 2), (3, 3), (10, 4), (4, 10)] {
            let data: Vec<f64> = (0..m * n).map(|i| ((i * 7) % 11) as f64 - 5.0).collect();
            let svs = safe_envelope(m, n, &data).unwrap();
            assert_eq!(svs.len(), m.min(n), "m={m} n={n} sv count mismatch");
        }
    }

    // Property E6: descending order per LAPACK dgesvd convention.
    #[test]
    fn envelope_singular_values_in_descending_order() {
        for &(m, n) in &[(2, 2), (3, 3), (4, 5), (5, 4), (8, 8)] {
            let data: Vec<f64> = (0..m * n).map(|i| ((i * 13 + 7) % 19) as f64 - 9.0).collect();
            let svs = safe_envelope(m, n, &data).unwrap();
            assert!(svs.windows(2).all(|w| w[0] >= w[1]), "m={m} n={n} not descending");
        }
    }

    // Property E7: singular values are non-negative.
    #[test]
    fn envelope_singular_values_non_negative() {
        for &(m, n) in &[(3, 3), (4, 6), (6, 4), (8, 8), (10, 5)] {
            let data: Vec<f64> = (0..m * n).map(|i| ((i * 5) as f64).sin() * 3.0).collect();
            let svs = safe_envelope(m, n, &data).unwrap();
            for (i, &s) in svs.iter().enumerate() {
                assert!(s >= -1e-9, "m={m} n={n} sv[{i}]={s} < 0");
            }
        }
    }

    // Property E8: singular values are finite.
    #[test]
    fn envelope_singular_values_finite() {
        for &(m, n) in &[(2, 2), (3, 5), (5, 3), (8, 8), (12, 6)] {
            let data: Vec<f64> = (0..m * n).map(|i| (i as f64).cos() * 100.0).collect();
            let svs = safe_envelope(m, n, &data).unwrap();
            for &s in &svs {
                assert!(s.is_finite(), "m={m} n={n} produced non-finite sv {s}");
            }
        }
    }

    // Property E9: rank-1 outer product uv^T has exactly ONE nonzero
    // singular value equal to ||u|| * ||v||.
    #[test]
    fn envelope_rank_one_outer_product_has_single_nonzero_sv() {
        for &(m, n) in &[(2, 2), (3, 5), (5, 3), (4, 4), (7, 6)] {
            let u: Vec<f64> = (0..m).map(|i| (i + 1) as f64).collect();
            let v: Vec<f64> = (0..n).map(|i| ((i + 1) as f64).sqrt()).collect();
            let data = outer_product(&u, &v);
            let svs = safe_envelope(m, n, &data).unwrap();
            let expected_top = vec_norm(&u) * vec_norm(&v);
            assert!(
                (svs[0] - expected_top).abs() < 1e-9 * (1.0 + expected_top.abs()),
                "m={m} n={n} rank-1 top sv {} ≠ ||u||·||v|| {expected_top}", svs[0]
            );
            for (i, &s) in svs.iter().skip(1).enumerate() {
                assert!(s.abs() < 1e-9, "m={m} n={n} rank-1 sv[{}]={s} should be 0", i + 1);
            }
        }
    }

    // Property E10: Frobenius norm squared == sum of squared singular values.
    #[test]
    fn envelope_frobenius_norm_squared_equals_sum_of_squared_svs() {
        for &(m, n) in &[(2, 2), (3, 5), (5, 3), (4, 4), (6, 8)] {
            let data: Vec<f64> = (0..m * n).map(|i| ((i * 3 + 1) as f64).sin() * 5.0).collect();
            let svs = safe_envelope(m, n, &data).unwrap();
            let sv_sq_sum: f64 = svs.iter().map(|x| x * x).sum();
            let frob_sq = frobenius_norm_squared(&data);
            assert!(
                (sv_sq_sum - frob_sq).abs() < 1e-8 * (1.0 + frob_sq),
                "m={m} n={n} Σσ²={sv_sq_sum} ≠ ||A||_F²={frob_sq}"
            );
        }
    }

    // Property E11: A^T A eigenvalues == singular values squared (for m≥n).
    #[test]
    fn envelope_svs_squared_equal_ata_eigenvalues() {
        for &(m, n) in &[(3, 2), (5, 3), (6, 4), (8, 5)] {
            let a: Vec<f64> = (0..m * n).map(|i| ((i * 7 + 3) as f64).sin() * 2.0).collect();
            let svs = safe_envelope(m, n, &a).unwrap();

            // Compute A^T (n×m) row-major
            let mut at = vec![0.0f64; n * m];
            for i in 0..m {
                for j in 0..n {
                    at[j * m + i] = a[i * n + j];
                }
            }
            // A^T A is n×n symmetric
            let ata = matmul_test(&at, &a, n, m, n);
            let ata_eigs = safe_eigenvalues_raw(n, &ata).unwrap();

            // Compare: ata_eigs ascending; svs descending. Squared-svs
            // reversed should equal ascending eigenvalues.
            let mut sv_sq: Vec<f64> = svs.iter().map(|x| x * x).collect();
            sv_sq.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for (i, (&want, &got)) in sv_sq.iter().zip(ata_eigs.iter()).enumerate() {
                assert!(
                    (want - got).abs() < 1e-7 * (1.0 + want.abs()),
                    "m={m} n={n} σ²[{i}]={want} ≠ eigval(A^TA)[{i}]={got}"
                );
            }
        }
    }

    // =============================================================
    // phase_lock() property tests — Kuramoto phase-lock via LAPACK-
    // adjacent BLAS matmul + trig at spectral_phase_lock Fortran
    // altitude. Full state-space coverage per Alex 2026-07-20
    // "full state space coverage of this" directive.
    // =============================================================

    fn safe_phase_lock(
        phases: &[f64],
        omegas: &[f64],
        k: f64,
        steps: usize,
        dt: f64,
    ) -> Result<(Vec<f64>, f64), String> {
        catch_unwind(AssertUnwindSafe(|| phase_lock(phases, omegas, k, steps, dt)))
            .map_err(|_| "matrix::phase_lock panicked".to_string())
    }

    /// Kuramoto order parameter r = |(1/N) Σ e^(iθ)| for a set of
    /// phases. Reference impl for cross-check.
    fn kuramoto_order(phases: &[f64]) -> f64 {
        let n = phases.len() as f64;
        let cos_sum: f64 = phases.iter().map(|t| t.cos()).sum();
        let sin_sum: f64 = phases.iter().map(|t| t.sin()).sum();
        (cos_sum * cos_sum + sin_sum * sin_sum).sqrt() / n
    }

    // Property P1: zero coupling + zero omegas → phases unchanged.
    #[test]
    fn phase_lock_zero_coupling_zero_omegas_stays_at_initial() {
        let phases = vec![0.1, 0.7, 1.3, 2.5, 4.0];
        let omegas = vec![0.0; 5];
        let (out, _r) = safe_phase_lock(&phases, &omegas, 0.0, 100, 0.01).unwrap();
        for (i, (&p_in, &p_out)) in phases.iter().zip(out.iter()).enumerate() {
            assert!(
                (p_in - p_out).abs() < 1e-12,
                "phase[{i}]: initial {p_in} ≠ final {p_out} with zero dynamics"
            );
        }
    }

    // Property P2: zero coupling + uniform omegas → rigid rotation
    // (all phases advance by same amount).
    #[test]
    fn phase_lock_zero_coupling_uniform_omegas_rigid_rotation() {
        let phases = vec![0.0, 0.5, 1.0, 1.5];
        let omegas = vec![2.0; 4];
        let steps = 50usize;
        let dt = 0.01;
        let (out, _r) = safe_phase_lock(&phases, &omegas, 0.0, steps, dt).unwrap();
        let expected_advance = omegas[0] * (steps as f64) * dt;
        for (i, (&p_in, &p_out)) in phases.iter().zip(out.iter()).enumerate() {
            let advance = p_out - p_in;
            assert!(
                (advance - expected_advance).abs() < 1e-10,
                "phase[{i}]: advance {advance} ≠ expected {expected_advance}"
            );
        }
    }

    // Property P3: N=1 singleton — no coupling term applies (self-
    // interaction sin(θ-θ)=0); phase = θ_0 + steps·dt·ω.
    #[test]
    fn phase_lock_singleton_n_1_free_drift() {
        for &omega in &[-3.0, -0.5, 0.0, 0.5, 3.0] {
            let phases = vec![0.7];
            let omegas = vec![omega];
            let steps = 100usize;
            let dt = 0.01;
            let (out, _r) = safe_phase_lock(&phases, &omegas, 5.0, steps, dt).unwrap();
            let expected = 0.7 + omega * (steps as f64) * dt;
            assert!(
                (out[0] - expected).abs() < 1e-10,
                "ω={omega}: got {} expected {expected}", out[0]
            );
        }
    }

    // Property P4: order parameter r ALWAYS in [0, 1].
    #[test]
    fn phase_lock_order_parameter_bounded_in_zero_one() {
        for n in 1..=10 {
            let phases: Vec<f64> = (0..n).map(|i| (i as f64) * 0.31 + 0.17).collect();
            let omegas: Vec<f64> = (0..n).map(|i| ((i as f64) * 0.7 - 1.0).sin()).collect();
            for &k in &[-2.0, 0.0, 1.0, 5.0] {
                let (_, r) = safe_phase_lock(&phases, &omegas, k, 20, 0.01).unwrap();
                assert!(
                    (0.0..=1.0 + 1e-10).contains(&r),
                    "n={n} k={k}: r={r} out of [0,1]"
                );
            }
        }
    }

    // Property P5: identical phases (∀i θ_i = c) → r=1 exactly regardless
    // of coupling. Coupling term Σ sin(θ_j - θ_i) = Σ sin(0) = 0.
    #[test]
    fn phase_lock_identical_phases_zero_omegas_order_r_is_one() {
        for n in 1..=10 {
            for &phase in &[0.0, 1.5, -2.0, 3.14, 100.0] {
                let phases = vec![phase; n];
                let omegas = vec![0.0; n];
                let (out, r) = safe_phase_lock(&phases, &omegas, 10.0, 50, 0.01).unwrap();
                assert!(
                    (r - 1.0).abs() < 1e-10,
                    "n={n} phase={phase}: r={r} ≠ 1"
                );
                for &p in &out {
                    assert!((p - phase).abs() < 1e-10);
                }
            }
        }
    }

    // Property P6: evenly distributed phases on the unit circle → r=0.
    // For N=4 at {0, π/2, π, 3π/2}: sum of e^(iθ) = 0 exactly.
    #[test]
    fn phase_lock_evenly_distributed_phases_order_r_is_zero() {
        use std::f64::consts::PI;
        for n in &[3usize, 4, 5, 6, 8, 12] {
            let phases: Vec<f64> = (0..*n).map(|i| 2.0 * PI * (i as f64) / (*n as f64)).collect();
            let omegas = vec![0.0; *n];
            let (_, r) = safe_phase_lock(&phases, &omegas, 0.0, 0, 0.0).unwrap();
            assert!(
                r.abs() < 1e-10,
                "n={n} evenly distributed: r={r} ≠ 0"
            );
        }
    }

    // Property P7: strong coupling + identical omegas → converges to r→1.
    // Ensures Kuramoto integration actually synchronizes (dynamics working).
    #[test]
    fn phase_lock_strong_coupling_identical_omegas_converges_to_synchrony() {
        // Start with widely-spread phases + identical omegas + strong coupling.
        // After many timesteps, order parameter should be near 1.
        let phases: Vec<f64> = (0..5).map(|i| (i as f64) * 1.2).collect();
        let omegas = vec![1.0; 5];
        let k = 20.0; // strong
        let dt = 0.01;
        let steps = 2000usize;
        let (_, r) = safe_phase_lock(&phases, &omegas, k, steps, dt).unwrap();
        assert!(
            r > 0.95,
            "strong coupling did not synchronize: final r={r} (expected ≥ 0.95)"
        );
    }

    // Property P8: output phases + order parameter are always finite.
    #[test]
    fn phase_lock_output_finite() {
        for n in 1..=8 {
            let phases: Vec<f64> = (0..n).map(|i| (i as f64) * 0.5 + 0.1).collect();
            let omegas: Vec<f64> = (0..n).map(|i| ((i as f64) * 0.3).cos() * 2.0).collect();
            let (out, r) = safe_phase_lock(&phases, &omegas, 3.0, 100, 0.01).unwrap();
            assert!(r.is_finite(), "r not finite for n={n}");
            for (i, &p) in out.iter().enumerate() {
                assert!(p.is_finite(), "phase[{i}]={p} not finite for n={n}");
            }
        }
    }

    // Property P9: permutation invariance of order parameter. Permuting
    // (phases, omegas) pairwise yields the SAME r after integration.
    #[test]
    fn phase_lock_permutation_invariance_of_order_parameter() {
        let phases = vec![0.1, 0.7, 1.4, 2.2, 3.1];
        let omegas = vec![0.5, -0.3, 0.8, -0.5, 0.1];
        let k = 1.5;
        let steps = 100usize;
        let dt = 0.01;

        let (_, r_original) = safe_phase_lock(&phases, &omegas, k, steps, dt).unwrap();

        // Reverse permutation
        let mut phases_rev = phases.clone();
        phases_rev.reverse();
        let mut omegas_rev = omegas.clone();
        omegas_rev.reverse();
        let (_, r_reversed) = safe_phase_lock(&phases_rev, &omegas_rev, k, steps, dt).unwrap();

        assert!(
            (r_original - r_reversed).abs() < 1e-10,
            "permutation broke order parameter: original {r_original} vs reversed {r_reversed}"
        );
    }

    // Property P10: N=2 antipodal + zero coupling stays antipodal.
    // For N=2 with θ=(0, π), K=0, ω=0: sin(π-0) = sin(π) = 0 anyway,
    // so dynamics are trivial. Verifies the trivial fixed point.
    #[test]
    fn phase_lock_n_2_antipodal_zero_dynamics() {
        use std::f64::consts::PI;
        let phases = vec![0.0, PI];
        let omegas = vec![0.0, 0.0];
        let (out, r) = safe_phase_lock(&phases, &omegas, 5.0, 100, 0.01).unwrap();
        // Antipodal is an unstable fixed point of Kuramoto but starts
        // exactly at it — dynamics are zero because sin(π)=sin(-π)=0.
        assert!(
            (out[0]).abs() < 1e-10 && (out[1] - PI).abs() < 1e-10,
            "antipodal drifted: {} vs 0; {} vs π", out[0], out[1]
        );
        // Order parameter: |e^i0 + e^iπ|/2 = |1 - 1|/2 = 0.
        assert!(r.abs() < 1e-10, "antipodal r={r} ≠ 0");
    }

    // Property P11: order parameter matches reference at t=0 for arbitrary
    // phases (steps=0 is a pure order-parameter measurement).
    #[test]
    fn phase_lock_order_parameter_matches_reference_at_t_zero() {
        let cases = vec![
            vec![0.0, 0.5, 1.0, 1.5, 2.0],
            vec![-1.0, 1.0],
            vec![0.1, 0.15, 0.2, 0.25],
            vec![3.14, -3.14, 0.0, 1.57],
        ];
        for phases in cases {
            let omegas = vec![0.0; phases.len()];
            let (_, r) = safe_phase_lock(&phases, &omegas, 0.0, 0, 0.0).unwrap();
            let expected = kuramoto_order(&phases);
            assert!(
                (r - expected).abs() < 1e-12,
                "n={} phases={:?}: got r={r} expected {expected}", phases.len(), phases
            );
        }
    }

    // Property P12: identity of dynamics under rigid phase shift. Shifting
    // all initial phases by Δ shifts all final phases by Δ (since sin is
    // shift-invariant in the difference); order parameter r is invariant.
    #[test]
    fn phase_lock_rigid_phase_shift_invariance() {
        use std::f64::consts::PI;
        let base_phases = vec![0.1, 0.7, 1.3, 2.5];
        let omegas = vec![0.5, -0.2, 0.3, 0.7];
        let k = 2.0;
        let steps = 50usize;
        let dt = 0.01;

        let (out_base, r_base) = safe_phase_lock(&base_phases, &omegas, k, steps, dt).unwrap();

        for &shift in &[0.5, PI, -1.7, 2.0 * PI] {
            let shifted: Vec<f64> = base_phases.iter().map(|p| p + shift).collect();
            let (out_shifted, r_shifted) = safe_phase_lock(&shifted, &omegas, k, steps, dt).unwrap();
            assert!(
                (r_base - r_shifted).abs() < 1e-9,
                "shift {shift}: order parameter broke: {r_base} vs {r_shifted}"
            );
            for (i, (&b, &s)) in out_base.iter().zip(out_shifted.iter()).enumerate() {
                assert!(
                    (s - b - shift).abs() < 1e-9,
                    "shift {shift}: phase[{i}] not rigidly shifted: {b} vs {s}"
                );
            }
        }
    }

    // Property E12: 2×2 closed form. For A = [[a,b],[c,d]]:
    // σ² are roots of λ² - (a²+b²+c²+d²)λ + (ad-bc)² = 0.
    #[test]
    fn envelope_n_equals_2_closed_form_cross_check() {
        for &(a, b, c, d) in &[
            (1.0, 0.0, 0.0, 1.0),
            (2.0, 1.0, 0.0, 3.0),
            (1.0, 2.0, 3.0, 4.0),
            (5.0, -1.0, 2.0, 1.0),
        ] {
            let data = vec![a, b, c, d];
            let svs = safe_envelope(2, 2, &data).unwrap();

            let s = a * a + b * b + c * c + d * d;
            let p = (a * d - b * c).powi(2);
            let disc = ((s * s - 4.0 * p).max(0.0)).sqrt();
            let lam_hi = (s + disc) / 2.0;
            let lam_lo = (s - disc) / 2.0;
            let sv_hi = lam_hi.sqrt();
            let sv_lo = lam_lo.max(0.0).sqrt();

            assert!(
                (svs[0] - sv_hi).abs() < 1e-9 * (1.0 + sv_hi),
                "2x2 [{a},{b},{c},{d}]: sv[0]={} expected {sv_hi}", svs[0]
            );
            assert!(
                (svs[1] - sv_lo).abs() < 1e-9 * (1.0 + sv_lo),
                "2x2 [{a},{b},{c},{d}]: sv[1]={} expected {sv_lo}", svs[1]
            );
        }
    }
}
