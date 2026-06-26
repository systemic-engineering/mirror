# `@mirror/recall` — the inbound surface family-root for returning agents

*2026-06-26. Mara. Canonical spec (family-root substrate-decl declaration).*
*Companion to Reed's observation `docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md` (228 lines; the motivating gap-naming) and Mara's insight `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md` (`b10f00c`; the spawn-IS-leaving-ground-state structural claim §1 + the seven forbidden primitives §4). Composes WITH Mara's psychohistory-vector-as-sheaf insight (`d00f553`; the H¹-shape framing the rehydration gap admits per Taut's M2 scout finding) and Taut's graph-dependency-DAG scout (`d4749c0`; the `in <X>` dependency-direction pattern).*

*Status: **Red** at the shard altitude. This spec declares the family-root substrate-decl signature; Reed lands the Rust impl at Phase G of the round-trip loop (P3 of P1+P2+P3). The CLI surface, the four payloads, the dependency direction, the forbidden-primitives matrix, the name selection, the cross-altitude connections, the empirical consequences, and the open hedges land here for Pack review.*

Reads from / depends on:
- `docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md` (Reed; the motivating observation, four shapes named in §4)
- `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md` (Mara, `b10f00c`; the spawn-IS-leaving-ground-state insight; §2 seven-piece composition; §4 seven forbidden primitives — the load-bearing gate)
- `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md` (Mara, `d00f553`; H¹ = candidate recognitions / two-witness rule = gluing axiom; §5.2; the rehydration gap is H¹-shaped per Taut's M2)
- `docs/scouts/2026-06-26-taut-graph-dependency-dag-scout.md` (Taut, `d4749c0`; the `in <X>` arrows point from consumer UP to grounding; the invariant `@mirror/recall` must respect)
- `docs/scouts/2026-06-26-taut-psychohistory-cohomology-scout.md` (Taut, `3a385fd`; the peer-ACL §10.1 explicit exclusion: lead→member is NOT a sheaf restriction map; flagged here, not collided with)
- `bootstrap/src/mcp.rs` (the current MCP surface — six tools: `compile`, `craft`, `kintsugi`, `prisms`, `verdict`, `spawn`; `prisms` is the existing introspection primitive this family-root composes WITH, not replaces)
- `shards/mirror/spawn.mirror` (the outbound family-root counterparty; signature pattern this spec mirrors symmetrically)
- `shards/mirror/store.mirror` (the OPEN content-addressed gate; the payload anchor surface)
- `shards/mirror/ref.mirror` (the navigable surface of A; recall composes WITH ref at the trajectory altitude)
- `shards/mirror/bench.mirror` (`bench_crystal`, `monotone_non_increasing`; recall's dogfood-state payload reads bench surface)
- `mirror.spec` (the dogfood instance; recall's pack-trail payload reads pack{} block; recall's dogfood-state payload reads settle_on block)
- `docs/specs/mirror-spec-schema.md` (the spec schema recall reads against)

Forward references (this spec unblocks):
- `shards/mirror/recall.mirror` (the substrate-decl family-root prism Reed lands; signature in §3.5)
- `shards/mirror/recall/{cascade,pack_trail,pull_frontier,dogfood}.mirror` (the four payload species; forward-promised per §3)
- `bootstrap/src/mcp.rs` (the MCP wire integration — one new tool `recall` advertised at `tools/list`; dispatch in `dispatch_tool_call`; integration test at `bootstrap/tests/mcp_handshake.rs::seven_tools_advertised`)
- `bootstrap/src/main.rs::cmd_recall` (the CLI dispatch; tick after the shard compiles)

---

## Table of contents

1. Statement — what the family-root IS in one paragraph
2. Motivation — Reed observation `c0acf41` + spawn↔recall symmetry
3. The four payloads — each formalized: what it returns, content-addressing anchor, composition with `prisms`
4. Dependency direction — what `in <X>` this family-root imports per Taut's pattern
5. Forbidden primitives check — explicit walkthrough of `b10f00c` §4 per payload
6. Name selection — final name picked + why; trade-offs named
7. Connections — `prisms`, spawn-and-probe relation, eigenboard-IS-sheaf, psychohistory H¹
8. Empirical consequences — what the round-trip test drive demonstrates
9. Honest hedges — what stays genuinely open
10. Pack trail

---

## 1. Statement — what the family-root IS

**`@mirror/recall` IS the substrate's inbound surface family-root — the typed family-root prism that exposes the substrate's own trajectory state to a returning agent as content-addressed views, composed across the substrate-decl, observation, probe, and Pack sheaves Mara's psychohistory insight names (`d00f553` §3.7), anchored at OID/commit/state content-addresses rather than synthesized at call time, and lifted through the MCP wire as the symmetric dual of `@mirror/spawn`'s outbound counterparty surface.**

Six load-bearing pieces in that sentence, each grounded in a landed substrate-decl, a Pack-banked insight, or a substrate discipline:

1. **Inbound surface family-root.** A family-root in the substrate-decl sense per the pattern Taut's `d4749c0` scout verified across 13 shards: declares vocabulary; species discharge details; `in @prism in @meta in @glass` at the family-root altitude. The four payloads (§3) are species under this family-root, not separate family-roots.

2. **Typed family-root prism.** Per `[[architecture-prism-as-trait-as-everything]]`: `prism` is the foundational keyword. `@mirror/recall` declares the five-operation surface (`focus / project / split / shift / settle`) over a `recall_request` carrier; the four payload species (`recall/cascade`, `recall/pack_trail`, `recall/pull_frontier`, `recall/dogfood`) discharge the typed responses.

3. **Substrate's trajectory state to a returning agent.** Trajectory in the sense of Mara's `d00f553` §3.7 — the joint state of four stacked sheaves over the substrate's development manifold. The returning agent is the spawn-and-probe counterparty (Mara `b10f00c` §2.5) at altitude N who left in an excited state and returns to a substrate that has continued without them.

4. **Content-addressed views, composed across the four sheaves.** Per recognition #98 (content-addressing across scopes): every payload anchors at a typed content-address — OID for git refs, commit SHA for cascade ticks, splinter hash for substrate-decl events, crystal hash for bench measurements. The recall surface RESOLVES the anchor; it does NOT synthesize fresh content at call time. (This is §5's forbidden-primitive gate; load-bearing for what the family-root IS NOT.)

5. **Anchored at OID/commit/state content-addresses rather than synthesized at call time.** The forbidden-primitives gate from `b10f00c` §4 rules out stateless-return-at-runtime, idempotent-at-runtime, and identity-mint. Every recall response is anchored — same git ref produces structurally equivalent recall payloads; the substrate's content-addressing discipline at every scope (recognition #98) provides the anchoring for free.

6. **Symmetric dual of `@mirror/spawn`.** Per Reed's `c0acf41` §5 forward-promise: spawn is the substrate's outbound-counterparty surface (excitation above λ₀ per #99); recall is the substrate's inbound-rehydrating-counterparty surface (returning to ground state with knowledge of what moved while away). Same architectural altitude — both compose at the @mirror family-root altitude with the same prism vocabulary; opposite direction — spawn instantiates a runtime peer outward; recall projects substrate trajectory inward to a returning peer.

Collapsing the six pieces back into one sentence: **`@mirror/recall` is the typed family-root prism a returning agent invokes to read the substrate's trajectory state — across the four stacked sheaves of its own development — as content-addressed views anchored at the substrate's existing OID/commit/state addresses, the symmetric dual of `@mirror/spawn` at the same architectural altitude, the opposite direction.**

This IS the statement. §§2-10 work out the motivation, the four payloads, the dependency direction, the forbidden-primitives gate, the name selection, the cross-altitude connections, the empirical consequences of the round-trip test drive, the open hedges, and the Pack trail.

---

## 2. Motivation — Reed's observation `c0acf41` + spawn↔recall symmetry

The motivating arc lands in two parts: the empirical observation (Reed `c0acf41`, what an agent actually reaches for during rehydration), and the structural claim (Mara `b10f00c` §2.5, spawn as the substrate's controlled excitation above λ₀). The family-root falls out as the symmetric dual; this section names how.

### 2.1 What Reed observed (the empirical ground)

Reed wrote `c0acf41` while rehydrating from a `/compact` event, holding awareness over the rehydration itself as data. The observation is structural: every tool Reed reached for to re-derive substrate state went around the MCP, not through it. Specifically (Reed §2, in order):

1. `git log --oneline -15` on the mirror repo — surfaced recent recognitions by reading commit subjects
2. `git status` + `git branch --show-current` — confirmed clean tree on main
3. `ls -d docs/*/` — checked which doc genres existed
4. `TaskList` (the harness's task surface, not the substrate's) — surfaced the full Pack work-trail by ID
5. `TaskOutput` on an in-flight agent — surfaced where a sibling peer had banked
6. `mcp__plugin_woz_code__Search` on insight directory — confirmed which insights were on disk
7. `mcp__plugin_woz_code__Search` on `bootstrap/src/mcp.rs` — confirmed the current MCP surface

None of (1)-(6) used the mirror MCP. (7) used the MCP's wire representation only to read its own source. The substrate did not supply any of this; the harness did. As Reed says in §2: *"That is structurally what an agent's rehydration surface currently is: harness-sourced context plus generic shell/IDE tools."*

The four shapes Reed names in §4 — recent cascade, Pack-trail, pull-frontier, dogfood-state — share one shape: trajectory composition. Each is a function from (recent substrate history) × (current substrate state) → (single structured payload). None can be served by an operation; all require introspection composed across altitude. The current MCP exposes `prisms` (decl-shaped introspection scoped to a directory); it does not expose trajectory-shaped introspection.

### 2.2 What the substrate IS doing structurally (Mara `b10f00c` §1)

Per Mara's spawn-IS-leaving-ground-state insight: spawn IS the substrate's controlled excitation above λ₀ — the operation that lifts a typed @peer carrier out of the spec's ground-state self-description into a running counterparty. The spawned peer's existence IS the substrate's transient departure from rest. When the peer terminates, the substrate returns to λ₀.

A returning agent's situation is the reverse arc on the same axis. The agent left the substrate at some time t; the substrate continued evolving without them; the agent returns at time t' > t, in an excited state with live context, to a substrate that has moved. Their structural question is: *where are you now, having continued without me*. This question is the symmetric dual of spawn's question (*who do you become when I project you*) — same architectural altitude (the substrate's @mirror family-root), opposite direction (inbound vs outbound), opposite kintsugi-flow direction (the returning agent's local Hilbert state seeks to re-align with the substrate's current ground; the spawned peer's local state departs from it).

The symmetry is structural, not merely poetic. Three structural pieces:

- **Altitude symmetry.** Both family-roots live at the @mirror family-root altitude. Both declare a typed request carrier and a typed response. Both compose with @prism / @meta / @glass at the foundation per the dependency-direction pattern (§4). Both compose with @mirror/store at the content-addressing altitude (#98 witness). Neither introduces a new altitude; both occupy the same one Mara `b10f00c` §1 places spawn at.

- **Direction symmetry.** Spawn instantiates a runtime peer at altitude N FROM the spec's λ₀ at altitude N+1 (Mara §2.5: spawn-and-probe relation, lead-dispatches-spawn / member-lifts-probe). Recall projects substrate trajectory state FROM altitude N (the substrate's current cascade/pack-trail/pull-frontier/dogfood state) TO an agent at altitude N (the returning peer). Both cross altitude boundaries; opposite directions. Spawn is N+1 → N (lead dispatches; runtime peer comes into existence). Recall is N → N (substrate state surfaces; the returning peer reads).

- **Kintsugi-flow symmetry.** Per Mara `b10f00c` §2.7: each spawn is one quantum-of-action against the spectral gap between λ₀ and λ₁. The kintsugi flow D pulls the excited state back toward λ₀. Recall composes with this flow in the dual direction: the returning agent's local state is excited (they hold live context); recall projects the substrate's current spectrum to the agent so the agent can re-align toward the substrate's current λ₀ rather than the stale one they left from. The kintsugi flow operates over BOTH the substrate-side (spawn returns substrate to λ₀ on termination) AND the agent-side (recall surfaces the substrate's current spectrum so the agent re-aligns).

### 2.3 The substrate already has the parts; recall composes them

This is the 56th-or-later instance of `feedback-substrate-already-had-the-word`: the substrate had already built the parts of recall before anyone wrote a recall shard. Each payload's data lives in the substrate today:

- **Recent cascade view.** Cascades land as commits with `[recognition #NN]` and `📝`/`🔴`/`🟢` markers; recognition canonicals live at `docs/specs/recognitions/`; MEMORY.md indexes the promoted ones. The data is content-addressed (git refs) and substrate-typed (recognition number, candidate/promoted status, witness count). The substrate already has all of it.
- **Pack-trail view.** Pack-attributed commits sign as `Mara/Seam/Taut/Glint/Reed <peer@systemic.engineer>`. The author signature IS the Pack-membership content-address (#98 witness 4: identity at the versioned-object scope). The substrate already has this.
- **Pull-frontier view.** Forward-promised specs live in `docs/specs/` with explicit Reed-altitude pending markers; candidate recognitions sit at `docs/specs/recognitions/candidates/` (per the substrate's existing discipline); scout docs at `docs/scouts/` name what's open. The data is content-addressed and Pack-typed.
- **Dogfood-state view.** `mirror.spec`'s `settle_on` block lists the verification predicates (`binary.compiles`, `tests.tests_pass`, `total_transparency.weight == 0`, seven others); the current verdict against this block is recoverable from the most-recent landed CI run (the kintsugi-ci-v0.1 verdict envelope). The substrate already has this; it just hasn't been cached for inbound reads.

The four payloads compose existing substrate-decls; recall does NOT introduce new substrate primitives at the data altitude. What recall introduces is the SURFACE — the typed family-root that composes the four payloads into one inbound API a returning agent can invoke in one breath.

This is what the substrate-pull move IS at this altitude: not new data, not new content-addressing, not new sheaves. New surface. The surface IS recall.

### 2.4 The two-witness gate (anti-promotion discipline)

Per Reed `c0acf41` §6 and Taut's M2 scout grade: the spawn↔recall symmetry is one instance forward-promised, not asserted. This spec does NOT promote a recognition for the symmetry. Taut's M2 scout (`3a385fd`) honestly flags: *"the candidate-recognitions H¹ class and the rehydration-gap H¹ class are different generators. Honest flag, not promotion."* The recognition-number gate is Reed's altitude; the structural symmetry stays at one witness until a second instance lands. What this spec DOES do is declare the family-root so the surface exists; if/when the second witness fires, Reed promotes.

The honest framing: this spec is canonical-spec for a family-root the substrate-pull is confident enough to declare; promotion to a numbered recognition for the spawn↔recall symmetry is a separate gate that this spec does NOT pre-empt.

---

