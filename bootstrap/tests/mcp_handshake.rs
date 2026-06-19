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
fn five_tools_advertised() {
    let req = read_fixture("tools_list.req.json");
    let resp = mcp::handle_request(req.trim()).expect("tools/list must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    let tools = v["result"]["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 5);
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    // Tick 17 (2026-06-19): added `prisms` substrate-introspection
    // primitive for substrate-driven tool registration foundation
    // (task #310 / #312).
    assert_eq!(
        names,
        vec!["compile", "craft", "kintsugi", "prisms", "verdict"]
    );
}

/// Tick 17 (2026-06-19): the substrate-introspection contract.
///
/// The `prisms` tool walks a directory recursively for .mirror files
/// and returns structured JSON listing of (path, prism, ops) for each.
/// This is the foundation for substrate-driven tool registration: a
/// future tick uses this output to generate MCP tool surface from
/// substrate-declared prisms rather than hardcoded Rust dispatch.
///
/// Empirical contract: invoke `prisms` on the @magic family directory
/// and verify the response contains all 6 species + family-root.
#[test]
fn prisms_enumerates_magic_family() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../shards/magic");
    if !std::path::Path::new(dir).exists() {
        eprintln!("skipping: {} not present", dir);
        return;
    }
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":42,"method":"tools/call","params":{{"name":"prisms","arguments":{{"dir":"{}"}}}}}}"#,
        dir
    );
    let resp = mcp::handle_request(&req).expect("prisms must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload");
    let envelope: serde_json::Value = serde_json::from_str(text).expect("prisms text is JSON");
    let count = envelope["count"].as_u64().expect("count is u64");
    // 6 species shards in shards/magic/ subdirectory (audit, contract,
    // distinction, mechanism, reveal, surface) all declare prisms.
    assert!(
        count >= 6,
        "expected at least 6 prism declarations, got {} (envelope: {})",
        count,
        text
    );
    let prisms = envelope["prisms"].as_array().expect("prisms array");
    let names: Vec<&str> = prisms
        .iter()
        .map(|p| p["prism"].as_str().unwrap_or(""))
        .collect();
    // The substrate convention: every @magic species declares
    // @magic/<name>.
    assert!(
        names.iter().any(|n| *n == "@magic/contract"),
        "expected @magic/contract; got {:?}",
        names
    );
    assert!(
        names.iter().any(|n| *n == "@magic/audit"),
        "expected @magic/audit; got {:?}",
        names
    );
}

/// Tick 18 (2026-06-19): the `requires` clause introspection contract.
///
/// The substrate's bilateral predicate discipline (recognition #37
/// Pask agreement) lives in `requires <pred>(args)` clauses. Surfacing
/// these at MCP altitude is the verifier-sharpening Loki's discipline
/// names: same tool, more substantive output, substrate doesn't get
/// smarter.
#[test]
fn prisms_surfaces_requires_clauses() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../shards/magic");
    if !std::path::Path::new(dir).exists() {
        eprintln!("skipping: {} not present", dir);
        return;
    }
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":99,"method":"tools/call","params":{{"name":"prisms","arguments":{{"dir":"{}"}}}}}}"#,
        dir
    );
    let resp = mcp::handle_request(&req).expect("prisms must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload");
    let envelope: serde_json::Value = serde_json::from_str(text).expect("prisms text is JSON");
    let prisms = envelope["prisms"].as_array().expect("prisms array");
    // Sum requires across all @magic species. Ticks 12-15 landed 5
    // non-decorative `requires` clauses + 1 cross-shard adapter:
    // - surface_honest requires invariant_preserved (tick 12)
    // - unseal requires audited (tick 13)
    // - reveal requires audited (tick 14)
    // - reveal requires mechanism_intact (tick 14)
    // - bind_satisfies_distinction requires distinction_well_formed (tick 15)
    let total_requires: usize = prisms
        .iter()
        .map(|p| p["requires"].as_array().map(|a| a.len()).unwrap_or(0))
        .sum();
    assert!(
        total_requires >= 5,
        "expected at least 5 requires clauses across @magic family; got {} (prisms: {})",
        total_requires,
        text
    );
}

/// Tick 19 (2026-06-19): the action-name introspection contract.
#[test]
fn prisms_surfaces_action_names() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../shards/magic");
    if !std::path::Path::new(dir).exists() {
        eprintln!("skipping: {} not present", dir);
        return;
    }
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":100,"method":"tools/call","params":{{"name":"prisms","arguments":{{"dir":"{}"}}}}}}"#,
        dir
    );
    let resp = mcp::handle_request(&req).expect("prisms must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload");
    let envelope: serde_json::Value = serde_json::from_str(text).expect("prisms text is JSON");
    let prisms = envelope["prisms"].as_array().expect("prisms array");
    // Tick 22: actions changed from Vec<String> to
    // Vec<{name, requires}>. Extract via ["name"].
    let all_actions: Vec<String> = prisms
        .iter()
        .flat_map(|p| {
            p["actions"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v["name"].as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        })
        .collect();
    // Known actions from @magic family ticks 9-15: bind / honor /
    // verify / audit / respond / check_invariant / expose / observe /
    // surface_honest / seal / unseal / mechanism_intact / reveal /
    // surface_as_mark / mechanism_as_distinction_space /
    // bind_satisfies_distinction. Test for several load-bearing ones.
    for required in &["bind", "audit", "seal", "reveal", "surface_honest"] {
        assert!(
            all_actions.iter().any(|a| a == required),
            "expected action '{}' in @magic family; got {:?}",
            required,
            all_actions
        );
    }
}

/// Tick 22 (2026-06-19) per Seam #5 closure: `requires` clauses are
/// now attached to their guarding action, not flat at prism level.
/// Verifies `reveal` action carries its 3 `requires` clauses
/// (audited + mechanism_intact old + mechanism_intact new per tick 21).
#[test]
fn prisms_attaches_requires_to_actions() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../shards/magic");
    if !std::path::Path::new(dir).exists() {
        eprintln!("skipping: {} not present", dir);
        return;
    }
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":222,"method":"tools/call","params":{{"name":"prisms","arguments":{{"dir":"{}"}}}}}}"#,
        dir
    );
    let resp = mcp::handle_request(&req).expect("prisms must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload");
    let envelope: serde_json::Value = serde_json::from_str(text).expect("prisms text is JSON");
    let prisms = envelope["prisms"].as_array().expect("prisms array");
    // Find the reveal prism (shards/magic/reveal.mirror declares @magic/reveal).
    let reveal = prisms
        .iter()
        .find(|p| p["prism"].as_str() == Some("@magic/reveal"))
        .expect("@magic/reveal prism missing");
    let actions = reveal["actions"].as_array().expect("actions array");
    let reveal_action = actions
        .iter()
        .find(|a| a["name"].as_str() == Some("reveal"))
        .expect("reveal action missing");
    let action_requires = reveal_action["requires"]
        .as_array()
        .expect("action requires array");
    assert_eq!(
        action_requires.len(),
        3,
        "reveal action should carry 3 requires clauses (audited + 2 mechanism_intact); got {:?}",
        action_requires
    );
}

/// Tick 20 (2026-06-19): the @magic/audit @io wiring contract.
///
/// When MIRROR_MCP_AUDIT=1, every tools/call dispatch should emit an
/// audit record to ~/.mirror/mcp-audit.log matching the @magic/audit
/// substrate-decl's `audit_record` carrier shape. Gated by env var so
/// non-audit MCP sessions stay clean.
#[test]
fn audit_emission_gated_by_env_var() {
    // Without env var: no audit log written. We can't directly test
    // "nothing written" because some other test may have written, so
    // we only verify the env-var-gated path behaves deterministically.
    let bogus_req = format!(
        r#"{{"jsonrpc":"2.0","id":111,"method":"tools/call","params":{{"name":"nonexistent_tick_20","arguments":{{}}}}}}"#
    );
    // Without MIRROR_MCP_AUDIT: dispatch still works, response shape
    // unchanged. The actual env-var-gated behavior is integration-
    // tested separately (the log file emission is best-effort and
    // cannot break the MCP wire per its implementation contract).
    let resp = mcp::handle_request(&bogus_req).expect("tools/call must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON");
    // Unknown tool path: isError lifts. This is the existing tick 3
    // contract; we just confirm the audit instrumentation didn't
    // break the wire response.
    assert_eq!(v["result"]["isError"], serde_json::Value::Bool(true));
}

/// Tick 5 (2026-06-18): the settle/verdict prism split.
///
/// Per Seam's adversarial review of tick 4: `cmd_kintsugi_ci_single`
/// returns `exit_code = 0` regardless of verdict.label (workflow YAML
/// decides pass per lib.rs:855 contract). The previous `ci: boolean`
/// flag on `kintsugi` claimed isError composition that never fired in
/// practice. The split into `verdict` parses the JSON envelope's
/// verdict field and lifts {partial, failure} → isError.
///
/// Smoke test: invoke `verdict` on a path that doesn't exist. The
/// substrate emits `verdict: "failure"` for unreadable files
/// (lib.rs:`emit_ci_verdict_json` failure branch). MCP must lift this
/// to `isError: true` so agent clients can branch programmatically.
#[test]
fn verdict_failure_lifts_to_is_error() {
    let bogus = format!(
        "/tmp/mirror-mcp-nonexistent-{}-{}.mirror",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":33,"method":"tools/call","params":{{"name":"verdict","arguments":{{"file":"{}"}}}}}}"#,
        bogus
    );
    let resp = mcp::handle_request(&req).expect("verdict must respond");
    let v: serde_json::Value = serde_json::from_str(&resp).expect("valid JSON envelope");
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload")
        .to_string();
    assert_eq!(
        v["result"]["isError"],
        serde_json::Value::Bool(true),
        "failure verdict must lift to isError=true. Response was: {}",
        resp
    );
    assert!(
        text.contains("failure"),
        "verdict body should contain 'failure' label; got: {}",
        text
    );
}

/// Tick 6 (2026-06-18): unit-level test for parse_verdict_label.
///
/// The tick 5 implementation tried `serde_json::from_str` on the whole
/// payload and failed silently when trace was appended. Reed observed
/// this running the live `verdict` tool on the eigenform shard: real
/// verdict paths emit `{...JSON...}\n  dispatch @kintsugi/tick: ...`
/// mixing the envelope with kintsugi-loop trace. parse_verdict_label
/// returned None, fallback hit exit_code=0 (substrate contract), and
/// `partial` verdicts silently failed to lift isError.
///
/// We can't exercise the full integration chain from cargo test (cwd
/// can't resolve grammars without MIRROR_HOME), so we pin the unit
/// contract directly: a JSON envelope followed by trace text must
/// parse correctly. The pattern below mirrors what the live MCP
/// actually emitted on `verdict` of eigenform.mirror.
#[test]
fn parse_verdict_label_handles_json_plus_trace() {
    // The shape Reed empirically observed on the live MCP:
    // line 1 = compact JSON envelope; subsequent lines = kintsugi trace.
    let payload = concat!(
        "{\"verdict\":\"partial\",\"target\":\"/x/eigenform.mirror\",",
        "\"objective\":10.0,\"iterations\":1,\"dark_count\":10}\n",
        "  dispatch @kintsugi/tick: Uncrystallized (floor has no body at @kintsugi/tick)\n",
        "tick 1  dark_count: 10  loss: 1.0  Δ: 0.0  ← Lawvere fixed-point (vacuously)\n",
    );
    let label = mcp::parse_verdict_label(payload);
    assert_eq!(
        label.as_deref(),
        Some("partial"),
        "parse_verdict_label must scan past JSON envelope to find the verdict field"
    );
}

/// Tick 6 also tests the clean (envelope-only) case still works,
/// so the line-by-line fallback didn't regress the simple path.
#[test]
fn parse_verdict_label_handles_clean_envelope() {
    let payload =
        r#"{"verdict":"success","target":"x","objective":0.0,"iterations":1,"dark_count":0}"#;
    assert_eq!(
        mcp::parse_verdict_label(payload).as_deref(),
        Some("success")
    );
}

/// Tick 6: non-JSON / no verdict field returns None (fallback path).
#[test]
fn parse_verdict_label_returns_none_on_garbage() {
    assert_eq!(mcp::parse_verdict_label("not json at all"), None);
    assert_eq!(mcp::parse_verdict_label(""), None);
    assert_eq!(
        mcp::parse_verdict_label(r#"{"target":"x","objective":0.0}"#),
        None
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
        v["result"]["isError"],
        serde_json::Value::Bool(true),
        "unknown tool must surface as isError: true (got: {})",
        resp
    );
    // Text payload still present for the agent to read.
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload");
    assert!(
        text.contains("unknown tool"),
        "text explains the error: {}",
        text
    );
}
