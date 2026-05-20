//! Renderer. Mirror canonical form + grammar-aware reverse rendering.
//!
//! Mirrors C `render_ast`, `render_ast_mirror`, `render_ast_with_grammar`.

use crate::ast::{AstKind, AstNode};
use crate::grammar::{grammar_path_for_ref, load_grammar, Grammar};

fn append_indent(out: &mut Vec<u8>, depth: i32) {
    for _ in 0..depth {
        out.extend_from_slice(b"  ");
    }
}

/// Default mirror canonical form.
fn render_ast_mirror(node: &AstNode, depth: i32, out: &mut Vec<u8>) {
    match node.kind {
        AstKind::Focus => {
            if node.name == "root" && depth == 0 {
                for c in &node.children {
                    render_ast(c, depth, out);
                }
                return;
            }
            append_indent(out, depth);
            if node.name.as_bytes().first() == Some(&b'@') {
                out.extend_from_slice(b"grammar ");
            } else {
                out.extend_from_slice(b"focus ");
            }
            out.extend_from_slice(node.name.as_bytes());
            if !node.children.is_empty() {
                out.extend_from_slice(b" {\n");
                for c in &node.children {
                    render_ast(c, depth + 1, out);
                }
                append_indent(out, depth);
                out.extend_from_slice(b"}\n");
            } else {
                out.push(b'\n');
            }
        }
        AstKind::In => {
            append_indent(out, depth);
            out.extend_from_slice(b"in ");
            if node.name.as_bytes().first() != Some(&b'@') {
                out.push(b'@');
            }
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
        }
        AstKind::Out => {
            append_indent(out, depth);
            out.extend_from_slice(b"out ");
            if node.name.as_bytes().first() != Some(&b'@') {
                out.push(b'@');
            }
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
        }
        AstKind::Project => {
            append_indent(out, depth);
            out.extend_from_slice(b"project ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
        }
        AstKind::Split => {
            append_indent(out, depth);
            out.extend_from_slice(b"type ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            for c in &node.children {
                render_ast(c, depth + 1, out);
            }
        }
        AstKind::Zoom => {
            append_indent(out, depth);
            out.extend_from_slice(b"zoom ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            for c in &node.children {
                render_ast(c, depth + 1, out);
            }
        }
        AstKind::Refract => {
            append_indent(out, depth);
            out.extend_from_slice(b"refract ");
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            for c in &node.children {
                render_ast(c, depth + 1, out);
            }
        }
        AstKind::IoBinding => {
            append_indent(out, depth);
            out.extend_from_slice(b"io ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
        }
        AstKind::MatchExpr => {
            append_indent(out, depth);
            out.extend_from_slice(b"match ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
        }
        AstKind::SelectExpr => {
            append_indent(out, depth);
            out.extend_from_slice(b"select ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
        }
    }
}

/// Render an AST using a grammar's reverse mapping.
fn render_ast_with_grammar(node: &AstNode, depth: i32, g: &Grammar, out: &mut Vec<u8>) {
    match node.kind {
        AstKind::Focus => {
            if node.name == "root" && depth == 0 {
                for c in &node.children {
                    render_ast_with_grammar(c, depth, g, out);
                }
                return;
            }
            // Verbatim body path for LLVM-IR-style FOCUS nodes.
            if let Some(body) = &node.body {
                if !body.is_empty() {
                    let fk: &str = if !node.keyword.is_empty() {
                        &node.keyword
                    } else if let Some(k) = g.keyword_for_kind(AstKind::Focus) {
                        k
                    } else if node.name.as_bytes().first() == Some(&b'@') {
                        "grammar"
                    } else {
                        "focus"
                    };
                    append_indent(out, depth);
                    out.extend_from_slice(fk.as_bytes());
                    if !node.name.is_empty() {
                        out.push(b' ');
                        out.extend_from_slice(node.name.as_bytes());
                    }
                    out.extend_from_slice(body.as_bytes());
                    out.push(b'\n');
                    for c in &node.children {
                        render_ast_with_grammar(c, depth + 1, g, out);
                    }
                    return;
                }
            }
            let kw: &str = g
                .keyword_for_kind(AstKind::Focus)
                .unwrap_or_else(|| {
                    if node.name.as_bytes().first() == Some(&b'@') {
                        "grammar"
                    } else {
                        "focus"
                    }
                });
            append_indent(out, depth);
            out.extend_from_slice(kw.as_bytes());
            out.push(b' ');
            out.extend_from_slice(node.name.as_bytes());
            if !node.children.is_empty() {
                out.extend_from_slice(b" {\n");
                for c in &node.children {
                    render_ast_with_grammar(c, depth + 1, g, out);
                }
                append_indent(out, depth);
                out.extend_from_slice(b"}\n");
            } else {
                out.push(b'\n');
            }
            return;
        }
        AstKind::In => {
            append_indent(out, depth);
            out.extend_from_slice(b"in ");
            if node.name.as_bytes().first() != Some(&b'@') {
                out.push(b'@');
            }
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            return;
        }
        AstKind::Out => {
            append_indent(out, depth);
            out.extend_from_slice(b"out ");
            if node.name.as_bytes().first() != Some(&b'@') {
                out.push(b'@');
            }
            out.extend_from_slice(node.name.as_bytes());
            out.push(b'\n');
            return;
        }
        AstKind::IoBinding => {
            append_indent(out, depth);
            out.extend_from_slice(b"io ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
            return;
        }
        AstKind::MatchExpr => {
            append_indent(out, depth);
            out.extend_from_slice(b"match ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
            return;
        }
        AstKind::SelectExpr => {
            append_indent(out, depth);
            out.extend_from_slice(b"select ");
            out.extend_from_slice(node.name.as_bytes());
            if let Some(body) = &node.body {
                out.extend_from_slice(body.as_bytes());
            }
            out.push(b'\n');
            return;
        }
        _ => {}
    }

    // PROJECT, SPLIT, ZOOM, REFRACT — unified path with optional verbatim body.
    let fallback = match node.kind {
        AstKind::Project => "project",
        AstKind::Split => "type",
        AstKind::Zoom => "zoom",
        AstKind::Refract => "refract",
        _ => unreachable!(),
    };
    let kw: String = if !node.keyword.is_empty() {
        node.keyword.clone()
    } else {
        g.keyword_for_kind(node.kind)
            .map(|s| s.to_string())
            .unwrap_or_else(|| fallback.to_string())
    };

    if let Some(body) = &node.body {
        if !body.is_empty() {
            let first = node.name.as_bytes().first().copied();
            let sigil_name = !node.name.is_empty()
                && matches!(first, Some(b'@') | Some(b'%') | Some(b'!') | Some(b'#'));
            append_indent(out, depth);
            if !sigil_name {
                out.extend_from_slice(kw.as_bytes());
                if !node.name.is_empty() {
                    out.push(b' ');
                }
            }
            if !node.name.is_empty() {
                out.extend_from_slice(node.name.as_bytes());
            }
            out.extend_from_slice(body.as_bytes());
            out.push(b'\n');
            for c in &node.children {
                render_ast_with_grammar(c, depth + 1, g, out);
            }
            return;
        }
    }

    append_indent(out, depth);
    out.extend_from_slice(kw.as_bytes());
    out.push(b' ');
    out.extend_from_slice(node.name.as_bytes());
    out.push(b'\n');
    for c in &node.children {
        render_ast_with_grammar(c, depth + 1, g, out);
    }
}

/// Dispatch on the node's grammar tag.
pub fn render_ast(node: &AstNode, depth: i32, out: &mut Vec<u8>) {
    let tag = node.grammar_tag.as_str();
    if tag.is_empty() || tag == "@mirror/grammar" || tag == "@mirror" {
        render_ast_mirror(node, depth, out);
        return;
    }
    let path = match grammar_path_for_ref(tag) {
        Some(p) => p,
        None => {
            render_ast_mirror(node, depth, out);
            return;
        }
    };
    let g = match load_grammar(&path) {
        Ok(g) => g,
        Err(_) => {
            render_ast_mirror(node, depth, out);
            return;
        }
    };
    render_ast_with_grammar(node, depth, &g, out);
}
