//! Logic — the TypeRegistry as an explicit logic program.
//!
//! The TypeRegistry IS a Datalog fact store. This module makes that
//! interpretation explicit by exposing the type surface as queryable
//! facts and Horn clauses. Compilation succeeding is a satisfiability
//! proof. The OID is the proof certificate.
//!
//! # The model-checking interpretation
//!
//! - `types` map → unary facts: `type_has_variant(type_name, variant)`
//! - `params` map → implications: `variant_refs(type_name, variant) :- type_exists(ref_type)`
//! - `acts` map → action facts: `action_field(action_name, field_name, type_ref)`
//! - `calls` map → cross-domain implications: `action_calls(action, target_domain, target_action)`
//!
//! Because the grammar is NOT Turing-complete (finite types, no recursion,
//! no unbounded computation), Rice's theorem does not apply. Verification
//! is decidable. The type checker is a model checker.
//!
//! # Mercury's determinism hierarchy
//!
//! Mercury classifies predicates by solution cardinality:
//! - `det`     — exactly 1 solution  → Iso (bijection)
//! - `semidet` — 0 or 1 solutions   → Prism (partial match)
//! - `multi`   — 1+ solutions        → Traversal (at least one)
//! - `nondet`  — 0+ solutions        → AffineTraversal (maybe none, maybe many)
//!
//! This maps exactly to the optics hierarchy in the framework crate.

use std::collections::{BTreeSet, HashMap, VecDeque};

use crate::resolve::TypeRegistry;

// ---------------------------------------------------------------------------
// Fact — a ground truth in the grammar's logic program
// ---------------------------------------------------------------------------

/// A fact in the grammar's logic program.
///
/// These are the atoms of the Datalog interpretation. Every fact
/// is a ground term — no variables, no unification. The grammar
/// produces exactly these facts and no others.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Fact {
    /// `type_exists(domain, type_name)` — a named type is declared.
    TypeExists { domain: String, type_name: String },

    /// `type_has_variant(domain, type_name, variant)` — a type includes this variant.
    TypeHasVariant {
        domain: String,
        type_name: String,
        variant: String,
    },

    /// `variant_refs(domain, type_name, variant, ref_type)` — a variant's parameter
    /// references another type. This is an implication: the ref_type must exist.
    VariantRefs {
        domain: String,
        type_name: String,
        variant: String,
        ref_type: String,
    },

    /// `action_exists(domain, action_name)` — an action is declared.
    ActionExists { domain: String, action_name: String },

    /// `action_field(domain, action_name, field_name, type_ref)` — an action has a field.
    ActionField {
        domain: String,
        action_name: String,
        field_name: String,
        type_ref: Option<String>,
    },

    /// `action_calls(domain, action_name, target_domain, target_action)` — cross-domain call.
    ActionCalls {
        domain: String,
        action_name: String,
        target_domain: String,
        target_action: String,
    },
}

impl Fact {
    /// The domain this fact belongs to.
    pub fn domain(&self) -> &str {
        match self {
            Fact::TypeExists { domain, .. }
            | Fact::TypeHasVariant { domain, .. }
            | Fact::VariantRefs { domain, .. }
            | Fact::ActionExists { domain, .. }
            | Fact::ActionField { domain, .. }
            | Fact::ActionCalls { domain, .. } => domain,
        }
    }
}

// ---------------------------------------------------------------------------
// FactStore — extract facts from a TypeRegistry
// ---------------------------------------------------------------------------

/// A set of ground facts extracted from one or more TypeRegistries.
///
/// This is the Datalog fact store. You can query it.
#[derive(Clone, Debug, Default)]
pub struct FactStore {
    facts: BTreeSet<Fact>,
}

impl FactStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract all facts from a TypeRegistry.
    pub fn from_registry(registry: &TypeRegistry) -> Self {
        let mut store = Self::new();
        store.add_registry(registry);
        store
    }

    /// Add all facts from a TypeRegistry to this store.
    pub fn add_registry(&mut self, registry: &TypeRegistry) {
        let domain = &registry.domain;

        // Type facts
        for type_name in registry.type_names() {
            self.facts.insert(Fact::TypeExists {
                domain: domain.clone(),
                type_name: type_name.to_string(),
            });

            for variant in registry.variants(type_name).unwrap_or_default() {
                self.facts.insert(Fact::TypeHasVariant {
                    domain: domain.clone(),
                    type_name: type_name.to_string(),
                    variant: variant.to_string(),
                });

                // Check for parameterized variant refs
                if let Some(ref_type) = registry.variant_param(type_name, variant) {
                    self.facts.insert(Fact::VariantRefs {
                        domain: domain.clone(),
                        type_name: type_name.to_string(),
                        variant: variant.to_string(),
                        ref_type: ref_type.to_string(),
                    });
                }
            }
        }

        // Action facts
        for action_name in registry.act_names() {
            self.facts.insert(Fact::ActionExists {
                domain: domain.clone(),
                action_name: action_name.to_string(),
            });

            for (field_name, type_ref) in registry.action_fields(action_name).unwrap_or(&[]) {
                self.facts.insert(Fact::ActionField {
                    domain: domain.clone(),
                    action_name: action_name.to_string(),
                    field_name: field_name.clone(),
                    type_ref: type_ref.clone(),
                });
            }

            // Cross-domain calls
            for (target_domain, target_action, _args) in registry.action_calls(action_name) {
                self.facts.insert(Fact::ActionCalls {
                    domain: domain.clone(),
                    action_name: action_name.to_string(),
                    target_domain: target_domain.clone(),
                    target_action: target_action.clone(),
                });
            }
        }
    }

    /// All facts in the store.
    pub fn facts(&self) -> &BTreeSet<Fact> {
        &self.facts
    }

    /// Number of facts.
    pub fn len(&self) -> usize {
        self.facts.len()
    }

    /// True when the store has no facts.
    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    /// Query: all types in a domain.
    pub fn types_in(&self, domain: &str) -> Vec<&str> {
        self.facts
            .iter()
            .filter_map(|f| match f {
                Fact::TypeExists {
                    domain: d,
                    type_name,
                } if d == domain => Some(type_name.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Query: all variants of a type in a domain.
    pub fn variants_of(&self, domain: &str, type_name: &str) -> Vec<&str> {
        self.facts
            .iter()
            .filter_map(|f| match f {
                Fact::TypeHasVariant {
                    domain: d,
                    type_name: t,
                    variant,
                } if d == domain && t == type_name => Some(variant.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Query: all actions in a domain.
    pub fn actions_in(&self, domain: &str) -> Vec<&str> {
        self.facts
            .iter()
            .filter_map(|f| match f {
                Fact::ActionExists {
                    domain: d,
                    action_name,
                } if d == domain => Some(action_name.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Query: all cross-domain calls from a domain.
    pub fn calls_from(&self, domain: &str) -> Vec<(&str, &str)> {
        self.facts
            .iter()
            .filter_map(|f| match f {
                Fact::ActionCalls {
                    domain: d,
                    target_domain,
                    target_action,
                    ..
                } if d == domain => Some((target_domain.as_str(), target_action.as_str())),
                _ => None,
            })
            .collect()
    }

    /// Query: all domains that depend on a given domain (call into it).
    pub fn dependents_of(&self, target_domain: &str) -> Vec<&str> {
        let mut dependents: BTreeSet<&str> = BTreeSet::new();
        for fact in &self.facts {
            if let Fact::ActionCalls {
                domain,
                target_domain: td,
                ..
            } = fact
            {
                if td == target_domain {
                    dependents.insert(domain);
                }
            }
        }
        dependents.into_iter().collect()
    }
}

// ---------------------------------------------------------------------------
// ProofCertificate — structured proof beyond bare OIDs
// ---------------------------------------------------------------------------

/// A proof certificate for a successful grammar compilation.
///
/// Not just an OID — a structured chain showing what was proven:
/// which types were checked, which references were validated,
/// which actions were verified. The OID is the hash of this
/// chain, making it a commitment to the full proof.
#[derive(Clone, Debug, PartialEq)]
pub struct ProofCertificate {
    /// The domain that was compiled.
    pub domain: String,
    /// Facts that constitute the proof.
    pub facts: BTreeSet<Fact>,
    /// Obligations that were discharged (type refs that were validated).
    pub discharged: Vec<Obligation>,
    /// The proof hash — content address of the entire certificate.
    pub proof_oid: crate::Oid,
}

/// An obligation that compilation discharged.
///
/// "variant X references type Y" → obligation: Y must exist.
/// The compiler checked this. The certificate records that it did.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Obligation {
    /// What needed to be true.
    pub requirement: String,
    /// What satisfied it.
    pub evidence: String,
}

impl ProofCertificate {
    /// Build a proof certificate from a compiled TypeRegistry.
    pub fn from_registry(registry: &TypeRegistry) -> Self {
        let store = FactStore::from_registry(registry);
        let facts = store.facts().clone();

        // Collect discharged obligations: every VariantRefs fact was validated
        let mut discharged = Vec::new();
        for fact in &facts {
            if let Fact::VariantRefs {
                type_name,
                variant,
                ref_type,
                ..
            } = fact
            {
                discharged.push(Obligation {
                    requirement: format!(
                        "variant \"{}\" in type \"{}\" references type \"{}\"",
                        variant, type_name, ref_type
                    ),
                    evidence: format!("type \"{}\" exists in grammar", ref_type),
                });
            }
        }
        discharged.sort();

        // The proof OID hashes the facts + obligations
        let mut hasher = crate::Oid::hasher();
        hasher.update(b"proof:");
        hasher.update(registry.domain.as_bytes());
        for fact in &facts {
            hasher.update(format!("{:?}", fact).as_bytes());
        }
        for ob in &discharged {
            hasher.update(ob.requirement.as_bytes());
            hasher.update(ob.evidence.as_bytes());
        }
        let proof_oid = hasher.finalize();

        ProofCertificate {
            domain: registry.domain.clone(),
            facts,
            discharged,
            proof_oid,
        }
    }
}

// ---------------------------------------------------------------------------
// Reachability — enumerate the state space from declarations
// ---------------------------------------------------------------------------

/// The reachable state space of a grammar.
///
/// Starting from any type, which variants are reachable through
/// parameterized references? This is the transitive closure of
/// the type reference graph. For a well-formed grammar, every
/// referenced type is reachable.
#[derive(Clone, Debug)]
pub struct ReachabilityMap {
    /// type_name → set of reachable type names (transitive closure).
    reachable: HashMap<String, BTreeSet<String>>,
    /// All types that are referenced but never declared.
    /// Should be empty for a compiled grammar (the compiler validates this).
    unreachable: BTreeSet<String>,
}

impl ReachabilityMap {
    /// Compute the reachable state space from a TypeRegistry.
    pub fn from_registry(registry: &TypeRegistry) -> Self {
        // Build the type reference graph
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let declared: BTreeSet<String> = registry
            .type_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        for type_name in registry.type_names() {
            for variant in registry.variants(type_name).unwrap_or_default() {
                if let Some(ref_type) = registry.variant_param(type_name, variant) {
                    edges
                        .entry(type_name.to_string())
                        .or_default()
                        .push(ref_type.to_string());
                }
            }
        }

        // Compute transitive closure via BFS from each type
        let mut reachable: HashMap<String, BTreeSet<String>> = HashMap::new();
        let mut all_referenced: BTreeSet<String> = BTreeSet::new();

        for start_type in &declared {
            let mut visited = BTreeSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start_type.clone());

            while let Some(current) = queue.pop_front() {
                if !visited.insert(current.clone()) {
                    continue;
                }
                if let Some(targets) = edges.get(&current) {
                    for target in targets {
                        all_referenced.insert(target.clone());
                        queue.push_back(target.clone());
                    }
                }
            }

            visited.remove(start_type);
            reachable.insert(start_type.clone(), visited);
        }

        let unreachable: BTreeSet<String> = all_referenced.difference(&declared).cloned().collect();

        ReachabilityMap {
            reachable,
            unreachable,
        }
    }

    /// Types reachable from the given starting type (transitive closure).
    pub fn reachable_from(&self, type_name: &str) -> Option<&BTreeSet<String>> {
        self.reachable.get(type_name)
    }

    /// Types that are referenced but never declared.
    /// Empty for valid compiled grammars.
    pub fn unreachable(&self) -> &BTreeSet<String> {
        &self.unreachable
    }

    /// True if all referenced types are declared.
    pub fn is_complete(&self) -> bool {
        self.unreachable.is_empty()
    }

    /// Total number of types in the grammar.
    pub fn type_count(&self) -> usize {
        self.reachable.len()
    }

    /// The total number of reachability edges (sum of all reachable sets).
    pub fn edge_count(&self) -> usize {
        self.reachable.values().map(|s| s.len()).sum()
    }
}

// ---------------------------------------------------------------------------
// Determinism — Mercury-style classification of type surface
// ---------------------------------------------------------------------------

/// Mercury determinism classification for a type lookup.
///
/// Maps the optics hierarchy:
/// - Det     → Iso (exactly 1 variant — the type IS its only variant)
/// - Semidet → Prism (0 or 1 — the type may or may not match)
/// - Multi   → Traversal (1+ — always at least one match)
/// - Nondet  → AffineTraversal (0+ — unconstrained)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Determinism {
    /// Exactly 1 variant. `det` in Mercury. Iso in optics.
    Det,
    /// 0 or 1 variants. `semidet` in Mercury. Prism in optics.
    Semidet,
    /// 1+ variants. `multi` in Mercury. Traversal in optics.
    Multi,
    /// 0+ variants (type doesn't exist or has 0 variants).
    /// `nondet` in Mercury. AffineTraversal in optics.
    Nondet,
}

impl Determinism {
    /// Classify a type lookup in the TypeRegistry.
    ///
    /// The classification depends on the number of variants:
    /// - Type not found → Nondet (0+ = unknown)
    /// - 0 variants → Semidet (type exists but empty)
    /// - 1 variant → Det (exactly determined)
    /// - 2+ variants → Multi (multiple solutions)
    pub fn classify(registry: &TypeRegistry, type_name: &str) -> Self {
        match registry.variants(type_name) {
            None => Determinism::Nondet,
            Some(variants) => match variants.len() {
                0 => Determinism::Semidet,
                1 => Determinism::Det,
                _ => Determinism::Multi,
            },
        }
    }

    /// Classify a type lookup in a FactStore (cross-domain).
    pub fn classify_in_store(store: &FactStore, domain: &str, type_name: &str) -> Self {
        let variants = store.variants_of(domain, type_name);
        match variants.len() {
            0 => {
                // Check if the type exists at all
                if store.types_in(domain).contains(&type_name) {
                    Determinism::Semidet
                } else {
                    Determinism::Nondet
                }
            }
            1 => Determinism::Det,
            _ => Determinism::Multi,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Span};
    use crate::domain::conversation::Kind;
    use crate::prism::{self, Prism};
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    // -- helpers --

    fn span() -> Span {
        Span::new(0, 0)
    }

    fn ref_(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    /// Build a minimal grammar AST for testing.
    /// `grammar @test { type = a | b | c  type op = gt | lt }`
    fn test_grammar() -> Prism<AstNode> {
        let variant_a = prism::shard(
            ref_("variant-a"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "a".into(),
                span: span(),
            },
        );
        let variant_b = prism::shard(
            ref_("variant-b"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "b".into(),
                span: span(),
            },
        );
        let variant_c = prism::shard(
            ref_("variant-c"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "c".into(),
                span: span(),
            },
        );
        let default_type = prism::fractal(
            ref_("type-def-default"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "".into(),
                span: span(),
            },
            vec![variant_a, variant_b, variant_c],
        );

        let variant_gt = prism::shard(
            ref_("variant-gt"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "gt".into(),
                span: span(),
            },
        );
        let variant_lt = prism::shard(
            ref_("variant-lt"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "lt".into(),
                span: span(),
            },
        );
        let op_type = prism::fractal(
            ref_("type-def-op"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "op".into(),
                span: span(),
            },
            vec![variant_gt, variant_lt],
        );

        // Action with a field
        let field_node = prism::shard(
            ref_("field-source"),
            AstNode {
                kind: Kind::Atom,
                name: "field".into(),
                value: "source".into(),
                span: span(),
            },
        );
        let action_def = prism::fractal(
            ref_("action-compile"),
            AstNode {
                kind: Kind::Form,
                name: "action-def".into(),
                value: "compile".into(),
                span: span(),
            },
            vec![field_node],
        );

        prism::fractal(
            ref_("grammar-test"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@test".into(),
                span: span(),
            },
            vec![default_type, op_type, action_def],
        )
    }

    /// Build a grammar with parameterized variants for reachability testing.
    /// `grammar @linked { type color = red(shade) | blue  type shade = light | dark }`
    fn linked_grammar() -> Prism<AstNode> {
        // shade type
        let variant_light = prism::shard(
            ref_("variant-light"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "light".into(),
                span: span(),
            },
        );
        let variant_dark = prism::shard(
            ref_("variant-dark"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "dark".into(),
                span: span(),
            },
        );
        let shade_type = prism::fractal(
            ref_("type-def-shade"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "shade".into(),
                span: span(),
            },
            vec![variant_light, variant_dark],
        );

        // color type with parameterized red(shade)
        let type_ref_shade = prism::shard(
            ref_("type-ref-shade"),
            AstNode {
                kind: Kind::Ref,
                name: "type-ref".into(),
                value: "shade".into(),
                span: span(),
            },
        );
        let variant_red = prism::fractal(
            ref_("variant-red"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "red".into(),
                span: span(),
            },
            vec![type_ref_shade],
        );
        let variant_blue = prism::shard(
            ref_("variant-blue"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "blue".into(),
                span: span(),
            },
        );
        let color_type = prism::fractal(
            ref_("type-def-color"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "color".into(),
                span: span(),
            },
            vec![variant_red, variant_blue],
        );

        prism::fractal(
            ref_("grammar-linked"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@linked".into(),
                span: span(),
            },
            vec![color_type, shade_type],
        )
    }

    /// Build a grammar with a cross-domain action call.
    /// `grammar @caller { action invoke { @target.run(arg1) } }`
    fn calling_grammar() -> Prism<AstNode> {
        let arg_node = prism::shard(
            ref_("arg-arg1"),
            AstNode {
                kind: Kind::Atom,
                name: "arg".into(),
                value: "arg1".into(),
                span: span(),
            },
        );
        let call_node = prism::fractal(
            ref_("action-call-target-run"),
            AstNode {
                kind: Kind::Ref,
                name: "action-call".into(),
                value: "@target.run".into(),
                span: span(),
            },
            vec![arg_node],
        );
        let action_def = prism::fractal(
            ref_("action-invoke"),
            AstNode {
                kind: Kind::Form,
                name: "action-def".into(),
                value: "invoke".into(),
                span: span(),
            },
            vec![call_node],
        );

        prism::fractal(
            ref_("grammar-caller"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@caller".into(),
                span: span(),
            },
            vec![action_def],
        )
    }

    // -----------------------------------------------------------------------
    // FactStore tests
    // -----------------------------------------------------------------------

    #[test]
    fn fact_store_extracts_type_facts() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);

        // Should have TypeExists facts
        let types = store.types_in("test");
        assert!(types.contains(&""), "should have default type");
        assert!(types.contains(&"op"), "should have op type");
    }

    #[test]
    fn fact_store_extracts_variant_facts() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);

        let default_variants = store.variants_of("test", "");
        assert!(default_variants.contains(&"a"));
        assert!(default_variants.contains(&"b"));
        assert!(default_variants.contains(&"c"));

        let op_variants = store.variants_of("test", "op");
        assert!(op_variants.contains(&"gt"));
        assert!(op_variants.contains(&"lt"));
    }

    #[test]
    fn fact_store_extracts_action_facts() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);

        let actions = store.actions_in("test");
        assert!(actions.contains(&"compile"));
    }

    #[test]
    fn fact_store_extracts_variant_refs() {
        let registry = TypeRegistry::compile(&linked_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);

        // color.red references shade
        let has_ref = store.facts().iter().any(|f| {
            matches!(f, Fact::VariantRefs {
                domain,
                type_name,
                variant,
                ref_type,
            } if domain == "linked" && type_name == "color" && variant == "red" && ref_type == "shade")
        });
        assert!(has_ref, "should have VariantRefs fact for red(shade)");
    }

    #[test]
    fn fact_store_extracts_cross_domain_calls() {
        let registry = TypeRegistry::compile(&calling_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);

        let calls = store.calls_from("caller");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("target", "run"));
    }

    #[test]
    fn fact_store_dependents() {
        let caller_reg = TypeRegistry::compile(&calling_grammar()).unwrap();
        let mut store = FactStore::new();
        store.add_registry(&caller_reg);

        let deps = store.dependents_of("target");
        assert!(deps.contains(&"caller"));
    }

    #[test]
    fn fact_store_empty() {
        let store = FactStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn fact_store_len() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);
        assert!(store.len() > 0);
        assert!(!store.is_empty());
    }

    #[test]
    fn fact_domain_accessor() {
        let fact = Fact::TypeExists {
            domain: "test".into(),
            type_name: "op".into(),
        };
        assert_eq!(fact.domain(), "test");
    }

    #[test]
    fn fact_domain_accessor_all_variants() {
        // Exercise domain() on each Fact variant for coverage
        let facts = vec![
            Fact::TypeExists {
                domain: "a".into(),
                type_name: "t".into(),
            },
            Fact::TypeHasVariant {
                domain: "b".into(),
                type_name: "t".into(),
                variant: "v".into(),
            },
            Fact::VariantRefs {
                domain: "c".into(),
                type_name: "t".into(),
                variant: "v".into(),
                ref_type: "r".into(),
            },
            Fact::ActionExists {
                domain: "d".into(),
                action_name: "a".into(),
            },
            Fact::ActionField {
                domain: "e".into(),
                action_name: "a".into(),
                field_name: "f".into(),
                type_ref: None,
            },
            Fact::ActionCalls {
                domain: "f".into(),
                action_name: "a".into(),
                target_domain: "g".into(),
                target_action: "h".into(),
            },
        ];
        let expected_domains = ["a", "b", "c", "d", "e", "f"];
        for (fact, expected) in facts.iter().zip(expected_domains.iter()) {
            assert_eq!(fact.domain(), *expected);
        }
    }

    // -----------------------------------------------------------------------
    // ProofCertificate tests
    // -----------------------------------------------------------------------

    #[test]
    fn proof_certificate_simple_grammar() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let cert = ProofCertificate::from_registry(&registry);

        assert_eq!(cert.domain, "test");
        assert!(!cert.facts.is_empty());
        // No parameterized variants → no obligations
        assert!(cert.discharged.is_empty());
    }

    #[test]
    fn proof_certificate_with_obligations() {
        let registry = TypeRegistry::compile(&linked_grammar()).unwrap();
        let cert = ProofCertificate::from_registry(&registry);

        assert_eq!(cert.domain, "linked");
        assert!(!cert.discharged.is_empty());
        // red(shade) → obligation: shade exists
        assert!(cert
            .discharged
            .iter()
            .any(|o| o.requirement.contains("shade")));
    }

    #[test]
    fn proof_certificate_deterministic() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let cert1 = ProofCertificate::from_registry(&registry);
        let cert2 = ProofCertificate::from_registry(&registry);

        assert_eq!(cert1.proof_oid, cert2.proof_oid);
        assert_eq!(cert1.facts, cert2.facts);
    }

    #[test]
    fn proof_certificate_different_grammars_differ() {
        let reg1 = TypeRegistry::compile(&test_grammar()).unwrap();
        let reg2 = TypeRegistry::compile(&linked_grammar()).unwrap();
        let cert1 = ProofCertificate::from_registry(&reg1);
        let cert2 = ProofCertificate::from_registry(&reg2);

        assert_ne!(cert1.proof_oid, cert2.proof_oid);
    }

    // -----------------------------------------------------------------------
    // ReachabilityMap tests
    // -----------------------------------------------------------------------

    #[test]
    fn reachability_no_references() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let reach = ReachabilityMap::from_registry(&registry);

        assert!(reach.is_complete());
        assert_eq!(reach.edge_count(), 0);
    }

    #[test]
    fn reachability_with_references() {
        let registry = TypeRegistry::compile(&linked_grammar()).unwrap();
        let reach = ReachabilityMap::from_registry(&registry);

        assert!(reach.is_complete());
        // color → shade (via red(shade))
        let from_color = reach.reachable_from("color").unwrap();
        assert!(from_color.contains("shade"));
    }

    #[test]
    fn reachability_type_count() {
        let registry = TypeRegistry::compile(&linked_grammar()).unwrap();
        let reach = ReachabilityMap::from_registry(&registry);

        assert_eq!(reach.type_count(), 2); // color, shade
    }

    #[test]
    fn reachability_nonexistent_type() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let reach = ReachabilityMap::from_registry(&registry);

        assert!(reach.reachable_from("nonexistent").is_none());
    }

    #[test]
    fn reachability_unreachable_empty_for_valid() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let reach = ReachabilityMap::from_registry(&registry);

        assert!(reach.unreachable().is_empty());
    }

    #[test]
    fn reachability_diamond_deduplicates() {
        // grammar @diamond {
        //   type container = box(inner) | bag(inner)
        //   type inner = leaf
        // }
        // Both box and bag reference inner → BFS from container pushes inner twice.
        // Second visit should be deduplicated.
        let ref_inner_1 = prism::shard(
            ref_("type-ref-inner-1"),
            AstNode {
                kind: Kind::Ref,
                name: "type-ref".into(),
                value: "inner".into(),
                span: span(),
            },
        );
        let variant_box = prism::fractal(
            ref_("variant-box"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "box".into(),
                span: span(),
            },
            vec![ref_inner_1],
        );
        let ref_inner_2 = prism::shard(
            ref_("type-ref-inner-2"),
            AstNode {
                kind: Kind::Ref,
                name: "type-ref".into(),
                value: "inner".into(),
                span: span(),
            },
        );
        let variant_bag = prism::fractal(
            ref_("variant-bag"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "bag".into(),
                span: span(),
            },
            vec![ref_inner_2],
        );
        let container_type = prism::fractal(
            ref_("type-def-container"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "container".into(),
                span: span(),
            },
            vec![variant_box, variant_bag],
        );

        let variant_leaf = prism::shard(
            ref_("variant-leaf"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "leaf".into(),
                span: span(),
            },
        );
        let inner_type = prism::fractal(
            ref_("type-def-inner"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "inner".into(),
                span: span(),
            },
            vec![variant_leaf],
        );

        let grammar = prism::fractal(
            ref_("grammar-diamond"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@diamond".into(),
                span: span(),
            },
            vec![container_type, inner_type],
        );
        let registry = TypeRegistry::compile(&grammar).unwrap();
        let reach = ReachabilityMap::from_registry(&registry);

        assert!(reach.is_complete());
        let from_container = reach.reachable_from("container").unwrap();
        assert!(from_container.contains("inner"));
        // inner is only reachable once despite two edges
        assert_eq!(from_container.len(), 1);
    }

    // -----------------------------------------------------------------------
    // Determinism tests
    // -----------------------------------------------------------------------

    #[test]
    fn determinism_det_single_variant() {
        // Build a grammar with exactly 1 variant
        let variant = prism::shard(
            ref_("variant-only"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "only".into(),
                span: span(),
            },
        );
        let type_def = prism::fractal(
            ref_("type-def-singular"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "singular".into(),
                span: span(),
            },
            vec![variant],
        );
        let grammar = prism::fractal(
            ref_("grammar-det"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@det".into(),
                span: span(),
            },
            vec![type_def],
        );
        let registry = TypeRegistry::compile(&grammar).unwrap();
        assert_eq!(
            Determinism::classify(&registry, "singular"),
            Determinism::Det
        );
    }

    #[test]
    fn determinism_multi_variants() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        // Default type has 3 variants (a, b, c)
        assert_eq!(Determinism::classify(&registry, ""), Determinism::Multi);
        // Op type has 2 variants (gt, lt)
        assert_eq!(Determinism::classify(&registry, "op"), Determinism::Multi);
    }

    #[test]
    fn determinism_nondet_missing_type() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        assert_eq!(
            Determinism::classify(&registry, "nonexistent"),
            Determinism::Nondet
        );
    }

    #[test]
    fn determinism_semidet_empty_type() {
        // Build a grammar with an empty type (no variants)
        let type_def = prism::fractal(
            ref_("type-def-empty"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "empty".into(),
                span: span(),
            },
            vec![],
        );
        let grammar = prism::fractal(
            ref_("grammar-semi"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@semi".into(),
                span: span(),
            },
            vec![type_def],
        );
        let registry = TypeRegistry::compile(&grammar).unwrap();
        assert_eq!(
            Determinism::classify(&registry, "empty"),
            Determinism::Semidet
        );
    }

    #[test]
    fn determinism_classify_in_store() {
        let registry = TypeRegistry::compile(&test_grammar()).unwrap();
        let store = FactStore::from_registry(&registry);

        // Multi: default type has 3 variants
        assert_eq!(
            Determinism::classify_in_store(&store, "test", ""),
            Determinism::Multi
        );

        // Nondet: type doesn't exist
        assert_eq!(
            Determinism::classify_in_store(&store, "test", "nonexistent"),
            Determinism::Nondet
        );
    }

    #[test]
    fn determinism_det_in_store() {
        // Single variant type via store
        let variant = prism::shard(
            ref_("variant-solo"),
            AstNode {
                kind: Kind::Form,
                name: "variant".into(),
                value: "solo".into(),
                span: span(),
            },
        );
        let type_def = prism::fractal(
            ref_("type-def-solo"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "solo_type".into(),
                span: span(),
            },
            vec![variant],
        );
        let grammar = prism::fractal(
            ref_("grammar-solo"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@solo".into(),
                span: span(),
            },
            vec![type_def],
        );
        let registry = TypeRegistry::compile(&grammar).unwrap();
        let store = FactStore::from_registry(&registry);

        assert_eq!(
            Determinism::classify_in_store(&store, "solo", "solo_type"),
            Determinism::Det
        );
    }

    #[test]
    fn determinism_semidet_in_store() {
        // Empty type via store
        let type_def = prism::fractal(
            ref_("type-def-empty-s"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "empty".into(),
                span: span(),
            },
            vec![],
        );
        let grammar = prism::fractal(
            ref_("grammar-semi-s"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@semi".into(),
                span: span(),
            },
            vec![type_def],
        );
        let registry = TypeRegistry::compile(&grammar).unwrap();
        let store = FactStore::from_registry(&registry);

        assert_eq!(
            Determinism::classify_in_store(&store, "semi", "empty"),
            Determinism::Semidet
        );
    }

    // -----------------------------------------------------------------------
    // Multi-registry store tests
    // -----------------------------------------------------------------------

    #[test]
    fn multi_registry_store() {
        let reg1 = TypeRegistry::compile(&test_grammar()).unwrap();
        let reg2 = TypeRegistry::compile(&calling_grammar()).unwrap();

        let mut store = FactStore::new();
        store.add_registry(&reg1);
        store.add_registry(&reg2);

        // Both domains present
        assert!(!store.types_in("test").is_empty());
        assert!(!store.actions_in("caller").is_empty());
    }
}
