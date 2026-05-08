//! evaluate — grammar-parameterized evaluation of source text.
//!
//! `evaluate(grammar, source)` applies a grammar's keyword-to-operation
//! mappings to tokenize and classify source text into MirrorAST nodes.
//!
//! The grammar declares mappings like:
//!   zoom fn       (fn keyword maps to Zoom operation)
//!   split struct  (struct keyword maps to Split operation)
//!   focus impl    (impl keyword maps to Focus operation)
//!
//! evaluate() reads these mappings, then tokenizes the source and builds
//! MirrorAST nodes according to the grammar's rules.

use std::collections::HashMap;

use crate::declaration::{MirrorFragment, MirrorFragmentExt};
use crate::mirror_ast::{
    FocusNode, Identifier, MirrorAST, ModuleNode, ProjectNode, RefractNode, SplitNode, ZoomNode,
};

// ---------------------------------------------------------------------------
// Grammar rule extraction
// ---------------------------------------------------------------------------

/// A keyword-to-operation mapping extracted from a grammar.
#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Focus,
    Project,
    Split,
    Zoom,
    Refract,
}

/// Extract keyword->operation mappings from a compiled grammar fragment.
///
/// Walks the grammar's children looking for operation declarations where
/// the declaration name is the keyword. For example:
///   zoom fn    -> maps "fn" to Zoom
///   split struct -> maps "struct" to Split
fn extract_rules(grammar: &MirrorFragment) -> HashMap<String, Operation> {
    let mut rules = HashMap::new();

    fn walk(frag: &MirrorFragment, rules: &mut HashMap<String, Operation>) {
        let ast = frag.mirror_ast();
        match ast.kind_name() {
            "zoom" => {
                let name = ast.name();
                if !name.is_empty() && !name.starts_with('@') {
                    // Check if this is a keyword mapping (no params, no children)
                    // vs an action declaration (has params or children)
                    if ast.params_as_strings().is_empty() && frag.mirror_children().is_empty() {
                        // This could be a mapping like `zoom fn` where the first
                        // param/target is the keyword. But the parser puts the word
                        // after `zoom` as the name. So `zoom fn` -> name="fn".
                        rules.insert(name.to_string(), Operation::Zoom);
                    }
                }
            }
            "split" => {
                let name = ast.name();
                if !name.is_empty()
                    && !name.starts_with('@')
                    && ast.variants_as_strings().is_empty()
                    && frag.mirror_children().is_empty()
                {
                    rules.insert(name.to_string(), Operation::Split);
                }
            }
            "focus" => {
                let name = ast.name();
                if !name.is_empty()
                    && !name.starts_with('@')
                    && frag.mirror_children().is_empty()
                {
                    rules.insert(name.to_string(), Operation::Focus);
                }
            }
            "project" => {
                let name = ast.name();
                if !name.is_empty()
                    && !name.starts_with('@')
                    && frag.mirror_children().is_empty()
                {
                    rules.insert(name.to_string(), Operation::Project);
                }
            }
            "refract" => {
                let name = ast.name();
                if !name.is_empty()
                    && !name.starts_with('@')
                    && ast.params_as_strings().is_empty()
                    && frag.mirror_children().is_empty()
                {
                    rules.insert(name.to_string(), Operation::Refract);
                }
            }
            _ => {}
        }

        // Recurse into children (e.g. grammar block contains the mappings)
        for child in frag.mirror_children() {
            walk(child, rules);
        }
    }

    walk(grammar, &mut rules);
    rules
}

// ---------------------------------------------------------------------------
// Source tokenizer (lightweight, language-agnostic)
// ---------------------------------------------------------------------------

/// A token in the source being evaluated.
#[derive(Clone, Debug, PartialEq)]
enum SrcTok {
    /// An identifier or keyword
    Word(String),
    /// Opening brace
    LBrace,
    /// Closing brace
    RBrace,
    /// Opening paren
    LParen,
    /// Closing paren
    RParen,
    /// Semicolon or comma
    Punct(char),
    /// Everything else (operators, literals, etc.)
    Other(String),
}

/// Tokenize source code into a sequence of tokens.
/// This is a lightweight tokenizer — not language-specific.
fn tokenize_source(source: &str) -> Vec<SrcTok> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                i += 1;
            }
            '{' => {
                tokens.push(SrcTok::LBrace);
                i += 1;
            }
            '}' => {
                tokens.push(SrcTok::RBrace);
                i += 1;
            }
            '(' => {
                tokens.push(SrcTok::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(SrcTok::RParen);
                i += 1;
            }
            ';' | ',' => {
                tokens.push(SrcTok::Punct(c));
                i += 1;
            }
            '/' if i + 1 < bytes.len() && bytes[i + 1] == b'/' => {
                // Line comment — skip to end of line
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            '/' if i + 1 < bytes.len() && bytes[i + 1] == b'*' => {
                // Block comment — skip to */
                i += 2;
                while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    i += 1;
                }
                if i + 1 < bytes.len() {
                    i += 2;
                }
            }
            '"' => {
                // String literal — skip to closing quote
                i += 1;
                let start = i;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                        i += 1; // skip escaped char
                    }
                    i += 1;
                }
                let s = String::from_utf8_lossy(&bytes[start..i]).to_string();
                tokens.push(SrcTok::Other(format!("\"{}\"", s)));
                if i < bytes.len() {
                    i += 1; // skip closing quote
                }
            }
            _ if c.is_ascii_alphabetic() || c == '_' || c == '#' => {
                let start = i;
                while i < bytes.len() {
                    let cc = bytes[i] as char;
                    if cc.is_ascii_alphanumeric() || cc == '_' || cc == '!' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(SrcTok::Word(source[start..i].to_string()));
            }
            _ => {
                // Operators and other characters
                let start = i;
                i += 1;
                tokens.push(SrcTok::Other(source[start..i].to_string()));
            }
        }
    }

    tokens
}

// ---------------------------------------------------------------------------
// evaluate() — the main function
// ---------------------------------------------------------------------------

/// Apply a grammar's keyword-to-operation mappings to source text.
///
/// Returns a MirrorAST::Module containing the classified items.
///
/// # Example
///
/// ```ignore
/// let grammar = load_grammar("@code/rust");
/// let ast = evaluate(&grammar, "fn main() { }");
/// // ast is Module containing a Zoom node named "main"
/// ```
pub fn evaluate(grammar: &MirrorFragment, source: &str) -> MirrorAST {
    let rules = extract_rules(grammar);
    let tokens = tokenize_source(source);
    let mut items: Vec<MirrorAST> = Vec::new();
    let mut cursor = 0;

    while cursor < tokens.len() {
        // Skip attribute annotations like #[derive(...)]
        if matches!(tokens.get(cursor), Some(SrcTok::Word(w)) if w.starts_with('#')) {
            skip_attribute(&tokens, &mut cursor);
            continue;
        }

        // Skip visibility modifiers
        if matches!(tokens.get(cursor), Some(SrcTok::Word(w)) if w == "pub") {
            cursor += 1;
            // Handle `pub(crate)` etc.
            if matches!(tokens.get(cursor), Some(SrcTok::LParen)) {
                skip_parens(&tokens, &mut cursor);
            }
            continue;
        }

        // Check if current token is a mapped keyword
        if let Some(SrcTok::Word(keyword)) = tokens.get(cursor) {
            if let Some(op) = rules.get(keyword.as_str()) {
                cursor += 1;

                // Extract name (the identifier after the keyword)
                let name = extract_name(&tokens, &mut cursor);

                // Extract params if present
                let params = if matches!(tokens.get(cursor), Some(SrcTok::LParen)) {
                    extract_paren_content(&tokens, &mut cursor)
                } else {
                    String::new()
                };

                // Skip to body or semicolon
                let body = skip_to_body_or_end(&tokens, &mut cursor);

                // Build the appropriate MirrorAST node
                let node = match op {
                    Operation::Zoom => MirrorAST::Zoom(ZoomNode {
                        name: Identifier::new(&name),
                        target: None,
                        params: parse_rust_params(&params),
                        grammar_ref: None,
                        children: vec![],
                        body: if body.is_empty() {
                            None
                        } else {
                            Some(vec![])
                        },
                    }),
                    Operation::Split => {
                        // For struct/enum: extract variants or fields from body
                        let (variants, fields) = parse_split_body(&body);
                        let body_type = if !fields.is_empty() {
                            Some(crate::mirror_ast::TypeBody::Struct(fields))
                        } else if !variants.is_empty() {
                            None // variants are in the variants field
                        } else {
                            Some(crate::mirror_ast::TypeBody::Unit)
                        };
                        MirrorAST::Split(SplitNode {
                            name: Identifier::new(&name),
                            variants,
                            params: vec![],
                            body: body_type,
                            children: vec![],
                        })
                    }
                    Operation::Focus => MirrorAST::Focus(FocusNode {
                        name: Identifier::new(&name),
                        target: None,
                        children: vec![],
                    }),
                    Operation::Project => MirrorAST::Project(ProjectNode {
                        name: Identifier::new(&name),
                        target: None,
                        children: vec![],
                    }),
                    Operation::Refract => MirrorAST::Refract(RefractNode {
                        name: Identifier::new(&name),
                        target: None,
                        params: vec![],
                        children: vec![],
                    }),
                };

                items.push(node);
                continue;
            }
        }

        // Not a mapped keyword — skip this token
        cursor += 1;
    }

    MirrorAST::Module(ModuleNode {
        name: Identifier::new(""),
        children: items,
    })
}

// ---------------------------------------------------------------------------
// Helpers for evaluate()
// ---------------------------------------------------------------------------

/// Skip an attribute like `#[derive(Clone)]` or `#[cfg(test)]`
fn skip_attribute(tokens: &[SrcTok], cursor: &mut usize) {
    // Skip #
    *cursor += 1;
    // Skip [ ... ]
    if matches!(tokens.get(*cursor), Some(SrcTok::Other(s)) if s == "[") {
        *cursor += 1;
        let mut depth = 1;
        while *cursor < tokens.len() && depth > 0 {
            match tokens.get(*cursor) {
                Some(SrcTok::Other(s)) if s == "[" => depth += 1,
                Some(SrcTok::Other(s)) if s == "]" => depth -= 1,
                _ => {}
            }
            *cursor += 1;
        }
    }
}

/// Skip balanced parentheses.
fn skip_parens(tokens: &[SrcTok], cursor: &mut usize) {
    if !matches!(tokens.get(*cursor), Some(SrcTok::LParen)) {
        return;
    }
    *cursor += 1;
    let mut depth = 1;
    while *cursor < tokens.len() && depth > 0 {
        match tokens.get(*cursor) {
            Some(SrcTok::LParen) => depth += 1,
            Some(SrcTok::RParen) => depth -= 1,
            _ => {}
        }
        *cursor += 1;
    }
}

/// Extract an identifier name from the current position.
fn extract_name(tokens: &[SrcTok], cursor: &mut usize) -> String {
    // Skip generic params like <T> or lifetime params like <'a>
    // but first get the name
    match tokens.get(*cursor) {
        Some(SrcTok::Word(w)) => {
            let name = w.clone();
            *cursor += 1;
            name
        }
        _ => String::new(),
    }
}

/// Extract the content between parentheses as a string.
fn extract_paren_content(tokens: &[SrcTok], cursor: &mut usize) -> String {
    if !matches!(tokens.get(*cursor), Some(SrcTok::LParen)) {
        return String::new();
    }
    *cursor += 1;
    let mut content = String::new();
    let mut depth = 1;
    while *cursor < tokens.len() && depth > 0 {
        match tokens.get(*cursor) {
            Some(SrcTok::LParen) => {
                depth += 1;
                content.push('(');
            }
            Some(SrcTok::RParen) => {
                depth -= 1;
                if depth > 0 {
                    content.push(')');
                }
            }
            Some(SrcTok::Word(w)) => {
                if !content.is_empty() && !content.ends_with('(') {
                    content.push(' ');
                }
                content.push_str(w);
            }
            Some(SrcTok::Punct(c)) => content.push(*c),
            Some(SrcTok::Other(s)) => content.push_str(s),
            Some(SrcTok::LBrace) => content.push('{'),
            Some(SrcTok::RBrace) => content.push('}'),
            None => break,
        }
        *cursor += 1;
    }
    content
}

/// Skip tokens until we find a brace body or a semicolon.
/// Returns the body content if braces found, empty string otherwise.
fn skip_to_body_or_end(tokens: &[SrcTok], cursor: &mut usize) -> String {
    // Skip generic params, where clauses, return types, etc.
    while *cursor < tokens.len() {
        match tokens.get(*cursor) {
            Some(SrcTok::LBrace) => {
                return extract_brace_content(tokens, cursor);
            }
            Some(SrcTok::Punct(';')) => {
                *cursor += 1;
                return String::new();
            }
            _ => {
                *cursor += 1;
            }
        }
    }
    String::new()
}

/// Extract content between braces (balanced).
fn extract_brace_content(tokens: &[SrcTok], cursor: &mut usize) -> String {
    if !matches!(tokens.get(*cursor), Some(SrcTok::LBrace)) {
        return String::new();
    }
    *cursor += 1;
    let mut content = String::new();
    let mut depth = 1;
    while *cursor < tokens.len() && depth > 0 {
        match tokens.get(*cursor) {
            Some(SrcTok::LBrace) => {
                depth += 1;
                content.push('{');
            }
            Some(SrcTok::RBrace) => {
                depth -= 1;
                if depth > 0 {
                    content.push('}');
                }
            }
            Some(SrcTok::Word(w)) => {
                if !content.is_empty()
                    && !content.ends_with('{')
                    && !content.ends_with('\n')
                {
                    content.push(' ');
                }
                content.push_str(w);
            }
            Some(SrcTok::Punct(c)) => content.push(*c),
            Some(SrcTok::Other(s)) => content.push_str(s),
            Some(SrcTok::LParen) => content.push('('),
            Some(SrcTok::RParen) => content.push(')'),
            None => break,
        }
        *cursor += 1;
    }
    content
}

/// Parse Rust-style function parameters into Fields.
fn parse_rust_params(params_str: &str) -> Vec<crate::mirror_ast::Field> {
    if params_str.is_empty() {
        return vec![];
    }
    // Split on commas (respecting nested parens/generics)
    let parts = split_params(params_str);
    parts
        .iter()
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() || p == "self" || p == "&self" || p == "&mut self" {
                return None;
            }
            // Look for `name: type` pattern
            if let Some(colon_pos) = p.find(':') {
                let name = p[..colon_pos].trim();
                let typ = p[colon_pos + 1..].trim();
                Some(crate::mirror_ast::Field {
                    name: Identifier::new(name),
                    type_ref: Identifier::new(typ),
                })
            } else {
                Some(crate::mirror_ast::Field {
                    name: Identifier::new(p),
                    type_ref: Identifier::new("_"),
                })
            }
        })
        .collect()
}

/// Split a parameter string on commas, respecting nested brackets.
fn split_params(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '(' | '<' | '[' => {
                depth += 1;
                current.push(c);
            }
            ')' | '>' | ']' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                parts.push(current.clone());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Parse struct fields or enum variants from a brace body.
fn parse_split_body(body: &str) -> (Vec<Identifier>, Vec<crate::mirror_ast::Field>) {
    if body.is_empty() {
        return (vec![], vec![]);
    }

    // Check if this looks like struct fields (has `:` separator) or enum variants
    let parts = split_params(body);
    let has_colons = parts.iter().any(|p| p.contains(':'));

    if has_colons {
        // Struct fields
        let fields = parts
            .iter()
            .filter_map(|p| {
                let p = p.trim();
                if p.is_empty() {
                    return None;
                }
                // Skip pub modifier
                let p = if p.starts_with("pub") {
                    p.trim_start_matches("pub").trim()
                } else {
                    p
                };
                if let Some(colon_pos) = p.find(':') {
                    let name = p[..colon_pos].trim();
                    let typ = p[colon_pos + 1..].trim();
                    Some(crate::mirror_ast::Field {
                        name: Identifier::new(name),
                        type_ref: Identifier::new(typ),
                    })
                } else {
                    None
                }
            })
            .collect();
        (vec![], fields)
    } else {
        // Enum variants
        let variants = parts
            .iter()
            .filter_map(|p| {
                let p = p.trim();
                if p.is_empty() {
                    return None;
                }
                // Take just the variant name (before any parens or braces)
                let name = p
                    .split(['(', '{', ' '])
                    .next()
                    .unwrap_or(p)
                    .trim();
                if name.is_empty() {
                    None
                } else {
                    Some(Identifier::new(name))
                }
            })
            .collect();
        (variants, vec![])
    }
}

// ---------------------------------------------------------------------------
// Grammar loading helper
// ---------------------------------------------------------------------------

/// Load a grammar by name from the boot directory.
///
/// Looks for the grammar file at `boot/std/code/<name>.mirror` relative to
/// the crate manifest directory.
pub fn load_grammar(grammar_ref: &str) -> Option<MirrorFragment> {
    let path = grammar_ref.trim_start_matches('@');
    let file_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std")
        .join(format!("{}.mirror", path));

    if !file_path.exists() {
        return None;
    }

    let source = std::fs::read_to_string(&file_path).ok()?;
    let result = crate::mirror_runtime::parse_form(&source);
    result.ok()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Rule extraction
    // -----------------------------------------------------------------------

    #[test]
    fn extract_rules_from_rust_grammar() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let rules = extract_rules(&grammar);

        assert_eq!(rules.get("fn"), Some(&Operation::Zoom), "fn should map to Zoom");
        assert_eq!(rules.get("struct"), Some(&Operation::Split), "struct should map to Split");
        assert_eq!(rules.get("enum"), Some(&Operation::Split), "enum should map to Split");
        assert_eq!(rules.get("impl"), Some(&Operation::Focus), "impl should map to Focus");
        assert_eq!(rules.get("use"), Some(&Operation::Project), "use should map to Project");
        assert_eq!(rules.get("trait"), Some(&Operation::Refract), "trait should map to Refract");
    }

    // -----------------------------------------------------------------------
    // evaluate() — basic Rust constructs
    // -----------------------------------------------------------------------

    #[test]
    fn evaluate_rust_function() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "fn main() { println!(\"hello\"); }");
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1, "should have one item");
                assert!(
                    matches!(&m.children[0], MirrorAST::Zoom(_)),
                    "fn should produce Zoom, got {:?}",
                    m.children[0].kind_name()
                );
                assert_eq!(m.children[0].name(), "main");
            }
            _ => panic!("evaluate should return Module, got {:?}", ast.kind_name()),
        }
    }

    #[test]
    fn evaluate_rust_struct() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "struct Point { x: f64, y: f64 }");
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1, "should have one item");
                assert!(
                    matches!(&m.children[0], MirrorAST::Split(_)),
                    "struct should produce Split, got {:?}",
                    m.children[0].kind_name()
                );
                assert_eq!(m.children[0].name(), "Point");
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_rust_enum() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "enum Color { Red, Green, Blue }");
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(&m.children[0], MirrorAST::Split(_)));
                assert_eq!(m.children[0].name(), "Color");
                if let MirrorAST::Split(s) = &m.children[0] {
                    assert_eq!(s.variants.len(), 3, "should have 3 variants");
                }
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_rust_impl() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "impl Point { fn new() -> Self { Point { x: 0.0, y: 0.0 } } }");
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty(), "should have items");
                assert!(
                    matches!(&m.children[0], MirrorAST::Focus(_)),
                    "impl should produce Focus, got {:?}",
                    m.children[0].kind_name()
                );
                assert_eq!(m.children[0].name(), "Point");
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_rust_use() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "use std::collections::HashMap;");
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 1);
                assert!(matches!(&m.children[0], MirrorAST::Project(_)));
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_rust_trait() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "trait Display { fn fmt(&self) -> String; }");
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty());
                assert!(
                    matches!(&m.children[0], MirrorAST::Refract(_)),
                    "trait should produce Refract, got {:?}",
                    m.children[0].kind_name()
                );
                assert_eq!(m.children[0].name(), "Display");
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_rust_multiple_items() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let source = r#"
            use std::fmt;

            struct Point {
                x: f64,
                y: f64,
            }

            fn distance(a: Point, b: Point) -> f64 {
                ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
            }
        "#;
        let ast = evaluate(&grammar, source);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 3, "should have 3 items (use, struct, fn)");
                assert!(matches!(&m.children[0], MirrorAST::Project(_)));
                assert!(matches!(&m.children[1], MirrorAST::Split(_)));
                assert!(matches!(&m.children[2], MirrorAST::Zoom(_)));
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_rust_pub_items() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "pub fn hello() { } pub struct World;");
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.children.len(), 2, "should have 2 items");
                assert!(matches!(&m.children[0], MirrorAST::Zoom(_)));
                assert!(matches!(&m.children[1], MirrorAST::Split(_)));
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    #[test]
    fn evaluate_empty_source() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let ast = evaluate(&grammar, "");
        match &ast {
            MirrorAST::Module(m) => {
                assert!(m.children.is_empty(), "empty source should produce empty module");
            }
            _ => panic!("evaluate should return Module"),
        }
    }

    // -----------------------------------------------------------------------
    // evaluate() matches code_rust.rs output structure
    // -----------------------------------------------------------------------

    #[test]
    fn evaluate_matches_code_rust_item_count() {
        let grammar = load_grammar("@code/rust").expect("@code/rust grammar must exist");
        let source = r#"
            use std::fmt;
            struct Point { x: f64, y: f64 }
            fn main() { println!("hello"); }
            impl Point { fn distance(&self) -> f64 { 0.0 } }
            trait Drawable { fn draw(&self); }
            enum Shape { Circle, Square }
        "#;

        let via_evaluate = evaluate(&grammar, source);

        // code_rust.rs would produce 6 items from this source
        match &via_evaluate {
            MirrorAST::Module(m) => {
                assert_eq!(
                    m.children.len(),
                    6,
                    "evaluate should produce 6 items, got {}",
                    m.children.len()
                );
            }
            _ => panic!("evaluate should return Module"),
        }
    }
}
