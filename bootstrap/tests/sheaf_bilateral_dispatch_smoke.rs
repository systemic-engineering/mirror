//! Bridge β empirical dispatch RED — @subject/visibility/sheaf
//! bilateral predicates dispatch through apply_h::act.
//!
//! Per Seam Phase D adjudication (task #164 + #168 + #170), Bridge β's
//! target shifted from autopoietic-loop parser extension → @subject/
//! visibility/sheaf.restrict/section_at + bilateral predicates
//! empirical dispatch through apply_h::act. Rationale (Seam): unblocks
//! @gestalt.project (which composes over @subject/visibility/sheaf.
//! restrict for reader-ACL restriction per canonical spec
//! docs/specs/gestalt-as-song-unfolding.md §5.4).
//!
//! This landing dispatches the two bilateral predicates (restriction_
//! admissible + section_admissible) per the Arc-1 bilateral-predicate
//! pattern established at
//! bootstrap/tests/evaluator_shard_body_dispatch_smoke.rs. Constructor
//! actions (restrict, section_at) are `\`-obligation-blocked at the
//! substrate-decl altitude per craft-not-deliver; their bodies
//! discharge at consumer altitude (Arc-2.3 peer_persistence
//! composition at bootstrap/src/peer_persistence.rs, LANDED).
//!
//! Substrate-decl anchors:
//!   - restriction_admissible per shards/subject/visibility/sheaf.
//!     mirror:381: Pass iff peer_ref two-witness pass + acl resolves
//!     in @subject.pack.members + admitted_stalks byte-visible sub-
//!     list of Δ_F.
//!   - section_admissible per shards/subject/visibility/sheaf.mirror:
//!     428: Pass iff sheaf_ref transitively restriction_admissible +
//!     crystal_ref's stalk ∈ sheaf_ref's admitted_stalks.
//!
//! RED-phase expectation: neither resolver arm exists in apply_h::act;
//! all three tests return Partial per the missing-shard_action_ref
//! fallback. GREEN-phase expectation: both bilaterals dispatch;
//! two-witness + acl-resolves + stalks-bounded sentinels pass; missing
//! sentinels fail.
//!
//! Marker: [substrate-floor:@io-boundary] — Bridge β dispatch surface
//! at Rust FLOOR. Audit-cite: docs/audits/2026-07-15-seam-autopoietic-
//! loop-phase-d.md (55dbf20) + task #168 Seam Phase D adjudication.
//! Signed-off-by: Seam.

use mirror::apply_h::{act, Value, Verdict};

#[test]
fn sheaf_restriction_admissible_passes_on_witnessed_bounded_restriction() {
    // Per shards/subject/visibility/sheaf.mirror:381 restriction_
    // admissible docblock: three sub-checks (peer two-witness pass;
    // acl resolves in @subject.pack.members; admitted_stalks byte-
    // visible sub-list of Δ_F). Byte-check for the composed sentinel.
    let sr_ref = Value {
        oid: "sheaf-restriction-cli:peer=witnessed,acl=resolves,stalks=bounded"
            .to_string(),
    };

    let verdict = act(
        "@subject/visibility/sheaf.restriction_admissible".to_string(),
        vec![sr_ref],
    );

    assert!(
        matches!(verdict, Verdict::Pass),
        "restriction_admissible on witnessed+resolves+bounded sentinel should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn sheaf_section_admissible_passes_on_admissible_sheaf_and_admitted_stalk() {
    // Per shards/subject/visibility/sheaf.mirror:428 section_admissible
    // docblock: two sub-checks (sheaf_ref transitively restriction_
    // admissible; crystal_ref's stalk ∈ sheaf_ref's admitted_stalks).
    let sec_ref = Value {
        oid: "section-at-stalk-cli:sheaf=admissible,stalk=admitted".to_string(),
    };

    let verdict = act(
        "@subject/visibility/sheaf.section_admissible".to_string(),
        vec![sec_ref],
    );

    assert!(
        matches!(verdict, Verdict::Pass),
        "section_admissible on admissible-sheaf + admitted-stalk sentinel should Pass; \
         got {:?}",
        verdict
    );
}

#[test]
fn sheaf_restriction_admissible_fails_cleanly_on_unwitnessed_peer() {
    // Missing the peer=witnessed sentinel: substrate-honest Fail
    // (not Pass, not Partial). No crash; clean diagnostic.
    let sr_ref = Value {
        oid: "sheaf-restriction-cli:peer=unwitnessed,acl=resolves,stalks=bounded"
            .to_string(),
    };

    let verdict = act(
        "@subject/visibility/sheaf.restriction_admissible".to_string(),
        vec![sr_ref],
    );

    assert!(
        matches!(verdict, Verdict::Fail(_)),
        "restriction_admissible on unwitnessed peer MUST Fail (not Pass, not \
         Partial); got {:?}",
        verdict
    );
}
