//! P4 first empirical crystal — Cholesky on 2×2 SPD via LAPACK `dpotrf`.
//!
//! ## Composition (four shards, one FFI discharge)
//!
//! The Cholesky arc composes four substrate-decl shards end-to-end plus this
//! Rust discharge module at the @io/silicon boundary:
//!
//!   1. `shards/epistemologic/math/cholesky.mirror` — the theorem shard
//!      (predicate: symmetric-positive-definite ⇒ LL^T decomposition;
//!      landed at `7c09993`). Declares `spd_matrix`,
//!      `lower_triangular_matrix`, `cholesky_decomposition`, `verify`,
//!      `discharge_via` typed carriers.
//!   2. `shards/glue/math_silicon.mirror` — the @glue species Mesland
//!      correspondence carrying `morphism_kind:
//!      @arxiv/math/cholesky_decomposition.lapack_realization` (landed at
//!      `5edd3e9`). Names the composition mechanism.
//!   3. `shards/reality/algebra/silicon.mirror` — the silicon species
//!      naming the target altitude the discharge lands at (Mara
//!      `9ca6723`).
//!   4. LAPACK `dpotrf_` FFI binding — this module. The discharge boundary
//!      the substrate-decl surface has been forward-promising.
//!
//! ## Witness
//!
//! For the canonical 2×2 SPD matrix
//!
//! ```text
//! A = [[4.0, 12.0], [12.0, 37.0]]
//! ```
//!
//! Cholesky's theorem yields the unique lower-triangular
//!
//! ```text
//! L = [[2.0, 0.0], [6.0, 1.0]]
//! ```
//!
//! with strictly positive diagonal. `L·L^T = [[4, 12], [12, 36+1]] = A`.
//! Byte-equal on IEEE-754 f64: `4 = 2²` and `37 = 6²+1²` are exactly
//! representable; no rounding at this rank and magnitude.
//!
//! ## Why raw `extern "C"` (not `prismqueer::ffi::dpotrf`)
//!
//! `prismqueer` v0.1.1 with `features = ["bundle", "lapack"]` wraps its
//! `native/prism.f90` shim exposing `spectral_eigenvalues` (`dsyev`),
//! `spectral_singular_values` (`dgesvd`), `spectral_svd`, and the four
//! prism-projection routines. It does NOT expose `dpotrf`.
//!
//! However, `prismqueer`'s `build.rs` under the `lapack` feature emits
//! `cargo:rustc-link-lib=framework=Accelerate` on darwin (and
//! `-l lapack -l blas` on Linux). The Accelerate framework provides the
//! entire LAPACK symbol table, including `dpotrf_`; the linker resolution
//! is therefore already in place. Naming another symbol from the same
//! library via `extern "C"` is the substrate-pull-honest path — no new
//! link, no shim, no dependency, just the FFI binding this module IS.
//!
//! This mirrors the pattern in `sheaf_laplacian.rs` where LAPACK is
//! reached via `prismqueer::ffi::eigenvalues`; the only difference is
//! that `prismqueer` already had a Fortran shim for `dsyev` but does not
//! have one for `dpotrf`. Adding another Fortran shim in `prismqueer`
//! would be the "cleaner" architectural move but crosses a crate
//! boundary; declaring the extern here keeps the Cholesky discharge
//! self-contained at the boundary where it lands.
//!
//! ## Column-major marshalling
//!
//! LAPACK is Fortran; matrices are laid out in COLUMN-major order. The
//! Rust `[[f64; 2]; 2]` type is naturally row-major (`a[i][j]` is `a[i]`
//! followed by `[j]`). The marshal is deterministic for 2×2:
//!
//! | col-major index | Fortran (col, row) | Rust `a[row][col]` |
//! |-----------------|--------------------|--------------------|
//! | `buffer[0]`     | `(0, 0)`           | `a[0][0]`          |
//! | `buffer[1]`     | `(0, 1)`           | `a[1][0]`          |
//! | `buffer[2]`     | `(1, 0)`           | `a[0][1]`          |
//! | `buffer[3]`     | `(1, 1)`           | `a[1][1]`          |
//!
//! With `UPLO = 'L'` LAPACK reads only the lower triangle (indices 0, 1,
//! 3) and skips index 2. On successful return the same three entries
//! hold the L values in column-major:
//!
//! ```text
//! L[0][0] = buffer[0]
//! L[1][0] = buffer[1]
//! L[1][1] = buffer[3]
//! L[0][1] = 0.0    // upper triangular; LAPACK leaves buffer[2]
//!                  // untouched at its input value, so we explicitly
//!                  // zero the output's upper triangle.
//! ```
//!
//! ## Info parameter
//!
//! `dpotrf` reports the outcome via its `info` argument:
//!
//! - `info == 0` — success; the L factor is in the lower triangle.
//! - `info > 0` — the leading minor of order `info` is not positive
//!   definite; the factorisation could not be completed. Returned as
//!   `None` — the substrate's imperfect-verdict failure branch.
//! - `info < 0` — the `-info`-th argument was invalid (wrapper bug).
//!   Returned as `None` conservatively; in practice this cannot fire
//!   for the fixed 2×2 signature.
//!
//! ## Substrate decisions
//!
//! - `[[architecture-flang-mirror-numerical-split]]` — LAPACK is the
//!   flang-altitude realisation the @glue/math_silicon correspondence
//!   discharges into. This module is a boundary; the substrate-decl
//!   home is the shard family.
//! - `[[feedback-no-bare-types]] — the `[[f64; 2]; 2]` signature at the
//!   FFI boundary is a test contract; the typed carriers (`spd_matrix`,
//!   `lower_triangular_matrix`, `cholesky_decomposition`) live at
//!   substrate altitude. The boundary is the marshal.
//! - `[[architecture-fragmentation-is-the-rust-substrate]]` — this
//!   module lives at the Rust substrate because FFI marshalling IS the
//!   Rust substrate at this altitude.

#![allow(dead_code)]

use std::os::raw::{c_char, c_int};

// ---------------------------------------------------------------------------
// Raw LAPACK extern — dpotrf_ (Cholesky factorisation of a real symmetric
// positive-definite matrix).
//
// Fortran bind(C) convention: the symbol name is `dpotrf_` (trailing
// underscore). LAPACK/BLAS uses this convention on macOS Accelerate,
// OpenBLAS, MKL, and gfortran-linked LAPACK.
//
// Reference: LAPACK Users' Guide (Anderson et al. 1999), §5.4.5.
// ---------------------------------------------------------------------------

extern "C" {
    /// `SUBROUTINE DPOTRF(UPLO, N, A, LDA, INFO)`
    ///
    /// Computes the Cholesky factorisation `A = L·L^T` (if `UPLO = 'L'`)
    /// or `A = U^T·U` (if `UPLO = 'U'`) of a real symmetric positive-
    /// definite matrix `A`.
    ///
    /// - `uplo` — `'L'` for the lower triangle, `'U'` for the upper.
    /// - `n` — order of the matrix.
    /// - `a` — column-major matrix buffer of size `lda × n`. On successful
    ///   exit the factor `L` (or `U`) overwrites the requested triangle.
    /// - `lda` — leading dimension of `a`; for a packed square matrix this
    ///   equals `n`.
    /// - `info` — 0 on success; `> 0` if the leading minor of that order
    ///   is not positive definite; `< 0` if the corresponding argument
    ///   had an illegal value.
    fn dpotrf_(
        uplo: *const c_char,
        n: *const c_int,
        a: *mut f64,
        lda: *const c_int,
        info: *mut c_int,
    );
}

// ---------------------------------------------------------------------------
// factor — the Rust-facing discharge boundary.
// ---------------------------------------------------------------------------

/// Cholesky decomposition `A = L·L^T` on 2×2 symmetric-positive-definite
/// `f64` matrices via LAPACK `dpotrf`. Returns `None` if `A` is not
/// positive-definite (the LAPACK `info` parameter is `> 0`).
///
/// Byte-equal on IEEE-754 f64 for exactly-representable inputs. The
/// substrate's canonical 2×2 SPD witness is:
///
/// ```text
/// A = [[4.0, 12.0], [12.0, 37.0]]  →  L = [[2.0, 0.0], [6.0, 1.0]]
/// ```
///
/// See the module documentation for the column-major marshal contract
/// and the LAPACK `info` parameter conventions.
pub fn factor(a: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    // Column-major buffer per the marshal in the module docs.
    //   buffer[0] = a[0][0]  (col 0, row 0)
    //   buffer[1] = a[1][0]  (col 0, row 1)
    //   buffer[2] = a[0][1]  (col 1, row 0)   — upper triangular; LAPACK
    //                                            skips this with UPLO='L'
    //   buffer[3] = a[1][1]  (col 1, row 1)
    let mut buffer: [f64; 4] = [a[0][0], a[1][0], a[0][1], a[1][1]];

    // UPLO = 'L' — read/write the LOWER triangle. The b'L' byte is the
    // Fortran character literal LAPACK's dpotrf pattern-matches on.
    let uplo: c_char = b'L' as c_char;
    let n: c_int = 2;
    let lda: c_int = 2;
    let mut info: c_int = 0;

    // SAFETY: all pointers refer to stack-owned values whose lifetimes
    // enclose the call. `buffer` is a `[f64; 4]` — 4 × 8 = 32 bytes, the
    // exact leading-dimension × n column-major layout LAPACK expects for
    // a 2×2 matrix. `dpotrf_` is a pure numerical routine; it does not
    // capture pointers past the call. The Accelerate framework provides
    // the symbol on darwin per `prismqueer`'s `build.rs` (`lapack`
    // feature).
    unsafe {
        dpotrf_(
            &uplo as *const c_char,
            &n as *const c_int,
            buffer.as_mut_ptr(),
            &lda as *const c_int,
            &mut info as *mut c_int,
        );
    }

    // info == 0  → success; the L factor is in the lower triangle of
    //              buffer (indices 0, 1, 3 in column-major).
    // info >  0  → the leading minor of order `info` is not positive
    //              definite; factorisation aborted. Return None per the
    //              substrate's imperfect-verdict failure branch.
    // info <  0  → the `-info`-th argument was invalid; wrapper bug.
    //              Return None conservatively.
    if info != 0 {
        return None;
    }

    // Materialise the row-major `[[f64; 2]; 2]` L from the column-major
    // lower-triangle values in `buffer`. The upper-triangular entry is
    // zeroed explicitly — LAPACK leaves `buffer[2]` at its input value
    // when UPLO='L', so we must NOT copy it through to the output.
    let l = [
        [buffer[0], 0.0],       // L[0][0] = buffer[0]; L[0][1] = 0.0
        [buffer[1], buffer[3]], // L[1][0] = buffer[1]; L[1][1] = buffer[3]
    ];

    Some(l)
}

// ---------------------------------------------------------------------------
// Tests (module-internal — the integration test at `tests/cholesky_
// first_crystal.rs` covers the byte-equal witness at the public boundary).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_returns_some_for_canonical_spd_witness() {
        let a: [[f64; 2]; 2] = [[4.0, 12.0], [12.0, 37.0]];
        let l = factor(&a).expect("A is SPD; dpotrf must succeed");
        assert_eq!(l[0][0], 2.0);
        assert_eq!(l[0][1], 0.0);
        assert_eq!(l[1][0], 6.0);
        assert_eq!(l[1][1], 1.0);
    }

    #[test]
    fn factor_returns_none_for_negative_definite() {
        let a: [[f64; 2]; 2] = [[-1.0, 0.0], [0.0, -1.0]];
        assert!(factor(&a).is_none());
    }
}
