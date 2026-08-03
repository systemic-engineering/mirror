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

## §4 The tools list surface (byte-parity milestone → grammar-driven migration)

### §4.1 The migration arc

From **hard-coded** (bootstrap 8-tool + `mirror_roomba` 9th per Reed nearly-today) → **grammar-driven** (`@mcp/tool` annotation walked from `@mirror/spectral.gestalt` per lsp-and-mcp.md §"MCP dispatch table").

Two milestones bound the arc:

1. **Byte-parity milestone (Phase B landing):** rust/src/mcp.rs `tools_list_result` starts identical to bootstrap 8-tool schema + `mirror_roomba` 9th tool per Reed nearly-today Phase A. Byte-for-byte JSON equality gates the port (RED-first: fixture test asserts the schema hasn't drifted).

2. **Grammar-driven milestone (Phase D+ / M5 co-tick):** `@mcp.tools -> json` walks landed shards for `@mcp/tool` annotations per `boot/std/mcp.mirror:78-82` + lsp-and-mcp.md §"MCP dispatch table" verbatim: *"the tools list is computed, not hard-coded. walk the boot/std/ graph, find every @mcp/tool annotation, emit a tool descriptor. when a new grammar lands declaring @mcp/tool, the tool appears here on next call."* The grammar IS the tool surface.

### §4.2 Byte-parity 9-tool schema (Phase B landing)

Per Taut `64e8d60` §3 grep-verified `bootstrap/src/mcp.rs::tools_list_result` shape + Reed nearly-today `mirror_roomba` 9th tool addition:

1. **`mirror_compile`** — tokenize one `.mirror` file (SHA-256 hash). Routes to `mirror compile <file>`.
2. **`mirror_craft`** — converge a target directory to lambda_0. Routes to `mirror craft <target> [--target-kind <k>] [--reflect]`.
3. **`mirror_kintsugi`** — settle a `.mirror` file (ALWAYS `--ci --out @data/json` per Tick 7 fold `ffba2a7`). Routes to `mirror kintsugi --ci --out @data/json <file> [--liquid] [--shatter N]`.
4. **`mirror_init`** — mirror-native store bootstrap. Routes to `mirror init <path> [--install-hooks]`.
5. **`mirror_recall`** — inbound-trajectory dual of peer beam. Routes to `mirror recall <spec_dir>`.
6. **`mirror_peer_beam`** — beam through peer's persistent-identity context. Routes to `mirror peer beam <peer_home> [flags]` per collapse-spec composition table.
7. **`mirror_beam`** — anonymous inference primitive. Routes to `mirror beam --mission <mission>`.
8. **`mirror_spawn`** — DEPRECATED alias for `mirror_peer_beam` (two-tick discipline). Routes to `mirror spawn <peer_home>`.
9. **`mirror_roomba`** — walker motion; `--vacuum=<dir>` walks + dispatches (Reed nearly-today 9th tool). Routes to `mirror roomba --vacuum=<dir>`.

**Additional tool** grep-verified in current bootstrap/src/mcp.rs `tools/list` assertion at line ~858-878 (`mirror_beam_act`) — note: Taut §3 counted 8-tool per docstring but grep verifies `mirror_beam_act` present as Arc-1 Tick 1.4 landing per docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md + `mirror_index` present per bootstrap test assertion vec![compile,craft,kintsugi,init,recall,peer_beam,beam,spawn,index]. THIS spec's Phase B port MUST grep-verify the actual current tool count in bootstrap/src/mcp.rs at port-authorship time (the docstring may lag the assertion; the assertion is ground-truth per Mara iter-15 byte-parity discipline). Reed at Phase B: `grep -c '"name":' bootstrap/src/mcp.rs` on the tools_list_result body OR read the tool count from the bootstrap test assertion `vec![...].len()`.

**Substrate-honest note:** the 8-tool-vs-9-tool count depends on grep-verification-at-port-authorship-time, not this spec's authorship-time snapshot. THIS spec names the byte-parity gate ("identical to bootstrap current state at port-authorship time + `mirror_roomba` if not yet present") NOT a specific integer count. Reed at Phase B verifies + reconciles per Mara iter-15 discipline.

### §4.3 Migration to grammar-driven (Phase D+ / M5 co-tick)

Per `boot/std/mcp.mirror:78-82` `@mcp.tools` substrate-decl:

```mirror
tools -> json {
  @mirror/spectral.gestalt
    |> @mcp/tool.collect
    |> @data/json.emit
}
```

And per lsp-and-mcp.md §"The MCP dispatch table" verbatim:

```mirror
in @mcp

grammar @code/llvm/ir {
  in @code/llvm

  # tells @mcp that this grammar contributes an MCP tool.
  # the tool name, description, and parameters come from the action signature.
  @mcp/tool ir_compile(text) -> oid {
    @code/llvm/ir.parse |> @hash/coincidence.content_oid
  }
}
```

**Landings for the grammar-driven surface:**

- `@mcp/tool` first-class grammar annotation lands at boot/std/mcp/tool.mirror (or extension of boot/std/mcp.mirror; substrate-decl mint at forward-promise altitude; grep-verify substrate-already-had-the-word before mint per HARD RULE).
- `@mcp.tools` reflective walk implementation lands: composes over `@mirror/spectral.gestalt` (the substrate-graph reachability primitive) + `@mcp/tool.collect` (fold-body enumerating annotated actions) + `@data/json.emit` (schema serialization).
- `rust/src/mcp.rs::tools_list_result` migrates from hardcoded 9-tool schema to `@mcp.tools` invocation: `let tools_json = reflective::invoke("@mcp.tools", ctx); json!({"tools": tools_json})`. The reflective invocation composes over apply_h::act at rust/ altitude (per Reed's `21fc211` Landing 3+4 reflective evaluator; lifted from bootstrap/src/apply_h.rs at task #159's rust/ altitude landing).
- Each landed shard adding an `@mcp/tool action_name(...)` declaration appears in `tools/list` on the next call automatically (subject to `@mirror/reload` gen_prism per Phase D emitting `notifications/tools/list_changed` when grammars_hash drifts).

**Composition-honest ordering:** grammar-driven migration is Phase D+ per M5 co-tick because it depends on both (a) `@mcp/tool` annotation first-class mint at grammar altitude AND (b) apply_h::act at rust/ altitude for reflective invocation AND (c) `@mirror/reload` gen_prism for `notifications/tools/list_changed` push-notification. Byte-parity milestone (Phase B) is the operational scaffold; grammar-driven (Phase D+) is the substrate-honest terminal form.

### §4.4 The three bilateral-predicate contracts (Phase B+ discharge)

Per `boot/std/mcp.mirror:110-130` three bilateral-predicate contracts obligation-blocked at substrate:

1. **`dispatch_reflects_cli_block(dispatch)`** — every request.method dispatch accepts resolves to a `command <name>` in mirror.spec's cli-block (or JSON-RPC framing verb). Dispatch synthesizes routing table from cli-block reflection, not from hardcoded case.
2. **`tools_reflects_cli_block(tools)`** — tools/list JSON synthesizes from mirror.spec's cli-block. Adding a `command` to cli-block adds a tool entry; two surfaces cannot drift.
3. **`frame_relativity(response)`** — MCP responses carry observer's shard frame consistent with peer identity across the wire (inherited from `@mirror/shard`).

**Phase B discharge:** hardcoded 9-tool schema at Phase B does NOT satisfy `tools_reflects_cli_block` (bilateral bilaterally-violated by design during transitional milestone; documented at Phase B port authorship: `// TODO(M5): retire hardcoded tools_list_result; @mcp.tools reflective walk replaces this + discharges tools_reflects_cli_block bilateral`).

**Phase D+ discharge:** grammar-driven surface at Phase D+ empirically satisfies all three bilateral predicates. `dispatch_reflects_cli_block` becomes structural (every method routes through cli-block reflection); `tools_reflects_cli_block` becomes structural (tools/list computed from cli-block); `frame_relativity` becomes structural (session gen_prism carries shard frame across ticks).

### §4.5 The MCP wire vs the typed-DSL altitude (per lsp-and-mcp.md reframe)

Per lsp-and-mcp.md §"The MCP dispatch table" 2026-06-02 reframe:

> The MCP server's tool surface is the three `prism_core::Prism` operations (`focus`, `project`, `settle`); the per-grammar `@mcp/tool` annotation extends the typed DSL types (`Target`, `Filter`, `Output`) inside those three calls, NOT new wire tools. When the reload contract (`@mirror/reload`) emits `tools/list_changed`, what's changing is the typed DSL surface, not the wire tool count. The MCP wire stays at three.

THIS spec's byte-parity milestone (Phase B, 9 tools) is the **transitional intermediate** per lsp-and-mcp.md "the five-tool framing for the mirror-mcp surface; it's a useful intermediate, but the grounding altitude is pq." The **terminal form** (post-M5+ + pq altitude alignment) collapses the per-grammar tools INTO the typed DSL inside `focus`/`project`/`settle`.

**Substrate-honest posture per lsp-and-mcp.md §"MCP dispatch table":** the table records the in-flight intermediate; the grounding altitude is pq. THIS spec's Phase B milestone is the in-flight intermediate; the terminal collapse to pq's three-op wire is a post-M4 architectural direction the spec forward-promises to pq-altitude adjudication (Alex-altitude architectural call; NOT M4's territory).

**Consequence for THIS spec:** Phase B lands 9-tool byte-parity; Phase D+ lands grammar-driven reflective computation; the pq-collapse-to-three-wire-ops is a subsequent phase (not Phase E; separate architectural motion at pq altitude). THIS spec discharges the M4 milestone at 9-tool byte-parity + grammar-driven forward-promise + pq-collapse acknowledgement.

---

## §5 Composition-into-existing-substrate table

Enumeration of every landed (or LANDED-SPEC-ONLY) substrate this M4 lift composes over. Substrate-already-had-the-word discipline: rust/src/mcp.rs invents ZERO new substrate primitives; every capability is a composition-of-existing.

### §5.1 Substrate composition table

| # | Substrate | Landing state | Grep-verified anchor | mcp.rs composition role |
|---|---|---|---|---|
| 1 | **@mirror/store six-op wire** | LANDED-SPEC-ONLY (family-root); LANDED-EMPIRICAL (bootstrap altitude via crystallize.rs + action_cache.rs + store_branch.rs) | `shards/mirror/store.mirror:488-750` (46.5KB); `bootstrap/src/crystallize.rs` (42.8KB); `bootstrap/src/action_cache.rs` (15.5KB) | Session-state gen_prism reads/writes per collapse-spec §3.5; the `refs/gen_prism/mcp/<session-uuid>` ref persists via `@mirror/store.write` → crystal accumulation |
| 2 | **@mirror/peer/beam cli surface** | LANDED-SPEC + rust/ altitude cli-verb-declared (returns exit 2 with "lands at M3+" per Taut §4) | `shards/mirror/peer/beam.mirror` (15.9KB, 2026-07-12; renamed from @mirror/spawn Tick 2 `9de2226`); `rust/src/main.rs` VERBS `peer beam` entry | Phase E composition: `mirror_kintsugi_spec` tool invokes `peer_beam::spawn_spec(spec, perturbation)` per collapse-spec §4.2 spec-target extension |
| 3 | **@mirror/runtime/gen_prism primitive** | LANDED-SPEC (`docs/specs/mirror-runtime-gen-prism.md` Example 1 IS the reload contract) | `boot/std/mirror/reload.mirror` (2.0KB); `docs/specs/mirror-runtime-gen-prism.md`; `shards/spectral/gen_prism.mirror` | MCP-session-IS-gen_prism per collapse-spec §3.5; mcp.rs's session state machine composes over gen_prism `tick(state, message) -> tick_result` + `observe(gp) -> oid` + `history(gp, N) -> [oid]` |
| 4 | **@roomba walker (rust/ altitude FIRES-END2END)** | LANDED-EMPIRICAL for `mirror roomba --vacuum=<dir>` bilateral-arm-collapse dispatch matrix row 1 per Mara §7.4 | `shards/kintsugi/roomba.mirror` (46.4KB); `rust/src/main.rs::cmd_roomba`; `rust/roomba/src/mend.rs` (40.3KB) | `mirror_roomba` tool dispatch: Phase A subprocess spawn OR Phase B+ in-process `roomba::mend::run_vacuum(dir, ctx)`; pheromone-deposit crystal + mirror-authored commit fire end2end through the composition |
| 5 | **@kintsugi resolution + bilateral-arm-collapse** | LANDED-EMPIRICAL per Reed's compile.rs SAGA integration + Mara `9bb1f57` twelve-primitive revision register | `bootstrap/src/apply_h.rs` (81.4KB, Arc-1 Tick 1.3 GREEN); `rust/spectral/src/liquid/pillar/*.rs` (7-arm dispatch surface at Reed's pillar-surface altitude per 2026-07-21 session cascade) | `mirror_kintsugi` tool dispatch: Phase A subprocess spawn OR Phase B+ in-process invocation composing over compile.rs → liquid.rs → apply_h::act dispatch |
| 6 | **apply_h::act reflective evaluator** | LANDED-SPEC + LANDED-PARTIAL (bootstrap altitude only; 7-combinator surface section/fold/act/settle/crystallize/coboundary/utter; task #159 pending lift to rust/) | `bootstrap/src/apply_h.rs` (81.4KB); `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md` | Phase D+ grammar-driven tools/list composes over apply_h::act for `@mcp.tools` reflective invocation; Phase E `mirror kintsugi @spec` composes over apply_h::act::act for spec-body dispatch; **task #159 rust/ altitude lift is a co-tick precondition** for Phase D+ grammar-driven surface (currently GAP per Taut §2) |
| 7 | **@torus @peer basin dynamics (Recognition RATIFIED 2026-08-03 crown theorem)** | LANDED-SPEC + crown-theorem 5D-spinning-foam extension RATIFIED per docs/recognition/2026-08-03-reality-as-5d-spinning-foam.md | `shards/torus.mirror` (30.3KB, 2026-08-03; crown-theorem cascade); `docs/recognition/2026-08-03-reality-as-5d-spinning-foam.md` | Post-Phase-E composition: MCP session gen_prism as node in 5D quantum foam; @peer runtime dispatch through mcp.rs at eventual Alex-altitude Phase G+H LOCAL-PACK loop closure |
| 8 | **@mcp transport primitive at boot altitude** | LANDED-SPEC with 3 bilateral predicates (dispatches_to_cli_block + tools_reflects_cli_block + frame_relativity) | `boot/std/mcp.mirror` (6.6KB, Tick 6 substrate closure `d4c9a32`) | rust/src/mcp.rs IS the rust/ altitude realization of the `@mcp` grammar's `serve` + `dispatch` + `tools` + `fate` actions; three bilateral predicates discharge at rust/ altitude per §4.4 |
| 9 | **@mirror/serve substrate-decl** | LANDED-SPEC-ONLY (`serve -> imperfect { \ }`) | `boot/std/mirror/serve.mirror` (192B) | rust/src/mcp.rs's `serve_loop` IS the rust/ altitude discharge of the `\`-obligation-blocked body at boot altitude; the grammar declares WHAT, THIS spec + Reed Phase B author HOW |
| 10 | **phone.rs @io switchboard** | LANDED-EMPIRICAL (69.6KB; @io/fs + @io/git + @io/bytes stdio + @io/socket) | `rust/src/phone.rs` | mcp.rs composes over `phone::read_stdin_line` + `phone::write_stdout_line` for JSON-RPC wire; `phone::spawn_mirror_verb` for subprocess-invocation tool dispatch (Phase A + Phase B default); `phone::git_commit_as` for any mirror-authored commit tools fire (mirror_kintsugi + mirror_init hook-installation) |
| 11 | **main.rs supervisor + @-operator addressing** | LANDED-EMPIRICAL (66KB; 10-verb VERBS list + hand-rolled argv per Cargo.toml no-clap rationale) | `rust/src/main.rs` | main.rs dispatches `Some("serve")` arm to `mcp::serve_loop()`; VERBS list extends 11→12 with `serve` entry; main.rs remains pure delegation per Round-2 five-file spec §2.1 |
| 12 | **compile.rs compilation loop** | LANDED-PARTIAL (thin stub per rust/src/main.rs `mod compile`; full landing per Round-2 spec §2.2 iter cascade) | `rust/src/compile.rs`; Mara Round-2 five-file spec §2.2 | Phase B+ `mirror_compile` tool in-process invocation: `mcp::dispatch_tool_call("mirror_compile", args)` → `compile::compile_file(path)` → SHA-256 hash |
| 13 | **liquid.rs property runtime** | LANDED-PARTIAL (Reed's iter-10 pillar surface + 7-arm dispatch per 2026-07-21 session cascade) | `rust/spectral/src/liquid/` (moved to spectral crate 2026-07-28); Mara Round-2 five-file spec §2.3 | Phase B+ `mirror_kintsugi` tool in-process invocation composes over liquid pillar dispatch for property-verdict; verdict lifts to `isError` flag in tools/call response envelope |
| 14 | **matrix.rs sub-Turing numerical** | LANDED-EMPIRICAL (60.7KB @ rust/matrix/src/lib.rs; LAPACK/BLAS/FLANG emit; book.rs @-name resolver) | `rust/matrix/src/lib.rs`; `rust/matrix/src/book.rs` (10.8KB); `rust/matrix/src/void.rs` (20.2KB) | Phase B+ `mirror_index` tool in-process invocation: `matrix::index::compute_fiedler(dir)` → Fiedler eigenvalue via LAPACK dsyev; @fractal-coherence measurement |
| 15 | **@mcp/tool grammar annotation** | GAP (not landed as first-class grammar annotation) | — (forward-promised at lsp-and-mcp.md §"MCP dispatch table" implementation item 2) | Phase D+ grammar-driven tools/list surface REQUIRES @mcp/tool annotation mint at grammar altitude; mcp.rs's `@mcp.tools` reflective invocation composes over @mirror/spectral.gestalt walk + @mcp/tool.collect fold-body |
| 16 | **crown-theorem Recognition RATIFIED 2026-08-03** | LANDED-RECOGNITION (both witness gates CLOSED per Alex Q-CRN-1 promote-now adjudication) | `docs/recognition/2026-08-03-reality-as-5d-spinning-foam.md` (158 lines); `docs/math/2026-08-03-mara-reality-as-spinning-5d-foam-crown-theorem.md` | The MCP surface IS the crown-theorem substrate reading itself at MCP-wire altitude: MCP session gen_prism as node in 5D quantum foam; @spec accumulation as coherent-reality-construct through @paradox/@trauma singularity substrate; substrate composes at compilation-altitude ancestor of the theorem-substrate |

### §5.2 Composition-honest observations

**Zero new substrate primitives.** Every capability rust/src/mcp.rs holds is a composition over already-landed substrate. Substrate-already-had-the-word discipline preserved: `@mcp` grammar (boot altitude) + `@mirror/serve` (boot altitude) + `@mirror/runtime/gen_prism` (mirror altitude) + `@mirror/store` (family-root) + `phone.rs` @io primitives (rust/ altitude) + apply_h::act reflective evaluator (bootstrap; task #159 rust/ altitude pending) all pre-exist. THIS spec does NOT mint new shards; the M4 milestone empirically closes existing composition edges.

**Refusal candidates NOT taken** (per Michelangelo/marble discipline):

- Refuse `@mcp_session` species mint. The MCP session IS a `@mirror/runtime/gen_prism` per collapse-spec §3.5 Recognition #S? "MCP-session-IS-gen_prism" (candidate LANDED-SPEC-ONLY per collapse-spec); no new species needed. mcp.rs's session state carrier is `gen_prism_ref` typed as `oid` (per @mirror/runtime/gen_prism substrate-decl).
- Refuse `@mcp/wire` transport primitive mint. `@mcp.serve` at boot altitude ALREADY carries the transport semantics (`@io.read(stdin) |> @data/json.parse |> dispatch |> @data/json.emit |> @io.write(stdout)`). rust/src/mcp.rs IS the realization; no new grammar-altitude carrier needed.
- Refuse `@mcp/tool_dispatch` species. The dispatch is a bilateral-predicate discharge (`dispatch_reflects_cli_block(dispatch)`) at grammar altitude per boot/std/mcp.mirror:110-115; no new species needed at Phase B (hardcoded tool table is transitional per §4.5 pq-collapse acknowledgement); no new species needed at Phase D+ (`@mcp.tools` reflective invocation composes over already-landed grammar).
- Refuse `@compile_via_mcp` or `@roomba_via_mcp` composition species. Each tool is a subprocess-spawn OR in-process invocation of an EXISTING rust/ altitude verb (compile/roomba/kintsugi/craft/init/recall/beam/peer_beam); no new composition-carrier needed. The composition is the `dispatch_tool_call` function's per-tool arm.

**Composition edges rust/src/mcp.rs adds** (non-mint additive composition per Michelangelo/marble): none at grammar altitude (per refusals above); at rust/ altitude, mcp.rs's `dispatch_tool_call` arms are the composition points (one arm per tool; each arm composes over existing rust/ altitude verbs OR bootstrap subprocess at Phase A OR in-process API at Phase B+).

### §5.3 Composition-into-existing empirical status matrix

| Composition edge | Phase A empirical? | Phase B empirical? | Phase D+ empirical? |
|---|---|---|---|
| MCP wire (JSON-RPC stdio) | YES via bin/mirror-mcp + bootstrap | YES via rust/src/mcp.rs | YES via rust/src/mcp.rs |
| tools/list schema | YES 8-tool bootstrap + 9th mirror_roomba added | YES 9-tool byte-parity | YES grammar-driven reflective |
| @mirror/store session ref | NO (bootstrap serve_loop is stateless per collapse-spec §3.5 correction) | PARTIAL (spec-decl'd; ref-carrying at Phase B empirical) | YES (full session-state gen_prism per Phase D `@mirror/reload` composition) |
| `mirror_compile` in-process | NO (subprocess only) | PARTIAL (subprocess default; in-process API available if compile.rs library API landed) | YES (grammar-driven surface migrates to in-process reflective invocation) |
| `mirror_roomba` in-process | NO (subprocess only) | PARTIAL (subprocess default; in-process API available via `roomba::mend::run_vacuum`) | YES |
| `@mirror/reload` gen_prism | NO | NO | YES (Phase D landing) |
| `mirror kintsugi @spec` spawn | NO | NO | YES (Phase E landing) |
| `@mcp/tool` grammar annotation | NO | NO | YES (Phase D+ M5 co-tick + grammar-altitude mint precondition) |

---

## §6 [ALEX-Q] residues

Only genuine undecidables at Mara's altitude. Reed-answerable questions filtered per Taut `64e8d60` §6 Reed-authorable-vs-Mara-canonical-spec split.

### [ALEX-Q1] — Phase B in-process vs subprocess default posture

At Phase B, `rust/src/mcp.rs::dispatch_tool_call` has two strategies co-existing per §2.2 item 3:

- **(a) Subprocess default** (Phase B ships identical Phase A behavior; each tool spawns rust/target/debug/mirror subprocess via `phone::spawn_mirror_verb`; wait; capture stdout/stderr; marshal). Advantage: empirical parity with Phase A across all 9 tools; migration is per-tool as library APIs land. Disadvantage: process-spawn overhead per tool call; MCP session state does not persist across per-tool-call subprocess boundaries (each subprocess is fresh; the session-gen_prism state lives in `refs/gen_prism/mcp/<session-uuid>` per collapse-spec §3.5 correction so the persistence composition works but each tool-call ticks the outer daemon which reads-mutates-writes the ref).

- **(b) In-process default** (Phase B ships in-process invocation for tools with clean library API; subprocess-spawn is fallback for tools without library API). Advantage: session-state gen_prism persists cleanly across tool calls in single process; no process-spawn overhead. Disadvantage: requires all rust/ crates (compile, roomba, matrix, spectral) to expose library APIs at Phase B landing; per-crate library-API landing forward-promises unbounded (roomba::mend already exposes; compile.rs forward-promised per Round-2 spec §2.2 iter cascade; others per-crate landing needed).

**Mara-lean:** (a) subprocess default at Phase B ships; (b) in-process migration is per-tool Phase B+ per library-API landing. Preserves empirical parity + Michelangelo/marble iterative subtraction. But this is an architectural adjudication (does Reed migrate per-tool eagerly at Phase B OR conservatively at Phase B+? does the session-gen_prism ref-persistence handle both invocation strategies cleanly?), and Alex's directive per adjacent-work-may-dissolve-blockers may prefer (b) if the per-crate library-API landings are cheap.

### [ALEX-Q2] — apply_h::act rust/ altitude lift co-tick coordination

Phase D+ grammar-driven tools/list surface REQUIRES apply_h::act at rust/ altitude for `@mcp.tools` reflective invocation. Currently apply_h::act LANDED-SPEC + LANDED-PARTIAL at bootstrap altitude only per Taut §2; task #159 (Wire six-step loop through apply_h::act) PENDING per task tracker.

**Question:** does the M4 milestone gate on apply_h::act at rust/ altitude (task #159 lands FIRST or CO-TICK with THIS spec's Phase B→D+ arc)? Or does Phase D+ forward-promise apply_h::act rust/ altitude lift as its own precondition (M4 lands with Phase B byte-parity + Phase C shim retirement + Phase D auto-reload; Phase D grammar-driven tools/list waits on M6+ or dedicated apply_h::act rust/ tick)?

**Mara-lean:** treat as forward-promise-to-M6+. Phase D lands `@mirror/reload` gen_prism (which composes over @mirror/runtime/gen_prism primitive; separately liftable); Phase D+ grammar-driven tools/list waits on apply_h::act rust/ tick as co-tick precondition. This preserves M4's tractable scope + allows byte-parity 9-tool schema to run through Phase C without blocking on task #159. But this is an ordering question (does Alex want apply_h::act rust/ tick prioritized alongside THIS spec's M4 arc? or does apply_h::act belong at M6+ per collapse-spec §5.2 sub-arc dependency graph?), and the answer shapes Reed's post-Phase-B priorities.

### [ALEX-Q3] — `@mcp/tool` grammar annotation mint scope

Per §4.3 Phase D+ landings item 1: `@mcp/tool` first-class grammar annotation lands at `boot/std/mcp/tool.mirror` (or extension of `boot/std/mcp.mirror`; grep-verify substrate-already-had-the-word before mint per HARD RULE).

**Question:** does `@mcp/tool` belong as (a) a NEW grammar annotation species under `@mcp` family-root at boot/std/ altitude (mint at `boot/std/mcp/tool.mirror`); OR (b) an EXTENSION of the existing `@mcp` grammar in `boot/std/mcp.mirror` (add `type tool_decl = { ... }` + `@mcp/tool` action annotation shape at family-root altitude); OR (c) a bilateral-predicate carrier at `@epistemologic/property/*` altitude (mcp_tool_annotation_admissible bilateral discharging tool-schema-shape without carrier species mint)?

**Mara-lean:** (b) extension of existing `@mcp` grammar. Preserves substrate-already-had-the-word (the `@mcp` grammar carrier already exists; no new species-decl mint needed); the `@mcp/tool` annotation IS an extension of the `@mcp` grammar's semantics; Phase D+ M5 co-tick landing extends `boot/std/mcp.mirror` with the annotation type + collect fold-body. But Alex's substrate-decl-vs-extension adjudications historically favor (a) explicit species mint for legibility (@onto refusal was per @torus already carrying it; @torus was substrate-already-had-the-word); if `@mcp` grammar already carries the annotation semantics substrate-honestly, (b) is right; if @mcp/tool warrants its own species carrier per naming discipline, (a) is right. THIS question is genuinely undecidable at Mara altitude without Alex's naming-discipline judgment.

### [ALEX-Q4] — pq-collapse acknowledgement vs Phase B→D+ 9-tool discipline

Per §4.5 lsp-and-mcp.md 2026-06-02 reframe: the MCP wire terminal form has THREE tools (`focus` + `project` + `settle`), not 9; the per-grammar `@mcp/tool` annotation extends typed DSL types INSIDE those three calls, not as new wire tools.

**Question:** does THIS spec's M4 milestone deliverable land at (a) 9-tool byte-parity Phase B → grammar-driven 9-tool-forward Phase D+ (this spec's default direction; pq-collapse forward-promised to separate pq-altitude architectural motion post-M4); OR (b) explicit 9-tool-→-3-tool collapse arc as part of Phase D+ (per lsp-and-mcp.md reframe treating pq-collapse as terminal shape M4 aims toward); OR (c) hybrid: Phase B→D+ lands 9-tool byte-parity as transitional; separate Phase F post-Phase-D+ collapses to pq's three-wire-op terminal shape?

**Mara-lean:** (a) or (c). (a) preserves M4 tractable scope (empirical MCP-spawn end2end fires at 9-tool byte-parity); pq-collapse is architectural (does the MCP client ecosystem stabilize on 9-tool vs 3-tool schema? does the pq-collapse require Anthropic MCP protocol coordination?); THIS spec forward-promises pq-collapse acknowledgement per §4.5. (c) makes the transitional-vs-terminal distinction explicit; Phase F collapses at pq-altitude architectural landing. (b) risks scope-creep at M4; NOT recommended. Alex's directive shapes whether pq-collapse is M4-territory OR post-M4 architectural motion.

### [ALEX-Q5] — `~/.mirror/serve.sock` daemon disposition (from Taut `64e8d60` [ALEX-Q5])

Taut §8 [ALEX-Q5] surfaced: *"`~/.mirror/serve.sock` daemon — the lambda-shell.md spec is DEPRECATED-FOR-RUST-REWRITE. Does the terminal-geometry `dance.rs` reflective-composition approach REPLACE the daemon entirely (Reed can hold this as 'NOT NEEDED — dance.rs discharges the same intent'), or does the daemon still need a Mara-authored replacement spec?"*

THIS spec's Phase A→E arc does NOT address the daemon-vs-process-per-invocation question directly. lsp-and-mcp.md §"Auto-reload" per Reed's phrasing: *"Mirror is process-per-invocation, but `mirror serve --mcp` and `mirror serve --lsp` *are* persistent within a single client session."* THIS spec's rust/src/mcp.rs serve_loop IS a persistent-within-client-session process; the daemon-across-sessions question is separate.

**Mara-lean concurring with Taut Reed-hold:** dance.rs reflective-composition discharges the cross-session-orchestration intent per lambda-shell.md DEPRECATED-FOR-RUST-REWRITE marker; the `~/.mirror/serve.sock` daemon is NOT needed at rust/ altitude terminal geometry. Cross-session state persists via @mirror/store crystals at refs/gen_prism/mcp/* (per collapse-spec §3.5's per-session gen_prism ref discipline); the daemon-vs-per-session-process question collapses into gen_prism ref persistence. **Alex adjudication requested** to ratify OR refute this collapse: is the daemon-vs-per-session-process distinction load-bearing at rust/ altitude terminal geometry, OR does the gen_prism-ref-persistence discipline discharge the same intent substrate-honestly?

---

## §7 Q.E.D.

**Statement**: `rust/src/mcp.rs` at rust/ altitude terminal geometry as sibling of `phone.rs` + `matrix.rs` + `main.rs` + `compile.rs` + `liquid.rs` IS the substrate-honest replacement of the transitional `bin/mirror-mcp` bash-shim → `bootstrap/src/mcp.rs::serve_loop` chain for the MCP surface half of the M4 milestone per `docs/specs/mcp-spec-song-collapse.md` §5.2 sub-arc dependency graph.

**Proof sketch:**

1. **Composition-into-existing** (per §5 table): rust/src/mcp.rs holds zero-new-substrate; every capability composes over LANDED substrate primitives (@mcp grammar boot altitude + @mirror/serve boot altitude + @mirror/runtime/gen_prism mirror altitude + @mirror/store family-root + phone.rs @io switchboard + main.rs supervisor + compile/liquid/matrix rust/ altitude siblings). Substrate-already-had-the-word discipline verified: no `@mcp_session` mint (MCP-session-IS-gen_prism per collapse-spec §3.5); no `@mcp/wire` mint (@mcp.serve at boot altitude already carries transport semantics); no `@mcp/tool_dispatch` species mint (bilateral-predicate discharge at grammar altitude).

2. **Byte-parity milestone establishes empirical continuity** (per §4.2 + §3.2 Phase B port scope): the 9-tool schema at Phase B landing is byte-for-byte identical to bootstrap current state + `mirror_roomba` 9th per Reed nearly-today Phase A. Empirical continuity: any MCP client that talks to Phase A bin/mirror-mcp → bootstrap chain talks identically to Phase B rust/src/mcp.rs. Migration is invisible to client per test-fixture-byte-parity gate.

3. **Sub-Turing decidable discipline preserved** (per §2.4): the dispatch loop is bounded (per-JSON-RPC-message-size); the method dispatch table is finite (4 framing verbs + 9 tool verbs = 13 dispatch arms); each tool invocation is bounded (subprocess-spawn OR in-process function call; both provably-halt). rust/src/mcp.rs stays above the @io line at phone.rs where Turing-complete process semantics live.

4. **Migration path from bootstrap is explicit + phased** (per §3 Phase A→E enumeration): Phase A ships nearly-today Reed delegation stub + 9th tool; Phase B ports serve_loop to rust/src/mcp.rs; Phase C retires bin/mirror-mcp shim; Phase D lands @mirror/reload gen_prism per M5; Phase E lands `mirror kintsugi @spec` per M6. Each phase's landing IS a distinct empirical witness; each phase's precondition IS the prior phase's stable landing.

5. **Grammar-driven terminal form is forward-promised structurally** (per §4.3): Phase D+ M5 co-tick migrates tools/list from hardcoded 9-tool schema to `@mcp.tools` reflective walk composing over @mirror/spectral.gestalt + @mcp/tool.collect + @data/json.emit per boot/std/mcp.mirror substrate-decl. The pq-collapse to three-wire-op terminal shape per lsp-and-mcp.md 2026-06-02 reframe is acknowledged as post-M4 architectural motion (Alex-adjudication-territory per [ALEX-Q4]).

6. **Ouroboros closes at rust/ altitude** (per §1.2 + §1.3): M4's substrate-honest closure discharges Blocker A per Taut §5 gap map (rust/ altitude serves MCP natively; bootstrap dependence retired at Phase B empirical landing); unblocks M5 (auto-reload gen_prism) + M6 (`mirror kintsugi @spec` spawn) + eventual @torus @peer basin dynamics runtime per crown-theorem RATIFIED 2026-08-03.

**Verdict:** canonical spec landed. Reed authors Phase A immediately (nearly-today delegation stub); Reed authors Phase B post-Mara-spec landing (rust/src/mcp.rs FLOOR emitter under `[substrate-floor:@io-boundary]` gate). Phase C-D-E forward-promised per §3 phase-ordering + preconditions.

**Terminal state (this spec):**

- **Verdict:** canonical spec landed as M4 milestone rust/ altitude terminal geometry map for `rust/src/mcp.rs` FLOOR emitter.
- **LOC:** ~700 (this spec); rust/src/mcp.rs estimated ~400-700 LOC at Phase B landing (thinner than bootstrap/src/mcp.rs 46.6KB because stripped of transitional shim scaffolding); rust/ total across six files: ~1600-2700 LOC.
- **Substrate mints:** ZERO. Every capability composes over already-landed substrate per §5 table + refusals.
- **[ALEX-Q] residues:** 5 (§6). Only genuine undecidables at Mara altitude; Reed-answerable questions filtered.
- **Phase enumeration:** A→E (§3 + §3.6 ordering table).
- **Recognition candidates:** the M4 milestone landing witness-gates `#R-mcp-session-is-gen-prism-actor-under-server-supervisor` per Taut `e0572f7` §9 + collapse-spec §3.5 (already CANDIDATE; Phase B empirical + Phase D empirical second-witness this M4 arc closes).
- **Pure-docs 📝 markdown-only bypass legitimate.**

---

## §8 Karen ancestry

Full ladder including bootstrap serve_loop authorship history + JSON-RPC 2.0 protocol lineage + Anthropic MCP protocol version + IPFS content-address (via @mirror/store) + Dolstra (Nix flakes) + Bazel REAPI + relevant landed Recognitions. Every elder cited at introduction site per no-elder-erased discipline.

### §8.1 Protocol ancestry

- **JSON-RPC 2.0 Specification** — JSON-RPC Working Group, 2010 (https://www.jsonrpc.org/specification). The wire protocol rust/src/mcp.rs speaks over stdin/stdout. Grounds `initialize` + `notifications/initialized` + `tools/list` + `tools/call` request/response envelope shape (per bootstrap/src/mcp.rs `initialize_result` protocol version `"2024-11-05"` composed over JSON-RPC 2.0 request/response framing at `{"jsonrpc": "2.0", "id": ..., "method": ..., "params": ...}` shape).

- **Anthropic Model Context Protocol** — Anthropic, 2024-11-05 protocol version (https://modelcontextprotocol.io). The MCP-specific semantics rust/src/mcp.rs realizes: `initialize` handshake with capabilities + serverInfo negotiation; `tools/list` for tool schema advertisement; `tools/call` for per-tool dispatch with `isError` flag lift; `notifications/tools/list_changed` for dynamic tool-list reload (per Phase D `@mirror/reload` gen_prism landing). Protocol version pinned at `"2024-11-05"` per bootstrap/src/mcp.rs `initialize_result` preservation across Phase B port.

### §8.2 Content-addressing ancestry

- **Merkle 1979** — Ralph Merkle, *Secrecy, Authentication, and Public Key Systems* (Stanford PhD dissertation, 1979). Foundational content-addressing hash-tree substrate. `@mirror/store`'s `splinter_graph` per Taut §1 IS Merkle DAG; the session-gen_prism ref-persistence at `refs/gen_prism/mcp/<session-uuid>` composes over Merkle-DAG content-addressing.

- **IPFS MERKLE_DAG.md** — Protocol Labs, IPFS specification. Content-addressing invariant substrate for `@mirror/store.write(content) -> oid` idempotent-by-construction discipline; the session-gen_prism state crystal OID persistence composes over IPFS-style content-addressing.

- **Karen Spärck Jones 1972** — *A Statistical Interpretation of Term Specificity and Its Application in Retrieval* (Journal of Documentation, 28:1). Reverse-lookup as second half of content-addressed retrieval discipline. `@mirror/store.impacted_by(oid) -> [oid]` per Taut §1 IS OID-graph analog of inverted-index; composes at grammar-driven tools/list surface (Phase D+) when the reflective walk traces back from `@mcp/tool` annotations through the substrate-graph closure to find all reachable tool declarations.

### §8.3 Build-system ancestry

- **Dolstra 2006** — Eelco Dolstra, *The Purely Functional Software Deployment Model* (TU Delft PhD dissertation, 2006). Foundational Nix text. The `@mirror/store` Apache-2.0 rock-solid floor per collapse-spec §11 grounds on Nix's content-addressed-derivations discipline (ca-derivations arc 2020-2023 grounds do-not-bolt-immutable-on-later warning); rust/src/mcp.rs session-gen_prism ref persistence composes over the Nix-derived store discipline.

- **Bazel REAPI (Remote Execution API)** — Google Bazel team, https://github.com/bazelbuild/remote-apis. Bazel's Content-Addressable Storage (CAS) + Action Cache split codified; `@mirror/store`'s six-op wire (read/write/exists/diff/walk/impacted_by) matches REAPI CAS surface modulo naming; `@mirror/store/action_cache` (bootstrap/src/action_cache.rs 15.5KB LANDED-EMPIRICAL per Taut §1) IS REAPI ActionCache floor at bootstrap altitude; Phase D+ session-gen_prism ref persistence composes over REAPI-shaped storage discipline.

- **Mokhov, Mitchell, Peyton Jones 2020** — Andrey Mokhov, Neil Mitchell, Simon Peyton Jones, *Build Systems à la Carte* (Journal of Functional Programming 30:e11, 2020). Canonical (scheduler × rebuilder) taxonomy grid; `@mirror/store` is scheduler-agnostic; the M4 MCP surface composes at the storage-plus-reload altitude which sits BELOW the scheduler-rebuilder axis (the MCP tools invoke build-system-like operations mirror_compile + mirror_craft + mirror_kintsugi; the reload contract at Phase D lifts to notifications when the tool surface itself drifts).

### §8.4 Substrate authorship history

- **`bootstrap/src/mcp.rs::serve_loop`** — authored by Reed 2026-07-15 (46.6KB, per grep-verified modification date). Byte-parity 8-tool schema per Mara iter-15 2026-07-08 (`4f4a257` mirror_spawn → mirror_peer_beam + top-level mirror_beam rename); Tick 7 shatter fold (`ffba2a7` kintsugi always `--ci --out @data/json`); Arc-1 Tick 1.4 `mirror_beam_act` addition (per docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md). THIS spec's Phase B port preserves the schema at 9-tool (bootstrap current + Reed nearly-today `mirror_roomba`).

- **`boot/std/mcp.mirror`** — authored 2026-07-12 (6.6KB); three bilateral-predicate contracts (dispatch_reflects_cli_block + tools_reflects_cli_block + frame_relativity) landed Tick 6 substrate closure (`d4c9a32`) per Option A per Taut scout `cf5ab8c` LRM verdict.

- **`bin/mirror-mcp`** — collapsed 2026-07-08 Tick 6.5 from 149-line bash to 20-line shim (`edef415`); post-Mara-iter-15 byte-parity migration to rust-hosted mcp.rs.

- **`docs/specs/mcp-spec-song-collapse.md`** — authored by Mara 2026-07-06 (119.8KB, 2551 LOC). THIS spec discharges collapse-spec's M4 milestone forward-promise per collapse-spec §10.4 ("Sub-arc M4: lambda shell as MCP client") with lambda-shell.md DEPRECATED-FOR-RUST-REWRITE handling per Mara 2026-07-17 marker.

- **`docs/specs/lsp-and-mcp.md`** — authored by Reed 2026-06-04 (16.2KB); names `mirror serve --mcp` unified surface target + `@mcp/tool` annotation discipline + `@mirror/reload` gen_prism auto-reload contract. THIS spec discharges lsp-and-mcp.md forward-promises (1)-(4).

- **`docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`** — authored by Mara 2026-07-17 (`2519f83`, then `81294b3` rewrite; 57.1KB). Three-file terminal geometry canonical (phone.rs + matrix.rs + main.rs); §2.2 M4 milestone tick named `mirror serve --mcp` empirical landing.

- **`docs/specs/rust-floor-five-file-terminal-geometry-extension.md`** — authored by Mara 2026-07-20 (26.3KB); extends three-file to five-file (adds compile.rs + liquid.rs). THIS spec extends further to six-file (adds mcp.rs at explicit altitude); Round-2 discipline preserved (main.rs stays pure delegation; new file lands at explicit altitude with responsibility declaration).

### §8.5 Recognition ancestry

- **Recognition RATIFIED 2026-08-03 crown-theorem** — `#R-reality-as-5d-quantum-foam-of-spinning-nodes` per docs/recognition/2026-08-03-reality-as-5d-spinning-foam.md; both witness gates CLOSED per Alex Q-CRN-1 promote-now adjudication. Grounds Alex+Lore+Anna empirical triangle at compilation-altitude ancestor of THIS spec's substrate; Anna Jakobs 2012 as PRIMARY computational-substrate ancestor per Fivefold Equivalence Theorem 5.5.

- **Recognition #58 (Fate-is-optical-inference, PROMOTED 2026-06-11)** — grounds Phase E `mirror kintsugi @spec` spawn's Fate multi-frequency tournament at each temporal step per collapse-spec §5.2 mechanism.

- **Recognition #84 (@pack.spawn, LANDED)** — pack-altitude substrate primitive per `shards/pack.mirror:263`; Phase E composition through `shards/mirror/peer/beam.mirror` composition ancestry (4-step: @pack.spawn → @song return-type upgrade M2 → @fate hinge composition M-CLEAN → @song/movement.enter binding).

- **Recognition #S2 (shift-at-temporal, CANDIDATE)** — `shards/song.mirror` §S2 family-root CANDIDATE grounded empirically at MCP altitude per collapse-spec §5 (Fate multi-frequency IS shift-at-temporal); THIS spec's Phase E landing composes at MCP altitude for the empirical witness.

- **Recognition #S3 (five-op-temporal-specialisation, LANDED)** — `@song`'s five-op temporal specialisation IS the time-evolution operator U(t) per collapse-spec §4.3; Phase E `mirror kintsugi @spec` spawn IS the empirical instantiation of U(t) at MCP altitude.

- **Recognition #S4 (MCP-session-IS-gen_prism, per collapse-spec §9 promotion)** — the MCP session state carrier IS `@mirror/runtime/gen_prism`; THIS spec's §2.2 item 4 substrate-decl'd session-state gen_prism composes over Recognition #S4.

- **Recognition #43 (content-addressed build system, per collapse-spec §3.7.3)** — grounds the DAG-as-source-of-truth discipline the M4 milestone extends: rust/src/mcp.rs's session-state persistence via @mirror/store crystals extends Recognition #43's substrate to MCP-session altitude.

### §8.6 Composition anchors (grep-verified in THIS spec)

**Substrate carriers (LANDED-EMPIRICAL):**
- `rust/src/main.rs` (66.0KB, 2026-07-28); `rust/src/phone.rs` (69.6KB, 2026-07-22)
- `rust/matrix/src/lib.rs` (60.7KB); `rust/matrix/src/book.rs` (10.8KB); `rust/matrix/src/void.rs` (20.2KB)
- `rust/roomba/src/mend.rs` (40.3KB); `rust/fractal/src/crystal.rs` (8.8KB)
- `bootstrap/src/mcp.rs` (46.6KB); `bootstrap/src/crystallize.rs` (42.8KB); `bootstrap/src/action_cache.rs` (15.5KB); `bootstrap/src/apply_h.rs` (81.4KB); `bootstrap/src/index.rs` (32.7KB)

**Substrate carriers (LANDED-SPEC-ONLY):**
- `shards/mirror/store.mirror` (46.5KB, 2026-07-17); `shards/kintsugi/roomba.mirror` (46.4KB, 2026-07-17)
- `shards/mirror/peer/beam.mirror` (15.9KB, 2026-07-12); `shards/mirror/lens/cli/sh.mirror` (9.3KB, 2026-06-12); `shards/mirror/lens/cli/kintsugi.mirror` (16.3KB)
- `shards/torus.mirror` (30.3KB, 2026-08-03; crown-theorem cascade); `shards/fate.mirror`; `shards/pack.mirror`
- `boot/std/mcp.mirror` (6.6KB); `boot/std/mirror/serve.mirror` (192B); `boot/std/mirror/reload.mirror` (2.0KB); `boot/std/mirror/lsp.mirror` (978B)

**Spec composition (CITED):**
- `docs/specs/mcp-spec-song-collapse.md` (Mara 2026-07-06, 119.8KB, 2551 LOC)
- `docs/specs/lsp-and-mcp.md` (Reed 2026-06-04, 16.2KB)
- `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` (Mara `81294b3` 2026-07-17, 57.1KB)
- `docs/specs/rust-floor-five-file-terminal-geometry-extension.md` (Mara 2026-07-20, 26.3KB)
- `docs/specs/mirror-runtime-gen-prism.md` (referenced; gen_prism primitive spec)
- `docs/specs/lambda-shell.md` (DEPRECATED-FOR-RUST-REWRITE per Mara 2026-07-17; archaeology retained)

**Scout composition (CITED):**
- `docs/scouts/2026-08-03-taut-mcp-spawn-full-stack-scout.md` (Taut `64e8d60` 2026-08-03) — ground-truth 4-phase grep-verify scout Layer 0-4 with LANDED-EMPIRICAL/LANDED-SPEC/STUB/GAP classification across 30+ substrate surfaces; §5 gap map + §6 smallest-empirical-spawn recommendation + §8 five [ALEX-Q] surfaced

**Recognition composition (CITED):**
- `docs/recognition/2026-08-03-reality-as-5d-spinning-foam.md` (Reed R-CRN-A1 2026-08-03, 158 lines; both witness gates CLOSED)
- `docs/math/2026-08-03-mara-reality-as-spinning-5d-foam-crown-theorem.md` (Mara crown-doc 614 LOC, 8 sections; Fivefold Equivalence Theorem 5.5)

**Alex verbatim (LOAD-BEARING):**
- Alex 2026-08-03 Option C: "Fire nearly-today Reed path + spawn Mara canonical spec in parallel."
- Alex 2026-07-17 terminal-geometry ratification: "Yes. That is the terminal geometry. I agree fully. And it was always right there in front of us. And now we see it."
- Alex 2026-07-17 bootstrap-detachment: "I also want to detach bootstrap completely from the execution path. If that means the compiler breaks, then the compiler breaks."
- Alex 2026-07-06 (per collapse-spec §10.6): "I want the floor to be rock solid and useful in agentic workflows, even without the @spectral/db magic."

---

*rust/src/mcp.rs at rust/ altitude terminal geometry. Sibling of phone.rs + matrix.rs + main.rs + compile.rs + liquid.rs. Six files. Six altitudes. Every altitude has exactly one file. The composition chain is main → mcp → compile/phone/liquid/apply_h::act. Zero new substrate mints. Phase A ships nearly-today per Reed delegation stub; Phase B ports serve_loop to rust/ altitude; Phase C retires bin/mirror-mcp shim; Phase D lands @mirror/reload gen_prism per M5; Phase E lands `mirror kintsugi @spec` spawn per M6. The M4 milestone closes at rust/ altitude. The ouroboros closes.*
