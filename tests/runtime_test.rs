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
    let source = "\
grammar @linked {
  type color = red(shade) | blue
  type shade = light | dark
  requires connected
}
";
    let domain = parse_to_domain(source).expect("parse_to_domain should succeed");
    let result = check::verify(domain);
    assert!(
        result.is_ok(),
        "connected grammar should pass verification: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 2: disconnected grammar fails with readable error
// ---------------------------------------------------------------------------

#[test]
fn disconnected_grammar_fails_verification() {
    let source = "\
grammar @broken {
  type color = red | blue
  type shape = circle | square
  requires connected
}
";
    let domain = parse_to_domain(source).expect("parse_to_domain should succeed");
    let result = check::verify(domain);
    assert!(
        result.is_err(),
        "disconnected grammar should fail verification"
    );

    let violations = result.unwrap_err();
    let msg = format!("{}", violations);

    assert!(
        msg.contains("broken"),
        "error message should contain domain name 'broken': {}",
        msg
    );
    assert!(
        msg.contains("connected"),
        "error message should contain property name 'connected': {}",
        msg
    );
    assert!(
        msg.contains("disconnected"),
        "error message should contain 'disconnected': {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// Test 3: register and dispatch — non-actor grammar
// ---------------------------------------------------------------------------

#[tokio::test]
async fn register_and_dispatch_non_actor() {
    let source = "\
grammar @tools {
  type = exec | query
  action run {
    command: exec
  }
}
";
    let domain = parse_to_domain(source).expect("parse_to_domain should succeed");
    assert!(!domain.is_actor(), "tools domain should not be an actor");

    let verified = check::verify(domain).expect("tools domain should verify");

    let mut rt = RactorRuntime::new();
    rt.register(&verified)
        .await
        .expect("register should succeed");

    let domain_name = DomainName::new("tools");
    let action_name = ActionName::new("run");
    let resp = rt
        .dispatch(&domain_name, &action_name, Args::Empty)
        .await
        .expect("dispatch should succeed");

    assert!(
        matches!(resp, Response::Ok(_)),
        "dispatch should return Ok: {:?}",
        resp
    );
}

// ---------------------------------------------------------------------------
// Test 4: register and dispatch — actor grammar
// ---------------------------------------------------------------------------

#[tokio::test]
async fn register_and_dispatch_actor() {
    let source = "\
in @actor

grammar @compiler {
  type = target | artifact
  action compile {
    source: artifact
  }
}
";
    let domain = parse_to_domain(source).expect("parse_to_domain should succeed");
    assert!(domain.is_actor(), "compiler domain should be an actor");

    let verified = check::verify(domain).expect("compiler domain should verify");

    let mut rt = RactorRuntime::new();
    rt.register(&verified)
        .await
        .expect("register should succeed");

    let domain_name = DomainName::new("compiler");
    let action_name = ActionName::new("compile");
    let resp = rt
        .dispatch(
            &domain_name,
            &action_name,
            Args::Single(Value::Text("main.conv".into())),
        )
        .await
        .expect("dispatch should succeed");

    assert!(
        matches!(resp, Response::Ok(_)),
        "actor dispatch should return Ok: {:?}",
        resp
    );

    rt.shutdown(&domain_name)
        .await
        .expect("shutdown should succeed");
}

// ---------------------------------------------------------------------------
// Test 5: error message readable — the key litmus test
// ---------------------------------------------------------------------------

#[test]
fn error_message_readable() {
    let source = "\
grammar @training {
  type signal = feedback | reinforcement
  type example = positive | negative
  type session = active | paused
  type batch = full | partial
  type metric = loss | accuracy
  type checkpoint = saved | pending
  invariant connected
}
";
    let domain = parse_to_domain(source).expect("parse_to_domain should succeed");
    let result = check::verify(domain);
    assert!(
        result.is_err(),
        "training grammar with 6 disconnected types should fail verification"
    );

    let violations = result.unwrap_err();
    let msg = format!("{}", violations);

    eprintln!("--- litmus test error output ---\n{}\n--- end ---", msg);

    assert!(
        msg.contains("6 disconnected components"),
        "error message should report '6 disconnected components': {}",
        msg
    );
}
