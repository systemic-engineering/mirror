//! Domain model types.
//!
//! Make illegal state unrepresentable. This module provides newtypes and
//! domain structs that serve as the public API for compiled grammars,
//! Domain model types for compiled grammars.

use std::fmt;

use crate::ast::AstNode;
use crate::prism::Prism;
use crate::resolve::Visibility;
use crate::{ContentAddressed, Oid};
use fragmentation::encoding::Encode;

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

/// A parameter name within an action definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamName(String);

impl ParamName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ParamName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ParamName {
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

/// The body of an action — a target domain and raw source text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBody {
    pub target: DomainName,
    pub source: String,
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

/// A compiled domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Domain {
    pub name: DomainName,
    pub types: Vec<TypeDef>,
    pub actions: Vec<Action>,
    pub lenses: Vec<Lens>,
    pub extends: Vec<DomainName>,
    pub calls: Vec<ActionCall>,
    pub properties: Properties,
}

domain_oid!(/// Content address for domains.
pub DomainOid);

impl Encode for Domain {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"domain:");
        bytes.extend_from_slice(self.name.as_str().as_bytes());
        for typedef in &self.types {
            bytes.extend_from_slice(b":type:");
            bytes.extend_from_slice(typedef.name.as_str().as_bytes());
            for variant in &typedef.variants {
                bytes.extend_from_slice(b":");
                bytes.extend_from_slice(variant.name.as_str().as_bytes());
            }
        }
        for action in &self.actions {
            bytes.extend_from_slice(b":action:");
            bytes.extend_from_slice(action.name.as_str().as_bytes());
        }
        for prop in &self.properties.requires {
            bytes.extend_from_slice(b":requires:");
            bytes.extend_from_slice(prop.as_str().as_bytes());
        }
        for prop in &self.properties.invariants {
            bytes.extend_from_slice(b":invariant:");
            bytes.extend_from_slice(prop.as_str().as_bytes());
        }
        for prop in &self.properties.ensures {
            bytes.extend_from_slice(b":ensures:");
            bytes.extend_from_slice(prop.as_str().as_bytes());
        }
        bytes
    }
}

impl ContentAddressed for Domain {
    type Oid = DomainOid;
    fn content_oid(&self) -> DomainOid {
        DomainOid::from(Oid::hash(&self.encode()))
    }
}

// ---------------------------------------------------------------------------
// DomainSpectrum / DomainComplexity
// ---------------------------------------------------------------------------

/// Spectral analysis of a domain's type graph.
/// Only constructable internally — eigenvalues come from the Laplacian.
#[derive(Clone)]
pub struct DomainSpectrum {
    eigenvalues: coincidence::eigenvalues::Eigenvalues,
}

impl DomainSpectrum {
    pub(crate) fn new(eigenvalues: coincidence::eigenvalues::Eigenvalues) -> Self {
        DomainSpectrum { eigenvalues }
    }

    pub fn eigenvalues(&self) -> &coincidence::eigenvalues::Eigenvalues {
        &self.eigenvalues
    }
}

/// Grammars with no type reference edges have no type reference graph.
/// Not "spectrum with zeros" — absence of spectrum entirely.
#[derive(Clone)]
pub enum DomainComplexity {
    Trivial,
    Spectrum(DomainSpectrum),
}

impl DomainComplexity {
    /// Returns `true` if this complexity is trivial (no type reference graph).
    pub fn is_trivial(&self) -> bool {
        matches!(self, DomainComplexity::Trivial)
    }

    /// Extract the spectrum, returning `None` for trivial domains.
    pub fn spectrum(self) -> Option<DomainSpectrum> {
        match self {
            DomainComplexity::Trivial => None,
            DomainComplexity::Spectrum(s) => Some(s),
        }
    }
}

impl Domain {
    /// Returns `true` if this domain has a lens targeting `"actor"`.
    pub fn is_actor(&self) -> bool {
        self.lenses.iter().any(|l| l.target.as_str() == "actor")
    }

    // -----------------------------------------------------------------------
    // Query methods
    // -----------------------------------------------------------------------

    /// The domain name as a string.
    pub fn domain_name(&self) -> &str {
        self.name.as_str()
    }

    /// All type names declared in this domain.
    pub fn type_names(&self) -> Vec<&str> {
        self.types.iter().map(|t| t.name.as_str()).collect()
    }

    /// Check if a named type exists in this domain.
    pub fn has_type(&self, name: &str) -> bool {
        self.types.iter().any(|t| t.name.as_str() == name)
    }

    /// All variants for a named type. Returns None if the type doesn't exist.
    pub fn variants(&self, type_name: &str) -> Option<Vec<&str>> {
        self.types
            .iter()
            .find(|t| t.name.as_str() == type_name)
            .map(|t| t.variants.iter().map(|v| v.name.as_str()).collect())
    }

    /// Check if a variant exists under a given type name.
    pub fn has_variant(&self, type_name: &str, variant: &str) -> bool {
        self.types
            .iter()
            .find(|t| t.name.as_str() == type_name)
            .is_some_and(|t| t.variants.iter().any(|v| v.name.as_str() == variant))
    }

    /// The parameter type reference for a parameterized variant, if any.
    pub fn variant_param(&self, type_name: &str, variant: &str) -> Option<&str> {
        self.types
            .iter()
            .find(|t| t.name.as_str() == type_name)
            .and_then(|t| {
                t.variants
                    .iter()
                    .find(|v| v.name.as_str() == variant)
                    .and_then(|v| v.params.first().map(|(_, tr)| tr.type_name().as_str()))
            })
    }

    /// Check if a named action exists in this domain.
    pub fn has_action(&self, name: &str) -> bool {
        self.actions.iter().any(|a| a.name.as_str() == name)
    }

    /// All action names declared in this domain.
    pub fn act_names(&self) -> Vec<&str> {
        self.actions.iter().map(|a| a.name.as_str()).collect()
    }

    /// Get the fields of a named action: (field_name, type_ref_name).
    ///
    /// Returns field name and type ref as string tuples, matching
    /// Returns field name and optional type ref as string tuples.
    pub fn act_fields(&self, name: &str) -> Option<Vec<(&str, Option<&str>)>> {
        self.actions
            .iter()
            .find(|a| a.name.as_str() == name)
            .map(|a| {
                a.fields
                    .iter()
                    .map(|(field_name, type_ref)| {
                        let tr = type_ref.type_name().as_str();
                        let tr_opt = if tr.is_empty() { None } else { Some(tr) };
                        (field_name.as_str(), tr_opt)
                    })
                    .collect()
            })
    }

    /// Get the cross-actor calls for a named action: (domain, action, args).
    pub fn action_calls(&self, name: &str) -> Vec<(&str, &str, Vec<&str>)> {
        self.actions
            .iter()
            .find(|a| a.name.as_str() == name)
            .map(|a| {
                a.calls
                    .iter()
                    .map(|c| {
                        let args: Vec<&str> =
                            c.args.iter().map(|tr| tr.type_name().as_str()).collect();
                        (c.domain.as_str(), c.action.as_str(), args)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get the visibility of a named action. Defaults to Protected.
    pub fn action_visibility(&self, name: &str) -> Visibility {
        self.actions
            .iter()
            .find(|a| a.name.as_str() == name)
            .map(|a| a.visibility.clone())
            .unwrap_or(Visibility::Protected)
    }

    /// All `requires` property names declared in this domain.
    pub fn required_properties(&self) -> Vec<&str> {
        self.properties
            .requires
            .iter()
            .map(|p| p.as_str())
            .collect()
    }

    /// All `invariant` property names declared in this domain.
    pub fn invariants(&self) -> Vec<&str> {
        self.properties
            .invariants
            .iter()
            .map(|p| p.as_str())
            .collect()
    }

    /// All `ensures` property names declared in this domain.
    pub fn ensures(&self) -> Vec<&str> {
        self.properties.ensures.iter().map(|p| p.as_str()).collect()
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

        // Extract extends declarations from grammar children.
        let extends: Vec<DomainName> = node
            .children()
            .iter()
            .filter(|c| c.data().is_ref("extends"))
            .map(|c| DomainName::new(c.data().value.as_str()))
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
            } else if child.data().is_decl("action-def") || child.data().is_form("action-def") {
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
                    } else if item.data().is_atom("param") {
                        // New syntax: param node value is "name:type"
                        let param_val = item.data().value.as_str();
                        let (pname, ptype) = param_val
                            .split_once(':')
                            .unwrap_or((param_val, ""));
                        let field_name = ActionName::new(pname);
                        let type_ref = TypeRef::new(TypeName::new(ptype));
                        fields.push((field_name, type_ref));
                    } else if item.data().is_atom("field") {
                        // Legacy: field nodes from older ASTs
                        let field_name = ActionName::new(item.data().value.as_str());
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

        // Aggregate calls from all actions to the domain level.
        let calls: Vec<ActionCall> = actions.iter().flat_map(|a| a.calls.clone()).collect();

        Ok(Domain {
            name: domain_name,
            types,
            actions,
            lenses,
            extends,
            calls,
            properties,
        })
    }

    /// Compute the spectral complexity of this domain's type reference graph.
    ///
    /// Types are vertices, parameterized variant references are directed edges.
    /// If there are no inter-type references, returns `Trivial` — not a zero
    /// spectrum, but absence of spectrum entirely.
    pub fn complexity(&self) -> DomainComplexity {
        const MAX_TYPE_GRAPH_SIZE: usize = 500;

        let type_names: Vec<String> = self
            .types
            .iter()
            .map(|t| t.name.as_str().to_string())
            .collect();

        if type_names.is_empty() || type_names.len() > MAX_TYPE_GRAPH_SIZE {
            return DomainComplexity::Trivial;
        }

        // Build edges from parameterized variant references
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (i, typedef) in self.types.iter().enumerate() {
            for variant in &typedef.variants {
                for (_param_name, type_ref) in &variant.params {
                    if let Some(j) = type_names.iter().position(|n| n == type_ref.0.as_str()) {
                        if i != j {
                            edges.push((i, j));
                        }
                    }
                }
            }
        }

        if edges.is_empty() {
            return DomainComplexity::Trivial;
        }

        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&type_names, &edges);
        let eigenvalues = laplacian.eigenvalues();
        DomainComplexity::Spectrum(DomainSpectrum::new(eigenvalues))
    }
}

impl Domain {
    /// Test-only: build a domain with a parameterized variant whose type ref
    /// is NOT declared. This exercises the `None => continue` defensive path
    /// in `generate::derive_type`.
    #[cfg(test)]
    pub(crate) fn with_dangling_param(
        domain: &str,
        type_name: &str,
        variant: &str,
        param_ref: &str,
    ) -> Self {
        Domain {
            name: DomainName::new(domain),
            types: vec![TypeDef {
                name: TypeName::new(type_name),
                variants: vec![Variant {
                    name: VariantName::new(variant),
                    params: vec![(
                        VariantName::new(variant),
                        TypeRef::new(TypeName::new(param_ref)),
                    )],
                }],
            }],
            actions: vec![],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        }
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
            extends: vec![],
            calls: vec![],
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
            extends: vec![],
            calls: vec![],
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

    // --- Domain query methods ---

    fn make_rich_domain() -> Domain {
        Domain {
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
            extends: vec![],
            calls: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("initialized")],
                invariants: vec![PropertyName::new("non_empty")],
                ensures: vec![PropertyName::new("delivered")],
            },
        }
    }

    #[test]
    fn domain_query_domain_name() {
        let d = make_rich_domain();
        assert_eq!(d.domain_name(), "reed");
    }

    #[test]
    fn domain_query_type_names() {
        let d = make_rich_domain();
        let names = d.type_names();
        assert_eq!(names, vec!["signal"]);
    }

    #[test]
    fn domain_query_has_type() {
        let d = make_rich_domain();
        assert!(d.has_type("signal"));
        assert!(!d.has_type("nonexistent"));
    }

    #[test]
    fn domain_query_variants() {
        let d = make_rich_domain();
        let vs = d.variants("signal").unwrap();
        assert!(vs.contains(&"tick"));
        assert!(vs.contains(&"data"));
        assert!(d.variants("nonexistent").is_none());
    }

    #[test]
    fn domain_query_has_variant() {
        let d = make_rich_domain();
        assert!(d.has_variant("signal", "tick"));
        assert!(d.has_variant("signal", "data"));
        assert!(!d.has_variant("signal", "nonexistent"));
        assert!(!d.has_variant("nonexistent", "tick"));
    }

    #[test]
    fn domain_query_variant_param() {
        let d = make_rich_domain();
        // "data" variant has a param referencing "signal"
        assert_eq!(d.variant_param("signal", "data"), Some("signal"));
        // "tick" variant has no params
        assert!(d.variant_param("signal", "tick").is_none());
        // nonexistent type
        assert!(d.variant_param("nonexistent", "data").is_none());
    }

    #[test]
    fn domain_query_has_action() {
        let d = make_rich_domain();
        assert!(d.has_action("send"));
        assert!(!d.has_action("nonexistent"));
    }

    #[test]
    fn domain_query_act_names() {
        let d = make_rich_domain();
        assert_eq!(d.act_names(), vec!["send"]);
    }

    #[test]
    fn domain_query_act_fields() {
        let d = make_rich_domain();
        let fields = d.act_fields("send").unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, "payload");
        assert_eq!(fields[0].1, Some("signal"));
        assert!(d.act_fields("nonexistent").is_none());
    }

    #[test]
    fn domain_query_act_fields_empty_type_ref() {
        // Action with a field whose type ref is empty string → None.
        let d = Domain {
            name: DomainName::new("test"),
            types: vec![],
            actions: vec![Action {
                name: ActionName::new("ping"),
                fields: vec![(ActionName::new("target"), TypeRef::new(TypeName::new("")))],
                visibility: Visibility::Protected,
                calls: vec![],
            }],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        };
        let fields = d.act_fields("ping").unwrap();
        assert_eq!(fields[0].1, None);
    }

    #[test]
    fn domain_query_action_calls() {
        let d = make_rich_domain();
        let calls = d.action_calls("send");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "erlang");
        assert_eq!(calls[0].1, "exec");
        assert_eq!(calls[0].2, vec!["mfa"]);
        // nonexistent action returns empty
        assert!(d.action_calls("nonexistent").is_empty());
    }

    #[test]
    fn domain_query_action_visibility() {
        let d = make_rich_domain();
        assert_eq!(d.action_visibility("send"), Visibility::Public);
        // nonexistent action defaults to Protected
        assert_eq!(d.action_visibility("nonexistent"), Visibility::Protected);
    }

    #[test]
    fn domain_query_required_properties() {
        let d = make_rich_domain();
        assert_eq!(d.required_properties(), vec!["initialized"]);
    }

    #[test]
    fn domain_query_invariants() {
        let d = make_rich_domain();
        assert_eq!(d.invariants(), vec!["non_empty"]);
    }

    #[test]
    fn domain_query_ensures() {
        let d = make_rich_domain();
        assert_eq!(d.ensures(), vec!["delivered"]);
    }

    // --- DomainComplexity tests ---

    #[test]
    fn domain_complexity_trivial_for_no_types() {
        use crate::{Parse, Vector};
        let source = "grammar @empty {}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let complexity = domain.complexity();
        assert!(complexity.is_trivial());
        assert!(complexity.spectrum().is_none());
    }

    #[test]
    fn domain_complexity_trivial_for_flat_types() {
        use crate::{Parse, Vector};
        let source = "grammar @flat {\n  type = a | b\n  type op = gt | lt\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(matches!(domain.complexity(), DomainComplexity::Trivial));
    }

    #[test]
    fn domain_complexity_spectrum_for_referenced_types() {
        use crate::{Parse, Vector};
        let source =
            "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let spectrum = domain
            .complexity()
            .spectrum()
            .expect("expected Spectrum, got Trivial");
        let ev = spectrum.eigenvalues();
        assert!(ev.len() >= 2);
        assert!(ev.fiedler_value().unwrap() > 0.0);
    }

    #[test]
    fn domain_complexity_trivial_for_external_type_refs() {
        // A manually-constructed Domain where variant params reference
        // types outside this domain — those edges are skipped, leaving
        // no internal edges → Trivial.
        let domain = Domain {
            name: DomainName::new("manual"),
            types: vec![TypeDef {
                name: TypeName::new("request"),
                variants: vec![Variant {
                    name: VariantName::new("send"),
                    params: vec![(
                        VariantName::new("payload"),
                        TypeRef::new(TypeName::new("external_type")),
                    )],
                }],
            }],
            actions: vec![],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        };
        assert!(matches!(domain.complexity(), DomainComplexity::Trivial));
    }

    #[test]
    fn domain_complexity_trivial_for_self_referential_types() {
        // A type that only references itself (i == j) → self-edges skipped → Trivial.
        let domain = Domain {
            name: DomainName::new("recursive"),
            types: vec![TypeDef {
                name: TypeName::new("tree"),
                variants: vec![
                    Variant {
                        name: VariantName::new("leaf"),
                        params: vec![],
                    },
                    Variant {
                        name: VariantName::new("node"),
                        params: vec![(
                            VariantName::new("child"),
                            TypeRef::new(TypeName::new("tree")),
                        )],
                    },
                ],
            }],
            actions: vec![],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        };
        assert!(matches!(domain.complexity(), DomainComplexity::Trivial));
    }

    #[test]
    fn domain_complexity_trivial_for_oversized_type_graph() {
        // More than 500 types → short-circuit to Trivial (DoS protection).
        let types: Vec<TypeDef> = (0..501)
            .map(|i| TypeDef {
                name: TypeName::new(format!("t{}", i)),
                variants: vec![Variant {
                    name: VariantName::new("v"),
                    params: vec![],
                }],
            })
            .collect();
        let domain = Domain {
            name: DomainName::new("huge"),
            types,
            actions: vec![],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        };
        assert!(matches!(domain.complexity(), DomainComplexity::Trivial));
    }

    // --- extends and calls ---

    #[test]
    fn domain_has_extends() {
        let domain = Domain {
            name: DomainName::new("sub"),
            types: vec![],
            actions: vec![],
            lenses: vec![],
            extends: vec![DomainName::new("base")],
            calls: vec![],
            properties: Properties::empty(),
        };
        assert!(domain.extends.iter().any(|d| d.as_str() == "base"));
    }

    #[test]
    fn domain_has_action_calls() {
        // With new syntax, action calls only exist in bodies
        // Without a body, the domain has no calls
        use crate::{Parse, Vector};
        let source = "grammar @ai {\n  type = observation\n\n  action decide(observation)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(
            domain.calls.is_empty(),
            "no calls without a body"
        );
    }

    #[test]
    fn domain_content_addressed() {
        use crate::{Parse, Vector};
        let source = "grammar @test {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let d1 = Domain::from_grammar(grammar).unwrap();
        let d2 = Domain::from_grammar(grammar).unwrap();
        assert_eq!(d1.content_oid(), d2.content_oid());
    }

    #[test]
    fn domain_different_content_different_oid() {
        use crate::{Parse, Vector};
        let ast1 = Parse
            .trace("grammar @a {\n  type = x\n}\n".to_string())
            .unwrap();
        let g1 = ast1
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let d1 = Domain::from_grammar(g1).unwrap();

        let ast2 = Parse
            .trace("grammar @b {\n  type = y\n}\n".to_string())
            .unwrap();
        let g2 = ast2
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let d2 = Domain::from_grammar(g2).unwrap();

        assert_ne!(d1.content_oid(), d2.content_oid());
    }

    #[test]
    fn domain_content_addressed_with_actions() {
        use crate::{Parse, Vector};
        let source = "grammar @ai {\n  type = observation\n\n  action decide(observation)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let d1 = Domain::from_grammar(grammar).unwrap();
        let d2 = Domain::from_grammar(grammar).unwrap();
        // Same source → same OID.
        assert_eq!(d1.content_oid(), d2.content_oid());
        // OID differs from a domain without actions.
        let source2 = "grammar @ai {\n  type = observation\n}\n";
        let ast2 = Parse.trace(source2.to_string()).unwrap();
        let g2 = ast2
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let d3 = Domain::from_grammar(g2).unwrap();
        assert_ne!(d1.content_oid(), d3.content_oid());
    }

    #[test]
    fn from_grammar_extracts_extends() {
        use crate::{Parse, Vector};
        let source = "grammar @fox extends @smash, @controller {\n  type = move\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert_eq!(domain.extends.len(), 2);
        assert_eq!(domain.extends[0].as_str(), "smash");
        assert_eq!(domain.extends[1].as_str(), "controller");
    }

    #[test]
    fn from_grammar_no_extends_empty() {
        use crate::{Parse, Vector};
        let source = "grammar @plain {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(domain.extends.is_empty());
    }

    // --- ParamName and ActionBody ---

    #[test]
    fn param_name_new_display_as_ref() {
        let p = ParamName::new("source");
        assert_eq!(p.as_str(), "source");
        assert_eq!(p.as_ref(), "source");
        assert_eq!(p.to_string(), "source");
    }

    #[test]
    fn action_body_construction() {
        let body = ActionBody {
            target: DomainName::new("rust"),
            source: "fn main() {}".to_string(),
        };
        assert_eq!(body.target.as_str(), "rust");
        assert_eq!(body.source, "fn main() {}");
    }

    // --- DomainOid ---

    #[test]
    fn domain_oid_from_and_display() {
        let oid = DomainOid::new("abc123");
        assert_eq!(oid.as_ref(), "abc123");
        assert_eq!(oid.to_string(), "abc123");
    }
}
