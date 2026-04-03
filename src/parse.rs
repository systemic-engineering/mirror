//! Parser story. Source text → AST tree.
//!
//! The parser IS a story: it records a transformation from source to tree.

use crate::ast::{self, AstNode, Span};
use crate::domain::conversation::Kind;
use crate::prism::Prism;
use crate::Trace;
use crate::Vector;

/// Comparison operators shared by `when` predicates and `case` arm patterns.
/// Two-char operators listed first to avoid false prefix matches.
const CMP_OPS: &[(&str, &str)] = &[
    (">=", "gte"),
    ("<=", "lte"),
    ("!=", "ne"),
    ("==", "eq"),
    (">", "gt"),
    ("<", "lt"),
];

/// The parse traceable. Source → AST.
#[derive(Clone, Debug, Default)]
pub struct Parse;

/// What can go wrong during parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub span: Option<Span>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.span {
            Some(span) => write!(
                f,
                "parse error at {}..{}: {}",
                span.start, span.end, self.message
            ),
            None => write!(f, "parse error: {}", self.message),
        }
    }
}

impl std::error::Error for ParseError {}

impl Vector<String, Prism<AstNode>> for Parse {
    type Error = ParseError;

    fn trace(&self, source: String) -> Trace<Prism<AstNode>, ParseError> {
        use crate::ContentAddressed;
        match parse_source(&source) {
            Ok(tree) => {
                let oid = tree.content_oid();
                Trace::success(tree, oid.into(), None)
            }
            Err(e) => Trace::failure(e, crate::TraceOid::new("error"), None),
        }
    }
}

/// Line + byte offset tracking.
struct Lines<'a> {
    lines: Vec<&'a str>,
    /// Byte offset where each line starts in the source.
    offsets: Vec<u32>,
    pos: usize,
}

impl<'a> Lines<'a> {
    fn new(source: &'a str) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        let mut offsets = Vec::with_capacity(lines.len());
        let mut offset = 0u32;
        for line in &lines {
            offsets.push(offset);
            offset += line.len() as u32 + 1; // +1 for \n
        }
        Lines {
            lines,
            offsets,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&'a str> {
        self.lines.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn span_for(&self, idx: usize) -> Span {
        let start = self.offsets[idx];
        let end = start + self.lines[idx].len() as u32;
        Span::new(start, end)
    }

    fn current_span(&self) -> Span {
        self.span_for(self.pos)
    }
}

// ---------------------------------------------------------------------------
// Keyword dispatch table
// ---------------------------------------------------------------------------

type KeywordHandler = fn(&str, &mut Lines) -> Result<Prism<AstNode>, ParseError>;

/// Keyword → handler table. Checked in order; first prefix match wins.
/// Each handler receives the rest of the line after stripping the keyword
/// prefix and is responsible for consuming all lines it needs (including
/// calling `lines.advance()` for single-line keywords).
const KEYWORD_TABLE: &[(&str, KeywordHandler)] = &[
    ("in ", parse_in_keyword),
    ("use ", parse_use_keyword),
    ("when ", parse_when_keyword),
    ("case ", parse_case),
    ("template ", parse_template),
    ("out ", parse_out),
    ("grammar ", parse_grammar),
];

/// Try to dispatch a line via the keyword table.
/// Returns `Ok(Some(node))` on match, `Ok(None)` if no keyword matched.
fn dispatch_keyword(
    trimmed: &str,
    lines: &mut Lines,
) -> Result<Option<Prism<AstNode>>, ParseError> {
    for &(prefix, handler) in KEYWORD_TABLE {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return Ok(Some(handler(rest, lines)?));
        }
    }
    Ok(None)
}

/// `in @domain` / `in @domain(params)` / `in @domain as $name`
fn parse_in_keyword(rest: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let span = lines.current_span();
    let rest = rest.trim();

    // Split alias: "in @domain as $name" or "in @domain(params) as $name"
    let (domain_part, alias) = match rest.split_once(" as ") {
        Some((d, a)) => (d.trim(), Some(a.trim())),
        None => (rest, None),
    };

    // Parse domain: @name(params) or @name or bare
    let (value, param) = if let Some(paren) = domain_part.find('(') {
        let name = &domain_part[..paren];
        let params = domain_part[paren + 1..].trim_end_matches(')');
        (name, Some(params))
    } else {
        (domain_part, None)
    };

    let mut in_children = Vec::new();
    if let Some(p) = param {
        in_children.push(ast::ast_leaf(Kind::Ref, "domain-param", p, span));
    }
    if let Some(a) = alias {
        in_children.push(ast::ast_leaf(Kind::Ref, "alias", a, span));
    }

    let node = if in_children.is_empty() {
        ast::ast_leaf(Kind::Decl, "in", value, span)
    } else {
        ast::ast_branch(Kind::Decl, "in", value, span, in_children)
    };
    lines.advance();
    Ok(node)
}

/// `use ...` — single-line, wraps parse_use.
fn parse_use_keyword(rest: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let node = parse_use(rest, lines.current_span());
    lines.advance();
    Ok(node)
}

/// `when ...` — single-line, wraps parse_when.
fn parse_when_keyword(rest: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let node = parse_when(rest, lines.current_span())?;
    lines.advance();
    Ok(node)
}

fn parse_source(source: &str) -> Result<Prism<AstNode>, ParseError> {
    let mut lines = Lines::new(source);
    let mut children = Vec::new();
    let root_span = Span::new(0, source.len() as u32);

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.advance();
            continue;
        }

        // Keyword dispatch
        if let Some(node) = dispatch_keyword(trimmed, &mut lines)? {
            children.push(node);
            continue;
        }

        // branch(.path) { ... } — special syntax (not keyword + space)
        if trimmed.starts_with("branch(") {
            children.push(parse_branch(trimmed, &mut lines)?);
            continue;
        }

        // annotate(@domain) — inline (leaf) or block (branch with children)
        if trimmed.starts_with("annotate(") {
            let span = lines.current_span();
            children.push(parse_annotate(trimmed, span, &mut lines)?);
            continue;
        }

        // --- separator: sugar for annotate(@test) { ... EOF }
        if trimmed == "---" {
            let span = lines.current_span();
            lines.advance();
            let remaining = collect_remaining(&mut lines);
            let test_children = parse_test_ast(&remaining, span)?;
            children.push(ast::ast_branch(
                Kind::Decl,
                "annotate",
                "@test",
                span,
                test_children,
            ));
            break;
        }

        // Pipeline ending in branch: @json | branch(.path) { ... }
        if trimmed.contains("| branch(") {
            children.push(parse_pipeline_with_branch(trimmed, &mut lines)?);
            continue;
        }

        // Pipeline: A | G | B
        if trimmed.contains('|') {
            children.push(parse_pipeline(trimmed, lines.current_span()));
            lines.advance();
            continue;
        }

        return Err(ParseError {
            message: format!("unexpected: {}", trimmed),
            span: Some(lines.current_span()),
        });
    }

    Ok(ast::ast_branch(
        Kind::Form,
        "group",
        "root",
        root_span,
        children,
    ))
}

fn parse_template(header: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let before_brace = header.split('{').next().unwrap().trim();
    let start_span = lines.current_span();
    lines.advance(); // consume template line

    // Check for param list: $name(params) vs $name
    let (name, mut children) = if let Some(paren) = before_brace.find('(') {
        let name = before_brace[..paren].trim();
        let inner = &before_brace[paren + 1..];
        let closing = inner.rfind(')').unwrap_or(inner.len());
        let param_text = inner[..closing].trim();
        let params = parse_param_list(param_text, start_span)?;
        (name, params)
    } else {
        (before_brace, Vec::new())
    };

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            let span = start_span.merge(&end_span);
            return Ok(ast::ast_branch(
                Kind::Decl,
                "template",
                name,
                span,
                children,
            ));
        }

        if trimmed.is_empty() {
            lines.advance();
            continue;
        }

        let field = parse_field(trimmed, lines.current_span());
        children.push(field);
        lines.advance();
    }

    Err(ParseError {
        message: "unclosed template block".into(),
        span: Some(start_span),
    })
}

fn parse_param_list(text: &str, span: Span) -> Result<Vec<Prism<AstNode>>, ParseError> {
    // Split on commas respecting paren nesting
    let mut segments = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                segments.push(&text[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    segments.push(&text[start..]);

    segments
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| parse_param(s, span))
        .collect()
}

fn parse_param(text: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    // Check for "name: expr" — but only if left side doesn't start with @ or $
    if let Some((left, right)) = text.split_once(':') {
        let left = left.trim();
        if !left.starts_with('@') && !left.starts_with('$') {
            let expr = right.trim();
            let child = parse_param_expr(expr, span);
            return Ok(ast::ast_branch(
                Kind::Atom,
                "param",
                left,
                span,
                vec![child],
            ));
        }
    }

    // Infer name from expression
    let name = infer_name(text, span)?;
    let child = parse_param_expr(text, span);
    Ok(ast::ast_branch(
        Kind::Atom,
        "param",
        &name,
        span,
        vec![child],
    ))
}

fn infer_name(text: &str, span: Span) -> Result<String, ParseError> {
    if text.contains('|') {
        return Err(ParseError {
            message: format!("pipeline param must be explicitly named: {}", text),
            span: Some(span),
        });
    }

    if let Some(without_at) = text.strip_prefix('@') {
        if without_at.contains('.') {
            return Err(ParseError {
                message: format!("dotted path param must be explicitly named: {}", text),
                span: Some(span),
            });
        }
        let name = if let Some(paren) = without_at.find('(') {
            &without_at[..paren]
        } else {
            without_at
        };
        return Ok(name.to_string());
    }

    Err(ParseError {
        message: format!("cannot infer name for param: {}", text),
        span: Some(span),
    })
}

fn parse_param_expr(text: &str, span: Span) -> Prism<AstNode> {
    if text.contains('|') {
        parse_pipeline(text, span)
    } else if text.starts_with('@') && text[1..].contains('.') {
        ast::ast_leaf(Kind::Atom, "path", text, span)
    } else {
        parse_pipeline_segment(text, span)
    }
}

fn parse_field(text: &str, span: Span) -> Prism<AstNode> {
    if let Some((name, rest)) = text.split_once(':') {
        let name = name.trim();
        let rest = rest.trim();

        // Check for pipe: "article | @html"
        let parts: Vec<&str> = rest.splitn(2, '|').collect();
        let mut children = Vec::new();

        let qualifier = parts[0].trim();
        children.push(ast::ast_leaf(Kind::Atom, "qualifier", qualifier, span));

        if parts.len() > 1 {
            let pipe_value = parts[1].trim();
            children.push(ast::ast_leaf(Kind::Atom, "pipe", pipe_value, span));
        }

        ast::ast_branch(Kind::Atom, "field", name, span, children)
    } else if let Some((name, pipe)) = text.split_once('|') {
        // Bare field with pipe: "slug | @sha"
        let name = name.trim();
        let pipe_value = pipe.trim();
        let children = vec![ast::ast_leaf(Kind::Atom, "pipe", pipe_value, span)];
        ast::ast_branch(Kind::Atom, "field", name, span, children)
    } else {
        ast::ast_leaf(Kind::Atom, "field", text.trim(), span)
    }
}

fn parse_out(header: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let span = lines.current_span();
    if let Some((name, _)) = header.split_once('{') {
        let name = name.trim();
        lines.advance();
        let (children, end_span) = parse_block_body(lines, span)?;
        let merged = span.merge(&end_span);
        Ok(ast::ast_branch(Kind::Decl, "out", name, merged, children))
    } else {
        let name = header.trim();
        lines.advance();
        Ok(ast::ast_leaf(Kind::Decl, "out", name, span))
    }
}

fn parse_block_body(
    lines: &mut Lines,
    open_span: Span,
) -> Result<(Vec<Prism<AstNode>>, Span), ParseError> {
    let mut children = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            return Ok((children, end_span));
        }

        if trimmed.is_empty() {
            lines.advance();
            continue;
        }

        // Select: "name: folder { $template }"
        // Field expression: "name: expr"
        if let Some((output_part, rest)) = trimmed.split_once(':') {
            let rest = rest.trim();
            if let Some((folder, template_part)) = rest.split_once('{') {
                let span = lines.current_span();
                let folder_name = folder.trim();
                let template_name = template_part.trim().trim_end_matches('}').trim();

                let select_children = vec![
                    ast::ast_leaf(Kind::Ref, "domain-ref", folder_name, span),
                    ast::ast_leaf(Kind::Ref, "template-ref", template_name, span),
                ];
                children.push(ast::ast_branch(
                    Kind::Form,
                    "select",
                    output_part.trim(),
                    span,
                    select_children,
                ));
                lines.advance();
                continue;
            }

            // Field with expression value: "name: expr"
            let span = lines.current_span();
            let expr_child = ast::ast_leaf(Kind::Atom, "expr", rest, span);
            children.push(ast::ast_branch(
                Kind::Atom,
                "field",
                output_part.trim(),
                span,
                vec![expr_child],
            ));
            lines.advance();
            continue;
        }

        // Group: "name {"
        if let Some((name, rest)) = trimmed.split_once('{') {
            let name = name.trim();
            let span = lines.current_span();

            if rest.trim() == "}" {
                // Empty group: "name {}"
                children.push(ast::ast_branch(Kind::Form, "group", name, span, vec![]));
                lines.advance();
            } else {
                lines.advance();
                let (group_children, end_span) = parse_block_body(lines, span)?;
                let group_span = span.merge(&end_span);
                children.push(ast::ast_branch(
                    Kind::Form,
                    "group",
                    name,
                    group_span,
                    group_children,
                ));
            }
            continue;
        }

        // Bare expression
        let span = lines.current_span();
        children.push(ast::ast_leaf(Kind::Atom, "expr", trimmed, span));
        lines.advance();
    }

    Err(ParseError {
        message: "unclosed block".into(),
        span: Some(open_span),
    })
}

fn push_path_segments(rest: &str, span: Span, children: &mut Vec<Prism<AstNode>>) {
    for seg in rest.split('/').filter(|s| !s.is_empty()) {
        children.push(ast::ast_leaf(Kind::Atom, "path", seg, span));
    }
}

/// Parse the source expression of a `use` statement into origin + path segments.
///
/// - `@domain`          → [DomainRef]
/// - `@domain/sub/path` → [DomainRef, Path, Path]
/// - `$HOME/shared`     → [Home, Path]
/// - `$SELF/templates`  → [Self_, Path]
/// - `./templates`      → [Self_, Path]  (desugar)
fn parse_use_source(source: &str, span: Span, children: &mut Vec<Prism<AstNode>>) {
    // Desugar ./ to $SELF/
    if let Some(rest) = source.strip_prefix("./") {
        children.push(ast::ast_leaf(Kind::Ref, "self", "$SELF", span));
        push_path_segments(rest, span, children);
        return;
    }

    // Check for $HOME/ or $SELF/ prefix
    if let Some(rest) = source.strip_prefix("$HOME/") {
        children.push(ast::ast_leaf(Kind::Ref, "home", "$HOME", span));
        push_path_segments(rest, span, children);
        return;
    }
    if let Some(rest) = source.strip_prefix("$SELF/") {
        children.push(ast::ast_leaf(Kind::Ref, "self", "$SELF", span));
        push_path_segments(rest, span, children);
        return;
    }

    // Check for @domain/path — split into DomainRef + Path segments
    if source.starts_with('@') {
        if let Some(slash_idx) = source.find('/') {
            children.push(ast::ast_leaf(
                Kind::Ref,
                "domain-ref",
                &source[..slash_idx],
                span,
            ));
            push_path_segments(&source[slash_idx + 1..], span, children);
            return;
        }
    }

    // Bare source or @domain without path — treat as DomainRef
    children.push(ast::ast_leaf(Kind::Ref, "domain-ref", source, span));
}

/// Parse a use statement.
///
/// Forms:
/// - `$name from @domain`           → single import
/// - `{ $a, $b } from @domain`      → destructured
/// - `$name from @domain sha: ABC`  → locked
fn parse_use(rest: &str, span: Span) -> Prism<AstNode> {
    let mut children = Vec::new();

    // Split on " from " to get names part and source part
    let (names_part, source_part) = match rest.split_once(" from ") {
        Some((n, s)) => (n.trim(), s.trim()),
        None => (rest, ""),
    };

    // Parse names: either `{ $a, $b }` or `$name`
    if names_part.starts_with('{') {
        let inner = names_part.trim_start_matches('{').trim_end_matches('}');
        for name in inner.split(',') {
            let name = name.trim();
            if !name.is_empty() {
                children.push(ast::ast_leaf(Kind::Ref, "template-ref", name, span));
            }
        }
    } else {
        children.push(ast::ast_leaf(Kind::Ref, "template-ref", names_part, span));
    }

    // Parse source: path expression possibly followed by `sha: ABC`
    let (source_expr, sha_param) = match source_part.split_once(" sha: ") {
        Some((d, s)) => (d.trim(), Some(format!("sha: {}", s.trim()))),
        None => (source_part, None),
    };

    if !source_expr.is_empty() {
        parse_use_source(source_expr, span, &mut children);
    }

    if let Some(param) = sha_param {
        children.push(ast::ast_leaf(Kind::Ref, "domain-param", param, span));
    }

    ast::ast_branch(Kind::Decl, "use", "use", span, children)
}

/// Parse a when predicate: `error.rate > 0.1`, `status == "active"`, etc.
///
/// Operator detection: two-char operators before single-char to avoid false matches.
/// Structure: Decl("when/{op}") with Path (left) and Literal (right) as children.
fn parse_when(rest: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    for (sym, op_name) in CMP_OPS {
        if let Some(idx) = rest.find(sym) {
            let path = rest[..idx].trim();
            let literal = rest[idx + sym.len()..].trim();
            let children = vec![
                ast::ast_leaf(Kind::Atom, "path", path, span),
                ast::ast_leaf(Kind::Atom, "literal", literal, span),
            ];
            let name = format!("when/{}", op_name);
            return Ok(ast::ast_branch(Kind::Decl, name, "", span, children));
        }
    }
    Err(ParseError {
        message: format!("when: no comparison operator in: {}", rest),
        span: Some(span),
    })
}

/// Parse a case block: `error.rate {\n  > 0.1 -> alert\n  _ -> pass\n}`
///
/// Header has already been stripped of `case `. Contains subject + `{`.
/// Arms parsed line-by-line inside the block.
fn parse_case(header: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let subject = header.split('{').next().unwrap().trim();
    let start_span = lines.current_span();
    lines.advance(); // consume case line

    let mut arms = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            let span = start_span.merge(&end_span);
            return Ok(ast::ast_branch(Kind::Decl, "case", subject, span, arms));
        }

        if trimmed.is_empty() {
            lines.advance();
            continue;
        }

        let arm = parse_arm(trimmed, lines.current_span())?;
        arms.push(arm);
        lines.advance();
    }

    Err(ParseError {
        message: "unclosed case block".into(),
        span: Some(start_span),
    })
}

/// Parse a single case arm: `> 0.1 -> alert` or `_ -> pass`.
///
/// Split on ` -> ` to separate pattern from body.
/// Pattern is either `_` (Wild) or an operator + literal (Cmp).
fn parse_arm(text: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    let (pattern_str, body_str) = match text.split_once(" -> ") {
        Some((p, b)) => (p.trim(), b.trim()),
        None => {
            return Err(ParseError {
                message: format!("arm: missing ' -> ' in: {}", text),
                span: Some(span),
            })
        }
    };

    let body = ast::ast_leaf(Kind::Atom, "expr", body_str, span);

    let pattern = if pattern_str == "_" {
        ast::ast_leaf(Kind::Atom, "wild", "", span)
    } else {
        parse_cmp(pattern_str, span)?
    };

    Ok(ast::ast_branch(
        Kind::Form,
        "arm",
        "",
        span,
        vec![pattern, body],
    ))
}

/// Parse a comparison pattern: `> 0.1`, `>= 3`, `== "active"`, etc.
///
/// Operator detection: two-char operators before single-char to avoid false matches.
/// The operator must be a prefix of the pattern text.
fn parse_cmp(text: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    for (sym, op_name) in CMP_OPS {
        if let Some(rest) = text.strip_prefix(sym) {
            let literal = rest.trim();
            let name = format!("cmp/{}", op_name);
            return Ok(ast::ast_leaf(Kind::Atom, name, literal, span));
        }
    }
    Err(ParseError {
        message: format!("arm: no comparison operator in: {}", text),
        span: Some(span),
    })
}

/// Parse a branch block: `branch(.action) {\n  "hold" => ..\n  "exit" => exit\n}`
///
/// Header is the full first line (e.g. `branch(.action) {`).
/// Arms parsed line-by-line. Patterns are string literals or `_` (Wild).
fn parse_branch(header: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    // Extract query path from branch(.action) {
    let after_branch = header.strip_prefix("branch(").unwrap();
    let paren_end = after_branch.find(')').ok_or_else(|| ParseError {
        message: "branch: missing closing ')'".into(),
        span: Some(lines.current_span()),
    })?;
    let query = &after_branch[..paren_end];
    let start_span = lines.current_span();
    lines.advance(); // consume branch line

    let mut arms = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            let span = start_span.merge(&end_span);
            return Ok(ast::ast_branch(Kind::Decl, "branch", query, span, arms));
        }

        if trimmed.is_empty() {
            lines.advance();
            continue;
        }

        let arm = parse_branch_arm(trimmed, lines.current_span())?;
        arms.push(arm);
        lines.advance();
    }

    Err(ParseError {
        message: "unclosed branch block".into(),
        span: Some(start_span),
    })
}

/// Parse a single branch arm: `"hold" => ..` or `_ => exit`.
///
/// Split on ` => ` to separate pattern from action.
fn parse_branch_arm(text: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    let (pattern_str, action_str) = match text.split_once(" => ") {
        Some((p, a)) => (p.trim(), a.trim()),
        None => {
            return Err(ParseError {
                message: format!("branch arm: missing ' => ' in: {}", text),
                span: Some(span),
            })
        }
    };

    let action = ast::ast_leaf(Kind::Atom, "expr", action_str, span);

    let pattern = if pattern_str == "_" {
        ast::ast_leaf(Kind::Atom, "wild", "", span)
    } else if pattern_str.starts_with('"') && pattern_str.ends_with('"') {
        // Strip quotes from string literal
        let inner = &pattern_str[1..pattern_str.len() - 1];
        ast::ast_leaf(Kind::Atom, "literal", inner, span)
    } else {
        return Err(ParseError {
            message: format!(
                "branch arm: pattern must be a quoted string or '_', got: {}",
                pattern_str
            ),
            span: Some(span),
        });
    };

    Ok(ast::ast_branch(
        Kind::Form,
        "arm",
        "",
        span,
        vec![pattern, action],
    ))
}

/// Parse a pipeline that ends with a branch block:
/// `@json | branch(.action) {\n  "hold" => ..\n}\n`
///
/// Splits at `| branch(`, parses prefix as pipeline segments,
/// then parses the branch block. Returns a Pipeline wrapping both.
fn parse_pipeline_with_branch(
    header: &str,
    lines: &mut Lines,
) -> Result<Prism<AstNode>, ParseError> {
    let span = lines.current_span();
    let split_idx = header.find("| branch(").unwrap();
    let prefix = header[..split_idx].trim();
    let branch_part = header[split_idx + 2..].trim(); // skip "| "

    // Parse prefix segments
    let prefix_segments: Vec<Prism<AstNode>> = prefix
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|seg| parse_pipeline_segment(seg, span))
        .collect();

    // Parse the branch block
    let branch_node = parse_branch(branch_part, lines)?;

    let mut pipeline_children = prefix_segments;
    pipeline_children.push(branch_node);
    Ok(ast::ast_branch(
        Kind::Form,
        "pipeline",
        "root",
        span,
        pipeline_children,
    ))
}

/// Parse a pipeline: `@git(branch: "master") | HEAD | @git(branch: "test")`
///
/// Segments separated by `|`. Each segment is either:
/// - `@name(params)` → DomainRef with DomainParam children
/// - `@name` → DomainRef (leaf)
/// - bare name → Ref
fn parse_pipeline(text: &str, span: Span) -> Prism<AstNode> {
    let segments: Vec<&str> = text.split('|').map(|s| s.trim()).collect();
    let children: Vec<Prism<AstNode>> = segments
        .iter()
        .map(|seg| parse_pipeline_segment(seg, span))
        .collect();
    ast::ast_branch(Kind::Form, "pipeline", "root", span, children)
}

fn parse_pipeline_segment(seg: &str, span: Span) -> Prism<AstNode> {
    if seg.starts_with('@') {
        // Domain ref, possibly with params: @git(branch: "master")
        if let Some(paren_start) = seg.find('(') {
            let name = &seg[..paren_start];
            let params = seg[paren_start + 1..].trim_end_matches(')');
            let param_child = ast::ast_leaf(Kind::Ref, "domain-param", params, span);
            ast::ast_branch(Kind::Ref, "domain-ref", name, span, vec![param_child])
        } else {
            ast::ast_leaf(Kind::Ref, "domain-ref", seg, span)
        }
    } else {
        ast::ast_leaf(Kind::Ref, "ref", seg, span)
    }
}

/// Parse a grammar block: `@name {\n  type = ...\n  type op = ...\n}\n`
///
/// Header has already been stripped of `grammar `. Contains `@name {`.
/// Type definitions and continuation lines parsed inside the block.
/// Parse `@name extends @a, @b` from the grammar header (before `{`).
///
/// Returns `(name, extends_domains)` where extends_domains is empty if no
/// `extends` clause is present.
fn parse_extends_clause(name_and_extends: &str) -> (&str, Vec<&str>) {
    if let Some((name, extends_part)) = name_and_extends.split_once(" extends ") {
        let domains: Vec<&str> = extends_part
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        (name.trim(), domains)
    } else {
        (name_and_extends, vec![])
    }
}

/// Parse a parameter list from between parentheses.
///
/// Supports two forms:
/// - Sugar: `name` expands to `name:name`
/// - Explicit: `name: type` stays as `name:type`
///
/// Returns Atom nodes with name="param", value="param_name:type_name".
fn parse_action_params(params_str: &str, span: Span) -> Vec<Prism<AstNode>> {
    if params_str.trim().is_empty() {
        return vec![];
    }
    params_str
        .split(',')
        .map(|p| {
            let p = p.trim();
            if let Some((name, typ)) = p.split_once(':') {
                let name = name.trim();
                let typ = typ.trim();
                ast::ast_leaf(Kind::Atom, "param", format!("{}:{}", name, typ), span)
            } else {
                // Sugar: name alone expands to name:name
                ast::ast_leaf(Kind::Atom, "param", format!("{}:{}", p, p), span)
            }
        })
        .collect()
}

/// Parse `abstract action name(params)` — a signature-only action declaration.
///
/// `rest` is the text after "abstract action ", e.g. `observe(observable)`.
/// Returns a Decl node with name="abstract-action".
fn parse_abstract_action_decl(rest: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    let paren = rest.find('(').ok_or_else(|| ParseError {
        message: format!("abstract action: expected '(' in: abstract action {}", rest),
        span: Some(span),
    })?;
    let name = rest[..paren].trim();
    let close = rest.find(')').ok_or_else(|| ParseError {
        message: format!("abstract action: expected ')' in: abstract action {}", rest),
        span: Some(span),
    })?;
    let params_str = &rest[paren + 1..close];
    let children = parse_action_params(params_str, span);
    Ok(ast::ast_branch(
        Kind::Decl,
        "abstract-action",
        name,
        span,
        children,
    ))
}

/// Parse `name(params) in @target { body }` — an action with body and target.
///
/// `rest` is the text after the action keyword (with visibility stripped),
/// e.g. `decide(observation) in @rust { ... }`.
/// Returns a Form node with name="action-def".
fn parse_action_body_def(
    rest: &str,
    visibility: &str,
    span: Span,
    lines: &mut Lines,
) -> Result<Prism<AstNode>, ParseError> {
    // Caller guarantees '(' exists (checked before dispatch).
    let paren = rest.find('(').expect("caller checked '(' exists");
    let name = rest[..paren].trim();
    let close = rest.find(')').ok_or_else(|| ParseError {
        message: format!("action: expected ')' in: action {}", rest),
        span: Some(span),
    })?;
    let params_str = &rest[paren + 1..close];
    let after_parens = rest[close + 1..].trim();

    // Parse "in @target { body }" or "in @target {\n  body\n}"
    let target_rest = after_parens.strip_prefix("in ").ok_or_else(|| ParseError {
        message: format!(
            "action: expected 'in @target' after params in: action {}",
            rest
        ),
        span: Some(span),
    })?;

    // Extract @target
    let brace_pos = target_rest.find('{').ok_or_else(|| ParseError {
        message: format!("action: expected '{{' in: action {}", rest),
        span: Some(span),
    })?;
    let target_name = target_rest[..brace_pos].trim().trim_start_matches('@');
    let after_brace = target_rest[brace_pos + 1..].trim();

    let vis_node = ast::ast_leaf(Kind::Atom, "visibility", visibility, span);
    let target_node = ast::ast_leaf(Kind::Atom, "target", target_name, span);
    let mut params = parse_action_params(params_str, span);

    // Collect body text
    let body_text = if after_brace.ends_with('}') {
        // Single-line: `in @rust { body }` or `in @rust {}`
        let body = after_brace.trim_end_matches('}').trim();
        lines.advance();
        body.to_string()
    } else {
        // Multi-line body
        lines.advance(); // consume the action header line
        let mut body_lines = Vec::new();
        if !after_brace.is_empty() {
            body_lines.push(after_brace.to_string());
        }
        let mut found_close = false;
        while let Some(line) = lines.peek() {
            let trimmed = line.trim();
            if trimmed == "}" {
                lines.advance();
                found_close = true;
                break;
            }
            body_lines.push(trimmed.to_string());
            lines.advance();
        }
        if !found_close {
            return Err(ParseError {
                message: "unclosed action body block".into(),
                span: Some(span),
            });
        }
        body_lines.join("\n")
    };

    let body_node = ast::ast_leaf(Kind::Atom, "body", &body_text, span);

    let mut children = vec![vis_node, target_node];
    children.append(&mut params);
    children.push(body_node);

    Ok(ast::ast_branch(
        Kind::Decl,
        "action-def",
        name,
        span,
        children,
    ))
}

/// Extract visibility modifier from an action line.
/// Returns `(rest_of_line, visibility)` if it matches, or `None`.
fn parse_action_visibility(line: &str) -> Option<(&str, &str)> {
    if let Some(rest) = line.strip_prefix("public action ") {
        Some((rest, "public"))
    } else if let Some(rest) = line.strip_prefix("protected action ") {
        Some((rest, "protected"))
    } else if let Some(rest) = line.strip_prefix("private action ") {
        Some((rest, "private"))
    } else if let Some(rest) = line.strip_prefix("action ") {
        Some((rest, "protected"))
    } else {
        None
    }
}

fn parse_grammar(header: &str, lines: &mut Lines) -> Result<Prism<AstNode>, ParseError> {
    let start_span = lines.current_span();

    // Extract @name and verify opening brace
    let (name_part, rest) = match header.split_once('{') {
        Some((n, r)) => (n.trim(), r),
        None => {
            return Err(ParseError {
                message: format!("grammar: expected '{{' in: grammar {}", header),
                span: Some(start_span),
            })
        }
    };

    // Parse optional extends clause from header
    let (name, extends_domains) = parse_extends_clause(name_part);

    // Build extends children as Ref nodes
    let extends_children: Vec<Prism<AstNode>> = extends_domains
        .iter()
        .map(|domain| ast::ast_leaf(Kind::Ref, "extends", *domain, start_span))
        .collect();

    // Check for single-line empty grammar: `grammar @name {}`
    if rest.trim() == "}" {
        lines.advance();
        return Ok(ast::ast_branch(
            Kind::Decl,
            "grammar",
            name,
            start_span,
            extends_children,
        ));
    }

    lines.advance(); // consume grammar header line

    let mut defs: Vec<Prism<AstNode>> = Vec::new();
    // Accumulate variants for the current type def (name, span, variants)
    let mut current: Option<(String, Span, Vec<Prism<AstNode>>)> = None;

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            // Flush any pending type def
            if let Some((type_name, type_span, variants)) = current.take() {
                defs.push(ast::ast_branch(
                    Kind::Form,
                    "type-def",
                    &*type_name,
                    type_span,
                    variants,
                ));
            }
            let end_span = lines.current_span();
            lines.advance();
            let span = start_span.merge(&end_span);
            let mut all_children = extends_children;
            all_children.append(&mut defs);
            return Ok(ast::ast_branch(
                Kind::Decl,
                "grammar",
                name,
                span,
                all_children,
            ));
        }

        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.advance();
            continue;
        }

        // Continuation line: starts with `|`
        if trimmed.starts_with('|') {
            if let Some((_, _, ref mut variants)) = current {
                let span = lines.current_span();
                variants.append(&mut parse_variants(trimmed, span));
            }
            lines.advance();
            continue;
        }

        // Abstract action: `abstract action name(params)` — signature only
        if let Some(rest) = trimmed.strip_prefix("abstract action ") {
            // Flush any pending type def
            if let Some((type_name, type_span, variants)) = current.take() {
                defs.push(ast::ast_branch(
                    Kind::Form,
                    "type-def",
                    &*type_name,
                    type_span,
                    variants,
                ));
            }
            let span = lines.current_span();
            defs.push(parse_abstract_action_decl(rest, span)?);
            lines.advance();
            continue;
        }

        // Action with visibility modifier: `public action read {`, etc.
        if let Some((rest, visibility)) = parse_action_visibility(trimmed) {
            // Flush any pending type def
            if let Some((type_name, type_span, variants)) = current.take() {
                defs.push(ast::ast_branch(
                    Kind::Form,
                    "type-def",
                    &*type_name,
                    type_span,
                    variants,
                ));
            }
            let span = lines.current_span();
            // Check if this is the new param/body form: name(params) in @target { body }
            let has_paren = rest.find('(');
            let has_brace = rest.find('{');
            if has_paren.is_some() && (has_brace.is_none() || has_paren < has_brace) {
                defs.push(parse_action_body_def(rest, visibility, span, lines)?);
            } else {
                defs.push(parse_action_def(rest, visibility, span, lines)?);
            }
            continue;
        }

        // Legacy `act name(params) {` keyword: treat as `action name {`.
        if let Some(act_rest) = trimmed.strip_prefix("act ") {
            if act_rest.contains('{') {
                // Flush any pending type def
                if let Some((type_name, type_span, variants)) = current.take() {
                    defs.push(ast::ast_branch(
                        Kind::Form,
                        "type-def",
                        &*type_name,
                        type_span,
                        variants,
                    ));
                }
                // Strip optional params: `enact(effect) {` → `enact {`
                let action_header = if let Some(paren) = act_rest.find('(') {
                    let name = act_rest[..paren].trim();
                    format!("{} {{", name)
                } else {
                    act_rest.to_string()
                };
                let span = lines.current_span();
                defs.push(parse_action_def(&action_header, "protected", span, lines)?);
                continue;
            }
        }

        // Property declarations: `requires name` or `invariant name`
        if let Some(prop_name) = trimmed.strip_prefix("requires ") {
            let span = lines.current_span();
            defs.push(ast::ast_leaf(
                Kind::Decl,
                "requires",
                prop_name.trim(),
                span,
            ));
            lines.advance();
            continue;
        }

        if let Some(prop_name) = trimmed.strip_prefix("invariant ") {
            let span = lines.current_span();
            defs.push(ast::ast_leaf(
                Kind::Decl,
                "invariant",
                prop_name.trim(),
                span,
            ));
            lines.advance();
            continue;
        }

        if let Some(prop_name) = trimmed.strip_prefix("ensures ") {
            let span = lines.current_span();
            defs.push(ast::ast_leaf(Kind::Decl, "ensures", prop_name.trim(), span));
            lines.advance();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("type ") {
            // Flush previous type def
            if let Some((type_name, type_span, variants)) = current.take() {
                defs.push(ast::ast_branch(
                    Kind::Form,
                    "type-def",
                    &*type_name,
                    type_span,
                    variants,
                ));
            }
            let span = lines.current_span();
            let (type_name, variants) = parse_type_def_parts(rest, span);
            current = Some((type_name, span, variants));
            lines.advance();
            continue;
        }

        lines.advance();
    }

    Err(ParseError {
        message: "unclosed grammar block".into(),
        span: Some(start_span),
    })
}

/// Parse a type definition into name + initial variants.
///
/// `= a | b | c` → name="" variants=[a,b,c]
/// `op = gt | lt` → name="op" variants=[gt,lt]
fn parse_type_def_parts(rest: &str, span: Span) -> (String, Vec<Prism<AstNode>>) {
    let (name, variants_text) = match rest.split_once('=') {
        Some((n, v)) => (n.trim(), v.trim()),
        None => ("", rest.trim()),
    };

    let variants = parse_variants(variants_text, span);
    (name.to_string(), variants)
}

/// Parse variants from a `|`-separated list.
///
/// Each segment is either:
/// - `name(param)` → Variant with TypeRef child
/// - `name` → Variant leaf
/// - empty → skipped (from leading `|` on continuation lines)
fn parse_variants(text: &str, span: Span) -> Vec<Prism<AstNode>> {
    text.split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|seg| {
            if let Some(paren) = seg.find('(') {
                let name = seg[..paren].trim();
                let param = seg[paren + 1..].trim_end_matches(')').trim();
                let type_ref = ast::ast_leaf(Kind::Ref, "type-ref", param, span);
                ast::ast_branch(Kind::Form, "variant", name, span, vec![type_ref])
            } else {
                ast::ast_leaf(Kind::Form, "variant", seg, span)
            }
        })
        .collect()
}

/// Parse an action definition block inside a grammar.
///
/// `send { from: address\n  to: address }` → Form("action-def", "send") with field children
/// `noop {}` → Form("action-def", "noop") with no children
///
/// The visibility parameter becomes an `Atom("visibility", vis)` child node,
/// inserted as the first child of the action-def Form.
fn parse_action_def(
    header: &str,
    visibility: &str,
    span: Span,
    lines: &mut Lines,
) -> Result<Prism<AstNode>, ParseError> {
    let (name, rest) = match header.split_once('{') {
        Some((n, r)) => (n.trim(), r.trim()),
        None => {
            return Err(ParseError {
                message: format!("action: expected '{{' in: action {}", header),
                span: Some(span),
            })
        }
    };

    let vis_node = ast::ast_leaf(Kind::Atom, "visibility", visibility, span);

    // Single-line empty: `action noop {}`
    if rest == "}" {
        lines.advance();
        return Ok(ast::ast_branch(
            Kind::Form,
            "action-def",
            name,
            span,
            vec![vis_node],
        ));
    }

    lines.advance(); // consume the action header line

    let mut fields: Vec<Prism<AstNode>> = vec![vis_node];

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            return Ok(ast::ast_branch(
                Kind::Form,
                "action-def",
                name,
                span.merge(&end_span),
                fields,
            ));
        }

        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.advance();
            continue;
        }

        let field_span = lines.current_span();

        // Action call: @domain.action(args...)
        if trimmed.starts_with('@') {
            if let Some(call_node) = parse_action_call(trimmed, field_span) {
                fields.push(call_node);
                lines.advance();
                continue;
            }
        }

        if let Some((fname, ftype)) = trimmed.split_once(':') {
            let fname = fname.trim();
            let ftype = ftype.trim();
            let type_ref = ast::ast_leaf(Kind::Ref, "type-ref", ftype, field_span);
            fields.push(ast::ast_branch(
                Kind::Atom,
                "field",
                fname,
                field_span,
                vec![type_ref],
            ));
        } else {
            fields.push(ast::ast_leaf(Kind::Atom, "field", trimmed, field_span));
        }

        lines.advance();
    }

    Err(ParseError {
        message: "unclosed action block".into(),
        span: Some(span),
    })
}

/// Parse `@domain.action(arg1, arg2)` into a Ref("action-call") node.
///
/// Returns None if the line doesn't match the pattern (falls through to field parsing).
fn parse_action_call(trimmed: &str, span: Span) -> Option<Prism<AstNode>> {
    // Find the '(' that separates target from arguments
    let paren_pos = trimmed.find('(')?;
    let target = &trimmed[..paren_pos]; // "@domain.action"

    // Must contain a dot separating domain from action name
    if !target.contains('.') {
        return None;
    }

    // Extract args between parens
    let rest = &trimmed[paren_pos + 1..];
    let close = rest.find(')')?;
    let args_str = rest[..close].trim();

    let arg_nodes: Vec<Prism<AstNode>> = if args_str.is_empty() {
        vec![]
    } else {
        args_str
            .split(',')
            .map(|a| ast::ast_leaf(Kind::Ref, "arg-ref", a.trim(), span))
            .collect()
    };

    Some(ast::ast_branch(
        Kind::Ref,
        "action-call",
        target,
        span,
        arg_nodes,
    ))
}

/// Parse `annotate(@domain)` — inline leaf or `annotate(@domain) { ... }` block.
fn parse_annotate(
    trimmed: &str,
    span: Span,
    lines: &mut Lines,
) -> Result<Prism<AstNode>, ParseError> {
    let inner_start = "annotate(".len();

    // Block form: annotate(@domain) { ... }
    if let Some(paren_end) = trimmed.find(") {") {
        let domain = trimmed[inner_start..paren_end].trim();
        lines.advance();

        let mut body_lines = Vec::new();
        while let Some(line) = lines.peek() {
            let t = line.trim();
            if t == "}" {
                lines.advance();
                break;
            }
            body_lines.push(line.to_string());
            lines.advance();
        }

        let body = body_lines.join("\n");
        let children = parse_test_ast(&body, span)?;
        return Ok(ast::ast_branch(
            Kind::Decl,
            "annotate",
            domain,
            span,
            children,
        ));
    }

    // Inline form: annotate(@domain)
    if trimmed.ends_with(')') {
        let domain = trimmed[inner_start..trimmed.len() - 1].trim();
        lines.advance();
        return Ok(ast::ast_leaf(Kind::Decl, "annotate", domain, span));
    }

    Err(ParseError {
        message: format!("invalid annotate syntax: {}", trimmed),
        span: Some(span),
    })
}

/// Collect all remaining lines from the iterator into a single string.
fn collect_remaining(lines: &mut Lines) -> String {
    let mut parts = Vec::new();
    while let Some(line) = lines.peek() {
        parts.push(line.to_string());
        lines.advance();
    }
    parts.join("\n")
}

/// Parse test section content into AST nodes.
///
/// Recognizes `test "name" { ... }`, `property "name" { ... }`,
/// and `generate @domain { ... }` directives.
fn parse_test_ast(source: &str, span: Span) -> Result<Vec<Prism<AstNode>>, ParseError> {
    let mut children = Vec::new();
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("test ") {
            let (name, body) = parse_directive_block(rest, &mut lines)?;
            let assertions = split_to_leaves(&body, "assertion", span);
            children.push(ast::ast_branch(Kind::Form, "test", &name, span, assertions));
        } else if let Some(rest) = trimmed.strip_prefix("property ") {
            let (name, body) = parse_directive_block(rest, &mut lines)?;
            let checks = split_to_leaves(&body, "check", span);
            children.push(ast::ast_branch(Kind::Form, "property", &name, span, checks));
        } else if let Some(rest) = trimmed.strip_prefix("generate ") {
            let (domain, body) = parse_generate_block(rest, &mut lines)?;
            let overrides = split_to_leaves(&body, "override", span);
            let domain_val = format!("@{}", domain);
            children.push(ast::ast_branch(
                Kind::Form,
                "generate",
                &domain_val,
                span,
                overrides,
            ));
        }
        // Unknown lines silently skipped
    }

    Ok(children)
}

/// Split semicolon-joined body text into leaf AST nodes.
fn split_to_leaves(body: &str, name: &str, span: Span) -> Vec<Prism<AstNode>> {
    body.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| ast::ast_leaf(Kind::Atom, name, s, span))
        .collect()
}

// -- Test section DSL --

/// A parsed directive from a test section (below `---`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TestDirective {
    /// `test "name" { @domain has variant; ... }`
    Test {
        name: String,
        assertions: Vec<HasAssertion>,
    },
    /// `property "name" { @domain preserves property_name }`
    Property {
        name: String,
        checks: Vec<PropertyCheck>,
    },
    /// `generate @domain { type = custom_variant; ... }`
    Generate {
        domain: String,
        overrides: Vec<(String, Vec<String>)>,
    },
}

/// `@domain has variant` or `@domain.type has variant`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HasAssertion {
    pub domain: String,
    pub type_name: Option<String>,
    pub variant: String,
}

/// `@domain preserves property_name`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyCheck {
    pub domain: String,
    pub property: String,
}

/// Parse a test section into directives.
pub fn parse_test_section(source: &str) -> Result<Vec<TestDirective>, ParseError> {
    let mut directives = Vec::new();
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("test ") {
            let (name, body) = parse_directive_block(rest, &mut lines)?;
            let assertions = parse_has_assertions(&body)?;
            directives.push(TestDirective::Test { name, assertions });
        } else if let Some(rest) = trimmed.strip_prefix("property ") {
            let (name, body) = parse_directive_block(rest, &mut lines)?;
            let checks = parse_property_checks(&body)?;
            directives.push(TestDirective::Property { name, checks });
        } else if let Some(rest) = trimmed.strip_prefix("generate ") {
            let (domain, body) = parse_generate_block(rest, &mut lines)?;
            let overrides = parse_generate_overrides(&body)?;
            directives.push(TestDirective::Generate { domain, overrides });
        }
        // Unknown lines are silently skipped (comments, blank, etc.)
    }

    Ok(directives)
}

/// Parse `"name" { ... }` — extracts the quoted name and brace-delimited body.
fn parse_directive_block(
    header: &str,
    lines: &mut std::iter::Peekable<std::str::Lines<'_>>,
) -> Result<(String, String), ParseError> {
    let header = header.trim();

    // Extract quoted name
    let name = if let Some(stripped) = header.strip_prefix('"') {
        let end = stripped.find('"').ok_or_else(|| ParseError {
            message: "unclosed quote in directive name".into(),
            span: None,
        })?;
        stripped[..end].to_string()
    } else {
        return Err(ParseError {
            message: "directive name must be quoted".into(),
            span: None,
        });
    };

    // Find opening brace — must be on same line as name
    let after_name = &header[name.len() + 2..].trim();
    let mut body = String::new();

    if let Some(rest) = after_name.strip_prefix('{') {
        if let Some(end) = rest.find('}') {
            body.push_str(rest[..end].trim());
            return Ok((name, body));
        }
        body.push_str(rest.trim());
    }

    // Multi-line: read until closing brace
    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed == "}" || trimmed.ends_with('}') {
            let before = trimmed.strip_suffix('}').unwrap_or("").trim();
            if !before.is_empty() {
                if !body.is_empty() {
                    body.push(';');
                }
                body.push_str(before);
            }
            return Ok((name, body));
        }
        if !trimmed.is_empty() {
            if !body.is_empty() {
                body.push(';');
            }
            body.push_str(trimmed);
        }
    }

    Err(ParseError {
        message: format!("unclosed block for \"{}\"", name),
        span: None,
    })
}

/// Parse `@domain { ... }` for generate blocks.
fn parse_generate_block(
    header: &str,
    lines: &mut std::iter::Peekable<std::str::Lines<'_>>,
) -> Result<(String, String), ParseError> {
    let header = header.trim();
    let domain_end = header
        .find(|c: char| c == '{' || c.is_whitespace())
        .unwrap_or(header.len());
    let raw_domain = header[..domain_end].trim();
    let domain = raw_domain
        .strip_prefix('@')
        .unwrap_or(raw_domain)
        .to_string();

    let rest = header[domain_end..].trim();
    let mut body = String::new();

    if let Some(inline) = rest.strip_prefix('{') {
        if let Some(end) = inline.find('}') {
            body.push_str(inline[..end].trim());
            return Ok((domain, body));
        }
        body.push_str(inline.trim());
    }

    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed == "}" || trimmed.ends_with('}') {
            let before = trimmed.strip_suffix('}').unwrap_or("").trim();
            if !before.is_empty() {
                if !body.is_empty() {
                    body.push(';');
                }
                body.push_str(before);
            }
            return Ok((domain, body));
        }
        if !trimmed.is_empty() {
            if !body.is_empty() {
                body.push(';');
            }
            body.push_str(trimmed);
        }
    }

    Err(ParseError {
        message: format!("unclosed generate block for @{}", domain),
        span: None,
    })
}

/// Parse `@domain has variant; @domain.type has variant` assertions.
fn parse_has_assertions(body: &str) -> Result<Vec<HasAssertion>, ParseError> {
    let mut assertions = Vec::new();
    for stmt in body.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }
        let parts: Vec<&str> = stmt.splitn(3, ' ').collect();
        if parts.len() < 3 || parts[1] != "has" {
            return Err(ParseError {
                message: format!("expected `@domain has variant`, got: {}", stmt),
                span: None,
            });
        }
        let target = parts[0].strip_prefix('@').unwrap_or(parts[0]);
        let (domain, type_name) = if let Some((d, t)) = target.split_once('.') {
            (d.to_string(), Some(t.to_string()))
        } else {
            (target.to_string(), None)
        };
        assertions.push(HasAssertion {
            domain,
            type_name,
            variant: parts[2].to_string(),
        });
    }
    Ok(assertions)
}

/// Parse `@domain preserves property_name` checks.
fn parse_property_checks(body: &str) -> Result<Vec<PropertyCheck>, ParseError> {
    let mut checks = Vec::new();
    for stmt in body.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }
        let parts: Vec<&str> = stmt.splitn(3, ' ').collect();
        if parts.len() < 3 || parts[1] != "preserves" {
            return Err(ParseError {
                message: format!("expected `@domain preserves property`, got: {}", stmt),
                span: None,
            });
        }
        let domain = parts[0].strip_prefix('@').unwrap_or(parts[0]).to_string();
        checks.push(PropertyCheck {
            domain,
            property: parts[2].to_string(),
        });
    }
    Ok(checks)
}

/// Parse `type = custom_variant; type op = custom` overrides.
fn parse_generate_overrides(body: &str) -> Result<Vec<(String, Vec<String>)>, ParseError> {
    let mut overrides = Vec::new();
    for stmt in body.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }
        let rest = stmt.strip_prefix("type").ok_or_else(|| ParseError {
            message: format!("expected `type = variant | ...`, got: {}", stmt),
            span: None,
        })?;
        let rest = rest.trim();
        let (type_name, variants_str) = if let Some((name, vs)) = rest.split_once('=') {
            (name.trim().to_string(), vs)
        } else {
            return Err(ParseError {
                message: format!("missing `=` in generate override: {}", stmt),
                span: None,
            });
        };
        let variants: Vec<String> = variants_str
            .split('|')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect();
        overrides.push((type_name, variants));
    }
    Ok(overrides)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

    /// Test helpers to extract TestDirective variants.
    impl TestDirective {
        fn as_test(&self) -> (&str, &[HasAssertion]) {
            match self {
                TestDirective::Test { name, assertions } => (name, assertions),
                _ => panic!("expected Test, got {:?}", self),
            }
        }
        fn as_property(&self) -> (&str, &[PropertyCheck]) {
            match self {
                TestDirective::Property { name, checks } => (name, checks),
                _ => panic!("expected Property, got {:?}", self),
            }
        }
        fn as_generate(&self) -> (&str, &[(String, Vec<String>)]) {
            match self {
                TestDirective::Generate { domain, overrides } => (domain, overrides),
                _ => panic!("expected Generate, got {:?}", self),
            }
        }
    }

    #[test]
    #[should_panic(expected = "expected Test")]
    fn as_test_panics_on_wrong_variant() {
        let d = TestDirective::Property {
            name: "x".into(),
            checks: vec![],
        };
        d.as_test();
    }

    #[test]
    #[should_panic(expected = "expected Property")]
    fn as_property_panics_on_wrong_variant() {
        let d = TestDirective::Test {
            name: "x".into(),
            assertions: vec![],
        };
        d.as_property();
    }

    #[test]
    #[should_panic(expected = "expected Generate")]
    fn as_generate_panics_on_wrong_variant() {
        let d = TestDirective::Test {
            name: "x".into(),
            assertions: vec![],
        };
        d.as_generate();
    }

    // -- Parse `in @domain` --

    #[test]
    fn parse_in_domain() {
        let source = "in @filesystem\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let children = tree.children();
        let in_node = children
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        assert!(in_node.is_shard());
        assert_eq!(in_node.data().value, "@filesystem");
    }

    // -- Parse `template $name { fields }` --

    #[test]
    fn parse_template_with_fields() {
        let source = "template $corpus {\n\tslug\n\texcerpt\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let children = tree.children();
        let tmpl = children
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        assert!(tmpl.is_fractal());
        assert_eq!(tmpl.data().value, "$corpus");
        assert_eq!(tmpl.children().len(), 2);
        assert_eq!(tmpl.children()[0].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[0].data().value, "slug");
        assert_eq!(tmpl.children()[1].data().value, "excerpt");
    }

    #[test]
    fn parse_field_with_qualifier() {
        let source = "template $t {\n\theadlines: h2\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().kind, Kind::Atom);
        assert_eq!(field.data().value, "headlines");
        assert!(field.is_fractal());
        assert_eq!(field.children()[0].data().kind, Kind::Atom);
        assert_eq!(field.children()[0].data().value, "h2");
    }

    #[test]
    fn parse_field_with_pipe() {
        let source = "template $t {\n\thtml: article | @html\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().value, "html");
        assert!(field.is_fractal());
        let children = field.children();
        assert_eq!(children[0].data().kind, Kind::Atom);
        assert_eq!(children[0].data().value, "article");
        assert_eq!(children[1].data().kind, Kind::Atom);
        assert_eq!(children[1].data().value, "@html");
    }

    // -- Parse `out name { ... }` --

    #[test]
    fn parse_out_with_group_and_selects() {
        let source = "out blog {\n\tpieces {\n\t\tdraft: 1draft { $corpus }\n\t}\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let out = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        assert_eq!(out.data().value, "blog");
        let group = &out.children()[0];
        assert_eq!(group.data().kind, Kind::Form);
        assert_eq!(group.data().value, "pieces");
        let select = &group.children()[0];
        assert_eq!(select.data().kind, Kind::Form);
        assert_eq!(select.data().value, "draft");
        assert_eq!(select.children().len(), 2);
        assert_eq!(select.children()[0].data().kind, Kind::Ref);
        assert_eq!(select.children()[0].data().value, "1draft");
        assert_eq!(select.children()[1].data().kind, Kind::Ref);
        assert_eq!(select.children()[1].data().value, "$corpus");
    }

    // -- Full file parse --

    #[test]
    fn parse_full_conv_file() {
        let source = include_str!("../systemic.engineering.conv").to_string();
        let tree = Parse.trace(source).unwrap();

        // Root has children: In, Template, Out
        let children = tree.children();
        let in_node = children.iter().find(|c| c.data().is_decl("in")).unwrap();
        assert_eq!(in_node.data().value, "@filesystem");

        let tmpl = children
            .iter()
            .find(|c| c.data().is_decl("template"))
            .unwrap();
        assert_eq!(tmpl.data().value, "$corpus");
        assert_eq!(tmpl.children().len(), 4); // slug, excerpt, headlines, html

        let out = children.iter().find(|c| c.data().is_decl("out")).unwrap();
        assert_eq!(out.data().value, "blog");
    }

    // -- Error paths --

    #[test]
    fn parse_without_output_succeeds() {
        // Parser is syntax only. Missing output is a resolver concern.
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        assert!(!tree.children().is_empty());
    }

    #[test]
    fn parse_error_unexpected_line() {
        let source = "garbage\n".to_string();
        let err = Parse.trace(source).into_result().unwrap_err();
        assert!(err.span.is_some(), "error should carry a span");
    }

    #[test]
    fn parse_error_unclosed_block() {
        let source = "out blog {\n\tpieces {\n".to_string();
        let err = Parse.trace(source).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "error should mention unclosed: {}",
            err
        );
    }

    // -- Spans are tracked --

    #[test]
    fn parse_spans_track_byte_offsets() {
        let source = "in @filesystem\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let in_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        // "in @filesystem" starts at byte 0
        assert_eq!(in_node.data().span.start, 0);
        assert!(in_node.data().span.end > 0);
    }

    // -- Display --

    #[test]
    fn parse_error_display_with_span() {
        let err = ParseError {
            message: "bad".into(),
            span: Some(Span::new(5, 10)),
        };
        assert_eq!(format!("{}", err), "parse error at 5..10: bad");
    }

    #[test]
    fn parse_error_display_without_span() {
        let err = ParseError {
            message: "bad".into(),
            span: None,
        };
        assert_eq!(format!("{}", err), "parse error: bad");
    }

    // -- Coverage: empty group, unclosed template, unexpected output --

    #[test]
    fn parse_empty_group() {
        let source = "out root {\n\tempty {}\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let out = &tree.children()[0];
        let group = &out.children()[0];
        assert_eq!(group.data().kind, Kind::Form);
        assert_eq!(group.data().value, "empty");
        assert_eq!(group.children().len(), 0);
    }

    #[test]
    fn parse_error_unclosed_template() {
        let source = "template $t {\n\tslug\n".to_string();
        let err = Parse.trace(source).into_result().unwrap_err();
        assert!(err.message.contains("unclosed"), "{}", err);
    }

    #[test]
    fn parse_bare_expr_in_output() {
        let source = "out root {\n\tnonsense\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let out = &tree.children()[0];
        let expr = &out.children()[0];
        assert_eq!(expr.data().kind, Kind::Atom);
        assert_eq!(expr.data().value, "nonsense");
    }

    #[test]
    fn parse_blank_lines_and_comments_skipped() {
        let source = "# comment\n\n# another\nin @fs\n".to_string();
        let tree = Parse.trace(source).unwrap();
        assert_eq!(tree.children().len(), 1);
        assert_eq!(tree.children()[0].data().value, "@fs");
    }

    #[test]
    fn parse_template_with_blank_lines() {
        let source = "template $t {\n\n\tslug\n\n\texcerpt\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.children().len(), 2);
    }

    #[test]
    fn parse_out_with_blank_lines() {
        let source = "out r {\n\n\tg {\n\n\t\tx: f { $t }\n\n\t}\n\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.children().len(), 1);
    }

    #[test]
    fn parse_json_fixture() {
        let source = include_str!("../fixtures/json.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.data().kind, Kind::Decl);
        assert_eq!(out.data().value, "@json");
        assert!(out.is_shard());
    }

    #[test]
    fn parse_parameterized_in() {
        let source = "in @git(branch: \"main\")\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let in_node = &tree.children()[0];
        assert_eq!(in_node.data().kind, Kind::Decl);
        assert_eq!(in_node.data().value, "@git");
        assert!(in_node.is_fractal());
        assert_eq!(in_node.children()[0].data().kind, Kind::Ref);
        assert_eq!(in_node.children()[0].data().value, "branch: \"main\"");
    }

    #[test]
    fn parse_coverage_fixture() {
        let source = include_str!("../fixtures/coverage-on-last-3-main-commits.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children.len(), 2); // in + pipeline

        let in_node = &children[0];
        assert_eq!(in_node.data().kind, Kind::Decl);
        assert_eq!(in_node.data().value, "@git");
        assert_eq!(in_node.children()[0].data().kind, Kind::Ref);
        assert_eq!(in_node.children()[0].data().value, "branch: \"main\"");

        let pipeline = &children[1];
        assert_eq!(pipeline.data().kind, Kind::Form);
        assert_eq!(pipeline.children().len(), 2);
    }

    #[test]
    fn parse_bare_out() {
        let source = "out @json\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.data().kind, Kind::Decl);
        assert_eq!(out.data().value, "@json");
        assert!(out.is_shard());
    }

    #[test]
    fn parse_empty_source() {
        let source = "".to_string();
        let tree = Parse.trace(source).unwrap();
        assert_eq!(tree.children().len(), 0);
    }

    #[test]
    fn parse_field_expr_in_output() {
        let source = "out root {\n\tlabel: value\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let out = &tree.children()[0];
        let field = &out.children()[0];
        assert_eq!(field.data().kind, Kind::Atom);
        assert_eq!(field.data().value, "label");
        assert_eq!(field.children()[0].data().kind, Kind::Atom);
        assert_eq!(field.children()[0].data().value, "value");
    }

    // -- Pipeline: A | G | B --

    #[test]
    fn parse_commit_from_main_to_test_fixture() {
        let source = include_str!("../fixtures/commit-from-main-to-test.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Form);
        assert_eq!(pipeline.children().len(), 3);
    }

    #[test]
    fn parse_additive_fixture() {
        let source = include_str!("../fixtures/additive.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children.len(), 3); // two ins + one out

        assert_eq!(children[0].data().kind, Kind::Decl);
        assert_eq!(children[0].data().value, "@number");
        assert_eq!(children[0].children()[0].data().value, "$a");

        assert_eq!(children[1].data().kind, Kind::Decl);
        assert_eq!(children[1].data().value, "@number");
        assert_eq!(children[1].children()[0].data().value, "$b");

        let out = &children[2];
        assert_eq!(out.data().kind, Kind::Decl);
        assert_eq!(out.data().value, "");
        assert_eq!(out.children().len(), 3);
    }

    #[test]
    fn parse_aliased_in() {
        let source = "in @number as $a\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let in_node = &tree.children()[0];
        assert_eq!(in_node.data().kind, Kind::Decl);
        assert_eq!(in_node.data().value, "@number");
        assert!(in_node.is_fractal());
        assert_eq!(in_node.children()[0].data().kind, Kind::Ref);
        assert_eq!(in_node.children()[0].data().value, "$a");
    }

    #[test]
    fn parse_anonymous_out_with_exprs() {
        let source = "out {\n\tsimple: $a + $b\n\tcurried + $b\n\tmagic: +\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.data().kind, Kind::Decl);
        assert_eq!(out.data().value, "");
        assert_eq!(out.children().len(), 3);

        let simple = &out.children()[0];
        assert_eq!(simple.data().kind, Kind::Atom);
        assert_eq!(simple.data().value, "simple");
        assert_eq!(simple.children()[0].data().kind, Kind::Atom);
        assert_eq!(simple.children()[0].data().value, "$a + $b");

        let curried = &out.children()[1];
        assert_eq!(curried.data().kind, Kind::Atom);
        assert_eq!(curried.data().value, "curried + $b");

        let magic = &out.children()[2];
        assert_eq!(magic.data().kind, Kind::Atom);
        assert_eq!(magic.data().value, "magic");
        assert_eq!(magic.children()[0].data().kind, Kind::Atom);
        assert_eq!(magic.children()[0].data().value, "+");
    }

    // -- Parse `use` imports --

    #[test]
    fn parse_use_single() {
        let source = "use $corpus from @shared\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let children = tree.children();
        let use_node = children
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        assert!(use_node.is_fractal());
        assert_eq!(use_node.data().value, "use");
        let use_children = use_node.children();
        assert_eq!(use_children.len(), 2);
        assert_eq!(use_children[0].data().kind, Kind::Ref);
        assert_eq!(use_children[0].data().value, "$corpus");
        assert_eq!(use_children[1].data().kind, Kind::Ref);
        assert_eq!(use_children[1].data().value, "@shared");
    }

    #[test]
    fn parse_use_destructured() {
        let source = "use { $a, $b } from @other\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let use_children = use_node.children();
        assert_eq!(use_children.len(), 3); // $a, $b, @other
        assert_eq!(use_children[0].data().kind, Kind::Ref);
        assert_eq!(use_children[0].data().value, "$a");
        assert_eq!(use_children[1].data().kind, Kind::Ref);
        assert_eq!(use_children[1].data().value, "$b");
        assert_eq!(use_children[2].data().kind, Kind::Ref);
        assert_eq!(use_children[2].data().value, "@other");
    }

    #[test]
    fn parse_use_with_sha_lock() {
        let source = "use $garden from @garden@systemic.engineering sha: ABC123\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let use_children = use_node.children();
        // $garden, @garden@systemic.engineering, sha param
        assert_eq!(use_children[0].data().kind, Kind::Ref);
        assert_eq!(use_children[0].data().value, "$garden");
        assert_eq!(use_children[1].data().kind, Kind::Ref);
        assert_eq!(use_children[1].data().value, "@garden@systemic.engineering");
        assert_eq!(use_children[2].data().kind, Kind::Ref);
        assert_eq!(use_children[2].data().value, "sha: ABC123");
    }

    #[test]
    fn parse_use_without_from() {
        // Malformed: no "from" keyword — still parses, just has no domain ref
        let source = "use $orphan\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let use_children = use_node.children();
        assert_eq!(use_children.len(), 1);
        assert_eq!(use_children[0].data().kind, Kind::Ref);
        assert_eq!(use_children[0].data().value, "$orphan");
    }

    #[test]
    fn parse_use_appears_before_template() {
        let source =
            "use $t from @shared\ntemplate $local {\n\tslug\n}\nout r {\n\tx: f { $local }\n}\n"
                .to_string();
        let tree = Parse.trace(source).unwrap();
        let children = tree.children();
        assert_eq!(children[0].data().kind, Kind::Decl);
        assert_eq!(children[1].data().kind, Kind::Decl);
        assert_eq!(children[2].data().kind, Kind::Decl);
    }

    // -- Parse `use` with path expressions --

    #[test]
    fn parse_use_home_path() {
        let source = "use $t from $HOME/shared\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let ch = use_node.children();
        // [TemplateRef($t), Home, Path(shared)]
        assert_eq!(ch.len(), 3);
        assert_eq!(ch[0].data().kind, Kind::Ref);
        assert_eq!(ch[0].data().value, "$t");
        assert_eq!(ch[1].data().kind, Kind::Ref);
        assert_eq!(ch[2].data().kind, Kind::Atom);
        assert_eq!(ch[2].data().value, "shared");
    }

    #[test]
    fn parse_use_self_path() {
        let source = "use $t from $SELF/templates\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let ch = use_node.children();
        // [TemplateRef($t), Self_, Path(templates)]
        assert_eq!(ch.len(), 3);
        assert_eq!(ch[0].data().kind, Kind::Ref);
        assert_eq!(ch[0].data().value, "$t");
        assert_eq!(ch[1].data().kind, Kind::Ref);
        assert_eq!(ch[2].data().kind, Kind::Atom);
        assert_eq!(ch[2].data().value, "templates");
    }

    #[test]
    fn parse_use_dot_slash() {
        let source = "use $t from ./templates\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let ch = use_node.children();
        // ./templates desugars to Self_ + Path(templates)
        assert_eq!(ch.len(), 3);
        assert_eq!(ch[0].data().kind, Kind::Ref);
        assert_eq!(ch[0].data().value, "$t");
        assert_eq!(ch[1].data().kind, Kind::Ref);
        assert_eq!(ch[2].data().kind, Kind::Atom);
        assert_eq!(ch[2].data().value, "templates");
    }

    #[test]
    fn parse_use_namespace_path() {
        let source = "use $t from @X/sub\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let ch = use_node.children();
        // [TemplateRef($t), DomainRef(@X), Path(sub)]
        assert_eq!(ch.len(), 3);
        assert_eq!(ch[0].data().kind, Kind::Ref);
        assert_eq!(ch[0].data().value, "$t");
        assert_eq!(ch[1].data().kind, Kind::Ref);
        assert_eq!(ch[1].data().value, "@X");
        assert_eq!(ch[2].data().kind, Kind::Atom);
        assert_eq!(ch[2].data().value, "sub");
    }

    #[test]
    fn parse_use_deep_path() {
        let source = "use $t from $HOME/a/b/c\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let ch = use_node.children();
        // [TemplateRef($t), Home, Path(a), Path(b), Path(c)]
        assert_eq!(ch.len(), 5);
        assert_eq!(ch[0].data().kind, Kind::Ref);
        assert_eq!(ch[1].data().kind, Kind::Ref);
        assert_eq!(ch[2].data().kind, Kind::Atom);
        assert_eq!(ch[2].data().value, "a");
        assert_eq!(ch[3].data().kind, Kind::Atom);
        assert_eq!(ch[3].data().value, "b");
        assert_eq!(ch[4].data().kind, Kind::Atom);
        assert_eq!(ch[4].data().value, "c");
    }

    #[test]
    fn parse_use_bare_source() {
        let source = "use $t from bare_name\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Decl)
            .unwrap();
        let ch = use_node.children();
        // [TemplateRef($t), DomainRef(bare_name)]
        assert_eq!(ch.len(), 2);
        assert_eq!(ch[0].data().kind, Kind::Ref);
        assert_eq!(ch[0].data().value, "$t");
        assert_eq!(ch[1].data().kind, Kind::Ref);
        assert_eq!(ch[1].data().value, "bare_name");
    }

    // -- Parse `when` predicates --

    #[test]
    fn parse_when_greater_than() {
        let node = parse_when("error.rate > 0.1", Span::new(0, 20)).unwrap();
        assert!(node.data().is_decl("when/gt"));
        assert!(node.children()[0].data().is_atom("path"));
        assert_eq!(node.children()[0].data().value, "error.rate");
        assert!(node.children()[1].data().is_atom("literal"));
        assert_eq!(node.children()[1].data().value, "0.1");
    }

    #[test]
    fn parse_when_equals() {
        let node = parse_when("status == \"active\"", Span::new(0, 20)).unwrap();
        assert!(node.data().is_decl("when/eq"));
        assert_eq!(node.children()[0].data().value, "status");
        assert_eq!(node.children()[1].data().value, "\"active\"");
    }

    #[test]
    fn parse_when_less_equal() {
        let node = parse_when("count <= 10", Span::new(0, 15)).unwrap();
        assert!(node.data().is_decl("when/lte"));
        assert_eq!(node.children()[0].data().value, "count");
        assert_eq!(node.children()[1].data().value, "10");
    }

    #[test]
    fn parse_when_not_equal() {
        let node = parse_when("mode != \"debug\"", Span::new(0, 20)).unwrap();
        assert!(node.data().is_decl("when/ne"));
        assert_eq!(node.children()[0].data().value, "mode");
        assert_eq!(node.children()[1].data().value, "\"debug\"");
    }

    #[test]
    fn parse_when_dotted_path() {
        let node = parse_when("error.rate.current > 0.5", Span::new(0, 25)).unwrap();
        assert!(node.data().is_decl("when/gt"));
        assert!(node.children()[0].data().is_atom("path"));
        assert_eq!(node.children()[0].data().value, "error.rate.current");
        assert_eq!(node.children()[1].data().value, "0.5");
    }

    #[test]
    fn parse_when_appears_between_in_and_out() {
        let source = "in @datadog\nwhen error.rate > 0.1\nout r {\n\tx {}\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        let when_node = children
            .iter()
            .find(|c| c.data().name.starts_with("when/"))
            .unwrap();
        assert!(when_node.data().is_decl("when/gt"));
    }

    #[test]
    fn parse_bare_field_with_pipe() {
        // "slug | @sha" in a template: bare field (no colon) with a pipe
        let source = "template $t {\n\tslug | @sha\n}\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().kind, Kind::Atom);
        assert_eq!(field.data().value, "slug");
        assert!(field.is_fractal());
        assert_eq!(field.children()[0].data().kind, Kind::Atom);
        assert_eq!(field.children()[0].data().value, "@sha");
    }

    #[test]
    fn parse_when_no_operator_errors() {
        let err = parse_when("active", Span::new(0, 6)).unwrap_err();
        assert!(err.message.contains("active"), "{}", err.message);
    }

    #[test]
    fn parse_when_no_operator_propagates_parse_error() {
        let err = Parse
            .trace("when active\n".to_string())
            .into_result()
            .unwrap_err();
        assert!(err.message.contains("active"), "{}", err.message);
    }

    #[test]
    fn parse_pipeline_bare_domain() {
        let source = "@fs | data | @json\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Form);
        assert_eq!(pipeline.children().len(), 3);

        let left = &pipeline.children()[0];
        assert_eq!(left.data().kind, Kind::Ref);
        assert_eq!(left.data().value, "@fs");
        assert!(left.children().is_empty());

        let mid = &pipeline.children()[1];
        assert_eq!(mid.data().kind, Kind::Ref);
        assert_eq!(mid.data().value, "data");

        let right = &pipeline.children()[2];
        assert_eq!(right.data().kind, Kind::Ref);
        assert_eq!(right.data().value, "@json");
        assert!(right.children().is_empty());
    }

    #[test]
    fn parse_pipeline() {
        let source = "@git(branch: \"master\") | HEAD | @git(branch: \"test\")\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Form);
        assert_eq!(pipeline.children().len(), 3);

        let left = &pipeline.children()[0];
        assert_eq!(left.data().kind, Kind::Ref);
        assert_eq!(left.data().value, "@git");
        assert_eq!(left.children().len(), 1);
        assert_eq!(left.children()[0].data().kind, Kind::Ref);
        assert_eq!(left.children()[0].data().value, "branch: \"master\"");

        let mid = &pipeline.children()[1];
        assert_eq!(mid.data().kind, Kind::Ref);
        assert_eq!(mid.data().value, "HEAD");

        let right = &pipeline.children()[2];
        assert_eq!(right.data().kind, Kind::Ref);
        assert_eq!(right.data().value, "@git");
        assert_eq!(right.children().len(), 1);
        assert_eq!(right.children()[0].data().kind, Kind::Ref);
        assert_eq!(right.children()[0].data().value, "branch: \"test\"");
    }

    // -- Parse `case` dispatch --

    #[test]
    fn parse_case_single_arm() {
        let node = parse_case("x {", &mut Lines::new("x {\n  > 1 -> a\n}\n")).unwrap();
        assert!(node.data().is_decl("case"));
        assert_eq!(node.data().value, "x");
        assert_eq!(node.children().len(), 1);
        let arm = &node.children()[0];
        assert!(arm.data().is_form("arm"));
        assert_eq!(arm.children().len(), 2);
        assert!(arm.children()[0].data().is_atom("cmp/gt"));
        assert_eq!(arm.children()[0].data().value, "1");
        assert!(arm.children()[1].data().is_atom("expr"));
        assert_eq!(arm.children()[1].data().value, "a");
    }

    #[test]
    fn parse_case_wildcard_arm() {
        let node = parse_case("x {", &mut Lines::new("x {\n  _ -> b\n}\n")).unwrap();
        assert_eq!(node.children().len(), 1);
        let arm = &node.children()[0];
        assert!(arm.data().is_form("arm"));
        assert!(arm.children()[0].data().is_atom("wild"));
        assert!(arm.children()[1].data().is_atom("expr"));
        assert_eq!(arm.children()[1].data().value, "b");
    }

    #[test]
    fn parse_case_multiple_arms() {
        let source =
            "case error.rate {\n  > 0.1 -> critical\n  > 0.05 -> warning\n  _ -> pass\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let case_node = &tree.children()[0];
        assert!(case_node.data().is_decl("case"));
        assert_eq!(case_node.data().value, "error.rate");
        assert_eq!(case_node.children().len(), 3);

        let arm0 = &case_node.children()[0];
        assert!(arm0.children()[0].data().is_atom("cmp/gt"));
        assert_eq!(arm0.children()[0].data().value, "0.1");
        assert_eq!(arm0.children()[1].data().value, "critical");

        let arm1 = &case_node.children()[1];
        assert!(arm1.children()[0].data().is_atom("cmp/gt"));
        assert_eq!(arm1.children()[0].data().value, "0.05");
        assert_eq!(arm1.children()[1].data().value, "warning");

        let arm2 = &case_node.children()[2];
        assert!(arm2.children()[0].data().is_atom("wild"));
        assert_eq!(arm2.children()[1].data().value, "pass");
    }

    #[test]
    fn parse_case_all_cmp_ops() {
        let source = "case x {\n  > 1 -> a\n  < 2 -> b\n  >= 3 -> c\n  <= 4 -> d\n  == 5 -> e\n  != 6 -> f\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let case_node = &tree.children()[0];
        assert_eq!(case_node.children().len(), 6);
        assert!(case_node.children()[0].children()[0]
            .data()
            .is_atom("cmp/gt"));
        assert!(case_node.children()[1].children()[0]
            .data()
            .is_atom("cmp/lt"));
        assert!(case_node.children()[2].children()[0]
            .data()
            .is_atom("cmp/gte"));
        assert!(case_node.children()[3].children()[0]
            .data()
            .is_atom("cmp/lte"));
        assert!(case_node.children()[4].children()[0]
            .data()
            .is_atom("cmp/eq"));
        assert!(case_node.children()[5].children()[0]
            .data()
            .is_atom("cmp/ne"));
    }

    #[test]
    fn parse_case_appears_in_source() {
        let source =
            "in @datadog\ncase error.rate {\n  > 0.1 -> alert\n  _ -> pass\n}\nout @json\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert!(children[0].data().is_decl("in"));
        assert!(children[1].data().is_decl("case"));
        assert!(children[2].data().is_decl("out"));
    }

    #[test]
    fn parse_case_error_no_arrow() {
        let source = "case x {\n  > 1 oops\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("->"),
            "expected mention of '->': {}",
            err.message
        );
    }

    #[test]
    fn parse_case_error_no_operator() {
        let source = "case x {\n  1 -> a\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("operator") || err.message.contains("1"),
            "expected mention of operator: {}",
            err.message
        );
    }

    #[test]
    fn parse_case_with_blank_lines() {
        let source = "case x {\n\n  > 1 -> a\n\n  _ -> b\n\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let case_node = &tree.children()[0];
        assert_eq!(case_node.data().kind, Kind::Decl);
        assert_eq!(case_node.children().len(), 2);
    }

    #[test]
    fn parse_case_error_unclosed() {
        let source = "case x {\n  > 1 -> a\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "expected 'unclosed': {}",
            err.message
        );
    }

    // -- Parse `branch(.path) { ... }` — value dispatch --

    #[test]
    fn parse_branch_single_arm() {
        let source = "branch(.action) {\n  \"hold\" => ..\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Decl);
        assert_eq!(branch.data().value, ".action");
        assert_eq!(branch.children().len(), 1);
        let arm = &branch.children()[0];
        assert_eq!(arm.data().kind, Kind::Form);
        assert_eq!(arm.children()[0].data().kind, Kind::Atom);
        assert_eq!(arm.children()[0].data().value, "hold");
        assert_eq!(arm.children()[1].data().kind, Kind::Atom);
        assert_eq!(arm.children()[1].data().value, "..");
    }

    #[test]
    fn parse_branch_multiple_arms() {
        let source = "branch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Decl);
        assert_eq!(branch.data().value, ".action");
        assert_eq!(branch.children().len(), 2);

        let arm0 = &branch.children()[0];
        assert_eq!(arm0.children()[0].data().kind, Kind::Atom);
        assert_eq!(arm0.children()[0].data().value, "hold");
        assert_eq!(arm0.children()[1].data().value, "..");

        let arm1 = &branch.children()[1];
        assert_eq!(arm1.children()[0].data().kind, Kind::Atom);
        assert_eq!(arm1.children()[0].data().value, "exit");
        assert_eq!(arm1.children()[1].data().value, "exit");
    }

    #[test]
    fn parse_branch_wildcard_arm() {
        let source = "branch(.status) {\n  \"ok\" => ..\n  _ => exit\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Decl);
        assert_eq!(branch.children().len(), 2);
        let wild_arm = &branch.children()[1];
        assert_eq!(wild_arm.children()[0].data().kind, Kind::Atom);
        assert_eq!(wild_arm.children()[1].data().value, "exit");
    }

    #[test]
    fn parse_branch_in_pipeline() {
        let source = "@json | branch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Form);
        assert_eq!(pipeline.children().len(), 2);

        let domain = &pipeline.children()[0];
        assert_eq!(domain.data().kind, Kind::Ref);
        assert_eq!(domain.data().value, "@json");

        let branch = &pipeline.children()[1];
        assert_eq!(branch.data().kind, Kind::Decl);
        assert_eq!(branch.data().value, ".action");
        assert_eq!(branch.children().len(), 2);
    }

    #[test]
    fn parse_branch_with_blank_lines() {
        let source = "branch(.x) {\n\n  \"a\" => ..\n\n  \"b\" => exit\n\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Decl);
        assert_eq!(branch.children().len(), 2);
    }

    #[test]
    fn parse_branch_error_unclosed() {
        let source = "branch(.x) {\n  \"a\" => ..\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "expected 'unclosed': {}",
            err.message
        );
    }

    #[test]
    fn parse_branch_error_no_arrow() {
        let source = "branch(.x) {\n  \"a\" oops\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("=>"),
            "expected mention of '=>': {}",
            err.message
        );
    }

    #[test]
    fn parse_branch_error_missing_paren() {
        let source = "branch(.x {\n  \"a\" => ..\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains(")") || err.message.contains("paren"),
            "expected paren error: {}",
            err.message
        );
    }

    #[test]
    fn parse_branch_error_invalid_pattern() {
        let source = "branch(.x) {\n  bare => ..\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("quoted string") || err.message.contains("pattern"),
            "expected pattern error: {}",
            err.message
        );
    }

    #[test]
    fn parse_branch_appears_in_source() {
        let source =
            "in @json\nbranch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\nout @json\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children[0].data().kind, Kind::Decl);
        assert_eq!(children[1].data().kind, Kind::Decl);
        assert_eq!(children[2].data().kind, Kind::Decl);
    }

    // -- Parse `grammar @name { ... }` — vocabulary declaration --

    #[test]
    fn parse_grammar_empty() {
        let source = "grammar @test {\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@test");
        assert_eq!(grammar.children().len(), 0);
    }

    #[test]
    fn parse_grammar_empty_single_line() {
        let source = "grammar @test {}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@test");
        assert_eq!(grammar.children().len(), 0);
    }

    #[test]
    fn parse_grammar_single_type() {
        let source = "grammar @test {\n  type = a | b | c\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@test");
        assert_eq!(grammar.children().len(), 1);

        let typedef = &grammar.children()[0];
        assert_eq!(typedef.data().kind, Kind::Form);
        assert_eq!(typedef.data().value, "");
        assert_eq!(typedef.children().len(), 3);
        assert_eq!(typedef.children()[0].data().kind, Kind::Form);
        assert_eq!(typedef.children()[0].data().value, "a");
        assert_eq!(typedef.children()[1].data().value, "b");
        assert_eq!(typedef.children()[2].data().value, "c");
    }

    #[test]
    fn parse_grammar_named_type() {
        let source = "grammar @test {\n  type op = gt | lt\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let typedef = &grammar.children()[0];
        assert_eq!(typedef.data().kind, Kind::Form);
        assert_eq!(typedef.data().value, "op");
        assert_eq!(typedef.children().len(), 2);
        assert_eq!(typedef.children()[0].data().value, "gt");
        assert_eq!(typedef.children()[1].data().value, "lt");
    }

    #[test]
    fn parse_grammar_parameterized() {
        let source = "grammar @test {\n  type = when(op) | plain\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let typedef = &grammar.children()[0];
        assert_eq!(typedef.children().len(), 2);

        let when_variant = &typedef.children()[0];
        assert_eq!(when_variant.data().kind, Kind::Form);
        assert_eq!(when_variant.data().value, "when");
        assert_eq!(when_variant.children().len(), 1);
        assert_eq!(when_variant.children()[0].data().kind, Kind::Ref);
        assert_eq!(when_variant.children()[0].data().value, "op");

        let plain = &typedef.children()[1];
        assert_eq!(plain.data().kind, Kind::Form);
        assert_eq!(plain.data().value, "plain");
        assert_eq!(plain.children().len(), 0);
    }

    #[test]
    fn parse_grammar_continuation() {
        let source = "grammar @test {\n  type = a | b\n       | c | d\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let typedef = &grammar.children()[0];
        assert_eq!(typedef.children().len(), 4);
        assert_eq!(typedef.children()[0].data().value, "a");
        assert_eq!(typedef.children()[1].data().value, "b");
        assert_eq!(typedef.children()[2].data().value, "c");
        assert_eq!(typedef.children()[3].data().value, "d");
    }

    #[test]
    fn parse_grammar_multiple_types() {
        let source = "grammar @test {\n  type = a | when(op)\n\n  type op = gt | lt\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 2);

        let root_type = &grammar.children()[0];
        assert_eq!(root_type.data().kind, Kind::Form);
        assert_eq!(root_type.data().value, "");
        assert_eq!(root_type.children().len(), 2);

        let sub_type = &grammar.children()[1];
        assert_eq!(sub_type.data().kind, Kind::Form);
        assert_eq!(sub_type.data().value, "op");
        assert_eq!(sub_type.children().len(), 2);
    }

    #[test]
    fn parse_grammar_comments_blanks() {
        let source = "grammar @test {\n  # a comment\n\n  type = x\n  # another\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);
        assert_eq!(grammar.children()[0].data().kind, Kind::Form);
    }

    #[test]
    fn parse_grammar_skips_unknown_lines() {
        let source = "grammar @test {\n  unknown line\n  type = a\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);
        assert_eq!(grammar.children()[0].data().kind, Kind::Form);
    }

    #[test]
    fn parse_grammar_type_without_equals() {
        let source = "grammar @test {\n  type a | b | c\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);
        let typedef = &grammar.children()[0];
        assert_eq!(typedef.data().kind, Kind::Form);
        assert_eq!(typedef.data().value, "");
        // Without '=', the whole rest is parsed as variant list
        assert_eq!(typedef.children().len(), 3);
    }

    #[test]
    fn parse_grammar_in_source() {
        let source = "in @test\ngrammar @test {\n  type = a\n}\nout @json\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children[0].data().kind, Kind::Decl);
        assert_eq!(children[1].data().kind, Kind::Decl);
        assert_eq!(children[2].data().kind, Kind::Decl);
    }

    #[test]
    fn parse_grammar_conversation_litmus() {
        let source = "\
grammar @conversation {
  type = in | out | template | field | qualifier | pipe
       | group | select | template-ref | domain-ref
       | pipeline | domain-param | ref | alias | expr
       | use | home | self | path | literal
       | case | arm | wild | branch | param
       | when(op) | cmp(op)

  type op = gt | lt | gte | lte | eq | ne
}
";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@conversation");
        assert_eq!(grammar.children().len(), 2);

        let root_type = &grammar.children()[0];
        assert_eq!(root_type.data().kind, Kind::Form);
        assert_eq!(root_type.data().value, "");
        // 25 simple + 2 parameterized = 27 variants
        assert_eq!(root_type.children().len(), 27);

        // Check parameterized variants
        let when_v = root_type
            .children()
            .iter()
            .find(|c| c.data().value == "when")
            .unwrap();
        assert_eq!(when_v.children().len(), 1);
        assert_eq!(when_v.children()[0].data().kind, Kind::Ref);
        assert_eq!(when_v.children()[0].data().value, "op");

        let cmp_v = root_type
            .children()
            .iter()
            .find(|c| c.data().value == "cmp")
            .unwrap();
        assert_eq!(cmp_v.children().len(), 1);
        assert_eq!(cmp_v.children()[0].data().kind, Kind::Ref);
        assert_eq!(cmp_v.children()[0].data().value, "op");

        let op_type = &grammar.children()[1];
        assert_eq!(op_type.data().kind, Kind::Form);
        assert_eq!(op_type.data().value, "op");
        assert_eq!(op_type.children().len(), 6);
    }

    #[test]
    fn parse_grammar_error_unclosed() {
        let source = "grammar @test {\n  type = a\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "expected 'unclosed': {}",
            err.message
        );
    }

    #[test]
    fn parse_grammar_error_no_brace() {
        let source = "grammar @test\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("{"),
            "expected mention of '{{': {}",
            err.message
        );
    }

    // -- Parse `action` in grammar blocks --

    #[test]
    fn parse_grammar_action_single() {
        let source =
            "grammar @test {\n  action send {\n    from: address\n    to: address\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);

        let action = &grammar.children()[0];
        assert_eq!(action.data().kind, Kind::Form);
        assert_eq!(action.data().name, "action-def");
        assert_eq!(action.data().value, "send");
        assert_eq!(action.children().len(), 3);

        assert_eq!(action.children()[0].data().name, "visibility");
        assert_eq!(action.children()[0].data().value, "protected");

        let from = &action.children()[1];
        assert_eq!(from.data().kind, Kind::Atom);
        assert_eq!(from.data().name, "field");
        assert_eq!(from.data().value, "from");
        assert_eq!(from.children().len(), 1);
        assert_eq!(from.children()[0].data().kind, Kind::Ref);
        assert_eq!(from.children()[0].data().name, "type-ref");
        assert_eq!(from.children()[0].data().value, "address");

        let to = &action.children()[2];
        assert_eq!(to.data().kind, Kind::Atom);
        assert_eq!(to.data().name, "field");
        assert_eq!(to.data().value, "to");
        assert_eq!(to.children()[0].data().value, "address");
    }

    #[test]
    fn parse_grammar_action_untyped_field() {
        let source = "grammar @test {\n  action send {\n    subject\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        assert_eq!(action.data().name, "action-def");
        assert_eq!(action.data().value, "send");
        assert_eq!(action.children().len(), 2);

        let field = &action.children()[1];
        assert_eq!(field.data().kind, Kind::Atom);
        assert_eq!(field.data().name, "field");
        assert_eq!(field.data().value, "subject");
        assert!(field.is_shard()); // no children — untyped
    }

    #[test]
    fn parse_grammar_action_empty() {
        let source = "grammar @test {\n  action noop {}\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);

        let action = &grammar.children()[0];
        assert_eq!(action.data().kind, Kind::Form);
        assert_eq!(action.data().name, "action-def");
        assert_eq!(action.data().value, "noop");
        assert_eq!(action.children().len(), 1);
        assert_eq!(action.children()[0].data().name, "visibility");
    }

    #[test]
    fn parse_grammar_mixed_types_and_actions() {
        let source = "grammar @test {\n  type = a | b\n  action send {\n    to: address\n  }\n  type address = email | uri\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        // grammar has 3 children: type-def, action-def, type-def
        assert_eq!(grammar.children().len(), 3);

        assert_eq!(grammar.children()[0].data().name, "type-def");
        assert_eq!(grammar.children()[0].data().value, "");
        assert_eq!(grammar.children()[1].data().name, "action-def");
        assert_eq!(grammar.children()[1].data().value, "send");
        assert_eq!(grammar.children()[2].data().name, "type-def");
        assert_eq!(grammar.children()[2].data().value, "address");
    }

    #[test]
    fn parse_grammar_action_error_unclosed() {
        let source = "grammar @test {\n  action send {\n    from: address\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "expected 'unclosed': {}",
            err.message
        );
    }

    #[test]
    fn parse_grammar_action_comments_blanks() {
        let source =
            "grammar @test {\n  action send {\n    # a comment\n\n    from: address\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        assert_eq!(action.data().name, "action-def");
        assert_eq!(action.children().len(), 2); // visibility + field, comments/blanks skipped
        assert_eq!(action.children()[1].data().value, "from");
    }

    #[test]
    fn parse_grammar_action_call() {
        let source = "grammar @test {\n  action commit {\n    source: source\n    @filesystem.write(source)\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        assert_eq!(action.data().value, "commit");
        assert_eq!(action.children().len(), 3); // visibility + field + action-call

        let call = &action.children()[2];
        assert_eq!(call.data().kind, Kind::Ref);
        assert_eq!(call.data().name, "action-call");
        assert_eq!(call.data().value, "@filesystem.write");
        assert_eq!(call.children().len(), 1); // one argument
        assert_eq!(call.children()[0].data().value, "source");
    }

    #[test]
    fn parse_grammar_action_call_multiple_args() {
        let source = "grammar @test {\n  action send {\n    from: address\n    to: address\n    @mail.deliver(from, to)\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        assert_eq!(action.children().len(), 4); // visibility + 2 fields + 1 action-call

        let call = &action.children()[3];
        assert_eq!(call.data().name, "action-call");
        assert_eq!(call.data().value, "@mail.deliver");
        assert_eq!(call.children().len(), 2);
        assert_eq!(call.children()[0].data().value, "from");
        assert_eq!(call.children()[1].data().value, "to");
    }

    #[test]
    fn parse_grammar_action_call_no_args() {
        let source = "grammar @test {\n  action ping {\n    @health.check()\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        assert_eq!(action.children().len(), 2);

        let call = &action.children()[1];
        assert_eq!(call.data().name, "action-call");
        assert_eq!(call.data().value, "@health.check");
        assert_eq!(call.children().len(), 0);
    }

    #[test]
    fn parse_grammar_action_call_no_dot_falls_through() {
        // @nodot(x) has parens but no dot — falls through to field
        let source = "grammar @test {\n  action ping {\n    @nodot(x)\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        // Parsed as untyped field, not action-call
        assert_eq!(action.children().len(), 2);
        assert_eq!(action.children()[1].data().name, "field");
    }

    #[test]
    fn parse_grammar_action_call_no_parens_falls_through() {
        // @domain.action without parens — falls through to field
        let source = "grammar @test {\n  action ping {\n    @domain.action\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let action = &grammar.children()[0];
        assert_eq!(action.children().len(), 2);
        assert_eq!(action.children()[1].data().name, "field");
    }

    // -- Legacy `act` keyword --

    #[test]
    fn parse_grammar_act_with_params() {
        // act enact(effect) { ... } parsed as action "enact"
        let source = "grammar @test {\n  act enact(effect) {\n    target: path\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);
        let action = &grammar.children()[0];
        assert_eq!(action.data().name, "action-def");
        assert_eq!(action.data().value, "enact");
    }

    #[test]
    fn parse_grammar_act_without_params() {
        // act enact { ... } without parens — falls into else branch
        let source = "grammar @test {\n  act enact {\n    target: path\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);
        let action = &grammar.children()[0];
        assert_eq!(action.data().name, "action-def");
        assert_eq!(action.data().value, "enact");
    }

    #[test]
    fn parse_grammar_act_without_brace_skipped() {
        // act without { on the line — falls through to unknown-line handler
        let source = "grammar @test {\n  act enact\n  type = a\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        // Only the type-def should be present; the act line is skipped
        assert_eq!(grammar.children().len(), 1);
        assert_eq!(grammar.children()[0].data().name, "type-def");
    }

    #[test]
    fn parse_grammar_act_flushes_pending_type() {
        // type def before act — pending type flushed before action
        let source =
            "grammar @test {\n  type = a | b\n  act enact(effect) {\n    target: path\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        // Should have type-def followed by action-def
        assert_eq!(grammar.children().len(), 2);
        assert_eq!(grammar.children()[0].data().name, "type-def");
        assert_eq!(grammar.children()[1].data().name, "action-def");
    }

    #[test]
    fn parse_grammar_action_error_no_brace() {
        let source = "grammar @test {\n  action send\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("{"),
            "expected mention of '{{': {}",
            err.message
        );
    }

    #[test]
    fn parse_abstract_action() {
        let source =
            "grammar @cogito {\n  type = observable\n\n  abstract action observe(observable)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("abstract-action"))
            .expect("should have abstract-action node");
        assert_eq!(action.data().value, "observe");
        let params: Vec<_> = action
            .children()
            .iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].data().value, "observable:observable");
    }

    #[test]
    fn parse_action_with_body_and_target() {
        let source = "grammar @ai {\n  type = observation\n\n  action decide(observation) in @rust {\n    provider.infer(observation)\n  }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def node");
        assert_eq!(action.data().value, "decide");
        let target = action
            .children()
            .iter()
            .find(|c| c.data().name == "target")
            .expect("should have target node");
        assert_eq!(target.data().value, "rust");
        let body = action
            .children()
            .iter()
            .find(|c| c.data().name == "body")
            .expect("should have body node");
        assert!(body.data().value.contains("provider.infer"));
    }

    #[test]
    fn parse_action_param_sugar() {
        let source = "grammar @test {\n  type = x | y\n\n  action f(x) in @rust { body }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def node");
        let params: Vec<_> = action
            .children()
            .iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].data().value, "x:x");
    }

    #[test]
    fn parse_action_param_explicit() {
        let source = "grammar @test {\n  type observation = a | b\n\n  action f(obs: observation) in @rust { body }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def node");
        let params: Vec<_> = action
            .children()
            .iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].data().value, "obs:observation");
    }

    #[test]
    fn parse_abstract_action_multiple_params() {
        let source = "grammar @test {\n  type observation = a\n  type schedule = b\n\n  abstract action decide(observation, schedule)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("abstract-action"))
            .expect("should have abstract-action node");
        let params: Vec<_> = action
            .children()
            .iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].data().value, "observation:observation");
        assert_eq!(params[1].data().value, "schedule:schedule");
    }

    #[test]
    fn parse_action_empty_body() {
        let source = "grammar @test {\n  type = x\n\n  action f(x) in @rust {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def");
        let body = action.children().iter().find(|c| c.data().name == "body");
        // Empty body is valid — the body node exists but with empty value
        assert!(body.is_some());
    }

    #[test]
    fn parse_action_body_no_params() {
        // Exercises the empty-params path in parse_action_params
        let source = "grammar @test {\n  type = x\n\n  action f() in @rust { body }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def");
        let params: Vec<_> = action
            .children()
            .iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn parse_abstract_action_error_no_paren() {
        let source = "grammar @test {\n  abstract action observe\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("("),
            "expected mention of '(': {}",
            err.message
        );
    }

    #[test]
    fn parse_abstract_action_error_no_close_paren() {
        let source = "grammar @test {\n  abstract action observe(x\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains(")"),
            "expected mention of ')': {}",
            err.message
        );
    }

    #[test]
    fn parse_action_body_error_no_close_paren() {
        let source = "grammar @test {\n  action f(x in @rust { body }\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains(")"),
            "expected mention of ')': {}",
            err.message
        );
    }

    #[test]
    fn parse_action_body_error_no_in() {
        let source = "grammar @test {\n  action f(x) @rust { body }\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("in @target"),
            "expected mention of 'in @target': {}",
            err.message
        );
    }

    #[test]
    fn parse_action_body_error_no_brace() {
        let source = "grammar @test {\n  action f(x) in @rust\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("{"),
            "expected mention of '{{': {}",
            err.message
        );
    }

    #[test]
    fn parse_action_body_multiline() {
        // Exercises the multi-line body path
        let source =
            "grammar @test {\n  type = x\n\n  action f(x) in @rust {\n    line_one\n    line_two\n  }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def");
        let body = action
            .children()
            .iter()
            .find(|c| c.data().name == "body")
            .expect("should have body");
        assert!(body.data().value.contains("line_one"));
        assert!(body.data().value.contains("line_two"));
    }

    #[test]
    fn parse_action_body_multiline_text_after_brace() {
        // Exercises the after_brace non-empty path in multi-line body
        let source =
            "grammar @test {\n  type = x\n\n  action f(x) in @rust { first_line\n    second_line\n  }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar
            .children()
            .iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def");
        let body = action
            .children()
            .iter()
            .find(|c| c.data().name == "body")
            .expect("should have body");
        assert!(body.data().value.contains("first_line"));
        assert!(body.data().value.contains("second_line"));
    }

    #[test]
    fn parse_action_body_unclosed() {
        // Multi-line action body without closing brace → error
        let source =
            "grammar @test {\n  type = x\n\n  action f(x) in @rust {\n    line_one\n    line_two\n";
        let result = Parse.trace(source.to_string()).into_result();
        assert!(result.is_err(), "unclosed action body should return error");
        let err = result.unwrap_err();
        assert!(
            err.message.contains("unclosed action body block"),
            "error message should mention unclosed body, got: {}",
            err.message
        );
    }

    #[test]
    fn parse_grammar_extends() {
        let source = "grammar @fox extends @smash, @controller {\n  type = move | attack\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("grammar block");
        assert_eq!(grammar.data().value, "@fox");

        // Should have extends children as Ref nodes
        let extends: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().kind == Kind::Ref && c.data().name == "extends")
            .collect();
        assert_eq!(extends.len(), 2, "expected 2 extends refs");
        assert_eq!(extends[0].data().value, "@smash");
        assert_eq!(extends[1].data().value, "@controller");
    }

    #[test]
    fn parse_grammar_extends_single() {
        let source = "grammar @cat extends @animal {\n  type = purr\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("grammar block");
        assert_eq!(grammar.data().value, "@cat");

        let extends: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().kind == Kind::Ref && c.data().name == "extends")
            .collect();
        assert_eq!(extends.len(), 1, "expected 1 extends ref");
        assert_eq!(extends[0].data().value, "@animal");
    }

    #[test]
    fn parse_grammar_no_extends() {
        // Existing grammars without extends should still work and have no extends children
        let source = "grammar @plain {\n  type = a | b\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("grammar block");
        assert_eq!(grammar.data().value, "@plain");

        let extends: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().kind == Kind::Ref && c.data().name == "extends")
            .collect();
        assert_eq!(extends.len(), 0, "expected no extends refs");
    }

    #[test]
    fn parse_grammar_fixture() {
        let source = include_str!("../main.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        // main.conv: three root grammars — @conversation, @prism, @test
        let grammars: Vec<_> = tree
            .children()
            .iter()
            .filter(|c| c.data().is_decl("grammar"))
            .collect();
        assert_eq!(grammars.len(), 3);
        assert_eq!(grammars[0].data().value, "@conversation");
        assert_eq!(grammars[1].data().value, "@prism");
        assert_eq!(grammars[2].data().value, "@test");
    }

    #[test]
    fn parse_requires_in_grammar() {
        let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let requires: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("requires"))
            .collect();
        assert_eq!(requires.len(), 1);
        assert_eq!(requires[0].data().value, "shannon_equivalence");
    }

    #[test]
    fn parse_invariant_in_grammar() {
        let source = "grammar @test {\n  type = a | b\n\n  invariant connected\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let invariants: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("invariant"))
            .collect();
        assert_eq!(invariants.len(), 1);
        assert_eq!(invariants[0].data().value, "connected");
    }

    #[test]
    fn parse_multiple_properties() {
        let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n  invariant connected\n  requires exhaustive\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let requires: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("requires"))
            .collect();
        let invariants: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("invariant"))
            .collect();
        assert_eq!(requires.len(), 2);
        assert_eq!(invariants.len(), 1);
    }

    #[test]
    fn parse_ensures_in_grammar() {
        let source = "grammar @test {\n  type = a | b\n\n  ensures response_time\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let ensures: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("ensures"))
            .collect();
        assert_eq!(ensures.len(), 1);
        assert_eq!(ensures[0].data().value, "response_time");
    }

    #[test]
    fn parse_all_three_property_kinds() {
        let source = "grammar @test {\n  type = a | b\n\n  requires shannon_equivalence\n  invariant connected\n  ensures response_time\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let requires: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("requires"))
            .collect();
        let invariants: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("invariant"))
            .collect();
        let ensures: Vec<_> = grammar
            .children()
            .iter()
            .filter(|c| c.data().is_decl("ensures"))
            .collect();
        assert_eq!(requires.len(), 1);
        assert_eq!(invariants.len(), 1);
        assert_eq!(ensures.len(), 1);
    }

    #[test]
    fn parse_mail_conv() {
        let source = include_str!("../conv/mail.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children.len(), 2); // grammar + template

        // Grammar: @mail with 6 type-defs + 3 action-defs = 9 children
        let grammar = &children[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@mail");
        assert_eq!(grammar.children().len(), 9);

        // Root type: 5 variants (message | thread | attachment | address | server)
        let root_type = &grammar.children()[0];
        assert_eq!(root_type.data().kind, Kind::Form);
        assert_eq!(root_type.data().name, "type-def");
        assert_eq!(root_type.data().value, "");
        assert_eq!(root_type.children().len(), 5);

        // Header type: 10 variants
        let header_type = &grammar.children()[1];
        assert_eq!(header_type.data().name, "type-def");
        assert_eq!(header_type.data().value, "header");
        assert_eq!(header_type.children().len(), 10);

        // Flag type: 5 variants
        let flag_type = &grammar.children()[2];
        assert_eq!(flag_type.data().name, "type-def");
        assert_eq!(flag_type.data().value, "flag");
        assert_eq!(flag_type.children().len(), 5);

        // Protocol type: 3 variants (smtp | imap | jmap)
        let protocol_type = &grammar.children()[3];
        assert_eq!(protocol_type.data().name, "type-def");
        assert_eq!(protocol_type.data().value, "protocol");
        assert_eq!(protocol_type.children().len(), 3);

        // Server type: 3 variants (stalwart | maddy | mailbox)
        let server_type = &grammar.children()[4];
        assert_eq!(server_type.data().name, "type-def");
        assert_eq!(server_type.data().value, "server");
        assert_eq!(server_type.children().len(), 3);

        // DNS type: 5 variants (spf | dkim | dmarc | mta-sts | dane)
        let dns_type = &grammar.children()[5];
        assert_eq!(dns_type.data().name, "type-def");
        assert_eq!(dns_type.data().value, "dns");
        assert_eq!(dns_type.children().len(), 5);

        // Action: send (4 fields, 2 typed + 2 untyped)
        let action_send = &grammar.children()[6];
        assert_eq!(action_send.data().name, "action-def");
        assert_eq!(action_send.data().value, "send");
        assert_eq!(action_send.children().len(), 5);

        // Action: reply (2 fields)
        let action_reply = &grammar.children()[7];
        assert_eq!(action_reply.data().name, "action-def");
        assert_eq!(action_reply.data().value, "reply");
        assert_eq!(action_reply.children().len(), 3);

        // Action: forward (2 fields)
        let action_forward = &grammar.children()[8];
        assert_eq!(action_forward.data().name, "action-def");
        assert_eq!(action_forward.data().value, "forward");
        assert_eq!(action_forward.children().len(), 3);

        // Template: $message with 1 param + 6 fields = 7 children
        let template = &children[1];
        assert_eq!(template.data().kind, Kind::Decl);
        assert_eq!(template.data().value, "$message");
        assert_eq!(template.children().len(), 7); // 1 param + 6 fields

        // First child is the @imap param
        let param = &template.children()[0];
        assert_eq!(param.data().kind, Kind::Atom);
        assert_eq!(param.data().value, "imap");
        assert_eq!(param.children()[0].data().kind, Kind::Ref);
        assert_eq!(param.children()[0].data().value, "@imap");

        // Remaining 6 are fields
        for field in &template.children()[1..] {
            assert_eq!(field.data().kind, Kind::Atom);
        }
    }

    // -- Parse parameterized templates --

    #[test]
    fn parse_template_single_domain_param() {
        let source = "template $t(@json) {\n  slug\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.data().kind, Kind::Decl);
        assert_eq!(tmpl.data().value, "$t");
        assert_eq!(tmpl.children().len(), 2); // Param + Field
        let param = &tmpl.children()[0];
        assert_eq!(param.data().kind, Kind::Atom);
        assert_eq!(param.data().value, "json"); // inferred name
        assert_eq!(param.children().len(), 1);
        assert_eq!(param.children()[0].data().kind, Kind::Ref);
        assert_eq!(param.children()[0].data().value, "@json");
        let field = &tmpl.children()[1];
        assert_eq!(field.data().kind, Kind::Atom);
        assert_eq!(field.data().value, "slug");
    }

    #[test]
    fn parse_template_named_param() {
        let source = "template $t(data: @json) {\n  slug\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.children().len(), 2); // Param + Field
        let param = &tmpl.children()[0];
        assert_eq!(param.data().kind, Kind::Atom);
        assert_eq!(param.data().value, "data"); // explicit name
        assert_eq!(param.children()[0].data().kind, Kind::Ref);
        assert_eq!(param.children()[0].data().value, "@json");
    }

    #[test]
    fn parse_template_multiple_params() {
        let source = "template $t(@json, @csv) {\n  slug\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.children().len(), 3); // 2 Params + 1 Field
        assert_eq!(tmpl.children()[0].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[0].data().value, "json");
        assert_eq!(tmpl.children()[1].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[1].data().value, "csv");
        assert_eq!(tmpl.children()[2].data().kind, Kind::Atom);
    }

    #[test]
    fn parse_template_pipeline_param_named() {
        let source = "template $t(authors: @csv | select(.author)) {\n  slug\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        let param = &tmpl.children()[0];
        assert_eq!(param.data().kind, Kind::Atom);
        assert_eq!(param.data().value, "authors");
        assert_eq!(param.children()[0].data().kind, Kind::Form);
        let pipeline = &param.children()[0];
        assert_eq!(pipeline.children()[0].data().kind, Kind::Ref);
        assert_eq!(pipeline.children()[0].data().value, "@csv");
        assert_eq!(pipeline.children()[1].data().kind, Kind::Ref);
        assert_eq!(pipeline.children()[1].data().value, "select(.author)");
    }

    #[test]
    fn parse_template_dotted_param_named() {
        let source = "template $t(lines: @json.type.Array) {\n  slug\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        let param = &tmpl.children()[0];
        assert_eq!(param.data().kind, Kind::Atom);
        assert_eq!(param.data().value, "lines");
        assert_eq!(param.children()[0].data().kind, Kind::Atom);
        assert_eq!(param.children()[0].data().value, "@json.type.Array");
    }

    #[test]
    fn parse_template_domain_param_with_domain_params() {
        let source = "template $t(@git(branch: \"main\")) {\n  slug\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        let param = &tmpl.children()[0];
        assert_eq!(param.data().kind, Kind::Atom);
        assert_eq!(param.data().value, "git"); // inferred: @git(branch: "main") → "git"
        assert_eq!(param.children()[0].data().kind, Kind::Ref);
        assert_eq!(param.children()[0].data().value, "@git");
        assert_eq!(param.children()[0].children()[0].data().kind, Kind::Ref);
    }

    #[test]
    fn parse_template_params_and_fields_order() {
        let source = "template $t(@json, @csv) {\n  slug\n  title\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        // Params come first, then fields
        assert_eq!(tmpl.children()[0].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[1].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[2].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[3].data().kind, Kind::Atom);
    }

    #[test]
    fn parse_template_no_params_unchanged() {
        // Backward compat: template without params still works
        let source = "template $t {\n  slug\n  title\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.data().value, "$t");
        assert_eq!(tmpl.children().len(), 2);
        assert_eq!(tmpl.children()[0].data().kind, Kind::Atom);
        assert_eq!(tmpl.children()[1].data().kind, Kind::Atom);
    }

    #[test]
    fn parse_template_error_unnamed_pipeline() {
        let source = "template $t(@csv | select(.author)) {\n  slug\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("must be explicitly named"),
            "expected naming error: {}",
            err.message
        );
    }

    #[test]
    fn parse_template_error_unnamed_dotted() {
        let source = "template $t(@json.type.Array) {\n  slug\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("must be explicitly named"),
            "expected naming error: {}",
            err.message
        );
    }

    #[test]
    fn parse_template_error_unnamed_bare() {
        let source = "template $t(foo) {\n  slug\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("cannot infer name"),
            "expected inference error: {}",
            err.message
        );
    }

    #[test]
    fn parse_template_error_in_multi_param() {
        // Error in second param of a comma-separated list (exercises ? in comma branch)
        let source = "template $t(@json, @csv | transform) {\n  slug\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("must be explicitly named"),
            "expected naming error: {}",
            err.message
        );
    }

    // -- Grammar parse tests for stdlib domains --

    #[test]
    fn parse_beam_conv() {
        let source = include_str!("../conv/beam.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@beam");
        assert_eq!(grammar.children().len(), 1); // 1 type def

        let root_type = &grammar.children()[0];
        assert_eq!(root_type.data().kind, Kind::Form);
        assert_eq!(root_type.data().value, "");
        assert_eq!(root_type.children().len(), 3); // process | supervision | module
    }

    #[test]
    fn parse_git_conv() {
        let source = include_str!("../conv/git.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@git");
        assert_eq!(grammar.children().len(), 1); // 1 type def

        let root_type = &grammar.children()[0];
        assert_eq!(root_type.data().kind, Kind::Form);
        assert_eq!(root_type.data().value, "");
        assert_eq!(root_type.children().len(), 4); // ref | commit | entry | blob
    }

    // -- Content addressing tests for stdlib grammars --

    #[test]
    fn beam_grammar_content_addressed() {
        use crate::prism::content_oid;
        let source = "grammar @beam {\n  type = process | supervision | module\n}\n";
        let a = Parse.trace(source.to_string()).unwrap();
        let b = Parse.trace(source.to_string()).unwrap();
        assert_eq!(content_oid(&a), content_oid(&b));
    }

    #[test]
    fn beam_grammar_different_source_different_oid() {
        use crate::prism::content_oid;
        let a = Parse
            .trace("grammar @beam {\n  type = process | supervision | module\n}\n".to_string())
            .unwrap();
        let b = Parse
            .trace("grammar @beam {\n  type = process | supervision\n}\n".to_string())
            .unwrap();
        assert_ne!(content_oid(&a), content_oid(&b));
    }

    #[test]
    fn git_grammar_content_addressed() {
        use crate::prism::content_oid;
        let source = "grammar @git {\n  type = ref | commit | entry | blob\n}\n";
        let a = Parse.trace(source.to_string()).unwrap();
        let b = Parse.trace(source.to_string()).unwrap();
        assert_eq!(content_oid(&a), content_oid(&b));
    }

    #[test]
    fn git_grammar_different_source_different_oid() {
        use crate::prism::content_oid;
        let a = Parse
            .trace("grammar @git {\n  type = ref | commit | entry | blob\n}\n".to_string())
            .unwrap();
        let b = Parse
            .trace("grammar @git {\n  type = ref | commit | entry\n}\n".to_string())
            .unwrap();
        assert_ne!(content_oid(&a), content_oid(&b));
    }

    // -- Parse `annotate(@target)` --

    #[test]
    fn parse_annotate_gleam() {
        let source = "annotate(@gleam)\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let node = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .expect("annotate node");
        assert!(node.is_shard());
        assert_eq!(node.data().value, "@gleam");
    }

    #[test]
    fn parse_annotate_elixir() {
        let source = "annotate(@elixir)\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let node = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        assert_eq!(node.data().value, "@elixir");
    }

    #[test]
    fn parse_annotate_multiple_targets() {
        let source = "annotate(@gleam)\nannotate(@elixir)\nannotate(@fortran)\n".to_string();
        let tree = Parse.trace(source).unwrap();
        let annotations: Vec<_> = tree
            .children()
            .iter()
            .filter(|c| c.data().is_decl("annotate"))
            .collect();
        assert_eq!(annotations.len(), 3);
        assert_eq!(annotations[0].data().value, "@gleam");
        assert_eq!(annotations[1].data().value, "@elixir");
        assert_eq!(annotations[2].data().value, "@fortran");
    }

    #[test]
    fn parse_annotate_with_in_and_grammar() {
        let source = "grammar @color {\n  type = red | green | blue\n}\nannotate(@gleam)\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let has_grammar = tree.children().iter().any(|c| c.data().is_decl("grammar"));
        let has_annotate = tree.children().iter().any(|c| c.data().is_decl("annotate"));
        assert!(has_grammar);
        assert!(has_annotate);
    }

    // -- Annotate block form + --- sugar --

    #[test]
    fn parse_annotate_block() {
        let source = "annotate(@test) {\n  test \"types\" { @beam has process }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .expect("annotate node");
        assert_eq!(annotate.data().value, "@test");
        assert!(!annotate.children().is_empty());
        let test_node = &annotate.children()[0];
        assert_eq!(test_node.data().kind, Kind::Form);
        assert_eq!(test_node.data().name, "test");
        assert_eq!(test_node.data().value, "types");
        assert_eq!(test_node.children().len(), 1);
        assert_eq!(test_node.children()[0].data().name, "assertion");
        assert_eq!(test_node.children()[0].data().value, "@beam has process");
    }

    #[test]
    fn parse_separator_as_annotate_test() {
        let source = "grammar @g {\n  type = a\n}\n---\ntest \"check\" { @g has a }\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .expect("--- should produce annotate(@test)");
        assert_eq!(annotate.data().value, "@test");
        assert!(!annotate.children().is_empty());
        let test_node = &annotate.children()[0];
        assert_eq!(test_node.data().name, "test");
        assert_eq!(test_node.data().value, "check");
    }

    #[test]
    fn parse_separator_multiple_tests() {
        let source = "grammar @g {\n  type = a | b\n}\n---\ntest \"first\" { @g has a }\ntest \"second\" { @g has b }\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        assert_eq!(annotate.children().len(), 2);
        assert_eq!(annotate.children()[0].data().value, "first");
        assert_eq!(annotate.children()[1].data().value, "second");
    }

    #[test]
    fn parse_separator_property_directive() {
        let source = "grammar @g {\n  type = a\n}\n---\nproperty \"shannon\" { @g preserves shannon_equivalence }\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        let prop = &annotate.children()[0];
        assert_eq!(prop.data().name, "property");
        assert_eq!(prop.data().value, "shannon");
        assert_eq!(prop.children().len(), 1);
        assert_eq!(
            prop.children()[0].data().value,
            "@g preserves shannon_equivalence"
        );
    }

    #[test]
    fn parse_annotate_block_empty() {
        let source = "annotate(@ci) {\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        assert_eq!(annotate.data().value, "@ci");
        assert!(annotate.children().is_empty());
    }

    #[test]
    fn parse_separator_empty() {
        let source = "grammar @g {\n  type = a\n}\n---\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        assert_eq!(annotate.data().value, "@test");
        assert!(annotate.children().is_empty());
    }

    #[test]
    fn parse_separator_mixed_directives() {
        let source = "grammar @g {\n  type = a\n}\n---\ntest \"t\" { @g has a }\ngenerate @g {\n  type = x\n}\nproperty \"p\" { @g preserves shannon_equivalence }\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        assert_eq!(annotate.children().len(), 3);
        assert_eq!(annotate.children()[0].data().name, "test");
        assert_eq!(annotate.children()[1].data().name, "generate");
        assert_eq!(annotate.children()[2].data().name, "property");
    }

    #[test]
    fn parse_annotate_invalid_syntax() {
        let source = "annotate(@test {\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("annotate"),
            "expected annotate error: {}",
            err.message
        );
    }

    #[test]
    fn parse_separator_with_comments_and_unknown() {
        let source =
            "grammar @g {\n  type = a\n}\n---\n# comment\n\nunknown line\ntest \"check\" { @g has a }\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        // unknown line is silently skipped
        assert_eq!(annotate.children().len(), 1);
    }

    #[test]
    fn parse_separator_generate_directive() {
        let source = "grammar @g {\n  type = a\n}\n---\ngenerate @g {\n  type = x | y\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let annotate = tree
            .children()
            .iter()
            .find(|c| c.data().is_decl("annotate"))
            .unwrap();
        let gen = &annotate.children()[0];
        assert_eq!(gen.data().name, "generate");
        assert_eq!(gen.data().value, "@g");
        assert_eq!(gen.children().len(), 1);
        assert_eq!(gen.children()[0].data().name, "override");
        assert_eq!(gen.children()[0].data().value, "type = x | y");
    }

    // -- Test section DSL --

    #[test]
    fn parse_test_section_test_inline() {
        let source = "test \"basic\" { @beam has process; @beam has module }";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (name, assertions) = directives[0].as_test();
        assert_eq!(name, "basic");
        assert_eq!(assertions.len(), 2);
        assert_eq!(assertions[0].domain, "beam");
        assert_eq!(assertions[0].variant, "process");
        assert_eq!(assertions[1].variant, "module");
    }

    #[test]
    fn parse_test_section_test_multiline() {
        let source = "test \"multi\" {\n  @beam has process\n  @beam has module\n}\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (name, assertions) = directives[0].as_test();
        assert_eq!(name, "multi");
        assert_eq!(assertions.len(), 2);
    }

    #[test]
    fn parse_test_section_typed_assertion() {
        let source = "test \"typed\" { @beam.target has gleam }";
        let directives = parse_test_section(source).unwrap();
        let (_, assertions) = directives[0].as_test();
        assert_eq!(assertions[0].domain, "beam");
        assert_eq!(assertions[0].type_name, Some("target".to_string()));
        assert_eq!(assertions[0].variant, "gleam");
    }

    #[test]
    fn parse_test_section_property() {
        let source = "property \"shannon\" { @beam preserves shannon_equivalence }";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (name, checks) = directives[0].as_property();
        assert_eq!(name, "shannon");
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].domain, "beam");
        assert_eq!(checks[0].property, "shannon_equivalence");
    }

    #[test]
    fn parse_test_section_property_multiline() {
        let source = "property \"multi\" {\n  @beam preserves shannon_equivalence\n  @git preserves shannon_equivalence\n}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, checks) = directives[0].as_property();
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].domain, "beam");
        assert_eq!(checks[1].domain, "git");
    }

    #[test]
    fn parse_test_section_generate() {
        let source = "generate @beam { type = custom_a | custom_b }";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (domain, overrides) = directives[0].as_generate();
        assert_eq!(domain, "beam");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].0, "");
        assert_eq!(overrides[0].1, vec!["custom_a", "custom_b"]);
    }

    #[test]
    fn parse_test_section_generate_named_type() {
        let source = "generate @test {\n  type op = custom_gt\n}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides[0].0, "op");
        assert_eq!(overrides[0].1, vec!["custom_gt"]);
    }

    #[test]
    fn parse_test_section_multiple_directives() {
        let source =
            "test \"a\" { @x has y }\nproperty \"b\" { @x preserves shannon_equivalence }\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 2);
        assert!(matches!(&directives[0], TestDirective::Test { .. }));
        assert!(matches!(&directives[1], TestDirective::Property { .. }));
    }

    #[test]
    fn parse_test_section_skips_comments_and_blanks() {
        let source = "# comment\n\ntest \"a\" { @x has y }\n\n# another\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
    }

    #[test]
    fn parse_test_section_empty() {
        let directives = parse_test_section("").unwrap();
        assert!(directives.is_empty());
    }

    #[test]
    fn parse_test_section_unclosed_block() {
        let source = "test \"broken\" {\n  @x has y\n";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("unclosed"));
    }

    #[test]
    fn parse_test_section_bad_assertion() {
        let source = "test \"bad\" { @x missing y }";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("@domain has variant"));
    }

    #[test]
    fn parse_test_section_bad_property() {
        let source = "property \"bad\" { @x violates stuff }";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("@domain preserves property"));
    }

    // -- Error paths: parse_directive_block --

    #[test]
    fn parse_directive_block_blank_line_in_body() {
        // Blank line inside a multi-line directive block should be skipped
        let source = "test \"blank\" {\n  @beam has process\n\n  @beam has module\n}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, assertions) = directives[0].as_test();
        assert_eq!(assertions.len(), 2);
    }

    #[test]
    fn parse_directive_block_unclosed_quote() {
        let source = "test \"broken {\n  @x has y\n}\n";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("unclosed quote"));
    }

    #[test]
    fn parse_directive_block_unquoted_name() {
        let source = "test broken { @x has y }";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("must be quoted"));
    }

    #[test]
    fn parse_directive_block_multiline_with_opening_brace_content() {
        // Opening brace on same line as name, body spans multiple lines
        let source = "test \"inline\" { @beam has process\n  @beam has module\n}\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (_, assertions) = directives[0].as_test();
        assert_eq!(assertions.len(), 2);
    }

    #[test]
    fn parse_directive_block_content_before_closing_brace() {
        // Body content on the same line as closing brace: `@beam has x }`
        let source = "test \"inline\" {\n  @beam has process\n  @beam has module}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, assertions) = directives[0].as_test();
        assert_eq!(assertions.len(), 2);
    }

    // -- Error paths: parse_generate_block --

    #[test]
    fn parse_generate_block_multiline_with_opening_brace_content() {
        // Opening brace on same line, body spans multiple lines
        let source = "generate @test { type = a\n  type op = b\n}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides.len(), 2);
    }

    #[test]
    fn parse_generate_block_content_before_closing_brace() {
        // Content on same line as closing brace
        let source = "generate @test {\n  type = a\n  type op = b}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides.len(), 2);
    }

    #[test]
    fn parse_generate_block_unclosed() {
        let source = "generate @test {\n  type = a\n";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("unclosed generate block"));
    }

    #[test]
    fn parse_generate_block_multiline_body() {
        // Multiple lines in generate block body
        let source = "generate @test {\n  type = x\n  type op = y\n}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides.len(), 2);
    }

    #[test]
    fn parse_generate_block_blank_line_in_body() {
        // Blank line inside a multi-line generate block should be skipped
        let source = "generate @test {\n  type = x\n\n  type op = y\n}\n";
        let directives = parse_test_section(source).unwrap();
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides.len(), 2);
    }

    // -- Error paths: parse_has_assertions, parse_property_checks, parse_generate_overrides --

    #[test]
    fn parse_has_assertions_trailing_semicolon() {
        // Trailing semicolon produces empty stmt -> continue
        let source = "test \"trail\" { @x has y; }";
        let directives = parse_test_section(source).unwrap();
        let (_, assertions) = directives[0].as_test();
        assert_eq!(assertions.len(), 1);
    }

    #[test]
    fn parse_property_checks_trailing_semicolon() {
        let source = "property \"trail\" { @x preserves shannon_equivalence; }";
        let directives = parse_test_section(source).unwrap();
        let (_, checks) = directives[0].as_property();
        assert_eq!(checks.len(), 1);
    }

    #[test]
    fn parse_generate_overrides_trailing_semicolon() {
        let source = "generate @test { type = a; }";
        let directives = parse_test_section(source).unwrap();
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides.len(), 1);
    }

    #[test]
    fn parse_generate_override_non_type_line() {
        let source = "generate @test { nonsense = a }";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("expected `type = variant | ...`"));
    }

    #[test]
    fn parse_generate_override_missing_equals() {
        let source = "generate @test { type no_equals }";
        let err = parse_test_section(source).unwrap_err();
        assert!(err.message.contains("missing `=`"));
    }

    // -- Unknown directive line (line 1158: fall-through) --

    #[test]
    fn parse_test_section_unknown_directive_skipped() {
        let source = "unknown_directive stuff\ntest \"a\" { @x has y }\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
    }

    #[test]
    fn parse_test_section_brace_on_header_multiline() {
        // Opening brace on header line, content on next lines
        let source = "test \"inline-start\" {\n  @x has y\n}\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (_, assertions) = directives[0].as_test();
        assert_eq!(assertions.len(), 1);
    }

    #[test]
    fn parse_test_section_brace_on_next_line() {
        // No brace on header line — body starts on next line (false branch of starts_with('{'))
        let source = "test \"deferred\"\n  @x has y\n}\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (name, assertions) = directives[0].as_test();
        assert_eq!(name, "deferred");
        assert_eq!(assertions.len(), 1);
    }

    #[test]
    fn parse_test_section_generate_brace_on_header_multiline() {
        // Opening brace on header line for generate, content on next lines
        let source = "generate @test {\n  type = custom\n}\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (_, overrides) = directives[0].as_generate();
        assert_eq!(overrides[0].1, vec!["custom"]);
    }

    #[test]
    fn parse_test_section_generate_brace_on_next_line() {
        // No brace on header line for generate (false branch of starts_with('{'))
        let source = "generate @test\n  type = custom\n}\n";
        let directives = parse_test_section(source).unwrap();
        assert_eq!(directives.len(), 1);
        let (domain, overrides) = directives[0].as_generate();
        assert_eq!(domain, "test");
        assert_eq!(overrides[0].1, vec!["custom"]);
    }

    // -- Keyword dispatch table --

    #[test]
    fn dispatch_known_keyword() {
        let source = "in @filesystem\n";
        let mut lines = Lines::new(source);
        let trimmed = lines.peek().unwrap().trim();
        let result = dispatch_keyword(trimmed, &mut lines).unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().data().is_decl("in"));
    }

    #[test]
    fn dispatch_returns_none_for_unknown() {
        let source = "xyzzy 42\n";
        let mut lines = Lines::new(source);
        let trimmed = lines.peek().unwrap().trim();
        let result = dispatch_keyword(trimmed, &mut lines).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn dispatch_advances_single_line_keywords() {
        let source = "when .x == 1\nnext line\n";
        let mut lines = Lines::new(source);
        let trimmed = lines.peek().unwrap().trim();
        let _ = dispatch_keyword(trimmed, &mut lines).unwrap();
        // Single-line keyword should advance past the keyword line
        assert_eq!(lines.peek(), Some("next line"));
    }
}
