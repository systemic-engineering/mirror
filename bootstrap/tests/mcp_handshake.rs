//! MCP handshake fixture tests — the bash→Rust lift verification.
//!
//! Tick 1 of the mirror-mcp+lsp self-improving loop (Seam pre-loop
//! review at `docs/specs/seam-pre-loop-mirror-mcp-lsp-review-2026-06-18.md`,
//! Finding 2.3.B). These tests assert that the Rust MCP server in
//! `bootstrap/src/mcp.rs` produces byte-equal responses to the
//! 145-line bash wrapper at `bin/mirror-mcp` for the canonical
//! JSON-RPC handshake messages.
//!
//! The fixtures in `bootstrap/tests/mcp_fixtures/` were captured by
//! Taut on 2026-06-18 by piping requests through the bash wrapper and
//! recording the responses verbatim. The lift is correct iff every
//! `*.req.json` fixture, fed through `mirror::mcp::handle_request`,
//! produces the matching `*.resp.json` line.
//!
//! Pure substrate tests — no subprocess spawn, no installed binary required.
//!
//! ## Comparison shape: semantic JSON equality
//!
//! The bash wrapper hand-rolled responses as ordered byte strings
//! (key order: `jsonrpc, id, result`). `serde_json`'s `Value`-based
//! serialization re-emits keys in sorted order. JSON-RPC clients
//! (the agent on the other end of the stdio pipe) parse JSON; they
//! do not care about key ordering. The substrate-pull-natural
//! verification is therefore **semantic equality** — parse both,
//! compare `Value`s. Content drift IS still caught (any tool added,
//! removed, or renamed shows up as a `Value` mismatch); only
//! whitespace and key-order are tolerated.

use mirror::mcp;
use serde_json::Value;

fn read_fixture(name: &str) -> String {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let p = format!("{}/tests/mcp_fixtures/{}", manifest, name);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read fixture {}: {}", p, e))
}

fn assert_json_eq(got: &str, expected: &str) {
    let g: Value = serde_json::from_str(got)
        .unwrap_or_else(|e| panic!("got is not valid JSON: {}\n{}", e, got));
    let e: Value = serde_json::from_str(expected)
        .unwrap_or_else(|e| panic!("expected fixture not valid JSON: {}", e));
    assert_eq!(
        g, e,
        "semantic JSON mismatch\ngot:      {}\nexpected: {}",
        got, expected
    );
}

#[test]
fn initialize_handshake_matches_bash_fixture() {
    let req = read_fixture("initialize.req.json");
    let expected = read_fixture("initialize.resp.json");
    let resp = mcp::handle_request(req.trim()).expect("initialize must respond");
    assert_json_eq(&resp, expected.trim_end());
}

#[test]
fn tools_list_matches_bash_fixture() {
    let req = read_fixture("tools_list.req.json");
    let expected = read_fixture("tools_list.resp.json");
    let resp = mcp::handle_request(req.trim()).expect("tools/list must respond");
    assert_json_eq(&resp, expected.trim_end());
}

#[test]
fn notifications_initialized_emits_no_response() {
    let req = read_fixture("notifications_initialized.req.json");
    let resp = mcp::handle_request(req.trim());
    assert!(
        resp.is_none(),
        "notifications/initialized must not emit a response (got: {:?})",
        resp
    );
}

#[test]
fn unknown_method_silent_drop() {
    // Bash wrapper's case block has no default arm; unknown methods
    // are silently dropped. Lift preserves the behavior so the fixture
    // diff catches drift.
    let req = r#"{"jsonrpc":"2.0","id":99,"method":"made_up","params":{}}"#;
    assert!(mcp::handle_request(req).is_none());
}

#[test]
fn three_tools_advertised() {
    let req = read_fixture("tools_list.req.json");
    let resp = mcp::handle_request(req.trim()).expect("tools/list must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    let tools = v["result"]["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 3);
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(
        names,
        vec!["mirror_compile", "mirror_craft", "mirror_kintsugi"]
    );
}

/// Tick 3 (2026-06-18): the @io error contract.
///
/// Per [[architecture-error-as-tomm-probe.md]]: substrate errors at
/// the @io boundary are structured signals, not opaque text. The MCP
/// `tools/call` response carries `isError: true` when the underlying
/// `kintsugi_main` exit_code is non-zero (kintsugi REJECT, compile
/// error, etc.); agent clients then branch programmatically rather
/// than scraping stderr text.
#[test]
fn tools_call_unknown_tool_sets_is_error() {
    let req = r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"nonexistent_tool","arguments":{}}}"#;
    let resp = mcp::handle_request(req).expect("tools/call must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    assert_eq!(
        v["result"]["isError"], serde_json::Value::Bool(true),
        "unknown tool must surface as isError: true (got: {})", resp
    );
    // Text payload still present for the agent to read.
    let text = v["result"]["content"][0]["text"].as_str().expect("text payload");
    assert!(text.contains("unknown tool"), "text explains the error: {}", text);
}
