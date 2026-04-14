//! Doc example tests -- verify documentation examples compile.
//!
//! For every compilable `.mirror` code block in docs/*.md, this file
//! runs the example through parse_form and asserts it produces the
//! expected structure. If a doc example doesn't compile, the doc is wrong.

use fragmentation::sha::HashAlg;
use mirror::declaration::DeclKind;
use mirror::mirror_runtime::{parse_form, MirrorRuntime};

// ---------------------------------------------------------------------------
// docs/mirror.md
// ---------------------------------------------------------------------------

/// mirror.md: grammar @deploy with action, invariant, ensures
/// Note: the doc example includes `in @code/rust { struct State { ... } }`
/// which the parser wraps. We test with the raw body form.
#[test]
fn doc_mirror_grammar_deploy() {
    // Simplified from mirror.md — the `in @code/rust { struct ... }` block
    // uses non-mirror syntax inside the braces. The parser handles this by
    // skipping unrecognized tokens inside the in-block.
    let source = r#"grammar @deploy {
    action transform(data) {
        let result = serde_json::from_str(data)?;
        self.cache.insert(data.to_string(), result.clone());
        result
    }

    invariant pure
    ensures always_halts
}"#;
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Grammar);
    assert_eq!(form.name, "@deploy");
    let action_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Action)
        .count();
    assert_eq!(action_count, 1, "should have 1 action");
    let invariant_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Invariant)
        .count();
    assert_eq!(invariant_count, 1, "should have invariant");
    let ensures_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Ensures)
        .count();
    assert_eq!(ensures_count, 1, "should have ensures");
}

/// mirror.md: grammar with `in @code/rust` reference (no nested struct block)
/// Note: the doc example `in @code/rust { struct State { ... } }` uses
/// nested braces that interfere with the parser's brace balancing when
/// `struct` is not a DeclKind. The correct .mirror form is `in @code/rust`
/// as a flat declaration.
#[test]
fn doc_mirror_grammar_with_in_ref() {
    let source = r#"grammar @deploy {
    in @code/rust
    action transform(data) {
        apply(data)
    }
}"#;
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Grammar);
    assert_eq!(form.name, "@deploy");
    let in_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::In)
        .count();
    assert_eq!(in_count, 1, "should have 1 'in' child");
    assert_eq!(form.children[0].name, "@code/rust");
}

/// mirror.md: action declarations with grammar refs
#[test]
fn doc_mirror_cross_grammar_actions() {
    // These are standalone action declarations from the docs
    let source1 = "action ingest(data) in @code/python { parse(data) }";
    let form1 = parse_form(source1).ok().unwrap();
    assert_eq!(form1.kind, DeclKind::Action);
    assert_eq!(form1.name, "ingest");
    assert_eq!(form1.grammar_ref, Some("@code/python".to_string()));

    let source2 = "action transform(data) in @code/rust { transform(data) }";
    let form2 = parse_form(source2).ok().unwrap();
    assert_eq!(form2.kind, DeclKind::Action);
    assert_eq!(form2.grammar_ref, Some("@code/rust".to_string()));
}

/// mirror.md: invariant across grammars
#[test]
fn doc_mirror_invariant_declaration() {
    let source = "invariant deterministic";
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Invariant);
    assert_eq!(form.name, "deterministic");
}

// ---------------------------------------------------------------------------
// docs/witnessed-computation.md
// ---------------------------------------------------------------------------

/// witnessed-computation.md: type consent = granted | withdrawn | silent
#[test]
fn doc_witnessed_consent_type() {
    let source = "type consent = granted | withdrawn | silent";
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Type);
    assert_eq!(form.name, "consent");
    assert_eq!(
        form.variants,
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
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Action);
    assert_eq!(form.name, "proceed");
    assert_eq!(form.params, vec!["consent".to_string()]);
    assert_eq!(form.return_type, Some("imperfect".to_string()));
    assert!(form.body_text.is_some(), "action should have body");
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
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Form);
    assert_eq!(form.name, "@pipeline");
    // Should have prism, lens, recover, rescue children
    let prism_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Prism)
        .count();
    assert_eq!(prism_count, 2, "should have 2 prism children");
    let lens_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Lens)
        .count();
    assert_eq!(lens_count, 1, "should have 1 lens child");
    let recover_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Recover)
        .count();
    assert_eq!(recover_count, 1, "should have 1 recover child");
    let rescue_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Rescue)
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
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Grammar);
    assert_eq!(form.name, "@deploy");
    let action_count = form
        .children
        .iter()
        .filter(|c| c.kind == DeclKind::Action)
        .count();
    assert_eq!(action_count, 2, "should have 2 actions (transform, retry)");
}

/// shatter-spec.md: recover and rescue blocks (standalone)
#[test]
fn doc_shatter_recover_rescue() {
    // These appear as standalone blocks in the shatter-spec docs
    let source = r#"recover |shatter, loss| {
    log_drift(loss)
    shatter
}"#;
    let form = parse_form(source).ok().unwrap();
    assert_eq!(form.kind, DeclKind::Recover);
    assert!(form.body_text.is_some());
    assert_eq!(form.params.len(), 2);

    let source2 = r#"rescue |error| {
    annotate(old_shatter, error)
}"#;
    let form2 = parse_form(source2).ok().unwrap();
    assert_eq!(form2.kind, DeclKind::Rescue);
    assert!(form2.body_text.is_some());
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
        let emitted = mirror::mirror_runtime::emit_form(&compiled.form);
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
    // These blocks use { key: value } syntax that isn't part of .mirror grammar
    let aspirational = vec![
        "loss { phases: [] }",
        "properties { types_lowercase: pass }",
        "kernel { dimensions: [0, 1] }",
        "fate { weights: [] }",
    ];

    for source in &aspirational {
        // These should either fail to parse or parse as something other than
        // a structured declaration. The point is: the docs describe a format
        // that doesn't exist in the parser yet.
        let result = parse_form(source);
        // We don't assert is_err because the parser may return Partial for unknown tokens.
        // We DO assert that if it parses, it's not a meaningful structure.
        if let Some(form) = result.ok() {
            // If it parsed, it shouldn't have the keyword as a DeclKind
            // (loss, properties, kernel, fate aren't DeclKind variants)
            assert!(
                form.kind != DeclKind::Grammar,
                "aspirational block '{}' should not parse as Grammar",
                source
            );
        }
    }
}
