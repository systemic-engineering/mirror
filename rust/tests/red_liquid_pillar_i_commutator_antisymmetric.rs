//! RED-turned-GREEN — Pillar I base commutator property: antisymmetry.
//!
//! Mathematical claim (Mara `5d3040d` §2 + `3cd9a42` §3):
//!
//!     [A, B] = A·B - B·A = -(B·A - A·B) = -[B, A]
//!     |[A, B]| = |[B, A]|
//!
//! Ground-level algebraic property. Not a threshold, not a persistence
//! integral, not a colimit — just the definition.
//!
//! # From RED to GREEN
//!
//! At `028ccc2` this test failed to compile because `prismqueer::liquid`
//! did not exist and `TestBundle` was hidden inside `#[cfg(test)]`.
//! Under task #249 the following landed in `prism/prismqueer`:
//!
//! 1. `pub mod liquid` — `LiquidConnection` blanket-impl over
//!    `Transport`, `commutator` / `commutator_norm` functions, pillar
//!    verdict module. See `prism/prismqueer/src/liquid.rs`.
//! 2. `pub mod bundle::examples` — `TestBundle`, `LiquidTestBundle`,
//!    `TestFiber`, `TestConnection` lifted out of `#[cfg(test)]`.
//! 3. Ouroboros first layer in `prism/prismqueer/tests/liquid_ouroboros.rs`
//!    — 22 tests witnessing antisymmetry / self-annihilation /
//!    non-negativity / triangle / abelian vanishing / pillar verdicts.
//!
//! # Substrate-honest claim
//!
//! For `TestBundle` (`Cyclic<4>` gauge, state-dependent loss) the
//! commutator vanishes for *any* pair at *any* state — because Cyclic
//! groups are abelian AND the transport loss depends only on the state
//! (not on the bundle). Antisymmetry and self-annihilation therefore
//! hold trivially with both sides equal to `ScalarLoss::zero()`. This
//! is the substrate-honest first-layer witness: the machinery composes,
//! the mathematical properties are inherited from the `Metric` trait.
//!
//! Non-vanishing commutators (needed for Pillar II algedonic / Pillar
//! III viability) require `LiquidTestBundle` (bundle-dependent loss),
//! witnessed in the prismqueer-side ouroboros suite.

use prismqueer::bundle::examples::TestBundle;
use prismqueer::liquid::commutator_norm;
use prismqueer::Loss;

#[test]
fn commutator_norm_is_symmetric_over_test_bundle() {
    let a = TestBundle::default();
    let b = TestBundle::default();

    let norm_ab = commutator_norm(&a, &b);
    let norm_ba = commutator_norm(&b, &a);

    assert_eq!(
        norm_ab, norm_ba,
        "commutator norm MUST be symmetric per antisymmetric algebra: \
         |[A, B]| = |[B, A]|. Got norm_ab={norm_ab:?} norm_ba={norm_ba:?}",
    );
}

#[test]
fn commutator_norm_of_bundle_with_itself_is_zero() {
    // [A, A] = 0. Second-most-fundamental commutator identity after
    // antisymmetry. Any correct commutator MUST satisfy this.
    let a = TestBundle::default();

    let norm_self = commutator_norm(&a, &a);

    assert!(
        norm_self.is_zero(),
        "commutator [A, A] MUST equal Loss::zero(); got {norm_self:?}",
    );
}

#[test]
fn commutator_norm_vanishes_for_abelian_cyclic_gauge_pairs() {
    // Non-Default TestBundle instances — Cyclic<4> is abelian, so their
    // commutator must vanish. Witnesses that the mathematical claim in
    // the module docblock survives non-trivial strategy values.
    for i in 0..4u8 {
        for j in 0..4u8 {
            let a = TestBundle::with_strategy(i);
            let b = TestBundle::with_strategy(j);
            let m = commutator_norm(&a, &b);
            assert!(
                m.is_zero(),
                "abelian Cyclic<4> commutator MUST vanish for strategies \
                 i={i} j={j}; got {m:?}",
            );
        }
    }
}
