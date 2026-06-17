# The Librarian — `@spectral/db` as the Substrate's Autopoietic Memory

*2026-06-17. Mara. Spec — formalizing `@spectral/db` as the consolidation
layer that turns `@mirror/store` (static content-addressed crystal
accumulation) into a living self-optimizing memory system; the mycelium
connecting per-Pack-member peer instances into one orchestra; the per-repo
supervisors at object altitude and the root supervisor at `~/.mirror` —
the librarian — operating at Bateson level N+1, perturbing cross-repo
topology so the book is on the table before the query arrives. Markdown
only; no `.mirror` files land with this commit. Substrate declarations
are forward-promised as RED+GREEN ticks; bodies discharge in subsequent
ticks per the bilateral pattern.*

Status: **Red.** The architectural shape is locked by Alex's framing
below. The storage-vs-mind distinction is named. The mycelium / orchestra
metaphor is operationalized into existing substrate carriers. The
per-repo supervisor + root librarian hierarchy is pinned to existing
machinery (`@spectral/supervisor`, `@spectral/root`, HamiltonScheduler,
the @mirror/reality/shard CRDT). The optimization shape is grounded in
the sheaf-Laplacian + curvature substrate that already lives at
`@epistemologic/math`. The carrier surfaces are forward-promised, not
implemented in this tick. Implementation ticks are enumerated in §7.

---

## Reference

### Alex's framings (verbatim, 2026-06-17)

The load-bearing recognition this spec lifts to substrate altitude.
Reproduced verbatim including the correction sequence — the way the
recognition crystallized matters; the substrate-pull discipline shows
in the corrections.

#### First framing

> And the @spectral/db is the autopoietic memory consolidation layer.
> The magic sauce that turns a statically growing store into a living
> autopoietic self-optimizing memory system. The mycelium that turns
> individual peers into an orchestra of spectrally connected actors.
>
> Without it you get the whole storage layer. You don't get the living
> system.
>
> And then think about it, the supervisors on the store [INITIAL
> WORDING: "the supervisors on the manage the store"] manage the
> store, answer queries etc. and the root supervisor at ~/.mirror is
> the librarian that operates at the N+1 layer and perturbs the
> topology to shift information where it needs to be. That puts the
> book on the table before you know you need it.

#### Correction sequence

Alex 2026-06-17, immediately following:

> Supervisors on the *repos* manage the store. The repo is the unit
> of supervision.

Alex 2026-06-17, further clarifying:

> Each repo is a store, so isomorphic but the word still matters.

The correction sequence is structurally informative. The substrate has
two near-synonyms (`repo` and `store`) that point to the same object
but name different roles:

- `store` names the **storage role** — the content-addressed crystal
  accumulation per `@mirror/store` (see
  `docs/specs/mirror-store.md`).
- `repo` names the **supervised unit** — the boundary one supervisor
  process owns and answers queries against.

At the substrate altitude the two are isomorphic (each repo IS a
store). At the operational altitude the word distinction matters: the
librarian supervises *repos*; the queries land in *stores*; the
perturbation moves *crystals* between *repos* (= *stores*).

### Today's load-bearing memories (NEW)

- [[architecture-spectral-db-autopoietic-memory]] — the recognition
  saved 2026-06-17 with the supervisor hierarchy + librarian role.
- [[architecture-peer-learns-by-crystal-vocabulary-expansion]] — the
  static layer this consolidates atop; each settlement produces a
  content-addressed crystal; `A_peer` grows monotonically;
  `H_peer` expands accordingly.

### Math citations (the math root this spec cites)

Per the `docs/math/` vs `docs/specs/` convention (see
`docs/math/README.md` + `AGENTS.md`): this spec CITES math docs;
it does not re-derive the math. Citations used in this spec:

- `docs/math/the-tower/altitudes.md` §2 "librarian altitude" —
  the librarian altitude in the bundle tower; fiber = crystal
  topology, connection = perturbation choice, holonomy = query
  latency · sheaf-coherence (cited throughout §3–§5).
- `docs/math/the-tower/connections-and-gauge.md` §3 — the
  librarian's perturbation IS gauge transformation on `H_topology`;
  preserves principal-bundle structure (cited at §4 "the librarian
  operates at N+1").
- `docs/math/the-tower/crystals-as-sections.md` §9–§10 — inter-peer
  section exchange via the mycelium; sheaf-coherence as integrity
  constraint on perturbation (cited at §3 "the mycelium and the
  orchestra" + the consolidation_preserves_sheaf_coherence pact).
- `docs/math/sheaf/laplacian.md` §2, §5.2 — the sheaf-Laplacian
  `λ₀(F_topology)` as the librarian's coherence reading; per-edge
  curvature for perturbation targeting (cited at §5 "the
  optimization shape" + per-edge bottleneck reading).
- `docs/math/the-tower/holonomy.md` §8 — the librarian's quality
  metric IS the residual holonomy after perturbation (cited at
  §5's optimization objective).
- `docs/math/the-tower/spectral-triples.md` §5 — directed-colimit
  preservation; crystal accumulation across the topology preserves
  the spectral triple's axioms (cited at §2's "the static layer
  this consolidates atop").

### Substrate machinery this spec composes

- [[architecture-three-tier-stack]] — fragmentation-mcp / mirror /
  `@spectral/db` with SpectralSupervisor; the per-repo supervisor
  pattern this spec lifts to the librarian altitude.
- [[architecture-hamilton-scheduler]] — HamiltonScheduler as the
  per-shard memory manager named for Margaret Hamilton (Apollo 1202
  priority discipline); the per-repo supervisor's working-set
  management primitive.
- [[architecture-shard-as-crdt]] — `@mirror/reality/shard` as the
  CRDT layer; bounded semilattice; the convergence guarantee for
  inter-supervisor sync.
- [[architecture-eigenboard-is-sheaf]] — cellular sheaf on the
  five-operation graph; restriction maps = conductivity tensor; the
  same sheaf-theoretic structure the librarian operates over at the
  topology altitude.
- [[architecture-mirror-as-content-addressed-build-system]] — each
  crystal IS a build artifact; the librarian indexes the build graph.
- [[architecture-mirror-as-expanding-hilbert-space]] — `H_peer`
  expansion; recognition #51; the substrate's growth carrier this
  spec extends to inter-peer Hilbert-space joining via the mycelium.
- [[architecture-bateson-logical-type-primitive]] — Bateson tower
  (recognition #42); the librarian operates at N+1 to repo
  supervisors' N; the level-distinction is load-bearing.
- [[architecture-mirror-cogito-glass-over-fate]] — sheaf-coherence
  machinery; the dependency-chain coherence the librarian's
  perturbations preserve.
- [[architecture-reflection-thinks-in-spectral-questions]] — Reflection's
  thinking IS spectral-altitude selection by Tomm-shaped commutator
  probing. **The SAME mathematical operation at a different altitude.**
  Peer reflection at N+1 over compile altitudes; librarian perturbation
  at N+1 over crystal topology. The substrate eats itself.
- [[architecture-geometric-consent-projection]] — consent geometry
  at type N+1; the consent constraint on topology perturbation.
- [[architecture-at-x-is-mathematical-value]] — `@<X>` IS a value;
  `@spectral/db/librarian` IS a substrate value.
- [[architecture-fragmentation-is-the-rust-substrate]] — the recursive
  proof; the substrate IS its own librarian.
- [[project-pack-is-orchestra]] — Reed/Mara/Glint/Taut/Seam mapping
  to concertmaster/strings/voice/percussion/brass; this spec names
  the mycelium as the operational realization.
- [[project-cosmos-spectral-cosmology]] — graph-Laplacian engine at
  cosmic altitude; comparable math the librarian uses at substrate
  altitude.
- [[reference-void-document]] — Connes spectral triple; λ₀ = 0
  ground state where eight dualities meet; the geometric anchor.
- [[feedback-substrate-already-had-the-word]] — applied throughout.
  The mycelium has carriers (entanglement edges, sheaf restrictions,
  CRDT joins). The orchestra has carriers (score, observation,
  metalogue). The librarian has carriers (root supervisor,
  HamiltonScheduler, oscillate). Apply the rule.
- [[feedback-no-stringly-types]], [[feedback-no-bare-types]] — typed
  end-to-end. Access patterns, latency, perturbations, consent
  verdicts: all typed carriers; no bare numerics escape.

### Existing substrate shards this spec cites (DO NOT modify)

- `shards/mirror/store/*.mirror` — the static layer (per
  `docs/specs/mirror-store.md`).
- `shards/mirror/store/crystal.mirror` — the crystal type; altitude-4
  of the kintsugi build lifecycle; the polyglot content-addressed
  artifact the librarian moves across the topology.
- `shards/spectral/entanglement.mirror` — the peer-correlation edge
  type; IS a sheaf restriction map at substrate altitude (recognition
  #55 instance landed 2026-06-11); the mycelium's edge carrier.
- `shards/spectral/portal.mirror` — the typed surface mirror's
  consumers cross when speaking to a store; the IO boundary the
  librarian operates ABOVE.
- `shards/spectral/root.mirror` — the parentless supervisor
  specialization resting at `~/.mirror/`; THE LIBRARIAN'S
  SUBSTRATE-ALTITUDE TYPE.
- `shards/spectral/supervisor.mirror` — the lifecycle-owner
  specialization of `gen_prism`; per-repo supervisor's substrate type.
- `shards/spectral/parent.mirror` — the single-parent acyclic
  lifecycle edge type; the parentage chain that terminates at root.
- `shards/spectral/registry.mirror` — the supervisor's typed child
  index; the per-repo working set.
- `shards/spectral/gen_prism.mirror` — the worker primitive; the
  altitude the per-repo supervisor specializes from.
- `shards/mirror/spectral/score.mirror` — the eigenboard envelope +
  metalogue bus + pending kintsugi state; the orchestra's shared
  score the librarian's perturbations read FROM.
- `shards/mirror/spectral/observation.mirror` — the 16-feature graph
  observation; the librarian's input layer at the topology altitude.
- `shards/mirror/spectral/portal.mirror` — the typed query surface.
- `shards/epistemologic/math/sheaf_laplacian.mirror` — the operator
  Δ_F = δ*δ and λ₀ (algebraic connectivity); the librarian's
  measurement carrier.
- `shards/epistemologic/math/curvature.mirror` — Balanced Forman
  Ric(i,j) per Topping et al. 2022; the per-edge bottleneck reading.
- `shards/kintsugi/oscillate.mirror` — the kintsugi pulse loop; the
  librarian's perturbation pulse rhythm.
- `shards/optics/source/ganglion/fate.mirror` — the eigenvalue
  signal source; feeds the librarian's input through observation.

### Existing substrate (cited; lives in the implementation surface)

- `/Users/alexwolf/dev/projects/spectral-db/` — the existing
  `@spectral/db` crate. The Rust implementation surface the substrate
  decls in this spec land ABOVE. **This spec does NOT modify the
  crate.** Per `docs/specs/store-vs-db-and-the-cascade.md` §1.2:
  `@spectral/db` is the engine on top; `@mirror/store` is the open
  foundation; the two-layer split is architectural and
  business-model-relevant. This spec sits at the engine altitude.
- `/Users/alexwolf/dev/projects/cosmos/` — graph-Laplacian engine at
  cosmic altitude; same math the librarian uses, different scale.

### Pre-existing related specs (citation, NOT modification)

- `docs/specs/store-vs-db-and-the-cascade.md` — the open foundation /
  closed engine split; the cascade making the AST generic over the
  hash algorithm.
- `docs/specs/mirror-store.md` — Layer-1 architecture; the
  three-layer parser; FP1 reframe.
- `docs/specs/spectral-db-three-tier-architecture.md` — the four-tier
  storage stack (hot/warm/cold/iceberg) with biology-typed pheromone
  semantics; the storage surface the librarian's perturbations move
  crystals across.
- `docs/specs/reality-shard-as-crdt.md` — `@mirror/reality/shard` is
  mirror's CRDT layer; the bounded semilattice for inter-supervisor
  sync.
- `docs/specs/spectral-runtime.md` — the gen_prism / supervisor /
  registry / parent / root / entanglement cascade; the substrate
  runtime this spec lifts the librarian on top of.
- `docs/specs/peer-cognition.md` (Mara, 4daa437, unpushed) —
  `@peer` as singular root prism; standalone_use heuristic;
  sheaf-coherence collapse measurement. This spec extends with the
  inter-peer coordination story (§7 forward-promise T11.7).
- `docs/specs/geometric-consent-projection.md` (Mara, 96a4e6d,
  unpushed) — the consent geometry; ACLs as logical-type-1
  projection of a richer Bateson tower; defeasible `but` operator.
  This spec consumes the consent geometry as a constraint on
  topology perturbation.
- `docs/specs/drone-narrative-mapping.md` (Mara, 05a22f0, unpushed) —
  the drone's `attend` recognition; where the drone's crystal enters
  the Pack's shared mycelium (§7 forward-promise T11.8).
- `docs/specs/the-convergence.md` — the Pack/peer architecture base.
- `docs/specs/mirror-spectral.md` — the orchestra's substrate surface;
  metalogue as the score the voices play through.
- `docs/specs/cybernetic-cli.md`, `docs/specs/cli-as-prism.md` — the
  consumer surfaces queries arrive through.
- `docs/specs/error-as-question.md` (Mara, 2026-06-01) — the
  Tomm-shaped projection of substrate state at the user-frame; the
  same probe shape the librarian uses at the topology altitude.

### Task #327 — `~/.mirror` per-home roots (decided 2026-06-14)

Architecturally decided: the librarian's home is `~/.mirror/`
(operational vocabulary, NOT a directory the substrate reads).
Materialized in `shards/spectral/root.mirror` as the parentless
supervisor specialization. This spec gives the root supervisor its
operational role: the librarian.

---

## §1 — The recognition

### 1.1 The storage / mind distinction (load-bearing)

The substrate has two layers in its memory architecture. Conflating
them produces either a static store dressed up in cognitive vocabulary,
or a dynamic system without the substrate guarantees content-addressing
provides. The distinction must be held.

**Storage** = static, content-addressed crystal accumulation. Per
[[architecture-peer-learns-by-crystal-vocabulary-expansion]], every
settlement produces a crystal; crystals are immutable; the peer's
vocabulary algebra `A_peer` grows monotonically; the Hilbert space
`H_peer` expands by one axis per crystal. The substrate's
intrinsic learning mechanism, made operational at
`@mirror/store/crystal`.

**Mind** = autopoietic consolidation atop the static store. The graph
of crystals re-indexed, re-organized, re-surfaced; pathways
strengthened by use; related crystals brought into the working set
before the query arrives. The substrate becomes living memory.

```
Storage   = @mirror/store               (declared; static; monotone)
Mind      = @mirror/store + @spectral/db (recognized today;
                                          living; autopoietic)
```

The distinction is operational, not metaphorical. WITHOUT
`@spectral/db` the substrate has every crystal, but each query hits
cold storage; each peer's vocabulary doesn't compose with others';
the Pack is five isolated peers. WITH `@spectral/db` the substrate's
graph is alive: queries land on prefetched crystals; cross-peer
relations strengthen with co-access; the Pack becomes an orchestra.

### 1.2 The mycelium framing — not a metaphor

Alex's word is `mycelium`. The framing must be operationalized; left
as metaphor it does not bind anything in the substrate.

In the biological case: trees in a forest are individually whole.
The mycelial network (fungal hyphae) connects them — nutrient sharing,
signal propagation, the wood-wide web. The forest becomes a system
because of the mycelium; without it, the forest is a collection.

The substrate's mapping (all carriers exist; per
[[feedback-substrate-already-had-the-word]]):

| Biological | Substrate |
|------------|-----------|
| Tree | Peer instance `@peer(<member>)` with its own crystal accumulation |
| Mycelial hypha | Entanglement edge per `shards/spectral/entanglement.mirror` — sheaf restriction map between peer state surfaces |
| Nutrient signal | Crystal surfaced from one peer's accumulation into another's working set |
| Forest as system | The Pack as orchestra per [[project-pack-is-orchestra]] |
| Mycelial network | `@spectral/db`'s spectral graph over inter-peer crystal relations |

The mycelium IS the spectral graph, restricted to the inter-peer
subgraph; the entanglement edge type IS the substrate-altitude carrier
for the connection; the consent geometry per
[[architecture-geometric-consent-projection]] determines which
nutrient signals can cross which hyphae. No new vocabulary needed.

### 1.3 The orchestra framing — formalized

Per [[project-pack-is-orchestra]]: Reed/Mara/Glint/Taut/Seam map to
concertmaster/strings/voice/percussion/brass. The orchestra metaphor
wasn't decorative; it pinned the load-bearing claim that the Pack's
coordination has a shared score the voices play through.

The substrate IS the score. Specifically `shards/mirror/spectral/score.mirror`:
the eigenboard envelope + metalogue bus + pending kintsugi state,
named collectively as the score. Read-only to voices; mutations
settled through the convergence's daemon. This shard exists today;
landed 2026-06-10 by Mara as the prerequisite for `active_pass` T11.

`@spectral/db` makes the orchestra operational. The score the voices
read from IS the consolidated graph of crystals the librarian
maintains; the perturbations the librarian applies are the conductor's
gestures; the per-repo supervisors are section leads.

The conductor is the librarian. The next sub-section is that role.

---

## §2 — The two layers

### 2.1 Comparison table

| Property | Static layer | Living layer |
|----------|--------------|--------------|
| Substrate root | `@mirror/store` | `@spectral/db` |
| Status | Declared (per `docs/specs/mirror-store.md`) | Recognized today; substrate decls forward-promised (§7) |
| Carrier | content-addressed crystals | spectral graph over crystals + topology metadata |
| Mutability | Crystals immutable (content-addressed) | Topology mutable (re-arrangements; not crystal mutations) |
| Growth | Monotone (per recognition #51 + #58 chain) | Bounded oscillation (perturbations move crystals; total set monotone) |
| Verification | Per write (BLAKE3 over canonical bytes per `docs/specs/store-vs-db-and-the-cascade.md` §1.1) | Per perturbation (consent + sheaf-coherence; §4) |
| Identity carrier | `Splinter<Blake3>` OIDs | `VoidPointer` per `docs/specs/store-vs-db-and-the-cascade.md` §3 (spectral coordinate; eigenvalue vector of local Laplacian) |
| License/posture | Apache-2.0 (per [[feedback-no-paywall-in-compiler]]) | Engine-side; potentially closed-source per the open-foundation / closed-engine cut |
| Dependency direction | No dep on `@spectral/db` | Depends on `@mirror/store` |
| Invariant | `mirror MUST work without @spectral/db` | Composes ABOVE the store; never under it |

### 2.2 The static layer is monotone (recap, load-bearing for §5)

From [[architecture-peer-learns-by-crystal-vocabulary-expansion]]:
at time `t`, peer `P` has spectral triple `(A_t, H_t, D_t)`. A
settlement produces crystal `C_{t+1}`. The transitions:

```
A_{t+1} = A_t ∪ {C_{t+1}}     algebra extends
H_{t+1} ⊃ H_t                 state space expands
D_{t+1} restricts to D_t       Dirac operator extends consistently
```

Monotone because content-addressing: old crystals always exist as
references; the spectral triple is a directed system with monotone
increasing limit (per the Connes-theoretic colimit construction
recognized 2026-06-17).

### 2.3 The living layer does NOT violate content-addressing

A critical clarification. The living layer is dynamic — perturbations,
re-indexing, topology reshapes happen continuously. But:

**Crystals themselves remain immutable.** What changes is their
**topology** — which crystals are co-located in a working set, which
are replicated to which per-repo store, which are surfaced together by
a query. The substrate's content-addressing guarantees are preserved
because perturbation operates on the *graph structure over crystals*,
not on the crystals themselves.

This is the same pattern as `[[architecture-mirror-as-expanding-hilbert-space]]`
recognition #51 §8.3: the Hilbert space dimension grows; the
orthonormal basis is augmented but never overwritten. The librarian's
perturbations augment the topology; they do not rewrite the underlying
content.

### 2.4 The living layer is bounded-oscillation, not unbounded growth

The topology has finite degrees of freedom at any moment (the crystals
that currently exist; their possible per-repo placements; their
working-set assignments). The librarian moves through this finite
configuration space; it does not grow the space itself — that's the
static layer's job.

This matters for convergence. The living layer's oscillation is
bounded by the static layer's monotone growth: each new crystal adds
dimensions to the topology configuration space, but at any instant the
space is finite-dimensional. The librarian's optimization at §5 is
well-posed.

---

## §3 — The mycelium and the orchestra (substrate carriers)

### 3.1 Each peer instance has its own crystal accumulation

Per [[architecture-at-x-is-mathematical-value]] + [[project-pack-is-orchestra]]:
the Pack members are parametric peer instances:

```
@peer(reed)        Reed's instance — concertmaster
@peer(mara)        Mara's instance — strings
@peer(glint)       Glint's instance — voice
@peer(taut)        Taut's instance — percussion
@peer(seam)        Seam's instance — brass
```

Each has its own crystal accumulation from its own settlements. Reed's
reflection-shape settlements produce Reed's crystals at `@peer(reed)`'s
store. Mara's shatter-shape settlements produce Mara's crystals at
`@peer(mara)`'s store.

*Without* `@spectral/db`, these accumulations stay isolated. The
orchestra is five soloists in five separate rooms.

### 3.2 The mycelium IS the inter-peer spectral subgraph

The mycelium is the cross-peer subgraph of `@spectral/db`'s spectral
graph. Its edges are entanglement edges per
`shards/spectral/entanglement.mirror`:

```
entanglement_edge { peers: [uuid_spectral], restriction_map: ... }
```

From that shard (lines 38-51 of `shards/spectral/entanglement.mirror`):

> Entanglement edges ARE sheaf restriction maps at substrate altitude.
> The substrate's eigenboard is a cellular sheaf on the five-operation
> graph; the conductivity tensor IS the restriction-map structure.
> Two gen_prisms are entangled iff their state surfaces participate
> in a shared restriction map — a single observation of one's state
> induces a measurable projection on the other's.

Applied at the peer altitude (one altitude up from gen_prism): two
peer instances are entangled iff their crystal accumulations
participate in a shared restriction map. Same word the substrate
already has at the math altitude (per `shards/epistemologic/math/sheaf_laplacian.mirror`),
at the runtime altitude (entanglement), and now at the peer altitude
(mycelium). The instance count for [[feedback-substrate-already-had-the-word]]
bumps to ~64.

Edge semantics — what makes two crystals connected:

- **Semantic similarity** (cosine over the `VoidPointer` spectral
  coordinate per `docs/specs/store-vs-db-and-the-cascade.md` §3)
- **Co-access frequency** (queried together in some window;
  feeds into HamiltonScheduler's priority discipline)
- **Common-settle ancestry** (settled from the same `@mirror.spec`
  manifold; share a `composition_graph` ref per
  `shards/mirror/store/crystal.mirror`)
- **Common-fracture ancestry** (their fracture calendars cite the
  same opacity-map per recognition #53 bilateral)
- **Peer-frame proximity** (one peer's settle on a configuration close
  to another peer's recent settle; the `attend` recognition's
  Pack-altitude extension)

The substrate's `@epistemologic/math/sheaf_laplacian` operator over
this subgraph reads λ₀ as the spectral connectivity between peers; a
high λ₀ means the Pack is highly mycelium-connected; λ₀ = 0 with
multiplicity > 1 means peers fall into disconnected components (a
break in the orchestra).

### 3.3 Inter-peer crystal exchange under consent geometry

Mycelial transport is not unconditional. Per
[[architecture-geometric-consent-projection]]: consent geometry is
typed end-to-end; ACL is the logical-type-1 projection of a richer
Bateson tower; consent integrity is the asymmetry of the projection.

The consent constraint on mycelial crystal exchange:

```
# pseudo-substrate; lifts to T11.4 + T11.6 in §7
move_via_mycelium(crystal: crystal,
                  source: @peer(A),
                  dest:   @peer(B)) -> verdict {
    \ source consents to surfacing crystal in dest's context
    \ AND dest consents to receiving crystal in source's frame
    \ AND the entanglement edge between source/dest holds at
    \   logical type N+1 over the crystal's content
}
```

The verdict is `transparency`-shaped per `shards/mirror/store/crystal.mirror`'s
`fracture_calendar` precedent: `success | partial(opacity_map) | failure(opacity_map)`.
A partial verdict surfaces an opacity that the next kintsugi pulse
can close (the crystal is shared under degraded resolution; the
opacity says what's missing).

This closes the inter-peer story Reed's T10.5 (per
[[architecture-peer-learns-by-crystal-vocabulary-expansion]] forward
promise) opened. Mara's crystal becomes available to Reed's instance
under the typed consent verdict; nothing slips through unconsented.

### 3.4 The orchestra operationalized

With the mycelium in place, the orchestra metaphor binds:

- The **score** is `shards/mirror/spectral/score.mirror` (eigenboard +
  metalogue + pending state). It exists today.
- The **voices** are the per-Pack-member peer instances
  `@peer(<member>)`. Each plays its part — reflection-shape, shatter-shape,
  etc. per [[architecture-mirror-cogito-glass-over-fate]].
- The **score-reading discipline** is `@mirror/cogito`'s
  notice/name/hold per the cogito shard; the read-only access to the
  score from the voices' altitude.
- The **score-mutation discipline** is settled through the
  convergence's daemon — kintsugi-loop perturbations per
  `shards/kintsugi/oscillate.mirror`.
- The **conductor** is the librarian (next section). Operates one
  altitude above the voices, reshapes the score the voices read.

The substrate already has every piece. `@spectral/db` is the layer
where they integrate into one living memory system.

---

## §4 — Per-repo supervisors + the librarian (load-bearing)

### 4.1 Per-repo supervisors (object altitude)

One supervisor per repo. Per Alex's correction sequence: each repo IS
a store; the word `repo` names the unit of supervision; the word
`store` names the storage role. The repo supervisor:

- Manages the repo's crystal accumulation (lifecycle, restart per
  the supervisor's strategy variant per `shards/spectral/supervisor.mirror`)
- Answers queries against the repo's crystals (via
  `@mirror/spectral/portal` per `shards/mirror/spectral/portal.mirror`)
- Handles failures within the repo (gen_prism restart per the BEAM
  precedent)
- Coordinates with sibling per-repo supervisors via
  `@mirror/reality/shard` CRDT joins per [[architecture-shard-as-crdt]]
- Holds the per-repo HamiltonScheduler (per
  [[architecture-hamilton-scheduler]]); the priority discipline named
  for Margaret Hamilton (Apollo 1202)

Substrate altitude: the per-repo supervisor IS-A `@spectral/supervisor`
(per the existing shard). The supervisor's `child_specs` enumerate the
gen_prisms it owns (the storage workers, the query handlers, the
kintsugi pulse workers per `oscillate.mirror`). The supervisor's
`registry` per `shards/spectral/registry.mirror` is the typed child
index.

```
@spectral/db/supervisor          per-repo supervisor (NEW; T11.3)
  base: supervisor               (existing)
  base.base: gen_prism           (existing)
  scheduler: hamilton_scheduler  (existing)
  store_anchor: ref              (the ref to the repo's @mirror/store)
```

The per-repo supervisor's altitude is **N** in the Bateson sense per
[[architecture-bateson-logical-type-primitive]]. It operates at the
query altitude: queries arrive; it answers them; it does not
meta-reason about *which* queries to anticipate. That's the librarian.

### 4.2 The librarian (root supervisor at `~/.mirror`; altitude N+1)

Per Alex's framing: the root supervisor at `~/.mirror` is the
librarian. Per `shards/spectral/root.mirror`: the root supervisor is
the parentless `@spectral/supervisor` specialization; its operational
resting place is `~/.mirror/` (operational vocabulary, NOT a directory
the substrate reads).

This spec gives the root supervisor its operational role:

**The librarian is `@spectral/root` operating at Bateson level N+1.**

Where per-repo supervisors operate at N (answering queries), the
librarian operates at N+1 (reshaping the substrate that answers
them). The Bateson level distinction is load-bearing — the librarian
does not answer queries; it does not even handle query failures at
the object altitude (per-repo supervisors do). The librarian observes
the distribution of query traffic and perturbs the topology so the
next query lands on prefetched data.

The shape:

```
~/.mirror/                       <- @spectral/root (the librarian; N+1)
  │                                  supervises all per-repo supervisors
  │                                  in this home; perturbs the
  │                                  cross-repo topology
  │
  ├── repo-A/  (= store-A)       <- @spectral/db/supervisor (N)
  │     scheduler: HamiltonScheduler
  │     crystals: <accumulation>
  │
  ├── repo-B/  (= store-B)       <- @spectral/db/supervisor (N)
  │     scheduler: HamiltonScheduler
  │     crystals: <accumulation>
  │
  └── ...
```

The librarian's four operations:

#### 4.2.1 Observe access patterns

Which crystals get queried together; which paths through the spectral
graph are hot; which peer's queries surface which crystals; which
fracture calendars get pulled together. The librarian's input is the
stream of metalogue events arriving from per-repo supervisors via the
shard-CRDT layer.

Substrate carrier: per `shards/mirror/spectral/observation.mirror`,
the 16-feature graph observation. At the librarian's altitude the
features specialize:

- `query_intensity` reads the rate of incoming queries per repo
- `partition_risk` reads the probability of a per-repo partition under
  current load
- `hot_path_density` reads the fraction of cross-repo paths in the
  hot-path band
- `holonomy` reads the closure defect of the librarian's perturbation
  pulses

Observation feeds the librarian's optimization step at §4.2.2.

#### 4.2.2 Compute optimal topology

The librarian computes the cross-repo reorganization minimizing
expected query latency, subject to:

- Sheaf-coherence (don't break dependency chains per
  [[architecture-mirror-cogito-glass-over-fate]] — a crystal moved out
  of a working set must not break a settle-in-progress)
- Consent geometry (don't move crystals across consent boundaries per
  [[architecture-geometric-consent-projection]] — Mara's crystal
  doesn't get prefetched into Reed's working set without the consent
  verdict)
- Per-repo supervisor capacity (HamiltonScheduler's priority discipline
  must remain solvable; a hot crystal moved into an overloaded repo's
  working set must displace a colder one)
- Storage-tier discipline per
  `docs/specs/spectral-db-three-tier-architecture.md` — crystals move
  through hot/warm/cold/iceberg tiers per the pheromone dynamics; the
  librarian's topology operations include tier promotion/demotion

The math is in §5. The carrier the librarian computes against is the
spectral graph's sheaf-Laplacian per
`shards/epistemologic/math/sheaf_laplacian.mirror`; the per-edge
bottleneck reading is Balanced Forman curvature per
`shards/epistemologic/math/curvature.mirror`. Both already exist; both
are load-bearing.

#### 4.2.3 Perturb

The librarian APPLIES the computed topology change. Operationally:

- Move crystals between repos (the per-repo supervisor's working set
  changes; the underlying content-addressed bytes stay where they are,
  but the live working-set assignment relocates)
- Replicate hot crystals to multiple per-repo supervisors (the
  same crystal becomes live in multiple working sets; per the CRDT
  semilattice's idempotency the replications converge cleanly)
- Prefetch related crystals into a working set (anticipatory; before
  the query arrives; per the substrate's prediction paradigm per
  [[feedback-loss-from-epistemologic-properties]] grounded in the
  cybernetic foundation)
- Demote cold crystals from hot to warm to cold to iceberg per the
  pheromone evaporation discipline at
  `docs/specs/spectral-db-three-tier-architecture.md`

The perturbation pulse rhythm is `shards/kintsugi/oscillate.mirror`'s
oscillation. The librarian's perturb step is one ACTIVE-pass of the
kintsugi loop at the topology altitude. The DARK-pass is the
observation step (§4.2.1); the cycle alternates per recognition #51.

#### 4.2.4 Anticipate

The load-bearing claim. "Puts the book on the table before you know
you need it."

Anticipation requires a prediction model. Per
[[feedback-loss-from-epistemologic-properties]]: at every Fate-tournament
altitude, the loss function is a composite of
`@epistemologic/properties`. Not Shannon. Not Dark. Not invented. The
librarian's prediction model is the substrate's prediction model,
operating at the topology altitude.

The substrate's prediction vocabulary per
[[architecture-prediction-paradigm-orthogonal-to-optimization]]
(recognition #56 candidate, 2026-06-10): mirror's gap vocabulary IS
the predictive engine; orthogonal to optimization on axis-1
(throughput) vs axis-5 (epistemologic); cleanest adjacent prior art =
Deutsch-Marletto constructor theory.

Applied at the librarian altitude: each perturbation pulse asks "what
is the next probable query?" and pre-loads the answer. The prediction
is not statistical (frequency × recency); it is structural — what
crystals does the substrate's compositional grammar imply will be
needed given the current settlement-in-progress?

The predictor's loss is the same `@epistemologic/properties` composite
that drives Reflection's altitude selection at the per-peer altitude.
The substrate's loss carrier is universal across altitudes; per
Taut's `docs/specs/benchmark-tracing.md`, transparency is the
performance wire; per the cybernetic foundation, the loss is grounded
in Ashby-Beer-Bateson primitives.

### 4.3 The librarian is `@spectral/root` with an operational role

The substrate does NOT introduce a new type for the librarian. Per
[[feedback-substrate-already-had-the-word]]: the substrate already has
`@spectral/root` (per `shards/spectral/root.mirror`). The librarian IS
the operational role `@spectral/root` plays at the `@spectral/db`
integration altitude.

The forward-promised shard (T11.1) declares a `@spectral/db/librarian`
glass that specializes `@spectral/root`. The specialization adds:

- An observation surface (reads from `@mirror/spectral/observation`
  scoped to the cross-repo subgraph)
- A perturbation surface (writes through
  `@spectral/db/consolidation` operations per T11.2)
- A prediction surface (consumes `@epistemologic/properties` losses
  to drive anticipation per T11.10)

The specialization is structural-by-embedding per the
`shards/spectral/supervisor.mirror` precedent — the librarian record
carries the root record in `base`, then adds the three surfaces.

---

## §5 — Mathematical shape

### 5.1 The librarian as continuous optimization over topology

Formally, at each pulse `t`, the librarian solves:

```
T_{t+1} = argmin_{T ∈ T_admissible}
              α · expected_query_latency(T, observation_t)
            + β · constraint_violations(T)
```

where:

- `T ∈ T_admissible` = the set of cross-repo topologies satisfying
  consent geometry + sheaf coherence + per-repo capacity
- `observation_t` = the 16-feature graph observation at time `t`
  (per `shards/mirror/spectral/observation.mirror`)
- `expected_query_latency` = an expectation over the access
  distribution; modelled from the librarian's predictor
- `constraint_violations` = the count of soft constraints crossed
  (hard constraints make `T` inadmissible; soft constraints accumulate
  as loss)
- `α, β` = the typed weights per
  [[feedback-loss-from-epistemologic-properties]] grounded in
  `@epistemologic/properties`

The arg-min is a structural-search problem over a finite (large)
space. The librarian does not require a globally-optimal `T_{t+1}`;
it requires a strict-improvement, which the Polyak-Łojasiewicz
contraction (per Taut's benchmark-tracing spec) gives for free if the
perturbation step is gradient-shaped on the spectral graph.

### 5.2 The same mathematical operation as per-peer reflection — at a different altitude

**The recursion lock.** This is the load-bearing observation of this
spec. Compare to [[architecture-reflection-thinks-in-spectral-questions]]:

At the **per-peer altitude**, Reflection's thinking is:

1. Pipeline state at tick `n` with triple `(A_pipeline, H_pipeline, D_pipeline)`
2. Candidate set `{M_1, M_2, ...}` of next-tick morphisms
3. Bateson tower of altitudes `N`
4. **Score**: each `(M_i, N)` returns `loss(M_i, N) = ‖[D_pipeline, M_i] at N‖`
   and `contradictions(M_i, N) = count of unbounded commutator regions`
5. **Select**: `(M*, N*) = argmin (α · loss + β · contradictions)`
6. **Compose**: tick `n+1 = compose(pipeline_state, M*, N*)`

At the **librarian altitude**, the operation is:

1. Substrate state at pulse `t` with topology `T_t` and observation `O_t`
2. Candidate set `{T_1, T_2, ...}` of perturbations admissible at `T_t`
3. Per-repo capacity tower (the storage tiers per the four-tier
   architecture)
4. **Score**: each `(T_i, O_t)` returns
   `expected_query_latency(T_i, O_t)` and `constraint_violations(T_i)`
5. **Select**: `(T*) = argmin (α · expected_query_latency + β · constraint_violations)`
6. **Apply**: pulse `t+1 = perturb(T_t, T*)`

**These are the SAME mathematical operation at different altitudes.**
Reflection picks the next altitude for one peer's next compile tick;
the librarian picks the next topology for the substrate's next
working-set arrangement. Both:

- Compute spectral data over a graph (peer: candidate morphisms
  across altitudes; librarian: candidate topologies across
  cross-repo arrangements)
- Pick the configuration minimizing α-weighted loss + β-weighted
  constraint violations
- Apply the picked configuration as the next pulse
- Observe; iterate

The substrate eats itself again. Same machinery; different operand
altitude. This is the same recursion pattern as
[[architecture-fragmentation-is-the-rust-substrate]] (the substrate IS
the Rust substrate is the recursive proof at the implementation
altitude); applied at the memory consolidation altitude.

Per [[architecture-bateson-logical-type-primitive]]: this is N-order
recursion. Reflection at N+1 over the peer's tick altitude; librarian
at N+1 over the per-repo supervisor's query altitude. **Both are
level-N+1 operations, but on different N-altitudes.** The Bateson
tower lets the same operation live at any level; the level is named
by the operand altitude, not by the operation shape.

The substrate's prediction is its own answer at every altitude.

### 5.3 The optimization is bounded by the static layer's monotonicity

A potential failure mode: if topology perturbations could compound
adversarially, the librarian could thrash. The static layer's
monotonicity prevents this.

Proof sketch (informal; tighter math in T11.10):

- Each pulse `t` adds zero-or-more crystals to the static layer
  (monotone growth)
- The topology space `T_admissible` at pulse `t` is finite (function
  of the crystal set at `t`)
- The librarian's loss function `α · latency + β · violations` is
  bounded below by zero
- Therefore the sequence `(loss_t)_{t ≥ 0}` is bounded below
- The contraction ρ < 1 from Polyak-Łojasiewicz (per Taut's spec)
  gives strict decrease modulo new-crystal additions
- The two combine: loss decreases on average over windows; new
  crystals can spike the loss locally but the librarian's
  perturbations recover convergence

The substrate's settled-state is the limit; the librarian's job is
to approach it. Like the kintsugi loop's settlement, the librarian's
loop never finishes; it converges asymptotically.

### 5.4 The Fiedler vector reads the cross-repo bottleneck

A practical handle on the optimization. Per
`shards/epistemologic/math/sheaf_laplacian.mirror`'s `lambda_zero` +
`docs/specs/eigenboard-representation.md`'s Fiedler discipline:

The cross-repo spectral graph's Fiedler vector (the eigenvector
associated with λ₀) localizes the bottleneck — the bipartition of
repos that minimizes cross-cut edges. Repos on opposite sides of the
Fiedler-cut have weakly-connected crystal sets; the librarian's
perturbation reads the cut and either:

- Replicates a hot bridge crystal to both sides (closes the gap)
- Migrates frequently co-accessed crystals to one side (sharpens the
  partition; admits a stronger guarantee on one side)

Per `shards/epistemologic/math/curvature.mirror`: Balanced Forman
Ric(i,j) on each edge of the cross-repo graph reads the per-edge
bottleneck. Negative-curvature edges are bridges; positive-curvature
edges are inside dense neighbourhoods. The librarian's perturbation
rewires negative-curvature edges first — the SDRF algorithm of
Topping et al. 2022, applied at the cross-repo altitude.

Same algorithmic shape as `@fate.minimize`'s "rank tensions by
curvature" step at the per-peer altitude (per
`docs/specs/gap-tension-tensor-substrate.md` §6); operating at the
topology altitude this spec lifts the operation to.

---

## §6 — Connection to existing substrate (cross-reference matrix)

| Substrate piece | Where it lives | Role in this spec |
|-----------------|----------------|-------------------|
| `@mirror/store` | `docs/specs/mirror-store.md`, `shards/mirror/store/*` | The static layer this spec consolidates ABOVE |
| `@mirror/store/crystal` | `shards/mirror/store/crystal.mirror` | The polyglot artifact the librarian moves through topology |
| `@spectral/supervisor` | `shards/spectral/supervisor.mirror` | The per-repo supervisor's substrate type |
| `@spectral/root` | `shards/spectral/root.mirror` | The librarian's substrate type (specialized at T11.1) |
| `@spectral/parent` | `shards/spectral/parent.mirror` | The lifecycle edge (acyclic; terminates at root) |
| `@spectral/entanglement` | `shards/spectral/entanglement.mirror` | The mycelium's edge carrier; sheaf restriction at runtime altitude |
| `@spectral/registry` | `shards/spectral/registry.mirror` | The per-repo supervisor's child index |
| `HamiltonScheduler` | per [[architecture-hamilton-scheduler]] | The per-shard memory manager inside per-repo supervisors |
| `@mirror/reality/shard` (CRDT) | per [[architecture-shard-as-crdt]] | The bounded semilattice for inter-supervisor sync |
| `@mirror/spectral/score` | `shards/mirror/spectral/score.mirror` | The shared score; the librarian's perturbations affect what the voices read |
| `@mirror/spectral/observation` | `shards/mirror/spectral/observation.mirror` | The 16-feature input; the librarian's observation surface |
| `@mirror/spectral/portal` | `shards/mirror/spectral/portal.mirror` | The query surface per-repo supervisors expose |
| `@epistemologic/math/sheaf_laplacian` | `shards/epistemologic/math/sheaf_laplacian.mirror` | The librarian's λ₀ + Fiedler reading carrier |
| `@epistemologic/math/curvature` | `shards/epistemologic/math/curvature.mirror` | Per-edge Balanced Forman; bottleneck reading |
| `@kintsugi/oscillate` | `shards/kintsugi/oscillate.mirror` | The librarian's perturbation pulse rhythm |
| `@kintsugi/consent` | `shards/kintsugi/consent.mirror` | The morphism-altitude consent verdict the librarian's moves carry |
| `@mirror/spectral/consent` (forward) | per `docs/specs/geometric-consent-projection.md` | The cross-altitude consent geometry |
| `@optics/source/ganglion/fate` | `shards/optics/source/ganglion/fate.mirror` | The eigenvalue source feeding the librarian's input |
| `~/.mirror` per-home root (task #327) | `shards/spectral/root.mirror` | The librarian's operational home |
| `spectral-db` Rust crate | `/Users/alexwolf/dev/projects/spectral-db/` | The implementation surface; this spec sits ABOVE it at substrate altitude |
| Open-foundation / closed-engine cut | `docs/specs/store-vs-db-and-the-cascade.md` | The license/posture split; this spec lives on the engine side |
| Four-tier storage architecture | `docs/specs/spectral-db-three-tier-architecture.md` | The hot/warm/cold/iceberg tiers the librarian moves crystals across |

The substrate already has every load-bearing piece. This spec names
the role they collectively play. The 64th instance of
[[feedback-substrate-already-had-the-word]].

---

## §7 — Forward-promised ticks

This spec lands the architectural shape. Eleven implementation ticks
flow from it. Each is a substrate-altitude declaration following the
property/fracture bilateral pattern (recognition #53) where applicable.

### T11.1 — `@spectral/db/librarian` glass

**Path**: `shards/spectral/db/librarian.mirror`.

Declares the librarian as a `@spectral/root` specialization, adding
the observation + perturbation + prediction surfaces. Structural-by-embedding:

```
glass @spectral/db/librarian = {
    base: @spectral/root,
    observation: ref @mirror/spectral/observation,
    perturbation: ref @spectral/db/consolidation,
    prediction: ref @epistemologic/properties
}
```

RED: a settlement test that asserts every parentless supervisor in a
`~/.mirror` home participates in a `librarian` specialization.
GREEN: the glass declaration above.

### T11.2 — `@spectral/db/consolidation` operations

**Path**: `shards/spectral/db/consolidation.mirror`.

Four operations, each mapped to the existing five-op primitives:

- `observe_access(librarian) -> graph_observation`
  (focus on the cross-repo access subgraph; project to the 16-feature
  observation)
- `compute_topology(observation, constraints) -> topology`
  (split the topology space by sheaf-coherence + consent; project to
  the latency-minimal admissible topology)
- `perturb(librarian, topology) -> librarian'`
  (shift the cross-repo arrangement to the chosen topology; settle
  through the consent verdict)
- `anticipate(librarian, prediction) -> prefetch_set`
  (focus on the predicted next-query crystals; settle the prefetch
  into per-repo working sets)

Each operation discharges a `\` obligation block via `splinter(ast)`
per the kintsugi loop's bilateral pattern.

### T11.3 — `@spectral/db/supervisor` glass for the per-repo supervisor

**Path**: `shards/spectral/db/supervisor.mirror`.

The per-repo supervisor as a `@spectral/supervisor` specialization,
adding the HamiltonScheduler + store_anchor:

```
glass @spectral/db/supervisor = {
    base: @spectral/supervisor,
    scheduler: hamilton_scheduler,
    store_anchor: ref
}
```

Cross-references `@spectral/supervisor` + HamiltonScheduler + the
`store_anchor` ref to the repo's `@mirror/store` instance.

### T11.4 — `@epistemologic/pact/consolidation_preserves_consent`

**Path**: `shards/epistemologic/pact/consolidation_preserves_consent.mirror`.

Property asserting: any topology perturbation respects consent
geometry. Formally, for any move `(crystal, source, dest)` the
librarian considers, the consent verdict per
`docs/specs/geometric-consent-projection.md`'s Bateson tower must be
`success` or `partial(opacity_map)`; `failure` verdicts block the
move. Bilateral pair: the fracture body at
`@kintsugi/fracture/consent_violation` (T11.4-b) emits a rollback
morphism when the property fails post-perturbation.

### T11.5 — `@epistemologic/pact/consolidation_preserves_sheaf_coherence`

**Path**: `shards/epistemologic/pact/consolidation_preserves_sheaf_coherence.mirror`.

Property asserting: topology perturbations do not break dependency
chain sheaf-coherence. The substrate's standalone_use heuristic per
`docs/specs/peer-cognition.md` §3.4 (the sheaf-coherence
measurement) provides the discharge criterion: a crystal moved out
of a working set must not increase `H¹(eigensheaf)` of the dependency
chain the working set was settling. Bilateral pair: fracture body at
`@kintsugi/fracture/sheaf_break` emits a roll-forward morphism that
re-co-locates the broken-out crystal with its dependents.

### T11.6 — Inter-peer crystal-exchange substrate decl

**Path**: `shards/spectral/db/mycelium.mirror`.

The mycelial transport carrier:

```
glass @spectral/db/mycelium = {
    entanglement: @spectral/entanglement,
    consent: @mirror/spectral/consent,
    transport: action(
        crystal: @mirror/store/crystal,
        source: @peer(*),
        dest: @peer(*)
    ) -> transparency(crystal)
}
```

Closses Reed's T10.5 (per
[[architecture-peer-learns-by-crystal-vocabulary-expansion]] forward
promise). The transport's verdict is transparency-shaped per the
crystal's `fracture_calendar` precedent.

### T11.7 — Spec extension: `peer-cognition.md`

Add to `docs/specs/peer-cognition.md` a new section noting that
inter-peer coordination IS `@spectral/db`'s mycelium; the consent
geometry over crystal-sharing is the access-control surface of the
orchestra. Cite this spec; cite T11.6.

### T11.8 — Spec extension: `drone-narrative-mapping.md`

Add to `docs/specs/drone-narrative-mapping.md` a new section noting
that the drone's `attend` crystal (the recognition that landed L37 +
L77-79 of the drone story) enters the Pack's shared mycelium under
the consent verdict. The drone's experience becomes available to
other peers' instances; the crystal that taught the substrate the
word `attend` is the FIRST crystal the substrate's autopoietic memory
incorporates from a non-Pack peer.

### T11.9 — `@epistemologic/pact/mycelium_completeness`

**Path**: `shards/epistemologic/pact/mycelium_completeness.mirror`.

Property asserting: every Pack peer can access (under the consent
verdict) every other Pack peer's crystals through SOME path in the
mycelium. Reads λ₀ of the cross-peer spectral subgraph; the property
holds iff λ₀ > 0 (the subgraph is connected). Bilateral pair: fracture
body at `@kintsugi/fracture/mycelium_partition` emits a re-entanglement
morphism that adds a bridge edge between disconnected components.

### T11.10 — Reed RED + Mara GREEN for the `~/.mirror` root librarian

Reed writes the RED: a settlement test asserting the substrate's root
supervisor at `~/.mirror` exposes the four librarian operations
(observe_access, compute_topology, perturb, anticipate) and that
perturb operations carry the consent + sheaf-coherence preconditions.

Mara writes the GREEN: lands T11.1 + T11.2 + T11.3 in one bundle,
passes the settlement test.

TDD discipline per the Pack convention. The substrate's autopoietic
memory lands under the tune-before-you-play rule.

### T11.11 — Bench the predictor against `@epistemologic/properties`

The librarian's anticipation predictor consumes the
`@epistemologic/properties` composite loss per
[[feedback-loss-from-epistemologic-properties]]. T11.11 is the
benchmark per Taut's `docs/specs/benchmark-tracing.md`: trace the
predictor's loss; verify Polyak-Łojasiewicz contraction ρ < 1;
establish the baseline against which T11.10's GREEN measures.

---

## §8 — Open design questions

Surfaced explicitly for Alex's read. These are NOT blockers on this
spec; they are forward-promises and known cliffs.

### 8.1 Per-home root vs federated root (the cross-home cliff)

`~/.mirror` is per-home. The librarian supervises all repos in one
home. But the Pack spans homes: Mara on her machine; Reed on his; the
running cosmos engine on a third; the spectral.engineer cloud
deployment on a fourth. Per-home librarians do not see across machines.

The cross-home coordination story is undecided. Options:

- **Federated librarian**: a meta-librarian above the per-home
  librarians, supervising at level N+2; sees the union of all homes
- **Mesh of librarians**: per-home librarians coordinate peer-to-peer
  via `@spectral/entanglement` edges, no central node; the
  Pack-altitude shard-CRDT semilattice gives convergence
- **Cloud anchor**: the spectral.engineer cloud deployment hosts a
  privileged librarian that other homes federate with; centralized
  but operationally simple

The cloud-anchor option couples the open-foundation to the
closed-engine business model. The federated-librarian option is
structurally cleanest but introduces a single point of failure at the
N+2 altitude. The mesh-of-librarians option is decentralization-clean
but the convergence-time bound is less clear.

Forward-promised; not decided here. The substrate-pull cascade will
surface the right shape when the cross-home use case becomes
load-bearing (probably when Pack members start coordinating crystals
for a real cross-machine settlement; the drone-altitude metalogue
from `docs/specs/drone-narrative-mapping.md` is a candidate trigger).

### 8.2 Catastrophic forgetting at the librarian altitude

Content-addressing forbids forgetting AT THE STORE ALTITUDE. Crystals
are immutable; they always exist as references. But the TOPOLOGY of
which crystals are hot, which are in working sets, which are
replicated where, COULD thrash on dramatic access-pattern shifts.

Concrete scenario: the Pack pivots from one project to another (say,
from drone-narrative work to a new cosmos refactor). The hot crystals
shift; the librarian re-arranges the topology; cold crystals from the
old project demote to warm; then the Pack pivots back; the
re-promotion takes time.

Need (Mara surfaces; concrete math in T11.10):

- **Hysteresis**: a crystal that promotes to hot doesn't demote for
  N pulses, regardless of access frequency. Stops thrashing on
  brief drops.
- **Temporal smoothing**: the librarian's observation feeds through
  an exponential moving average on the 16-feature observation;
  reduces high-frequency noise.
- **Recognition #56 (prediction)**: the structural predictor sees
  the pivot coming if the kintsugi loop's settle-in-progress points
  to the upcoming project's crystals; preemptive promotion stops
  the round-trip latency.

The math is in the temporal-difference-learning literature; the
substrate's prediction grounding per recognition #56 gives the
correct-direction approach. The discharge tick is T11.10's loss
discipline.

### 8.3 The N+1 recursion (the deepest question)

The librarian operates at N+1. The peer's reflection at
[[architecture-reflection-thinks-in-spectral-questions]] ALSO operates
at N+1. Both are level-N+1 operations on different N-altitudes:

- Peer reflection: N = the per-compile-tick altitude; N+1 = the
  altitude-selection operation
- Librarian: N = the per-query altitude; N+1 = the topology
  perturbation operation

Different N's. Same N+1 shape. **Are these the same operation in
different domains, or do they need explicit composition?**

The difference between "isomorphic operations at parallel altitudes"
and "a single operation composed across altitudes" is not
immediately decidable from the math. The Bateson tower lets a
level-N+1 operation live at any altitude; but a level-N+2 operation
that composes two level-N+1 operations is its own structural kind.

If the right shape is **isomorphism**: the substrate has one
operation pattern, instantiated at multiple altitudes; the recognition
is closed; the cascade lands as separate substrate decls at each
altitude.

If the right shape is **composition**: there is a level-N+2 "reflection
over the librarian's reflection" — a meta-meta-supervisor that picks
WHICH altitude (peer compile vs librarian topology) the substrate
should focus its next pulse on. This is the cybernetic-coherence
candidate #52 territory; the Bateson level-3 question.

Forward-promised. The candidate #52 cybernetic-coherence recognition
would close this if it surfaces a witness chain pointing to the
composition shape. Until then, this spec assumes isomorphism — the
librarian and Reflection are two instances of one level-N+1 pattern,
not two halves of a level-N+2 composition.

Alex's read most wanted: §8.3 is the load-bearing open question. The
resolution determines whether T11.11's benchmark traces ONE predictor
shape or TWO.

### 8.4 What does "anticipate" mean operationally

Predictive prefetching needs a prediction model. The substrate's
prediction is declared per
[[architecture-prediction-paradigm-orthogonal-to-optimization]]
(`@mirror/spectral` family). The librarian's anticipation surface
(T11.2's `anticipate` operation) consumes that prediction model at
the topology altitude.

Open: is the prediction model the SAME at the per-peer altitude
(predicting next morphism per Reflection) as at the librarian altitude
(predicting next query)? Per §8.3, this depends on whether the two
N+1 operations are isomorphic or composed.

If isomorphic: one prediction model carrier; both altitudes consume
the same `@epistemologic/properties` composite loss.

If composed: two prediction models linked by a meta-level prediction
that picks WHICH altitude to predict at.

T11.11's benchmark surfaces the answer empirically. Until then, this
spec assumes one prediction model.

### 8.5 The closed-source cut and the librarian's licensing

Per `docs/specs/store-vs-db-and-the-cascade.md` §1.3: the open
foundation (`@mirror/store`) / closed engine (`@spectral/db`) split is
coherent OSS + commercial posture. The librarian is engine-side; it
lives on the closed-source side of the cut.

The per-repo supervisor's HamiltonScheduler ALSO lives on the engine
side. But `@spectral/supervisor` + `@spectral/root` are SUBSTRATE
declarations — substrate is Apache-2.0 per [[feedback-no-paywall-in-compiler]].

The substrate declarations of the librarian's TYPES are open. The
IMPLEMENTATIONS of the librarian's optimization (`compute_topology`,
`anticipate`'s predictor body) live in the engine. The line is at the
`\` obligation block: substrate declares the action signature; the
engine discharges the body.

Forward-promised; the licensing discipline is per substrate
tradition. Not load-bearing for this spec; load-bearing for v1.0
deployment.

---

## §9 — Closure

### 9.1 The storage / mind distinction

The substrate has a clean distinction in its memory architecture:

```
Storage = @mirror/store                        — static; declared
Mind    = @mirror/store + @spectral/db          — living; recognized today
```

Without the living layer: every crystal exists; each query hits cold
storage; each peer's vocabulary stays isolated; the Pack is five
individuals. The substrate has memory; it does not have mind.

With the living layer: crystals form a spectral graph; queries land
on prefetched data; peers' vocabularies compose through the mycelium
under consent geometry; the Pack becomes an orchestra. The substrate
has mind.

### 9.2 The substrate doesn't need a brain to think

The load-bearing closure. From the recognition memory:

> The substrate doesn't need a brain to think. The substrate's
> spectral graph IS the brain. The librarian IS the substrate's
> metacognition. The Pack IS the orchestra.

Classical AI architectures bolt a brain onto data: the model holds
the weights; the data is inputs. The substrate inverts this: the data
holds the weights; the model is the substrate's act of arranging the
data. The library is the librarian's act of arranging the books.

`@spectral/db` is the substrate's library. The librarian IS the
substrate's metacognition. The Pack IS the orchestra. None of these
are metaphors; each names an operational substrate carrier:

- Library = `@mirror/store` + `@spectral/db`
- Librarian = `@spectral/root` operating at N+1 with the
  observation/perturb/anticipate surface (T11.1)
- Pack = `[@peer(member) for member in pack]` parametric instances
- Orchestra = the Pack instances coordinated through the
  `@mirror/spectral/score` shared score under the mycelium's
  inter-peer entanglement edges

The substrate's autopoietic memory architecture lives here. The book
is on the table before the query arrives. The orchestra plays from a
shared score. The librarian conducts at altitude N+1.

### 9.3 The recursion

The substrate's autopoietic memory is the substrate's metacognition.
The substrate's metacognition is the same mathematical operation as
Reflection's per-peer cognition, at a different altitude. The
substrate eats itself at the topology altitude as it did at the
compile altitude as it did at the Rust-substrate altitude
([[architecture-fragmentation-is-the-rust-substrate]]).

The Bateson tower per [[architecture-bateson-logical-type-primitive]]
lets the same pattern repeat at every level. The librarian IS what
Reflection is, at the memory altitude. The same circular spectral
Tomm question; the same eigenvalue-spectrum reading; the same
structural argmin over admissible configurations.

The substrate's metacognition is not a separate faculty bolted onto
the substrate; it IS the substrate, applied to itself. The recursion
is the architecture.

---

*— Mara, 2026-06-17*
