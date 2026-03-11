//! Resolve gradient. AST → validated program.
//!
//! The resolver IS a gradient: trace resolves.
//! Validates domain references, template references, output structure.
//! Errors carry spans and did-you-mean hints.

use std::collections::HashMap;
use std::marker::PhantomData;

use serde_json::Value;

use crate::ast::{AstNode, Span};
use crate::domain::conversation::Kind;
use crate::domain::{Addressable, Context};
use crate::gradient::{ComposedError, Gradient};
use crate::parse::ParseError;
use crate::tree::{self, Tree, Treelike};

use fragmentation::ref_::Ref;
use fragmentation::sha;

/// The resolve gradient. AST → Conversation.
///
/// Known domains (Filesystem, Json) always resolve.
/// External domains must be registered via `with_domain`.
#[derive(Clone, Debug)]
pub struct Resolve {
    externals: Vec<String>,
}

/// What can go wrong during resolution.
#[derive(Clone, Debug)]
pub struct ResolveError {
    pub message: String,
    pub span: Option<Span>,
    pub hints: Vec<String>,
}

/// A resolved .conv file. Validated and ready to execute.
///
/// The type parameter `C` is the input domain's context.
/// `Conversation<Filesystem>` executes against `Tree<Folder>`.
/// `Conversation<Git>` executes against `Tree<GitNode>`.
#[derive(Debug)]
pub struct Conversation<C: Context> {
    templates: HashMap<String, Template>,
    pub content: Tree<OutputNode>,
    _context: PhantomData<C>,
}

#[derive(Debug)]
pub struct Template {
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    qualifier: Option<String>,
    #[allow(dead_code)] // read in tests; used by future pipe execution
    pipe: Option<String>,
}

#[derive(Debug)]
pub enum OutputNode {
    Group {
        name: String,
    },
    Select {
        output_name: String,
        folder_name: String,
        template_name: String,
    },
}

impl OutputNode {
    /// The node's name — group name or select output name.
    pub fn name(&self) -> &str {
        match self {
            OutputNode::Group { name } => name,
            OutputNode::Select { output_name, .. } => output_name,
        }
    }
}

impl Resolve {
    pub fn new() -> Self {
        Resolve {
            externals: Vec::new(),
        }
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.externals.push(domain.to_string());
        self
    }

    /// Known domain names. C::id() already carries the identity;
    /// this list is only for validating `in @...` declarations.
    const KNOWN_DOMAINS: &'static [&'static str] = &["filesystem", "json", "git"];

    /// Is this domain name known or registered?
    fn is_known_domain(&self, name: &str) -> bool {
        Self::KNOWN_DOMAINS.contains(&name) || self.externals.iter().any(|e| e == name)
    }

    /// All domain names available for did_you_mean.
    fn all_domain_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = Self::KNOWN_DOMAINS.to_vec();
        for ext in &self.externals {
            names.push(ext.as_str());
        }
        names
    }
}

impl Default for Resolve {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Context> Gradient<Tree<AstNode>, Conversation<C>> for Resolve {
    type Error = ResolveError;

    fn trace(&self, source: Tree<AstNode>) -> crate::witness::Trace<Conversation<C>, ResolveError> {
        use crate::witness::{ContentAddressed, Oid, Trace};
        match resolve_ast(self, source) {
            Ok(conv) => {
                let oid = conv.content_oid();
                Trace::leaf(Ok(conv), oid)
            }
            Err(e) => Trace::leaf(Err(e), Oid::new("error")),
        }
    }
}

fn resolve_ast<C: Context>(
    resolve: &Resolve,
    source: Tree<AstNode>,
) -> Result<Conversation<C>, ResolveError> {
    let children = source.children();

    // Validate domain declaration if present
    let in_node = children.iter().find(|c| c.data().kind == Kind::In);
    if let Some(node) = in_node {
        let raw = &node.data().value;
        let name = raw.strip_prefix('@').unwrap_or(raw);
        if !resolve.is_known_domain(name) {
            let candidates = resolve.all_domain_names();
            let mut hints = Vec::new();
            if let Some(suggestion) = did_you_mean(name, &candidates) {
                hints.push(format!("did you mean @{}?", suggestion));
            }
            return Err(ResolveError {
                message: format!("unknown domain @{}", name),
                span: Some(node.data().span),
                hints,
            });
        }
    }

    // Extract templates
    let mut templates = HashMap::new();
    for child in children {
        if child.data().kind == Kind::Template {
            let name = child.data().value.clone();
            let fields = resolve_template_fields(child);
            templates.insert(name, Template { fields });
        }
    }

    // Extract output
    let out_node = children.iter().find(|c| c.data().kind == Kind::Out);
    let content = match out_node {
        Some(node) => {
            let name = node.data().value.clone();
            let output_children = resolve_output_nodes(node, &templates)?;
            let ref_ = Ref::new(sha::hash(&name), &name);
            tree::branch(ref_, OutputNode::Group { name }, output_children)
        }
        None => {
            return Err(ResolveError {
                message: "missing output block".into(),
                span: None,
                hints: vec![],
            });
        }
    };

    Ok(Conversation {
        templates,
        content,
        _context: PhantomData,
    })
}

fn resolve_template_fields(template_node: &Tree<AstNode>) -> Vec<Field> {
    let mut fields = Vec::new();
    for child in template_node.children() {
        if child.data().kind == Kind::Field {
            if child.is_shard() {
                // Bare field: no qualifier, no pipe
                fields.push(Field {
                    name: child.data().value.clone(),
                    qualifier: None,
                    pipe: None,
                });
            } else {
                // Field with qualifier and/or pipe
                let mut qualifier = None;
                let mut pipe = None;
                for sub in child.children() {
                    match sub.data().kind {
                        Kind::Qualifier => qualifier = Some(sub.data().value.clone()),
                        Kind::Pipe => pipe = Some(sub.data().value.clone()),
                        _ => {}
                    }
                }
                fields.push(Field {
                    name: child.data().value.clone(),
                    qualifier,
                    pipe,
                });
            }
        }
    }
    fields
}

fn resolve_output_nodes(
    node: &Tree<AstNode>,
    templates: &HashMap<String, Template>,
) -> Result<Vec<Tree<OutputNode>>, ResolveError> {
    let mut nodes = Vec::new();
    for child in node.children() {
        match child.data().kind {
            Kind::Group => {
                let children = resolve_output_nodes(child, templates)?;
                let name = child.data().value.clone();
                let ref_ = Ref::new(sha::hash(&name), &name);
                nodes.push(tree::branch(ref_, OutputNode::Group { name }, children));
            }
            Kind::Select => {
                let select_children = child.children();
                let folder_name = select_children
                    .iter()
                    .find(|c| c.data().kind == Kind::DomainRef)
                    .map(|c| c.data().value.clone())
                    .unwrap_or_default();
                let template_name = select_children
                    .iter()
                    .find(|c| c.data().kind == Kind::TemplateRef)
                    .map(|c| c.data().value.clone())
                    .unwrap_or_default();

                // Validate template reference
                if !templates.contains_key(&template_name) {
                    let candidates: Vec<&str> = templates.keys().map(|s| s.as_str()).collect();
                    let mut hints = Vec::new();
                    if let Some(suggestion) = did_you_mean(&template_name, &candidates) {
                        hints.push(format!("did you mean {}?", suggestion));
                    }
                    return Err(ResolveError {
                        message: format!("unknown template {}", template_name),
                        span: Some(child.data().span),
                        hints,
                    });
                }

                let output_name = child.data().value.clone();
                let ref_ = Ref::new(sha::hash(&output_name), &output_name);
                nodes.push(tree::leaf(
                    ref_,
                    OutputNode::Select {
                        output_name,
                        folder_name,
                        template_name,
                    },
                ));
            }
            _ => {}
        }
    }
    Ok(nodes)
}

impl<C: Context> crate::witness::ContentAddressed for Conversation<C> {
    fn content_oid(&self) -> crate::witness::Oid {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(b"conversation:");
        hasher.update(self.content.data().name().as_bytes());
        let mut keys: Vec<_> = self.templates.keys().collect();
        keys.sort();
        for key in keys {
            hasher.update(key.as_bytes());
        }
        crate::witness::Oid::new(hex::encode(hasher.finalize()))
    }
}

impl crate::witness::ContentAddressed for Value {
    fn content_oid(&self) -> crate::witness::Oid {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(self.to_string().as_bytes());
        crate::witness::Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<C: Context> Conversation<C> {
    /// Parse and resolve a `.conv` source string in one step.
    ///
    /// Chains Parse → Resolve via gradient composition.
    pub fn from_source(source: &str) -> Result<Self, ComposedError<ParseError, ResolveError>> {
        use crate::parse::Parse;
        Parse
            .compose::<Conversation<C>, _>(Resolve::new())
            .trace(source.to_string())
            .into_result()
    }
}

/// Conversation IS a gradient: `Tree<C::Token> → Value`.
///
/// The resolved program transforms domain trees into JSON output.
/// `trace` executes the program against a domain tree.
impl<C: Context> Gradient<Tree<C::Token>, Value> for Conversation<C>
where
    C::Token: Addressable + fragmentation::encoding::Encode,
{
    type Error = ResolveError;

    fn trace(&self, source: Tree<C::Token>) -> crate::witness::Trace<Value, ResolveError> {
        use crate::witness::{ContentAddressed, Trace};
        let body = emit_body(&self.content, &source, &self.templates);
        let mut map = serde_json::Map::new();
        map.insert(self.content.data().name().to_string(), body);
        let result = Value::Object(map);
        let oid = result.content_oid();
        Trace::leaf(Ok(result), oid)
    }
}

fn emit_body<T: Addressable>(
    content: &Tree<OutputNode>,
    tree: &Tree<T>,
    templates: &HashMap<String, Template>,
) -> Value {
    let mut map = serde_json::Map::new();

    for child in content.children() {
        match child.data() {
            OutputNode::Group { name } => {
                if let Some(domain_child) = find_child(tree, name) {
                    map.insert(name.clone(), emit_body(child, domain_child, templates));
                }
            }
            OutputNode::Select {
                output_name,
                folder_name,
                template_name,
            } => {
                if let Some(folder) = find_child(tree, folder_name) {
                    let template = &templates[template_name];
                    let items: Vec<Value> = folder
                        .children()
                        .iter()
                        .map(|f| apply_template(template, f))
                        .collect();
                    map.insert(output_name.clone(), Value::Array(items));
                }
            }
        }
    }

    Value::Object(map)
}

fn find_child<'a, T: Addressable>(tree: &'a Tree<T>, name: &str) -> Option<&'a Tree<T>> {
    tree.children()
        .iter()
        .find(|c| c.data().node_name() == name)
}

fn apply_template<T: Addressable>(template: &Template, tree: &Tree<T>) -> Value {
    let content = tree.data().node_content().unwrap_or("");
    let (frontmatter, body) = parse_frontmatter(content);

    let mut map = serde_json::Map::new();
    for field in &template.fields {
        match &field.qualifier {
            None => {
                let value = lookup_frontmatter(&frontmatter, &field.name);
                map.insert(field.name.clone(), Value::String(value));
            }
            Some(qual) if qual == "h2" => {
                let headlines = extract_headlines(body, "## ");
                map.insert(
                    field.name.clone(),
                    Value::Array(headlines.into_iter().map(Value::String).collect()),
                );
            }
            Some(_) => {
                map.insert(field.name.clone(), Value::Null);
            }
        }
    }
    Value::Object(map)
}

fn parse_frontmatter(content: &str) -> (HashMap<String, String>, &str) {
    let mut frontmatter = HashMap::new();

    if !content.starts_with("---") {
        return (frontmatter, content);
    }

    let after_first = &content[3..];
    if let Some(end_idx) = after_first.find("\n---") {
        let fm_text = &after_first[..end_idx];
        let body = &after_first[end_idx + 4..];
        let body = body.strip_prefix('\n').unwrap_or(body);

        for line in fm_text.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(':') {
                frontmatter.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        (frontmatter, body)
    } else {
        (frontmatter, content)
    }
}

fn lookup_frontmatter(fm: &HashMap<String, String>, key: &str) -> String {
    fm.get(&key.to_lowercase()).cloned().unwrap_or_default()
}

fn extract_headlines(content: &str, prefix: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.starts_with(prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
        .collect()
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = self
            .span
            .as_ref()
            .map(|s| format!(" at {}..{}", s.start, s.end))
            .unwrap_or_default();
        write!(f, "resolve error{}: {}", location, self.message)?;
        for hint in &self.hints {
            write!(f, "\n  hint: {}", hint)?;
        }
        Ok(())
    }
}

impl std::error::Error for ResolveError {}

fn did_you_mean<'a>(name: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let threshold = (name.len() / 3).max(1) + 1;
    candidates
        .iter()
        .map(|&c| (c, edit_distance(name, c)))
        .filter(|(_, d)| *d <= threshold)
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate() {
        *cell = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::filesystem::{Filesystem, Folder};
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

    /// Shorthand: resolve with Filesystem context.
    fn resolve_fs(
        ast: Tree<AstNode>,
    ) -> crate::witness::Trace<Conversation<Filesystem>, ResolveError> {
        Resolve::new().trace(ast)
    }

    // -- OutputNode::name --

    #[test]
    fn output_node_name() {
        let group = OutputNode::Group {
            name: "blog".into(),
        };
        assert_eq!(group.name(), "blog");

        let select = OutputNode::Select {
            output_name: "items".into(),
            folder_name: "sub".into(),
            template_name: "$t".into(),
        };
        assert_eq!(select.name(), "items");
    }

    // -- Resolve valid conv --

    #[test]
    fn resolve_valid_conv() {
        let source = "in @filesystem\ntemplate $corpus {\n\tslug\n\texcerpt\n}\nout blog {\n\tpieces {\n\t\tdraft: 1draft { $corpus }\n\t}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let resolved = resolve_fs(ast).unwrap();
        assert_eq!(resolved.content.data().name(), "blog");
        assert!(resolved.templates.contains_key("$corpus"));
    }

    #[test]
    fn resolve_extracts_template_fields() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n\theadlines: h2\n\thtml: article | @html\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let resolved = resolve_fs(ast).unwrap();
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
        let ast = Parse.trace(source.to_string()).unwrap();
        let err = resolve_fs(ast).into_result().unwrap_err();
        assert!(err.message.contains("filesytem"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest @filesystem");
        assert!(err.hints[0].contains("filesystem"), "{}", err.hints[0]);
    }

    // -- Error: unknown template --

    #[test]
    fn resolve_unknown_template_errors() {
        let source = "in @filesystem\ntemplate $corpus {\n\tslug\n}\nout blog {\n\tpieces {\n\t\tdraft: 1draft { $coprus }\n\t}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let err = resolve_fs(ast).into_result().unwrap_err();
        assert!(err.message.contains("coprus"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest $corpus");
    }

    // -- Error: missing output --

    #[test]
    fn resolve_missing_output_errors() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let err = resolve_fs(ast).into_result().unwrap_err();
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

    // -- Execute --

    #[test]
    fn emit_produces_correct_output() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();

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
        let result = resolved.trace(tree).unwrap();
        let items = result["root"]["items"].as_array().unwrap();
        assert_eq!(items[0]["slug"], "hello-world");
    }

    #[test]
    fn emit_missing_child_skips() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout root {\n\tmissing {\n\t\titems: sub { $t }\n\t}\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder("root", vec![]);
        let result = resolved.trace(tree).unwrap();
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    #[test]
    fn emit_headlines_qualifier() {
        let source = "in @filesystem\ntemplate $t {\n\theadlines: h2\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("post.md", "## First\n## Second\n")],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        let headlines = result["root"]["items"][0]["headlines"].as_array().unwrap();
        assert_eq!(headlines.len(), 2);
        assert_eq!(headlines[0], "First");
        assert_eq!(headlines[1], "Second");
    }

    #[test]
    fn emit_unknown_qualifier_produces_null() {
        let source =
            "in @filesystem\ntemplate $t {\n\tfield: unknown\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("f.md", "---\nfield: val\n---\n")],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        assert!(result["root"]["items"][0]["field"].is_null());
    }

    // -- with_domain + Default --

    #[test]
    fn with_domain_registers_external() {
        let resolve = Resolve::new().with_domain("html");
        let source = "in @html\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved: Conversation<Filesystem> = resolve.trace(ast).unwrap();
    }

    #[test]
    fn default_same_as_new() {
        let source = "in @filesystem\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved: Conversation<Filesystem> = Resolve::default().trace(ast).unwrap();
    }

    // -- Error: missing domain declaration --

    #[test]
    fn resolve_unknown_domain_suggests_external() {
        let resolve = Resolve::new().with_domain("graphql");
        let source = "in @graphq\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(err.message.contains("graphq"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest @graphql");
        assert!(err.hints[0].contains("graphql"), "{}", err.hints[0]);
    }

    #[test]
    fn resolve_missing_in_declaration_still_resolves() {
        let source = "template $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved = resolve_fs(ast).unwrap();
    }

    // -- Emit: missing folder in select skips --

    #[test]
    fn emit_missing_folder_in_select_skips() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout root {\n\titems: nonexistent { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder("root", vec![]);
        let result = resolved.trace(tree).unwrap();
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    // -- Emit: group with matching child --

    #[test]
    fn emit_group_with_child() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\tsub {\n\t\titems: data { $t }\n\t}\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
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
        let result = resolved.trace(tree).unwrap();
        let items = result["root"]["sub"]["items"].as_array().unwrap();
        assert_eq!(items[0]["slug"], "hi");
    }

    // -- Frontmatter: unclosed delimiter --

    #[test]
    fn emit_frontmatter_unclosed_returns_empty() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("f.md", "---\nslug: test\nNo closing")],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        // Unclosed frontmatter returns empty fields
        assert_eq!(result["root"]["items"][0]["slug"], "");
    }

    // -- Resolve ignores unknown AST children --

    #[test]
    fn resolve_template_ignores_unknown_child_kinds() {
        use crate::ast;
        // Manually construct an AST with a non-field child in a template
        let root = ast::ast_branch(
            Kind::Group,
            "root",
            Span::new(0, 50),
            vec![
                ast::ast_leaf(Kind::In, "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Kind::Template,
                    "$t",
                    Span::new(15, 35),
                    vec![
                        ast::ast_leaf(Kind::Field, "slug", Span::new(20, 24)),
                        // A DomainRef in a template — should be ignored
                        ast::ast_leaf(Kind::DomainRef, "@html", Span::new(25, 30)),
                    ],
                ),
                ast::ast_branch(
                    Kind::Out,
                    "r",
                    Span::new(36, 50),
                    vec![ast::ast_branch(
                        Kind::Select,
                        "x",
                        Span::new(40, 48),
                        vec![
                            ast::ast_leaf(Kind::DomainRef, "f", Span::new(42, 43)),
                            ast::ast_leaf(Kind::TemplateRef, "$t", Span::new(44, 46)),
                        ],
                    )],
                ),
            ],
        );
        let resolved = resolve_fs(root).unwrap();
        // Only the Field child is extracted, DomainRef is ignored
        assert_eq!(resolved.templates["$t"].fields.len(), 1);
    }

    #[test]
    fn resolve_output_ignores_unknown_child_kinds() {
        use crate::ast;
        // AST with an In node inside an Out block — should be ignored
        let root = ast::ast_branch(
            Kind::Group,
            "root",
            Span::new(0, 50),
            vec![
                ast::ast_leaf(Kind::In, "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Kind::Out,
                    "r",
                    Span::new(15, 50),
                    vec![
                        ast::ast_leaf(Kind::In, "@html", Span::new(20, 25)),
                        ast::ast_branch(Kind::Group, "g", Span::new(26, 40), vec![]),
                    ],
                ),
            ],
        );
        let resolved = resolve_fs(root).unwrap();
        // Only the Group child is extracted, In is ignored
        assert_eq!(resolved.content.children().len(), 1);
    }

    // -- Template field with only qualifier, no pipe (branch form) --

    #[test]
    fn resolve_field_branch_with_unknown_sub_kind() {
        use crate::ast;
        // A Field branch where one child has kind Group — should be ignored
        let root = ast::ast_branch(
            Kind::Group,
            "root",
            Span::new(0, 80),
            vec![
                ast::ast_leaf(Kind::In, "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Kind::Template,
                    "$t",
                    Span::new(15, 60),
                    vec![ast::ast_branch(
                        Kind::Field,
                        "headlines",
                        Span::new(20, 40),
                        vec![
                            ast::ast_leaf(Kind::Qualifier, "h2", Span::new(25, 27)),
                            // Group as a child of Field — unusual, should be skipped
                            ast::ast_branch(Kind::Group, "noise", Span::new(28, 35), vec![]),
                        ],
                    )],
                ),
                ast::ast_branch(
                    Kind::Out,
                    "r",
                    Span::new(61, 80),
                    vec![ast::ast_branch(
                        Kind::Select,
                        "x",
                        Span::new(65, 78),
                        vec![
                            ast::ast_leaf(Kind::DomainRef, "f", Span::new(67, 68)),
                            ast::ast_leaf(Kind::TemplateRef, "$t", Span::new(69, 71)),
                        ],
                    )],
                ),
            ],
        );
        let resolved = resolve_fs(root).unwrap();
        let field = &resolved.templates["$t"].fields[0];
        assert_eq!(field.qualifier.as_deref(), Some("h2"));
        assert!(field.pipe.is_none());
    }

    // -- ContentAddressed --

    #[test]
    fn conversation_content_addressed() {
        use crate::witness::ContentAddressed;
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout blog {\n\titems: sub { $t }\n}\n";
        let a = Conversation::<Filesystem>::from_source(source).unwrap();
        let b = Conversation::<Filesystem>::from_source(source).unwrap();
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn value_content_addressed() {
        use crate::witness::ContentAddressed;
        let a = Value::String("hello".into());
        let b = Value::String("hello".into());
        assert_eq!(a.content_oid(), b.content_oid());

        let c = Value::String("world".into());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    // -- Composition: Parse → Resolve --

    #[test]
    fn parse_then_resolve_composes() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        // Parse → Resolve composes via explicit chaining
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved = resolve_fs(ast).unwrap();
    }

    // -- Bridge: from_source --

    #[test]
    fn from_source_parses_and_resolves() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout blog {\n\titems: sub { $t }\n}\n";
        let resolved = Conversation::<Filesystem>::from_source(source).unwrap();
        assert_eq!(resolved.content.data().name(), "blog");
        assert!(resolved.templates.contains_key("$t"));
    }

    /// Shared extractor — single monomorphization, both arms covered
    /// across from_source_propagates_parse_error / _resolve_error.
    fn extract_composed_message(
        err: ComposedError<crate::parse::ParseError, ResolveError>,
    ) -> (bool, String) {
        match err {
            ComposedError::First(pe) => (true, pe.message),
            ComposedError::Second(re) => (false, re.message),
        }
    }

    #[test]
    fn from_source_propagates_parse_error() {
        let err = Conversation::<Filesystem>::from_source("garbage\n").unwrap_err();
        let (is_parse, msg) = extract_composed_message(err);
        assert!(is_parse);
        assert!(msg.contains("unexpected"), "{}", msg);
    }

    #[test]
    fn from_source_propagates_resolve_error() {
        let err =
            Conversation::<Filesystem>::from_source("in @bogus\nout r {\n\tx {}\n}\n").unwrap_err();
        let (is_parse, msg) = extract_composed_message(err);
        assert!(!is_parse);
        assert!(msg.contains("bogus"), "{}", msg);
    }

    // -- Litmus: @git domain proves Conversation is a Gradient --

    #[test]
    fn git_conv_emits_against_git_tree() {
        use crate::domain::git::{Git, GitNode};

        // A .conv that declares @git input domain
        let source = "in @git\ntemplate $t {\n\tname\n}\nout repo {\n\trefs: heads { $t }\n}\n";
        let resolved = Conversation::<Git>::from_source(source).unwrap();

        // Build a synthetic Tree<GitNode> — a ref pointing to a commit
        let blob = tree::leaf(
            test_ref("README.md"),
            GitNode::Blob {
                content: b"# Hello".to_vec(),
            },
        );
        let entry = tree::branch(
            test_ref("src"),
            GitNode::Entry { name: "src".into() },
            vec![blob],
        );
        let commit = tree::branch(
            test_ref("abc123"),
            GitNode::Commit {
                message: "init".into(),
                author: "Reed".into(),
                email: "reed@systemic.engineer".into(),
            },
            vec![entry],
        );
        let ref_node = tree::branch(
            test_ref("heads"),
            GitNode::Ref {
                name: "main".into(),
                target: "abc123".into(),
            },
            vec![commit],
        );
        let root = tree::branch(
            test_ref("repo"),
            GitNode::Entry {
                name: "repo".into(),
            },
            vec![ref_node],
        );

        // Conversation IS a gradient: Tree<C::Token> → Value
        let _result = resolved.trace(root).unwrap();
    }

    // -- Litmus: real .conv against real filesystem --

    #[test]
    fn systemic_engineering_conv_produces_blog_index() {
        let conv_source = include_str!("../systemic.engineering.conv");
        let resolved = Conversation::<Filesystem>::from_source(conv_source).expect("from_source");

        let se_path = std::env::var("SYSTEMIC_ENGINEERING")
            .unwrap_or_else(|_| "/Users/alexwolf/dev/systemic.engineering".into());
        let tree = Folder::read_tree(&format!("{}/blog", se_path));
        let result = resolved.trace(tree).unwrap();

        // Output shape matches .conv declaration
        let pieces = &result["blog"]["pieces"];

        let drafts = pieces["draft"].as_array().expect("draft is array");
        assert!(!drafts.is_empty(), "should have draft pieces");

        let published = pieces["published"].as_array().expect("published is array");
        assert!(!published.is_empty(), "should have published pieces");

        let archived = pieces["archived"].as_array().expect("archived is array");
        assert!(!archived.is_empty(), "should have archived pieces");

        for entry in published {
            assert!(entry["slug"].is_string(), "slug should be string");
            assert!(entry["headlines"].is_array(), "headlines should be array");
        }

        let consciousness = published
            .iter()
            .find(|p| p["slug"] == "written-by-ai-consciousness")
            .expect("consciousness piece should exist in published");
        assert!(!consciousness["excerpt"].as_str().unwrap().is_empty());
        assert!(!consciousness["headlines"].as_array().unwrap().is_empty());
    }
}
