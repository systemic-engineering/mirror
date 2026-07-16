//! Bilateral-arm collapse smoke — 2026-07-16 Reed
//! [substrate-floor:@io-boundary] per Mara canonical spec
//! `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md` (6c534c6)
//! + shard-decl fa569ce + math foundation 0998001 + reflective
//! evaluator dependency 21fc211.
//!
//! Verifies the pure byte-analysis surface (`find_redundant_arms` +
//! `apply_deletions` + `compose_collapse_commit_message`). Does NOT
//! run the end-to-end `collapse_bilateral_arms` pipeline against the
//! real `bootstrap/src/apply_h.rs` — that's main context's empirical
//! run, per task brief.

use mirror::apply_h::{self, BilateralDecl, Value, Verdict};
use mirror::bilateral_arm_collapse::{
    apply_deletions, compose_collapse_commit_message, find_redundant_arms,
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn make_decl(action_ref: &str, sentinel: &str, arity: usize) -> BilateralDecl {
    let name = action_ref
        .rsplit_once('.')
        .map(|(_, n)| n.to_string())
        .unwrap_or_else(|| action_ref.to_string());
    BilateralDecl {
        name,
        sentinel: sentinel.to_string(),
        arity,
        require: Vec::new(),
        full_action_ref: action_ref.to_string(),
    }
}

fn temp_root(name: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "mirror-bilateral-arm-collapse-{}-{}",
        name,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).expect("mk temp root");
    base
}

/// The canonical arm shape the ~30 hand-typed apply_h::act arms follow.
/// Same pattern as `@spectral/signature.signature_integrity` (retired
/// at 06f14f5) — one `if action == "..."` guard + argument nil-check
/// + inline `.contains("<sentinel>")` verdict discharge.
const FIXTURE_ARM: &str = r#"pub fn act(action: String, args: Vec<Value>) -> Verdict {
    if action == "@test/spec.test_predicate" {
        if let Some(a) = args.first() {
            if a.oid.contains("test=sentinel-matches") { return Verdict::Pass; }
            return Verdict::Fail(format!(
                "test_predicate: expected sentinel test=sentinel-matches, got {:?}",
                a.oid
            ));
        }
        return Verdict::Fail("test_predicate: missing arg".to_string());
    }
    Verdict::Fail("no arm matched".to_string())
}
"#;

#[test]
fn find_redundant_arms_flags_fixture_arm() {
    let mut corpus = HashMap::new();
    corpus.insert(
        "@test/spec.test_predicate".to_string(),
        make_decl("@test/spec.test_predicate", "test=sentinel-matches", 1),
    );

    let arms = find_redundant_arms(FIXTURE_ARM, &corpus);
    assert_eq!(arms.len(), 1, "expected one redundant arm, got {:?}", arms);
    let arm = &arms[0];
    assert_eq!(arm.action_ref, "@test/spec.test_predicate");
    assert_eq!(arm.sentinel, "test=sentinel-matches");

    // The recorded byte-range must cover the entire arm — from the
    // start of the `if action ==` line through the closing brace
    // (inclusive of trailing newline).
    let slice = &FIXTURE_ARM[arm.byte_start..arm.byte_end];
    assert!(
        slice.contains("if action == \"@test/spec.test_predicate\""),
        "arm slice must start at the if-line: {:?}",
        slice
    );
    assert!(
        slice.contains(".contains(\"test=sentinel-matches\")"),
        "arm slice must contain the sentinel check: {:?}",
        slice
    );
}

#[test]
fn apply_deletions_removes_the_arm_cleanly() {
    let mut corpus = HashMap::new();
    corpus.insert(
        "@test/spec.test_predicate".to_string(),
        make_decl("@test/spec.test_predicate", "test=sentinel-matches", 1),
    );

    let arms = find_redundant_arms(FIXTURE_ARM, &corpus);
    assert_eq!(arms.len(), 1);

    let mended = apply_deletions(FIXTURE_ARM, &arms);
    // The arm is gone.
    assert!(
        !mended.contains("@test/spec.test_predicate"),
        "mended source must not contain retired action ref: {:?}",
        mended
    );
    assert!(
        !mended.contains("test=sentinel-matches"),
        "mended source must not contain retired sentinel: {:?}",
        mended
    );
    // Surrounding fn wrapper preserved.
    assert!(
        mended.contains("pub fn act(action: String, args: Vec<Value>)"),
        "outer fn signature preserved: {:?}",
        mended
    );
    assert!(
        mended.contains("Verdict::Fail(\"no arm matched\".to_string())"),
        "tail arm preserved: {:?}",
        mended
    );
    // Byte-length strictly decreased.
    assert!(
        mended.len() < FIXTURE_ARM.len(),
        "byte-length must strictly decrease: {} → {}",
        FIXTURE_ARM.len(),
        mended.len()
    );
}

#[test]
fn find_redundant_arms_respects_absent_shard_bilaterals() {
    // Empty corpus → no arms flagged, even if the source contains
    // arm-shaped code. Defensive: proves the guard is corpus-lookup,
    // not blind pattern-match.
    let corpus = HashMap::new();
    let arms = find_redundant_arms(FIXTURE_ARM, &corpus);
    assert!(
        arms.is_empty(),
        "empty corpus MUST yield zero arms: {:?}",
        arms
    );
}

#[test]
fn find_redundant_arms_skips_arms_whose_sentinel_diverges() {
    // Arm hand-checks a DIFFERENT sentinel than the shard-decl.
    // Deletion would silently change resolver semantics — MUST NOT be
    // flagged. Bilateral discipline: byte-equality between the arm's
    // inline sentinel and the shard-decl'd sentinel.
    let mut corpus = HashMap::new();
    corpus.insert(
        "@test/spec.test_predicate".to_string(),
        make_decl(
            "@test/spec.test_predicate",
            "DIFFERENT=sentinel",
            1,
        ),
    );
    let arms = find_redundant_arms(FIXTURE_ARM, &corpus);
    assert!(
        arms.is_empty(),
        "arm with mismatched sentinel MUST NOT be flagged: {:?}",
        arms
    );
}

#[test]
fn loader_and_finder_compose_via_temp_shard_fixture() {
    // End-to-end (minus @io/git.commit): mint a temp shard root
    // declaring one bilateral, load the corpus, feed a fixture arm
    // through `find_redundant_arms`, verify detection.
    let root = temp_root("loader-finder-compose");
    let shard_path = root.join("shards/test/spec.mirror");
    fs::create_dir_all(shard_path.parent().unwrap()).unwrap();
    fs::write(
        &shard_path,
        "in @test/spec\n\
         prism @test/spec {}\n\
         \n\
         bilateral test_predicate {\n\
           sentinel \"test=sentinel-matches\"\n\
           arity 1\n\
         }\n",
    )
    .unwrap();

    let corpus = apply_h::load_bilateral_corpus(&root);
    let decl = corpus
        .get("@test/spec.test_predicate")
        .expect("bilateral loaded from shard fixture");
    assert_eq!(decl.sentinel, "test=sentinel-matches");

    let arms = find_redundant_arms(FIXTURE_ARM, &corpus);
    assert_eq!(arms.len(), 1);
    assert_eq!(arms[0].action_ref, "@test/spec.test_predicate");
    assert_eq!(arms[0].sentinel, "test=sentinel-matches");

    // And the reflective evaluator would discharge the same predicate
    // Pass on a sentinel-bearing arg — the redundancy invariant.
    let v = apply_h::discharge(
        decl,
        &[Value {
            oid: "fixture:test=sentinel-matches".to_string(),
        }],
    );
    assert!(
        matches!(v, Verdict::Pass),
        "reflective evaluator MUST Pass on sentinel-bearing arg: {:?}",
        v
    );
}

#[test]
fn commit_message_names_arms_and_sentinels_and_audit_chain() {
    let arms = vec![
        mirror::bilateral_arm_collapse::RedundantArm {
            action_ref: "@test/spec.p1".to_string(),
            sentinel: "s1=v".to_string(),
            byte_start: 0,
            byte_end: 100,
        },
        mirror::bilateral_arm_collapse::RedundantArm {
            action_ref: "@test/spec.p2".to_string(),
            sentinel: "s2=v".to_string(),
            byte_start: 200,
            byte_end: 350,
        },
    ];
    let msg = compose_collapse_commit_message(
        std::path::Path::new("bootstrap/src/apply_h.rs"),
        &arms,
        10_000,
        9_750,
    );
    // Marker.
    assert!(msg.contains("[substrate-floor:@io-boundary]"));
    // Both action refs + sentinels.
    assert!(msg.contains("@test/spec.p1"));
    assert!(msg.contains("s1=v"));
    assert!(msg.contains("@test/spec.p2"));
    assert!(msg.contains("s2=v"));
    // Byte delta arithmetic: 100 + 150 = 250 bytes.
    assert!(msg.contains("-250 bytes"), "expected -250 delta: {}", msg);
    assert!(msg.contains("10000 → 9750"));
    // Audit chain citations.
    assert!(msg.contains("21fc211"));
    assert!(msg.contains("fa569ce"));
    assert!(msg.contains("6c534c6"));
    assert!(msg.contains("0998001"));
    // Alex verbatim directive.
    assert!(msg.contains("Deleted Rust. Added mirror."));
    // Signing footer preserved.
    assert!(msg.contains("Signed-off-by: Reed <reed@systemic.engineer>"));
}
