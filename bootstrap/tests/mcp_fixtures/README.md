# MCP fixtures — bash-wrapper behavioral capture

*Seeded by Taut on 2026-06-18 as part of the bash→Rust lift of `bin/mirror-mcp`.
The fixture files in this directory capture the exact byte-level responses
the 145-line bash wrapper at `bin/mirror-mcp` emitted for canonical
JSON-RPC requests, BEFORE the lift.*

## Why this exists

Per Seam's pre-loop review (`docs/specs/seam-pre-loop-mirror-mcp-lsp-review-2026-06-18.md`)
finding 2.3.B and 2.5.A: a build succeeding is not sufficient verification
that the MCP layer's capability is real. The bash wrapper was the prior
substrate carrier; the Rust lift must preserve its behavior exactly
for the three tools (`mirror_compile`, `mirror_craft`, `mirror_kintsugi`).

The `tests/mcp_fixtures/` discipline:

1. Each `.req.json` file is a single JSON-RPC line written to stdin.
2. Each `.resp.json` file is the line the bash wrapper emitted on stdout
   (or empty if no response — `notifications/initialized` is such a case).
3. The integration test `tests/mcp_handshake.rs` runs the captured
   request through `mirror::mcp::handle_request` (the Rust lift) and
   asserts byte-equality against the captured response.

The lift is correct iff every fixture's response matches.

## Fixtures captured at tick 0 (bash baseline)

| Fixture                            | What it captures                                  |
|------------------------------------|---------------------------------------------------|
| `initialize`                       | The MCP `initialize` handshake response           |
| `notifications_initialized`        | The notification (no response expected)           |
| `tools_list`                       | The three-tool capability advertisement           |

The `tools/call` per-tool fixtures are deferred to tick 2 because
their responses depend on the installed mirror binary's behavior
(which IS what subsequent loop ticks evolve). The handshake +
tools/list fixtures are the load-bearing surface: they prove the
lifted Rust server speaks the same JSON-RPC dialect at the same
capability altitude.
