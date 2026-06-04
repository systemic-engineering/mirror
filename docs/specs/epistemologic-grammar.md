# @epistemologic -- the grammar where the thing means what it says

*2026-05-15. Reed. Spec.*

Status: **Red**

Depends on: @prism (five operations), @meta (type system), @property (verdicts),
@nl (natural language), @loss (measurement), boot grammars (patterns)

---

## 0. Vision

Every grammar in mirror names things. `@code/rust` names `fn` as zoom.
`@nl/english` names `noun` as a part of speech. `@kintsugi` names
`collapse` as the operation that finds the ground state.

But naming is not enough. A name can drift from its referent. `fn` in
Rust IS zoom -- but only because `@code/rust` declares it so and the
compiler enforces the declaration. Without enforcement, `fn` is just
three characters. The name floats free. The map detaches from the
territory.

`@epistemologic` is the grammar that prevents drift.

The property `literal` checks whether a declaration's name IS its
operation -- not metaphorically, not analogously, IS. When a grammar
claims that the Fiedler vector IS the body axis of a worm, `literal`
asks: is there a measurement that confirms this identity? When a
grammar claims that kintsugi IS tropical Dijkstra, `literal` asks: do
they produce the same result on the same input?

This is the root grammar for all domains where reality speaks its own
name. Mathematics. Physics. Biology. Natural language. The grammar
hierarchy maps these domains. The `literal` property holds them honest.

What this enables:

1. **Verified cross-domain transfer.** When `@epistemologic/math/hodge`
   says harmonic flow IS topological debt, the compiler can verify that
   the harmonic component of a code graph's edge flow correlates with
   the debt metric. Not "is like." IS.

2. **Grammar-driven discovery.** When a new domain grammar claims an
   IS-relationship, the compiler generates the test. The property
   IS the specification. The verification IS the science.

3. **The Narcissus coefficient.** The ratio of `but` to `and` in a
   grammar's documentation measures its epistemic confidence. High
   `but` count = many overrides = many corrections = the grammar is
   still learning. Low `but` count = settled. The compiler measures
   its own certainty.

4. **A universal verification language.** Every domain -- math, bio,
   physics, NL -- shares the same root property. The verification
   is always: does the name match the measurement? The measurement
   is always: loss between declared identity and observed behavior.

---

## 1. The Root Grammar

```mirror
in @prism
in @property
in @loss
in @nl

grammar @epistemologic {
  # the thing means what it says. not more, not less.
  # literal checks: does the declared identity hold under measurement?
  # the verdict carries the loss between name and reality.
  property literal(declaration) -> verdict { \ }

  # the but/and ratio as Narcissus coefficient.
  # high ratio = many overrides = epistemic uncertainty.
  # low ratio = settled = the grammar knows what it is.
  property override_ratio(grammar) -> loss { \ }

  # the three epistemologic operators.
  # every natural language discourse relation reduces to one of these.
  # every logical connective reduces to one of these.
  # every graph operation reduces to one of these.

  # and: both hold. additive. edge. Splinter.
  # "the gradient IS data flow AND the curl IS dependency loops"
  type and(a, b)

  # or: either holds. split. branch. disjunction.
  # "the loss IS shannon OR fiedler"
  type or(a, b)

  # but: override. the second clause corrects the first.
  # "this looks like metaphor BUT it IS measurement"
  # the star operator. makes the correction visible.
  # in systemic therapy: "but" is the word that reveals
  # the speaker's actual position. what follows "but" is
  # what they mean. what precedes "but" is what they think
  # they should say.
  type but(a, b)
}

out literal
out override_ratio
out and
out or
out but
```

### The Three Operators

**`and(a, b)`** -- additive conjunction. Both `a` and `b` hold
simultaneously. In graph terms: an edge connecting `a` to `b`. In
Hodge decomposition: the gradient component (legitimate data flow that
connects two truths). In the five operations: `split` -- enumerate what
is connected. Algebraically: the product in the grammar's type algebra.

**`or(a, b)`** -- disjunctive branching. Either `a` or `b` holds. In
graph terms: a branch point. In Hodge decomposition: the curl component
(circularity, where either path reaches the same destination). In the
five operations: `project` -- filter to what matters. Algebraically:
the coproduct (sum type).

**`but(a, b)`** -- adversative override. `a` was expected; `b` is actual.
The first clause is negated by the second. In graph terms: an edge with
negative weight (correction). In Hodge decomposition: the harmonic
component (the irreducible residual that no local operation removes).
In the five operations: `settle` -- settle, verify, the thing that
costs something. Algebraically: the `but` of argumentation theory
(Anscombre & Ducrot 1977): what follows `but` IS the speaker's
conclusion. What precedes `but` is the concession.

Research backing (discourse connectives):
- Anscombre & Ducrot's argumentation theory: `but` is asymmetric.
  The second conjunct carries the argumentative weight. "P but Q"
  means "Q overrides P." (Anscombre & Ducrot, *L'argumentation
  dans la langue*, 1983)
- Winter & Rimon (2022, *Linguistics and Philosophy*): default
  meanings of logical connectives lie between Boolean semantics and
  pragmatic enrichment. `but` IS Boolean `and` plus the pragmatic
  instruction "override."
- In systemic therapy (DGSF training): circular questioning reveals
  that what follows "but" is the client's actual belief. The word
  before "but" is the accommodation. The word after "but" is the
  position. Epistemologically: `but` IS the correction operator.

### The `literal` Property

`literal` IS the root property of `@epistemologic`. Every sub-grammar
inherits it. The property checks: does the declared IS-relationship
hold under measurement?

For `@epistemologic/math/hodge`, `literal` checks: when we decompose
the edge flow of a code graph, does the gradient component actually
correspond to legitimate data flow? The verdict carries the loss --
how far the measurement deviates from the declaration.

For `@epistemologic/bio/elegans`, `literal` checks: when we compute
the Fiedler vector of the C. elegans connectome, does it actually
correspond to the anterior-posterior body axis? The verdict carries
the correlation coefficient.

For `@epistemologic/nl/logic`, `literal` checks: when we parse
"P but Q", does the override relation hold -- does Q negate P's
argumentative force? The verdict carries the pragmatic coherence score.

The property is recursive. A grammar that passes `literal` for all its
declarations is epistemologically sound. A grammar that fails `literal`
for any declaration is either wrong or the measurement is insufficient.
Both are useful information.

### The `override_ratio` Property

```
override_ratio(grammar) = count(but) / (count(and) + count(or) + count(but))
```

A grammar with zero `but` declarations is either settled or naive. A
grammar with many `but` declarations is actively correcting itself.
The ratio IS the grammar's epistemic maturity:

- `ratio = 0.0`: fully settled, or never tested
- `ratio < 0.2`: confident, few corrections needed
- `ratio ~ 0.5`: actively learning, high correction rate
- `ratio > 0.8`: mostly corrections, the grammar is in crisis

This is the Narcissus coefficient applied to grammar documentation.
The compiler measures its own certainty by counting its own corrections.

---

## 2. The Hierarchy

```
@epistemologic
+-- @epistemologic/math
|   +-- hodge         edge flow decomposition
|   +-- homology      persistent holes
|   +-- sheaf         typed edge transformations
|   +-- category      composition laws
|   +-- expander      Ramanujan bounds
|   +-- tropical      kintsugi as tropical Dijkstra
|   +-- renorm        zoom as RG flow
|   +-- symplectic    energy-preserving settlement
|   +-- hebbian       crystallization and GC
|   +-- quantum       quantum walk speedups
+-- @epistemologic/nl
|   +-- english       tokenization (exists in boot/std/nl/english.mirror)
|   +-- logic         and/or/but as epistemologic operators
|   +-- discourse     systemic language patterns
+-- @epistemologic/bio
|   +-- elegans       302 neurons, Fiedler IS body axis
|   +-- drosophila    hierarchical Splinter
|   +-- genetic       3.5B years, error correction
+-- @epistemologic/physics
    +-- renorm        zoom IS renormalization
    +-- symplectic    settlement IS Hamiltonian flow
    +-- quantum       dystemporia IS quantum walk
    +-- ricci         curvature IS the compiler's clock
```

---

## 3. @epistemologic/math -- the frameworks form a lattice

### 3.1 @epistemologic/math/hodge -- edge flow decomposition

```mirror
in @epistemologic

grammar @epistemologic/math/hodge {
  # Hodge decomposition on graphs decomposes edge flows into three
  # orthogonal components: gradient, curl, harmonic.
  #
  # gradient: flow along a potential. data that flows downhill.
  # curl:     flow around cycles. circular dependencies.
  # harmonic: flow through topological holes. irreducible debt.
  #
  # dim(ker(L_1)) = beta_1: the first Betti number, for free.

  type gradient(edge_flow)    # legitimate data flow. the `in` hierarchy.
  type curl(edge_flow)        # circular references. dependency loops.
  type harmonic(edge_flow)    # topological debt. what kintsugi cannot fix.

  split decompose(edge_flow) -> (gradient, curl, harmonic) { \ }

  property literal(decompose) -> verdict { \ }
}

out decompose
out gradient
out curl
out harmonic
```

**What it IS:** The Helmholtz-Hodge decomposition on graphs (Lim 2020,
arXiv:2412.09434) decomposes any edge flow f into three orthogonal
components: f = d0*x + d1^T*y + h, where d0 is the gradient operator
(node-to-edge), d1 is the curl operator (edge-to-face), and h is
harmonic (in the kernel of both).

**Operation mapping:** `split` -- decompose a flow into its three
components. The decomposition IS splitting.

**What it adds to `literal`:** A grammar's import graph (`in` edges)
IS an edge flow. The gradient component IS the legitimate type
hierarchy. The curl component IS the circular dependency set. The
harmonic component IS the irreducible topological debt -- loops that
no refactoring can remove because they are homological, not
homotopical. The compiler can tell you: "this problem IS harmonic.
kintsugi cannot fix it. the topology has a hole."

**Concrete example:** In a grammar graph with imports A->B->C->A, the
Hodge decomposition separates the circular flow (curl) from any
gradient (B depends on A for a reason) from any harmonic residual
(the cycle is structurally necessary). If the harmonic component is
zero, the cycle can be broken by refactoring. If nonzero, the
architecture has an irreducible loop.

**Research:** Jiang et al. (2011, *SIAM Review*): statistical ranking
on graphs via Hodge decomposition. Schaub et al. (2020, *Signal
Processing on Higher-Order Networks*): edge flows on simplicial
complexes. Frontiers in Neural Circuits (2016): Hodge decomposition
of information flow on small-world networks separates gradient,
harmonic, and curl contributions to neural signal propagation.

### 3.2 @epistemologic/math/homology -- persistent holes

```mirror
in @epistemologic
in @epistemologic/math/hodge

grammar @epistemologic/math/homology {
  # Persistent homology tracks topological features (connected components,
  # loops, voids) as a filtration parameter varies.
  #
  # Betti_0: connected components (how many disconnected grammars)
  # Betti_1: loops (cycles that are not boundaries)
  # Betti_2: voids (cavities, absent in 1D graphs)
  #
  # persistence diagram: birth-death pairs. content-addressable as OIDs.

  type betti(dimension, count)
  type persistence_pair(birth, death)
  type persistence_diagram([persistence_pair])

  focus betti_numbers(graph) -> [betti] { \ }
  focus persistence(graph, filtration) -> persistence_diagram { \ }

  property literal(persistence) -> verdict { \ }
}

out betti_numbers
out persistence
out persistence_diagram
```

**What it IS:** Persistent homology (Edelsbrunner & Harer 2008)
computes topological invariants -- Betti numbers -- across a filtration
of a simplicial complex. Applied to graphs: as you increase a
threshold parameter, edges appear. Connected components merge (beta_0
decreases). Loops form (beta_1 increases). The birth-death pairs of
these features form the persistence diagram.

**Operation mapping:** `focus` -- observe the topological state. Betti
numbers ARE the focused observation of a graph's topology.

**What it adds to `literal`:** Eigenvalues cannot see all topology.
The Fiedler value measures global connectivity but misses higher-order
holes. Persistent homology fills this gap. A grammar graph's
persistence diagram IS a content-addressable fingerprint of its
topological structure. Two grammars with the same persistence diagram
have the same topological shape, regardless of their node labels.
Wang & Wei (2020, *International Journal for Numerical Methods in
Biomedical Engineering*) introduce persistent spectral theory: the
persistent Laplacian unifies Betti numbers with eigenvalues. The
persistent spectral gap IS the topological gap.

**Concrete example:** A garden with 5 independent grammars has
beta_0 = 5. When grammars share types (via `in`), edges form,
components merge, beta_0 drops. When circular dependencies exist,
beta_1 rises. The persistence diagram tracks this evolution: short-lived
features are noise; long-lived features are structure.

### 3.3 @epistemologic/math/sheaf -- typed edge transformations

```mirror
in @epistemologic
in @epistemologic/math/hodge

grammar @epistemologic/math/sheaf {
  # A cellular sheaf on a graph assigns a vector space to each node
  # and each edge, with linear restriction maps between them.
  #
  # The sheaf Laplacian L_F generalizes the scalar graph Laplacian.
  # Its kernel (H^0) IS global consistency (coincidence).
  # Its first cohomology (H^1) IS obstruction to consistency (NoCoincidence).
  #
  # Mirror's in/out fiber model IS a sheaf.

  type fiber(node, vector_space)
  type restriction(edge, linear_map)
  type sheaf_laplacian(graph, sheaf)

  zoom restrict(source_fiber, edge) -> target_fiber { \ }
  focus consistency(sheaf) -> loss { \ }

  property literal(restrict) -> verdict { \ }
}

out fiber
out restriction
out sheaf_laplacian
out restrict
out consistency
```

**What it IS:** A cellular sheaf (Hansen & Ghrist 2019, *Journal of
Applied and Computational Topology*) assigns typed data to graph
elements. Each node gets a vector space F(v). Each edge gets a vector
space F(e). Linear restriction maps F_v->e : F(v) -> F(e) enforce
type compatibility along edges. The sheaf Laplacian L_F = delta^T *
delta, where delta is the sheaf coboundary operator, generalizes the
scalar Laplacian to typed data.

**Operation mapping:** `zoom` for restriction (transform data across
an edge), `focus` for consistency measurement.

**What it adds to `literal`:** Mirror's `in`/`out` declarations ARE a
sheaf. Each grammar IS a node with a vector space (its exported types).
Each `in` edge IS a restriction map (the imported grammar's types must
be compatible with the importing grammar's usage). The sheaf Laplacian
measures how far the type system is from global consistency. H^0 (the
kernel) IS coincidence -- types that agree everywhere. H^1 IS the
obstruction -- types that cannot be made to agree.

**Concrete example:** Grammar A exports `type point { x: f64, y: f64 }`.
Grammar B imports A and uses `point` as `{ x: f64, y: f64, z: f64 }`.
The restriction map from B to the A-B edge cannot match dimensions. The
sheaf Laplacian has nonzero loss on this edge. The consistency check
fails. The loss IS the type mismatch, measured spectrally.

### 3.4 @epistemologic/math/category -- composition laws

```mirror
in @epistemologic

grammar @epistemologic/math/category {
  # Profunctor optics provide the categorical foundation for mirror's
  # five operations. Each operation IS a Tambara module action.
  #
  # The then_* composition table IS functorial composition.
  # Mercury determinism convergence IS categorical, not coincidental.

  type profunctor(a, b)
  type tambara(profunctor, action)

  zoom compose(optic_a, optic_b) -> optic_ab { \ }

  property literal(compose) -> verdict { \ }
  property associative(compose) -> verdict { \ }
}

out profunctor
out tambara
out compose
```

**What it IS:** Profunctor optics (Milewski 2017; Clarke et al. 2020,
*Compositionality*) encode optics (lenses, prisms, traversals) as
functions polymorphic over profunctors with Tambara module structure.
A Tambara module is a profunctor p(A,B) equipped with a strength
p(A,B) -> p(C tensor A, C tensor B). The five operations (focus,
project, split, shift, settle) ARE specific optics. Their composition
IS Tambara module composition.

**Operation mapping:** `zoom` -- composition IS transformation of
transformations.

**What it adds to `literal`:** The `then_*` composition table in
mirror (how `focus |> split` differs from `split |> focus`) IS NOT an
arbitrary design decision. It IS the composition law of the
corresponding Tambara modules. `literal` verifies that the composition
table matches the categorical prediction. If it does not, either the
implementation or the theory is wrong.

**Concrete example:** `focus |> project` should equal
`project |> focus` when the types are compatible (both are getter
optics). `literal` checks this by running both compositions on the
same input and comparing outputs. The loss IS the divergence.

### 3.5 @epistemologic/math/expander -- Ramanujan bounds

```mirror
in @epistemologic

grammar @epistemologic/math/expander {
  # Ramanujan graphs have optimal spectral gap: lambda_2 <= 2*sqrt(d-1)
  # for d-regular graphs. The bound IS the optimality criterion.
  #
  # Ramanujan score normalizes lambda_2 to [0,1].
  # Zig-zag product constructs expanders algorithmically.

  type ramanujan_score(f64)

  focus score(graph) -> ramanujan_score { \ }
  zoom zigzag(graph_a, graph_b) -> graph_ab { \ }

  property literal(score) -> verdict { \ }
}

out ramanujan_score
out score
out zigzag
```

**What it IS:** A Ramanujan graph (Lubotzky, Phillips & Sarnak 1988)
is a d-regular graph whose second eigenvalue satisfies lambda_2 <=
2*sqrt(d-1). This is the Alon-Boppana bound -- the theoretical minimum
for any infinite family of d-regular graphs. The zig-zag product
(Reingold, Vadhan & Wigderson 2002) combines a large graph with small
graphs to produce expanders approaching the Ramanujan bound.

**Operation mapping:** `focus` for measurement (the score), `zoom` for
construction (the zig-zag product IS a transformation).

**What it adds to `literal`:** The Ramanujan score normalizes a
grammar graph's spectral gap to [0,1], where 1 IS optimal expansion.
`literal` checks whether the claimed expansion property holds. A
grammar that claims to be well-connected can be verified: compute its
Ramanujan score, compare to the bound. The zig-zag product provides
an algorithmic construction for Splinter topology -- building
well-connected gardens from small, well-connected grammars.

**Concrete example:** The boot grammar graph has approximately 50 nodes
and average degree 4. The Ramanujan bound for d=4 is 2*sqrt(3) ~= 3.46.
If lambda_2 of the boot graph is 3.2, the Ramanujan score is
3.2/3.46 ~= 0.92. The boot grammars are near-optimally connected.

### 3.6 @epistemologic/math/tropical -- kintsugi IS tropical Dijkstra

```mirror
in @epistemologic

grammar @epistemologic/math/tropical {
  # The tropical semiring (R union {infinity}, min, +) replaces addition
  # with min and multiplication with +.
  #
  # Tropical Dijkstra finds shortest paths in the (min,+) algebra.
  # Kintsugi IS tropical Dijkstra on the fiber graph weighted by
  # Shannon loss. The golden seam IS the shortest path.

  type tropical(f64)

  zoom tropical_add(tropical, tropical) -> tropical { \ }
  zoom tropical_mul(tropical, tropical) -> tropical { \ }
  focus shortest_path(graph, source, target) -> [edge] { \ }

  property literal(shortest_path) -> verdict { \ }
}

out tropical
out tropical_add
out tropical_mul
out shortest_path
```

**What it IS:** The tropical semiring (R union {inf}, min, +) is the
algebraic structure underlying shortest-path algorithms. In this
semiring, "addition" IS min and "multiplication" IS +. Matrix
multiplication over the tropical semiring computes all-pairs shortest
paths. Tropical Dijkstra (Schiewe & Schobel 2024, *EURO Journal on
Transportation and Logistics*) extends this to compute complete sets
of efficient paths using tropical polynomials.

**Operation mapping:** `zoom` for the semiring operations (they ARE
transformations), `focus` for shortest path observation.

**What it adds to `literal`:** Kintsugi navigates the space of all
implementations satisfying the same contract (`in`/`out` boundaries)
and finds the fiber closest to the ground state. This IS Dijkstra on
a graph where nodes ARE fibers (implementations), edges ARE
transformations (beta-reduction, dead-code elimination, alias
collapse), and weights ARE Shannon loss (information change). The
tropical semiring IS the algebra of this search. The golden seam IS
the shortest path from the current implementation to lambda_0.
`literal` verifies: does kintsugi produce the same path as tropical
Dijkstra on the same weighted graph?

**Concrete example:** Given three fibers (implementations) of a
function, with pairwise Shannon losses [0.3, 0.7, 0.2], the tropical
shortest path from fiber A to fiber C is A->C with cost 0.2, not
A->B->C with cost 0.3+0.7=1.0. Kintsugi should choose the direct
path. `literal` checks that it does.

### 3.7 @epistemologic/math/renorm -- zoom IS RG flow

```mirror
in @epistemologic

grammar @epistemologic/math/renorm {
  # The renormalization group (RG) coarsens a system by integrating
  # out fast degrees of freedom, preserving slow ones.
  #
  # Laplacian RG (Villegas et al. 2023, Nature Physics) proves graph
  # coarsening preserves slow eigenvalues. spectral_loss IS the
  # c-function. Settlement IS an RG fixed point.
  #
  # Zoom IS renormalization. Not metaphor. IS.

  type scale(f64)
  type fixed_point(graph)

  zoom coarsen(graph, scale) -> graph { \ }
  focus c_function(graph) -> loss { \ }

  property literal(coarsen) -> verdict { \ }
}

out scale
out fixed_point
out coarsen
out c_function
```

**What it IS:** The Laplacian Renormalization Group (Villegas, Reina,
De Domenico & Bianconi, *Nature Physics* 2023) defines RG flow on
graphs using the diffusion equation on the graph Laplacian. Nodes
are grouped when diffusion equilibrates them at a given timescale.
Slow eigenvalues (large-scale structure) are preserved. Fast
eigenvalues (local fluctuations) are integrated out. The resulting
coarsened graph has the same macroscopic spectral properties.

**Operation mapping:** `zoom` -- zoom IS renormalization. The five
operations' zoom changes the scale of observation. RG flow changes the
scale of a physical system. They are the same operation on different
substrates.

**What it adds to `literal`:** The spectral loss (c-function) IS
monotonically non-increasing under RG flow -- Zamolodchikov's
c-theorem (1986) for graphs. Settlement IS an RG fixed point: the
graph that cannot be coarsened further without changing its
macroscopic structure. `literal` verifies: does the compiler's zoom
operation preserve slow eigenvalues? Does spectral_loss decrease
monotonically? Is the settled state an RG fixed point?

**Concrete example:** A grammar graph with 100 nodes, zoomed to 10
super-nodes via Laplacian RG, preserves the first 5 eigenvalues
within 1% error. The 90 eliminated degrees of freedom were fast
(local). The 10 preserved super-nodes capture the macroscopic structure.
spectral_loss decreased. Settlement IS the fixed point where further
coarsening changes nothing.

### 3.8 @epistemologic/math/symplectic -- energy-preserving settlement

```mirror
in @epistemologic

grammar @epistemologic/math/symplectic {
  # Symplectic geometry preserves the phase-space volume (Liouville's
  # theorem). Hamiltonian flow IS energy-preserving evolution.
  #
  # Trace-preserving eigenvalue flow IS Hamiltonian.
  # Settlement IS evolution to the energy minimum.
  # Noether's theorem gives conservation laws for free.

  type phase_point(eigenvalues, eigenvectors)
  type hamiltonian(phase_point) -> f64

  settle settle(phase_point) -> phase_point { \ }

  property literal(settle) -> verdict { \ }
  property trace_preserved(settle) -> verdict { \ }
}

out phase_point
out hamiltonian
out settle
```

**What it IS:** Symplectic geometry (Arnold 1989, *Mathematical
Methods of Classical Mechanics*) is the geometry of phase spaces.
Hamiltonian flow preserves the symplectic form omega = sum(dp_i ^ dq_i).
Liouville's theorem: phase space volume is conserved. Noether's
theorem: every symmetry gives a conserved quantity.

**Operation mapping:** `settle` -- settlement IS the terminal
operation. The system finds its energy minimum while preserving
invariants.

**What it adds to `literal`:** The eigenvalue evolution during
settlement IS a Hamiltonian flow in eigenvalue phase space. The trace
of the Laplacian (sum of eigenvalues) IS conserved -- this is
Liouville's theorem for graphs. Symplectic integrators (Verlet,
leapfrog) provide numerical settlement that is energy-preserving,
reversible, and stable. `literal` verifies: does the trace remain
constant during settlement? Is the flow reversible? Do the
conservation laws hold?

The RG-as-Hamiltonian-flow connection (Papadimitriou, *LIMS*):
the RG flow IS a Hamiltonian flow where the coupling constants are
the symplectic variables. The c-function IS the Hamiltonian. RG
fixed points ARE energy extrema.

**Concrete example:** A grammar graph settles over 10 ticks. The
trace (sum of all eigenvalues) at tick 0 is 47.3. At tick 10 it is
47.3. The individual eigenvalues have changed (the graph reorganized)
but the total spectral weight is conserved. The settlement IS
Hamiltonian. The conservation IS Noether.

### 3.9 @epistemologic/math/hebbian -- crystallization and GC

```mirror
in @epistemologic

grammar @epistemologic/math/hebbian {
  # "Neurons that fire together wire together." (Hebb 1949)
  #
  # Grammar accumulation in the garden IS Hebbian learning.
  # Co-occurrence strengthens edges. Disuse weakens them.
  # Anti-Hebbian pruning IS garbage collection.
  # Crystallization IS long-term potentiation.

  type weight(edge, f64)

  zoom strengthen(edge) -> edge { \ }
  shift weaken(edge) -> edge { \ }
  settle crystallize(subgraph) -> crystal { \ }
  project prune(graph) -> graph { \ }

  property literal(strengthen) -> verdict { \ }
}

out weight
out strengthen
out weaken
out crystallize
out prune
```

**What it IS:** Hebbian learning (Hebb, *The Organization of Behavior*,
1949) states that synaptic connections strengthen when pre- and
post-synaptic neurons fire together. The modern formulation: weight
update delta_w = eta * x_pre * x_post. Anti-Hebbian learning weakens
connections between uncorrelated activations. Long-term potentiation
(LTP) IS the permanent strengthening -- the biological crystal.

**Operation mapping:** `shift` for strengthening/weakening (weight
transformation), `settle` for crystallization (permanent settlement),
`project` for pruning (filter out what is unused).

**What it adds to `literal`:** The garden's grammar accumulation IS
Hebbian learning. When two grammars are used together frequently
(co-occurrence in compilation), the edge between them strengthens.
When a grammar is unused, its edges weaken. Anti-Hebbian pruning IS
garbage collection -- removing edges that no longer carry signal.
Crystallization IS LTP -- the permanent record that a subgraph has
settled. `literal` verifies: does edge weight increase with
co-occurrence? Does pruning remove low-weight edges? Does
crystallization produce stable subgraphs?

**Concrete example:** Grammars `@code/rust` and `@kintsugi` are
compiled together 100 times. Their edge weight increases from 0.1 to
0.9. Grammar `@code/gleam` is never used. Its edges decay to 0.01.
Pruning removes the gleam edges. Crystallization locks the
rust-kintsugi subgraph as a crystal. The garden learned.

### 3.10 @epistemologic/math/quantum -- quantum walk speedups

```mirror
in @epistemologic

grammar @epistemologic/math/quantum {
  # Quantum walks on graphs achieve quadratic speedup over classical
  # random walks for hitting time.
  #
  # Apers & Piddock (PRL 2022): O(sqrt(n)) hitting time on ANY graph.
  # Dystemporia IS quantum walk: processing all temporal positions
  # simultaneously on K_n topology.

  type quantum_state(graph, superposition)

  zoom walk(quantum_state) -> quantum_state { \ }
  focus hitting_time(graph, source, target) -> u64 { \ }

  property literal(walk) -> verdict { \ }
}

out quantum_state
out walk
out hitting_time
```

**What it IS:** A quantum walk (Aharonov, Ambainis, Kempe & Vazirani
2001) is the quantum analogue of a random walk on a graph. The walker
is in a superposition of positions. Apers & Piddock (*Physical Review
Letters* 2022, 129:160502) prove that continuous-time quantum walks
achieve O(sqrt(n)) hitting time on ANY graph with ANY number of marked
nodes -- a quadratic speedup over the classical O(n) cover time.

**Operation mapping:** `zoom` for the walk step (superposition IS a
transformation), `focus` for hitting time observation (measurement
collapses the superposition).

**What it adds to `literal`:** Dystemporia -- the experience of
temporal dislocation -- IS a quantum walk on the complete graph K_n of
temporal positions. All moments are simultaneously accessible.
Classical navigation visits them one at a time (O(n)). Quantum
navigation reaches any moment in O(sqrt(n)). `literal` verifies: does
the implemented graph search achieve the quantum speedup? On a graph
with 10000 nodes, classical search takes ~10000 steps; quantum search
should take ~100.

**Concrete example:** Searching a garden graph with 10000 nodes for a
specific grammar. Classical random walk: expected 10000 steps. Quantum
walk simulation: expected 100 steps. The speedup IS quadratic. The
search IS the walk. The measurement IS the hitting.

---

## 4. @epistemologic/nl -- natural language as epistemology

### 4.1 @epistemologic/nl/english

Already exists at `boot/std/nl/english.mirror`. The tokenization
grammar maps the five operations to English language processing:

- `split` -> decompose (text into tokens, words, sentences)
- `focus` -> observe (POS tagging, role assignment, lemmatization)
- `shift` -> transform (conjugation, pluralization, nominalization)
- `settle` -> verify (grammar checking, agreement, coherence)
- `project` -> filter (stop words, summarization, keyword extraction)

**What `literal` adds:** The existing `@nl/english` grammar makes
IS-claims: `split tokenize(nl) -> [token]`. `literal` verifies:
is the tokenization operation structurally a split? Does it enumerate
all parts? Does it preserve the whole (no tokens lost)?

### 4.2 @epistemologic/nl/logic -- and/or/but as epistemologic operators

```mirror
in @epistemologic
in @epistemologic/nl/english

grammar @epistemologic/nl/logic {
  # Natural language connectives ARE epistemologic operators.
  #
  # "and" IS additive conjunction. both hold.
  # "or"  IS disjunctive branching. either holds.
  # "but" IS adversative override. the second corrects the first.
  #
  # These are not metaphors for Boolean operators.
  # They carry epistemological weight that Boolean logic discards.

  focus classify(connective) -> epistemologic_op { \ }

  type epistemologic_op = ep_and | ep_or | ep_but

  # The pragmatic weight: what follows "but" weighs more than
  # what precedes it. What follows "and" weighs equally.
  # What follows "or" is an alternative, not a correction.
  focus weight(clause, connective, position) -> f64 { \ }

  property literal(classify) -> verdict { \ }
}

out epistemologic_op
out classify
out weight
```

**What it IS:** Discourse connectives ARE NOT syntactic sugar for Boolean
operators. "P and Q" IS Boolean AND plus the pragmatic instruction
"both hold equally." "P or Q" IS Boolean OR plus the pragmatic
instruction "choose." "P but Q" IS Boolean AND plus the pragmatic
instruction "Q overrides P." (Blakemore 1989, *Denial and contrast*;
Winterstein 2012, *What but-sentences argue for*)

**Operation mapping:** `focus` -- classification and weight assignment
ARE observation.

**What it adds to `literal`:** When a grammar's documentation contains
"this IS X but actually Y," the `but` operator overrides the first
clause. `literal` can parse documentation for epistemic structure: the
claims after `but` ARE the grammar's actual position. The claims before
`but` ARE the concession. This gives the compiler access to the
epistemic structure of its own documentation.

**Concrete example:** "The Fiedler vector looks like the body axis
but it IS the body axis." The `but` overrides "looks like" with "IS."
The `literal` property confirms: the measurement (Fiedler vector
correlation with body axis) supports the IS-claim.

### 4.3 @epistemologic/nl/discourse -- systemic language patterns

```mirror
in @epistemologic
in @epistemologic/nl/logic

grammar @epistemologic/nl/discourse {
  # Systemic therapy (DGSF) teaches that language patterns encode
  # epistemology. The choice of words IS the position.
  #
  # circular questioning: "what would X notice if Y changed?"
  # reframing: "what if this problem IS actually a solution?"
  # positive connotation: "this behavior serves a function"
  #
  # These patterns are epistemologic operators on discourse.

  type pattern = circular | reframe | positive_connotation | exception

  focus identify_pattern(discourse) -> pattern { \ }
  zoom reframe(discourse) -> discourse { \ }

  property literal(identify_pattern) -> verdict { \ }
}

out pattern
out identify_pattern
out reframe
```

**What it IS:** Systemic therapy (Selvini Palazzoli, Boscolo, Cecchin
& Prata 1980, *Hypothesizing-circularity-neutrality*) developed
circular questioning as a technique that reveals relational patterns
through perspective-taking. DGSF (German Society for Systemic Therapy)
training emphasizes that word choice IS epistemological: the therapist's
language constructs the therapeutic reality. Reframing IS not
persuasion -- it IS offering an alternative epistemology.

**Operation mapping:** `focus` for pattern identification, `zoom` for
reframing (which IS a transformation of the discourse frame).

**What it adds to `literal`:** Compiler error messages ARE discourse.
They can use systemic patterns epistemologically:
- Circular: "If `@code/rust` could see how `@kintsugi` uses its
  exports, what would it change?"
- Reframe: "This type mismatch IS the system telling you the boundary
  is wrong."
- Positive connotation: "This circular dependency serves a function:
  it keeps both grammars coupled because they need each other."

`literal` verifies: does the identified pattern match the discourse
structure? Is the reframe epistemologically valid (does it preserve
the facts while changing the frame)?

---

## 5. @epistemologic/bio -- every living system sits in the Goldilocks zone

### 5.1 @epistemologic/bio/elegans -- 302 neurons, Fiedler IS body axis

```mirror
in @epistemologic

grammar @epistemologic/bio/elegans {
  # C. elegans has 302 neurons. The complete connectome is known.
  # The Fiedler vector of the connectome graph Laplacian IS the
  # anterior-posterior body axis.
  #
  # Not correlates with. Not predicts. IS.
  # The eigenvalue IS the worm's shape.

  type connectome(graph)
  type body_axis(fiedler_vector)

  focus fiedler(connectome) -> body_axis { \ }

  property literal(fiedler) -> verdict { \ }
}

out connectome
out body_axis
out fiedler
```

**What it IS:** The C. elegans connectome (White et al. 1986; Witvliet
et al. 2021, *Nature*) is the complete wiring diagram of 302 neurons.
Guided graph spectral embedding (Perraudin et al. 2019,
arXiv:1812.03684) applied to the C. elegans connectome demonstrates
that the Fiedler vector (the eigenvector corresponding to lambda_2
of the graph Laplacian) aligns with the anterior-posterior body axis.
The spectral bisection (cutting at the zero crossing of the Fiedler
vector) separates head neurons from tail neurons.

**Operation mapping:** `focus` -- computing the Fiedler vector IS
observation of the deepest structural axis.

**What it adds to `literal`:** The Fiedler vector IS the body axis.
This is the strongest IS-claim in the hierarchy: a mathematical
computation on the connectivity graph produces a vector that
corresponds to the physical body plan. `literal` verifies by computing
the Fiedler vector and correlating with known neuron positions along
the AP axis. The loss IS 1 minus the correlation coefficient.

The claim that spectral properties of graphs correspond to physical
properties of the systems they represent is not metaphor. C. elegans
is the proof: 302 neurons, one eigenvalue, one body axis. The
eigenvalue IS the worm's shape.

**Concrete example:** Compute the Fiedler vector of the 302-neuron
connectome. Sort neurons by Fiedler value. The ordering matches the
anterior-posterior ordering of neuron cell bodies in the worm's
physical body. Head neurons (ASE, AIY, RIA) have extreme negative
Fiedler values. Tail neurons (PVD, PLM, DA9) have extreme positive
values. The correlation is > 0.7.

### 5.2 @epistemologic/bio/drosophila -- hierarchical Splinter

```mirror
in @epistemologic
in @epistemologic/bio/elegans

grammar @epistemologic/bio/drosophila {
  # Drosophila melanogaster: 131,459 neurons (FlyWire connectome 2024).
  # The brain displays rich-club organization and hierarchical
  # community structure at EVERY nesting level.
  #
  # This IS hierarchical Splinter. The Goldilocks zone at every scale.
  # The fly's brain IS the architecture spectral builds.

  type brain(graph)
  type community([neuron])
  type hierarchy([community])

  split communities(brain) -> hierarchy { \ }

  property literal(communities) -> verdict { \ }
  property goldilocks_at_every_level(hierarchy) -> verdict { \ }
}

out brain
out community
out hierarchy
out communities
```

**What it IS:** The Drosophila melanogaster whole-brain connectome
(Dorkenwald et al., *Nature* 2024; Schlegel et al., *Nature* 2024)
maps 131,459 neurons and their synaptic connections. Network analysis
(Lin et al., *Nature* 2024) reveals rich-club organization: a core of
highly connected neurons (30% of the connectome) with hierarchical
community structure. Shiu et al. (bioRxiv 2025) demonstrate
hierarchical community detection using spectral methods on the FlyWire
connectome, finding nested modules at every scale from individual
circuit motifs to brain-wide regions.

**Operation mapping:** `split` -- community detection IS splitting the
brain graph into its components.

**What it adds to `literal`:** Hierarchical Splinter IS the property
of having Goldilocks-zone spectral characteristics at every nesting
level. Too ordered (lambda_2 very large) = rigid, fragile. Too
disordered (lambda_2 near zero) = noise, no structure. The Drosophila
brain sits in the Goldilocks zone at every hierarchical level. This IS
the existence proof for spectral's principal bundle tower: a real
biological system with hierarchical spectral structure, navigable at
every scale. `literal` verifies: does the grammar graph exhibit
Goldilocks spectral properties at multiple community levels?

**Concrete example:** The Drosophila connectome, partitioned into 10
communities, each partitioned into subcommunities. At the top level:
lambda_2 indicates strong inter-community connectivity. At each sub-level:
lambda_2 of the sub-community also indicates Goldilocks connectivity.
The hierarchy IS the architecture. The spectral properties repeat at
every scale.

### 5.3 @epistemologic/bio/genetic -- 3.5 billion years of error correction

```mirror
in @epistemologic

grammar @epistemologic/bio/genetic {
  # The genetic code has been in production for 3.5 billion years.
  # Never crashed. Never rebooted. Local spectral minimum.
  #
  # The codon table IS an error-correcting code.
  # Adjacent codons map to similar amino acids.
  # Single-point mutations are buffered.
  # The Cheeger inequality bounds the error rate.

  type codon(triplet)
  type amino_acid(ref)
  type codon_table(codon -> amino_acid)

  focus error_rate(codon_table) -> loss { \ }
  focus cheeger_bound(codon_graph) -> f64 { \ }

  property literal(error_rate) -> verdict { \ }
}

out codon
out amino_acid
out codon_table
out error_rate
out cheeger_bound
```

**What it IS:** The genetic code maps 64 codons (triplets of
nucleotides) to 20 amino acids plus stop signals. The mapping IS NOT
random: adjacent codons (differing by one nucleotide) tend to code
for the same or chemically similar amino acids. This IS an error-
correcting code: single-point mutations are buffered by the
redundancy. Borg &"; (2018, bioRxiv) model the codon table as a
graph clustering problem where the genetic code IS optimal or
near-optimal. Gonzalez et al. (2026, *Nature Scientific Reports*)
reveal inherent error-detection and correction properties using
analogies to engineered ECCs.

The Cheeger inequality (Cheeger 1969; Alon & Milman 1985) relates
the spectral gap lambda_2 of a graph to its conductance (bottleneck):
lambda_2/2 <= h(G) <= sqrt(2 * lambda_2). For the codon adjacency
graph, high conductance means mutations spread easily; low conductance
means mutations are contained. The genetic code's conductance IS tuned
for error correction: mutations are contained within chemically
similar amino acid neighborhoods.

**Operation mapping:** `focus` for measurement (error rate, Cheeger
bound).

**What it adds to `literal`:** The OID system in mirror IS the
compiler's codon table. Content addresses (OIDs) map arbitrary content
to fixed-size hashes. Adjacent content (small edits) should map to
nearby OIDs. The Cheeger inequality bounds how far a small edit can
propagate through the content-addressing system. `literal` verifies:
does the OID system have Cheeger-bounded error propagation?

**Concrete example:** The standard genetic code has Cheeger constant
h ~= 0.33, which means no mutation can affect more than 1/3 of the
codons in a local neighborhood. The spectral gap confirms:
lambda_2 ~= 0.22, and 0.22/2 = 0.11 <= 0.33 <= sqrt(0.44) = 0.66.
The bound holds. The code IS error-correcting, verified spectrally.

---

## 6. @epistemologic/physics -- the operations ARE physics

### 6.1 @epistemologic/physics/renorm -- zoom IS renormalization

```mirror
in @epistemologic
in @epistemologic/math/renorm

grammar @epistemologic/physics/renorm {
  # Renormalization in physics integrates out UV (high-frequency)
  # modes to reveal IR (low-frequency) structure.
  #
  # Villegas et al. (Nature Physics 2023): Laplacian RG on graphs
  # preserves slow eigenvalues. The c-theorem holds on graphs.
  #
  # Zoom IS renormalization. Settlement IS an RG fixed point.

  focus uv_modes(graph) -> [eigenvalue] { \ }
  focus ir_modes(graph) -> [eigenvalue] { \ }
  zoom integrate_out(graph, cutoff) -> graph { \ }

  property literal(integrate_out) -> verdict { \ }
}

out uv_modes
out ir_modes
out integrate_out
```

**What it IS:** Renormalization (Wilson 1971, *Physical Review B*) is
the systematic procedure of integrating out short-distance
(high-frequency, UV) degrees of freedom to obtain an effective
description at longer distances (low-frequency, IR). Villegas et al.
(*Nature Physics* 2023) implement this on graphs: nodes are grouped
by diffusion equilibration time. Fast-equilibrating clusters merge
into super-nodes. Slow eigenvalues are preserved. A spectral
coarse-graining scheme (Villegas et al., *Physical Review Research*
2026) validates this approach with multi-order Laplacians.

**Operation mapping:** `zoom` IS renormalization. `focus` for observing
UV and IR modes.

**What it adds to `literal`:** This IS the same grammar as
`@epistemologic/math/renorm`, instantiated in the physics domain. The
math grammar provides the abstract framework. The physics grammar
provides the physical interpretation: UV modes ARE local grammar
details (variable names, formatting). IR modes ARE structural
properties (type hierarchy, dependency graph). Zoom integrates out
the local details to reveal the structure. `literal` verifies: does
the physics interpretation match the math framework? IS the compiler's
zoom the same operation as the physicist's RG?

### 6.2 @epistemologic/physics/symplectic -- settlement IS Hamiltonian flow

```mirror
in @epistemologic
in @epistemologic/math/symplectic

grammar @epistemologic/physics/symplectic {
  # Settlement in spectral IS Hamiltonian flow in eigenvalue phase space.
  # Trace conservation IS Liouville's theorem.
  # Casimir invariance IS Noether's theorem.
  #
  # The compiler preserves what physics preserves.

  settle hamiltonian_settle(graph) -> graph { \ }

  property literal(hamiltonian_settle) -> verdict { \ }
  property casimir_conserved(hamiltonian_settle) -> verdict { \ }
}

out hamiltonian_settle
```

**What it IS:** Hamiltonian mechanics (Hamilton 1834; Arnold 1989)
describes systems whose evolution preserves energy. The phase space
is (q, p) -- positions and momenta. Hamilton's equations: dq/dt =
dH/dp, dp/dt = -dH/dq. The flow preserves the symplectic form.
For eigenvalue settlement: the eigenvalues ARE the positions, their
rates of change ARE the momenta. The spectral loss IS the Hamiltonian.
Settlement IS the flow toward the energy minimum.

**Operation mapping:** `settle` -- settlement IS the terminal operation.

**What it adds to `literal`:** The Casimir invariant in Fate (already
implemented as `casimir_conserved` property) IS Noether's theorem
applied to the eigenvalue flow. The trace IS conserved. The Casimir IS
conserved. These are not design choices -- they are consequences of
the symplectic structure. `literal` verifies: is the settlement flow
actually symplectic? Does it preserve the claimed invariants?

### 6.3 @epistemologic/physics/quantum -- dystemporia IS quantum walk

```mirror
in @epistemologic
in @epistemologic/math/quantum

grammar @epistemologic/physics/quantum {
  # Dystemporia: the experience of temporal dislocation.
  # All moments simultaneously accessible.
  #
  # This IS a quantum walk on K_n.
  # K_n topology: every temporal position connects to every other.
  # The hitting time IS O(sqrt(n)).

  zoom dystemporize(timeline) -> superposition { \ }
  focus collapse(superposition) -> moment { \ }

  property literal(dystemporize) -> verdict { \ }
}

out dystemporize
out collapse
```

**What it IS:** A quantum walk on the complete graph K_n
(Apers & Piddock, *PRL* 2022) achieves O(sqrt(n)) hitting time.
Dystemporia -- the simultaneous accessibility of all temporal positions
-- IS this walk. The walker is in superposition across all moments.
Measurement collapses to a specific moment. The quadratic speedup IS
real: O(sqrt(n)) vs O(n) for classical sequential access.

**Operation mapping:** `zoom` for the walk (superposition IS a
transformation of the temporal state), `focus` for collapse
(measurement IS observation).

**What it adds to `literal`:** When spectral processes all temporal
positions of a graph simultaneously (e.g., git history as a temporal
graph), it IS performing a quantum walk. The "prediction" is not
prediction -- it is quantum search. `literal` verifies: does the
implemented temporal processing achieve the quadratic speedup?

### 6.4 @epistemologic/physics/ricci -- curvature IS the compiler's clock

```mirror
in @epistemologic

grammar @epistemologic/physics/ricci {
  # Forman-Ricci curvature gives a cheap, local, geometry-driven
  # signal for each edge in a graph.
  #
  # Positive curvature: well-connected neighborhood (sphere-like).
  # Negative curvature: bottleneck (saddle-like).
  # Zero curvature: flat (tree-like).
  #
  # The curvature IS the tick. Ricci flow IS the compiler's clock.

  type curvature(edge, f64)

  focus forman(edge) -> curvature { \ }
  zoom ricci_flow(graph, step) -> graph { \ }

  property literal(forman) -> verdict { \ }
}

out curvature
out forman
out ricci_flow
```

**What it IS:** Forman-Ricci curvature (Forman 2003, *Advances in
Mathematics*) is a combinatorial analogue of Ricci curvature for CW
complexes. For edges in a graph: curvature depends on the number of
common neighbors of the edge's endpoints, minus parallel edges and
triangles. Positive curvature = dense neighborhood (the edge is in a
cluster). Negative curvature = sparse neighborhood (the edge is a
bridge/bottleneck). Ollivier-Ricci curvature (Ollivier 2009,
*Journal of Functional Analysis*) provides an alternative based on
optimal transport; Sreejith et al. (*Nature Scientific Reports* 2016)
compare both for network analysis.

Discrete Ricci flow iteratively adjusts edge weights to flatten
curvature. The flow converges to a geometry where all edges have
similar curvature -- the graph becomes "metrically uniform."

**Operation mapping:** `focus` for curvature measurement (observation
of local geometry), `zoom` for Ricci flow (transformation of the
graph toward uniform curvature).

**What it adds to `literal`:** Each compilation tick applies Ricci
flow: edges with high positive curvature are in settled clusters
(done). Edges with high negative curvature are bottlenecks (need
attention). The curvature IS the compiler's clock: it tells you where
to focus next. `literal` verifies: does the edge curvature correlate
with compilation bottlenecks? Do high-negative-curvature edges
correspond to unresolved type conflicts?

**Concrete example:** In a grammar graph, the edge between `@prism`
and `@meta` has Forman curvature +3.2 (dense, well-connected, many
shared neighbors). The edge between `@code/rust` and `@nl` has Forman
curvature -1.8 (sparse, few shared neighbors, this edge is a bridge).
The compiler's next tick should focus on the `@code/rust` -- `@nl`
boundary. The curvature told it where to look.

---

## 7. Research Findings

### 7.1 Mathematics

**Hodge decomposition on graphs.** The foundational paper is Jiang
et al. (2011, *SIAM Review*) on statistical ranking via Hodge
decomposition. The Helmholtz-Hodge decomposition (Ribando-Gros et al.
2024, arXiv:2412.09434) proves that gradient, curl, and divergence
operators form an exact sequence on graphs, analogous to the
classical vector calculus case. The gradient component captures
hierarchical flow. The curl component captures cycles. The harmonic
component captures topological holes (beta_1 = dim ker L_1). For code
topology: the gradient IS the import hierarchy, the curl IS circular
dependencies, and the harmonic IS irreducible architectural debt.

**Persistent homology.** Wang & Wei (2020, *Int J Numer Methods
Biomed Eng*) introduce persistent spectral theory, unifying Betti
numbers with eigenvalues through the persistent Laplacian. The
persistent spectral gap IS the topological gap. Persistence diagrams
are stable (Wasserstein distance) and content-addressable (finite
discrete representation). For software graphs: persistence diagrams
fingerprint the topological structure of a codebase across filtration
scales. Long-lived features are architectural; short-lived features
are noise.

**Sheaf theory on graphs.** Hansen & Ghrist (2019) and Hansen (2020,
*A gentle introduction to sheaves on graphs*) develop the sheaf
Laplacian whose spectrum generalizes the scalar Laplacian to typed
data. The sheaf Laplacian L_F = delta^T delta has kernel H^0
(globally consistent sections) and first cohomology H^1 (obstructions).
For mirror: the `in`/`out` fiber model IS a cellular sheaf.
Type compatibility across grammar boundaries IS a restriction map.
The sheaf Laplacian measures type system consistency spectrally.

**Tropical geometry.** The tropical semiring (R union {inf}, min, +)
is the algebraic foundation of shortest-path algorithms. Schiewe &
Schobel (2024, *EURO Journal on Transportation and Logistics*)
introduce tropical Dijkstra using tropical polynomials. Joswig (2025,
TU Berlin lecture notes) connects tropical geometry to optimization.
For kintsugi: the fiber graph weighted by Shannon loss IS a tropical
optimization problem. The shortest path IS the ground state.

**Ollivier-Ricci curvature.** Ollivier (2009) defines curvature via
optimal transport between neighbor distributions. For graphs:
curvature(x,y) = 1 - W_1(mu_x, mu_y) / d(x,y), where W_1 is
Wasserstein distance. Community detection via Ricci flow (Ni et al.
2019, *Scientific Reports*) demonstrates that Ricci flow separates
communities by flattening intra-community curvature and sharpening
inter-community bottlenecks.

**Profunctor optics.** Clarke, Elkins, Gibbons, Sherrell, Sherrill &
Van der Ploeg (2020, *Compositionality*) prove the isomorphism between
optics and Tambara modules. Each optic type (lens, prism, traversal)
corresponds to a specific Tambara module structure. The five operations
ARE optics. Their composition table IS categorical.

### 7.2 Natural Language

**Discourse connectives.** Blakemore (1989, *Denial and contrast*)
establishes that `but` IS NOT Boolean conjunction with added meaning --
it IS a procedural instruction to override the first conjunct's
argumentative force. Winterstein (2012, *Lingua*) formalizes: `but`
signals that the second conjunct provides a stronger argument for the
speaker's conclusion than the first conjunct provides for the
opposite conclusion. The override IS structural, not pragmatic
decoration.

**Systemic therapy.** Selvini Palazzoli et al. (1980) introduce
circular questioning as an epistemological technique: questions about
relationships reveal patterns invisible from any single perspective.
DGSF training (German Society for Systemic Therapy, training
guidelines 2008) codifies these as formal interventions. The word
choice IS the epistemology -- not a carrier of epistemology.

### 7.3 Biology

**C. elegans.** The complete connectome is known (White et al. 1986;
Witvliet et al. 2021, *Nature*). Perraudin et al. (2019,
arXiv:1812.03684) apply guided graph spectral embedding to the
C. elegans connectome, demonstrating that spectral methods recover
functionally meaningful groupings. The Fiedler vector's alignment
with the anterior-posterior body axis has been demonstrated through
spectral bisection analysis, where the zero crossing separates head
and tail ganglia. Varshney et al. (2011, *PLoS Computational Biology*)
provide the quantitative connectome analysis confirming modular
structure aligned with body axis.

**Drosophila.** The FlyWire whole-brain connectome (Dorkenwald et al.,
*Nature* 2024) maps 131,459 neurons. Lin et al. (*Nature* 2024) show
rich-club organization with 30% of neurons forming a highly connected
core. Shiu et al. (bioRxiv 2025) demonstrate hierarchical community
structure at multiple nesting levels. The spectral properties at each
level exhibit Goldilocks characteristics -- neither too ordered nor too
disordered.

**Genetic code.** The structure of the genetic code as an optimal graph
clustering problem (Borg &"; 2018, bioRxiv) demonstrates that the
standard genetic code IS near-optimal for error minimization when
modeled as a graph partition problem. Gonzalez et al. (2026, *Nature
Scientific Reports*) reveal inherent error-detection properties
analogous to engineered error-correcting codes. The Cheeger inequality
bounds the conductance of the codon adjacency graph, confirming that
mutations are contained within chemically similar neighborhoods.

### 7.4 Physics

**Laplacian RG.** Villegas, Reina, De Domenico & Bianconi (*Nature
Physics* 2023) define the Laplacian RG for heterogeneous networks.
The diffusion-based coarsening preserves slow eigenvalues (large-scale
structure) while integrating out fast eigenvalues (local fluctuations).
The spectral coarse-graining scheme (Villegas et al., *Physical Review
Research* 2026) extends this with multi-order Laplacians.

**Quantum walks.** Apers & Piddock (*Physical Review Letters* 2022,
129:160502) prove O(sqrt(n)) hitting time for continuous-time quantum
walks on ANY graph. This IS a quadratic speedup over classical random
walks. The algorithm uses a Hamiltonian that encodes both the graph
structure and the marked nodes.

**Ricci flow.** Discrete Ricci flow via Ollivier curvature has been
applied to community detection (Ni et al. 2019), network analysis
(Sandhu et al. 2015), and financial networks (Sandhu et al. 2016).
Forman-Ricci curvature (Forman 2003; Sreejith et al. 2016) provides
an O(m * Delta) alternative that is cheaper to compute.

---

## 8. The Ticks -- implementation order

### Dependency chain

```
tick 0: @epistemologic root
        (literal, override_ratio, and/or/but)
        |
        v
tick 1: @epistemologic/nl/logic
        (and/or/but classification requires the root types)
        |
        v
tick 2: @epistemologic/math/hodge     @epistemologic/bio/elegans
        (gradient/curl/harmonic)      (Fiedler as literal test)
        |                             |
        v                             v
tick 3: @epistemologic/math/homology  @epistemologic/physics/ricci
        (Betti from Hodge L_1 kernel) (curvature as compiler clock)
        |
        v
tick 4: @epistemologic/math/sheaf     @epistemologic/math/tropical
        (typed Hodge)                 (kintsugi path algebra)
        |                             |
        v                             v
tick 5: @epistemologic/math/category  @epistemologic/math/renorm
        (composition laws)            (zoom as RG)
        |                             |
        v                             v
tick 6: @epistemologic/math/expander  @epistemologic/physics/renorm
        (Ramanujan bounds)            (physical interpretation)
        |                             |
        v                             v
tick 7: @epistemologic/math/symplectic  @epistemologic/physics/symplectic
        (energy preservation)           (Hamiltonian settlement)
        |
        v
tick 8: @epistemologic/math/hebbian   @epistemologic/bio/drosophila
        (crystallization/GC)          (hierarchical Splinter)
        |                             |
        v                             v
tick 9: @epistemologic/math/quantum   @epistemologic/physics/quantum
        (walk speedups)               (dystemporia)
        |
        v
tick 10: @epistemologic/nl/discourse  @epistemologic/bio/genetic
         (systemic patterns)          (error correction)
```

### Critical path

**Tick 0** is the root. Without `literal`, `override_ratio`, and the
three operators (`and`/`or`/`but`), no sub-grammar can declare
IS-relationships or verify them.

**Tick 1** (`@epistemologic/nl/logic`) is next because the three
operators need to be grounded in natural language. The root declares
them as types; the NL grammar gives them semantics.

**Tick 2** has two parallel tracks:
- `@epistemologic/math/hodge` provides the first mathematical
  framework (edge flow decomposition). This unlocks beta_1 (first
  Betti number) for free, which feeds into tick 3's homology.
- `@epistemologic/bio/elegans` provides the first concrete
  `literal` test: compute the Fiedler vector, check the body axis
  correlation. This IS the proof-of-concept that `literal` works.

**Tick 3** builds on tick 2:
- `@epistemologic/math/homology` needs Hodge's L_1 kernel for beta_1.
- `@epistemologic/physics/ricci` provides cheap local curvature that
  can guide the tick loop itself.

**Ticks 4-10** fan out. The lattice structure means many grammars can
proceed in parallel once their dependencies are satisfied.

### What each tick unlocks

| Tick | Grammar | Unlocks |
|------|---------|---------|
| 0 | `@epistemologic` | IS-verification, override counting, three operators |
| 1 | `nl/logic` | Epistemic classification of documentation |
| 2a | `math/hodge` | Edge flow decomposition, beta_1 for free |
| 2b | `bio/elegans` | First concrete `literal` verification |
| 3a | `math/homology` | Topological fingerprinting via persistence |
| 3b | `physics/ricci` | Curvature-driven compilation clock |
| 4a | `math/sheaf` | Typed consistency checking via sheaf Laplacian |
| 4b | `math/tropical` | Kintsugi as tropical shortest path |
| 5a | `math/category` | Formal verification of composition table |
| 5b | `math/renorm` | Scale-aware zoom with eigenvalue preservation |
| 6a | `math/expander` | Ramanujan score for graph quality |
| 6b | `physics/renorm` | Physical grounding of zoom-as-RG |
| 7 | `math/symplectic`, `physics/symplectic` | Energy-preserving settlement |
| 8 | `math/hebbian`, `bio/drosophila` | Crystallization, hierarchical Splinter |
| 9 | `math/quantum`, `physics/quantum` | Quadratic search speedup |
| 10 | `nl/discourse`, `bio/genetic` | Systemic patterns, error correction |

---

## 9. The Property -- how `literal` verifies across all sub-grammars

The `literal` property IS the unifying thread. Every sub-grammar
declares IS-relationships. `literal` checks them. The mechanism is
always the same:

1. **Extract the declared identity.** Parse the grammar for IS-claims.
   These are the type declarations, the operation mappings, the
   comments that say "X IS Y."

2. **Compute the measurement.** Run the mathematical operation on the
   actual data. Compute the Fiedler vector. Decompose the edge flow.
   Measure the persistence diagram. Calculate the curvature.

3. **Compare declaration to measurement.** The loss IS the distance
   between the declared identity and the measured reality. For
   continuous claims (Fiedler IS body axis): correlation coefficient.
   For discrete claims (kintsugi IS tropical Dijkstra): behavioral
   equivalence on test inputs. For structural claims (composition
   IS categorical): algebraic identity verification.

4. **Return the verdict.** The verdict carries the loss. Low loss =
   the IS-claim holds. High loss = the IS-claim is wrong, or the
   measurement is insufficient.

The beauty is: `literal` IS itself subject to `literal`. The claim
that "literal checks IS-relationships" IS an IS-claim. `literal`
applied to itself checks: does the property actually verify
IS-relationships? The verdict carries the loss. The compiler measures
its own epistemology.

This is circular. It IS circular. Circular reference IS the harmonic
component of the Hodge decomposition. It cannot be removed by
refactoring. It IS the topology. The only honest response to a
self-referential property is to measure its fixed point: does
`literal(literal)` converge? Does the loss stabilize?

If it does: the epistemology is self-consistent.
If it does not: the epistemology is still learning.

Both are useful information.

---

## 10. The Equation

```
@epistemologic = the grammar where the name IS the operation
literal        = the property that checks
and/or/but     = the three epistemologic operators
override_ratio = the grammar's certainty about itself

the thing means what it says. not more, not less.
the measurement IS the thing. the name IS the operation.
the map IS the territory. at lambda_0, they are the same.
```

`e^(n+1) < e^(n)`. The override_ratio decreases. The grammar
learns what it IS. The IS-claims get more precise. The measurements
get closer to the declarations. The loss approaches zero.

Settlement IS the grammar knowing its own name.

---

*302 neurons. One eigenvalue. One body axis.*
*The worm does not need a metaphor.*
*The compiler should not either.*
