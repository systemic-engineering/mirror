//! Property pipeline integration test.
//!
//! End-to-end: .conv grammar with `requires`/`invariant` declarations
//! -> parse -> resolve -> compile -> declarations pass through without evaluation
//! -> ETF output with raw declaration lists.

use conversation::ffi::{self, CompileResult};
use conversation::logic::ProofCertificate;
use conversation::parse::Parse;
use conversation::resolve::TypeRegistry;
use conversation::Vector;

fn compile_grammar(source: &str) -> TypeRegistry {
    let ast = Parse.trace(source.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .expect("source must contain a grammar block");
    TypeRegistry::compile(grammar).unwrap()
}

#[test]
fn grammar_with_requires_passes_through_declarations() {
    let source = "grammar @test {\n  type = a | b | c\n\n  requires shannon_equivalence\n}\n";
    let reg = compile_grammar(source);
    assert_eq!(
        reg.required_properties(),
        &["shannon_equivalence".to_string()]
    );
    assert!(reg.invariants().is_empty());
    let cert = ProofCertificate::from_registry(&reg);
    assert!(!cert.facts.is_empty());
    assert!(!cert.proof_oid.as_ref().is_empty());
}

#[test]
fn grammar_with_invariant_passes_through_declarations() {
    let source = "grammar @test {\n  type = x | y\n\n  invariant connected\n}\n";
    let reg = compile_grammar(source);
    assert!(reg.required_properties().is_empty());
    assert_eq!(reg.invariants(), &["connected".to_string()]);
}

#[test]
fn grammar_with_mixed_declarations() {
    let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n  invariant connected\n  requires exhaustive\n}\n";
    let reg = compile_grammar(source);
    assert_eq!(reg.required_properties().len(), 2);
    assert!(reg
        .required_properties()
        .contains(&"shannon_equivalence".to_string()));
    assert!(reg
        .required_properties()
        .contains(&"exhaustive".to_string()));
    assert_eq!(reg.invariants().len(), 1);
    assert!(reg.invariants().contains(&"connected".to_string()));
}

#[test]
fn grammar_without_properties_has_empty_declarations() {
    let source = "grammar @plain {\n  type = x | y | z\n}\n";
    let reg = compile_grammar(source);
    let cert = ProofCertificate::from_registry(&reg);
    assert!(reg.required_properties().is_empty());
    assert!(reg.invariants().is_empty());
    assert!(!cert.facts.is_empty());
    assert!(!cert.proof_oid.as_ref().is_empty());
}

#[test]
fn ffi_pipeline_includes_declarations_in_compile_result() {
    let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n  invariant connected\n}\n";
    let result: CompileResult = ffi::compile_grammar_with_phases(source).unwrap();
    assert_eq!(
        result.required_properties,
        vec!["shannon_equivalence".to_string()]
    );
    assert_eq!(result.invariants, vec!["connected".to_string()]);
}

#[test]
fn ffi_pipeline_etf_contains_declaration_keys() {
    let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n}\n";
    let result: CompileResult = ffi::compile_grammar_with_phases(source).unwrap();
    assert!(!result.proof_etf.is_empty());
    assert_eq!(result.proof_etf[0], 131, "valid ETF version byte");
    let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
    let s = format!("{:?}", term);
    assert!(
        s.contains("requires"),
        "proof ETF should contain 'requires' key: {}",
        s
    );
    assert!(
        s.contains("invariants"),
        "proof ETF should contain 'invariants' key: {}",
        s
    );
    let name_bytes = "shannon_equivalence".as_bytes();
    assert!(
        result
            .proof_etf
            .windows(name_bytes.len())
            .any(|w| w == name_bytes),
        "proof ETF should contain property name 'shannon_equivalence' as binary bytes"
    );
}

#[test]
fn ffi_pipeline_no_properties_still_has_declaration_keys() {
    let source = "grammar @plain {\n  type = a | b\n}\n";
    let result = ffi::compile_grammar_with_phases(source).unwrap();
    assert!(result.required_properties.is_empty());
    assert!(result.invariants.is_empty());
    let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
    let s = format!("{:?}", term);
    assert!(
        s.contains("requires"),
        "proof ETF should contain 'requires' key even when empty: {}",
        s
    );
    assert!(
        s.contains("invariants"),
        "proof ETF should contain 'invariants' key even when empty: {}",
        s
    );
}

#[test]
fn pipeline_deterministic() {
    let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n  invariant connected\n}\n";
    let cert_a = ProofCertificate::from_registry(&compile_grammar(source));
    let cert_b = ProofCertificate::from_registry(&compile_grammar(source));
    assert_eq!(cert_a.proof_oid, cert_b.proof_oid);
}
