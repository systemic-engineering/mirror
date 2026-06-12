# The Convergence — One Runtime, Four Lenses, Five Operations

*2026-06-05. Reed + Alex. Spec (naming what already converged).*
*Vocabulary cascade 2026-06-12 (Mara): `transport family` → `@mirror/lens`
family (migration 2026-06-06); `*Actor` → `gen_prism` per
`spectral-runtime.md`; `prism @mirror/transport/<name>` → species under
`@mirror/lens`; `glass` declarations narrowed per `optical-keywords.md`
§1. Structural content preserved; only vocabulary cascades.*

Status: **Red** (proposal — names; doesn't implement).
Branch: `reed/cybernetic-cli` (continues the substrate-floor + cybernetic-cli arc).
Reads from: `cybernetic-cli.md` and `lambda-shell.md` (this dir; lambda-shell lifted from spectral 2026-06-05), `insights/mcp-lsp-unification.md` (in spectral; consolidation pending), `tick-4-five-operations.md`, `spec-files.md`, `optical-keywords.md` (§1 the eight optical keywords), `spectral-runtime.md` (§2 `gen_prism`, §3 supervision).
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
  shatter, bootstrap, join, watch, reflect) over plumbing (focus,
  project, split, shift, settle), every response a conversation with an
  eigenboard + a compose block + (for `settle`) a proof block, the third
  state (`\`) as a first-class working surface.

They are not three products. They are four **lenses** of one runtime
over one graph through one algebra. This spec names that. (The 2026-06-06
family-naming closed the recognition under `@mirror/lens`; the earlier
vocabulary `transport` survives in this spec as a synonym for the
transport-side species — `@mirror/lens/{cli,shell,mcp,lsp}` — and is
narrowed accordingly.)

The cybernetic frame stays load-bearing because the convergence has the
exact shape Beer described for a viable system: one regulator (the daemon),
one algedonic surface (the eigenboard), four channels of observation, one
write channel (`settle`), and the observer is always in the system. The
three specs each saw one face of the same polytope; this spec names the
polytope.

---

## 1. The architecture — one runtime, four lenses

### 1.1 The runtime

```
        +-------------------------------------+
        |        spectral serve  (daemon)     |
        |   ~/.spectral/serve.sock  +  stdio  |
        |                                     |
        |   @spectral/supervisor (gen_prism)  |
        |   |-- memory   gen_prism (SpectralDb)|
        |   |-- fate     gen_prism (425-param) |
        |   |-- compiler gen_prism (MirrorRuntime)|
        |   |-- cascade  gen_prism (eigenboard) |
        |   |-- mcp   gen_prism ─┐             |
        |   |-- lsp   gen_prism ─┤ lens heads  |
        |   |-- shell gen_prism ─┤ (per @mirror/lens) |
        |   `-- cli   gen_prism ─┘             |
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
                  |  (@spectral/db)|
                  +---------------+
```

One process. One root supervisor. One gen_prism tree. One graph.
**One algebra:** the five operations. Everything visible to the user —
whether they typed `λ> focus`, ran `mirror compile`, opened an editor, or
made an MCP tool call — is a projection of an expression in this algebra
against this graph.

Each lens head IS a `gen_prism` specialisation per
`docs/specs/spectral-runtime.md` §2: identity (a `uuid_spectral`), state
as a `@mirror/store` shard ref, and a five-op tool surface as wire
protocol. The supervisor is itself a `gen_prism` whose state surface is
the child-registry shard (§3 of the runtime spec). The earlier draft
named these `MemoryActor` / `FateActor` / `McpActor` / etc. — `gen_prism`
is the load-bearing substrate vocabulary; the `*Actor` names were
pre-cascade Rust-side drift.

### 1.2 The four lenses

| Lens | Surface | Input shape | Output shape | Audience |
|-----------|---------|-------------|--------------|----------|
| **`@mirror/lens/shell`** (λsh) | interactive shell | mq expressions, peer toggles | mirror-text + eigenboard prompt | human at a terminal |
| **`@mirror/lens/cli`** (mirror CLI) | porcelain commands | argv + flags | mirror-text + algedonic exit code | human in a script, CI, hooks |
| **`@mirror/lens/mcp`** | stdio JSON-RPC tool dispatch | tool name + args | mirror-text in `content[]` | agent (Claude, etc.) |
| **`@mirror/lens/lsp`** | substrate-generated LSP adapter | LSP methods + ranges | LSP responses + eigenboard-tagged diagnostics | editor (VS Code, Helix, Neovim) |

All four are **species under the `@mirror/lens` family — projections
onto the same five operations**. None of them is the runtime. None of
them owns state. None of them can mutate the graph without going
through `settle` (which is in the algebra, not in any lens).

The four lenses differ only in:
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

**Note on the LSP head.** `@mirror/lens/lsp` is substrate-generated via
the `@code/rust/lens-server` macro shim — there is no `tower-lsp`
dependency in the runtime crate. The LSP method surface is derived from
the five-op tool surface plus the `@lsp` grammar's method-to-operation
map; the same shim discipline applies to `@mirror/lens/mcp`. The earlier
draft named tower-lsp as the adapter; that was pre-cascade and pre-
`@code/rust/lens-server`.

**Note on λsh and `mirror join`.** λsh and the `mirror join` verb are
**the same lens under two names**: λsh names the running mode
(interactive, persistent), `mirror join` names the entry verb. Same
daemon socket. Same algebra. Same eigenboard. The standalone `λsh`
binary is a thin alias for `mirror join`. "Four lenses" counts the
species once — the verb and the running-mode name are not separate
things. See `lambda-shell.md` §"Entry from the mirror CLI" and
`cli-as-prism.md` §2.1 (the `join` species under `@mirror/lens/cli`).

That is what "one runtime, four lenses" buys: the porcelain verb, the
shell pipe, the MCP tool, and the LSP method that share a name share an
implementation, because they share an operation in the algebra.

### 1.3 mq is the canonical expression language

Every lens accepts mq either directly or as the normal form of its own
surface:

- **`@mirror/lens/shell` (λsh):** accepts mq verbatim. `focus |> project @code/rust |> split imports`.
- **`@mirror/lens/cli` (mirror CLI):** porcelain expands to mq before dispatch.
  `mirror compile T` ≡ `focus T |> project ast |> shift @target |> settle store`.
  Plumbing IS mq with whitespace for `|>`.
- **`@mirror/lens/mcp`:** the tool args, deserialized, ARE an mq expression. The dispatch
  layer is a translation from JSON to mq before evaluation.
- **`@mirror/lens/lsp`:** an LSP method on a range is an mq expression scoped to a
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

The lenses are **observers**. They are not regulators. They cannot
diverge: there is exactly one source of truth, and they all read it. When
two lenses look at the same node, they see the same eigenvalue, because
they are looking at the same graph through the same algebra. Any
disagreement between lenses IS a bug, by construction. (`shards/mirror/lens.mirror`
states this as the family-level invariant: "Two lenses pointed at the
same node MUST see the same eigenvalue; divergence is a bug by
construction.")

This is what makes the convergence non-cosmetic: it isn't that the four
surfaces happen to use similar names; it's that they cannot disagree
without something being broken in the runtime, because they share a graph
and share an algebra, and the names in each surface are derived from the
operation in the algebra that runs underneath.

---

## 2. How the four lenses compose

### 2.1 The composition table

Porcelain verb on the left. The same intent expressed across four surfaces.

| Cybernetic verb           | λsh sequence (`@mirror/lens/shell`)                                           | MCP tool call (`@mirror/lens/mcp`)           | LSP method (`@mirror/lens/lsp`)             |
|---------------------------|-------------------------------------------------------------------------------|----------------------------------------------|---------------------------------------------|
| `mirror compile`          | `focus T \|> project ast \|> shift @target \|> settle store`                  | `settle({evaluate: {grammar, source}})`      | `publishDiagnostics` + `documentSymbol`     |
| `mirror kintsugi`         | `settle kintsugi target=T`                                                    | `settle({kintsugi: {grammar, file, strategy}})` | `codeAction(kind=source.kintsugi)`        |
| `mirror shatter`          | `focus T \|> shift @target \|> settle store path=...`                         | `settle({shatter: {target, format}})`        | `formatting` / `executeCommand("shatter")`  |
| `mirror bootstrap`        | `settle bootstrap phase=N`                                                    | `settle({bootstrap: {phase}})`               | `executeCommand("bootstrap")`               |
| `mirror watch`            | (continuous) `focus loss=true` + eigenboard prompt                            | `focus({loss: true})` polled by agent        | `codeLens` per declaration (loss/coupling)  |
| `mirror reflect`          | `shift history \|> focus eigenboard=true`                                     | `focus({eigenboard: true})`                  | `hover` (extended block on declarations)    |
| `mirror compose` *(new)*  | `shift emulate=delta` — no `settle` follows                                   | `shift({emulate: {grammar, delta, predict}})` | **predictiveDiagnostic** (new) — see §3.2  |
| `mirror join @mara`       | `\@mara` (sub-graph spawn; eigenvalue-ordered context)                        | `settle({join: {peer, neighborhood, k}})`    | n/a (LSP is single-author)                  |
| `mirror crack settle --open <name>` | `settle crack open name=N`                                          | `settle({crack: {open: name}})`              | `codeAction(kind=quickfix.crack-open)`      |
| `mirror crack focus`      | `focus crack=true`                                                            | `focus({crack: true})`                       | `codeLens` (per crack) + workspace diagnostics |
| `mirror crack settle --force <name>` | `settle crack force name=N`                                        | `settle({crack: {force: name}})`             | `codeAction(kind=quickfix.crack-force)`     |
| `mirror crack settle <name>` | `settle crack seal name=N`                                                 | `settle({crack: {seal: name}})`              | `codeAction(kind=refactor.crack-seal)`      |
| `mirror time settle tick=N` | `settle time tick=N` — see §3.3                                             | `settle({time: {restore, tick}})`            | `codeAction(kind=source.time-restore)`      |

Each row is **one algebraic expression** in four notations. The runtime
sees one operation; the lenses merely rendered it differently.

### 2.2 Composition rules across lenses

Three rules keep the composition consistent:

1. **Algebra invariance.** If two cells in a row produce different
   eigenboards on the same graph state, the runtime is wrong; not one of
   the lenses. The proof block (on `settle`) is the only verdict.

2. **Read/write parity.** All four lenses can run all four reads
   (`focus`, `project`, `split`, `shift`). All four lenses can request
   `settle` — but only the daemon performs it. A lens's `settle`
   request is a **proposal**; the daemon decides and returns the proof.
   This collapses the LSP's "code action" / "rename" / "format" / "format
   on save" methods into one shape.

3. **One conversation per turn.** Every response across every lens
   includes an eigenboard (algedonic surface) and a compose block (the
   substrate's next-utterance proposal). LSP wraps these into hover or
   inline-suggestion payloads; MCP and λsh pass mirror-text directly; CLI
   prints mirror-text to stdout. The conversation is invariant; the
   wrapping is per-lens.

### 2.3 The sub-graph as agent context (λsh native, MCP capable)

λsh's `\@<peer>` is not editor chat. It is **sub-graph allocation**: the
peer is spawned with an eigenvalue-ordered slice of the graph as its
context window, not a flat token buffer. The agent operates on the graph,
not on files. `settle` from the agent produces a commit (files are the
projection).

`@mirror/lens/mcp` exposes the same primitive as `settle({join: ...})`
because it lives in the algebra: the spawn IS a `settle` on the graph (it
creates a new sub-graph node tied to a peer identity, with edges to the
neighborhood). Operationally, the spawn is a child `gen_prism` rooted at
the peer's home supervisor (per `spectral-runtime.md` §3) and entangled
with the parent's sub-graph (§4: entanglement edges as sheaf restriction
maps). `@mirror/lens/lsp` cannot expose it; LSP is single-author. The CLI
lens exposes it through `mirror join @peer`. This is the protocol Glint's
agent-to-agent gap was reaching for; it falls out of the convergence at
zero new substrate cost (the sub-graph is already the graph; the peer is
already a node).

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

**(Lens, verb):** **(`@mirror/lens/shell`, `@>` suggestion)**. The
`@mirror/lens/mcp` equivalent is `focus({patterns: true})` — agents can
introspect the same eigenvalue map and surface it to a human peer.
`@mirror/lens/cli` surfaces it through `mirror reflect` (which reads the
same eigenboard). `@mirror/lens/lsp` renders it as a workspace-level
`codeLens` ("you've used this pipeline 14 times — alias it?"). Same data,
four wrappings.

**Status:** closed in the existing spec (`lambda-shell.md` §"The Unnamed
Peer"). The convergence adds the cross-transport projection.

### 3.2 Bo's preview gap → `emulate` / `mirror compose`

**Gap.** Bo wants to see what a proposed grammar change will do *before*
committing.

**Resolution.** `mcp-lsp-unification.md` §6 already specified
`emulate(grammar, delta)` as a `shift` (see-from-there-without-going-there).
The cybernetic CLI already named it `mirror compose`. The convergence:
**name the verb once across all four lenses.**

**Decision.** Use **`compose`** as the porcelain name everywhere:

- **`@mirror/lens/shell`:** `shift emulate=delta` — note that no `settle`
  follows; this is the tell that it's preview.
- **`@mirror/lens/mcp`:** `shift({emulate: {grammar, delta, predict: [...]}})`.
- **`@mirror/lens/lsp`:** introduce **`predictiveDiagnostic`** — a new
  method that publishes diagnostics for a proposed edit *that is not in
  the document yet*. This is what existing LSP can't do; the convergence
  makes it natural because the algebra has `shift(emulate)` already.
- **`@mirror/lens/cli`:** `mirror compose <target> --delta=...`.

**(Lens, verb):** **(any, `shift(emulate)`)**. The porcelain is
`compose` for humans; the plumbing is `shift(emulate)` everywhere else.

**Status:** closed; the convergence resolves the naming ambiguity between
`mirror compose` (cybernetic CLI) and `shift(emulate)` (MCP-LSP) by making
them the same operation under one porcelain name.

### 3.3 Charlie's revert gap → `settle(time)` over existing substrate

**Gap.** Charlie ran `mirror kintsugi` and wants to roll back to tick N-1
with an honest record that they tried.

**Resolution.** The substrate already declares the action.
`boot/std/time.mirror` exposes `action restore(snapshot, ref) -> imperfect`
alongside `replay`, `fork`, `browse`, `step`, and `compare` over the
timeline. `cli-as-prism.md`'s `time` glass (§2.1) condenses these 9
substrate actions into a 5-op view; `time settle tick=N` composes
`time.restore` across the ref-set that constitutes tick N's manifold,
producing a new tick whose state matches tick N. The graph never loses
history; `time.restore` adds a node, it does not delete one. This is
`settle`, not `shift`, because it changes substrate position. The eigenboard
reflects the reverted manifold; the log records both events. CI sees a
`loss_delta = +Δ` and a reason tag (`reason: time.restore(N+1 → N)`), so
the `loss_after > loss_before` algedonic signal fires honestly. The user
accepted the regression in exchange for the prior state; the system says so.

**(Lens, verb):**

- **`@mirror/lens/shell`:** `settle time tick=N`.
- **`@mirror/lens/mcp`:** `settle({time: {restore, tick: N}})` (the
  arg-shape names the substrate action and the tick selector).
- **`@mirror/lens/lsp`:** `codeAction(kind=source.time-restore, data={tick: N})`.
- **`@mirror/lens/cli`:** `mirror time settle tick=N`.

**Status:** **mostly closed** — the substrate carries it
(`boot/std/time.mirror`). What's missing is the proof-block shape for
`time.restore`'s composition (specifically: how `restore` composes with
`\!` cracks, and whether restoring past a sealed crack reopens it). The
`@mirror/lens/cli` `time` surface is named in `cli-as-prism.md`; the
substrate completion is **Track G** (`@epistemologic/reality/time`,
deferred per LRM). The `revert` verb itself dissolved into one operation
on the `time` manifold per `cli-as-prism.md` §5.3.

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

**(Lens, verb):** **(`@mirror/lens/shell`, `\@<peer>`)** with
**(`@mirror/lens/mcp`, `settle({join: {peer, neighborhood, k}})`)** as
the wire form for agents handing off to other agents. `@mirror/lens/cli`
exposes it as `mirror join @<peer>` (per `cli-as-prism.md` §2.1's `join`
surface — same verb, same algebra, four projections). `@mirror/lens/lsp`
has no equivalent and shouldn't (LSP is the editor's view of one human's
session).

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

**(Lens, verb):**

- **`@mirror/lens/shell`:** `shift history scope=self` (the peer is
  implicit; `self` means "the current shell session's peer").
- **`@mirror/lens/mcp`:** `shift({history: {scope: peer_id, since, until}})`.
- **`@mirror/lens/lsp`:** not surfaced; this is a self-reflection verb,
  and LSP doesn't represent the user as a peer.
- **`@mirror/lens/cli`:** `mirror reflect --self` (already in
  `cybernetic-cli.md` §2.2 as a flag-extension of `mirror reflect`).

The implementation is a `shift` over the log subgraph filtered by peer.
No new substrate; the data is already there. The cybernetic CLI's
existing `mirror reflect` reads the *system's* eigenboard; `--self`
filters it to the peer.

**Status:** closed; only the flag and the projection are missing in
existing tooling. Both fall out of the convergence at zero substrate cost.

### 3.6 Gap-summary table

| # | Story          | Gap                       | Verb (plumbing)            | Porcelain          | Lens(es)                | Status        |
|---|----------------|---------------------------|----------------------------|--------------------|-------------------------|---------------|
| 1 | Aki            | discovery / aliasing       | `focus(patterns)`          | (none — `@>` UX)   | shell primary; all four projections | closed |
| 2 | Bo             | preview / impact           | `shift(emulate)`           | `mirror compose`   | all four                | closed (naming unified) |
| 3 | Charlie        | revert / honest regression | `settle(time.restore)`     | `mirror time settle tick=N` | shell, mcp, lsp, cli | **mostly closed** — substrate carries it; proof shape + `\!` interaction open |
| 4 | Glint          | agent-to-agent             | `settle(join)`             | `mirror join @peer`        | shell primary; mcp wire | mostly closed — neighborhood fn + return protocol open |
| 5 | Dana           | self-reflection            | `shift(history, scope=self)` | `mirror reflect --self` | shell, mcp, cli       | closed (flag + projection only) |

Two of the open items become **substrate work**; see §5.

---

## 4. What the convergence implies but doesn't yet name

Two items live in the union of the three specs without any of them owning
the name. Surfacing both honestly:

### 4.1 The lens family (landed 2026-06-06)

If four lenses share one runtime, then **deciding which lens is talking**
is itself an algebraic operation on the daemon side. None of the three
upstream specs named this. The substrate-pull-correct shape is **not a
sibling Rust gen_prism per head** but a **lens family**: each transport
IS a species under `@mirror/lens` over the same algebra, and "routing" is
path-walking against `shards/mirror/lens/` — exactly the
dispatcher-as-path-walk shape `cli-as-prism.md` §5.5 surfaced at the
lens-cli altitude.

The substrate-floor family landed 2026-06-06 (`shards/mirror/lens.mirror`):

```
shards/mirror/
├── lens.mirror               # prism @mirror/lens — the algebra shared across heads
└── lens/
    ├── shell.mirror          # @mirror/lens/shell   (λsh + `mirror join`)
    ├── cli.mirror            # @mirror/lens/cli     (mirror CLI)
    ├── mcp.mirror            # @mirror/lens/mcp     (JSON-RPC stdio)
    ├── lsp.mirror            # @mirror/lens/lsp     (substrate-generated LSP)
    ├── transit.mirror        # @mirror/lens/transit (measurement: runtime cost)
    ├── refract.mirror        # @mirror/lens/refract (measurement: grammar spectrum)
    └── unix.mirror           # @mirror/lens/unix    (OS as audience; v0 materialize-to-tempdir)
```

Each species has the five operations on its lens-specific manifold:
`focus` reads lens-state (connection, session); `project` filters
messages; `split` walks dependent lenses; `shift` re-shapes a message
between lens-native and mq; `settle` runs the operation the message
encodes. The router is the path-walker; there is no dispatch table. The
four transport-side lens heads (`shell` / `cli` / `mcp` / `lsp`) and the
measurement-side lenses (`transit` / `refract`) and the OS-side lens
(`unix`) all sit under one family because they all project the same
algebra through a typed surface and compose under the same
`Transparency<P>` monoid (per `shards/mirror/lens.mirror`'s family
discipline).

Each lens head's runtime realisation IS a `gen_prism` specialisation (per
`spectral-runtime.md` §2): identity (a `uuid_spectral`), state as a
`@mirror/store` shard ref, five-op tool surface as wire protocol. The
substrate names the shape; the Rust-side gen_prism implements it.

Without the substrate lens family, "one runtime, four lenses" remained a
slogan and the Rust gen_prisms carried meaning the substrate should
carry. With it, lenses become substrate-first (additive: a future
WebSocket head is one new `shards/mirror/lens/ws.mirror`; no router edit
required). The earlier draft's `TransportActor` name was pre-cascade
drift; `prism @mirror/lens/<name>` is the load-bearing shape, and the
`*Actor` Rust types of the §1.1 diagram are the `gen_prism` specialisations
that realise each lens species.

### 4.2 The peer identity primitive (already in the substrate)

**Correction (Seam pass, 2026-06-05):** the substrate already has `peer`
as a typed primitive. `boot/std/peer.mirror` declares:

```mirror
type peer = {
  identity:   mirror,        # focus(self):    immutable manifold
  gestalt:    mirror,        # project(self):  the accumulated lens
  tensions:   mirror,        # split(self):    open branches the peer carries
  eigenboard: shard,         # shift(self):    current spectral state
  shatter:    mirror,        # settle(self):   rendered history; ancestor chain
}
```

— the five-axis fixed point where each field is the output of one Prism
operation applied to the peer's own bias_tree. This IS the typed `peer`
λsh, sub-graph spawn, and `shift(history, scope=self)` all reference.

What the convergence reveals is missing is **not** a new peer type but
a **session-binding aperture over the existing one**: how a `peer` is
bound into a current session (which lens they're talking through, where
their `config.spec` lives, when the algedonic surface escalates). The
substrate-pull-correct keyword for this declaration is `aperture` per
`optical-keywords.md` §1.3 — it is exactly the typed beam channel at a
boundary (peer ↔ lens-per-session). Propose a thin aperture alongside
the type:

```mirror
aperture @peer/binding {
  kind              = human | agent | shell    # who is at the keyboard
  home_spec         = ref(spec)                # which .spec is theirs
  arousal_threshold = f64                      # @> escalation point
  lens              = ref(@mirror/lens)        # current head (§4.1)
}
```

The binding is **per-session, per-lens**; the peer is **always the
five-axis fixed point**. Sub-graph spawn types against the peer; the
unnamed peer's threshold-keeping types against the binding. Without
the binding, three of the five gap-closures above carry session state
as implicit object.

The original proposal here (a brand-new peer type with `{ id, kind,
home_spec, arousal_threshold }` declared in `shards/glass.mirror`) was
substrate-pull drift — it would have invented a peer type while the
substrate already had one of a different (and load-bearing) shape. A
subsequent draft typed the binding as a `glass`; per the 2026-06-11
optical-keywords cascade, `glass` narrows to MATERIAL substance only, so
the binding's declaration is an `aperture`. The fix names the existing
type, names what's actually missing (the binding), uses the load-bearing
optical keyword for it, and stops asserting against the substrate
without checking.

---

## 5. Forward references — what substrate work this implies next

Two ticks, both small, both load-bearing:

### 5.1 Tick: `@peer/binding` aperture over the existing `@peer` type

- The peer type already exists at `boot/std/peer.mirror` (5-axis fixed
  point); migrate it to `shards/std/peer.mirror` as part of the broader
  shards/ migration arc.
- Declare a new `@peer/binding` aperture in `shards/peer/binding.mirror`
  with `{ kind, home_spec, arousal_threshold, lens }` — the
  session-binding fields the convergence needs but the peer-identity
  type does not (and should not) carry.
- Wire `@<name>` syntax in mq to typed `peer` references; bindings
  attach per-session via the daemon.
- Migrate `lambda-shell.md`, `cybernetic-cli.md`, and
  `insights/mcp-lsp-unification.md` references to the typed form (no
  surface change; substrate stops being implicit).

This is the prerequisite for §3.4 (sub-graph spawn return protocol)
and §3.5 (`scope=self`).

### 5.2 Tick: `time` surface-on-substrate composition

- The `time` surface lives at `shards/mirror/lens/cli/time.mirror`
  (per `cli-as-prism.md` §2.1; cli-as-prism's path namespace itself
  migrates under `@mirror/lens/cli` in the cli-as-prism cascade tick)
  and presents the 5-op view over `boot/std/time.mirror`'s 9 substrate
  actions. The composition map:
  `focus` ← `enter`; `shift` ← `browse` + `step`; `split` ← `fork`;
  `settle` ← `restore` (composed across the ref-set of the target
  tick); `project` ← timeline filter (`timeline.snapshots` filtered
  by predicate).
- Proof-block shape for `time.settle` (composed restores): the
  algedonic signal for "intentional regression" (`loss_delta > 0` with
  `reason: time.restore` is **not** a CI failure; `loss_delta > 0`
  without that reason **is**).
- Crack interaction: does `time.settle` past a sealed crack reopen it?
  Default: yes — restored state IS the state, including the open `\`.
  Spec out in **Track G** (`@epistemologic/reality/time`, deferred per
  LRM).

This is §3.3 made concrete. The revert verb dissolved into one
operation on the time manifold (per `cli-as-prism.md` §5.3); this
tick is the substrate-side completion of the same dissolution.

Neither tick adds new operations to the algebra. Both make implicit
substrate explicit. That's the right shape for substrate-pull: the
convergence reveals what the substrate already needed (binding glass,
composition map).

---

## 6. What this spec does NOT propose

- **No new lenses.** The four transport-side species are the four. (A
  future WebSocket head is mechanical now that the `@mirror/lens`
  family landed — one new `shards/mirror/lens/ws.mirror`, no router
  edit.)
- **No new operations.** The algebra is still `focus`, `project`, `split`,
  `shift`, `settle`. Everything above is composition.
- **No new shards in this round.** §4.2 and §5.1 propose `peer` as a
  substrate type but defer the actual shard work to its own tick.
- **No reimplementation.** The MCP + LSP + CLI + λsh implementations
  already exist or are spec'd; this names how they share the runtime,
  not how to rebuild them.
- **No collapse.** The four lenses do not merge into one binary.
  They share a daemon. That's the convergence: same regulator, four
  observers.

---

## 7. The one-sentence claim

**`λsh`, `mirror`, MCP, and LSP are four lenses (species under
`@mirror/lens`) onto one runtime over one graph through one algebra; the
daemon is the regulator, the eigenboard is the algedonic surface, mq is
the canonical expression, and every visible verb across every surface is
a projection of one of the five operations against the same spectral
state.**

---

## 8. Citations

- **`./lambda-shell.md`** — the shell, the unnamed peer, the sub-graph
  spawn, the daemon socket. The lens that named the *cadence*
  (REPL turns) and the *peer toggle*. Lifted to mirror canonical home
  2026-06-05; substrate-pull corrections applied (`zoom`→`shift`,
  `refract`→`settle`).
- **`docs/insights/mcp-lsp-unification.md`** — the MCP↔LSP unification
  through five operations, `evaluate`/`emulate`, the `@lsp` grammar. The
  lens-pair that named the *adapter layer*.
- **`./cybernetic-cli.md`** — the porcelain verbs,
  the algedonic surface, the third state, the proof block. The lens
  that named the *cybernetic frame*.
- **`docs/specs/tick-4-five-operations.md`** — the canonical mirror-text
  envelope (eigenboard + compose + optional proof) every lens rewraps.
- **`docs/specs/spec-files.md`** — `.spec` as the structural-coupling
  trace; how peer overrides become substrate over ticks.
- **`docs/specs/optical-keywords.md`** — the eight optical schematic
  keywords (§1); the source of authority for the `prism` /
  `glass` / `aperture` cascade applied to this spec.
- **`docs/specs/spectral-runtime.md`** — `gen_prism` (§2), the
  supervision tree (§3), entanglement (§4), the ouroboros pipeline
  (§5). The source of authority for the `*Actor` → `gen_prism` cascade.
- **`shards/mirror/lens.mirror`** — the lens-family root (landed
  2026-06-06). The substrate-side anchor for §4.1's family-naming.

Inline: Beer (VSM, algedonic signal), Ashby (requisite variety), Glanville
(conversation as substrate), von Foerster (observer in the system).

---

*One runtime. Four lenses. Five operations. One graph.*
*The daemon is the regulator. The eigenboard is the algedonic surface.*
*mq is the canonical expression. Every verb is a projection.*
*The observer is in the system. The substrate is the regulator. eⁿ⁺¹ ≤ eⁿ.*
