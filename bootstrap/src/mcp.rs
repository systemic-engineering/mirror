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
//! Eight `mirror_`-prefixed tools per Mara iter-15 schema
//! reconciliation (2026-07-08): the Rust `tools_list_result` +
//! `dispatch_tool_call` are now byte-parity with the ground-truth
//! `bin/mirror-mcp` bash wrapper (post-Tick-3 rename + Tick 7 shatter
//! fold). Agent-side invocation is `mcp__mirror__mirror_compile` etc.
//!
//! - `mirror_compile`   — tokenize one `.mirror` file (SHA-256 hash).
//! - `mirror_craft`     — converge a target directory to lambda_0.
//! - `mirror_kintsugi`  — settle a `.mirror` file; ALWAYS `--ci --out
//!                        @data/json` per Tick 7 fold (`ffba2a7`);
//!                        returns typed verdict envelope.
//! - `mirror_init`      — mirror-native store bootstrap.
//! - `mirror_recall`    — inbound-trajectory dual of peer beam.
//! - `mirror_peer_beam` — beam through a peer's persistent-identity
//!                        context (Tick 3 rename `4f4a257`).
//! - `mirror_beam`      — anonymous inference primitive (top-level).
//! - `mirror_spawn`     — DEPRECATED alias for `mirror_peer_beam`
//!                        (two-tick discipline).
//!
//! `kintsugi` no longer splits into a separate `verdict` tool at MCP
//! altitude. Per Tick 7 shatter fold (`ffba2a7`) the wrapper always
//! routes kintsugi through `--ci --out @data/json`, so the substrate
//! is the linearizer and the wrapper is the transport-framer. This
//! module preserves the verdict-label lift: on the JSON envelope's
//! `verdict` field, `label ∈ {partial, failure}` lifts to MCP
//! `isError: true` (the underlying `--ci` invocation always exits 0
//! by design; per `cmd_kintsugi_ci_single`'s contract at lib.rs:855).
//! `parse_verdict_label` is preserved for that lift.
//!
//! The stale `prisms` + `verdict` tools (pre-Tick-3, no matching
//! cli-block) are removed as part of the reconciliation.
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

use crate::Ctx;

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
/// Eight `mirror_`-prefixed tools, byte-parity with `bin/mirror-mcp`
/// (Mara iter-15 schema reconciliation, 2026-07-08). Each new row of
/// mirror.spec's cli-block table becomes a new entry here plus a match
/// arm in `dispatch_tool_call`.
///
/// **Substrate frame**: the schema mirrors the bash wrapper verbatim
/// (descriptions, inputSchemas, `required` sets). The reflective form
/// (parse mirror.spec at runtime) is a heavier substrate-motion left
/// for a future tick — this landing discharges the
/// `tools_reflects_cli_block` bilateral predicate at
/// hardcoded-schema altitude.
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
                "description": "split: converge a target directory to lambda_0. --target-kind emits code (binary|rust|gleam). --reflect verifies properties without emission.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "target":      { "type": "string", "description": "Build target directory (e.g. boot)" },
                        "target_kind": { "type": "string", "description": "Emit backend (substrate-honest name; binary accepts both --target-kind and --target)", "enum": ["crystal", "binary", "rust", "gleam"] },
                        "reflect":     { "type": "boolean", "description": "If true, verify only (no emission)" }
                    },
                    "required": ["target"]
                }
            },
            {
                "name": "mirror_kintsugi",
                "description": "settle: kintsugi a .mirror file. --liquid writes inferred properties below ---. --shatter N seeds N cracks. `--ci` walks a corpus. Returns canonical source or typed verdict envelope.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "file":    { "type": "string", "description": "Path to .mirror file or directory" },
                        "liquid":  { "type": "boolean", "description": "Write inferred properties" },
                        "shatter": { "type": "integer", "description": "Seed N cracks" }
                    },
                    "required": ["file"]
                }
            },
            {
                "name": "mirror_init",
                "description": "init: mirror-native store bootstrap. Splinter + insert_persistent + set_ref(HEAD). Substrate: @mirror/init (docs/specs/mirror-init.md). Returns JSON envelope { spec_version, operation, repo, store, indexed, bytes_total, root_oid, hooks_installed, verdict }.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path":          { "type": "string",  "description": "Repo path (~d) to initialize as mirror-native store" },
                        "install_hooks": { "type": "boolean", "description": "Install pre-commit / commit-msg hooks. Default: false." }
                    },
                    "required": ["path"]
                }
            },
            {
                "name": "mirror_recall",
                "description": "recall: inbound-trajectory dual of peer beam. Observer returns to substrate in excited state asking for trajectory. Four payloads: cascade / pack_trail / pull_frontier / dogfood. Substrate: @mirror/recall (docs/specs/mirror-recall.md).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "spec_dir": { "type": "string", "description": "Spec directory to recall trajectory from" }
                    },
                    "required": ["spec_dir"]
                }
            },
            {
                "name": "mirror_peer_beam",
                "description": "peer beam: the peer HAS a torus. Beam through a peer's persistent-identity context. @song/movement.enter at cli altitude — frame-entry action of a temporal-bounded epoch at runtime. Returns @song envelope with peer identity, content-addressed spec_oid, peer_recall (4 sheaf sections), composition_pieces (7 substrate anchors). --hello-world emits structured JSON; default emits text. --mission carries a peer-side task file. Substrate: @mirror/peer/beam (shards/mirror/peer/beam.mirror action-decl `beam(...) -> @song`; renamed 2026-07-08 Tick 2 from @mirror/spawn).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "peer_home":   { "type": "string",  "description": "Peer home directory (~d). Must contain mirror.spec." },
                        "hello_world": { "type": "boolean", "description": "Emit structured JSON envelope. Default false = text envelope." },
                        "mission":     { "type": "string",  "description": "Mission file path (~f). Substrate-honest name; binary accepts both --mission and --task. Optional; substrate-absent when omitted." }
                    },
                    "required": ["peer_home"]
                }
            },
            {
                "name": "mirror_beam",
                "description": "beam: anonymous inference primitive. NO persistent-identity context. Fires @fate::select on Shape B features and emits a beam envelope. Substrate: @mirror/beam (mirror.spec top-level `command beam`; 96aa752 Tick 3 Landing 1). Per beam-as-substrate-primitive.md §3 composition table: `mirror beam <mission>` primitive; `mirror peer beam <peer_home>` persistent-identity (use mirror_peer_beam for the latter).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "mission": { "type": "string", "description": "Mission file path (~f). Substrate-honest name; binary accepts both --mission and --task." }
                    },
                    "required": ["mission"]
                }
            },
            {
                "name": "mirror_spawn",
                "description": "DEPRECATED (2026-07-08): use mirror_peer_beam instead. Backward-compat alias per two-tick discipline; routes through the cli `spawn` alias which now emits a stderr deprecation notice (b012d3f Landing 2). Substrate: renamed to @mirror/peer/beam at 9de2226 (Tick 2 atomic substrate-decl move). This tool will be removed in a subsequent tick.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "peer_home":   { "type": "string",  "description": "Peer home directory (~d). Must contain mirror.spec." },
                        "hello_world": { "type": "boolean", "description": "Emit structured JSON envelope. Default false = text envelope." },
                        "mission":     { "type": "string",  "description": "Mission file path (~f). Substrate-honest name; binary accepts both --mission and --task. Optional; substrate-absent when omitted." }
                    },
                    "required": ["peer_home"]
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

// NOTE (Mara iter-15 schema reconciliation, 2026-07-08): the `prisms`
// tool + its walker helpers (`list_prisms_in_dir`, `walk_for_prisms`,
// `extract_prism_declaration`) were removed as part of the byte-parity
// alignment with `bin/mirror-mcp`. The 8-tool bash schema has no
// `prisms` entry; introspection is out of scope for the Rust runtime
// discharge at this altitude. Future substrate-introspection ticks
// re-land the walker under a substrate-declared prism (task #312 /
// #310) rather than the hardcoded MCP arm.

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
    let try_full: Option<String> = serde_json::from_str::<Value>(payload).ok().and_then(|v| {
        v.get("verdict")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    });
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
fn run_mirror(args: &[&str], ctx: &Ctx) -> (String, i32) {
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
    //
    // Arc 2 (Seam audit `5e7fd6d` correction #3): thread `ctx.cwd()`
    // via `kintsugi_main_in` so the dispatch chain resolves relative
    // shard paths against the MCP session's cwd (typically
    // `$MIRROR_HOME`) — not the MCP server's process cwd. Removes the
    // `set_current_dir($MIRROR_HOME)` mutation that used to live in
    // `serve_loop`, closing the last process-wide cwd mutation in the
    // binary.
    let output = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::kintsugi_main_in(&argv, ctx.cwd())
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
            return (format!("mcp panic in kintsugi dispatch: {}\n", msg), 2);
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
/// Mirrors the bash wrapper's `case "$tool" in ... esac` block — the
/// 8-tool schema post-Tick-3 rename (`4f4a257`) + Tick 7 shatter fold
/// (`ffba2a7`), byte-parity with `bin/mirror-mcp` as of Mara iter-15
/// schema reconciliation (2026-07-08):
///
/// - `mirror_compile`   → `mirror compile <file>`
/// - `mirror_craft`     → `mirror craft <target> [--target-kind <k>] [--reflect]`
/// - `mirror_kintsugi`  → `mirror kintsugi --ci --out @data/json <file> [--liquid] [--shatter N]`
///                        (always `--ci --out @data/json` per Tick 7 fold;
///                        the substrate linearizes at @io, the wrapper is
///                        the transport-framer; `verdict.label ∈
///                        {partial, failure}` lifts to `isError: true`.)
/// - `mirror_init`      → `mirror init <path> [--install-hooks]`
/// - `mirror_recall`    → `mirror recall <spec_dir>`
/// - `mirror_peer_beam` → `mirror peer beam <peer_home> [--hello-world] [--mission <f>]`
/// - `mirror_beam`      → `mirror beam --mission <mission>`
/// - `mirror_spawn`     → `mirror spawn <peer_home> [--hello-world] [--mission <f>]`
///                        (DEPRECATED alias per two-tick discipline;
///                        cli-side `spawn` alias emits stderr notice per
///                        `b012d3f` Landing 2.)
///
/// Returns `(text, is_error)`. `is_error` lifts to MCP's `isError` flag
/// in the `tools/call` response so clients can programmatically
/// distinguish substrate failure (kintsugi REJECT, compile error, unknown
/// tool) from success without scraping stderr text.
fn dispatch_tool_call(tool: &str, args: &Value, ctx: &Ctx) -> (String, bool) {
    let s =
        |k: &str| -> Option<String> { args.get(k).and_then(|v| v.as_str()).map(|s| s.to_string()) };
    let b = |k: &str| -> bool { args.get(k).and_then(|v| v.as_bool()).unwrap_or(false) };
    let i = |k: &str| -> Option<i64> { args.get(k).and_then(|v| v.as_i64()) };

    let (text, exit_code) = match tool {
        "mirror_compile" => {
            let file = s("file").unwrap_or_default();
            run_mirror(&["compile", &file], ctx)
        }
        "mirror_craft" => {
            let target = s("target").unwrap_or_default();
            let mut argv: Vec<String> = vec!["craft".into(), target];
            // Substrate-honest name `target_kind` maps to the binary's
            // `--target-kind` flag (per bin/mirror-mcp; the binary
            // accepts `--target` as a backward-compat alias).
            if let Some(kind) = s("target_kind") {
                argv.push("--target-kind".into());
                argv.push(kind);
            }
            if b("reflect") {
                argv.push("--reflect".into());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "mirror_kintsugi" => {
            // Tick 7 shatter fold (`ffba2a7`): the wrapper ALWAYS routes
            // kintsugi through `--ci --out @data/json` — substrate is the
            // linearizer, wrapper is the transport-framer. The `--ci`
            // path emits the typed verdict envelope per
            // `emit_ci_verdict_json` (lib.rs). `verdict.label ∈
            // {partial, failure}` lifts to `isError: true` via
            // `parse_verdict_label`, since the underlying `--ci`
            // invocation always exits 0 by design (the workflow YAML
            // decides pass per `cmd_kintsugi_ci_single`'s contract at
            // lib.rs:855).
            let file = s("file").unwrap_or_default();
            let mut argv: Vec<String> = vec![
                "kintsugi".into(),
                "--ci".into(),
                "--out".into(),
                "@data/json".into(),
                file,
            ];
            if b("liquid") {
                argv.push("--liquid".into());
            }
            if let Some(n) = i("shatter") {
                argv.push("--shatter".into());
                argv.push(n.to_string());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            let (text, exit_code) = run_mirror(&refs, ctx);
            let is_error = match parse_verdict_label(text.trim()) {
                Some(label) => label != "success",
                // No parseable verdict + non-zero exit → substrate-internal
                // failure (e.g. panic guard). Surface as error.
                None => exit_code != 0,
            };
            return (text, is_error);
        }
        "mirror_init" => {
            let path = s("path").unwrap_or_default();
            let mut argv: Vec<String> = vec!["init".into(), path];
            if b("install_hooks") {
                argv.push("--install-hooks".into());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "mirror_recall" => {
            // Substrate-honest arg name is `spec_dir` (per
            // bin/mirror-mcp); routes to `mirror recall <dir>`.
            let spec_dir = s("spec_dir").unwrap_or_default();
            run_mirror(&["recall", &spec_dir], ctx)
        }
        "mirror_peer_beam" => {
            // Tick 3 Landing 1 (`96aa752` mirror.spec cli-block `command
            // peer { command beam }`) + Landing 2 (`b012d3f` cli dispatch
            // cmd_peer_beam). Substrate-honest `--mission` name; binary
            // accepts `--task` as backward-compat alias.
            let peer_home = s("peer_home").unwrap_or_default();
            let mut argv: Vec<String> = vec!["peer".into(), "beam".into(), peer_home];
            if b("hello_world") {
                argv.push("--hello-world".into());
            }
            if let Some(mission) = s("mission") {
                argv.push("--mission".into());
                argv.push(mission);
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "mirror_beam" => {
            // Tick 3 Landing 1 top-level `command beam` in mirror.spec +
            // Landing 2 top-level dispatch (`b012d3f`). Anonymous variant:
            // no persistent-identity context; requires mission.
            let mission = s("mission").unwrap_or_default();
            run_mirror(&["beam", "--mission", &mission], ctx)
        }
        "mirror_spawn" => {
            // DEPRECATED backward-compat alias. Routes through the cli
            // `spawn` alias (b012d3f Landing 2 dispatch alias arm) which
            // emits a stderr deprecation notice. Two-tick removal window.
            let peer_home = s("peer_home").unwrap_or_default();
            let mut argv: Vec<String> = vec!["spawn".into(), peer_home];
            if b("hello_world") {
                argv.push("--hello-world".into());
            }
            if let Some(mission) = s("mission") {
                argv.push("--mission".into());
                argv.push(mission);
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
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
///
/// Convenience wrapper delegating to [`handle_request_in`] with a
/// process-cwd-derived `Ctx`. Preserved so existing tests + fixture
/// harnesses keep the historical signature. Live MCP dispatch (Arc 2)
/// uses [`handle_request_in`] with the `$MIRROR_HOME` Ctx built by
/// [`serve_loop`], so `tools/call` payloads dispatch against the
/// substrate home rather than the MCP server's process cwd.
pub fn handle_request(line: &str) -> Option<String> {
    let ctx = Ctx::from_process_cwd();
    handle_request_in(line, &ctx)
}

/// Ctx-threaded variant of [`handle_request`]. Threads `ctx` through
/// `dispatch_tool_call` → `run_mirror` → `kintsugi_main_in` so the
/// dispatch chain resolves relative shard paths against `ctx.cwd()`.
///
/// Seam audit `5e7fd6d` correction #3: closes the MCP path's remaining
/// process-cwd dependency, retiring the `set_current_dir($MIRROR_HOME)`
/// mutation in `serve_loop`.
pub fn handle_request_in(line: &str, ctx: &Ctx) -> Option<String> {
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
            let (text, is_error) = dispatch_tool_call(&tool, &args, ctx);
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
///
/// Arc 2 (Seam audit `5e7fd6d` correction #3): the bash wrapper's
/// `cd $MIRROR_HOME` used to be preserved as a process-wide
/// `set_current_dir($MIRROR_HOME)` here — the LAST process-cwd
/// mutation in the binary. It is retired: `serve_loop` now constructs
/// a `Ctx` rooted at `$MIRROR_HOME` (falling back to the process cwd
/// when unset) and threads it through `handle_request_in` →
/// `dispatch_tool_call` → `run_mirror` → `kintsugi_main_in`. Grammar
/// paths resolve against `ctx.cwd()` explicitly; the MCP server's
/// process cwd is left untouched.
pub fn serve_loop() -> i32 {
    let ctx = match std::env::var("MIRROR_HOME") {
        Ok(home) => Ctx::new(std::path::PathBuf::from(home)),
        Err(_) => Ctx::from_process_cwd(),
    };

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
        if let Some(resp) = handle_request_in(trimmed, &ctx) {
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
    fn tools_list_advertises_eight_tools() {
        // Mara iter-15 schema reconciliation (2026-07-08): byte-parity
        // alignment with `bin/mirror-mcp` 8-tool schema — post-Tick-3
        // rename (`4f4a257` mirror_spawn → mirror_peer_beam +
        // top-level mirror_beam) + Tick 7 shatter fold (`ffba2a7`
        // kintsugi always `--ci --out @data/json`). The stale `prisms`
        // + `verdict` tools (pre-Tick-3, no matching cli-block) were
        // removed as part of the reconciliation. All tools carry the
        // `mirror_` prefix per bin/mirror-mcp.
        let req = r#"{"jsonrpc":"2.0","id":42,"method":"tools/list","params":{}}"#;
        let resp_line = handle_request(req).expect("tools/list must respond");
        let resp: Value = serde_json::from_str(&resp_line).expect("valid JSON");
        let tools = resp["result"]["tools"]
            .as_array()
            .expect("tools is an array");
        assert_eq!(tools.len(), 8);
        let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(
            names,
            vec![
                "mirror_compile",
                "mirror_craft",
                "mirror_kintsugi",
                "mirror_init",
                "mirror_recall",
                "mirror_peer_beam",
                "mirror_beam",
                "mirror_spawn",
            ]
        );
    }
}
