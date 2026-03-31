//! Spectral — grammar geometry via the coincidence crate.
//!
//! The grammar's AST is a `Prism<AstNode>` which implements `Fragmentable`.
//! The coincidence crate's `Laplacian::from_tree()` accepts any `Fragmentable`.
//! This module bridges the two: feed grammar trees to spectral analysis.
//!
//! # What the spectrum tells us
//!
//! - **Fiedler value** (second smallest eigenvalue): algebraic connectivity.
//!   Higher = more tightly connected type surface. A grammar with many
//!   cross-references between types is more "rigid" than one with isolated types.
//!
//! - **Spectral distance**: how far apart two grammars are in geometry space.
//!   Same grammar shape → distance 0. Different topology → nonzero.
//!
//! - **Component count**: number of near-zero eigenvalues. A grammar with
//!   disconnected type namespaces has multiple components.
//!
//! # Where the math breaks
//!
//! The spectral analysis operates on the AST tree structure, NOT on the
//! type reference graph. The Laplacian captures parent-child containment
//! edges (how the grammar is parsed), not type-level dependencies (how
//! types reference each other). These are different graphs:
//!
//! - AST graph: `grammar → type-def → variant → type-ref` (syntactic)
//! - Type graph: `color → shade` (semantic, via parameterized variants)
//!
//! The AST spectrum tells you about syntactic shape. The type graph
//! spectrum would tell you about semantic shape. Both are interesting.
//! This module provides both.

use coincidence::projection::Projection;
use coincidence::spectral::{Laplacian, SpectralDistance, Spectrum};
use coincidence::state::StateVector;

use crate::ast::AstNode;
use crate::prism::Prism;
use crate::model::Domain;

/// Spectral analysis of a grammar's AST tree.
///
/// The AST is a `Prism<AstNode>` (Fragmentable). The Laplacian
/// captures the syntactic structure: how declarations, types,
/// variants, and references are nested.
#[derive(Clone, Debug)]
pub struct GrammarSpectrum {
    /// The AST Laplacian.
    pub laplacian: Laplacian,
    /// The eigenvalue spectrum.
    pub spectrum: Spectrum,
}

impl GrammarSpectrum {
    /// Compute the spectrum of a grammar AST.
    pub fn from_ast(grammar: &Prism<AstNode>) -> Self {
        let laplacian = Laplacian::from_tree(grammar);
        let spectrum = laplacian.spectrum();
        GrammarSpectrum {
            laplacian,
            spectrum,
        }
    }

    /// Spectral distance to another grammar.
    pub fn distance(&self, other: &GrammarSpectrum) -> SpectralDistance {
        self.spectrum.distance(&other.spectrum)
    }

    /// Algebraic connectivity (Fiedler value).
    /// Higher values indicate a more tightly connected AST.
    pub fn connectivity(&self) -> f64 {
        self.laplacian.fiedler_value()
    }

    /// Number of connected components in the AST.
    /// Should always be 1 for a well-formed grammar.
    pub fn components(&self) -> usize {
        self.laplacian.components()
    }

    /// Number of nodes in the AST.
    pub fn node_count(&self) -> usize {
        self.laplacian.n()
    }
}

/// Spectral analysis of a grammar's type reference graph.
///
/// This is the SEMANTIC graph: types connected by parameterized
/// variant references. Different from the AST graph.
///
/// Built from the ReachabilityMap's edge structure, not from the
/// Fragmentable tree. The Laplacian is constructed manually from
/// the type-level adjacency.
#[derive(Clone, Debug)]
pub struct TypeGraphSpectrum {
    /// The type-level Laplacian.
    pub laplacian: Laplacian,
    /// The eigenvalue spectrum.
    pub spectrum: Spectrum,
}

/// Build a Laplacian directly from type names and edges.
///
/// This constructs the graph Laplacian for the type reference
/// graph, which is NOT the same as the AST tree's Laplacian.
fn laplacian_from_edges(type_names: &[String], edges: &[(usize, usize)]) -> Laplacian {
    Laplacian::from_adjacency(type_names, edges)
}

impl TypeGraphSpectrum {
    /// Compute the spectrum of a grammar's type reference graph.
    ///
    /// Returns None if the grammar has no types (empty spectrum is meaningless).
    pub fn from_domain(domain: &Domain) -> Option<Self> {
        let type_names: Vec<String> = {
            let mut names: Vec<String> = domain
                .type_names()
                .iter()
                .map(|s| s.to_string())
                .collect();
            names.sort();
            names
        };

        if type_names.is_empty() {
            return None;
        }

        // Build edges from parameterized variant refs
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (i, type_name) in type_names.iter().enumerate() {
            for variant in domain.variants(type_name).unwrap_or_default() {
                if let Some(ref_type) = domain.variant_param(type_name, variant) {
                    if let Some(j) = type_names.iter().position(|t| t == ref_type) {
                        edges.push((i, j));
                    }
                }
            }
        }

        let laplacian = laplacian_from_edges(&type_names, &edges);
        let spectrum = laplacian.spectrum();

        Some(TypeGraphSpectrum {
            laplacian,
            spectrum,
        })
    }

    /// Spectral distance to another type graph.
    pub fn distance(&self, other: &TypeGraphSpectrum) -> SpectralDistance {
        self.spectrum.distance(&other.spectrum)
    }

    /// Algebraic connectivity (Fiedler value).
    /// Positive = connected type surface. Zero = disconnected types.
    pub fn connectivity(&self) -> f64 {
        self.laplacian.fiedler_value()
    }

    /// Number of connected components.
    /// Multiple components = independent type namespaces.
    pub fn components(&self) -> usize {
        self.laplacian.components()
    }
}

// ---------------------------------------------------------------------------
// Grammar Projection — the type surface as a measurement apparatus
// ---------------------------------------------------------------------------

/// The grammar's type surface as a projection operator.
///
/// # How grammars project
///
/// A grammar with types `{color, shade}` where `color = red | blue`
/// and `shade = light | dark` defines a 4-dimensional space:
///
///   `{color.red, color.blue, shade.light, shade.dark}`
///
/// Each variant is a basis vector. A typed value like `color.red`
/// is the basis vector `|color.red>`. The grammar's projection
/// operator P satisfies:
///
/// - P|color.red> = |color.red> (data that matches stays)
/// - P|unknown>   = 0           (data outside the type surface vanishes)
/// - P^2 = P                    (idempotent — projecting twice = projecting once)
///
/// The projection is the grammar's identity in the coincidence framework.
/// Two grammars with the same type surface have the same projection.
/// Different type surfaces → different projections → measurable distance.
///
/// # Where the math breaks (part 2)
///
/// This projection treats all variants as orthogonal. In reality,
/// `color.red(shade.light)` has a dependency between dimensions —
/// `red` constrains the `shade` parameter. The product space structure
/// is richer than a flat projection captures. That's the gap between
/// the flat variant space and the dependent type space.
///
/// A more precise model would use tensor products: `|color.red> ⊗ |shade.light>`.
/// This module stays flat. The tensor product is the next break to push through.
#[derive(Clone, Debug)]
pub struct GrammarProjection {
    /// The domain name.
    pub domain: String,
    /// The projection operator.
    pub projection: Projection,
    /// Dimension labels: `type.variant` strings.
    pub labels: Vec<String>,
}

impl GrammarProjection {
    /// Build the grammar projection from a Domain.
    ///
    /// Returns None if the grammar has no types (no space to project into).
    pub fn from_domain(domain: &Domain) -> Option<Self> {
        let mut labels: Vec<String> = Vec::new();

        let mut type_names: Vec<String> = domain
            .type_names()
            .iter()
            .map(|s| s.to_string())
            .collect();
        type_names.sort();

        if type_names.is_empty() {
            return None;
        }

        for type_name in &type_names {
            let mut variants: Vec<String> = domain
                .variants(type_name)
                .unwrap_or_default()
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            variants.sort();
            for variant in &variants {
                labels.push(format!("{}.{}", type_name, variant));
            }
        }

        // Edge case: types exist but have no variants
        if labels.is_empty() {
            return None;
        }

        let space = format!("grammar:{}", domain.domain_name());

        // Build the identity projection: every variant basis vector is preserved.
        // This is P = sum_i |e_i><e_i| over all variant basis vectors.
        let entries: Vec<((String, String), f64)> = labels
            .iter()
            .map(|l| ((l.clone(), l.clone()), 1.0))
            .collect();

        let projection = Projection::from_entries(&space, labels.clone(), entries);

        Some(GrammarProjection {
            domain: domain.domain_name().to_string(),
            projection,
            labels,
        })
    }

    /// The dimension of the grammar's type space.
    /// Equal to the total number of variants across all types.
    pub fn dimension(&self) -> usize {
        self.labels.len()
    }

    /// The space name for this grammar's projection.
    pub fn space(&self) -> &str {
        self.projection.space()
    }

    /// Project a state vector through this grammar.
    ///
    /// Data matching the grammar's type surface is preserved.
    /// Data outside the type surface is projected to zero.
    pub fn project(&self, v: &StateVector) -> Result<StateVector, coincidence::CoincidenceError> {
        self.projection.apply(v)
    }

    /// Create a basis state vector for a specific variant.
    ///
    /// Returns None if the variant label doesn't exist in this grammar.
    pub fn basis(&self, type_name: &str, variant: &str) -> Option<StateVector> {
        let label = format!("{}.{}", type_name, variant);
        if self.labels.contains(&label) {
            Some(StateVector::basis(label, self.space()))
        } else {
            None
        }
    }

    /// Create a uniform superposition over all variants of a type.
    ///
    /// For a type with n variants, each variant gets coefficient 1/sqrt(n).
    /// Returns None if the type doesn't exist or has no variants.
    pub fn type_superposition(&self, type_name: &str) -> Option<StateVector> {
        let prefix = format!("{}.", type_name);
        let matching: Vec<&String> = self
            .labels
            .iter()
            .filter(|l| l.starts_with(&prefix))
            .collect();
        if matching.is_empty() {
            return None;
        }
        let coeff = 1.0 / (matching.len() as f64).sqrt();
        let entries: Vec<(String, f64)> =
            matching.into_iter().map(|l| (l.clone(), coeff)).collect();
        Some(StateVector::from_entries(self.space(), entries))
    }

    /// Check if this grammar's projection is idempotent (it always should be).
    pub fn verify_idempotent(&self) -> bool {
        self.projection.is_idempotent(1e-10)
    }

    /// Serialize the projection as ETF bytes for BEAM consumption.
    ///
    /// The projection becomes an Erlang proplist:
    /// ```erlang
    /// [{space, <<"grammar:test">>},
    ///  {dimension, 3},
    ///  {labels, [<<"type.a">>, <<"type.b">>, ...]},
    ///  {entries, [{<<"type.a">>, <<"type.a">>, 1.0}, ...]}]
    /// ```
    pub fn to_etf(&self) -> Vec<u8> {
        use eetf::{Atom, FixInteger, Float, List, Term, Tuple};

        let space_pair = Term::from(Tuple::from(vec![
            Term::from(Atom::from("space")),
            Term::from(eetf::Binary::from(self.space().as_bytes())),
        ]));

        let dim_pair = Term::from(Tuple::from(vec![
            Term::from(Atom::from("dimension")),
            Term::from(FixInteger::from(self.dimension() as i32)),
        ]));

        let label_terms: Vec<Term> = self
            .labels
            .iter()
            .map(|l| Term::from(eetf::Binary::from(l.as_bytes())))
            .collect();
        let labels_pair = Term::from(Tuple::from(vec![
            Term::from(Atom::from("labels")),
            Term::from(List::from(label_terms)),
        ]));

        // Entries: the diagonal of the projection matrix.
        // For the identity projection, each entry is {label, label, 1.0}.
        //
        // DESIGN BREAK: This is the flat projection. Each variant is an
        // independent basis vector. For `color.red(shade)`, the projection
        // says "color.red is valid" and "shade.light is valid" separately.
        // It CANNOT say "color.red requires shade.light or shade.dark" —
        // that's a constraint between dimensions, not a diagonal entry.
        //
        // To express dependent types, we need:
        //   |color.red> tensor |shade.light>  and  |color.red> tensor |shade.dark>
        // as joint basis vectors in a product space of dimension n*m.
        //
        // The flat projection has dimension n+m (sum of variant counts).
        // The tensor projection has dimension n*m (product of variant counts).
        //
        // For dispatch, the flat projection is a membership check.
        // For type-safe dispatch, the tensor projection is a joint check.
        // The GenServer can use the flat projection for "does this type exist?"
        // but NOT for "is this combination of type and parameter valid?"
        //
        // That's the wall. The flat model answers set membership.
        // The tensor model answers constraint satisfaction.
        // Grammar semantics require constraint satisfaction.
        let entry_terms: Vec<Term> = self
            .labels
            .iter()
            .map(|l| {
                Term::from(Tuple::from(vec![
                    Term::from(eetf::Binary::from(l.as_bytes())),
                    Term::from(eetf::Binary::from(l.as_bytes())),
                    Term::from(Float { value: 1.0 }),
                ]))
            })
            .collect();
        let entries_pair = Term::from(Tuple::from(vec![
            Term::from(Atom::from("entries")),
            Term::from(List::from(entry_terms)),
        ]));

        let proplist = Term::from(List::from(vec![
            space_pair,
            dim_pair,
            labels_pair,
            entries_pair,
        ]));

        let mut buf = Vec::new();
        proplist
            .encode(&mut buf)
            .expect("ETF encoding should not fail");
        buf
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

    fn span() -> Span {
        Span::new(0, 0)
    }

    fn ref_(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    /// Simple grammar: `grammar @test { type = a | b | c }`
    fn simple_grammar() -> Prism<AstNode> {
        let variants: Vec<Prism<AstNode>> = ["a", "b", "c"]
            .iter()
            .map(|v| {
                prism::shard(
                    ref_(&format!("variant-{}", v)),
                    AstNode {
                        kind: Kind::Form,
                        name: "variant".into(),
                        value: v.to_string(),
                        span: span(),
                    },
                )
            })
            .collect();

        let type_def = prism::fractal(
            ref_("type-def-default"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "".into(),
                span: span(),
            },
            variants,
        );

        prism::fractal(
            ref_("grammar-test"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@test".into(),
                span: span(),
            },
            vec![type_def],
        )
    }

    /// Different grammar: `grammar @other { type = x | y }`
    fn other_grammar() -> Prism<AstNode> {
        let variants: Vec<Prism<AstNode>> = ["x", "y"]
            .iter()
            .map(|v| {
                prism::shard(
                    ref_(&format!("variant-{}", v)),
                    AstNode {
                        kind: Kind::Form,
                        name: "variant".into(),
                        value: v.to_string(),
                        span: span(),
                    },
                )
            })
            .collect();

        let type_def = prism::fractal(
            ref_("type-def-other"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "".into(),
                span: span(),
            },
            variants,
        );

        prism::fractal(
            ref_("grammar-other"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@other".into(),
                span: span(),
            },
            vec![type_def],
        )
    }

    /// Grammar with parameterized refs:
    /// `grammar @linked { type color = red(shade) | blue  type shade = light | dark }`
    fn linked_grammar_ast() -> Prism<AstNode> {
        let ref_shade = prism::shard(
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
            vec![ref_shade],
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

        let shade_variants: Vec<Prism<AstNode>> = ["light", "dark"]
            .iter()
            .map(|v| {
                prism::shard(
                    ref_(&format!("variant-{}", v)),
                    AstNode {
                        kind: Kind::Form,
                        name: "variant".into(),
                        value: v.to_string(),
                        span: span(),
                    },
                )
            })
            .collect();
        let shade_type = prism::fractal(
            ref_("type-def-shade"),
            AstNode {
                kind: Kind::Form,
                name: "type-def".into(),
                value: "shade".into(),
                span: span(),
            },
            shade_variants,
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

    // -- AST spectrum tests --

    #[test]
    fn grammar_spectrum_single_grammar() {
        let gs = GrammarSpectrum::from_ast(&simple_grammar());
        // grammar → type-def → {a, b, c} = 5 nodes
        assert_eq!(gs.node_count(), 5);
        assert_eq!(gs.components(), 1);
        assert!(gs.connectivity() > 0.0);
    }

    #[test]
    fn grammar_spectrum_distance_self_is_zero() {
        let gs1 = GrammarSpectrum::from_ast(&simple_grammar());
        let gs2 = GrammarSpectrum::from_ast(&simple_grammar());
        assert!(gs1.distance(&gs2).value() < 1e-9);
    }

    #[test]
    fn grammar_spectrum_different_grammars_nonzero_distance() {
        let gs1 = GrammarSpectrum::from_ast(&simple_grammar());
        let gs2 = GrammarSpectrum::from_ast(&other_grammar());
        assert!(gs1.distance(&gs2).value() > 0.0);
    }

    #[test]
    fn grammar_spectrum_linked_grammar() {
        let gs = GrammarSpectrum::from_ast(&linked_grammar_ast());
        // grammar → color{red{ref}, blue} + shade{light, dark} = 8 nodes
        assert_eq!(gs.node_count(), 8);
        assert_eq!(gs.components(), 1);
    }

    // -- Type graph spectrum tests --

    #[test]
    fn type_graph_no_refs_disconnected() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let tgs = TypeGraphSpectrum::from_domain(&registry).unwrap();
        // Single type, no references → 1 component, connectivity 0
        assert_eq!(tgs.components(), 1);
        assert!(tgs.connectivity().abs() < 1e-9);
    }

    #[test]
    fn type_graph_with_refs_connected() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let tgs = TypeGraphSpectrum::from_domain(&registry).unwrap();
        // color → shade reference → connected
        assert_eq!(tgs.components(), 1);
        assert!(tgs.connectivity() > 0.0);
    }

    #[test]
    fn type_graph_distance_self_zero() {
        let reg = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let tgs1 = TypeGraphSpectrum::from_domain(&reg).unwrap();
        let tgs2 = TypeGraphSpectrum::from_domain(&reg).unwrap();
        assert!(tgs1.distance(&tgs2).value() < 1e-9);
    }

    #[test]
    fn type_graph_distance_different_nonzero() {
        let reg1 = Domain::from_grammar(&simple_grammar()).unwrap();
        let reg2 = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let tgs1 = TypeGraphSpectrum::from_domain(&reg1).unwrap();
        let tgs2 = TypeGraphSpectrum::from_domain(&reg2).unwrap();
        assert!(tgs1.distance(&tgs2).value() > 0.0);
    }

    // -- Grammar projection tests --

    #[test]
    fn grammar_projection_simple() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        // type "" has variants {a, b, c} → 3 dimensions
        assert_eq!(gp.dimension(), 3);
        assert!(gp.verify_idempotent());
    }

    #[test]
    fn grammar_projection_linked() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        // color={red,blue} + shade={light,dark} → 4 dimensions
        assert_eq!(gp.dimension(), 4);
        assert!(gp.verify_idempotent());
    }

    #[test]
    fn grammar_projection_basis_vector() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();

        let v = gp.basis("color", "red").unwrap();
        let projected = gp.project(&v).unwrap();
        // Basis vector should survive projection unchanged
        assert_eq!(projected.entries().len(), 1);
        assert_eq!(projected.entries().get("color.red"), Some(&1.0));
    }

    #[test]
    fn grammar_projection_unknown_variant_returns_none() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        assert!(gp.basis("unknown", "nope").is_none());
    }

    #[test]
    fn grammar_projection_projects_outside_to_zero() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();

        // State vector outside the grammar's type space
        let outside = StateVector::basis("alien.thing", gp.space());
        let projected = gp.project(&outside).unwrap();
        assert!(projected.is_zero());
    }

    #[test]
    fn grammar_projection_idempotent_application() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();

        let v = gp.basis("shade", "dark").unwrap();
        let once = gp.project(&v).unwrap();
        let twice = gp.project(&once).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn grammar_projection_type_superposition() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();

        let sup = gp.type_superposition("color").unwrap();
        // color has 2 variants → coefficient = 1/sqrt(2) for each
        let expected_coeff = 1.0 / 2.0f64.sqrt();
        assert!((sup.entries().get("color.blue").unwrap() - expected_coeff).abs() < 1e-9);
        assert!((sup.entries().get("color.red").unwrap() - expected_coeff).abs() < 1e-9);
    }

    #[test]
    fn grammar_projection_superposition_unknown_type() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        assert!(gp.type_superposition("nonexistent").is_none());
    }

    #[test]
    fn grammar_projection_superposition_survives_projection() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();

        let sup = gp.type_superposition("shade").unwrap();
        let projected = gp.project(&sup).unwrap();
        // Superposition is within the grammar's space, so it should survive
        let expected_coeff = 1.0 / 2.0f64.sqrt();
        assert!((projected.entries().get("shade.dark").unwrap() - expected_coeff).abs() < 1e-9);
        assert!((projected.entries().get("shade.light").unwrap() - expected_coeff).abs() < 1e-9);
    }

    #[test]
    fn grammar_projection_space_name() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        assert_eq!(gp.space(), "grammar:test");
    }

    #[test]
    fn grammar_projection_empty_grammar_returns_none() {
        let grammar = prism::fractal(
            ref_("grammar-empty"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@empty".into(),
                span: span(),
            },
            vec![],
        );
        let registry = Domain::from_grammar(&grammar).unwrap();
        assert!(GrammarProjection::from_domain(&registry).is_none());
    }

    #[test]
    fn grammar_projection_empty_type_returns_none() {
        // Type exists but has zero variants
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
            ref_("grammar-empty-type"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@evac".into(),
                span: span(),
            },
            vec![type_def],
        );
        let registry = Domain::from_grammar(&grammar).unwrap();
        assert!(GrammarProjection::from_domain(&registry).is_none());
    }

    #[test]
    fn grammar_projection_domain_stored() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        assert_eq!(gp.domain, "linked");
    }

    // -----------------------------------------------------------------------
    // Projection ETF serialization tests
    // -----------------------------------------------------------------------

    #[test]
    fn grammar_projection_to_etf_simple() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        let etf = gp.to_etf();
        // Should be valid ETF
        assert!(!etf.is_empty());
        assert_eq!(etf[0], 131, "should start with ETF version byte");
        // Should be decodable
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let s = format!("{:?}", term);
        // Should contain space name
        assert!(s.contains("space"), "should have space key");
        // Should contain labels
        assert!(s.contains("labels"), "should have labels key");
        // Should contain dimension
        assert!(s.contains("dimension"), "should have dimension key");
    }

    #[test]
    fn grammar_projection_to_etf_linked() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        let etf = gp.to_etf();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let s = format!("{:?}", term);
        // Should contain the variant labels as bytes
        let red_bytes: Vec<u8> = "color.red".bytes().collect();
        assert!(
            s.contains(&format!("{:?}", red_bytes)),
            "should contain color.red label"
        );
    }

    #[test]
    fn grammar_projection_to_etf_deterministic() {
        let registry = Domain::from_grammar(&simple_grammar()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        let a = gp.to_etf();
        let b = gp.to_etf();
        assert_eq!(a, b);
    }

    #[test]
    fn grammar_projection_to_etf_contains_entries() {
        let registry = Domain::from_grammar(&linked_grammar_ast()).unwrap();
        let gp = GrammarProjection::from_domain(&registry).unwrap();
        let etf = gp.to_etf();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let s = format!("{:?}", term);
        // Should contain projection entries
        assert!(s.contains("entries"), "should have entries key: {}", s);
    }

    #[test]
    fn type_graph_empty_grammar_returns_none() {
        // Grammar with no type-def children → empty types map
        let grammar = prism::fractal(
            ref_("grammar-empty"),
            AstNode {
                kind: Kind::Decl,
                name: "grammar".into(),
                value: "@empty".into(),
                span: span(),
            },
            vec![],
        );
        let registry = Domain::from_grammar(&grammar).unwrap();
        assert!(TypeGraphSpectrum::from_domain(&registry).is_none());
    }
}
