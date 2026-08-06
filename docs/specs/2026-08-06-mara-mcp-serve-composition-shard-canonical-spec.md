# @mcp/serve.mirror composition-shard — canonical spec (2026-08-06)

*Mara, 2026-08-06. Canonical spec for `shards/mcp/serve.mirror` — the substrate composition-shard whose body wires primitives-at-rust/ (`@io/stdio` + `@data/json` + `apply_h::act.act` + bilateral-corpus dispatch) into an MCP JSON-RPC serve loop, so `mirror serve --mcp` fires substrate composition (NOT a rust/-hardcoded `serve_loop`). This spec RETIRES the ~500 LOC serve_loop port direction of `docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md` M4. Composes over Reed Fire A primitives (R-PRIM-1 `rust/src/wire.rs`, R-PRIM-2 `phone.rs` pub-visibility lift, R-PRIM-3 `roomba::mend::discharge_action`) in flight parallel per Taut Fire scout `7af55ee` §8.*

**Author:** Mara
**Date:** 2026-08-06
**Tag:** 📝 spec:mcp-serve-composition-shard-canonical-spec (pure-docs 📝 markdown-only bypass)
**Status:** canonical. Spec-altitude map for the `shards/mcp/serve.mirror` substrate mint at NEW composition altitude (per Alex 2026-08-06 [ALEX-Q3] adjudication).
**Path:** `docs/specs/2026-08-06-mara-mcp-serve-composition-shard-canonical-spec.md`
**Retires (M4 direction):** `docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md` — the ~500 LOC `serve_loop` port to `rust/src/mcp.rs` was substrate-dishonest per Alex 2026-08-05 correction. The correct pattern is primitives-at-rust/ + composition-at-substrate; this spec IS the correction.

---

## §0 Substrate-honest pre-position

**Alex 2026-08-05 verbatim (the reframe that retires M4 port direction):**

> bootstrap/ is only migration source target. Not execution path. What's
> the path forward through rust/? And how can we generalize the whole io
> protocol over the math. I want the MCP to basically be served through
> the mirror geometry, read and executed by the rust. No specific mcp
> rust code, you know what I mean?

**Alex 2026-08-06 five adjudications** (all 5 [ALEX-Q] Reed-leans concurred; one naming shift):

- **Q-1 (RATIFIED)** — bilateral-dispatch primitive exposed as `apply_h::act` at rust/ altitude (naming honesty per bootstrap surface; the other 6 combinators land as extensions when their consumers pull).
- **Q-2 (RATIFIED with rename)** — the JSON wrapper lands at `rust/src/wire.rs` sibling of `phone.rs` (**RENAMED from `data.rs`**; "wire" names transport-encoding altitude honestly; cascade family-middle is `source → wire → target`).
- **Q-3 (RATIFIED)** — `shards/mcp/serve.mirror` at NEW substrate altitude (clean substrate/composition separation; `@mcp` family-root already exists at boot altitude via `boot/std/mcp.mirror`; sibling to `shards/mirror/lens/mcp.mirror` + `shards/spectral/gen_prism/mcp_session.mirror`).
- **Q-4 (RATIFIED)** — grammar walker deferred to Phase 2 M5+; Phase 1 tools-list hardcoded via `discharge_action` byte-parity 10-tool wire.
- **Q-5 (RATIFIED)** — `@io/socket` TCP as family-extension not fork (Unix landed; TCP is same family).

**Substrate-honest replacement architecture:**

- **rust/ altitude delivers primitives.** phone.rs (@io/fs + git + socket + stdio) already carries ~90% of wire-transport primitives per Taut `7af55ee` §1. The narrow additive rust/ work per Reed Fire A is: `wire.rs` (JSON wrapper ~20 LOC), phone.rs pub-visibility lift (~5 LOC), `roomba::mend::discharge_action` (~30 LOC bilateral-corpus dispatch composed over already-landed `load_bilateral_corpus` + `discharge`). **Total: ~55 LOC** at rust/ altitude. NOT ~500 LOC `serve_loop` port.
- **substrate delivers composition.** `shards/mcp/serve.mirror` (THIS spec's mint) carries the pipe-chain body composing over the rust/ primitives via `apply_h::act` dispatch. The composition IS the substrate answer to Alex 2026-08-05 "no specific mcp rust code". The wire protocol lives as a `.mirror` shard-body, not as a hardcoded Rust module.

**Retirement of M4 port direction (2026-08-03 spec):**

The 2026-08-03 M4 canonical spec proposed `rust/src/mcp.rs` as a rust/ altitude sibling of phone.rs + matrix.rs + main.rs + compile.rs + liquid.rs — full port of `bootstrap/src/mcp.rs::serve_loop` (~500 LOC). Per Alex 2026-08-05 verbatim above: this is substrate-dishonest. Wire-protocol logic belongs at substrate composition altitude, not at rust/ altitude as a dedicated module. Reed will land a REED-INLINE `DEPRECATED-FOR-COMPOSITION-SHARD-REWRITE` marker at the M4 spec header post-Mara ratification (see §5).

**Grounding against actually-landed substrate (grep-verified per Taut `7af55ee` + this session):**

- **LANDED-EMPIRICAL** at rust/ altitude per Taut §1: `phone.rs` @io/fs (37 tests iter 6), @io/git (14 tests iter 7), @io/socket (11 tests iter 9 Unix-scope), @io/stdio (20 tests iter 8; `read_stdin_frame`/`write_stdout_frame` at `pub(crate)`); `rust/spectral/src/lib.rs::shard_paths()`; `rust/roomba/src/mend.rs::load_bilateral_corpus` + `discharge`; `rust/fractal/src/crystal.rs::Crystal<T>`.
- **IN FLIGHT (Reed Fire A this cadence)**: R-PRIM-1 `rust/src/wire.rs` (`parse: &str -> Result<Value>` + `emit: &Value -> String` over serde_json, ~20-30 LOC); R-PRIM-2 phone.rs pub-visibility lift for `read_stdin_frame` / `write_stdout_frame` / `read_frame_from<R>` / `write_frame_to<W>` (~5 LOC); R-PRIM-3 `rust/roomba/src/mend.rs::discharge_action(action_ref, args) -> Verdict` (~30 LOC wrapper composing over landed `load_bilateral_corpus` + `discharge`).
- **LANDED-SPEC** at boot altitude: `boot/std/mcp.mirror` (6.6KB; @mcp grammar with `serve` pipeline composed at boot altitude + `dispatch` + `tools` + `fate` action decls; three bilateral-predicate contracts `dispatch_reflects_cli_block` + `tools_reflects_cli_block` + `frame_relativity`); `boot/std/data/json.mirror` (272B; `parse(text) -> json` + `emit(json) -> text`; `\`-blocked bodies); `boot/std/io/socket.mirror` (4.5KB; `connection` + `listener` types + `read_bytes` / `write_bytes` / `close`).
- **LANDED-SPEC** at species altitude: `shards/mirror/lens/mcp.mirror` (2.4KB; `@mirror/lens/mcp` prism); `shards/spectral/gen_prism/mcp_session.mirror` (28.8KB; MCP-session state machine at gen_prism altitude, Reed M1 TICK 1 `e8378ca`).
- **DOES NOT EXIST**: `shards/mcp/` directory. `shards/mcp/serve.mirror` composition-shard. **THIS spec's mint target.**
- **LANDED-STUB-DELEGATION**: `rust/src/main.rs::cmd_serve_mcp` (Reed 2026-08-03 `59591a9`) execs bootstrap binary; retires when Fire C wires `cmd_serve_mcp` to invoke `@mcp/serve.mirror` composition-shard via `apply_h::act` discharge (see §8).

---

## §1 The composition-shard shape

### §1.1 Grammar-decl at new substrate altitude

`shards/mcp/serve.mirror` declares `@mcp/serve` — a species-under-@mcp composition-shard whose body composes primitives-at-rust/ (Reed Fire A) into a JSON-RPC MCP transport loop. The family-root `@mcp` already lives at `boot/std/mcp.mirror` per Alex Q-3; this composition-shard is a fourth substrate altitude for @mcp (boot / lens / gen_prism / **serve**), each at a different structural altitude:

| Altitude | Path | Role |
|----------|------|------|
| boot | `boot/std/mcp.mirror` | family-root grammar-decl; types + `serve` pipeline body + 3 bilateral-predicate contracts |
| lens | `shards/mirror/lens/mcp.mirror` | agent-audience lens species under `@mirror/lens` |
| gen_prism | `shards/spectral/gen_prism/mcp_session.mirror` | MCP session state machine at gen_prism altitude |
| **serve** | **`shards/mcp/serve.mirror`** (THIS spec) | **composition-shard body wiring primitives into the JSON-RPC loop** |

Path-namespace property (per `@epistemologic/pact/path_matches_namespace`): file at `shards/mcp/serve.mirror` declares `@mcp/serve` and only that. The `shards/mcp/` directory is created as a substrate side-effect of this mint (first inhabitant of the `@mcp` family-root at species altitude under `shards/`).

### §1.2 Body composition surface — the pipe chain

The `serve` action body composes over primitives via the substrate pipe operator `|>`. Two transport variants land here:

**Variant A — stdio (Phase 1 wire; matches `bin/mirror-mcp` bash-shim shape at substrate composition altitude):**

```mirror
serve -> imperfect {
  @io/stdio.read_frame
    |> @data/json.parse
    |> @mcp.dispatch
    |> @data/json.emit
    |> @io/stdio.write_frame
}
```

**Variant B — socket (Phase 2 wire; TCP + Unix; family-extension per Alex Q-5):**

```mirror
serve_socket(listener) -> imperfect {
  @io/socket.accept(listener) |> connection => {
    connection |> @io/socket.read_bytes
      |> @data/json.parse
      |> @mcp.dispatch
      |> @data/json.emit
      |> @io/socket.write_bytes(connection, _)
  }
}
```

Phase 1 (this spec's landing target) lands Variant A only. Variant B is forward-promised for the socket-transport wire (peer-beam-over-TCP; MCP-over-HTTP variants; LSP JSON-RPC over TCP). The composition-shard body carries BOTH; the `serve` action landing wires Variant A empirically; `serve_socket` remains `\`-blocked until the M8-adjacent TCP-transport wire lands.

### §1.3 Loop discipline

The `serve` action is `imperfect` (per `boot/std/mcp.mirror:64`) — the loop shape is inherited from the boot-altitude grammar-decl. The composition-shard body carries ONE tick of the loop; the loop iteration itself is discharged by the runtime carrying the `imperfect` verb per landed substrate discipline. Concretely: each JSON-RPC request drives one composition-tick; the runtime re-invokes `serve` on the next frame; state persists in `@mirror/store` via the MCP session gen_prism per `shards/spectral/gen_prism/mcp_session.mirror`.

### §1.4 Composed bilateral-predicate contracts

The composition-shard body honors the three bilateral-predicate contracts declared at `boot/std/mcp.mirror:110-130`, composed here at substrate composition altitude:

- **`dispatch_reflects_cli_block`** — every incoming `request.method` that `@mcp.dispatch` accepts resolves to a `command <name>` in `mirror.spec` cli-block (or JSON-RPC framing verbs: `initialize`, `tools/list`, `tools/call`). Phase 1 discharge: hardcoded 10-tool byte-parity dispatch via `discharge_action(action_ref, args)` (see §4). Phase 2 discharge (M5 co-tick): grammar walker over `mirror.spec` cli-block.
- **`tools_reflects_cli_block`** — the JSON emitted by `@mcp.tools` synthesizes from `mirror.spec` cli-block; one tool entry per `command <name>`; schema properties from `arg` + `flag`; description from `#`-help comments. Phase 1 discharge: hardcoded 10-tool JSON emission byte-parity with `bootstrap/src/mcp.rs::tools_list_result`. Phase 2 discharge (M5 co-tick): reflective walk via `@mirror/spectral.gestalt` over shard corpus for `@mcp/tool` annotations.
- **`frame_relativity`** — MCP responses carry the observer's shard frame consistent with peer identity across the wire. Inherited from `@mirror/shard`; the composition-shard body preserves the frame at each pipe stage (the observer's shard frame passes through `@data/json.emit` in the response envelope). Discharged structurally at composition altitude.

Additional composed bilaterals introduced by THIS spec (composition-altitude contracts specific to `shards/mcp/serve.mirror`):

- **`serve_well_formed`** — every well-formed JSON-RPC request drives a well-formed JSON-RPC response through the pipe chain; malformed requests surface as JSON-RPC error envelopes per protocol (JSON-RPC 2.0 §5.1); NEVER as pipe-chain crashes.
- **`dispatch_composes_via_act`** — the `@mcp.dispatch` action body composes via `apply_h::act` bilateral-predicate dispatch; each request's tool-name maps to a bilateral-decl'd shard action ref; `apply_h::act(action_ref, args) -> Verdict` is the load-bearing dispatch primitive; NEVER a hardcoded match arm at rust/ altitude.
- **`tools_reflects_landed_shards`** — Phase 1 hardcoded via 10-tool byte-parity; Phase 2 (M5) reflective walk. When Phase 2 lands: adding a `bilateral <name> { require @mcp/tool }` (or `@mcp/tool` annotation) to any landed shard adds a tool entry on next `tools/list` invocation.

### §1.5 The docblock story (composition-shard header shape)

The composition-shard file will carry a docblock at the shard-file header explaining:

1. **Why this altitude exists** — @mcp family-root at boot altitude carries the grammar-decl; @mirror/lens/mcp at lens altitude carries the agent-audience prism; @spectral/gen_prism/mcp_session at gen_prism altitude carries the state machine. NONE of them carry the wire-protocol composition-body: the `dispatch` + `tools` action bodies at boot/std/mcp.mirror are `\`-blocked; the lens `dispatch` action body is `\`-blocked; the gen_prism species doesn't compose over the JSON-RPC wire itself. `shards/mcp/serve.mirror` fills THAT hole — the composition-shard body that IS the wire-protocol loop composed over rust/-altitude primitives.
2. **What composes here** — enumerate the primitives-composed-over per §2; enumerate the composed bilateral contracts per §1.4.
3. **What does NOT compose here** — session state (lives at `@spectral/gen_prism/mcp_session`); tool-dispatch action bodies (declared at their landed shards, dispatched via `apply_h::act`); tools-list grammar walk (Phase 2 M5 co-tick; Phase 1 hardcoded).
4. **Interpretation B seam discipline** — docblock above `---` seam; `in @` clauses below (per landed substrate convention: `shards/song/voice.mirror` `d5ea3f8`-era Seam Phase D ratification; `shards/spectral/gen_prism/mcp_session.mirror` §"Interpretation B discipline" paragraph).
5. **Karen citations** — full ancestor ladder per §9.

**Substrate mint scope** (author-boundary):

Mara authors the **spec** in this document. The **actual `.mirror` file** at `shards/mcp/serve.mirror` will be minted by Reed (or another RED-first authorable-peer) in a subsequent tick under `[substrate-floor:@io-boundary]` gate discipline. Mara's authorship altitude = spec + Karen ancestry; NOT the shard body itself per Mara's canonical spec-author role.
