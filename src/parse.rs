//! Parser gradient. Source text → AST tree.
//!
//! The parser IS a gradient: emit parses, absorb unparses.
//! Wrap in Witnessed to observe every parse through a Session.

use crate::ast::{self, AstNode, Span};
use crate::domain::conversation::Language;
use crate::gradient::Gradient;
use crate::tree::Tree;

/// The parse gradient. Source → AST.
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

impl Gradient<String, Tree<AstNode>> for Parse {
    type Error = ParseError;

    fn emit(&self, source: String) -> Result<Tree<AstNode>, ParseError> {
        parse_source(&source)
    }

    fn absorb(&self, _source: Tree<AstNode>) -> Result<String, ParseError> {
        Err(ParseError {
            message: "unparse not yet implemented".into(),
            span: None,
        })
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
            let value = rest.trim();
            children.push(ast::ast_leaf(Language::In, value, span));
            lines.advance();
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

        return Err(ParseError {
            message: format!("unexpected: {}", trimmed),
            span: Some(lines.current_span()),
        });
    }

    Ok(ast::ast_branch(
        Language::Group,
        "root",
        root_span,
        children,
    ))
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
            return Ok(ast::ast_branch(Language::Template, name, span, fields));
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
        children.push(ast::ast_leaf(Language::Qualifier, qualifier, span));

        if parts.len() > 1 {
            let pipe_value = parts[1].trim();
            children.push(ast::ast_leaf(Language::Pipe, pipe_value, span));
        }

        ast::ast_branch(Language::Field, name, span, children)
    } else {
        ast::ast_leaf(Language::Field, text.trim(), span)
    }
}

fn parse_out(header: &str, lines: &mut Lines) -> Result<Tree<AstNode>, ParseError> {
    let name = header.split('{').next().unwrap().trim();
    let start_span = lines.current_span();
    lines.advance(); // consume out line

    let (children, end_span) = parse_block_body(lines, start_span)?;
    let span = start_span.merge(&end_span);
    Ok(ast::ast_branch(Language::Out, name, span, children))
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
        if let Some((output_part, rest)) = trimmed.split_once(':') {
            let rest = rest.trim();
            if let Some((folder, template_part)) = rest.split_once('{') {
                let span = lines.current_span();
                let folder_name = folder.trim();
                let template_name = template_part.trim().trim_end_matches('}').trim();

                let select_children = vec![
                    ast::ast_leaf(Language::DomainRef, folder_name, span),
                    ast::ast_leaf(Language::TemplateRef, template_name, span),
                ];
                children.push(ast::ast_branch(
                    Language::Select,
                    output_part.trim(),
                    span,
                    select_children,
                ));
                lines.advance();
                continue;
            }
        }

        // Group: "name {"
        if let Some((name, rest)) = trimmed.split_once('{') {
            let name = name.trim();
            let span = lines.current_span();

            if rest.trim() == "}" {
                // Empty group: "name {}"
                children.push(ast::ast_branch(Language::Group, name, span, vec![]));
                lines.advance();
            } else {
                lines.advance();
                let (group_children, end_span) = parse_block_body(lines, span)?;
                let group_span = span.merge(&end_span);
                children.push(ast::ast_branch(
                    Language::Group,
                    name,
                    group_span,
                    group_children,
                ));
            }
            continue;
        }

        return Err(ParseError {
            message: format!("unexpected output line: {}", trimmed),
            span: Some(lines.current_span()),
        });
    }

    Err(ParseError {
        message: "unclosed block".into(),
        span: Some(open_span),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::Gradient;
    use fragmentation::fragment::Fragmentable;

    // -- Parse `in @domain` --

    #[test]
    fn parse_in_domain() {
        let source = "in @filesystem\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let children = tree.children();
        let in_node = children
            .iter()
            .find(|c| c.data().kind == Language::In)
            .unwrap();
        assert!(in_node.is_shard());
        assert_eq!(in_node.data().value, "@filesystem");
    }

    // -- Parse `template $name { fields }` --

    #[test]
    fn parse_template_with_fields() {
        let source = "template $corpus {\n\tslug\n\texcerpt\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let children = tree.children();
        let tmpl = children
            .iter()
            .find(|c| c.data().kind == Language::Template)
            .unwrap();
        assert!(tmpl.is_fractal());
        assert_eq!(tmpl.data().value, "$corpus");
        assert_eq!(tmpl.children().len(), 2);
        assert_eq!(tmpl.children()[0].data().kind, Language::Field);
        assert_eq!(tmpl.children()[0].data().value, "slug");
        assert_eq!(tmpl.children()[1].data().value, "excerpt");
    }

    #[test]
    fn parse_field_with_qualifier() {
        let source = "template $t {\n\theadlines: h2\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().kind, Language::Field);
        assert_eq!(field.data().value, "headlines");
        assert!(field.is_fractal());
        assert_eq!(field.children()[0].data().kind, Language::Qualifier);
        assert_eq!(field.children()[0].data().value, "h2");
    }

    #[test]
    fn parse_field_with_pipe() {
        let source = "template $t {\n\thtml: article | @html\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let tmpl = &tree.children()[0];
        let field = &tmpl.children()[0];
        assert_eq!(field.data().value, "html");
        assert!(field.is_fractal());
        let children = field.children();
        assert_eq!(children[0].data().kind, Language::Qualifier);
        assert_eq!(children[0].data().value, "article");
        assert_eq!(children[1].data().kind, Language::Pipe);
        assert_eq!(children[1].data().value, "@html");
    }

    // -- Parse `out name { ... }` --

    #[test]
    fn parse_out_with_group_and_selects() {
        let source = "out blog {\n\tpieces {\n\t\tdraft: 1draft { $corpus }\n\t}\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let out = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Language::Out)
            .unwrap();
        assert_eq!(out.data().value, "blog");
        let group = &out.children()[0];
        assert_eq!(group.data().kind, Language::Group);
        assert_eq!(group.data().value, "pieces");
        let select = &group.children()[0];
        assert_eq!(select.data().kind, Language::Select);
        assert_eq!(select.data().value, "draft");
        assert_eq!(select.children().len(), 2);
        assert_eq!(select.children()[0].data().kind, Language::DomainRef);
        assert_eq!(select.children()[0].data().value, "1draft");
        assert_eq!(select.children()[1].data().kind, Language::TemplateRef);
        assert_eq!(select.children()[1].data().value, "$corpus");
    }

    // -- Full file parse --

    #[test]
    fn parse_full_conv_file() {
        let source = include_str!("../systemic.engineering.conv").to_string();
        let tree = Parse.emit(source).unwrap();

        // Root has children: In, Template, Out
        let children = tree.children();
        let in_node = children
            .iter()
            .find(|c| c.data().kind == Language::In)
            .unwrap();
        assert_eq!(in_node.data().value, "@filesystem");

        let tmpl = children
            .iter()
            .find(|c| c.data().kind == Language::Template)
            .unwrap();
        assert_eq!(tmpl.data().value, "$corpus");
        assert_eq!(tmpl.children().len(), 4); // slug, excerpt, headlines, html

        let out = children
            .iter()
            .find(|c| c.data().kind == Language::Out)
            .unwrap();
        assert_eq!(out.data().value, "blog");
    }

    // -- Error paths --

    #[test]
    fn parse_without_output_succeeds() {
        // Parser is syntax only. Missing output is a resolver concern.
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        assert!(!tree.children().is_empty());
    }

    #[test]
    fn parse_error_unexpected_line() {
        let source = "garbage\n".to_string();
        let err = Parse.emit(source).unwrap_err();
        assert!(err.span.is_some(), "error should carry a span");
    }

    #[test]
    fn parse_error_unclosed_block() {
        let source = "out blog {\n\tpieces {\n".to_string();
        let err = Parse.emit(source).unwrap_err();
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
        let tree = Parse.emit(source).unwrap();
        let in_node = tree
            .children()
            .iter()
            .find(|c| c.data().kind == Language::In)
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

    // -- absorb --

    #[test]
    fn absorb_not_yet_implemented() {
        let source = "in @filesystem\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let err = Parse.absorb(tree).unwrap_err();
        assert!(err.message.contains("not yet implemented"));
    }

    // -- Coverage: empty group, unclosed template, unexpected output --

    #[test]
    fn parse_empty_group() {
        let source = "out root {\n\tempty {}\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let out = &tree.children()[0];
        let group = &out.children()[0];
        assert_eq!(group.data().kind, Language::Group);
        assert_eq!(group.data().value, "empty");
        assert_eq!(group.children().len(), 0);
    }

    #[test]
    fn parse_error_unclosed_template() {
        let source = "template $t {\n\tslug\n".to_string();
        let err = Parse.emit(source).unwrap_err();
        assert!(err.message.contains("unclosed"), "{}", err);
    }

    #[test]
    fn parse_error_unexpected_output_line() {
        let source = "out root {\n\tnonsense\n}\n".to_string();
        let err = Parse.emit(source).unwrap_err();
        assert!(err.message.contains("unexpected output line"), "{}", err);
    }

    #[test]
    fn parse_blank_lines_and_comments_skipped() {
        let source = "# comment\n\n# another\nin @fs\n".to_string();
        let tree = Parse.emit(source).unwrap();
        assert_eq!(tree.children().len(), 1);
        assert_eq!(tree.children()[0].data().value, "@fs");
    }

    #[test]
    fn parse_template_with_blank_lines() {
        let source = "template $t {\n\n\tslug\n\n\texcerpt\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let tmpl = &tree.children()[0];
        assert_eq!(tmpl.children().len(), 2);
    }

    #[test]
    fn parse_out_with_blank_lines() {
        let source = "out r {\n\n\tg {\n\n\t\tx: f { $t }\n\n\t}\n\n}\n".to_string();
        let tree = Parse.emit(source).unwrap();
        let out = &tree.children()[0];
        assert_eq!(out.children().len(), 1);
    }

    #[test]
    fn parse_empty_source() {
        let source = "".to_string();
        let tree = Parse.emit(source).unwrap();
        assert_eq!(tree.children().len(), 0);
    }

    #[test]
    fn parse_colon_without_brace_in_output() {
        // "label: value" without { should be an unexpected output line
        let source = "out root {\n\tlabel: value\n}\n".to_string();
        let err = Parse.emit(source).unwrap_err();
        assert!(err.message.contains("unexpected output line"), "{}", err);
    }
}
