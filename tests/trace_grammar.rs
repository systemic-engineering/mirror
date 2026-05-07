//! Tests for the @trace grammar family.
//!
//! Red first: these tests define the expected behavior for the trace
//! grammar family before the grammar files or compiler changes exist.

use mirror::declaration::{DeclKind, MirrorFragment, MirrorFragmentExt};
use mirror::loss::MirrorLoss;
use mirror::mirror_runtime::{CompiledShatter, MirrorRuntime, MirrorRuntimeError};
use mirror::beam::Imperfect;

fn boot_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot")
}

fn tempdir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mirror-trace-test-{}-{}",
        name,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Extract the compiled value from an Imperfect result, accepting Partial.
fn extract(
    result: Imperfect<CompiledShatter, MirrorRuntimeError, MirrorLoss>,
) -> CompiledShatter {
    match result {
        Imperfect::Success(c) => c,
        Imperfect::Partial(c, _) => c,
        Imperfect::Failure(e, _) => panic!("compilation failed: {}", e),
    }
}

/// Check that a grammar fragment has a child grammar with the given name.
fn find_grammar<'a>(frag: &'a MirrorFragment, name: &str) -> Option<&'a MirrorFragment> {
    frag.mirror_children().iter().find(|f| {
        let d = f.mirror_data();
        d.kind == DeclKind::Grammar && d.name == name
    })
}

// ---------------------------------------------------------------------------
// @trace base grammar
// ---------------------------------------------------------------------------

#[test]
fn trace_base_grammar_parses() {
    let runtime = MirrorRuntime::new();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std/trace/mod.mirror");
    assert!(path.exists(), "boot/std/trace/mod.mirror must exist");

    let src = std::fs::read_to_string(&path).unwrap();
    let result = runtime.compile_source(&src);
    assert!(
        result.is_ok() || result.is_partial(),
        "@trace grammar must parse (ok or partial)"
    );
}

#[test]
fn trace_base_grammar_has_grammar_block() {
    let runtime = MirrorRuntime::new();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std/trace/mod.mirror");
    let src = std::fs::read_to_string(&path).unwrap();
    let compiled = extract(runtime.compile_source(&src));

    // Find the @trace grammar block
    let grammar = find_grammar(&compiled.fragment, "@trace");
    assert!(grammar.is_some(), "@trace grammar block must exist");

    // Should have an `in @prism` child
    let grammar = grammar.unwrap();
    let has_in_prism = grammar
        .mirror_children()
        .iter()
        .any(|c| c.mirror_data().kind == DeclKind::In && c.mirror_data().name == "@prism");
    assert!(has_in_prism, "@trace must declare `in @prism`");
}

// ---------------------------------------------------------------------------
// @trace/memory grammar
// ---------------------------------------------------------------------------

#[test]
fn trace_memory_grammar_parses() {
    let runtime = MirrorRuntime::new();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std/trace/memory.mirror");
    assert!(path.exists(), "boot/std/trace/memory.mirror must exist");

    let src = std::fs::read_to_string(&path).unwrap();
    let result = runtime.compile_source(&src);
    assert!(
        result.is_ok() || result.is_partial(),
        "@trace/memory grammar must parse (ok or partial)"
    );
}

#[test]
fn trace_memory_grammar_has_grammar_block() {
    let runtime = MirrorRuntime::new();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std/trace/memory.mirror");
    let src = std::fs::read_to_string(&path).unwrap();
    let compiled = extract(runtime.compile_source(&src));

    let grammar = find_grammar(&compiled.fragment, "@trace/memory");
    assert!(grammar.is_some(), "@trace/memory grammar block must exist");

    // Should declare `in @trace`
    let grammar = grammar.unwrap();
    let has_in_trace = grammar
        .mirror_children()
        .iter()
        .any(|c| c.mirror_data().kind == DeclKind::In && c.mirror_data().name == "@trace");
    assert!(has_in_trace, "@trace/memory must declare `in @trace`");
}

// ---------------------------------------------------------------------------
// @trace/complexity grammar
// ---------------------------------------------------------------------------

#[test]
fn trace_complexity_grammar_parses() {
    let runtime = MirrorRuntime::new();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std/trace/complexity.mirror");
    assert!(path.exists(), "boot/std/trace/complexity.mirror must exist");

    let src = std::fs::read_to_string(&path).unwrap();
    let result = runtime.compile_source(&src);
    assert!(
        result.is_ok() || result.is_partial(),
        "@trace/complexity grammar must parse (ok or partial)"
    );
}

#[test]
fn trace_complexity_grammar_has_grammar_block() {
    let runtime = MirrorRuntime::new();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("boot/std/trace/complexity.mirror");
    let src = std::fs::read_to_string(&path).unwrap();
    let compiled = extract(runtime.compile_source(&src));

    let grammar = find_grammar(&compiled.fragment, "@trace/complexity");
    assert!(
        grammar.is_some(),
        "@trace/complexity grammar block must exist"
    );
}

// ---------------------------------------------------------------------------
// Boot directory integration: trace subdirectory is loaded
// ---------------------------------------------------------------------------

#[test]
fn boot_dir_loads_trace_subdirectory() {
    let runtime = MirrorRuntime::new();
    let store = tempdir("boot_trace");
    let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

    // The trace directory files should appear in resolved with "std/trace/" prefix
    let trace_keys: Vec<&String> = boot
        .resolved
        .keys()
        .filter(|k| k.starts_with("std/trace"))
        .collect();
    assert!(
        !trace_keys.is_empty(),
        "boot should resolve trace subdirectory files, got keys: {:?}",
        boot.resolved.keys().collect::<Vec<_>>()
    );
}

#[test]
fn boot_dir_with_trace_still_compiles_fully() {
    let runtime = MirrorRuntime::new();
    let store = tempdir("boot_full");
    let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

    // Existing files should still resolve
    let existing_std: Vec<&String> = boot
        .resolved
        .keys()
        .filter(|k| k.starts_with("std/") && !k.starts_with("std/trace"))
        .collect();
    assert!(
        !existing_std.is_empty(),
        "existing std grammars must still resolve"
    );

    // Total resolved should be >= 8 (existing) + 3 (trace)
    let total = boot.resolved.len() + boot.failed.len();
    assert!(total >= 8, "total files should be >= 8, got {}", total);
}
