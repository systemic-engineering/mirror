//! Matrix encoding for Prism projection matrices.
//!
//! A Prism IS a projection matrix. This module:
//! 1. Extracts the type vocabulary from a parsed grammar's type-def nodes.
//! 2. Encodes a Prism as an n×n projection matrix.
//! 3. Calls Fortran for linear algebra (preview, review, modify, compose).

use crate::ast::AstNode;
use crate::prism::Prism;

/// An n×n matrix in row-major order.
pub struct Matrix {
    pub n: usize,
    pub data: Vec<f64>, // n×n, row-major
}

impl Matrix {
    /// Create an n×n zero matrix.
    pub fn zeros(n: usize) -> Self {
        Matrix {
            n,
            data: vec![0.0; n * n],
        }
    }

    /// Create an n×n identity matrix.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n);
        for i in 0..n {
            m.data[i * n + i] = 1.0;
        }
        m
    }

    /// Get element at (row, col).
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.n + col]
    }

    /// Set element at (row, col).
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row * self.n + col] = val;
    }

    /// Transpose from row-major to column-major (for Fortran).
    fn to_column_major(&self) -> Vec<f64> {
        let mut col_major = vec![0.0; self.n * self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                col_major[j * self.n + i] = self.data[i * self.n + j];
            }
        }
        col_major
    }

    /// Create from column-major data (from Fortran).
    fn from_column_major(n: usize, col_major: &[f64]) -> Self {
        let mut data = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                data[i * n + j] = col_major[j * n + i];
            }
        }
        Matrix { n, data }
    }

    /// Serialize to bytes: 8-byte n (little-endian u64) + n*n f64s (little-endian IEEE 754).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.n * self.n * 8);
        bytes.extend_from_slice(&(self.n as u64).to_le_bytes());
        for &val in &self.data {
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        bytes
    }

    /// Deserialize from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        let n = u64::from_le_bytes(bytes[0..8].try_into().ok()?) as usize;
        let expected = 8 + n * n * 8;
        if bytes.len() != expected {
            return None;
        }
        let mut data = Vec::with_capacity(n * n);
        for i in 0..n * n {
            let start = 8 + i * 8;
            let val = f64::from_le_bytes(bytes[start..start + 8].try_into().ok()?);
            data.push(val);
        }
        Some(Matrix { n, data })
    }
}

// ---------------------------------------------------------------------------
// Fortran FFI
// ---------------------------------------------------------------------------

extern "C" {
    fn prism_preview(
        n: i32,
        projection: *const f64,
        source: *const f64,
        focus: *mut f64,
        matched: *mut i32,
    );
    fn prism_review(n: i32, projection: *const f64, focus: *const f64, result: *mut f64);
    fn prism_modify(
        n: i32,
        projection: *const f64,
        source: *const f64,
        transform: *const f64,
        result: *mut f64,
    );
    fn prism_compose(n: i32, p1: *const f64, p2: *const f64, composed: *mut f64);
}

/// Project source through the prism. Returns (focus, matched).
pub fn preview(projection: &Matrix, source: &[f64]) -> (Vec<f64>, bool) {
    assert_eq!(source.len(), projection.n);
    let n = projection.n as i32;
    let col_major = projection.to_column_major();
    let mut focus = vec![0.0; projection.n];
    let mut matched: i32 = 0;
    unsafe {
        prism_preview(
            n,
            col_major.as_ptr(),
            source.as_ptr(),
            focus.as_mut_ptr(),
            &mut matched,
        );
    }
    (focus, matched != 0)
}

/// Embed focus back through the prism transpose.
pub fn review(projection: &Matrix, focus: &[f64]) -> Vec<f64> {
    assert_eq!(focus.len(), projection.n);
    let n = projection.n as i32;
    let col_major = projection.to_column_major();
    let mut result = vec![0.0; projection.n];
    unsafe {
        prism_review(n, col_major.as_ptr(), focus.as_ptr(), result.as_mut_ptr());
    }
    result
}

/// Modify: keep complement, transform the matched part.
pub fn modify(projection: &Matrix, source: &[f64], transform: &Matrix) -> Vec<f64> {
    assert_eq!(source.len(), projection.n);
    assert_eq!(transform.n, projection.n);
    let n = projection.n as i32;
    let proj_cm = projection.to_column_major();
    let trans_cm = transform.to_column_major();
    let mut result = vec![0.0; projection.n];
    unsafe {
        prism_modify(
            n,
            proj_cm.as_ptr(),
            source.as_ptr(),
            trans_cm.as_ptr(),
            result.as_mut_ptr(),
        );
    }
    result
}

/// Compose two prisms: composed = p2 * p1.
pub fn compose(p1: &Matrix, p2: &Matrix) -> Matrix {
    assert_eq!(p1.n, p2.n);
    let n = p1.n as i32;
    let p1_cm = p1.to_column_major();
    let p2_cm = p2.to_column_major();
    let mut composed_cm = vec![0.0; p1.n * p1.n];
    unsafe {
        prism_compose(n, p1_cm.as_ptr(), p2_cm.as_ptr(), composed_cm.as_mut_ptr());
    }
    Matrix::from_column_major(p1.n, &composed_cm)
}

// ---------------------------------------------------------------------------
// Vocabulary extraction + Prism → Matrix encoding
// ---------------------------------------------------------------------------

/// Extract the type vocabulary from a parsed tree's type-def nodes.
///
/// Walks the tree, finds Form("type-def") nodes, collects their variant
/// children's values. For `type = grammar | type` → `["grammar", "type"]`.
pub fn extract_vocabulary(tree: &Prism<AstNode>) -> Vec<String> {
    let mut vocab = Vec::new();
    collect_vocabulary(tree, &mut vocab);
    vocab
}

fn collect_vocabulary(node: &Prism<AstNode>, vocab: &mut Vec<String>) {
    if node.data().is_form("type-def") {
        for child in node.children() {
            if child.data().is_form("variant") {
                let name = &child.data().value;
                if !vocab.contains(name) {
                    vocab.push(name.clone());
                }
            }
        }
    }
    for child in node.children() {
        collect_vocabulary(child, vocab);
    }
}

/// Encode a Prism tree as a projection matrix.
///
/// n = vocabulary.len(). Walks the tree to determine structural reachability
/// between types. P[i][j] = 1.0 if type j is reachable from type i.
pub fn encode(tree: &Prism<AstNode>, vocabulary: &[String]) -> Matrix {
    let n = vocabulary.len();
    let mut matrix = Matrix::zeros(n);

    // Walk the tree, determine reachability
    collect_reachability(tree, vocabulary, &mut matrix);

    // Ensure diagonal — every type reaches itself
    for i in 0..n {
        matrix.set(i, i, 1.0);
    }

    matrix
}

fn collect_reachability(node: &Prism<AstNode>, vocabulary: &[String], matrix: &mut Matrix) {
    // If this node's value matches a type in the vocabulary, check what its
    // children can reach.
    let parent_idx = vocabulary.iter().position(|v| v == &node.data().value);

    for child in node.children() {
        let child_idx = vocabulary.iter().position(|v| v == &child.data().value);

        if let (Some(pi), Some(ci)) = (parent_idx, child_idx) {
            matrix.set(pi, ci, 1.0);
        }

        collect_reachability(child, vocabulary, matrix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parse;
    use crate::Vector;

    #[test]
    fn extract_vocabulary_from_grammar() {
        let source = "out @grammar {\n  type = grammar | type\n}\n";
        let tree = Parse.trace(source.to_string()).into_result().unwrap();
        let vocab = extract_vocabulary(&tree);
        assert_eq!(vocab, vec!["grammar", "type"]);
    }

    #[test]
    fn extract_vocabulary_empty_tree() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\n";
        let tree = Parse.trace(source.to_string()).into_result().unwrap();
        let vocab = extract_vocabulary(&tree);
        assert!(vocab.is_empty());
    }

    #[test]
    fn extract_vocabulary_from_main_conv() {
        let source = include_str!("../main.conv");
        let tree = Parse.trace(source.to_string()).into_result().unwrap();
        let vocab = extract_vocabulary(&tree);
        assert_eq!(vocab, vec!["grammar", "type"]);
    }

    #[test]
    fn encode_produces_2x2_matrix() {
        let source = "out @grammar {\n  type = grammar | type\n}\n";
        let tree = Parse.trace(source.to_string()).into_result().unwrap();
        let vocab = extract_vocabulary(&tree);
        let matrix = encode(&tree, &vocab);
        assert_eq!(matrix.n, 2);
        // Diagonal should be 1.0 (self-reachability)
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(1, 1), 1.0);
    }

    #[test]
    fn matrix_identity() {
        let m = Matrix::identity(3);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(0, 1), 0.0);
        assert_eq!(m.get(1, 0), 0.0);
    }

    #[test]
    fn matrix_serialization_roundtrip() {
        let mut m = Matrix::zeros(2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 0.5);
        m.set(1, 0, 0.0);
        m.set(1, 1, 1.0);
        let bytes = m.to_bytes();
        let m2 = Matrix::from_bytes(&bytes).unwrap();
        assert_eq!(m2.n, 2);
        assert_eq!(m2.get(0, 0), 1.0);
        assert_eq!(m2.get(0, 1), 0.5);
        assert_eq!(m2.get(1, 0), 0.0);
        assert_eq!(m2.get(1, 1), 1.0);
    }

    #[test]
    fn matrix_from_bytes_too_short() {
        assert!(Matrix::from_bytes(&[0; 4]).is_none());
    }

    #[test]
    fn matrix_from_bytes_wrong_size() {
        // Header says n=2 but not enough data
        let mut bytes = (2u64).to_le_bytes().to_vec();
        bytes.extend_from_slice(&[0; 8]); // only 1 f64 instead of 4
        assert!(Matrix::from_bytes(&bytes).is_none());
    }

    // -- Fortran FFI tests --

    #[test]
    fn fortran_compose_identity() {
        let id = Matrix::identity(2);
        let composed = compose(&id, &id);
        assert_eq!(composed.n, 2);
        assert!((composed.get(0, 0) - 1.0).abs() < 1e-12);
        assert!((composed.get(0, 1) - 0.0).abs() < 1e-12);
        assert!((composed.get(1, 0) - 0.0).abs() < 1e-12);
        assert!((composed.get(1, 1) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn fortran_preview_matched() {
        let mut proj = Matrix::zeros(2);
        proj.set(0, 0, 1.0); // project onto first axis
        let source = vec![1.0, 0.0];
        let (focus, matched) = preview(&proj, &source);
        assert!(matched);
        assert!((focus[0] - 1.0).abs() < 1e-12);
        assert!((focus[1] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn fortran_preview_unmatched() {
        let mut proj = Matrix::zeros(2);
        proj.set(0, 0, 1.0); // project onto first axis
        let source = vec![0.0, 1.0]; // orthogonal to projection
        let (focus, matched) = preview(&proj, &source);
        assert!(!matched);
        assert!((focus[0] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn fortran_review() {
        let id = Matrix::identity(2);
        let focus = vec![3.0, 4.0];
        let result = review(&id, &focus);
        assert!((result[0] - 3.0).abs() < 1e-12);
        assert!((result[1] - 4.0).abs() < 1e-12);
    }

    #[test]
    fn fortran_modify() {
        let mut proj = Matrix::zeros(2);
        proj.set(0, 0, 1.0); // project onto first axis
        let mut transform = Matrix::identity(2);
        transform.set(0, 0, 2.0); // double the first component
        let source = vec![1.0, 1.0];
        let result = modify(&proj, &source, &transform);
        // complement: (I-P)*source = [0.0, 1.0]
        // T * P * source = T * [1.0, 0.0] = [2.0, 0.0]
        // result = [2.0, 1.0]
        assert!((result[0] - 2.0).abs() < 1e-12);
        assert!((result[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn fortran_compose_projection_idempotent() {
        // A projection matrix P satisfies P*P = P
        let mut p = Matrix::zeros(2);
        p.set(0, 0, 1.0);
        let pp = compose(&p, &p);
        assert!((pp.get(0, 0) - 1.0).abs() < 1e-12);
        assert!((pp.get(0, 1) - 0.0).abs() < 1e-12);
        assert!((pp.get(1, 0) - 0.0).abs() < 1e-12);
        assert!((pp.get(1, 1) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn extract_vocabulary_dedup() {
        // A grammar with duplicate variant names should deduplicate
        let source = "grammar @test {\n  type = a | b | a\n}\n";
        let tree = Parse.trace(source.to_string()).into_result().unwrap();
        let vocab = extract_vocabulary(&tree);
        assert_eq!(vocab, vec!["a", "b"]);
    }

    #[test]
    fn extract_vocabulary_skips_non_variants() {
        // A type-def with a non-variant child should be skipped
        use crate::ast::{self, Span};
        use crate::domain::conversation::Kind;
        let span = Span::new(0, 0);
        let variant = ast::ast_leaf(Kind::Form, "variant", "a", span);
        let non_variant = ast::ast_leaf(Kind::Atom, "field", "extra", span);
        let type_def = ast::ast_branch(
            Kind::Form,
            "type-def",
            "=",
            span,
            vec![variant, non_variant],
        );
        let tree = ast::ast_branch(Kind::Form, "root", "", span, vec![type_def]);
        let vocab = extract_vocabulary(&tree);
        assert_eq!(vocab, vec!["a"]);
    }

    #[test]
    fn encode_reachability_parent_child() {
        // When a vocabulary-named node contains a child whose value also matches
        // a vocabulary entry, collect_reachability sets the off-diagonal element.
        // Build a synthetic tree: a "parent" node containing a "child" node.
        use crate::ast::{self, Span};
        use crate::domain::conversation::Kind;
        let span = Span::new(0, 0);
        let child_node = ast::ast_leaf(Kind::Atom, "field", "child", span);
        let parent_node = ast::ast_branch(Kind::Atom, "field", "parent", span, vec![child_node]);
        let tree = ast::ast_branch(Kind::Form, "root", "", span, vec![parent_node]);

        let vocab = vec!["parent".to_string(), "child".to_string()];
        let matrix = encode(&tree, &vocab);
        assert_eq!(matrix.n, 2);
        // parent → child reachability
        assert_eq!(matrix.get(0, 1), 1.0);
        // Diagonal — self-reachability
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(1, 1), 1.0);
    }

    #[test]
    fn column_major_roundtrip() {
        let mut m = Matrix::zeros(2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 2.0);
        m.set(1, 0, 3.0);
        m.set(1, 1, 4.0);
        let cm = m.to_column_major();
        // Column-major: col 0 = [1, 3], col 1 = [2, 4]
        assert_eq!(cm, vec![1.0, 3.0, 2.0, 4.0]);
        let m2 = Matrix::from_column_major(2, &cm);
        assert_eq!(m2.get(0, 0), 1.0);
        assert_eq!(m2.get(0, 1), 2.0);
        assert_eq!(m2.get(1, 0), 3.0);
        assert_eq!(m2.get(1, 1), 4.0);
    }
}
