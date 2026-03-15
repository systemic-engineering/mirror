//! Parser story. Source text → AST tree.
//!
//! The parser IS a story: it records a transformation from source to tree.

use crate::ast::{self, AstNode, Span};
use crate::domain::conversation::{Kind, Op};
use crate::tree::Tree;
use crate::Cut;
use crate::Story;

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

impl Story<String, Tree<AstNode>> for Parse {
    type Error = ParseError;

    fn record(&self, source: String) -> Cut<Tree<AstNode>, ParseError> {
        use crate::ContentAddressed;
        match parse_source(&source) {
            Ok(tree) => {
                let oid = tree.content_oid();
                Cut::success(tree, oid.into(), None)
            }
            Err(e) => Cut::failure(e, crate::CutOid::new("error"), None),
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
                in_children.push(ast::ast_leaf(Kind::DomainParam, p, span));
            }
            if let Some(a) = alias {
                in_children.push(ast::ast_leaf(Kind::Alias, a, span));
            }

            if in_children.is_empty() {
                children.push(ast::ast_leaf(Kind::In, value, span));
            } else {
                children.push(ast::ast_branch(Kind::In, value, span, in_children));
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

    Ok(ast::ast_branch(Kind::Group, "root", root_span, children))
}

fn parse_template(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
    let name = header.split('{').next().unwrap().trim();
    let start_span = lines.current_span();
    lines.advance(); // consume template line

    let mut fields = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed == "}" {
            let end_span = lines.current_span();
            lines.advance();
            let span = start_span.merge(&end_span);
            return Ok(ast::ast_branch(Kind::Template, name, span, fields));
        }

        if trimmed.is_empty() {
            lines.advance();
            continue;
        }

        let field = parse_field(trimmed, lines.current_span());
        fields.push(field);
        lines.advance();
    }

    Err(ParseError {
        message: "unclosed template block".into(),
        span: Some(start_span),
    })
}

fn parse_field(text: &str, span: Span) -> Tree<AstNode> {
    if let Some((name, rest)) = text.split_once(':') {
        let name = name.trim();
        let rest = rest.trim();

        // Check for pipe: "article | @html"
        let parts: Vec<&str> = rest.splitn(2, '|').collect();
        let mut children = Vec::new();

        let qualifier = parts[0].trim();
        children.push(ast::ast_leaf(Kind::Qualifier, qualifier, span));

        if parts.len() > 1 {
            let pipe_value = parts[1].trim();
            children.push(ast::ast_leaf(Kind::Pipe, pipe_value, span));
        }

        ast::ast_branch(Kind::Field, name, span, children)
    } else if let Some((name, pipe)) = text.split_once('|') {
        // Bare field with pipe: "slug | @sha"
        let name = name.trim();
        let pipe_value = pipe.trim();
        let children = vec![ast::ast_leaf(Kind::Pipe, pipe_value, span)];
        ast::ast_branch(Kind::Field, name, span, children)
    } else {
        ast::ast_leaf(Kind::Field, text.trim(), span)
    }
}

fn parse_out(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
    let span = lines.current_span();
    if let Some((name, _)) = header.split_once('{') {
        let name = name.trim();
        lines.advance();
        let (children, end_span) = parse_block_body(lines, span)?;
        let merged = span.merge(&end_span);
        Ok(ast::ast_branch(Kind::Out, name, merged, children))
    } else {
        let name = header.trim();
        lines.advance();
        Ok(ast::ast_leaf(Kind::Out, name, span))
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
                    ast::ast_leaf(Kind::DomainRef, folder_name, span),
                    ast::ast_leaf(Kind::TemplateRef, template_name, span),
                ];
                children.push(ast::ast_branch(
                    Kind::Select,
                    output_part.trim(),
                    span,
                    select_children,
                ));
                lines.advance();
                continue;
            }

            // Field with expression value: "name: expr"
            let span = lines.current_span();
            let expr_child = ast::ast_leaf(Kind::Expr, rest, span);
            children.push(ast::ast_branch(
                Kind::Field,
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
                children.push(ast::ast_branch(Kind::Group, name, span, vec![]));
                lines.advance();
            } else {
                lines.advance();
                let (group_children, end_span) = parse_block_body(lines, span)?;
                let group_span = span.merge(&end_span);
                children.push(ast::ast_branch(
                    Kind::Group,
                    name,
                    group_span,
                    group_children,
                ));
            }
            continue;
        }

        // Bare expression
        let span = lines.current_span();
        children.push(ast::ast_leaf(Kind::Expr, trimmed, span));
        lines.advance();
    }

    Err(ParseError {
        message: "unclosed block".into(),
        span: Some(open_span),
    })
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
                children.push(ast::ast_leaf(Kind::TemplateRef, name, span));
            }
        }
    } else {
        children.push(ast::ast_leaf(Kind::TemplateRef, names_part, span));
    }

    // Parse source: `@domain` possibly followed by `sha: ABC`
    let (domain, sha_param) = match source_part.split_once(" sha: ") {
        Some((d, s)) => (d.trim(), Some(format!("sha: {}", s.trim()))),
        None => (source_part, None),
    };

    if !domain.is_empty() {
        children.push(ast::ast_leaf(Kind::DomainRef, domain, span));
    }

    if let Some(param) = sha_param {
        children.push(ast::ast_leaf(Kind::DomainParam, param, span));
    }

    ast::ast_branch(Kind::Use, "use", span, children)
}

/// Parse a when predicate: `error.rate > 0.1`, `status == "active"`, etc.
///
/// Operator detection: two-char operators before single-char to avoid false matches.
/// Structure: When(Op) with Path (left) and Literal (right) as children.
fn parse_when(rest: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
    let ops: &[(&str, Op)] = &[
        (">=", Op::Gte),
        ("<=", Op::Lte),
        ("!=", Op::Ne),
        ("==", Op::Eq),
        (">", Op::Gt),
        ("<", Op::Lt),
    ];
    for (sym, op) in ops {
        if let Some(idx) = rest.find(sym) {
            let path = rest[..idx].trim();
            let literal = rest[idx + sym.len()..].trim();
            let children = vec![
                ast::ast_leaf(Kind::Path, path, span),
                ast::ast_leaf(Kind::Literal, literal, span),
            ];
            return Ok(ast::ast_branch(Kind::When(op.clone()), "", span, children));
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
            return Ok(ast::ast_branch(Kind::Case, subject, span, arms));
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

    let body = ast::ast_leaf(Kind::Expr, body_str, span);

    let pattern = if pattern_str == "_" {
        ast::ast_leaf(Kind::Wild, "", span)
    } else {
        parse_cmp(pattern_str, span)?
    };

    Ok(ast::ast_branch(Kind::Arm, "", span, vec![pattern, body]))
}

/// Parse a comparison pattern: `> 0.1`, `>= 3`, `== "active"`, etc.
///
/// Operator detection: two-char operators before single-char to avoid false matches.
/// The operator must be a prefix of the pattern text.
fn parse_cmp(text: &str, span: Span) -> Result<Tree<AstNode>, ParseError> {
    let ops: &[(&str, Op)] = &[
        (">=", Op::Gte),
        ("<=", Op::Lte),
        ("!=", Op::Ne),
        ("==", Op::Eq),
        (">", Op::Gt),
        ("<", Op::Lt),
    ];
    for (sym, op) in ops {
        if let Some(rest) = text.strip_prefix(sym) {
            let literal = rest.trim();
            return Ok(ast::ast_leaf(Kind::Cmp(op.clone()), literal, span));
        }
    }
    Err(ParseError {
        message: format!("arm: no comparison operator in: {}", text),
        span: Some(span),
    })
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
    ast::ast_branch(Kind::Pipeline, "root", span, children)
}

fn parse_pipeline_segment(seg: &str, span: Span) -> Tree<AstNode> {
    if seg.starts_with('@') {
        // Domain ref, possibly with params: @git(branch: "master")
        if let Some(paren_start) = seg.find('(') {
            let name = &seg[..paren_start];
            let params = seg[paren_start + 1..].trim_end_matches(')');
            let param_child = ast::ast_leaf(Kind::DomainParam, params, span);
            ast::ast_branch(Kind::DomainRef, name, span, vec![param_child])
        } else {
            ast::ast_leaf(Kind::DomainRef, seg, span)
        }
    } else {
        ast::ast_leaf(Kind::Ref, seg, span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Story;
    use fragmentation::fragment::Fragmentable;

    // -- Parse `in @domain` --

    #[test]
    fn parse_in_domain() {
        let source = "in @filesystem\n".to_string();
        let tree = Parse.record(source).unwrap();
        let children = tree.children();
        let in_node = children.iter().find(|c| c.data().kind == Kind::In).unwrap();
        assert!(in_node.is_shard());
        assert_eq!(in_node.data().value, "@filesystem");
    }

    // -- Parse `template $name { fields }` --

    #[test]
    fn parse_template_with_fields() {
        let source = "template $corpus {\n\tslug\n\texcerpt\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let children = tree.children();
        let tmpl = children
            .iter()
            .find(|c| c.data().kind == Kind::Template)
            .unwrap();
        assert!(tmpl.is_fractal());
        assert_eq!(tmpl.data().value, "$corpus");
        assert_eq!(tmpl.children().len(), 2);
        assert_eq!(tmpl.children()[0].data().kind, Kind::Field);
        assert_eq!(tmpl.children()[0].data().value, "slug");
        assert_eq!(tmpl.children()[1].data().value, "excerpt");
    }

    #[test]
    fn parse_field_with_qualifier() {
        let source = "template $t {\n\theadlines: h2\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().kind, Kind::Field);
        assert_eq!(field.data().value, "headlines");
        assert!(field.is_fractal());
        assert_eq!(field.children()[0].data().kind, Kind::Qualifier);
        assert_eq!(field.children()[0].data().value, "h2");
    }

    #[test]
    fn parse_field_with_pipe() {
        let source = "template $t {\n\thtml: article | @html\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().value, "html");
        assert!(field.is_fractal());
        let children = field.children();
        assert_eq!(children[0].data().kind, Kind::Qualifier);
        assert_eq!(children[0].data().value, "article");
        assert_eq!(children[1].data().kind, Kind::Pipe);
        assert_eq!(children[1].data().value, "@html");
    }

    // -- Parse `out name { ... }` --

    #[test]
    fn parse_out_with_group_and_selects() {
        let source = "out blog {\n\tpieces {\n\t\tdraft: 1draft { $corpus }\n\t}\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let out = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Out)
            .unwrap();
        assert_eq!(out.data().value, "blog");
        let group = &out.children()[0];
        assert_eq!(group.data().kind, Kind::Group);
        assert_eq!(group.data().value, "pieces");
        let select = &group.children()[0];
        assert_eq!(select.data().kind, Kind::Select);
        assert_eq!(select.data().value, "draft");
        assert_eq!(select.children().len(), 2);
        assert_eq!(select.children()[0].data().kind, Kind::DomainRef);
        assert_eq!(select.children()[0].data().value, "1draft");
        assert_eq!(select.children()[1].data().kind, Kind::TemplateRef);
        assert_eq!(select.children()[1].data().value, "$corpus");
    }

    // -- Full file parse --

    #[test]
    fn parse_full_conv_file() {
        let source = include_str!("../systemic.engineering.conv").to_string();
        let tree = Parse.record(source).unwrap();

        // Root has children: In, Template, Out
        let children = tree.children();
        let in_node = children.iter().find(|c| c.data().kind == Kind::In).unwrap();
        assert_eq!(in_node.data().value, "@filesystem");

        let tmpl = children
            .iter()
            .find(|c| c.data().kind == Kind::Template)
            .unwrap();
        assert_eq!(tmpl.data().value, "$corpus");
        assert_eq!(tmpl.children().len(), 4); // slug, excerpt, headlines, html

        let out = children
            .iter()
            .find(|c| c.data().kind == Kind::Out)
            .unwrap();
        assert_eq!(out.data().value, "blog");
    }

    // -- Error paths --

    #[test]
    fn parse_without_output_succeeds() {
        // Parser is syntax only. Missing output is a resolver concern.
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        assert!(!tree.children().is_empty());
    }

    #[test]
    fn parse_error_unexpected_line() {
        let source = "garbage\n".to_string();
        let err = Parse.record(source).into_result().unwrap_err();
        assert!(err.span.is_some(), "error should carry a span");
    }

    #[test]
    fn parse_error_unclosed_block() {
        let source = "out blog {\n\tpieces {\n".to_string();
        let err = Parse.record(source).into_result().unwrap_err();
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
        let tree = Parse.record(source).unwrap();
        let in_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::In)
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
        let tree = Parse.record(source).unwrap();
        let out = &tree.children()[0];
        let group = &out.children()[0];
        assert_eq!(group.data().kind, Kind::Group);
        assert_eq!(group.data().value, "empty");
        assert_eq!(group.children().len(), 0);
    }

    #[test]
    fn parse_error_unclosed_template() {
        let source = "template $t {\n\tslug\n".to_string();
        let err = Parse.record(source).into_result().unwrap_err();
        assert!(err.message.contains("unclosed"), "{}", err);
    }

    #[test]
    fn parse_bare_expr_in_output() {
        let source = "out root {\n\tnonsense\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let out = &tree.children()[0];
        let expr = &out.children()[0];
        assert_eq!(expr.data().kind, Kind::Expr);
        assert_eq!(expr.data().value, "nonsense");
    }

    #[test]
    fn parse_blank_lines_and_comments_skipped() {
        let source = "# comment\n\n# another\nin @fs\n".to_string();
        let tree = Parse.record(source).unwrap();
        assert_eq!(tree.children().len(), 1);
        assert_eq!(tree.children()[0].data().value, "@fs");
    }

    #[test]
    fn parse_template_with_blank_lines() {
        let source = "template $t {\n\n\tslug\n\n\texcerpt\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.children().len(), 2);
    }

    #[test]
    fn parse_out_with_blank_lines() {
        let source = "out r {\n\n\tg {\n\n\t\tx: f { $t }\n\n\t}\n\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.children().len(), 1);
    }

    #[test]
    fn parse_json_fixture() {
        let source = include_str!("../fixtures/json.conv");
        let tree = Parse.record(source.to_string()).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.data().kind, Kind::Out);
        assert_eq!(out.data().value, "@json");
        assert!(out.is_shard());
    }

    #[test]
    fn parse_parameterized_in() {
        let source = "in @git(branch: \"main\")\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let in_node = &tree.children()[0];
        assert_eq!(in_node.data().kind, Kind::In);
        assert_eq!(in_node.data().value, "@git");
        assert!(in_node.is_fractal());
        assert_eq!(in_node.children()[0].data().kind, Kind::DomainParam);
        assert_eq!(in_node.children()[0].data().value, "branch: \"main\"");
    }

    #[test]
    fn parse_coverage_fixture() {
        let source = include_str!("../fixtures/coverage-on-last-3-main-commits.conv");
        let tree = Parse.record(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children.len(), 2); // in + pipeline

        let in_node = &children[0];
        assert_eq!(in_node.data().kind, Kind::In);
        assert_eq!(in_node.data().value, "@git");
        assert_eq!(in_node.children()[0].data().kind, Kind::DomainParam);
        assert_eq!(in_node.children()[0].data().value, "branch: \"main\"");

        let pipeline = &children[1];
        assert_eq!(pipeline.data().kind, Kind::Pipeline);
        assert_eq!(pipeline.children().len(), 2);
    }

    #[test]
    fn parse_bare_out() {
        let source = "out @json\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.data().kind, Kind::Out);
        assert_eq!(out.data().value, "@json");
        assert!(out.is_shard());
    }

    #[test]
    fn parse_empty_source() {
        let source = "".to_string();
        let tree = Parse.record(source).unwrap();
        assert_eq!(tree.children().len(), 0);
    }

    #[test]
    fn parse_field_expr_in_output() {
        let source = "out root {\n\tlabel: value\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let out = &tree.children()[0];
        let field = &out.children()[0];
        assert_eq!(field.data().kind, Kind::Field);
        assert_eq!(field.data().value, "label");
        assert_eq!(field.children()[0].data().kind, Kind::Expr);
        assert_eq!(field.children()[0].data().value, "value");
    }

    // -- Pipeline: A | G | B --

    #[test]
    fn parse_commit_from_main_to_test_fixture() {
        let source = include_str!("../fixtures/commit-from-main-to-test.conv");
        let tree = Parse.record(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Pipeline);
        assert_eq!(pipeline.children().len(), 3);
    }

    #[test]
    fn parse_additive_fixture() {
        let source = include_str!("../fixtures/additive.conv");
        let tree = Parse.record(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children.len(), 3); // two ins + one out

        assert_eq!(children[0].data().kind, Kind::In);
        assert_eq!(children[0].data().value, "@number");
        assert_eq!(children[0].children()[0].data().value, "$a");

        assert_eq!(children[1].data().kind, Kind::In);
        assert_eq!(children[1].data().value, "@number");
        assert_eq!(children[1].children()[0].data().value, "$b");

        let out = &children[2];
        assert_eq!(out.data().kind, Kind::Out);
        assert_eq!(out.data().value, "");
        assert_eq!(out.children().len(), 3);
    }

    #[test]
    fn parse_aliased_in() {
        let source = "in @number as $a\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let in_node = &tree.children()[0];
        assert_eq!(in_node.data().kind, Kind::In);
        assert_eq!(in_node.data().value, "@number");
        assert!(in_node.is_fractal());
        assert_eq!(in_node.children()[0].data().kind, Kind::Alias);
        assert_eq!(in_node.children()[0].data().value, "$a");
    }

    #[test]
    fn parse_anonymous_out_with_exprs() {
        let source = "out {\n\tsimple: $a + $b\n\tcurried + $b\n\tmagic: +\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.data().kind, Kind::Out);
        assert_eq!(out.data().value, "");
        assert_eq!(out.children().len(), 3);

        let simple = &out.children()[0];
        assert_eq!(simple.data().kind, Kind::Field);
        assert_eq!(simple.data().value, "simple");
        assert_eq!(simple.children()[0].data().kind, Kind::Expr);
        assert_eq!(simple.children()[0].data().value, "$a + $b");

        let curried = &out.children()[1];
        assert_eq!(curried.data().kind, Kind::Expr);
        assert_eq!(curried.data().value, "curried + $b");

        let magic = &out.children()[2];
        assert_eq!(magic.data().kind, Kind::Field);
        assert_eq!(magic.data().value, "magic");
        assert_eq!(magic.children()[0].data().kind, Kind::Expr);
        assert_eq!(magic.children()[0].data().value, "+");
    }

    // -- Parse `use` imports --

    #[test]
    fn parse_use_single() {
        let source = "use $corpus from @shared\n".to_string();
        let tree = Parse.record(source).unwrap();
        let children = tree.children();
        let use_node = children
            .iter()
            .find(|c| c.data().kind == Kind::Use)
            .unwrap();
        assert!(use_node.is_fractal());
        assert_eq!(use_node.data().value, "use");
        let use_children = use_node.children();
        assert_eq!(use_children.len(), 2);
        assert_eq!(use_children[0].data().kind, Kind::TemplateRef);
        assert_eq!(use_children[0].data().value, "$corpus");
        assert_eq!(use_children[1].data().kind, Kind::DomainRef);
        assert_eq!(use_children[1].data().value, "@shared");
    }

    #[test]
    fn parse_use_destructured() {
        let source = "use { $a, $b } from @other\n".to_string();
        let tree = Parse.record(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Use)
            .unwrap();
        let use_children = use_node.children();
        assert_eq!(use_children.len(), 3); // $a, $b, @other
        assert_eq!(use_children[0].data().kind, Kind::TemplateRef);
        assert_eq!(use_children[0].data().value, "$a");
        assert_eq!(use_children[1].data().kind, Kind::TemplateRef);
        assert_eq!(use_children[1].data().value, "$b");
        assert_eq!(use_children[2].data().kind, Kind::DomainRef);
        assert_eq!(use_children[2].data().value, "@other");
    }

    #[test]
    fn parse_use_with_sha_lock() {
        let source = "use $garden from @garden@systemic.engineering sha: ABC123\n".to_string();
        let tree = Parse.record(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Use)
            .unwrap();
        let use_children = use_node.children();
        // $garden, @garden@systemic.engineering, sha param
        assert_eq!(use_children[0].data().kind, Kind::TemplateRef);
        assert_eq!(use_children[0].data().value, "$garden");
        assert_eq!(use_children[1].data().kind, Kind::DomainRef);
        assert_eq!(use_children[1].data().value, "@garden@systemic.engineering");
        assert_eq!(use_children[2].data().kind, Kind::DomainParam);
        assert_eq!(use_children[2].data().value, "sha: ABC123");
    }

    #[test]
    fn parse_use_without_from() {
        // Malformed: no "from" keyword — still parses, just has no domain ref
        let source = "use $orphan\n".to_string();
        let tree = Parse.record(source).unwrap();
        let use_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Kind::Use)
            .unwrap();
        let use_children = use_node.children();
        assert_eq!(use_children.len(), 1);
        assert_eq!(use_children[0].data().kind, Kind::TemplateRef);
        assert_eq!(use_children[0].data().value, "$orphan");
    }

    #[test]
    fn parse_use_appears_before_template() {
        let source =
            "use $t from @shared\ntemplate $local {\n\tslug\n}\nout r {\n\tx: f { $local }\n}\n"
                .to_string();
        let tree = Parse.record(source).unwrap();
        let children = tree.children();
        assert_eq!(children[0].data().kind, Kind::Use);
        assert_eq!(children[1].data().kind, Kind::Template);
        assert_eq!(children[2].data().kind, Kind::Out);
    }

    // -- Parse `when` predicates --

    #[test]
    fn parse_when_greater_than() {
        let node = parse_when("error.rate > 0.1", Span::new(0, 20)).unwrap();
        assert_eq!(node.data().kind, Kind::When(Op::Gt));
        assert_eq!(node.children()[0].data().kind, Kind::Path);
        assert_eq!(node.children()[0].data().value, "error.rate");
        assert_eq!(node.children()[1].data().kind, Kind::Literal);
        assert_eq!(node.children()[1].data().value, "0.1");
    }

    #[test]
    fn parse_when_equals() {
        let node = parse_when("status == \"active\"", Span::new(0, 20)).unwrap();
        assert_eq!(node.data().kind, Kind::When(Op::Eq));
        assert_eq!(node.children()[0].data().value, "status");
        assert_eq!(node.children()[1].data().value, "\"active\"");
    }

    #[test]
    fn parse_when_less_equal() {
        let node = parse_when("count <= 10", Span::new(0, 15)).unwrap();
        assert_eq!(node.data().kind, Kind::When(Op::Lte));
        assert_eq!(node.children()[0].data().value, "count");
        assert_eq!(node.children()[1].data().value, "10");
    }

    #[test]
    fn parse_when_not_equal() {
        let node = parse_when("mode != \"debug\"", Span::new(0, 20)).unwrap();
        assert_eq!(node.data().kind, Kind::When(Op::Ne));
        assert_eq!(node.children()[0].data().value, "mode");
        assert_eq!(node.children()[1].data().value, "\"debug\"");
    }

    #[test]
    fn parse_when_dotted_path() {
        let node = parse_when("error.rate.current > 0.5", Span::new(0, 25)).unwrap();
        assert_eq!(node.data().kind, Kind::When(Op::Gt));
        assert_eq!(node.children()[0].data().kind, Kind::Path);
        assert_eq!(node.children()[0].data().value, "error.rate.current");
        assert_eq!(node.children()[1].data().value, "0.5");
    }

    #[test]
    fn parse_when_appears_between_in_and_out() {
        let source = "in @datadog\nwhen error.rate > 0.1\nout r {\n\tx {}\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let children = tree.children();
        let when_node = children
            .iter()
            .find(|c| matches!(c.data().kind, Kind::When(_)))
            .unwrap();
        assert_eq!(when_node.data().kind, Kind::When(Op::Gt));
    }

    #[test]
    fn parse_bare_field_with_pipe() {
        // "slug | @sha" in a template: bare field (no colon) with a pipe
        let source = "template $t {\n\tslug | @sha\n}\n".to_string();
        let tree = Parse.record(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().kind, Kind::Field);
        assert_eq!(field.data().value, "slug");
        assert!(field.is_fractal());
        assert_eq!(field.children()[0].data().kind, Kind::Pipe);
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
            .record("when active\n".to_string())
            .into_result()
            .unwrap_err();
        assert!(err.message.contains("active"), "{}", err.message);
    }

    #[test]
    fn parse_pipeline_bare_domain() {
        let source = "@fs | data | @json\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Pipeline);
        assert_eq!(pipeline.children().len(), 3);

        let left = &pipeline.children()[0];
        assert_eq!(left.data().kind, Kind::DomainRef);
        assert_eq!(left.data().value, "@fs");
        assert!(left.children().is_empty());

        let mid = &pipeline.children()[1];
        assert_eq!(mid.data().kind, Kind::Ref);
        assert_eq!(mid.data().value, "data");

        let right = &pipeline.children()[2];
        assert_eq!(right.data().kind, Kind::DomainRef);
        assert_eq!(right.data().value, "@json");
        assert!(right.children().is_empty());
    }

    #[test]
    fn parse_pipeline() {
        let source = "@git(branch: \"master\") | HEAD | @git(branch: \"test\")\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Pipeline);
        assert_eq!(pipeline.children().len(), 3);

        let left = &pipeline.children()[0];
        assert_eq!(left.data().kind, Kind::DomainRef);
        assert_eq!(left.data().value, "@git");
        assert_eq!(left.children().len(), 1);
        assert_eq!(left.children()[0].data().kind, Kind::DomainParam);
        assert_eq!(left.children()[0].data().value, "branch: \"master\"");

        let mid = &pipeline.children()[1];
        assert_eq!(mid.data().kind, Kind::Ref);
        assert_eq!(mid.data().value, "HEAD");

        let right = &pipeline.children()[2];
        assert_eq!(right.data().kind, Kind::DomainRef);
        assert_eq!(right.data().value, "@git");
        assert_eq!(right.children().len(), 1);
        assert_eq!(right.children()[0].data().kind, Kind::DomainParam);
        assert_eq!(right.children()[0].data().value, "branch: \"test\"");
    }

    // -- Parse `case` dispatch --

    #[test]
    fn parse_case_single_arm() {
        let node = parse_case("x {", &mut Lines::new("x {\n  > 1 -> a\n}\n")).unwrap();
        assert_eq!(node.data().kind, Kind::Case);
        assert_eq!(node.data().value, "x");
        assert_eq!(node.children().len(), 1);
        let arm = &node.children()[0];
        assert_eq!(arm.data().kind, Kind::Arm);
        assert_eq!(arm.children().len(), 2);
        assert_eq!(arm.children()[0].data().kind, Kind::Cmp(Op::Gt));
        assert_eq!(arm.children()[0].data().value, "1");
        assert_eq!(arm.children()[1].data().kind, Kind::Expr);
        assert_eq!(arm.children()[1].data().value, "a");
    }

    #[test]
    fn parse_case_wildcard_arm() {
        let node = parse_case("x {", &mut Lines::new("x {\n  _ -> b\n}\n")).unwrap();
        assert_eq!(node.children().len(), 1);
        let arm = &node.children()[0];
        assert_eq!(arm.data().kind, Kind::Arm);
        assert_eq!(arm.children()[0].data().kind, Kind::Wild);
        assert_eq!(arm.children()[1].data().kind, Kind::Expr);
        assert_eq!(arm.children()[1].data().value, "b");
    }

    #[test]
    fn parse_case_multiple_arms() {
        let source =
            "case error.rate {\n  > 0.1 -> critical\n  > 0.05 -> warning\n  _ -> pass\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let case_node = &tree.children()[0];
        assert_eq!(case_node.data().kind, Kind::Case);
        assert_eq!(case_node.data().value, "error.rate");
        assert_eq!(case_node.children().len(), 3);

        let arm0 = &case_node.children()[0];
        assert_eq!(arm0.children()[0].data().kind, Kind::Cmp(Op::Gt));
        assert_eq!(arm0.children()[0].data().value, "0.1");
        assert_eq!(arm0.children()[1].data().value, "critical");

        let arm1 = &case_node.children()[1];
        assert_eq!(arm1.children()[0].data().kind, Kind::Cmp(Op::Gt));
        assert_eq!(arm1.children()[0].data().value, "0.05");
        assert_eq!(arm1.children()[1].data().value, "warning");

        let arm2 = &case_node.children()[2];
        assert_eq!(arm2.children()[0].data().kind, Kind::Wild);
        assert_eq!(arm2.children()[1].data().value, "pass");
    }

    #[test]
    fn parse_case_all_cmp_ops() {
        let source = "case x {\n  > 1 -> a\n  < 2 -> b\n  >= 3 -> c\n  <= 4 -> d\n  == 5 -> e\n  != 6 -> f\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let case_node = &tree.children()[0];
        assert_eq!(case_node.children().len(), 6);
        assert_eq!(
            case_node.children()[0].children()[0].data().kind,
            Kind::Cmp(Op::Gt)
        );
        assert_eq!(
            case_node.children()[1].children()[0].data().kind,
            Kind::Cmp(Op::Lt)
        );
        assert_eq!(
            case_node.children()[2].children()[0].data().kind,
            Kind::Cmp(Op::Gte)
        );
        assert_eq!(
            case_node.children()[3].children()[0].data().kind,
            Kind::Cmp(Op::Lte)
        );
        assert_eq!(
            case_node.children()[4].children()[0].data().kind,
            Kind::Cmp(Op::Eq)
        );
        assert_eq!(
            case_node.children()[5].children()[0].data().kind,
            Kind::Cmp(Op::Ne)
        );
    }

    #[test]
    fn parse_case_appears_in_source() {
        let source =
            "in @datadog\ncase error.rate {\n  > 0.1 -> alert\n  _ -> pass\n}\nout @json\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children[0].data().kind, Kind::In);
        assert_eq!(children[1].data().kind, Kind::Case);
        assert_eq!(children[2].data().kind, Kind::Out);
    }

    #[test]
    fn parse_case_error_no_arrow() {
        let source = "case x {\n  > 1 oops\n}\n";
        let err = Parse.record(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("->"),
            "expected mention of '->': {}",
            err.message
        );
    }

    #[test]
    fn parse_case_error_no_operator() {
        let source = "case x {\n  1 -> a\n}\n";
        let err = Parse.record(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("operator") || err.message.contains("1"),
            "expected mention of operator: {}",
            err.message
        );
    }

    #[test]
    fn parse_case_with_blank_lines() {
        let source = "case x {\n\n  > 1 -> a\n\n  _ -> b\n\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let case_node = &tree.children()[0];
        assert_eq!(case_node.data().kind, Kind::Case);
        assert_eq!(case_node.children().len(), 2);
    }

    #[test]
    fn parse_case_error_unclosed() {
        let source = "case x {\n  > 1 -> a\n";
        let err = Parse.record(source.to_string()).into_result().unwrap_err();
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
        let tree = Parse.record(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Branch);
        assert_eq!(branch.data().value, ".action");
        assert_eq!(branch.children().len(), 1);
        let arm = &branch.children()[0];
        assert_eq!(arm.data().kind, Kind::Arm);
        assert_eq!(arm.children()[0].data().kind, Kind::Literal);
        assert_eq!(arm.children()[0].data().value, "hold");
        assert_eq!(arm.children()[1].data().kind, Kind::Expr);
        assert_eq!(arm.children()[1].data().value, "..");
    }

    #[test]
    fn parse_branch_multiple_arms() {
        let source = "branch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Branch);
        assert_eq!(branch.data().value, ".action");
        assert_eq!(branch.children().len(), 2);

        let arm0 = &branch.children()[0];
        assert_eq!(arm0.children()[0].data().kind, Kind::Literal);
        assert_eq!(arm0.children()[0].data().value, "hold");
        assert_eq!(arm0.children()[1].data().value, "..");

        let arm1 = &branch.children()[1];
        assert_eq!(arm1.children()[0].data().kind, Kind::Literal);
        assert_eq!(arm1.children()[0].data().value, "exit");
        assert_eq!(arm1.children()[1].data().value, "exit");
    }

    #[test]
    fn parse_branch_wildcard_arm() {
        let source = "branch(.status) {\n  \"ok\" => ..\n  _ => exit\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Branch);
        assert_eq!(branch.children().len(), 2);
        let wild_arm = &branch.children()[1];
        assert_eq!(wild_arm.children()[0].data().kind, Kind::Wild);
        assert_eq!(wild_arm.children()[1].data().value, "exit");
    }

    #[test]
    fn parse_branch_in_pipeline() {
        let source = "@json | branch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let pipeline = &tree.children()[0];
        assert_eq!(pipeline.data().kind, Kind::Pipeline);
        assert_eq!(pipeline.children().len(), 2);

        let domain = &pipeline.children()[0];
        assert_eq!(domain.data().kind, Kind::DomainRef);
        assert_eq!(domain.data().value, "@json");

        let branch = &pipeline.children()[1];
        assert_eq!(branch.data().kind, Kind::Branch);
        assert_eq!(branch.data().value, ".action");
        assert_eq!(branch.children().len(), 2);
    }

    #[test]
    fn parse_branch_with_blank_lines() {
        let source = "branch(.x) {\n\n  \"a\" => ..\n\n  \"b\" => exit\n\n}\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let branch = &tree.children()[0];
        assert_eq!(branch.data().kind, Kind::Branch);
        assert_eq!(branch.children().len(), 2);
    }

    #[test]
    fn parse_branch_error_unclosed() {
        let source = "branch(.x) {\n  \"a\" => ..\n";
        let err = Parse.record(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("unclosed"),
            "expected 'unclosed': {}",
            err.message
        );
    }

    #[test]
    fn parse_branch_error_no_arrow() {
        let source = "branch(.x) {\n  \"a\" oops\n}\n";
        let err = Parse.record(source.to_string()).into_result().unwrap_err();
        assert!(
            err.message.contains("=>"),
            "expected mention of '=>': {}",
            err.message
        );
    }

    #[test]
    fn parse_branch_appears_in_source() {
        let source =
            "in @json\nbranch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\nout @json\n";
        let tree = Parse.record(source.to_string()).unwrap();
        let children = tree.children();
        assert_eq!(children[0].data().kind, Kind::In);
        assert_eq!(children[1].data().kind, Kind::Branch);
        assert_eq!(children[2].data().kind, Kind::Out);
    }
}
