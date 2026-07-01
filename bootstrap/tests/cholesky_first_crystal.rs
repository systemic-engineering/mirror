//! P4 first empirical crystal — Cholesky on 2×2 SPD via LAPACK dpotrf.
//!
//! The Cholesky arc composes four shards end-to-end (see
//! `shards/epistemologic/math/cholesky.mirror` §Composition):
//!
//!   1. `shards/epistemologic/math/cholesky.mirror` — the theorem
//!      shard (predicate: symmetric-positive-definite ⇒ LL^T
//!      decomposition; landed at `7c09993`).
//!   2. `shards/glue/math_silicon.mirror` — the @glue species
//!      (Mesland correspondence carrying morphism_kind
//!      `@arxiv/math/cholesky_decomposition.lapack_realization`;
//!      landed at `5edd3e9`).
//!   3. `shards/reality/algebra/silicon.mirror` — the silicon
//!      species (target altitude the discharge lands at;
//!      Mara `9ca6723`).
//!   4. LAPACK dpotrf FFI binding — Reed's discharge boundary
//!      (this RED asserts the empirical byte-equal witness before
//!      the module exists; GREEN wires prismqueer's `lapack` feature
//!      or direct extern "C" against `dpotrf_`).
//!
//! Witness: for the canonical 2×2 SPD matrix
//!
//!   A = [[4.0, 12.0], [12.0, 37.0]]
//!
//! Cholesky's theorem yields the unique lower-triangular
//!
//!   L = [[2.0, 0.0], [6.0, 1.0]]
//!
//! with strictly positive diagonal. Verify: L·L^T =
//! [[4, 12], [12, 36+1]] = [[4, 12], [12, 37]] = A. Byte-equal on
//! IEEE-754 f64: 4=2² and 37=6²+1² are both exactly representable;
//! no rounding at this rank + magnitude.
//!
//! RED phase discipline: `mirror::cholesky::factor` does not exist.
//! This test file will fail to compile. That IS the RED. GREEN lands
//! `bootstrap/src/cholesky.rs` with the LAPACK discharge, adds
//! `pub mod cholesky;` to `bootstrap/src/lib.rs`, and the test
//! transitions to runtime-passing.

#[test]
#[allow(clippy::approx_constant)]
fn cholesky_factor_2x2_spd_byte_equal_witness() {
    // Input: 2×2 SPD matrix A per Cholesky's theorem hypothesis.
    let a: [[f64; 2]; 2] = [[4.0, 12.0], [12.0, 37.0]];

    // Expected: unique lower-triangular L such that A = L·L^T with
    // strictly positive diagonal per Cholesky 1910 / Benoît 1924.
    let expected_l: [[f64; 2]; 2] = [[2.0, 0.0], [6.0, 1.0]];

    // The empirical discharge — LAPACK dpotrf via the
    // @glue/math_silicon Mesland correspondence. Reed's boundary
    // lift; the theorem's realisation at silicon altitude.
    let l = mirror::cholesky::factor(&a).expect("A is SPD; factor must succeed");

    assert_eq!(
        l, expected_l,
        "Cholesky L must be byte-equal to [[2.0, 0.0], [6.0, 1.0]] for A = [[4, 12], [12, 37]]"
    );
}

#[test]
fn cholesky_factor_reconstructs_original_matrix() {
    // The theorem's algebraic identity: L·L^T = A.
    let a: [[f64; 2]; 2] = [[4.0, 12.0], [12.0, 37.0]];
    let l = mirror::cholesky::factor(&a).expect("A is SPD");

    // L · L^T reconstruction — indexed byte-equal on the SPD cone.
    let mut reconstructed = [[0.0f64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                // L[i][k] · L^T[k][j] = L[i][k] · L[j][k].
                reconstructed[i][j] += l[i][k] * l[j][k];
            }
        }
    }

    assert_eq!(
        reconstructed, a,
        "L·L^T must reconstruct A byte-equal on IEEE-754 f64"
    );
}

#[test]
fn cholesky_factor_rejects_non_spd_matrix() {
    // Cholesky's theorem quantifies over symmetric POSITIVE-definite
    // matrices. A negative-definite matrix must NOT decompose;
    // LAPACK's dpotrf reports the leading minor at which positive-
    // definiteness broke (the substrate carries this as `None` in
    // the imperfect wrapper's failure branch).
    let not_spd: [[f64; 2]; 2] = [[-1.0, 0.0], [0.0, -1.0]];

    let result = mirror::cholesky::factor(&not_spd);

    assert!(
        result.is_none(),
        "factor must reject non-positive-definite A; got {:?}",
        result
    );
}
