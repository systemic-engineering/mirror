//! Package testing. `.conv` files test themselves.
//!
//! Everything above `---` is the specification.
//! Everything below is verification.
//!
//! Two assertion forms:
//!   `.path.to.key`              — output key must exist
//!   `@domain.type has variant`  — grammar type must include variant

use std::collections::HashMap;

use crate::domain::filesystem::{Filesystem, Folder};
use crate::resolve::{Conversation, TypeRegistry};
use crate::Vector;

/// A single assertion in a test block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Assertion {
    /// `.path.to.key` — the JSON output must contain this path.
    OutputPath(Vec<String>),
    /// `@domain.type has variant` — the grammar must include this variant.
    GrammarHas {
        domain: String,
        type_name: String,
        variant: String,
    },
}

/// A named test block.
#[derive(Clone, Debug)]
pub struct TestBlock {
    pub name: String,
    pub assertions: Vec<Assertion>,
}

/// Result of running a single assertion.
#[derive(Clone, Debug)]
pub struct AssertionResult {
    pub assertion: Assertion,
    pub passed: bool,
    pub message: String,
}

/// Result of running a test block.
#[derive(Clone, Debug)]
pub struct TestResult {
    pub name: String,
    pub assertions: Vec<AssertionResult>,
}

impl TestResult {
    pub fn passed(&self) -> bool {
        self.assertions.iter().all(|a| a.passed)
    }
}

/// Split a .conv source on `---`. Returns (spec, tests).
/// If no separator, tests portion is empty.
pub fn split_source(source: &str) -> (&str, &str) {
    if let Some(pos) = source.find("\n---\n") {
        (&source[..pos], &source[pos + 5..])
    } else if let Some(pos) = source.find("\n---") {
        // Handle trailing --- at end of file
        if pos + 4 == source.len() || source[pos + 4..].chars().all(|c| c.is_whitespace()) {
            (&source[..pos], "")
        } else {
            (&source[..pos], &source[pos + 4..])
        }
    } else {
        (source, "")
    }
}

/// Run tests from a .conv file. Returns exit code (0 = pass, 1 = fail).
pub fn run_file(source: &str, input_path: &str, conv_path: &str) -> i32 {
    let (spec, test_source) = split_source(source);

    if test_source.is_empty() {
        eprintln!(
            "conversation test: {}: no test section (missing ---)",
            conv_path
        );
        return 1;
    }

    // Parse test blocks
    let blocks = match parse_tests(test_source) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("conversation test: {}: {}", conv_path, e);
            return 1;
        }
    };

    if blocks.is_empty() {
        eprintln!("conversation test: {}: no test blocks found", conv_path);
        return 1;
    }

    // Evaluate the spec
    let resolved: Conversation<Filesystem> = match Conversation::from_source(spec) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("conversation test: {}: {}", conv_path, e);
            return 1;
        }
    };

    let tree = Folder::read_tree(input_path);
    let output: serde_json::Value = resolved.trace(tree).into_result().unwrap();

    // Run tests
    let results = run_tests(&blocks, &output, &resolved.grammars);

    // Report
    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;

    for result in &results {
        for a in &result.assertions {
            total += 1;
            if a.passed {
                passed += 1;
                eprintln!("  \x1b[32m✓\x1b[0m {}", a.message);
            } else {
                failed += 1;
                eprintln!("  \x1b[31m✗\x1b[0m {}", a.message);
            }
        }
    }

    eprintln!();
    if failed == 0 {
        eprintln!("\x1b[32m{} assertions passed\x1b[0m ({})", total, conv_path);
        0
    } else {
        eprintln!(
            "\x1b[31m{} failed\x1b[0m, {} passed ({})",
            failed, passed, conv_path
        );
        1
    }
}

/// Parse test blocks from the test section of a .conv file.
pub fn parse_tests(source: &str) -> Result<Vec<TestBlock>, String> {
    let mut blocks = Vec::new();
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("test ") {
            let block = parse_test_block(rest, &mut lines)?;
            blocks.push(block);
            continue;
        }

        return Err(format!("unexpected in test section: {}", trimmed));
    }

    Ok(blocks)
}

fn parse_test_block(
    header: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
) -> Result<TestBlock, String> {
    // Extract name from "name" { or "name"{
    let name = header
        .trim()
        .trim_end_matches('{')
        .trim()
        .trim_matches('"')
        .to_string();

    let mut assertions = Vec::new();

    for line in lines.by_ref() {
        let trimmed = line.trim();

        if trimmed == "}" {
            break;
        }

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let assertion = parse_assertion(trimmed)?;
        assertions.push(assertion);
    }

    Ok(TestBlock { name, assertions })
}

fn parse_assertion(line: &str) -> Result<Assertion, String> {
    // Grammar assertion: @domain.type has variant  or  @domain has variant (default type)
    if line.starts_with('@') {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() == 3 && parts[1] == "has" {
            let path = parts[0].strip_prefix('@').unwrap();
            let (domain, type_name) = match path.split_once('.') {
                Some((d, t)) => (d.to_string(), t.to_string()),
                None => (path.to_string(), String::new()),
            };
            return Ok(Assertion::GrammarHas {
                domain,
                type_name,
                variant: parts[2].to_string(),
            });
        }
        return Err(format!(
            "expected '@domain has variant' or '@domain.type has variant', got: {}",
            line
        ));
    }

    // Output path assertion: .path.to.key
    if let Some(rest) = line.strip_prefix('.') {
        let path: Vec<String> = rest
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        if path.is_empty() {
            return Err("empty output path".into());
        }
        return Ok(Assertion::OutputPath(path));
    }

    Err(format!("unknown assertion: {}", line))
}

/// Run test blocks against output JSON and grammar registries.
pub fn run_tests(
    blocks: &[TestBlock],
    output: &serde_json::Value,
    grammars: &HashMap<String, TypeRegistry>,
) -> Vec<TestResult> {
    blocks
        .iter()
        .map(|b| run_block(b, output, grammars))
        .collect()
}

fn run_block(
    block: &TestBlock,
    output: &serde_json::Value,
    grammars: &HashMap<String, TypeRegistry>,
) -> TestResult {
    let assertions = block
        .assertions
        .iter()
        .map(|a| run_assertion(a, output, grammars))
        .collect();
    TestResult {
        name: block.name.clone(),
        assertions,
    }
}

fn run_assertion(
    assertion: &Assertion,
    output: &serde_json::Value,
    grammars: &HashMap<String, TypeRegistry>,
) -> AssertionResult {
    match assertion {
        Assertion::OutputPath(path) => {
            let mut current = output;
            for key in path {
                match current.get(key) {
                    Some(v) => current = v,
                    None => {
                        return AssertionResult {
                            assertion: assertion.clone(),
                            passed: false,
                            message: format!(".{} — key \"{}\" not found", path.join("."), key),
                        };
                    }
                }
            }
            AssertionResult {
                assertion: assertion.clone(),
                passed: true,
                message: format!(".{}", path.join(".")),
            }
        }
        Assertion::GrammarHas {
            domain,
            type_name,
            variant,
        } => match grammars.get(domain) {
            None => AssertionResult {
                assertion: assertion.clone(),
                passed: false,
                message: format!("@{} — grammar not found", domain),
            },
            Some(registry) => {
                let label = if type_name.is_empty() {
                    format!("@{}", domain)
                } else {
                    format!("@{}.{}", domain, type_name)
                };
                if registry.has_variant(type_name, variant) {
                    AssertionResult {
                        assertion: assertion.clone(),
                        passed: true,
                        message: format!("{} has {}", label, variant),
                    }
                } else {
                    AssertionResult {
                        assertion: assertion.clone(),
                        passed: false,
                        message: format!("{} has {} — variant not found", label, variant),
                    }
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- split_source --

    #[test]
    fn split_with_separator() {
        let source = "grammar @x { type = a }\n---\ntest \"t\" {\n  .x\n}";
        let (spec, tests) = split_source(source);
        assert_eq!(spec, "grammar @x { type = a }");
        assert!(tests.contains("test"));
    }

    #[test]
    fn split_without_separator() {
        let source = "grammar @x { type = a }";
        let (spec, tests) = split_source(source);
        assert_eq!(spec, source);
        assert!(tests.is_empty());
    }

    #[test]
    fn split_trailing_separator() {
        let source = "grammar @x { type = a }\n---";
        let (spec, tests) = split_source(source);
        assert_eq!(spec, "grammar @x { type = a }");
        assert!(tests.is_empty());
    }

    #[test]
    fn split_separator_no_trailing_newline() {
        let source = "grammar @x { type = a }\n---test content";
        let (spec, tests) = split_source(source);
        assert_eq!(spec, "grammar @x { type = a }");
        assert_eq!(tests, "test content");
    }

    // -- parse_tests --

    #[test]
    fn parse_output_path_assertion() {
        let source = "test \"check output\" {\n  .glue\n  .glue.signals\n}";
        let blocks = parse_tests(source).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].name, "check output");
        assert_eq!(blocks[0].assertions.len(), 2);
        assert_eq!(
            blocks[0].assertions[0],
            Assertion::OutputPath(vec!["glue".into()])
        );
        assert_eq!(
            blocks[0].assertions[1],
            Assertion::OutputPath(vec!["glue".into(), "signals".into()])
        );
    }

    #[test]
    fn parse_grammar_has_assertion() {
        let source = "test \"types\" {\n  @glue.signal has message\n  @glue.signal has exit\n}";
        let blocks = parse_tests(source).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].assertions.len(), 2);
        assert_eq!(
            blocks[0].assertions[0],
            Assertion::GrammarHas {
                domain: "glue".into(),
                type_name: "signal".into(),
                variant: "message".into(),
            }
        );
    }

    #[test]
    fn parse_multiple_blocks() {
        let source = "test \"a\" {\n  .x\n}\n\ntest \"b\" {\n  .y\n}";
        let blocks = parse_tests(source).unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].name, "a");
        assert_eq!(blocks[1].name, "b");
    }

    #[test]
    fn parse_skips_comments_and_blanks() {
        let source = "# comment\n\ntest \"t\" {\n  # inner comment\n  .x\n}";
        let blocks = parse_tests(source).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].assertions.len(), 1);
    }

    #[test]
    fn parse_error_on_unknown_line() {
        let source = "garbage";
        let err = parse_tests(source).unwrap_err();
        assert!(err.contains("unexpected"));
    }

    #[test]
    fn parse_error_on_bad_assertion() {
        let source = "test \"t\" {\n  garbage\n}";
        let err = parse_tests(source).unwrap_err();
        assert!(err.contains("unknown assertion"));
    }

    #[test]
    fn parse_default_type_grammar_assertion() {
        let source = "test \"t\" {\n  @glue has session\n}";
        let blocks = parse_tests(source).unwrap();
        assert_eq!(
            blocks[0].assertions[0],
            Assertion::GrammarHas {
                domain: "glue".into(),
                type_name: String::new(),
                variant: "session".into(),
            }
        );
    }

    #[test]
    fn parse_error_on_bad_grammar_syntax() {
        let source = "test \"t\" {\n  @glue.signal missing variant\n}";
        let err = parse_tests(source).unwrap_err();
        assert!(err.contains("@domain"));
    }

    #[test]
    fn parse_error_on_empty_output_path() {
        let source = "test \"t\" {\n  .\n}";
        let err = parse_tests(source).unwrap_err();
        assert!(err.contains("empty"));
    }

    // -- run_tests --

    #[test]
    fn run_output_path_passes() {
        let output: serde_json::Value = serde_json::json!({"glue": {"signals": {"message": {}}}});
        let blocks = vec![TestBlock {
            name: "t".into(),
            assertions: vec![
                Assertion::OutputPath(vec!["glue".into()]),
                Assertion::OutputPath(vec!["glue".into(), "signals".into()]),
                Assertion::OutputPath(vec!["glue".into(), "signals".into(), "message".into()]),
            ],
        }];
        let results = run_tests(&blocks, &output, &HashMap::new());
        assert!(results[0].passed());
    }

    #[test]
    fn run_output_path_fails_missing_key() {
        let output: serde_json::Value = serde_json::json!({"glue": {}});
        let blocks = vec![TestBlock {
            name: "t".into(),
            assertions: vec![Assertion::OutputPath(vec!["glue".into(), "signals".into()])],
        }];
        let results = run_tests(&blocks, &output, &HashMap::new());
        assert!(!results[0].passed());
        assert!(results[0].assertions[0].message.contains("not found"));
    }

    #[test]
    fn run_grammar_has_passes() {
        let source = "grammar @glue {\n  type signal = message | exit\n}\n";
        let ast = crate::Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let registry = TypeRegistry::compile(grammar).unwrap();
        let mut grammars = HashMap::new();
        grammars.insert("glue".to_string(), registry);

        let blocks = vec![TestBlock {
            name: "t".into(),
            assertions: vec![Assertion::GrammarHas {
                domain: "glue".into(),
                type_name: "signal".into(),
                variant: "message".into(),
            }],
        }];
        let results = run_tests(&blocks, &serde_json::Value::Null, &grammars);
        assert!(results[0].passed());
    }

    #[test]
    fn run_grammar_has_fails_missing_variant() {
        let source = "grammar @glue {\n  type signal = message\n}\n";
        let ast = crate::Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let registry = TypeRegistry::compile(grammar).unwrap();
        let mut grammars = HashMap::new();
        grammars.insert("glue".to_string(), registry);

        let blocks = vec![TestBlock {
            name: "t".into(),
            assertions: vec![Assertion::GrammarHas {
                domain: "glue".into(),
                type_name: "signal".into(),
                variant: "exit".into(),
            }],
        }];
        let results = run_tests(&blocks, &serde_json::Value::Null, &grammars);
        assert!(!results[0].passed());
        assert!(results[0].assertions[0].message.contains("not found"));
    }

    #[test]
    fn run_grammar_has_fails_missing_domain() {
        let blocks = vec![TestBlock {
            name: "t".into(),
            assertions: vec![Assertion::GrammarHas {
                domain: "missing".into(),
                type_name: "x".into(),
                variant: "y".into(),
            }],
        }];
        let results = run_tests(&blocks, &serde_json::Value::Null, &HashMap::new());
        assert!(!results[0].passed());
        assert!(results[0].assertions[0].message.contains("not found"));
    }

    #[test]
    fn run_mixed_pass_fail() {
        let output: serde_json::Value = serde_json::json!({"x": 1});
        let blocks = vec![TestBlock {
            name: "mixed".into(),
            assertions: vec![
                Assertion::OutputPath(vec!["x".into()]),
                Assertion::OutputPath(vec!["missing".into()]),
            ],
        }];
        let results = run_tests(&blocks, &output, &HashMap::new());
        assert!(!results[0].passed());
        assert!(results[0].assertions[0].passed);
        assert!(!results[0].assertions[1].passed);
    }

    // -- run_file --

    #[test]
    fn run_file_no_test_section() {
        let source = "grammar @x { type = a }";
        assert_eq!(run_file(source, ".", "test.conv"), 1);
    }

    #[test]
    fn run_file_parse_error() {
        let source = "grammar @x { type = a }\n---\ngarbage";
        assert_eq!(run_file(source, ".", "test.conv"), 1);
    }

    #[test]
    fn run_file_empty_blocks() {
        let source = "grammar @x { type = a }\n---\n# only comments\n";
        assert_eq!(run_file(source, ".", "test.conv"), 1);
    }

    #[test]
    fn run_file_invalid_spec() {
        let source = "{ unclosed\n---\ntest \"t\" {\n  .x\n}";
        assert_eq!(run_file(source, ".", "test.conv"), 1);
    }

    #[test]
    fn run_file_passing() {
        let source = "grammar @x {\n  type = a\n}\n\nin @x\n\nout x {\n  items {}\n}\n\n---\n\ntest \"root\" {\n  .x\n}\n\ntest \"grammar\" {\n  @x has a\n}";
        assert_eq!(run_file(source, ".", "test.conv"), 0);
    }

    #[test]
    fn run_file_failing() {
        let source = "grammar @x {\n  type = a\n}\n\nin @x\n\nout x {\n  items {}\n}\n\n---\n\ntest \"missing\" {\n  @x has b\n}";
        assert_eq!(run_file(source, ".", "test.conv"), 1);
    }
}
