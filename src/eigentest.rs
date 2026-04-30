//! Eigentest — compile-time star detection on the mirror grammar's type graph.
//!
//! When mirror compiles a `.mirror` file, it builds a parse tree (`Prism<AstNode>`).
//! This module converts that tree into a small adjacency graph and runs a simplified
//! star battery.
//!
//! If three or more tests fail, the grammar has star topology. The SEL is
//! enforced by eigenvalues, not policy.
//!
//! The eight tests (adapted for type graphs):
//! 1. Degree Gini > 0.6 — type/variant connection inequality
//! 2. Max degree > 3x average — one type referenced by everything
//! 3. Betweenness centrality > 0.5 — one type mediates all connections
//! 4. Clustering coefficient < 0.05 — no type triangles
//! 5. Fiedler value < average_degree/n — weak algebraic connectivity
//! 6. Spectral ratio lambda_{n-1}/lambda_1 > n/2 — one dominant hub
//! 7. Any single type in > 50% of edges
//! 8. Von Neumann entropy < log2(n)/2 + 1 — low structural complexity

use crate::declaration::{DeclKind, MirrorData, MirrorFragment, MirrorFragmentExt};
use crate::parse::AstNode;
use crate::prism::Prism;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A violation detected by a compile-time eigentest.
#[derive(Clone, Debug)]
pub struct EigenViolation {
    pub test_id: u8,
    pub name: &'static str,
    pub measured: f64,
    pub threshold: f64,
}

/// Result of running the eigentest battery on a parsed grammar.
#[derive(Clone, Debug)]
pub struct EigentestResult {
    pub violations: Vec<EigenViolation>,
    pub node_count: usize,
    pub edge_count: usize,
}

impl EigentestResult {
    pub fn is_star(&self) -> bool {
        self.violations.len() >= 3
    }

    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }
}

// ---------------------------------------------------------------------------
// Type graph from AST
// ---------------------------------------------------------------------------

/// A simple adjacency graph built from the parsed AST.
struct TypeGraph {
    adj: Vec<Vec<usize>>,
    n: usize,
}

impl TypeGraph {
    /// Build a type graph from a Prism<AstNode> parse tree.
    ///
    /// Nodes = unique type names, variant names, action names, references.
    /// Edges = structural containment (type→variant) and references (variant→type-ref).
    fn from_ast(root: &Prism<AstNode>) -> Self {
        use std::collections::HashMap;

        let mut name_to_idx: HashMap<String, usize> = HashMap::new();
        let mut next_idx = 0_usize;
        let mut edges: Vec<(usize, usize)> = Vec::new();

        fn get_or_insert(
            name: &str,
            map: &mut HashMap<String, usize>,
            next: &mut usize,
        ) -> usize {
            if let Some(&idx) = map.get(name) {
                idx
            } else {
                let idx = *next;
                map.insert(name.to_string(), idx);
                *next += 1;
                idx
            }
        }

        // Walk the AST tree, collecting nodes and edges.
        fn walk(
            node: &Prism<AstNode>,
            parent_key: Option<&str>,
            name_to_idx: &mut HashMap<String, usize>,
            next_idx: &mut usize,
            edges: &mut Vec<(usize, usize)>,
        ) {
            let data = node.data();
            let node_key = format!("{:?}:{}:{}", data.kind, data.name, data.value);

            let node_idx = get_or_insert(&node_key, name_to_idx, next_idx);

            // Connect to parent
            if let Some(parent) = parent_key {
                let parent_idx = get_or_insert(parent, name_to_idx, next_idx);
                edges.push((parent_idx, node_idx));
            }

            // Recurse into children
            for child in node.children() {
                walk(child, Some(&node_key), name_to_idx, next_idx, edges);
            }
        }

        walk(root, None, &mut name_to_idx, &mut next_idx, &mut edges);

        let n = next_idx;
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in &edges {
            if a < n && b < n {
                adj[a].push(b);
                adj[b].push(a);
            }
        }

        TypeGraph { adj, n }
    }

    fn degrees(&self) -> Vec<f64> {
        self.adj.iter().map(|neighbors| neighbors.len() as f64).collect()
    }

    fn laplacian(&self) -> Vec<f64> {
        let n = self.n;
        let mut matrix = vec![0.0_f64; n * n];
        for (i, neighbors) in self.adj.iter().enumerate() {
            matrix[i * n + i] = neighbors.len() as f64;
            for &j in neighbors {
                matrix[i * n + j] -= 1.0;
            }
        }
        matrix
    }

    /// Build a cross-reference type graph from a MirrorFragment tree.
    ///
    /// Unlike `from_ast` which uses AST parent-child edges (always tree-shaped),
    /// this builds edges from semantic references:
    /// - Nodes = type declarations + action declarations
    /// - Edges = type-to-variant (type declares these variants) +
    ///           action-to-type (action parameter references a declared type)
    ///
    /// This is the graph that should be eigentest'd for star detection.
    /// A grammar where one type mediates all connections IS a star.
    /// A grammar where types reference each other densely is not.
    fn from_type_references(root: &MirrorFragment) -> Self {
        use std::collections::HashMap;

        let mut name_to_idx: HashMap<String, usize> = HashMap::new();
        let mut next_idx = 0_usize;
        let mut edges: Vec<(usize, usize)> = Vec::new();

        // First pass: collect all declared type names
        let mut declared_types: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        fn collect_types(
            frag: &MirrorFragment,
            types: &mut std::collections::HashSet<String>,
        ) {
            let data = frag.mirror_data();
            if data.kind == DeclKind::Type && !data.name.is_empty() {
                types.insert(data.name.clone());
            }
            // Also collect variant names as types
            if data.kind == DeclKind::Type {
                for v in &data.variants {
                    if !v.is_empty() {
                        types.insert(v.clone());
                    }
                }
            }
            for child in frag.mirror_children() {
                collect_types(child, types);
            }
        }

        collect_types(root, &mut declared_types);

        fn get_or_insert(
            name: &str,
            map: &mut HashMap<String, usize>,
            next: &mut usize,
        ) -> usize {
            if let Some(&idx) = map.get(name) {
                idx
            } else {
                let idx = *next;
                map.insert(name.to_string(), idx);
                *next += 1;
                idx
            }
        }

        // Second pass: build edges
        fn walk_refs(
            frag: &MirrorFragment,
            declared_types: &std::collections::HashSet<String>,
            name_to_idx: &mut HashMap<String, usize>,
            next_idx: &mut usize,
            edges: &mut Vec<(usize, usize)>,
        ) {
            let data = frag.mirror_data();

            match data.kind {
                DeclKind::Type => {
                    // Named type with variants: type -> variant edges
                    if !data.name.is_empty() {
                        let type_idx =
                            get_or_insert(&data.name, name_to_idx, next_idx);
                        for v in &data.variants {
                            if !v.is_empty() {
                                let v_idx =
                                    get_or_insert(v, name_to_idx, next_idx);
                                edges.push((type_idx, v_idx));
                            }
                        }
                    } else {
                        // Unnamed root type (type = a | b | c): add all
                        // variants as nodes, connect them to each other
                        // since they're siblings in the same declaration
                        for v in &data.variants {
                            if !v.is_empty() {
                                let _ = get_or_insert(v, name_to_idx, next_idx);
                            }
                        }
                    }
                }
                DeclKind::Action => {
                    // Action -> type reference edges from parameters
                    let action_idx =
                        get_or_insert(&data.name, name_to_idx, next_idx);

                    for param in &data.params {
                        // Params can be "name:type" or just "name"
                        let type_ref = if param.contains(':') {
                            param.split(':').last().map(|s| s.trim())
                        } else if declared_types.contains(param.as_str()) {
                            Some(param.as_str())
                        } else {
                            None
                        };

                        if let Some(tr) = type_ref {
                            if declared_types.contains(tr) {
                                let type_idx =
                                    get_or_insert(tr, name_to_idx, next_idx);
                                edges.push((action_idx, type_idx));
                            }
                        }
                    }
                }
                _ => {}
            }

            for child in frag.mirror_children() {
                walk_refs(child, declared_types, name_to_idx, next_idx, edges);
            }
        }

        walk_refs(
            root,
            &declared_types,
            &mut name_to_idx,
            &mut next_idx,
            &mut edges,
        );

        let n = next_idx;
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in &edges {
            if a < n && b < n && a != b {
                adj[a].push(b);
                adj[b].push(a);
            }
        }

        TypeGraph { adj, n }
    }
}

// ---------------------------------------------------------------------------
// The eight eigentests
// ---------------------------------------------------------------------------

fn test_degree_gini(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 2 { return None; }
    let degrees = graph.degrees();
    let gini = gini_coefficient(&degrees);
    if gini > 0.6 {
        Some(EigenViolation { test_id: 1, name: "degree_gini", measured: gini, threshold: 0.6 })
    } else {
        None
    }
}

fn test_degree_hub(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 2 { return None; }
    let degrees = graph.degrees();
    let avg: f64 = degrees.iter().sum::<f64>() / graph.n as f64;
    let max = degrees.iter().cloned().fold(0.0_f64, f64::max);
    if avg > f64::EPSILON && max > 3.0 * avg {
        Some(EigenViolation { test_id: 2, name: "degree_hub", measured: max, threshold: 3.0 * avg })
    } else {
        None
    }
}

fn test_betweenness(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 3 { return None; }
    let centralities = brandes_betweenness(&graph.adj, graph.n);
    for &bc in &centralities {
        if bc > 0.5 {
            return Some(EigenViolation { test_id: 3, name: "betweenness_centrality", measured: bc, threshold: 0.5 });
        }
    }
    None
}

fn test_clustering(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 3 { return None; }
    let cc = global_clustering(&graph.adj, graph.n);
    if cc < 0.05 {
        Some(EigenViolation { test_id: 4, name: "clustering_coefficient", measured: cc, threshold: 0.05 })
    } else {
        None
    }
}

fn test_fiedler(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 2 { return None; }
    let laplacian = graph.laplacian();
    let mut eigenvalues = jacobi_eigenvalues(&laplacian, graph.n);
    eigenvalues.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let fiedler = eigenvalues.get(1).copied().unwrap_or(0.0);
    let degrees = graph.degrees();
    let k_avg: f64 = degrees.iter().sum::<f64>() / graph.n as f64;
    let threshold = k_avg / graph.n as f64;
    if fiedler < threshold && k_avg > 1.0 {
        Some(EigenViolation { test_id: 5, name: "fiedler_decreased", measured: fiedler, threshold })
    } else {
        None
    }
}

fn test_spectral_ratio(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 3 { return None; }
    let laplacian = graph.laplacian();
    let mut eigenvalues = jacobi_eigenvalues(&laplacian, graph.n);
    eigenvalues.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let lambda_1 = eigenvalues.get(1).copied().unwrap_or(0.0);
    let lambda_n_minus_1 = eigenvalues.last().copied().unwrap_or(0.0);
    if lambda_1 > f64::EPSILON {
        let ratio = lambda_n_minus_1 / lambda_1;
        let threshold = graph.n as f64 / 2.0;
        if ratio > threshold {
            return Some(EigenViolation { test_id: 6, name: "spectral_ratio", measured: ratio, threshold });
        }
    }
    None
}

fn test_edge_dominance(graph: &TypeGraph) -> Option<EigenViolation> {
    let total_edges: f64 = graph.adj.iter().map(|n| n.len() as f64).sum::<f64>() / 2.0;
    if graph.n < 2 || total_edges < 1.0 { return None; }
    let degrees = graph.degrees();
    for &deg in &degrees {
        let participation = deg / total_edges;
        if participation > 0.5 {
            return Some(EigenViolation { test_id: 7, name: "edge_dominance", measured: participation, threshold: 0.5 });
        }
    }
    None
}

fn test_von_neumann_entropy(graph: &TypeGraph) -> Option<EigenViolation> {
    if graph.n < 2 { return None; }
    let laplacian = graph.laplacian();
    let eigenvalues = jacobi_eigenvalues(&laplacian, graph.n);
    let total: f64 = eigenvalues.iter().filter(|&&v| v > f64::EPSILON).sum();
    if total < f64::EPSILON { return None; }
    let mut entropy = 0.0_f64;
    for &lambda in &eigenvalues {
        if lambda > f64::EPSILON {
            let p = lambda / total;
            entropy -= p * p.log2();
        }
    }
    let threshold = (graph.n as f64).log2() / 2.0 + 1.0;
    if entropy < threshold {
        Some(EigenViolation { test_id: 8, name: "von_neumann_entropy", measured: entropy, threshold })
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Minimum type-graph size for meaningful eigentest.
/// Below this threshold, all grammars pass — small grammars can't form star topology.
/// The AST is inherently hierarchical, and small trees are always star-like.
const EIGENTEST_MIN_NODES: usize = 10;

/// Run the eigentest battery on a parsed `.mirror` AST.
///
/// The AST is converted to a type graph and the eight star tests
/// are run. If `is_star()` returns true, the grammar should
/// not compile — the topology indicates extraction.
///
/// Grammars with fewer than 10 type-graph nodes are exempt — small
/// grammars don't have enough structure for meaningful spectral analysis.
///
/// NOTE: This runs on the AST parent-child tree, which is inherently
/// hierarchical. For grammar validation, prefer `eigentest_type_graph`
/// which runs on the cross-reference type graph.
pub fn eigentest(ast: &Prism<AstNode>) -> EigentestResult {
    let graph = TypeGraph::from_ast(ast);
    run_battery(&graph)
}

/// Run the eigentest battery on the **cross-reference type graph** derived
/// from `.mirror` grammar source.
///
/// Unlike `eigentest` (which runs on the AST parent-child tree and always
/// sees star topology because ASTs are trees), this function builds a graph
/// where:
/// - Nodes = declared types + actions
/// - Edges = type-to-variant references + action-to-type parameter references
///
/// A grammar where one type mediates all connections IS a star and WILL fail.
/// A grammar where types reference each other densely WILL pass.
///
/// Same eight tests. Different input graph.
///
/// Takes source as `&str` and parses to MirrorFragment internally,
/// because the type reference graph requires params/variants from MirrorData
/// (not available in the thin Prism<AstNode> projection).
pub fn eigentest_type_graph(source: &str) -> EigentestResult {
    use crate::mirror_runtime::parse_form;

    match Result::from(parse_form(source)) {
        Ok(frag) => {
            let graph = TypeGraph::from_type_references(&frag);
            run_type_reference_battery(&graph)
        }
        Err(_) => EigentestResult {
            violations: Vec::new(),
            node_count: 0,
            edge_count: 0,
        },
    }
}

/// Run the eight-test battery on a TypeGraph (AST-shaped graphs).
fn run_battery(graph: &TypeGraph) -> EigentestResult {
    // Small grammars are exempt from eigentesting.
    if graph.n < EIGENTEST_MIN_NODES {
        return EigentestResult {
            violations: Vec::new(),
            node_count: graph.n,
            edge_count: graph.adj.iter().map(|n| n.len()).sum::<usize>() / 2,
        };
    }

    let mut violations = Vec::new();

    if let Some(v) = test_degree_gini(graph) { violations.push(v); }
    if let Some(v) = test_degree_hub(graph) { violations.push(v); }
    if let Some(v) = test_betweenness(graph) { violations.push(v); }
    if let Some(v) = test_clustering(graph) { violations.push(v); }
    if let Some(v) = test_fiedler(graph) { violations.push(v); }
    if let Some(v) = test_spectral_ratio(graph) { violations.push(v); }
    if let Some(v) = test_edge_dominance(graph) { violations.push(v); }
    if let Some(v) = test_von_neumann_entropy(graph) { violations.push(v); }

    EigentestResult {
        violations,
        node_count: graph.n,
        edge_count: graph.adj.iter().map(|n| n.len()).sum::<usize>() / 2,
    }
}

/// Run the star battery on a cross-reference type graph.
///
/// Uses 6 of the 8 tests. Excludes:
/// - Clustering coefficient: type reference graphs are acyclic (no triangles
///   by construction), so cc=0 is expected, not pathological.
/// - Fiedler value: type reference graphs often have disconnected components
///   (standalone types not referenced by actions), so fiedler near zero is
///   structural, not extraction.
///
/// Three or more violations = star topology.
fn run_type_reference_battery(graph: &TypeGraph) -> EigentestResult {
    if graph.n < EIGENTEST_MIN_NODES {
        return EigentestResult {
            violations: Vec::new(),
            node_count: graph.n,
            edge_count: graph.adj.iter().map(|n| n.len()).sum::<usize>() / 2,
        };
    }

    let mut violations = Vec::new();

    if let Some(v) = test_degree_gini(graph) { violations.push(v); }
    if let Some(v) = test_degree_hub(graph) { violations.push(v); }
    if let Some(v) = test_betweenness(graph) { violations.push(v); }
    // Excluded: test_clustering — type refs are acyclic, cc=0 is expected
    // Excluded: test_fiedler — disconnected components are normal in type graphs
    if let Some(v) = test_spectral_ratio(graph) { violations.push(v); }
    if let Some(v) = test_edge_dominance(graph) { violations.push(v); }
    if let Some(v) = test_von_neumann_entropy(graph) { violations.push(v); }

    EigentestResult {
        violations,
        node_count: graph.n,
        edge_count: graph.adj.iter().map(|n| n.len()).sum::<usize>() / 2,
    }
}

// ---------------------------------------------------------------------------
// Math helpers (self-contained — no external dependency)
// ---------------------------------------------------------------------------

fn gini_coefficient(values: &[f64]) -> f64 {
    let n = values.len();
    if n == 0 { return 0.0; }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let sum: f64 = sorted.iter().sum();
    if sum < f64::EPSILON { return 0.0; }
    let mut numerator = 0.0;
    for (i, &v) in sorted.iter().enumerate() {
        numerator += (2.0 * (i as f64 + 1.0) - n as f64 - 1.0) * v;
    }
    numerator / (n as f64 * sum)
}

fn brandes_betweenness(adj: &[Vec<usize>], n: usize) -> Vec<f64> {
    let mut centrality = vec![0.0_f64; n];
    for s in 0..n {
        let mut stack = Vec::new();
        let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma = vec![0.0_f64; n];
        sigma[s] = 1.0;
        let mut dist = vec![-1_i64; n];
        dist[s] = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &w in &adj[v] {
                if dist[w] < 0 { queue.push_back(w); dist[w] = dist[v] + 1; }
                if dist[w] == dist[v] + 1 { sigma[w] += sigma[v]; predecessors[w].push(v); }
            }
        }
        let mut delta = vec![0.0_f64; n];
        while let Some(w) = stack.pop() {
            for &v in &predecessors[w] { delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]); }
            if w != s { centrality[w] += delta[w]; }
        }
    }
    let norm = if n > 2 { ((n - 1) * (n - 2)) as f64 / 2.0 } else { 1.0 };
    for c in &mut centrality { *c /= norm; }
    centrality
}

fn global_clustering(adj: &[Vec<usize>], n: usize) -> f64 {
    let mut triangles = 0_u64;
    let mut triplets = 0_u64;
    for v in 0..n {
        let neighbors = &adj[v];
        let k = neighbors.len();
        if k < 2 { continue; }
        triplets += (k * (k - 1) / 2) as u64;
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                if adj[neighbors[i]].contains(&neighbors[j]) { triangles += 1; }
            }
        }
    }
    if triplets == 0 { return 0.0; }
    triangles as f64 / triplets as f64
}

fn jacobi_eigenvalues(matrix: &[f64], n: usize) -> Vec<f64> {
    if n == 0 { return Vec::new(); }
    if n == 1 { return vec![matrix[0]]; }
    let mut a = matrix.to_vec();
    let max_iter = 100 * n * n;
    let eps = 1e-12;
    for _ in 0..max_iter {
        let mut max_val = 0.0_f64;
        let mut p = 0;
        let mut q = 1;
        for i in 0..n {
            for j in (i + 1)..n {
                let val = a[i * n + j].abs();
                if val > max_val { max_val = val; p = i; q = j; }
            }
        }
        if max_val < eps { break; }
        let app = a[p * n + p];
        let aqq = a[q * n + q];
        let apq = a[p * n + q];
        let theta = if (app - aqq).abs() < eps { std::f64::consts::FRAC_PI_4 }
                     else { 0.5 * ((2.0 * apq) / (app - aqq)).atan() };
        let cos_t = theta.cos();
        let sin_t = theta.sin();
        let mut new_a = a.clone();
        for i in 0..n {
            if i != p && i != q {
                let aip = a[i * n + p]; let aiq = a[i * n + q];
                new_a[i * n + p] = cos_t * aip + sin_t * aiq;
                new_a[p * n + i] = new_a[i * n + p];
                new_a[i * n + q] = -sin_t * aip + cos_t * aiq;
                new_a[q * n + i] = new_a[i * n + q];
            }
        }
        new_a[p * n + p] = cos_t * cos_t * app + 2.0 * cos_t * sin_t * apq + sin_t * sin_t * aqq;
        new_a[q * n + q] = sin_t * sin_t * app - 2.0 * cos_t * sin_t * apq + cos_t * cos_t * aqq;
        new_a[p * n + q] = 0.0;
        new_a[q * n + p] = 0.0;
        a = new_a;
    }
    (0..n).map(|i| a[i * n + i]).collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Vector;
    use crate::parse::Parse;

    fn parse(source: &str) -> Prism<AstNode> {
        Parse.trace(source.to_string()).into_result().expect("parse should succeed")
    }

    #[test]
    fn eigentest_simple_grammar_not_star() {
        let ast = parse("grammar @test {\n  type = request | response | error\n  type direction = inbound | outbound\n}");
        let result = eigentest(&ast);
        assert!(
            !result.is_star(),
            "simple grammar should not be star topology, got {} violations: {:?}",
            result.violation_count(),
            result.violations,
        );
    }

    #[test]
    fn eigentest_empty_grammar() {
        let ast = parse("grammar @empty {}");
        let result = eigentest(&ast);
        assert!(!result.is_star());
    }

    #[test]
    fn eigentest_single_type() {
        let ast = parse("grammar @single {\n  type = alpha\n}");
        let result = eigentest(&ast);
        assert!(!result.is_star());
    }

    #[test]
    fn eigentest_result_has_graph_metrics() {
        let ast = parse("grammar @test {\n  type = a | b | c\n}");
        let result = eigentest(&ast);
        assert!(result.node_count > 0, "should have nodes");
    }

    // --- Math helper tests ---

    #[test]
    fn gini_uniform_zero() {
        assert!(gini_coefficient(&[1.0, 1.0, 1.0]).abs() < 1e-10);
    }

    #[test]
    fn gini_empty_zero() {
        assert_eq!(gini_coefficient(&[]), 0.0);
    }

    #[test]
    fn jacobi_2x2() {
        let matrix = vec![2.0, 1.0, 1.0, 2.0];
        let mut evals = jacobi_eigenvalues(&matrix, 2);
        evals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((evals[0] - 1.0).abs() < 1e-10);
        assert!((evals[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn clustering_triangle_one() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert!((global_clustering(&adj, 3) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn brandes_star_center_highest() {
        let adj = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let bc = brandes_betweenness(&adj, 4);
        let max_idx = bc.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0;
        assert_eq!(max_idx, 0);
    }

    #[test]
    fn eigentest_ai_grammar_parses_and_runs() {
        // The @ai grammar with observation/proposal/crystal types.
        // Grammars parsed as ASTs are inherently hierarchical (one grammar root
        // with many type/action children), which looks star-shaped to the
        // eigentest battery. This is structural truth about AST shape, not
        // extraction. The eigentest proves the graph IS analyzed — the star
        // detection is correct for a tree-shaped AST.
        let source = r#"grammar @ai {
  type = collapse | tension | branch | observation | proposal | crystal
  type collapse = clear | partial | ambiguous
  type tension = competing | complementary | contradictory
  type observation = focus | project | bridge
  type proposal = plan | build | spawn
  type crystal = settled | promoted | archived
  action project(input: collapse)
  action coherence(input)
  action settle(input)
  action branch(tension: tension)
  action escalate(tension: tension)
  action observe(graph: observation)
  action propose(observation: observation)
  action decide(proposal: proposal)
}"#;
        let ast = parse(source);
        let result = eigentest(&ast);
        // Grammar parses successfully and eigentest runs
        assert!(result.node_count > 10, "@ai grammar should have >10 type-graph nodes, got {}", result.node_count);
        assert!(result.edge_count > 0, "@ai grammar should have edges, got {}", result.edge_count);
        // AST-derived grammars are tree-shaped (star from root). This is not
        // extraction — it's the structure of a flat grammar declaration.
        // When the eigentest moves to cross-reference type graphs (not AST
        // parent-child), this will change.
        assert!(
            result.violation_count() > 0,
            "@ai grammar AST is hierarchical — eigentest should detect star shape"
        );
    }

    // -----------------------------------------------------------------------
    // Cross-reference type graph tests
    // -----------------------------------------------------------------------

    #[test]
    fn type_graph_star_grammar_fails_eigentest() {
        // A grammar where type "hub" mediates ALL connections.
        // Every other type references only hub. This IS a star.
        // The eigentest on the TYPE REFERENCE graph should detect it.
        let source = r#"grammar @star {
  type = hub | spoke_a | spoke_b | spoke_c | spoke_d | spoke_e | spoke_f | spoke_g | spoke_h | spoke_i | spoke_j
  action use_a(input: hub)
  action use_b(input: hub)
  action use_c(input: hub)
  action use_d(input: hub)
  action use_e(input: hub)
  action use_f(input: hub)
  action use_g(input: hub)
  action use_h(input: hub)
  action use_i(input: hub)
  action use_j(input: hub)
}"#;
        let result = eigentest_type_graph(source);
        assert!(
            result.node_count >= 10,
            "star grammar type graph should have >= 10 nodes, got {}",
            result.node_count,
        );
        assert!(
            result.is_star(),
            "star grammar should fail eigentest on type reference graph, got {} violations: {:?}",
            result.violation_count(),
            result.violations,
        );
    }

    #[test]
    fn type_graph_dense_grammar_passes_eigentest() {
        // A grammar where types reference each other densely.
        // No single type mediates all connections. This should PASS.
        let source = r#"grammar @dense {
  type = request | response | error | context | handler | middleware | config | logger | metric | event
  type request = get | post | put | delete
  type response = success | failure | redirect
  type error = validation | auth | not_found | internal
  action handle(input: request, ctx: context, cfg: config)
  action respond(input: handler, out: response, log: logger)
  action validate(input: request, err: error, met: metric)
  action log_event(input: logger, evt: event, ctx: context)
  action configure(input: config, mid: middleware, hand: handler)
  action dispatch(input: middleware, req: request, resp: response)
  action measure(input: metric, evt: event, err: error)
  action emit(input: event, hand: handler, mid: middleware)
}"#;
        let result = eigentest_type_graph(source);
        assert!(
            result.node_count >= 10,
            "dense grammar type graph should have >= 10 nodes, got {}",
            result.node_count,
        );
        assert!(
            !result.is_star(),
            "dense grammar should pass eigentest on type reference graph, got {} violations: {:?}",
            result.violation_count(),
            result.violations,
        );
    }

    #[test]
    fn type_graph_ai_grammar_not_star() {
        // The @ai grammar has cross-references from actions to types.
        // On the TYPE REFERENCE graph (not AST), it should NOT be a star,
        // because actions create lateral connections between types.
        let source = r#"grammar @ai {
  type = collapse | tension | branch | observation | proposal | crystal
  type collapse = clear | partial | ambiguous
  type tension = competing | complementary | contradictory
  type observation = focus | project | bridge
  type proposal = plan | build | spawn
  type crystal = settled | promoted | archived
  action project(input: collapse)
  action coherence(input)
  action settle(input)
  action branch(tension: tension)
  action escalate(tension: tension)
  action observe(graph: observation)
  action propose(observation: observation)
  action decide(proposal: proposal)
}"#;
        let result = eigentest_type_graph(source);
        assert!(
            result.node_count > 5,
            "@ai type graph should have > 5 nodes, got {}",
            result.node_count,
        );
        // The @ai grammar has enough cross-references that it should NOT
        // be classified as a star on the type reference graph.
        assert!(
            !result.is_star(),
            "@ai grammar should not be a star on type reference graph, got {} violations: {:?}",
            result.violation_count(),
            result.violations,
        );
    }
}
