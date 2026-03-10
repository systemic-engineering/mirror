//! Resolve gradient. AST → validated program.
//!
//! The resolver IS a gradient: emit resolves, absorb reconstructs AST.
//! Validates domain references, template references, output structure.
//! Errors carry spans and did-you-mean hints.

use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::ast::{AstNode, Span};
use crate::domain::conversation::Language;
use crate::gradient::Gradient;
use crate::tree::{Tree, Treelike};

use crate::conv::Folder;

/// The resolve gradient. AST → Resolved.
///
/// Holds the set of known domain names.
/// Default includes "filesystem".
#[derive(Clone, Debug)]
pub struct Resolve {
    domains: HashSet<String>,
}

/// What can go wrong during resolution.
#[derive(Clone, Debug)]
pub struct ResolveError {
    pub message: String,
    pub span: Option<Span>,
    pub hints: Vec<String>,
}

/// A resolved .conv file. Validated and ready to execute.
#[derive(Debug)]
pub struct Resolved {
    #[allow(dead_code)] // read in tests; used by future domain-aware execution
    domain: String,
    templates: HashMap<String, ResolvedTemplate>,
    output_name: String,
    output: Vec<OutputNode>,
}

#[derive(Debug)]
pub struct ResolvedTemplate {
    fields: Vec<ResolvedField>,
}

#[derive(Debug)]
pub struct ResolvedField {
    name: String,
    qualifier: Option<String>,
    #[allow(dead_code)] // read in tests; used by future pipe execution
    pipe: Option<String>,
}

#[derive(Debug)]
pub enum OutputNode {
    Group {
        name: String,
        children: Vec<OutputNode>,
    },
    Select {
        output_name: String,
        folder_name: String,
        template_name: String,
    },
}

impl Resolve {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        todo!()
    }
}

impl Default for Resolve {
    fn default() -> Self {
        Self::new()
    }
}

impl Gradient<Tree<AstNode>, Resolved> for Resolve {
    type Error = ResolveError;

    fn emit(&self, source: Tree<AstNode>) -> Result<Resolved, ResolveError> {
        todo!()
    }

    fn absorb(&self, _source: Resolved) -> Result<Tree<AstNode>, ResolveError> {
        todo!()
    }
}

impl Resolved {
    /// Execute the resolved program against a filesystem tree.
    pub fn execute(&self, tree: &Tree<Folder>) -> Value {
        todo!()
    }
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ResolveError {}

fn did_you_mean<'a>(name: &str, candidates: &[&'a str]) -> Option<&'a str> {
    todo!()
}

fn edit_distance(a: &str, b: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::Gradient;
    use crate::parse::Parse;
    use crate::tree;
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    fn test_ref(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    fn leaf_folder(name: &str, content: &str) -> Tree<Folder> {
        tree::leaf(
            test_ref(name),
            Folder {
                name: name.into(),
                content: Some(content.into()),
            },
        )
    }

    fn dir_folder(name: &str, children: Vec<Tree<Folder>>) -> Tree<Folder> {
        tree::branch(
            test_ref(name),
            Folder {
                name: name.into(),
                content: None,
            },
            children,
        )
    }

    // -- Resolve valid conv --

    #[test]
    fn resolve_valid_conv() {
        let source = "in @filesystem\ntemplate $corpus {\n\tslug\n\texcerpt\n}\nout blog {\n\tpieces {\n\t\tdraft: 1draft { $corpus }\n\t}\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        assert_eq!(resolved.domain, "filesystem");
        assert_eq!(resolved.output_name, "blog");
        assert!(resolved.templates.contains_key("$corpus"));
    }

    #[test]
    fn resolve_extracts_template_fields() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n\theadlines: h2\n\thtml: article | @html\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        let tmpl = &resolved.templates["$t"];
        assert_eq!(tmpl.fields.len(), 3);
        assert_eq!(tmpl.fields[0].name, "slug");
        assert!(tmpl.fields[0].qualifier.is_none());
        assert_eq!(tmpl.fields[1].name, "headlines");
        assert_eq!(tmpl.fields[1].qualifier.as_deref(), Some("h2"));
        assert_eq!(tmpl.fields[2].name, "html");
        assert_eq!(tmpl.fields[2].qualifier.as_deref(), Some("article"));
        assert_eq!(tmpl.fields[2].pipe.as_deref(), Some("@html"));
    }

    // -- Error: unknown domain --

    #[test]
    fn resolve_unknown_domain_errors() {
        let source = "in @filesytem\ntemplate $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let err = Resolve::new().emit(ast).unwrap_err();
        assert!(err.message.contains("filesytem"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest @filesystem");
        assert!(err.hints[0].contains("filesystem"), "{}", err.hints[0]);
    }

    // -- Error: unknown template --

    #[test]
    fn resolve_unknown_template_errors() {
        let source = "in @filesystem\ntemplate $corpus {\n\tslug\n}\nout blog {\n\tpieces {\n\t\tdraft: 1draft { $coprus }\n\t}\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let err = Resolve::new().emit(ast).unwrap_err();
        assert!(err.message.contains("coprus"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest $corpus");
    }

    // -- Error: missing output --

    #[test]
    fn resolve_missing_output_errors() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let err = Resolve::new().emit(ast).unwrap_err();
        assert!(err.message.contains("output"), "{}", err);
    }

    // -- did_you_mean --

    #[test]
    fn did_you_mean_finds_close_match() {
        let result = did_you_mean("filesytem", &["filesystem", "html", "json"]);
        assert_eq!(result, Some("filesystem"));
    }

    #[test]
    fn did_you_mean_no_match_when_too_far() {
        let result = did_you_mean("xyz", &["filesystem", "html", "json"]);
        assert_eq!(result, None);
    }

    // -- edit_distance --

    #[test]
    fn edit_distance_identical() {
        assert_eq!(edit_distance("abc", "abc"), 0);
    }

    #[test]
    fn edit_distance_single_edit() {
        assert_eq!(edit_distance("filesystem", "filesytem"), 1);
    }

    #[test]
    fn edit_distance_empty() {
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", ""), 3);
    }

    // -- Display --

    #[test]
    fn resolve_error_display_with_hints() {
        let err = ResolveError {
            message: "unknown domain @filesytem".into(),
            span: Some(Span::new(3, 15)),
            hints: vec!["did you mean @filesystem?".into()],
        };
        let display = format!("{}", err);
        assert!(display.contains("unknown domain"), "{}", display);
        assert!(display.contains("did you mean"), "{}", display);
    }

    #[test]
    fn resolve_error_display_without_hints() {
        let err = ResolveError {
            message: "missing output block".into(),
            span: None,
            hints: vec![],
        };
        let display = format!("{}", err);
        assert!(display.contains("missing output"), "{}", display);
    }

    // -- absorb --

    #[test]
    fn absorb_not_yet_implemented() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        let err = Resolve::new().absorb(resolved).unwrap_err();
        assert!(err.message.contains("not yet implemented"));
    }

    // -- Execute --

    #[test]
    fn execute_produces_correct_output() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder(
                    "post.md",
                    "---\nslug: hello-world\n---\nContent here",
                )],
            )],
        );
        let result = resolved.execute(&tree);
        let items = result["root"]["items"].as_array().unwrap();
        assert_eq!(items[0]["slug"], "hello-world");
    }

    #[test]
    fn execute_missing_child_skips() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout root {\n\tmissing {\n\t\titems: sub { $t }\n\t}\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        let tree = dir_folder("root", vec![]);
        let result = resolved.execute(&tree);
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    #[test]
    fn execute_headlines_qualifier() {
        let source = "in @filesystem\ntemplate $t {\n\theadlines: h2\n}\nout root {\n\titems: sub { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("post.md", "## First\n## Second\n")],
            )],
        );
        let result = resolved.execute(&tree);
        let headlines = result["root"]["items"][0]["headlines"].as_array().unwrap();
        assert_eq!(headlines.len(), 2);
        assert_eq!(headlines[0], "First");
        assert_eq!(headlines[1], "Second");
    }

    #[test]
    fn execute_unknown_qualifier_produces_null() {
        let source =
            "in @filesystem\ntemplate $t {\n\tfield: unknown\n}\nout root {\n\titems: sub { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("f.md", "---\nfield: val\n---\n")],
            )],
        );
        let result = resolved.execute(&tree);
        assert!(result["root"]["items"][0]["field"].is_null());
    }

    // -- with_domain + Default --

    #[test]
    fn with_domain_registers_custom() {
        let resolve = Resolve::new().with_domain("html");
        let source = "in @html\nout r {\n\tx {}\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = resolve.emit(ast).unwrap();
        assert_eq!(resolved.domain, "html");
    }

    #[test]
    fn default_same_as_new() {
        let resolve = Resolve::default();
        let source = "in @filesystem\nout r {\n\tx {}\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = resolve.emit(ast).unwrap();
        assert_eq!(resolved.domain, "filesystem");
    }

    // -- Error: missing domain declaration --

    #[test]
    fn resolve_missing_domain_errors() {
        let source = "template $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let err = Resolve::new().emit(ast).unwrap_err();
        assert!(err.message.contains("domain"), "{}", err);
    }

    // -- Execute: missing folder in select skips --

    #[test]
    fn execute_missing_folder_in_select_skips() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout root {\n\titems: nonexistent { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        let tree = dir_folder("root", vec![]);
        let result = resolved.execute(&tree);
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    // -- Execute: group with matching child --

    #[test]
    fn execute_group_with_child() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\tsub {\n\t\titems: data { $t }\n\t}\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![dir_folder(
                    "data",
                    vec![leaf_folder("a.md", "---\nslug: hi\n---\n")],
                )],
            )],
        );
        let result = resolved.execute(&tree);
        let items = result["root"]["sub"]["items"].as_array().unwrap();
        assert_eq!(items[0]["slug"], "hi");
    }

    // -- Frontmatter: unclosed delimiter --

    #[test]
    fn execute_frontmatter_unclosed_returns_empty() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n";
        let ast = Parse.emit(source.to_string()).unwrap();
        let resolved = Resolve::new().emit(ast).unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("f.md", "---\nslug: test\nNo closing")],
            )],
        );
        let result = resolved.execute(&tree);
        // Unclosed frontmatter returns empty fields
        assert_eq!(result["root"]["items"][0]["slug"], "");
    }

    // -- Resolve ignores unknown AST children --

    #[test]
    fn resolve_template_ignores_unknown_child_kinds() {
        use crate::ast;
        // Manually construct an AST with a non-field child in a template
        let root = ast::ast_branch(
            Language::Group,
            "root",
            Span::new(0, 50),
            vec![
                ast::ast_leaf(Language::In, "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Language::Template,
                    "$t",
                    Span::new(15, 35),
                    vec![
                        ast::ast_leaf(Language::Field, "slug", Span::new(20, 24)),
                        // A DomainRef in a template — should be ignored
                        ast::ast_leaf(Language::DomainRef, "@html", Span::new(25, 30)),
                    ],
                ),
                ast::ast_branch(
                    Language::Out,
                    "r",
                    Span::new(36, 50),
                    vec![ast::ast_branch(
                        Language::Select,
                        "x",
                        Span::new(40, 48),
                        vec![
                            ast::ast_leaf(Language::DomainRef, "f", Span::new(42, 43)),
                            ast::ast_leaf(Language::TemplateRef, "$t", Span::new(44, 46)),
                        ],
                    )],
                ),
            ],
        );
        let resolved = Resolve::new().emit(root).unwrap();
        // Only the Field child is extracted, DomainRef is ignored
        assert_eq!(resolved.templates["$t"].fields.len(), 1);
    }

    #[test]
    fn resolve_output_ignores_unknown_child_kinds() {
        use crate::ast;
        // AST with an In node inside an Out block — should be ignored
        let root = ast::ast_branch(
            Language::Group,
            "root",
            Span::new(0, 50),
            vec![
                ast::ast_leaf(Language::In, "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Language::Out,
                    "r",
                    Span::new(15, 50),
                    vec![
                        ast::ast_leaf(Language::In, "@html", Span::new(20, 25)),
                        ast::ast_branch(Language::Group, "g", Span::new(26, 40), vec![]),
                    ],
                ),
            ],
        );
        let resolved = Resolve::new().emit(root).unwrap();
        // Only the Group child is extracted, In is ignored
        assert_eq!(resolved.output.len(), 1);
    }

    // -- Template field with only qualifier, no pipe (branch form) --

    #[test]
    fn resolve_field_branch_with_unknown_sub_kind() {
        use crate::ast;
        // A Field branch where one child has kind Group — should be ignored
        let root = ast::ast_branch(
            Language::Group,
            "root",
            Span::new(0, 80),
            vec![
                ast::ast_leaf(Language::In, "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Language::Template,
                    "$t",
                    Span::new(15, 60),
                    vec![ast::ast_branch(
                        Language::Field,
                        "headlines",
                        Span::new(20, 40),
                        vec![
                            ast::ast_leaf(Language::Qualifier, "h2", Span::new(25, 27)),
                            // Group as a child of Field — unusual, should be skipped
                            ast::ast_branch(Language::Group, "noise", Span::new(28, 35), vec![]),
                        ],
                    )],
                ),
                ast::ast_branch(
                    Language::Out,
                    "r",
                    Span::new(61, 80),
                    vec![ast::ast_branch(
                        Language::Select,
                        "x",
                        Span::new(65, 78),
                        vec![
                            ast::ast_leaf(Language::DomainRef, "f", Span::new(67, 68)),
                            ast::ast_leaf(Language::TemplateRef, "$t", Span::new(69, 71)),
                        ],
                    )],
                ),
            ],
        );
        let resolved = Resolve::new().emit(root).unwrap();
        let field = &resolved.templates["$t"].fields[0];
        assert_eq!(field.qualifier.as_deref(), Some("h2"));
        assert!(field.pipe.is_none());
    }

    // -- Composition: Parse → Resolve --

    #[test]
    fn parse_then_resolve_composes() {
        let pipeline = Parse.compose(Resolve::new());
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let resolved = pipeline.emit(source.to_string()).unwrap();
        assert_eq!(resolved.domain, "filesystem");
    }
}
