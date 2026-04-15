//! Feature extraction bridge: @ai.measure() codegen.
//!
//! Maps .mirror compilation output to Fate's 16-dimensional feature space.
//! The compiler measuring itself.

use std::path::Path;

use fate::Features;
use prism::lambda::LambdaFn;
use prism::Loss;

use crate::lambda_phases::{Parse, SourceText};
use crate::loss::MirrorLoss;
use crate::mirror_ast::MirrorAST;
use prism::MerkleTree;

/// Count children of a MirrorAST node (top-level declarations).
fn ast_child_count(ast: &MirrorAST) -> usize {
    ast.children().len()
}

/// Count import nodes (`in @X`) in a MirrorAST's children.
fn ast_import_count(ast: &MirrorAST) -> usize {
    ast.children()
        .iter()
        .filter(|c| matches!(c, MirrorAST::Import(_)))
        .count()
}

/// Extract Fate features from a .mirror file.
///
/// Feature mapping:
/// - [0] holonomy (information loss rate)
/// - [1] parse resolution (0.0-1.0)
/// - [2] declaration count
/// - [3] unrecognized count (novelty)
/// - [4] error count
/// - [5] grammar ref count (in @X)
/// - [6..15] dark dimensions (zero until trained)
pub fn extract_features(source: &str) -> (Features, MirrorLoss) {
    let parsed = Parse.reduce(SourceText(source.to_string()));

    match parsed {
        prism::Imperfect::Success(ast) => {
            let decl_count = ast_child_count(&ast.0);
            let grammar_ref_count = ast_import_count(&ast.0);

            let mut features = [0.0_f64; 16];
            features[0] = 0.0; // holonomy: zero for crystal
            features[1] = 1.0; // full resolution
            features[2] = decl_count as f64;
            features[3] = 0.0; // no unrecognized
            features[4] = 0.0; // no errors
            features[5] = grammar_ref_count as f64;

            (features, MirrorLoss::zero())
        }
        prism::Imperfect::Partial(ast, loss) => {
            let decl_count = ast_child_count(&ast.0);
            let grammar_ref_count = ast_import_count(&ast.0);

            let holonomy = loss.holonomy();
            let unrecognized = loss.parse.warnings.len();

            let mut features = [0.0_f64; 16];
            features[0] = holonomy;
            features[1] = loss.resolution.resolution_ratio;
            features[2] = decl_count as f64;
            features[3] = unrecognized as f64;
            features[4] = 0.0; // partial, not failure
            features[5] = grammar_ref_count as f64;

            (features, loss)
        }
        prism::Imperfect::Failure(_err, loss) => {
            let mut features = [0.0_f64; 16];
            features[0] = loss.holonomy();
            features[1] = 0.0;
            features[4] = 1.0; // error
            (features, loss)
        }
    }
}

/// Extract features from a file path.
pub fn extract_features_from_file(path: &Path) -> Result<(Features, MirrorLoss), std::io::Error> {
    let source = std::fs::read_to_string(path)?;
    Ok(extract_features(&source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism::Loss;

    #[test]
    fn extract_features_returns_16_dimensions() {
        let source = "in @mirror\ntype x = a | b\n";
        let (features, _loss) = extract_features(source);
        assert_eq!(features.len(), 16);
    }

    #[test]
    fn clean_file_has_zero_holonomy() {
        let source = "in @mirror\ntype x = a | b\n";
        let (features, loss) = extract_features(source);
        assert_eq!(features[0], 0.0, "holonomy should be 0 for clean file");
        assert!(loss.is_zero() || features[0] == 0.0);
    }

    #[test]
    fn partial_file_has_positive_holonomy() {
        // 'io' is not a recognized keyword, so it generates parse warnings
        let source = "in @mirror\nio tick(features) => imperfect\ntype x = a\n";
        let (features, _loss) = extract_features(source);
        assert!(
            features[0] > 0.0,
            "holonomy should be > 0 for partial file, got {}",
            features[0]
        );
    }

    #[test]
    fn different_content_produces_different_features() {
        let source_a = "in @mirror\ntype x = a | b\n";
        let source_b = "in @mirror\nin @property\ntype x = a | b\ntype y = c | d\n";
        let (fa, _) = extract_features(source_a);
        let (fb, _) = extract_features(source_b);
        // Different declaration counts
        assert_ne!(
            fa[2], fb[2],
            "different sources should have different declaration counts"
        );
    }

    #[test]
    fn grammar_refs_counted() {
        let source = "in @mirror\nin @property\nin @fate\ntype x = a\n";
        let (features, _) = extract_features(source);
        assert_eq!(
            features[5], 3.0,
            "should count 3 grammar refs (in @mirror, in @property, in @fate)"
        );
    }
}
