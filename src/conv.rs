use serde_json::Value;

use crate::tree::Tree;

/// A parsed `.conv` file — typed transformation pipeline.
///
/// A `.conv` file declares:
/// - `in Type` — the input tree type
/// - `template $name { fields }` — reusable extraction shapes
/// - `out name { structure }` — the output shape
///
/// The file's shape IS the output's shape.
pub struct Conv {
    _private: (),
}

/// A filesystem entry as tree data.
///
/// Branch = directory (has children, no content).
/// Leaf = file (has content, no children).
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub content: Option<String>,
}

/// Parse error for `.conv` files.
#[derive(Debug)]
pub struct ConvError(pub String);

impl std::fmt::Display for ConvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conv parse error: {}", self.0)
    }
}

impl std::error::Error for ConvError {}

impl Conv {
    /// Parse a `.conv` file from its source text.
    pub fn parse(_input: &str) -> Result<Self, ConvError> {
        todo!("parse .conv surface language")
    }

    /// Execute the pipeline against a tree, producing a JSON value.
    pub fn execute(&self, _tree: &Tree<Folder>) -> Value {
        todo!("execute .conv pipeline against tree")
    }
}

impl Folder {
    /// Build a `Tree<Folder>` from a filesystem path.
    ///
    /// Directories become branches, files become leaves.
    pub fn read_tree(_path: &str) -> Tree<Folder> {
        todo!("build Tree<Folder> from filesystem path")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn systemic_engineering_conv_produces_blog_index() {
        let conv_source = include_str!("../systemic.engineering.conv");
        let conv = Conv::parse(conv_source).expect("parses .conv file");

        let se_path = std::env::var("SYSTEMIC_ENGINEERING")
            .unwrap_or_else(|_| "/Users/alexwolf/dev/systemic.engineering".into());
        let tree = Folder::read_tree(&format!("{}/blog", se_path));
        let result = conv.execute(&tree);

        // Output shape matches .conv declaration
        let pieces = &result["blog"]["pieces"];

        // 1draft — draft pieces
        let drafts = pieces["draft"].as_array().expect("draft is array");
        assert!(!drafts.is_empty(), "should have draft pieces");

        // 3published — published pieces
        let published = pieces["published"]
            .as_array()
            .expect("published is array");
        assert!(!published.is_empty(), "should have published pieces");

        // 4archived — archived pieces
        let archived = pieces["archived"].as_array().expect("archived is array");
        assert!(!archived.is_empty(), "should have archived pieces");

        // Each entry has corpus fields: slug, excerpt, headlines
        for entry in published {
            assert!(entry["slug"].is_string(), "slug should be string");
            assert!(entry["headlines"].is_array(), "headlines should be array");
        }

        // Known piece: Consciousness (SLUG: written-by-ai-consciousness)
        let consciousness = published
            .iter()
            .find(|p| p["slug"] == "written-by-ai-consciousness")
            .expect("consciousness piece should exist in published");
        assert!(!consciousness["excerpt"].as_str().unwrap().is_empty());
        assert!(!consciousness["headlines"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}
