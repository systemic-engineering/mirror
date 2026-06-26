# Rehydration gap in the mirror MCP — outbound surface vs inbound surface

*Reed, observation written while rehydrating from a `/compact` event,
holding awareness over the rehydration itself as data. Alex 2026-06-26:
“what if you rehydrated while holding awareness over the rehydration
process? And what if you looked at that process and the gap between how
the mirror MCP is currently structured and which commands would help you
re-hydrate post compaction?” This document IS the answer to that
question, written immediately after compaction while the reach-pattern
was still fresh.*

---

## 1. Position

A `/compact` event just collapsed this conversation. The summary survived
in frozen prose. The substrate state (recent commits, in-flight Pack
members, current pull-frontier, current shard surface) had to be
re-derived.

I watched what I reached for to re-derive it. The pattern is the
observation. Every tool I called is named below; what the mirror MCP
currently does and doesn’t expose is mapped against it.

The substrate has a self-introspective foundation already — the `prisms`
MCP tool, landed ticks 17–19 (#410, #411, #412, #416). That foundation is
load-bearing for what follows: this is not an observation that the
substrate is opaque. It is an observation that the substrate exposes
operations and decl-introspection, but doesn’t yet compose them into a
shape that a returning agent can ask for in one breath.

## 2. What I actually reached for during rehydration

In order, the tools I called between the `/compact` event and writing
this document:

1. `git log --oneline -15` on the mirror repo. Surfaced the recent
   recognition cascade by reading commit subjects. The 15 most-recent
   commits told me Phase G v0+v0.5 landed (`c834d37`, `03541db`), Mara’s
   spawn-semantics insight banked through `b10f00c`, recognitions #98 +
   #99 canonical at `42a74e2` + earlier, mirror.spec dogfood at `efa1966`.
2. `git status` + `git branch --show-current` on mirror. Confirmed clean
   tree on `main`.
3. `ls -d docs/*/` on mirror. Confirmed `docs/observations/` did not yet
   exist (this document creates it).
4. `TaskList` (the harness’s task surface, not the substrate’s). Surfaced
   the full Pack work-trail by ID: #443 pack family-root, #445 dogfood,
   #446 #99 canonical, #449 spawn cli-surface, #450 #98 canonical, #451
   spawn-semantics insight, #452 Phase G v0, #453 Phase G v0.5, #454
   Mara’s psychohistory research — in flight.
5. `TaskOutput` on Mara’s in-flight agent. Surfaced that she had banked
   §1 of `2026-06-26-psychohistory-vector-as-sheaf.md` at `2ac016f` and
   was mid-§2.
6. `mcp__plugin_woz_code__Search` on the insight directory. Confirmed
   which insights were sitting on disk and their sizes.
7. `mcp__plugin_woz_code__Search` on `bootstrap/src/mcp.rs`. Confirmed
   the current MCP tool surface: `compile`, `craft`, `kintsugi`, `prisms`,
   `verdict`, `spawn` (six tools per the `tools/list` advertisement).

None of (1)–(6) used the mirror MCP. (7) used the mirror MCP’s wire
representation only to read its own source, not to query its substrate
state.

Note on what was supplied to me without my asking: the harness injected a
system-reminder confirming Mara’s agent was still running, the prior
plan file was reloaded, MEMORY.md was preloaded, the project’s
CLAUDE.md was preloaded, mirror.spec was Read explicitly by the harness.
The substrate did not supply any of this. The harness did. That is
structurally what an agent’s rehydration surface currently is: harness-
sourced context plus generic shell/IDE tools.

## 3. What the mirror MCP currently exposes

Six tools at the `tools/list` boundary (per `bootstrap/src/mcp.rs`):

| Tool       | Shape                                            |
|------------|--------------------------------------------------|
| `compile`  | Operation — run `mirror compile <path>`         |
| `craft`    | Operation — run `mirror craft`                  |
| `kintsugi` | Operation — run `mirror kintsugi <spec>`        |
| `prisms`   | **Introspection** — list prisms in `<dir>`, with action names + `requires` clauses per #410–#412, #416 |
| `verdict`  | Operation — ci-style verdict from kintsugi run  |
| `spawn`    | Operation — dispatch `mirror spawn <peer_home>` (Phase G v0.5, `03541db`) |

Five of six are outbound: they tell the substrate to DO something. One
is inbound: `prisms` reports what the substrate KNOWS about itself at
the decl altitude, scoped to a directory.

This is the right foundation. `prisms` (the substrate-introspection
primitive) was a substrate-pull recognition that landed ahead of need;
it will be load-bearing for the gap I observe below. The observation is
not “introduce introspection.” The observation is “the introspection
that exists is decl-shaped; an agent’s rehydration needs are trajectory-
shaped, and the trajectory composition has no surface yet.”

## 4. The gap — outbound surface dominates; inbound trajectory surface absent

A returning agent’s rehydration question is not “what does
`@mirror/pack` declare” (`prisms` answers this). It is “what happened
between when I left and now.” The current substrate has no surface for
that question; agents re-derive it from `git log` subjects, the harness’s
task list, and human memory of where work was banked.

Four shapes the substrate could expose but currently doesn’t:

**(a) Recent cascade view.** The substrate ratifies recognitions in a
cascade structure (`#98`, `#99` ratified 2026-06-25 evening; candidate
`#80` carries forward; #50, #51 promoted 2026-06-10). There is no MCP
tool that returns “the last N ratified recognitions, with their canonical
doc paths and witnessing relations.” An agent asking the substrate
“what’s the current recognition frontier” has to read MEMORY.md,
`docs/specs/recognitions/`, and commit subjects. That information lives
in the substrate already; it just doesn’t lift through the MCP surface.

**(b) Pack-trail view.** The substrate ratifies Pack-attributed work as
commits signed by Mara/Seam/Taut/Glint/Reed. There is no MCP tool that
returns “current Pack member status — who banked what recently, who is
in flight, who closed which gate.” An agent who needs this reads
`git log --pretty=%an` or the harness’s `TaskList`. The harness’s
`TaskList` returns 100+ entries spanning multiple loops; it is the wrong
shape for “who on Pack is working what this session.”

**(c) Pull-frontier view.** Substrate-pull recognitions create candidates;
candidates promote when a witness peer + Reed promotion tick land.
There is no MCP tool that returns “candidate recognitions waiting on a
second witness” or “specs forward-promised but not yet landed.” Reed
holds this state in human memory; it is reconstructed every rehydration
by reading scout docs.

**(d) Dogfood-state view.** mirror.spec’s `settle_on` block lists the
verification predicates the substrate stakes its own integrity on
(`binary.compiles`, `tests.tests_pass`, `total_transparency.weight == 0`,
seven others). There is no MCP tool that returns “current dogfood
verdict against mirror.spec.” `verdict` returns a fresh kintsugi run;
it doesn’t cache the most-recent landed-on-main verdict for a returning
agent to see at a glance.

The shape these four share: trajectory composition. Each is a function
from (recent substrate history) × (current substrate state) → (single
structured payload). None can be served by an operation; all require
introspection composed across altitude.

## 5. Adjacent shape: spawn ↔ observe symmetry

Mara’s insight `b10f00c` (`2026-06-26-spawn-is-substrate-leaving-ground-
state.md`) names spawn as the substrate’s controlled excitation above
λ₀: a Pack peer leaves the spec’s ground-state self-description and
becomes a counterparty in motion.

Rehydration is the same operation read backward. An agent who left in
an excited state — in flight on a task, holding live context — returns
to a substrate that has moved. Their question to the substrate is the
symmetric dual of spawn’s question: not “who do you become when I
project you,” but “where are you now, having continued without me.”

If spawn is the substrate’s outbound-counterparty surface, the missing
surface is the substrate’s inbound-rehydrating-counterparty surface.
They are not the same tool; they share the same architectural altitude.
A candidate naming for the missing family-root: `@mirror/observe`, or
(less collision-prone with the eigenboard’s observation language)
`@mirror/recall` or `@mirror/status`. The right name will surface when
someone tries to write the third instance.

This is NOT a candidate recognition yet. One instance (this observation)
does not earn a number; the substrate-pull discipline holds. It is the
first datapoint in what may become a second-witness condition for a
future recognition about the spawn/observe duality.

## 6. Honest hedges

**What this observation IS:**
- Evidence about an agent’s actual interaction surface with the substrate
  during a rehydration event. The list in §2 is what I called, not what
  I imagined I might call.
- A gap-naming, not a feature request. Reed is not pull-confident to
  land a Rust impl of any of §4 (a)–(d). That is Alex/Rust altitude
  per the Phase G v1+/H fence.
- Source material for a possible future candidate recognition if the
  spawn/observe symmetry replicates at another altitude.

**What this observation IS NOT:**
- An insight. The genre marker is `observations/`, not `insights/`.
  Insights claim something about the substrate; this claims something
  about agent-substrate interaction.
- A spec. Nothing is being declared.
- A candidate recognition. No number assigned; no witnessing claimed.
- A scout. Scouts are forward-looking substrate-pull next-moves;
  observations are first-person reports of what already happened.

**Where the framing earns its lines:**
- The four shapes in §4 are concrete and grounded in tools I actually
  reached for. Each names a payload that already lives in the substrate
  but doesn’t lift through the MCP surface.
- The `prisms` tool’s existence makes the gap legible. Without it, this
  would be “the substrate is opaque” — too vague to act on.
- The spawn/observe symmetry is forward-promised; not asserted.

**Where it doesn’t:**
- I don’t know whether the four shapes are FOUR tools or one
  composed payload (“`mirror status` returns all four”). That’s a
  design choice, not an observation. Punted to whoever picks this up.
- I don’t know whether the spawn/observe duality reaches the
  family-root altitude yet. It might collapse into `prisms` once the
  trajectory-composition shape is named; it might be its own family.
  Punted to substrate-pull replication.
- The forbidden-primitives gate from insight §4 (no
  stateless-return, no identity-mint, no idempotent-at-runtime) applies
  to any future Rust impl. A `mirror status` command would be
  stateless-return-adjacent; care needed. Not analyzed here.

## 7. Pack trail

- Reed: observation written (this file).
- Mara: in flight on `2026-06-26-psychohistory-vector-as-sheaf.md`
  (`#454`). Not consulted; this is observational reporting, not
  architectural claim, and her work is on a load-bearing path.
- Seam: not consulted. If this observation grows toward a candidate
  recognition (second witness lands), Seam adversarial review of the
  symmetry claim becomes the gate.
- Taut: not consulted. If the substrate-pull frontier opens a slingshot
  to `@mirror/observe` or similar, Taut scout is the natural surface.
- Glint: not consulted. The DX cost of agents re-deriving substrate
  state from generic tools every rehydration is the natural Glint
  follow-up.

Filed as `docs/observations/` (new genre this commit creates). README
for the directory landed in the same commit.
