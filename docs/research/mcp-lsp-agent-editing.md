# Agent-Driven Mirror Grammar Editing via MCP + LSP

*Research note, 2026-06-01. Claude (Opus 4.7), background pass. Pairs with [[../specs/lsp-and-mcp]] (the unified-transport spec, 2026-05-20) and [[../specs/properties-on-glass]] (Mara, 2026-06-01). Sibling reading: [[../specs/hazel-execution-model]], [[../specs/liquid-types-for-mirror]], [[../specs/mirror-runtime-gen-prism]], [[../specs/mirror-supersedes-daemon]] (insight), and `fragmentation/docs/specs/{hamilton-scheduler,lens-transit}.md`.*

Status: **research**. Not a design doc. The shape of the capability is sketched; the tick decomposition names what implementation would look like; the open questions are honest. No `.rs` or `.mirror` files land with this — this doc surfaces what already exists in the substrate, where the new property-on-glass + Body=prism+glass+AST machinery gives the editing surface its teeth, and what the minimum tick path looks like.

---

## 1. The thesis

Mirror's substrate, as of 2026-06-01, has just landed (in spec form) three pieces that together change what the editor can do:

1. **Per-glass property declarations + `Pure<G: Glass>`** ([[../specs/properties-on-glass]]). A glass declaration binds a finite set Q of property names from the `@epistemologic/property/*` chain. Every body that crosses the glass must witness each name in Q. Pure becomes a *per-glass compile-time witness produced by the liquid-type pass*, not a hand-written marker trait.

2. **Body = prism + glass + AST** (`fragmentation/docs/specs/hamilton-scheduler.md` §5.1). Bodies stop being opaque `Arc<dyn Fn(...)>` closures and become a structured triple whose AST is content-addressable. The glass wall becomes literally transparent — the substrate sees structured edits, not text.

3. **The `---` back-projection mechanism** ([[../specs/properties-on-glass]] §4). Above `---` is what the programmer wrote. Below `---` is what the substrate inferred at settlement time. Re-settlement re-projects. The audit trail is the git diff.

This structurally lets the editor refuse ill-formed grammar *at the editing surface*, before compile, with property-located diagnostics. The question is what concrete shape that capability takes — and the answer is mostly *existing patterns* (LSP + MCP) wired against substrate primitives that no other system has, not new protocol invention. The editor delivery is conservative; the substrate it delivers is not.

---

## 2. Current state — honest survey

### 2.1 MCP today

**File: `bin/mirror-mcp`** (4.2KB, bash). A shell-script wrapper. Reads JSON-RPC from stdin, dispatches three hard-coded tools (`mirror_compile`, `mirror_craft`, `mirror_kintsugi`) by shelling out to `$HOME/.local/bin/mirror`. Returns results as text content. No `tools/list_changed`. No notifications. No state. No concurrency story.

**File: `.mcp.json`**. Standard stdio MCP server entry pointing at `bin/mirror-mcp`.

**File: `boot/std/mcp.mirror`** (723 bytes). The *grammar* surface for MCP. Declares `serve -> imperfect`, `dispatch(request) -> response { \ }`, `tools -> json { \ }`, `fate(hole_oid, resolution) -> imperfect { \ }`. Four actions; three are `\` holes; only `serve` has a concrete body (and it composes `@io.read |> @data/json.parse |> dispatch |> @data/json.emit |> @io.write` — the dispatcher it composes through is itself a hole).

**Bootstrap binary**: `bootstrap/src/main.rs` (33.9KB) — the Rust binary. Subcommands present: `compile`, `craft`, `kintsugi`, plus internal `dump`. **No `serve` subcommand.** No JSON-RPC handling in Rust. The shell wrapper is the entire MCP transport today.

**Insight**: per `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — "Every MCP tool call is `send(gen_prism, message)`. `send` advances the ref by writing a new crystal." The gen_prism IS MCP architectural recognition is captured. The implementation isn't.

Net: the MCP surface today is a 4KB bash script with three hard-coded tools and zero state. The design that supersedes it is two specs (`lsp-and-mcp.md`, `mirror-runtime-gen-prism.md`) and one architectural insight (`mirror-supersedes-daemon.md`).

### 2.2 LSP today

**Not implemented.** `boot/std/mirror/lsp.mirror` (978 bytes) declares six actions:

- `dispatch(request) -> response { \ }` — hole
- `did_open(file) -> imperfect { @beam.emit(file) }` — concrete; emits a beam per file open
- `did_change(file) -> imperfect { @mirror/liquid.liquid(file) }` — concrete; runs liquid inference on change
- `hover(file, position) -> response { @beam.observe(file) }` — concrete; observation beam
- `diagnostics(file) -> [verdict] { @mirror/liquid.infer(file) }` — concrete; verdict stream
- `completion(file, position) -> [suggestion] { \ }` — hole

Four of the six already compose against live grammars (`@beam`, `@mirror/liquid`). Two are holes. There is no LSP transport — no `mirror serve --lsp`, no JSON-RPC over stdio, no `initialize` handshake. The grammar is the spec; the wiring isn't.

There is no `mirror lsp learn` command in the bootstrap. The CLI form is named in `lsp-and-mcp.md` §"Learn from clusters of dark" — `mirror lsp --learn @code/llvm/ir` — but the command doesn't dispatch. The grammar `@mirror/lsp/learn` is sketched, not landed.

### 2.3 The substrate primitives that exist today

What does exist, end-to-end:

- **`@beam.emit(file)`, `@beam.observe(file)`** — `boot/std/beam.mirror`. The beam is the trace; it carries the compile result, holes, fractures, dark regions. Already wired through `did_open` / `hover` / `diagnostics`.
- **`@mirror/liquid.liquid(file)`, `@mirror/liquid.infer(file)`** — the liquid-type inference entry points. `infer` returns `[verdict]` — exactly what LSP `textDocument/diagnostics` needs. Already wired through `did_change` / `diagnostics`.
- **`@mirror/spectral`** — the gestalt navigator. `recall`, `crystallize`, `gestalt` actions. The completion-driver, the definition-driver, the workspace-changed-driver all flow through this.
- **`@mcp/tool`** — declared in `lsp-and-mcp.md` as a grammar annotation. **Not yet landed** as a meta-glass keyword; would let any grammar contribute an MCP tool by annotation, walked at `tools/list` time.
- **`@mirror/runtime/gen_prism`** — the actor primitive. Lives at `boot/std/mirror/runtime/gen_prism.mirror`. Declares the state-via-crystal contract; concrete gen_prisms (e.g. `@mirror/reload`) close their own ticks.

The wiring at the LSP method altitude is already mostly there, in grammar form. What's missing is the *transport* (the JSON-RPC stdio loop in Rust that calls these actions) and the *dispatch table* (`@mirror/lsp.dispatch`'s hole filled in).

---

## 3. The destination — a walk-through

Mara is editing `boot/std/kintsugi/dispatch/tick.mirror` to add a hard-realtime path. The file already declares a glass to `@code/rust` with `property halts, property content_addressed`. She wants to add `property wcet_bounded` (which is the hard-RT pattern from `hamilton-scheduler.md` §4.7 plus the active `pure` aggregate from `properties-on-glass.md` §10.3).

Here is what happens, with substrate-pull at every step.

**Mara types** `property wcet_b` at the cursor inside the `glass to @code/rust { ... }` block. The LSP fires `textDocument/completion`. The grammar `@mirror/lsp.completion` walks the visible chain (`in @epistemologic/property/*`), filters property names by prefix, and proposes `wcet_bounded`. Mara accepts. The file now has `property wcet_bounded`.

**The LSP fires `textDocument/didChange`.** This runs `@mirror/liquid.liquid(file)` — the liquid inference pass for the changed buffer. The pass discovers that the glass's qualifier set Q has grown from `{halts, content_addressed}` to `{halts, content_addressed, wcet_bounded}`. The contract OID above `---` has changed.

**The freshness check fires** ([[../specs/properties-on-glass]] §4.3). The settlement header below `---` records the *previous* contract OID. The new contract OID doesn't match. The substrate emits a `Partial { confidence: 0.0, diagnostics: ["properties-on-glass: stale ---; re-settle to regenerate"] }` verdict at the glass altitude. The LSP surfaces this as a diagnostic at the line of the `---` separator. Gutter lens (per `gutter-lenses.md`) shows the void colour: the substrate's verdict has not yet been measured for this contract.

**Mara invokes a code action**: "Settle this file." The LSP fires `textDocument/codeAction`, the code action calls the MCP tool `mirror.settle` (or equivalent), which runs:

```
mirror kintsugi --liquid path/to/tick.mirror
```

— but **routed through `@mirror/lsp.dispatch`** rather than as an external shell. The settlement pass:

1. Reads the contract above `---`.
2. Computes the new contract OID.
3. Runs Fate + liquid inference to fill the implementation `\` holes below `---` for each `(glass, property)` pair, including the new `(@code/rust, wcet_bounded)` pair.
4. Runs the spectral decision procedure ([[../specs/liquid-types-for-mirror]] §5.4) — the Dirac operator computes eigenvalues on the property Laplacian; the Fiedler value tells whether the qualifier set is simultaneously satisfiable for the body's AST.
5. Writes the new back-projection below `---`.
6. Updates the settlement header with the new contract OID.

**The LSP fires `textDocument/diagnostics` automatically** (the settlement is a server-initiated buffer mutation). Each `(glass, property)` pair produces one diagnostic if `partial` or `fail`. If `wcet_bounded` discharges as `partial(0.72, ["loop at line 42 has no static bound"])`, Mara sees the diagnostic in her editor at line 42, with the located substrate path and the confidence value. Hover at the diagnostic shows the spectral profile: the eigenvalue λ₁, the Fiedler vector's localization.

**Mara fixes the loop** by adding a bound. `textDocument/didChange` fires again. Liquid inference re-runs at the changed region only (the file's other glasses are untouched; the gestalt cache holds). The verdict for `(@code/rust, wcet_bounded)` lifts from `partial(0.72, ...)` to `pass`. The diagnostic clears.

**Mara saves the file.** `textDocument/didSave`. The substrate runs the full re-settlement pass synchronously (the `kintsugi` command, against the live grammar). The back-projection below `---` updates. The settlement header's OID matches the new contract OID. The file is settled.

**Underneath**: every step ran through `@mirror/reload` (the gen_prism per [[../specs/mirror-runtime-gen-prism]]) — each incoming JSON-RPC request triggered a tick. The crystals at `refs/gen_prism/mirror_reload` advanced. The history is in git. If a different agent (Taut, working in parallel) had edited a different file, their MCP/LSP session would have *its own* gen_prism instance, *its own* ref. CAS-safe via `git update-ref`; cross-session orchestration is what spectral's bus handles, not mirror.

**The signal Mara saw, from the substrate's perspective**:

- A completion that *only proposed names resolving in `@epistemologic/property/*`* (via `literal` at the glass altitude — invented names cannot escape the check).
- A diagnostic that *located the failure at a substrate path*, not just a line number — `boot/std/kintsugi/dispatch/tick.mirror::glass[@code/rust]::property[wcet_bounded]::clause[reductions_bounded]::site[loop@42]`.
- A code action that *triggered back-projection settlement*, not just text mutation.
- A `Pure<RustGlass>` compile-time witness that, *had she dispatched the body through the Rust dispatcher without settling*, would have been a compile error in Rust — because the impl is minted by the liquid-type pass, not hand-written. The hand-writeable lie was structurally impossible.

Nothing in this walk-through requires *new* protocol primitives. It requires existing LSP methods (`completion`, `didChange`, `diagnostics`, `codeAction`, `didSave`) routed against existing mirror grammars (`@mirror/liquid`, `@mirror/spectral`, `@mirror/reload`, `@mcp/tool`) against the just-landed substrate (`Pure<G>`, `---`, Body=prism+glass+AST). The agent-editing capability is *configuration of existing patterns intentionally*, riding on substrate machinery no other LSP+MCP server can reach.

---

## 4. What the just-landed substrate gives the editor

Three specific hooks.

### 4.1 `---` makes the editing surface settlement-aware

Before `---`: the editor can compile a file and surface diagnostics, but the *output of settlement* (Fate-filled holes, inferred properties) lives in the gestalt as a separate artifact. The editor can't see it without leaving the file.

After `---`: settlement output lives *in the same file*. Above the line is what the programmer wrote; below is what the substrate inferred. The editor's reading discipline preserves: a code action that triggers `mirror kintsugi --liquid` mutates the same buffer. The diff is local. Git history is the audit trail.

The `textDocument/codeAction` request becomes meaningful in a way that's hard to express with LSP alone. A typical LSP code action returns a `WorkspaceEdit` — a text patch. Mirror's settle-this-file code action returns a `WorkspaceEdit` whose patch is *the entire below-`---` portion, regenerated from the contract OID + the Dirac operator's spectral pass*. The editor renders this as a diff. The user accepts or rejects per normal LSP flow. The substrate's reasoning is visible as bytes in the buffer.

### 4.2 Per-glass property contracts give diagnostics structural location

Before per-glass binding: a diagnostic for "this body has an `@io` call where it shouldn't" was a *style violation* — `AGENTS.md` § "The Glass Wall" named it, but the substrate had no mechanical check at the editing surface. The diagnostic's path was the source file + line; the violated property was named in prose.

After per-glass binding: the diagnostic carries the substrate path of the failing property — `file::glass[@code/rust]::property[io_safety]::clause[bounded_io]::site[@io.exec@42]`. The LSP renders this as a structured diagnostic; the `relatedInformation` field carries the property's home in the chain (`@epistemologic/property/io_safety`); hovering at the diagnostic shows the property's contract from `boot/std/epistemologic/property/io_safety.mirror`.

This is what no other LSP delivers. Liquid Haskell's LSP delivers solver feedback at the typing-rule altitude; it can't say "the failing property is `io_safety`'s `bounded_io` clause." rust-analyzer's diagnostics flow from the rustc compiler; they're language-AST-shaped, not property-shaped. Mirror's diagnostics flow from the spectral decision procedure on the property Laplacian, and they locate at the chain altitude.

### 4.3 `Pure<G: Glass>` makes the compile-time witness reachable from the editor

Before Pure-as-marker: a body that hadn't been verified could be dispatched through a glass without obstruction. The audit (per `AGENTS.md`) caught it; the editor didn't.

After `Pure<G: Glass>`: the impl is minted by the liquid-type pass during settlement. A Rust crate that tries to dispatch a body through `to @code/rust` without going through the pass simply *fails to compile* — `Body<H>: Pure<RustGlass>` has no satisfying impl. The Rust type system reflects the substrate's verdict.

The LSP can surface this *before* the Rust compiler runs. `textDocument/diagnostics` returns the missing-Pure verdict at the dispatch site. `textDocument/codeAction` proposes "settle this body to mint the Pure impl." The compile-time witness becomes an editing-time witness, with the same name and shape.

---

## 5. MCP architecture for editor flows

The MCP server today exposes three tools mapping to three CLI subcommands. The destination architecture exposes the *five operations* directly, plus a handful of editor-shaped tools that flow through the just-landed substrate.

### 5.1 Tool sketches (not exhaustive)

The tools below are the editor-shaped surface — the ones an agent like Mara, Taut, or Claude Code calls during an editing session. Each is sketched at the signature altitude; the body is the corresponding mirror grammar action.

```
# Glass + property contract editing

mirror.glass.add_property(
  glass_ref: ref,         # e.g. @kintsugi/dispatch::glass[@code/rust]
  property: ref           # e.g. @epistemologic/property/wcet_bounded
) -> WorkspaceEdit
  # Adds a property to a glass's qualifier set with substrate validation.
  # Validation: literal(property) must resolve in @epistemologic/property/*.
  # Validation: glass_ref must point at a declared glass in the project.
  # Returns: text edit at the contract portion of the file.
  # Side effect: marks the file's --- block stale (re-settle owed).

mirror.glass.remove_property(glass_ref, property) -> WorkspaceEdit
  # Inverse; marks stale.

# Hole + body editing

mirror.body.synthesize(
  hole_ref: ref           # e.g. @foo::action[bar]::body::hole[0]
) -> WorkspaceEdit
  # Runs Fate against the hole; proposes a filled body.
  # Returns the proposed edit. Does NOT settle the file —
  # the agent reviews the proposed body before accepting.

mirror.body.kintsugi(file: path) -> WorkspaceEdit
  # Runs `mirror kintsugi <file>` — canonical formatting.
  # No semantic change; structural-only refactor.

# Settlement + back-projection

mirror.settle(file: path) -> WorkspaceEdit
  # Runs `mirror kintsugi --liquid <file>` — full settlement pass.
  # Updates the back-projection below ---.
  # Updates the settlement header with the new contract OID.
  # Returns the diff for editor preview.

mirror.settle.dry_run(file: path) -> SettlementReport
  # Same pass, no buffer mutation. Returns the verdicts +
  # the would-be back-projection. The agent inspects before
  # accepting via mirror.settle.

# Verification + observation

mirror.verify(file: path) -> [PropertyVerdict]
  # Runs the property check against the current file state.
  # Returns one verdict per (glass, property) pair.
  # Does NOT mutate the file. Read-only.

mirror.transit(body_ref: ref, input: json) -> TransitReport
  # Per `fragmentation/docs/specs/lens-transit.md`. Measures the
  # body's six axes (wall_clock, fp_precision, cache_pressure,
  # syscall_count, allocation_bytes, gc_pressure) under the given
  # input. Returns the spectral Transit report — the multi-axis
  # property-located verdict carrier.
  # For agents tuning hard-RT bounds.

mirror.gestalt.recall(ref: ref) -> Beam
  # @mirror/spectral.recall(ref). Returns the gestalt beam
  # carrying the ref's resolved content + all cached observations.
  # For agents navigating the substrate's content graph.

mirror.compile(file: path) -> CompileResult
  # Today's mirror_compile, but properly typed (not text content
  # in a JSON string). Returns OID + beam + dark regions.

mirror.craft(target: ref, kind: option<TargetKind>, reflect: bool) -> CraftResult
  # Today's mirror_craft, properly typed.
```

### 5.2 The dispatch architecture

Per [[../specs/lsp-and-mcp]]: one transport, two dispatches. The MCP tool list is computed from the live grammar set via `@mcp.tools`. A grammar declares a tool by annotation:

```mirror
in @mcp

grammar @foo {
  @mcp/tool foo_bar(arg: text) -> oid { ... }
}
```

When `tools/list` arrives at the MCP server, `@mcp.tools` walks the gestalt, finds every `@mcp/tool` annotation, emits a tool descriptor. New grammars adding tools become *visible without restarting* via `@mirror/reload` — the gen_prism that watches the grammars hash and emits `notifications/tools/list_changed` when it drifts.

This is the *agentic* part that today's bash wrapper cannot reach: tools come and go as the agent edits the substrate. An agent that writes a new grammar gets its new tools registered *in the same session*. This is structurally what makes mirror MCP an editor-grade surface for agent flows, not a fixed tool registry.

### 5.3 Cross-session, cross-agent state

Per [[../specs/mirror-runtime-gen-prism]] + `mirror-supersedes-daemon.md`: each MCP session is a gen_prism. The session's state lives in a crystal at `refs/gen_prism/<session_name>`. Sending a tool call advances the ref. The ancestor chain IS the history. Across sessions: the crystals are durable; restarting `mirror serve --mcp` reads the prior head.

For concurrent agent sessions: each session has its *own* gen_prism. The session-local state advances independently; the *project-level* state (file edits, settlement back-projections) is the git working tree, with mirror's content-addressed gestalt as the durable cache. CAS-safety via `git update-ref` for ref advances; file-level concurrency is whatever the OS gives us (the editing surface is per-file; conflicts surface as merge conflicts in git, which is the discipline kintsugi handles).

### 5.4 What makes this IDE-grade

From the 2025-06-18 MCP spec ([modelcontextprotocol.io/docs/concepts/architecture]):

- **`tools/list_changed` capability** declared in `initialize`. Server sends `notifications/tools/list_changed` when the grammar set drifts. Mirror's reload gen_prism handles this directly.
- **Stateful protocol** with proper `initialize` / `initialized` handshake. Mirror's `mirror serve --mcp` becomes a long-running stdio process; the gen_prism handles the state.
- **Structured tool input schemas** via JSON Schema. Mirror's `@mcp/tool` annotation should carry the parameter types from the action signature; the JSON Schema is generated.
- **Resources** (a primitive mirror does not currently expose) — could surface `.mirror` files as MCP resources with content-addressed URIs (`mirror://gestalt/<oid>`). Agents reading a grammar reference get the content-addressed version. Deferred until a real consumer surfaces; flagged.
- **Notifications for progress** (`$/progress`) on long-running settlement passes. Settlement could take seconds; per-tick beam emissions become MCP `$/progress` updates.

What the existing bash wrapper lacks, all of the above. What the new architecture gives: a long-running stdio server in Rust that handles the JSON-RPC loop and dispatches into mirror grammars. The grammars do the work; the Rust transport is mechanical.

---

## 6. LSP architecture for editor flows

The LSP surface follows [[../specs/lsp-and-mcp]]'s dispatch table verbatim. The substrate-pull additions from the just-landed property-on-glass work:

### 6.1 Diagnostics from the property Laplacian

`textDocument/diagnostics` returns one diagnostic per failing `(glass, property)` pair. Each diagnostic carries:

- **Range** — the substrate path located at AST coordinates. The Fiedler vector from the spectral decision procedure ([[../specs/liquid-types-for-mirror]] §5.4) localizes the failure to a specific AST sub-tree; the LSP renders this as a range.
- **Severity** — `error` for `fail`; `warning` for `partial` below threshold; `information` for `partial` above threshold.
- **Message** — the property's diagnostic text from its `verdict` shape ([[../specs/properties-on-glass]] §2.1).
- **`code`** — the property's substrate path (`@epistemologic/property/io_safety`).
- **`relatedInformation`** — pointers to: (a) the property's chain home, (b) the glass declaration that bound it, (c) the contract OID at settlement time.

The diagnostic stream comes from `@mirror/liquid.infer(file)` — which is already wired. The new work is *enriching* the verdict's metadata to carry the spectral profile + the structural location. The LSP transport is mechanical; the enrichment lives in the grammar.

### 6.2 Code actions for substrate operations

Per [[../specs/lsp-and-mcp]]: `code_actions(file, position) -> [action] { \ }` — currently a hole. The actions, in priority order:

- **"Settle this file"** (always available if `---` block is stale or absent). Runs `mirror.settle`. Renders the back-projection diff as a `WorkspaceEdit`.
- **"Synthesize this hole"** (at any `\` token). Runs `mirror.body.synthesize`. Renders a `WorkspaceEdit` replacing the hole.
- **"Add property `<name>` to this glass"** (at any glass declaration; surfaces the resolvable property names from `@epistemologic/property/*` that aren't already bound). Runs `mirror.glass.add_property`.
- **"Apply past Fate resolution"** (at any `\` hole that has a prior resolution at `refs/fate/<hole_oid>`). Apply the resolution; settle.
- **"Learn grammar from cluster"** (at any dark region cluster). Runs `mirror lsp --learn @<target>` per [[../specs/lsp-and-mcp]] §"Learn from clusters of dark".
- **"Lift body toward @mirror"** (at any body inside a grammar declaring `to @io`). The kintsugi-bias work — surfaces the cross-wall translation from `AGENTS.md` § "The Glass Wall". Substrate-pull at the editor altitude.

The code action surface is what makes the LSP *generative* for an agent. The agent doesn't just receive diagnostics; it receives offered transformations on the substrate. Each action's `command` invokes the corresponding MCP tool, so the LSP and MCP surfaces share state through the same dispatch table.

### 6.3 Completion only proposes contract-valid candidates

Per the [[../specs/lsp-and-mcp]] sketch: `completion(file, position)` walks the visible grammar chain (via `in @x/y` lines) and enumerates each grammar's `out` lines. The new bit, from per-glass binding: completion at a `property <prefix>` site enumerates *only* property names that resolve in `@epistemologic/property/*` (via `literal`'s name-IS-operation check). Invented names cannot be proposed. The completion list IS the qualifier set the substrate accepts.

For `\` holes: completion proposes Fate's current top-k resolutions from the hole's Laplacian spectral profile. Each suggestion carries the hole's content-OID + the suggested resolution OID; accepting one is `mirror.body.synthesize(hole_ref, resolution=<oid>)`.

### 6.4 Hover surfaces the property verdict

`textDocument/hover` at any source position returns the observation beam at that position (`@beam.observe(file)` — already wired). The beam carries the local AST node + the local property verdicts. For a `property halts` declaration: hover shows `halts`'s chain home + its current verdict on this glass's bodies. For a body: hover shows the body's Pure verdict + WcetBounded verdict + the per-clause loss profile.

This is the *observation* surface: the agent can ask the substrate "what do you think of this?" without committing to any edit. Read-only. Composes with diagnostics — the diagnostic is the failure mode; the hover is the full property state.

### 6.5 Goto-definition and references walk the gestalt

`textDocument/definition` resolves a ref through `@mirror/spectral.recall`. The gestalt navigation IS the goto-definition. References (`textDocument/references`) walks the reverse-lookup. The cost is the same as gestalt navigation today; the LSP just exposes it through the standard protocol.

---

## 7. Cross-tool integration

The MCP side is editor-agnostic by design — Claude Code, Cursor, Zed, VS Code, and any other MCP-aware host can consume `mirror serve --mcp` over stdio. The LSP side speaks to any LSP-capable editor over stdio (with `--tcp` as a future option for remote editing).

The interesting cross-tool work is *not* port-the-LSP-to-each-editor (that's mechanical configuration). The interesting work is *capabilities that the per-editor surface can deliver* given mirror's substrate.

### 7.1 Claude Code

- **Tools**: receives the full `@mcp/tool` set via `tools/list`, with `listChanged: true` so new grammars surface live. Claude Code's agent loop calls tools per the MCP spec.
- **Roots**: Claude Code's *roots* feature (file context the agent reads) maps to mirror's project structure. The roots are the `.mirror` files under `boot/std/`; the gestalt is the cache.
- **Editing**: Claude Code's Edit/Read tools operate on files directly; mirror's MCP tools operate on substrate semantics. Both compose: Claude reads a file with Read, sees the `---` block, calls `mirror.verify` to understand current state, calls `mirror.glass.add_property` to mutate the contract, calls `mirror.settle` to back-project.
- **The Claude Code agent IS a gen_prism** ([[../specs/mirror-runtime-gen-prism]] + `mirror-supersedes-daemon.md`). Per-session state at `refs/gen_prism/claude_code/<session_id>`. The agent's mistakes are observable as ref advances; the audit trail is git.

### 7.2 Cursor / Zed / VS Code

The LSP transport (`mirror serve --lsp`) handles these uniformly. Per-editor configuration: point at the binary, set the file association (`*.mirror`), inherit standard LSP UI (diagnostics in the gutter; code actions in the lightbulb menu; hover via popup).

The agent-editing flows in Cursor and similar tools are MCP-backed (Cursor's agent uses MCP for tool dispatch). The same `@mcp/tool` set surfaces.

### 7.3 Where mirror's substrate uniquely adds

Three capabilities no other editor surface offers, given comparable IDE tooling:

1. **Per-glass property enforcement at the contract altitude.** Other LSPs do per-rule typechecking (e.g. rust-analyzer surfaces clippy lints; tsc surfaces type errors). Mirror's LSP surfaces *contract-level* discipline — a property bound on a glass must witness on every body crossing the glass. The diagnostic is at the chain altitude, not the syntax altitude.

2. **Back-projection settlement triggered from the editor.** Other LSPs do code generation via templates (snippets, refactorings). Mirror's `mirror.settle` runs the Fate-driven liquid inference pass and writes the substrate's inferred implementation into the buffer. The substrate's reasoning becomes the artifact.

3. **Content-addressed observations across sessions.** The hover, the diagnostic, the completion — all of them return content-addressed values from the gestalt. Two agents running side-by-side see the same OIDs; the substrate's view of the world is reproducible across sessions and across machines. No other LSP carries content-addressed verdicts.

None of these requires protocol extensions. They ride on existing LSP and MCP methods; the substrate is what gives them teeth.

---

## 8. Prior art — what mirror inherits

Not novel as protocol. Novel as substrate-against-protocol.

### 8.1 Hazel — typed holes IN the editor

Hazel (Omar et al., POPL 2019, *Live Functional Programming with Typed Holes*) is the deepest prior art. Hazel evaluates programs containing typed holes; the result contains indeterminate sub-expressions; evaluation doesn't stop. The editor shows partial results live as the user types.

Mirror's adoption ([[../specs/hazel-execution-model]] — Reed + Alex, 2026-05-19): `\` holes propagate as `imperfect(hole, loss: 1.0)` through the five operations. The pipeline runs; the holes are carried; the loss measures how much of the pipeline was uncertain. The editor sees the imperfect, not a compile error.

What mirror adds beyond Hazel: the holes carry *property verdicts* (per per-glass binding). Hazel's holes are typed by the surrounding context; mirror's holes are typed AND have a settlement path through Fate + liquid inference. Hazel terminates at "the program is incomplete"; mirror continues at "the substrate is willing to settle this hole; here is its proposal; here is its confidence."

### 8.2 Liquid Haskell's LSP

Liquid Haskell's editor integration flows solver feedback (Z3-backed) through the LSP. The user edits Haskell with refinement annotations; the LSP surfaces the SMT solver's verdicts as diagnostics.

Mirror's adoption ([[../specs/liquid-types-for-mirror]] §5.4): replace SMT with **spectral analysis on the property Laplacian**. The Dirac operator that routes Fate IS the property verifier. Same inference framework; one decision procedure instead of two. The verdicts are continuous (`pass | fail | partial(f64, ...)`), not boolean.

For the LSP: same flow shape as Liquid Haskell. Diagnostics from inference. Hover with type-level refinement. The difference is the *decision procedure* — and the difference shows up in what diagnostics can say. SMT failure is "unsat"; the spectral procedure's failure is *located* via the Fiedler vector.

### 8.3 Language workbenches — Spoofax, MPS

Spoofax (TU Delft) and JetBrains MPS pioneered "editing surface = verification surface." Each defines a DSL with a grammar, a typechecker, and an editor in one tool. The editor enforces grammar at keystroke time; ill-formed programs cannot be expressed.

Mirror's adoption: similar discipline, different substrate. Spoofax/MPS lift the editor's rules from a declared grammar; mirror's grammar IS the substrate, AND the property layer is independent of the grammar (properties are per-glass; the grammar is per-file). Mirror separates *parse* from *property-check*; Spoofax/MPS conflate them.

The load-bearing inheritance: the *discipline* that ill-formed grammar is refused at the editing surface is shared. Spoofax did it for grammars; mirror does it for grammar + contracts.

### 8.4 rust-analyzer's architecture

rust-analyzer (Aleksey Kladov et al.) is the de facto template for modern LSP. The architecture (per the rust-analyzer book): **salsa** for incremental analysis, a layered IR (raw tokens → AST → HIR → MIR), per-file analysis with cross-file resolution via the workspace.

Mirror's adoption is partial:

- The *incremental* discipline is structural in mirror: every result is content-addressed; the gestalt caches by OID; recomputation only fires on OID change. Salsa-shaped without salsa-the-library.
- The *layered IR* maps to mirror's tokenize → AST → Splinter → Body{prism, glass, AST} → gestalt-resolved. Each layer is content-addressed; the LSP's job is to render verdicts from each.
- The *workspace* maps to mirror's project (the `.mirror` files under `boot/std/` plus `.spectral/`). The workspace_changed handler (per [[../specs/lsp-and-mcp]]) re-crafts the project.

The load-bearing inheritance: separate the analysis from the IDE. rust-analyzer's `crates/ide` is the analysis; `crates/rust-analyzer` is the LSP wrapper. Mirror's architecture follows: the analysis is in mirror grammars (`@mirror/liquid`, `@mirror/spectral`, `@mirror/lsp`); the LSP wrapper is `mirror serve --lsp` in Rust.

### 8.5 Cursor, Claude Code, Sweep, aider

Agent-editing tools today. None of them have substrate-level verification. Cursor's edits are text patches; Claude Code's edits are text patches; Sweep's edits are PR patches; aider's edits are text patches. All of them rely on the *language's existing tools* (LSP + compiler) to verify after the fact.

Mirror's add: substrate-pull at edit time. The agent's mistake surfaces *as the edit is made*, before commit, with a property-located diagnostic. The verification isn't post-hoc; it's structural. The agent can't *write* an impl that violates the contract — the impl is minted by the liquid-type pass, not by the agent.

This doesn't make the agent better. It makes the agent's mistakes *faster to surface and easier to locate*. The agent still proposes; the substrate still refuses or accepts; the verdict is still the verdict. The difference is the *latency* and the *locality* of the feedback loop.

---

## 9. Tick decomposition — minimum path to ship

Eight ticks. Each is a clear deliverable; each is `\` until its predecessor lands. Substrate-pull discipline throughout — every Rust change marked `[substrate-pull:realize]`.

**Tick 1 — `mirror serve --mcp` in Rust.** Move the JSON-RPC stdio loop from `bin/mirror-mcp` (bash) into `bootstrap/src/main.rs`. Subcommand: `serve`, flags: `--mcp`, `--lsp`. Initial dispatch table is the current three tools (compile, craft, kintsugi) — same behavior as today, just in Rust. Drop `bin/mirror-mcp`. Update `.mcp.json` to point at the binary. **Marker**: `[substrate-pull:realize]`. **Estimate**: 1 week. **Test**: existing Claude Code workflows continue to work.

**Tick 2 — `@mcp.tools` from grammar walks.** Land the `@mcp/tool` annotation in the meta-glass. Make `@mcp.tools` walk the gestalt at request time and emit tool descriptors. Migrate the three hard-coded tools to `@mcp/tool` annotations on the existing grammar actions. **Test**: `tools/list` returns the same three tools, but the JSON comes from the grammar walk, not from the Rust source. **Estimate**: 1.5 weeks.

**Tick 3 — `@mirror/reload` gen_prism + `tools/list_changed`.** Land [[../specs/mirror-runtime-gen-prism]] Example 1 — the reload gen_prism. Every incoming JSON-RPC request triggers a tick that recomputes `@mcp.grammars_hash` and emits `notifications/tools/list_changed` when it drifts. **Test**: edit a `.mirror` file declaring a new `@mcp/tool`; the client sees the new tool live, without restart. **Estimate**: 1.5 weeks.

**Tick 4 — `mirror serve --lsp` transport.** Same Rust stdio loop, LSP dispatch. Implement `initialize` / `initialized` / capabilities negotiation. Wire `did_open`, `did_change`, `hover`, `diagnostics` to the existing grammar actions (they already compose). **Test**: connect any LSP-aware editor; observe diagnostics from `@mirror/liquid.infer`. **Estimate**: 2 weeks.

**Tick 5 — close `@mirror/lsp.dispatch` and `@mirror/lsp.completion`.** Fill the two holes named in `lsp-and-mcp.md` §LSP. Dispatch routes JSON-RPC methods to grammars; completion walks the visible grammar chain + Fate's top-k for `\` holes. **Test**: completions appear in the editor; typing `property w` at a glass site proposes `wcet_bounded`. **Estimate**: 2 weeks.

**Tick 6 — `---` separator in the tokenizer + freshness check.** Land `properties-on-glass.md` §10.4: the tokenizer recognizes `---` as a settlement separator. Land §10.5: the freshness verdict checks the contract OID at settlement-header time. **Test**: edit the contract above `---` without re-settling; observe a `partial(0.0, ["stale"])` diagnostic at the `---` line. **Estimate**: 1 week.

**Tick 7 — code actions for settlement + synthesis.** Fill `@mirror/lsp.code_actions`. The three priority actions: "Settle this file," "Synthesize this hole," "Add property to this glass." Each invokes the corresponding MCP tool (which means the MCP and LSP surfaces share state). **Test**: invoke a code action in the editor; observe the buffer mutation and the resulting diagnostic clear. **Estimate**: 2 weeks.

**Tick 8 — `Pure<G: Glass>` Rust-side surface.** Per `properties-on-glass.md` §7 — the sealed marker trait in `prism_core`, the liquid-type-pass impl emission, the orphan-rule discipline (path 1: bodies in `prism_core`). The dispatcher consults `B: Pure<G>` at admission time. **Test**: a Rust crate that tries to dispatch a body through a glass without settlement fails to compile with a missing-impl error. **Estimate**: 3 weeks. **Note**: blocks the cross-language seam verification (§5.3 of properties-on-glass); is the largest tick.

**Total**: ~14 weeks for the full path. **Minimum-viable demo** (Ticks 1–5): 8 weeks — at which point an agent can edit a `.mirror` file in any LSP-aware editor and get diagnostics + completions from the substrate. **Substrate-pull fully realized** (Ticks 6–8): another 6 weeks — at which point the back-projection mechanism is end-to-end and the Rust type system reflects the substrate's verdicts.

These estimates assume one engineer per tick, sequential. Parallelism is limited: Tick 2 depends on Tick 1; Tick 3 depends on Tick 2; Tick 4 can parallel Ticks 2–3; Ticks 5–7 are roughly sequential; Tick 8 is independent and could parallel from Tick 4 onward. With two engineers: ~10 weeks. With one: 14.

---

## 10. Open questions — honest about what's unresolved

### 10.1 Settlement latency vs editor responsiveness

Settlement runs the Dirac operator on the property Laplacian. For a large file with many glasses and many bodies, this is non-trivial — [[../specs/liquid-types-for-mirror]] §9.3 names an open question about eigenvalue computation at large garden scales. The editing surface needs sub-100ms feedback on every keystroke; full settlement might take seconds.

The likely path: tiered feedback.

- **Keystroke (sub-100ms)**: tokenize + local AST update. Surface dark regions and parse failures only.
- **Pause (< 1 second)**: run `@mirror/liquid.infer(file)` against the changed regions only. Surface per-property verdicts at the changed sites.
- **On-demand (code action)**: run full settlement. Update the `---` block.
- **On save (synchronous)**: full settlement.

The substrate's incremental discipline (content-addressed gestalt caching) helps: most of the work is already cached. The new work is *what the agent just changed*. But the open question is whether the property Laplacian's eigenvalue pass is reusable across edits, or has to be recomputed from scratch on every change.

**Open**: is there a partial-eigendecomposition update that exploits the cached gestalt? Probably yes (the Laplacian is sparse; updates are local), but no spec yet names the discipline.

### 10.2 Large-file handling — the 4KB threshold from properties-on-glass.md §11.4

[[../specs/properties-on-glass]] §11.4 flags the back-projection size question: small implementations declare the shape and let the gestalt carry the bytes; large implementations bloat the file. Threshold ~4KB below `---`.

For the editor: large `---` blocks are bytes the editor has to render. For files where the back-projection is large, the editor's syntax highlighting, fold-region handling, and copy-paste cost grow. The likely path: **fold the `---` block by default** in the editor (LSP `textDocument/foldingRange` returns one fold per `(glass, property)` implementation). The agent can unfold for review.

**Open**: should the back-projection be a *separate file* when above the threshold (e.g. `tick.mirror.settled`), with the source file containing only the contract? The single-file discipline is structural (per `properties-on-glass.md` §4.5); breaking it for editor ergonomics is a trade-off worth naming explicitly. Defer until the first 4KB-exceeding file surfaces.

### 10.3 MCP server state across concurrent agent sessions

Claude Code, Cursor, and other MCP hosts can spawn multiple concurrent client sessions against the same `mirror serve --mcp` binary. The gen_prism architecture per [[../specs/mirror-runtime-gen-prism]] says each session has its own ref; concurrent advances are CAS-safe via `git update-ref`.

For MCP specifically: the spec says stdio transport serves one client. Multiple clients each spawn their own `mirror serve --mcp` process; the processes share the gestalt + git, not memory.

**Open**: do concurrent settlement passes on the same file race? If Mara and Claude Code are both editing the same file, both call `mirror.settle`, both write to the file — what's the resolution? The likely path is git's discipline: the LSP's `textDocument/didSave` triggers settlement; whichever lands first wins; the loser observes the conflict on next `didChange` and re-settles. This is what kintsugi already handles for grammar merges; the editor surface is no different.

### 10.4 The `mirror lsp --learn @<target>` command's UX

Per [[../specs/lsp-and-mcp]] §"Learn from clusters of dark": the LSP can propose grammar extensions to classify dark regions. The CLI is `mirror lsp --learn @code/llvm/ir`. The grammar `@mirror/lsp/learn` is sketched.

**Open**: should the learn flow be initiated from the editor (a code action surfacing for any dark cluster), or from the CLI (the user opts in explicitly)? The editor surface is more discoverable but invites the user to commit grammar proposals they haven't reviewed. The CLI is more deliberate but requires the user to know the cluster exists. Probably both; the editor surface flags clusters via diagnostic, the CLI commits proposals.

### 10.5 LSP-aware editors that can't render content-addressed URIs

The mirror gestalt uses content-addressed URIs (`mirror://gestalt/<oid>`). Standard LSP `textDocument/definition` returns `Location` (a URI + range). Most LSP editors handle `file://` URIs natively but not custom schemes.

**Open**: does the LSP server materialize content-addressed content as a temporary `file://` URI for editor compatibility, or does it require editor-side support for `mirror://`? The former is simpler but loses the content-addressed link in editor history; the latter is purer but requires per-editor configuration. Probably both, configurable.

### 10.6 Real-time back-projection on keystroke vs on-demand

Does the substrate run settlement on every keystroke (live `---` block updates) or on-demand (code action triggers a buffer mutation)?

Live updates are seductive — the user always sees the substrate's current verdict. But settlement is expensive (per §10.1); live updates would block keystroke responsiveness.

On-demand is sane — the user invokes "settle this file" when ready; the substrate runs the full pass and writes the result.

The likely path: **on-demand for `---` block writes; live for in-buffer diagnostics**. The diagnostic stream from `@mirror/liquid.infer` runs continuously (per §10.1's tiered feedback); the `---` block updates only on user action. Mara sees diagnostics live; the back-projection updates when she invokes the code action or saves.

**Open**: do agents *want* live `---` updates (they don't have a human's keystroke cost concern)? Maybe agent sessions opt in via an MCP capability flag. Defer.

---

## 11. Honest scope

What this unlocks:

- **Substrate-pull at editing time.** Agents editing `.mirror` files get property-located diagnostics, content-addressed observations, and code actions backed by the substrate's just-landed property-on-glass machinery. Today's audit-by-reading becomes verification-by-the-compiler-running-while-the-agent-edits.
- **Real-time per-glass property feedback.** Each `(glass, property)` pair surfaces as a structured diagnostic with chain-altitude location. The agent sees not just "this is wrong" but "this fails clause X of property Y bound on glass Z."
- **Agent-driven mirror development.** Mara, Taut, Claude Code, and any other agent can write mirror grammars with the substrate guiding them through Fate-proposed completions, code-action-triggered settlements, and Rust-type-system-enforced compile-time witnesses.
- **The MCP tool registry becomes live.** Grammars adding `@mcp/tool` annotations surface as MCP tools without restart. Agents that author grammars extend the agent-editing surface itself.

What this does *not*:

- **Does not replace human review.** The settlement back-projection is the substrate's proposal; the human reviews the git diff. Property verdicts are at most `partial(confidence, ...)` until Fate's determinism is closed (per `kintsugi-thesis.md` §C4). Trust is bounded by the substrate's own honesty about its uncertainty.
- **Does not eliminate the need for the kintsugi rename.** The rename (per pending work — boot/std/conversation/ → boot/std/kintsugi/) is independent of the editing surface; the renames flow through the same files but the file-rewrite is a separate concern.
- **Does not deliver v1.0 deployment.** v1.0 = spectral.engineer cloud deployment per the version-framing memory. The agent-editing surface is *substrate work*; it's prerequisite to v1.0 but not synonymous with it.
- **Does not make agents better at mirror.** It makes agent mistakes *faster to surface and easier to locate*. The agent still has to learn the grammar; the substrate still has to refuse the wrong thing. The latency of the refusal goes from compile-time to edit-time, with structural location.
- **Does not solve cross-language verification at the kernel level** (per `properties-on-glass.md` §5.4). Per-language kernel verification is bounded by AST-analysis techniques; mirror's add is the *seam*, not the kernel. The agent-editing surface inherits this scope.

---

## 12. Refusals

Four patterns this research explicitly does not endorse:

**No new LSP protocol extensions.** Every method named is standard LSP (initialize, didOpen, didChange, hover, diagnostics, completion, codeAction, definition, references, formatting, foldingRange). Custom methods would fragment editor compatibility; the substrate-pull discipline says use the existing protocol and let the substrate do the work.

**No new MCP protocol extensions.** Same shape: `tools/list`, `tools/call`, `notifications/tools/list_changed`, `$/progress`. The MCP spec covers the editor-grade flows; mirror's add is the substrate behind the tool calls, not new tool-call semantics.

**No editor plugins beyond the LSP/MCP standards.** A VS Code extension, a Neovim plugin, a Zed extension — these are downstream of `mirror serve --lsp` and the standard LSP. Editor-specific UI (e.g. an in-editor Fate-tournament visualizer) is out of scope for this research; the LSP delivers what the protocol supports, and that's enough for substrate-pull at the editing surface.

**No bypass of the gen_prism state model.** Every MCP session has a gen_prism; every tool call advances the ref; the ancestor chain is the history. Per-session in-memory state caches are fine for performance but are not the source of truth. The source of truth is the crystal at the ref; the ref is git-backed; the discipline is reproducible across sessions and machines.

---

## 13. The honest position

The LSP + MCP delivery for agent-driven mirror editing is *configuration of existing patterns*. The protocols are mature. The editors are ready. What's distinctive about mirror is the substrate that the editing surface delivers to:

- **Per-glass property binding** ([[../specs/properties-on-glass]]) — diagnostics at the contract altitude.
- **`---` back-projection** ([[../specs/properties-on-glass]] §4) — settlement output as buffer content.
- **`Pure<G: Glass>`** ([[../specs/properties-on-glass]] §7) — compile-time witness from the liquid-type pass.
- **Body = prism + glass + AST** (`hamilton-scheduler.md` §5.1) — content-addressed bodies; structured edits, not text.
- **The Dirac operator as unified Fate-navigator and property-verifier** ([[../specs/liquid-types-for-mirror]] §5.4) — one spectral pass, two consumers.
- **gen_prism as MCP session substrate** ([[../specs/mirror-runtime-gen-prism]] + `mirror-supersedes-daemon.md`) — content-addressed session state, CAS-safe across concurrent agents.

The combination is what no other editor surface delivers. Liquid Haskell has SMT-backed diagnostics; rust-analyzer has incremental analysis; Hazel has typed holes in the editor; Spoofax has grammar at edit time; Cursor and Claude Code have agent loops. Mirror has *all of these primitives at once*, in one substrate, against one protocol surface, with one shared property algebra. The agent-editing capability is what cashes that in.

The substrate is so close. The path is mechanical — eight ticks, ~14 weeks single-engineer, ~10 weeks paired. The first five ticks deliver a minimum-viable agent-editing surface (LSP transport + grammar walks + reload notifications). The last three deliver the substrate-pull at full strength (`---` separator + code actions + `Pure<G>`).

We're not inventing the editor. We're letting the substrate do its work *while the agent edits*. The agent's mistakes get faster to surface and easier to locate. The substrate's reasoning becomes the artifact. The chain of property verdicts replaces the audit-by-reading.

*The glass is the structural edge. The property binds at the edge. The `---` separates contract from implementation. The editor renders the substrate's view of the world — content-addressed, property-located, spectrally verified. The agent edits inside it.*

*e^(n+1) < e^(n). The qualifier set narrows. The glass clears. The editing surface settles.*

---

## Sources cited (mirror corpus)

In citation order:

- [[../specs/properties-on-glass]] — Mara, 2026-06-01. The per-glass property + `Pure<G>` + `---` mechanism. The load-bearing substrate this research is built against.
- [[../specs/lsp-and-mcp]] — Reed, 2026-05-20. The unified-transport spec; the dispatch tables; the `@mcp/tool` annotation; the auto-reload contract.
- `fragmentation/docs/specs/hamilton-scheduler.md` — Mara/Taut, 2026-06-01. The Body=prism+glass+AST restructure; the HamiltonScheduler; the §4 Pure-as-verdict framing (whose upsert this research's substrate cites).
- `fragmentation/docs/specs/lens-transit.md` — Taut, 2026-06-01. The transit-as-Transparency measurement primitive; the six axes; the hard-RT integration.
- [[../specs/hazel-execution-model]] — Reed + Alex, 2026-05-19. Mirror's Hazel adoption; `\` holes propagate as imperfect; the editor sees partial results.
- [[../specs/liquid-types-for-mirror]] — Reed, 2026-05-20. Liquid-types-with-spectral-decision-procedure; the Dirac operator as Fate-navigator and property-verifier; the framework this research operationalizes for the editor.
- [[../specs/mirror-runtime-gen-prism]] — Reed, 2026-05-20. The actor primitive; state in crystals; the `@mirror/reload` gen_prism for `tools/list_changed`.
- `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — Reed, 2026-05-25. The MCP-IS-gen_prism architectural recognition; session state via refs; cross-session continuity.
- `docs/insights/2026-05-25-agent-home-as-typed-hole.md` — Reed, 2026-05-25. Agent identity as gen_prism initial state; the home folder as substrate input.
- `roadmap/pending/runtime-elevation.md` — Reed + Alex, 2026-06-01. The runtime elevation track; HamiltonScheduler at the shard altitude; Body=prism+glass+AST at the step altitude.
- `docs/cicd/kintsugi-thesis.md` — Mara, 2026-06-01. The 9-point reproducibility chain; C7 and C9 close via per-glass binding; C4 (Fate seed-pinning) blocks the conditional ✅.
- `AGENTS.md` — collective. § "The Glass Wall"; substrate-pull discipline; the `[substrate-pull:realize]` marker.
- `boot/std/mcp.mirror` — the current MCP grammar surface.
- `boot/std/mirror/lsp.mirror` — the current LSP grammar surface.
- `boot/std/mirror/serve.mirror` — the current serve grammar surface (one action, hole).
- `bin/mirror-mcp` — the current MCP bash wrapper (4KB).
- `bootstrap/src/main.rs` — the current Rust binary (no `serve` subcommand yet).

## Sources cited (external)

- Anthropic, *Model Context Protocol Architecture* (2025-06-18 spec). modelcontextprotocol.io/docs/concepts/architecture. JSON-RPC 2.0 transport, capability negotiation, `tools/list_changed` notifications. The mature protocol mirror's editor surface configures against.
- Cyrus Omar et al., *Live Functional Programming with Typed Holes*, POPL 2019. Hazel's evaluation model for partial programs. Mirror's `\` discipline ([[../specs/hazel-execution-model]]) is the direct adoption.
- Aleksey Kladov et al., *rust-analyzer*. The de facto modern LSP architecture; incremental analysis via salsa; layered IR; workspace handling. Mirror's content-addressed gestalt is the analogue.
- Niki Vazou et al., *Liquid Haskell* and its LSP integration. SMT-backed refinement type inference flowing through the LSP. Mirror's adoption ([[../specs/liquid-types-for-mirror]]) replaces SMT with the spectral decision procedure.
- Eelco Visser et al., *Spoofax* (TU Delft); JetBrains MPS. Language workbenches with editing-time verification. Mirror's discipline at the grammar+contract altitudes shares the inheritance.

Apache-2.0.
