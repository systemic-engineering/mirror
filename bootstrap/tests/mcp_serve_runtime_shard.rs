//! Tick 6.5 RED — @mcp.serve Rust runtime discharge.
//!
//! The `boot/std/mcp.mirror` substrate closure (Mara Tick 6, `d4c9a32`)
//! landed three bilateral-predicate contracts at grammar altitude:
//! `dispatch_reflects_cli_block(dispatch)`, `tools_reflects_cli_block(tools)`,
//! `frame_relativity(response)`. What was missing at runtime altitude: Path
//! B in `bootstrap/src/lib.rs` `dispatch()` sees `@mcp.serve` and routes it
//! through `execute_pipeline` — which does NOT know how to dispatch @mcp.
//! Empirically: `mirror /dev/stdin @mcp.serve` returns exit 1 with zero
//! stdout. Silent no-op. Bilateral predicates DECLARED but NEVER FIRED.
//!
//! This shard pins the runtime discharge contract: `mirror /dev/stdin
//! @mcp.serve` MUST route to `mirror::mcp::serve_loop` (the lifted MCP
//! server that already discharges the substrate — commit `mcp_handshake.rs`
//! landings). The routing is a special-case in `dispatch()` mirroring the
//! `@mirror/kintsugi` and `@mirror/butterfly` cases in `execute_pipeline`.
//!
//! Substrate-honest naming: the Rust `dispatch()` special-case IS the
//! runtime discharge of the bilateral predicates. `mcp::serve_loop` +
//! `mcp::handle_request_in` name what `dispatches_to_cli_block` +
//! `tools_reflects_cli_block` discharge against — the substrate contract
//! is realized by the Rust body.
//!
//! **Test-altitude**: subprocess. Uses `std::process::Command` on the
//! installed mirror binary at `~/.local/bin/mirror` (or `$MIRROR_BIN`).
//! Requires the binary to be built + installed — same convention as
//! `mcp_serve_dispatch_shard.rs` T1. Skips gracefully when unavailable.
//!
//! **Predecessors**:
//! - Taut scout `cf5ab8c` (`docs/scouts/2026-07-08-taut-mcp-serve-lift-scope.md`)
//! - Mara Tick 6 substrate closure `d4c9a32` (boot/std/mcp.mirror requires)
//! - Mara Tick 7 shatter fold `ffba2a7` (bin/mirror-mcp mirror_kintsugi)
//! - `bootstrap/tests/mcp_handshake.rs` (mcp::handle_request already lifted)

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn mirror_bin() -> Option<PathBuf> {
    // Prefer the freshly-built cargo test binary so RED/GREEN cycles run
    // against the tree under test, not a stale ~/.local/bin/mirror. Falls
    // back to $MIRROR_BIN and ~/.local/bin/mirror for parity with existing
    // shard conventions when the CARGO_BIN_EXE symbol is absent.
    let cargo_exe: &str = env!("CARGO_BIN_EXE_mirror");
    let p = PathBuf::from(cargo_exe);
    if p.exists() {
        return Some(p);
    }
    if let Ok(bin) = std::env::var("MIRROR_BIN") {
        let p = PathBuf::from(bin);
        if p.exists() {
            return Some(p);
        }
    }
    let home = std::env::var("HOME").ok()?;
    let p = PathBuf::from(home).join(".local/bin/mirror");
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent")
        .to_path_buf()
}

/// Pipe `stdin_bytes` through `mirror /dev/stdin @mcp.serve` and return
/// (stdout, stderr, exit_code). None if the binary is unavailable.
fn run_mcp_serve(stdin_bytes: &[u8]) -> Option<(String, String, i32)> {
    let bin = mirror_bin()?;
    let mut child = Command::new(&bin)
        .arg("/dev/stdin")
        .arg("@mcp.serve")
        .current_dir(repo_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    child
        .stdin
        .as_mut()
        .expect("stdin piped")
        .write_all(stdin_bytes)
        .expect("write stdin");
    // Close stdin so serve_loop hits EOF and exits.
    drop(child.stdin.take());
    let out = child.wait_with_output().expect("child wait");
    Some((
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    ))
}

// === T1: Path B routing — @mcp.serve MUST NOT silent-no-op ===

#[test]
fn t01_mirror_stdin_mcp_serve_emits_response_not_silent() {
    let Some((stdout, stderr, code)) =
        run_mcp_serve(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}\n")
    else {
        eprintln!("skip: mirror binary not installed at $HOME/.local/bin/mirror or $MIRROR_BIN");
        return;
    };
    assert!(
        !stdout.trim().is_empty(),
        "T1: `mirror /dev/stdin @mcp.serve` MUST emit a JSON-RPC response on valid input, not silently no-op. stdout=<{}> stderr=<{}> code={}",
        stdout, stderr, code
    );
    assert_eq!(
        code, 0,
        "T1: exit code MUST be 0 on clean EOF. stderr=<{}>",
        stderr
    );
}

// === T2: initialize returns capabilities envelope ===

#[test]
fn t02_initialize_returns_capabilities_envelope() {
    let Some((stdout, _stderr, _code)) =
        run_mcp_serve(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}\n")
    else {
        eprintln!("skip: mirror binary not installed");
        return;
    };
    let line = stdout.lines().next().expect("at least one response line");
    let v: serde_json::Value = serde_json::from_str(line).unwrap_or_else(|e| {
        panic!(
            "T2: response line must be valid JSON: {} — line=<{}>",
            e, line
        )
    });
    assert_eq!(v["jsonrpc"], "2.0");
    assert_eq!(v["id"], 1);
    assert_eq!(v["result"]["serverInfo"]["name"], "mirror");
    assert_eq!(v["result"]["protocolVersion"], "2024-11-05");
    assert!(
        v["result"]["capabilities"]["tools"].is_object(),
        "T2: capabilities.tools MUST be an object; got response={}",
        v
    );
}

// === T3: tools/list returns non-empty tool array ===

#[test]
fn t03_tools_list_returns_tool_array() {
    let Some((stdout, _stderr, _code)) =
        run_mcp_serve(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n")
    else {
        eprintln!("skip: mirror binary not installed");
        return;
    };
    let line = stdout.lines().next().expect("at least one response line");
    let v: serde_json::Value = serde_json::from_str(line).unwrap_or_else(|e| {
        panic!(
            "T3: response line must be valid JSON: {} — line=<{}>",
            e, line
        )
    });
    let tools = v["result"]["tools"]
        .as_array()
        .expect("T3: result.tools MUST be an array");
    assert!(
        !tools.is_empty(),
        "T3: tools/list MUST advertise at least one tool (bilateral predicate tools_reflects_cli_block); got: {}",
        v
    );
    // Runtime discharges tools_reflects_cli_block: any Rust-lifted tool
    // set is admissible for this tick (Simple approach). Exact schema
    // reflection to mirror.spec cli-block is future substrate-motion.
    let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(
        names.iter().any(|n| *n == "compile"),
        "T3: tools/list MUST advertise `compile` (the load-bearing verb per mirror.spec cli-block); got: {:?}",
        names
    );
}

// === T4: tools/call routes through existing dispatch ===

#[test]
fn t04_tools_call_compile_dispatches_returns_oid() {
    let root = repo_root();
    let target = root.join("boot/std/mcp.mirror");
    let target_str = target.to_string_lossy().replace('\\', "\\\\");
    let req = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"tools/call\",\"params\":{{\"name\":\"compile\",\"arguments\":{{\"file\":\"{}\"}}}}}}\n",
        target_str
    );
    let Some((stdout, stderr, _code)) = run_mcp_serve(req.as_bytes()) else {
        eprintln!("skip: mirror binary not installed");
        return;
    };
    let line = stdout.lines().next().unwrap_or_else(|| {
        panic!(
            "T4: tools/call MUST emit a response line; stdout=<{}> stderr=<{}>",
            stdout, stderr
        )
    });
    let v: serde_json::Value = serde_json::from_str(line)
        .unwrap_or_else(|e| panic!("T4: response must be valid JSON: {} — line=<{}>", e, line));
    let is_error = v["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        !is_error,
        "T4: `compile` of boot/std/mcp.mirror MUST succeed (Tick 6 landed substrate closure); got: {}",
        v
    );
    let text = v["result"]["content"][0]["text"]
        .as_str()
        .expect("text payload");
    // Compile emits a 64-char hex OID line; may be followed by a
    // `(cached)` trailer from the store-level cache. Substrate-honest
    // check: OID appears as SOME line in the payload, not necessarily
    // the last (`(cached)` can trail).
    let oid_line = text
        .trim()
        .lines()
        .map(|l| l.trim())
        .find(|l| l.len() == 64 && l.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(
        oid_line.is_some(),
        "T4: compile via tools/call MUST include a 64-char hex OID line in text payload; full=<{}>",
        text
    );
}

// === T5: multiple JSON-RPC lines process sequentially ===

#[test]
fn t05_multiple_lines_process_sequentially() {
    let stdin = b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}\n{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n";
    let Some((stdout, _stderr, _code)) = run_mcp_serve(stdin) else {
        eprintln!("skip: mirror binary not installed");
        return;
    };
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(
        lines.len() >= 2,
        "T5: two request lines MUST produce two response lines; got {} response line(s): stdout=<{}>",
        lines.len(),
        stdout
    );
    let first: serde_json::Value = serde_json::from_str(lines[0]).expect("line 1 valid JSON");
    let second: serde_json::Value = serde_json::from_str(lines[1]).expect("line 2 valid JSON");
    assert_eq!(
        first["id"], 1,
        "T5: response ordering preserved; first id=1"
    );
    assert_eq!(
        second["id"], 2,
        "T5: response ordering preserved; second id=2"
    );
}

// === T6: notifications/initialized emits no response ===

#[test]
fn t06_notifications_initialized_emits_no_response() {
    // notifications/initialized has no response per JSON-RPC 2.0 §4.1.5.
    // We follow it with a subsequent initialize so we can detect that the
    // loop kept running (the second response line appears).
    let stdin = b"{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"notifications/initialized\",\"params\":{}}\n{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"initialize\",\"params\":{}}\n";
    let Some((stdout, _stderr, _code)) = run_mcp_serve(stdin) else {
        eprintln!("skip: mirror binary not installed");
        return;
    };
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "T6: notifications/initialized MUST NOT emit a response line; expected exactly 1 line (for the follow-on initialize id=4); got {}: stdout=<{}>",
        lines.len(),
        stdout
    );
    let v: serde_json::Value = serde_json::from_str(lines[0]).expect("initialize resp valid JSON");
    assert_eq!(
        v["id"], 4,
        "T6: the sole response is the follow-on initialize (id=4)"
    );
}
