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

---

## §4 Tools list byte-parity — Phase 1 10-tool wire

**Byte-parity grep-verification** against `bootstrap/src/mcp.rs::tools_list_result` at HEAD (2026-08-06):

The bootstrap `tools_list_result` JSON emits **10 tools** (via `"name": "mirror_..."` entries grep-verified; see §4.1 enumeration). The `assertion vec!` at bootstrap `bootstrap/src/mcp.rs:876-887` asserts **9 stable names + mirror_index** (10 total counting mirror_spawn deprecated alias). Byte-parity Phase 1 discharge of the `tools_reflects_cli_block` bilateral contract MUST emit these 10 tools verbatim (naming + description + inputSchema).

### §4.1 The 10-tool wire (grep-verified enumeration)

Per grep of `bootstrap/src/mcp.rs` for `"name": "mirror_`:

| # | Tool name | CLI verb dispatch | Substrate anchor |
|---|-----------|-------------------|------------------|
| 1 | `mirror_compile` | `mirror compile <file>` | `@mirror/compile.compile` (`shards/mirror/compile.mirror`) |
| 2 | `mirror_craft` | `mirror craft <target> [--target-kind <k>] [--reflect]` | `@mirror/craft.craft` |
| 3 | `mirror_kintsugi` | `mirror kintsugi --ci --out @data/json <file> [--liquid] [--shatter N]` (Tick 7 shatter fold `ffba2a7`) | `@kintsugi.kintsugi` |
| 4 | `mirror_init` | `mirror init <path> [--install-hooks]` | `@mirror/init.init` (`docs/specs/mirror-init.md`) |
| 5 | `mirror_recall` | `mirror recall <spec_dir>` | `@mirror/recall.recall` (`docs/specs/mirror-recall.md`) |
| 6 | `mirror_peer_beam` | `mirror peer beam <peer_home> [flags…]` | `@mirror/peer/beam.beam` (Tick 3 rename `4f4a257` from mirror_spawn) |
| 7 | `mirror_beam` | `mirror beam --mission <mission>` | `@mirror/beam.beam` (mirror.spec top-level `command beam` `96aa752`) |
| 8 | `mirror_spawn` | `mirror spawn <peer_home>` (DEPRECATED alias per two-tick discipline) | `@mirror/peer/beam.beam` via `spawn` alias arm (`b012d3f`) |
| 9 | `mirror_beam_act` | `mirror beam act <ref> <predicate>` | `@mirror/beam/act.act` (Arc-1 Tick 1.4 `546c2f6`) |
| 10 | `mirror_roomba` | `mirror roomba --vacuum=<dir>` | `@kintsugi/roomba.vacuum` (Reed 2026-08-03 addition per Alex Option C) |
| 11 | `mirror_index` | `mirror index <path> [--fiedler] [--full-profile]` | `@mirror/index.index` (Rung 8 Landing 5 `77b8e14` + `317e830`) |

**Count reconciliation**: 11 tool entries in `tools_list_result` JSON emission. The bootstrap test assertion at `bootstrap/src/mcp.rs:876-887` asserts a 9-name `vec!` (excluding `mirror_spawn` alias + INCLUDING `mirror_index` per `77b8e14` Rung 8 landing). Effective wire byte-parity target: **10 unique tools** (mirror_compile / mirror_craft / mirror_kintsugi / mirror_init / mirror_recall / mirror_peer_beam / mirror_beam / mirror_beam_act / mirror_roomba / mirror_index) + `mirror_spawn` deprecated alias per two-tick discipline (11 total emitted; alias is DEPRECATED and will be removed in a subsequent tick per bootstrap docblock).

**Phase 1 landing target**: emit all 11 tool entries verbatim (schema + description + name) for byte-parity with bootstrap `tools_list_result`. The `mirror_spawn` alias retains DEPRECATED status per two-tick discipline; Phase 2 (M5 co-tick) reflective walk drops it when the alias arm retires at `bootstrap/src/mcp.rs`.

### §4.2 Composition body — Phase 1 hardcoded emission

The `@mcp.tools` action body at `shards/mcp/serve.mirror` (Phase 1 landing):

```mirror
@mcp.tools -> json {
  # Phase 1 (this spec) — hardcoded 11-tool byte-parity with bootstrap/src/mcp.rs::tools_list_result.
  # Discharges the tools_reflects_cli_block bilateral-predicate contract byte-parity-empirically;
  # substrate-structural discharge lifts at Phase 2 (M5 co-tick) via reflective grammar walk.
  [
    { "name": "mirror_compile",    "description": "focus: tokenize one .mirror file through grammar lens. Returns SHA-256 hash on success.",
      "inputSchema": { "type": "object", "properties": { "file": { "type": "string" } }, "required": ["file"] } },
    { "name": "mirror_craft",      "description": "split: converge a target directory to lambda_0. --target-kind emits code (binary|rust|gleam). --reflect verifies properties without emission.",
      "inputSchema": { … } },
    { "name": "mirror_kintsugi",   "description": "settle: kintsugi a .mirror file. --liquid writes inferred properties below ---. --shatter N seeds N cracks. `--ci` walks a corpus. Returns canonical source or typed verdict envelope.",
      "inputSchema": { … } },
    { "name": "mirror_init",       "description": "init: mirror-native store bootstrap. …", "inputSchema": { … } },
    { "name": "mirror_recall",     "description": "recall: inbound-trajectory dual of peer beam. …", "inputSchema": { … } },
    { "name": "mirror_peer_beam",  "description": "peer beam: the peer HAS a torus. …", "inputSchema": { … } },
    { "name": "mirror_beam",       "description": "beam: anonymous inference primitive. …", "inputSchema": { … } },
    { "name": "mirror_spawn",      "description": "DEPRECATED (2026-07-08): use mirror_peer_beam instead. …", "inputSchema": { … } },
    { "name": "mirror_beam_act",   "description": "beam act: dispatch a substrate-decl'd action against the 7-combinator evaluator surface (@apply_h). …", "inputSchema": { … } },
    { "name": "mirror_roomba",     "description": "roomba: substrate walker that back-projects to @mirror/store. …", "inputSchema": { … } },
    { "name": "mirror_index",      "description": "@mirror/fractal-coherence measurement: walk substrate DAG, compute graph Laplacian's top-16 eigenvalues …", "inputSchema": { … } }
  ]
}
```

**Full description + inputSchema verbatim from bootstrap/src/mcp.rs::tools_list_result** — the composition-shard body carries the full 11-entry byte-parity JSON literal. Reed's actual `.mirror` file mint at `shards/mcp/serve.mirror` copies the descriptions + inputSchema fields verbatim from bootstrap (grep-verified per §4.1); spec-altitude here elides the full schema for brevity.

### §4.3 Composition body — @mcp.dispatch Phase 1 hardcoded tool-name → action-ref map

The `@mcp.dispatch` action body at `shards/mcp/serve.mirror` for `tools/call` dispatch (Phase 1):

```mirror
@mcp.tool_action_ref(name) -> ref {
  # Phase 1 (this spec) — hardcoded tool-name → substrate action-ref map.
  # Byte-parity with bootstrap/src/mcp.rs::dispatch_tool_call match arms.
  match name {
    "mirror_compile"    => @mirror/compile.compile,
    "mirror_craft"      => @mirror/craft.craft,
    "mirror_kintsugi"   => @kintsugi.kintsugi_ci,       # --ci --out @data/json per Tick 7 fold
    "mirror_init"       => @mirror/init.init,
    "mirror_recall"     => @mirror/recall.recall,
    "mirror_peer_beam"  => @mirror/peer/beam.beam,
    "mirror_beam"       => @mirror/beam.beam,
    "mirror_spawn"      => @mirror/peer/beam.beam,      # DEPRECATED alias
    "mirror_beam_act"   => @mirror/beam/act.act,
    "mirror_roomba"     => @kintsugi/roomba.vacuum,
    "mirror_index"      => @mirror/index.index,
    _ => @mcp.error_tool_not_found(name)
  }
}
```

Phase 2 (M5 co-tick) lifts this map to reflective corpus walk via `roomba::mend::load_bilateral_corpus` + `@mirror/spectral.gestalt` — every landed shard with `@mcp/tool` annotation becomes a wire tool, dispatched via `apply_h::act(action_ref, args)`.

### §4.4 Grammar walker deferred to Phase 2 (M5 co-tick per Alex Q-4)

Alex Q-4 RATIFIED grammar walker deferral to Phase 2 M5+; Phase 1 tools-list hardcoded via `discharge_action` byte-parity 10-tool wire. Rationale (from Alex Q-4 concurrence with Reed-lean):

- Byte-parity Phase 1 establishes empirical continuity with landed bootstrap MCP wire; MCP clients (Claude Code, etc.) observe zero behavior change during rust/-altitude cut-over from bootstrap-exec-delegation to substrate-composition-native dispatch.
- Reflective walker requires additional rust/-altitude primitive (grammar walker / AST walk; GAP per Taut §3 Phase 3 verdict); Fire A + B + C is scoped to smallest primitive-gap closure per Taut §7 ranking. Grammar walker is Fire D (M5+ co-tick) NOT this spec's blocker.
- Bilateral-predicate contract `tools_reflects_cli_block` is discharged bilaterally-violated-by-design at Phase 1 (byte-parity carries the empirical claim; the reflective claim discharges structurally at Phase 2). TODO(M5) marker embedded in the composition-shard body; empirical firing at Phase 1 verifies the byte-parity claim; structural firing at Phase 2 verifies the reflective claim.

**Phase 2 M5 co-tick preconditions**:

1. **Grammar walker primitive at rust/ altitude** (Reed Fire D) — port `bootstrap/src/spectral.rs::Fold5` AST walker to rust/ altitude, OR compose over already-landed `roomba::mend::load_bilateral_corpus` extended to detect `@mcp/tool` annotations on shard bilateral blocks.
2. **`@mcp/tool` annotation mint** — first-class grammar annotation per `docs/specs/lsp-and-mcp.md`; shard bilateral blocks that carry `@mcp/tool` annotation become wire tools automatically.
3. **`@mirror/reload` gen_prism at rust/ altitude** (per `boot/std/mirror/reload.mirror`) — emits `notifications/tools/list_changed` on grammars_hash drift; MCP client re-queries `tools/list` on notification.

**Phase 2 M5 landing shape** (forward-promise; NOT this spec's landing target):

```mirror
@mcp.tools -> json {
  # Phase 2 M5 co-tick — reflective grammar walk.
  @mirror/spectral.gestalt
    |> shards.filter(has_annotation(@mcp/tool))
    |> shards.map(shard_to_tool_json_schema)
    |> @data/json.array
}

# @mirror/reload gen_prism fires on every request; emits notifications/tools/list_changed
# when grammars_hash drifts. MCP client re-queries tools/list on notification.
```

---

## §5 M4 direction retirement — DEPRECATED-FOR-COMPOSITION-SHARD-REWRITE

### §5.1 What the 2026-08-03 M4 canonical spec proposed

Per `docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md`:

- **Direction**: full port of `bootstrap/src/mcp.rs::serve_loop` (~500 LOC) to rust/ altitude as `rust/src/mcp.rs` — sixth rust/-altitude sibling of phone.rs / matrix.rs / main.rs / compile.rs / liquid.rs.
- **Scope**: rust/src/mcp.rs holds `serve_loop` natively; retires `bin/mirror-mcp` bash shim; `.mcp.json` points at `~/.local/bin/mirror` with `args: ["serve", "--mcp"]`; discharges Blocker A per Taut `64e8d60` scout §5 gap map (`"No mirror serve --mcp verb at rust/src/main.rs. MCP still routes through bin/mirror-mcp → bootstrap/mirror binary."`).
- **Substrate composition**: rust/src/mcp.rs at spec's §5 composition-into-existing-substrate table treated as substrate-honest additive extension (zero new substrate primitives; 16-row composition edge enumeration; four refusal candidates NOT taken per Michelangelo/marble).

### §5.2 What this spec substrate-honestly replaces

Per Alex 2026-08-05 verbatim reframe (§0 above): rust/ altitude serve_loop port is substrate-dishonest. Wire-protocol logic lives at substrate composition altitude, not rust/ altitude. The correct pattern is primitives-at-rust + composition-at-substrate:

| M4 (2026-08-03) proposed | THIS spec substrate-honestly replaces |
|--------------------------|---------------------------------------|
| `rust/src/mcp.rs` sixth rust/-altitude sibling (~400-700 LOC) | `shards/mcp/serve.mirror` composition-shard body (~200-400 lines substrate composition; no rust/ module) |
| `serve_loop` at rust/ altitude | `serve -> imperfect { pipe-chain }` composition at substrate altitude (§1.2 Variant A) |
| `tools_list_result` at rust/ altitude (11-tool hardcoded JSON) | `@mcp.tools -> json { hardcoded byte-parity Phase 1; reflective walk Phase 2 }` composition body (§4.2) |
| `dispatch_tool_call` at rust/ altitude (11-arm match) | `@mcp.dispatch(request) -> response { apply_h::act(tool_action_ref(name), args) }` composition body (§4.3) |
| ~500 LOC rust/ altitude serve_loop port | ~55 LOC rust/ altitude primitives per Reed Fire A (R-PRIM-1 wire.rs + R-PRIM-2 pub-visibility + R-PRIM-3 discharge_action) |
| M4 milestone charter at collapse-spec §5.2 | Milestone charter reframed: M4 IS composition-shard mint + Fire A primitives lift + Fire C empirical wire; NOT rust/src/mcp.rs port |

### §5.3 Retirement mechanism — REED-INLINE marker at M4 spec header

Reed will land a REED-INLINE marker at the header of `docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md` post-ratification of THIS spec. Marker shape:

```markdown
> **DEPRECATED-FOR-COMPOSITION-SHARD-REWRITE (2026-08-06):** the `rust/src/mcp.rs`
> port direction proposed in this spec is substrate-dishonest per Alex 2026-08-05
> verbatim reframe ("MCP served through mirror geometry, no specific mcp rust code").
> The substrate-honest replacement is `docs/specs/2026-08-06-mara-mcp-serve-composition-shard-canonical-spec.md`
> — primitives-at-rust (~55 LOC per Reed Fire A) + composition-at-substrate
> (`shards/mcp/serve.mirror` composition-shard body per Mara Fire B THIS spec).
> The 2026-08-03 M4 spec is retained for archival ancestry only; §1-4 (M4 in context,
> rust/src/mcp.rs shape, Phase C-D-E migration, tools list surface) are superseded;
> §5-8 (composition-into-existing-substrate table, [ALEX-Q]s, Q.E.D. sketch, Karen
> ancestry) remain useful cross-references.
```

**Marker landing altitude**: REED-INLINE per Reed's inline-edit authorship pattern (Reed 2026-07-16 REED-INLINE #2 at `docs/loop/CURRENT.md` per `f1767c0` precedent); pure-docs 📝 markdown-only bypass; single-line-edit at spec header line 4 (Status field) OR docblock addition immediately after title.

### §5.4 What survives from the 2026-08-03 M4 spec

- **§5 composition-into-existing-substrate table** (16 rows enumerating substrate composition anchors): survives as cross-reference; the composition graph the M4 spec enumerated is substrate-real and this spec inherits the composition edges (see §7 below).
- **§6 five [ALEX-Q] residues** (Phase B in-process-vs-subprocess default; apply_h::act rust/ altitude lift co-tick; @mcp/tool grammar annotation mint scope; pq-collapse vs 9-tool discipline; ~/.mirror/serve.sock daemon disposition): all five survive as cross-reference; Alex 2026-08-06 adjudication touched Q-1 (apply_h::act naming; ratified concurrent with Reed-lean) and structurally shifted the others (Q-2 in-process moot at composition altitude; Q-3 @mcp/tool mint still forward-promise; Q-4 9-tool wire ratified byte-parity Phase 1; Q-5 daemon socket unchanged).
- **§7 Q.E.D. six-move proof sketch**: superseded — this spec's §8 Q.E.D. re-derives the proof at composition-shard altitude with the primitives-vs-composition partition explicit.
- **§8 Karen ancestry ladder**: SURVIVES ENTIRELY; this spec's §9 EXTENDS the ladder with Alex 2026-08-05 verbatim + Taut Fire scout `7af55ee` + Recognition `#R-reality-as-5d-spinning-foam` RATIFIED 2026-08-03 + `feedback-rust-delivers-primitives-substrate-delivers-composition` memory.

### §5.5 What survives from Reed's Phase A delegation stub

Reed's Phase A delegation stub landing (`rust/src/main.rs::cmd_serve_mcp` at `59591a9`) — the transitional bootstrap-exec-delegation that ships empirical MCP-spawn TODAY — SURVIVES as transitional-until-Fire-C. Post-Fire C: `cmd_serve_mcp` is re-wired to invoke `@mcp/serve.mirror` composition-shard via `apply_h::act` discharge; bootstrap-exec-delegation retires. The stub is NOT retired at Phase 1 landing; it retires at Fire C landing when the composition-shard fires empirical MCP round-trip. Two-tick discipline: stub + composition co-exist; empirical parity verified; stub retires.

---

## §6 [ALEX-Q] residues — genuine undecidables at Mara altitude

Only genuine undecidables surfaced here. All five of Alex 2026-08-06 adjudications (Q-1 through Q-5) are already RATIFIED per §0; those are NOT residues. Residues below are questions that emerged during Fire B spec authorship that Mara cannot decide at spec-author altitude.

### §6.1 [ALEX-Q-R1] Substrate namespace `@data/json` vs `@wire/json`

**Fully articulated at §3 above.** Position A (Mara-lean): keep `@data/json` at substrate + `wire.rs` at rust/ altitude; substrate-already-had-the-word + substrate/rust altitude decoupling per phone.rs precedent. Position B: mirror the rust/ rename at substrate → mint `@wire/json` family-root + species; requires substrate migration. Alex Q-2 rename signal is ambiguous between (a) rust/-altitude naming honesty only vs (b) prescient substrate migration seed.

**Landing assumption Phase 1**: Position A. Composition-shard body uses `@data/json.parse` / `@data/json.emit`. If Alex ratifies Position B, substrate migration tick lands first; composition-shard body edits at 2 sites in the `serve` pipeline.

### §6.2 [ALEX-Q-R2] `@mcp/tool` annotation extension shape

The Phase 2 M5 co-tick reflective grammar walk requires `@mcp/tool` first-class grammar annotation per `docs/specs/lsp-and-mcp.md` §"MCP dispatch table". Grep-verified 2026-08-06: `@mcp/tool` DOES NOT EXIST as a landed annotation in any shard (grep for `@mcp/tool` across `shards/**/*.mirror`: zero matches; `@code/rust` shows 46 matches by contrast — the pattern precedent exists but @mcp/tool is unminted).

**Three candidate mint shapes** (Mara cannot decide at spec-author altitude):

- **(a) Species-under-@mcp**: `shards/mcp/tool.mirror` species — `@mcp/tool { name: text, description: text, inputSchema: json }`. Composes as `bilateral <name> { require @mcp/tool { name = "mirror_compile" description = "…" } }` at each landed shard opting-in.
- **(b) Extension of existing @mcp grammar**: extend `boot/std/mcp.mirror` grammar with `tool` annotation as first-class grammar carrier (like `@code/rust` at `shards/code/rust.mirror`; annotates ~46 shards' code emission).
- **(c) Bilateral-predicate composition**: no new mint; `@mcp.tools` reflective walk uses existing bilateral-corpus loader + heuristic (e.g., detect `bilateral <name>` blocks with specific naming pattern like `mirror_*` prefix). Composition-only; zero substrate mint.

**Mara-lean**: (b) extension of existing @mcp grammar. Rationale: `@code/rust` grammar annotation precedent lands ~46 shards at composition altitude with a single annotation shape; @mcp/tool at extension altitude of @mcp family-root at boot/std/mcp.mirror keeps the composition graph flat (@mcp is family-root; @mcp/tool is grammar annotation species; symmetry).

**BUT**: (a) species-under-@mcp has clean substrate placement (siblings THIS spec's `shards/mcp/serve.mirror` composition-shard mint at `shards/mcp/` altitude). (c) bilateral-predicate composition zero-mint may honor substrate-already-had-the-word discipline more strictly.

**Genuinely undecidable at Mara altitude.** Landing assumption Phase 1: composition-shard body forward-promises `@mcp/tool` annotation via TODO(M5) marker; concrete annotation shape unresolved until Alex adjudicates.

### §6.3 [ALEX-Q-R3] Grammar walker M5 co-tick priority

Per Alex Q-4 ratified: grammar walker deferred to Phase 2 M5+. But the M5 co-tick priority (WHEN in the M1-M6 sequence) is not fully specified. Options:

- **M5-primary**: grammar walker lands at M5 as prerequisite for `@mirror/reload` gen_prism (per lsp-and-mcp.md §"Auto-reload"); tools/list reflective walk lands at M5 alongside walker.
- **M5-adjacent**: grammar walker lands post-M5 as forward-promise; M5 itself is `@mirror/reload` gen_prism only; reflective walk lands at M5.5 or M6-adjacent.
- **M6-adjacent**: grammar walker lands after `mirror kintsugi @spec` spawn wire per collapse-spec §4; walker is @spec-consumer-driven.

**Mara-lean**: M5-primary. Rationale: reflective walk IS the substrate-decl'd body of `tools_reflects_cli_block` bilateral-predicate contract per `boot/std/mcp.mirror:118`; the contract fires structurally when walker lands; deferring walker past M5 lengthens the bilaterally-violated-by-design window.

**Not blocking THIS spec's landing**: Phase 1 byte-parity fires empirically without walker. Alex adjudication needed only for Phase 2 timing.

### §6.4 [ALEX-Q-R4] Composition-shard body altitude vs `boot/std/mcp.mirror` body

The composition-shard body at `shards/mcp/serve.mirror` fills `dispatch` + `tools` action bodies declared at `boot/std/mcp.mirror:71,76` (currently `\`-blocked at boot altitude). Two placement options:

- **(a) Species-shard composition** (Mara-lean per Alex Q-3 concurrence): body at `shards/mcp/serve.mirror` composition-shard; boot/std/mcp.mirror stays `\`-blocked at action-body altitude; substrate composition pulls body from species altitude when @mcp is instantiated.
- **(b) Boot-altitude body**: fill `dispatch` + `tools` bodies at `boot/std/mcp.mirror` directly; no composition-shard mint needed.

**Alex Q-3 RATIFIED (a)**: `shards/mcp/serve.mirror` at NEW substrate altitude. So (a) is not a residue — it's ratified. The residue is: **does `boot/std/mcp.mirror` stay `\`-blocked at `dispatch` + `tools` bodies, OR does it get a `body-composed-at @mcp/serve` reference pointer added?**

**Mara-lean**: add reference pointer at boot/std/mcp.mirror as a docblock note (`# dispatch body composed at shards/mcp/serve.mirror`; not a structural substrate edit — a discovery-aid comment). Actual body substrate-pulled from species altitude at runtime. Substrate-honest per two-altitude discipline.

**Not blocking THIS spec's landing**: composition-shard body stands on its own; boot-altitude reference pointer is a documentation improvement Reed can land at Fire B substrate mint time or Fire C wiring time.

### §6.5 [ALEX-Q-R5] Deprecated `mirror_spawn` alias retirement tick

The 11-tool byte-parity list includes `mirror_spawn` (DEPRECATED alias for `mirror_peer_beam` per Tick 3 rename `4f4a257`). Bootstrap docblock at `bootstrap/src/mcp.rs::13-14` states: *"DEPRECATED alias per two-tick discipline. This tool will be removed in a subsequent tick."*

**Residue**: WHEN does `mirror_spawn` alias retire? Options:

- **(a) Phase 1 (immediately)** — this spec's composition-shard drops `mirror_spawn` from the 11-tool list; MCP clients using `mirror_spawn` receive `method_not_found` error.
- **(b) Phase 2 (M5 co-tick)** — reflective walk drops alias when bootstrap `spawn` alias arm retires; two-tick discipline honored via reflective source-of-truth shift.
- **(c) Phase 3 (post-M5, on Alex signal)** — Alex explicitly signals alias retirement; substrate honors user-facing deprecation window (typically 2+ tick major-version equivalent).

**Mara-lean**: (b) — retire alias when bootstrap `spawn` arm at `bootstrap/src/mcp.rs:640` retires per two-tick discipline; substrate reflects source-of-truth. Phase 1 emits 11-tool wire (INCLUDING `mirror_spawn`); Phase 2 reflective walk drops it when bootstrap does.

**Not blocking THIS spec's landing**: Phase 1 byte-parity requires including `mirror_spawn`. Alex adjudication needed only for Phase 2+ retirement timing.

### §6.6 [ALEX-Q-R6] Composition-shard mint authorship

Mara canonical spec-author role (per Mara MEMORY.md `feedback-composition-primitive-naming-convention` + `AGENTS.md` role: "canonical spec author, math-first") — Mara authors SPECS + MATH + shard-decl mints. The actual `.mirror` file at `shards/mcp/serve.mirror` mint could be authored by:

- **(a) Reed (recommended)** — Reed's Fire C wiring altitude is adjacent (rust/src/main.rs::cmd_serve_mcp wire needs composition-shard body to exist for runtime dispatch); Reed authors under [substrate-floor:@io-boundary] gate; RED-first empirical test lands with composition body.
- **(b) Mara** — Mara authors spec + shard-decl body together (spec-and-shard co-landing); precedent per Mara's other canonical shard mints (`shards/butterfly.mirror` `2026-07-18` per project_butterfly_substrate_species memory).
- **(c) Glint** — Glint's essayist / prose cascade closure role — the composition-shard body's docblock (§1.5 five-paragraph header shape) sits at prose cascade altitude; body itself technical composition.

**Mara-lean**: (a) Reed at Fire C. Rationale: Reed's Fire A + C rhythm carries the empirical burden of round-trip firing; the composition-shard body co-lands with the rust/-altitude wire (`cmd_serve_mcp` invokes composition-shard body via apply_h::act discharge); RED-first test discipline lands together. Mara's authorship altitude = THIS spec + Karen ancestry.

**Not blocking THIS spec's landing**: authorship attribution is a coordination question, not a substrate structural question. Reed or Mara can mint; the composition-shard shape is spec'd here regardless.

---

## §7 Composition-into-existing-substrate table

Enumeration of how `shards/mcp/serve.mirror` composition-shard sits in already-landed substrate. Each row: existing substrate anchor + composition edge THIS spec's mint introduces + edge type (compose-over / extend / reference / co-declare).

| # | Existing substrate anchor | Landing state | Composition edge from `shards/mcp/serve.mirror` | Edge type |
|---|---------------------------|---------------|-------------------------------------------------|-----------|
| 1 | `boot/std/mcp.mirror` @mcp family-root grammar-decl (types + serve pipeline + dispatch/tools body-blocked + 3 bilateral-predicates) | LANDED-SPEC (Mara Tick 6 2026-07-08 substrate-decl closure) | Composition-shard fills `dispatch` + `tools` action bodies declared at family-root; honors 3 bilateral-predicate contracts (`dispatch_reflects_cli_block` + `tools_reflects_cli_block` + `frame_relativity`) at composition altitude | compose-over (body-fill) |
| 2 | `boot/std/data/json.mirror` @data/json grammar (parse/emit body-blocked) | LANDED-SPEC-ONLY | Composition-shard references `@data/json.parse` + `@data/json.emit` at `serve` pipe-stages 2 + 4; body dispatched via apply_h::act to rust/-altitude `wire::parse` / `wire::emit` (Reed R-PRIM-1) | reference |
| 3 | `boot/std/io/socket.mirror` @io/socket grammar (connection + listener types + read_bytes/write_bytes/close actions) | LANDED-SPEC-ONLY | Composition-shard references `@io/socket.accept` + `@io/socket.read_bytes` + `@io/socket.write_bytes` at `serve_socket` Variant B pipe-stages (Phase 2 forward-promise; Unix landed at phone.rs iter 9; TCP family-extension per Alex Q-5) | reference (forward-promise) |
| 4 | `boot/std/mirror/serve.mirror` @mirror/serve substrate-decl (`serve -> imperfect { \ }`) | LANDED-SPEC (192B) | Composition-shard extends @mirror/serve pattern to @mcp altitude; @mirror/serve is generic transport; @mcp/serve is JSON-RPC MCP transport specialisation | extend |
| 5 | `boot/std/mirror/reload.mirror` @mirror/reload gen_prism (auto-reload on grammars_hash drift) | LANDED-SPEC-ONLY (2.0KB) | Composition-shard forward-promises composition with @mirror/reload at Phase 2 M5 co-tick; `@mirror/reload.tick` fires on every request; emits `notifications/tools/list_changed` on drift | reference (forward-promise M5) |
| 6 | `shards/mirror/lens/mcp.mirror` @mirror/lens/mcp prism (agent-audience transport-side lens) | LANDED-SPEC (2.4KB, 2026-06-06) | Composition-shard sits at composition altitude adjacent to lens altitude; lens declares abstract `dispatch(call: ref) -> mcp` action; composition-shard fills the wire-protocol composition of that abstract dispatch surface | co-declare (adjacent species) |
| 7 | `shards/spectral/gen_prism/mcp_session.mirror` @spectral/gen_prism/mcp_session (session state machine at gen_prism altitude) | LANDED-SPEC (28.8KB, Reed M1 TICK 1 `e8378ca`) | Composition-shard references mcp_session for state discipline (state lives in @mirror/store; process is stateless; session persists across daemon restart); dispatch body composes with mcp_session.tick for stateful requests | reference (composition-time state carrier) |
| 8 | `rust/src/phone.rs` @io switchboard (fs + git + socket + stdio; ~90% wire-transport primitives) | LANDED-EMPIRICAL (69.6KB) | Composition-shard's `serve` body pipe-stages 1 + 5 dispatch through phone.rs @io/stdio primitives (`read_stdin_frame` + `write_stdout_frame`); Reed R-PRIM-2 lifts pub-visibility for cross-crate composition | compose-over |
| 9 | `rust/src/wire.rs` @data/json rust/-altitude wrapper (Reed R-PRIM-1 IN FLIGHT) | IN FLIGHT (Reed Fire A ~20-30 LOC) | Composition-shard's `serve` body pipe-stages 2 + 4 dispatch through wire::parse + wire::emit via `@data/json` substrate-decl'd action refs | compose-over (via substrate-decl) |
| 10 | `rust/roomba/src/mend.rs::discharge_action` (Reed R-PRIM-3 IN FLIGHT; composes over landed `load_bilateral_corpus` + `discharge`) | IN FLIGHT (Reed Fire A ~30 LOC wrapper over LANDED-EMPIRICAL 40.3KB) | Composition-shard's `@mcp.dispatch` body dispatches via `apply_h::act(action_ref, args) -> Verdict`; discharge_action IS the bilateral-predicate `act` path at rust/ altitude | compose-over (via apply_h::act naming) |
| 11 | `bootstrap/src/apply_h.rs` 7-combinator surface (section/fold/act/settle/crystallize/coboundary/utter) | LANDED-EMPIRICAL bootstrap-only (81.4KB Arc-1 Tick 1.3 GREEN) | Composition-shard uses ONLY the bilateral-predicate `act` path (composed via rust/-altitude `discharge_action`); other 6 combinators NOT structurally required per Taut §2 verdict | reference (act-only subset) |
| 12 | `mirror.spec` cli-block (10-verb VERBS list at rust/src/main.rs `const VERBS: &[(&str, &str)]`; hardcoded per Mara §5.2 M2 milestone) | LANDED-EMPIRICAL (hardcoded); reflective at M2 (GAP per Taut §3 red_spec_claims.rs verifies) | Composition-shard's Phase 1 hardcoded tool-name → action-ref map (§4.3) mirrors VERBS list byte-parity; Phase 2 M5 co-tick reflective walk collapses hardcoded map to reflective source-of-truth per bilateral-predicate `dispatch_reflects_cli_block` contract | reference (byte-parity Phase 1; reflective Phase 2) |
| 13 | `bootstrap/src/mcp.rs::tools_list_result` (11-tool JSON emission byte-parity with `bin/mirror-mcp`) | LANDED-EMPIRICAL (49.2KB; DYING per Alex 2026-07-22) | Composition-shard's `@mcp.tools` Phase 1 body copies 11-entry byte-parity verbatim (name + description + inputSchema per §4.2); replaces bootstrap tools_list_result at composition altitude when Fire C wires cmd_serve_mcp to composition-shard | replace (byte-parity Phase 1) |
| 14 | `bin/mirror-mcp` bash shim (149 lines; execs bootstrap binary; MCP client wire) | LANDED-EMPIRICAL (transitional; retirement at Tick 6.5/7 per boot/std/mcp.mirror docblock) | Composition-shard's landing enables bash-shim retirement per boot/std/mcp.mirror:80-83 docblock ("Tick 6.5/7 collapses `bin/mirror-mcp` (149 lines of bash) to `exec ~/.local/bin/mirror /dev/stdin @mcp.serve` once the Rust runtime discharges the bilateral-predicate contracts") | retire (post-Fire-C empirical parity) |
| 15 | `rust/src/main.rs::cmd_serve_mcp` (Reed 2026-08-03 `59591a9` stub-delegation; execs bootstrap binary) | LANDED-STUB-DELEGATION | Composition-shard's landing + Fire C wiring re-wires cmd_serve_mcp to invoke `@mcp/serve.mirror` composition-shard body via apply_h::act discharge; stub-delegation retires | re-wire (Fire C) |
| 16 | `shards/reflection.mirror` + `shards/mirror/spectral/portal.mirror` (references to @mcp.serve at composition altitude) | LANDED-SPEC references | Composition-shard closes the compose-target hole those references pointed at; @mcp.serve is now compose-target-existing at species altitude | close-hole (compose-target-existing) |

### §7.1 Composition-honest observations

- **Zero new substrate mints beyond `shards/mcp/serve.mirror` itself.** Every composition edge references already-landed substrate anchors (rows 1-16 above). The single mint is the composition-shard file at `shards/mcp/serve.mirror` (species-under-@mcp) + implicit `shards/mcp/` directory creation.
- **Two forward-promise references remain**: `@mcp/tool` grammar annotation (row 5 M5 co-tick precondition) + `@mirror/reload` gen_prism composition (row 5 M5 co-tick discharge). Neither blocks Phase 1 empirical landing.
- **Three refusal candidates NOT taken** per Michelangelo/marble discipline:
  - REFUSE minting `@mcp/wire` transport primitive — @mcp.serve at boot altitude ALREADY carries transport semantics (pipe body composed at boot altitude); composition-shard fills the composition body without minting parallel wire primitive.
  - REFUSE minting `@mcp/dispatcher` composition primitive — dispatch is bilateral-predicate discharge via apply_h::act; naming already-carried by @mcp.dispatch action decl at boot altitude.
  - REFUSE minting `@mcp/serve/tools` sub-species — @mcp.tools action decl at boot altitude carries tools discovery; composition-shard body fills the action body without minting parallel species.
- **Composition edges are additive, not substrate-mutating.** No existing substrate file is edited by THIS spec's mint; composition-shard body is drop-in-additive. Reed's REED-INLINE marker at 2026-08-03 M4 spec header (§5.3) is the only edit to already-landed substrate; that edit is a documentation deprecation marker, not a structural mutation.

### §7.2 Composition-into-existing empirical status matrix

| Composition edge | Phase 1 (this spec landing) | Phase 2 (M5 co-tick) | Phase 3 (post-M6) |
|------------------|------------------------------|---------------------|-------------------|
| serve body pipe-chain composition | LANDED-COMPOSED (empirical Fire C) | Same | Same |
| @mcp.dispatch body via apply_h::act | LANDED-COMPOSED (hardcoded 11-tool map) | Reflective bilateral-corpus walk | Same |
| @mcp.tools body byte-parity 11-tool | LANDED-COMPOSED (hardcoded JSON literal) | Reflective grammar walk over @mcp/tool annotations | Same |
| bilateral-predicate contract discharge (3 contracts) | Bilaterally-violated-by-design (byte-parity carries empirical claim) | Structurally-discharged (reflective claim) | Same |
| bin/mirror-mcp bash shim retirement | Not yet (transitional; Fire C parity verification needed) | Retired (composition-shard is source-of-truth) | Retired |
| Reed cmd_serve_mcp re-wire | Fire C landing (post-composition-shard mint) | Same | Same |
| @mirror/reload gen_prism composition | Not yet | LANDED-COMPOSED (grammars_hash drift → notifications/tools/list_changed) | Same |
| Session state via mcp_session gen_prism | Referenced (composition edge declared) | Composed-with-state (mcp_session.tick per request) | Composed-with-state |
| mirror kintsugi @spec spawn wire (M6) | N/A (M6 territory) | N/A | LANDED-COMPOSED (composition-shard exposes session-state gen_prism for @spec accumulation) |
| Socket transport Variant B | `\`-blocked (Phase 2 forward-promise) | Unix-scope LANDED (phone.rs iter 9 landed empirical) | TCP family-extension LANDED (M8-adjacent) |
