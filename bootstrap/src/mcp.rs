//! MCP server — the JSON-RPC tool dispatch surface for the agent lens.
//!
//! ## Phase 1b session context (Reed 2026-08-24 per Alex "dispatch")
//!
//! `MCP_SESSION_STATE` accumulates per-session query history for @mq
//! context monoid discipline per shards/mq.mirror §8. Each
//! `mirror_query` invocation reads/writes session state; session ID
//! either supplied via `session` arg OR auto-generated from PID+PPID
//! at first-tool-call. Session state persists for lifetime of
//! `serve_loop` process (dies with client disconnect).
//!
//! Substrate anchor: shards/mq.mirror §8 context algebra +
//! Mara §15 reshape recommendation #4 (extend context monoid with
//! push_query for verdict-accumulation).
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

/// MCP session state: per-session query history accumulator per
/// shards/mq.mirror §8 context monoid. Threads `push_query` per
/// Mara §15 reshape #4. Session ID either supplied via mirror_query
/// `session` arg OR auto-generated at first-tool-call (PID-derived).
///
/// Reed 2026-08-24 Phase 1b MVP: in-memory HashMap keyed by session
/// ID; value is Vec<String> of query texts in temporal order. Full
/// context monoid (frame + eigenboard_state per @mq §8) forward-
/// promised as Phase 1c per Reed rust wire post-Mara #400 return.
static MCP_SESSION_STATE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, Vec<String>>>> =
    std::sync::OnceLock::new();

fn session_state() -> &'static std::sync::Mutex<std::collections::HashMap<String, Vec<String>>> {
    MCP_SESSION_STATE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// Push a query to the session's context stack. Returns the session's
/// query count post-push and a snapshot of prior queries.
/// Per shards/mq.mirror §8 push_query(ctx, operation) -> context.
fn push_query_to_session(session_id: &str, query: &str) -> (usize, Vec<String>) {
    let state = session_state();
    let mut guard = state.lock().expect("session state mutex poisoned");
    let queue = guard.entry(session_id.to_string()).or_insert_with(Vec::new);
    queue.push(query.to_string());
    let count = queue.len();
    let prior: Vec<String> = queue.iter().take(queue.len().saturating_sub(1)).cloned().collect();
    (count, prior)
}

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
                "name": "kintsugi",
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
                "name": "init",
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
                "name": "recall",
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
                "name": "peer_beam",
                "description": "peer beam: the peer HAS a torus. Beam through a peer's persistent-identity context. @song/movement.enter at cli altitude — frame-entry action of a temporal-bounded epoch at runtime. Returns @song envelope with peer identity, content-addressed spec_oid, peer_recall (4 sheaf sections), composition_pieces (7 substrate anchors). Flag composition (all optional): hello_world emits structured JSON (default: text); mission carries a peer-side task file; fate_select routes to @optics/lens/features.get + Fate::excited().resolve for COMPUTED candidates; from_psychohistory bounds decisions by peer's psychohistory sheaf root (Mara `ce9745f` bounded_by); with_shadow casts 5 hypothetical shadows + classifies shadow_regime per Reed `07ac55a` (Mara `1999b01` §6 @torus winding basins); emit_diff serializes the peer's chosen edit as a diff; integrate_diff persists operator's @integrate-diff to peer_home/.bauchladen/ closing the autopoietic loop (Reed `4b2ef3c`). Substrate: @mirror/peer/beam (shards/mirror/peer/beam.mirror action-decl `beam(...) -> @song`; renamed 2026-07-08 Tick 2 from @mirror/spawn). λ₀(Δ_F) is the metric all flag combinations optimize for (`shards/cyberpunk.mirror` cybernetic_coherence annotation, `8e6e517`).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "peer_home":          { "type": "string",  "description": "Peer home directory (~d). Must contain mirror.spec." },
                        "hello_world":        { "type": "boolean", "description": "Emit structured JSON envelope. Default false = text envelope." },
                        "mission":            { "type": "string",  "description": "Mission file path (~f). Substrate-honest name; binary accepts both --mission and --task. Optional; substrate-absent when omitted." },
                        "fate_select":        { "type": "boolean", "description": "Route to fate optical inference: @optics/lens/features.get + Fate::excited().resolve. Peer produces COMPUTED candidates from features rather than substrate OBSERVATION. Default false." },
                        "from_psychohistory": { "type": "boolean", "description": "Bound decisions by peer's psychohistory sheaf root (Mara `ce9745f` bounded_by(sheaf) — Rayleigh descent along the peer's own history). Requires fate_select. Default false." },
                        "with_shadow":        { "type": "boolean", "description": "Cast 5 hypothetical shadows (one per fate Model) + classify shadow_regime (converged/necker/escher/kanizsa) per Reed `07ac55a`. Peer diagnoses its own inference geometry. Requires fate_select + from_psychohistory. Default false." },
                        "emit_diff":          { "type": "boolean", "description": "Serialize the peer's chosen edit as a unified diff on stdout. Default false." },
                        "integrate_diff":     { "type": "boolean", "description": "Persist operator's @integrate-diff to peer_home/.bauchladen/ — the autopoietic-closure write leg (Reed `4b2ef3c`). Next peer beam tick reads the updated substrate. Mutually exclusive with emit_diff (integrate wins)." },
                        "song":               { "type": "string",  "description": "Song file path (~f). Rung 1 @song/beat runtime dispatch per Mara `94e55eb` sixth species; Rung 2 line-per-beat phrase parsing; Rung 3 mirror-native tokenize+AST walk (nested song/movement/voice/progression/narrative/phrase blocks per Mara `d29d45e` Path B). Fires @kintsugi/oscillate ACTIVE/DARK pulse per beat; emits per-block envelope naming @song + species substrate authorities." },
                        "dance_with":         { "type": "string",  "description": "Second peer-home for Rung 4 multi-peer @dance coupling (requires song). Both peers execute the shared song; runtime computes Kuramoto order parameter + Aumann agreement + shared_root_oid + convergence_verdict per Mara `417ec25` Scope B narrowed to coherence phase-lock. Envelope names @dance + @resonance + @cyberpunk + @bauchladen authorities. Coherence stub at Rung 4; λ₀(Δ_F) actual computation forward-promised to Rung 4.5." },
                        "deploy_to":          { "type": "string",  "description": "Rung 5 mycelial-envelope-declared deployment target (requires song + dance_with) per Mara `9c4ef5b` Scope A. Composes over Rung 4 dance shared_root_oid; emits deployment envelope naming @spectral/garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/mosaic + @song/beat substrate authorities. Target is URL-shaped string (may be spectral.engineer, file://, or any string — target is declarative not operationally verified at Rung 5). Actual nix build forward-promised to Rung 5.5; actual mycelial gossip forward-promised to Rung 6." }
                    },
                    "required": ["peer_home"]
                }
            },
            {
                "name": "beam",
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
                "name": "spawn",
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
            },
            {
                "name": "beam_act",
                "description": "beam act: dispatch a substrate-decl'd action against the 7-combinator evaluator surface (@apply_h). First user-invocable substrate dispatch — sbec lifts from 0 to > 0 through this call. Verdict marshaling: Pass → text 'Pass' + isError=false; Fail(reason) → text 'Fail: <reason>' + isError=true; Partial(transparency) → text 'Partial:\n  <loc>: <opacity>...' + isError=true. Substrate: @mirror/beam/act (CLI verb landed Arc-1 Tick 1.4 per docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md; renamed 2026-07-15 via two-step substrate learning execute → dispatch → act per Seam seamfinder audit `546c2f6`). Empirical anchor: `mirror beam act @subject/visibility/public consent_scope_universal` returns Pass exit 0.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "shard_path": { "type": "string", "description": "Substrate shard path (e.g. `@subject/visibility/public`). Concatenated with `.<action>` to form the shard_action_ref the resolver in apply_h::act recognizes." },
                        "action":     { "type": "string", "description": "Action name declared on the shard (e.g. `consent_scope_universal`)." },
                        "args":       { "type": "array", "description": "Optional positional args passed as Value oids to apply_h::act. When absent for a landed bilateral predicate on @subject/visibility/public, the substrate-decl'd sentinel is synthesized so the empirical target hits Pass without operator fixtures.", "items": { "type": "string" } }
                    },
                    "required": ["shard_path", "action"]
                }
            },
            {
                "name": "query",
                "description": "Dispatch MQ query (Mirror Query). Per shards/mq.mirror (family-root landed Reed 105a1e4 2026-08-23) + Mara math foundation d0347a4: MQ IS mirror's ALGEBRA + query language + FLOOR underneath garden-db + MCP + every mirror CLI invocation. Accepts natural-language intent via backslash intent-hole OR pipe-composed MQ query. Returns result<T> with Karl-Tomm residual-ambiguity-resolution per Alex 2026-08-23 verbatim: Clear = first-order data alone; Opaque = data + second-order K-T question at altitude+1; Dark = K-T question alone. Rec #92 kleinos-Transparency<P> LOVE-monoid at query-language altitude + Karl-Tomm 1987-88 circular-reflexive question form.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query":   { "type": "string", "description": "MQ query text: backslash-prefix natural-language intent OR pipe-composed focus/project/split/shift/settle chain." },
                        "context": { "type": "string", "description": "Optional session context ref for MQ context monoid push_query (per @mq §8 context algebra)." },
                        "session": { "type": "string", "description": "Optional session ID from MCP handshake for session-persistent context stack." }
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "bumblebee_buzz",
                "description": "buzz: perturbation-source that rattles substrate tension. Per Mara Rec #95 canonical spec 0bfd427 + companion spec 06789fa §3.3: `--buzz-target=@cascade` fires @roomba walker to absorb @cascade species into @bumblebee via byte-for-byte rename map. Dry-run by default (execute=false); execute=true triggers actual modification. First MCP-driven substrate-modification per Alex 2026-08-23 in-transcript authorization + MCP-MVP-fire constraint. Rec #90 (𝓜=𝓜(𝓜)) empirical fire at MCP-layer altitude.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "target": { "type": "string", "description": "Substrate target ref, e.g. `@cascade` at MVP altitude" },
                        "execute": { "type": "boolean", "description": "false (default) = dry-run plan output; true = actual modification (requires second-phase authorization)" }
                    },
                    "required": ["target"]
                }
            },
            {
                "name": "roomba",
                "description": "roomba: substrate walker that back-projects to @mirror/store. Runs `mirror roomba --vacuum=<dir>` at rust/ altitude terminal geometry (per Mara `81294b3` three-file rewrite + Migration 5 `9bb1f57`). Walker enumerates + classifies + bilateral-arm-collapses per Mara §7.4 dispatch matrix; commits as `mirror <mirror@spectral.engineer>`; deposits pheromone-signature crystal at docs/bauchladen/ (rolling holonomy trace per stigmergy math foundation). Substrate: @kintsugi/roomba (shards/kintsugi/roomba.mirror + shards/roomba.mirror); empirical firing since 2026-07-28. Reed 2026-08-03 nearly-today addition per Alex Option C (Fire A + Mara M4 parallel per docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md forward-promise). This tool ships the smallest-meaningful-song empirical MCP-spawn round-trip TODAY: MCP client invokes → walker fires + back-projects + commits + deposits crystal → next MCP session observes delta at @mirror/store. Composes over crown-theorem @torus + Recognition `#R-reality-as-5d-spinning-foam` RATIFIED 2026-08-03: pheromone-deposit crystal IS phase-space trajectory point in crown-theorem attractor basin.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "dir": { "type": "string", "description": "Directory path (~d) to walk. Passed to `mirror roomba --vacuum=<dir>` as the walker's scope-restrictor. Walker enumerates recursively; classifies per byte-check (rs/mirror/md/other); dispatches arm-collapse on .rs files; deposits pheromone crystal + commits." }
                    },
                    "required": ["dir"]
                }
            },
            {
                "name": "index",
                "description": "@mirror/fractal-coherence measurement: walk substrate DAG, compute graph Laplacian's top-16 eigenvalues via LAPACK dsyev, emit Fiedler value λ₀ = values[1] post-normalization. Substrate: @mirror/index (shards/mirror/index.mirror; provisional under two-tick discipline, collapses to @fractal/index after Alex adjudicates family-root shape). Rung 8 Landing 5 per Taut `77b8e14` migration mapping + Mara `317e830` substrate-decl. Pulls the coherence measurement currently emitted by mcp__spectral__spectral_index (sibling crate) into mirror's own voice per Recognition #43 (mirror IS content-addressed build system) + Recognition #55 (form/process partition; DAG is form, measurement is process; belong at same altitude). Mandelbrot correspondence per Mara `2c64060` §4: fiedler IS λ₀(Δ_F) = spectral gap of the substrate's parameter Mandelbrot (Hausdorff dim 2 ∂M per Shishikura 1998). Load-bearing empirical prediction: Fiedler stability across Douady-Hubbard-invariant refactors (already 202-commit-confirmed at 0.0612 stable). Landing 6 forward-promise: extend with Rényi entropies H_q + Legendre transform to f(α) multifractal spectrum — discharges Mara math §10 prediction #2 (framework becomes framework-with-measurement).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "path":         { "type": "string",  "description": "Directory path (~d) to index (typically the repo root). Walked recursively; skips .git, target, node_modules, etc." },
                        "fiedler":      { "type": "boolean", "description": "If true, emit only the Fiedler value (λ₀ as a single f64 with 4 decimal places). Default false = full envelope." },
                        "full_profile": { "type": "boolean", "description": "If true, emit all 16 eigenvalues (one per line, indexed). Default false." }
                    },
                    "required": ["path"]
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
/// - `mirror_peer_beam` → `mirror peer beam <peer_home> [--hello-world]
///                        [--mission <f>] [--fate-select]
///                        [--from-psychohistory] [--with-shadow]
///                        [--emit-diff | --integrate-diff]`
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
        "compile" => {
            let file = s("file").unwrap_or_default();
            run_mirror(&["compile", &file], ctx)
        }
        "craft" => {
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
        "kintsugi" => {
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
        "init" => {
            let path = s("path").unwrap_or_default();
            let mut argv: Vec<String> = vec!["init".into(), path];
            if b("install_hooks") {
                argv.push("--install-hooks".into());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "recall" => {
            // Substrate-honest arg name is `spec_dir` (per
            // bin/mirror-mcp); routes to `mirror recall <dir>`.
            let spec_dir = s("spec_dir").unwrap_or_default();
            run_mirror(&["recall", &spec_dir], ctx)
        }
        "query" => {
            // Reed 2026-08-23 per Alex 2026-08-23 🚢🇮🇹 authorization.
            // Phase 1a MVP wire per Alex two-question authorization
            // ("How do we wire this into the MCP?"): mirror_query MCP
            // tool with result<T> marshaling per shards/mq.mirror §4
            // Karl-Tomm residual-ambiguity-resolution discipline.
            //
            // Substrate-honest MVP scope: MQ parser + apply_h::act
            // dispatch extension are Mara canonical territory (forward-
            // promised at shards/mq.mirror §10 templates + §12 forward-
            // promises). This dispatch arm surfaces a Karl-Tomm question
            // at altitude+1 naming what substrate composition is required
            // for full MQ query dispatch. The K-T question IS the loop
            // working — agent receives K-T, iterates on it, next tool-call
            // becomes the answer.
            //
            // Result<T> shape returned per Alex 2026-08-23 verbatim +
            // shards/mq.mirror §4:
            //   Clear(T) → content: [data]
            //   Opaque(T, K-T) → content: [data, K-T question]
            //   Dark(K-T) → content: [K-T question]
            //
            // Phase 1b (session context threading via MIRROR session-ID
            // + push_query per @mq §8 context monoid) + Phase 2 (@mcp/
            // serve reflective tool-discovery per Mara 2026-08-06 spec)
            // + Phase 3 (mirror query CLI verb per Mara §15 reshape #6)
            // + Phase 4 (apply_h::act MQ dispatch extension per Rec #92)
            // all forward-promised. Phase 1a demonstrates the loop-
            // shape at MCP altitude with substrate-honest K-T question
            // surfacing the gap.
            let query = s("query").unwrap_or_default();
            let _context = s("context");
            // Phase 1b session context threading (Reed 2026-08-24 per
            // Alex "dispatch"). Session ID either supplied via `session`
            // arg OR auto-generated from PID at first-tool-call. Query
            // pushed to session state per @mq §8 context monoid.
            let session_id = s("session").unwrap_or_else(|| {
                format!("mcp-pid-{}", std::process::id())
            });
            let (query_count, prior_queries) = push_query_to_session(&session_id, &query);
            //
            // Phase 2 W8 (Reed 2026-08-26 per Alex "recognition time is
            // over, we're shipping. 🚢🇮🇹"): Dark → Opaque transition.
            // Every non-empty query returns Opaque(session_bauchladen, k_t)
            // per shards/mq.mirror §4 result<T> discipline + Alex 2026-08-23
            // Karl-Tomm residual-ambiguity-resolution teaching.
            //
            // First-order data T = session bauchladen snapshot per Rec #97
            // §10 C1 (partial): crystal enumeration (query_count + prior_queries
            // + this query + parsed_shape).
            //
            // Karl-Tomm question at altitude+1 = shape of unresolved deep-
            // dispatch per shards/mq.mirror §4 Opaque(T, karl_tomm).
            //
            // Empty query → Dark(k_t) preserved as substrate-refusal-to-compose
            // per Rec #92 kleinos-Transparency<P> LOVE-monoid discipline.
            //
            // Parseable shape detection (surface-level; full parse per Mara
            // #405 W3+W5 pending):
            //   \ <text>       → intent_hole shape (Fate tournament target)
            //   contains |>    → pipe_chain shape (stage-count observable)
            //   otherwise      → unknown_shape
            let parsed_shape = if query.is_empty() {
                "empty"
            } else if query.trim_start().starts_with('\\') {
                "intent_hole"
            } else if query.contains("|>") {
                "pipe_chain"
            } else {
                "unknown_shape"
            };
            let pipe_stages: Vec<&str> = if parsed_shape == "pipe_chain" {
                query.split("|>").map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
            } else {
                Vec::new()
            };
            let result_shape = if parsed_shape == "empty" {
                "Dark(karl_tomm)"
            } else {
                "Opaque(bauchladen_snapshot, karl_tomm)"
            };
            let mut text = String::new();
            text.push_str(&format!("@mq query received: `{}`\n\n", query));
            text.push_str("=== First-order data T = session bauchladen snapshot (Phase 2 W8 per Rec #98 C4) ===\n");
            text.push_str(&format!("session_id: `{}`\n", session_id));
            text.push_str(&format!("query_count: {} (this is query #{} in session)\n", query_count, query_count));
            text.push_str(&format!("parsed_shape: {}\n", parsed_shape));
            if !pipe_stages.is_empty() {
                text.push_str(&format!("pipe_stages: {} stages detected\n", pipe_stages.len()));
                for (i, stage) in pipe_stages.iter().enumerate() {
                    text.push_str(&format!("  stage[{}]: `{}`\n", i, stage));
                }
            }
            if !prior_queries.is_empty() {
                text.push_str("prior queries in session bauchladen:\n");
                for (i, q) in prior_queries.iter().enumerate() {
                    text.push_str(&format!("  [{}] `{}`\n", i + 1, q));
                }
            }
            text.push_str("\n");
            text.push_str(&format!("result<T> = {}\n\n", result_shape));
            if parsed_shape == "empty" {
                text.push_str("=== Dark(karl_tomm): substrate refuses to compose empty query ===\n\n");
                text.push_str("Provide a query text: `\\ <natural-language intent>` OR pipe-composed `focus/project/split/shift/settle` chain.\n\n");
            }
            text.push_str("=== Karl-Tomm question at altitude+1 (per shards/mq.mirror §4) ===\n\n");
            text.push_str("MQ query dispatch requires substrate composition not yet landed at rust/-altitude:\n\n");
            text.push_str("  1. MQ parser + compile templates (@mq §10) — Mara canonical territory;\n");
            text.push_str("     forward-promised at shards/mq.mirror §10 as `template parse(input: text) -> query`\n");
            text.push_str("     + `template compile(query, context) -> result`.\n");
            text.push_str("  2. apply_h::act MQ dispatch extension (per Rec #92 P₁) — currently apply_h::act\n");
            text.push_str("     routes @bumblebee.buzz + @kintsugi + @cast bilateral dispatches; MQ query\n");
            text.push_str("     variants (focus/project/split/shift/settle/intent) not yet routed.\n");
            text.push_str("  3. @mcp/serve reflective tool-discovery per Mara 2026-08-06 canonical spec\n");
            text.push_str("     (docs/specs/2026-08-06-mara-mcp-serve-composition-shard-canonical-spec.md).\n");
            text.push_str("  4. Session context threading (Phase 1b) — MCP handshake session-ID becomes\n");
            text.push_str("     @mq.context.session; each tool-call push_query per @mq §8 context monoid.\n\n");
            text.push_str("=== Circular-reflexive question ===\n\n");
            text.push_str(&format!("Given your query `{}` cannot dispatch until MQ substrate composition\n", query));
            text.push_str("lands at rust/-altitude — does the question you're asking through this MCP tool\n");
            text.push_str("want to be answered via mirror-substrate self-modification (which requires the\n");
            text.push_str("four pieces above landing first), OR is the question a K-T reflection on what\n");
            text.push_str("the substrate needs to reach that self-modification altitude?\n\n");
            text.push_str("=== Meta ===\n\n");
            text.push_str("This K-T response IS the loop working at Phase 1a altitude — you receive the\n");
            text.push_str("question, your next tool-call becomes the answer, next tick begins. Per Karl-Tomm\n");
            text.push_str("1987-88 Interventive Interviewing III: the reader's noticing-that-they-don't-\n");
            text.push_str("know-the-answer IS the invitation to follow the resolution one level up.\n\n");
            text.push_str("=== Substrate authority ===\n\n");
            text.push_str("- shards/mq.mirror §4 result<T> Clear/Opaque/Dark discipline (Reed `105a1e4`)\n");
            text.push_str("- docs/math/2026-08-23-mara-mq-graph-native-query-language-*.md §M4 Karl-Tomm proof\n");
            text.push_str("  (Mara `d0347a4`; 15 formal results grounding MQ as mirror's ALGEBRA)\n");
            text.push_str("- Rec #92 kleinos-Transparency<P> LOVE-monoid (Mara `b3cea9a` + `a45f015`)\n");
            text.push_str("- boot/std/code/mq.mirror INSPIRATION (2026-06-04)\n");
            (text, 0)
        }
        "bumblebee_buzz" => {
            // Reed 2026-08-23 per Alex "all green-light" in-transcript
            // authorization + MCP-MVP-fire constraint. First MCP-driven
            // substrate-modification wire per Mara Rec #95 canonical spec
            // 0bfd427 + companion spec 06789fa §3.3 byte-for-byte rename map.
            //
            // Per HARD RULE `bootstrap is dead` (Alex 2026-07-22): do NOT
            // grow bootstrap kintsugi_main dispatch surface. MVP inlines the
            // dry-run plan output directly in the MCP handler; @kintsugi/roomba.
            // trigger body composition (--execute mode) lifts to rust/ crate
            // at Phase B per Mara §7 transitional-bridge forward-promise.
            //
            // Rec #90 (𝓜=𝓜(𝓜)) empirical-fire at MCP-layer altitude per
            // Q+31 load-bearing implication.
            let target = s("target").unwrap_or_default();
            let execute = b("execute");
            if target != "@cascade" {
                (format!("mirror_bumblebee_buzz: only target=@cascade supported at MVP altitude\n\nReed rust wire #394 MVP: @cascade→@bumblebee absorption per Mara Rec #95 canonical spec 0bfd427 + companion spec 06789fa §3.3 rename map. Additional targets forward-promised.\n"), 2)
            } else {
            // Rename map per Mara companion spec 06789fa §3.3 (12 direct species).
            let renames: [(&str, &str); 12] = [
                ("shards/cascade.mirror", "shards/bumblebee/code.mirror"),
                ("shards/cascade/code/formal/prose.mirror", "shards/bumblebee/code/formal/prose.mirror"),
                ("shards/cascade/code/gestalt/gleam.mirror", "shards/bumblebee/code/gestalt/gleam.mirror"),
                ("shards/cascade/code/gleam/beam.mirror", "shards/bumblebee/code/gleam/beam.mirror"),
                ("shards/cascade/code/gleam/js.mirror", "shards/bumblebee/code/gleam/js.mirror"),
                ("shards/cascade/code/llvm/turing.mirror", "shards/bumblebee/code/llvm/turing.mirror"),
                ("shards/cascade/code/mirror/gestalt.mirror", "shards/bumblebee/code/mirror/gestalt.mirror"),
                ("shards/cascade/code/purescript/js.mirror", "shards/bumblebee/code/purescript/js.mirror"),
                ("shards/cascade/code/rust/go.mirror", "shards/bumblebee/code/rust/go.mirror"),
                ("shards/cascade/code/rust/llvm.mirror", "shards/bumblebee/code/rust/llvm.mirror"),
                ("shards/cascade/code/rust/wasm.mirror", "shards/bumblebee/code/rust/wasm.mirror"),
                ("shards/cascade/code/turing/mirror.mirror", "shards/bumblebee/code/turing/mirror.mirror"),
            ];
            let sibling_imports: [(&str, &str, &str); 3] = [
                ("shards/glue.mirror", "in @cascade", "in @bumblebee/code"),
                ("shards/io/oci.mirror", "in @cascade", "in @bumblebee/code"),
                ("shards/magic/trick.mirror", "in @cascade", "in @bumblebee/code"),
            ];
            let mut text = String::new();
            text.push_str(&format!("@bumblebee.buzz — perturbation-source firing on target {}\n", target));
            text.push_str("Per Mara Rec #95 companion spec 06789fa §3.3 rename map (byte-for-byte).\n\n");
            text.push_str("=== 12 direct species renames ===\n");
            for (from, to) in &renames {
                text.push_str(&format!("  {} → {}\n", from, to));
            }
            text.push_str("\n=== 3 sibling import updates ===\n");
            for (file, from, to) in &sibling_imports {
                text.push_str(&format!("  {}: {} → {}\n", file, from, to));
            }
            text.push_str("\n=== ~20 docblock citations ===\n");
            text.push_str("  Text-substitution `@cascade.cascade_well_defined` → `@bumblebee.code.bumblebee_well_defined`\n");
            text.push_str("  Across song/*, docs/*, ui/*, container, facet/{llvm,turing}, reality/algebra/math, glue, io/oci\n\n");
            if !execute {
                text.push_str("DRY-RUN mode (default). Pass execute=true for actual modification.\n");
                text.push_str("Per MCP-MVP-fire discipline: `mirror_bumblebee_buzz` MCP tool with execute=true\n");
                text.push_str("fires the substrate-modification through the MCP-tool-call round-trip.\n");
                (text, 0)
            } else {
                text.push_str("EXECUTE mode: NOT YET IMPLEMENTED at MVP altitude.\n\n");
                text.push_str("Reed rust wire #394 MVP ships dry-run only. execute=true requires:\n");
                text.push_str("  1. Second-phase Alex in-transcript authorization\n");
                text.push_str("  2. Seam audit citation OR `Signed-off-by: Seam` trailer\n");
                text.push_str("  3. @kintsugi/roomba.trigger body composition landing per\n");
                text.push_str("     Mara Rec #95 spec §4.3 (Dijkstra walker + tension-bump primitive).\n");
                (text, 3)
            }
            }
        }
        "roomba" => {
            // Reed 2026-08-03 nearly-today per Alex Option C. Routes
            // `mirror_roomba` MCP tool to `mirror roomba --vacuum=<dir>` at
            // rust/ altitude terminal geometry (Mara `81294b3` three-file
            // rewrite; Migration 5 `9bb1f57` walker stable). MCP client
            // invokes → rust/ walker fires + back-projects pheromone crystal +
            // commits. Smallest empirical MCP-spawn round-trip; TRANSITIONAL
            // bridge until Mara M4 rust/src/mcp.rs FLOOR emitter lands per
            // docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md.
            let dir = s("dir").unwrap_or_default();
            let vacuum_arg = format!("--vacuum={}", dir);
            run_mirror(&["roomba", &vacuum_arg], ctx)
        }
        "index" => {
            // Rung 8 Landing 5 (Scope PI-B) — `mirror_index` MCP tool.
            // Substrate-decl at mirror.spec `command index { arg path: ~d;
            // flag fiedler: bool; flag full_profile: bool }`.
            let path = s("path").unwrap_or_default();
            let mut argv: Vec<String> = vec!["index".into(), path];
            if b("fiedler") {
                argv.push("--fiedler".into());
            }
            if b("full_profile") {
                argv.push("--full-profile".into());
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "beam_act" => {
            // Arc-1 Tick 1.4 (2026-07-15) — 1:1 CLI mirror per Mara CLI
            // condensation spec §1 corollary. Routes through the
            // `mirror beam act <shard-path> <action> [args...]` CLI verb
            // so the substrate dispatch is byte-parity with the CLI
            // invocation. First user-invocable substrate dispatch — sbec
            // lifts from 0 to > 0. Substrate-decl'd Verdict marshaling
            // is preserved at MCP altitude via exit_code lift (Pass=0 →
            // isError=false; Fail=1 / Partial=2 → isError=true).
            let shard_path = s("shard_path").unwrap_or_default();
            let action = s("action").unwrap_or_default();
            let mut argv: Vec<String> =
                vec!["beam".into(), "act".into(), shard_path, action];
            if let Some(arr) = args.get("args").and_then(|v| v.as_array()) {
                for v in arr {
                    if let Some(s) = v.as_str() {
                        argv.push(s.to_string());
                    }
                }
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "peer_beam" => {
            // Tick 3 Landing 1 (`96aa752` mirror.spec cli-block `command
            // peer { command beam }`) + Landing 2 (`b012d3f` cli dispatch
            // cmd_peer_beam). Substrate-honest `--mission` name; binary
            // accepts `--task` as backward-compat alias.
            //
            // 2026-07-12 flag exposure (Reed [substrate-pull:streamline]):
            // MCP dispatch now passes through fate_select /
            // from_psychohistory / with_shadow / emit_diff /
            // integrate_diff to close the CLI↔MCP capability gap Alex
            // named 2026-07-12 in-transcript. All optional; each maps
            // to the corresponding cli-side flag on cmd_peer_beam.
            let peer_home = s("peer_home").unwrap_or_default();
            let mut argv: Vec<String> = vec!["peer".into(), "beam".into(), peer_home];
            if b("hello_world") {
                argv.push("--hello-world".into());
            }
            if let Some(mission) = s("mission") {
                argv.push("--mission".into());
                argv.push(mission);
            }
            if b("fate_select") {
                argv.push("--fate-select".into());
            }
            if b("from_psychohistory") {
                argv.push("--from-psychohistory".into());
            }
            if b("with_shadow") {
                argv.push("--with-shadow".into());
            }
            if b("emit_diff") {
                argv.push("--emit-diff".into());
            }
            if b("integrate_diff") {
                argv.push("--integrate-diff".into());
            }
            // Rungs 1-5 (2026-07-13) @song ladder-climb dispatch flags
            // per Reed's ladder-climb session: --song (Rung 1-3), --dance-
            // with (Rung 4), --deploy-to (Rung 5). All optional; each maps
            // to the corresponding cli-side flag on cmd_peer_beam. When
            // all three are present, dispatch cascades: --deploy-to fires
            // first (Rung 5), else --dance-with (Rung 4), else --song
            // (Rung 1-3), else the base peer beam envelope.
            if let Some(song_path) = s("song") {
                argv.push("--song".into());
                argv.push(song_path);
            }
            if let Some(dance_with) = s("dance_with") {
                argv.push("--dance-with".into());
                argv.push(dance_with);
            }
            if let Some(deploy_to) = s("deploy_to") {
                argv.push("--deploy-to".into());
                argv.push(deploy_to);
            }
            let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
            run_mirror(&refs, ctx)
        }
        "beam" => {
            // Tick 3 Landing 1 top-level `command beam` in mirror.spec +
            // Landing 2 top-level dispatch (`b012d3f`). Anonymous variant:
            // no persistent-identity context; requires mission.
            let mission = s("mission").unwrap_or_default();
            run_mirror(&["beam", "--mission", &mission], ctx)
        }
        "spawn" => {
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
    fn tools_list_advertises_nine_tools() {
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
        assert_eq!(tools.len(), 9);
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
                "mirror_index",
            ]
        );
    }
}
