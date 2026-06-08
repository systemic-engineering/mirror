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
//!
//! Per Taut #286 Win 2: once bootstrap is split into bin+lib, the build
//! script's `cargo:rustc-link-lib=static=dot` directive lives on the lib
//! crate. Integration tests that don't reference the lib don't get the
//! transitive link directive, so the linker drops `_dot5`. Importing
//! `mirror` here keeps the lib's rlib (and its `#[link]` attribute on
//! `libdot.a`) in the link graph for this test's link step.

use mirror as _;

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
