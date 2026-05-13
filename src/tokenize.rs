//! @io/tokenize — the door from text to AST.
//!
//! Contract:
//! - in: source text (bytes) + grammar keyword mappings
//! - out: MirrorAST (the 7 variants)
//! - bound: O(n) in source length. Single pass. No recursion. Memory proportional to input.
//!
//! The tokenizer is NOT a parser. It understands: keywords, names, braces. That's enough.

use std::collections::HashMap;

use crate::mirror_ast::{
    FocusNode, Identifier, MirrorAST, ModuleNode, ProjectNode, RefractNode, SplitNode, ZoomNode,
};

/// Which AST variant a keyword maps to.
#[derive(Clone, Debug, PartialEq)]
pub enum AstKind {
    Focus,
    Split,
    Zoom,
    Project,
    Refract,
}

/// Grammar: keyword -> AstKind mappings extracted from a .mirror file.
#[derive(Clone, Debug)]
pub struct Grammar {
    /// keyword (e.g. "fn") -> AstKind (e.g. Zoom)
    pub mappings: HashMap<String, AstKind>,
}

/// Load a grammar from a .mirror file.
///
/// Scans for lines like `zoom fn`, `split struct`, etc. inside a `grammar @... { }` block.
/// These are keyword->operation mappings.
pub fn load_grammar(path: &str) -> Result<Grammar, String> {
    let _source = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read grammar {}: {}", path, e))?;
    todo!("load_grammar not yet implemented")
}

/// Tokenize source text using grammar keyword mappings.
///
/// Single pass, O(n). Returns a Module wrapping all top-level items.
pub fn tokenize(_source: &str, _grammar: &Grammar) -> MirrorAST {
    todo!("tokenize not yet implemented")
}

// ---------------------------------------------------------------------------
// Tests — RED: these must fail until implementation
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_fn_produces_zoom() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("fn hello() { }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Zoom(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_struct_produces_split() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("struct Point { x: f64 }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Split(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_enum_produces_split() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("enum Color { Red, Blue }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Split(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_impl_produces_focus() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("impl Point { fn x(&self) -> f64 { self.x } }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Focus(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_trait_produces_refract() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("trait Display { fn fmt(&self); }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Refract(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_use_produces_project() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("use std::collections::HashMap;", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Project(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_mod_produces_focus() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("mod tests { fn test_one() {} }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Focus(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_multiple_items() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = "struct A {}\nfn b() {}\nenum C {}";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 3);
                assert!(matches!(m.children[0], MirrorAST::Split(_)));
                assert!(matches!(m.children[1], MirrorAST::Zoom(_)));
                assert!(matches!(m.children[2], MirrorAST::Split(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_names_extracted() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("fn hello() { }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children[0].name(), "hello");
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_nested_braces() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = "fn outer() { if true { nested() } }";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert_eq!(m.children[0].name(), "outer");
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_impl_has_children() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = "impl Foo { fn bar() {} fn baz() {} }";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                match &m.children[0] {
                    MirrorAST::Focus(f) => {
                        assert_eq!(f.name.as_str(), "Foo");
                        assert_eq!(f.children.len(), 2);
                        assert!(matches!(f.children[0], MirrorAST::Zoom(_)));
                        assert!(matches!(f.children[1], MirrorAST::Zoom(_)));
                    }
                    _ => panic!("expected Focus for impl"),
                }
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_bounded_memory() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = "fn f() {}\n".repeat(100_000);
        let ast = tokenize(&source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 100_000);
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_real_file() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = std::fs::read_to_string("src/mirror_ast.rs").unwrap();
        let ast = tokenize(&source, &grammar);
        // Must not OOM. Must produce children.
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty());
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_grammar_driven() {
        // Same source through different grammars produces different AST
        let rust_grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let kintsugi_grammar = load_grammar("boot/std/kintsugi.mirror").unwrap();
        let source = "fn hello() { }";
        let rust_ast = tokenize(source, &rust_grammar);
        let kintsugi_ast = tokenize(source, &kintsugi_grammar);
        // Rust grammar recognizes fn. Kintsugi grammar doesn't.
        let rust_count = match &rust_ast {
            MirrorAST::Module(m) => m.children.len(),
            _ => 0,
        };
        let kintsugi_count = match &kintsugi_ast {
            MirrorAST::Module(m) => m.children.len(),
            _ => 0,
        };
        assert!(rust_count > kintsugi_count);
    }

    #[test]
    fn tokenize_empty_source() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert!(m.children.is_empty());
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_content_oid_deterministic() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = "fn hello() { }";
        let ast1 = tokenize(source, &grammar);
        let ast2 = tokenize(source, &grammar);
        assert_eq!(ast1.content_oid(), ast2.content_oid());
    }

    #[test]
    fn tokenize_pub_fn_produces_zoom() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let ast = tokenize("pub fn hello() { }", &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(m.children[0], MirrorAST::Zoom(_)));
                assert_eq!(m.children[0].name(), "hello");
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_use_no_braces() {
        // `use` statements end at semicolon, not braces
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        let source = "use std::io;\nfn main() {}";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 2);
                assert!(matches!(m.children[0], MirrorAST::Project(_)));
                assert!(matches!(m.children[1], MirrorAST::Zoom(_)));
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn load_grammar_extracts_mappings() {
        let grammar = load_grammar("boot/std/code/rust.mirror").unwrap();
        assert_eq!(grammar.mappings.get("fn"), Some(&AstKind::Zoom));
        assert_eq!(grammar.mappings.get("struct"), Some(&AstKind::Split));
        assert_eq!(grammar.mappings.get("enum"), Some(&AstKind::Split));
        assert_eq!(grammar.mappings.get("impl"), Some(&AstKind::Focus));
        assert_eq!(grammar.mappings.get("mod"), Some(&AstKind::Focus));
        assert_eq!(grammar.mappings.get("use"), Some(&AstKind::Project));
        assert_eq!(grammar.mappings.get("trait"), Some(&AstKind::Refract));
    }

    #[test]
    fn load_grammar_kintsugi_no_keyword_mappings() {
        let grammar = load_grammar("boot/std/kintsugi.mirror").unwrap();
        // Kintsugi grammar has `collapse(ast, ast) -> imperfect { \ }`
        // but no keyword-to-operation lines like `zoom fn`
        assert!(grammar.mappings.is_empty());
    }
}
