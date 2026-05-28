//! The smallest possible flang-FFI proof: a Fortran function, compiled by
//! flang, linked into the bootstrap, called from Rust, returning a correct
//! numerical result.
//!
//! This de-risks the entire numerical-substrate direction (eigendecomposition,
//! d_s spectral dimension, cosmos-mirror) by proving the integration pathway
//! works on a trivial function — so integration bugs surface now, cheaply.
//!
//! Per docs/specs/numerical-substrate-via-fortran.md. The Fortran source lives
//! at bootstrap/fortran/dot.f90; build.rs compiles + archives it and emits the
//! cargo link directives.

extern "C" {
    fn dot5(a: *const f64, b: *const f64) -> f64;
}

#[test]
fn fortran_dot5_links_and_computes() {
    let a = [1.0f64, 2.0, 3.0, 4.0, 5.0];
    let b = [1.0f64, 1.0, 1.0, 1.0, 1.0];
    let r = unsafe { dot5(a.as_ptr(), b.as_ptr()) };
    assert_eq!(r, 15.0);
}
