//! Tokenizer. Byte-for-byte equivalent to the C `scan_items`/`tokenize`.

use crate::ast::{AstKind, AstNode};
use crate::grammar::{is_skip_word, Grammar};

fn is_ir_ident_char(c: u8) -> bool {
    matches!(c,
        b'a'..=b'z'
        | b'A'..=b'Z'
        | b'0'..=b'9'
        | b'_' | b'.' | b'$' | b'-'
    )
}

fn find_eol(bytes: &[u8], mut pos: usize) -> usize {
    let len = bytes.len();
    while pos < len && bytes[pos] != b'\n' {
        pos += 1;
    }
    pos
}

/// Scan a balanced brace block starting AT the opening `{`. Returns the index
/// of the closing `}` (or len if unbalanced). Mirrors C `scan_brace_block`.
fn scan_brace_block(bytes: &[u8], mut pos: usize) -> usize {
    let len = bytes.len();
    if pos >= len || bytes[pos] != b'{' {
        return pos;
    }
    pos += 1;
    let mut depth = 1;
    while pos < len && depth > 0 {
        if bytes[pos] == b'{' {
            depth += 1;
        } else if bytes[pos] == b'}' {
            depth -= 1;
        }
        if depth > 0 {
            pos += 1;
        }
    }
    pos
}

fn is_word_char(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
}

fn is_name_char(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'/')
}

/// Truncate a byte slice to at most 255 bytes and copy into a String
/// (matching the C `name[256]` field semantics).
fn name_string(slice: &[u8]) -> String {
    let n = slice.len().min(255);
    String::from_utf8_lossy(&slice[..n]).into_owned()
}

fn scan_items(source: &[u8], grammar: &Grammar, parent: &mut AstNode) {
    let bytes = source;
    let len = bytes.len();
    let mut pos = 0usize;
    let llvm = grammar.is_llvm_ir();
    let mut at_line_start = true;

    while pos < len {
        // Skip whitespace.
        while pos < len
            && (bytes[pos] == b' '
                || bytes[pos] == b'\t'
                || bytes[pos] == b'\n'
                || bytes[pos] == b'\r')
        {
            if bytes[pos] == b'\n' {
                at_line_start = true;
            }
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // LLVM IR comment: `;` at line start skips to EOL.
        if llvm && at_line_start && bytes[pos] == b';' {
            pos = find_eol(bytes, pos);
            continue;
        }

        // LLVM IR sigil-prefix forms at line start: @id %id !id
        if llvm
            && at_line_start
            && (bytes[pos] == b'@' || bytes[pos] == b'%' || bytes[pos] == b'!')
        {
            let sigil = bytes[pos];
            let name_start = pos;
            pos += 1;
            while pos < len && is_ir_ident_char(bytes[pos]) {
                pos += 1;
            }
            let name_end = pos;
            if name_end == name_start + 1 {
                // bare sigil, skip to EOL
                pos = find_eol(bytes, pos);
                continue;
            }
            let name = name_string(&bytes[name_start..name_end]);
            // Body = name_end .. eol, trim trailing whitespace.
            let body_start = name_end;
            let eol = find_eol(bytes, body_start);
            let mut body_end = eol;
            while body_end > body_start
                && matches!(bytes[body_end - 1], b' ' | b'\t' | b'\r')
            {
                body_end -= 1;
            }
            let kind = if sigil == b'@' {
                AstKind::Project
            } else {
                AstKind::Split
            };
            let mut node = AstNode::new(kind, &name);
            node.set_body(&String::from_utf8_lossy(&bytes[body_start..body_end]));
            parent.add_child(node);
            pos = eol;
            continue;
        }

        // // comment
        if pos + 1 < len && bytes[pos] == b'/' && bytes[pos + 1] == b'/' {
            while pos < len && bytes[pos] != b'\n' {
                pos += 1;
            }
            continue;
        }
        // -- comment
        if pos + 1 < len && bytes[pos] == b'-' && bytes[pos + 1] == b'-' {
            while pos < len && bytes[pos] != b'\n' {
                pos += 1;
            }
            continue;
        }
        // /* ... */ comment
        if pos + 1 < len && bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
            pos += 2;
            while pos + 1 < len && !(bytes[pos] == b'*' && bytes[pos + 1] == b'/') {
                pos += 1;
            }
            if pos + 1 < len {
                pos += 2;
            }
            continue;
        }

        // String literal
        if bytes[pos] == b'"' {
            pos += 1;
            while pos < len && bytes[pos] != b'"' {
                if bytes[pos] == b'\\' {
                    pos += 1;
                }
                pos += 1;
            }
            if pos < len {
                pos += 1;
            }
            at_line_start = false;
            continue;
        }

        // Attribute # / shebang #!
        if bytes[pos] == b'#' {
            if llvm {
                pos = find_eol(bytes, pos);
                continue;
            }
            pos += 1;
            if pos < len && bytes[pos] == b'!' {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'[' {
                pos += 1;
                let mut bracket_depth = 1;
                while pos < len && bracket_depth > 0 {
                    if bytes[pos] == b'[' {
                        bracket_depth += 1;
                    } else if bytes[pos] == b']' {
                        bracket_depth -= 1;
                    }
                    pos += 1;
                }
            } else {
                while pos < len && bytes[pos] != b'\n' {
                    pos += 1;
                }
            }
            continue;
        }

        if bytes[pos] == b')' || bytes[pos] == b']' || bytes[pos] == b'}' {
            pos += 1;
            at_line_start = false;
            continue;
        }

        // Read a word.
        let word_start = pos;
        while pos < len && is_word_char(bytes[pos]) {
            pos += 1;
        }
        if pos == word_start {
            // unrecognised char, advance one byte.
            pos += 1;
            at_line_start = false;
            continue;
        }

        let word_at_line_start = at_line_start;
        at_line_start = false;

        let word_slice = &bytes[word_start..pos];
        let word_len = word_slice.len().min(255);
        let word: String = String::from_utf8_lossy(&word_slice[..word_len]).into_owned();

        // `pub` followed by optional (...) — skip over it.
        if word == "pub" {
            let saved = pos;
            while pos < len
                && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r')
            {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'(' {
                pos += 1;
                let mut paren_depth = 1;
                while pos < len && paren_depth > 0 {
                    if bytes[pos] == b'(' {
                        paren_depth += 1;
                    } else if bytes[pos] == b')' {
                        paren_depth -= 1;
                    }
                    pos += 1;
                }
            } else {
                pos = saved;
            }
            continue;
        }

        if let Some(kind) = grammar.lookup(&word) {
            // LLVM IR keyword forms keep the body verbatim.
            if llvm && word_at_line_start {
                // Skip inter-token space but stay on same line.
                while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
                    pos += 1;
                }
                let name_start = pos;
                if pos < len
                    && (bytes[pos] == b'@'
                        || bytes[pos] == b'%'
                        || bytes[pos] == b'!'
                        || bytes[pos] == b'#')
                {
                    pos += 1;
                }
                while pos < len && is_ir_ident_char(bytes[pos]) {
                    pos += 1;
                }
                let name = name_string(&bytes[name_start..pos]);
                let body_start = pos;
                let mut scan = pos;
                let mut found_brace = false;
                while scan < len && bytes[scan] != b'\n' {
                    if bytes[scan] == b'{' {
                        found_brace = true;
                        break;
                    }
                    scan += 1;
                }
                let body_end;
                if found_brace {
                    let close = scan_brace_block(bytes, scan);
                    body_end = if close < len { close + 1 } else { len };
                    pos = body_end;
                } else {
                    body_end = scan;
                    pos = scan; // leave '\n' for outer loop
                }
                let mut be = body_end;
                while be > body_start
                    && matches!(bytes[be - 1], b' ' | b'\t' | b'\r')
                {
                    be -= 1;
                }
                let mut node = AstNode::new(kind, &name);
                node.set_keyword(&word);
                node.set_body(&String::from_utf8_lossy(&bytes[body_start..be]));
                parent.add_child(node);
                continue;
            }

            // Standard path: skip whitespace, read name.
            while pos < len
                && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r')
            {
                pos += 1;
            }
            let name_start = pos;
            if pos < len && bytes[pos] == b'@' {
                pos += 1;
            }
            while pos < len && is_name_char(bytes[pos]) {
                pos += 1;
            }

            let name = if pos > name_start {
                name_string(&bytes[name_start..pos])
            } else {
                "_".to_string()
            };

            if kind == AstKind::Project {
                while pos < len && bytes[pos] != b';' && bytes[pos] != b'\n' {
                    pos += 1;
                }
                if pos < len && bytes[pos] == b';' {
                    pos += 1;
                }
                if word == "in" && name.starts_with('@') {
                    parent.add_child(AstNode::new(AstKind::In, &name));
                } else if word == "out" {
                    parent.add_child(AstNode::new(AstKind::Out, &name));
                } else {
                    parent.add_child(AstNode::new(AstKind::Project, &name));
                }
                continue;
            }

            if kind == AstKind::Split {
                let mut peek = pos;
                let mut has_brace = false;
                while peek < len && bytes[peek] != b'\n' {
                    if bytes[peek] == b'{' {
                        has_brace = true;
                        break;
                    }
                    peek += 1;
                }
                if !has_brace {
                    if peek < len {
                        pos = peek;
                    } else {
                        pos = len;
                    }
                    parent.add_child(AstNode::new(kind, &name));
                    continue;
                }
            }

            while pos < len
                && bytes[pos] != b'{'
                && bytes[pos] != b';'
                && bytes[pos] != b'\n'
            {
                pos += 1;
            }

            if pos >= len || bytes[pos] == b';' || bytes[pos] == b'\n' {
                if pos < len && bytes[pos] == b';' {
                    pos += 1;
                }
                parent.add_child(AstNode::new(kind, &name));
                continue;
            }

            // Open brace: capture body and recurse if FOCUS or REFRACT.
            let body_start = pos + 1;
            pos += 1;
            let mut depth = 1;
            while pos < len && depth > 0 {
                if bytes[pos] == b'{' {
                    depth += 1;
                } else if bytes[pos] == b'}' {
                    depth -= 1;
                }
                pos += 1;
            }
            let body_end = if pos > 0 { pos - 1 } else { pos };

            let mut node = AstNode::new(kind, &name);
            if kind == AstKind::Focus || kind == AstKind::Refract {
                scan_items(&bytes[body_start..body_end], grammar, &mut node);
            }
            parent.add_child(node);
        } else if is_skip_word(&word) {
            continue;
        } else {
            // Unknown word: skip an optional `{ ... }` block.
            while pos < len
                && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r')
            {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'{' {
                pos += 1;
                let mut depth = 1;
                while pos < len && depth > 0 {
                    if bytes[pos] == b'{' {
                        depth += 1;
                    } else if bytes[pos] == b'}' {
                        depth -= 1;
                    }
                    pos += 1;
                }
            }
        }
    }
}

pub fn tokenize(source: &[u8], grammar: &Grammar) -> AstNode {
    let mut root = AstNode::new(AstKind::Focus, "root");
    scan_items(source, grammar, &mut root);
    if !grammar.r#ref.is_empty() {
        root.tag_recursive(&grammar.r#ref);
    }
    root
}
