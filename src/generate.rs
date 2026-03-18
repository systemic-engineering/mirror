//! Grammar-derived generator. Exhaustive derivation from TypeRegistry.
//!
//! The grammar IS the generator. `type = a | b | c` means exactly {a, b, c}.
//! No randomness — full enumeration. The derivation space is finite.

use crate::ast::{self, AstNode, Span};
use crate::domain::conversation::Kind;
use crate::resolve::TypeRegistry;
use crate::prism::Prism;

/// A single derivation from a grammar type.
#[derive(Clone, Debug)]
pub struct Derivation {
    pub type_name: String,
    pub variant: String,
    pub tree: Prism<AstNode>,
}

/// Synthetic span for generated trees. Not from source — zero-width.
const GEN_SPAN: Span = Span { start: 0, end: 0 };

/// Derive all valid trees for a named type.
///
/// For `type = a | b | c`, produces 3 Derivation values.
/// Parameterized variants like `when(op)` expand recursively:
/// one derivation per (variant, param-variant) pair.
pub fn derive_type(registry: &TypeRegistry, type_name: &str) -> Vec<Derivation> {
    let variants = match registry.variants(type_name) {
        Some(vs) => vs,
        None => return Vec::new(),
    };

    let mut sorted: Vec<&str> = variants;
    sorted.sort();

    let mut derivations = Vec::new();
    for variant in sorted {
        if let Some(param_type) = registry.variant_param(type_name, variant) {
            // Parameterized: expand recursively
            let param_variants = match registry.variants(param_type) {
                Some(vs) => {
                    let mut vs = vs;
                    vs.sort();
                    vs
                }
                None => continue,
            };
            for pv in param_variants {
                let param_leaf = ast::ast_leaf(Kind::Form, "variant", pv, GEN_SPAN);
                let tree =
                    ast::ast_branch(Kind::Form, "variant", variant, GEN_SPAN, vec![param_leaf]);
                derivations.push(Derivation {
                    type_name: type_name.to_string(),
                    variant: format!("{}({})", variant, pv),
                    tree,
                });
            }
        } else {
            // Simple variant: leaf
            let tree = ast::ast_leaf(Kind::Form, "variant", variant, GEN_SPAN);
            derivations.push(Derivation {
                type_name: type_name.to_string(),
                variant: variant.to_string(),
                tree,
            });
        }
    }

    derivations
}

/// Derive all types in the grammar.
///
/// Iterates type names in sorted order, derives each, concatenates.
pub fn derive_all(registry: &TypeRegistry) -> Vec<Derivation> {
    let mut type_names = registry.type_names();
    type_names.sort();

    let mut all = Vec::new();
    for tn in type_names {
        all.extend(derive_type(registry, tn));
    }

    // Derive acts: one derivation per act with fields as children
    let mut act_names = registry.act_names();
    act_names.sort();
    for act_name in act_names {
        let fields = registry.action_fields(act_name).unwrap_or(&[]);
        let children: Vec<Prism<AstNode>> = fields
            .iter()
            .map(|(name, type_ref)| {
                let value = type_ref.as_deref().unwrap_or("");
                ast::ast_leaf(Kind::Atom, "field", name, GEN_SPAN)
                    // Attach type-ref child if present
                    ;
                if value.is_empty() {
                    ast::ast_leaf(Kind::Atom, "field", name, GEN_SPAN)
                } else {
                    let type_child = ast::ast_leaf(Kind::Ref, "type-ref", value, GEN_SPAN);
                    ast::ast_branch(Kind::Atom, "field", name, GEN_SPAN, vec![type_child])
                }
            })
            .collect();

        let tree = ast::ast_branch(Kind::Form, "act-def", act_name, GEN_SPAN, children);
        all.push(Derivation {
            type_name: format!("act:{}", act_name),
            variant: act_name.to_string(),
            tree,
        });
    }

    all
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parse;
    use crate::resolve::TypeRegistry;
    use crate::Vector;

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
    fn derive_simple_type() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let derivations = derive_type(&reg, "");
        assert_eq!(derivations.len(), 3);
        let variants: Vec<&str> = derivations.iter().map(|d| d.variant.as_str()).collect();
        assert_eq!(variants, vec!["a", "b", "c"]);
    }

    #[test]
    fn derive_named_type() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt | eq\n}\n");
        let derivations = derive_type(&reg, "op");
        assert_eq!(derivations.len(), 3);
        let variants: Vec<&str> = derivations.iter().map(|d| d.variant.as_str()).collect();
        assert_eq!(variants, vec!["eq", "gt", "lt"]);
    }

    #[test]
    fn derive_parameterized_expands() {
        let reg =
            compile_grammar("grammar @test {\n  type = plain | when(op)\n  type op = gt | lt\n}\n");
        let derivations = derive_type(&reg, "");
        // plain + when(gt) + when(lt) = 3
        assert_eq!(derivations.len(), 3);
        let variants: Vec<&str> = derivations.iter().map(|d| d.variant.as_str()).collect();
        assert_eq!(variants, vec!["plain", "when(gt)", "when(lt)"]);
    }

    #[test]
    fn derive_parameterized_tree_structure() {
        let reg = compile_grammar("grammar @test {\n  type = when(op)\n  type op = gt | lt\n}\n");
        let derivations = derive_type(&reg, "");
        let d = &derivations[0]; // when(gt)
        assert!(d.tree.is_fractal());
        assert_eq!(d.tree.data().value, "when");
        assert_eq!(d.tree.children().len(), 1);
        assert_eq!(d.tree.children()[0].data().value, "gt");
    }

    #[test]
    fn derive_missing_type_returns_empty() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert!(derive_type(&reg, "missing").is_empty());
    }

    #[test]
    fn derive_all_covers_all_types() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        let all = derive_all(&reg);
        // "" has 2, "op" has 2, no acts = 4
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn derive_all_includes_acts() {
        let reg = compile_grammar(
            "grammar @test {\n  type = a\n  action send {\n    to\n    subject\n  }\n}\n",
        );
        let all = derive_all(&reg);
        // 1 type variant + 1 act = 2
        assert_eq!(all.len(), 2);
        let act = all.iter().find(|d| d.type_name == "act:send").unwrap();
        assert_eq!(act.variant, "send");
        assert!(act.tree.is_fractal());
        assert_eq!(act.tree.children().len(), 2);
    }

    #[test]
    fn derive_all_act_with_typed_field() {
        let reg = compile_grammar(
            "grammar @test {\n  type address = email | uri\n  action send {\n    to: address\n  }\n}\n",
        );
        let all = derive_all(&reg);
        let act = all.iter().find(|d| d.type_name == "act:send").unwrap();
        let field = &act.tree.children()[0];
        assert_eq!(field.data().name, "field");
        assert_eq!(field.data().value, "to");
        // Typed field has a type-ref child
        assert!(field.is_fractal());
        assert_eq!(field.children()[0].data().value, "address");
    }

    #[test]
    fn derivations_are_content_addressed() {
        use crate::ContentAddressed;

        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let derivations = derive_type(&reg, "");

        // Different variants → different content OIDs
        let oid_a = derivations[0].tree.data().content_oid();
        let oid_b = derivations[1].tree.data().content_oid();
        assert_ne!(oid_a, oid_b);

        // Same derivation twice → same OID (deterministic)
        let derivations2 = derive_type(&reg, "");
        assert_eq!(
            derivations[0].tree.data().content_oid(),
            derivations2[0].tree.data().content_oid()
        );
    }

    #[test]
    fn derive_all_sorted_order() {
        let reg = compile_grammar("grammar @test {\n  type = z | a | m\n  type op = lt | gt\n}\n");
        let all = derive_all(&reg);
        let type_names: Vec<&str> = all.iter().map(|d| d.type_name.as_str()).collect();
        // "" comes before "op" (sorted); within each, variants sorted
        assert_eq!(type_names, vec!["", "", "", "op", "op"]);
        assert_eq!(all[0].variant, "a");
        assert_eq!(all[1].variant, "m");
        assert_eq!(all[2].variant, "z");
        assert_eq!(all[3].variant, "gt");
        assert_eq!(all[4].variant, "lt");
    }

    #[test]
    fn derive_parameterized_dangling_ref_skips() {
        // TypeRegistry with a parameterized variant whose type ref is NOT declared.
        // derive_type should skip it (None => continue).
        let reg = TypeRegistry::with_dangling_param("test", "", "when", "nonexistent");
        let derivations = derive_type(&reg, "");
        // The "when" variant has a param ref to "nonexistent" which has no variants,
        // so it is skipped entirely.
        assert!(derivations.is_empty());
    }
}
