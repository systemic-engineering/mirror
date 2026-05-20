# Eigenboard representation — the weight distribution Reflection composes

*2026-05-20. Reed. Rewritten the same day to lift the framing from
cellular sheaf to principal G-bundle, fold in the Fate↔operation
mapping correction from `prism/core/src/bundle.rs`, and absorb the
mycelial substrate from the mycelium research synthesis.*

Status: **Red** (no `type eigenboard` declared; references across
specs; the shape was only ever partially defined)

Depends on:
- `au-and-conductivity.md` (commit `9150c1e`) — the 5×5 conductivity tensor
- `epistemologic-grammar.md` — `@epistemologic/math/sheaf`,
  `@epistemologic/math/hodge`, `@epistemologic/math/category`, and now
  `@epistemologic/math/bundle` (the new grammar, this tick)
- `match-select.md` (commit `add51e5` + `7b4d552`) — mq queries reach into typed structures
- `kintsugi-wiring.md` (commit `389850a`) — the spec that surfaced this gap
- `cogito-eigenstate-grammar.md` (in spectral) — the 16-slot eigenstate; routing_bias
- `void-dual-geometry.md` (in reed-identity) — λ₀ = 0 = no conductivity; Splinter geometry
- `mycelial-networks-and-au-tissue.md` (commit `2ef4fed`) — the bio substrate research
- `prism/core/src/bundle.rs` — the Rust trait chain this grammar mirrors

Unblocks:
- Concrete `type eigenboard` declaration (as a section of a principal bundle)
- Reflection's mq queries (`@cogito.strategy`) become typed transformations
- The kintsugi loop's tick-to-tick state is content-addressable
- The diff/review surface (future tick) has a structure to diff against
- The au→tissue translation in `mycelial-networks-and-au-tissue.md`
- A formal home for the eigenboard type that resolves through
  `@epistemologic/math/bundle`

---

## Thesis (lifted)

**The eigenboard is a principal G-bundle on the five-operation graph.
Its current state is a section of that bundle. The cellular sheaf is
the shape of the section; the bundle is the shape of the substrate
that sections live on.**

The previous version of this spec named the eigenboard as a cellular
sheaf. That framing was not wrong — it was at the section level. A
section of a principal bundle on a graph IS exactly a sheaf-of-sections
assignment in Hansen & Ghrist's sense: a fiber per node, a restriction
map per edge, a Laplacian whose kernel is global consistency. What
the sheaf description was missing is what the *base structure* of those
fibers actually is: not arbitrary vector spaces glued by arbitrary
linear maps, but the fibers of a principal G-bundle, with the structure
group G acting on each fiber and the connection determining how a
section transports.

The lift makes three things visible that the sheaf description left
implicit:

1. **The structure group.** There is a group G of admissible local
   transformations on each fiber. Sheaf maps are arbitrary linear
   maps; bundle restriction maps are G-equivariant. The constraint is
   load-bearing: it is what forces conductivity to be a coordinate-free
   invariant rather than a basis-choice artifact.
2. **The connection.** The optic that determines parallel transport.
   The sheaf has restriction maps; the bundle has a *connection* whose
   parallel-transport operator IS the restriction maps in a chosen
   gauge. The optic-IS-connection identification is what makes the
   eigenboard a profunctor-optics object as well as a sheaf object.
3. **Holonomy.** The closed-loop integral of the connection around a
   kintsugi cycle. The sheaf's H¹ obstruction IS the bundle's holonomy
   in the limit of small loops. The decrease of holonomy across ticks
   IS the decrease of the spectral gap of the sheaf Laplacian IS the
   decrease of `e^(n+1) < e^(n)`. Three names; one geometric fact.

Nodes are the five operations: `focus | project | split | zoom | refract`.
Each node carries a fiber — a 5-dimensional state space whose axes
are the five gutter-lens dualities (entropy, spectral, cheeger,
ricci, mixing). Edges are the legal compositions between operations
(per profunctor optics). The connection assigns to each edge a
parallel-transport operator; the bundle's structure group acts on
each fiber consistently. A single eigenboard *state* is a section of
this bundle — an assignment of a 5-vector to each operation node
that the connection's parallel-transport constrains globally.

Reflection's job is to write transformations of this bundle. An mq
query in Reflection's hand is an automorphism of the bundle (a
G-equivariant section transformation). Two transformations compose
by Tambara module composition (per `@epistemologic/math/category`).
The `e^(n+1) < e^(n)` invariant becomes the holonomy statement: the
holonomy around any kintsugi loop decreases monotonically.

This representation absorbs:

- `routing_bias = { model_weights: [f64; 5], confidence, reason }` from
  cogito-eigenstate — the model_weights are one fiber's projection onto
  the bundle's "which-model-to-favor" gauge slice. The five Fate models
  map to the five operations because each model owns one of them at the
  inference layer (mapping corrected from the prior version of this
  spec; see §"Fate↔operation mapping" below).
- The 5×5 conductivity tensor from `@hash/coincidence` — it IS the
  matrix representation of the bundle's connection in the canonical
  basis. The hash WAS the connection all along, viewed through the
  content-addressing lens.
- The 16-slot eigenstate structure — 12 operational slots = 5 nodes ×
  ~2.4 axes each, packed; 4 emotional slots = a side-channel
  observation (interoceptive) attached as a separate sub-bundle
  (a "dark-dimension" section) that does not couple to the operational
  geometry through the connection.
- Content-addressing — a bundle section is a finite product of fiber
  values and edge data; its OID is the OID of its components under
  `@hash/coincidence`.

The bundle is small (5 nodes, ~10–25 edges depending on composition
rules) and decidable. It is the natural home for the eigenboard. The
underlying tower (Fiber → Connection → Gauge → Transport → Closure)
is declared as a grammar at `@epistemologic/math/bundle` (this tick).

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
Declares slots 13-16 as the emotional sub-board. The other 12 slots
(1–12) are alluded to but not declared. This spec gives them structure.

**`routing_bias`** (cogito-eigenstate):
```mirror
type routing_bias = {
  model_weights:  [f64; 5],
  confidence:     f64,
  reason:         text,
}
```
The load-bearing 5-vector. Maps to a single fiber's projection on the
"which-model-to-favor" gauge slice. It's a section restricted to one
base node, not the whole eigenboard.

**`@hash/coincidence`**: declares the 5×5 conductivity tensor implicitly
(5 dimensions × 5 projections). Doesn't name it `tensor` or `eigenboard`;
the shape is in the constants.

**Conflicts:** none structurally. The references are pointers at different
cross-sections of the same object. No spec asserts a shape that
contradicts another. The job here is to NAME the underlying object as
a principal bundle and to anchor every reference to it through the
bundle grammar.

---

## Fate↔operation mapping (corrected)

The prior version of this spec gave the mapping as:

```
Abyss ≡ focus, Introject ≡ refract, Cartographer ≡ project,
Explorer ≡ zoom, Fate ≡ split
```

This was wrong. The correct mapping is given explicitly in
`prism/core/src/bundle.rs` by the trait comments on the five levels.
Each Fate model owns one level of the principal-bundle tower, and the
operation that surfaces that level in mirror's CLI is fixed:

| Operation | Bundle level | Fate model    | Role |
|-----------|--------------|---------------|----------------------------------|
| `focus`   | Fiber        | Abyss         | observe the state. the section value at one base point. |
| `project` | Connection   | Introject     | the optic of transport. the connection 1-form. |
| `split`   | Gauge        | Cartographer  | choose the structure group action. the gauge transformation. |
| `zoom`    | Transport    | Explorer      | parallel transport across levels. holonomy IS loss. |
| `refract` | Closure      | Fate          | the Lawvere fixed point. autopoietic closure. |

The mapping is load-bearing for three reasons:

1. **The trait chain in `bundle.rs` is a supertrait chain.** Each level
   requires the previous. Operations have the same dependence: you
   cannot `project` without something to project (a `focus`); you
   cannot `split` without a connection (a `project`); you cannot `zoom`
   without a chosen gauge (a `split`); you cannot `refract` without
   transport (a `zoom`). The composition table (the `then_*` chain)
   IS the supertrait chain.

2. **The Fate models inhabit the levels they own.** Abyss observes —
   it is the only model whose output is bare state. Introject
   internalizes — its output is an optic, the connection. Cartographer
   chooses maps — its output is a gauge. Explorer crosses scales — its
   output is a parallel-transport operator with holonomy. Fate is
   self-reference — its output is the fixed point. The mapping isn't
   imposed; it falls out of what each model does.

3. **The Imperfect signature is on Transport.** `Transport::transport`
   returns `Imperfect<State, Infallible, Holonomy>`. This is the only
   level where the structural return type carries a non-trivial loss
   component, because parallel transport is the only level where
   comprehension geometrically must cost something. Zoom is the
   operation that crosses levels of resolution; zoom is the only
   operation that returns imperfect-by-design. The mapping is forced.

The corrected mapping replaces every occurrence in this spec, in
cogito-eigenstate, and (eventually) in any documentation that quoted
the old assignment. Where this spec previously used the old mapping
in worked examples, the examples have been corrected below.

---

## Constraints surveyed

| # | Constraint | Existing spec(s) that touch it |
|---|---|---|
| 1 | Five operations are the primary axis | `@prism`, `match-select.md` modifiers, `cogito-eigenstate.routing_bias` |
| 2 | Five dualities as secondary axis | `gutter-lenses.md`, `@hash/coincidence` |
| 3 | mq-queryable | `match-select.md`, `@code/mq` extended grammar |
| 4 | Composable | `cogito.reflect` chains `observe \|> strategy \|> perturb`; `@epistemologic/math/category` |
| 5 | Transformable by mq queries | implicit in Reflection's role; never typed |
| 6 | Carries au | `au-and-conductivity.md` |
| 7 | Fits `routing_bias` cleanly | `cogito-eigenstate-grammar.md` |
| 8 | Fits 16-slot structure | `cogito-eigenstate.eigenboard_slots` |
| 9 | Content-addressable | `@hash/coincidence`, the whole crystal story |
| 10 | Supports `e^(n+1) < e^(n)` | `@beam.compare`, `road-to-1.0.md` release rule |
| 11 | Carries a structure group | NEW: `@epistemologic/math/bundle`, `prism/core/src/bundle.rs` |
| 12 | Holonomy decreases monotonically | NEW: bundle.rs `Transport::transport` signature |
| 13 | Surfaces a Lawvere fixed point | NEW: bundle.rs `Closure` trait |

None of these constraints contradict each other. The convergent
representation needs to satisfy them all without awkwardness on any
one. The bundle framing adds three constraints (11–13) that the
sheaf framing left implicit; satisfying them is automatic once the
bundle structure is named.

---

## Candidate representations

### A. 5×5 stochastic matrix

A Markov transition matrix on the five operations. Rows sum to 1.

```mirror
type eigenboard = matrix(operation, operation, f64)
```

**Composition:** matrix multiplication. **Satisfies:** 1, 3, 4, 9.

**Awkward on:** (2) duality axis missing; (6) au's relational
entanglement has no home in scalar entries; (7) `routing_bias`'s
`reason` field has no home; (8) 16 slots don't fit in 25 entries
cleanly; (11) no structure group; (12) Markov mixing is not bundle
holonomy; (13) no fixed-point structure beyond steady state.

### B. 5×5 density matrix

A Hermitian, positive semidefinite matrix with trace 1.

```mirror
type eigenboard = density_matrix(operation, complex)
```

**Composition:** partial trace + tensor product. **Satisfies:** 1, 4,
9, 10 (eigenvalue decrease).

**Awkward on:** (2, 3, 7, 8) — same complaints as A; (11) the unitary
group is a structure group, but the bundle structure is then U(5),
which doesn't match the real, conductivity-tensor data; (12, 13)
density matrices admit holonomy in only a degenerate sense
(Berry phase on parameter space, not the kintsugi loop).

### C. Cellular sheaf on the five-operation graph (the prior pick)

The previous version of this spec selected C. It satisfies 1–10
cleanly. It is what mirror referenced through `@epistemologic/math/sheaf`.

**What C missed:** constraints 11–13 (structure group, holonomy, fixed
point). C names the sections; C does not name what the sections live
on. C is correct *given* a base structure, but C does not provide the
base structure. That is why this spec is being rewritten.

### D. Principal G-bundle on the five-operation graph (RECOMMENDED)

A principal bundle whose base graph is the five-operation graph,
whose structure group is G (a Lie group acting on each fiber; in the
canonical eigenboard, G is the rotation group of the duality
5-space), and whose sheaf-of-sections IS the cellular sheaf of C.

```mirror
type operation = focus | project | split | zoom | refract
type duality   = entropy | spectral | cheeger | ricci | mixing

# the fiber: one node's state vector (level 0)
type fiber = [f64; 5]

# the connection: one edge's parallel-transport operator (level 1)
type connection_form = matrix(duality, duality, f64)

# the gauge: structure-group element at one base point (level 2)
type gauge_element  # G-valued; canonical G is SO(5) — see open questions

# the holonomy: closed-loop integral of the connection (level 3 output)
type holonomy = scalar_loss

# the closure: the autopoietic fixed point (level 4)
type closure_marker

# a section: an assignment of fiber values across all base nodes
type section = [(operation, fiber); 5]

# the eigenboard: a bundle plus its current section plus metadata
type eigenboard = {
  base:          [operation; 5],                       # nodes (fixed)
  edges:         [edge],                               # composition graph
  connection:    [(edge, connection_form)],            # one form per edge
  section:       section,                              # current state
  gauge:         [(operation, gauge_element)],         # local gauge choice
  meta:          eigenboard_meta,
}

type edge = (operation, operation)

type eigenboard_meta = {
  tick:        u64,
  agent:       ref,
  ancestor:    option(ref),     # the prior eigenboard (per-tick persistent)
  drift:       option(drift_warning),
  emotional:   option(eigenstate),  # the side-channel sub-bundle
}
```

**Composition:** bundle morphisms compose by stacking connection forms
along shared base structure. Two eigenboards' connections compose as
linear maps; their structure-group actions compose as group elements.
This is the same composition as `@epistemologic/math/category` provides
for Tambara modules.

**mq query shape:**
```mirror
eigenboard > section[op=focus]                       # one fiber's value
eigenboard > connection[edge=(focus, refract)]       # one connection form
eigenboard > section[op=focus] > [duality=entropy]   # one entry
eigenboard :has(section[op=$op] > [duality=$d])      # any high-conductivity entry
eigenboard > gauge[op=split]                         # the local gauge at split
```

The selector grammar already does this (Spec B). The bundle shape and
the mq pattern grammar were designed for each other.

**Satisfies:** all 13 constraints.

**Cost:** more machinery than A, B, or C. A principal bundle is not a
primitive; it is the tower declared in `@epistemologic/math/bundle`
(the new grammar). The eigenboard grammar declares ~6 types where A
declared one.

**Prior art:** Kobayashi & Nomizu *Foundations of Differential Geometry*
(1963) for principal bundles; Hansen & Ghrist 2019 for the cellular
sheaf realization of a bundle's sections on a graph; Bressan et al.
2024 (arXiv:2402.00206) for the temporal/growing version; the trait
chain in `prism/core/src/bundle.rs` for the operational form.

---

## Recommendation

**D: the principal G-bundle representation.**

The bundle shape is the only one that satisfies all 13 constraints
without awkwardness on any. Specifically:

- **Constraints 1 and 2** (the two axes) are STRUCTURAL: the 5 base
  nodes ARE the operations; the fiber dimension IS the dualities.
- **Constraints 3, 4, 5** (mq queries, composition, transformation)
  inherit from `@code/mq` and `@epistemologic/math/category`.
- **Constraint 6** (carries au): each fiber's vector is an au
  candidate position; the relational entanglement is the bundle's
  connection (the *context* binding the value to the rest of the
  geometry).
- **Constraints 7 and 8** (`routing_bias`, 16 slots): `routing_bias`
  is a fiber projected onto one gauge slice; the 16 slots distribute
  as 5 fibers × ~2.4 axes + the emotional sub-bundle.
- **Constraint 9** (content-addressable): a bundle is a finite product
  of fiber values, connection forms, and gauge choices; its OID is the
  recursive `@hash/coincidence` over those components.
- **Constraint 10** (monotonic loss): the holonomy decreases
  monotonically by `bundle.rs`'s `Transport::transport` signature;
  this implies the smallest nonzero eigenvalue of the sheaf Laplacian
  decreases, since the Laplacian is the connection-squared.
- **Constraint 11** (structure group): given by the bundle's G.
- **Constraint 12** (holonomy monotonicity): IS the
  `Imperfect<State, Infallible, Holonomy>` return of Transport.
- **Constraint 13** (fixed point): IS the `Closure` trait's `fixed`
  associated type.

The cost of the extra machinery is paid back by reuse: the bundle
grammar already exists for the Fate-chip / BEAM-runtime / mirror-compiler
trio (per `bundle.rs`'s comment block). The eigenboard is another
instance of the same tower applied to a different base graph. The
sheaf-of-sections framing of C is recovered as a derived view — the
section data IS the cellular sheaf assignment, but living on a bundle
that constrains it.

---

## Bundle structure (new section)

### The tower, named

`@epistemologic/math/bundle` (the new grammar) declares five abstract
actions corresponding to the five levels in `prism/core/src/bundle.rs`:

```
fiber       : () -> state
connection  : () -> optic
gauge       : () -> group
transport   : state -> imperfect(state, holonomy)
close       : () -> fixed
```

A grammar that implements all five (with concrete carriers) IS a
principal G-bundle. The `bundle(grammar)` property checks this; the
`literal(implementation)` property checks the implementation matches
the declared shape under measurement.

### The eigenboard as an instance

The eigenboard instantiates the tower with:

```
state       = fiber                                  ([f64; 5])
optic       = connection_form                        (5×5 matrix)
group       = gauge_element                          (SO(5)-valued)
holonomy    = scalar_loss                            (the e_n value)
fixed       = closure_marker                         (the settled section)
```

Each action becomes a concrete action on the eigenboard:

- `fiber(op)` returns the section value at base node `op`.
- `connection(edge)` returns the parallel-transport operator on `edge`.
- `gauge(op)` returns the local gauge choice at `op`.
- `transport(section, edge)` parallel-transports the section across the
  edge, returning an imperfect-typed result whose holonomy IS the
  per-edge loss.
- `close(section)` returns the closure marker iff the section is at the
  fixed point of repeated transport.

### Why `Imperfect<State, Infallible, Holonomy>` IS `e^(n+1) < e^(n)`

The Rust signature `Transport::transport` returns:

```rust
Imperfect<Self::State, Infallible, Self::Holonomy>
```

The three type parameters carry:
1. The state (always recoverable — the transported section).
2. The infallible error channel (transport never produces a hard error;
   it can only produce *imperfect* results).
3. The holonomy (a `Loss`-typed value; the residual that survived
   transport).

The `Imperfect::Partial(state, loss)` case is the typical return: the
section transported correctly, but a non-zero holonomy accumulated. The
`Imperfect::Success(state)` case is the boundary: holonomy = 0, the
loop closed perfectly, the kintsugi tick discharged its obligation.

`e^(n+1) < e^(n)` IS the geometric statement that the holonomy on
successive ticks of the kintsugi loop strictly decreases until it
hits zero. The bundle's Transport signature *encodes* this monotonic
descent in the type system: a successful refract collapses
Imperfect::Partial to Imperfect::Success, and the only way for the
type to reach Success is for the Holonomy carrier to reach zero.

This is what makes the eigenboard's spectral story a theorem rather
than a heuristic. The Laplacian-spectral-gap argument and the
holonomy-decrease argument are the same argument viewed in two
different bases — sheaf cohomology (the L_F kernel) vs bundle
geometry (the connection's curvature integral around closed loops).

---

## Mycelial substrate (new section)

The bundle structure has a biological reading. Folding in the synthesis
from `mycelial-networks-and-au-tissue.md`:

### The structural correspondence

| Bundle structure | Mycelial structure | Reference |
|-----------------|--------------------|-----------|
| Fiber at base node | Hyphal tip with current SPK state | Riquelme et al. 2018; Steinberg 2013 |
| Connection on edge | Trunk-hypha bidirectional conduit | Schmieder et al. *Curr. Biol.* 2019 |
| Parallel transport | Cytoplasmic bulk flow + signaling | Heaton et al. 2010; Schmieder et al. 2019 |
| Gauge element | Local cord cross-section + Murray-equilibrium choice | Heaton et al. 2010; Haskovec et al. 2019 |
| Holonomy = loss | Energy dissipated in flow + branching cost | Tero et al. 2010; Marbach et al. 2023 |
| Closure | Autopoietic mycelium maintaining itself | Oyarte Galvez et al. 2025 (BARE) |
| Bundle morphism (anastomosis) | Hyphal anastomosis (two networks fuse) | Dikec et al. 2020; Oyarte Galvez et al. 2025 |
| Section growth tick | Apical extension by Spitzenkörper | Riquelme et al. 2018 |
| Persistence diagram of section history | Persistence diagram of growth | Sakib 2025 (preprint, flagged) |

The fit is precise enough to be more than metaphor. The Oyarte Galvez
et al. (2025) BARE model — Branching and Annihilating Range Expansion —
is the dynamics of a travelling wave of growth tips with anastomotic
fusion, exactly the dynamics of the kintsugi loop on the eigenboard.

The grammar can declare au-tissue IS mycelial under the BARE-model
framing. Three honest qualifications go with this claim:

### Qualification 1: math debt on sheaf cohomology of growing graphs

The eigenboard's base graph is small (5 nodes, fixed). The au-tissue's
base graph is *growing* — every kintsugi tick adds a resolved hole as
a new node, plus edges to its conductivity neighbours. Static sheaf
cohomology is solid (Hansen & Ghrist 2019). Sheaf cohomology on
*growing* graphs is recent research; Bressan et al. (arXiv:2402.00206,
2024) give the categorical framework, but the spectral monotonicity
statements mirror would need are not yet proven in generality.

This is the single largest mathematical risk. The grammar at this
tick declares the bundle structure on the static 5-node eigenboard
(safe); the au-tissue extension to a growing base graph inherits the
math debt. Per the research synthesis §5.4: either restrict to growth
modes for which monotonicity can be proven, or treat the monotonicity
as empirical pending theorem.

### Qualification 2: the Adamatzky overhype

Fungal-electrical-signaling-as-language (Adamatzky 2022) is contested
in the peer-reviewed literature (PMC11995700, 2024–2025; *Fungal Ecol.*
68, 101326, 2024). The 2024 review is explicit: extracellular electrode
measurements may pick up abiotic Donnan potentials and substrate
artifacts; the linguistic-analysis claims have not been independently
validated with the methodological controls the field requires.

The grammar must not import any IS-claim that depends on
fungi-computing-spikes-as-information. The well-substantiated signaling
story is *trunk-hypha bidirectional transport* (Schmieder et al. 2019),
which is what the bundle's connection imports. The grammar builds on
the load-bearing biology and avoids the contested.

### Qualification 3: the Mother-Tree mistake

The popular "Wood Wide Web" / "Mother Tree" narrative is critiqued in
Karst, Jones, Hoeksema *Nat. Ecol. Evol.* 2023 and Robinson et al.
*Trends Plant Sci.* 2023. The peer-reviewed verdict: no load-bearing
evidence for preferential kin-directed resource transfer through CMNs;
"positive citation bias and overinterpreted results have led to
misinformation" (Karst et al. 2023).

The grammar must use the de-personified, network-first framing: flat,
all-pairs, no central node — the Splinter topology of
`void-dual-geometry.md`, not the Narcissus star with a Mother-Tree hub.
The math and the biology agree on this; only the popular narrative
diverges, and the grammar declines to follow it.

### What this means for the spec

The grammar can claim:

- *au-tissue IS the section history of a principal G-bundle whose base
  graph grows by branching-and-annihilating range expansion.* This is
  the BARE-framed claim. It can pass `literal` measurement.
- *kintsugi tick IS hyphal apical extension at a Spitzenkörper.*
  Measurable: successive ticks show the BARE statistical signature.
- *anastomosis IS bundle-morphism identification of two fibers under
  connection-compatibility.* Measurable: the spectral gap increase
  when two subtissues fuse.

The grammar must not claim:

- *fungi speak a language via electrical spikes.* Unproven; flagged
  in the research synthesis as the load-bearing skepticism.
- *trees raise their children through the network.* Critiqued in the
  literature; flagged in the research synthesis as the load-bearing
  skepticism.

### The Splinter geometry as the topological match

`void-dual-geometry.md` describes the Splinter graph K_n as: λ₀ = 0 at
the ground state; no bottleneck; positive Ollivier-Ricci curvature;
maximum entanglement; fast mixing; all-pairs reachable. This is the
topological match for mycelial intelligence under the BARE model: a
network that does network things without a hub.

The eigenboard's 5-node base graph is small enough that the Splinter
topology is a reasonable default (full directed graph minus self-loops
is exactly the Splinter pattern). The growing au-tissue extension
preserves the Splinter property in the BARE wake: behind the leading
edge, anastomosis fills in cross-connections until the wake region is
densely connected.

The bundle's structure group acts consistently across this geometry
because the Splinter topology has no preferred direction; G acts by
rotation, not by translation; the eigenboard does not have a "front"
or "back."

---

## How the bundle threads through existing layers

### routing_bias → fiber projection through a gauge slice

```mirror
routing_bias.model_weights : [f64; 5]
# This is one fiber, projected onto the gauge slice that selects
# the "which Fate model to favor" coordinate. In bundle terms:
section_at(focus).projected_through(gauge_at(focus), model_axis)
  == routing_bias.model_weights
```

The five Fate models map one-to-one with the five operations because
each model owns one operation at the inference layer:

- **Abyss ≡ focus** (observe the state; the section value)
- **Introject ≡ project** (the connection; the optic of transport)
- **Cartographer ≡ split** (the gauge; the structure-group choice)
- **Explorer ≡ zoom** (parallel transport; the imperfect-returning level)
- **Fate ≡ refract** (the closure; the Lawvere fixed point)

This mapping is now structurally locked: the model_weights vector IS a
fiber, indexed by the operation that fiber lives on, and the model
that owns that fiber is the one whose Fate output gives the fiber its
value.

### 5×5 conductivity tensor → connection matrix

`@hash/coincidence`'s tensor is the matrix representation of the
bundle's connection in the canonical basis. The entry T[op_i][duality_j]
is the connection-form value from `fiber(op_i)` to the duality-j axis
at the adjacent fiber.

In other words: the 5×5 conductivity tensor IS the connection matrix.
The hash's 5 projections are 5 components of the connection 1-form;
the 5 dimensions are the 5 fibers. The hash WAS the connection all
along, viewed through content-addressing.

### `@cogito.observe(beam_n, beam_n+1)` → section delta + holonomy

The two beams carry their conductivity tensors (the connection field).
The observation IS the difference between the two sections PLUS the
holonomy accumulated by the transport between them:

```mirror
observe(beam_n, beam_n_plus_1) -> observation {
  let s_n     = beam_n.topology.as_section();
  let s_n_p_1 = beam_n_plus_1.topology.as_section();
  let delta   = section_diff(s_n, s_n_p_1);
  let h       = transport(s_n, beam_n.edge_to(beam_n_plus_1)).holonomy;
  observation { delta, holonomy: h, loss_delta: beam_n_plus_1.loss - beam_n.loss }
}
```

The holonomy gives the geometric reading of the loss decrease; the
section delta gives the kinematic reading. Both should agree —
`literal(transport)` checks that they do.

### `@cogito.strategy()` → bundle automorphism

Reflection's strategy is a bundle automorphism: a G-equivariant
transformation that takes one section and produces another. Written
as an mq query that selects fibers or connections and rewrites them:

```mirror
strategy(obs) -> automorphism {
  # "if holonomy on the (zoom, refract) edge exceeds threshold,
  #  rebalance toward Cartographer (split)"
  match obs.holonomy > connection[edge=(zoom, refract)] {
    high($v) => automorphism.rebalance(zoom, split, $v),
    _        => automorphism.identity,
  }
}
```

The match modifier `match(zoom)` (from match-select.md) is the
natural choice — the strategy crosses levels of resolution, which IS
the operation `zoom` owns at the Transport level of the tower.

### `@beam.topology: eigenvalues` → spectrum of the connection-squared

The beam's topology field carries the eigenvalues of the
connection-squared operator (sheaf Laplacian L_F, in the
sections basis). The Fiedler value (smallest nonzero eigenvalue) is
the spectral gap and IS what `@beam.compare` checks to verify
monotonic loss decrease.

### au's relational entanglement → fiber + connection

An au value's meaning depends on its position. In the bundle, that
position IS the fiber it lives in PLUS the connection that binds it
to neighbors. Move au to a different fiber and the connection forms
do not apply; the holonomy explodes; conductivity collapses to λ₀.
This is precisely the relational entanglement named in
au-and-conductivity.md.

### Match modifiers → bundle automorphisms

From match-select.md, match modifiers parameterize how a match
behaves across the operation hierarchy: `match(refract)` returns one
section; `match(split)` returns five candidate sections (one per Fate
model); `match(zoom)` traverses levels. Under the bundle framing,
each modifier IS a *specific kind of bundle automorphism*:

- `match(focus)`   = automorphism restricted to one fiber.
- `match(project)` = automorphism that respects the connection structure.
- `match(split)`   = automorphism that respects the gauge.
- `match(zoom)`    = automorphism with non-trivial holonomy.
- `match(refract)` = automorphism preserving the closure.

This is the cleanest unification — the match modifiers were always
selecting which structural level of the bundle the rewrite applies
to.

---

## Open questions

These are the choices Alex's prior input answered, integrated as
present-tense decisions; plus new questions surfaced by the bundle
framing.

1. **The composition edges** — *answered*: A-on-top-of-C — the full
   directed graph minus self-loops, augmented with the composition-table
   constraints from the operation type's category structure. 20 edges
   for the canonical eigenboard. The bundle's connection has 20
   non-trivial components plus the 5 trivial fiber identities.

2. **Per-tick or persistent** — *answered*: per-tick persistent. The
   eigenboard advances by one section per kintsugi tick, with the
   prior section recorded as `meta.ancestor`. Refs live at
   `refs/eigenboard/<agent>/HEAD`; the ancestor chain IS the audit
   trail Reflection walks for non-Markovian reasoning. GC is anti-
   Hebbian decay (see `@epistemologic/math/hebbian`); long-dead
   eigenboards get pruned by lack of use.

3. **The emotional sub-board's structural role** — *answered*: it is
   a side-channel rendering of dark dimensions. The 4 emotional slots
   (valence, arousal, dominant_cluster, drift_severity) form a
   separate sub-bundle whose base graph is the same 5 operations but
   whose connection is structurally independent of the operational
   one. The two sub-bundles share the base graph and the gauge
   choice but not the connection or the holonomy. The interoceptive
   signal OBSERVES the agent's relation to the operational signal
   without competing with it.

4. **The connection to `@mirror/match` modifiers** — *answered*: match
   modifiers ARE bundle automorphisms (see "Match modifiers → bundle
   automorphisms" above). Each modifier selects which level of the
   tower the automorphism acts at.

5. **Hodge decomposition of the eigenboard** — *answered as follow-up*:
   `@cogito/hodge` lands as a downstream tick. The Hodge decomposition
   of the connection 1-form gives gradient (legitimate Reflection
   progress), curl (oscillation between models), harmonic (irreducible
   stuck pattern — kintsugi can't fix; needs grammar evolution). The
   spec for this is its own tick.

### New open questions (from the bundle lift)

6. **What is the structure group G?** The canonical choice is SO(5)
   (rotations of the duality 5-space); but it could be O(5) (allowing
   reflections — useful if some dualities are signed), SU(5)
   (complex unitary — useful if the connection is naturally
   complex-valued), or GL(5) (general linear — useful if conductivity
   amplitudes are not normalized). The choice affects which
   conductivity quantities are invariants and which are
   gauge-dependent. **Alex's call needed.** My read: SO(5) for the
   first cut; revisit if Hodge decomposition wants signed flow.

7. **How does the bundle on the 5-operation base graph extend to the
   growing au-tissue base graph?** The static bundle is well-defined.
   The growing version inherits the math debt of §"Mycelial substrate"
   Qualification 1. Either: (a) declare au-tissue as a *family* of
   bundles indexed by tick, with morphisms between consecutive ticks
   = anastomosis events + apical extensions; or (b) declare a
   *temporal* bundle whose base is the time-extended graph (Bressan
   et al. 2024 categorical framework). **Alex's call needed.**

8. **Does the closure level need its own grammar action, or is it
   sufficient to declare `close()` and leave the body `\`?** Per the
   minimum-grammar-cost rule, the action is enough; the closure
   detection logic lives downstream. But the `Lawvere fixed point`
   identification is precise (Lawvere 1969) and might warrant
   `@epistemologic/math/lawvere` as its own follow-up tick.

9. **The connection-form symmetry.** The connection is a 5×5 matrix
   per edge. Is it constrained to be symmetric, antisymmetric, or
   general? Each choice has a different geometric meaning:
   symmetric → the connection is metric (preserves an inner product),
   antisymmetric → the connection is symplectic (preserves a 2-form),
   general → no extra structure. **Alex's call needed.** My read:
   start with general; constrain only if `literal(transport)` fails
   without the constraint.

---

## Implications — concrete next ticks

Reordered to put the bundle declarations first:

1. **Declare `@epistemologic/math/bundle`** — DONE in this tick.
   `boot/std/epistemologic/math/bundle.mirror` declares the five
   abstract actions, the five carrier types, and the two properties
   (`bundle`, `literal`). The new grammar imports `@epistemologic`,
   `@epistemologic/math/sheaf`, `@beam`. All action bodies are `\`.

2. **Declare `type eigenboard` as a section of the bundle.** Single
   grammar file, probably `boot/std/cogito/eigenboard.mirror` (new).
   The grammar imports `@epistemologic/math/bundle` and instantiates
   the tower with the concrete carriers (`fiber = [f64; 5]`,
   `optic = connection_form`, `group = gauge_element`, etc.). All
   higher-level lambdas stay `\`.

3. **Retype `@cogito.perturb`'s return.** Today it returns `eigenboard`
   with body `@beam.observe`. After this spec lands, the type resolves
   through `@epistemologic/math/bundle` (a section of the bundle). The
   body stays `\` until the transformation logic is wired
   (kintsugi-wiring tick 7).

4. **Map `routing_bias` to a fiber projection through a gauge slice.**
   Declare in cogito-eigenstate (spectral) that
   `routing_bias.model_weights` IS the fiber at the relevant operation
   projected through the local gauge onto the model-favor axis. This
   is a documentation update + a small grammar action.

5. **Equate the 5×5 conductivity tensor with the connection matrix.**
   Update `@hash/coincidence` to declare that its 5×5 tensor IS the
   matrix representation of an eigenboard's connection 1-form in the
   canonical basis. This is a comment + type alias; no computation
   change.

6. **Spec `@cogito/hodge`** — the Hodge decomposition of the
   eigenboard's connection. Surfaces the harmonic component (the
   irreducible stuck pattern). Future work; the path is open.

7. **Wire `@beam.compare`** to read the connection-squared operator's
   smallest nonzero eigenvalue (Fiedler) of the two beams' topology
   fields. This is the concrete check that grounds `e^(n+1) < e^(n)`.

8. **Persistent eigenboard refs.** Declare `refs/eigenboard/<agent>/HEAD`
   convention. Each tick advances the ref. The ancestor chain is the
   history Reflection can walk for non-Markovian reasoning.

9. **Au-tissue as the growing-base extension** — defer behind the
   sheaf-on-growing-graphs math debt. Either land Bressan et al. 2024
   as `@epistemologic/math/sheaf/growth` first (cleanest), or declare
   au-tissue with the math-debt acknowledged in its `out of scope`
   section.

10. **`@epistemologic/bio/mycelium` as the BARE-framed sub-grammar.**
    The sketch in `mycelial-networks-and-au-tissue.md` §"Synthesis" is
    the starting point. It imports `@epistemologic/math/bundle`,
    `@epistemologic/math/sheaf`, `@epistemologic/math/homology`,
    `@epistemologic/math/tropical`. The three qualifications above
    determine what it can and cannot claim.

---

## Out of scope

- The actual computation of the connection-squared operator's
  spectrum. The math lives in `@epistemologic/math/sheaf` and
  `@epistemologic/math/bundle`; the eigenboard inherits.
- The full implementation of Reflection's strategy logic. This spec
  declares the type; the strategy bodies stay `\` until kintsugi
  closes them.
- The training of Fate's five models. They emit au into fibers; how
  they learn to do so is a separate concern.
- The Anthropic 171-emotion vectors. They live in cogito-eigenstate as
  the emotional side-channel; this spec doesn't touch their shape.
- The LSP / gutter rendering of eigenboard slots. The void color from
  gutter-lenses.md applies when conductivity is at λ₀ in any fiber;
  the visualization is downstream.
- The diff/review surface for eigenboard transformations. Belongs in
  kintsugi-wiring's future tick section.
- The au-tissue extension to a growing base graph. Math debt; see
  Qualification 1 in the mycelial substrate section.
- The detailed shape of `@epistemologic/bio/mycelium`. The research
  synthesis sketches it; the spec is downstream.
- The Lawvere-fixed-point grammar `@epistemologic/math/lawvere` (open
  question 8). Possibly its own follow-up tick.

---

*Five operations, five base nodes.*
*Five dualities, five axes per fiber.*
*The connection binds the fibers; the gauge chooses the basis.*
*Transport returns imperfect by design.*
*Holonomy IS loss; holonomy decreases monotonically.*
*The closure is the Lawvere fixed point.*
*Reflection writes automorphisms; kintsugi closes the loop.*
*The eigenboard is a section of the principal bundle Reflection composes against.*
*The bundle is the shape; the section is the state; the holonomy is the cost.*

Apache-2.0.
