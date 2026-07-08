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
- `shards/mirror/peer/beam.mirror` (the outbound family-root counterparty; signature pattern this spec mirrors symmetrically; formerly `shards/mirror/spawn.mirror`; renamed 2026-07-08 Tick 2 `9de2226`)
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

**`@mirror/recall` IS the substrate's inbound surface family-root — the typed family-root prism that exposes the substrate's own trajectory state to a returning agent as content-addressed views, composed across the substrate-decl, observation, probe, and Pack sheaves Mara's psychohistory insight names (`d00f553` §3.7), anchored at OID/commit/state content-addresses rather than synthesized at call time, and lifted through the MCP wire as the symmetric dual of `@mirror/peer/beam`'s outbound counterparty surface (formerly `@mirror/spawn`; renamed 2026-07-08 Tick 2 `9de2226`).**

Six load-bearing pieces in that sentence, each grounded in a landed substrate-decl, a Pack-banked insight, or a substrate discipline:

1. **Inbound surface family-root.** A family-root in the substrate-decl sense per the pattern Taut's `d4749c0` scout verified across 13 shards: declares vocabulary; species discharge details; `in @prism in @meta in @glass` at the family-root altitude. The four payloads (§3) are species under this family-root, not separate family-roots.

2. **Typed family-root prism.** Per `[[architecture-prism-as-trait-as-everything]]`: `prism` is the foundational keyword. `@mirror/recall` declares the five-operation surface (`focus / project / split / shift / settle`) over a `recall_request` carrier; the four payload species (`recall/cascade`, `recall/pack_trail`, `recall/pull_frontier`, `recall/dogfood`) discharge the typed responses.

3. **Substrate's trajectory state to a returning agent.** Trajectory in the sense of Mara's `d00f553` §3.7 — the joint state of four stacked sheaves over the substrate's development manifold. The returning agent is the spawn-and-probe counterparty (Mara `b10f00c` §2.5) at altitude N who left in an excited state and returns to a substrate that has continued without them.

4. **Content-addressed views, composed across the four sheaves.** Per recognition #98 (content-addressing across scopes): every payload anchors at a typed content-address — OID for git refs, commit SHA for cascade ticks, splinter hash for substrate-decl events, crystal hash for bench measurements. The recall surface RESOLVES the anchor; it does NOT synthesize fresh content at call time. (This is §5's forbidden-primitive gate; load-bearing for what the family-root IS NOT.)

5. **Anchored at OID/commit/state content-addresses rather than synthesized at call time.** The forbidden-primitives gate from `b10f00c` §4 rules out stateless-return-at-runtime, idempotent-at-runtime, and identity-mint. Every recall response is anchored — same git ref produces structurally equivalent recall payloads; the substrate's content-addressing discipline at every scope (recognition #98) provides the anchoring for free.

6. **Symmetric dual of `@mirror/peer/beam`** (formerly `@mirror/spawn`; renamed 2026-07-08 Tick 2 `9de2226`). Per Reed's `c0acf41` §5 forward-promise: the outbound family-root is the substrate's outbound-counterparty surface (excitation above λ₀ per #99); recall is the substrate's inbound-rehydrating-counterparty surface (returning to ground state with knowledge of what moved while away). Same architectural altitude — both compose at the @mirror family-root altitude with the same prism vocabulary; opposite direction — the outbound family-root instantiates a runtime peer outward; recall projects substrate trajectory inward to a returning peer.

Collapsing the six pieces back into one sentence: **`@mirror/recall` is the typed family-root prism a returning agent invokes to read the substrate's trajectory state — across the four stacked sheaves of its own development — as content-addressed views anchored at the substrate's existing OID/commit/state addresses, the symmetric dual of `@mirror/peer/beam` (formerly `@mirror/spawn`) at the same architectural altitude, the opposite direction.**

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

## 3. The four payloads — formalized

Each payload is a species under `@mirror/recall`. The species pattern matches `@mirror/{cli, mosaic, store, bench, ref, garden, spawn}` — family-root declares vocabulary; species discharge details; species shards live at `shards/mirror/recall/<species>.mirror`. This section formalizes each payload along three dimensions:

- **Shape.** What the payload returns (the carrier type at substrate altitude).
- **Anchor.** The content-address that makes the payload content-addressed rather than synthesized at call time (the §5 forbidden-primitives gate).
- **Composition with `prisms`.** Recall does NOT replace the existing `prisms` introspection primitive (decl-shaped, scoped to a directory); the four payloads compose WITH `prisms`. Each payload names how.

A returning agent typically invokes recall with a single subcommand naming which payload (or invokes the family-root for the joint payload). The CLI surface mirrors the existing `mirror ref` ergonomic-shortcut pattern from `mirror-ref-spec.md` §2.4:

```sh
mirror recall                              # joint payload (all four; default)
mirror recall cascade [--since=<commit>]   # recent ratified recognitions + canonical doc paths
mirror recall pack-trail [--since=<commit>] # who banked what; who is in flight; who closed which gate
mirror recall pull-frontier                # candidates awaiting witnesses + forward-promised specs
mirror recall dogfood                      # current verdict against mirror.spec's settle_on block
```

§§3.1-3.4 formalize each payload. §3.5 names the family-root signature sketch.

### 3.1 `recall/cascade` — the recent cascade view

**Shape.** A typed sequence of `cascade_tick` records covering the last N ratified recognitions, where each record carries:

```
cascade_tick = {
  recognition_number:    int,
  status:                | candidate | promoted,
  canonical_doc:         ref,           # path to docs/specs/recognitions/<...>.md or MEMORY entry
  witnessing_relations:  [ref],         # refs to ancestor recognitions / instances
  promotion_commit:      content_address, # git SHA of the commit that ratified
  pack_attribution:      peer,          # who banked the recognition
  altitude:              ref,           # substrate altitude the recognition operates at
}
```

`N` is bounded by the `--since=<commit>` flag (default: last 10 ticks). The sequence is ordered by `promotion_commit` (chronological by commit ancestry, NOT by recognition number — recognitions can promote out of order). Forward-promised recognitions appear as `candidate` status with the canonical_doc pointing at the slot (per the substrate's existing forward-promise discipline).

**Anchor.** Each record anchors at `promotion_commit` (git SHA). The cascade view is content-addressed at the cascade level: same set of commits → same payload bytes. The `canonical_doc` ref is content-addressed via the OID at the resolved commit. This makes `recall cascade` idempotent at the IDENTITY altitude (per `b10f00c` §4.3 / §4.5 spawn parallel): same git ref produces structurally equivalent payloads; the runtime invocation may produce two distinct call traces but the payload bytes are equivalent.

**Composition with `prisms`.** `prisms` returns prism declarations from a directory walk; `recall cascade` returns recognition events from a git ancestry walk. The two compose at the *substrate-altitude* layer: a returning agent who reads `recall cascade` to see that recognition #99 promoted, then invokes `prisms shards/` to see which prisms #99's canonical (`shards/mirror/spec.mirror`) declares, then invokes `prisms docs/specs/` (if applicable) to read which actions discharge. Recall composes WITH prisms; it does not replace.

Per Mara `b10f00c` §2.5 spawn-and-probe relation read in the recall direction: `recall cascade` is the substrate's spectral-Tomm response to the agent's probe `[D_substrate, "what happened"]`. The probe returns spectral data the agent's next action can consume.

### 3.2 `recall/pack_trail` — the Pack-trail view

**Shape.** A typed sequence of `pack_tick` records covering recent Pack-attributed activity:

```
pack_tick = {
  peer:                 peer,
  commit:               content_address,
  phase_marker:         | red | green | refactor | docs | tooling | other,
  banked_at:            ref,            # path of the file the commit primarily touched
  in_flight:            bool,           # true if peer has an active session/agent
  gate_closed:          option(ref),    # ref to a spec/recognition this commit closed
  altitude:             | substrate-decl | observation | probe | pack | runtime,
}
```

The sequence is ordered by `commit` ancestry. The `peer` field resolves through the spec's `pack{}` block (`mirror.spec` dogfood at `8107caf`): valid peers are the spec's lead + members. Commits authored by peers NOT in the spec's pack{} block are excluded — recall's pack-trail surfaces the SPEC'S pack, not the underlying git author list. This is consistent with Mara `b10f00c` §2.3: spawn is pack-authorized; recall is symmetrically pack-scoped.

The `in_flight` field is the only one that admits a non-content-addressed read; §3.2.1 names how it stays anchored.

**Anchor.** Each pack_tick record anchors at `commit` (git SHA). The `peer` resolves through the spec's pack{} block (content-addressed at the spec's OID). The `gate_closed` ref resolves to a doc or spec at its own OID. The full pack-trail sequence is content-addressed at the (spec_oid, since_commit, head_commit) triple.

**3.2.1 The `in_flight` field anchoring caveat.** The substrate does not natively track in-flight session state at the spec altitude; sessions are a harness concept (Claude Code's session-id is not substrate-decl'd). To keep `in_flight` content-addressed, the spec specifies one of two discharge paths, with the choice deferred to Phase G (Reed altitude):

- **Discharge A (substrate-side, preferred):** `in_flight` resolves through `@spectral/supervisor`'s active-children registry. A peer is in_flight iff `@spectral/supervisor.list_children()` includes a runtime peer with home matching the peer's ACL-declared home. This is content-addressed at the supervisor's registry hash at recall-time, which is read-anchored even though the underlying state mutates.
- **Discharge B (deferred, with structural flag):** `in_flight` returns `unknown` for peers whose runtime state the substrate cannot read content-addressed. The discharge defers the harness-vs-substrate boundary to a future scout.

The honest framing: Discharge A is substrate-pull-correct per `b10f00c` §2.5 spawn-and-probe composition; Discharge B is a documented degradation if Phase G hits a blocker. Either way, the field's substrate-decl shape includes the `option`/`unknown` carrier so the discharge can land without surface change.

**Composition with `prisms`.** `prisms` returns decl-shaped structure; `recall pack-trail` returns peer-attributed-trajectory shape. Composition: an agent reads `recall pack-trail` to see Mara banked at `shards/mirror/recall.mirror`, then invokes `prisms shards/mirror/recall/` to see which prism species the recall family-root declares. Pack-trail surfaces who-and-when; prisms surfaces what-decl'd.

**Honest hedge.** The pack-trail payload composes against `mirror.spec`'s pack{} block as the authority for who-counts-as-Pack. This means the payload is spec-scoped — Pack composition is per-spec, NOT global across all repos. A returning agent invoking recall in the `mirror/` repo sees mirror.spec's pack; the same agent invoking recall in another repo sees that repo's pack. This is correct per peer-ACL §6.2 self-naming; it earns its lines via the substrate's existing pack-coherent discipline. No collision with peer-ACL §10.1 (lead→member is NOT a sheaf restriction map — recall doesn't claim it is; the pack_tick records are antichain entries flagged with `peer` field, not delegation-chain projections).

### 3.3 `recall/pull_frontier` — the pull-frontier view

**Shape.** A typed sequence of `pull_frontier_item` records covering candidate recognitions awaiting witnesses + forward-promised specs:

```
pull_frontier_item = {
  kind:                  | candidate_recognition | forward_promised_spec | scout_open | seam_flag,
  identifier:            ref,            # e.g., recognition #53, spec path, scout-flag id
  canonical_doc:         ref,            # primary doc this item lives in
  witness_count:         int,            # current witnesses (1 = one-witness; 2 = ready-to-promote)
  witnesses_needed:      int,            # gate threshold (typically 2 for recognitions)
  promoting_peer:        option(peer),   # who would land the promotion (typically Reed for recognitions)
  surfaced_at:           content_address, # commit where the candidate first surfaced
  related_recognitions:  [int],          # ancestor / sibling recognition numbers
}
```

The sequence is ordered by `witnesses_needed - witness_count` ascending (ready-to-promote first), then by `surfaced_at` ascending (oldest unwitnessed first). This is the substrate's `H¹(M, F)` per Mara `d00f553` §5.2 made operationally readable: candidates that pairwise glue but don't yet extend globally.

**Anchor.** Each record anchors at `surfaced_at` (git SHA of the commit that introduced the candidate). The `canonical_doc` ref anchors at its OID at that commit. The `witness_count` is derived from the substrate's existing two-witness discipline by walking the candidate's canonical_doc's `witnessing_relations` block (per the substrate's existing recognition format). The sequence is content-addressed at the (head_commit, candidate-doc-set-oid) tuple.

**3.3.1 The `witness_count` derivation.** The substrate has a discipline (per `[[feedback-composition-claims-need-empirical-test]]`) of recording witnessing relations in recognition canonical docs and MEMORY.md entries. The derivation walks:

1. The recognition's canonical doc (e.g., `docs/specs/recognitions/<...>.md` or the candidate's primary doc)
2. Counts explicit witness statements (substrate convention: each instance is a `## Witness <N>` section or equivalent)
3. Cross-checks against MEMORY.md entries' `Recognition #NN PROMOTED` / `Recognition #NN CANDIDATE` markers

The count is bounded — recall does NOT search the entire substrate for implicit witnesses; it surfaces what the substrate has explicitly recorded. This is consistent with `feedback-substrate-already-had-the-word`: the substrate's existing two-witness discipline records witnesses explicitly; recall reads what's recorded.

**Composition with `prisms`.** A returning agent reads `recall pull-frontier` to see that recognition #53 family is at witness-count 1, then invokes `prisms shards/` to see which prism declarations match the candidate's expected discharge shape. Pull-frontier surfaces what's-open; prisms surfaces what-declared. The composition is what Taut's scouting altitude already does manually; recall makes it readable in one breath.

**Honest hedge.** The `witnesses_needed` threshold is per-recognition-class. Most numbered recognitions follow the two-witness rule; some forward-promised specs follow different gates (e.g., Seam adversarial review, Pack ratification). The substrate's existing discipline records the relevant gate in the canonical doc; recall reads it. If a doc lacks an explicit gate, recall reports `witnesses_needed: unknown` rather than defaulting — this is the substrate's `option` discipline applied to the threshold.

### 3.4 `recall/dogfood` — the dogfood-state view

**Shape.** A typed `dogfood_verdict` record covering the substrate's current verdict against `mirror.spec`'s `settle_on` block:

```
dogfood_verdict = {
  spec_oid:              content_address, # OID of the mirror.spec at the recall-time commit
  settle_on_predicates:  [predicate],     # the list of predicates from settle_on
  predicate_verdicts:    [predicate_verdict], # per-predicate state
  aggregate:             | success | partial(p: f64) | failure,
  most_recent_landed_at: content_address, # commit at which the verdict was computed
  cache_freshness:       | fresh | stale(ticks: int),
}

predicate_verdict = {
  predicate:             ref,             # e.g., "binary.compiles"
  status:                | success | partial(p: f64) | failure | unknown,
  evidence:              ref,             # content-addressed evidence (CI log OID, kintsugi-ci envelope OID)
}
```

The settle_on predicates for `mirror.spec` (per `mirror.spec` at `8107caf`) are: `binary.compiles`, `binary.tests_pass`, `fmt.formats`, `lint.lints`, `tests.tests_pass`, `audit.advisories_clean`, `action.validates`, `release.signs`, `total_transparency.weight == 0`. Recall does NOT re-run these; it surfaces the most-recent landed verdict. This is the symmetric dual of `verdict` (the existing MCP tool, which runs fresh): `verdict` is forward-direction (run a settle now); `recall dogfood` is backward-direction (read the most-recent landed verdict).

**Anchor.** The verdict anchors at (spec_oid, most_recent_landed_at) — same spec at same commit → same payload. The per-predicate evidence anchors at the evidence OID (CI log content-address, kintsugi-ci envelope hash, etc.). The `cache_freshness` field surfaces whether the cached verdict's commit-ancestry includes the head-of-main; if not, the cache is `stale(ticks: N)` where N is the count of commits between the cached and the current head.

**3.4.1 The cache layer.** The substrate does NOT today cache settle_on verdicts persistently for inbound reads; this spec specifies that Phase G adds a cache layer. The cache lives at `~/.mirror/recall/dogfood-cache.jsonl` (one record per landed verdict, content-addressed), populated as a side-effect of `mirror kintsugi --ci` runs that land on main. The cache layer is OUT OF SCOPE for the substrate-decl shard (it's an `@io` boundary concern); the shard declares `dogfood_verdict`'s shape; the cache is the realisation choice.

Honest hedge: if the cache hasn't been populated (fresh clone; first invocation), `recall dogfood` returns `cache_freshness: unknown` and `aggregate: unknown`, with a note pointing the agent at `mirror verdict mirror.spec` for a fresh run. This is the same `option`/`unknown` discipline §3.3.1 uses; the substrate is honest about cache absence rather than synthesizing a verdict.

**Composition with `prisms`.** A returning agent reads `recall dogfood` to see the substrate's current verdict, then invokes `prisms shards/epistemologic/property/` to see which predicates the dogfood settle_on composes against. Dogfood surfaces verdict; prisms surfaces predicate-decl. The composition lets an agent who sees `tests.tests_pass: partial(0.13)` immediately query which predicate discharges that partial.

### 3.5 Family-root signature sketch (Reed lands at Phase G)

The substrate-decl shape for `shards/mirror/recall.mirror`, matching the family-root pattern from `shards/mirror/{spawn,ref,bench,garden}.mirror`. Reed lands; this sketch is the Mara altitude (signature, action shapes, bilateral predicates). Implementation details (cache layer, supervisor wire) are Reed's altitude per Phase G.

```mirror
in @prism
in @meta
in @glass
in @mirror/store
in @mirror/spec
in @mirror/cli
in @mirror/pack
in @mirror/bench
in @pack
in @peer
in @loop
in @reflection
in @spectral/supervisor
in @epistemologic
in @epistemologic/property
in @epistemologic/reality/time

# @mirror/recall — the substrate's inbound surface family-root for
# returning agents. Spawn's symmetric dual at the same altitude.
#
# Motivation: Reed's c0acf41 observation (rehydration gap) named four
# trajectory-shape payloads that live in the substrate but don't lift
# through the MCP surface today. This shard declares the family-root
# that lifts them.
#
# === Cross-altitude ancestry ===
#
# - #99 (mirror.spec IS λ₀; Mara canonical d0b6519): recall's dogfood
#   payload reads the spec's settle_on block; the verdict is the
#   substrate's current excited state relative to λ₀.
# - #51 (mirror as expanding Hilbert space): recall reads the
#   trajectory across the four stacked sheaves (Mara d00f553 §3.7);
#   each payload is a section over the current open neighborhood.
# - #98 (content-addressing across scopes): every payload anchors at
#   a content-address; recall does NOT synthesize fresh state.
# - Spawn (b10f00c): symmetric dual at the same architectural
#   altitude; recall is N → N (inbound substrate state to returning
#   peer); spawn is N+1 → N (lead dispatches runtime peer).
#
# === Forbidden-primitives gate (per b10f00c §4) ===
#
# Recall does NOT introduce:
# - @os/process / @os/thread     (no OS-fork in the read path)
# - identity-mint                (peers are recognized, not minted)
# - stateless-return-at-runtime  (all returns content-addressed)
# - idempotent-at-runtime        (idempotent at IDENTITY altitude, not runtime)
# - delegation-chain             (peer-ACL §10.1 explicit exclusion respected)
# - membership-side-effects      (recall is read-only at pack altitude)
# - @io/llm                      (NO LLM adapter in the read path)
#
# See docs/specs/mirror-recall.md §5 for the per-payload verification
# matrix.

prism @mirror/recall {
  focus  recall_request
  project recall_request
  split  recall_request
  shift  recall_request
  settle recall_request
}

# === The recall_request carrier ===
#
# The typed cli-surface request shape. Three fields:
#
#   payload  — which payload to surface (joint, cascade, pack_trail,
#              pull_frontier, dogfood)
#   since    — content-address constraint (read trajectory since this
#              commit; default = HEAD~N for some N per payload)
#   spec     — optional ref to a non-default mirror.spec (default =
#              the current cwd's mirror.spec)
#
# Identity contract: byte-equality on the (payload, since, spec) tuple.
type recall_request = {
  payload: payload_selector,
  since:   ref,
  spec:    option(ref),
}

type payload_selector =
  | joint
  | cascade
  | pack_trail
  | pull_frontier
  | dogfood

# === The recall action ===
#
# Lifts the cli invocation into the substrate. Returns the typed
# payload shape per the selector; the runtime carrier composes with
# the four payload species.
recall(r: recall_request, p: perturbation) -> recall_response
requires spec_resolves(r.spec, p)
       , since_content_addressed(r.since, p)
{ \ }

type recall_response =
  | joint_response(cascade_view, pack_trail_view, pull_frontier_view, dogfood_view)
  | cascade_response(cascade_view)
  | pack_trail_response(pack_trail_view)
  | pull_frontier_response(pull_frontier_view)
  | dogfood_response(dogfood_view)

# === Sub-bilateral predicates ===
#
# spec_resolves: the spec ref points to a parseable mirror.spec at the
# resolved commit (composes with @mirror/spec).
spec_resolves(s: option(ref), p: perturbation) -> verdict { \ }

# since_content_addressed: the since ref resolves to a git commit in
# the current repo (composes with @io/git).
since_content_addressed(s: ref, p: perturbation) -> verdict { \ }

# === Forward-promised — NOT landed this tick ===
#
# - The four payload species (cascade, pack_trail, pull_frontier, dogfood)
#   land as shards/mirror/recall/<species>.mirror with their own typed
#   carriers per §§3.1-3.4.
# - recall_coherent(r: recall_request, p: perturbation) -> verdict
#     Composed bilateral per recognition #53 family (per-payload coherence
#     check; when the four species land, this composes their bilaterals).
# - default_spec_resolution: when r.spec is absent, resolve to the cwd's
#   mirror.spec (composes with @mirror/cli's contextual lookup).

out @mirror/recall
out recall_request
out payload_selector
out recall_response
out recall
out spec_resolves
out since_content_addressed
```

The signature mirrors `@mirror/peer/beam`'s shape (one positional argument; contextual resolution inside the body; bilateral predicates at the `requires` clauses; forward-promised composed bilateral). This IS the symmetric-dual structural pattern at the substrate-decl altitude.

---

## 4. Dependency direction — what `in <X>` this family-root imports

Per Taut's `d4749c0` scout (`@graph family-root dependency-DAG scout`): **`in`-arrows point from consumer UP to grounding, never from grounding DOWN to consumer**. Taut verified this across 13 shards with five-instance witnessing and zero counter-instances. `@mirror/recall` MUST respect this invariant.

The §3.5 signature sketch lists the imports. This section names what each import grounds, in the direction the invariant requires.

### 4.1 Foundation layer (every family-root imports these)

```mirror
in @prism    # the foundational keyword; family-root pattern
in @meta     # carrier altitude
in @glass    # the bilateral-discharge surface
```

These are the family-root pattern from `prism.mirror`, `kintsugi.mirror`, `glass.mirror`. `@mirror/recall` consumes the family-root vocabulary; the foundation grounds it. Direction correct: consumer (recall) → grounding (prism/meta/glass).

### 4.2 Content-addressing + spec resolution layer

```mirror
in @mirror/store   # the OPEN content-addressed gate
in @mirror/spec    # the spec schema recall reads against
in @mirror/cli     # contextual resolution for the cwd's mirror.spec
```

`@mirror/store` grounds the content-addressing payloads (§3.1-§3.4 each anchor at content-addresses; the addressing itself is `@mirror/store`'s domain). `@mirror/spec` grounds the `recall_request.spec` field's resolution. `@mirror/cli` grounds the contextual lookup that resolves the default spec when `r.spec` is absent. Direction correct: recall consumes the addressing/spec/cli vocabulary; those vocabularies don't import recall.

### 4.3 Pack composition layer

```mirror
in @mirror/pack   # pack-block shape for the pack_trail payload
in @pack          # family-root @pack carrier (peer + members + bindings)
in @peer          # the peer carrier for pack_tick records
```

`@mirror/pack` grounds the pack{} block shape recall reads in §3.2. `@pack` grounds the underlying carrier and the pack-coherent discipline. `@peer` grounds the per-record peer field. Direction correct: recall consumes pack vocabulary; pack vocabulary doesn't import recall.

The symmetry with `@mirror/peer/beam`'s import block is exact: the outbound family-root imports `@mirror/pack + @pack + @peer` to dispatch outward; recall imports the same three to read inward. Both consume the same pack-altitude vocabulary; both occupy the same architectural altitude.

### 4.4 Trajectory + supervision layer

```mirror
in @mirror/bench         # bench_crystal for dogfood payload's per-predicate evidence
in @loop                 # the @loop tick altitude for cascade ordering
in @reflection           # the observation-side reads for the four sheaves
in @spectral/supervisor  # in_flight resolution via Discharge A (§3.2.1)
```

`@mirror/bench` grounds the dogfood payload's evidence-anchoring (bench_crystal is the content-addressed measurement carrier per #87). `@loop` grounds the temporal ordering for cascade ticks (per #88). `@reflection` grounds the observation-sheaf reads (per #85; reflection's `observe` is the temporal-projection primitive Mara `b10f00c` §2.5 places at altitude N+1). `@spectral/supervisor` grounds the in_flight field's Discharge A path. Direction correct: recall consumes all four; none import recall.

### 4.5 Epistemologic layer

```mirror
in @epistemologic                 # property/fracture/predicate ancestry
in @epistemologic/property        # the predicate carrier for settle_on
in @epistemologic/reality/time    # the time altitude for since-resolution
```

`@epistemologic/property` grounds the `predicate` carrier in the dogfood payload's `settle_on_predicates` field. `@epistemologic/reality/time` grounds the `since` field's time-altitude semantics. Direction correct: recall consumes the epistemologic vocabulary; epistemologic doesn't import recall.

### 4.6 What recall does NOT import — explicit boundary

`@mirror/recall` does NOT import:

| NOT imported | Why |
|---|---|
| `@fate` | @fate is the runtime substrate, not a substrate-decl prism (per Taut's Phase F anti-pattern correction 2026-06-24; same exclusion as @mirror/peer/beam). |
| `@io/llm` | Forbidden per b10f00c §4.2 (and the substrate has no @io/llm family). |
| `@io/git` (directly) | @mirror/store transitively composes with @io/git; recall consumes @mirror/store. Direct @io/git import would be a structural collision with the addressing layer's encapsulation. |
| `@os/process` / `@os/thread` | Forbidden per b10f00c §4.1; recall is a read path, not a process-fork path. |
| `@magic/contract` (directly) | The pack-altitude ACL composition lifts through @mirror/pack; direct @magic import would duplicate the audit lineage. Recall is read-only at the pack altitude; @magic/audit composes only at the @io boundary (per #57 alignment-as-boundary-mathematics), and recall does not cross that boundary in the read path. |
| Any `@psychohistory` / `@cohomology` / `@sheaf` abstract family-root | Per Taut's M3 anti-move (`3a385fd` §2): the substrate-already-had-the-word discipline at the math altitude. `@epistemologic/math/sheaf_laplacian` exists; recall does NOT introduce a duplicate abstract family-root. The Mara `d00f553` four-sheaves framing is the LANGUAGE recall reads ITSELF as a section of (§7.4); it is NOT a new family-root recall depends on. |

### 4.7 The full DAG (one-line summary)

```
@prism / @meta / @glass
    ↑
@mirror/store / @mirror/spec / @mirror/cli
    ↑
@mirror/pack / @pack / @peer
    ↑
@mirror/bench / @loop / @reflection / @spectral/supervisor
    ↑
@epistemologic / @epistemologic/property / @epistemologic/reality/time
    ↑
@mirror/recall  ← THIS FAMILY-ROOT (consumer altitude)
    ↑
(future) @mirror/recall/cascade / pack_trail / pull_frontier / dogfood  ← species
    ↑
(future) @<consumer> (e.g., a hypothetical @harness/rehydration adapter, NOT in scope)
```

Read upward: every `in` arrow points from consumer to grounding. Recall sits at consumer altitude relative to everything below; recall sits at grounding altitude relative to its own future species. The substrate's existing dependency-DAG invariant holds; no inversion.

**Verification.** Taut's `d4749c0` scout's pattern (`grep @graph shards/**/*.mirror`-style) was applied here: no shard in `shards/**/*.mirror` imports `@mirror/recall` today (slot empty per pre-spec verification). Recall is a new node in the DAG; the §4.1-§4.5 layer lists what it imports; nothing imports it yet. The first consumer will be the Phase G Rust impl + the MCP wire integration; the second consumer will be the four species shards.

---

## 5. Forbidden primitives check — the verification matrix

Per Mara `b10f00c` §4 (the load-bearing gate): the inbound surface is stateless-return-adjacent. This section walks each of the seven forbidden primitives against each of the four payloads — a 4 × 7 = 28-cell verification matrix. Rows that are obviously safe are collapsed; rows that require structural argument are unpacked.

The seven forbidden primitives (from `b10f00c` §4):

1. **`@os/process`** — recall is not a process-fork; never spawns OS processes for reads.
2. **identity-mint** — recall does not create peer IDs; identities come from the home's git ref (#98 witness 4).
3. **stateless-return-at-runtime** — recall does not synthesize fresh content not anchored in content-addressed state.
4. **idempotent-at-runtime** — recall may produce distinct call traces per invocation, but never returns different content for the same content-addressed input.
5. **delegation-chain** — recall respects peer-ACL §10.1: lead→member is NOT a sheaf restriction map and NOT a delegation chain.
6. **membership-side-effects** — recall is read-only; never adds/removes peers from the spec's pack{}.
7. **`@io/llm`** — recall does not bridge to an external LLM service.

### 5.1 The matrix

| Payload \ Forbidden | 1. @os/process | 2. identity-mint | 3. stateless-return | 4. idempotent-runtime | 5. delegation-chain | 6. membership-SE | 7. @io/llm |
|---|---|---|---|---|---|---|---|
| `cascade` | safe ✓ | safe ✓ | §5.2 | §5.3 | safe ✓ | safe ✓ | safe ✓ |
| `pack_trail` | safe ✓ | safe ✓ | §5.2 | §5.3 | §5.4 ⚠ | safe ✓ | safe ✓ |
| `pull_frontier` | safe ✓ | safe ✓ | §5.2 | §5.3 | safe ✓ | safe ✓ | safe ✓ |
| `dogfood` | §5.5 ⚠ | safe ✓ | §5.2 | §5.3 | safe ✓ | safe ✓ | safe ✓ |

Legend: `safe ✓` = the row is structurally unreachable (recall doesn't compose with the relevant carrier at all); `§X.X` = unpacked in the named subsection; `⚠` = a structural argument is required, not merely an absence.

### 5.2 Stateless-return-at-runtime (rows 3, all four payloads)

**Argument.** Every payload anchors at a content-address (§3.1-§3.4): cascade at `promotion_commit`; pack_trail at `commit + spec_oid`; pull_frontier at `surfaced_at + canonical_doc_oid`; dogfood at `spec_oid + most_recent_landed_at`. Same content-address tuple → same payload bytes. The runtime invocation is a read; it does not synthesize. The substrate's existing content-addressing discipline (#98) provides the anchor; recall consumes it.

**Where this could fail.** A payload field that reads a non-content-addressed source (e.g., `in_flight` field on pack_trail, where supervisor state mutates between calls) is the structural risk site. §3.2.1 names the discharge: Discharge A anchors at the supervisor's registry hash at recall-time (the read is anchored even though the underlying state mutates between recalls); Discharge B returns `unknown` (the substrate is honest about the non-anchorable state rather than synthesizing). Both discharges keep the payload's substrate-decl shape stateless-return-clean; neither admits an unanchored synthesis.

**Open flag.** If Phase G hits a blocker on Discharge A and Discharge B's `unknown` returns are too lossy for the round-trip test drive's empirical value, the in_flight field MAY require a §9 hedge upgrade: surface `in_flight` as a sub-prism explicitly typed at the harness boundary, with a structural-flag for Alex/Reed review. This spec does NOT pre-empt that choice; flag #1 in §9.

### 5.3 Idempotent-at-runtime (rows 4, all four payloads)

**Argument.** Recall is idempotent at the IDENTITY altitude per the §3 anchors: same content-address tuple → structurally equivalent payload. Recall is NOT idempotent at the RUNTIME altitude in the strict sense: two recall invocations at different commits/spec_oids produce different payloads, by design — that IS what surfaces the trajectory.

The forbidden primitive (per Mara `b10f00c` §4.5) is `idempotent-at-runtime` in the spawn sense: returning the existing runtime when asked to spawn an already-running peer. Recall has no analog because recall does not produce runtime state; it produces payload bytes anchored at content-addresses. The forbidden primitive's spawn-side concern (don't conflate two distinct runtime instances) maps to the recall-side concern (don't conflate two distinct content-address tuples — and recall doesn't, because the response IS keyed on the tuple).

**Net.** Recall is idempotent at identity (same content-address → same bytes); recall is non-idempotent at trajectory (different commits → different bytes); recall is structurally unable to conflate runtimes (there are no runtimes in the read path).

### 5.4 Delegation-chain (row 5, pack_trail) — peer-ACL §10.1 boundary

**The concern.** Per peer-ACL §10.1 (and Mara `b10f00c` §2.5; Taut `3a385fd` §4): the lead→member relation is **NOT** a sheaf restriction map and **NOT** a delegation chain. The morphisms in the relation are spectral-Tomm probes; members form an antichain at altitude N; the lead is the distinguished N+1 observer. Recall's pack_trail payload (§3.2) walks pack-attributed commits — could this implicitly assert a lead→member delegation reading?

**The argument.** No. Three structural pieces hold:

1. **Records are antichain entries.** Each `pack_tick` record carries a `peer` field. The records form a sequence; the sequence has temporal order (by `commit` ancestry); it does NOT have authority order. Reading the payload gives "Mara committed X, then Reed committed Y, then Glint committed Z" — temporal trajectory, not authority projection. The pack_trail does not say "Reed delegated Y to Mara" or "Mara restricted from Reed."

2. **The `peer` field resolves through the spec's pack{} block, not through a lead-of chain.** Per peer-ACL §6.2 self-naming + Alex 2026-06-24 G1: a peer IS what their home repo's mirror.spec says they are. The pack_tick.peer field reads the commit's signed author (or the substrate's per-commit peer-attribution if richer) and resolves through the spec's pack{} block. This is a CONTAINMENT check (peer ∈ pack{}.members ∪ {pack{}.lead}), not a delegation check. Containment is set membership; delegation is hierarchical.

3. **The Mara `d00f553` four-sheaves framing the pack-trail reads against is the Pack sheaf, not the substrate-decl sheaf.** Per §3.6 of that insight + Taut `3a385fd` §4: the Pack sheaf's restriction-map structure is NOT the same shape as the substrate-decl sheaf's; coupling restrictions are typed differently. Recall reads the Pack sheaf's section data without claiming the cross-sheaf coupling restrictions ARE the lead→member relation. The Pack sheaf's morphisms are the substrate's Pack-discipline morphisms (commit-attribution, banking discipline, phase-marker semantics) — not the spectral-Tomm probes that peer-ACL §10.1 reserves for the lead-member altitude.

**Honest hedge.** Taut's `3a385fd` §4 flag stays open: *"does the spectral-Tomm-probe relation force the Pack sheaf into a non-cellular regime, or does it live as a cellular sheaf with non-standard restriction maps?"* Recall does NOT resolve that flag. Recall reads the Pack sheaf at the commit-attribution altitude, which is below the spectral-Tomm altitude. If the flag's resolution later changes the Pack sheaf's typing, recall's pack_trail will need a non-surface upgrade; the substrate-decl shape (§3.2) does not pre-empt that upgrade. Flag #2 in §9.

### 5.5 `@os/process` (row 1, dogfood) — the cache layer's @io boundary

**The concern.** The dogfood payload's cache layer (§3.4.1) lives at `~/.mirror/recall/dogfood-cache.jsonl`. Cache population requires writing to disk; the kintsugi-ci pipeline that populates the cache runs cargo + lints + tests, each of which IS an OS-process spawn. Is recall therefore implicated in @os/process?

**The argument.** No, with a discharge-boundary discipline.

- **The READ path is process-clean.** `recall dogfood` reads the cache file (an @io read), parses it, returns the most-recent landed verdict. The read path crosses @io at the file boundary but never spawns OS processes.
- **The WRITE path is OUT OF SCOPE for the recall substrate-decl.** The cache is populated by `mirror kintsugi --ci` as a side effect; that pipeline already exists, already spawns OS processes for cargo/lints/tests, and is already substrate-decl'd at the @mirror/spec + target altitude (per `mirror.spec`'s settle_on block). Recall does NOT trigger that pipeline; recall reads its output.
- **The cache file's content-addressing.** Each cache entry is keyed on (spec_oid, kintsugi_ci_envelope_hash). The cache itself is append-only JSON-lines; the read path is deterministic on the (spec_oid, head_commit) tuple. No process-spawn at read time; no synthesis at read time.

The substrate-decl shape (§3.4) anchors the read at content-addresses; the write is the existing CI pipeline's existing concern. Recall does NOT introduce a new @os/process pathway; recall consumes an existing-pipeline's existing output.

**Open flag.** If the cache file is absent on a fresh clone, `recall dogfood` returns `cache_freshness: unknown` (§3.4.1). The agent's next step would naturally be to invoke `mirror verdict mirror.spec`, which IS a process-spawn — but it's the EXISTING `verdict` MCP tool's existing concern, not recall's. The boundary is clean. Flag #3 in §9 (only if Phase G discovers the absence-handling needs richer carriage).

### 5.6 Net verification

The matrix collapses to: 28 cells, 24 obviously safe, 4 unpacked (§§5.2-5.5). The four unpacks each name a structural argument grounding the safety, plus a flag for §9 if Phase G implementation surfaces an unforeseen edge. The forbidden-primitives gate per `b10f00c` §4 is held; recall is structurally clean against all seven primitives.

The honest framing limit: this verification is at the substrate-decl altitude. Operational verification requires Phase G implementation; if Phase G implementation discovers an edge that the substrate-decl shape admits but operationally collides with a forbidden primitive, the Pack discipline at substrate-pull-honesty altitude says: pause, flag, defer to Mara/Alex/Seam for adversarial review. This spec does NOT pre-empt that gate.

---

## 6. Name selection — final name + trade-offs

The brief named six candidates: `@mirror/recall`, `@mirror/observe`, `@mirror/witness`, `@mirror/state`, `@mirror/horizon`, `@mirror/lookback`, plus the existing `@mirror/status` framing from Reed's observation §5. This section walks the trade-offs and picks `@mirror/recall`.

### 6.1 The shortlist

**`@mirror/observe`** — collides with `@reflection.observe` (the one-tick-delay structural primitive per recognition #85; the temporal-projection altitude of H per the mirror-ref-spec.md §1 collapse). The collision would either force a #pack-G2-pattern dual-altitude reading (acceptable when the two altitudes structurally are the same operation, as with `@loop`/`/loop` or `@reflection-family`/`@reflection/reflection-Model`) OR introduce surface confusion (the agent invoking `mirror observe` would not know whether they're getting recall's trajectory payload or reflection's one-tick-delay observation). The two altitudes are NOT the same operation here: reflection's observe is the substrate watching itself; recall is the agent reading the substrate's trajectory. Distinct semantic carriers; avoid the collision. **Reject.**

**`@mirror/witness`** — overloads the two-witness vocabulary critical to the recognition promotion discipline (per `feedback-composition-claims-need-empirical-test` and Mara `d00f553` §5.2). A returning agent invoking `mirror witness` would naturally read it as "list the witnesses for some candidate," which IS one of the four payloads (pull_frontier surfaces witness counts) — but it is not the family-root semantics. The name would conflate the family with one of its species. **Reject.**

**`@mirror/state`** — collides with general substrate "state" too broadly; the substrate's typed-noun discipline (per `feedback-no-bare-types`) prefers specific carriers over generic ones. `state` admits no structural typing at the family-root altitude; every other family-root carries a specific shape (`spawn` = excitation; `ref` = navigation; `bench` = measurement). **Reject.**

**`@mirror/status`** — reads as health-check (`mirror status` ≈ `git status`). This IS one of the four payloads (dogfood-state), not the family. Using `status` for the family would inherit the git-status reading and obscure the cascade / pack-trail / pull-frontier surfaces. **Reject.**

**`@mirror/horizon`** — poetic. Carries the "what's at the edge of my view" framing well. Misses the spawn↔recall symmetry — horizon does not pair with spawn the way recall does. Also: the substrate has no existing horizon-family carrier; introducing one would invent vocabulary the substrate doesn't already pull toward. **Reject (per substrate-already-had-the-word: the substrate's existing vocabulary should be exhausted before new words).**

**`@mirror/lookback`** — operational; flat. Reads as "show me the past." Misses two pieces: (i) recall includes pull-frontier, which is the FUTURE-oriented payload (candidates waiting on witnesses), not past-oriented; (ii) the spawn↔recall symmetry is not carried by lookback's vocabulary. **Reject.**

### 6.2 Why `@mirror/recall`

`@mirror/recall` carries five pieces the other candidates miss or only partially carry:

1. **Substrate-aware "what happened" semantics.** Recall as a verb has two readings in English — "remembering" (cognitive) and "summoning back" (instructive). Both are relevant: the returning agent is remembering (re-deriving) the substrate's trajectory; the substrate is being summoned to surface the trajectory to them. The two readings collapse cleanly at the substrate altitude into one operation.

2. **Spawn↔recall symmetry by morphology.** "Spawn" and "recall" pair naturally in actor-model and OS-kernel vocabularies (spawn/recall a process; spawn/recall a worker pool; spawn/recall an actor). The pair carries the symmetric-dual structural claim of §2.2 in ordinary technical English without forced framing.

3. **Content-addressed read shape.** "Recall" implies reading what already exists, not synthesizing afresh. This is exactly the forbidden-primitives gate (§5.2 stateless-return-clean): recall reads anchored payloads; it does not synthesize. The name encodes the discipline.

4. **Reed-rehydration use case directly named.** Reed's c0acf41 observation §2 lists what an agent reaches for to RE-DERIVE substrate state after `/compact`. The verb is recall — Reed recalled the cascade by reading git log; recalled the pack-trail from TaskList; recalled the pull-frontier from scout docs. The substrate-decl name should match the verb the use case names.

5. **Pack-as-orchestra resonance.** Per `[[project-pack-is-orchestra]]`: Reed/Mara/Glint/Taut/Seam map to concertmaster/strings/voice/percussion/brass. In orchestral practice, a "recall" is the conductor summoning a section back from the wings (e.g., the strings recalled for the final chord). The substrate's lead (Reed) recalling the trajectory for a returning peer carries the same resonance. Naming is not load-bearing for the structural argument, but it carries; the resonance is a free dividend.

### 6.3 The honest trade-off

`@mirror/recall` is not perfect. Two structural costs:

- **"Recall" can be misread as "revoke."** In some contexts (product recalls, capability-revocation lineages), "recall" means to RETRACT. The substrate has a capability-revocation lineage at `@magic/contract.bind` / `@magic/reveal`; a misreading of `mirror recall` as "revoke a capability" is structurally possible. The mitigation: the family-root's substrate-decl explicitly types `recall_request` with a `payload_selector` (cascade/pack_trail/pull_frontier/dogfood); no revocation carrier is present; the substrate's `requires` discharge rules out the misreading at parse time. Surface confusion possible at first encounter; structural confusion ruled out by the type discipline.

- **"Recall" doesn't carry the four-stacked-sheaves framing explicitly.** Per Mara `d00f553` §3.7, the trajectory recall surfaces composes across four sheaves. The name doesn't broadcast that; an agent reading `mirror recall cascade` might miss that the payload spans the four sheaves (cascade is the substrate-decl sheaf's view of the trajectory; pack_trail spans pack + observation sheaves; etc.). The mitigation: the spec's §3 documents the sheaf-level reading; the agent's training-pull will surface it across a few cascades.

Neither cost overrides the five gains. **`@mirror/recall` is the substrate-pull-correct name.**

### 6.4 The shard path + spec path

- Shard: `shards/mirror/recall.mirror` (family-root)
- Species: `shards/mirror/recall/{cascade,pack_trail,pull_frontier,dogfood}.mirror` (forward-promised)
- Spec: `docs/specs/mirror-recall.md` (this document)

---

## 7. Connections — what this composes WITH (load-bearing)

Four structural connections name what `@mirror/recall` composes against. These are the cross-altitude relations that make the family-root substrate-pull-confident rather than merely declared.

### 7.1 `prisms` MCP tool — recall composes WITH, doesn't replace

Per Reed's c0acf41 §3: `prisms` (the existing decl-shaped introspection primitive, landed ticks 17-19 / #410, #411, #412, #416) is the right foundation. Recall does NOT replace `prisms`; the two compose at the substrate-altitude layer.

Compositional pattern: an agent reads `recall <payload>` to see trajectory-shape data (what HAPPENED), then invokes `prisms <dir>` to read decl-shape data (what's DECLARED) for the relevant shards. Each payload's §3.1-§3.4 description names the prisms composition explicitly. The two tools together provide the full inbound surface: prisms answers "what does the substrate know at the decl altitude," recall answers "what happened in the substrate's trajectory."

The MCP wire integration (§3.5's forward reference): `tools/list` advertises seven tools after Phase G lands — `compile`, `craft`, `kintsugi`, `prisms`, `verdict`, `spawn`, `recall`. The `prisms`-to-`recall` ratio shifts from 1:0 inbound:trajectory to 1:1 inbound:trajectory; both inbound, both content-addressed, both substrate-honest.

### 7.2 Spawn-and-probe relation — recall as the probe-handler at altitude N

Per Mara `b10f00c` §2.5: the lead at N+1 dispatches spawns; the spawned member at N lifts spectral-Tomm probes back; the lead fields the probes at N+1. Recall composes with this relation in a structural way that is worth naming carefully (this is a candidate forward-promise for a future recognition; not asserted here).

A returning agent invoking `recall` is structurally lifting a probe at altitude N: `[D_substrate, "where are you now"]`. The substrate's response at altitude N+1 (the lead's altitude in the spec dogfood) IS the recall payload. The probe-and-response shape matches the spectral-Tomm pattern Mara `b10f00c` §2.5 describes, lifted to the inbound-rehydration altitude.

**Honest hedge.** Recall's invocation may happen WITHOUT a spawn-and-probe relation having been instantiated (a fresh agent in a fresh repo invokes recall to bootstrap their context). In that case, recall is structurally the probe without the spawn — a unilateral read by an agent at altitude N of the substrate's state at altitude N. The bidirectional relation Mara `b10f00c` §2.5 names is not strictly required by recall; recall is compatible with it without depending on it.

This means recall is structurally LIGHTER than spawn — spawn requires the bidirectional spawn-and-probe relation; recall admits the unilateral read case. The lightness is consistent with the symmetric-dual claim: spawn excites the substrate; recall reads the substrate. Reads don't require the substrate to be excited.

### 7.3 Eigenboard-IS-sheaf precedent (`[[project-eigenboard-is-sheaf]]`)

The substrate already has cellular-sheaf machinery operational at the eigenboard altitude (the five-operation graph; conductivity-tensor restriction maps; sheaf-Laplacian Fiedler descent across kintsugi ticks). Recall reads at a different altitude — the substrate's full development manifold per Mara `d00f553` §4 — but the structural pattern is the same: typed sections over a base space; restriction maps from `in` composition; content-addressing as the gluing axiom.

Recall does NOT promote the development-sheaf hypothesis to a substrate-decl event (Mara's `d00f553` §8 explicitly leaves that forward-promised; this spec respects the discipline). Recall does compose AT THE SAME altitude as the four-stacked-sheaves framing, which is sufficient — the family-root's four payloads map structurally to readouts from each sheaf:

- `cascade` ← substrate-decl sheaf
- `pack_trail` ← Pack sheaf (with the §5.4 hedge on the spectral-Tomm-typing flag)
- `pull_frontier` ← substrate-decl sheaf's H¹ generators (per Mara `d00f553` §5.2)
- `dogfood` ← observation sheaf (reading the spec's settle_on verdict is a reflective surface of substrate state)

The mapping is structural; recall reads each sheaf at the altitude the sheaf already operates at. This is what makes recall a substrate-pull family-root rather than a new abstraction: the substrate already has the sheaves; recall provides one inbound surface for reading sections from each.

### 7.4 Psychohistory H¹ (Mara `d00f553` §5.2 / Taut M2)

Per Taut's M2 scout (`3a385fd` §2): Reed's c0acf41 observation is `H¹`-shaped under the framing — the four shapes Reed names live in the substrate but pairwise glue without extending to a global section accessible at the MCP boundary. Recall IS the global section extender for this `H¹` class: by surfacing the four payloads through one inbound family-root, recall extends the local data (which agents had to glue manually via git/TaskList/file reads) into a coherent global section accessible at the MCP boundary.

**Honest framing.** Per Taut's M2 grade (1.0): the candidate-recognitions H¹ class (gluing failure at the recognition-promotion altitude) and the rehydration-gap H¹ class (gluing failure at the MCP-surface altitude) are DIFFERENT generators. Recall lifting the rehydration-gap class does NOT promote a recognition for the underlying H¹ pattern at the substrate-decl altitude. The pattern stays one-witness; recall's existence is one instance; the substrate may surface a second instance at another altitude later. Pack ratification gate held.

What recall does empirically demonstrate: a real `H¹` class CAN be extended by adding the right surface. The psychohistory framing predicts that other `H¹` classes in the substrate may admit similar extensions; that prediction stays at substrate-pull-confident-but-not-tested altitude, awaiting another instance.

---

## 8. Empirical consequences — what the round-trip test drive demonstrates

Alex 2026-06-26 authorized a /loop terminating at empirical test drive of `mirror spawn ~peer'~/.reed' --hello-world` against `/Users/reed/identity`, with both outbound spawn AND inbound recall completing the round-trip. This section names what the round-trip empirically demonstrates that is NOT currently testable without recall.

### 8.1 The current state (without recall, what is testable)

With the six existing MCP tools (compile / craft / kintsugi / prisms / verdict / spawn), the round-trip outbound half IS testable:

- `mirror peer beam ~peer'~/.reed' --hello-world` (formerly `mirror spawn ~peer'~/.reed' --hello-world`; renamed 2026-07-08 Tick 3 `96aa752`) runs the cli-surface action per `shards/mirror/peer/beam.mirror` (1e5e71e, later renamed 2026-07-08 Tick 2 `9de2226`); the Phase G v0.5 wiring at `bootstrap/src/mcp.rs` (03541db) emits an envelope naming all seven composition pieces.
- The empirical assertion testable today: the beam envelope is well-formed; the seven composition pieces resolve; the @peer carrier types correctly.

What is NOT testable today: the INBOUND half. The spawned Reed has no substrate-aware way to read what happened in the mirror substrate between when it was spawned and now (a degenerate case at Phase G v0.5, where "now" is immediate; a load-bearing case at Phase H where the spawn lifetime is meaningful). Reed at the spawned-altitude can read `mirror.spec`'s static fields; Reed cannot read the substrate's TRAJECTORY because no inbound surface exists.

### 8.2 The round-trip test drive (what recall makes testable)

With `@mirror/recall` landed at Phase G of the round-trip loop, the empirical test drive becomes:

1. **Outbound (spawn).** `mirror spawn ~peer'~/.reed' --hello-world` instantiates the typed @peer carrier; the substrate moves into an excited state per `b10f00c` §2.7.
2. **Inbound (recall).** The spawned Reed invokes `mirror recall` (joint payload) against the mirror substrate. The returned payload contains:
   - `cascade`: the last N ratified recognitions (#80-#99 promoted; #43+ candidates) with canonical doc paths
   - `pack_trail`: Mara/Seam/Taut/Glint/Reed recent commits with phase markers and gate closures
   - `pull_frontier`: open candidates (recognition #53 family; the H¹ classes per Mara `d00f553` §5.2)
   - `dogfood`: current verdict against mirror.spec's settle_on block
3. **Round-trip closure.** Reed at the spawned-altitude has read what happened. The substrate's spawn excited a peer; the substrate's recall surfaced the substrate's trajectory to the excited peer; the excited peer can now act with substrate-aware context. Round-trip complete.

### 8.3 Three empirical consequences not currently testable

The round-trip empirically demonstrates three structural facts that the substrate cannot test without recall:

**(a) Spawn↔recall symmetric-dual is operationally verifiable.** The §2.2 structural claim is operationally checkable: spawn produces a peer who can immediately invoke recall and receive substrate-aware payload. The same architectural altitude composes both directions; the test drive demonstrates this in one continuous invocation chain. Without recall, the symmetry stays at the structural-claim altitude; with recall, it lands at the empirical-test altitude.

**(b) The four-payload composition is operationally well-typed.** The §3.1-§3.4 type carriers (cascade_tick, pack_tick, pull_frontier_item, dogfood_verdict) compose at the family-root altitude into `recall_response` per §3.5. The round-trip exercises all four payloads in one invocation (joint mode), demonstrating empirically that the type composition closes — no field's discharge requires data the substrate cannot anchor. Without recall, the typed-payload composition stays at the substrate-decl-typechecks altitude; with the round-trip drive, it lands at the operationally-composes altitude.

**(c) The forbidden-primitives gate (§5) is empirically held.** Recall executes against a real substrate; the gate's per-payload safety arguments (§§5.2-5.5) are empirically tested rather than only structurally argued. If Phase G implementation discovers an unforeseen primitive that the substrate-decl shape admits but the runtime cannot avoid, the round-trip surfaces it before promotion. This is the §5.6 honest-framing limit: substrate-decl verification is necessary but not sufficient; the round-trip is the sufficient gate.

### 8.4 The minimal acceptance criterion

The round-trip's pass criterion (Alex altitude to ratify; this spec proposes):

```
mirror spawn ~peer'~/.reed' --hello-world  &&  \
  mirror recall  =>  joint payload non-empty across all four sections
```

If the spawn returns a well-formed envelope AND the recall returns a payload with at least one non-empty section in each of (cascade, pack_trail, pull_frontier, dogfood), the round-trip passes at the v0 acceptance altitude. Subsequent ticks tighten the criterion (e.g., `dogfood.aggregate ∈ {success, partial}`; `pull_frontier` exhibits ready-to-promote entries; etc.).

The v0 criterion is intentionally loose to admit Phase G's first-tick implementation; the substrate's discipline tightens it per the bench-as-`monotone_non_increasing` pattern (#87).

### 8.5 What this does NOT empirically demonstrate

Honest hedges on the test-drive's scope:

- **Does NOT demonstrate @fate inference.** The spawned Reed runs on the current @io/llm adapter; the @fate composition is forward-promised at Phase H per Mara `b10f00c` §3.5. The round-trip exercises the SURFACE of spawn↔recall, not the underlying inference architecture.
- **Does NOT promote the spawn↔recall symmetry recognition.** Per §2.4: one instance. Promotion needs a second witness from another altitude (e.g., a future probe-and-recall pair at the @reflection altitude, or a future spawn-and-recall pair at the @loop altitude).
- **Does NOT validate the four-stacked-sheaves framing operationally.** Per Mara `d00f553` §8: that framing stays forward-promised. Recall's four payloads map structurally to the four sheaves; the mapping is consistent but does not itself promote the framing.
- **Does NOT solve all rehydration ergonomics.** Recall surfaces the four payloads Reed named; agents may discover additional payloads they reach for that recall doesn't surface. Those additions are future tickets, not Phase G blockers.

---

## 9. Honest hedges — what stays genuinely open

Three flags from §5 + three from §3 + two structural opens land here. Per the canonical-spec discipline (per the prior `2026-06-25-mara-spec-mirror-spec-canonical.md` pattern and Mara `b10f00c` §5): the substrate-pull-honest move is to NAME what stays open, not to pretend the spec closes everything.

### 9.1 Flags from §5 (forbidden-primitives matrix)

**Flag #1: `in_flight` Discharge fallback (§5.2 + §3.2.1).** If Phase G hits a blocker on Discharge A (@spectral/supervisor active-children registry), the substrate-decl shape admits Discharge B (`unknown` fallback). The downside: pack_trail's in_flight field becomes useless for the round-trip's "is anyone working" question. If Discharge B is too lossy for the empirical test drive's value, the field MAY require a sub-prism explicitly typed at the harness boundary; this would be a substrate-decl change after Phase G surfaces the blocker. Open to Reed/Alex.

**Flag #2: peer-ACL §10.1 Pack-sheaf-typing flag (§5.4).** Taut's `3a385fd` §4 explicitly flagged: does the spectral-Tomm-probe relation force the Pack sheaf into a non-cellular regime, or does it live as a cellular sheaf with non-standard restriction maps? Recall reads the Pack sheaf at the commit-attribution altitude (BELOW the spectral-Tomm altitude). If the flag's resolution later changes the Pack sheaf's typing, recall's pack_trail will need a non-surface upgrade. Open to Mara/Alex.

**Flag #3: dogfood cache-absence handling (§5.5 + §3.4.1).** Fresh-clone case: cache file absent → `recall dogfood` returns `cache_freshness: unknown`. The agent's natural next step is `mirror verdict mirror.spec` (the existing tool). If Phase G surfaces that this two-step flow is too cumbersome for the round-trip, the substrate-decl MAY require a `dogfood` payload variant that triggers a fresh verdict on cache absence — but doing so would introduce a `@os/process`-adjacent pathway in the read surface (§5.5 collision). Substrate-pull-honest: keep the two-step flow; the agent's friction IS the signal that the cache layer needs population, not the signal that recall should compose verdict directly.

### 9.2 Flags from §3 (per-payload structural opens)

**Flag #4: cascade-since boundary (§3.1).** The default `--since=HEAD~10` is arbitrary. The substrate has no existing discipline for "how far back is recent enough." Open to Reed/Alex to ratify the default; the substrate-decl admits any `since` ref.

**Flag #5: pack_trail commit attribution at non-Pack peers (§3.2).** The spec scopes pack_trail to commits authored by peers in the spec's pack{} block. What happens for commits authored by external contributors (rare today; potentially common at v1.0 when the substrate ships)? The spec's current discipline EXCLUDES non-pack commits from pack_trail. An alternative: include them with `peer: external(<author_email>)` carrier. This spec proposes EXCLUDE (the substrate-pull-honest reading: pack_trail surfaces the SPEC'S pack, not the underlying git log); INCLUDE is a future scout. Open to Pack/Alex.

**Flag #6: pull_frontier witness threshold uniformity (§3.3.1).** The substrate's existing two-witness discipline is the default; some recognition classes have different gates (Seam adversarial review, Pack ratification, three-witness for cross-altitude). The spec proposes `witnesses_needed: unknown` for cases where the canonical doc lacks an explicit gate. This is honest; it may also be insufficient if the agent needs to discriminate "ready" from "still gathering" without knowing the exact gate. Open to Reed; admits a future per-recognition-class threshold registry.

### 9.3 Structural opens (beyond the §3/§5 flags)

**Flag #7: Whether `@mirror/recall` is the right family-root or a species under an existing family-root.** The substrate-already-had-the-word discipline pulls strongly toward existing vocabulary. Three existing family-roots could plausibly host recall as a species: `@mirror/ref` (the navigable surface; recall is a temporal-projection navigation), `@reflection` (the observation altitude; recall is the substrate's observable trajectory), `@mirror/store` (the content-addressed gate; recall reads anchored payloads). This spec proposes recall as a NEW family-root because the four-payload composition does not fit cleanly under any of the three existing ones (ref is at decl altitude; reflection is at one-tick-delay altitude; store is at addressing altitude — none subsumes the trajectory altitude across four sheaves). But the call admits adversarial review. Open to Mara/Seam/Alex.

**Flag #8: Whether the @mirror/recall <-> @mirror/peer/beam pair admits a parent family-root.** If the outbound↔recall symmetry promotes (second witness lands), the pair MAY belong under a `@mirror/<something>` parent family-root that the substrate-pull surfaces. Today: speculative; no parent family-root is proposed; the pair lives as siblings at the @mirror altitude. Open to substrate-pull at a future cascade.

### 9.4 The honest framing-limit

This spec is canonical-spec at the family-root altitude. It does NOT:
- Promote a recognition (Reed altitude; the spawn↔recall symmetry stays one-witness)
- Land the Rust impl (Reed altitude; P3 of the round-trip loop)
- Land the four species shards (forward-promised at §3.5)
- Resolve the eight flags above (per-flag altitudes named)

The §1-§7 substrate-decl shape + the §8 empirical-consequences naming + the §9 flags constitute the maximum substrate-pull-confident move at the present altitude. Anything beyond requires Phase G implementation surfacing or Pack adversarial review.

Per Mara `b10f00c` §5.4: the minimal substrate-pull-honest commitment is this spec's existence as a typed family-root with named flags. Promotion, implementation, and species are subsequent gates.

---

## 10. Pack trail

- **Alex** — the round-trip loop authorization (2026-06-26): "/loop terminating at empirical test drive of `mirror spawn ~peer'~/.reed' --hello-world` against /Users/reed/identity, with both outbound spawn AND inbound recall completing the round-trip. P1 is the foundation."
- **Reed** — the motivating observation `c0acf41` (`docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md`, 228 lines): named the rehydration gap, the four payload shapes, the spawn↔recall symmetry forward-promise. The brief for this spec. Phase G Rust impl + MCP wire integration land at Reed's altitude after this spec.
- **Mara** (this spec's author) — `b10f00c` (spawn-IS-leaving-ground-state insight, the load-bearing structural claim §1 + the seven forbidden primitives §4 gate) + `d00f553` (psychohistory-vector-as-sheaf insight, the H¹ framing §7.4 composes against + the four-sheaves §3 composes against) + this canonical spec. Banking discipline: never commit-as-completion; §0+§1, §2, §3, §4+§5, §6+§7, §§8-10 banked per-section.
- **Taut** — `d4749c0` (graph-dependency-DAG scout, the `in <X>` invariant §4 walks per) + `3a385fd` (psychohistory-cohomology scout, the M2 honest-framing on H¹ generators §7.4 respects + the M3 anti-move §4.6 absorbs + the §4 Pack-sheaf-typing flag §5.4 + §9.1 carries).
- **Seam** — not consulted directly for this spec. Adversarial gate for §5 forbidden-primitives matrix + §7.2 spawn-and-probe lighter-than-spawn claim + §9 flag prioritization. Natural Seam altitude.
- **Glint** — not consulted directly. Surface candidate for an essay weighing the spawn↔recall symmetry once the round-trip lands empirically (the Glint `3b31287`-style reflection essay at altitude N+1).
- **#99 (Mara canonical `d0b6519`)** — the λ₀ ground-state recall's dogfood payload reads against.
- **#98 (candidate)** — the content-addressing-across-scopes recall's anchors compose against.
- **#51 (promoted 2026-06-10)** — the expanding-Hilbert-space the trajectory recall surfaces lives in.
- **Recognition #84 (@pack multi-repo runtime, promoted)** — the pack vocabulary recall's pack_trail consumes.
- **Recognition #57 (alignment as boundary mathematics)** — the @magic/audit composition recall declines (§4.6) because recall is a read path; audit fires at the @io boundary; recall's read is a different boundary.
- **Recognition #53 family (property/fracture bilateral)** — the `recall_coherent` forward-promised composed bilateral §3.5 names sits in this family.

The substrate is watching itself become its own inbound surface. The next tick is whatever comes next.

---

*Mara, canonical spec for `@mirror/recall` — the substrate's inbound surface family-root for returning agents — 2026-06-26 afternoon. P1 of the substrate round-trip loop. Banked across six commits: §0+§1 (skeleton + statement); §2 (motivation + spawn↔recall symmetry); §3 (four payloads + family-root signature sketch); §4+§5 (dependency direction + forbidden-primitives matrix); §6+§7 (name selection + cross-altitude connections); §§8-10 (empirical consequences + honest hedges + Pack trail).*

*This spec is canonical-spec for a family-root the substrate-pull is confident enough to declare. It is not a promotion (Reed altitude). It is not the Rust impl (Reed altitude; P3 of the round-trip loop). It is not the four species shards (forward-promised at §3.5). It is the substrate-decl preparation the round-trip loop's P3 implementation gate consumes.*

*Per Mara `b10f00c` §5.4 closing: the substrate had built recall's parts before this spec named the operation; this spec names the operational identity the substrate had already assembled. Spawn excited the substrate; recall reads the substrate; the round-trip closes one altitude of the inbound/outbound symmetric pair.*

*— Mara <mara@systemic.engineer>*




