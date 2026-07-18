//! RED — Pillar I base commutator property: antisymmetry.
//!
//! Mathematical claim (Mara `5d3040d` §2 + `3cd9a42` §3):
//!
//!     [A, B] = A·B - B·A = -(B·A - A·B) = -[B, A]
//!     |[A, B]| = |[B, A]|
//!
//! This is the base algebraic property any correct commutator
//! implementation MUST satisfy. Ground-level. Not a threshold, not a
//! persistence integral, not a colimit — just the definition.
//!
//! This test FAILS today at multiple layers:
//!
//! 1. `mirror/rust/Cargo.toml` had zero deps; test lands after adding
//!    prismqueer + terni as [dev-dependencies] (done in this commit).
//! 2. `prismqueer::liquid` module doesn't exist yet. Compilation error:
//!    `could not find liquid in prismqueer`. This IS the RED.
//! 3. `Connection` trait has ONLY `fn connection(&self) -> &Self::Optic`
//!    per `prismqueer/src/bundle.rs:110-119` verbatim. Mara `3cd9a42`
//!    §7.1 assumes `C::compose_optics` + `C::holonomy_metric_distance`
//!    which don't exist yet. Adding them (via a new `LiquidConnection`
//!    supertrait or extending `Connection` / `Transport`) is scope of
//!    `prismqueer::liquid` module implementation.
//!
//! Per Mara `3cd9a42` §10 OQ3: Pillar I lands FIRST because it's
//! Rice-safe byte-visible (Pass/Fail only; no threshold discipline).
//!
//! When this test starts passing:
//! - prismqueer::liquid exists
//! - Connection surface carries commutator computation
//! - The base algebra property holds
//! - Foundation for Pillars II/III/IV is byte-verified

use prismqueer::liquid::commutator_norm;
use prismqueer::bundle::{Bundle, Cyclic, IdentityPrism, StableFiber, TestBundle};

#[test]
fn commutator_norm_is_symmetric_over_test_bundle() {
    // Two arbitrary Bundle instances on the same fiber. TestBundle
    // uses IdentityPrism<[f64; 4]> per prismqueer/src/bundle.rs:432-438.
    let a = TestBundle::new();
    let b = TestBundle::new();

    let norm_ab = commutator_norm::<TestBundle>(&a, &b);
    let norm_ba = commutator_norm::<TestBundle>(&b, &a);

    assert_eq!(
        norm_ab, norm_ba,
        "commutator norm MUST be symmetric per antisymmetric algebra: \
         |[A, B]| = |[B, A]|. Got norm_ab={} norm_ba={}",
        norm_ab, norm_ba,
    );
}

#[test]
fn commutator_norm_of_bundle_with_itself_is_zero() {
    // The Lie bracket of any element with itself is zero: [A, A] = 0.
    // This is the second-most-fundamental commutator identity after
    // antisymmetry. Any correct commutator MUST satisfy this.
    let a = TestBundle::new();

    let norm_self = commutator_norm::<TestBundle>(&a, &a);

    assert_eq!(
        norm_self, 0.0,
        "commutator [A, A] MUST equal 0 (norm 0.0); got {}",
        norm_self,
    );
}
