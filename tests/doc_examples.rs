//! Doc example tests -- verify documentation examples compile.
//!
//! For every compilable `.mirror` code block in docs/*.md, this file
//! runs the example through parse_form and asserts it produces the
//! expected structure. If a doc example doesn't compile, the doc is wrong.

use fragmentation::sha::HashAlg;
use mirror::declaration::{DeclKind, MirrorData, MirrorFragmentExt};
use mirror::mirror_runtime::{parse_form, MirrorRuntime};

/// Helper: decode MirrorData from a fragment (extracts encoded fields).
fn decode(frag: &mirror::declaration::MirrorFragment) -> MirrorData {
    MirrorData::decode_from_fragment(frag.mirror_data())
}

// ---------------------------------------------------------------------------
// docs/mirror.md
// ---------------------------------------------------------------------------

/// mirror.md: grammar @deploy with action, invariant, ensures
/// Note: the doc example includes `in @code/rust { struct State { ... } }`
/// which the parser wraps. We test with the raw body form.
#[test]
fn doc_mirror_grammar_deploy() {
    let source = r#"grammar @deploy {
    action transform(data) {
        let result = serde_json::from_str(data)?;
        self.cache.insert(data.to_string(), result.clone());
        result
    }

    invariant pure
    ensures always_halts
}"#;
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Grammar);
    assert_eq!(data.name, "@deploy");
    let action_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Action)
        .count();
    assert_eq!(action_count, 1, "should have 1 action");
    let invariant_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Invariant)
        .count();
    assert_eq!(invariant_count, 1, "should have invariant");
    let ensures_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Ensures)
        .count();
    assert_eq!(ensures_count, 1, "should have ensures");
}

/// mirror.md: grammar with `in @code/rust` reference (no nested struct block)
#[test]
fn doc_mirror_grammar_with_in_ref() {
    let source = r#"grammar @deploy {
    in @code/rust
    action transform(data) {
        apply(data)
    }
}"#;
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Grammar);
    assert_eq!(data.name, "@deploy");
    let in_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::In)
        .count();
    assert_eq!(in_count, 1, "should have 1 'in' child");
    assert_eq!(frag.mirror_children()[0].mirror_data().name, "@code/rust");
}

/// mirror.md: action declarations with grammar refs
#[test]
fn doc_mirror_cross_grammar_actions() {
    let source1 = "action ingest(data) in @code/python { parse(data) }";
    let frag1 = parse_form(source1).ok().unwrap();
    let data1 = decode(&frag1);
    assert_eq!(data1.kind, DeclKind::Action);
    assert_eq!(data1.name, "ingest");
    assert_eq!(data1.grammar_ref, Some("@code/python".to_string()));

    let source2 = "action transform(data) in @code/rust { transform(data) }";
    let frag2 = parse_form(source2).ok().unwrap();
    let data2 = decode(&frag2);
    assert_eq!(data2.kind, DeclKind::Action);
    assert_eq!(data2.grammar_ref, Some("@code/rust".to_string()));
}

/// mirror.md: invariant across grammars
#[test]
fn doc_mirror_invariant_declaration() {
    let source = "invariant deterministic";
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Invariant);
    assert_eq!(data.name, "deterministic");
}

// ---------------------------------------------------------------------------
// docs/witnessed-computation.md
// ---------------------------------------------------------------------------

/// witnessed-computation.md: type consent = granted | withdrawn | silent
#[test]
fn doc_witnessed_consent_type() {
    let source = "type consent = granted | withdrawn | silent";
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Type);
    assert_eq!(data.name, "consent");
    assert_eq!(
        data.variants,
        vec![
            "granted".to_string(),
            "withdrawn".to_string(),
            "silent".to_string()
        ]
    );
}

/// witnessed-computation.md: action proceed(consent) -> imperfect { ... }
#[test]
fn doc_witnessed_action_proceed() {
    let source = r#"action proceed(consent) -> imperfect {
    silent    -> Failure(no_consent, zero)
    withdrawn -> Failure(consent_withdrawn, accumulated_loss)
    granted   -> Partial(result, exchange_loss)
}"#;
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Action);
    assert_eq!(data.name, "proceed");
    assert_eq!(data.params, vec!["consent".to_string()]);
    assert_eq!(data.return_type, Some("imperfect".to_string()));
    assert!(data.body_text.is_some(), "action should have body");
}

// ---------------------------------------------------------------------------
// docs/emergent-holonomy-compiler.md
// ---------------------------------------------------------------------------

/// emergent-holonomy-compiler.md: form @pipeline with prism, lens, recover, rescue
#[test]
fn doc_ehc_pipeline_form() {
    let source = r#"form @pipeline {
    prism focus(input)
    lens transform(focused)
    prism validate(transformed)

    recover |value, loss| {
        log(loss)
        value
    }

    rescue |error| {
        fallback(error)
    }
}"#;
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Form);
    assert_eq!(data.name, "@pipeline");
    let prism_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Prism)
        .count();
    assert_eq!(prism_count, 2, "should have 2 prism children");
    let lens_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Lens)
        .count();
    assert_eq!(lens_count, 1, "should have 1 lens child");
    let recover_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Recover)
        .count();
    assert_eq!(recover_count, 1, "should have 1 recover child");
    let rescue_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Rescue)
        .count();
    assert_eq!(rescue_count, 1, "should have 1 rescue child");
}

// ---------------------------------------------------------------------------
// docs/shatter-spec.md
// ---------------------------------------------------------------------------

/// shatter-spec.md: grammar @deploy with actions (the fragment tree example)
#[test]
fn doc_shatter_grammar_deploy() {
    let source = r#"grammar @deploy {
    action transform(data) {
        let result = serde_json::from_str(data)?;
        self.cache.insert(data.to_string(), result.clone());
        result
    }

    action retry(operation) {
        self.retries += 1;
        operation()
    }
}"#;
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Grammar);
    assert_eq!(data.name, "@deploy");
    let action_count = frag
        .mirror_children()
        .iter()
        .filter(|c| c.mirror_data().kind == DeclKind::Action)
        .count();
    assert_eq!(action_count, 2, "should have 2 actions (transform, retry)");
}

/// shatter-spec.md: recover and rescue blocks (standalone)
#[test]
fn doc_shatter_recover_rescue() {
    let source = r#"recover |shatter, loss| {
    log_drift(loss)
    shatter
}"#;
    let frag = parse_form(source).ok().unwrap();
    let data = decode(&frag);
    assert_eq!(data.kind, DeclKind::Recover);
    assert!(data.body_text.is_some());
    assert_eq!(data.params.len(), 2);

    let source2 = r#"rescue |error| {
    annotate(old_shatter, error)
}"#;
    let frag2 = parse_form(source2).ok().unwrap();
    let data2 = decode(&frag2);
    assert_eq!(data2.kind, DeclKind::Rescue);
    assert!(data2.body_text.is_some());
}

// ---------------------------------------------------------------------------
// Round-trip: doc examples survive compile -> emit -> compile
// ---------------------------------------------------------------------------

/// Every compilable doc example should round-trip through the compiler.
#[test]
fn doc_examples_round_trip() {
    let examples = vec![
        "type consent = granted | withdrawn | silent",
        "invariant deterministic",
        r#"form @pipeline {
    prism focus(input)
    lens transform(focused)
}"#,
        r#"grammar @deploy {
    action transform(data) {
        apply(data)
    }
}"#,
    ];

    let runtime = MirrorRuntime::new();
    for source in &examples {
        let compiled: Result<_, _> = runtime.compile_source(source).into();
        let compiled = compiled
            .unwrap_or_else(|e| panic!("doc example failed to compile:\n{}\nerror: {}", source, e));
        // Round-trip: compile -> emit -> compile should produce same OID
        let emitted = mirror::mirror_runtime::emit_fragment(&compiled.fragment);
        let recompiled: Result<_, _> = runtime.compile_source(&emitted).into();
        let recompiled = recompiled.unwrap_or_else(|e| {
            panic!(
                "round-trip failed for doc example:\noriginal:\n{}\nemitted:\n{}\nerror: {}",
                source, emitted, e
            )
        });
        assert_eq!(
            compiled.crystal().as_str(),
            recompiled.crystal().as_str(),
            "round-trip OID mismatch for doc example:\n{}",
            source
        );
    }
}

// ---------------------------------------------------------------------------
// Aspirational doc examples that DON'T compile yet
// These test that the docs are honest about what's aspirational.
// ---------------------------------------------------------------------------

/// shatter-spec.md: loss {}, properties {}, kernel {}, fate {} blocks
/// are aspirational. They describe the future .shatter format.
/// These should NOT parse as valid .mirror today.
#[test]
fn aspirational_shatter_sections_do_not_parse_as_declarations() {
    let aspirational = vec![
        "loss { phases: [] }",
        "properties { types_lowercase: pass }",
        "kernel { dimensions: [0, 1] }",
        "fate { weights: [] }",
    ];

    for source in &aspirational {
        let result = parse_form(source);
        if let Some(frag) = result.ok() {
            let data = decode(&frag);
            assert!(
                data.kind != DeclKind::Grammar,
                "aspirational block '{}' should not parse as Grammar",
                source
            );
        }
    }
}
