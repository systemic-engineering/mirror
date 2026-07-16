//! Tick 3 empirical dispatch — @kintsugi/roomba bump/vacuum/gc_mark_
//! terminal + @mirror/store.gc_reachability_closure_second_witness
//! bilaterals dispatch through apply_h::act at Arc-1 evaluator FLOOR.
//!
//! Landed per /loop "ship it until first rust roomba" 2026-07-16 as
//! Tick 3 substrate movement. Composes over Mara @roomba bump+vacuum+gc
//! landing (d457501 + 17697e6 + a19fea2) ratified by Seam Phase D task
//! #180 with SHIP-WITH-REED-INLINE. Direct analog of Bridge β (@sheaf)
//! + @uuid/spectral/time bilateral dispatch patterns. sbec +4.
//!
//! Substrate-decl anchors:
//!   bump_witnessing(dispatch) — sentinel
//!     `bump=witnessing-all-conjuncts-pass` (composed:
//!     fracture_species_admissible ∧ morphism_selected_from_fracture_
//!     algebra ∧ metalogue_turn_composable) per shards/kintsugi/
//!     roomba.mirror
//!   vacuum_admissible(mark) — sentinel
//!     `vacuum=admissible-all-conjuncts-pass` (composed:
//!     fragment_is_dangling ∧ mark_age_monotone ∧ dangling_consistency_
//!     second_witness) per same shard
//!   gc_mark_terminal(mark) — sentinel `gc_mark=horizon-in-future`
//!     (prune_horizon > marked_at strictly; grace-period safety per
//!     git-gc(1) rationale per Kagi citation) per same shard
//!   gc_reachability_closure_second_witness(refs, dangling) — sentinel
//!     `gc=reachability-second-witness-holds` (walk-vs-impacted_by
//!     consistency per math §2.5 dangling-consistency) per shards/
//!     mirror/store.mirror
//!
//! Marker: [substrate-floor:@io-boundary] Bridge-β-pattern extension
//! at Rust FLOOR. Audit-cite: Seam Phase D task #180 SHIP-WITH-REED-
//! INLINE. Signed-off-by: Seam.

use mirror::apply_h::{act, Value, Verdict};

#[test]
fn bump_witnessing_passes_on_all_conjuncts_witness() {
    let dispatch = Value {
        oid: "bump-cli:bump=witnessing-all-conjuncts-pass".to_string(),
    };
    let verdict = act(
        "@kintsugi/roomba.bump_witnessing".to_string(),
        vec![dispatch],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "bump_witnessing on all-conjuncts-pass sentinel should Pass; got {:?}",
        verdict
    );
}

#[test]
fn vacuum_admissible_passes_on_all_conjuncts_witness() {
    let mark = Value {
        oid: "vacuum-cli:vacuum=admissible-all-conjuncts-pass".to_string(),
    };
    let verdict = act(
        "@kintsugi/roomba.vacuum_admissible".to_string(),
        vec![mark],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "vacuum_admissible on all-conjuncts-pass sentinel should Pass; got {:?}",
        verdict
    );
}

#[test]
fn gc_mark_terminal_passes_on_horizon_in_future() {
    let mark = Value {
        oid: "gc-mark-cli:gc_mark=horizon-in-future".to_string(),
    };
    let verdict = act(
        "@kintsugi/roomba.gc_mark_terminal".to_string(),
        vec![mark],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "gc_mark_terminal on horizon-in-future sentinel should Pass; got {:?}",
        verdict
    );
}

#[test]
fn gc_reachability_closure_second_witness_passes_on_consistency_witness_pair() {
    // Per shards/mirror/store.mirror gc_reachability_closure_second_witness
    // docblock: refs + dangling both carry the walk-vs-impacted_by
    // consistency second-witness sentinel per math §2.5.
    let refs = Value {
        oid: "refs-cli:gc=reachability-second-witness-holds".to_string(),
    };
    let dangling = Value {
        oid: "dangling-cli:gc=reachability-second-witness-holds".to_string(),
    };
    let verdict = act(
        "@mirror/store.gc_reachability_closure_second_witness".to_string(),
        vec![refs, dangling],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "gc_reachability_closure_second_witness on consistency-witness pair should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn bump_witnessing_fails_cleanly_on_malformed_dispatch() {
    // Missing all-conjuncts-pass sentinel: substrate-honest Fail
    // (not Pass, not Partial). Clean diagnostic.
    let dispatch = Value {
        oid: "bump-cli:bump=malformed-not-all-conjuncts".to_string(),
    };
    let verdict = act(
        "@kintsugi/roomba.bump_witnessing".to_string(),
        vec![dispatch],
    );
    assert!(
        matches!(verdict, Verdict::Fail(_)),
        "bump_witnessing on malformed dispatch MUST Fail; got {:?}",
        verdict
    );
}
