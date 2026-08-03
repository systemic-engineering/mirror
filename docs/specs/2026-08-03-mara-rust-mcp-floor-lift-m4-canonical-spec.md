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
