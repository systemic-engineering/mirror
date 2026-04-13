//! Integration tests for `mirror lsp learn` command.

use mirror::lsp::{generate, language, node_types};

#[test]
fn detect_all_supported_languages() {
    let langs = [
        "python",
        "py",
        "rust",
        "rs",
        "gleam",
        "javascript",
        "js",
        "typescript",
        "ts",
        "nix",
    ];
    for lang in &langs {
        assert!(
            language::detect(lang).is_some(),
            "should detect language: {}",
            lang
        );
    }
}

#[test]
fn detect_unknown_language() {
    assert!(language::detect("brainfuck").is_none());
}

#[test]
fn parse_python_fixture_and_generate_grammar() {
    let fixture = include_str!("../fixtures/node-types/python.json");
    let types = node_types::parse_node_types(fixture).unwrap();

    // Python has ~129 named types.
    assert!(types.len() > 100, "got {} types", types.len());

    let config = language::detect("python").unwrap();
    let caps = language::LspCapabilities::all();
    let grammar = generate::generate_grammar(&config, &types, &caps);

    // Structure checks.
    assert!(grammar.starts_with("-- auto-generated"));
    assert!(grammar.contains("-- tree-sitter: tree-sitter-python"));
    assert!(grammar.contains("-- lsp: pyright-langserver"));
    assert!(grammar.contains("in @code"));
    assert!(grammar.contains("grammar @code/python {"));
    assert!(grammar.ends_with("out @code/python\n"));

    // Key Python types.
    assert!(grammar.contains("function_definition"));
    assert!(grammar.contains("class_definition"));
    assert!(grammar.contains("identifier"));
    assert!(grammar.contains("import_statement"));

    // Function definition should be a product type with fields.
    assert!(grammar.contains("name: identifier"));
    assert!(grammar.contains("body: block"));

    // All 6 LSP actions.
    assert!(grammar.contains("action complete(position) in @code/python { }"));
    assert!(grammar.contains("action diagnose(range) in @code/python { }"));
    assert!(grammar.contains("action hover(position) in @code/python { }"));
    assert!(grammar.contains("action definition(position) in @code/python { }"));
    assert!(grammar.contains("action references(position) in @code/python { }"));
    assert!(grammar.contains("action tokens(range) in @code/python { }"));

    // No "not supported" section when all capabilities are present.
    assert!(!grammar.contains("-- not supported by server"));
}

#[test]
fn generate_grammar_without_lsp() {
    let fixture = include_str!("../fixtures/node-types/python.json");
    let types = node_types::parse_node_types(fixture).unwrap();

    let config = language::detect("python").unwrap();
    let caps = language::LspCapabilities::default();
    let grammar = generate::generate_grammar(&config, &types, &caps);

    // No active actions.
    assert!(!grammar.contains("    action complete"));
    assert!(!grammar.contains("    action hover"));

    // All actions should be commented out.
    assert!(grammar.contains("-- not supported by server"));
    assert!(grammar.contains("-- action complete(position) in @code/python { }"));
}

#[test]
fn generate_grammar_partial_lsp() {
    let config = language::detect("rust").unwrap();
    let caps = language::LspCapabilities {
        completion: true,
        hover: true,
        definition: true,
        references: true,
        diagnostics: true,
        semantic_tokens: false,
    };
    let grammar = generate::generate_grammar(&config, &[], &caps);

    assert!(grammar.contains("action complete(position) in @code/rust { }"));
    assert!(grammar.contains("action hover(position) in @code/rust { }"));
    // semantic_tokens not supported — should be commented.
    assert!(grammar.contains("-- action tokens(range) in @code/rust { }"));
}

#[test]
fn generate_grammar_for_all_languages() {
    let fixture = include_str!("../fixtures/node-types/python.json");
    let types = node_types::parse_node_types(fixture).unwrap();
    let caps = language::LspCapabilities::all();

    // Use the same types for all languages — the grammar structure
    // should still be valid regardless of language name.
    for lang in &["python", "rust", "gleam", "typescript", "javascript", "nix"] {
        let config = language::detect(lang).unwrap();
        let grammar = generate::generate_grammar(&config, &types, &caps);
        assert!(
            grammar.contains(&format!("grammar @code/{}", config.name)),
            "grammar for {} should have correct name",
            lang
        );
        assert!(
            grammar.contains(&format!("out @code/{}", config.name)),
            "grammar for {} should have out declaration",
            lang
        );
    }
}

#[test]
fn node_types_three_categories() {
    let fixture = include_str!("../fixtures/node-types/python.json");
    let types = node_types::parse_node_types(fixture).unwrap();

    let leaves = types
        .iter()
        .filter(|t| matches!(t, node_types::NodeType::Leaf { .. }))
        .count();
    let sums = types
        .iter()
        .filter(|t| matches!(t, node_types::NodeType::Sum { .. }))
        .count();
    let products = types
        .iter()
        .filter(|t| matches!(t, node_types::NodeType::Product { .. }))
        .count();

    assert!(leaves > 0, "should have leaf types");
    assert!(sums > 0, "should have sum types");
    assert!(products > 0, "should have product types");
    assert_eq!(leaves + sums + products, types.len());
}

#[test]
fn write_grammar_to_file() {
    let dir = tempfile::TempDir::new().unwrap();
    let out_file = dir.path().join("python.mirror");

    let fixture = include_str!("../fixtures/node-types/python.json");
    let types = node_types::parse_node_types(fixture).unwrap();
    let config = language::detect("python").unwrap();
    let caps = language::LspCapabilities::all();
    let grammar = generate::generate_grammar(&config, &types, &caps);

    std::fs::write(&out_file, &grammar).unwrap();

    let read_back = std::fs::read_to_string(&out_file).unwrap();
    assert_eq!(grammar, read_back);
}
