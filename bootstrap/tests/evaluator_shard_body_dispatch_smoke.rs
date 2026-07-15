//! Arc-1 Tick 1.2 RED — smoke test for 7-combinator shard-body dispatch.
//!
//! Dispatches the landed `@subject/visibility/public.consent_scope_universal`
//! bilateral predicate action through `apply_h::act`. In RED phase this
//! panics at `todo!()` inside the combinator body — that IS the RED state.
//! Tick 1.3 fills the combinator implementations reusing spectral.rs's
//! `Combinator`, `Fold5`, `compose_a`, `apply_h`, `eigen_d` primitives;
//! at that point this test discharges Pass without modification.
//! Tick 1.4 then wires `mirror beam act` as the CLI verb the same
//! dispatch surface answers.
//!
//! Test target rationale:
//!   `@subject/visibility/public.consent_scope_universal` (landed at
//!   `shards/subject/visibility/public.mirror`, `\`-obligation-blocked
//!   per [[feedback-craft-not-deliver]]) is the smallest tractable
//!   bilateral predicate on the surface — single input (`visibility_scope`),
//!   single output (`verdict`), no @io composition, no metalogue write.
//!   The dispatch through `act` exercises exactly one recursion into the
//!   surface + the shard-body resolver's byte-equality check on the
//!   consent_scope [everyone]-sentinel. Minimal surface = smallest
//!   RED→GREEN diff at Tick 1.3.
//!
//! Alex directive verbatim (2026-07-15, in-transcript):
//!   "Sounds perfect. Let's get that shipped and then effectively call
//!    `mirror roomba --commit`. I want the compiler itself to make the
//!    commit, you know? The whole end2end flow as an empirical CLI call
//!    proof."
//!
//! This test is Landing 1 of the 5-tick cascade toward that empirical
//! proof. See `docs/loop/CURRENT.md` @kintsugi/ouroboros arc for the
//! full ladder.

use mirror::apply_h::{self, Value, Verdict};

/// The landed shard-action ref under test. Verified landed by grep of
/// `shards/subject/visibility/public.mirror` action-decl block.
const CONSENT_SCOPE_UNIVERSAL: &str = "@subject/visibility/public.consent_scope_universal";

#[test]
fn evaluator_dispatches_landed_shard_action() {
    // Construct the visibility_scope argument as a substrate-ref Value.
    // The @subject/visibility/public species-decl's public terminal-state
    // scope has consent_scope = [everyone] per line "consent_scope = [everyone]
    // (open-set sentinel)". Tick 1.3's `act` resolver will content-address
    // this ref against the landed crystal store; RED phase never gets past
    // the first combinator body.
    let visibility_scope_ref = Value {
        oid: "public-scope-fixture:consent_scope=[everyone]".to_string(),
    };

    // Dispatch through the 7-combinator surface. RED-phase expectation:
    // `act` panics at `todo!("Arc-1 Tick 1.3: resolve shard_action_ref +
    // recurse via apply_h")`. GREEN-phase expectation (Tick 1.3): returns
    // `Verdict::Pass` because the fixture consent_scope IS the [everyone]
    // open-set sentinel.
    let verdict = apply_h::act(
        CONSENT_SCOPE_UNIVERSAL.to_string(),
        vec![visibility_scope_ref],
    );

    // Tick 1.3 GREEN: `act` now returns a real Verdict rather than
    // panicking. The bilateral-predicate resolver in `apply_h::act`
    // recognizes the `consent_scope_universal` shard_action_ref,
    // byte-checks the arg OID against the `[everyone]` sentinel per
    // `shards/subject/visibility/public.mirror` docblock lines 143–147,
    // and discharges Pass. sbec empirically lifts from 0 to > 0 the
    // instant this assertion holds.
    assert!(
        matches!(verdict, Verdict::Pass),
        "consent_scope_universal on [everyone]-sentinel should Pass; got {:?}",
        verdict
    );
}
