//! Model checker — static verification of Domain properties.
//!
//! Pure functions, no actors, no async. Takes a Domain, checks all
//! `requires` and `invariant` properties against the type graph, and
//! returns either a `Verified` proof wrapper or a `Violations` report.
//!
//! `ensures` properties are NOT checked here — they require a running
//! system and are validated at runtime.

use std::fmt;

use crate::model::{Domain, DomainName, PropertyName, TypeName};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A domain that passed all property checks.
///
/// Can only be constructed by `verify()`. Callers that receive a `Verified`
/// know the domain satisfies all static properties without needing to re-run
/// the checks.
pub struct Verified(Domain);

impl fmt::Debug for Verified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Verified").field(&self.0.name).finish()
    }
}

impl Verified {
    /// Borrow the verified domain.
    pub fn domain(&self) -> &Domain {
        &self.0
    }

    /// Consume the wrapper and return the inner domain.
    pub fn into_domain(self) -> Domain {
        self.0
    }
}

/// Why a domain failed verification.
pub struct Violations {
    pub domain: DomainName,
    pub violations: Vec<PropertyViolation>,
}

impl fmt::Debug for Violations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Violations({} violations for @{})",
            self.violations.len(),
            self.domain
        )
    }
}

impl fmt::Display for Violations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "@{}: property verification failed", self.domain)?;
        for v in &self.violations {
            writeln!(f, "  {} property '{}' violated", v.kind, v.property)?;
            match &v.evidence {
                Evidence::Spectral {
                    measure,
                    value,
                    threshold,
                } => {
                    writeln!(
                        f,
                        "    {}: {:.4} (connectivity requires > {:.4})",
                        measure, value, threshold
                    )?;
                }
                Evidence::Disconnected { components } => {
                    writeln!(
                        f,
                        "    Type graph has {} disconnected components:",
                        components.len()
                    )?;
                    for component in components {
                        let names: Vec<&str> = component.iter().map(|t| t.as_str()).collect();
                        writeln!(f, "      {{{}}}", names.join(", "))?;
                    }
                }
                Evidence::Unresolvable { name, candidates } => {
                    write!(f, "    Unresolvable type '{}'", name)?;
                    if candidates.is_empty() {
                        writeln!(f)?;
                    } else {
                        let cs: Vec<&str> = candidates.iter().map(|t| t.as_str()).collect();
                        writeln!(f, "; candidates: {}", cs.join(", "))?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// A single property that was violated.
pub struct PropertyViolation {
    pub domain: DomainName,
    pub property: PropertyName,
    pub kind: PropertyKind,
    pub reason: String,
    pub evidence: Evidence,
}

/// Which clause of the property declaration was violated.
pub enum PropertyKind {
    Required,
    Invariant,
    Ensures,
}

impl fmt::Display for PropertyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyKind::Required => f.write_str("required"),
            PropertyKind::Invariant => f.write_str("invariant"),
            PropertyKind::Ensures => f.write_str("ensures"),
        }
    }
}

/// Structured evidence for a violation.
pub enum Evidence {
    /// The type graph has disconnected components.
    Disconnected { components: Vec<Vec<TypeName>> },
    /// A spectral measure fell below its threshold.
    Spectral {
        measure: String,
        value: f64,
        threshold: f64,
    },
    /// A type name could not be resolved.
    Unresolvable {
        name: TypeName,
        candidates: Vec<TypeName>,
    },
}

// ---------------------------------------------------------------------------
// verify() — stub, always returns Ok (🔴 tests will fail)
// ---------------------------------------------------------------------------

/// Check all `requires` and `invariant` properties of a domain.
///
/// Returns `Ok(Verified)` if all properties pass.
/// Returns `Err(Violations)` with structured evidence if any fail.
///
/// NOT YET IMPLEMENTED: currently a stub that always passes.
pub fn verify(domain: Domain) -> Result<Verified, Violations> {
    Ok(Verified(domain))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Domain, DomainName, Properties, PropertyName, TypeDef, TypeRef, Variant, VariantName,
    };

    fn empty_domain(name: &str) -> Domain {
        Domain {
            name: DomainName::new(name),
            types: vec![],
            actions: vec![],
            lenses: vec![],
            properties: Properties::empty(),
        }
    }

    fn type_def_simple(name: &str, variants: &[&str]) -> TypeDef {
        TypeDef {
            name: TypeName::new(name),
            variants: variants
                .iter()
                .map(|v| Variant {
                    name: VariantName::new(*v),
                    params: vec![],
                })
                .collect(),
        }
    }

    /// Connected domain: color references shade via parameterized variant.
    fn connected_domain() -> Domain {
        let shade_variant = Variant {
            name: VariantName::new("red"),
            params: vec![(
                VariantName::new("red"),
                TypeRef::new(TypeName::new("shade")),
            )],
        };
        let blue_variant = Variant {
            name: VariantName::new("blue"),
            params: vec![],
        };
        let color = TypeDef {
            name: TypeName::new("color"),
            variants: vec![shade_variant, blue_variant],
        };
        let shade = type_def_simple("shade", &["light", "dark"]);

        Domain {
            name: DomainName::new("connected"),
            types: vec![color, shade],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
        }
    }

    /// Disconnected domain: two unconnected types, requires connected.
    fn disconnected_domain() -> Domain {
        let color = type_def_simple("color", &["red", "blue"]);
        let shape = type_def_simple("shape", &["circle", "square"]);

        Domain {
            name: DomainName::new("broken"),
            types: vec![color, shape],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
        }
    }

    #[test]
    fn verify_no_properties_succeeds() {
        let domain = empty_domain("clean");
        assert!(verify(domain).is_ok());
    }

    #[test]
    fn verify_connected_domain_succeeds() {
        let domain = connected_domain();
        assert!(verify(domain).is_ok());
    }

    #[test]
    fn verify_disconnected_domain_fails() {
        // FAILS: stub always returns Ok.
        let domain = disconnected_domain();
        assert!(verify(domain).is_err());
    }

    #[test]
    fn verify_disconnected_has_evidence() {
        // FAILS: stub always returns Ok, no Err to inspect.
        let domain = disconnected_domain();
        let err = verify(domain).unwrap_err();
        assert_eq!(err.violations.len(), 1);
        match &err.violations[0].evidence {
            Evidence::Disconnected { components } => {
                assert!(components.len() >= 2);
            }
            other => panic!(
                "expected Disconnected evidence: {:?}",
                std::mem::discriminant(other)
            ),
        }
    }

    #[test]
    fn violations_display_is_readable() {
        // FAILS: stub always returns Ok.
        let domain = disconnected_domain();
        let err = verify(domain).unwrap_err();
        let output = format!("{}", err);
        assert!(output.contains("broken"));
        assert!(output.contains("connected"));
        assert!(output.contains("required"));
        assert!(output.contains("component"));
    }

    #[test]
    fn verified_domain_accessor() {
        let domain = empty_domain("ok");
        let verified = verify(domain).unwrap();
        assert_eq!(verified.domain().name.as_str(), "ok");
        let inner = verified.into_domain();
        assert_eq!(inner.name.as_str(), "ok");
    }

    #[test]
    fn verify_single_type_trivially_connected() {
        let domain = Domain {
            name: DomainName::new("solo"),
            types: vec![type_def_simple("signal", &["tick", "data"])],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
        };
        assert!(verify(domain).is_ok());
    }

    #[test]
    fn verify_invariant_connected_also_checked() {
        // FAILS: stub always returns Ok.
        let color = type_def_simple("color", &["red", "blue"]);
        let shape = type_def_simple("shape", &["circle", "square"]);
        let domain = Domain {
            name: DomainName::new("guarded"),
            types: vec![color, shape],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![],
                invariants: vec![PropertyName::new("connected")],
                ensures: vec![],
            },
        };
        let err = verify(domain).unwrap_err();
        assert_eq!(err.violations[0].property.as_str(), "connected");
        match err.violations[0].kind {
            PropertyKind::Invariant => {}
            _ => panic!("expected Invariant kind"),
        }
    }

    #[test]
    fn verify_unknown_property_passes() {
        let domain = Domain {
            name: DomainName::new("future"),
            types: vec![],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("future_property_not_yet_known")],
                invariants: vec![],
                ensures: vec![],
            },
        };
        assert!(verify(domain).is_ok());
    }

    #[test]
    fn verify_ensures_skipped() {
        let color = type_def_simple("color", &["red"]);
        let shape = type_def_simple("shape", &["circle"]);
        let domain = Domain {
            name: DomainName::new("deferred"),
            types: vec![color, shape],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![],
                invariants: vec![],
                ensures: vec![PropertyName::new("connected")],
            },
        };
        assert!(verify(domain).is_ok());
    }
}
