//! Property pipeline integration test.
//!
//! End-to-end: .conv grammar with `requires`/`invariant` declarations
//! → parse → resolve → compile → ProofCertificate with property results
//! → ETF output with property tuples.
//!
//! This is the capstone test for the model checker pipeline.

use conversation::ffi::{self, CompileResult};
use conversation::logic::{ProofCertificate, PropertyKind, PropertyResult};
use conversation::parse::Parse;
use conversation::resolve::TypeRegistry;
use conversation::Vector;

// ---------------------------------------------------------------------------
// Helper: parse grammar → TypeRegistry
// ---------------------------------------------------------------------------

fn compile_grammar(source: &str) -> TypeRegistry {
    let ast = Parse.trace(source.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .expect("source must contain a grammar block");
    TypeRegistry::compile(grammar).unwrap()
}

// ---------------------------------------------------------------------------
// Full pipeline: grammar with `requires` → proof certificate
// ---------------------------------------------------------------------------

/// Grammar with a `requires` declaration produces a ProofCertificate
/// containing the corresponding PropertyResult.
#[test]
fn grammar_with_requires_produces_property_result() {
    let source = "\
grammar @test {
  type = a | b | c

  requires shannon_equivalence
}
";
    let reg = compile_grammar(source);
    let cert = ProofCertificate::from_registry(&reg);

    // The certificate should contain exactly one property result
    assert_eq!(cert.property_results.len(), 1);
    let pr = &cert.property_results[0];
    assert_eq!(pr.name, "shannon_equivalence");
    assert!(pr.satisfied, "shannon_equivalence should be satisfied");
    assert_eq!(pr.kind, PropertyKind::Required);
}

/// Grammar with an `invariant` declaration produces an Invariant-kind result.
#[test]
fn grammar_with_invariant_produces_invariant_result() {
    let source = "\
grammar @test {
  type = x | y

  invariant connected
}
";
    let reg = compile_grammar(source);
    let cert = ProofCertificate::from_registry(&reg);

    assert_eq!(cert.property_results.len(), 1);
    let pr = &cert.property_results[0];
    assert_eq!(pr.name, "connected");
    assert_eq!(pr.kind, PropertyKind::Invariant);
}

/// Grammar with both `requires` and `invariant` gets both in the certificate.
#[test]
fn grammar_with_mixed_properties() {
    let source = "\
grammar @test {
  type = a | b

  requires shannon_equivalence
  invariant connected
  requires exhaustive
}
";
    let reg = compile_grammar(source);
    let cert = ProofCertificate::from_registry(&reg);

    assert_eq!(cert.property_results.len(), 3);

    let names: Vec<&str> = cert.property_results.iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"shannon_equivalence"));
    assert!(names.contains(&"connected"));
    assert!(names.contains(&"exhaustive"));

    let required: Vec<&PropertyResult> = cert
        .property_results
        .iter()
        .filter(|r| r.kind == PropertyKind::Required)
        .collect();
    let invariants: Vec<&PropertyResult> = cert
        .property_results
        .iter()
        .filter(|r| r.kind == PropertyKind::Invariant)
        .collect();
    assert_eq!(required.len(), 2);
    assert_eq!(invariants.len(), 1);
}

// ---------------------------------------------------------------------------
// Clean passthrough: grammar without properties
// ---------------------------------------------------------------------------

/// Grammar with NO property declarations has empty property_results.
#[test]
fn grammar_without_properties_has_empty_results() {
    let source = "\
grammar @plain {
  type = x | y | z
}
";
    let reg = compile_grammar(source);
    let cert = ProofCertificate::from_registry(&reg);

    assert!(
        cert.property_results.is_empty(),
        "grammar without properties should have empty property_results"
    );

    // But the certificate still has facts and a valid proof OID
    assert!(!cert.facts.is_empty());
    assert!(!cert.proof_oid.as_ref().is_empty());
}

// ---------------------------------------------------------------------------
// Full FFI pipeline: grammar → compile_grammar_with_phases → ETF with properties
// ---------------------------------------------------------------------------

/// The full FFI pipeline: source → CompileResult → proof_etf contains property tuples.
#[test]
fn ffi_pipeline_includes_property_results_in_etf() {
    let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n}\n";
    let result: CompileResult = ffi::compile_grammar_with_phases(source).unwrap();

    // ETF should be non-empty and valid
    assert!(!result.proof_etf.is_empty());
    assert_eq!(result.proof_etf[0], 131, "valid ETF version byte");

    // Decode and verify properties key is present with our property
    let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
    let s = format!("{:?}", term);
    assert!(
        s.contains("properties"),
        "proof ETF should contain 'properties' key: {}",
        s
    );
    assert!(
        s.contains("shannon_equivalence"),
        "proof ETF should contain property name 'shannon_equivalence': {}",
        s
    );
}

/// The FFI pipeline with no properties still includes an empty properties list.
#[test]
fn ffi_pipeline_no_properties_still_has_properties_key() {
    let source = "grammar @plain {\n  type = a | b\n}\n";
    let result = ffi::compile_grammar_with_phases(source).unwrap();

    let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
    let s = format!("{:?}", term);
    assert!(
        s.contains("properties"),
        "proof ETF should contain 'properties' key even when empty: {}",
        s
    );
}

/// The proof OID changes when properties are added to a grammar.
#[test]
fn proof_oid_changes_with_properties() {
    let without = "grammar @test {\n  type = a | b\n}\n";
    let with = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n}\n";

    let cert_without = ProofCertificate::from_registry(&compile_grammar(without));
    let cert_with = ProofCertificate::from_registry(&compile_grammar(with));

    assert_ne!(
        cert_without.proof_oid, cert_with.proof_oid,
        "adding properties should change the proof OID"
    );
}

/// The full pipeline is deterministic — same source → same proof OID.
#[test]
fn pipeline_deterministic() {
    let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n  invariant connected\n}\n";

    let cert_a = ProofCertificate::from_registry(&compile_grammar(source));
    let cert_b = ProofCertificate::from_registry(&compile_grammar(source));

    assert_eq!(cert_a.proof_oid, cert_b.proof_oid);
    assert_eq!(cert_a.property_results.len(), cert_b.property_results.len());
}
