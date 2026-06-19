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
//! ## Tools advertised
//!
//! Tool names omit the `mirror_` prefix because the MCP server itself
//! is `mirror` — the server name IS the namespace per Alex's
//! 2026-06-18 correction. Agent-side invocation is
//! `mcp__mirror__compile`, not `mcp__mirror__mirror_compile`.
//!
//! - `compile`  — `focus`:   tokenize one `.mirror` file.
//! - `craft`    — `split`:   converge a target to lambda_0.
//! - `kintsugi` — `refract`: settle a `.mirror` file (mutates).
//! - `verdict`  — `focus`:   render a structured verdict envelope
//!                          (read-only inspection).
//!
//! `kintsugi` and `verdict` are TWO prisms, not one tool with a flag. The substrate distinction is: `kintsugi` writes
//! canonical output (effect at @io); `verdict` returns structured
//! observation (pure focus). Per Seam's adversarial review of tick 4,
//! collapsing them under a `ci: boolean` flag hid the substrate-pull
//! partition at the MCP wire altitude. The split closes the verdict
//! exit-code contract: `mirror_verdict` parses `verdict.label` and
//! lifts `label ∈ {partial, failure}` to MCP `isError: true`, since
//! the underlying `--ci` invocation always exits 0 by design (the
//! workflow YAML, not the binary, decides what verdict counts as pass
//! per cmd_kintsugi_ci_single's contract at lib.rs:855).
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
                "name": "compile",
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
                "name": "craft",
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
                "name": "kintsugi",
                "description": "refract: settle a .mirror file, write canonical. --liquid writes inferred properties below ---. --shatter N recursively settles N levels deep. Mutates the file. For read-only verdict inspection use `verdict`.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "file":    { "type": "string", "description": "Path to .mirror file" },
                        "liquid":  { "type": "boolean", "description": "If true, pass --liquid (project inferred properties below ---)" },
                        "shatter": { "type": "integer", "description": "If set, pass --shatter N (recursive settle N levels)" }
                    },
                    "required": ["file"]
                }
            },
            {
                "name": "prisms",
                "description": "focus: enumerate prism declarations across a directory of .mirror files. Returns structured JSON listing of (path, prism_name, ops, requires, actions). Substrate-introspection primitive for substrate-driven tool registration (task #312 / lens-server gen_prism MVP per task #310). Tick 18 added `requires` clauses (bilateral predicates); tick 19 added `actions` (action names per prism).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "dir": { "type": "string", "description": "Path to directory containing .mirror files (recursive walk)" }
                    },
                    "required": ["dir"]
                }
            },
            {
                "name": "verdict",
                "description": "focus: render a structured PASS/REJECT verdict envelope for a .mirror file or directory (corpus mode). Read-only — does NOT modify the file. Returns the substrate's JSON verdict envelope (fields: verdict, target, objective, iterations, dark_count). isError lifts to true when verdict.label ∈ {partial, failure}; success returns isError absent.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "file":    { "type": "string", "description": "Path to .mirror file or directory (corpus mode)" },
                        "shatter": { "type": "integer", "description": "If set, pass --shatter N (recursive verdict N levels)" }
                    },
                    "required": ["file"]
                }
            }
        ]
    })
}

/// Emit a substrate-typed audit record for a tool/call dispatch.
///
/// Tick 20 (2026-06-19): substrate-pull USE of @magic/audit's
/// `audit_record` carrier (shards/magic/audit.mirror) at the actual
/// @io boundary. Realizes alignment-as-boundary-mathematics (#57)
/// operationally; closes the structural identity that Mara hedged at
/// tick 11 ("supervisor composes-with @magic/audit at supervision
/// altitude" — the audit record is the substrate-decl's data shape).
///
/// Record shape (matches audit_record carrier):
/// ```json
/// {
///   "contract":  { "tool": <name>, "args": <args> },
///   "verdict":   "success" | "failure",
///   "witness":   { "tool": <name>, "args": <args> },
///   "timestamp": <UTC seconds>
/// }
/// ```
///
/// Appended to `~/.mirror/mcp-audit.log` one record per line
/// (JSON-lines). Best-effort: failures to write are silently dropped
/// so audit failure cannot break the MCP wire.
fn emit_audit_record(tool: &str, args: &Value, is_error: bool) {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return,
    };
    let dir = format!("{}/.mirror", home);
    let _ = std::fs::create_dir_all(&dir);
    let log_path = format!("{}/mcp-audit.log", dir);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let record = json!({
        "contract":  { "tool": tool, "args": args },
        "verdict":   if is_error { "failure" } else { "success" },
        "witness":   { "tool": tool, "args": args },
        "timestamp": timestamp,
    });
    if let Ok(line) = serde_json::to_string(&record) {
        // Best-effort append; audit failure must NOT break MCP wire.
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            use std::io::Write as _;
            let _ = writeln!(f, "{}", line);
        }
    }
}

/// Walk `dir` recursively for `.mirror` files; for each, extract the
/// prism declaration line of shape `prism @path { ... }` plus the
/// five-op signature lines. Return structured JSON.
///
/// Tick 17 (substrate-introspection primitive). The smallest viable
/// substrate-driven tool registration foundation per task #310/#312.
/// Uses simple text matching — the prism declaration form is regular
/// enough that full grammar parse isn't required at this altitude.
fn list_prisms_in_dir(dir: &str) -> String {
    let mut prisms: Vec<Value> = Vec::new();
    walk_for_prisms(std::path::Path::new(dir), &mut prisms);
    let envelope = json!({ "dir": dir, "count": prisms.len(), "prisms": prisms });
    serde_json::to_string(&envelope).expect("prisms envelope is serializable")
}

/// Recursively walk `path` for `.mirror` files; extract each file's
/// prism declaration into `acc`.
fn walk_for_prisms(path: &std::path::Path, acc: &mut Vec<Value>) {
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk_for_prisms(&p, acc);
        } else if p.extension().and_then(|e| e.to_str()) == Some("mirror") {
            if let Some(prism) = extract_prism_declaration(&p) {
                acc.push(prism);
            }
        }
    }
}

/// Extract the prism declaration from one `.mirror` file. Returns
/// `Some(json)` with shape `{ path, prism, ops }` on success.
///
/// Substrate convention: prism declarations look like
/// ```
/// prism @path/to/prism {
///   focus name
///   project name
///   split name
///   shift name
///   settle name
/// }
/// ```
fn extract_prism_declaration(path: &std::path::Path) -> Option<Value> {
    let content = std::fs::read_to_string(path).ok()?;
    let mut prism_name: Option<String> = None;
    let mut ops: Vec<String> = Vec::new();
    let mut requires_clauses: Vec<String> = Vec::new();
    // Tick 19 (2026-06-19): also extract action names. Same
    // verifier-sharpening discipline as tick 18; substrate-driven
    // dispatch foundation now has the complete picture per prism:
    // (ops, requires, actions). Future ticks consume this envelope to
    // generate MCP tool surface from substrate-declared prisms.
    let mut actions: Vec<String> = Vec::new();
    let mut in_prism_block = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("prism @") {
            let name = rest.split('{').next().unwrap_or(rest).trim();
            prism_name = Some(format!("@{}", name));
            in_prism_block = true;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("requires ") {
            let clause = rest.trim_end_matches(|c: char| c.is_whitespace() || c == '{').trim();
            if !clause.is_empty() {
                requires_clauses.push(clause.to_string());
            }
            continue;
        }
        if in_prism_block {
            if trimmed.starts_with('}') {
                in_prism_block = false;
                continue;
            }
            for op in &["focus", "project", "split", "shift", "settle"] {
                if let Some(rest) = trimmed.strip_prefix(*op) {
                    let target = rest.trim();
                    if !target.is_empty() {
                        ops.push(format!("{} {}", op, target));
                    }
                }
            }
            continue;
        }
        // Outside the prism block, look for action signatures.
        // Substrate convention: actions are
        //   snake_case_name(params) -> return_type
        // Tick 21 (Seam retrospective gap #6): tightened from
        // "snake_case + (" to "snake_case + (...) ->" to avoid false
        // positives on type constructor calls and inline expressions.
        if let Some(open) = trimmed.find('(') {
            let head = &trimmed[..open];
            if !head.is_empty()
                && head.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            {
                let after_open = &trimmed[open..];
                if let Some(close) = after_open.find(')') {
                    let after_close = after_open[close + 1..].trim_start();
                    if after_close.starts_with("->") {
                        actions.push(head.to_string());
                    }
                }
            }
        }
    }
    let name = prism_name?;
    Some(json!({
        "path": path.to_string_lossy(),
        "prism": name,
        "ops": ops,
        "requires": requires_clauses,
        "actions": actions,
    }))
}

/// Parse a `mirror kintsugi --ci --out @data/json` stdout payload and
/// extract the `verdict.label` field. Returns `Some(label)` when the
/// payload parses as JSON with a string `verdict` field; `None` when
/// the payload is empty, not JSON, or lacks the field.
///
/// Both single-file (`CiVerdict`) and corpus (`CorpusVerdict`) envelopes
/// share the top-level `verdict` field per the substrate's typed
/// records at `boot/std/kintsugi.mirror`.
pub fn parse_verdict_label(payload: &str) -> Option<String> {
    // Tick 6 (2026-06-18): the substrate's verdict path emits the JSON
    // envelope as a single line followed by kintsugi-loop trace output
    // (stderr concatenated via bash 2>&1 semantic in run_mirror). The
    // tick 5 implementation tried `from_str` on the whole payload and
    // failed silently when trace was appended. Robust strategy: try
    // whole first (clean case: failure verdict, no kintsugi loop runs),
    // then scan line-by-line (mixed case: real verdict + trace).
    let try_full: Option<String> = serde_json::from_str::<Value>(payload)
        .ok()
        .and_then(|v| v.get("verdict").and_then(|s| s.as_str()).map(|s| s.to_string()));
    if try_full.is_some() {
        return try_full;
    }
    for line in payload.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.starts_with('{') {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            if let Some(label) = v.get("verdict").and_then(|s| s.as_str()) {
                return Some(label.to_string());
            }
        }
    }
    None
}

/// Run mirror's library entry with `args`, returning the combined
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

    // Tick 5 panic guard (Seam finding #2): a panic in kintsugi_main
    // would unwind through serve_loop and kill the MCP server. Catch
    // and convert to an error-shape response so the server survives
    // and the agent gets a wire-level isError signal. The shape
    // matches `error: <panic message>` on stderr with exit_code=2 —
    // distinct from a substrate-clean failure (exit_code=1) so callers
    // can distinguish substrate-internal vs panic-in-server.
    let output = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::kintsugi_main(&argv)
    })) {
        Ok(o) => o,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "<non-string panic payload>".to_string()
            };
            return (
                format!("mcp panic in kintsugi dispatch: {}\n", msg),
                2,
            );
        }
    };

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
        "compile" => {
            let file = s("file").unwrap_or_default();
            run_mirror(&["compile", &file])
        }
        "craft" => {
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
        "kintsugi" => {
            // Settle prism: mutates the file. Read-only verdict
            // inspection lives in `verdict` below (tick 5 split).
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
        "prisms" => {
            // Substrate-introspection primitive (tick 17). Walks the
            // given directory recursively for .mirror files; reads
            // each file's prism declaration via simple text matching;
            // returns structured JSON list. Does NOT use full mirror
            // grammar parse — the prism declaration form
            // `prism @path { focus N project N split N shift N settle N }`
            // is regular enough to extract via regex match without
            // dragging the grammar load path through MCP dispatch.
            //
            // Per task #310 (lens-server gen_prism MVP) and task #312
            // (MCP substrate-driven tool registration): this is the
            // foundation for the L of MCP+LSP. Future ticks build on
            // this output to generate MCP tool surface from substrate-
            // declared prisms rather than hardcoded Rust dispatch.
            let dir = s("file").or_else(|| s("dir")).unwrap_or_default();
            let json = list_prisms_in_dir(&dir);
            return (json, false);
        }
        "verdict" => {
            // Verdict prism: read-only observation. Per Seam tick 4
            // review: the substrate's `cmd_kintsugi_ci_*` always exits
            // 0 (the workflow YAML decides pass; lib.rs:855 contract);
            // so isError lift via exit_code alone is insufficient. We
            // parse verdict.label from the JSON envelope and lift
            // {partial, failure} → isError directly.
            let file = s("file").unwrap_or_default();
            let mut argv: Vec<String> = vec![
                "kintsugi".into(),
                file,
                "--ci".into(),
                "--out".into(),
                "@data/json".into(),
            ];
            if let Some(n) = i("shatter") {
                argv.push("--shatter".into());
                argv.push(n.to_string());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            let (text, exit_code) = run_mirror(&refs);
            let is_error = match parse_verdict_label(text.trim()) {
                Some(label) => label != "success",
                // No parseable verdict + non-zero exit → substrate-internal
                // failure (e.g. panic guard). Surface as error.
                None => exit_code != 0,
            };
            return (text, is_error);
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
            // Tick 20 (2026-06-19): substrate-pull USE of @magic/audit
            // at @io boundary. When MIRROR_MCP_AUDIT=1, emit an audit
            // record (matching the audit_record carrier from
            // shards/magic/audit.mirror: contract / verdict / witness /
            // timestamp) to ~/.mirror/mcp-audit.log. This realizes
            // alignment-as-boundary-mathematics (#57) operationally
            // and demonstrates @magic/audit's substrate-decl in use.
            //
            // The audit_record's `contract` field holds the tool +
            // arguments (the substrate-pull-correct "what was bound");
            // `verdict` carries success/failure; `witness` is the tool
            // call payload; `timestamp` is the UTC instant.
            //
            // Gated by env var so non-audit MCP sessions stay clean.
            if std::env::var("MIRROR_MCP_AUDIT").is_ok() {
                emit_audit_record(&tool, &args, is_error);
            }
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
