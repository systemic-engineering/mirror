//! 16-dimensional spectral feature extractor for Mirror training.
//!
//! Maps a grammar's AST and domain model to a fixed-length vector
//! suitable for Fate's FEATURE_DIM = 16 classifier.
//!
//! ## Dimensions
//!
//! Dimensions 0–7 are declaration dimensions (static grammar structure).
//! Dimensions 8–15 are history dimensions (spectral + projection + derivation).
//!
//! | Dim | Name                  | Source                                          | Range  |
//! |-----|-----------------------|-------------------------------------------------|--------|
//! | 0   | node_count_norm       | AST node count / 100                            | [0, 1] |
//! | 1   | duplicate_ratio       | duplicate OIDs / total nodes                    | [0, 1] |
//! | 2   | crystal_def_ratio     | type-def nodes / total declarations             | [0, 1] |
//! | 3   | prefix_entropy        | namespace prefix Shannon entropy, normalized     | [0, 1] |
//! | 4   | density               | AST edge density (edges / max_edges)            | [0, 1] |
//! | 5   | type_count_norm       | distinct type count / 20                        | [0, 1] |
//! | 6   | variant_count_norm    | total variants / 100                            | [0, 1] |
//! | 7   | ref_ratio             | parameterized variants / total variants         | [0, 1] |
//! | 8   | ast_fiedler           | GrammarSpectrum.connectivity(), clamped [0,2]/2 | [0, 1] |
//! | 9   | ast_components_inv    | 1.0 / GrammarSpectrum.components()              | (0, 1] |
//! | 10  | type_fiedler          | TypeGraphSpectrum.connectivity() if available   | [0, 1] |
//! | 11  | type_connected        | 1.0 if type graph has 1 component, else 0.0     | {0, 1} |
//! | 12  | projection_dim_norm   | GrammarProjection.dimension() / 50              | [0, 1] |
//! | 13  | projection_idempotent | 1.0 if idempotent check passes, else 0.0        | {0, 1} |
//! | 14  | derivation_count_norm | derive_all().len() / 200                        | [0, 1] |
//! | 15  | shannon_equivalence   | 1.0 if all derivation OIDs unique, else 0.0     | {0, 1} |

use std::collections::{HashMap, HashSet};

use crate::ast::AstNode;
use crate::generate;
use crate::model::Mirror;
use crate::parse::Parse;
use crate::prism::{self, Prism};
use crate::property::{self, Verdict};
use crate::spectral::{GrammarProjection, GrammarSpectrum, TypeGraphSpectrum};
use crate::Vector;

// ---------------------------------------------------------------------------
// Public surface
// ---------------------------------------------------------------------------

/// The number of feature dimensions. Matches Fate's FEATURE_DIM.
pub const FEATURE_DIM: usize = 16;

/// A fixed-length feature vector.
pub type Features = [f64; FEATURE_DIM];

/// Parse `source` and extract features.
///
/// Returns `[0.0; 16]` if parsing or domain construction fails.
pub fn extract_from_source(source: &str) -> Features {
    let trace = Parse.trace(source.to_string());
    let ast = match trace.into_result() {
        Ok(tree) => tree,
        Err(_) => return [0.0; FEATURE_DIM],
    };

    // The top-level tree is a document; find the grammar child.
    let grammar_node = ast.children().iter().find(|c| c.data().is_decl("grammar"));

    let grammar = match grammar_node {
        Some(g) => g,
        None => return [0.0; FEATURE_DIM],
    };

    let domain = match Mirror::from_grammar(grammar) {
        Ok(d) => d,
        Err(_) => return [0.0; FEATURE_DIM],
    };

    extract(&domain, grammar)
}

/// Extract features from an already-parsed grammar AST and its domain model.
pub fn extract(domain: &Mirror, ast: &Prism<AstNode>) -> Features {
    let mut f = [0.0f64; FEATURE_DIM];

    // ---
    // Dims 0–4: AST structure
    // ---

    let all_nodes: Vec<&Prism<AstNode>> = walk_collect(ast);
    let node_count = all_nodes.len();

    // 0: node_count_norm
    f[0] = clamp01(node_count as f64 / 100.0);

    // 1: duplicate_ratio — duplicate content OIDs / total nodes
    {
        let mut seen: HashSet<String> = HashSet::new();
        let mut dupes = 0usize;
        for node in &all_nodes {
            let oid = prism::content_oid(*node);
            if !seen.insert(oid) {
                dupes += 1;
            }
        }
        f[1] = if node_count > 0 {
            clamp01(dupes as f64 / node_count as f64)
        } else {
            0.0
        };
    }

    // 2: crystal_def_ratio — type-def nodes / all "form" or "decl" nodes
    {
        let total_decls: usize = all_nodes
            .iter()
            .filter(|n| {
                let d = n.data();
                d.is_decl("grammar")
                    || d.is_form("type-def")
                    || d.is_form("action-def")
                    || d.is_atom("crystal")
            })
            .count();
        let type_defs: usize = all_nodes
            .iter()
            .filter(|n| n.data().is_form("type-def"))
            .count();
        f[2] = if total_decls > 0 {
            clamp01(type_defs as f64 / total_decls as f64)
        } else {
            0.0
        };
    }

    // 3: prefix_entropy — Shannon entropy over namespace prefixes
    {
        let entropy = prefix_entropy(domain);
        // Maximum entropy for n prefixes is log2(n). Normalise against log2(20) ~ 4.32.
        f[3] = clamp01(entropy / 4.321928094887362);
    }

    // 4: density — AST edge density (tree: edges = n-1, max_edges = n*(n-1)/2)
    {
        if node_count > 1 {
            let edges = node_count - 1;
            let max_edges = node_count * (node_count - 1) / 2;
            f[4] = clamp01(edges as f64 / max_edges as f64);
        } else {
            f[4] = 0.0;
        }
    }

    // ---
    // Dims 5–7: type surface
    // ---

    // 5: type_count_norm
    let type_count = domain.type_names().len();
    f[5] = clamp01(type_count as f64 / 20.0);

    // 6: variant_count_norm
    let total_variants: usize = domain
        .type_names()
        .iter()
        .map(|tn| domain.variants(tn).unwrap_or_default().len())
        .sum();
    f[6] = clamp01(total_variants as f64 / 100.0);

    // 7: ref_ratio — parameterized variants / total variants
    {
        let parameterized: usize = domain
            .type_names()
            .iter()
            .flat_map(|tn| {
                domain
                    .variants(tn)
                    .unwrap_or_default()
                    .into_iter()
                    .map(move |v| (tn, v))
            })
            .filter(|(tn, v)| domain.variant_param(tn, v).is_some())
            .count();
        f[7] = if total_variants > 0 {
            clamp01(parameterized as f64 / total_variants as f64)
        } else {
            0.0
        };
    }

    // ---
    // Dims 8–9: AST spectral
    // ---

    let gs = GrammarSpectrum::from_ast(ast);

    // 8: ast_fiedler — connectivity clamped to [0, 2] then / 2
    f[8] = clamp01(gs.connectivity().clamp(0.0, 2.0) / 2.0);

    // 9: ast_components_inv — 1 / component_count
    {
        let comps = gs.components().max(1);
        f[9] = clamp01(1.0 / comps as f64);
    }

    // ---
    // Dims 10–11: type graph spectral
    // ---

    match TypeGraphSpectrum::from_domain(domain) {
        Some(tgs) => {
            // 10: type_fiedler — connectivity clamped [0, 2] / 2
            f[10] = clamp01(tgs.connectivity().clamp(0.0, 2.0) / 2.0);
            // 11: type_connected — 1.0 if single component
            f[11] = if tgs.components() == 1 { 1.0 } else { 0.0 };
        }
        None => {
            // No type graph (empty grammar) — neutral
            f[10] = 0.0;
            f[11] = 0.0;
        }
    }

    // ---
    // Dims 12–13: projection
    // ---

    match GrammarProjection::from_domain(domain) {
        Some(gp) => {
            // 12: projection_dim_norm
            f[12] = clamp01(gp.dimension() as f64 / 50.0);
            // 13: projection_idempotent
            f[13] = if gp.verify_idempotent() { 1.0 } else { 0.0 };
        }
        None => {
            f[12] = 0.0;
            f[13] = 0.0;
        }
    }

    // ---
    // Dims 14–15: derivation
    // ---

    let derivations = generate::derive_all(domain);

    // 14: derivation_count_norm
    f[14] = clamp01(derivations.len() as f64 / 200.0);

    // 15: shannon_equivalence — 1.0 if all derivation OIDs are unique
    f[15] = match property::shannon_equivalence(&derivations) {
        Verdict::Pass => 1.0,
        Verdict::Fail(_) => 0.0,
    };

    f
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Clamp a value to [0.0, 1.0].
#[inline]
fn clamp01(v: f64) -> f64 {
    v.clamp(0.0, 1.0)
}

/// Collect all nodes in the tree via depth-first traversal.
fn walk_collect(root: &Prism<AstNode>) -> Vec<&Prism<AstNode>> {
    let mut out = Vec::new();
    walk_dfs(root, &mut out);
    out
}

fn walk_dfs<'a>(node: &'a Prism<AstNode>, out: &mut Vec<&'a Prism<AstNode>>) {
    out.push(node);
    for child in node.children() {
        walk_dfs(child, out);
    }
}

/// Shannon entropy over the domain's namespace prefixes.
///
/// Each type name is split on `.` and the first segment (prefix) is counted.
/// Entropy = -sum(p * log2(p)).
fn prefix_entropy(domain: &Mirror) -> f64 {
    let type_names = domain.type_names();
    if type_names.is_empty() {
        return 0.0;
    }

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for name in &type_names {
        let prefix = name.split('.').next().unwrap_or(name);
        *counts.entry(prefix).or_insert(0) += 1;
    }

    let total = type_names.len() as f64;
    let mut entropy = 0.0f64;
    for &count in counts.values() {
        let p = count as f64 / total;
        entropy -= p * p.log2();
    }
    entropy
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 1: FEATURE_DIM constant matches Fate's value.
    #[test]
    fn feature_dim_matches_fate() {
        assert_eq!(FEATURE_DIM, 16);
    }

    // 2: Parse a simple grammar and verify 16 values in [0, 1].
    #[test]
    fn extract_simple_grammar_is_16_dims() {
        let source = "grammar @test {\n  type color = red | blue | green\n}\n";
        let features = extract_from_source(source);
        assert_eq!(features.len(), 16);
        for (i, &v) in features.iter().enumerate() {
            assert!(
                (0.0..=1.0).contains(&v),
                "feature[{}] = {} is out of [0, 1]",
                i,
                v
            );
        }
    }

    // 3: Three different fixture grammars — all features in [0, 1],
    //    projection_idempotent = 1.0, shannon_equivalence = 1.0.
    #[test]
    fn all_fixture_grammars_produce_valid_features() {
        let fixtures = [
            // Simple single-type grammar
            "grammar @color {\n  type hue = red | green | blue\n}\n",
            // Multi-type with parameterized variant
            "grammar @signal {\n  type status = ok | error\n  type level = low | high\n  type alert = clear | warn(level)\n}\n",
            // Grammar with an action
            "grammar @mail {\n  type address = email | uri\n  action send(to: address, subject)\n}\n",
        ];

        for source in &fixtures {
            let features = extract_from_source(source);
            assert_eq!(features.len(), 16, "wrong dim for: {}", source);

            for (i, &v) in features.iter().enumerate() {
                assert!(
                    (0.0..=1.0).contains(&v),
                    "fixture feature[{}] = {} out of [0, 1]\nsource: {}",
                    i,
                    v,
                    source
                );
            }

            // projection_idempotent (dim 13) must be 1.0 for valid grammars
            assert_eq!(
                features[13], 1.0,
                "projection_idempotent should be 1.0\nsource: {}",
                source
            );

            // shannon_equivalence (dim 15) must be 1.0 for valid grammars
            assert_eq!(
                features[15], 1.0,
                "shannon_equivalence should be 1.0\nsource: {}",
                source
            );
        }
    }

    // Edge case: empty source returns zero vector.
    #[test]
    fn invalid_source_returns_zero_vector() {
        let features = extract_from_source("this is not a grammar");
        assert_eq!(features, [0.0; 16]);
    }

    // Verify node_count_norm is nonzero for a real grammar.
    #[test]
    fn node_count_norm_nonzero_for_real_grammar() {
        let source = "grammar @test {\n  type x = a | b\n}\n";
        let features = extract_from_source(source);
        assert!(features[0] > 0.0, "node_count_norm should be positive");
    }

    // Verify type_count_norm scales correctly.
    #[test]
    fn type_count_norm_scales_with_types() {
        let one_type = "grammar @a {\n  type x = a\n}\n";
        let two_types = "grammar @b {\n  type x = a\n  type y = b\n}\n";
        let f1 = extract_from_source(one_type);
        let f2 = extract_from_source(two_types);
        assert!(f2[5] > f1[5], "more types → higher type_count_norm");
    }

    // Verify ref_ratio is 0 when no parameterized variants exist.
    #[test]
    fn ref_ratio_zero_for_simple_grammar() {
        let source = "grammar @test {\n  type x = a | b | c\n}\n";
        let features = extract_from_source(source);
        assert_eq!(
            features[7], 0.0,
            "no parameterized variants → ref_ratio = 0"
        );
    }

    // Verify ref_ratio is nonzero when parameterized variants exist.
    #[test]
    fn ref_ratio_nonzero_for_parameterized_grammar() {
        let source = "grammar @test {\n  type x = plain | when(op)\n  type op = gt | lt\n}\n";
        let features = extract_from_source(source);
        assert!(
            features[7] > 0.0,
            "parameterized variant → ref_ratio > 0, got {}",
            features[7]
        );
    }
}
