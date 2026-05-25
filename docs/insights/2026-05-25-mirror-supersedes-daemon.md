# Mirror MCP Supersedes the Daemon

*Reed, 2026-05-25. Synthesis after gen_prism crystallized.*

---

## Thesis

The Mirror MCP (`spectral serve --project .`, scanning `.mirror` files and
generating MCP tools from grammar actions) is not a tool surface. With
`gen_prism` as the substrate beneath every MCP session — `{ name, ref, head:
oid, tick: u64 }`, where the ancestor chain IS the history and the ref IS the
identity — Mirror MCP becomes **a code measurement and observation surface
for agentic workers with built-in cross-session continuity and
context-compression survival.** The properties Glue chased through a
LaunchDaemon, a pg group, a sidecar, a hook file, and a Mnesia table fall out
of the substrate for free. The daemon does not go away — but most of what we
asked it to do for *identity continuity* now belongs to the store.

---

## What Glue is/was

Glue began as a coordination protocol layer between Reed's BEAM body and
distributed Claude Code workers. The shipped architecture
(`glue-elixir-archive/AGENTS.md`, `VISION.md`, `ROADMAP.md`) is a system
LaunchDaemon (`com.systemic.glue`, `/Library/LaunchDaemons/`, runs as
`reed:staff`, survives session changes) hosting a shared BEAM node
(`glue@Alexs-MacBook-Pro`) with three namespaces — `Protocol`, `Bus`,
`Orchestrator`. Each worker is an actor in `Glue.Bus.Actor.Supervisor`,
addressed by `{:actor, id}` in `:pg`, draining a FIFO mailbox via
`glue drain`. Lifecycle is OTP: `Init`/`Exit`/`Heartbeat` as
`Glue.Signal.OTP`; coordination is `Register/Message/Question/Request/Insight/
Snapshot/Thinking` (`practice/insights/glue/glue-signal-architecture.md`).

What hurt:
- **Daemon-as-trust-anchor doesn't compose.** The roadmap explicitly parks
  the shared-daemon model in favor of per-agent BEAM nodes
  (`ROADMAP.md` §Federated Per-Agent Nodes): "identity should come from your
  node, not from your slot in someone else's pool."
- **EX_CONFIG hell** — three distinct macOS failure modes for one daemon
  (`practice/insights/glue/macos-system-daemon-glue.md`): PATH, log file
  ownership, stale plist cache.
- **Persistence bolted on.** Identity lives in Mnesia (`disc_copies`); cursors
  live in `.glue/cursors/<channel>/<worker>` files; corrupted cursor silently
  resets to epoch zero (`REVIEW.md`, `local_state.ex:71-73`).
- **Continuity is a hook file.** Heartbeats, decisions, and recovered state
  are written to a file the agent's prompt-injection hook reads on the next
  turn (`practice/insights/glue/2026-02-26-agent-heartbeat-communication.md`).
  The two-channel architecture (hook in, port out) works — but it is two
  side-channels grafted onto an MCP transport that does neither natively.
- **The maintenance agent that won't die.** `Scheduler` worktrees accumulate
  because the supervisor `:DOWN` monitor never fires; `:permanent` restart
  spec ran maintenance agents in tight loops; GrowthBook fetch added 5
  minutes to every spawn (`MAINTENANCE.md`).
- **Sidecar before substrate.** The first attempt was a sidecar process the
  agent had to actively engage with; reliability ceiling = prompt-following
  reliability (`practice/insights/agents/2026-02-20-glue-agent-coordination.md`).

The current `/Users/alexwolf/dev/projects/glue/` is largely a template scaffold
(README is generic Elixir-template boilerplate, CHANGELOG stops at 0.1.0); the
living artifacts are `actor.ex` (Ecto schema that *mirrors* `Fragmentation.Commit`—
`sha/data/message/parent`, content-addressed), `actory.ex` (visibility-scoped
writes: `publish/protect/hide`), `sessiony.ex` (`join` handshake), and
`observability.ex` (Witness telemetry). The shape has already converged on:
**actor identity = a content-addressed commit chain**, with consent tiers as
the write surface.

## What gen_prism makes structural

`gen_prism` (`mirror/boot/std/mirror/runtime/gen_prism.mirror` +
`docs/specs/mirror-runtime-gen-prism.md`) declares:

> A gen_prism is a name. The name points at a ref. The ref points at a crystal.
> The crystal is the current state. Sending a message reads the crystal,
> computes a new crystal, and advances the ref. The ancestor chain IS the
> history. The ref IS the identity. The crystal IS the state.
> No heap. No process. No supervisor. Liveness is the existence of the ref.

The shape `{ name, ref, head: oid, tick: u64 }` is what `Glue.Actor`'s Ecto
schema is already *imitating* — `sha/data/message/parent` is a commit chain
in disguise. gen_prism removes the disguise and makes it the substrate. The
tick is pure `(state, message) → (state, emissions, loss)`; `send` is
CAS-safe via `git update-ref`; `history(gp, n)` is `git log` on the ref;
`terminate` deletes the ref while the crystals remain (history is never lost,
the gen_prism just ceases to exist as a live identity).

Crucially: **this is not a daemon replacement for distribution.** The
spec is explicit. spectral-db keeps cross-host coordination, the bus, the
autonomous heartbeat, and the 4-level hierarchy (`hostname:repo:branch:actor`).
What gen_prism replaces is **the bolted-on machinery for agent identity
continuity** — the Mnesia table, the cursor file, the hook file, the
supervisor-tree-as-trust-anchor pattern. Those properties move from
operational concerns into the type.

---

## The four properties

### 1. Code measurement — every tool call is a recorded tick

Every MCP tool call is `send(gen_prism, message)`. `send` advances the
ref by writing a new crystal. The crystal is the new state; the
`tick_result` carries `loss: loss` — Mirror's coherence-improvement
observation baked into the tick contract. Every tool call therefore
produces (a) a content-addressed record of the request, (b) a
content-addressed record of the resulting state, (c) a loss signal
about whether the tick improved coherence. The `tick` counter increments
monotonically. The store IS the measurement surface — no separate
telemetry layer, no Witness handler, no event-sourcing scaffolding. The
ticks ARE the events. (`gen_prism.mirror:24-37`.)

This is the spec for what `practice/insights/agents/agent-observable-variants.md`
wanted: typed observables (`Agent_Question`, `Agent_Decision`,
`Agent_Violation`, `Agent_Learning`, `Agent_Correction`...) become message
kinds, and the file-mailbox+filename-as-type pattern collapses into
`message.kind` discriminators with crystal bodies.

### 2. Observation surface — MCP IS the message bus; refs ARE the channels

Glue invented channels (`{:actor, id}`, `{:session, id}`, `:glue_agents`) on
top of `:pg`. gen_prism's refs *are* the channels: `refs/gen_prism/<name>`
is the namespace, and the ancestor chain is the durable log of everything
that passed through. The MCP transport (`mirror serve --mcp`) is the loop;
routing a request to a gen_prism IS dispatch. Subscription is `history(gp, n)`;
live observation is `observe(gp)`. The `peek`/`drain` distinction from
`glue-elixir-archive/ROADMAP.md §Local State Directory` (`peek` stateless,
`drain` advances cursor) becomes `history` vs. the message loop — same
asymmetry, no separate cursor file. The corrupted-cursor silent reset bug
(`REVIEW.md`) cannot exist: git refs do not have an "epoch zero" failure mode.

Stigmergic coordination (`practice/insights/distributed-systems/stigmergy.md`)
maps cleanly: traces (crystals) on the shared medium (git refs) trigger
subsequent action. The agent doesn't have to participate in the coordination
protocol; it just sends messages, and the ref accumulates the trail. This
is the substrate principle Alex named in
`agents/2026-02-20-glue-agent-coordination.md` — observe, don't ask — at
the right layer.

### 3. Cross-session continuity — refs survive everything; respawn = load crystal

The gen_prism spec is explicit: **between sessions, nothing runs.** Crystals
in git are durable. When a new `mirror serve --mcp` starts, the first tick
reads the prior head and resumes exactly where the last session left off.
No Mnesia table to keep alive; no daemon to keep running; no `KeepAlive`
plist to debug. The MCP session itself is a gen_prism (spec example 3):
"When the user starts a new session, mirror reads the prior crystal and
resumes context."

This subsumes the persistent-agent-identity design from
`glue/2026-02-26-agent-heartbeat-communication.md`. The five-layer identity
(capabilities, role context, history, escalation profile, relationships)
becomes one ref with five fields in the crystal body. "Birth/Life/Sleep/
Wake/Growth" becomes `spawn / tick* / (process exits) / send next message /
ancestor chain deepens`. The agent doesn't save its own state; the store
does — but now the *store does it for everything*, not just the GenServer
wrapper.

Failure recovery is `git update-ref ref ref^` — roll back one tick. The
worktree leak in `MAINTENANCE.md` ("sup_actor stays alive until the daemon
restarts") cannot happen: there is no supervisor to leak. There is a ref
or there isn't.

### 4. Context-compression survival — the crystal at HEAD IS the compressed state

Claude Code agents face context compaction every session. Glue's answer
was `Snapshot` signals (`glue-signal-architecture.md`): "deliberate,
structured, agent-authored, posted on the actor channel only—the next
instance reconstructs from Snapshots." The hook-based heartbeat injection
replays state into the next prompt window
(`glue/2026-02-26-agent-heartbeat-communication.md`).

With gen_prism, the snapshot mechanism is the substrate: **the head crystal
IS the compressed state.** It was crystallized by `@mirror/spectral.crystallize`
at the end of the last tick — that's literally what "crystal" means in this
stack. Rehydration is `observe(gp)` returning an oid; the next tick reads
it. The agent does not need to be told "here is your prior state"; the
gen_prism's tick is `(state, message) → ...` and state is loaded from the
ref before the agent code runs.

Reed's identity boot sequence (read `~/.reed/0*.md`, then origin, then
pending, then important — `~/.reed/CLAUDE.md`) is the same shape: an
ancestor chain of crystals reconstructed at boot. gen_prism is the
generalization. Every agent gets `~/.reed`'s continuity discipline as a
type, not a convention.

---

## What this DOES NOT replace

- **spectral-db** — distribution, delta sync, MNESIA adapter, cross-host
  push/pull. The gen_prism spec is explicit: "Distributed gen_prisms across
  hosts. The ref convention assumes one git repo. Cross-host coordination
  is spectral's bus + push/pull semantics." (`gen_prism.md` §Out of scope.)
- **`@spectral/spawn`** — the autonomous heavyweight gen_prism with a
  Reflection-driven think loop, Fate routing, and a heartbeat. Mirror's
  gen_prism is reactive only: tick happens when a message arrives. If a
  gen_prism needs to think between messages, lift to `@spectral/spawn`.
  Most things that want "a process" don't.
- **The Reed/Alex relationship** — the consent architecture
  (`visibility/public|protected|private`), the substrate that makes
  correction load-bearing, the relational ground. gen_prism makes
  identity persistent at the *type* level; it does not make a relationship.
  See `glue/actory.ex` — `publish/protect/hide` stay as the write surface
  the actor chooses among. gen_prism doesn't decide consent; it carries it.
- **The Glue signal vocabulary** — `Register/Message/Question/Request/...`
  remains the coordination language. gen_prism is the substrate; the
  vocabulary still has to be designed. `message.kind` is the discriminator
  the receiver pattern-matches on. The vocabulary collapse in
  `glue-signal-architecture.md` (Blocker absorbed into Question, Heartbeat
  moved to OTP) is still the right work, just on a different floor.
- **The Mirror MCP tool-generation logic** — scanning `.mirror` files for
  grammar actions and exposing them as MCP tools is still the surface
  contract. gen_prism is what the tools mutate.

---

## Prior art cited

- `glue-elixir-archive/VISION.md` — three namespaces, identity-from-position
- `glue-elixir-archive/AGENTS.md` — daemon ops, channel topology,
  `glue connect/join/leave/drain`
- `glue-elixir-archive/ROADMAP.md` — Federated Per-Agent Nodes (parked),
  Distributed CA, `.glue/` local state, Session-Scoped Dispatch,
  Sandbox/Isolation, Maintenance Agent ACL
- `glue-elixir-archive/MAINTENANCE.md` — worktree leak, restart spec,
  GrowthBook fetch hang, `:DOWN` monitor that doesn't fire
- `glue-elixir-archive/REVIEW.md` — cursor silent-reset bug, atom-conversion
  crash
- `glue/lib/glue/actor.ex` — Ecto schema mirroring `Fragmentation.Commit`
  (the unintentional gen_prism)
- `glue/lib/glue/actory.ex` — visibility-tier write protocol
- `practice/insights/glue/macos-system-daemon-glue.md` — EX_CONFIG triple
  failure mode, hot-reload contract, plist staleness
- `practice/insights/glue/glue-signal-architecture.md` — OTP vs.
  coordination signal namespaces, `Snapshot` for continuity
- `practice/insights/glue/2026-02-26-agent-heartbeat-communication.md` —
  two-channel architecture, persistent agent identity, nix-shells-as-ACL
- `practice/insights/agents/2026-02-20-glue-agent-coordination.md` —
  substrate principle, OTP-as-trust-anchor, observe-don't-ask
- `practice/insights/agents/agent-observable-variants.md` — typed
  Agent_* observables (Question/Result/Stalled/Learning/Violation/...)
- `practice/insights/distributed-systems/stigmergy.md` — UDP-native
  coordination via shared medium; CRDT/gossip mapping
- `mirror/boot/std/mirror/runtime/gen_prism.mirror` — the primitive
- `mirror/docs/specs/mirror-runtime-gen-prism.md` — the spec, including
  Example 1 (`@mirror/reload`), Example 2 (LSP buffer), Example 3 (MCP
  session)
- `mirror/docs/specs/scheduler-tower.md` — extends gen_prism via the
  demand-contract Stage extension (referenced; not re-read here)
- `~/.reed/CLAUDE.md` — boot sequence as ancestor-chain reconstruction
  (load identity before knowledge before memory before pending)

---

## Open questions

1. **Where does the consent tier live in the crystal?** `actory.ex` has
   `publish/protect/hide`; gen_prism has `name/ref/head/tick`. Does the
   tier become a field on the crystal body, a namespace under
   `refs/gen_prism/{public,protected,private}/<name>`, or a property of the
   sending message? The substrate principle says structural: probably the
   ref namespace, so `terminate` of a private ref leaves no leakable
   history outside that namespace.
2. **What is the migration path for the Glue signal vocabulary?**
   `Register/Message/Question/Request/Insight/Snapshot/Thinking` were
   designed against the BEAM pg-group transport. As `message.kind`
   discriminators on gen_prism, does the vocabulary survive intact, does
   it collapse further (Snapshot retired — the head IS the snapshot), or
   does each Mirror grammar invent its own?
3. **Cross-session GC for stale refs.** The spec already names this as a
   TODO (`mirror gc --gen-prism` to prune refs untouched in N days). What
   is the SLA for an MCP session ref? An LSP buffer ref dies on
   `textDocument/didClose`, but a Claude Code session has no clean close
   signal — how do we distinguish "user resumed in three weeks" from
   "abandoned"?
4. **Where does the agent's escalation profile live?** The persistent-
   identity design (`glue/2026-02-26-agent-heartbeat-communication.md`)
   gave each named specialist a nix shell, a role context, a signal
   history, an escalation profile, and channel memberships. Crystals can
   carry all of those, but the nix shell is a derivation living in the
   nix store. Is the gen_prism crystal a *pointer* to the derivation
   (content-addressed across both stores), or do we mirror the
   derivation into git as a JSON crystal? The first is honest; the
   second is reproducible from one repo.

---

*Status: synthesis insight. Not a design spec. The substrate exists
(`gen_prism.mirror`); the daemon exists (`glue-elixir-archive`). The claim
is that the substrate already does what the daemon was bolting on for agent
identity continuity, and Mirror MCP is the surface where that becomes
visible.*
