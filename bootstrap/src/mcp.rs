//! MCP server — the JSON-RPC tool dispatch surface for the agent lens.
//!
//! Lifted from `bin/mirror-mcp` (145-line bash wrapper) to Rust on
//! 2026-06-18 as tick 1 of the mirror-mcp+lsp self-improving loop
//! (Seam pre-loop review at
//! `docs/specs/seam-pre-loop-mirror-mcp-lsp-review-2026-06-18.md`).
//!
//! ## Substrate frame
//!
//! Per `shards/mirror/lens/mcp.mirror` and `docs/specs/the-convergence.md`
//! §2.1, the MCP lens is the JSON notation of the same five-operation
//! algebra the CLI renders as argv. This module IS the Rust body of
//! the `tool(name, args: ref) -> mcp { \ }` and `dispatch(call: ref) -> mcp { \ }`
//! actions whose substrate declarations are family-header-only.
//!
//! ## Behavior parity
//!
//! This module preserves the bash wrapper's behavior exactly. Each
//! captured fixture in `bootstrap/tests/mcp_fixtures/` is byte-equal
//! to the response this module produces. The lift dissolves the bash
//! into Rust; subsequent ticks land tool additions cleanly in Rust
//! +`.mirror` rather than in shell.
//!
//! ## Tools advertised this tick
//!
//! Three tools, identical to the bash wrapper:
//!
//! - `mirror_compile` — `focus`: tokenize one `.mirror` file.
//! - `mirror_craft`   — `split`: converge a target to lambda_0.
//! - `mirror_kintsugi`— `refract`: settle a `.mirror` file.
//!
//! New tools (per `the-convergence.md` §2.1's twelve-row composition
//! table) land in subsequent ticks by adding a row to `tools_list_value()`
//! and a match arm in `dispatch_tool_call()`.
//!
//! ## Wire shape
//!
//! Standard MCP/stdio JSON-RPC: one JSON object per line on stdin;
//! one JSON object per line on stdout (or no line for notifications).
//! `serve_loop` reads stdin, dispatches each request through
//! `handle_request`, and writes the response.
//!
//! Per Taut's profiling discipline (#286): the implementation is
//! `serde_json::Value`-based rather than typed structs because the
//! protocol surface this tick is small and the schema is published
//! in the captured fixtures. Typed structs land when the surface
//! grows past the point where the fixture diff catches drift
//! cheaply.

use std::io::{BufRead, BufReader, Write};

use serde_json::{json, Value};

/// Build the response value for the MCP `initialize` request.
///
/// Preserves the bash wrapper's exact response shape (server name
/// `"mirror"`, version `"0.1.0"`, protocol version `"2024-11-05"`,
/// capabilities advertising `tools.listChanged: false`).
fn initialize_result() -> Value {
    json!({
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo":   { "name": "mirror", "version": "0.1.0" },
        "protocolVersion": "2024-11-05"
    })
}

/// Build the response value for the MCP `tools/list` request.
///
/// Three tools, identical to the bash wrapper's hand-rolled string.
/// Per Seam's review (§2.5.A): this is the surface the loop's later
/// ticks extend. Each new row of `the-convergence.md` §2.1's table
/// becomes a new entry here plus a match arm in `dispatch_tool_call`.
fn tools_list_result() -> Value {
    json!({
        "tools": [
            {
                "name": "mirror_compile",
                "description": "focus: tokenize one .mirror file through grammar lens. Returns SHA-256 hash on success.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "file": { "type": "string", "description": "Path to .mirror file" }
                    },
                    "required": ["file"]
                }
            },
            {
                "name": "mirror_craft",
                "description": "split: converge a target to lambda_0. --target emits code (binary|rust|gleam). --reflect verifies properties without emission.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "target":      { "type": "string", "description": "Build target directory (e.g. boot)" },
                        "emit_target": { "type": "string", "description": "Optional --target value: binary, rust, or gleam", "enum": ["binary", "rust", "gleam"] },
                        "reflect":     { "type": "boolean", "description": "If true, pass --reflect (verify only, no emission)" }
                    },
                    "required": ["target"]
                }
            },
            {
                "name": "mirror_kintsugi",
                "description": "refract: settle a .mirror file, write canonical. --liquid writes inferred properties below ---. --shatter N recursively settles N levels deep.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "file":    { "type": "string", "description": "Path to .mirror file" },
                        "liquid":  { "type": "boolean", "description": "If true, pass --liquid (project inferred properties below ---)" },
                        "shatter": { "type": "integer", "description": "If set, pass --shatter N (recursive settle N levels)" }
                    },
                    "required": ["file"]
                }
            }
        ]
    })
}

/// Lookup helper: pull the mirror binary path the dispatcher should invoke.
///
/// Mirrors the bash wrapper's `MIRROR="${MIRROR_BIN:-$HOME/.local/bin/mirror}"`
/// resolution. The `MIRROR_BIN` env var lets the integration test point
/// the dispatcher at a non-default binary (or a no-op stub).
#[allow(dead_code)]
fn mirror_binary_path() -> String {
    if let Ok(p) = std::env::var("MIRROR_BIN") {
        return p;
    }
    if let Ok(home) = std::env::var("HOME") {
        return format!("{}/.local/bin/mirror", home);
    }
    // No HOME — unusual but fall back to PATH-relative.
    "mirror".to_string()
}

/// Run the configured mirror binary with `args`, returning the combined
/// stdout+stderr UTF-8 string AND the exit code.
///
/// Tick 3 (loop 2026-06-18): exit_code is now propagated so handle_request
/// can lift @io substrate failure into the MCP wire-level `isError` flag.
/// Per [[architecture-error-as-tomm-probe.md]]: errors at the @io
/// boundary IS a structured signal, not opaque text. The substrate's
/// `mirror kintsugi --ci` REJECT verdict exits non-zero; the MCP wire
/// must surface that as structured failure for the agent caller.
fn run_mirror(args: &[&str]) -> (String, i32) {
    // kintsugi_main expects full argv with program name at args[0].
    let mut argv: Vec<String> = vec!["mirror".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));

    let output = crate::kintsugi_main(&argv);

    // Preserve bash wrapper's `2>&1`: stderr concatenated after stdout.
    let out = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);
    let text = if !err.is_empty() {
        if out.is_empty() {
            err.into_owned()
        } else {
            format!("{}{}", out, err)
        }
    } else {
        out.into_owned()
    };
    (text, output.exit_code)
}

/// Dispatch a `tools/call` request to the appropriate mirror invocation.
///
/// Mirrors the bash wrapper's `case "$tool" in ... esac` block:
///
/// - `mirror_compile` → `mirror compile <file>`
/// - `mirror_craft`   → `mirror craft <target> [--target <emit>] [--reflect]`
/// - `mirror_kintsugi`→ `mirror kintsugi <file> [--liquid] [--shatter N]`
///
/// Returns `(text, is_error)`. `is_error` lifts to MCP's `isError` flag
/// in the `tools/call` response so clients can programmatically
/// distinguish substrate failure (kintsugi REJECT, compile error) from
/// success without scraping stderr text.
fn dispatch_tool_call(tool: &str, args: &Value) -> (String, bool) {
    let s =
        |k: &str| -> Option<String> { args.get(k).and_then(|v| v.as_str()).map(|s| s.to_string()) };
    let b = |k: &str| -> bool { args.get(k).and_then(|v| v.as_bool()).unwrap_or(false) };
    let i = |k: &str| -> Option<i64> { args.get(k).and_then(|v| v.as_i64()) };

    let (text, exit_code) = match tool {
        "mirror_compile" => {
            let file = s("file").unwrap_or_default();
            run_mirror(&["compile", &file])
        }
        "mirror_craft" => {
            let target = s("target").unwrap_or_default();
            let mut argv: Vec<String> = vec!["craft".into(), target];
            if let Some(emit) = s("emit_target") {
                argv.push("--target".into());
                argv.push(emit);
            }
            if b("reflect") {
                argv.push("--reflect".into());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs)
        }
        "mirror_kintsugi" => {
            let file = s("file").unwrap_or_default();
            let mut argv: Vec<String> = vec!["kintsugi".into(), file];
            if b("liquid") {
                argv.push("--liquid".into());
            }
            if let Some(n) = i("shatter") {
                argv.push("--shatter".into());
                argv.push(n.to_string());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs)
        }
        other => return (format!("unknown tool: {}", other), true),
    };
    (text, exit_code != 0)
}

/// Whether the response for a given method should be serialized in
/// compact (single-line) form. We always emit compact JSON, matching
/// the bash wrapper's hand-rolled string responses.
///
/// Result is either `Some(line)` to write to stdout, or `None` when the
/// request is a notification (no response per JSON-RPC 2.0 §4.1.5).
pub fn handle_request(line: &str) -> Option<String> {
    let v: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => return None, // bash wrapper silently dropped unparseable lines.
    };
    let method = v.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let id = v.get("id").cloned();

    match method {
        "initialize" => {
            let resp = json!({
                "jsonrpc": "2.0",
                "id":      id.unwrap_or(Value::Null),
                "result":  initialize_result(),
            });
            Some(serde_json::to_string(&resp).expect("initialize is serializable"))
        }
        "notifications/initialized" => None,
        "tools/list" => {
            let resp = json!({
                "jsonrpc": "2.0",
                "id":      id.unwrap_or(Value::Null),
                "result":  tools_list_result(),
            });
            Some(serde_json::to_string(&resp).expect("tools_list is serializable"))
        }
        "tools/call" => {
            let params = v.get("params").cloned().unwrap_or(Value::Null);
            let tool = params
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string();
            let args = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let (text, is_error) = dispatch_tool_call(&tool, &args);
            // Per MCP spec: `isError: true` signals tool execution failure
            // (substrate REJECT / compile error / unknown tool) so agent
            // clients can branch programmatically rather than scraping text.
            let mut result_obj = json!({
                "content": [ { "type": "text", "text": text } ]
            });
            if is_error {
                result_obj["isError"] = Value::Bool(true);
            }
            let resp = json!({
                "jsonrpc": "2.0",
                "id":      id.unwrap_or(Value::Null),
                "result":  result_obj,
            });
            Some(serde_json::to_string(&resp).expect("tools_call is serializable"))
        }
        // The bash wrapper's `case` block has no default arm — unknown
        // methods are silently dropped. Preserve the behavior so the
        // fixture diff catches any drift.
        _ => None,
    }
}

/// Serve loop: read JSON-RPC lines from stdin, write responses to stdout.
///
/// Mirrors the bash wrapper's `while IFS= read -r line; do ... done`
/// loop. Exits 0 on EOF.
pub fn serve_loop() -> i32 {
    // The bash wrapper changed directory to MIRROR_HOME. Preserve that
    // for parity — some `mirror compile` invocations resolve grammar
    // paths relative to cwd.
    if let Ok(home) = std::env::var("MIRROR_HOME") {
        let _ = std::env::set_current_dir(&home);
    }

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    loop {
        line.clear();
        let n = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(_) => return 1,
        };
        if n == 0 {
            return 0; // EOF.
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(resp) = handle_request(trimmed) {
            let mut out = stdout.lock();
            // Newline-terminate; flush so the agent sees the frame.
            if writeln!(out, "{}", resp).is_err() {
                return 1;
            }
            if out.flush().is_err() {
                return 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse both sides, compare `Value`s. Tolerates key-order drift
    /// between the bash wrapper's hand-rolled order and serde_json's
    /// sorted-key serialization. Content drift is still caught.
    fn assert_json_eq(got: &str, expected: &str) {
        let g: Value = serde_json::from_str(got).expect("got valid JSON");
        let e: Value = serde_json::from_str(expected).expect("expected valid JSON");
        assert_eq!(g, e);
    }

    #[test]
    fn initialize_response_matches_bash_fixture() {
        let req = include_str!("../tests/mcp_fixtures/initialize.req.json");
        let expected = include_str!("../tests/mcp_fixtures/initialize.resp.json");
        let got = handle_request(req.trim()).expect("initialize must respond");
        assert_json_eq(&got, expected.trim_end());
    }

    #[test]
    fn tools_list_response_matches_bash_fixture() {
        let req = include_str!("../tests/mcp_fixtures/tools_list.req.json");
        let expected = include_str!("../tests/mcp_fixtures/tools_list.resp.json");
        let got = handle_request(req.trim()).expect("tools/list must respond");
        assert_json_eq(&got, expected.trim_end());
    }

    #[test]
    fn notifications_initialized_returns_no_response() {
        let req = include_str!("../tests/mcp_fixtures/notifications_initialized.req.json");
        let got = handle_request(req.trim());
        assert!(
            got.is_none(),
            "notifications/initialized must not emit a response line (got: {:?})",
            got
        );
    }

    #[test]
    fn unknown_method_returns_no_response() {
        // The bash wrapper's case block has no default; unknown methods
        // are silently dropped. Preserve this so the fixture diff catches drift.
        let got = handle_request(r#"{"jsonrpc":"2.0","id":99,"method":"nonexistent","params":{}}"#);
        assert!(got.is_none());
    }

    #[test]
    fn unparseable_line_returns_no_response() {
        // Bash wrapper's `grep` extraction silently produces empty
        // strings on non-matching input; the case block then misses.
        // Rust mirrors this by returning None on serde_json::Error.
        let got = handle_request("not valid json");
        assert!(got.is_none());
    }

    #[test]
    fn tools_list_advertises_three_tools() {
        let req = r#"{"jsonrpc":"2.0","id":42,"method":"tools/list","params":{}}"#;
        let resp_line = handle_request(req).expect("tools/list must respond");
        let resp: Value = serde_json::from_str(&resp_line).expect("valid JSON");
        let tools = resp["result"]["tools"]
            .as_array()
            .expect("tools is an array");
        assert_eq!(tools.len(), 3);
        let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(
            names,
            vec!["mirror_compile", "mirror_craft", "mirror_kintsugi"]
        );
    }
}
