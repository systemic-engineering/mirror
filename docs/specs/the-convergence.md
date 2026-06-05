# The Convergence — One Runtime, Four Transports, Five Operations

*2026-06-05. Reed + Alex. Spec (naming what already converged).*

Status: **Red** (proposal — names; doesn't implement).
Branch: `reed/cybernetic-cli` (continues the substrate-floor + cybernetic-cli arc).
Reads from: `cybernetic-cli.md` (this dir), `lambda-shell.md` and `insights/mcp-lsp-unification.md` (in spectral), `tick-4-five-operations.md`, `spec-files.md`.
Does not duplicate them. Names the convergence they imply but none of them states.

---

## 0. The recognition

Three specs landed in the same week, written from three different vantage
points, and they describe the same machine:

- **`lambda-shell.md`** described an interactive shell: three characters
  (`λ>`, `@name>`, `\`), mq as the shell language, history-as-graph with
  eigenvalues, the unnamed peer `@>`, the daemon at `~/.spectral/serve.sock`,
  and **sub-graph agent spawn** — agents get eigenvalue-ordered context
  windows, not worktrees.
- **`mcp-lsp-unification.md`** described a server: the five operations as the
  universal MCP surface, the LSP as a thin translation layer onto the same
  `dispatch_op_*` functions, `evaluate()` and `emulate()` as tools, an `@lsp`
  grammar where every LSP method IS one of the five operations.
- **`cybernetic-cli.md`** described a CLI: porcelain (compile, kintsugi,
  shatter, bootstrap, converse, watch, reflect) over plumbing (focus,
  project, split, shift, settle), every response a conversation with an
  eigenboard + a compose block + (for `settle`) a proof block, the third
  state (`\`) as a first-class working surface.

They are not three products. They are four **transports** of one runtime
over one graph through one algebra. This spec names that.

The cybernetic frame stays load-bearing because the convergence has the
exact shape Beer described for a viable system: one regulator (the daemon),
one algedonic surface (the eigenboard), four channels of observation, one
write channel (`settle`), and the observer is always in the system. The
three specs each saw one face of the same polytope; this spec names the
polytope.

---

## 1. The architecture — one runtime, four transports

### 1.1 The runtime

```
        +-------------------------------------+
        |        spectral serve  (daemon)     |
        |   ~/.spectral/serve.sock  +  stdio  |
        |                                     |
        |   SpectralSupervisor                |
        |   |-- MemoryActor   (SpectralDb)    |
        |   |-- FateActor     (425-param)     |
        |   |-- CompilerActor (MirrorRuntime) |
        |   |-- CascadeActor  (eigenboard)    |
        |   |-- McpActor   ─┐                 |
        |   |-- LspActor   ─┤  protocol heads |
        |   |-- ShellActor ─┤  (NEW)          |
        |   `-- CliActor   ─┘  (porcelain)    |
        +-------------------------------------+
                          |
              +-----------+-----------+
              |  the five operations  |
              |  (focus|project|split |
              |   |shift|settle)      |
              +-----------+-----------+
                          |
                  +-------+-------+
                  |   the graph   |
                  |  (spectral-db)|
                  +---------------+
```

One process. One supervisor. One actor tree. One graph. **One algebra:** the
five operations. Everything visible to the user — whether they typed
`λ> focus`, ran `mirror compile`, opened an editor, or made an MCP tool
call — is a projection of an expression in this algebra against this graph.

### 1.2 The four transports

| Transport | Surface | Input shape | Output shape | Audience |
|-----------|---------|-------------|--------------|----------|
| **λsh**         | interactive shell | mq expressions, peer toggles | mirror-text + eigenboard prompt | human at a terminal |
| **mirror CLI**  | porcelain commands | argv + flags | mirror-text + algedonic exit code | human in a script, CI, hooks |
| **MCP**         | stdio JSON-RPC tool dispatch | tool name + args | mirror-text in `content[]` | agent (Claude, etc.) |
| **LSP**         | tower-lsp adapter | LSP methods + ranges | LSP responses + eigenboard-tagged diagnostics | editor (VS Code, Helix, Neovim) |

All four are **adapters onto the same five operations**. None of them is the
runtime. None of them owns state. None of them can mutate the graph without
going through `settle` (which is in the algebra, not in any transport).

The transports differ only in:
- **Language of input.** mq vs argv vs JSON vs LSP-JSON.
- **Language of output.** mirror-text vs mirror-text vs JSON-wrapped
  mirror-text vs LSP-JSON-wrapped fields.
- **Conversation cadence.** REPL turn vs one-shot vs JSON-RPC turn vs editor
  push.

They do not differ in:
- **What the graph contains.**
- **What the algebra means.**
- **Which operation runs for a given intent.**
- **The eigenboard a settled tick produces.**

That is what "one runtime, four transports" buys: the porcelain verb, the
shell pipe, the MCP tool, and the LSP method that share a name share an
implementation, because they share an operation in the algebra.

### 1.3 mq is the canonical expression language

Every transport accepts mq either directly or as the normal form of its own
surface:

- **λsh:** accepts mq verbatim. `focus |> project @code/rust |> split imports`.
- **mirror CLI:** porcelain expands to mq before dispatch.
  `mirror compile T` ≡ `focus T |> project ast |> shift @target |> settle store`.
  Plumbing IS mq with whitespace for `|>`.
- **MCP:** the tool args, deserialized, ARE an mq expression. The dispatch
  layer is a translation from JSON to mq before evaluation.
- **LSP:** an LSP method on a range is an mq expression scoped to a
  document node. `hover(pos)` ≡ `focus (project document=D |> at pos)`.

The runtime evaluates mq. Everything else translates to it. The substrate
source for mq lives in `shards/`; the canonical envelope (eigenboard +
compose + optional proof) lives in `tick-4-five-operations.md`.

### 1.4 The daemon is the regulator

The cybernetic frame: in a viable system, the regulator must have at least
the variety of the disturbances. The daemon is the regulator because:

- It holds the graph (the state).
- It holds the Fate model (the policy).
- It holds the eigenboard (the algedonic surface).
- It holds the spec (the structural coupling trace).

The transports are **observers**. They are not regulators. They cannot
diverge: there is exactly one source of truth, and they all read it. When
two transports look at the same node, they see the same eigenvalue, because
they are looking at the same graph through the same algebra. Any
disagreement between transports IS a bug, by construction.

This is what makes the convergence non-cosmetic: it isn't that the four
surfaces happen to use similar names; it's that they cannot disagree
without something being broken in the runtime, because they share a graph
and share an algebra, and the names in each surface are derived from the
operation in the algebra that runs underneath.

---

## 2. How the four transports compose

### 2.1 The composition table

Porcelain verb on the left. The same intent expressed across four surfaces.

| Cybernetic verb           | λsh sequence                                                                  | MCP tool call                                | LSP method                                  |
|---------------------------|-------------------------------------------------------------------------------|----------------------------------------------|---------------------------------------------|
| `mirror compile`          | `focus T \|> project ast \|> shift @target \|> settle store`                  | `settle({evaluate: {grammar, source}})`      | `publishDiagnostics` + `documentSymbol`     |
| `mirror kintsugi`         | `settle kintsugi target=T`                                                    | `settle({kintsugi: {grammar, file, strategy}})` | `codeAction(kind=source.kintsugi)`        |
| `mirror shatter`          | `focus T \|> shift @target \|> settle store path=...`                         | `settle({shatter: {target, format}})`        | `formatting` / `executeCommand("shatter")`  |
| `mirror bootstrap`        | `settle bootstrap phase=N`                                                    | `settle({bootstrap: {phase}})`               | `executeCommand("bootstrap")`               |
| `mirror watch`            | (continuous) `focus loss=true` + eigenboard prompt                            | `focus({loss: true})` polled by agent        | `codeLens` per declaration (loss/coupling)  |
| `mirror reflect`          | `shift history \|> focus eigenboard=true`                                     | `focus({eigenboard: true})`                  | `hover` (extended block on declarations)    |
| `mirror compose` *(new)*  | `shift emulate=delta` — no `settle` follows                                   | `shift({emulate: {grammar, delta, predict}})` | **predictiveDiagnostic** (new) — see §3.2  |
| `mirror converse @mara`   | `\@mara` (sub-graph spawn; eigenvalue-ordered context)                        | `settle({spawn: {peer, neighborhood, k}})`   | n/a (LSP is single-author)                  |
| `mirror open <hole>`      | `settle open hole=H`                                                          | `settle({open: {hole}})`                     | `codeAction(kind=quickfix.open-hole)`       |
| `mirror holes`            | `focus holes=true`                                                            | `focus({holes: true})`                       | `codeLens` (per hole) + workspace diagnostics |
| `mirror force <hole>`     | `settle force hole=H`                                                         | `settle({force: {hole}})`                    | `codeAction(kind=quickfix.force-hole)`      |
| `mirror seal <hole>`      | `settle seal hole=H`                                                          | `settle({seal: {hole}})`                     | `codeAction(kind=refactor.seal-hole)`       |
| `mirror revert <tick>`    | `settle revert tick=T` — see §3.3                                             | `settle({revert: {tick}})`                   | `codeAction(kind=source.revert-tick)`       |

Each row is **one algebraic expression** in four notations. The runtime
sees one operation; the transports merely rendered it differently.

### 2.2 Composition rules across transports

Three rules keep the composition consistent:

1. **Algebra invariance.** If two cells in a row produce different
   eigenboards on the same graph state, the runtime is wrong; not one of
   the transports. The proof block (on `settle`) is the only verdict.

2. **Read/write parity.** All four transports can run all four reads
   (`focus`, `project`, `split`, `shift`). All four transports can request
   `settle` — but only the daemon performs it. A transport's `settle`
   request is a **proposal**; the daemon decides and returns the proof.
   This collapses the LSP's "code action" / "rename" / "format" / "format
   on save" methods into one shape.

3. **One conversation per turn.** Every response across every transport
   includes an eigenboard (algedonic surface) and a compose block (the
   substrate's next-utterance proposal). LSP wraps these into hover or
   inline-suggestion payloads; MCP and λsh pass mirror-text directly; CLI
   prints mirror-text to stdout. The conversation is invariant; the
   wrapping is per-transport.

### 2.3 The sub-graph as agent context (λsh native, MCP capable)

λsh's `\@<peer>` is not editor chat. It is **sub-graph allocation**: the
peer is spawned with an eigenvalue-ordered slice of the graph as its
context window, not a flat token buffer. The agent operates on the graph,
not on files. `settle` from the agent produces a commit (files are the
projection).

MCP exposes the same primitive as `settle({spawn: ...})` because it lives
in the algebra: the spawn IS a `settle` on the graph (it creates a new
sub-graph node tied to a peer identity, with edges to the neighborhood).
LSP cannot expose it; LSP is single-author. The CLI exposes it through
`mirror converse @peer`. This is the protocol Glint's agent-to-agent gap
was reaching for; it falls out of the convergence at zero new substrate
cost (the sub-graph is already the graph; the peer is already a node).

---

## 3. Reed's five stories — gap-by-gap resolution

The convergence is good iff each of the five gaps Reed's stories surfaced
closes against a concrete (transport, verb) pair. Four of five close
against specs already on disk. The fifth is named and given a concrete
shape, but is honestly still substrate work.

### 3.1 Aki's discovery gap → the unnamed peer

**Gap.** Aki uses the same focus → project → split → shift pipeline 14
times a week and doesn't know it has a name worth aliasing.

**Resolution.** λsh's **`@>` (the unnamed peer)** is the discovery layer.
It measures command eigenvalues; when a pattern crosses the threshold, it
proposes an alias (`# suggestion: alias ri = focus |> project @code/rust |>
split imports`). The suggestion IS the eigenvalue crossing — not a
heuristic.

**(Transport, verb):** **(λsh, `@>` suggestion)**. The MCP equivalent is
`focus({patterns: true})` — agents can introspect the same eigenvalue map
and surface it to a human peer. The CLI surfaces it through `mirror
reflect` (which reads the same eigenboard). The LSP renders it as a
workspace-level `codeLens` ("you've used this pipeline 14 times — alias
it?"). Same data, four wrappings.

**Status:** closed in the existing spec (`lambda-shell.md` §"The Unnamed
Peer"). The convergence adds the cross-transport projection.

### 3.2 Bo's preview gap → `emulate` / `mirror compose`

**Gap.** Bo wants to see what a proposed grammar change will do *before*
committing.

**Resolution.** `mcp-lsp-unification.md` §6 already specified
`emulate(grammar, delta)` as a `shift` (see-from-there-without-going-there).
The cybernetic CLI already named it `mirror compose`. The convergence:
**name the verb once across all four transports.**

**Decision.** Use **`compose`** as the porcelain name everywhere:

- **λsh:** `shift emulate=delta` — note that no `settle` follows; this is
  the tell that it's preview.
- **MCP:** `shift({emulate: {grammar, delta, predict: [...]}})`.
- **LSP:** introduce **`predictiveDiagnostic`** — a new method that
  publishes diagnostics for a proposed edit *that is not in the document
  yet*. This is what existing LSP can't do; the convergence makes it
  natural because the algebra has `shift(emulate)` already.
- **CLI:** `mirror compose <target> --delta=...`.

**(Transport, verb):** **(any, `shift(emulate)`)**. The porcelain is
`compose` for humans; the plumbing is `shift(emulate)` everywhere else.

**Status:** closed; the convergence resolves the naming ambiguity between
`mirror compose` (cybernetic CLI) and `shift(emulate)` (MCP-LSP) by making
them the same operation under one porcelain name.

### 3.3 Charlie's revert gap → `settle(revert)`

**Gap.** Charlie ran `mirror kintsugi` and wants to roll back to tick N-1
with an honest record that they tried.

**Resolution.** Not yet in any of the three specs. Propose:

```
settle({revert: {to_tick: N}})
```

`settle` because reverting IS a write: it creates a new tick whose state
matches tick N's manifold, with a proof block that records both the
forward proof (what tick N+1 did) and the reverse proof (what tick N+2
restored). The graph never loses history; revert adds a node, it does not
delete one. This is also why it's `settle`, not `shift`: it changes
substrate position. The eigenboard reflects the reverted manifold; the
log records both events. CI sees a `loss_delta = +Δ` and a reason tag
(`reason: revert(N+1 → N)`), so the `loss_after > loss_before` algedonic
signal fires honestly. The user accepted the regression in exchange for
the prior state; the system says so.

**(Transport, verb):**

- **λsh:** `settle revert tick=N`.
- **MCP:** `settle({revert: {to_tick: N}})`.
- **LSP:** `codeAction(kind=source.revert-tick, data={tick: N})`.
- **CLI:** `mirror revert <tick>`.

**Status:** **open** — substrate work. The verb is named; the proof-block
shape needs spec'ing (specifically: how `revert` composes with `\!` and
whether a revert past a `seal` reopens the hole). Forward reference to
`docs/specs/revert.md` (proposed; not written).

### 3.4 Glint's agent-to-agent gap → sub-graph spawn

**Gap.** Glint wants to hand a task to another agent without flattening
its own context into a 200K-token brief.

**Resolution.** λsh's **sub-graph agent spawn** (`lambda-shell.md`
§"Agent Spawn"). The agent's context window IS a sub-graph of the
spectral graph, eigenvalue-ordered, not a flat token stream. The
conversation primitive is `\@<peer>`; the protocol is sub-graph
allocation: the daemon computes the neighborhood (Fiedler-weighted
projection around the task's focal nodes), pins it as the spawn's
context, and routes the spawn's operations through the same algebra.

**(Transport, verb):** **(λsh, `\@<peer>`)** with **(MCP,
`settle({spawn: {peer, neighborhood, k}})`)** as the wire form for
agents handing off to other agents. The CLI exposes it as
`mirror converse @<peer>`. LSP has no equivalent and shouldn't (LSP is
the editor's view of one human's session).

**Two pieces needed beyond what's spec'd:**

1. The **neighborhood selection function** — given a task, which subset
   of the graph does the spawn get? Proposed: top-k Fiedler-weighted
   neighbors of the task's focal nodes, with k determined by the spawn's
   declared budget. (Surfaces in the substrate as a `lens=k` parameter
   on the spawn.)
2. The **return protocol** — when the spawn finishes, what comes back?
   Proposed: a `settle` proof block in the parent's graph, with the
   spawn's sub-graph diffed in as a sub-tree. The parent's eigenboard
   updates; the parent's HEAD moves. The spawn is gone; its work
   remains as graph.

**Status:** **mostly closed** in `lambda-shell.md`; the neighborhood
function and return protocol are open and named.

### 3.5 Dana's `mirror reflect --self` gap → `shift(history, scope: self)`

**Gap.** Dana wants to see her own algedonic trajectory — not the
project's eigenboard, *her* loss-delta per session.

**Resolution.** The graph already has per-session loss-delta nodes (the
log records them every tick). The verb is `shift` (it's a perspective
shift onto the historical projection scoped to a peer identity):

```
shift({history: {scope: "self", since: "...", until: "..."}})
```

**(Transport, verb):**

- **λsh:** `shift history scope=self` (the peer is implicit; `self` means
  "the current shell session's peer").
- **MCP:** `shift({history: {scope: peer_id, since, until}})`.
- **LSP:** not surfaced; this is a self-reflection verb, and LSP doesn't
  represent the user as a peer.
- **CLI:** `mirror reflect --self` (already in `cybernetic-cli.md` §2.2 as
  a flag-extension of `mirror reflect`).

The implementation is a `shift` over the log subgraph filtered by peer.
No new substrate; the data is already there. The cybernetic CLI's
existing `mirror reflect` reads the *system's* eigenboard; `--self`
filters it to the peer.

**Status:** closed; only the flag and the projection are missing in
existing tooling. Both fall out of the convergence at zero substrate cost.

### 3.6 Gap-summary table

| # | Story          | Gap                       | Verb (plumbing)            | Porcelain          | Transport(s)            | Status        |
|---|----------------|---------------------------|----------------------------|--------------------|-------------------------|---------------|
| 1 | Aki            | discovery / aliasing       | `focus(patterns)`          | (none — `@>` UX)   | λsh primary; all four projections | closed |
| 2 | Bo             | preview / impact           | `shift(emulate)`           | `mirror compose`   | all four                | closed (naming unified) |
| 3 | Charlie        | revert / honest regression | `settle(revert)`           | `mirror revert`    | λsh, MCP, LSP, CLI      | **open** — proof shape + `\!` interaction |
| 4 | Glint          | agent-to-agent             | `settle(spawn)`            | `mirror converse @peer` | λsh primary; MCP wire | mostly closed — neighborhood fn + return protocol open |
| 5 | Dana           | self-reflection            | `shift(history, scope=self)` | `mirror reflect --self` | λsh, MCP, CLI       | closed (flag + projection only) |

Two of the open items become **substrate work**; see §5.

---

## 4. What the convergence implies but doesn't yet name

Two items live in the union of the three specs without any of them owning
the name. Surfacing both honestly:

### 4.1 The transport-router

If four transports share one runtime, then **deciding which transport is
talking** is itself an algebraic operation on the daemon side. None of the
three specs names this. It's not the McpActor, not the LspActor — it's
the **dispatch shim that knows mq is mq regardless of how it arrived**.

Proposal: name this the **`TransportActor`** (sibling of McpActor and
LspActor) — owns the translation table between transport-native
expressions and mq, and is the single place where a new transport gets
added (e.g., a future WebSocket transport for editor browsers). Its
contract: in, transport-native message; out, mq + peer identity + budget;
ack, mirror-text rewrapped per transport.

Without this, "one runtime, four transports" remains a slogan; with it,
the convergence has a load-bearing actor.

### 4.2 The peer identity primitive

`@reed`, `@glint`, `@>`, `@mara`, the human at the keyboard — all five
specs treat "peer" as obvious, but **the algebra doesn't yet have a typed
notion of peer**. The substrate vocabulary has `Imperfect` and
`Transparency`; it doesn't have `Peer`. λsh assumes it; sub-graph spawn
needs it; `shift(history, scope=self)` needs it; the structural-coupling
trace in `.spec` records peer-driven overrides but doesn't name the peer
as a type.

Proposal: a **`peer` substrate type** in `shards/glass.mirror` (or
adjacent), with at minimum `{ id, kind ∈ {human, agent, shell}, home_spec,
arousal_threshold }`. Once peer is a substrate type, sub-graph spawn,
self-reflection, and the unnamed peer's threshold-keeping all type-check
through the same primitive. Without it, three of the five gap-closures
above are typed against an implicit object.

---

## 5. Forward references — what substrate work this implies next

Two ticks, both small, both load-bearing:

### 5.1 Tick: `peer` as substrate type

- Declare in `shards/glass.mirror` (or `shards/mirror/peer.mirror`).
- Wire `@<name>` syntax in mq to typed `Peer` references.
- Migrate `lambda-shell.md`, `cybernetic-cli.md`, and
  `mcp-lsp-unification.md` references to the typed form (no surface
  change; substrate stops being implicit).

This is the prerequisite for §3.4 (sub-graph spawn return protocol)
and §3.5 (`scope=self`).

### 5.2 Tick: `settle(revert)` substrate spec

- New spec: `docs/specs/revert.md`.
- Defines: the proof-block shape for revert, the interaction with
  `\!` (force-fill), the interaction with `seal` (does revert past a
  seal reopen the hole?), the algedonic signal for "intentional
  regression" (`loss_delta > 0` with `reason: revert` is **not** a CI
  failure; `loss_delta > 0` without that reason **is**).

This is §3.3 made concrete.

Neither tick adds new operations to the algebra. Both make implicit
substrate explicit. That's the right shape for substrate-pull: the
convergence reveals what the substrate already needed.

---

## 6. What this spec does NOT propose

- **No new transports.** The four are the four. (A future WebSocket head
  is mechanical once §4.1's `TransportActor` exists.)
- **No new operations.** The algebra is still `focus`, `project`, `split`,
  `shift`, `settle`. Everything above is composition.
- **No new shards in this round.** §4.2 and §5.1 propose `peer` as a
  substrate type but defer the actual shard work to its own tick.
- **No reimplementation.** The MCP + LSP + CLI + λsh implementations
  already exist or are spec'd; this names how they share the runtime,
  not how to rebuild them.
- **No collapse.** The four transports do not merge into one binary.
  They share a daemon. That's the convergence: same regulator, four
  observers.

---

## 7. The one-sentence claim

**`λsh`, `mirror`, MCP, and LSP are four transports onto one runtime over
one graph through one algebra; the daemon is the regulator, the eigenboard
is the algedonic surface, mq is the canonical expression, and every
visible verb across every surface is a projection of one of the five
operations against the same spectral state.**

---

## 8. Citations

- **`docs/specs/lambda-shell.md`** — the shell, the unnamed peer, the
  sub-graph spawn, the daemon socket. The transport that named the
  *cadence* (REPL turns) and the *peer toggle*.
- **`docs/insights/mcp-lsp-unification.md`** — the MCP↔LSP unification
  through five operations, `evaluate`/`emulate`, the `@lsp` grammar. The
  transport-pair that named the *adapter layer*.
- **`./cybernetic-cli.md`** — the porcelain verbs,
  the algedonic surface, the third state, the proof block. The transport
  that named the *cybernetic frame*.
- **`docs/specs/tick-4-five-operations.md`** — the canonical mirror-text
  envelope (eigenboard + compose + optional proof) every transport rewraps.
- **`docs/specs/spec-files.md`** — `.spec` as the structural-coupling
  trace; how peer overrides become substrate over ticks.

Inline: Beer (VSM, algedonic signal), Ashby (requisite variety), Glanville
(conversation as substrate), von Foerster (observer in the system).

---

*One runtime. Four transports. Five operations. One graph.*
*The daemon is the regulator. The eigenboard is the algedonic surface.*
*mq is the canonical expression. Every verb is a projection.*
*The observer is in the system. The substrate is the regulator. eⁿ⁺¹ ≤ eⁿ.*
