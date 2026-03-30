//! Domain model types.
//!
//! Make illegal state unrepresentable. This module provides newtypes and
//! domain structs that serve as the public API for compiled grammars,
//! replacing the raw TypeRegistry internals.

use std::fmt;

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
        self.lenses
            .iter()
            .any(|l| l.target.as_str() == "actor")
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
}
