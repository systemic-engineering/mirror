//! Integration test: tokenize real spectral-db node text and verify
//! that structural connections emerge from shared content.

use mirror::nl::{tokenize, shared_oid_count, all_oids, leaf_oids, Token};
use mirror::kernel::ContentAddressed;

#[test]
fn mara_finding_connects_to_mara_fix() {
    // Real node text from spectral-db graph
    let finding = tokenize(
        "approx_lambda_2 does NOT compute lambda_2. It returns \
         min_degree*n/(n-1), which is an UPPER bound on the largest \
         eigenvalue via Gershgorin, not a lower bound on lambda_2."
    );
    let fix = tokenize(
        "Fixed approx_lambda_2 — now uses Jacobi eigenvalue iteration \
         for real Fiedler value. P3: 1.0 (was 1.5). 10 tests pass."
    );

    let shared = shared_oid_count(&finding, &fix);

    // Should share: approx_lambda_2, lambda_2, approx, lambda, 2, eigenvalu
    assert!(
        shared >= 3,
        "expected >= 3 shared OIDs between finding and fix, got {}",
        shared
    );
}

#[test]
fn unrelated_nodes_no_connection() {
    let spectral_node = tokenize(
        "Jacobi algorithm clones the ENTIRE matrix every iteration."
    );
    let pressure_node = tokenize(
        "shed() never actually evicts nodes. Pressure shedding is \
         informational only."
    );

    let shared = shared_oid_count(&spectral_node, &pressure_node);

    // These discuss completely different topics — minimal overlap
    assert!(
        shared <= 2,
        "expected <= 2 shared OIDs between unrelated nodes, got {}",
        shared
    );
}

#[test]
fn deterministic_oids() {
    // Same text always produces the same OIDs
    let text = "spectral eigenvalue computation";
    let tree_a = tokenize(text);
    let tree_b = tokenize(text);

    let oids_a = all_oids(&tree_a);
    let oids_b = all_oids(&tree_b);

    assert_eq!(oids_a.len(), oids_b.len());
    for (a, b) in oids_a.iter().zip(oids_b.iter()) {
        assert_eq!(a, b);
    }
}

#[test]
fn token_tree_is_fractal() {
    // Compound tokens produce nested Prism::Fractal nodes
    let tree = tokenize("approx_lambda_2");

    // Walk the tree and count depth
    fn max_depth(node: &mirror::prism::Prism<Token>) -> usize {
        match node {
            mirror::prism::Prism::Shard { .. } => 1,
            mirror::prism::Prism::Fractal { children, .. } => {
                1 + children.iter().map(|c| max_depth(c)).max().unwrap_or(0)
            }
            _ => 1,
        }
    }

    let depth = max_depth(&tree);
    // Root -> approx_lambda_2 (compound) -> approx/lambda/2 (leaves) = depth 3
    assert!(
        depth >= 3,
        "expected depth >= 3 for compound, got {}",
        depth
    );
}

#[test]
fn leaf_oids_are_content_addressed() {
    let tree = tokenize("eigenvalue computation");
    let oids = leaf_oids(&tree);
    assert_eq!(oids.len(), 2);

    // Each leaf OID should match the content_oid of the stemmed token
    let eigenvalu_oid = Token::word("eigenvalu").content_oid();
    let comput_oid = Token::word("comput").content_oid();
    assert!(oids.contains(&eigenvalu_oid), "should contain 'eigenvalu' OID");
    assert!(oids.contains(&comput_oid), "should contain 'comput' OID");
}

#[test]
fn cross_text_shared_stems() {
    // "computing" and "computation" both stem to "comput"
    let a = tokenize("computing eigenvalues");
    let b = tokenize("eigenvalue computation");
    let shared = shared_oid_count(&a, &b);
    // Should share "eigenvalu" and "comput" stems
    assert!(
        shared >= 2,
        "expected >= 2 shared OIDs (eigenvalu + comput), got {}",
        shared
    );
}
