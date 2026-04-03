//! Integration test — the litmus test.
//!
//! Full pipeline: `.conv` source text → parse → Domain → verify → Runtime dispatch.
//! Key property: violated invariants produce clear, structured error messages entirely in Rust.
//!
//! Test 6 proves the inference physics pipeline end-to-end:
//! .conv source → parse → Domain → verify with spectrum → actor boots with schedule
//! → decide uses temperature from eigenvalues.

use conversation::check;
use conversation::model::*;
use conversation::parse::Parse;
use conversation::runtime::*;
use conversation::Runtime;
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
// Test 3: compile and dispatch — non-actor grammar
// ---------------------------------------------------------------------------

#[tokio::test]
async fn compile_and_dispatch_non_actor() {
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
    let artifact = rt.compile(verified).await.expect("compile should succeed");

    let resp = ractor::call!(
        artifact,
        DomainMessage::Dispatch,
        ActionName::new("run"),
        Args::Empty
    )
    .expect("dispatch should succeed");

    assert!(
        matches!(resp, Ok(Response::Ok(_))),
        "dispatch should return Ok: {:?}",
        resp
    );

    artifact.stop(None);
}

// ---------------------------------------------------------------------------
// Test 4: compile and dispatch — actor grammar
// ---------------------------------------------------------------------------

#[tokio::test]
async fn compile_and_dispatch_actor() {
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
    let artifact = rt.compile(verified).await.expect("compile should succeed");

    let resp = ractor::call!(
        artifact,
        DomainMessage::Dispatch,
        ActionName::new("compile"),
        Args::Single(Value::Text("main.conv".into()))
    )
    .expect("dispatch should succeed");

    assert!(
        matches!(resp, Ok(Response::Ok(_))),
        "actor dispatch should return Ok: {:?}",
        resp
    );

    artifact.stop(None);
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

// ---------------------------------------------------------------------------
// Test 6: inference physics litmus — full pipeline
// ---------------------------------------------------------------------------

/// End-to-end proof that the inference physics pipeline works:
///
/// .conv source → parse → Domain → verify (spectrum computed) →
/// InferenceSchedule (Diffusion with eigenvalues) → temperature from
/// eigenvalues → actor boots with schedule → dispatch through runtime.
///
/// The grammar uses a parameterized variant (`combo(color)`) to create a
/// type reference edge, ensuring a non-trivial Laplacian spectrum.
#[tokio::test]
async fn inference_physics_litmus() {
    // Grammar with type references → non-trivial spectrum.
    // `combo(color)` creates an edge from `pair` to `color` in the type graph.
    let source = "\
in @actor

grammar @test {
  type color = red | blue
  type pair = combo(color)
  action decide {}
}
";

    // Parse
    let domain = parse_to_domain(source).expect("parse_to_domain should succeed");
    assert_eq!(domain.domain_name(), "test");
    assert!(domain.is_actor(), "test domain should be an actor");

    // Verify — spectrum computed here
    let verified = check::verify(domain).expect("test domain should verify");
    assert!(
        matches!(verified.complexity(), DomainComplexity::Spectrum(_)),
        "expected Spectrum complexity for a grammar with type references"
    );

    // Schedule from verified
    let schedule = InferenceSchedule::from_verified(&verified);
    match &schedule {
        InferenceSchedule::Diffusion(ev) => {
            // Fiedler value must be positive — the graph is connected.
            assert!(
                ev.fiedler_value().unwrap() > 0.0,
                "Fiedler value should be positive for connected type graph"
            );

            // Temperature behavior:
            // temperature_at(t) = K(t)/n where K(t) = sum exp(-lambda_i * t).
            // At t=0: K(0) = n, so temperature = 1.0 (max).
            // At t>0: K decays, so temperature < 1.0.
            //
            // diffusion_time(0.0) = 0.0 → temperature_at(0.0) = 1.0
            // diffusion_time(1.0) = 1/fiedler → temperature_at(1/fiedler) < 1.0
            //
            // Therefore: temperature(0.0) > temperature(1.0).
            // Higher context_complexity → longer diffusion → more cooling → LOWER temperature.
            let temp_full = schedule.temperature(1.0);
            let temp_zero = schedule.temperature(0.0);
            assert!(
                temp_full > 0.0,
                "temperature at full complexity should be positive"
            );
            assert!(
                temp_zero > temp_full,
                "temperature at zero complexity ({}) should be greater than at full complexity ({})",
                temp_zero,
                temp_full,
            );
        }
        InferenceSchedule::Immediate => {
            panic!("expected Diffusion schedule for non-trivial spectrum")
        }
    }

    // Runtime: compile to artifact, dispatch, stop.
    // The actor booted with the schedule's eigenvalues baked in.
    let mut runtime = RactorRuntime::new();
    let artifact = runtime
        .compile(verified)
        .await
        .expect("compile should succeed");

    // Dispatch "decide" — the action exists so the actor handles it.
    let resp = ractor::call!(
        artifact,
        DomainMessage::Dispatch,
        ActionName::new("decide"),
        Args::Empty
    )
    .expect("dispatch should succeed");
    assert!(
        matches!(resp, Ok(Response::Ok(_))),
        "actor dispatch should return Ok: {:?}",
        resp
    );

    artifact.stop(None);
}
