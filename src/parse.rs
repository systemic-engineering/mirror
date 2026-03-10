//! Parser gradient. Source text → AST tree.
//!
//! The parser IS a gradient: emit parses, absorb unparses.
//! Wrap in Witnessed to observe every parse through a Session.

use crate::ast::{AstNode, Span};
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
        todo!()
    }

    fn absorb(&self, _source: Tree<AstNode>) -> Result<String, ParseError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
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
    fn parse_error_no_output() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n".to_string();
        let err = Parse.emit(source).unwrap_err();
        assert!(
            err.message.contains("output"),
            "error should mention missing output: {}",
            err
        );
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
}
