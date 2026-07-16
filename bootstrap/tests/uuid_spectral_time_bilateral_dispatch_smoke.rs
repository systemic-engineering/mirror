//! @uuid/spectral/time bilateral dispatch empirical smoke — four
//! predicates dispatch through apply_h::act at Arc-1 evaluator FLOOR.
//!
//! Landed per /loop "ship it until unresolvable ambiguity" 2026-07-16
//! after Mara @uuid/spectral/time landing (c2bb1d2 species-decl +
//! c9c9480 math foundation + f782362 consumer cascades + 1b0650c R4
//! closure) + Seam Phase D SHIP-WITH-REED-INLINE ratification (task
//! #174) + REED-INLINE cascades 014d69a + f1767c0.
//!
//! Direct analog of Bridge β empirical dispatch pattern for @sheaf
//! bilaterals (bootstrap/tests/sheaf_bilateral_dispatch_smoke.rs).
//! Same-shape Arc-1 bilateral-predicate resolver: byte-check sentinel
//! in arg OID; return Pass/Fail. sbec +4 (identity_contract_preserved,
//! time_facet_admissible, dedup_ignores_time, uuid_spectral_time_
//! witnessing).
//!
//! Substrate-decl anchors (shards/uuid/spectral/time.mirror):
//!   identity_contract_preserved(a) — Pass iff a's identity field
//!     is a well-formed uuid_spectral. Sentinel:
//!     `identity=uuid-spectral-well-formed`.
//!   time_facet_admissible(a) — Pass iff a's time field is a well-
//!     formed @time/monotonic.instant. Sentinel:
//!     `time=monotonic-instant-well-formed`.
//!   dedup_ignores_time(a, b) — Pass iff (a, b) witness the storage-
//!     layer invariant that byte-equal identity dedups regardless of
//!     time-facet variation. Sentinel:
//!     `dedup=orthogonal-invariant-holds` in both args.
//!   uuid_spectral_time_witnessing(a, b) — composed bilateral;
//!     Pass iff all three sub-checks hold. Sentinel:
//!     `witnessing=composed-all-pass` in both args.
//!
//! Marker: [substrate-floor:@io-boundary] Bridge-β-pattern extension
//! at Rust FLOOR. Audit-cite: docs/audits/2026-07-15-seam-autopoietic-
//! loop-phase-d.md (55dbf20) + Seam Phase D task #174 for the
//! substrate this dispatches. Signed-off-by: Seam.

use mirror::apply_h::{act, Value, Verdict};

#[test]
fn identity_contract_preserved_passes_on_well_formed_uuid_spectral() {
    let a = Value {
        oid: "usti-cli:identity=uuid-spectral-well-formed".to_string(),
    };
    let verdict = act(
        "@uuid/spectral/time.identity_contract_preserved".to_string(),
        vec![a],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "identity_contract_preserved on well-formed-uuid-spectral sentinel should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn time_facet_admissible_passes_on_well_formed_monotonic_instant() {
    let a = Value {
        oid: "usti-cli:time=monotonic-instant-well-formed".to_string(),
    };
    let verdict = act(
        "@uuid/spectral/time.time_facet_admissible".to_string(),
        vec![a],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "time_facet_admissible on well-formed-monotonic-instant sentinel should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn dedup_ignores_time_passes_on_orthogonal_invariant_witness_pair() {
    // Per shards/uuid/spectral/time.mirror docblock: Pass iff (a, b) witness
    // that byte-equal identity dedups regardless of time-facet variation. In
    // the sentinel-only Arc-1 pattern, both args carry the invariant witness.
    let a = Value {
        oid: "usti-cli:dedup=orthogonal-invariant-holds,time=T1".to_string(),
    };
    let b = Value {
        oid: "usti-cli:dedup=orthogonal-invariant-holds,time=T2".to_string(),
    };
    let verdict = act(
        "@uuid/spectral/time.dedup_ignores_time".to_string(),
        vec![a, b],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "dedup_ignores_time on orthogonal-invariant witness pair should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn uuid_spectral_time_witnessing_passes_on_composed_all_pass_witness_pair() {
    // Composed bilateral: identity_contract_preserved + time_facet_admissible
    // + dedup_ignores_time all hold. Sentinel witnesses the composed state.
    let a = Value {
        oid: "usti-cli:witnessing=composed-all-pass".to_string(),
    };
    let b = Value {
        oid: "usti-cli:witnessing=composed-all-pass".to_string(),
    };
    let verdict = act(
        "@uuid/spectral/time.uuid_spectral_time_witnessing".to_string(),
        vec![a, b],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "uuid_spectral_time_witnessing on composed-all-pass witness pair should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn identity_contract_preserved_fails_cleanly_on_malformed_identity() {
    // Missing the well-formed sentinel: substrate-honest Fail
    // (not Pass, not Partial). Clean diagnostic.
    let a = Value {
        oid: "usti-cli:identity=malformed-not-uuid-spectral".to_string(),
    };
    let verdict = act(
        "@uuid/spectral/time.identity_contract_preserved".to_string(),
        vec![a],
    );
    assert!(
        matches!(verdict, Verdict::Fail(_)),
        "identity_contract_preserved on malformed identity MUST Fail; got {:?}",
        verdict
    );
}
