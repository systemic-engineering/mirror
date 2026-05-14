// FROZEN -- see AGENTS.md. Do not modify without explicit approval.
// This file is Rust substrate. All extensions happen through .mirror grammars.
// If you're adding code here, you're probably wrong. Write a grammar instead.

//! Dirac operator for spectral triples on finite graphs.
//!
//! The Dirac operator D unifies eigenvalues, distance, and action into a single matrix:
//!
//! - D is the block matrix [[0, B^T], [B, 0]] where B is the signed weighted incidence matrix
//! - D^2 restricted to 0-forms = graph Laplacian L_0
//! - D^2 restricted to 1-forms = edge Laplacian L_1
//! - D is self-adjoint (D = D^T for real matrices)
//! - Eigenvalues of D are symmetric about 0
//! - Connes distance = Dijkstra with edge lengths 1/sqrt(w)
//!
//! ## Architecture
//!
//! This module provides the bare math. No external dependencies beyond std.
//! The Jacobi eigenvalue solver is self-contained (same algorithm as eigentest.rs).

use std::collections::BinaryHeap;
use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// SparseMatrix — CSR format
// ---------------------------------------------------------------------------

/// Sparse matrix in Compressed Sparse Row (CSR) format.
#[derive(Clone, Debug)]
pub struct SparseMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Row pointers: row_ptr[i]..row_ptr[i+1] index into col_idx/values for row i.
    pub row_ptr: Vec<usize>,
    /// Column indices for each non-zero entry.
    pub col_idx: Vec<usize>,
    /// Values for each non-zero entry.
    pub values: Vec<f64>,
}

impl SparseMatrix {
    /// Create an empty matrix with given dimensions.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        SparseMatrix {
            nrows,
            ncols,
            row_ptr: vec![0; nrows + 1],
            col_idx: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Build a sparse matrix from a list of (row, col, value) triplets.
    /// Triplets must be sorted by (row, col).
    pub fn from_triplets(nrows: usize, ncols: usize, triplets: &mut Vec<(usize, usize, f64)>) -> Self {
        triplets.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut row_ptr = vec![0usize; nrows + 1];
        let mut col_idx = Vec::with_capacity(triplets.len());
        let mut values = Vec::with_capacity(triplets.len());

        for &(row, col, val) in triplets.iter() {
            row_ptr[row + 1] += 1;
            col_idx.push(col);
            values.push(val);
        }

        // Cumulative sum for row_ptr
        for i in 1..=nrows {
            row_ptr[i] += row_ptr[i - 1];
        }

        SparseMatrix { nrows, ncols, row_ptr, col_idx, values }
    }

    /// Get value at (row, col). O(nnz in row).
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let start = self.row_ptr[row];
        let end = self.row_ptr[row + 1];
        for idx in start..end {
            if self.col_idx[idx] == col {
                return self.values[idx];
            }
        }
        0.0
    }

    /// Convert to dense matrix (row-major).
    pub fn to_dense(&self) -> Vec<f64> {
        let mut dense = vec![0.0; self.nrows * self.ncols];
        for row in 0..self.nrows {
            let start = self.row_ptr[row];
            let end = self.row_ptr[row + 1];
            for idx in start..end {
                dense[row * self.ncols + self.col_idx[idx]] = self.values[idx];
            }
        }
        dense
    }

    /// Sparse matrix-vector multiply: y = A * x.
    pub fn mul_vec(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.ncols);
        let mut y = vec![0.0; self.nrows];
        for row in 0..self.nrows {
            let start = self.row_ptr[row];
            let end = self.row_ptr[row + 1];
            for idx in start..end {
                y[row] += self.values[idx] * x[self.col_idx[idx]];
            }
        }
        y
    }
}

// ---------------------------------------------------------------------------
// SpectralTriple
// ---------------------------------------------------------------------------

/// The spectral triple (A, H, D) for a grammar on a graph.
///
/// A = functions on vertices (commutative algebra)
/// H = l^2(V) + l^2(E) = R^(n+m)
/// D = [[0, B^T], [B, 0]] where B is the signed weighted incidence matrix
#[derive(Clone, Debug)]
pub struct SpectralTriple {
    /// The Dirac operator as a sparse (n+m) x (n+m) matrix.
    pub dirac: SparseMatrix,
    /// Total dimension of Hilbert space = n + m.
    pub dimension: usize,
    /// Number of vertices (nodes).
    pub node_count: usize,
    /// Number of edges.
    pub edge_count: usize,
    /// Edge list: (from, to, weight).
    pub edges: Vec<(usize, usize, f64)>,
}

/// First k eigenvectors of D projected onto node subspace.
#[derive(Clone, Debug)]
pub struct SpectralEmbedding {
    /// One 16-dimensional vector per node.
    pub components: Vec<[f64; 16]>,
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

/// Construct the Dirac operator from adjacency data.
///
/// The Dirac operator D is the (n+m) x (n+m) block matrix:
///   D = [[0, B^T], [B, 0]]
///
/// where B is the m x n signed weighted incidence matrix:
///   B[e, i] = -sqrt(w_e) if i is the source of edge e
///   B[e, j] = +sqrt(w_e) if j is the target of edge e
///
/// Orientation convention: source = min(from, to), target = max(from, to).
pub fn construct_dirac(
    nodes: usize,
    edges: &[(usize, usize, f64)],
) -> SpectralTriple {
    let m = edges.len();
    let dim = nodes + m;

    // Build triplets for the full D matrix.
    // Upper-right block: B^T (n x m), stored in rows 0..n, cols n..n+m
    // Lower-left block: B (m x n), stored in rows n..n+m, cols 0..n
    let mut triplets: Vec<(usize, usize, f64)> = Vec::with_capacity(4 * m);

    for (e_idx, &(from, to, weight)) in edges.iter().enumerate() {
        let sqrt_w = weight.sqrt();

        // Canonical orientation: source = min, target = max
        let (src, tgt) = if from <= to { (from, to) } else { (to, from) };

        // B[e_idx, src] = -sqrt(w), B[e_idx, tgt] = +sqrt(w)
        // In the full matrix, B is in rows n..n+m, cols 0..n
        let row_b = nodes + e_idx;
        triplets.push((row_b, src, -sqrt_w));
        triplets.push((row_b, tgt, sqrt_w));

        // B^T[src, e_idx] = -sqrt(w), B^T[tgt, e_idx] = +sqrt(w)
        // In the full matrix, B^T is in rows 0..n, cols n..n+m
        let col_bt = nodes + e_idx;
        triplets.push((src, col_bt, -sqrt_w));
        triplets.push((tgt, col_bt, sqrt_w));
    }

    let dirac = SparseMatrix::from_triplets(dim, dim, &mut triplets);

    SpectralTriple {
        dirac,
        dimension: dim,
        node_count: nodes,
        edge_count: m,
        edges: edges.to_vec(),
    }
}

// ---------------------------------------------------------------------------
// D^2 computation (for verification)
// ---------------------------------------------------------------------------

/// Compute D^2 as a dense matrix. Used for testing that D^2 = Hodge Laplacian.
pub fn d_squared_dense(triple: &SpectralTriple) -> Vec<f64> {
    let d = triple.dirac.to_dense();
    let dim = triple.dimension;
    let mut result = vec![0.0; dim * dim];
    for i in 0..dim {
        for j in 0..dim {
            let mut sum = 0.0;
            for k in 0..dim {
                sum += d[i * dim + k] * d[k * dim + j];
            }
            result[i * dim + j] = sum;
        }
    }
    result
}

/// Extract the upper-left n x n block of a dense (n+m) x (n+m) matrix.
/// This is D^2 restricted to 0-forms = the graph Laplacian L_0.
pub fn extract_node_block(dense: &[f64], dim: usize, n: usize) -> Vec<f64> {
    let mut block = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            block[i * n + j] = dense[i * dim + j];
        }
    }
    block
}

/// Extract the lower-right m x m block of a dense (n+m) x (n+m) matrix.
/// This is D^2 restricted to 1-forms = the edge Laplacian L_1.
pub fn extract_edge_block(dense: &[f64], dim: usize, n: usize, m: usize) -> Vec<f64> {
    let mut block = vec![0.0; m * m];
    for i in 0..m {
        for j in 0..m {
            block[i * m + j] = dense[(n + i) * dim + (n + j)];
        }
    }
    block
}

// ---------------------------------------------------------------------------
// Graph Laplacian (for comparison)
// ---------------------------------------------------------------------------

/// Build the graph Laplacian L_0 directly from edges. Used for verification.
/// L_0[i,i] = sum of weights of edges incident to i
/// L_0[i,j] = -w_{ij} if (i,j) is an edge
pub fn graph_laplacian(nodes: usize, edges: &[(usize, usize, f64)]) -> Vec<f64> {
    let mut l = vec![0.0; nodes * nodes];
    for &(from, to, weight) in edges {
        let (i, j) = if from <= to { (from, to) } else { (to, from) };
        l[i * nodes + j] -= weight;
        l[j * nodes + i] -= weight;
        l[i * nodes + i] += weight;
        l[j * nodes + j] += weight;
    }
    l
}

// ---------------------------------------------------------------------------
// Eigenvalue computation (Jacobi method)
// ---------------------------------------------------------------------------

/// Compute eigenvalues of a symmetric dense matrix using the Jacobi method.
/// Returns eigenvalues sorted by magnitude (ascending).
pub fn jacobi_eigenvalues(matrix: &[f64], n: usize) -> Vec<f64> {
    if n == 0 { return Vec::new(); }
    if n == 1 { return vec![matrix[0]]; }

    let mut a = matrix.to_vec();
    let max_iter = 100 * n * n;
    let eps = 1e-12;

    for _ in 0..max_iter {
        // Find largest off-diagonal element
        let mut max_val = 0.0_f64;
        let mut p = 0;
        let mut q = 1;
        for i in 0..n {
            for j in (i + 1)..n {
                let val = a[i * n + j].abs();
                if val > max_val {
                    max_val = val;
                    p = i;
                    q = j;
                }
            }
        }
        if max_val < eps {
            break;
        }

        let app = a[p * n + p];
        let aqq = a[q * n + q];
        let apq = a[p * n + q];
        let theta = if (app - aqq).abs() < eps {
            std::f64::consts::FRAC_PI_4
        } else {
            0.5 * ((2.0 * apq) / (app - aqq)).atan()
        };
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        let mut new_a = a.clone();
        for i in 0..n {
            if i != p && i != q {
                let aip = a[i * n + p];
                let aiq = a[i * n + q];
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

    let mut eigenvalues: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();
    eigenvalues.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap_or(Ordering::Equal));
    eigenvalues
}

/// Compute eigenvalues AND eigenvectors of a symmetric dense matrix.
/// Returns (eigenvalues, eigenvectors) where eigenvectors[i] is the i-th eigenvector.
/// Sorted by eigenvalue magnitude (ascending).
pub fn jacobi_eigen(matrix: &[f64], n: usize) -> (Vec<f64>, Vec<Vec<f64>>) {
    if n == 0 { return (Vec::new(), Vec::new()); }
    if n == 1 { return (vec![matrix[0]], vec![vec![1.0]]); }

    let mut a = matrix.to_vec();
    // Eigenvector accumulator: starts as identity
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }

    let max_iter = 100 * n * n;
    let eps = 1e-12;

    for _ in 0..max_iter {
        let mut max_val = 0.0_f64;
        let mut p = 0;
        let mut q = 1;
        for i in 0..n {
            for j in (i + 1)..n {
                let val = a[i * n + j].abs();
                if val > max_val {
                    max_val = val;
                    p = i;
                    q = j;
                }
            }
        }
        if max_val < eps {
            break;
        }

        let app = a[p * n + p];
        let aqq = a[q * n + q];
        let apq = a[p * n + q];
        let theta = if (app - aqq).abs() < eps {
            std::f64::consts::FRAC_PI_4
        } else {
            0.5 * ((2.0 * apq) / (app - aqq)).atan()
        };
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        // Update A
        let mut new_a = a.clone();
        for i in 0..n {
            if i != p && i != q {
                let aip = a[i * n + p];
                let aiq = a[i * n + q];
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

        // Update eigenvector matrix V: V' = V * G where G is the rotation
        for i in 0..n {
            let vip = v[i * n + p];
            let viq = v[i * n + q];
            v[i * n + p] = cos_t * vip + sin_t * viq;
            v[i * n + q] = -sin_t * vip + cos_t * viq;
        }
    }

    let eigenvalues: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();

    // Sort by magnitude, keeping eigenvectors aligned
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| {
        eigenvalues[a].abs().partial_cmp(&eigenvalues[b].abs()).unwrap_or(Ordering::Equal)
    });

    let sorted_eigenvalues: Vec<f64> = indices.iter().map(|&i| eigenvalues[i]).collect();
    let sorted_eigenvectors: Vec<Vec<f64>> = indices.iter().map(|&idx| {
        (0..n).map(|row| v[row * n + idx]).collect()
    }).collect();

    (sorted_eigenvalues, sorted_eigenvectors)
}

// ---------------------------------------------------------------------------
// SpectralEmbedding
// ---------------------------------------------------------------------------

/// Compute first k eigenvectors of D, project to node space.
///
/// The embedding takes the first k eigenvectors sorted by eigenvalue magnitude
/// (skipping zero modes), projects them onto the node subspace (first n components),
/// and packs up to 16 dimensions per node.
pub fn spectral_embedding(triple: &SpectralTriple, k: usize) -> SpectralEmbedding {
    let dim = triple.dimension;
    let n = triple.node_count;
    let dense = triple.dirac.to_dense();

    let (eigenvalues, eigenvectors) = jacobi_eigen(&dense, dim);

    // Take first min(k, 16, available) non-zero eigenvectors projected to node space
    let mut components = vec![[0.0f64; 16]; n];
    let max_k = k.min(16).min(eigenvectors.len());

    let mut used = 0;
    for (ev_idx, &eval) in eigenvalues.iter().enumerate() {
        if used >= max_k {
            break;
        }
        if eval.abs() < 1e-10 {
            continue; // skip zero modes (kernel)
        }
        // Project this eigenvector onto node subspace (first n entries)
        let ev = &eigenvectors[ev_idx];
        for node in 0..n {
            components[node][used] = ev[node];
        }
        used += 1;
    }

    SpectralEmbedding { components }
}

// ---------------------------------------------------------------------------
// Connes Distance (Dijkstra with weights 1/sqrt(w))
// ---------------------------------------------------------------------------

/// State for Dijkstra's priority queue.
#[derive(PartialEq)]
struct DijkstraState {
    cost: f64,
    node: usize,
}

impl Eq for DijkstraState {}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior with BinaryHeap (which is max-heap)
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

/// Compute the Connes distance between two vertices.
///
/// For commutative algebras on graphs, the Connes distance reduces to
/// the shortest path with edge lengths 1/sqrt(w_e).
///
/// Returns f64::INFINITY if the vertices are not connected.
pub fn connes_distance(
    triple: &SpectralTriple,
    p: usize,
    q: usize,
) -> f64 {
    if p == q {
        return 0.0;
    }

    let n = triple.node_count;
    assert!(p < n && q < n, "vertex indices must be < node_count");

    // Build adjacency list with Connes edge lengths
    let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for &(from, to, weight) in &triple.edges {
        let length = 1.0 / weight.sqrt();
        adj[from].push((to, length));
        adj[to].push((from, length));
    }

    // Dijkstra
    let mut dist = vec![f64::INFINITY; n];
    dist[p] = 0.0;
    let mut heap = BinaryHeap::new();
    heap.push(DijkstraState { cost: 0.0, node: p });

    while let Some(DijkstraState { cost, node }) = heap.pop() {
        if node == q {
            return cost;
        }
        if cost > dist[node] {
            continue;
        }
        for &(next, edge_len) in &adj[node] {
            let new_cost = cost + edge_len;
            if new_cost < dist[next] {
                dist[next] = new_cost;
                heap.push(DijkstraState { cost: new_cost, node: next });
            }
        }
    }

    dist[q]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ===================================================================
    // Phase 1: Dirac operator construction
    // ===================================================================

    /// Helper: build a path graph 0 -- 1 -- 2 with unit weights.
    fn path_3() -> (usize, Vec<(usize, usize, f64)>) {
        (3, vec![(0, 1, 1.0), (1, 2, 1.0)])
    }

    /// Helper: build a triangle graph 0 -- 1 -- 2 -- 0 with unit weights.
    fn triangle() -> (usize, Vec<(usize, usize, f64)>) {
        (3, vec![(0, 1, 1.0), (1, 2, 1.0), (0, 2, 1.0)])
    }

    /// Helper: build a path graph 0 -- 1 -- 2 -- 3 with unit weights.
    fn path_4() -> (usize, Vec<(usize, usize, f64)>) {
        (4, vec![(0, 1, 1.0), (1, 2, 1.0), (2, 3, 1.0)])
    }

    // --- D^2 = Hodge Laplacian ---

    #[test]
    fn d_squared_equals_graph_laplacian_on_node_block() {
        // For the path graph 0-1-2:
        // D^2 restricted to the node subspace should equal the graph Laplacian L_0.
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);

        let d2 = d_squared_dense(&triple);
        let l0_from_d2 = extract_node_block(&d2, triple.dimension, n);
        let l0_direct = graph_laplacian(n, &edges);

        for i in 0..n {
            for j in 0..n {
                assert!(
                    (l0_from_d2[i * n + j] - l0_direct[i * n + j]).abs() < 1e-10,
                    "D^2 node block [{},{}] = {}, expected {} (graph Laplacian)",
                    i, j, l0_from_d2[i * n + j], l0_direct[i * n + j]
                );
            }
        }
    }

    #[test]
    fn d_squared_equals_graph_laplacian_triangle() {
        let (n, edges) = triangle();
        let triple = construct_dirac(n, &edges);

        let d2 = d_squared_dense(&triple);
        let l0_from_d2 = extract_node_block(&d2, triple.dimension, n);
        let l0_direct = graph_laplacian(n, &edges);

        for i in 0..n {
            for j in 0..n {
                assert!(
                    (l0_from_d2[i * n + j] - l0_direct[i * n + j]).abs() < 1e-10,
                    "Triangle: D^2 node block [{},{}] = {}, expected {}",
                    i, j, l0_from_d2[i * n + j], l0_direct[i * n + j]
                );
            }
        }
    }

    // --- D is self-adjoint (D = D^T) ---

    #[test]
    fn d_is_self_adjoint() {
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let dense = triple.dirac.to_dense();
        let dim = triple.dimension;

        for i in 0..dim {
            for j in 0..dim {
                assert!(
                    (dense[i * dim + j] - dense[j * dim + i]).abs() < 1e-10,
                    "D is not self-adjoint at [{},{}]: {} != {}",
                    i, j, dense[i * dim + j], dense[j * dim + i]
                );
            }
        }
    }

    #[test]
    fn d_is_self_adjoint_triangle() {
        let (n, edges) = triangle();
        let triple = construct_dirac(n, &edges);
        let dense = triple.dirac.to_dense();
        let dim = triple.dimension;

        for i in 0..dim {
            for j in 0..dim {
                assert!(
                    (dense[i * dim + j] - dense[j * dim + i]).abs() < 1e-10,
                    "Triangle D not self-adjoint at [{},{}]",
                    i, j
                );
            }
        }
    }

    // --- Eigenvalues of D are symmetric about 0 ---

    #[test]
    fn d_spectrum_symmetric_about_zero() {
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let dense = triple.dirac.to_dense();
        let mut eigenvalues = jacobi_eigenvalues(&dense, triple.dimension);
        eigenvalues.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        // For each positive eigenvalue, there should be a matching negative one
        let pos: Vec<f64> = eigenvalues.iter().filter(|&&v| v > 1e-10).cloned().collect();
        let neg: Vec<f64> = eigenvalues.iter().filter(|&&v| v < -1e-10).map(|v| -v).collect();

        assert_eq!(pos.len(), neg.len(), "spectrum not symmetric: pos={:?}, neg={:?}", pos, neg);

        let mut pos_sorted = pos.clone();
        let mut neg_sorted = neg.clone();
        pos_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        neg_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (p, n) in pos_sorted.iter().zip(neg_sorted.iter()) {
            assert!(
                (p - n).abs() < 1e-8,
                "spectrum not symmetric: +{} vs -{}",
                p, n
            );
        }
    }

    // --- Eigenvalues of D include +/- sqrt(eigenvalues of L_0) ---

    #[test]
    fn d_eigenvalues_contain_sqrt_laplacian_eigenvalues() {
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);

        // Get Laplacian eigenvalues
        let l0 = graph_laplacian(n, &edges);
        let mut l0_evals = jacobi_eigenvalues(&l0, n);
        l0_evals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Get D eigenvalues
        let dense = triple.dirac.to_dense();
        let mut d_evals = jacobi_eigenvalues(&dense, triple.dimension);
        d_evals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // For each nonzero Laplacian eigenvalue lambda, D should have +/-sqrt(lambda)
        for &l_eval in &l0_evals {
            if l_eval < 1e-10 {
                continue;
            }
            let expected = l_eval.sqrt();
            let found_pos = d_evals.iter().any(|&d| (d - expected).abs() < 1e-6);
            let found_neg = d_evals.iter().any(|&d| (d + expected).abs() < 1e-6);
            assert!(
                found_pos,
                "D spectrum missing +sqrt({}) = {}. D evals: {:?}",
                l_eval, expected, d_evals
            );
            assert!(
                found_neg,
                "D spectrum missing -sqrt({}) = {}. D evals: {:?}",
                l_eval, expected, d_evals
            );
        }
    }

    // --- Kernel dimension = b_0 + b_1 ---

    #[test]
    fn d_kernel_dimension_path_3() {
        // Path graph P_3: b_0 = 1 (connected), b_1 = 0 (tree), dim(ker D) = 1
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let dense = triple.dirac.to_dense();
        let eigenvalues = jacobi_eigenvalues(&dense, triple.dimension);
        let kernel_dim = eigenvalues.iter().filter(|&&v| v.abs() < 1e-8).count();
        assert_eq!(kernel_dim, 1, "P_3: expected ker(D) dim = 1, got {}", kernel_dim);
    }

    #[test]
    fn d_kernel_dimension_triangle() {
        // Triangle C_3: b_0 = 1, b_1 = 1 (one cycle), dim(ker D) = 2
        let (n, edges) = triangle();
        let triple = construct_dirac(n, &edges);
        let dense = triple.dirac.to_dense();
        let eigenvalues = jacobi_eigenvalues(&dense, triple.dimension);
        let kernel_dim = eigenvalues.iter().filter(|&&v| v.abs() < 1e-8).count();
        assert_eq!(kernel_dim, 2, "C_3: expected ker(D) dim = 2, got {}. Evals: {:?}", kernel_dim, eigenvalues);
    }

    // ===================================================================
    // Phase 1: SpectralEmbedding
    // ===================================================================

    #[test]
    fn spectral_embedding_dimension() {
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let embedding = spectral_embedding(&triple, 16);
        assert_eq!(embedding.components.len(), n, "embedding should have one vector per node");
    }

    #[test]
    fn spectral_embedding_path_graph_ordering() {
        // For a path graph 0--1--2, the first eigenvector should place
        // node 1 (the middle) between nodes 0 and 2.
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let embedding = spectral_embedding(&triple, 16);

        let v0 = embedding.components[0][0];
        let v1 = embedding.components[1][0];
        let v2 = embedding.components[2][0];

        // Middle node should be between endpoints in the first embedding dimension
        // (or they could be sign-flipped, so check both orderings)
        let between = (v1 >= v0.min(v2) && v1 <= v0.max(v2)) ||
                      (v1 <= v0.max(v2) && v1 >= v0.min(v2));
        assert!(
            between,
            "Middle node should be between endpoints in embedding dim 0: v0={}, v1={}, v2={}",
            v0, v1, v2
        );
    }

    // ===================================================================
    // Phase 1: Connes Distance
    // ===================================================================

    #[test]
    fn connes_distance_self_is_zero() {
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        for i in 0..n {
            assert_eq!(connes_distance(&triple, i, i), 0.0);
        }
    }

    #[test]
    fn connes_distance_adjacent_unweighted() {
        // For unit weight edges: d(i,j) = 1/sqrt(1) = 1
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let d01 = connes_distance(&triple, 0, 1);
        let d12 = connes_distance(&triple, 1, 2);
        assert!(
            (d01 - 1.0).abs() < 1e-10,
            "d(0,1) should be 1.0, got {}",
            d01
        );
        assert!(
            (d12 - 1.0).abs() < 1e-10,
            "d(1,2) should be 1.0, got {}",
            d12
        );
    }

    #[test]
    fn connes_distance_path_graph() {
        // Path 0-1-2: d(0,2) = d(0,1) + d(1,2) = 1 + 1 = 2
        let (n, edges) = path_3();
        let triple = construct_dirac(n, &edges);
        let d02 = connes_distance(&triple, 0, 2);
        assert!(
            (d02 - 2.0).abs() < 1e-10,
            "d(0,2) should be 2.0 on path graph, got {}",
            d02
        );
    }

    #[test]
    fn connes_distance_path_4() {
        // Path 0-1-2-3: d(0,3) = 3
        let (n, edges) = path_4();
        let triple = construct_dirac(n, &edges);
        let d03 = connes_distance(&triple, 0, 3);
        assert!(
            (d03 - 3.0).abs() < 1e-10,
            "d(0,3) should be 3.0, got {}",
            d03
        );
    }

    #[test]
    fn connes_distance_triangle_inequality() {
        // For the triangle: d(0,2) <= d(0,1) + d(1,2)
        let (n, edges) = triangle();
        let triple = construct_dirac(n, &edges);

        for p in 0..n {
            for q in 0..n {
                for r in 0..n {
                    let dpq = connes_distance(&triple, p, q);
                    let dqr = connes_distance(&triple, q, r);
                    let dpr = connes_distance(&triple, p, r);
                    assert!(
                        dpr <= dpq + dqr + 1e-10,
                        "Triangle inequality violated: d({},{})={} > d({},{})={} + d({},{})={}",
                        p, r, dpr, p, q, dpq, q, r, dqr
                    );
                }
            }
        }
    }

    #[test]
    fn connes_distance_symmetric() {
        let (n, edges) = triangle();
        let triple = construct_dirac(n, &edges);
        for p in 0..n {
            for q in 0..n {
                let dpq = connes_distance(&triple, p, q);
                let dqp = connes_distance(&triple, q, p);
                assert!(
                    (dpq - dqp).abs() < 1e-10,
                    "Distance not symmetric: d({},{})={} != d({},{})={}",
                    p, q, dpq, q, p, dqp
                );
            }
        }
    }

    #[test]
    fn connes_distance_weighted_edge() {
        // Edge with weight 4: d = 1/sqrt(4) = 0.5
        let triple = construct_dirac(2, &[(0, 1, 4.0)]);
        let d01 = connes_distance(&triple, 0, 1);
        assert!(
            (d01 - 0.5).abs() < 1e-10,
            "d(0,1) with weight 4 should be 0.5, got {}",
            d01
        );
    }

    #[test]
    fn connes_distance_triangle_shortcut() {
        // Triangle with unit weights: d(0,2) = 1 (direct edge), not 2 (via 1)
        let (n, edges) = triangle();
        let triple = construct_dirac(n, &edges);
        let d02 = connes_distance(&triple, 0, 2);
        assert!(
            (d02 - 1.0).abs() < 1e-10,
            "d(0,2) in triangle should be 1.0 (direct edge), got {}",
            d02
        );
    }

    // ===================================================================
    // SparseMatrix unit tests
    // ===================================================================

    #[test]
    fn sparse_matrix_from_triplets_basic() {
        let mut triplets = vec![(0, 1, 2.0), (1, 0, 3.0)];
        let m = SparseMatrix::from_triplets(2, 2, &mut triplets);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(0, 0), 0.0);
    }

    #[test]
    fn sparse_matrix_to_dense_identity() {
        let mut triplets = vec![(0, 0, 1.0), (1, 1, 1.0), (2, 2, 1.0)];
        let m = SparseMatrix::from_triplets(3, 3, &mut triplets);
        let dense = m.to_dense();
        assert_eq!(dense, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn sparse_mul_vec() {
        // [[1, 2], [3, 4]] * [1, 1] = [3, 7]
        let mut triplets = vec![(0, 0, 1.0), (0, 1, 2.0), (1, 0, 3.0), (1, 1, 4.0)];
        let m = SparseMatrix::from_triplets(2, 2, &mut triplets);
        let result = m.mul_vec(&[1.0, 1.0]);
        assert!((result[0] - 3.0).abs() < 1e-10);
        assert!((result[1] - 7.0).abs() < 1e-10);
    }
}
