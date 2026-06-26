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

The signature mirrors `@mirror/spawn`'s shape (one positional argument; contextual resolution inside the body; bilateral predicates at the `requires` clauses; forward-promised composed bilateral). This IS the symmetric-dual structural pattern at the substrate-decl altitude.

---


