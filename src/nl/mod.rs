//! NL — natural language tokenizer for Mirror stdlib.
//!
//! Decomposes text into content-addressed token trees.
//! Each token is a `Prism<Token>` node. Compound tokens
//! (underscore-joined, CamelCase) become Fractal nodes whose
//! children are their decomposed parts.

use rust_stemmers::{Algorithm, Stemmer};
use unicode_segmentation::UnicodeSegmentation;

pub mod compound;
pub mod stop_words;
pub mod token;

pub use compound::{CompoundNode, decompose};
pub use token::{Token, TokenKind};

use crate::kernel::{ContentAddressed, Oid};
use crate::prism::Prism;
use fragmentation::ref_::Ref;
use fragmentation::sha;

/// Tokenize natural language text into a content-addressed token tree.
///
/// Pipeline: UAX #29 segmentation -> lowercase -> stop word filter ->
/// Porter2 stemming -> compound decomposition -> Prism<Token> tree.
///
/// The returned Prism is a Fractal whose children are the token nodes.
/// Compound tokens are nested Fractals. Simple words are Shards.
pub fn tokenize(text: &str) -> Prism<Token> {
    let stemmer = Stemmer::create(Algorithm::English);
    let mut children: Vec<Prism<Token>> = Vec::new();

    // Use split_word_bounds to get word-boundary segments, then
    // reassemble compound tokens (connected by underscores, dots).
    // This preserves "approx_lambda_2" as a single compound rather
    // than splitting it into three separate words.
    let segments = extract_compound_words(text);

    for word in &segments {
        // Skip pure punctuation / whitespace
        if !word.chars().any(|c| c.is_alphanumeric()) {
            continue;
        }

        let lower = word.to_lowercase();

        // Skip stop words (only single words, not compounds)
        if !lower.contains('_') && !lower.contains('.') && stop_words::is_stop_word(&lower) {
            continue;
        }

        // Skip very short tokens (< 2 chars) unless they're part of a compound
        if lower.len() < 2 && !lower.contains('_') && !lower.contains('.') {
            continue;
        }

        // Compound decomposition (on original casing for CamelCase)
        let compound_node = compound::decompose(word);

        // Build the prism tree from the compound tree
        let prism_node = compound_to_prism(&compound_node, &stemmer);
        children.push(prism_node);
    }

    // Root node: the full text content-addressed
    let root_data = Token::compound(text);
    let root_ref = make_ref(&root_data);
    Prism::Fractal {
        ref_: root_ref,
        data: root_data,
        children,
    }
}

/// Extract compound words from text, preserving underscore-joined
/// and dot-joined tokens as single units.
///
/// Uses UAX #29 word boundaries to find word segments, then joins
/// adjacent words connected by underscores or dots.
fn extract_compound_words(text: &str) -> Vec<String> {
    let bounds: Vec<&str> = text.split_word_bounds().collect();
    let mut result: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_compound = false;

    for segment in &bounds {
        if *segment == "_" || *segment == "." {
            // Connector — join with previous and next word
            current.push_str(segment);
            in_compound = true;
        } else if in_compound && segment.chars().any(|c| c.is_alphanumeric()) {
            // Continue the compound
            current.push_str(segment);
            in_compound = false;
        } else {
            // New segment: flush current if non-empty
            if !current.is_empty() {
                // Trim trailing connectors
                let trimmed = current.trim_end_matches(|c| c == '_' || c == '.');
                if !trimmed.is_empty() {
                    result.push(trimmed.to_string());
                }
                current.clear();
            }
            in_compound = false;

            if segment.chars().any(|c| c.is_alphanumeric()) {
                current.push_str(segment);
            }
        }
    }

    // Flush remaining
    if !current.is_empty() {
        let trimmed = current.trim_end_matches(|c| c == '_' || c == '.');
        if !trimmed.is_empty() {
            result.push(trimmed.to_string());
        }
    }

    result
}

/// Convert a CompoundNode tree into a Prism<Token> tree.
fn compound_to_prism(node: &CompoundNode, stemmer: &Stemmer) -> Prism<Token> {
    if node.is_leaf() {
        // Leaf: stem the text, create a Shard
        let stemmed = stemmer.stem(&node.text.to_lowercase()).to_string();
        let data = Token::word(stemmed);
        let ref_ = make_ref(&data);
        Prism::Shard { ref_, data }
    } else {
        // Branch: compound node with children
        let children: Vec<Prism<Token>> = node.children.iter()
            .map(|child| compound_to_prism(child, stemmer))
            .collect();
        let data = Token::compound(&node.text.to_lowercase());
        let ref_ = make_ref(&data);
        Prism::Fractal { ref_, data, children }
    }
}

/// Create a Ref from content-addressed Token.
fn make_ref(data: &Token) -> Ref {
    let label = format!("token:{}", data.text);
    Ref::new(sha::hash(&label), &label)
}

/// Collect all leaf OIDs from a token tree.
/// These are the content addresses that create coincidence edges
/// when shared between nodes.
pub fn leaf_oids(tree: &Prism<Token>) -> Vec<Oid> {
    let mut oids = Vec::new();
    collect_leaf_oids(tree, &mut oids);
    oids
}

fn collect_leaf_oids(node: &Prism<Token>, out: &mut Vec<Oid>) {
    match node {
        Prism::Shard { data, .. } => {
            out.push(data.content_oid());
        }
        Prism::Fractal { children, .. } => {
            for child in children {
                collect_leaf_oids(child, out);
            }
        }
        Prism::Lens { .. } | Prism::Optics { .. } => {}
    }
}

/// Collect all OIDs (leaves and branches) from a token tree.
/// Branches carry compound OIDs that are shared when two nodes
/// contain the same compound term.
pub fn all_oids(tree: &Prism<Token>) -> Vec<Oid> {
    let mut oids = Vec::new();
    collect_all_oids(tree, &mut oids);
    oids
}

fn collect_all_oids(node: &Prism<Token>, out: &mut Vec<Oid>) {
    match node {
        Prism::Shard { data, .. } => {
            out.push(data.content_oid());
        }
        Prism::Fractal { data, children, .. } => {
            out.push(data.content_oid());
            for child in children {
                collect_all_oids(child, out);
            }
        }
        Prism::Lens { .. } | Prism::Optics { .. } => {}
    }
}

/// Count the shared OIDs between two token trees.
/// This is the structural overlap — the number of content addresses
/// that appear in both trees.
pub fn shared_oid_count(a: &Prism<Token>, b: &Prism<Token>) -> usize {
    let oids_a: std::collections::HashSet<String> = all_oids(a)
        .into_iter()
        .map(|o| o.as_ref().to_string())
        .collect();
    let oids_b: std::collections::HashSet<String> = all_oids(b)
        .into_iter()
        .map(|o| o.as_ref().to_string())
        .collect();
    oids_a.intersection(&oids_b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Task 5: Tokenization pipeline tests --

    #[test]
    fn tokenize_simple_sentence() {
        let tree = tokenize("Fixed the eigenvalue computation");
        // "the" is a stop word, filtered out
        // Remaining: "Fixed" -> stem "fix", "eigenvalue" -> stem "eigenvalu",
        //            "computation" -> stem "comput"
        let children = match &tree {
            Prism::Fractal { children, .. } => children,
            _ => panic!("expected Fractal root"),
        };
        assert_eq!(children.len(), 3);

        let stems: Vec<&str> = children.iter().map(|c| c.data().text.as_str()).collect();
        assert_eq!(stems, vec!["fix", "eigenvalu", "comput"]);
    }

    #[test]
    fn tokenize_with_compound() {
        let tree = tokenize("approx_lambda_2 is wrong");
        // "is" filtered. "wrong" -> stem "wrong".
        // "approx_lambda_2" -> compound tree.
        let children = match &tree {
            Prism::Fractal { children, .. } => children,
            _ => panic!("expected Fractal root"),
        };
        assert_eq!(children.len(), 2); // compound + "wrong"

        // First child should be a Fractal (compound)
        match &children[0] {
            Prism::Fractal { data, children: sub, .. } => {
                assert_eq!(data.text, "approx_lambda_2");
                assert_eq!(sub.len(), 3); // "approx" + "lambda" + "2"
            }
            _ => panic!("expected compound Fractal"),
        }
    }

    #[test]
    fn shared_token_same_oid() {
        let tree_a = tokenize("eigenvalue computation");
        let tree_b = tokenize("eigenvalue decomposition");

        let oid_a = match &tree_a {
            Prism::Fractal { children, .. } => children[0].data().content_oid(),
            _ => panic!("expected Fractal"),
        };
        let oid_b = match &tree_b {
            Prism::Fractal { children, .. } => children[0].data().content_oid(),
            _ => panic!("expected Fractal"),
        };

        // Both stem to "eigenvalu" -> same OID
        assert_eq!(oid_a, oid_b);
    }

    #[test]
    fn compound_shares_subtree_with_leaf() {
        let tree_compound = tokenize("approx_lambda_2");
        let tree_leaf = tokenize("lambda");

        // Extract "lambda" from compound's children
        let lambda_in_compound = match &tree_compound {
            Prism::Fractal { children, .. } => {
                match &children[0] {
                    Prism::Fractal { children: sub, .. } => {
                        // sub[1] should be "lambda" (stemmed)
                        sub[1].data().content_oid()
                    }
                    _ => panic!("expected compound"),
                }
            }
            _ => panic!("expected root"),
        };

        // Extract "lambda" from the leaf tree
        let lambda_direct = match &tree_leaf {
            Prism::Fractal { children, .. } => {
                children[0].data().content_oid()
            }
            _ => panic!("expected root"),
        };

        // Same OID — shared subtree
        assert_eq!(lambda_in_compound, lambda_direct);
    }

    #[test]
    fn stop_words_filtered() {
        let tree = tokenize("the is a an and or but");
        let children = match &tree {
            Prism::Fractal { children, .. } => children,
            _ => panic!("expected Fractal"),
        };
        assert_eq!(children.len(), 0);
    }

    #[test]
    fn camel_case_decomposed() {
        let tree = tokenize("SpectralIndex");
        let children = match &tree {
            Prism::Fractal { children, .. } => children,
            _ => panic!("expected Fractal"),
        };
        assert_eq!(children.len(), 1);
        match &children[0] {
            Prism::Fractal { data, children: sub, .. } => {
                assert_eq!(data.kind, TokenKind::Compound);
                assert_eq!(sub.len(), 2);
                assert_eq!(sub[0].data().text, "spectral");
                assert_eq!(sub[1].data().text, "index");
            }
            _ => panic!("expected compound Fractal"),
        }
    }

    // -- Task 5 pipeline edge cases --

    #[test]
    fn extract_compound_words_preserves_underscore_tokens() {
        let words = extract_compound_words("approx_lambda_2 is wrong");
        assert!(words.contains(&"approx_lambda_2".to_string()));
        assert!(words.contains(&"is".to_string()));
        assert!(words.contains(&"wrong".to_string()));
    }

    #[test]
    fn extract_compound_words_preserves_dot_tokens() {
        let words = extract_compound_words("self.eigenvalues are computed");
        assert!(words.contains(&"self.eigenvalues".to_string()));
    }

    // -- Task 6: Token tree utility tests --

    #[test]
    fn leaf_oids_collects_words() {
        let tree = tokenize("eigenvalue computation");
        let oids = leaf_oids(&tree);
        assert_eq!(oids.len(), 2);
    }

    #[test]
    fn all_oids_includes_compounds() {
        let tree = tokenize("approx_lambda_2");
        let all = all_oids(&tree);
        // Root compound + approx_lambda_2 compound + approx + lambda + 2
        // = 5 OIDs minimum
        assert!(all.len() >= 5, "expected >= 5 OIDs, got {}", all.len());
    }

    #[test]
    fn shared_oids_between_related_texts() {
        let a = tokenize("approx_lambda_2 is wrong");
        let b = tokenize("Fixed lambda computation");
        let shared = shared_oid_count(&a, &b);
        // Should share at least: "lambda" leaf
        assert!(shared >= 1, "expected >= 1 shared OIDs, got {}", shared);
    }

    #[test]
    fn no_shared_oids_unrelated_texts() {
        let a = tokenize("eigenvalue matrix");
        let b = tokenize("coffee breakfast");
        let shared = shared_oid_count(&a, &b);
        assert_eq!(shared, 0);
    }
}
