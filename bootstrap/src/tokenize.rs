//! Tokenizer. Byte-for-byte equivalent to the C `scan_items`/`tokenize`.

use crate::ast::{AstKind, AstNode, DarkSpan};
use crate::grammar::{is_skip_word, Grammar};

/// True iff a brace-block body's content (between but excluding the outer
/// `{` and `}`) is, after trimming ASCII whitespace, exactly the `\` kintsugi
/// obligation marker. Such bodies are explicit holes — NOT dark regions.
fn body_is_obligation(bytes: &[u8]) -> bool {
    let mut start = 0usize;
    let mut end = bytes.len();
    while start < end
        && matches!(bytes[start], b' ' | b'\t' | b'\n' | b'\r')
    {
        start += 1;
    }
    while end > start
        && matches!(bytes[end - 1], b' ' | b'\t' | b'\n' | b'\r')
    {
        end -= 1;
    }
    end - start == 1 && bytes[start] == b'\\'
}

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

/// Identifier characters for io binding / match / select names.
/// Same as `is_name_char` minus `/` — io binding names are bare identifiers.
fn is_io_name_char(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
}

/// Skip horizontal whitespace (spaces, tabs). Does NOT consume newlines.
fn skip_hspace(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }
    pos
}

/// Scan a balanced `(...)` block starting at the opening `(`. Returns index
/// of the closing `)` + 1, or len if unbalanced. Used for io-binding args
/// and match/select subjects.
fn scan_paren_block(bytes: &[u8], mut pos: usize) -> usize {
    let len = bytes.len();
    if pos >= len || bytes[pos] != b'(' {
        return pos;
    }
    pos += 1;
    let mut depth = 1;
    while pos < len && depth > 0 {
        if bytes[pos] == b'(' {
            depth += 1;
        } else if bytes[pos] == b')' {
            depth -= 1;
        }
        pos += 1;
    }
    pos
}

/// Scan a balanced `{...}` block starting at the opening `{`. Returns the
/// index ONE PAST the closing `}` (or len if unbalanced). Differs from
/// `scan_brace_block` which returns the closing index itself.
fn scan_brace_block_past(bytes: &[u8], mut pos: usize) -> usize {
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
        pos += 1;
    }
    pos
}

/// Last non-whitespace byte in `bytes[start..end]`, or 0 if none.
fn last_non_ws(bytes: &[u8], start: usize, end: usize) -> u8 {
    let mut i = end;
    while i > start {
        i -= 1;
        if !matches!(bytes[i], b' ' | b'\t' | b'\r') {
            return bytes[i];
        }
    }
    0
}

/// Capture an io-binding body that may span multiple lines. Starts at the
/// position just past the `=`. Returns end position (just past final
/// newline, or at EOF).
///
/// Continuation rules: a line that ends with `=`, `,`, `(`, `[`, or `>`
/// continues onto the next; a blank-content line (after the introducing
/// `=`) also continues so the body's actual content can land on the next
/// line. Stops at the first line that completes the binding.
fn capture_io_body_end(bytes: &[u8], mut pos: usize) -> usize {
    let len = bytes.len();
    let mut saw_content = false;
    loop {
        let line_start = pos;
        while pos < len && bytes[pos] != b'\n' {
            pos += 1;
        }
        let last = last_non_ws(bytes, line_start, pos);
        let line_blank = last == 0;
        let cont_marker = matches!(last, b'=' | b',' | b'(' | b'[' | b'>');
        if !line_blank {
            saw_content = true;
        }
        if pos < len {
            pos += 1; // consume newline
        }
        // Continue if:
        //   - we haven't seen any content yet (the `=` was at end of intro line)
        //   - or the just-finished line ends with a continuation marker
        let cont = (!saw_content) || cont_marker;
        if !cont || pos >= len {
            return pos;
        }
    }
}

/// Truncate a byte slice to at most 255 bytes and copy into a String
/// (matching the C `name[256]` field semantics).
fn name_string(slice: &[u8]) -> String {
    let n = slice.len().min(255);
    String::from_utf8_lossy(&slice[..n]).into_owned()
}

fn scan_items(
    source: &[u8],
    grammar: &Grammar,
    parent: &mut AstNode,
    base_off: usize,
) {
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
            // Unrecognized non-whitespace bytes. Previously this branch
            // silently advanced a single byte, causing constructs like
            // `imperfect<portal>` to vanish into the gaps between
            // recognized tokens — `--strict` then saw zero darks and
            // reported success on hidden drift (#91).
            //
            // Restore the `--strict` substrate guarantee that every
            // source byte either enters an AST node, is comment/
            // whitespace, OR triggers an error: coalesce the run of
            // unrecognized non-whitespace bytes (up to the next
            // whitespace, word-char, recognized punctuation, or EOF)
            // into a Dark span so the existing strict-mode walker
            // surfaces them.
            //
            // Per docs/specs/strict-and-total-classification.md and the
            // RED test at bootstrap/tests/strict_byte_coverage.rs.
            let dark_start = pos;
            while pos < len
                && !matches!(
                    bytes[pos],
                    b' ' | b'\t' | b'\n' | b'\r'
                        | b'"' | b'#' | b';' | b'{'
                        | b')' | b']' | b'}'
                )
                && !is_word_char(bytes[pos])
            {
                pos += 1;
            }
            if pos == dark_start {
                // Defensive: shouldn't happen, but ensure forward progress.
                pos += 1;
            }
            let dark_bytes = String::from_utf8_lossy(&bytes[dark_start..pos]).into_owned();
            let span = DarkSpan {
                start: base_off + dark_start,
                end: base_off + pos,
            };
            parent.add_child(AstNode::dark(&dark_bytes, span));
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

        // Spec A — io lambda binding.
        //
        //   io <name>(<args>) = <lens-call> > <selector>[<attr>="literal"]
        //
        // The whole form, identifier through the selector tail, becomes a
        // single `IoBinding` AstNode. Active only inside @mirror/grammar so
        // it doesn't disturb other languages' tokenizations (Rust skip_words
        // still drop `io` everywhere it isn't a binding).
        if word == "io" && grammar.is_mirror() && word_at_line_start {
            let body_start_for_name = pos; // already past the `io` keyword
            let name_scan_start = skip_hspace(bytes, pos);
            let mut name_end = name_scan_start;
            while name_end < len && is_io_name_char(bytes[name_end]) {
                name_end += 1;
            }
            if name_end > name_scan_start {
                let name = name_string(&bytes[name_scan_start..name_end]);
                // Args list, if present.
                let after_args = if name_end < len && bytes[name_end] == b'(' {
                    scan_paren_block(bytes, name_end)
                } else {
                    name_end
                };
                // Look for the `=` that introduces the body. Allow it on the
                // current line or the next. Don't go past two newlines.
                let mut scan = after_args;
                let mut nl_seen = 0;
                while scan < len {
                    match bytes[scan] {
                        b'=' => break,
                        b'\n' => {
                            nl_seen += 1;
                            if nl_seen > 1 {
                                break;
                            }
                            scan += 1;
                        }
                        b' ' | b'\t' | b'\r' => scan += 1,
                        _ => break,
                    }
                }
                if scan < len && bytes[scan] == b'=' {
                    let end = capture_io_body_end(bytes, scan + 1);
                    // Verbatim body text: everything between (inclusive) the
                    // gap after the name and the final newline, minus the
                    // trailing newline itself for clean round-trip.
                    let mut body_end = end;
                    while body_end > body_start_for_name
                        && matches!(bytes[body_end - 1], b'\n' | b'\r')
                    {
                        body_end -= 1;
                    }
                    let body = String::from_utf8_lossy(
                        &bytes[name_end..body_end],
                    )
                    .into_owned();
                    let mut node = AstNode::new(AstKind::IoBinding, &name);
                    node.set_keyword("io");
                    node.set_body(&body);
                    parent.add_child(node);
                    pos = end;
                    // We consumed up to and past a newline; next token is at
                    // line start.
                    at_line_start = true;
                    continue;
                }
                // No `=` found within a reasonable window: fall through and
                // let the unknown-word path handle the leftovers.
                pos = after_args;
                continue;
            }
            // Bare `io` with no identifier after — drop and continue.
            continue;
        }

        // Spec B — match expression.
        //
        //   match <subject> { <pattern> => <body>, ... }
        //
        // The subject runs up to the opening `{`; the arms are captured as
        // the verbatim body of a single `MatchExpr` AstNode.
        if word == "match" && grammar.is_mirror() && word_at_line_start {
            let subj_start = skip_hspace(bytes, pos);
            // Find the opening `{` that introduces the arm list.
            let mut scan = subj_start;
            let mut depth_paren = 0i32;
            while scan < len {
                match bytes[scan] {
                    b'(' => {
                        depth_paren += 1;
                        scan += 1;
                    }
                    b')' => {
                        depth_paren -= 1;
                        scan += 1;
                    }
                    b'{' if depth_paren == 0 => break,
                    b'\n' if depth_paren == 0 => break,
                    _ => scan += 1,
                }
            }
            if scan < len && bytes[scan] == b'{' {
                let subj_end = {
                    let mut e = scan;
                    while e > subj_start
                        && matches!(bytes[e - 1], b' ' | b'\t' | b'\r')
                    {
                        e -= 1;
                    }
                    e
                };
                let subject = name_string(&bytes[subj_start..subj_end]);
                let block_end = scan_brace_block_past(bytes, scan);
                let body = String::from_utf8_lossy(
                    &bytes[subj_end..block_end],
                )
                .into_owned();
                let mut node = AstNode::new(AstKind::MatchExpr, &subject);
                node.set_keyword("match");
                node.set_body(&body);
                parent.add_child(node);
                pos = block_end;
                at_line_start = true;
                continue;
            }
            // No brace — let the rest of the line be reprocessed.
            pos = subj_start;
            continue;
        }

        // Spec B — select closure form.
        //
        //   select |<binder>| { <variant>(args) => <body>, ... }
        //
        // The binder is captured as the AST node's name. Body is verbatim.
        if word == "select" && grammar.is_mirror() && word_at_line_start {
            let after = skip_hspace(bytes, pos);
            let mut scan = after;
            // Find the opening `{`.
            while scan < len && bytes[scan] != b'{' && bytes[scan] != b'\n' {
                scan += 1;
            }
            if scan < len && bytes[scan] == b'{' {
                // The header between `select` and `{` is `|x|` (or empty).
                let mut header_end = scan;
                while header_end > after
                    && matches!(bytes[header_end - 1], b' ' | b'\t' | b'\r')
                {
                    header_end -= 1;
                }
                let binder = name_string(&bytes[after..header_end]);
                let block_end = scan_brace_block_past(bytes, scan);
                let body = String::from_utf8_lossy(
                    &bytes[header_end..block_end],
                )
                .into_owned();
                let mut node = AstNode::new(AstKind::SelectExpr, &binder);
                node.set_keyword("select");
                node.set_body(&body);
                parent.add_child(node);
                pos = block_end;
                at_line_start = true;
                continue;
            }
            pos = after;
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
                // Skip only trailing whitespace and an optional `;`.
                // The previous shape consumed everything to end of line,
                // which silently swallowed same-line content (e.g.
                // `in @y unknown_keyword { dark }` would lose the unknown
                // construct). Per Seam T1.1: distinct sibling content on
                // the same line must remain visible to the parser so the
                // Dark branch can capture it. Comments and newlines are
                // handled by the main loop on the next iteration.
                while pos < len && matches!(bytes[pos], b' ' | b'\t') {
                    pos += 1;
                }
                if pos < len && bytes[pos] == b';' {
                    pos += 1;
                }
                while pos < len && matches!(bytes[pos], b' ' | b'\t') {
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
                scan_items(
                    &bytes[body_start..body_end],
                    grammar,
                    &mut node,
                    base_off + body_start,
                );
            }
            parent.add_child(node);
        } else if is_skip_word(&word) {
            continue;
        } else {
            // Action declaration form (only inside @mirror/grammar, only at
            // line start). Surface shape:
            //
            //   <name>(<args>) -> <typename> { <body> }
            //   <decorator> <name>(<args>) -> <typename> { <body> }
            //
            // Examples:
            //   observe(imperfect) -> observation { @beam.emit }
            //   load(dir: ~dir) -> peer { peer { ... } }
            //   property autopoietic() -> verdict { @epistemologic/math/lawvere.is_autopoietic(@cogito) }
            //   fn filename(p: peer_file) -> text { identity => "identity.mirror", ... }
            //
            // Before this rule the unknown-word path saw only `<name>` (no
            // `{`), did nothing, and the `(`, args, `->`, return-type
            // identifier, and body were processed piecewise. The return
            // type identifier landed back here as a bare unknown word
            // followed by `{ <body> }` and got captured as a Dark region.
            // The full declaration becomes a single IoBinding node so the
            // return-type identifier is no longer surfaced as an
            // unrecognised construct.
            if word_at_line_start && grammar.is_mirror() {
                // Peek for the action-declaration shape. Three positions:
                //   name_start ..= name_end   : the action's identifier
                //   after_paren                : just past `(...)` (or =
                //                                name_end when args omitted)
                //   block_pos                  : opening `{` of the body
                // We only commit (advance `pos`) when the entire shape
                // matches. `word` itself may be either the action name
                // (bare form) or a decorator (`property`, `fn`,
                // `template`, `select`, ...). The `(<args>)` is optional
                // — nullary actions like `serve -> imperfect { ... }`
                // omit the parens.
                let mut name_start = word_start;
                let mut name_end = pos;
                let mut peek = skip_hspace(bytes, pos);
                // Decorator form: a second bare identifier separates the
                // decorator from the `(` (or the `->`, for nullary).
                if peek < len && is_word_char(bytes[peek]) {
                    let id_start = peek;
                    let mut id_end = id_start;
                    while id_end < len && is_word_char(bytes[id_end]) {
                        id_end += 1;
                    }
                    let after_id = skip_hspace(bytes, id_end);
                    if after_id < len
                        && (bytes[after_id] == b'('
                            || (after_id + 1 < len
                                && bytes[after_id] == b'-'
                                && bytes[after_id + 1] == b'>'))
                    {
                        name_start = id_start;
                        name_end = id_end;
                        peek = after_id;
                    }
                }
                let after_paren = if peek < len && bytes[peek] == b'(' {
                    scan_paren_block(bytes, peek)
                } else {
                    peek
                };
                {
                    let arrow = skip_hspace(bytes, after_paren);
                    if arrow + 1 < len
                        && bytes[arrow] == b'-'
                        && bytes[arrow + 1] == b'>'
                    {
                        let ty_start = skip_hspace(bytes, arrow + 2);
                        let mut ty_end = ty_start;
                        while ty_end < len && is_name_char(bytes[ty_end]) {
                            ty_end += 1;
                        }
                        // Parametric return-type form: `T(U)`.
                        //
                        // Per the parametric-types-and-fp-heritage insight
                        // (2026-05-25), the type-layer applications of the
                        // five Prism operations — zoom(T), refract(T), and
                        // their kin — must parse as return types. We admit
                        // a single balanced `(...)` immediately following
                        // the type-name identifier and absorb it into the
                        // return-type span. The full verbatim form lives in
                        // the IoBinding's body for round-trip; downstream
                        // grammar lenses lift it into a ParametricType
                        // node when they need the base/parameter split.
                        if ty_end < len && bytes[ty_end] == b'(' {
                            ty_end = scan_paren_block(bytes, ty_end);
                        }
                        if ty_end > ty_start {
                            let mut block_pos = ty_end;
                            while block_pos < len
                                && matches!(
                                    bytes[block_pos],
                                    b' ' | b'\t' | b'\n' | b'\r'
                                )
                            {
                                block_pos += 1;
                            }
                            if block_pos < len && bytes[block_pos] == b'{' {
                                let block_end =
                                    scan_brace_block_past(bytes, block_pos);
                                let action_name =
                                    name_string(&bytes[name_start..name_end]);
                                // Body: everything after the action name
                                // through the closing brace (args, arrow,
                                // return type, brace block) — verbatim
                                // for round-trip.
                                let body = String::from_utf8_lossy(
                                    &bytes[name_end..block_end],
                                )
                                .into_owned();
                                let mut node = AstNode::new(
                                    AstKind::IoBinding,
                                    &action_name,
                                );
                                node.set_keyword(&word);
                                node.set_body(&body);
                                parent.add_child(node);
                                pos = block_end;
                                at_line_start = true;
                                continue;
                            }
                        }
                    }
                }
            }
            // Unknown word followed by a `{ ... }` block. Previously the
            // block was silently swallowed — the silent absorption mode.
            // The Dark capture now spans from the unknown keyword's first
            // byte through the matching `}` so that distinct keywords
            // (and distinct brace shapes) produce distinct OIDs.
            // Per Seam T1.1 (docs/review/2026-05-20-seam-adversarial.md):
            // hashing only the inner bytes allowed three different source
            // files to crystal-collide on the same OID. The Dark span now
            // carries the leading keyword and the enclosing braces; the
            // renderer round-trips them verbatim.
            while pos < len
                && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r')
            {
                pos += 1;
            }
            if pos < len && bytes[pos] == b'{' {
                pos += 1;
                let content_start = pos; // first byte AFTER `{`
                let mut depth = 1;
                while pos < len && depth > 0 {
                    if bytes[pos] == b'{' {
                        depth += 1;
                    } else if bytes[pos] == b'}' {
                        depth -= 1;
                    }
                    pos += 1;
                }
                let block_end = pos; // one PAST the matching `}`
                let content_end = if block_end > 0 && bytes[block_end - 1] == b'}' {
                    block_end - 1
                } else {
                    block_end
                };
                let inner = &bytes[content_start..content_end];
                if !body_is_obligation(inner) {
                    // Dark span covers <keyword> + whitespace + `{` + inner + `}`,
                    // not just inner. Distinct unknown constructs now hash to
                    // distinct OIDs, and kintsugi round-trip preserves the
                    // surrounding structure verbatim.
                    let dark_range = &bytes[word_start..block_end];
                    let dark_bytes = String::from_utf8_lossy(dark_range).into_owned();
                    let span = DarkSpan {
                        start: base_off + word_start,
                        end: base_off + block_end,
                    };
                    parent.add_child(AstNode::dark(&dark_bytes, span));
                }
            }
        }
    }
}

pub fn tokenize(source: &[u8], grammar: &Grammar) -> AstNode {
    let mut root = AstNode::new(AstKind::Focus, "root");
    scan_items(source, grammar, &mut root, 0);
    if !grammar.r#ref.is_empty() {
        root.tag_recursive(&grammar.r#ref);
    }
    root
}
