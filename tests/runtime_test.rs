//! Integration test — the litmus test.
//!
//! Full pipeline: `.conv` source text → parse → Domain → verify → Runtime dispatch.
//! Key property: violated invariants produce clear, structured error messages entirely in Rust.

use conversation::check;
use conversation::model::*;
use conversation::parse::Parse;
use conversation::runtime::*;
use conversation::Vector;

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/// Parse a .conv source into a Domain.
fn parse_to_domain(source: &str) -> Result<Domain, String> {
    let ast = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| e.to_string())?;

    let grammar_node = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .ok_or_else(|| "no grammar block found".to_string())?;

    let lenses: Vec<String> = ast
        .children()
        .iter()
        .filter(|c| c.data().is_decl("in"))
        .map(|c| c.data().value.clone())
        .collect();

    Domain::from_grammar_with_lenses(grammar_node, &lenses)
}

// ---------------------------------------------------------------------------
// Test 1: connected grammar verifies
// ---------------------------------------------------------------------------

#[test]
fn connected_grammar_verifies() {
    todo!("implement: parse connected grammar and assert verify succeeds")
}

// ---------------------------------------------------------------------------
// Test 2: disconnected grammar fails with readable error
// ---------------------------------------------------------------------------

#[test]
fn disconnected_grammar_fails_verification() {
    todo!("implement: parse disconnected grammar and assert error contains domain/property names")
}

// ---------------------------------------------------------------------------
// Test 3: register and dispatch — non-actor grammar
// ---------------------------------------------------------------------------

#[tokio::test]
async fn register_and_dispatch_non_actor() {
    todo!("implement: parse tools grammar, verify, register, dispatch run action")
}

// ---------------------------------------------------------------------------
// Test 4: register and dispatch — actor grammar
// ---------------------------------------------------------------------------

#[tokio::test]
async fn register_and_dispatch_actor() {
    todo!("implement: parse actor grammar, verify, register actor, dispatch compile, shutdown")
}

// ---------------------------------------------------------------------------
// Test 5: error message readable — the key litmus test
// ---------------------------------------------------------------------------

#[test]
fn error_message_readable() {
    todo!("implement: parse 6-type disconnected grammar, assert '6 disconnected components' in error")
}
