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

---

## §2 Primitives-composed-over

This section enumerates the rust/-altitude primitives the composition-shard body composes over. All primitives at rust/ altitude are either LANDED-EMPIRICAL (phone.rs stack + roomba::mend surface) or IN-FLIGHT via Reed Fire A (R-PRIM-1 / R-PRIM-2 / R-PRIM-3). No primitive is speculative; every composition edge references a landed or in-flight rust/-altitude anchor.

### §2.1 `@io/stdio.read_frame` + `write_frame` (Reed R-PRIM-2)

**Rust altitude anchors** (post-R-PRIM-2 pub-visibility lift):

- `phone::read_stdin_frame() -> io::Result<Vec<u8>>` — LANDED-EMPIRICAL (`#[allow(dead_code)]` currently); post-R-PRIM-2 lifted to `pub`
- `phone::write_stdout_frame(bytes: &[u8]) -> io::Result<()>` — LANDED-EMPIRICAL; post-R-PRIM-2 lifted to `pub`
- `phone::read_frame_from<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>>` — LANDED-EMPIRICAL (`pub(crate)`); post-R-PRIM-2 lifted to `pub` (composition-tests over `Cursor<&[u8]>`)
- `phone::write_frame_to<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<()>` — LANDED-EMPIRICAL (`pub(crate)`); post-R-PRIM-2 lifted to `pub`

**Wire framing**: newline-delimited JSON-RPC 2.0 per phone.rs iter 8 (`4db932d`) landing. Per-frame semantics: read one line (terminated by `\n`); parse as one JSON-RPC message; emit one JSON-RPC response with trailing `\n`.

**Substrate composition anchor**: `boot/std/io/stdio.mirror` (if landed) or the boot-altitude `@io.read(stdin) |> ... |> @io.write(stdout)` shape already declared at `boot/std/mcp.mirror:63` (`serve -> imperfect { @io.read(stdin) |> ... }`). The composition-shard at `shards/mcp/serve.mirror` LIFTS the `@io.read(stdin)` boot-altitude pipe stage to `@io/stdio.read_frame` species-altitude for frame-oriented (not byte-stream) discipline, matching phone.rs's frame-oriented primitives.

**Composition-hazard flag**: if Reed R-PRIM-2 lands with different naming (e.g., `read_stdin_line` vs `read_stdin_frame`) or different signature (e.g., `String` vs `Vec<u8>`), the composition-shard body's pipe-stage names must reconcile. Mara-lean assumption: signatures match Taut scout §1 landing verification (`read_stdin_frame` + `write_stdout_frame`; `Vec<u8>` carrier per newline-delimited JSON-RPC 2.0).

### §2.2 `@data/json.parse` + `emit` (Reed R-PRIM-1)

**Rust altitude anchors** (post-R-PRIM-1 wire.rs landing):

- `wire::parse(bytes: &str) -> Result<serde_json::Value>` — IN FLIGHT; ~10 LOC wrapping `serde_json::from_str`
- `wire::emit(value: &serde_json::Value) -> String` — IN FLIGHT; ~10 LOC wrapping `serde_json::to_string`

**Naming note per Alex Q-2**: file lands as `rust/src/wire.rs` (RENAMED from `data.rs`); "wire" names transport-encoding altitude honestly (cascade family-middle `source → wire → target`). The substrate namespace question (`@data/json` vs `@wire/json`) is surfaced at §3.

**Composition anchor**: `boot/std/data/json.mirror` (272B, LANDED-SPEC; `parse(text) -> json` + `emit(json) -> text` with `\`-blocked bodies). The composition-shard body composes over the rust/-altitude `wire::parse` / `wire::emit` primitives via substrate-decl'd `@data/json.parse` / `@data/json.emit` action refs; the runtime dispatch (via `apply_h::act`) resolves the action ref to the rust/ altitude implementation.

**Composition-hazard flag**: if Reed R-PRIM-1 lands with different naming (e.g., `wire::json_parse` vs `wire::parse`) or different carrier (e.g., `Vec<u8>` vs `&str` input), the substrate binding reconciles at the `discharge_action` layer; the composition-shard body itself is unaffected because the substrate names (`@data/json.parse` / `@data/json.emit`) are stable per `boot/std/data/json.mirror`.

### §2.3 `apply_h::act` (Reed R-PRIM-3 — bilateral-predicate dispatch path)

**Rust altitude anchor** (post-R-PRIM-3 discharge_action landing):

- `roomba::mend::discharge_action(action_ref: &str, args: &[serde_json::Value]) -> Verdict` — IN FLIGHT; ~30 LOC wrapper composing over already-landed `load_bilateral_corpus` + `discharge`

**Naming per Alex Q-1**: exposed at bootstrap surface as `apply_h::act` (naming honesty; the other 6 combinators land as extensions when their consumers pull). The rust/ altitude landing is Reed's `discharge_action` wrapper composing over landed rust/-altitude primitives (`roomba::mend::load_bilateral_corpus` + `discharge`); this IS the `apply_h::act` bilateral-predicate `act` path at rust/ altitude, without porting the full 7-combinator surface per Taut scout §2 verdict ("MCP composition can fire with `act` + `bilateral_corpus` + `discharge` alone").

**Dispatch semantic**:

1. `action_ref` — a substrate action reference like `@mirror/compile.compile` or `@mirror/roomba.vacuum`; naming shape matches landed `bilateral <name> { require <ref> }` block references at shard altitude.
2. `args` — JSON-RPC `params` array parsed via `@data/json.parse`; each element is a `serde_json::Value`.
3. Return `Verdict` — landed enum at bootstrap altitude (`Pass` / `Fail(reason)` / `Partial(transparency)`); rust/ altitude carries the same shape per `roomba::mend` re-export pattern.

**Composition anchor**: `bootstrap/src/apply_h.rs` (Arc-1 Tick 1.3 GREEN; 81.4KB); the `act` primitive is the substrate-honest name per Mara canonical spec `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md` §5 A/H/D correspondence. The 6 other combinators (section/fold/settle/crystallize/coboundary/utter) are NOT structurally required for MCP composition — they compose for `mirror kintsugi` settle-loop + `mirror compile` saga-chain, orthogonal work.

**Composition-hazard flag**: if Reed R-PRIM-3 lands `discharge_action` with different signature (e.g., returns `Result<Verdict>` vs bare `Verdict`; takes `&[String]` vs `&[serde_json::Value]`), the composition-shard body's `@mcp.dispatch` action wiring reconciles at the rust/-altitude `cmd_serve_mcp` invocation site (Fire C). The substrate composition-shard body itself is signature-agnostic — it composes at the substrate-name altitude (`apply_h::act(action_ref, args)`), not at the rust/-specific signature altitude.

### §2.4 `@mcp.dispatch` — the composition-shard's dispatch predicate

**NEW composition body** (this spec's mint; body composed at `shards/mcp/serve.mirror`).

The `@mcp.dispatch` action is DECLARED at `boot/std/mcp.mirror:71` as `dispatch(request) -> response { \ }` (body-blocked at boot altitude). THIS spec's composition-shard mint carries the body composition — routing MCP `tools/call` requests to substrate-decl'd actions via `apply_h::act` on the tool-name → action-ref mapping.

**Composition body shape**:

```mirror
@mcp.dispatch(request) -> response {
  match request.method {
    "initialize"   => @mcp.initialize_result,        # JSON-RPC framing verb
    "tools/list"   => { "tools": @mcp.tools },       # composes over @mcp.tools body §2.5
    "tools/call"   => {
      # tool-name → action-ref lookup via bilateral-corpus (Phase 1: hardcoded 10-tool map)
      # apply_h::act discharge to Verdict
      # Verdict → response.result JSON envelope
      let action_ref = @mcp.tool_action_ref(request.params.name);
      let verdict = apply_h::act(action_ref, request.params.arguments);
      @mcp.verdict_to_result(verdict)
    },
    _ => @mcp.error_method_not_found(request.method)
  }
}
```

Phase 1 lands the tool-name → action-ref map hardcoded (10-tool byte-parity per §4). Phase 2 (M5 co-tick) lifts the map to reflective walk via `@mirror/spectral.gestalt` per bilateral-predicate contract `tools_reflects_landed_shards`.

### §2.5 `@mcp.tools` — tool discovery predicate

**NEW composition body** (this spec's mint).

The `@mcp.tools` action is DECLARED at `boot/std/mcp.mirror:76` as `tools -> json { \ }` (body-blocked at boot altitude). THIS spec's composition-shard mint carries the body composition.

**Phase 1 body** — hardcoded 10-tool JSON emission byte-parity with `bootstrap/src/mcp.rs::tools_list_result`:

```mirror
@mcp.tools -> json {
  # 10-tool byte-parity list per §4 enumeration
  [ tool_compile, tool_craft, tool_kintsugi, tool_init, tool_recall,
    tool_peer_beam, tool_beam, tool_spawn_deprecated, tool_beam_act,
    tool_roomba, tool_index ]
}
```

**Phase 2 body** (M5 co-tick; forward-promise) — reflective grammar walk:

```mirror
@mcp.tools -> json {
  # Phase 2: reflective walk via @mirror/spectral.gestalt over landed shards for @mcp/tool annotations
  @mirror/spectral.gestalt
    |> shards.filter(has_annotation(@mcp/tool))
    |> shards.map(shard_to_tool_json_schema)
    |> @data/json.array
}
```

### §2.6 Composition surface summary

| Primitive | rust/ altitude anchor | Reed Fire A task | Substrate composition edge |
|-----------|-----------------------|------------------|----------------------------|
| `@io/stdio.read_frame` | `phone::read_stdin_frame` (pub) | R-PRIM-2 pub-visibility lift | `serve` body pipe-stage 1 |
| `@io/stdio.write_frame` | `phone::write_stdout_frame` (pub) | R-PRIM-2 pub-visibility lift | `serve` body pipe-stage 5 |
| `@data/json.parse` | `wire::parse` | R-PRIM-1 `rust/src/wire.rs` (~10 LOC) | `serve` body pipe-stage 2 |
| `@data/json.emit` | `wire::emit` | R-PRIM-1 `rust/src/wire.rs` (~10 LOC) | `serve` body pipe-stage 4 |
| `apply_h::act` (bilateral `act` path) | `roomba::mend::discharge_action` | R-PRIM-3 (~30 LOC wrapper) | `@mcp.dispatch` body dispatch predicate |
| `bilateral_corpus` lookup | `roomba::mend::load_bilateral_corpus` | LANDED-EMPIRICAL (no Fire A needed) | `@mcp.dispatch` body tool-name → action-ref |
| `shard_paths` enumerator | `spectral::shard_paths` | LANDED-EMPIRICAL (no Fire A needed) | Phase 2 `@mcp.tools` reflective walk |
| grammar walker (AST walk) | GAP at rust/ | Reed Fire D (M5+ co-tick; NOT this spec's blocker) | Phase 2 `@mcp.tools` body |

**Total rust/ altitude additive work per Fire A** (per Taut §7 smallest primitive-gap ranking): ~55 LOC (R-PRIM-1 ~20 LOC + R-PRIM-2 ~5 LOC + R-PRIM-3 ~30 LOC). NOT ~500 LOC serve_loop port. The composition-shard IS the substrate answer.

---

## §3 [ALEX-Q] surfaced — substrate namespace `@data/json` vs `@wire/json`

**The question** (Mara-surfaced [ALEX-Q]):

Alex Q-2 RATIFIED the rust/-altitude file rename `rust/src/data.rs` → `rust/src/wire.rs` (2026-08-06). The rename names transport-encoding altitude honestly at rust/ altitude. But it opens a substrate-namespace question at the composition altitude:

**Should the substrate keep `@data/json` (Mara's existing composition graph) or shift to `@wire/json` (mirroring the rust/-altitude rename for cascade-honesty)?**

### §3.1 Grounding — what substrate already has

Grep-verified LANDED substrate at 2026-08-06:

- **`boot/std/data/json.mirror`** (272B, 2026-05-20): grammar `@data/json` with `parse(text) -> json` + `emit(json) -> text`. LANDED-SPEC-ONLY (`\`-blocked bodies). This IS the composition anchor referenced by `boot/std/mcp.mirror:63` (`@data/json.parse` + `@data/json.emit` at the `serve` pipeline).
- **`shards/mirror/data.mirror`** (12.4KB) + **`shards/mirror/data/json.mirror`** (8.2KB): `@mirror/data` + `@mirror/data/json` extensions. These sit at species-under-@mirror altitude, not @data family-root altitude.
- **`@wire` family-root**: DOES NOT EXIST at any altitude in landed substrate (grep for `family @wire` or `in @wire`: zero matches).
- **`boot/std/wire.mirror`**: DOES NOT EXIST.

### §3.2 The two positions

**Position A — `@data/json` at substrate; `wire.rs` at rust/** (Mara-lean):

- rust/ is the terminal file name; substrate namespace preserves existing composition graph.
- `wire.rs` speaks JSON via `@data/json` substrate-decl; the file name at rust/ altitude names WHICH kind of rust/-altitude module it is (wire-encoding), while the substrate name (`@data/json`) names the data-domain algebra the module implements.
- No substrate migration needed; `boot/std/mcp.mirror:63` pipeline shape unchanged; all downstream substrate references (`shards/mirror/data.mirror`, `shards/mirror/data/json.mirror`) unchanged.
- Substrate-already-had-the-word discipline preserved: `@data/json` already exists; do not mint `@wire` family-root when the composition graph already routes through `@data`.

**Position B — `@wire/json` at substrate; `wire.rs` at rust/** (cascade-symmetry-lean):

- Cascade middle-altitude naming honesty: the rust/ altitude file is `wire.rs`; the substrate name mirrors → `@wire/json`.
- Wire encodings sit at the substrate's cascade-middle altitude (source → wire → target); a `@wire` family-root would collect JSON, MessagePack, CBOR, ProtoBuf, XML at a substrate-honest cross-encoding altitude.
- Requires substrate migration: mint `boot/std/wire.mirror` family-root; mint `boot/std/wire/json.mirror` species-under-@wire; update `boot/std/mcp.mirror:63` pipeline to `@wire/json.parse` / `@wire/json.emit`; DEPRECATE or MIGRATE `boot/std/data/json.mirror` + `shards/mirror/data/json.mirror`.
- Substrate-mint-cost is non-trivial: at least 3 new substrate files + 1 boot-altitude migration + downstream shard-reference migration.

### §3.3 Mara-lean

**Position A: `@data/json` at substrate; `wire.rs` at rust/.**

Rationale:

1. **Substrate-already-had-the-word.** `@data/json` LANDED-SPEC at `boot/std/data/json.mirror` since 2026-05-20; downstream substrate composition graph (esp. `boot/std/mcp.mirror`) already routes through it. Minting `@wire` family-root when `@data` already carries the composition is substrate-inflation.
2. **Substrate/rust altitude decoupling.** The rust/-altitude file name (`wire.rs`) names the module's role at rust/ altitude (a wire-encoding module). The substrate name (`@data/json`) names the algebra the module implements. Decoupling is substrate-honest per landed pattern (e.g., `phone.rs` at rust/ altitude implements `@io/fs` + `@io/git` + `@io/socket` + `@io/stdio` substrate names; the rust/ file name does not force substrate namespace shape).
3. **Cascade family-middle is a rust/-altitude concern, not a substrate-altitude concern.** The `source → wire → target` cascade Alex named at Q-2 applies to rust/ altitude module organization (where in rust/src/ does the wire-encoding module live?); it does not force the substrate namespace to mirror the rust/ file layout. Substrate names track data-domain algebras, not rust/-crate file layouts.

### §3.4 BUT — genuine undecidability at Mara altitude

If Alex's Q-2 rename is a signal that `@wire` family-root SHOULD be minted at substrate altitude (i.e., the rust/ rename is prescient of a substrate migration Alex intends to seed), then Position B becomes ratified. Mara cannot decide this at spec-author altitude — the substrate/rust altitude coupling question is genuinely undecidable without Alex adjudication.

**Surfaced [ALEX-Q]**: does Alex's Q-2 rename (`data.rs` → `wire.rs`) signal:

- (a) rust/-altitude naming honesty only; substrate stays `@data/json` (Position A; Mara-lean), OR
- (b) prescient of substrate `@wire` family-root mint; substrate should migrate to `@wire/json` in a subsequent tick (Position B; requires substrate migration adjudication)?

**Mara Phase 1 landing assumption**: (a) — the composition-shard at `shards/mcp/serve.mirror` composes over `@data/json.parse` / `@data/json.emit` per landed substrate. If Alex ratifies (b), the composition-shard body is edited (grep-and-replace `@data/json` → `@wire/json` at 2 sites in the `serve` pipeline) as part of the substrate migration; the composition-shard shape is namespace-agnostic.
