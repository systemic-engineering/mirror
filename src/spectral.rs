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

use coincidence::spectral::{Laplacian, SpectralDistance, Spectrum};

use crate::ast::AstNode;
use crate::prism::Prism;
use crate::resolve::TypeRegistry;

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
fn laplacian_from_edges(
    type_names: &[String],
    edges: &[(usize, usize)],
) -> Laplacian {
    Laplacian::from_adjacency(type_names, edges)
}

impl TypeGraphSpectrum {
    /// Compute the spectrum of a grammar's type reference graph.
    ///
    /// Returns None if the grammar has no types (empty spectrum is meaningless).
    pub fn from_registry(registry: &TypeRegistry) -> Option<Self> {
        let type_names: Vec<String> = {
            let mut names: Vec<String> = registry.type_names().iter().map(|s| s.to_string()).collect();
            names.sort();
            names
        };

        if type_names.is_empty() {
            return None;
        }

        // Build edges from parameterized variant refs
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (i, type_name) in type_names.iter().enumerate() {
            for variant in registry.variants(type_name).unwrap_or_default() {
                if let Some(ref_type) = registry.variant_param(type_name, variant) {
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
        let registry = TypeRegistry::compile(&simple_grammar()).unwrap();
        let tgs = TypeGraphSpectrum::from_registry(&registry).unwrap();
        // Single type, no references → 1 component, connectivity 0
        assert_eq!(tgs.components(), 1);
        assert!(tgs.connectivity().abs() < 1e-9);
    }

    #[test]
    fn type_graph_with_refs_connected() {
        let registry = TypeRegistry::compile(&linked_grammar_ast()).unwrap();
        let tgs = TypeGraphSpectrum::from_registry(&registry).unwrap();
        // color → shade reference → connected
        assert_eq!(tgs.components(), 1);
        assert!(tgs.connectivity() > 0.0);
    }

    #[test]
    fn type_graph_distance_self_zero() {
        let reg = TypeRegistry::compile(&linked_grammar_ast()).unwrap();
        let tgs1 = TypeGraphSpectrum::from_registry(&reg).unwrap();
        let tgs2 = TypeGraphSpectrum::from_registry(&reg).unwrap();
        assert!(tgs1.distance(&tgs2).value() < 1e-9);
    }

    #[test]
    fn type_graph_distance_different_nonzero() {
        let reg1 = TypeRegistry::compile(&simple_grammar()).unwrap();
        let reg2 = TypeRegistry::compile(&linked_grammar_ast()).unwrap();
        let tgs1 = TypeGraphSpectrum::from_registry(&reg1).unwrap();
        let tgs2 = TypeGraphSpectrum::from_registry(&reg2).unwrap();
        assert!(tgs1.distance(&tgs2).value() > 0.0);
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
        let registry = TypeRegistry::compile(&grammar).unwrap();
        assert!(TypeGraphSpectrum::from_registry(&registry).is_none());
    }
}
