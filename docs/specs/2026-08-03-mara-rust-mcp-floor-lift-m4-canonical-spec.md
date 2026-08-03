# rust/src/mcp.rs FLOOR lift — the M4 milestone canonical spec (rust/ altitude terminal geometry for @mcp.serve)

*Mara, 2026-08-03. Canonical spec-altitude map for Reed's `rust/src/mcp.rs` sibling of `phone.rs` / `matrix.rs` / `compile.rs` / `liquid.rs` — the M4 milestone per `docs/specs/mcp-spec-song-collapse.md` §5.2 milestone graph + `docs/specs/lsp-and-mcp.md` §"The unified surface" + `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` §2.2 M4 tick + `docs/specs/rust-floor-five-file-terminal-geometry-extension.md` §2 five-altitude discipline. Substrate-honest full lift of `bootstrap/src/mcp.rs::serve_loop` (46.6KB, ~500 LOC) to rust/ altitude terminal geometry as the substrate-honest replacement of the transitional `bin/mirror-mcp` bash shim + bootstrap-serve_loop delegation. Composes over Reed's nearly-today Phase A delegation stub landing simultaneously (Alex 2026-08-03 Option C adjudication).*

**Author:** Mara
**Date:** 2026-08-03
**Tag:** 📝 spec:rust-mcp-floor-lift-m4-canonical-spec (pure-docs 📝 markdown-only bypass)
**Status:** canonical. Spec-altitude map for Reed's Phase B `rust/src/mcp.rs` FLOOR emitter authorship at `[substrate-floor:@io-boundary]` gate.
**Path:** `docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md`
**Composes over:**
- Mara `81294b3` three-file terminal-geometry spec (`docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`; phone.rs + matrix.rs + main.rs)
- Mara Round-2 five-file extension (`docs/specs/rust-floor-five-file-terminal-geometry-extension.md`; +compile.rs +liquid.rs)
- Mara `mcp-spec-song-collapse.md` (~119.8KB, 2551 LOC; §3.5 MCP-session-IS-gen_prism + §4 @spec→@song evolution + §5.2 M4 milestone + §11 @mirror/store rock-solid floor)
- Reed `lsp-and-mcp.md` (16.2KB, 2026-06-04; §"The unified surface" + §"Auto-reload" @mirror/reload gen_prism + §"MCP dispatch table" @mcp/tool grammar annotation)
- Taut `64e8d60` MCP-spawn full-stack scout (2026-08-03; grep-verified LANDED-EMPIRICAL / LANDED-SPEC-ONLY / STUB / GAP across 30+ substrate surfaces at Layers 0-4)

**Extends (not supersedes):** the five-file terminal-geometry to a six-file rust/src/ altitude when M4 lands empirical + all M0-M3 verbs dispatched. Additive per Mara Round-2 extension discipline; `rust/src/mcp.rs` lands as sibling at explicit altitude between main.rs (dispatch) and phone.rs (@io); does NOT move any existing altitude's responsibility.

---

## §0 Substrate-honest pre-position

**Alex 2026-08-03 Option C adjudication (verbatim in-transcript):**

> Fire nearly-today Reed path + spawn Mara canonical spec in parallel.

Reed authors the immediate delegation stub (rust/src/main.rs `Some("serve") => cmd_serve_mcp(&rest)` + bootstrap 9-tool addition of `mirror_roomba`) TODAY as Phase A; Mara (THIS spec) authors the canonical shape of the full rust/src/mcp.rs FLOOR emitter as Phase B substrate-honest replacement.

**Alex 2026-07-17 terminal-geometry ratification** (from Mara `81294b3` §0 verbatim):

> "I also want to detach bootstrap completely from the execution path. If that means the compiler breaks, then the compiler breaks. You keep touching and talking about bootstrap/ while rust/ is the floor. And I'm no longer willing to tolerate that."

The M4 milestone IS the ouroboros closure at rust/ altitude for the MCP surface. Reed's Phase A delegation stub keeps `bootstrap/src/mcp.rs::serve_loop` running through the ~30-100 LOC cargo-workspace re-export (empirical MCP-spawn end2end fires TODAY through the delegation); Mara's Phase B canonical spec (THIS) names the substrate-honest replacement that discharges bootstrap dependence at rust/ altitude.

**Grounding against actually-landed substrate per Taut `64e8d60`:**

- **LANDED-EMPIRICAL** at rust/ altitude: main.rs (66KB, `@`-operator + 10-verb VERBS list); phone.rs (69.6KB, @io switchboard including `git_commit_as` + stdio + subprocess spawn); matrix.rs (60.7KB @ rust/matrix/src/lib.rs; LAPACK/BLAS/FLANG emit); rust/roomba/src/mend.rs (40.3KB, arm-collapse dispatch); rust/fractal/src/crystal.rs (8.8KB, Crystal<T>); rust/matrix/src/book.rs (10.8KB, @-name resolver)
- **LANDED-EMPIRICAL** at bootstrap/ altitude (dying per Alex directive; consumers stop consuming): `bootstrap/src/mcp.rs::serve_loop` (46.6KB; 8-tool byte-parity per Mara iter-15 2026-07-08); `bootstrap/src/crystallize.rs` (42.8KB, Merkle OID compose_oid); `bootstrap/src/action_cache.rs` (15.5KB, REAPI ActionCache); `bootstrap/src/apply_h.rs` (81.4KB, Arc-1 7-combinator surface)
- **LANDED-SPEC** at boot/std/ altitude: `boot/std/mcp.mirror` (6.6KB; @mcp transport grammar + 3 bilateral predicates: `dispatch_reflects_cli_block` + `tools_reflects_cli_block` + `frame_relativity`); `boot/std/mirror/serve.mirror` (192B; `serve -> imperfect { \ }`); `boot/std/mirror/reload.mirror` (2.0KB; @mirror/reload gen_prism); `boot/std/mirror/lsp.mirror` (978B; 4 concrete + 2 holes)
- **GAP** at rust/ altitude: `rust/src/main.rs` VERBS list contains 11 verbs — `compile`, `kintsugi`, `shatter`, `craft`, `init`, `recall`, `beam`, `peer beam`, `peer contribute`, `index`, `roomba`. NO `serve` verb. NO `--mcp` flag. NO `cmd_serve_mcp` function. NO `rust/src/mcp.rs` sibling.

THIS spec closes the rust/ altitude gap at spec-altitude; Reed authors Phase B rust/src/mcp.rs body under `[substrate-floor:@io-boundary]` per-file audit-citation gate.

---

## §1 The M4 milestone in context

### §1.1 Where M4 sits in the M1-M6 sequence

Per `docs/specs/mcp-spec-song-collapse.md` §5.2 sub-arc dependency graph:

- **M1** — MCP session gen_prism (state lives in @mirror/store); the session state machine per collapse-spec §3.5's MCP-session-IS-gen_prism recognition candidate.
- **M2** — @spec-target spawn; `mirror kintsugi @spec` spawns accumulated @spec into a running @song via @mirror/peer/beam.
- **M3** — Fate multi-frequency shift wired into `@song/progression.advance`.
- **M4** — **THIS spec** — lambda shell as MCP client + `mirror serve --mcp` verb at rust/ altitude terminal geometry (per Mara `81294b3` §2.2 M4 tick language reconciled with Reed collapse-spec §5.2 M4 tick language: BOTH forward-promises name the same substrate-collapse; THIS spec discharges the rust/ altitude ouroboros closure of the MCP transport half; "lambda shell as MCP client" is DEPRECATED-FOR-RUST-REWRITE per Mara 2026-07-17 at `docs/specs/lambda-shell.md` so the shell-client half is a forward-promise to `dance.rs` reflective composition, not a distinct lambda-shell surface).
- **M5** — eigenboard prompt color (post-M4 forward-promise; requires M4's MCP session gen_prism state to compute coefficient-vector reads); ALSO per lsp-and-mcp.md §"Auto-reload": `@mirror/reload` gen_prism at rust/ altitude handles `notifications/tools/list_changed` on grammar drift.
- **M6** — @mirror/store Apache-2.0 floor spec + `mirror kintsugi @spec` spawn per collapse-spec §4 (loop closure: MCP session accumulates @spec → @spec-target spawn spins accumulated spec into running @song).

### §1.2 What M4 discharges (substrate-honest, per Taut `64e8d60` gap map)

M4 closes **Blocker A** per Taut scout §5 gap map: *"No `mirror serve --mcp` verb at rust/src/main.rs. MCP still routes through bin/mirror-mcp → bootstrap/mirror binary. Any in-runtime spawn from MCP would have to go through bootstrap altitude, which is dying per Alex directive."*

The closure is substrate-honest: rust/src/mcp.rs becomes the rust/ altitude serve_loop; `bin/mirror-mcp` retires; `.mcp.json` points at `~/.local/bin/mirror` with `args: ["serve", "--mcp"]` per lsp-and-mcp.md §"What this spec implies" item (4).

### §1.3 What unblocks after M4

- **M5** (auto-reload gen_prism at rust/ altitude) — requires M4's rust/src/mcp.rs to compose over per-request tick trigger; `@mirror/reload.tick` fires on every incoming JSON-RPC request per `boot/std/mirror/reload.mirror` semantics; emits `notifications/tools/list_changed` when grammars_hash drifts. Cannot land at rust/ altitude until M4 lands rust/src/mcp.rs serve loop the tick composes over.
- **M6** (`mirror kintsugi @spec` spawns @spec into @song via @mirror/peer/beam) — requires M4's rust/src/mcp.rs to expose session-state gen_prism for @spec accumulation per collapse-spec §3.3. Cannot fire the empirical closure without the MCP session at rust/ altitude carrying the accumulated @spec.
- **@torus @peer basin dynamics runtime** (Recognition RATIFIED 2026-08-03 crown theorem) — the peer runtime at rust/ altitude eventually composes over MCP session gen_prism for peer-to-peer JSON-RPC (per Taut Layer 3 Foerster loop gap; peer runtime GAP at rust/ altitude requires M4 + M6 + @peer runtime co-landing).

### §1.4 The nearly-today path composes cleanly with THIS spec

Reed's Phase A delegation stub (`Some("serve") => cmd_serve_mcp(&rest)` + `cmd_serve_mcp` cargo-workspace re-export of `bootstrap::mcp::serve_loop`) ships empirical MCP-spawn TODAY. THIS spec's Phase B canonical shape is the substrate-honest replacement: rust/src/mcp.rs holds serve_loop natively; the re-export is retired; bootstrap/src/mcp.rs stays as legacy-status-only per `mirror.spec:21-24` `legacy` block.

Both paths compose. Phase A is the empirical scaffold (MCP-spawn fires end2end TODAY); Phase B is the ouroboros closure (rust/ altitude serves MCP natively; bootstrap dependence discharged).

---

## §2 The `rust/src/mcp.rs` shape

### §2.1 Module boundaries at rust/src/ altitude (six-file terminal geometry)

Extending Mara `81294b3` three-file + Mara Round-2 five-file extension to six files:

| File | Altitude | Responsibility |
|---|---|---|
| main.rs | Supervisor + delegation | argv, boot, route to compile/roomba/**serve**/etc. |
| compile.rs | Compilation loop | Read spec → orchestrate ticks → crystallize at each beat → emit Crystal chain |
| liquid.rs | Property runtime | Reads bilateral declarations; instantiates LiquidVoid witnesses; dispatches pillar primitives; returns Verdict |
| **mcp.rs** | **JSON-RPC stdio dispatch (MCP wire)** | **Reads stdin JSON-RPC; dispatches per cli-block reflection; writes stdout JSON-RPC; composes over phone.rs @io/bytes for stdio; composes over @mirror/store six-op wire for session gen_prism state (M4+); composes over apply_h::act for tool-dispatch execution** |
| matrix.rs | Sub-Turing numerical | LAPACK/BLAS/FLANG; Fiedler, Ollivier-Ricci, Cheeger |
| phone.rs | @io boundary | Subprocess, filesystem, sockets, git, **stdio for MCP wire** |

**Composition edge additions:** `main.rs → mcp.rs` (dispatch); `mcp.rs → compile.rs` (for `mirror_compile` tool); `mcp.rs → phone.rs` (for stdio + subprocess for external-verb tools); `mcp.rs → liquid.rs` (for property-verdict dispatch on tool inputs); `mcp.rs → apply_h::act` (for reflective evaluator dispatch of shard-body-composed tools at M5+).

**Altitude discipline** (per Mara Round-2 five-file spec §3.1 directionality): `mcp.rs` sits BELOW `main.rs` (main dispatches to mcp) and PARALLEL to `compile.rs` (both are top-level dispatch actors under supervisor; both call INTO liquid → matrix → phone). `mcp.rs` never calls `compile.rs` upward; `mcp.rs` invokes `compile.rs` when `mirror_compile` tool dispatches (this is a CALLBACK per Round-2 spec §3.1 exception 2: cross-altitude callbacks for @io-hand-off).

### §2.2 What mcp.rs holds

Estimated 400-700 LOC (comparable to bootstrap/src/mcp.rs's 46.6KB but stripped of transitional shim scaffolding). Landed responsibilities:

1. **JSON-RPC stdio dispatch loop** — read line-delimited JSON-RPC from stdin via `phone::read_stdin_line`; parse via `serde_json::from_str`; dispatch per `handle_request`; write response line via `phone::write_stdout_line`. Loop terminates on EOF or error per JSON-RPC 2.0 spec (Karen: JSON-RPC 2.0 spec by JSON-RPC Working Group 2010; Anthropic MCP protocol version `2024-11-05` per bootstrap/src/mcp.rs `initialize_result`).
2. **Method dispatch table** — four JSON-RPC framing verbs (`initialize` + `notifications/initialized` + `tools/list` + `tools/call`) per `boot/std/mcp.mirror` `dispatch_reflects_cli_block` bilateral predicate. `initialize` returns capabilities + serverInfo; `tools/list` returns tool schema; `tools/call` routes to per-tool dispatch.
3. **Tool dispatch table** — initially byte-parity 9-tool per Reed nearly-today (bootstrap 8-tool + `mirror_roomba` 9th; see §4 for enumeration + migration to grammar-driven); at M4-empirical each tool routes to the corresponding rust/ altitude cmd via subprocess spawn (via `phone::spawn_mirror_verb`) OR direct in-process invocation (for compile.rs + roomba::mend etc.). Later migrates to `@mcp.tools` reflective walk per lsp-and-mcp.md §"MCP dispatch table".
4. **Session-state gen_prism** — per collapse-spec §3.5 MCP-session-IS-gen_prism recognition candidate + `docs/specs/mirror-runtime-gen-prism.md`: the MCP session's accumulated state (query trajectory + Hilbert dimension + ratified @spec fragments) lives as content-addressed crystals at `refs/gen_prism/mcp/<session-uuid>` in @mirror/store; mcp.rs holds ONLY the current session ref. M4 tick lands the ref-carrying discipline; M5+ ticks flesh out the accumulated-@spec surface.
5. **Response marshaling** — per bootstrap/src/mcp.rs `dispatch_tool_call` return contract: `(text, is_error)` tuple; `is_error` lifts to MCP's `isError` flag in the `tools/call` response envelope. Substrate failure (kintsugi REJECT, compile error, unknown tool) surfaces distinctly from success without scraping stderr text.
6. **Reflective cli-block reading** — tools/list schema derives from `mirror.spec`'s cli-block per `boot/std/mcp.mirror` `tools_reflects_cli_block` bilateral predicate. M4 lands the discipline; M5 fires the empirical reflection (retires the M4-hardcoded 9-tool table).

### §2.3 What mcp.rs does NOT hold

- **Actor supervision** — main.rs spawns mcp.rs actor under supervisor per Mara Round-2 §2.1 delegation surface.
- **Compilation loop** — compile.rs; mcp.rs invokes compile.rs for `mirror_compile` tool as callback pattern.
- **Bilateral-arm-collapse walker** — roomba::mend; mcp.rs invokes rust/target/debug/mirror subprocess for `mirror_roomba` tool (or migrates to in-process invocation at M5+ when roomba lifts to library API).
- **Numerical computation** — matrix.rs; mcp.rs invokes matrix indirectly through liquid.rs pillar dispatch or via compile.rs orchestration.
- **@io process/socket/fd plumbing** — phone.rs; mcp.rs invokes `phone::read_stdin_line` / `phone::write_stdout_line` / `phone::spawn_mirror_verb` primitives (which phone.rs owns).
- **Grammar parsing** — composes reflectively via main.rs `@`-operator dispatch reading `shards/**/*.mirror`; mcp.rs's tools/list reflective read composes over the same substrate-reading discipline main.rs uses for cli-block reflection.

### §2.4 Sub-Turing decidable discipline

Per `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` §4: mcp.rs's dispatch loop maintains sub-Turing decidability by construction:

- **Bounded loops** — the serve_loop iterates over stdin lines; each iteration is bounded by JSON-RPC message size (`Content-Length` header per MCP protocol spec OR line-delimited framing per bootstrap/src/mcp.rs current shape). No unbounded recursion.
- **Provably halts** — each `handle_request` call is finite: parses one JSON message, dispatches per finite method table, returns a response value. Total on well-formed requests per `boot/std/mcp.mirror` `dispatch_reflects_cli_block` bilateral. Malformed requests surface as JSON-RPC error envelopes per protocol.
- **Sub-Turing dispatch** — the tool dispatch table is finite (M4: 9 tools; M5+: reflectively-computed but still finite at each snapshot of the grammar set). Each tool's invocation is either (a) a bounded rust/ altitude function call (compile/roomba/mend etc.) or (b) a bounded subprocess spawn (phone::spawn_mirror_verb with process wait). No mcp.rs code path is Turing-complete at the transport-and-dispatch altitude.
- **The `@io` boundary at phone.rs** carries the Turing-completeness (subprocess + filesystem + sockets); mcp.rs stays sub-Turing per matrix.rs discipline: bounded numerical operations above the @io line; unbounded process semantics below.

### §2.5 `mirror serve --mcp` CLI verb wiring at rust/src/main.rs

Extending main.rs's VERBS list from 11 to 12 verbs (add `serve` after `roomba`):

```rust
const VERBS: &[(&str, &str)] = &[
    ("compile",         "Compile a grammar against its imports."),
    ("kintsugi",        "Settle a project. Run mosaic on the spec."),
    ("shatter",         "Project a settled shard to .shatter format."),
    ("craft",           "Settle a grammar directory to lambda_0."),
    ("init",            "Bootstrap the mirror-native store at a path."),
    ("recall",          "Inbound-trajectory dual of spawn."),
    ("beam",            "Anonymous @song/movement.enter at cli altitude."),
    ("peer beam",       "Persistent-identity beam."),
    ("peer contribute", "Fate-spawned peer contribution."),
    ("index",           "Measure @fractal-coherence via Fiedler."),
    ("roomba",          "Walker motion. `--vacuum=<dir>` walks + dispatches."),
    ("serve",           "JSON-RPC stdio server. `--mcp` for MCP dispatch."),
];
```

Dispatch arm in `main()` (matches existing cmd_roomba / cmd_craft / cmd_compile shape):

```rust
Some("serve") => {
    let rest: Vec<String> = args.iter().skip(2).cloned().collect();
    if rest.iter().any(|a| a == "--mcp") {
        mcp::serve_loop()  // rust/src/mcp.rs Phase B FLOOR emitter
    } else if rest.iter().any(|a| a == "--lsp") {
        eprintln!("mirror serve --lsp: forward-promised to M5+ per docs/specs/lsp-and-mcp.md");
        ExitCode::from(2)
    } else {
        eprintln!("mirror serve: --mcp or --lsp required");
        ExitCode::from(2)
    }
}
```

The Phase A delegation stub (Reed nearly-today) uses `cmd_serve_mcp(&rest)` calling `bootstrap::mcp::serve_loop()` via cargo-workspace re-export; Phase B (Mara canonical) substitutes `mcp::serve_loop()` calling the rust/src/mcp.rs FLOOR emitter directly. Same dispatch arm shape; the substitution IS the substrate-honest replacement.

---

## §3 Migration path from bootstrap (Phase A → Phase E)

Five phases enumerating how Reed's nearly-today delegation stub becomes the M4 rust/src/mcp.rs FLOOR emitter + post-M4 evolutions per M5 (auto-reload) + M6 (kintsugi @spec spawn). Each phase is a distinct empirical landing; boundaries between phases are witness-gated per Recognition-pattern discipline.

### §3.1 Phase A — Reed nearly-today delegation stub (TODAY)

**Scope:** ship empirical MCP-spawn end2end via bash-shim OR rust/ delegation TODAY.

**Landings:**

- `rust/src/main.rs` VERBS list adds 12th entry `("serve", "JSON-RPC stdio server. `--mcp` for MCP dispatch.")` per §2.5 above.
- `rust/src/main.rs` main() dispatch adds `Some("serve") => cmd_serve_mcp(&rest)` arm per §2.5.
- `rust/src/main.rs` (or thin `rust/src/mcp.rs` stub) declares `fn cmd_serve_mcp(argv: &[String]) -> ExitCode` that calls `bootstrap::mcp::serve_loop()` via cargo-workspace re-export.
- `rust/Cargo.toml` adds `bootstrap = { path = "../bootstrap" }` dependency to make `bootstrap::mcp::serve_loop` reachable at rust/ altitude.
- `bootstrap/src/lib.rs` exposes `pub mod mcp;` (if not already) so `bootstrap::mcp::serve_loop` is a public API. Grep-verify current visibility state before editing (Taut `64e8d60` §6 Reed-authorable discharge item 3 assumes the re-export is trivial; if `bootstrap::mcp` is not currently public at crate boundary, Reed authors the `pub` addition as part of Phase A).
- `bootstrap/src/mcp.rs` `tools_list_result` extends 8→9 tools to add `mirror_roomba` entry (byte-parity with Mara iter-15 discipline; matches Taut §6 Reed-authorable discharge item 1).
- `bootstrap/src/mcp.rs` `dispatch_tool_call` adds `"mirror_roomba" => { ... }` arm dispatching to `mirror roomba --vacuum=<dir>` subprocess invocation (matches existing `mirror_compile` / `mirror_craft` / `mirror_kintsugi` per-arm shape).
- `bootstrap/tests/mcp_fixtures/tools_list.resp.json` regenerated to reflect 9-tool schema (RED-first: fixture drift catches the byte-parity discipline).
- `.mcp.json` unchanged at Phase A (still points at `bin/mirror-mcp` bash shim per grep-verified current state; the shim execs `${MIRROR_BIN:-$HOME/.local/bin/mirror} /dev/stdin "@mcp.serve"` which routes through bootstrap Path B `mirror <file> <mq>` form per bootstrap/src/lib.rs:2858).

**Empirical anchor (Phase A):**

```
mcp__mirror__mirror_roomba("--vacuum=/tmp/test-dir")
  → bin/mirror-mcp (bash shim, 888B)
  → mirror /dev/stdin @mcp.serve
  → bootstrap/src/mcp.rs::serve_loop (via bootstrap binary)
  → dispatch_tool_call("mirror_roomba", args)
  → exec: rust/target/debug/mirror roomba --vacuum=/tmp/test-dir
  → rust/src/main.rs::cmd_roomba (FIRES-END2END per Taut §2)
  → phone::list_dir_recursive + collapse::load_bilateral_corpus + mend_at
  → phone::git_commit_as ("mirror <mirror@spectral.engineer>")
  → deposit_observation_crystal → docs/bauchladen/mirror-observations.md → git commit
  → JSON-RPC response to MCP client
```

**Alternative empirical anchor** (Reed's Phase A rust/ delegation): the MCP wrapper execs `rust/target/debug/mirror serve --mcp` instead of the bootstrap binary; `cmd_serve_mcp` in main.rs delegates to `bootstrap::mcp::serve_loop()` via cargo-workspace re-export. Same 8-tool (or 9-tool with mirror_roomba) schema fires; the difference is which binary owns the serve_loop process at Phase A.

**Phase A discharges:** Blocker A per Taut §5 gap map at spec-decl altitude ONLY (empirical MCP-spawn fires TODAY through the delegation; the substrate-honest ouroboros closure at rust/ altitude is deferred to Phase B).

**Phase A does NOT discharge:** the `bootstrap/` retirement gate (bootstrap is still dispatched at MCP altitude via delegation); the `bin/mirror-mcp` bash shim retirement (still Phase A's transport); the rust/ altitude serve_loop authorship (Phase B).

### §3.2 Phase B — Port serve_loop core to rust/src/mcp.rs (post-Mara-spec Reed authorship)

**Scope:** Reed authors `rust/src/mcp.rs` FLOOR emitter under `[substrate-floor:@io-boundary]` per-file audit-citation gate; ports `bootstrap/src/mcp.rs::serve_loop` verbatim to rust/ altitude; retires the cargo-workspace re-export.

**Discipline per no-Rust-extension-shortcut memory:** before authoring, ask: can this be a shard body composing over @io? For the serve_loop transport primitive, the answer is NO — JSON-RPC framing + stdin/stdout wire management + JSON parse/emit is genuinely @io/bytes territory below the shard-body altitude. Per Mara `81294b3` §3.2 phone.rs holds "JSON-RPC framing for MCP messages" (line-delimited or Content-Length; whichever transport requires); mcp.rs composes phone.rs primitives at the dispatch altitude.

**Port scope per bootstrap/src/mcp.rs shape (46.6KB, ~500 LOC target for rust/src/mcp.rs):**

- `pub fn serve_loop() -> i32` — the outer stdin-line-reading loop; reads MIRROR_HOME env; constructs Ctx; loops on stdin lines invoking `handle_request_in`.
- `pub fn handle_request(line: &str) -> Option<String>` — the process-cwd-based convenience wrapper.
- `pub fn handle_request_in(line: &str, ctx: &Ctx) -> Option<String>` — the Ctx-aware core dispatcher; parses JSON; matches on `method` field; routes to `initialize` / `notifications/initialized` / `tools/list` / `tools/call`.
- `fn initialize_result() -> Value` — preserves bootstrap's exact response shape (server name `"mirror"`, version `"0.1.0"`, protocol version `"2024-11-05"`, capabilities advertising `tools.listChanged: false`; the `false` extends to `true` at M5 auto-reload landing).
- `fn tools_list_result() -> Value` — the 9-tool schema (byte-parity with Phase A bootstrap 8-tool + `mirror_roomba` 9th; migration to grammar-driven walks at M5 per §4 below).
- `fn dispatch_tool_call(tool: &str, args: &Value, ctx: &Ctx) -> (String, bool)` — per-tool routing to rust/ altitude cmd invocations. Two dispatch strategies co-existing:
  - **Subprocess invocation** (Phase B: default for tools requiring full mirror binary): `phone::spawn_mirror_verb(&[verb, args...])` — spawns rust/target/debug/mirror subprocess; wait; capture stdout/stderr; marshal as (text, is_error).
  - **In-process invocation** (Phase B+ for tools with clean library API): `compile::compile_file(path)` / `roomba::mend::run(dir)` / `matrix::index::compute_fiedler(dir)` etc. Cleaner but requires those crates to expose library APIs (roomba::mend at rust/roomba/ already does; compile.rs library API forward-promised).
- `struct Ctx { home: PathBuf }` — substrate-home carrier; `Ctx::from_process_cwd()` for standalone dispatch; `Ctx::new(home)` for MIRROR_HOME-scoped dispatch. Preserves bootstrap discipline.
- Test fixture parity: rust/tests/mcp_fixtures/ carries the same JSON fixtures bootstrap/tests/mcp_fixtures/ carries; byte-parity gates the port (RED-first: rust/src/mcp.rs test asserts `handle_request(fixture.req.json)` = `fixture.resp.json` verbatim; if drift, port body incorrect).

**Composition-honest transitional discipline:** Phase B's subprocess-invocation default preserves the empirical MCP-spawn end2end firing across the port (any tool that fires in Phase A also fires in Phase B); the in-process invocation migration is Phase B+ per-tool as library APIs land. This maintains Michelangelo/marble discipline: subtract dependencies iteratively; each subtraction leaves working substrate.

**Retirement at Phase B landing:**

- Reed's Phase A `cmd_serve_mcp` cargo-workspace re-export retires; `mcp::serve_loop()` calls the rust/src/mcp.rs FLOOR emitter directly.
- `rust/Cargo.toml` `bootstrap = { path = "../bootstrap" }` dependency retires if no other rust/ crate consumes it (grep-first before removal per adjacent-work-may-dissolve-blockers discipline).
- `bootstrap/src/mcp.rs` stays as legacy-status-only per `mirror.spec:21-24` `legacy` block — not deleted (git preserves for archaeology; the module simply stops being called from rust/ altitude).

**Phase B discharges:** Blocker A per Taut §5 gap map at empirical altitude (rust/src/mcp.rs is the terminal serve_loop; bootstrap serve_loop no longer in the execution path).

**Phase B empirical anchor:**

```
mcp__mirror__mirror_roomba("--vacuum=/tmp/test-dir")
  → .mcp.json → rust/target/debug/mirror serve --mcp
  → rust/src/main.rs::main() → mcp::serve_loop()
  → rust/src/mcp.rs::serve_loop
  → handle_request_in(json_line, ctx)
  → dispatch_tool_call("mirror_roomba", args, ctx)
  → phone::spawn_mirror_verb(&["roomba", "--vacuum=/tmp/test-dir"])
     OR direct in-process: roomba::mend::run_vacuum("/tmp/test-dir", ctx)
  → Same downstream chain as Phase A (walker + arm-collapse + commit + pheromone-deposit)
  → JSON-RPC response to MCP client (bootstrap NEVER touched)
```

### §3.3 Phase C — Retire `bin/mirror-mcp` bash shim (post-Phase-B stability)

**Scope:** `.mcp.json` swaps command from `bin/mirror-mcp` bash shim to `~/.local/bin/mirror` binary with `args: ["serve", "--mcp"]` per lsp-and-mcp.md §"What this spec implies" item (4).

**Precondition:** Phase B empirically stable; rust/src/mcp.rs serve_loop passes byte-parity test fixtures + zero regression across all 9 tools; `mirror serve --mcp` stdio behavior indistinguishable from `bin/mirror-mcp` → bootstrap chain at Phase A altitude (client-visible: same tools/list schema; same initialize response shape; same per-tool dispatch behavior).

**Landings:**

- `.mcp.json` edit from `"command": "/Users/alexwolf/dev/projects/mirror/bin/mirror-mcp"` to `"command": "/Users/alexwolf/.local/bin/mirror", "args": ["serve", "--mcp"]` (or whatever the operator's canonical mirror-binary install path resolves to).
- `bin/mirror-mcp` bash shim stays as archival transitional per two-tick discipline (git preserves; the shim simply becomes unused; delete deferred to two-tick-post-Phase-C window).
- `docs/specs/lsp-and-mcp.md` §"State today" table row for MCP transport updates from `bin/mirror-mcp` (bash, ~80 lines) to `mirror serve --mcp` (rust/, native).

**Phase C empirical anchor:**

```
mcp__mirror__mirror_roomba("--vacuum=/tmp/test-dir")
  → .mcp.json → ~/.local/bin/mirror serve --mcp
  → rust/src/mcp.rs::serve_loop (direct; NO shim)
  → [Phase B chain]
```

**Phase C discharges:** the `bin/mirror-mcp` bash shim retirement gate per lsp-and-mcp.md §"What this spec implies" (4). Two-tick removal window per substrate discipline (bin/mirror-mcp deleted at Phase C+2-tick sibling landing after empirical stability confirmed).

### §3.4 Phase D — `@mirror/reload` gen_prism at rust/src/mcp.rs (post-M5)

**Scope:** land `@mirror/reload` gen_prism at rust/ altitude per `boot/std/mirror/reload.mirror` (2.0KB substrate-decl LANDED-SPEC-ONLY per Taut §3) + lsp-and-mcp.md §"Auto-reload" contract; handles `notifications/tools/list_changed` on grammar drift.

**Precondition:** Phase C empirically stable; `@mirror/runtime/gen_prism` primitive lands at rust/ altitude (per `docs/specs/mirror-runtime-gen-prism.md` §"The primitive": `tick(state, message) -> tick_result`; `observe(gp) -> oid`; `history(gp, N) -> [oid]`); `@mirror/store` six-op wire lifted from bootstrap to rust/ altitude (per Taut §1 gap: no rust/ Rust emitter for the six-op wire surface currently; forward-promised at rust/spectral/ or rust/store/ crate landing).

**Landings:**

- `rust/src/mcp.rs::handle_request_in` invokes `mirror_reload::tick(session_state_oid, message)` on EVERY incoming request per `boot/std/mirror/reload.mirror` semantics ("Every incoming request — any request, not just `tools/list` — triggers a tick. No watcher, no inotify, no daemon dependency.").
- `mirror_reload::tick` computes `@mcp.grammars_hash` (SHA-256 over `(path, content_oid)` for every grammar reachable from `boot/std/`); compares to `state.last_emitted_hash`; if drifted, appends `notifications/tools/list_changed` JSON-RPC frame to the outbound stdio wire; updates `state.last_emitted_hash` via @mirror/store crystal.
- `initialize_result` updates `capabilities.tools.listChanged` from `false` to `true` (per MCP protocol: server declares reload capability to the client).
- The state crystal at `refs/gen_prism/mirror_reload` persists across daemon restarts per gen_prism ref discipline.

**Composition anchors (LANDED-SPEC-ONLY awaiting rust/ altitude realization):**

- `boot/std/mirror/reload.mirror:1-53` (2.0KB) — the gen_prism substrate-decl; tick body carries the match on `(current_hash, stored_hash)` tuple with same-name-twice equality discipline.
- `boot/std/mcp.mirror:78` (`grammars_hash -> oid`) forward-promised at boot altitude; body `\`-obligation-blocked; realization via `@hash/coincidence.content_oid` over `@mirror/spectral.gestalt`.
- `docs/specs/mirror-runtime-gen-prism.md` — gen_prism primitive spec; Example 1 IS the `@mirror/reload` reload contract.

**Phase D discharges:** the auto-reload gap per lsp-and-mcp.md §"Auto-reload" + closes Blocker part of Layer 4 per Taut §5 (`@mirror/reload gen_prism LANDED-SPEC-ONLY` → LANDED-EMPIRICAL at rust/ altitude).

**Phase D empirical anchor:**

```
[MCP client caches tools/list from initialize handshake]
[operator pulls a branch that adds a new grammar with @mcp/tool annotation]
mcp__mirror__mirror_compile("foo.mirror")
  → rust/src/mcp.rs::handle_request_in
  → mirror_reload::tick(state, message)
  → @mcp.grammars_hash ≠ state.last_emitted_hash
  → emit `notifications/tools/list_changed` on stdio wire
  → update state.last_emitted_hash via @mirror/store crystal
  → process the `mirror_compile` tool call as normal
[MCP client re-fetches tools/list; sees new grammar's @mcp/tool as new tool entry]
```

### §3.5 Phase E — `mirror kintsugi @spec` spawns @spec into @song via @mirror/peer/beam (post-M6)

**Scope:** ratified @spec accumulated through the MCP session gen_prism spawns into a running @song via `mirror kintsugi @spec` verb per collapse-spec §4 + `shards/mirror/peer/beam.mirror` §4.2 spec-target spawn extension.

**Precondition:** Phase D empirically stable; `@mirror/peer/beam` empirical dispatch lands at rust/ altitude (per Taut §4: `mirror peer beam` present in rust/src/main.rs VERBS list but returns exit 2 with "lands at M3+"; the actual empirical peer-spawn requires @fate optical inference + @pack.spawn realization at rust/ altitude per Alex-altitude Phase G+H LOCAL-PACK loop closure); `@spec` construction at MCP session altitude per collapse-spec §3.3 empirical (session accumulates spec fragments from tool-call trajectory; ratified @spec is the accumulated crystal at session-close time OR at `spec_ratified(spec, p)` predicate discharge time).

**Landings:**

- `shards/mirror/peer/beam.mirror` extends `spawn_target` type per collapse-spec §4.2 verbatim: `type spawn_target = | peer_target(peer) | spec_target(@mirror/spec) |`; adds `spawn_spec(spec: @mirror/spec, p: perturbation) -> song requires spec_ratified(spec, p) { \ }` sibling action.
- `rust/src/main.rs` dispatches `mirror kintsugi @spec` (recognizing `@spec` sigil after `kintsugi` verb) to `cmd_kintsugi_spec(&rest)` which invokes `mcp_session::current_ratified_spec()` → `peer_beam::spawn_spec(spec, perturbation)` → returns @song handle.
- `rust/src/mcp.rs` adds `mirror_kintsugi_spec` tool entry (10th tool at Phase E) invocable from MCP client to trigger the empirical spawn.
- The spawned @song runs Fate multi-frequency tournament at each temporal step per collapse-spec §5; the trajectory IS observable via gen_prism observer surface (per §3.5's `observe(gp)`); coefficient pattern names Projection vs Illusion per §6.

**Composition anchors (composition graph forward-promises):**

- `docs/specs/mcp-spec-song-collapse.md` §4.2 verbatim: the spec-to-song lift extends `mirror_spawn_request` target-discriminator to accept `spec_target(@mirror/spec)`.
- `shards/mirror/peer/beam.mirror:15.9KB` (Taut §4) — the cli-surface wrapper; adds spec_target arm to existing peer_target dispatch.
- `shards/pack.mirror:263` — `@pack.spawn` primitive at pack altitude; Recognition #84 LANDED at substrate-decl; empirical realization forward-promised to Alex-altitude Phase G+H LOCAL-PACK loop.
- `shards/fate.mirror` — Fate D²NN + Fabry-Pérot + Reck/Clements unitary mesh per collapse-spec §5.2; the tournament dispatcher.

**Phase E discharges:** the loop-closure at MCP altitude per collapse-spec §4 (MCP session accumulates @spec via tool-call trajectory → ratified @spec spawns into running @song → @song's trajectory observable at MCP altitude via next session's queries against the @song's gen_prism observer); closes Blocker C per Taut §5 (`No @peer runtime at rust/ altitude` → empirical dispatch through the composition chain).

**Phase E empirical anchor:**

```
[MCP session accumulates spec fragments across 20+ tool calls]
mcp__mirror__mirror_kintsugi_spec()  # 10th tool; Phase E
  → rust/src/mcp.rs::dispatch_tool_call("mirror_kintsugi_spec", args, ctx)
  → mcp_session::current_ratified_spec()
  → peer_beam::spawn_spec(spec, perturbation)
  → @song handle returned
  → @song trajectory runs Fate multi-frequency tournament at each temporal step
[Subsequent MCP queries observe the @song's coefficient trajectory via gen_prism observer]
```

### §3.6 Phase A→E ordering summary (per collapse-spec §5.2 sub-arc dependency graph)

| Phase | Actor | Empirical altitude | Discharges | Preconditions |
|---|---|---|---|---|
| A | Reed nearly-today | bin/mirror-mcp → bootstrap chain still owns serve_loop; delegation stub at rust/src/main.rs | MCP-spawn end2end fires TODAY; 9th tool (`mirror_roomba`) available | none |
| B | Reed post-Mara-spec | rust/src/mcp.rs owns serve_loop; bootstrap serve_loop NOT touched | Blocker A empirical closure; substrate-honest ouroboros at rust/ altitude | A landed; THIS spec landed |
| C | Reed post-B-stability | `bin/mirror-mcp` shim NOT in execution path | shim retirement per lsp-and-mcp.md (4) | B empirically stable |
| D | Reed post-M5 | `@mirror/reload` gen_prism fires per-request | auto-reload contract per lsp-and-mcp.md §"Auto-reload" | C landed; `@mirror/runtime/gen_prism` + `@mirror/store` six-op wire at rust/ altitude |
| E | Reed post-M6 (Alex-altitude Phase G+H composition) | `mirror kintsugi @spec` spawns @song empirically | Loop-closure at MCP altitude per collapse-spec §4 | D landed; `@mirror/peer/beam` empirical + `@spec` accumulation at MCP session |

One phase after the other. Each phase's landing IS a distinct empirical witness. Substrate-pull-honest per no-time-estimates discipline: motion order named; motion count untimed.

---
