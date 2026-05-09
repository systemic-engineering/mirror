//! Integration tests for @code/rust kintsugi — Rust source to MirrorAST conversion.

use mirror::code_rust::*;
use mirror::ast::Ast;
use mirror::mirror_ast::MirrorAST;
use mirror::grammar_regions::{self, GrammarId};

// ---------------------------------------------------------------------------
// Parsing tests
// ---------------------------------------------------------------------------

#[test]
fn parse_simple_function() {
    let src = "fn hello(name: String) -> bool { true }";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Function {
            name,
            params,
            return_type,
            is_pub,
            ..
        } => {
            assert_eq!(name, "hello");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].0, "name");
            assert_eq!(params[0].1, "String");
            assert_eq!(return_type.as_deref(), Some("bool"));
            assert!(!is_pub);
        }
        other => panic!("expected Function, got {:?}", other),
    }
}

#[test]
fn parse_pub_function() {
    let src = "pub fn greet(who: &str) { println!(\"{}\", who); }";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Function { name, is_pub, .. } => {
            assert_eq!(name, "greet");
            assert!(is_pub);
        }
        other => panic!("expected Function, got {:?}", other),
    }
}

#[test]
fn parse_simple_struct() {
    let src = "pub struct Point {\n    pub x: f64,\n    pub y: f64,\n}";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Struct {
            name,
            fields,
            is_pub,
        } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].0, "x");
            assert_eq!(fields[0].1, "f64");
            assert_eq!(fields[1].0, "y");
            assert_eq!(fields[1].1, "f64");
            assert!(is_pub);
        }
        other => panic!("expected Struct, got {:?}", other),
    }
}

#[test]
fn parse_simple_enum() {
    let src = "pub enum Color {\n    Red,\n    Green,\n    Blue,\n}";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Enum {
            name,
            variants,
            is_pub,
        } => {
            assert_eq!(name, "Color");
            assert_eq!(variants, &["Red", "Green", "Blue"]);
            assert!(is_pub);
        }
        other => panic!("expected Enum, got {:?}", other),
    }
}

#[test]
fn parse_impl_block() {
    let src = "impl Point {\n    fn new(x: f64, y: f64) -> Self { Point { x, y } }\n}";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Impl {
            target,
            trait_name,
            items: inner,
            ..
        } => {
            assert_eq!(target, "Point");
            assert!(trait_name.is_none());
            assert_eq!(inner.len(), 1);
            match &inner[0] {
                RustItem::Function { name, .. } => assert_eq!(name, "new"),
                other => panic!("expected inner Function, got {:?}", other),
            }
        }
        other => panic!("expected Impl, got {:?}", other),
    }
}

#[test]
fn parse_trait_impl() {
    let src = "impl Display for Point {\n    fn fmt(&self, f: &mut Formatter) -> Result { Ok(()) }\n}";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Impl {
            target,
            trait_name,
            ..
        } => {
            assert_eq!(target, "Point");
            assert_eq!(trait_name.as_deref(), Some("Display"));
        }
        other => panic!("expected Impl, got {:?}", other),
    }
}

#[test]
fn parse_use_statement() {
    let src = "use std::collections::HashMap;";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Use { path, is_pub } => {
            assert_eq!(path, "std::collections::HashMap");
            assert!(!is_pub);
        }
        other => panic!("expected Use, got {:?}", other),
    }
}

#[test]
fn parse_trait_definition() {
    let src = "pub trait Greetable {\n    fn greet(&self) -> String;\n}";
    let items = parse_rust_items(src);
    assert_eq!(items.len(), 1);
    match &items[0] {
        RustItem::Trait { name, is_pub, .. } => {
            assert_eq!(name, "Greetable");
            assert!(is_pub);
        }
        other => panic!("expected Trait, got {:?}", other),
    }
}

#[test]
fn parse_multiple_items() {
    let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn origin() -> Self { Point { x: 0.0, y: 0.0 } }
}

pub fn distance(a: Point, b: Point) -> f64 { 0.0 }
";
    let items = parse_rust_items(src);
    let non_comment: Vec<&RustItem> = items
        .iter()
        .filter(|i| !matches!(i, RustItem::Comment(_) | RustItem::Other(_)))
        .collect();
    assert!(
        non_comment.len() >= 4,
        "expected at least 4 items (use, struct, impl, fn), got {}: {:?}",
        non_comment.len(),
        non_comment
    );
}

#[test]
fn parse_comments_preserved() {
    let src = "// This is a comment\nfn foo() { }";
    let items = parse_rust_items(src);
    assert!(items.iter().any(|i| matches!(i, RustItem::Comment(_))));
    assert!(items.iter().any(|i| matches!(i, RustItem::Function { .. })));
}

// ---------------------------------------------------------------------------
// MirrorAST conversion tests
// ---------------------------------------------------------------------------

#[test]
fn function_becomes_zoom() {
    let src = "fn hello(name: String) -> bool { true }";
    let items = parse_rust_items(src);
    let ast = item_to_mirror_ast(&items[0]).unwrap();
    assert!(matches!(ast, MirrorAST::Zoom(_)));
    assert_eq!(ast.name(), "hello");
}

#[test]
fn struct_becomes_split() {
    let src = "pub struct Point {\n    pub x: f64,\n    pub y: f64,\n}";
    let items = parse_rust_items(src);
    let ast = item_to_mirror_ast(&items[0]).unwrap();
    assert!(matches!(ast, MirrorAST::Split(_)));
    assert_eq!(ast.name(), "Point");
}

#[test]
fn enum_becomes_split_with_variants() {
    let src = "pub enum Color {\n    Red,\n    Green,\n    Blue,\n}";
    let items = parse_rust_items(src);
    let ast = item_to_mirror_ast(&items[0]).unwrap();
    match &ast {
        MirrorAST::Split(s) => {
            assert_eq!(s.name.as_str(), "Color");
            assert_eq!(s.variants.len(), 3);
        }
        other => panic!("expected Split, got {:?}", other),
    }
}

#[test]
fn impl_becomes_focus() {
    let src = "impl Point {\n    fn new() -> Self { Point {} }\n}";
    let items = parse_rust_items(src);
    let ast = item_to_mirror_ast(&items[0]).unwrap();
    assert!(matches!(ast, MirrorAST::Focus(_)));
    assert_eq!(ast.name(), "Point");
}

#[test]
fn use_becomes_project() {
    let src = "use std::collections::HashMap;";
    let items = parse_rust_items(src);
    let ast = item_to_mirror_ast(&items[0]).unwrap();
    assert!(matches!(ast, MirrorAST::Project(_)));
}

#[test]
fn trait_becomes_refract() {
    let src = "pub trait Greetable {\n    fn greet(&self) -> String;\n}";
    let items = parse_rust_items(src);
    let ast = item_to_mirror_ast(&items[0]).unwrap();
    assert!(matches!(ast, MirrorAST::Refract(_)));
    assert_eq!(ast.name(), "Greetable");
}

// ---------------------------------------------------------------------------
// Full file conversion tests
// ---------------------------------------------------------------------------

#[test]
fn rust_file_becomes_module() {
    let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
";
    let ast = rust_to_mirror_ast("point.rs", src);
    match &ast {
        MirrorAST::Module(m) => {
            assert_eq!(m.name.as_str(), "point.rs");
            assert!(
                m.children.len() >= 2,
                "expected at least Project + Split + Zoom, got {}",
                m.children.len()
            );
        }
        other => panic!("expected Module, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Base AST conversion tests (for kintsugi)
// ---------------------------------------------------------------------------

#[test]
fn rust_to_base_ast_produces_body() {
    let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
}

pub fn origin() -> Point { Point { x: 0.0 } }
";
    let ast = rust_to_base_ast("point.rs", src);
    assert!(matches!(ast, Ast::Body(_)));
    let node_count = ast.node_count();
    assert!(node_count > 0, "base AST should have nodes");
}

#[test]
fn base_ast_function_is_action_call() {
    let src = "fn hello(name: String) -> bool { true }";
    let ast = rust_to_base_ast("test.rs", src);
    if let Ast::Body(body) = &ast {
        let child = &body.children()[0];
        assert!(
            child.is_call("action"),
            "fn should become action Call, got {:?}",
            child
        );
        assert_eq!(child.decl_name(), Some("hello"));
    } else {
        panic!("expected Body");
    }
}

#[test]
fn base_ast_struct_is_type_call() {
    let src = "pub struct Point {\n    pub x: f64,\n}";
    let ast = rust_to_base_ast("test.rs", src);
    if let Ast::Body(body) = &ast {
        let child = &body.children()[0];
        assert!(
            child.is_call("type"),
            "struct should become type Call, got {:?}",
            child
        );
        assert_eq!(child.decl_name(), Some("Point"));
    } else {
        panic!("expected Body");
    }
}

// ---------------------------------------------------------------------------
// Kintsugi pipeline tests (the money tests)
// ---------------------------------------------------------------------------

#[test]
fn kintsugi_eliminate_dead_on_rust_ast() {
    // Build a Rust file with a struct referenced by a function,
    // and another struct not referenced by anything.
    let src = "\
pub struct Used {
    pub x: f64,
}

pub struct Orphan {
    pub y: f64,
}

pub fn process(input: Used) -> Used { input }
";
    let ast = rust_to_base_ast("test.rs", src);
    let before = ast.node_count();
    let simplified = ast.eliminate_dead();
    let after = simplified.node_count();
    // eliminate_dead should remove the Orphan struct
    assert!(
        after <= before,
        "eliminate_dead should not increase nodes: {} -> {}",
        before,
        after
    );

    // Verify Used survives (it's referenced by action)
    let mut found_used = false;
    simplified.walk(&mut |node| {
        if let Ast::Atom(a) = node {
            if a.as_str() == "Used" {
                found_used = true;
            }
        }
    });
    assert!(found_used, "Used type should survive eliminate_dead");
}

#[test]
fn kintsugi_pipeline_reduces_rust_ast() {
    // A Rust file with duplicate type aliases and dead code.
    let src = "\
pub enum Status {
    Active,
    Inactive,
}

pub enum State {
    Active,
    Inactive,
}

pub struct Orphan {
    pub x: f64,
}

pub fn process(input: Status) -> Status { input }
";
    let ast = rust_to_base_ast("test.rs", src);
    let before = ast.node_count();
    let simplified = ast.collapse_aliases().flatten_wrappers().eliminate_dead();
    let after = simplified.node_count();
    assert!(
        after < before,
        "kintsugi pipeline should reduce nodes: {} -> {}",
        before,
        after
    );
}

#[test]
fn kintsugi_preserves_depth_bound() {
    let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }
    pub fn origin() -> Self { Point { x: 0.0, y: 0.0 } }
}

pub fn distance(a: Point, b: Point) -> f64 { 0.0 }
";
    let ast = rust_to_base_ast("test.rs", src);
    let simplified = ast
        .clone()
        .collapse_aliases()
        .flatten_wrappers()
        .eliminate_dead();
    assert!(
        simplified.depth() <= ast.depth(),
        "kintsugi should not increase depth: {} -> {}",
        ast.depth(),
        simplified.depth()
    );
}

// ---------------------------------------------------------------------------
// Metrics tests
// ---------------------------------------------------------------------------

#[test]
fn metrics_count_items() {
    let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }
}

pub fn distance(a: Point, b: Point) -> f64 { 0.0 }

pub trait Measurable {
    fn measure(&self) -> f64;
}
";
    let items = parse_rust_items(src);
    let metrics = compute_metrics(&items);
    assert!(metrics.use_count >= 1, "should count use statements");
    assert!(metrics.type_count >= 1, "should count struct/enum");
    assert!(metrics.impl_count >= 1, "should count impl blocks");
    assert!(
        metrics.fn_count >= 2,
        "should count functions (including impl methods)"
    );
    assert!(metrics.trait_count >= 1, "should count traits");
}

// ---------------------------------------------------------------------------
// Integration with grammar_regions
// ---------------------------------------------------------------------------

#[test]
fn grammar_regions_identifies_rust_file() {
    let grammar = grammar_regions::primary_grammar("src/main.rs");
    assert_eq!(grammar, GrammarId::new("@code/rust"));
}

#[test]
fn full_pipeline_parse_regions_then_convert() {
    let src = "\
//! Module documentation.

use std::collections::HashMap;

/// A point in 2D space.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
";
    // Step 1: grammar_regions splits into regions
    let grammar = GrammarId::new("@code/rust");
    let regions = grammar_regions::split_regions(src, &grammar);
    assert!(!regions.is_empty());

    // Step 2: collect code regions
    let code_content: String = regions
        .iter()
        .filter(|r| r.grammar == GrammarId::new("@code/rust"))
        .map(|r| r.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    // Step 3: parse Rust items from code regions
    let items = parse_rust_items(&code_content);
    assert!(!items.is_empty(), "should parse items from code regions");

    // Step 4: convert to MirrorAST
    let mirror_ast = rust_to_mirror_ast("test.rs", src);
    match &mirror_ast {
        MirrorAST::Module(m) => {
            assert!(!m.children.is_empty(), "module should have children");
        }
        other => panic!("expected Module, got {:?}", other),
    }

    // Step 5: convert to base AST and run kintsugi
    let base_ast = rust_to_base_ast("test.rs", src);
    let before = base_ast.node_count();
    let simplified = base_ast.collapse_aliases().flatten_wrappers().eliminate_dead();
    let after = simplified.node_count();
    // Even without duplicates, pipeline should at least not increase nodes
    assert!(
        after <= before,
        "kintsugi pipeline should not increase nodes: {} -> {}",
        before,
        after
    );
}
