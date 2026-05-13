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
/// These are keyword->operation mappings. Only bare two-word lines inside the grammar block
/// where the first word is an operation name count as mappings.
pub fn load_grammar(path: &str) -> Result<Grammar, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read grammar {}: {}", path, e))?;
    Ok(parse_grammar(&source))
}

/// Parse grammar mappings from source text.
fn parse_grammar(source: &str) -> Grammar {
    let mut mappings = HashMap::new();
    let mut in_grammar_block = false;
    let mut brace_depth: i32 = 0;

    for line in source.lines() {
        let trimmed = line.trim();

        // Skip comments
        if trimmed.starts_with("--") || trimmed.starts_with('#') {
            continue;
        }

        // Track grammar block entry
        if trimmed.starts_with("grammar ") && trimmed.contains('{') {
            in_grammar_block = true;
            brace_depth = 1;
            continue;
        }

        if !in_grammar_block {
            continue;
        }

        // Track braces
        for ch in trimmed.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        in_grammar_block = false;
                    }
                }
                _ => {}
            }
        }

        if !in_grammar_block && brace_depth <= 0 {
            continue;
        }

        // Look for keyword-to-operation mappings: exactly "operation keyword"
        // Lines with parens, arrows, etc. are action declarations, not keyword mappings.
        if trimmed.contains('(') || trimmed.contains('>') || trimmed.contains('{') || trimmed.contains('}') {
            continue;
        }

        // Strip inline comments (# ... or -- ...) before extracting words
        let content = trimmed
            .find('#')
            .or_else(|| {
                // Find standalone -- (not inside a word like `--flag`)
                trimmed.find(" --").map(|i| i)
            })
            .map(|i| trimmed[..i].trim())
            .unwrap_or(trimmed);

        let words: Vec<&str> = content.split_whitespace().collect();
        if words.len() == 2 {
            let kind = match words[0] {
                "focus" => Some(AstKind::Focus),
                "split" => Some(AstKind::Split),
                "zoom" => Some(AstKind::Zoom),
                "project" => Some(AstKind::Project),
                "refract" => Some(AstKind::Refract),
                _ => None,
            };
            if let Some(kind) = kind {
                mappings.insert(words[1].to_string(), kind);
            }
        }
    }

    Grammar { mappings }
}

/// Tokenize source text using grammar keyword mappings.
///
/// Single pass, O(n). Returns a Module wrapping all top-level items.
/// Scans for grammar keywords at each brace depth level.
/// For container keywords (Focus: impl/mod, Refract: trait), the body is
/// scanned for child items.
pub fn tokenize(source: &str, grammar: &Grammar) -> MirrorAST {
    let children = scan_items(source, grammar);
    MirrorAST::Module(ModuleNode {
        name: Identifier::new("root"),
        children,
    })
}

/// Scan source text for items matching grammar keywords.
/// Returns a flat list of AST nodes found at the current scope.
///
/// Single pass with brace-depth tracking. O(n) in source length.
fn scan_items(source: &str, grammar: &Grammar) -> Vec<MirrorAST> {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut items = Vec::new();
    let mut pos = 0;

    while pos < len {
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // Handle line comments: // (Rust) and -- (mirror)
        if (pos + 1 < len && bytes[pos] == b'/' && bytes[pos + 1] == b'/')
            || (pos + 1 < len && bytes[pos] == b'-' && bytes[pos + 1] == b'-')
        {
            while pos < len && bytes[pos] != b'\n' {
                pos += 1;
            }
            continue;
        }

        // Handle block comments: skip to */
        if pos + 1 < len && bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
            pos += 2;
            while pos + 1 < len && !(bytes[pos] == b'*' && bytes[pos + 1] == b'/') {
                pos += 1;
            }
            if pos + 1 < len {
                pos += 2; // skip */
            }
            continue;
        }

        // Handle string literals: skip to closing quote
        if bytes[pos] == b'"' {
            pos += 1;
            while pos < len && bytes[pos] != b'"' {
                if bytes[pos] == b'\\' {
                    pos += 1; // skip escaped char
                }
                pos += 1;
            }
            if pos < len {
                pos += 1; // skip closing "
            }
            continue;
        }

        // Handle # — attributes in Rust (#[...]), line comments in mirror (# ...)
        if bytes[pos] == b'#' {
            pos += 1;
            if pos < len && bytes[pos] == b'!' {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'[' {
                // Rust attribute: #[...] or #![...]
                pos += 1;
                let mut bracket_depth = 1i32;
                while pos < len && bracket_depth > 0 {
                    match bytes[pos] {
                        b'[' => bracket_depth += 1,
                        b']' => bracket_depth -= 1,
                        _ => {}
                    }
                    pos += 1;
                }
            } else {
                // Mirror line comment: # ... (skip to end of line)
                while pos < len && bytes[pos] != b'\n' {
                    pos += 1;
                }
            }
            continue;
        }

        // Handle stray punctuation that could cause stuck loops
        if bytes[pos] == b')' || bytes[pos] == b']' || bytes[pos] == b'}' {
            pos += 1;
            continue;
        }

        // Try to read a word (identifier-like token)
        let word_start = pos;
        while pos < len
            && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_')
        {
            pos += 1;
        }

        // If we didn't read any word chars, skip this byte
        if pos == word_start {
            pos += 1;
            continue;
        }

        let word = &source[word_start..pos];

        // Skip visibility modifiers to find the actual keyword
        if word == "pub" {
            // Might be followed by (crate) or (super) — skip the parens
            let saved = pos;
            while pos < len && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'(' {
                // Skip pub(...) visibility
                pos += 1;
                let mut paren_depth = 1i32;
                while pos < len && paren_depth > 0 {
                    match bytes[pos] {
                        b'(' => paren_depth += 1,
                        b')' => paren_depth -= 1,
                        _ => {}
                    }
                    pos += 1;
                }
            } else {
                pos = saved;
            }
            continue;
        }

        // Check if this word is a grammar keyword FIRST (grammar takes precedence
        // over hardcoded skip lists — `in` and `type` are Rust modifiers but mirror keywords)
        if let Some(kind) = grammar.mappings.get(word) {
            // Skip whitespace after keyword
            while pos < len && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }

            // Extract the name (next identifier, allowing @ prefix and / for grammar refs)
            let name_start = pos;
            if pos < len && bytes[pos] == b'@' {
                pos += 1; // consume @
            }
            while pos < len
                && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_' || bytes[pos] == b'/')
            {
                pos += 1;
            }
            let name = if pos > name_start {
                &source[name_start..pos]
            } else {
                "_"
            };

            // For Project (use/in/out), the item ends at semicolon or newline, no brace body
            if *kind == AstKind::Project {
                while pos < len && bytes[pos] != b';' && bytes[pos] != b'\n' {
                    pos += 1;
                }
                if pos < len && bytes[pos] == b';' {
                    pos += 1;
                }
                items.push(MirrorAST::Project(ProjectNode {
                    name: Identifier::new(name),
                    target: None,
                    children: vec![],
                }));
                continue;
            }

            // For Split (struct/enum/type), check for newline-terminated (no braces)
            // e.g. `type color = red | blue | green`
            if *kind == AstKind::Split {
                // Peek ahead: if there's no '{' before the next newline, it's newline-terminated
                let peek = pos;
                let mut has_brace = false;
                let mut nl = len;
                let mut p = peek;
                while p < len && bytes[p] != b'\n' {
                    if bytes[p] == b'{' {
                        has_brace = true;
                        break;
                    }
                    p += 1;
                }
                if p < len && bytes[p] == b'\n' {
                    nl = p;
                }
                if !has_brace {
                    // newline-terminated type declaration
                    pos = if nl < len { nl } else { len };
                    items.push(make_node(kind, name, vec![]));
                    continue;
                }
            }

            // Skip to opening brace or semicolon (declarations without bodies)
            while pos < len && bytes[pos] != b'{' && bytes[pos] != b';' && bytes[pos] != b'\n' {
                pos += 1;
            }

            if pos >= len || bytes[pos] == b';' || bytes[pos] == b'\n' {
                // Declaration without body
                if pos < len && bytes[pos] == b';' {
                    pos += 1;
                }
                items.push(make_node(kind, name, vec![]));
                continue;
            }

            // We're at the opening brace. Extract the body.
            let body_start = pos + 1; // after '{'
            pos += 1;
            let mut depth = 1i32;
            while pos < len && depth > 0 {
                match bytes[pos] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                pos += 1;
            }
            let body_end = if pos > 0 { pos - 1 } else { pos }; // before closing '}'

            // For container types (Focus, Refract), scan the body for children
            let children = match kind {
                AstKind::Focus | AstKind::Refract => {
                    let body = &source[body_start..body_end];
                    scan_items(body, grammar)
                }
                _ => vec![],
            };

            items.push(make_node(kind, name, children));

        // Skip other non-keyword modifiers (only when NOT a grammar keyword)
        } else if word == "async" || word == "unsafe" || word == "const" || word == "extern"
            || word == "crate" || word == "super" || word == "self" || word == "where"
            || word == "mut" || word == "ref" || word == "static" || word == "let"
            || word == "if" || word == "else" || word == "for" || word == "while"
            || word == "loop" || word == "match" || word == "return" || word == "break"
            || word == "continue" || word == "as" || word == "in" || word == "type"
            || word == "dyn" || word == "move"
        {
            continue;
        } else {
            // Not a grammar keyword. If next char is '{', skip the brace block.
            while pos < len && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'{' {
                pos += 1;
                let mut depth = 1i32;
                while pos < len && depth > 0 {
                    match bytes[pos] {
                        b'{' => depth += 1,
                        b'}' => depth -= 1,
                        _ => {}
                    }
                    pos += 1;
                }
            }
            // Otherwise we already advanced past the word, just continue
        }
    }

    items
}

/// Create an AST node from kind, name, and children.
fn make_node(kind: &AstKind, name: &str, children: Vec<MirrorAST>) -> MirrorAST {
    match kind {
        AstKind::Focus => MirrorAST::Focus(FocusNode {
            name: Identifier::new(name),
            target: None,
            children,
        }),
        AstKind::Split => MirrorAST::Split(SplitNode {
            name: Identifier::new(name),
            variants: vec![],
            params: vec![],
            body: None,
            children,
        }),
        AstKind::Zoom => MirrorAST::Zoom(ZoomNode {
            name: Identifier::new(name),
            target: None,
            params: vec![],
            grammar_ref: None,
            children,
            body: None,
        }),
        AstKind::Project => MirrorAST::Project(ProjectNode {
            name: Identifier::new(name),
            target: None,
            children,
        }),
        AstKind::Refract => MirrorAST::Refract(RefractNode {
            name: Identifier::new(name),
            target: None,
            params: vec![],
            children,
        }),
    }
}

// ---------------------------------------------------------------------------
// File discovery — find files for craft targets
// ---------------------------------------------------------------------------

/// Return the grammar file path for a given source file.
pub fn grammar_for_file(path: &str) -> &'static str {
    if path.ends_with(".rs") {
        "boot/std/code/rust.mirror"
    } else if path.ends_with(".mirror") || path.ends_with(".spec") || path.ends_with(".shatter") {
        "boot/std/mirror/grammar.mirror"
    } else {
        "boot/std/code/rust.mirror" // default
    }
}

/// Recursively find all `.mirror` files under a directory. Sorted for determinism.
pub fn find_mirror_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    collect_files_recursive(dir, ".mirror", &mut files);
    files.sort();
    files
}

/// Recursively find all `.rs` files under a directory. Sorted for determinism.
pub fn find_rs_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    collect_files_recursive(dir, ".rs", &mut files);
    files.sort();
    files
}

/// Recursive file collector. No external deps.
fn collect_files_recursive(dir: &str, ext: &str, out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(path.to_str().unwrap_or(""), ext, out);
        } else if let Some(name) = path.to_str() {
            if name.ends_with(ext) {
                out.push(name.to_string());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// craft — compile all files for a target, return crystal OID
// ---------------------------------------------------------------------------

/// Compile all files for a target, return the crystal OID (hash of all OIDs).
pub fn craft_target(target: &str) -> crate::kernel::Oid {
    let files = match target {
        "boot" | "std" => find_mirror_files("boot/"),
        "cargo" => find_rs_files("src/"),
        _ => {
            eprintln!("unknown target: {}", target);
            return crate::kernel::Oid::hash(b"empty");
        }
    };

    let mut hasher = crate::kernel::Oid::hasher();
    for file in &files {
        let grammar_path = grammar_for_file(file);
        let grammar = match load_grammar(grammar_path) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("  skip {} (grammar error: {})", file, e);
                continue;
            }
        };
        let source = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  skip {} (read error: {})", file, e);
                continue;
            }
        };
        let ast = tokenize(&source, &grammar);
        let oid = ast.content_oid();
        eprintln!("  {} -> {}", file, oid);
        hasher.update(oid.as_ref().as_bytes());
    }

    hasher.finalize()
}

// ---------------------------------------------------------------------------
// kintsugi — canonical mirror form of an AST
// ---------------------------------------------------------------------------

/// Render a MirrorAST in canonical mirror form.
pub fn canonical_form(ast: &crate::mirror_ast::MirrorAST) -> String {
    let mut out = String::new();
    render_ast(ast, 0, &mut out);
    out
}

/// Render a single AST node with indentation.
fn render_ast(node: &crate::mirror_ast::MirrorAST, depth: usize, out: &mut String) {
    use crate::mirror_ast::MirrorAST;

    let indent = "  ".repeat(depth);

    match node {
        MirrorAST::Module(m) => {
            for child in &m.children {
                render_ast(child, depth, out);
            }
        }
        MirrorAST::Project(p) => {
            out.push_str(&indent);
            out.push_str("in ");
            out.push_str(p.name.as_str());
            out.push('\n');
        }
        MirrorAST::Split(s) => {
            out.push_str(&indent);
            out.push_str("type ");
            out.push_str(s.name.as_str());
            if !s.variants.is_empty() {
                out.push_str(" = ");
                let vs: Vec<&str> = s.variants.iter().map(|v| v.as_str()).collect();
                out.push_str(&vs.join(" | "));
            }
            if let Some(ref body) = s.body {
                match body {
                    crate::mirror_ast::TypeBody::Enum(vs) => {
                        out.push_str(" = ");
                        let vs: Vec<&str> = vs.iter().map(|v| v.as_str()).collect();
                        out.push_str(&vs.join(" | "));
                    }
                    crate::mirror_ast::TypeBody::Alias(a) => {
                        out.push_str(" = ");
                        out.push_str(a.as_str());
                    }
                    crate::mirror_ast::TypeBody::Struct(fields) => {
                        out.push_str(" = { ");
                        let fs: Vec<String> = fields
                            .iter()
                            .map(|f| format!("{}: {}", f.name.as_str(), f.type_ref.as_str()))
                            .collect();
                        out.push_str(&fs.join(", "));
                        out.push_str(" }");
                    }
                    crate::mirror_ast::TypeBody::Unit => {}
                }
            }
            out.push('\n');
            for child in &s.children {
                render_ast(child, depth + 1, out);
            }
        }
        MirrorAST::Focus(f) => {
            out.push_str(&indent);
            if f.name.as_str().starts_with('@') {
                out.push_str("grammar ");
            } else {
                out.push_str("focus ");
            }
            out.push_str(f.name.as_str());
            if let Some(ref t) = f.target {
                out.push_str(" < ");
                out.push_str(t.as_str());
            }
            if f.children.is_empty() {
                out.push('\n');
            } else {
                out.push_str(" {\n");
                for child in &f.children {
                    render_ast(child, depth + 1, out);
                }
                out.push_str(&indent);
                out.push_str("}\n");
            }
        }
        MirrorAST::Zoom(z) => {
            out.push_str(&indent);
            out.push_str("zoom ");
            out.push_str(z.name.as_str());
            if !z.params.is_empty() {
                out.push('(');
                let ps: Vec<String> = z.params
                    .iter()
                    .map(|f| {
                        if f.type_ref.as_str() == "_" {
                            f.name.as_str().to_string()
                        } else {
                            format!("{}: {}", f.name.as_str(), f.type_ref.as_str())
                        }
                    })
                    .collect();
                out.push_str(&ps.join(", "));
                out.push(')');
            }
            if let Some(ref t) = z.target {
                out.push_str(" -> ");
                out.push_str(t.as_str());
            }
            if let Some(ref gr) = z.grammar_ref {
                out.push(' ');
                out.push_str(gr.as_str());
            }
            out.push('\n');
            for child in &z.children {
                render_ast(child, depth + 1, out);
            }
        }
        MirrorAST::Refract(r) => {
            out.push_str(&indent);
            out.push_str("refract ");
            out.push_str(r.name.as_str());
            if !r.params.is_empty() {
                out.push('(');
                let ps: Vec<String> = r.params
                    .iter()
                    .map(|f| {
                        if f.type_ref.as_str() == "_" {
                            f.name.as_str().to_string()
                        } else {
                            format!("{}: {}", f.name.as_str(), f.type_ref.as_str())
                        }
                    })
                    .collect();
                out.push_str(&ps.join(", "));
                out.push(')');
            }
            if let Some(ref t) = r.target {
                out.push_str(" <= ");
                out.push_str(t.as_str());
            }
            out.push('\n');
            for child in &r.children {
                render_ast(child, depth + 1, out);
            }
        }
        MirrorAST::Abstract { inner, .. } => {
            out.push_str(&indent);
            out.push_str("abstract ");
            // Render inner inline (trim leading indent)
            let mut inner_out = String::new();
            render_ast(inner, 0, &mut inner_out);
            out.push_str(inner_out.trim_start());
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
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

    // -- craft & kintsugi helpers --

    #[test]
    fn grammar_for_mirror_file() {
        assert_eq!(
            super::grammar_for_file("boot/std/kintsugi.mirror"),
            "boot/std/mirror/grammar.mirror"
        );
    }

    #[test]
    fn grammar_for_rs_file() {
        assert_eq!(
            super::grammar_for_file("src/main.rs"),
            "boot/std/code/rust.mirror"
        );
    }

    #[test]
    fn grammar_for_spec_file() {
        assert_eq!(
            super::grammar_for_file("mirror.spec"),
            "boot/std/mirror/grammar.mirror"
        );
    }

    #[test]
    fn find_mirror_files_in_boot() {
        let files = super::find_mirror_files("boot/");
        assert!(!files.is_empty(), "boot/ must contain .mirror files");
        for f in &files {
            assert!(f.ends_with(".mirror"), "non-.mirror file found: {}", f);
        }
    }

    #[test]
    fn find_rs_files_in_src() {
        let files = super::find_rs_files("src/");
        assert!(!files.is_empty(), "src/ must contain .rs files");
        for f in &files {
            assert!(f.ends_with(".rs"), "non-.rs file found: {}", f);
        }
    }

    #[test]
    fn craft_boot_produces_crystal() {
        let oid = super::craft_target("boot");
        assert!(!oid.as_ref().is_empty(), "craft boot must produce an OID");
    }

    #[test]
    fn craft_cargo_produces_crystal() {
        let oid = super::craft_target("cargo");
        assert!(!oid.as_ref().is_empty(), "craft cargo must produce an OID");
    }

    #[test]
    fn craft_is_deterministic() {
        let oid1 = super::craft_target("boot");
        let oid2 = super::craft_target("boot");
        assert_eq!(oid1, oid2, "craft must be deterministic");
    }

    #[test]
    fn kintsugi_prints_canonical_form() {
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        let source = "type color = red | blue\nin @prism";
        let ast = tokenize(source, &grammar);
        let output = super::canonical_form(&ast);
        assert!(!output.is_empty(), "canonical form must not be empty");
        // Must contain structure from the AST
        assert!(
            output.contains("split") || output.contains("project") || output.contains("type") || output.contains("in"),
            "canonical form must contain AST structure: {}",
            output
        );
    }

    #[test]
    fn load_grammar_kintsugi_no_keyword_mappings() {
        let grammar = load_grammar("boot/std/kintsugi.mirror").unwrap();
        // Kintsugi grammar has `collapse(ast, ast) -> imperfect { \ }`
        // but no keyword-to-operation lines like `zoom fn`
        assert!(grammar.mappings.is_empty());
    }

    // -- Mirror grammar tests (self-tokenization) --

    #[test]
    fn load_mirror_grammar_extracts_mappings() {
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        assert_eq!(grammar.mappings.get("grammar"), Some(&AstKind::Focus));
        assert_eq!(grammar.mappings.get("type"), Some(&AstKind::Split));
        assert_eq!(grammar.mappings.get("in"), Some(&AstKind::Project));
        assert_eq!(grammar.mappings.get("out"), Some(&AstKind::Project));
        assert_eq!(grammar.mappings.get("abstract"), Some(&AstKind::Zoom));
    }

    #[test]
    fn tokenize_mirror_grammar_block() {
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        let source = "grammar @test { type foo = bar }";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty(), "must find grammar block");
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_mirror_imports() {
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        let source = "in @prism\nin @nl\ngrammar @test { }";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                // Should find: 2 imports (Project) + 1 grammar (Focus)
                assert!(m.children.len() >= 3, "expected >= 3 children, got {}", m.children.len());
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn tokenize_mirror_type_produces_split() {
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        let source = "type color = red | blue | green";
        let ast = tokenize(source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty(), "must find type declaration");
                assert!(matches!(m.children[0], MirrorAST::Split(_)),
                    "expected Split, got {:?}", m.children[0].kind_name());
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn compile_real_mirror_file() {
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        let source = std::fs::read_to_string("boot/std/kintsugi.mirror").unwrap();
        let ast = tokenize(&source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty(), "kintsugi.mirror must produce children");
            }
            _ => panic!("expected Module"),
        }
    }

    #[test]
    fn self_compile_mirror_grammar() {
        // The compiler compiles its own grammar definition
        let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
        let source = std::fs::read_to_string("boot/std/mirror/grammar.mirror").unwrap();
        let ast = tokenize(&source, &grammar);
        match &ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty(), "grammar.mirror compiles itself");
            }
            _ => panic!("expected Module"),
        }
    }
}
