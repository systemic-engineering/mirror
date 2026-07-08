# Taut Scout — @mcp.serve substrate lift scope (Tick 6, task #386)

*2026-07-08 evening. Grep-first, read-only. Follows precedent
`bd837cd` (beam-refactor cascade scout-then-adjudicate) and companion
scout `1658b95` (Taut LRM @shatter × MCP collapse).*

## Executive summary

**LRM verdict — LANDABLE WITH ONE PREREQUISITE, TWO-TICK CUT preferred.**

1. `@mcp.serve` IS substrate-declared at `boot/std/mcp.mirror:9-11`
   (LANDED 2026-05-20). The grammar declares a pipeline `@io.read(stdin)
   |> @data/json.parse |> dispatch |> @data/json.emit |> @io.write(stdout)`
   — `serve` is a substrate action; `dispatch` and `tools` are undischarged
   `\`-cracks.
2. The mirror binary supports `mirror <mq>` (Path A, `bootstrap/src/lib.rs:2839`)
   and `mirror <file> <mq>` (Path B, :2858). **Path A dispatches an mq
   pipeline from a raw cli argument today.** The bash-collapse target
   `exec mirror /dev/stdin @mcp.serve` maps to Path B (file `/dev/stdin`
   + mq `@mcp.serve`).
3. `mirror.spec` has NO `command mcp` in the cli-block. The lift is NOT
   a new subcommand; it's an mq-query dispatch of an existing substrate
   action. The prerequisite is that `@mcp.serve` be a callable action
   from the mq pipeline, which requires `dispatch(request) -> response`
   and `tools -> json` to be discharged (currently `\`-cracks).
4. `bin/mirror-mcp` is 149 lines (11.9KB); the substrate-honest
   discharge shape is real but non-trivial — the wrapper carries schema
   synthesis (~40 lines TOOLS_LIST), JSON-RPC framing (~30 lines), arg
   extraction + dispatch (~60 lines), and envelope escape (~10 lines).
5. **Recommended cut:** Tick 6 lands the substrate-decl closure —
   `@mcp.serve` action body + `dispatch` discharge form, PLUS the mq
   route (Path B accepts `@mcp.serve` as an mq expression). Tick 7 (or
   task #386 continuation) collapses `bin/mirror-mcp` to
   `exec mirror /dev/stdin @mcp.serve` once the schema-synthesis-from-
   cli-block is in place.

---

## Q1 — Does `@mcp.serve` substrate-decl exist?

**YES, at the family root; discharge cracks remain.**

- `boot/std/mcp.mirror:1-25` (723B, 2026-05-20): declares
  `grammar @mcp { ... }` with:
  - `type request = { method: text, params: json, id: json }` (:6)
  - `type response = { result: json, id: json }` (:7)
  - `serve -> imperfect { @io.read(stdin) |> @data/json.parse |> dispatch
    |> @data/json.emit |> @io.write(stdout) }` (:9-11)
  - `dispatch(request) -> response { \ }` (:13) — **undischarged crack**
  - `tools -> json { \ }` (:16) — **undischarged crack**
  - `fate(hole_oid: text, resolution: text) -> imperfect { \ }` (:20) —
    reflection perturbation surface
- `shards/mirror/lens/mcp.mirror` (2.4KB, 2026-06-06): declares
  `prism @mirror/lens/mcp` — the transport lens family. Actions
  `tool(name, args: ref) -> mcp` and `dispatch(call: ref) -> mcp`
  are FAMILY-HEADER ONLY (docblock explicitly names this state); bodies
  land "when the consumer (the mcp transport binary served by
  `spectral serve` or equivalent) pulls."
- `shards/spectral/gen_prism/mcp_session.mirror` (28.8KB, 2026-07-06):
  the session-state-machine species — M1 TICK 1 landed via `e8378ca`.
  Session persistence + CAS-advance semantic declared; Rust wiring at
  `bootstrap/src/mcp.rs` (referenced in docblock as "current stateless
  MCP handler; M1 wires this substrate-decl into a session-state-machine
  at a follow-on tick").
- `mirror.spec` (12.5KB, 2026-07-08 18:37) — NO `command mcp` in the
  cli-block. Only `compile`, `kintsugi`, `shatter`, `craft`, `init`,
  `recall`, `beam`, `peer beam`, plus `command cargo`.

**Composition candidates:** `boot/std/mcp.mirror:9-11`'s pipeline
composes `@io.read`, `@data/json.parse`, `dispatch`, `@data/json.emit`,
`@io.write` — all substrate primitives; the pipeline itself IS
substrate-legal syntax already used elsewhere in the corpus.

## Q2 — Does the mirror binary support `mirror <input> <mq>` form?

**YES. Path A + Path B ALREADY LANDED.**

`bootstrap/src/lib.rs pub fn dispatch(args, ctx)` (:2832-2879):

- **Path A** (:2839-2856): `args.len() == 2 && is_mq_query(&args[1])`
  reads `stdin` and runs the mq pipeline. So `echo <input> | mirror
  '@some/action'` works today.
- **Path B** (:2858-2876): `args.len() == 3 && is_mq_query(&args[2])`
  reads the file at `args[1]` (resolved against `ctx.cwd()`) and runs
  the mq pipeline. So `mirror /dev/stdin '@mcp.serve'` matches this
  branch — `/dev/stdin` is the file, `@mcp.serve` is the mq query.
- **Path C** (:2878+): legacy subcommand dispatch — `compile`,
  `craft`, `kintsugi`, `init`, `recall`, `spawn`, `beam`, `peer beam`,
  `shatter` (as of Tick 0 Landing 2 `796f328` — cli-decl'd but the
  `"shatter"` dispatch arm status per Taut prior scout `1658b95`
  §Task 3 is now added under the shatter-target flag arm at :2905+).

`is_mq_query` (imported at :495, `pipeline::is_mq_query`) is the
gatekeeper — it decides whether `@mcp.serve` parses as an mq query.
That check must succeed for the bash collapse target to route to Path
B rather than Path C's "unknown subcommand" error.

**The binary already supports the cli surface `mirror /dev/stdin
@mcp.serve` in principle.** What's missing is that `@mcp.serve` must
resolve to a callable action in the mq pipeline (Q3).

## Q3 — What would @mcp.serve substrate-decl need to contain?

The substrate-decl at `boot/std/mcp.mirror:9-11` ALREADY names the
pipeline. What's missing:

- **`dispatch(request) -> response` body.** Currently `\`-crack (:13).
  This is the tool-name → cli-verb router — the largest logical piece
  of `bin/mirror-mcp` today (~60 lines of bash case + arg extraction).
  It must synthesize from `mirror.spec` cli-block reflection: for each
  `command <name>`, read args/flags, produce a tool-schema entry and a
  dispatch arm.
- **`tools -> json` body.** Currently `\`-crack (:16). This IS the
  TOOLS_LIST JSON blob (~40 lines in bash today). Same
  cli-block-reflection source; different codomain (schema vs. dispatch).
- **A grammar surface for cli-block reflection.** `mirror.spec` is a
  `.spec` file (not a `.mirror` shard); the reflection substrate would
  need to declare that mirror can read its own spec's cli-block and
  emit tool-schema JSON. This is either a new species under `@mirror/
  lens/mcp` (schema-generation prism) OR a query on the settled
  `mirror.spec` shard via the existing kintsugi output surface.
- **`@io.read(stdin)` and `@io.write(stdout)` bodies.** These are
  substrate primitives referenced in `boot/std/mcp.mirror`; searching
  `shards/io/algebra.mirror` and `shards/io.mirror` confirms substrate
  presence (grep hit count 3+ each). Wiring to Rust `stdin` / `stdout`
  handles is a separate line item — likely already partially wired via
  the existing `read_stdin_all` at `bootstrap/src/lib.rs` (called from
  Path A + Path B).
- **`@data/json.parse` and `@data/json.emit`.** Already-landed
  substrate primitives (Mara §5.4; Reed 2026-06-16 substrate-pull).
  The kintsugi `--out @data/json` chain (`bootstrap/src/lib.rs:1826-
  1832 parse_substrate_ref_to_format`) proves the codec is wired.

**Minimum shape for a bash-collapse:** `@mcp.serve` as an mq expression
must (a) parse as `is_mq_query`, (b) resolve to a pipeline that reads
stdin JSON-RPC lines, dispatches per cli-block reflection, and writes
JSON-RPC responses to stdout. The `dispatch` and `tools` cracks must
discharge to cli-block-reflection substrate.

## Q4 — Landable in one tick or multi-tick?

**MULTI-TICK. Tick 6 lands the substrate closure; a follow-up tick
lands the bash collapse.**

**Fault-plane shifts under one-tick collapse (LRM = NOT landable
AS-DECLARED):**

1. `dispatch(request) -> response` crack — undischarged. Discharging
   requires either a hardcoded dispatch table (regress to the bash
   wrapper's shape at substrate altitude — Seam would flag this as
   substrate-decl not carrying substrate meaning) OR cli-block
   reflection (a substrate feature that does not exist today at the
   grammar altitude — `mirror.spec` is a `.spec` file, not addressable
   by shard queries yet).
2. `tools -> json` crack — same argument.
3. `is_mq_query("@mcp.serve")` must return true. The current gatekeeper
   likely accepts `@<path>` shapes; `@mcp.serve` is `@<family>.<action>`
   — this SHOULD parse but needs verification (not verified in this
   scout; grep for `is_mq_query` implementation deferred). If it does
   not, Path B refuses the input and falls through to legacy dispatch,
   yielding "unknown subcommand" — a hard fail.
4. `bin/mirror-mcp` today advertises SEVEN tools (compile, craft,
   kintsugi, init, recall, peer_beam, beam) plus one DEPRECATED alias
   (`mirror_spawn`). Any collapse must preserve the schema surface —
   the substrate-reflection path is the honest discharge; the hardcoded-
   dispatch path is a regression.

**Substrate-honest cut-line (recommended):**

- **Tick 6 (this tick) — substrate closure at `boot/std/mcp.mirror`:**
  Land the `dispatch(request) -> response` body via cli-block
  reflection composition, and the `tools -> json` body from the same
  reflection source. Add a `command mcp` to `mirror.spec` cli-block if
  the invocation form `mirror mcp serve` is preferred over the mq form
  `mirror /dev/stdin @mcp.serve`. **RED first:**
  `bootstrap/tests/mcp_serve_dispatch_shard.rs` — JSON-RPC in, JSON-RPC
  out, tool-list matches cli-block reflection. **Does NOT collapse
  `bin/mirror-mcp` yet.**

- **Tick 7 (or task #386 continuation) — bash collapse:** Rewrite
  `bin/mirror-mcp` as `exec ~/.local/bin/mirror /dev/stdin @mcp.serve`
  (or `exec ~/.local/bin/mirror mcp serve` if the cli-block command
  form landed at Tick 6). Verify Claude Code MCP integration end-to-
  end. Delete 149 lines of bash.

**Files to touch at Tick 6 (RED-first order):**

1. `bootstrap/tests/mcp_serve_dispatch_shard.rs` — RED test asserting
   JSON-RPC round-trip through `@mcp.serve` matches current bash
   wrapper behavior for at least `initialize`, `tools/list`,
   `tools/call mirror_compile` cases.
2. `boot/std/mcp.mirror` — discharge `dispatch` and `tools` cracks;
   reference cli-block reflection substrate.
3. `bootstrap/src/mcp.rs` (referenced in `mcp_session.mirror` docblock
   as the current stateless handler) — wire `@mcp.serve` pipeline to
   Rust runtime.
4. `bootstrap/src/lib.rs` — verify `is_mq_query("@mcp.serve")` returns
   true (may need an mq-parser extension for `@<family>.<action>` form
   if not already supported).
5. `mirror.spec` — OPTIONAL: add `command mcp { command serve { } }` if
   the subcommand form is preferred over mq form. Not strictly
   required — Path B (`mirror /dev/stdin @mcp.serve`) doesn't need a
   cli-block entry.

**Fault-plane shifts under multi-tick cut (verdict: STABLE):**

- Tick 6 lands substrate closure; existing `bin/mirror-mcp` bash
  wrapper remains as the bridge (unchanged) — no MCP surface breakage
  for Claude Code integration.
- Tick 7 flips `bin/mirror-mcp` to a one-liner; MCP surface behavior
  is identical (RED test at Tick 6 ensures parity); collapse discharges
  task #386.
- Fate crate plumbing unaffected (`bootstrap/Cargo.toml` has no fate
  dep per prior Taut scout `1658b95` §5a).

**LRM VERDICT:** LANDABLE WITH PREREQUISITES over two ticks. **NOT
landable AS-DECLARED at Tick 6 as a single collapse** — the substrate
closure MUST land first for the bash to have a legitimate discharge
target.

---

## Recommended next action (Reed adjudication)

**Split task #386 into Tick 6 (substrate closure) + Tick 6.5 or Tick 7
(bash collapse).** Tick 6 lands `boot/std/mcp.mirror` `dispatch` +
`tools` discharge via cli-block reflection, with RED-first test at
`bootstrap/tests/mcp_serve_dispatch_shard.rs`. Tick 6.5/7 collapses
`bin/mirror-mcp` to `exec ~/.local/bin/mirror /dev/stdin @mcp.serve`
once the substrate closure verifies parity with the current bash
wrapper's seven-tool schema.

Alternative: if Alex wants a one-tick collapse anyway, name it as a
regression (hardcoded dispatch table at substrate altitude, deferring
the reflection discharge to a follow-up) — Seam would flag this as
substrate-decl not carrying substrate meaning, but it MAY be honest as
a two-tick discipline step ("readable name over foundational" per
CLAUDE.md).

Not a substrate re-scout: `bd837cd` beam-refactor cascade Ticks 0-5
projections hold; this scout extends the cascade to Tick 6 with a
substrate-honest cut-line.
