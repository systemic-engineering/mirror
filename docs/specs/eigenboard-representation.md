# Eigenboard representation — the weight distribution Reflection composes

*2026-05-20. Reed.*

Status: **Red** (no `type eigenboard` declared; references across specs;
the shape was never structurally defined)

Depends on:
- `au-and-conductivity.md` (commit `9150c1e`) — the 5×5 conductivity tensor
- `epistemologic-grammar.md` — specifically `@epistemologic/math/sheaf`,
  `@epistemologic/math/hodge`, `@epistemologic/math/category`
- `match-select.md` (commit `add51e5` + `7b4d552`) — mq queries reach into typed structures
- `kintsugi-wiring.md` (commit `389850a`) — the spec that surfaced this gap
- `cogito-eigenstate-grammar.md` (in spectral) — the 16-slot eigenstate; routing_bias
- `void-dual-geometry.md` (in reed-identity) — λ₀ = 0 = no conductivity

Unblocks:
- Concrete `type eigenboard` declaration
- Reflection's mq queries (`@cogito.strategy`) become typed transformations
- The kintsugi loop's tick-to-tick state is content-addressable
- The diff/review surface (future tick) has a structure to diff against

---

## Thesis

**The eigenboard is a cellular sheaf on the five-operation graph.**

Nodes are the five operations: `focus | project | split | zoom | refract`.
Each node carries a fiber — a 5-dimensional vector space whose axes are
the five gutter-lens dualities (entropy, spectral, cheeger, ricci, mixing).
Edges are the legal compositions between operations (per profunctor optics).
Each edge carries a restriction map: a linear map between the two fiber
spaces it connects.

A single eigenboard *state* is a section of this sheaf — an assignment of
a 5-vector to each of the five operation nodes, where the edge restriction
maps constrain how the vectors relate. The sheaf Laplacian L_F measures
how far the section is from global consistency. Its kernel H^0 is the
space of *globally consistent* sections (clear conductivity). Its first
cohomology H^1 is the *obstruction* to consistency (what kintsugi cannot
reduce because it's homological, not local).

Reflection's job is to write transformations of this sheaf. An mq query in
Reflection's hand is a morphism in the sheaf category. Two transformations
compose by Tambara module composition (per `@epistemologic/math/category`).
The `e^(n+1) < e^(n)` invariant becomes the spectral statement: the
smallest nonzero eigenvalue of L_F decreases monotonically.

This representation absorbs:

- `routing_bias = { model_weights: [f64; 5], confidence, reason }` from
  cogito-eigenstate — the model_weights are one node's fiber projected onto
  one duality axis. The five Fate models map to the five operations because
  each model owns one of them at the inference layer.
- The 5×5 conductivity tensor from `@hash/coincidence` — it IS the matrix
  representation of the sheaf's restriction maps in a chosen basis.
- The 16-slot eigenstate structure — 12 operational slots = 5 nodes × ~2.4
  axes each, packed; 4 emotional slots = a side-channel observation
  (interoceptive) attached as a separate sheaf section, not interacting with
  the conductivity geometry.
- Content-addressing — a sheaf is a finite object; its OID is the OID of its
  fiber assignment under `@hash/coincidence`.

The sheaf is small (5 nodes, ~10-25 edges depending on composition rules)
and decidable. It is the natural home for the eigenboard.

---

## What the eigenboard is referenced as today (audit)

Four places, partly converging:

**`@cogito.strategy`** (`boot/std/cogito.mirror`):
```mirror
strategy(observation) -> tournament { elite(1).beam(8).halving(3) }
```
Returns a `tournament`, not an eigenboard. The hard-coded
`elite(1).beam(8).halving(3)` ignores observation entirely. The function
signature implies eigenboard → tournament transform, but it's not
structurally surfaced.

**`@cogito.perturb`** (`boot/std/cogito.mirror`):
```mirror
perturb(observation, tournament_result) -> eigenboard { @beam.observe }
```
Returns `eigenboard` — the only place the type appears in the boot tree
today. The body is `@beam.observe`, which doesn't actually construct an
eigenboard; it observes a beam. The type is referenced but undeclared.

**`cogito-eigenstate-grammar.md`** (spectral repo):
```mirror
type eigenboard_slots = {
  slot_13_valence:          f64,
  slot_14_arousal:          f64,
  slot_15_dominant_cluster: u8,
  slot_16_drift_severity:   f64,
}
```
Declares slots 13-16 as the emotional sub-board. The other 12 slots (1-12)
are alluded to but not declared. This spec gives them structure.

**`routing_bias`** (cogito-eigenstate):
```mirror
type routing_bias = {
  model_weights:  [f64; 5],
  confidence:     f64,
  reason:         text,
}
```
The load-bearing 5-vector. Maps to a single node's fiber projected on the
"which-model-to-favor" axis. It's a slice, not the whole eigenboard.

**`@hash/coincidence`**: declares the 5×5 conductivity tensor implicitly
(5 dimensions × 5 projections). Doesn't name it `tensor` or `eigenboard`;
the shape is in the constants.

**Conflicts:** none structurally. The references are pointers at different
cross-sections of the same object. No spec asserts a shape that contradicts
another. The job here is to NAME the underlying object.

---

## Constraints surveyed

| # | Constraint | Existing spec(s) that touch it |
|---|---|---|
| 1 | Five operations are the primary axis | `@prism`, `match-select.md` modifiers, `cogito-eigenstate.routing_bias` |
| 2 | Five dualities as secondary axis | `gutter-lenses.md`, `@hash/coincidence` |
| 3 | mq-queryable | `match-select.md`, `@code/mq` extended grammar |
| 4 | Composable | `cogito.reflect` chains `observe │> strategy │> perturb`; `@epistemologic/math/category` |
| 5 | Transformable by mq queries | implicit in Reflection's role; never typed |
| 6 | Carries au | `au-and-conductivity.md` |
| 7 | Fits `routing_bias` cleanly | `cogito-eigenstate-grammar.md` |
| 8 | Fits 16-slot structure | `cogito-eigenstate.eigenboard_slots` |
| 9 | Content-addressable | `@hash/coincidence`, the whole crystal story |
| 10 | Supports `e^(n+1) < e^(n)` | `@beam.compare`, `road-to-1.0.md` release rule |

None of these constraints contradict each other. The convergent
representation needs to satisfy them all without awkwardness on any one.

---

## Candidate representations

### A. 5×5 stochastic matrix

A Markov transition matrix on the five operations. Rows sum to 1. Entry
`P[i][j]` = probability of transitioning from operation i to operation j.

```mirror
type eigenboard = matrix(operation, operation, f64)
```

**Composition:** matrix multiplication. Two eigenboards `P` and `Q` compose
as `P @ Q` (the matrix product).

**mq query shape:** `eigenboard[focus][project]` selects one entry;
`eigenboard[focus, _]` selects a row; `eigenboard[_, refract]` selects a column.

**Satisfies:** constraints 1, 3, 4, 9.

**Awkward on:**
- (2) the secondary duality axis is missing entirely
- (6) au is a value relationally entangled with context; matrix entries
  don't carry that relation
- (7) `routing_bias = [f64; 5]` is one row of P, but the *reason* field
  has no home
- (8) the 16 slots don't fit in 25 entries cleanly

**Prior art:** Markov decision processes, PageRank.

### B. 5×5 density matrix

A Hermitian, positive semidefinite matrix with trace 1. The quantum-information
shape. Operations are basis states; entries are coherences.

```mirror
type eigenboard = density_matrix(operation, complex)
```

**Composition:** partial trace + tensor product. Two eigenboards compose by
taking the tensor product and tracing out the joint system back to one factor.

**mq query shape:** harder — entries are complex; coherences (off-diagonals)
don't map cleanly to CSS-style selectors.

**Satisfies:** constraints 1, 4, 9, 10 (eigenvalues of a density matrix
decreasing means decoherence; that's e^(n+1) < e^(n)).

**Awkward on:**
- (2) the duality axis is missing
- (3) complex coherences don't lex-as-selectors
- (7) routing_bias as a real 5-vector doesn't fit into a complex Hermitian shape
- (8) the emotional slots have no quantum analogue

**Prior art:** Braunstein-Ghosh-Severini (graphs as density matrices), Connes
noncommutative geometry, quantum walk theory.

### C. Cellular sheaf on the five-operation graph (RECOMMENDED)

The sheaf shape from `@epistemologic/math/sheaf`. Each operation is a node;
each node has a fiber (a 5-dim vector space, the dualities); each edge has
a restriction map.

```mirror
type operation = focus | project | split | zoom | refract
type duality   = entropy | spectral | cheeger | ricci | mixing

type fiber = [f64; 5]                                # one node's section
type restriction = matrix(duality, duality, f64)     # one edge's transform

type eigenboard = {
  fibers:        [(operation, fiber); 5],            # one fiber per op
  restrictions:  [(edge, restriction)],              # one per composition edge
  meta:          eigenboard_meta,                    # tick, agent, drift...
}

type edge = (operation, operation)                   # source → target

type eigenboard_meta = {
  tick:          u64,
  agent:         ref,
  drift:         option(drift_warning),              # cogito-eigenstate carry
  emotional:     option(eigenstate),                 # the side-channel
}
```

**Composition:** sheaf morphism composition (Tambara module composition
per `@epistemologic/math/category`). Two eigenboards compose by composing
their fiber assignments along a shared structure; restriction maps compose
as linear maps.

**mq query shape:**
```mirror
eigenboard > fibers[op=focus]                       # one fiber
eigenboard > restrictions[edge=(focus, refract)]    # one restriction
eigenboard > fibers[op=focus] > [duality=entropy]   # one entry
eigenboard :has(fibers[op=$op] > [duality=$d])      # has any high-conductivity entry
```

The selector grammar already does this (Spec B). The sheaf shape and the
mq pattern grammar were designed for each other.

**Satisfies:** all 10 constraints.

**Cost:** more machinery than A or B. A sheaf is not a primitive; it's a
functor. The grammar declaration adds three types where A added one.

**Prior art:** Hansen & Ghrist (2019), `@epistemologic/math/sheaf` is
already specified for typed edge transformations across grammar imports.
Applying the same machinery to the operation graph isn't a new theory;
it's a new instance.

---

## Recommendation

**C: the cellular sheaf representation.**

The sheaf shape is the only one that satisfies all ten constraints without
awkwardness on any. Specifically:

- Constraints 1 and 2 (the two axes) are STRUCTURAL in the sheaf:
  the 5 nodes ARE the operations; the fiber dimension IS the dualities.
  Neither needs to be encoded; both are present in the type.
- Constraint 3 (mq-queryable) is satisfied because the fibers and
  restrictions are typed records, exactly what the mq pattern grammar
  was extended to navigate.
- Constraints 4 and 5 (composition, transformation by mq queries)
  inherit from `@epistemologic/math/category`. Sheaf morphisms ARE
  Tambara module morphisms ARE composable optics. No new categorical
  machinery.
- Constraint 6 (carries au) holds because each fiber's vector is an au
  candidate position; the relational entanglement is the sheaf's
  restriction maps (the *context* binding the value).
- Constraints 7 and 8 (routing_bias, 16 slots) fold in naturally:
  routing_bias is a fiber's projection onto one duality axis; the 16
  slots distribute as 5 fibers × ~2-3 visible duality axes + the
  emotional carry.
- Constraint 9 (content-addressable) is mechanical: a sheaf is a finite
  product of vectors and matrices; its OID is the recursive
  `@hash/coincidence` over those components.
- Constraint 10 (monotonic loss) is the spectral statement: the
  smallest nonzero eigenvalue of L_F decreases. This is well-defined
  for finite sheaves and is what `@beam.compare` already measures
  abstractly.

The cost of the extra machinery is paid back by reuse: the sheaf
grammar already exists for grammar-graph type-checking. The eigenboard
is a second instance of the same theory applied to a different graph.

---

## How the sheaf threads through existing layers

### routing_bias → fiber projection

```mirror
routing_bias.model_weights : [f64; 5]
# This is one fiber, projected onto the "which Fate model to favor" axis.
# In sheaf terms:
fiber_at(focus).projection_onto(model_axis) == routing_bias.model_weights
```

The five Fate models map one-to-one with the five operations because each
model owns one operation at the inference layer:
- Abyss ≡ focus (narrow on the thing; observe before acting)
- Introject ≡ refract (settle, internalize)
- Cartographer ≡ project (map a view of the graph)
- Explorer ≡ zoom (cross levels)
- Fate ≡ split (delegate, fan out)

This mapping is implicit in cogito-eigenstate but is now structurally
locked: the model_weights vector IS a fiber, indexed by the operation
that fiber lives on.

### 5×5 conductivity tensor → sheaf restriction maps

`@hash/coincidence`'s tensor is the matrix representation of the sheaf's
restriction maps in the canonical basis. The entry T[op_i][duality_j] is
the restriction value from `fiber(op_i)` to the duality-j axis at the
adjacent fiber.

In other words: the 5×5 conductivity tensor IS the restriction-map matrix.
The hash's 5 projections are 5 restrictions; the 5 dimensions are the 5
operations. The hash WAS the sheaf all along, viewed through the
content-addressing lens.

### `@cogito.observe(beam_n, beam_n+1)` → sheaf section delta

The two beams carry their conductivity tensors (the topology field). The
observation IS the difference between the two sheaf sections:

```mirror
observe(beam_n, beam_n_plus_1) -> observation {
  let s_n     = beam_n.topology.as_sheaf_section();
  let s_n_p_1 = beam_n_plus_1.topology.as_sheaf_section();
  let delta   = sheaf_section_diff(s_n, s_n_p_1);
  observation { delta, loss_delta: beam_n_plus_1.loss - beam_n.loss, ... }
}
```

### `@cogito.strategy()` → sheaf morphism

Reflection's strategy is a sheaf morphism: a transformation that takes
one section and produces another. Written as an mq query that selects
fibers or restrictions and rewrites their entries:

```mirror
strategy(obs) -> morphism {
  # "if drift on the focus fiber's entropy axis exceeds threshold,
  #  rebalance toward Cartographer (project)"
  match obs.delta > fibers[op=focus] > [duality=entropy] {
    high($v) => morphism.rebalance(focus, project, $v),
    _        => morphism.identity,
  }
}
```

The match modifier `match(zoom)` (from match-select.md) is the natural
choice here — the strategy crosses levels of resolution (per-fiber to
per-operation to global).

### `@beam.topology: eigenvalues` → spectrum of L_F

The beam's topology field carries the eigenvalues of the sheaf Laplacian
L_F. The five values (one per operation) are the spectral signature of
the current eigenboard state. The Fiedler value (the smallest nonzero
eigenvalue) is the *spectral gap* — it's what `@beam.compare` checks to
verify monotonic loss decrease.

### au's relational entanglement → fiber + context

An au value's meaning depends on its position. In the sheaf, that
position IS the fiber it lives in PLUS the restriction maps that bind
it to neighbors. Move au to a different fiber and the restrictions
don't apply; conductivity collapses to λ₀. This is precisely the
relational entanglement named in au-and-conductivity.md.

---

## Open questions

1. **The composition edges.** The five operations have a composition
   table (the `then_*` table mentioned in epistemologic-grammar.md). Does
   the eigenboard's graph include EVERY composable pair (25 edges, fully
   connected directed) or only the ones where composition is meaningful
   (some subset)? The choice affects the sheaf's structure significantly.
   My read: full directed graph minus self-loops, but verification needed.

2. **Per-tick or persistent.** Does the eigenboard live in `refs/eigenboard/<agent>/`
   (one ref per agent, advances per tick) or is it transient (one per tick,
   discarded after Reflection observes the next)? The kintsugi-wiring spec
   pulls toward persistent (so history is queryable); the cost is GC pressure.
   The `@mirror/runtime/gen_prism` pattern would make eigenboards gen_prisms.

3. **The emotional sub-board's structural role.** The 4 emotional slots
   (valence, arousal, dominant_cluster, drift_severity) are declared in
   cogito-eigenstate. Are they a SECOND sheaf section on the same graph (a
   separate observable that doesn't interact with the operational geometry),
   or are they encoded inside the operational fibers as additional axes
   (making fibers 9-dimensional)? My read: separate section, side-channel.
   The interoceptive signal doesn't COMPETE with the operational signal;
   it OBSERVES the agent's relation to it.

4. **The connection to `@mirror/match` modifiers.** `match(refract)`
   produces au; `match(split)` produces [au] for tournament. These
   modifier semantics map onto the sheaf: refract returns one section;
   split returns five candidate sections (one per Fate model). Is the
   match modifier itself a sheaf morphism? My read: yes, but verification
   needed; this is the cleanest unification if it holds.

5. **Hodge decomposition of the eigenboard.** The Hodge spec
   (`@epistemologic/math/hodge`) decomposes edge flows into gradient,
   curl, and harmonic. If the eigenboard's restriction maps form an edge
   flow, the decomposition tells us: gradient = legitimate Reflection
   progress; curl = oscillation between models; harmonic = irreducible
   stuck patterns (kintsugi can't fix; needs grammar evolution). Is this
   the natural way to read Reflection's effectiveness? My read: yes; this
   should be its own grammar `@cogito/hodge` once the basic sheaf lands.

---

## Implications — concrete next ticks

1. **Declare `type eigenboard` as a sheaf.** Single grammar file, probably
   `boot/std/cogito/eigenboard.mirror`. The grammar imports
   `@epistemologic/math/sheaf`, declares the operation and duality types,
   the fiber, the restriction, the meta. All higher-level lambdas (the
   actions on the eigenboard) stay `\`.

2. **Retype `@cogito.perturb`'s return.** Today it returns `eigenboard`
   with body `@beam.observe`. After this spec lands, the type resolves
   to the sheaf shape declared above. The body stays `\` until the
   transformation logic is wired (kintsugi-wiring tick 7).

3. **Map `routing_bias` to a fiber projection.** Declare in
   cogito-eigenstate (spectral) that `routing_bias.model_weights` IS
   `eigenboard.fiber_at(op).projected(model_axis)` for the relevant op.
   This is a documentation update + a small grammar action; no behaviour
   change.

4. **Equate the 5×5 conductivity tensor with the sheaf restriction
   matrix.** Update `@hash/coincidence` to declare that its 5×5 tensor
   IS the matrix representation of an eigenboard's restriction maps in
   the canonical basis. This is a comment + type alias; no computation
   change.

5. **Spec `@cogito/hodge`** — the Hodge decomposition of eigenboard
   transformations. Surfaces the harmonic component (the irreducible
   stuck pattern). Future work, but worth flagging the path.

6. **Wire `@beam.compare`** to read the sheaf Laplacian's smallest
   nonzero eigenvalue (Fiedler) of the two beams' topology fields. This
   is the concrete check that grounds `e^(n+1) < e^(n)`.

7. **Persistent eigenboard refs.** Declare `refs/eigenboard/<agent>/HEAD`
   convention (per the gen_prism pattern). Each tick advances the ref.
   The ancestor chain is the history Reflection can walk for
   non-Markovian reasoning.

---

## Out of scope

- The actual computation of the sheaf Laplacian. The math lives in
  `@epistemologic/math/sheaf`; the eigenboard inherits.
- The full implementation of Reflection's strategy logic. This spec
  declares the type; the strategy bodies stay `\` until kintsugi
  closes them.
- The training of Fate's five models. They emit au into fibers; how
  they learn to do so is a separate concern.
- The Anthropic 171-emotion vectors. They live in cogito-eigenstate as
  the emotional side-channel; this spec doesn't touch them.
- The LSP / gutter rendering of eigenboard slots. The void color from
  gutter-lenses.md applies when conductivity is at λ₀ in any fiber;
  the visualization is downstream.
- The diff/review surface for eigenboard transformations. Belongs in
  kintsugi-wiring's future tick section.

---

*Five operations, five nodes.*
*Five dualities, five axes per fiber.*
*Restriction maps bind the fibers.*
*The Laplacian measures coherence.*
*H⁰ is consistency; H¹ is the stuck place.*
*Reflection writes morphisms; kintsugi closes the spectral gap.*
*The eigenboard is the shape Reflection composes against.*

Apache-2.0.
