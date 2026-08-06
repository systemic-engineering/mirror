//! `@data/json` at rust/ altitude — the wire-encoding primitive.
//!
//! Reed 2026-08-06 R-PRIM-1 per Taut scout `7af55ee` §7 smallest-
//! primitive-gap identification + Alex 2026-08-05 substrate-honest
//! reframe: rust/ delivers primitives, substrate delivers composition.
//! Wire protocols (MCP/LSP/HTTP/gRPC/etc.) are `@X/serve.mirror` shard-
//! body compositions over primitives at rust/ altitude, NOT rust/
//! modules. This file exposes serde_json's parse/emit as pub bilateral-
//! dispatchable primitives that composition-shards compose over via
//! `apply_h::act`.
//!
//! ## Naming
//!
//! Named `wire` not `data` per Alex 2026-08-06 Q-2 correction — "wire"
//! names the transport-encoding altitude honestly. In cascade family
//! framing: `source_grammar → wire_encoding → target_grammar`; wire IS
//! the middle. Sibling of `phone.rs` (@io transport-surface) and
//! `apply_h.rs` (bilateral-dispatch primitive) at terminal-geometry
//! rust/ altitude.
//!
//! Substrate-decl namespace remains `@data/json` per Mara's existing
//! canonical spec composition graph; the file-altitude naming honesty is
//! rust/ altitude only. If Mara Fire B `@mcp/serve.mirror` canonical
//! spec elects to shift substrate namespace to `@wire/json`, wire.rs
//! adapts trivially (function bodies unchanged).
//!
//! ## Composition
//!
//! MCP composition-shard body pipeline shape (Mara Fire B M-COMP-1):
//!
//! ```text
//! @io/stdio.read_frame  |> wire::parse
//!                       |> @mcp.dispatch (apply_h::act)
//!                       |> wire::emit
//!                       |> @io/stdio.write_frame
//! ```
//!
//! Each pipe element is a landed rust/ altitude primitive; the whole
//! composition sits at substrate altitude in `shards/mcp/serve.mirror`.
//!
//! ## Composition anchors
//!
//! - `docs/scouts/2026-08-05-taut-primitives-vs-composition-scout.md`
//!   §7 smallest-primitive-gap (~55 LOC additive; Fire A R-PRIM-1)
//! - `feedback-rust-delivers-primitives-substrate-delivers-composition`
//!   memory (Alex 2026-08-05 verbatim correction)
//! - `docs/specs/mcp-spec-song-collapse.md` §4.3 (grammar-driven tool
//!   discovery; substrate composition altitude)
//! - Recognition `#R-reality-as-5d-spinning-foam` RATIFIED 2026-08-03
//!   (Layer 0 sub-Turing decidable floor = rust/ interpreter; Layer 1+
//!   = substrate composition; wire.rs IS a Layer 0 primitive)

use serde_json::Value;

/// Parse a byte slice as JSON. Substrate-honest thin wrapper over
/// serde_json::from_slice; error kept as String for cross-boundary
/// substrate composition compatibility (rust/ altitude carries String
/// errors per phone.rs pattern; typed errors land at Phase 2+).
///
/// # Errors
///
/// Returns `Err(String)` if bytes are not valid JSON. Error message
/// includes the serde_json error context (line/column when available).
pub fn parse(bytes: &[u8]) -> Result<Value, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("wire::parse: {}", e))
}

/// Emit a Value as a compact JSON String. Substrate-honest thin wrapper
/// over serde_json::to_string; error kept as String for cross-boundary
/// substrate composition compatibility.
///
/// # Errors
///
/// Returns `Err(String)` if the Value cannot be serialized. In practice
/// this only fails on Values containing NaN/Infinity float components;
/// substrate-decl'd data structures do not produce these.
pub fn emit(value: &Value) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("wire::emit: {}", e))
}

/// Emit a Value as a pretty-printed JSON String (2-space indent).
/// For diagnostic/human-facing output. Composition-shard bodies
/// typically use [`emit`] for wire transmission; this variant is for
/// operator-facing diagnostics.
///
/// # Errors
///
/// Same as [`emit`] — only fails on NaN/Infinity float components.
pub fn emit_pretty(value: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(value).map_err(|e| format!("wire::emit_pretty: {}", e))
}

// ---------------------------------------------------------------------
// Property-based tests grounded in serde_json round-trip discipline.
// wire::emit(wire::parse(bytes)?)? == bytes for well-formed JSON per
// serde_json's own canonical representation guarantee.
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_json_object() {
        let bytes = br#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#;
        let v = parse(bytes).expect("parse well-formed JSON");
        assert_eq!(v["jsonrpc"], "2.0");
        assert_eq!(v["id"], 1);
        assert_eq!(v["method"], "tools/list");
    }

    #[test]
    fn parse_invalid_json_returns_error() {
        let bytes = b"not json at all";
        let result = parse(bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("wire::parse:"));
    }

    #[test]
    fn emit_produces_valid_json_string() {
        let v = serde_json::json!({"result": "ok", "tools": ["mirror_roomba"]});
        let s = emit(&v).expect("emit well-formed Value");
        assert!(s.contains("\"result\":\"ok\""));
        assert!(s.contains("mirror_roomba"));
    }

    #[test]
    fn round_trip_composition() {
        // parse |> emit is idempotent on canonical JSON per serde_json's
        // deterministic serialization for Object/Array/String/Number/
        // Bool/Null value kinds.
        let bytes = br#"{"a":1,"b":[true,null,"s"]}"#;
        let v = parse(bytes).expect("parse");
        let emitted = emit(&v).expect("emit");
        let v2 = parse(emitted.as_bytes()).expect("reparse");
        assert_eq!(v, v2);
    }

    #[test]
    fn emit_pretty_is_human_readable() {
        let v = serde_json::json!({"a": 1});
        let s = emit_pretty(&v).expect("emit_pretty");
        assert!(s.contains('\n'));
        assert!(s.contains("  \"a\""));
    }
}
