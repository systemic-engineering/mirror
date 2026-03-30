//! Model checker — static verification of Domain properties.
//!
//! Pure functions, no actors, no async. Takes a Domain, checks all
//! `requires` and `invariant` properties against the type graph, and
//! returns either a `Verified` proof wrapper or a `Violations` report.
//!
//! `ensures` properties are NOT checked here — they require a running
//! system and are validated at runtime.

use std::fmt;

use coincidence::spectral::Laplacian;

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
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("@{}: property verification failed", self.domain));
        for v in &self.violations {
            lines.push(format!(
                "  {} property '{}' violated: {}",
                v.kind, v.property, v.reason
            ));
            match &v.evidence {
                Evidence::Spectral {
                    measure,
                    value,
                    threshold,
                } => {
                    lines.push(format!(
                        "    {}: {:.4} (connectivity requires > {:.4})",
                        measure, value, threshold
                    ));
                }
                Evidence::Disconnected { components } => {
                    lines.push(format!(
                        "    Type graph has {} disconnected components:",
                        components.len()
                    ));
                    for component in components {
                        let names: Vec<&str> = component.iter().map(|t| t.as_str()).collect();
                        lines.push(format!("      {{{}}}", names.join(", ")));
                    }
                }
                Evidence::Unresolvable { name, candidates } => {
                    if candidates.is_empty() {
                        lines.push(format!("    Unresolvable type '{}'", name));
                    } else {
                        let cs: Vec<&str> = candidates.iter().map(|t| t.as_str()).collect();
                        lines.push(format!(
                            "    Unresolvable type '{}'; candidates: {}",
                            name,
                            cs.join(", ")
                        ));
                    }
                }
            }
        }
        writeln!(f, "{}", lines.join("\n"))
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
// verify()
// ---------------------------------------------------------------------------

/// Check all `requires` and `invariant` properties of a domain.
///
/// Returns `Ok(Verified)` if all properties pass.
/// Returns `Err(Violations)` with structured evidence if any fail.
///
/// `ensures` properties are skipped — they require a running system.
pub fn verify(domain: Domain) -> Result<Verified, Violations> {
    let mut violations: Vec<PropertyViolation> = Vec::new();

    for prop in &domain.properties.requires {
        if let Some(v) = check_property(&domain, prop, PropertyKind::Required) {
            violations.push(v);
        }
    }

    for prop in &domain.properties.invariants {
        if let Some(v) = check_property(&domain, prop, PropertyKind::Invariant) {
            violations.push(v);
        }
    }

    if violations.is_empty() {
        Ok(Verified(domain))
    } else {
        Err(Violations {
            domain: domain.name,
            violations,
        })
    }
}

// ---------------------------------------------------------------------------
// Property dispatch
// ---------------------------------------------------------------------------

fn check_property(
    domain: &Domain,
    property: &PropertyName,
    kind: PropertyKind,
) -> Option<PropertyViolation> {
    match property.as_str() {
        "connected" => check_connected(domain, property, kind),
        unknown => Some(PropertyViolation {
            domain: domain.name.clone(),
            property: property.clone(),
            kind,
            reason: format!("unknown property '{}'", unknown),
            evidence: Evidence::Unresolvable {
                name: TypeName::new(unknown),
                candidates: vec![TypeName::new("connected")],
            },
        }),
    }
}

// ---------------------------------------------------------------------------
// "connected" property check
// ---------------------------------------------------------------------------

fn check_connected(
    domain: &Domain,
    property: &PropertyName,
    kind: PropertyKind,
) -> Option<PropertyViolation> {
    // 0 or 1 types → trivially connected.
    if domain.types.len() <= 1 {
        return None;
    }

    // Build sorted type name list for stable indexing.
    let mut type_names: Vec<String> = domain
        .types
        .iter()
        .map(|t| t.name.as_str().to_owned())
        .collect();
    type_names.sort();

    // Build edges: for each parameterized variant ref, add an edge
    // from the containing type to the referenced type.
    // type_names is built from domain.types, so every typedef.name IS in type_names.
    let type_index = |name: &str| -> usize {
        type_names
            .iter()
            .position(|n| n == name)
            .expect("type_names built from domain.types — name must be present")
    };
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for typedef in &domain.types {
        let i = type_index(typedef.name.as_str());
        for variant in &typedef.variants {
            for (_param_name, type_ref) in &variant.params {
                let ref_name = type_ref.type_name().as_str();
                // Dangling refs (pointing outside declared types) are skipped.
                if let Some(j) = type_names.iter().position(|n| n == ref_name) {
                    if i != j {
                        edges.push((i, j));
                    }
                }
            }
        }
    }

    let laplacian = Laplacian::from_adjacency(&type_names, &edges);
    let num_components = laplacian.components();
    let fiedler = laplacian.fiedler_value();

    if num_components <= 1 {
        return None;
    }

    // Disconnected — gather components via union-find.
    let components = gather_components(&type_names, &edges, num_components);

    Some(PropertyViolation {
        domain: domain.name.clone(),
        property: property.clone(),
        kind,
        reason: format!(
            "type graph has {} disconnected components (Fiedler value: {:.4})",
            num_components, fiedler
        ),
        evidence: Evidence::Disconnected { components },
    })
}

// ---------------------------------------------------------------------------
// Union-find for component gathering
// ---------------------------------------------------------------------------

fn gather_components(
    type_names: &[String],
    edges: &[(usize, usize)],
    num_components: usize,
) -> Vec<Vec<TypeName>> {
    let n = type_names.len();
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, x: usize, y: usize) {
        let rx = find(parent, x);
        let ry = find(parent, y);
        if rx != ry {
            parent[rx] = ry;
        }
    }

    for &(a, b) in edges {
        union(&mut parent, a, b);
    }

    // Group indices by root.
    let mut groups: Vec<Vec<TypeName>> = vec![Vec::new(); num_components];
    let mut root_map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut next_group = 0usize;

    for (i, name) in type_names.iter().enumerate().take(n) {
        let root = find(&mut parent, i);
        let group_idx = *root_map.entry(root).or_insert_with(|| {
            let g = next_group;
            next_group += 1;
            g
        });
        groups[group_idx].push(TypeName::new(name.as_str()));
    }

    // Trim empty slots (defensive).
    groups.retain(|g| !g.is_empty());
    groups
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
    ///
    /// type color = red(shade) | blue
    /// type shade = light | dark
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
    ///
    /// type color = red | blue
    /// type shape = circle | square
    /// requires connected
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
        let domain = disconnected_domain();
        assert!(verify(domain).is_err());
    }

    #[test]
    fn verify_disconnected_has_evidence() {
        let domain = disconnected_domain();
        let err = verify(domain).unwrap_err();
        assert_eq!(err.violations.len(), 1);
        // Display includes "disconnected components" — confirms Evidence::Disconnected.
        let output = format!("{}", err);
        assert!(
            output.contains("disconnected components"),
            "expected disconnected components in display output: {}",
            output
        );
    }

    #[test]
    fn violations_display_is_readable() {
        let domain = disconnected_domain();
        let err = verify(domain).unwrap_err();
        let output = format!("{}", err);
        assert!(
            output.contains("broken"),
            "should contain domain name: {}",
            output
        );
        assert!(
            output.contains("connected"),
            "should contain property name: {}",
            output
        );
        assert!(
            output.contains("required"),
            "should contain property kind: {}",
            output
        );
        assert!(
            output.contains("component"),
            "should mention components: {}",
            output
        );
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
        // A domain with one type and "requires connected" should pass —
        // a single-node graph is trivially connected.
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
        // "invariant connected" should be checked the same way as "requires connected".
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
        assert!(matches!(err.violations[0].kind, PropertyKind::Invariant));
    }

    #[test]
    fn verify_unknown_property_fails() {
        // Unknown properties now return a violation — typos are caught.
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
        assert!(verify(domain).is_err());
    }

    #[test]
    fn verify_unknown_property_error_message() {
        let domain = Domain {
            name: DomainName::new("test"),
            types: vec![],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("conected")], // typo
                invariants: vec![],
                ensures: vec![],
            },
        };
        let result = verify(domain);
        assert!(result.is_err());
        let violations = result.unwrap_err();
        let msg = format!("{}", violations);
        assert!(
            msg.contains("unknown property"),
            "should mention unknown: {}",
            msg
        );
        assert!(msg.contains("conected"), "should mention the typo: {}", msg);
        assert!(
            msg.contains("connected"),
            "should suggest the correct name: {}",
            msg
        );
    }

    #[test]
    fn verify_ensures_skipped() {
        // "ensures" properties are not checked statically.
        // A domain with only "ensures connected" should always pass.
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

    // -----------------------------------------------------------------------
    // Coverage: Debug impls
    // -----------------------------------------------------------------------

    #[test]
    fn verified_debug_format() {
        let domain = empty_domain("dbg");
        let verified = verify(domain).unwrap();
        let s = format!("{:?}", verified);
        assert!(s.contains("Verified"), "debug output: {}", s);
    }

    #[test]
    fn violations_debug_format() {
        let domain = disconnected_domain();
        let err = verify(domain).unwrap_err();
        let s = format!("{:?}", err);
        assert!(s.contains("Violations"), "debug output: {}", s);
    }

    // -----------------------------------------------------------------------
    // Coverage: PropertyKind::Invariant and Ensures display
    // -----------------------------------------------------------------------

    #[test]
    fn property_kind_display_all_variants() {
        assert_eq!(format!("{}", PropertyKind::Required), "required");
        assert_eq!(format!("{}", PropertyKind::Invariant), "invariant");
        assert_eq!(format!("{}", PropertyKind::Ensures), "ensures");
    }

    // -----------------------------------------------------------------------
    // Coverage: Evidence::Spectral display
    // -----------------------------------------------------------------------

    #[test]
    fn violations_display_spectral_evidence() {
        let v = Violations {
            domain: DomainName::new("spectral-domain"),
            violations: vec![PropertyViolation {
                domain: DomainName::new("spectral-domain"),
                property: PropertyName::new("connected"),
                kind: PropertyKind::Required,
                reason: "test".to_owned(),
                evidence: Evidence::Spectral {
                    measure: "Fiedler value".to_owned(),
                    value: 0.0,
                    threshold: 0.01,
                },
            }],
        };
        let output = format!("{}", v);
        assert!(output.contains("Fiedler value"), "output: {}", output);
        assert!(output.contains("0.0000"), "output: {}", output);
    }

    // -----------------------------------------------------------------------
    // Coverage: Evidence::Unresolvable display (empty candidates)
    // -----------------------------------------------------------------------

    #[test]
    fn violations_display_unresolvable_no_candidates() {
        let v = Violations {
            domain: DomainName::new("unres-domain"),
            violations: vec![PropertyViolation {
                domain: DomainName::new("unres-domain"),
                property: PropertyName::new("connected"),
                kind: PropertyKind::Invariant,
                reason: "test".to_owned(),
                evidence: Evidence::Unresolvable {
                    name: TypeName::new("ghost"),
                    candidates: vec![],
                },
            }],
        };
        let output = format!("{}", v);
        assert!(output.contains("ghost"), "output: {}", output);
        assert!(output.contains("invariant"), "output: {}", output);
    }

    // -----------------------------------------------------------------------
    // Coverage: Evidence::Unresolvable display (with candidates)
    // -----------------------------------------------------------------------

    #[test]
    fn violations_display_unresolvable_with_candidates() {
        let v = Violations {
            domain: DomainName::new("maybe-domain"),
            violations: vec![PropertyViolation {
                domain: DomainName::new("maybe-domain"),
                property: PropertyName::new("connected"),
                kind: PropertyKind::Ensures,
                reason: "test".to_owned(),
                evidence: Evidence::Unresolvable {
                    name: TypeName::new("ghost"),
                    candidates: vec![TypeName::new("spirit"), TypeName::new("phantom")],
                },
            }],
        };
        let output = format!("{}", v);
        assert!(output.contains("ghost"), "output: {}", output);
        assert!(output.contains("spirit"), "output: {}", output);
        assert!(output.contains("ensures"), "output: {}", output);
    }

    // -----------------------------------------------------------------------
    // Coverage: self-referential type (i == j skip in edge building)
    // -----------------------------------------------------------------------

    #[test]
    fn verify_self_referential_type_skips_self_edges() {
        // type tree = leaf | node(tree)  — tree references itself.
        // Self-edges are skipped (i == j). With only one type, it's trivially
        // connected. This exercises the `if i != j` false branch.
        let self_ref_variant = Variant {
            name: VariantName::new("node"),
            params: vec![(
                VariantName::new("child"),
                TypeRef::new(TypeName::new("tree")),
            )],
        };
        let leaf_variant = Variant {
            name: VariantName::new("leaf"),
            params: vec![],
        };
        let tree = TypeDef {
            name: TypeName::new("tree"),
            variants: vec![leaf_variant, self_ref_variant],
        };
        let domain = Domain {
            name: DomainName::new("recursive"),
            types: vec![tree],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
        };
        // Single type — trivially connected even with self-reference.
        assert!(verify(domain).is_ok());
    }

    // -----------------------------------------------------------------------
    // Coverage: union-find with edges (3 types: a-b connected, c isolated)
    // -----------------------------------------------------------------------

    #[test]
    fn verify_three_types_two_connected_one_isolated_fails() {
        // type a = x(b) — a references b, so a-b are connected.
        // type b = y     — no refs out.
        // type c = z     — isolated.
        // requires connected → should fail with 2 components: {a,b} and {c}.
        // This exercises union-find with edges present (lines 281-291).
        let variant_x = Variant {
            name: VariantName::new("x"),
            params: vec![(VariantName::new("x"), TypeRef::new(TypeName::new("b")))],
        };
        let type_a = TypeDef {
            name: TypeName::new("a"),
            variants: vec![variant_x],
        };
        let type_b = type_def_simple("b", &["y"]);
        let type_c = type_def_simple("c", &["z"]);

        let domain = Domain {
            name: DomainName::new("partial"),
            types: vec![type_a, type_b, type_c],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
        };
        let err = verify(domain).unwrap_err();
        assert_eq!(err.violations.len(), 1);
        // Display includes the component count: "2 disconnected components".
        let output = format!("{}", err);
        assert!(
            output.contains("2 disconnected components"),
            "expected 2 components in output: {}",
            output
        );
    }

    // -----------------------------------------------------------------------
    // Coverage: dangling type ref (variant points to undeclared type — skipped)
    // -----------------------------------------------------------------------

    #[test]
    fn verify_dangling_type_ref_skipped_and_domain_disconnected() {
        // type color = red(ghost) | blue  — ghost is not declared.
        // type shape = circle | square
        // The dangling ref to "ghost" is skipped. color-shape are unconnected.
        // requires connected → fails with 2 components.
        // This exercises the `if let Some(j) = ... { }` false branch (line ~243).
        let dangling_variant = Variant {
            name: VariantName::new("red"),
            params: vec![(
                VariantName::new("ref"),
                TypeRef::new(TypeName::new("ghost")), // "ghost" not in types
            )],
        };
        let blue_variant = Variant {
            name: VariantName::new("blue"),
            params: vec![],
        };
        let color = TypeDef {
            name: TypeName::new("color"),
            variants: vec![dangling_variant, blue_variant],
        };
        let shape = type_def_simple("shape", &["circle"]);

        let domain = Domain {
            name: DomainName::new("dangling"),
            types: vec![color, shape],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
        };
        // Dangling ref is ignored; color and shape are unconnected → fails.
        assert!(verify(domain).is_err());
    }
}
