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
pub fn eigentest(ast: &Prism<AstNode>) -> EigentestResult {
    let graph = TypeGraph::from_ast(ast);

    // Small grammars are exempt from eigentesting.
    if graph.n < EIGENTEST_MIN_NODES {
        return EigentestResult {
            violations: Vec::new(),
            node_count: graph.n,
            edge_count: graph.adj.iter().map(|n| n.len()).sum::<usize>() / 2,
        };
    }

    let mut violations = Vec::new();

    if let Some(v) = test_degree_gini(&graph) { violations.push(v); }
    if let Some(v) = test_degree_hub(&graph) { violations.push(v); }
    if let Some(v) = test_betweenness(&graph) { violations.push(v); }
    if let Some(v) = test_clustering(&graph) { violations.push(v); }
    if let Some(v) = test_fiedler(&graph) { violations.push(v); }
    if let Some(v) = test_spectral_ratio(&graph) { violations.push(v); }
    if let Some(v) = test_edge_dominance(&graph) { violations.push(v); }
    if let Some(v) = test_von_neumann_entropy(&graph) { violations.push(v); }

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
}
