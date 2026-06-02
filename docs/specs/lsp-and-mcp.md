# Mirror LSP and MCP — the same transport, two dispatches

*2026-05-20. Reed.*

Mirror exposes two protocols: MCP for tools dispatched from a host like Claude
Code, LSP for editor integration. Today MCP runs through a shell-script
wrapper at `bin/mirror-mcp` against three subcommands of the bootstrap binary;
LSP is declared but unimplemented. Both protocols speak JSON-RPC over stdio.
They are the same transport. They differ only in the dispatch table.

This spec closes the gap, names the auto-reload contract for grammar changes,
and draws the boundary between what mirror owns and what spectral owns.

---

## State today

| Surface | File | What runs |
|---|---|---|
| `@mcp` | `boot/std/mcp.mirror` | type for request/response, `serve`, `dispatch(\)`, `tools(\)`, `fate(\)` |
| `@mirror/lsp` | `boot/std/mirror/lsp.mirror` | 6 actions, 4 concrete + 2 holes (`dispatch`, `completion`) |
| `@mirror/serve` | `boot/std/mirror/serve.mirror` | one action: `serve(\)` |
| MCP transport | `bin/mirror-mcp` | bash, ~80 lines, advertises three tools (`mirror_compile`, `mirror_craft`, `mirror_kintsugi`) |
| MCP discovery | `.mcp.json` | stdio entry pointing at `bin/mirror-mcp` |
| LSP transport | — | not implemented |

The grammars carry the design; the shell wrapper carries today's behaviour.
Neither side knows about the other. The gap is the absence of a shared
transport that the dispatch tables ride on.

---

## The unified surface

Per `surface-simplification.md`: `mcp` and `lsp` are not commands. They are
dispatches of the same JSON-RPC transport. The CLI form:

```
mirror serve --mcp        # MCP dispatch over stdio
mirror serve --lsp        # LSP dispatch over stdio
mirror serve --lsp --tcp 7340   # LSP dispatch over TCP (future)
```

One grammar implements the transport. Two grammars implement the dispatch
tables. `@mirror/serve` becomes the routing seam.

```mirror
grammar @mirror/serve {
  # one transport. JSON-RPC over a configurable stream.
  transport(stream) -> imperfect {
    @io.read(stream)
      |> @data/json.parse
      |> dispatch
      |> @data/json.emit
      |> @io.write(stream)
  }

  # dispatch is the typed pivot.
  # --mcp routes to @mcp.dispatch.
  # --lsp routes to @mirror/lsp.dispatch.
  dispatch(request, kind: mcp | lsp) -> response { \ }

  # the entry point. flags decide kind + stream.
  serve -> imperfect { \ }
}
```

Claude Code (or any MCP client) invokes `mirror serve --mcp`. An LSP-aware
editor invokes `mirror serve --lsp`. The binary picks the dispatcher; the
transport, the parsing, the emission, the beam handling are identical.

---

## The LSP dispatch table

Four of the six current actions in `@mirror/lsp` already compose against
live grammars. Two are holes. The spec resolves the holes and adds the
methods Claude Code and IDEs actually call.

| LSP method | Mirror grammar | Beam emission | Default gutter lens |
|---|---|---|---|
| `initialize` | `@mirror/lsp.initialize` | capabilities beam | — |
| `textDocument/didOpen` | `@beam.emit(file)` (today) | `light(crystal, 0.0)` if cached, else loss > 0 | default |
| `textDocument/didChange` | `@mirror/liquid.liquid(file)` (today) | beam carries `fractures: [oid]` for changed regions | default |
| `textDocument/hover` | `@beam.observe(file)` at position | observation beam, scoped to the prism at cursor | active lens |
| `textDocument/diagnostics` | `@mirror/liquid.infer(file)` → `[verdict]` | one beam per failing property | default |
| `textDocument/completion` | `@mirror/lsp.completion` — currently `\` | suggestions derived from grammar's `out` lines | — |
| `textDocument/definition` | `@mirror/lsp.definition` (new) | crystal lookup via `@mirror/spectral.recall` | — |
| `textDocument/references` | `@mirror/lsp.references` (new) | reverse-lookup over the gestalt graph | — |
| `textDocument/formatting` | `@mirror/lsp.formatting` (new) | runs `mirror kintsugi` on the buffer; returns the canonical form | — |
| `textDocument/codeAction` | `@mirror/lsp.code_actions` (new) | one action per `\` hole at cursor: "Resolve via Fate", "Learn grammar" | — |
| `workspace/didChangeWatchedFiles` | `@mirror/lsp.workspace_changed` (new) | re-crafts the project; emits a workspace-scope beam | — |
| `$/progress` | beams carry progress automatically | — | — |

Closing the holes:

```mirror
grammar @mirror/lsp {
  in @prism
  in @mcp
  in @code/mq
  in @mirror/execute
  in @mirror/liquid
  in @mirror/spectral
  in @beam

  # LSP method dispatch — routes JSON-RPC method strings to grammars.
  # the routing table IS the table above; the body is a typed match.
  dispatch(request) -> response { \ }

  # textDocument/completion — grammar-derived suggestions.
  # walks the grammar visible at `position` (via `in @x/y` chains),
  # enumerates each grammar's `out` lines, returns them as suggestions.
  # for a `\` hole at cursor, also enumerates Fate's current top-k.
  completion(file, position) -> [suggestion] {
    @mirror/spectral.grammar_at(file, position)
      |> @beam.observe
      |> @code/mq.suggestions
  }

  # textDocument/codeAction — one action per \ hole.
  # for any \ hole at cursor, surface: resolve via Fate (interactive),
  # learn grammar (if the dark region clusters with others), or apply a
  # past resolution at refs/fate/<oid>.
  code_actions(file, position) -> [action] { \ }
}
```

The LSP doesn't invent semantics. Every method routes to a grammar that
already exists or to a hole that's the spec for the next one.

---

## The MCP dispatch table

> **2026-06-02 reframe.** The MCP wire altitude has a name now: **pq**
> ([[../../../prism/docs/specs/pq]]). The MCP server's tool surface
> is the three `prism_core::Prism` operations (`focus`, `project`,
> `refract`); the per-grammar `@mcp/tool` annotation extends the
> typed DSL types (`Target`, `Filter`, `Output`) inside those three
> calls, NOT new wire tools. Below: the in-flight five-tool framing
> for the mirror-mcp surface; it's a useful intermediate, but the
> grounding altitude is pq. When the reload contract
> (`@mirror/reload`) emits `tools/list_changed`, what's changing is
> the typed DSL surface, not the wire tool count. The MCP wire stays
> at three.

Three tools today. The road to 1.0 adds two; the five operations expose
themselves directly.

> **Wire-altitude reload.** When mirror-mcp lands per [[../../../fragmentation/docs/specs/fragmentation-mcp]] §8, the five tools below collapse into pq's three on the wire; the per-tool surface here becomes the typed DSL inside `focus`/`project`/`refract`. The table records the in-flight intermediate; the grounding altitude is pq.

| Tool | Bootstrap subcommand | Status |
|---|---|---|
| `mirror_compile` | `compile <file>` | live |
| `mirror_craft` | `craft <target>` (with `--target`, `--reflect`) | live |
| `mirror_kintsugi` | `kintsugi <file>` (with `--liquid`, `--shatter N`) | live |
| `mirror_run` | `run <file>` | declared, awaits subcommand impl |
| `mirror_fate` | `fate <oid> <resolution>` | declared, awaits subcommand impl |

The five-operation surface is what tools should call. Today they collapse
into the four subcommands above; tomorrow each operation exposes itself:

```
mirror_focus(file)            → compile
mirror_project(file)          → run
mirror_split(target, flags)   → craft
mirror_zoom(hole, resolution) → fate
mirror_refract(file, flags)   → kintsugi
```

The collapse goes through `@mcp.tools` rather than through `bin/mirror-mcp`.
The tools list becomes a function of the live grammar set, not a fixed JSON
string in a shell script.

```mirror
grammar @mcp {
  # the tools list is computed, not hard-coded.
  # walk the boot/std/ graph, find every @mcp/tool annotation, emit a
  # tool descriptor. when a new grammar lands declaring @mcp/tool, the
  # tool appears here on next call.
  tools -> json {
    @mirror/spectral.gestalt
      |> @mcp/tool.collect
      |> @data/json.emit
  }

  # a grammars hash: SHA-256 over (path, content_oid) pairs across
  # boot/std/. the hash is the identity of the current grammar set.
  # changes ⇔ something an MCP client cares about may have changed.
  grammars_hash -> oid {
    @mirror/spectral.gestalt |> @hash/coincidence.content_oid
  }
}
```

A grammar declares a tool by extending `@mcp/tool`:

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

No shell script. No JSON edited by hand. The grammar IS the tool surface.

---

## The dark-region surface

When `mirror compile` tokenizes a file, regions the grammar doesn't recognise
don't fail. They get marked **dark**. Dark is not red. Dark is *not yet
measured* (per `gutter-lenses.md`). The grammar has no model for these bytes;
the honest gutter colour is the void colour.

The beam carries dark regions as `[dark_range]`:

```mirror
type dark_range {
  start: position,
  end: position,
  bytes: text,
}

# beam already carries holes and fractures.
# dark_regions are different — holes are explicit `\` in source;
# dark_regions are bytes the tokenizer couldn't classify.
type beam(t) {
  ...
  dark_regions: [dark_range],
  ...
}
```

The LSP renders dark ranges in the gutter using the void colour from
`gutter-lenses.md`. The user sees: "the compiler has no idea what this is."
Not a failure. An invitation.

### Learn from clusters of dark

Fate's job (`@fate/connectome` + `@fate/tournament`) is to recognise patterns
in dark regions across the project and propose grammar extensions that
resolve them. The CLI:

```
mirror lsp --learn @code/llvm/ir
```

is shorthand for the mq pipeline:

```
mirror '@mirror/lsp/learn |\> @code/llvm/ir'
```

`@mirror/lsp/learn` walks current dark clusters, asks Fate to propose a
grammar extending the named grammar (`@code/llvm/ir` here), then writes the
proposal as a `\` body in a new grammar file. The user reviews and commits.

```mirror
grammar @mirror/lsp/learn {
  in @beam
  in @fate/connectome
  in @fate/tournament

  # collect dark regions across the project's open beams.
  collect_dark -> [dark_cluster] { \ }

  # propose a grammar that classifies the clusters.
  # Fate picks the model; tournament selects the resolution.
  propose(clusters, target_grammar) -> grammar_proposal { \ }

  # entry point. CLI: `mirror lsp --learn @x/y` invokes this.
  learn(target_grammar) -> grammar_proposal {
    collect_dark |> propose(target_grammar)
  }
}
```

Dark feeds grammar evolution. The IDE makes the dark visible. The user sees
the shape of what the compiler doesn't know, and decides whether to teach
it.

---

## Auto-reload — the mirror/spectral boundary

**The problem.** A Claude Code session starts. Claude calls `tools/list`.
The MCP wrapper returns three tools. The user pulls a branch that adds a new
grammar declaring `@mcp/tool ir_compile`. Claude's cached tools list is
stale; the new tool is invisible until the session restarts.

The LSP protocol has the same gap: `workspace/didChangeWatchedFiles` covers
file changes, but the LSP server itself needs to be re-initialised when its
*own* grammar set changes — because the dispatcher, the completion table,
the diagnostic properties all derive from the grammar graph.

**The contract.** Mirror declares two pure functions:

- `@mcp.tools -> json` — the current tools list. Function of the live grammar set.
- `@mcp.grammars_hash -> oid` — SHA-256 over `(path, content_oid)` for every grammar reachable from `boot/std/`. Changes if and only if the tools list could have changed.

Given those, *something* must:

1. Watch the grammar files for changes.
2. Recompute `grammars_hash` on change.
3. If the hash differs from the previously-emitted hash, send `notifications/tools/list_changed` (MCP) and the LSP equivalents (`workspace/configuration` invalidation, `client/registerCapability` re-issue).

Mirror is process-per-invocation, but `mirror serve --mcp` and
`mirror serve --lsp` *are* persistent within a single client session. The
reload contract piggy-backs on traffic the client already sends: every
incoming JSON-RPC request triggers a check. No watcher, no inotify, no
daemon dependency.

The primitive that makes this work is `@mirror/runtime/gen_prism` — a
content-addressed actor whose state lives in a crystal at a git ref. See
`docs/specs/mirror-runtime-gen-prism.md`. The reload contract is implemented
as one such actor.

### `@mirror/reload` as a gen_prism

The state crystal records `last_emitted_hash`. Every incoming request — any
request, not just `tools/list` — triggers a tick. The tick recomputes
`@mcp.grammars_hash`, compares it to the stored value, and emits
`notifications/tools/list_changed` if it drifted.

The grammar lives at `boot/std/mirror/reload.mirror` (see
`docs/specs/mirror-runtime-gen-prism.md` Example 1 for the full body). It
declares one state type (`{ last_emitted_hash: oid }`), one message type
(any incoming method), and one `tick(state, message) -> tick_result`.

No cross-process bus is needed for the auto-reload concern. The session-local
`mirror serve` runs the tick inline; the notification rides the same stdio
the request arrived on.

### Boundary summary

| Concern | Owner |
|---|---|
| Compute the tools list | mirror (`@mcp.tools`) |
| Compute the grammars hash | mirror (`@mcp.grammars_hash`) |
| The actor primitive (state in crystals) | mirror (`@mirror/runtime/gen_prism`) |
| The reload contract | mirror (`@mirror/reload` gen_prism) |
| Run the tick on incoming requests | mirror (`mirror serve --mcp` / `--lsp`) |
| Persist `last_emitted_hash` across ticks | mirror (crystal at `refs/gen_prism/mirror_reload`) |
| Cross-session, cross-tool orchestration | spectral (daemon, the glue bus) |
| Autonomous heartbeat for `@spectral/spawn` gen_prisms | spectral (the autonomous tick loop) |

Mirror owns the auto-reload concern end-to-end via the gen_prism primitive.
Spectral retains the cross-session bus and the autonomous heartbeat for
`@spectral/spawn` — those genuinely need a daemon.

---

## What this spec implies

Concrete follow-ups, ordered:

1. **Mirror: close `@mirror/lsp.dispatch` and `@mirror/lsp.completion`.**
   Add `definition`, `references`, `formatting`, `code_actions`,
   `workspace_changed` as new actions. Most are `\` until Fate seeds them;
   that's correct kintsugi state.

2. **Mirror: `@mcp/tool` as a first-class grammar annotation.**
   Today MCP tools are hard-coded in `bin/mirror-mcp`. The grammar should
   surface tools via annotations on actions, and `@mcp.tools` should walk
   the gestalt to emit the list.

3. **Mirror: `@mirror/reload` gen_prism.**
   Per `mirror-runtime-gen-prism.md` Example 1. Lives at
   `boot/std/mirror/reload.mirror`. Ticks on every incoming request; emits
   `tools/list_changed` when `@mcp.grammars_hash` drifts. The grammar IS
   the spec.

4. **Mirror: replace `bin/mirror-mcp` with `mirror serve --mcp`.**
   Drop the shell wrapper. The bootstrap binary handles JSON-RPC stdio
   directly. `.mcp.json` points at `~/.local/bin/mirror` with `args: ["serve", "--mcp"]`.

5. **Spectral: cross-tool bus only.**
   The auto-reload concern moves into mirror via `@mirror/reload`. Spectral
   keeps the glue bus for cross-session orchestration and the autonomous
   heartbeat for `@spectral/spawn` gen_prisms.

Follow-ups (1)–(4) become candidate tasks. (5) is a scope reduction for
spectral, not new work.

---

## Out of scope for this spec

- Implementing `mirror serve --mcp` in the bootstrap. The binary already
  has the dispatcher seam (`bin/mirror-mcp` calls into it); moving the
  JSON-RPC handling into the binary is a future commit.
- LSP capabilities negotiation details. The protocol spec covers this.
  Mirror returns the static capabilities derived from the action set.
- The mq pipeline grammar (`@code/mq`). Covered in its own grammar file.
- The Fate-tournament shape for `@mirror/lsp/learn`. The connectome and
  tournament grammars exist; this spec uses them via `\` bodies that point
  at the existing actions.
- Editor-side configuration (VS Code extension, Neovim plugin). Mirror
  ships a spec-compliant LSP server; editor wiring is downstream.

---

*The glass holds because it can prove it holds. The light passes through
because the prisms agree on its shape. The dark stays dark until someone
asks Fate what it might mean.*

Apache-2.0.
