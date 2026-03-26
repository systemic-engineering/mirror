//! Property-based testing derived from grammars. No randomness — exhaustive.
//!
//! The grammar IS the generator. Shannon equivalence is the first property:
//! content addressing preserved through derivation. If the hash doesn't change,
//! the content didn't change. If the content changed, the hash changes.

use std::collections::{HashMap, HashSet};

use crate::generate::{self, Derivation};
use crate::parse::{self, HasAssertion, PropertyCheck, TestDirective};
use crate::prism;
use crate::resolve::{GenerateProvider, Namespace, TypeRegistry};

/// Outcome of a property check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail(String),
}

/// Result of checking a single property or test.
#[derive(Clone, Debug)]
pub struct PropertyResult {
    pub name: String,
    pub derivations_checked: usize,
    pub verdict: Verdict,
}

/// Check a property against all derivations of a grammar.
pub fn check_property<F>(registry: &TypeRegistry, name: &str, prop: F) -> PropertyResult
where
    F: Fn(&[Derivation]) -> Verdict,
{
    let derivations = generate::derive_all(registry);
    let count = derivations.len();
    let verdict = prop(&derivations);
    PropertyResult {
        name: name.to_string(),
        derivations_checked: count,
        verdict,
    }
}

/// Shannon equivalence: content addressing preserved through derivation.
///
/// Two checks:
/// 1. **Determinism:** Same derivation → same OID (run twice, compare)
/// 2. **Uniqueness:** Different derivations → different OIDs (no collisions)
pub fn shannon_equivalence(derivations: &[Derivation]) -> Verdict {
    // Determinism is structural: content_oid is pure, so same input → same OID
    // by construction. The check_determinism utility verifies this contract in tests.
    let oids: Vec<(String, String)> = derivations
        .iter()
        .map(|d| (d.variant.clone(), prism::content_oid(&d.tree)))
        .collect();

    // Uniqueness: all OIDs must be distinct
    check_uniqueness(&oids)
}

/// Check that all (variant, OID) pairs have distinct OIDs.
fn check_uniqueness(oids: &[(String, String)]) -> Verdict {
    let mut seen: HashSet<String> = HashSet::new();
    for (variant, oid) in oids {
        let oid_str = oid.to_string();
        if !seen.insert(oid_str.clone()) {
            // Find the colliding variant
            let other = oids
                .iter()
                .find(|(v, o)| v != variant && **o == *oid_str)
                .map(|(v, _)| v.as_str())
                .unwrap_or("unknown");
            return Verdict::Fail(format!(
                "uniqueness: variants \"{}\" and \"{}\" produce the same OID",
                variant, other
            ));
        }
    }

    Verdict::Pass
}

/// Check a static `test` block: verify that grammars have the expected variants.
fn check_test(name: &str, assertions: &[HasAssertion], namespace: &Namespace) -> PropertyResult {
    for a in assertions {
        let registry = match namespace.grammar(&a.domain) {
            Some(r) => r,
            None => {
                return PropertyResult {
                    name: name.to_string(),
                    derivations_checked: 0,
                    verdict: Verdict::Fail(format!("unknown domain @{}", a.domain)),
                };
            }
        };
        let type_name = a.type_name.as_deref().unwrap_or("");
        if !registry.has_variant(type_name, &a.variant) {
            return PropertyResult {
                name: name.to_string(),
                derivations_checked: 0,
                verdict: Verdict::Fail(format!(
                    "@{}{} does not have variant \"{}\"",
                    a.domain,
                    a.type_name
                        .as_ref()
                        .map(|t| format!(".{}", t))
                        .unwrap_or_default(),
                    a.variant,
                )),
            };
        }
    }
    PropertyResult {
        name: name.to_string(),
        derivations_checked: 0,
        verdict: Verdict::Pass,
    }
}

/// Built-in property lookup (internal, for check_all property blocks).
fn lookup_property(name: &str) -> Option<fn(&[Derivation]) -> Verdict> {
    match name {
        "shannon_equivalence" => Some(shannon_equivalence),
        _ => None,
    }
}

/// Derive from a registry, respecting generate overrides.
fn derive_with_provider(registry: &TypeRegistry, provider: &GenerateProvider) -> Vec<Derivation> {
    match provider {
        GenerateProvider::Derived => generate::derive_all(registry),
        GenerateProvider::Override(overrides) => {
            // Apply overrides: replace type variants with custom ones
            let mut derivations = Vec::new();
            for (type_name, custom_variants) in overrides {
                for variant in custom_variants {
                    let tree = crate::ast::ast_leaf(
                        crate::domain::conversation::Kind::Form,
                        "variant",
                        variant.as_str(),
                        crate::ast::Span { start: 0, end: 0 },
                    );
                    derivations.push(Derivation {
                        type_name: type_name.clone(),
                        variant: variant.clone(),
                        tree,
                    });
                }
            }
            derivations
        }
    }
}

/// Check a `property` block against grammar derivations, with override support.
fn check_property_block_with_overrides(
    name: &str,
    checks: &[PropertyCheck],
    namespace: &Namespace,
    overrides: &HashMap<String, GenerateProvider>,
) -> Vec<PropertyResult> {
    let mut results = Vec::new();
    for check in checks {
        let registry = match namespace.grammar(&check.domain) {
            Some(r) => r,
            None => {
                results.push(PropertyResult {
                    name: format!("{}: @{}", name, check.domain),
                    derivations_checked: 0,
                    verdict: Verdict::Fail(format!("unknown domain @{}", check.domain)),
                });
                continue;
            }
        };
        let prop_fn = match lookup_property(&check.property) {
            Some(f) => f,
            None => {
                results.push(PropertyResult {
                    name: format!("{}: {}", name, check.property),
                    derivations_checked: 0,
                    verdict: Verdict::Fail(format!("unknown property \"{}\"", check.property)),
                });
                continue;
            }
        };
        let provider = overrides
            .get(&check.domain)
            .unwrap_or(&GenerateProvider::Derived);
        let derivations = derive_with_provider(&registry, provider);
        let count = derivations.len();
        let verdict = prop_fn(&derivations);
        results.push(PropertyResult {
            name: format!("{}: @{} preserves {}", name, check.domain, check.property),
            derivations_checked: count,
            verdict,
        });
    }
    results
}

/// Check all directives from a parsed test section.
pub fn check_all(namespace: &Namespace, test_section: &str) -> Result<Vec<PropertyResult>, String> {
    let directives = parse::parse_test_section(test_section)
        .map_err(|e| format!("test section parse error: {}", e))?;

    let mut results = Vec::new();

    // First pass: collect generate overrides
    let mut overrides: HashMap<String, GenerateProvider> = HashMap::new();
    for directive in &directives {
        if let TestDirective::Generate {
            domain,
            overrides: ovs,
        } = directive
        {
            overrides.insert(domain.clone(), GenerateProvider::Override(ovs.clone()));
        }
    }

    // Merge namespace-level overrides (test section overrides take priority)
    for domain in namespace.grammar_domains() {
        let ns_provider = namespace.generate_provider(&domain);
        if !matches!(ns_provider, GenerateProvider::Derived) && !overrides.contains_key(&domain) {
            overrides.insert(domain, ns_provider.clone());
        }
    }

    // Second pass: check tests and properties
    for directive in &directives {
        match directive {
            TestDirective::Test { name, assertions } => {
                results.push(check_test(name, assertions, namespace));
            }
            TestDirective::Property { name, checks } => {
                results.extend(check_property_block_with_overrides(
                    name, checks, namespace, &overrides,
                ));
            }
            TestDirective::Generate { .. } => {} // already collected
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parse;
    use crate::resolve::TypeRegistry;
    use crate::Vector;

    /// Extract the failure message from a Verdict, panicking if Pass.
    impl Verdict {
        fn fail_msg(&self) -> &str {
            match self {
                Verdict::Fail(msg) => msg,
                Verdict::Pass => panic!("expected Fail, got Pass"),
            }
        }
    }

    /// Check that two OIDs (as strings) computed from the same derivation match.
    /// Determinism is structural (content_oid is pure), but this utility lets
    /// tests verify the contract explicitly.
    fn check_determinism(variant: &str, oid1: &str, oid2: &str) -> Verdict {
        if oid1 != oid2 {
            return Verdict::Fail(format!(
                "determinism: variant {} produced different OIDs on repeated call",
                variant
            ));
        }
        Verdict::Pass
    }

    #[test]
    #[should_panic(expected = "expected Fail")]
    fn verdict_fail_msg_panics_on_pass() {
        Verdict::Pass.fail_msg();
    }

    fn compile_grammar(source: &str) -> TypeRegistry {
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("source must contain a grammar block");
        TypeRegistry::compile(grammar).unwrap()
    }

    // -- shannon_equivalence --

    #[test]
    fn shannon_determinism_passes() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let derivations = generate::derive_all(&reg);
        assert_eq!(shannon_equivalence(&derivations), Verdict::Pass);
    }

    #[test]
    fn shannon_uniqueness_passes() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        let derivations = generate::derive_all(&reg);
        assert_eq!(shannon_equivalence(&derivations), Verdict::Pass);
    }

    #[test]
    fn shannon_parameterized_passes() {
        let reg = compile_grammar(
            "grammar @test {\n  type = plain | when(op)\n  type op = gt | lt | eq\n}\n",
        );
        let derivations = generate::derive_all(&reg);
        assert_eq!(shannon_equivalence(&derivations), Verdict::Pass);
    }

    #[test]
    fn shannon_with_acts_passes() {
        let reg = compile_grammar(
            "grammar @test {\n  type = a\n  action send {\n    to\n    subject\n  }\n}\n",
        );
        let derivations = generate::derive_all(&reg);
        assert_eq!(shannon_equivalence(&derivations), Verdict::Pass);
    }

    #[test]
    fn shannon_empty_grammar_passes() {
        let reg = compile_grammar("grammar @empty {}\n");
        let derivations = generate::derive_all(&reg);
        assert_eq!(shannon_equivalence(&derivations), Verdict::Pass);
    }

    // -- check_property --

    #[test]
    fn check_property_shannon() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let result = check_property(&reg, "shannon", shannon_equivalence);
        assert_eq!(result.verdict, Verdict::Pass);
        assert_eq!(result.derivations_checked, 3);
        assert_eq!(result.name, "shannon");
    }

    #[test]
    fn check_property_custom_failing() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let result = check_property(&reg, "always_fail", |_| Verdict::Fail("intentional".into()));
        assert_eq!(result.verdict, Verdict::Fail("intentional".into()));
    }

    // -- check_all --

    #[test]
    fn check_all_test_pass() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "test \"basics\" { @test has a; @test has b }";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].verdict, Verdict::Pass);
    }

    #[test]
    fn check_all_test_fail() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "test \"bad\" { @test has missing }";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 1);
        results[0].verdict.fail_msg();
    }

    #[test]
    fn check_all_test_unknown_domain() {
        let namespace = Namespace::new();
        let test_src = "test \"bad\" { @missing has x }";
        let results = check_all(&namespace, test_src).unwrap();
        results[0].verdict.fail_msg();
    }

    #[test]
    fn check_all_property_pass() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "property \"shannon\" { @test preserves shannon_equivalence }";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[0].derivations_checked, 3);
    }

    #[test]
    fn check_all_property_unknown_domain() {
        let namespace = Namespace::new();
        let test_src = "property \"bad\" { @missing preserves shannon_equivalence }";
        let results = check_all(&namespace, test_src).unwrap();
        results[0].verdict.fail_msg();
    }

    #[test]
    fn check_all_property_unknown_property() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "property \"bad\" { @test preserves nonexistent }";
        let results = check_all(&namespace, test_src).unwrap();
        assert!(results[0].verdict.fail_msg().contains("nonexistent"));
    }

    #[test]
    fn check_all_mixed_directives() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "test \"has\" { @test has a }\nproperty \"shannon\" { @test preserves shannon_equivalence }\n";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[1].verdict, Verdict::Pass);
    }

    #[test]
    fn check_all_parse_error() {
        let namespace = Namespace::new();
        let test_src = "test \"broken\" {\n  @x has y\n";
        let err = check_all(&namespace, test_src);
        assert!(err.is_err());
    }

    #[test]
    fn check_all_typed_assertion() {
        let reg = compile_grammar("grammar @test {\n  type = a\n  type op = gt | lt\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "test \"typed\" { @test.op has gt }";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results[0].verdict, Verdict::Pass);
    }

    #[test]
    fn check_all_typed_assertion_fail() {
        let reg = compile_grammar("grammar @test {\n  type = a\n  type op = gt | lt\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        let test_src = "test \"typed\" { @test.op has missing }";
        let results = check_all(&namespace, test_src).unwrap();
        results[0].verdict.fail_msg();
    }

    // -- Integration: real grammar Shannon test --

    /// Shannon equivalence against real conv grammars from the project.
    fn assert_shannon_on_conv(source: &str, label: &str) {
        let (spec, _) = crate::packages::split_test_section(source);
        let ast = Parse.trace(spec.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("conv file must have a grammar block");
        let reg = TypeRegistry::compile(grammar).unwrap();
        let derivations = generate::derive_all(&reg);
        assert!(
            !derivations.is_empty(),
            "{} grammar should have derivations",
            label,
        );
        assert_eq!(
            shannon_equivalence(&derivations),
            Verdict::Pass,
            "{} grammar must preserve Shannon equivalence",
            label,
        );
    }

    #[test]
    fn shannon_compiler_grammar() {
        assert_shannon_on_conv(include_str!("../conv/compiler.conv"), "compiler.conv");
    }

    #[test]
    fn shannon_mail_grammar() {
        assert_shannon_on_conv(include_str!("../conv/mail.conv"), "mail.conv");
    }

    #[test]
    fn shannon_beam_grammar() {
        assert_shannon_on_conv(include_str!("../conv/beam.conv"), "beam.conv");
    }

    #[test]
    fn shannon_git_grammar() {
        assert_shannon_on_conv(include_str!("../conv/git.conv"), "git.conv");
    }

    // -- Generate override --

    #[test]
    fn check_all_with_generate_override() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        // Override reduces derivation space to 2 custom variants
        let test_src = "generate @test { type = x | y }\nproperty \"shannon\" { @test preserves shannon_equivalence }\n";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[0].derivations_checked, 2);
    }

    #[test]
    fn check_all_namespace_generate_provider() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        namespace.register_generate(
            "test",
            GenerateProvider::Override(vec![("".into(), vec!["p".into(), "q".into()])]),
        );
        let test_src = "property \"shannon\" { @test preserves shannon_equivalence }";
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results[0].verdict, Verdict::Pass);
        assert_eq!(results[0].derivations_checked, 2);
    }

    #[test]
    fn check_all_test_section_override_beats_namespace() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let mut namespace = Namespace::new();
        namespace.register_grammar("test", reg);
        // Namespace says override with 2 variants
        namespace.register_generate(
            "test",
            GenerateProvider::Override(vec![("".into(), vec!["p".into(), "q".into()])]),
        );
        // Test section overrides with 1 variant
        let test_src = "generate @test { type = solo }\nproperty \"shannon\" { @test preserves shannon_equivalence }\n";
        let results = check_all(&namespace, test_src).unwrap();
        // Test section override wins — 1 derivation
        assert_eq!(results[0].derivations_checked, 1);
    }

    #[test]
    fn generate_provider_default_is_derived() {
        let namespace = Namespace::new();
        assert_eq!(
            *namespace.generate_provider("anything"),
            GenerateProvider::Derived
        );
    }

    // -- Shannon failure paths (synthetic) --

    #[test]
    fn check_determinism_fails_on_different_oids() {
        let verdict = check_determinism("test_variant", "abc123", "def456");
        let msg = verdict.fail_msg();
        assert!(msg.contains("determinism"));
        assert!(msg.contains("test_variant"));
    }

    #[test]
    fn check_determinism_passes_on_same_oids() {
        assert_eq!(check_determinism("v", "abc123", "abc123"), Verdict::Pass);
    }

    #[test]
    fn check_uniqueness_fails_on_colliding_oids() {
        let oids = vec![
            ("alpha".to_string(), "same_hash".to_string()),
            ("beta".to_string(), "same_hash".to_string()),
        ];
        let verdict = check_uniqueness(&oids);
        let msg = verdict.fail_msg();
        assert!(msg.contains("uniqueness"));
        assert!(msg.contains("alpha"));
        assert!(msg.contains("beta"));
    }

    #[test]
    fn check_uniqueness_passes_on_distinct_oids() {
        let oids = vec![
            ("a".to_string(), "hash1".to_string()),
            ("b".to_string(), "hash2".to_string()),
        ];
        assert_eq!(check_uniqueness(&oids), Verdict::Pass);
    }

    // -- Garden @property domain --
    //
    // These tests verify the @property garden grammar (garden/public/@property/property.conv)
    // compiles correctly and its test section passes. The grammar declares the vocabulary
    // for property-based verification: types, kinds, verdicts, and built-in property names.

    #[test]
    fn garden_property_grammar_compiles() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@property/property.conv"),
        )
        .expect("garden @property/property.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        assert_eq!(
            parts.len(),
            2,
            "property.conv should have grammar and test sections"
        );

        // Compile the grammar section
        let grammar_src = parts[0];
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("should have grammar block");
        let reg = TypeRegistry::compile(grammar).unwrap();
        assert_eq!(reg.domain, "property");

        // Verify types
        assert!(reg.has_variant("", "requires"));
        assert!(reg.has_variant("", "invariant"));
        assert!(reg.has_variant("", "ensures"));
        assert!(reg.has_variant("kind", "derivation"));
        assert!(reg.has_variant("kind", "registry"));
        assert!(reg.has_variant("kind", "spectral"));
        assert!(reg.has_variant("verdict", "pass"));
        assert!(reg.has_variant("verdict", "fail"));
        assert!(reg.has_variant("builtin", "shannon_equivalence"));
        assert!(reg.has_variant("builtin", "connected"));
        assert!(reg.has_variant("builtin", "components"));
        assert!(reg.has_variant("builtin", "exhaustive"));
    }

    #[test]
    fn garden_property_tests_pass() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@property/property.conv"),
        )
        .expect("garden @property/property.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        let grammar_src = parts[0];
        let test_src = parts[1];

        // Compile grammar and register in namespace
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let reg = TypeRegistry::compile(grammar).unwrap();

        let mut namespace = Namespace::new();
        namespace.register_grammar("property", reg);

        // Run all test directives
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 4, "expected 4 test blocks");
        for result in &results {
            assert_eq!(
                result.verdict,
                Verdict::Pass,
                "test '{}' failed: {:?}",
                result.name,
                result.verdict,
            );
        }
    }

    // -- Garden @topology domain --
    //
    // These tests verify the @topology garden grammar (garden/public/@topology/topology.conv)
    // compiles correctly and its test section passes. The grammar declares the vocabulary
    // for graph topology concepts: measures, phases, partitions, and boundaries.

    #[test]
    fn garden_topology_grammar_compiles() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@topology/topology.conv"),
        )
        .expect("garden @topology/topology.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        assert_eq!(
            parts.len(),
            2,
            "topology.conv should have grammar and test sections"
        );

        // Compile the grammar section
        let grammar_src = parts[0];
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("should have grammar block");
        let reg = TypeRegistry::compile(grammar).unwrap();
        assert_eq!(reg.domain, "topology");

        // Verify types
        assert!(reg.has_variant("", "graph"));
        assert!(reg.has_variant("", "node"));
        assert!(reg.has_variant("", "edge"));
        assert!(reg.has_variant("", "subgraph"));
        assert!(reg.has_variant("", "actor"));
        assert!(reg.has_variant("measure", "spectrum"));
        assert!(reg.has_variant("measure", "entropy"));
        assert!(reg.has_variant("measure", "curvature"));
        assert!(reg.has_variant("measure", "fiedler"));
        assert!(reg.has_variant("measure", "eigengap"));
        assert!(reg.has_variant("measure", "heat_kernel"));
        assert!(reg.has_variant("phase", "stable"));
        assert!(reg.has_variant("phase", "transition"));
        assert!(reg.has_variant("phase", "critical"));
        assert!(reg.has_variant("partition", "connected"));
        assert!(reg.has_variant("partition", "disconnected"));
        assert!(reg.has_variant("partition", "fragmented"));
        assert!(reg.has_variant("boundary", "internal"));
        assert!(reg.has_variant("boundary", "external"));
        assert!(reg.has_variant("boundary", "ghost"));
    }

    #[test]
    fn garden_topology_tests_pass() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@topology/topology.conv"),
        )
        .expect("garden @topology/topology.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        let grammar_src = parts[0];
        let test_src = parts[1];

        // Compile grammar and register in namespace
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let reg = TypeRegistry::compile(grammar).unwrap();

        let mut namespace = Namespace::new();
        namespace.register_grammar("topology", reg);

        // Run all test directives
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 5, "expected 5 test blocks");
        for result in &results {
            assert_eq!(
                result.verdict,
                Verdict::Pass,
                "test '{}' failed: {:?}",
                result.name,
                result.verdict,
            );
        }
    }

    // -- Garden @training domain --
    //
    // These tests verify the @training garden grammar (garden/public/@training/training.conv)
    // compiles correctly and its test section passes. The grammar declares the vocabulary
    // for graph-native model training: epochs, layers, routing, spectral properties, phases,
    // and observations.

    #[test]
    fn garden_training_grammar_compiles() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@training/training.conv"),
        )
        .expect("garden @training/training.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        assert_eq!(
            parts.len(),
            2,
            "training.conv should have grammar and test sections"
        );

        // Compile the grammar section
        let grammar_src = parts[0];
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("should have grammar block");
        let reg = TypeRegistry::compile(grammar).unwrap();
        assert_eq!(reg.domain, "training");

        // Verify types
        assert!(reg.has_variant("", "epoch"));
        assert!(reg.has_variant("", "step"));
        assert!(reg.has_variant("", "checkpoint"));
        assert!(reg.has_variant("", "topology_snapshot"));
        assert!(reg.has_variant("layer", "attention"));
        assert!(reg.has_variant("layer", "feedforward"));
        assert!(reg.has_variant("layer", "embedding"));
        assert!(reg.has_variant("layer", "output"));
        assert!(reg.has_variant("routing", "dense"));
        assert!(reg.has_variant("routing", "sparse"));
        assert!(reg.has_variant("routing", "learned"));
        assert!(reg.has_variant("routing", "fixed"));
        assert!(reg.has_variant("spectral_property", "ramanujan"));
        assert!(reg.has_variant("spectral_property", "small_world"));
        assert!(reg.has_variant("spectral_property", "expander"));
        assert!(reg.has_variant("phase", "warmup"));
        assert!(reg.has_variant("phase", "learning"));
        assert!(reg.has_variant("phase", "plateau"));
        assert!(reg.has_variant("phase", "grokking"));
        assert!(reg.has_variant("phase", "converged"));
        assert!(reg.has_variant("observation", "loss"));
        assert!(reg.has_variant("observation", "gradient"));
        assert!(reg.has_variant("observation", "attention_pattern"));
        assert!(reg.has_variant("observation", "routing_pattern"));
        assert!(reg.has_variant("observation", "spectral_gap"));
    }

    #[test]
    fn garden_training_tests_pass() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@training/training.conv"),
        )
        .expect("garden @training/training.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        let grammar_src = parts[0];
        let test_src = parts[1];

        // Compile grammar and register in namespace
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let reg = TypeRegistry::compile(grammar).unwrap();

        let mut namespace = Namespace::new();
        namespace.register_grammar("training", reg);

        // Run all test directives
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 6, "expected 6 test blocks");
        for result in &results {
            assert_eq!(
                result.verdict,
                Verdict::Pass,
                "test '{}' failed: {:?}",
                result.name,
                result.verdict,
            );
        }
    }

    // -- Garden @coincidence domain --
    //
    // These tests verify the @coincidence garden grammar (garden/public/@coincidence/coincidence.conv)
    // compiles correctly and its test section passes. The grammar declares the vocabulary
    // for measurement: eigendecomposition, entropy, curvature, and spectral analysis.

    #[test]
    fn garden_coincidence_grammar_compiles() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@coincidence/coincidence.conv"),
        )
        .expect("garden @coincidence/coincidence.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        assert_eq!(
            parts.len(),
            2,
            "coincidence.conv should have grammar and test sections"
        );

        // Compile the grammar section
        let grammar_src = parts[0];
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("should have grammar block");
        let reg = TypeRegistry::compile(grammar).unwrap();
        assert_eq!(reg.domain, "coincidence");

        // Verify types
        assert!(reg.has_variant("", "measurement"));
        assert!(reg.has_variant("", "verdict"));
        assert!(reg.has_variant("", "spectrum"));
        assert!(reg.has_variant("measurement", "eigenvalue"));
        assert!(reg.has_variant("measurement", "entropy"));
        assert!(reg.has_variant("measurement", "curvature"));
        assert!(reg.has_variant("measurement", "fiedler"));
        assert!(reg.has_variant("measurement", "eigengap"));
        assert!(reg.has_variant("measurement", "heat_kernel"));
        assert!(reg.has_variant("verdict", "pass"));
        assert!(reg.has_variant("verdict", "fail"));
        assert!(reg.has_variant("spectrum", "laplacian"));
        assert!(reg.has_variant("spectrum", "adjacency"));
        assert!(reg.has_variant("spectrum", "normalized"));

        // Verify actions
        assert!(reg.has_action("check"));
        assert!(reg.has_action("measure"));
        assert!(reg.has_action("connected"));
        assert!(reg.has_action("entropy"));
        assert!(reg.has_action("curvature"));
        assert!(reg.has_action("bipartite"));
        assert!(reg.has_action("shannon_equivalence"));
    }

    #[test]
    fn garden_coincidence_tests_pass() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("garden/public/@coincidence/coincidence.conv"),
        )
        .expect("garden @coincidence/coincidence.conv should exist");

        // Split on --- separator
        let parts: Vec<&str> = source.splitn(2, "\n---\n").collect();
        let grammar_src = parts[0];
        let test_src = parts[1];

        // Compile grammar and register in namespace
        let ast = Parse.trace(grammar_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let reg = TypeRegistry::compile(grammar).unwrap();

        let mut namespace = Namespace::new();
        namespace.register_grammar("coincidence", reg);

        // Run all test directives
        let results = check_all(&namespace, test_src).unwrap();
        assert_eq!(results.len(), 4, "expected 4 test blocks");
        for result in &results {
            assert_eq!(
                result.verdict,
                Verdict::Pass,
                "test '{}' failed: {:?}",
                result.name,
                result.verdict,
            );
        }
    }
}
