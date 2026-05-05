//! Compound decomposition — recursive splitting of compound tokens.
//!
//! Rules:
//! - Underscore splits: `approx_lambda_2` -> `[approx, lambda_2]`
//! - CamelCase splits: `SpectralIndex` -> `[Spectral, Index]`
//! - Dot splits: `self.eigenvalues` -> `[self, eigenvalues]`
//! - Recursive: `lambda_2` -> `[lambda, 2]`
//!
//! Each split level becomes a tree level. The tree structure IS the
//! compound decomposition.

/// A node in the compound decomposition tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompoundNode {
    /// The text at this level (full compound or leaf part).
    pub text: String,
    /// Children from decomposition. Empty = leaf.
    pub children: Vec<CompoundNode>,
}

impl CompoundNode {
    pub fn leaf(text: impl Into<String>) -> Self {
        CompoundNode {
            text: text.into(),
            children: vec![],
        }
    }

    pub fn branch(text: impl Into<String>, children: Vec<CompoundNode>) -> Self {
        CompoundNode {
            text: text.into(),
            children,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Decompose a token into a tree of compound parts.
///
/// Splits on underscores, CamelCase boundaries, and dots. Recursive:
/// each part is further decomposed until no more splits are possible.
pub fn decompose(token: &str) -> CompoundNode {
    // Try underscore split first
    let parts: Vec<&str> = token.split('_').collect();
    if parts.len() > 1 {
        let children: Vec<CompoundNode> = parts
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| decompose(p))
            .collect();
        if children.len() > 1 {
            return CompoundNode::branch(token, children);
        }
    }

    // Try dot split
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() > 1 {
        let children: Vec<CompoundNode> = parts
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| decompose(p))
            .collect();
        if children.len() > 1 {
            return CompoundNode::branch(token, children);
        }
    }

    // Try CamelCase split
    let camel_parts = split_camel_case(token);
    if camel_parts.len() > 1 {
        let children: Vec<CompoundNode> = camel_parts.iter().map(|p| decompose(p)).collect();
        return CompoundNode::branch(token, children);
    }

    // No splits — leaf
    CompoundNode::leaf(token)
}

/// Split a CamelCase string into parts.
/// "SpectralIndex" -> ["Spectral", "Index"]
/// "HTMLParser" -> ["HTML", "Parser"]
/// "simple" -> ["simple"]
fn split_camel_case(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];
        if i > 0 && c.is_uppercase() {
            let prev_lower = chars[i - 1].is_lowercase();
            let next_lower = chars.get(i + 1).map_or(false, |n| n.is_lowercase());
            // Split before uppercase that follows lowercase,
            // or before uppercase that precedes lowercase (for acronyms like HTML)
            if prev_lower || (next_lower && !current.is_empty()) {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
        }
        current.push(c);
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_word_is_leaf() {
        let node = decompose("eigenvalue");
        assert_eq!(node, CompoundNode::leaf("eigenvalue"));
    }

    #[test]
    fn underscore_split() {
        let node = decompose("lambda_2");
        assert_eq!(
            node,
            CompoundNode::branch(
                "lambda_2",
                vec![CompoundNode::leaf("lambda"), CompoundNode::leaf("2"),]
            )
        );
    }

    #[test]
    fn nested_underscore_split() {
        let node = decompose("approx_lambda_2");
        assert_eq!(
            node,
            CompoundNode::branch(
                "approx_lambda_2",
                vec![
                    CompoundNode::leaf("approx"),
                    CompoundNode::leaf("lambda"),
                    CompoundNode::leaf("2"),
                ]
            )
        );
    }

    #[test]
    fn camel_case_split() {
        let node = decompose("SpectralIndex");
        assert_eq!(
            node,
            CompoundNode::branch(
                "SpectralIndex",
                vec![CompoundNode::leaf("Spectral"), CompoundNode::leaf("Index"),]
            )
        );
    }

    #[test]
    fn dot_split() {
        let node = decompose("self.eigenvalues");
        assert_eq!(
            node,
            CompoundNode::branch(
                "self.eigenvalues",
                vec![
                    CompoundNode::leaf("self"),
                    CompoundNode::leaf("eigenvalues"),
                ]
            )
        );
    }

    #[test]
    fn single_char_leaf() {
        let node = decompose("x");
        assert_eq!(node, CompoundNode::leaf("x"));
    }

    #[test]
    fn camel_case_acronym() {
        let parts = split_camel_case("HTMLParser");
        assert_eq!(parts, vec!["HTML", "Parser"]);
    }

    #[test]
    fn no_camel_lowercase() {
        let parts = split_camel_case("simple");
        assert_eq!(parts, vec!["simple"]);
    }
}
