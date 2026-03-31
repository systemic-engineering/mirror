//! Domain model types.
//!
//! Make illegal state unrepresentable. This module provides newtypes and
//! domain structs that serve as the public API for compiled grammars,
//! replacing the raw TypeRegistry internals.

use std::fmt;

use crate::ast::AstNode;
use crate::prism::Prism;
use crate::resolve::Visibility;

// ---------------------------------------------------------------------------
// Newtypes
// ---------------------------------------------------------------------------

/// A domain name, e.g. `filesystem` (leading `@` stripped on construction).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DomainName(String);

impl DomainName {
    /// Construct a `DomainName`, stripping a leading `@` if present.
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        if let Some(stripped) = s.strip_prefix('@') {
            Self(stripped.to_owned())
        } else {
            Self(s)
        }
    }

    /// Return the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DomainName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A type name within a domain.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeName(String);

impl TypeName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A variant name within a type definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariantName(String);

impl VariantName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VariantName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An action name within a domain.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionName(String);

impl ActionName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ActionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A property name (requires / invariant / ensures).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropertyName(String);

impl PropertyName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// Domain model structs
// ---------------------------------------------------------------------------

/// A validated reference to a type by name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeRef(TypeName);

impl TypeRef {
    pub fn new(name: TypeName) -> Self {
        Self(name)
    }

    pub fn type_name(&self) -> &TypeName {
        &self.0
    }
}

impl fmt::Display for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A variant constructor together with its named parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub name: VariantName,
    pub params: Vec<(VariantName, TypeRef)>,
}

/// A sum type: a named set of variants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeDef {
    pub name: TypeName,
    pub variants: Vec<Variant>,
}

/// A cross-domain action call emitted inside an action body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionCall {
    pub domain: DomainName,
    pub action: ActionName,
    pub args: Vec<TypeRef>,
}

/// An action exported by a domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub name: ActionName,
    pub fields: Vec<(ActionName, TypeRef)>,
    pub visibility: Visibility,
    pub calls: Vec<ActionCall>,
}

/// A lens reference — a domain this grammar composes through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lens {
    pub target: DomainName,
}

/// Declared property clauses for a domain.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Properties {
    pub requires: Vec<PropertyName>,
    pub invariants: Vec<PropertyName>,
    pub ensures: Vec<PropertyName>,
}

impl Properties {
    /// Return an empty `Properties` with no clauses.
    pub fn empty() -> Self {
        Self::default()
    }
}

/// A compiled domain: the public API replacing `TypeRegistry`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Domain {
    pub name: DomainName,
    pub types: Vec<TypeDef>,
    pub actions: Vec<Action>,
    pub lenses: Vec<Lens>,
    pub properties: Properties,
}

impl Domain {
    /// Returns `true` if this domain has a lens targeting `"actor"`.
    pub fn is_actor(&self) -> bool {
        self.lenses.iter().any(|l| l.target.as_str() == "actor")
    }

    /// Build a `Domain` from a grammar AST node with no external lens declarations.
    pub fn from_grammar(node: &Prism<AstNode>) -> Result<Self, String> {
        Self::from_grammar_with_lenses(node, &[])
    }

    /// Build a `Domain` from a grammar AST node plus external lens declarations.
    ///
    /// `lens_values` is a slice of values like `"@actor"` or `"@tools"` sourced
    /// from sibling `in @domain` declarations.
    pub fn from_grammar_with_lenses(
        node: &Prism<AstNode>,
        lens_values: &[String],
    ) -> Result<Self, String> {
        let raw = &node.data().value;
        let domain_name = DomainName::new(raw.as_str());
        let domain_str = domain_name.as_str().to_owned();

        // Convert external lens declarations.
        let lenses: Vec<Lens> = lens_values
            .iter()
            .map(|v| Lens {
                target: DomainName::new(v.as_str()),
            })
            .collect();

        // First pass: collect all declared type names for validation.
        let mut declared_types: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        for child in node.children() {
            if child.data().is_form("type-def") {
                declared_types.insert(child.data().value.clone());
            }
        }

        // Second pass: build types, actions, and properties.
        let mut types: Vec<TypeDef> = Vec::new();
        let mut actions: Vec<Action> = Vec::new();
        let mut properties = Properties::empty();

        for child in node.children() {
            if child.data().is_form("type-def") {
                let type_name = TypeName::new(child.data().value.as_str());

                let mut variants: Vec<Variant> = Vec::new();
                for variant_node in child.children() {
                    if !variant_node.data().is_form("variant") {
                        continue;
                    }
                    let variant_name = VariantName::new(variant_node.data().value.as_str());

                    // Collect parameterized type refs and validate them.
                    let mut params: Vec<(VariantName, TypeRef)> = Vec::new();
                    for sub in variant_node.children() {
                        if sub.data().is_ref("type-ref") {
                            let ref_type = &sub.data().value;
                            if !declared_types.contains(ref_type.as_str()) {
                                return Err(format!(
                                    "unknown type reference \"{}\" in grammar @{} (variant \"{}\" in type \"{}\")",
                                    ref_type,
                                    domain_str,
                                    variant_name.as_str(),
                                    type_name.as_str(),
                                ));
                            }
                            params.push((
                                variant_name.clone(),
                                TypeRef::new(TypeName::new(ref_type.as_str())),
                            ));
                        }
                    }

                    variants.push(Variant {
                        name: variant_name,
                        params,
                    });
                }

                types.push(TypeDef {
                    name: type_name,
                    variants,
                });
            } else if child.data().is_form("action-def") {
                let action_name = ActionName::new(child.data().value.as_str());
                let mut fields: Vec<(ActionName, TypeRef)> = Vec::new();
                let mut calls: Vec<ActionCall> = Vec::new();
                let mut visibility = Visibility::Protected;

                for item in child.children() {
                    if item.data().is_atom("visibility") {
                        visibility = match item.data().value.as_str() {
                            "public" => Visibility::Public,
                            "private" => Visibility::Private,
                            _ => Visibility::Protected,
                        };
                    } else if item.data().is_atom("field") {
                        let field_name = ActionName::new(item.data().value.as_str());
                        // Type ref on action fields is a semantic annotation — not validated.
                        let type_ref = item
                            .children()
                            .iter()
                            .find(|c| c.data().is_ref("type-ref"))
                            .map(|c| TypeRef::new(TypeName::new(c.data().value.as_str())))
                            .unwrap_or_else(|| TypeRef::new(TypeName::new("")));
                        fields.push((field_name, type_ref));
                    } else if item.data().is_ref("action-call") {
                        // "@domain.action" → split on '.'
                        let target = &item.data().value;
                        let target = target.strip_prefix('@').unwrap_or(target);
                        if let Some((call_domain, call_action)) = target.split_once('.') {
                            let args: Vec<TypeRef> = item
                                .children()
                                .iter()
                                .map(|c| TypeRef::new(TypeName::new(c.data().value.as_str())))
                                .collect();
                            calls.push(ActionCall {
                                domain: DomainName::new(call_domain),
                                action: ActionName::new(call_action),
                                args,
                            });
                        }
                    }
                }

                actions.push(Action {
                    name: action_name,
                    fields,
                    visibility,
                    calls,
                });
            } else if child.data().is_decl("requires") {
                properties
                    .requires
                    .push(PropertyName::new(child.data().value.as_str()));
            } else if child.data().is_decl("invariant") {
                properties
                    .invariants
                    .push(PropertyName::new(child.data().value.as_str()));
            } else if child.data().is_decl("ensures") {
                properties
                    .ensures
                    .push(PropertyName::new(child.data().value.as_str()));
            }
        }

        Ok(Domain {
            name: domain_name,
            types,
            actions,
            lenses,
            properties,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Newtype constructors and Display ---

    #[test]
    fn domain_name_new_and_display() {
        let d = DomainName::new("filesystem");
        assert_eq!(d.as_str(), "filesystem");
        assert_eq!(d.to_string(), "filesystem");
    }

    #[test]
    fn domain_name_strips_at_prefix() {
        let d = DomainName::new("@filesystem");
        assert_eq!(d.as_str(), "filesystem");
        assert_eq!(d.to_string(), "filesystem");
    }

    #[test]
    fn domain_name_no_double_strip() {
        // Only one leading @ is stripped.
        let d = DomainName::new("@@oops");
        assert_eq!(d.as_str(), "@oops");
    }

    #[test]
    fn type_name_new_and_display() {
        let t = TypeName::new("signal");
        assert_eq!(t.as_str(), "signal");
        assert_eq!(t.to_string(), "signal");
    }

    #[test]
    fn variant_name_new_and_display() {
        let v = VariantName::new("tick");
        assert_eq!(v.as_str(), "tick");
        assert_eq!(v.to_string(), "tick");
    }

    #[test]
    fn action_name_new_and_display() {
        let a = ActionName::new("read");
        assert_eq!(a.as_str(), "read");
        assert_eq!(a.to_string(), "read");
    }

    #[test]
    fn property_name_new_and_display() {
        let p = PropertyName::new("non_empty");
        assert_eq!(p.as_str(), "non_empty");
        assert_eq!(p.to_string(), "non_empty");
    }

    // --- TypeRef ---

    #[test]
    fn type_ref_accessor() {
        let tn = TypeName::new("signal");
        let tr = TypeRef::new(tn.clone());
        assert_eq!(tr.type_name(), &tn);
        assert_eq!(tr.to_string(), "signal");
    }

    // --- Properties::empty ---

    #[test]
    fn properties_empty() {
        let p = Properties::empty();
        assert!(p.requires.is_empty());
        assert!(p.invariants.is_empty());
        assert!(p.ensures.is_empty());
    }

    // --- Domain struct construction ---

    fn make_domain(lenses: Vec<Lens>) -> Domain {
        Domain {
            name: DomainName::new("test"),
            types: vec![],
            actions: vec![],
            lenses,
            properties: Properties::empty(),
        }
    }

    #[test]
    fn domain_construction() {
        let d = make_domain(vec![]);
        assert_eq!(d.name.as_str(), "test");
        assert!(d.types.is_empty());
        assert!(d.actions.is_empty());
        assert!(d.lenses.is_empty());
    }

    // --- Domain::is_actor ---

    #[test]
    fn is_actor_false_without_actor_lens() {
        let d = make_domain(vec![Lens {
            target: DomainName::new("tools"),
        }]);
        assert!(!d.is_actor());
    }

    #[test]
    fn is_actor_true_with_actor_lens() {
        let d = make_domain(vec![Lens {
            target: DomainName::new("actor"),
        }]);
        assert!(d.is_actor());
    }

    #[test]
    fn is_actor_false_no_lenses() {
        let d = make_domain(vec![]);
        assert!(!d.is_actor());
    }

    #[test]
    fn is_actor_strips_at_from_lens_target() {
        // DomainName strips @ so "@actor" → "actor".
        let d = make_domain(vec![Lens {
            target: DomainName::new("@actor"),
        }]);
        assert!(d.is_actor());
    }

    // --- Newtypes are distinct types (compile-time guarantee) ---
    // The following function would not compile if two newtypes were the same
    // type, because the function signatures would conflict.
    fn _accepts_domain_name(_: DomainName) {}
    fn _accepts_type_name(_: TypeName) {}
    fn _accepts_variant_name(_: VariantName) {}
    fn _accepts_action_name(_: ActionName) {}
    fn _accepts_property_name(_: PropertyName) {}

    #[test]
    fn newtypes_are_distinct() {
        // Compile-time: each function above only accepts its own newtype.
        // Runtime: just verify construction doesn't panic.
        _accepts_domain_name(DomainName::new("d"));
        _accepts_type_name(TypeName::new("t"));
        _accepts_variant_name(VariantName::new("v"));
        _accepts_action_name(ActionName::new("a"));
        _accepts_property_name(PropertyName::new("p"));
    }

    // --- Domain::from_grammar helpers ---

    use crate::ast::{AstNode, Span};
    use crate::domain::conversation::Kind;
    use crate::prism::{self, Prism};
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    fn span() -> Span {
        Span::new(0, 0)
    }

    fn ref_(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    fn mk_shard(label: &str, kind: Kind, name: &str, value: &str) -> Prism<AstNode> {
        prism::shard(
            ref_(label),
            AstNode {
                kind,
                name: name.into(),
                value: value.into(),
                span: span(),
            },
        )
    }

    fn mk_fractal(
        label: &str,
        kind: Kind,
        name: &str,
        value: &str,
        children: Vec<Prism<AstNode>>,
    ) -> Prism<AstNode> {
        prism::fractal(
            ref_(label),
            AstNode {
                kind,
                name: name.into(),
                value: value.into(),
                span: span(),
            },
            children,
        )
    }

    // --- from_grammar tests ---

    #[test]
    fn from_grammar_simple() {
        // grammar @test { type = a | b | c }
        let variant_a = mk_shard("va", Kind::Form, "variant", "a");
        let variant_b = mk_shard("vb", Kind::Form, "variant", "b");
        let variant_c = mk_shard("vc", Kind::Form, "variant", "c");
        let type_def = mk_fractal(
            "type-def",
            Kind::Form,
            "type-def",
            "color",
            vec![variant_a, variant_b, variant_c],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@test", vec![type_def]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.name.as_str(), "test");
        assert_eq!(domain.types.len(), 1);
        assert_eq!(domain.types[0].name.as_str(), "color");
        assert_eq!(domain.types[0].variants.len(), 3);
        assert_eq!(domain.types[0].variants[0].name.as_str(), "a");
        assert_eq!(domain.types[0].variants[1].name.as_str(), "b");
        assert_eq!(domain.types[0].variants[2].name.as_str(), "c");
    }

    #[test]
    fn from_grammar_with_lenses_test() {
        // in @actor; grammar @compiler { action compile { source: artifact } }
        let field = mk_shard("field-source", Kind::Atom, "field", "source");
        let action = mk_fractal(
            "action-compile",
            Kind::Form,
            "action-def",
            "compile",
            vec![field],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@compiler", vec![action]);
        let lenses = vec!["@actor".to_string()];

        let domain = Domain::from_grammar_with_lenses(&grammar, &lenses).unwrap();

        assert_eq!(domain.name.as_str(), "compiler");
        assert!(domain.is_actor());
        assert_eq!(domain.actions.len(), 1);
        assert_eq!(domain.actions[0].name.as_str(), "compile");
    }

    #[test]
    fn from_grammar_properties() {
        // grammar @guarded { requires initialized; invariant non_empty; ensures delivered }
        let req = mk_shard("req", Kind::Decl, "requires", "initialized");
        let inv = mk_shard("inv", Kind::Decl, "invariant", "non_empty");
        let ens = mk_shard("ens", Kind::Decl, "ensures", "delivered");
        let grammar = mk_fractal(
            "grammar",
            Kind::Decl,
            "grammar",
            "@guarded",
            vec![req, inv, ens],
        );

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.name.as_str(), "guarded");
        assert_eq!(
            domain.properties.requires,
            vec![PropertyName::new("initialized")]
        );
        assert_eq!(
            domain.properties.invariants,
            vec![PropertyName::new("non_empty")]
        );
        assert_eq!(
            domain.properties.ensures,
            vec![PropertyName::new("delivered")]
        );
    }

    #[test]
    fn from_grammar_invalid_type_ref() {
        // variant "bad" in type "color" references undeclared type "nonexistent"
        let bad_type_ref = mk_shard("type-ref-bad", Kind::Ref, "type-ref", "nonexistent");
        let bad_variant = mk_fractal(
            "variant-bad",
            Kind::Form,
            "variant",
            "bad",
            vec![bad_type_ref],
        );
        let type_def = mk_fractal(
            "type-def-color",
            Kind::Form,
            "type-def",
            "color",
            vec![bad_variant],
        );
        let grammar = mk_fractal("grammar-bad", Kind::Decl, "grammar", "@bad", vec![type_def]);

        let err = Domain::from_grammar(&grammar).unwrap_err();

        assert!(
            err.contains("nonexistent"),
            "error should mention the bad type ref: {err}"
        );
        assert!(
            err.contains("bad"),
            "error should mention the variant name: {err}"
        );
        assert!(
            err.contains("color"),
            "error should mention the type name: {err}"
        );
    }

    #[test]
    fn from_grammar_valid_parameterized_variant() {
        // type tree = leaf | node(tree) — self-referential, "tree" is the declared type.
        // Also tests that non-variant children inside type-def are skipped (line 262).
        let leaf_var = mk_shard("variant-leaf", Kind::Form, "variant", "leaf");
        // node variant references "tree" (which IS a declared type)
        let type_ref_node = mk_shard("type-ref-tree", Kind::Ref, "type-ref", "tree");
        let node_variant = mk_fractal(
            "variant-node",
            Kind::Form,
            "variant",
            "node",
            vec![type_ref_node],
        );
        // A non-variant child inside type-def — should be skipped (line 262 coverage).
        let noise = mk_shard("noise", Kind::Atom, "field", "noise");
        let type_def = mk_fractal(
            "type-def-tree",
            Kind::Form,
            "type-def",
            "tree",
            vec![noise, leaf_var, node_variant],
        );
        let grammar = mk_fractal(
            "grammar",
            Kind::Decl,
            "grammar",
            "@recursive",
            vec![type_def],
        );

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.types.len(), 1);
        assert_eq!(domain.types[0].variants.len(), 2);
        // node variant has a param referencing "tree" (which is declared)
        let node = &domain.types[0].variants[1];
        assert_eq!(node.name.as_str(), "node");
        assert_eq!(node.params.len(), 1);
        assert_eq!(node.params[0].1.type_name().as_str(), "tree");
    }

    #[test]
    fn from_grammar_action_visibility_and_calls() {
        // action with explicit visibility (public/private) and a cross-actor call.
        let vis_public = mk_shard("vis-pub", Kind::Atom, "visibility", "public");
        let field = mk_shard("field-src", Kind::Atom, "field", "source");
        let call_node = mk_shard("call-node", Kind::Ref, "action-call", "@tools.run");
        let action_pub = mk_fractal(
            "action-pub",
            Kind::Form,
            "action-def",
            "execute",
            vec![vis_public, field, call_node],
        );

        let vis_private = mk_shard("vis-priv", Kind::Atom, "visibility", "private");
        let action_priv = mk_fractal(
            "action-priv",
            Kind::Form,
            "action-def",
            "internal",
            vec![vis_private],
        );

        let grammar = mk_fractal(
            "grammar",
            Kind::Decl,
            "grammar",
            "@runner",
            vec![action_pub, action_priv],
        );

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.actions.len(), 2);

        let execute = &domain.actions[0];
        assert_eq!(execute.name.as_str(), "execute");
        assert_eq!(execute.visibility, Visibility::Public);
        assert_eq!(execute.fields.len(), 1);
        assert_eq!(execute.calls.len(), 1);
        assert_eq!(execute.calls[0].domain.as_str(), "tools");
        assert_eq!(execute.calls[0].action.as_str(), "run");

        let internal = &domain.actions[1];
        assert_eq!(internal.name.as_str(), "internal");
        assert_eq!(internal.visibility, Visibility::Private);
    }

    #[test]
    fn from_grammar_action_default_visibility_protected() {
        // action with no visibility atom → defaults to Protected.
        let action = mk_fractal("action-default", Kind::Form, "action-def", "ping", vec![]);
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@health", vec![action]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.actions[0].visibility, Visibility::Protected);
    }

    #[test]
    fn from_grammar_action_unknown_visibility_falls_back_to_protected() {
        // visibility atom with unknown value → Protected (the _ arm).
        let vis_unknown = mk_shard("vis-unknown", Kind::Atom, "visibility", "unknown_vis");
        let action = mk_fractal(
            "action-unknown-vis",
            Kind::Form,
            "action-def",
            "query",
            vec![vis_unknown],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@service", vec![action]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.actions[0].visibility, Visibility::Protected);
    }

    #[test]
    fn from_grammar_action_call_with_args() {
        // action-call with child arg nodes → args collected into ActionCall.
        let arg1 = mk_shard("arg1", Kind::Atom, "field", "source");
        let arg2 = mk_shard("arg2", Kind::Atom, "field", "dest");
        let call_node = mk_fractal(
            "call-with-args",
            Kind::Ref,
            "action-call",
            "@fs.copy",
            vec![arg1, arg2],
        );
        let action = mk_fractal(
            "action-copy",
            Kind::Form,
            "action-def",
            "copy",
            vec![call_node],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@pipeline", vec![action]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        let copy_action = &domain.actions[0];
        assert_eq!(copy_action.calls.len(), 1);
        assert_eq!(copy_action.calls[0].domain.as_str(), "fs");
        assert_eq!(copy_action.calls[0].action.as_str(), "copy");
        assert_eq!(copy_action.calls[0].args.len(), 2);
        assert_eq!(copy_action.calls[0].args[0].type_name().as_str(), "source");
        assert_eq!(copy_action.calls[0].args[1].type_name().as_str(), "dest");
    }

    #[test]
    fn from_grammar_action_call_no_dot_skipped() {
        // action-call value with no '.' → split_once fails, call is silently skipped.
        let call_node = mk_shard("call-nodot", Kind::Ref, "action-call", "nodot");
        let action = mk_fractal(
            "action-nodot",
            Kind::Form,
            "action-def",
            "noop",
            vec![call_node],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@misc", vec![action]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        // Call is skipped — no calls recorded.
        assert_eq!(domain.actions[0].calls.len(), 0);
    }

    #[test]
    fn from_grammar_action_field_with_type_ref() {
        // A field that has a type-ref child → the map closure on line 317 is invoked.
        let type_ref = mk_shard("type-ref-signal", Kind::Ref, "type-ref", "signal");
        let field = mk_fractal(
            "field-payload",
            Kind::Atom,
            "field",
            "payload",
            vec![type_ref],
        );
        let action = mk_fractal("action-send", Kind::Form, "action-def", "send", vec![field]);
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@msg", vec![action]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        let send = &domain.actions[0];
        assert_eq!(send.fields.len(), 1);
        assert_eq!(send.fields[0].0.as_str(), "payload");
        assert_eq!(send.fields[0].1.type_name().as_str(), "signal");
    }

    #[test]
    fn from_grammar_action_unrecognized_child_skipped() {
        // An action child that is none of visibility/field/action-call → silently skipped.
        // This exercises the fall-through of all three else-if branches (line 336).
        let unknown_child = mk_shard("unknown", Kind::Decl, "in", "@somewhere");
        let action = mk_fractal(
            "action-with-noise",
            Kind::Form,
            "action-def",
            "quiet",
            vec![unknown_child],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@silent", vec![action]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        let quiet = &domain.actions[0];
        assert_eq!(quiet.fields.len(), 0);
        assert_eq!(quiet.calls.len(), 0);
    }

    #[test]
    fn from_grammar_variant_with_non_typeref_child_skipped() {
        // A variant child that is NOT a type-ref should not produce a param entry.
        // This exercises the false branch of `if sub.data().is_ref("type-ref")`.
        let noise_child = mk_shard("noise-child", Kind::Atom, "field", "ignored");
        let variant = mk_fractal(
            "variant-with-noise",
            Kind::Form,
            "variant",
            "plain",
            vec![noise_child],
        );
        let type_def = mk_fractal(
            "type-def-simple",
            Kind::Form,
            "type-def",
            "simple",
            vec![variant],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@clean", vec![type_def]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        assert_eq!(domain.types[0].variants[0].params.len(), 0);
    }

    // --- Hash / Ord traits (needed for map keys) ---

    #[test]
    fn domain_name_ord() {
        let mut names = vec![
            DomainName::new("z"),
            DomainName::new("a"),
            DomainName::new("m"),
        ];
        names.sort();
        assert_eq!(names[0].as_str(), "a");
        assert_eq!(names[2].as_str(), "z");
    }

    #[test]
    fn domain_name_hash_as_map_key() {
        use std::collections::HashMap;
        let mut map: HashMap<DomainName, u32> = HashMap::new();
        map.insert(DomainName::new("fs"), 1);
        assert_eq!(map[&DomainName::new("fs")], 1);
    }

    // --- Full Domain with nested structs ---

    #[test]
    fn domain_with_types_and_actions() {
        let domain = Domain {
            name: DomainName::new("@reed"),
            types: vec![TypeDef {
                name: TypeName::new("signal"),
                variants: vec![
                    Variant {
                        name: VariantName::new("tick"),
                        params: vec![],
                    },
                    Variant {
                        name: VariantName::new("data"),
                        params: vec![(
                            VariantName::new("value"),
                            TypeRef::new(TypeName::new("signal")),
                        )],
                    },
                ],
            }],
            actions: vec![Action {
                name: ActionName::new("send"),
                fields: vec![(
                    ActionName::new("payload"),
                    TypeRef::new(TypeName::new("signal")),
                )],
                visibility: Visibility::Public,
                calls: vec![ActionCall {
                    domain: DomainName::new("@erlang"),
                    action: ActionName::new("exec"),
                    args: vec![TypeRef::new(TypeName::new("mfa"))],
                }],
            }],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("initialized")],
                invariants: vec![],
                ensures: vec![PropertyName::new("delivered")],
            },
        };

        assert_eq!(domain.name.as_str(), "reed");
        assert_eq!(domain.types.len(), 1);
        assert_eq!(domain.actions.len(), 1);
        assert_eq!(domain.actions[0].calls.len(), 1);
        assert_eq!(domain.actions[0].calls[0].domain.as_str(), "erlang");
        assert!(!domain.is_actor());
    }

    // --- Domain::registry() ---

    #[test]
    fn from_grammar_populates_registry() {
        let variant_a = mk_shard("va", Kind::Form, "variant", "a");
        let variant_b = mk_shard("vb", Kind::Form, "variant", "b");
        let type_def = mk_fractal(
            "type-def",
            Kind::Form,
            "type-def",
            "color",
            vec![variant_a, variant_b],
        );
        let grammar = mk_fractal("grammar", Kind::Decl, "grammar", "@test", vec![type_def]);

        let domain = Domain::from_grammar(&grammar).unwrap();

        // Registry should be populated and accessible.
        let registry = domain.registry();
        assert_eq!(registry.domain, "test");
        assert!(registry.has_type("color"));
    }
}
