//! Emit — project AST back to `.mirror` syntax.
//!
//! The inverse of parse. `parse(emit(ast)) ≡ ast` for crystal keywords.
//!
//! ```text
//! fold-def leaf "input"                   → "fold input"
//! fold-def branch "observe" [param:"x"]   → "fold observe(x)"
//! traversal-def branch "t" [variant:"a", variant:"b"] → "traversal t = a | b"
//! ```

use crate::ast::AstNode;
use crate::prism::Prism;

/// Emit a single AST node as `.mirror` syntax.
///
/// Returns `None` if the node isn't a crystal keyword definition.
pub fn emit_node(node: &Prism<AstNode>) -> Option<String> {
    let data = node.data();
    let keyword = keyword_from_name(&data.name)?;

    match node.children().len() {
        0 => {
            // Leaf: `fold input`
            Some(format!("{} {}", keyword, data.value))
        }
        _ => {
            let children = node.children();
            let first_child = children[0].data();

            if first_child.name == "variant" {
                // Variant branch: `traversal type = a | b | c`
                let variants: Vec<&str> = children
                    .iter()
                    .filter(|c| c.data().name == "variant")
                    .map(|c| c.data().value.as_str())
                    .collect();
                Some(format!(
                    "{} {} = {}",
                    keyword,
                    data.value,
                    variants.join(" | ")
                ))
            } else if first_child.name == "param" {
                // Parameterized: `lens type(id)` or `fold observe(x, y)`
                let params: Vec<&str> = children
                    .iter()
                    .filter(|c| c.data().name == "param")
                    .map(|c| c.data().value.as_str())
                    .collect();
                Some(format!(
                    "{}{}({})",
                    keyword,
                    if data.value.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", data.value)
                    },
                    params.join(", ")
                ))
            } else {
                // Unknown children — emit as simple
                Some(format!("{} {}", keyword, data.value))
            }
        }
    }
}

/// Emit an entire AST tree as `.mirror` source.
///
/// Each top-level child becomes one line. Non-crystal nodes are skipped.
pub fn emit(tree: &Prism<AstNode>) -> String {
    let mut lines = Vec::new();
    for child in tree.children() {
        if let Some(line) = emit_node(child) {
            lines.push(line);
        }
    }
    let mut output = lines.join("\n");
    if !output.is_empty() {
        output.push('\n');
    }
    output
}

/// Extract the crystal keyword from a node name.
///
/// `"fold-def"` → `Some("fold")`, `"grammar"` → `None`
fn keyword_from_name(name: &str) -> Option<&str> {
    match name {
        "fold-def" => Some("fold"),
        "prism-def" => Some("prism"),
        "traversal-def" => Some("traversal"),
        "lens-def" => Some("lens"),
        "iso-def" => Some("iso"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{self, Span};
    use crate::domain::conversation::Kind;
    use crate::parse::Parse;
    use crate::Vector;

    fn span() -> Span {
        Span::new(0, 0)
    }

    #[test]
    fn emit_leaf() {
        let node = ast::ast_leaf(Kind::Decl, "fold-def", "input", span());
        assert_eq!(emit_node(&node), Some("fold input".to_string()));
    }

    #[test]
    fn emit_parameterized() {
        let params = vec![ast::ast_leaf(Kind::Atom, "param", "precision", span())];
        let node = ast::ast_branch(Kind::Decl, "prism-def", "eigenvalues", span(), params);
        assert_eq!(
            emit_node(&node),
            Some("prism eigenvalues(precision)".to_string())
        );
    }

    #[test]
    fn emit_multi_param() {
        let params = vec![
            ast::ast_leaf(Kind::Atom, "param", "x", span()),
            ast::ast_leaf(Kind::Atom, "param", "y", span()),
        ];
        let node = ast::ast_branch(Kind::Decl, "lens-def", "merge", span(), params);
        assert_eq!(emit_node(&node), Some("lens merge(x, y)".to_string()));
    }

    #[test]
    fn emit_variants() {
        let variants = vec![
            ast::ast_leaf(Kind::Atom, "variant", "red", span()),
            ast::ast_leaf(Kind::Atom, "variant", "blue", span()),
            ast::ast_leaf(Kind::Atom, "variant", "green", span()),
        ];
        let node = ast::ast_branch(Kind::Decl, "traversal-def", "color", span(), variants);
        assert_eq!(
            emit_node(&node),
            Some("traversal color = red | blue | green".to_string())
        );
    }

    #[test]
    fn emit_iso_leaf() {
        let node = ast::ast_leaf(Kind::Decl, "iso-def", "convergence", span());
        assert_eq!(emit_node(&node), Some("iso convergence".to_string()));
    }

    #[test]
    fn emit_non_crystal_returns_none() {
        let node = ast::ast_leaf(Kind::Decl, "grammar", "@test", span());
        assert_eq!(emit_node(&node), None);
    }

    #[test]
    fn round_trip_simple() {
        let source = "fold input\nprism eigenvalues(precision)\ntraversal color = red | blue | green\nlens transform(projection)\niso convergence\n";
        let ast = Parse.trace(source.to_string()).into_result().unwrap();
        let emitted = emit(&ast);
        assert_eq!(emitted, source);
    }

    #[test]
    fn round_trip_complex() {
        let source = "traversal state = declared | actual | observed\nfold observe(state)\nprism project(mode)\nlens merge(redundant)\niso settle\n";
        let ast = Parse.trace(source.to_string()).into_result().unwrap();
        let emitted = emit(&ast);
        assert_eq!(emitted, source);
    }

    #[test]
    fn round_trip_resolved_fixtures() {
        let resolved_dir =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/resolved");
        if !resolved_dir.exists() {
            return;
        }
        for entry in std::fs::read_dir(&resolved_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|x| x.to_str()) != Some("mirror") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            let ast = Parse
                .trace(source.clone())
                .into_result()
                .unwrap_or_else(|e| panic!("parse {}: {}", path.display(), e));
            let emitted = emit(&ast);
            assert_eq!(
                emitted,
                source,
                "resolved file {} does not round-trip",
                path.display()
            );
        }
    }
}
