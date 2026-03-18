//! Parser story. Source text → AST tree.
//!
//! The parser IS a story: it records a transformation from source to tree.

use crate::ast::{self, AstNode, Span};
use crate::domain::conversation::Kind;
use crate::tree::Tree;
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

impl Vector<String, Tree<AstNode>> for Parse {
    type Error = ParseError;

    fn trace(&self, source: String) -> Trace<Tree<AstNode>, ParseError> {
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

fn parse_source(source: &str) -> Result<Tree<AstNode>, ParseError> {
    let mut lines = Lines::new(source);
    let mut children = Vec::new();
    let root_span = Span::new(0, source.len() as u32);

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.advance();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("in ") {
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

            if in_children.is_empty() {
                children.push(ast::ast_leaf(Kind::Decl, "in", value, span));
            } else {
                children.push(ast::ast_branch(Kind::Decl, "in", value, span, in_children));
            }
            lines.advance();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("use ") {
            let use_node = parse_use(rest, lines.current_span());
            children.push(use_node);
            lines.advance();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("when ") {
            let when_node = parse_when(rest, lines.current_span())?;
            children.push(when_node);
            lines.advance();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("case ") {
            let case_node = parse_case(rest, &mut lines)?;
            children.push(case_node);
            continue;
        }

        // branch(.path) { ... } — standalone
        if trimmed.starts_with("branch(") {
            let branch_node = parse_branch(trimmed, &mut lines)?;
            children.push(branch_node);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("template ") {
            let tmpl = parse_template(rest, &mut lines)?;
            children.push(tmpl);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("out ") {
            let out = parse_out(rest, &mut lines)?;
            children.push(out);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("grammar ") {
            let grammar_node = parse_grammar(rest, &mut lines)?;
            children.push(grammar_node);
            continue;
        }

        if trimmed.starts_with("annotate(") && trimmed.ends_with(')') {
            let span = lines.current_span();
            let inner = &trimmed["annotate(".len()..trimmed.len() - 1];
            children.push(ast::ast_leaf(Kind::Decl, "annotate", inner.trim(), span));
            lines.advance();
            continue;
        }

        // Pipeline ending in branch: @json | branch(.path) { ... }
        if trimmed.contains("| branch(") {
            let pipeline_node = parse_pipeline_with_branch(trimmed, &mut lines)?;
            children.push(pipeline_node);
            continue;
        }

        // Pipeline: A | G | B
        if trimmed.contains('|') {
            let pipeline = parse_pipeline(trimmed, lines.current_span());
            children.push(pipeline);
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

fn parse_template(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
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

fn parse_param_list(text: &str, span: Span) -> Result<Vec<Tree<AstNode>>, ParseError> {
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

fn parse_param(text: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
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

fn parse_param_expr(text: &str, span: Span) -> Tree<AstNode> {
    if text.contains('|') {
        parse_pipeline(text, span)
    } else if text.starts_with('@') && text[1..].contains('.') {
        ast::ast_leaf(Kind::Atom, "path", text, span)
    } else {
        parse_pipeline_segment(text, span)
    }
}

fn parse_field(text: &str, span: Span) -> Tree<AstNode> {
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

fn parse_out(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
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
) -> Result<(Vec<Tree<AstNode>>, Span), ParseError> {
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

fn push_path_segments(rest: &str, span: Span, children: &mut Vec<Tree<AstNode>>) {
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
fn parse_use_source(source: &str, span: Span, children: &mut Vec<Tree<AstNode>>) {
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
fn parse_use(rest: &str, span: Span) -> Tree<AstNode> {
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
fn parse_when(rest: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
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
fn parse_case(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
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
fn parse_arm(text: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
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
fn parse_cmp(text: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
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
fn parse_branch(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
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
fn parse_branch_arm(text: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
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
) -> Result<Tree<AstNode>, ParseError> {
    let span = lines.current_span();
    let split_idx = header.find("| branch(").unwrap();
    let prefix = header[..split_idx].trim();
    let branch_part = header[split_idx + 2..].trim(); // skip "| "

    // Parse prefix segments
    let prefix_segments: Vec<Tree<AstNode>> = prefix
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
fn parse_pipeline(text: &str, span: Span) -> Tree<AstNode> {
    let segments: Vec<&str> = text.split('|').map(|s| s.trim()).collect();
    let children: Vec<Tree<AstNode>> = segments
        .iter()
        .map(|seg| parse_pipeline_segment(seg, span))
        .collect();
    ast::ast_branch(Kind::Form, "pipeline", "root", span, children)
}

fn parse_pipeline_segment(seg: &str, span: Span) -> Tree<AstNode> {
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
fn parse_grammar(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
    let start_span = lines.current_span();

    // Extract @name and verify opening brace
    let (name, rest) = match header.split_once('{') {
        Some((n, r)) => (n.trim(), r),
        None => {
            return Err(ParseError {
                message: format!("grammar: expected '{{' in: grammar {}", header),
                span: Some(start_span),
            })
        }
    };

    // Check for single-line empty grammar: `grammar @name {}`
    if rest.trim() == "}" {
        lines.advance();
        return Ok(ast::ast_branch(
            Kind::Decl,
            "grammar",
            name,
            start_span,
            vec![],
        ));
    }

    lines.advance(); // consume grammar header line

    let mut type_defs: Vec<Tree<AstNode>> = Vec::new();
    // Accumulate variants for the current type def (name, span, variants)
    let mut current: Option<(String, Span, Vec<Tree<AstNode>>)> = None;

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            // Flush any pending type def
            if let Some((type_name, type_span, variants)) = current.take() {
                type_defs.push(ast::ast_branch(
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
            return Ok(ast::ast_branch(
                Kind::Decl,
                "grammar",
                name,
                span,
                type_defs,
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

        if let Some(rest) = trimmed.strip_prefix("act ") {
            // Flush any pending type def
            if let Some((type_name, type_span, variants)) = current.take() {
                type_defs.push(ast::ast_branch(
                    Kind::Form,
                    "type-def",
                    &*type_name,
                    type_span,
                    variants,
                ));
            }
            let span = lines.current_span();
            type_defs.push(parse_act_def(rest, span, lines)?);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("type ") {
            // Flush previous type def
            if let Some((type_name, type_span, variants)) = current.take() {
                type_defs.push(ast::ast_branch(
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
fn parse_type_def_parts(rest: &str, span: Span) -> (String, Vec<Tree<AstNode>>) {
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
fn parse_variants(text: &str, span: Span) -> Vec<Tree<AstNode>> {
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

/// Parse an act definition block inside a grammar.
///
/// `send { from: address\n  to: address }` → Form("act-def", "send") with field children
/// `noop {}` → Form("act-def", "noop") with no children
fn parse_act_def(header: &str, span: Span, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
    let (name, rest) = match header.split_once('{') {
        Some((n, r)) => (n.trim(), r.trim()),
        None => {
            return Err(ParseError {
                message: format!("act: expected '{{' in: act {}", header),
                span: Some(span),
            })
        }
    };

    // Single-line empty: `act noop {}`
    if rest == "}" {
        lines.advance();
        return Ok(ast::ast_branch(Kind::Form, "act-def", name, span, vec![]));
    }

    lines.advance(); // consume the act header line

    let mut fields: Vec<Tree<AstNode>> = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            return Ok(ast::ast_branch(
                Kind::Form,
                "act-def",
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
        message: "unclosed act block".into(),
        span: Some(span),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

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

    // -- Parse `act` in grammar blocks --

    #[test]
    fn parse_grammar_act_single() {
        let source = "grammar @test {\n  act send {\n    from: address\n    to: address\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);

        let act = &grammar.children()[0];
        assert_eq!(act.data().kind, Kind::Form);
        assert_eq!(act.data().name, "act-def");
        assert_eq!(act.data().value, "send");
        assert_eq!(act.children().len(), 2);

        let from = &act.children()[0];
        assert_eq!(from.data().kind, Kind::Atom);
        assert_eq!(from.data().name, "field");
        assert_eq!(from.data().value, "from");
        assert_eq!(from.children().len(), 1);
        assert_eq!(from.children()[0].data().kind, Kind::Ref);
        assert_eq!(from.children()[0].data().name, "type-ref");
        assert_eq!(from.children()[0].data().value, "address");

        let to = &act.children()[1];
        assert_eq!(to.data().kind, Kind::Atom);
        assert_eq!(to.data().name, "field");
        assert_eq!(to.data().value, "to");
        assert_eq!(to.children()[0].data().value, "address");
    }

    #[test]
    fn parse_grammar_act_untyped_field() {
        let source = "grammar @test {\n  act send {\n    subject\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let act = &grammar.children()[0];
        assert_eq!(act.data().name, "act-def");
        assert_eq!(act.data().value, "send");
        assert_eq!(act.children().len(), 1);

        let field = &act.children()[0];
        assert_eq!(field.data().kind, Kind::Atom);
        assert_eq!(field.data().name, "field");
        assert_eq!(field.data().value, "subject");
        assert!(field.is_shard()); // no children — untyped
    }

    #[test]
    fn parse_grammar_act_empty() {
        let source = "grammar @test {\n  act noop {}\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.children().len(), 1);

        let act = &grammar.children()[0];
        assert_eq!(act.data().kind, Kind::Form);
        assert_eq!(act.data().name, "act-def");
        assert_eq!(act.data().value, "noop");
        assert_eq!(act.children().len(), 0);
    }

    #[test]
    fn parse_grammar_mixed_types_and_acts() {
        let source = "grammar @test {\n  type = a | b\n  act send {\n    to: address\n  }\n  type address = email | uri\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        // grammar has 3 children: type-def, act-def, type-def
        assert_eq!(grammar.children().len(), 3);

        assert_eq!(grammar.children()[0].data().name, "type-def");
        assert_eq!(grammar.children()[0].data().value, "");
        assert_eq!(grammar.children()[1].data().name, "act-def");
        assert_eq!(grammar.children()[1].data().value, "send");
        assert_eq!(grammar.children()[2].data().name, "type-def");
        assert_eq!(grammar.children()[2].data().value, "address");
    }

    #[test]
    fn parse_grammar_act_error_unclosed() {
        let source = "grammar @test {\n  act send {\n    from: address\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "expected 'unclosed': {}",
            err.message
        );
    }

    #[test]
    fn parse_grammar_act_comments_blanks() {
        let source =
            "grammar @test {\n  act send {\n    # a comment\n\n    from: address\n  }\n}\n";
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        let act = &grammar.children()[0];
        assert_eq!(act.data().name, "act-def");
        assert_eq!(act.children().len(), 1); // only the field, comments/blanks skipped
        assert_eq!(act.children()[0].data().value, "from");
    }

    #[test]
    fn parse_grammar_act_error_no_brace() {
        let source = "grammar @test {\n  act send\n}\n";
        let err = Parse.trace(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("{"),
            "expected mention of '{{': {}",
            err.message
        );
    }

    #[test]
    fn parse_grammar_fixture() {
        let source = include_str!("../main.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let grammar = &tree.children()[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@conversation");
        assert_eq!(grammar.children().len(), 2);
    }

    #[test]
    fn parse_mail_conv() {
        let source = include_str!("../conv/mail.conv");
        let tree = Parse.trace(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children.len(), 2); // grammar + template

        // Grammar: @mail with 3 type defs
        let grammar = &children[0];
        assert_eq!(grammar.data().kind, Kind::Decl);
        assert_eq!(grammar.data().value, "@mail");
        assert_eq!(grammar.children().len(), 3);

        // Root type: 4 variants (message | thread | attachment | address)
        let root_type = &grammar.children()[0];
        assert_eq!(root_type.data().kind, Kind::Form);
        assert_eq!(root_type.data().value, "");
        assert_eq!(root_type.children().len(), 4);

        // Header type: 10 variants
        let header_type = &grammar.children()[1];
        assert_eq!(header_type.data().kind, Kind::Form);
        assert_eq!(header_type.data().value, "header");
        assert_eq!(header_type.children().len(), 10);

        // Flag type: 5 variants
        let flag_type = &grammar.children()[2];
        assert_eq!(flag_type.data().kind, Kind::Form);
        assert_eq!(flag_type.data().value, "flag");
        assert_eq!(flag_type.children().len(), 5);

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
        use crate::tree::content_oid;
        let source = "grammar @beam {\n  type = process | supervision | module\n}\n";
        let a = Parse.trace(source.to_string()).unwrap();
        let b = Parse.trace(source.to_string()).unwrap();
        assert_eq!(content_oid(&a), content_oid(&b));
    }

    #[test]
    fn beam_grammar_different_source_different_oid() {
        use crate::tree::content_oid;
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
        use crate::tree::content_oid;
        let source = "grammar @git {\n  type = ref | commit | entry | blob\n}\n";
        let a = Parse.trace(source.to_string()).unwrap();
        let b = Parse.trace(source.to_string()).unwrap();
        assert_eq!(content_oid(&a), content_oid(&b));
    }

    #[test]
    fn git_grammar_different_source_different_oid() {
        use crate::tree::content_oid;
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
}
