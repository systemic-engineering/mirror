//! Tick 6 RED — `boot/std/mcp.mirror` substrate closure:
//! discharge `dispatch(request) -> response` and `tools -> json` `\`-cracks
//! via bilateral-predicate closure (Option A per Taut scout `cf5ab8c` +
//! Mara Tick 6 substrate-honest choice).
//!
//! **Substrate-decl closure at boot altitude**. The MCP transport primitive
//! at `boot/std/mcp.mirror` has carried three `\`-cracks since 2026-05-20:
//! `dispatch(request) -> response`, `tools -> json`, and `fate(...) ->
//! imperfect`. The Rust binary already routes `mirror /dev/stdin @mcp.serve`
//! through Path B (`bootstrap/src/lib.rs:2858`). What's missing is the
//! substrate-side semantic contract that names WHAT the dispatch discharges
//! against, so the follow-on tick (6.5/7) that collapses `bin/mirror-mcp`
//! (149 lines of bash) has a legitimate substrate target.
//!
//! **Option A (bilateral-predicate) chosen over Option B (compositional
//! `@mirror/cli.reflect_dispatch(r)` body)**: `@mirror/cli` today exposes
//! only `command`, `arg`, `flag`, `default` + type vocabulary (per
//! `shards/mirror/lens/cli.mirror` `out` block). Naming
//! `reflect_dispatch` would invent unwitnessed surface. The bilateral-
//! predicate form matches landed substrate discipline
//! (`boot/std/spectral/portal.mirror` `requires content_addressed(portal)`,
//! `boot/std/mirror/shard.mirror` `requires autopoietic(shard)`, etc.).
//!
//! **Substrate-pull chain**:
//! - Taut scout `cf5ab8c` LRM verdict — LANDABLE WITH ONE PREREQUISITE
//! - Tick 6 (this): substrate closure — bilateral-predicate contracts +
//!   RED-first admissibility tests
//! - Tick 6.5/7 (deferred): `bin/mirror-mcp` collapse to
//!   `exec ~/.local/bin/mirror /dev/stdin @mcp.serve` once parity verifies
//!
//! **Test-altitude**: COMPILE. These tests lock the substrate-decl surface
//! at grammar altitude. They do NOT test the Rust dispatch (that's what
//! Tick 6.5/7 parity-verifies).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_mcp_shard() -> String {
    let path = repo_root().join("boot/std/mcp.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read boot/std/mcp.mirror at {:?}: {}", path, e))
}

// === T1: compile-altitude green — mirror compile returns an OID ===

#[test]
fn t01_mirror_compile_returns_oid_for_boot_std_mcp() {
    let root = repo_root();
    let mcp_path = root.join("boot/std/mcp.mirror");
    let output = Command::new("mirror")
        .arg("compile")
        .arg(&mcp_path)
        .current_dir(&root)
        .output()
        .expect("invoke `mirror compile boot/std/mcp.mirror`");
    assert!(
        output.status.success(),
        "T1: `mirror compile boot/std/mcp.mirror` MUST succeed after substrate closure. stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let last_line = stdout.lines().last().unwrap_or("").trim();
    let is_hex_oid = last_line.len() == 64 && last_line.chars().all(|c| c.is_ascii_hexdigit());
    assert!(
        is_hex_oid,
        "T1: `mirror compile` MUST emit a 64-char hex OID as its final line; got `{}`",
        last_line
    );
}

// === T2-T3: dispatch action no longer purely `\`-cracked ===

#[test]
fn t02_dispatch_action_declares_bilateral_predicate_closure() {
    let content = read_mcp_shard();
    // The dispatch action's contract MUST be named — either via a
    // `requires` clause at grammar altitude naming the dispatch
    // predicate, OR via a bilateral-predicate action reference in the
    // body. Option A chosen: `requires` clause at grammar altitude.
    let has_requires_method_valid = content.contains("requires method_valid")
        || content.contains("requires dispatches_to_cli_block")
        || content.contains("requires dispatch_reflects_cli_block");
    assert!(
        has_requires_method_valid,
        "T2: grammar @mcp MUST declare `requires method_valid(...)` OR `requires dispatches_to_cli_block(...)` OR `requires dispatch_reflects_cli_block(...)` — the bilateral-predicate contract naming what `dispatch(request) -> response` discharges against. Follows landed pattern at `boot/std/spectral/portal.mirror` (`requires content_addressed(portal)`, etc.)."
    );
}

#[test]
fn t03_dispatch_action_body_is_no_longer_bare_crack_only() {
    let content = read_mcp_shard();
    // The dispatch action MAY retain a `\`-crack body (bilateral-predicate
    // closure discharges at grammar altitude, not action-body altitude),
    // but the file MUST name the semantic contract SOMEWHERE — either
    // via `requires` at grammar altitude (Option A) OR body composition
    // (Option B). Option A retains `dispatch(request) -> response { \ }`
    // + adds `requires` clauses. This test asserts the file WEAVES the
    // semantic contract, not just leaves three bare cracks.
    let bare_crack_count = content.matches("{ \\ }").count();
    let has_semantic_closure =
        content.contains("requires ") || content.contains("@mirror/cli.reflect");
    assert!(
        has_semantic_closure,
        "T3: substrate-decl MUST close the semantic contract via `requires <predicate>(...)` (Option A) OR compositional body via `@mirror/cli.<action>(...)` (Option B). Currently the file has {} bare `{{ \\\\ }}` cracks with NO semantic-contract weaving.",
        bare_crack_count
    );
}

// === T4-T5: tools action names cli-block reflection contract ===

#[test]
fn t04_tools_action_declares_cli_block_reflection_contract() {
    let content = read_mcp_shard();
    // `tools -> json` MUST name what the tool listing reflects against —
    // the mirror.spec cli-block is the substrate-honest source.
    let has_reflection_contract = content.contains("requires tools_reflects_cli_block")
        || content.contains("requires reflects_cli_block")
        || content.contains("requires reflects_mirror_spec")
        || (content.contains("tools") && content.contains("cli") && content.contains("requires"));
    assert!(
        has_reflection_contract,
        "T4: grammar @mcp MUST declare a `requires` clause naming that `tools -> json` reflects mirror.spec's cli-block (e.g. `requires tools_reflects_cli_block(tools)` or `requires reflects_cli_block(tools)`)."
    );
}

#[test]
fn t05_narrative_cites_cli_block_reflection_source() {
    let content = read_mcp_shard();
    // The narrative comment(s) MUST cite that dispatch + tools reflect
    // mirror.spec's cli-block — the substrate-honest source of tool
    // schema + verb dispatch.
    let has_narrative_citation = (content.contains("cli-block")
        || content.contains("cli block")
        || content.contains("mirror.spec"))
        && (content.contains("reflect") || content.contains("synthesize"));
    assert!(
        has_narrative_citation,
        "T5: narrative MUST cite that `dispatch` + `tools` reflect mirror.spec's cli-block (the substrate-honest source of tool schema + verb dispatch). Grep for `cli-block` + `reflect` OR `synthesize`."
    );
}

// === T6: pipeline preserved (regression guard) ===

#[test]
fn t06_serve_pipeline_preserved_load_bearing() {
    let content = read_mcp_shard();
    // The @io.read(stdin) |> @data/json.parse |> dispatch |> @data/json.emit
    // |> @io.write(stdout) pipeline at :9-11 is the substrate contract
    // that Path B of the mirror binary consumes. It MUST NOT drift.
    assert!(
        content.contains("@io.read(stdin)"),
        "T6: pipeline MUST retain `@io.read(stdin)` — the substrate entry point Path B consumes."
    );
    assert!(
        content.contains("@data/json.parse"),
        "T6: pipeline MUST retain `@data/json.parse` — the JSON-RPC codec entry."
    );
    assert!(
        content.contains("dispatch"),
        "T6: pipeline MUST retain the `dispatch` action reference."
    );
    assert!(
        content.contains("@data/json.emit"),
        "T6: pipeline MUST retain `@data/json.emit` — the JSON-RPC codec exit."
    );
    assert!(
        content.contains("@io.write(stdout)"),
        "T6: pipeline MUST retain `@io.write(stdout)` — the substrate exit point."
    );
}

// === T7: exports preserved (regression guard) ===

#[test]
fn t07_exports_preserved() {
    let content = read_mcp_shard();
    for export in ["out serve", "out dispatch", "out tools", "out fate"] {
        assert!(
            content.contains(export),
            "T7 (regression): existing export `{}` MUST remain in the `out` block. Substrate closure adds contracts; it does not remove surface.",
            export
        );
    }
}

// === T8: type surface preserved (regression guard) ===

#[test]
fn t08_request_and_response_types_preserved() {
    let content = read_mcp_shard();
    assert!(
        content.contains("type request") && content.contains("method: text"),
        "T8 (regression): `type request = {{ method: text, params: json, id: json }}` MUST remain declared."
    );
    assert!(
        content.contains("type response") && content.contains("result: json"),
        "T8 (regression): `type response = {{ result: json, id: json }}` MUST remain declared."
    );
}
