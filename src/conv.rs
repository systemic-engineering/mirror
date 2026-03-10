use std::collections::HashMap;

use serde_json::Value;

use crate::tree::{self, Tree, Treelike};

/// A parsed `.conv` file — typed transformation pipeline.
///
/// A `.conv` file declares:
/// - `in Type` — the input tree type
/// - `template $name { fields }` — reusable extraction shapes
/// - `out name { structure }` — the output shape
///
/// The file's shape IS the output's shape.
pub struct Conv {
    templates: HashMap<String, Template>,
    output_name: String,
    output_body: Vec<OutputEntry>,
}

struct Template {
    fields: Vec<Field>,
}

struct Field {
    name: String,
    qualifier: Option<String>,
}

enum OutputEntry {
    Group {
        name: String,
        children: Vec<OutputEntry>,
    },
    Select {
        output_name: String,
        folder_name: String,
        template_name: String,
    },
}

/// A filesystem entry as tree data.
///
/// Branch = directory (has children, no content).
/// Leaf = file (has content, no children).
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub content: Option<String>,
}

/// Parse error for `.conv` files.
#[derive(Debug)]
pub struct ConvError(pub String);

impl std::fmt::Display for ConvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conv parse error: {}", self.0)
    }
}

impl std::error::Error for ConvError {}

impl Conv {
    /// Parse a `.conv` file from its source text.
    pub fn parse(input: &str) -> Result<Self, ConvError> {
        let mut templates = HashMap::new();
        let mut output_name = None;
        let mut output_body = None;

        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() || line.starts_with('#') || line.starts_with("in ") {
                i += 1;
                continue;
            }

            if let Some(rest) = line.strip_prefix("template ") {
                let name = rest.split('{').next().unwrap().trim().to_string();
                i += 1;
                let mut fields = Vec::new();
                while i < lines.len() {
                    let field_line = lines[i].trim();
                    if field_line == "}" {
                        i += 1;
                        break;
                    }
                    if !field_line.is_empty() {
                        if let Some((name, qual)) = field_line.split_once(':') {
                            fields.push(Field {
                                name: name.trim().to_string(),
                                qualifier: Some(qual.trim().to_string()),
                            });
                        } else {
                            fields.push(Field {
                                name: field_line.to_string(),
                                qualifier: None,
                            });
                        }
                    }
                    i += 1;
                }
                templates.insert(name, Template { fields });
                continue;
            }

            if let Some(rest) = line.strip_prefix("out ") {
                let name = rest.split('{').next().unwrap().trim().to_string();
                output_name = Some(name);
                i += 1;
                let (body, new_i) = parse_output_body(&lines, i)?;
                output_body = Some(body);
                i = new_i;
                continue;
            }

            return Err(ConvError(format!("unexpected line: {}", line)));
        }

        let output_name = output_name.ok_or_else(|| ConvError("no output block".into()))?;
        // output_body is always set alongside output_name in the parser
        let output_body = output_body.unwrap();
        Ok(Conv {
            templates,
            output_name,
            output_body,
        })
    }

    /// Execute the pipeline against a tree, producing a JSON value.
    pub fn execute(&self, tree: &Tree<Folder>) -> Value {
        let body = self.execute_body(&self.output_body, tree);
        let mut map = serde_json::Map::new();
        map.insert(self.output_name.clone(), body);
        Value::Object(map)
    }

    fn execute_body(&self, entries: &[OutputEntry], tree: &Tree<Folder>) -> Value {
        let mut map = serde_json::Map::new();

        for entry in entries {
            match entry {
                OutputEntry::Group { name, children } => {
                    if let Some(child) = find_child(tree, name) {
                        map.insert(name.clone(), self.execute_body(children, child));
                    }
                }
                OutputEntry::Select {
                    output_name,
                    folder_name,
                    template_name,
                } => {
                    if let Some(folder) = find_child(tree, folder_name) {
                        let template = &self.templates[template_name];
                        let items: Vec<Value> = folder
                            .children()
                            .iter()
                            .map(|child| apply_template(template, child))
                            .collect();
                        map.insert(output_name.clone(), Value::Array(items));
                    }
                }
            }
        }

        Value::Object(map)
    }
}

fn parse_output_body(lines: &[&str], start: usize) -> Result<(Vec<OutputEntry>, usize), ConvError> {
    let mut entries = Vec::new();
    let mut i = start;

    while i < lines.len() {
        let line = lines[i].trim();

        if line == "}" {
            return Ok((entries, i + 1));
        }

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Select: "name: folder { $template }"
        if let Some((output_part, rest)) = line.split_once(':') {
            let rest = rest.trim();
            if let Some((folder, template_part)) = rest.split_once('{') {
                let folder_name = folder.trim().to_string();
                let template_name = template_part
                    .trim()
                    .trim_end_matches('}')
                    .trim()
                    .to_string();
                entries.push(OutputEntry::Select {
                    output_name: output_part.trim().to_string(),
                    folder_name,
                    template_name,
                });
                i += 1;
                continue;
            }
        }

        // Group: "name {"
        if let Some((name, rest)) = line.split_once('{') {
            let name = name.trim().to_string();
            if rest.trim() == "}" {
                entries.push(OutputEntry::Group {
                    name,
                    children: vec![],
                });
                i += 1;
            } else {
                i += 1;
                let (children, new_i) = parse_output_body(lines, i)?;
                entries.push(OutputEntry::Group { name, children });
                i = new_i;
            }
            continue;
        }

        return Err(ConvError(format!("unexpected output line: {}", line)));
    }

    Err(ConvError("unclosed output block".into()))
}

fn find_child<'a>(tree: &'a Tree<Folder>, name: &str) -> Option<&'a Tree<Folder>> {
    tree.children().iter().find(|c| c.data().name == name)
}

fn apply_template(template: &Template, tree: &Tree<Folder>) -> Value {
    let content = tree.data().content.as_deref().unwrap_or("");
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

impl Folder {
    /// Build a `Tree<Folder>` from a filesystem path.
    ///
    /// Directories become branches, files become leaves.
    pub fn read_tree(path: &str) -> Tree<Folder> {
        use fragmentation::ref_::Ref;
        use fragmentation::sha;

        let p = std::path::Path::new(path);
        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        if p.is_dir() {
            let mut children: Vec<Tree<Folder>> = Vec::new();
            if let Ok(entries) = std::fs::read_dir(p) {
                let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                entries.sort_by_key(|e| e.file_name());
                for entry in entries {
                    children.push(Folder::read_tree(entry.path().to_str().unwrap()));
                }
            }
            let ref_ = Ref::new(sha::hash(&name), &name);
            tree::branch(
                ref_,
                Folder {
                    name,
                    content: None,
                },
                children,
            )
        } else {
            let content = std::fs::read_to_string(p).ok();
            let ref_ = Ref::new(sha::hash(&name), &name);
            tree::leaf(ref_, Folder { name, content })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{self, Treelike};
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

    // -- Parser tests --

    #[test]
    fn parse_minimal_conv() {
        let input =
            "in Tree<Folder>\n\ntemplate $t {\n\tname\n}\n\nout root {\n\titems: sub { $t }\n}\n";
        let conv = Conv::parse(input).unwrap();
        assert_eq!(conv.output_name, "root");
        assert!(conv.templates.contains_key("$t"));
    }

    #[test]
    fn parse_error_unexpected_line() {
        let result = Conv::parse("garbage");
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("unexpected line"));
    }

    #[test]
    fn parse_error_no_output() {
        let result = Conv::parse("in Tree<Folder>\ntemplate $t {\n\tname\n}\n");
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("no output block"));
    }

    #[test]
    fn parse_error_unclosed_output() {
        let result = Conv::parse("out root {\n\tpieces {\n");
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("unclosed output block")
        );
    }

    #[test]
    fn parse_colon_without_brace_falls_through() {
        // A colon line without { falls through to the group/error parser
        let result = Conv::parse("out root {\n\tlabel: value\n}\n");
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("unexpected output line")
        );
    }

    #[test]
    fn parse_error_unexpected_output_line() {
        let result = Conv::parse("out root {\n\tnobraces\n}\n");
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("unexpected output line")
        );
    }

    #[test]
    fn parse_template_with_blank_lines() {
        let input = "template $t {\n\n\tname\n\n\ttitle\n}\n\nout root {\n\titems: sub { $t }\n}\n";
        let conv = Conv::parse(input).unwrap();
        assert_eq!(conv.templates["$t"].fields.len(), 2);
    }

    #[test]
    fn parse_output_with_blank_lines() {
        let input = "template $t {\n\tname\n}\n\nout root {\n\n\titems: sub { $t }\n\n}\n";
        let conv = Conv::parse(input).unwrap();
        assert_eq!(conv.output_name, "root");
    }

    #[test]
    fn parse_empty_group() {
        let input = "out root {\n\tempty {}\n}\n";
        let conv = Conv::parse(input).unwrap();
        let tree = dir_folder("root", vec![dir_folder("empty", vec![])]);
        let result = conv.execute(&tree);
        assert!(result["root"]["empty"].is_object());
    }

    #[test]
    fn conv_error_display() {
        let err = ConvError("test error".into());
        assert_eq!(format!("{}", err), "conv parse error: test error");
        // Also exercise Error trait
        assert!(std::error::Error::source(&err).is_none());
    }

    // -- Frontmatter tests --

    #[test]
    fn parse_frontmatter_with_content() {
        let content = "---\nSLUG: my-slug\nExcerpt: hello world\n---\n\n## Heading\nBody text.";
        let (fm, body) = parse_frontmatter(content);
        assert_eq!(fm.get("slug").unwrap(), "my-slug");
        assert_eq!(fm.get("excerpt").unwrap(), "hello world");
        assert!(body.contains("## Heading"));
    }

    #[test]
    fn parse_frontmatter_no_delimiters() {
        let content = "Just plain content\n## Heading";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_empty());
        assert_eq!(body, content);
    }

    #[test]
    fn parse_frontmatter_unclosed() {
        let content = "---\nSLUG: test\nNo closing delimiter";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_empty());
        assert_eq!(body, content);
    }

    #[test]
    fn lookup_frontmatter_case_insensitive() {
        let mut fm = HashMap::new();
        fm.insert("slug".to_string(), "test-slug".to_string());
        assert_eq!(lookup_frontmatter(&fm, "SLUG"), "test-slug");
        assert_eq!(lookup_frontmatter(&fm, "missing"), "");
    }

    #[test]
    fn extract_h2_headlines() {
        let content = "# Title\n## First\nParagraph.\n## Second\n### Not this";
        let headlines = extract_headlines(content, "## ");
        assert_eq!(headlines, vec!["First", "Second"]);
    }

    // -- Executor tests --

    #[test]
    fn execute_unknown_qualifier_produces_null() {
        let input =
            "template $t {\n\tfield: unknown_qual\n}\n\nout root {\n\titems: sub { $t }\n}\n";
        let conv = Conv::parse(input).unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("file.md", "---\nfield: val\n---\n")],
            )],
        );
        let result = conv.execute(&tree);
        assert!(result["root"]["items"][0]["field"].is_null());
    }

    #[test]
    fn execute_missing_child_skips() {
        let input =
            "out root {\n\tmissing {\n\t\titems: sub { $t }\n\t}\n}\ntemplate $t {\n\tname\n}\n";
        let conv = Conv::parse(input).unwrap();
        let tree = dir_folder("root", vec![]);
        let result = conv.execute(&tree);
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    #[test]
    fn execute_missing_folder_in_select_skips() {
        let input = "template $t {\n\tname\n}\n\nout root {\n\titems: nonexistent { $t }\n}\n";
        let conv = Conv::parse(input).unwrap();
        let tree = dir_folder("root", vec![]);
        let result = conv.execute(&tree);
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    #[test]
    fn folder_read_tree_builds_structure() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("child");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("file.txt"), "hello").unwrap();

        let tree = Folder::read_tree(dir.path().to_str().unwrap());
        assert!(tree.is_fractal());
        assert_eq!(tree.children().len(), 1);
        let child = &tree.children()[0];
        assert_eq!(child.data().name, "child");
        assert!(child.is_fractal());
        assert_eq!(child.children().len(), 1);
        let file = &child.children()[0];
        assert_eq!(file.data().name, "file.txt");
        assert_eq!(file.data().content.as_deref(), Some("hello"));
    }

    #[test]
    fn folder_read_tree_nonexistent_path_produces_leaf() {
        let tree = Folder::read_tree("/nonexistent/path/that/does/not/exist");
        assert!(tree.is_shard());
        assert!(tree.data().content.is_none());
    }

    #[test]
    fn folder_read_tree_unreadable_dir_produces_empty_branch() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let restricted = dir.path().join("noperm");
        std::fs::create_dir(&restricted).unwrap();
        std::fs::write(restricted.join("file.txt"), "hidden").unwrap();
        std::fs::set_permissions(&restricted, std::fs::Permissions::from_mode(0o000)).unwrap();

        let tree = Folder::read_tree(restricted.to_str().unwrap());
        assert!(tree.is_fractal());
        assert_eq!(tree.children().len(), 0);

        // Restore permissions for cleanup
        std::fs::set_permissions(&restricted, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    // -- End-to-end litmus test --

    #[test]
    fn systemic_engineering_conv_produces_blog_index() {
        let conv_source = include_str!("../systemic.engineering.conv");
        let conv = Conv::parse(conv_source).expect("parses .conv file");

        let se_path = std::env::var("SYSTEMIC_ENGINEERING")
            .unwrap_or_else(|_| "/Users/alexwolf/dev/systemic.engineering".into());
        let tree = Folder::read_tree(&format!("{}/blog", se_path));
        let result = conv.execute(&tree);

        // Output shape matches .conv declaration
        let pieces = &result["blog"]["pieces"];

        // 1draft — draft pieces
        let drafts = pieces["draft"].as_array().expect("draft is array");
        assert!(!drafts.is_empty(), "should have draft pieces");

        // 3published — published pieces
        let published = pieces["published"].as_array().expect("published is array");
        assert!(!published.is_empty(), "should have published pieces");

        // 4archived — archived pieces
        let archived = pieces["archived"].as_array().expect("archived is array");
        assert!(!archived.is_empty(), "should have archived pieces");

        // Each entry has corpus fields: slug, excerpt, headlines
        for entry in published {
            assert!(entry["slug"].is_string(), "slug should be string");
            assert!(entry["headlines"].is_array(), "headlines should be array");
        }

        // Known piece: Consciousness (SLUG: written-by-ai-consciousness)
        let consciousness = published
            .iter()
            .find(|p| p["slug"] == "written-by-ai-consciousness")
            .expect("consciousness piece should exist in published");
        assert!(!consciousness["excerpt"].as_str().unwrap().is_empty());
        assert!(!consciousness["headlines"].as_array().unwrap().is_empty());
    }
}
