//! Reflective bilateral dispatch smoke — Landings 3+4 empirical
//! witness. Per Mara canonical spec 9a77361 (docs/specs/bilateral-
//! predicate-substrate-shape.md) + math foundation 701828a
//! (docs/math/epistemologic/pact/bilateral-sentinel.md).
//!
//! Alex 2026-07-16 verbatim: "Q1. Let's mint it then. Properly.
//! Seems like it's load-bearing."
//!
//! Marker: [substrate-floor:@io-boundary] Reed 2026-07-16 the
//! reflective evaluator FLOOR that retires ~700 LOC of hand-typed
//! arms into ~50 LOC of grammar-loader-composed dispatch.
//!
//! This test uses the uncached `load_bilateral_corpus(temp_root)`
//! path so it doesn't pollute the process-wide corpus cache used by
//! `apply_h::act` for the real shards/**/*.mirror corpus.

use mirror::apply_h::{self, BilateralDecl, Value, Verdict};
use std::fs;
use std::path::PathBuf;

fn write_shard(root: &std::path::Path, rel: &str, content: &str) {
    let full = root.join(rel);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).expect("create shard parent");
    }
    fs::write(&full, content).expect("write shard fixture");
}

fn temp_root(name: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "mirror-reflective-bilateral-{}-{}",
        name,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).expect("mk temp root");
    base
}

#[test]
fn loader_extracts_base_bilateral_and_discharge_passes() {
    let root = temp_root("base-pass");
    write_shard(
        &root,
        "shards/test/reflective.mirror",
        "in @test/reflective\n\
         prism @test/reflective {}\n\
         \n\
         bilateral test_predicate {\n\
           sentinel \"test=sentinel-matches\"\n\
           arity 1\n\
         }\n",
    );

    let corpus = apply_h::load_bilateral_corpus(&root);
    let decl = corpus
        .get("@test/reflective.test_predicate")
        .expect("bilateral loaded into corpus");
    assert_eq!(decl.sentinel, "test=sentinel-matches");
    assert_eq!(decl.arity, 1);
    assert!(decl.require.is_empty());

    let verdict = apply_h::discharge(
        decl,
        &[Value {
            oid: "fixture:test=sentinel-matches".to_string(),
        }],
    );
    assert!(
        matches!(verdict, Verdict::Pass),
        "sentinel present in arg oid must Pass; got {:?}",
        verdict
    );
}

#[test]
fn discharge_fails_cleanly_when_sentinel_absent() {
    let decl = BilateralDecl {
        name: "test_predicate".to_string(),
        sentinel: "test=sentinel-matches".to_string(),
        arity: 1,
        require: Vec::new(),
        full_action_ref: "@test/reflective.test_predicate".to_string(),
    };
    let verdict = apply_h::discharge(
        &decl,
        &[Value {
            oid: "fixture:test=sentinel-DOES-NOT-MATCH".to_string(),
        }],
    );
    assert!(
        matches!(verdict, Verdict::Fail(_)),
        "absent sentinel MUST Fail; got {:?}",
        verdict
    );
}

#[test]
fn discharge_fails_on_arity_mismatch() {
    let decl = BilateralDecl {
        name: "test_predicate".to_string(),
        sentinel: "test=sentinel-matches".to_string(),
        arity: 2,
        require: Vec::new(),
        full_action_ref: "@test/reflective.test_predicate".to_string(),
    };
    let verdict = apply_h::discharge(
        &decl,
        &[Value {
            oid: "fixture:test=sentinel-matches".to_string(),
        }],
    );
    assert!(
        matches!(verdict, Verdict::Fail(_)),
        "arity mismatch MUST Fail; got {:?}",
        verdict
    );
}

#[test]
fn discharge_cross_shard_require_uses_full_ref_as_is() {
    // §4.3.1 verification per Seam Phase D audit `afcf3b2`
    // (docs/audits/2026-07-17-seam-phase-d-autopoietic-rust-consumption-arc.md).
    // When `require` field contains a `.` (cross-shard ref like
    // "@X/Y.foo"), `discharge()` must use it AS-IS for corpus lookup
    // (NOT prefix-mangled with enclosing shard). Verify by Fail-path:
    // the sub-bilateral isn't in the cached corpus, so `discharge()`
    // returns Fail naming the FULL cross-shard ref in the error message.
    let decl = BilateralDecl {
        name: "composed_predicate".to_string(),
        sentinel: "composed=matches".to_string(),
        arity: 1,
        require: vec!["@test/other_shard.other_predicate".to_string()],
        full_action_ref: "@test/composed_shard.composed_predicate".to_string(),
    };
    let verdict = apply_h::discharge(
        &decl,
        &[Value {
            oid: "fixture:composed=matches".to_string(),
        }],
    );
    match verdict {
        Verdict::Fail(msg) => {
            assert!(
                msg.contains("@test/other_shard.other_predicate"),
                "sub-bilateral not in corpus MUST name FULL cross-shard \
                 ref as-is (contains '.'); got msg: {}",
                msg
            );
        }
        other => panic!(
            "expected Fail for missing sub-bilateral; got {:?}",
            other
        ),
    }
}

#[test]
fn discharge_same_shard_require_prefixes_bare_name() {
    // §4.3.1 verification companion: when `require` field is bare-name
    // (no `.`), `discharge()` MUST prefix with the enclosing shard's
    // @-ref (extracted from `full_action_ref` via rsplit_once('.')).
    // Verify by Fail-path with expected prefix-mangled ref in message.
    let decl = BilateralDecl {
        name: "composed_predicate".to_string(),
        sentinel: "composed=matches".to_string(),
        arity: 1,
        require: vec!["bare_sub_name".to_string()],
        full_action_ref: "@test/composed_shard.composed_predicate".to_string(),
    };
    let verdict = apply_h::discharge(
        &decl,
        &[Value {
            oid: "fixture:composed=matches".to_string(),
        }],
    );
    match verdict {
        Verdict::Fail(msg) => {
            assert!(
                msg.contains("@test/composed_shard.bare_sub_name"),
                "same-shard require MUST prefix with enclosing shard's \
                 @-ref; got msg: {}",
                msg
            );
        }
        other => panic!(
            "expected Fail for missing sub-bilateral; got {:?}",
            other
        ),
    }
}

#[test]
fn loader_skips_grammar_and_pact_keywords_files() {
    let root = temp_root("skips");
    // The skip-list files: if we put a bilateral block in these,
    // the loader should NOT surface it.
    write_shard(
        &root,
        "shards/mirror/grammar.mirror",
        "bilateral should_be_skipped {\n  sentinel \"x=y\"\n  arity 1\n}\n",
    );
    write_shard(
        &root,
        "shards/epistemologic/pact/keywords.mirror",
        "bilateral also_skipped {\n  sentinel \"x=y\"\n  arity 1\n}\n",
    );
    // A regular shard for sanity — should be picked up.
    write_shard(
        &root,
        "shards/test/sanity.mirror",
        "prism @test/sanity {}\n\
         bilateral picked_up {\n  sentinel \"sanity=ok\"\n  arity 1\n}\n",
    );

    let corpus = apply_h::load_bilateral_corpus(&root);
    assert!(
        corpus.get("@test/sanity.picked_up").is_some(),
        "regular shard bilateral must be in corpus"
    );
    assert!(
        !corpus
            .keys()
            .any(|k| k.ends_with(".should_be_skipped") || k.ends_with(".also_skipped")),
        "grammar.mirror + pact/keywords.mirror must be skipped; corpus keys: {:?}",
        corpus.keys().collect::<Vec<_>>()
    );
}
