# @mirror/trace + @mirror/kintsugi -- the observation surface and repair pipeline

*2026-05-18. Reed. Spec.*

Status: **Spec** (definitive reference, no implementation)

Depends on: @prism (five operations), @epistemologic (literal, and/or/but),
@epistemologic/property (verdict type), @loss (measurement types), @error
(diagnostic types), @imperfect (three-state container), @kintsugi (collapse),
@kintsugi/shatter (recursive fracture), @mirror/refract (bench as measurement),
@ai/fate (tournament resolution), dirac.rs (spectral triple, Fiedler vector)

---

## 0. The Pipeline

```
trace -> kintsugi -> refract
observe -> act -> measure
ODA
```

The compiler does not produce warnings. It produces measurements. The
measurements form a surface. Kintsugi breaks along that surface.

`@mirror/trace` observes the grammar graph: holes, topology, fractures,
dependencies, loss gradient. It enumerates what the compiler knows it does
not know.

`@mirror/kintsugi` acts on the trace: fills holes, breaks and repairs,
translates, migrates, lifts. It changes the grammar toward lower loss.

`@mirror/refract` measures the result: did loss decrease? Which dualities
shifted? What does the gutter show?

One tick of the pipeline: trace -> kintsugi -> refract. Repeat until loss
stabilizes. The crystal forms when `e^(n+1) >= e^(n)`.

---

## 1. @mirror/trace -- the observation surface

### 1.1 What it observes

The trace enumerates everything the compiler knows it does not know, and
everything it can measure about what it does know.

**`\` holes: every unresolved intent in the grammar.**
Every `ZoomNode` with `is_abstract == true` is a `\` hole. The trace
enumerates them: name, location, input type, output type, parent grammar.
Each hole is a point on the observation surface. The hole's weight is the
Shannon loss between its input and output types -- how much information
must be created to fill it.

```mirror
in @prism
in @loss
in @epistemologic/property

grammar @mirror/trace {
  # enumerate all \ holes in the grammar graph
  holes(ast) -> [hole] { \ }

  type hole = {
    name: ref,
    location: location,
    input_type: ref,
    output_type: ref,
    grammar: ref,
    loss: loss,
    blocked_by: [ref],
  }
```

**Topology: the five dualities.**
The trace computes the five measurements that form the Void geometry
(Splinter <-> Narcissus) applied to the grammar graph:

| Duality | Measurement | What it sees |
|---------|-------------|-------------|
| entropy | Von Neumann entropy of the normalized graph Laplacian | order <-> disorder. Low = rigid/crystallized. High = chaotic |
| spectral | Fiedler value (lambda_2 of graph Laplacian) | connectivity <-> fragility. High = robust mesh. Low = fragile |
| cheeger | Cheeger constant (minimum edge boundary ratio) | flow <-> bottleneck. High = smooth data flow. Low = single point of failure |
| ricci | Forman-Ricci curvature per edge | expansion <-> contraction. Positive = well-connected cluster. Negative = bridge/bottleneck |
| mixing | Random walk mixing time | reachability <-> isolation. Fast = everything accessible. Slow = dead code, silos |

These are not metaphors. They are computed from the grammar graph's Laplacian.
The grammar graph is constructed from `.mirror` files: nodes are types and
actions, edges are references (type usage, action signatures, imports). The
Dirac operator (`construct_dirac()` in `dirac.rs`) builds the spectral triple.
The Laplacian is D^2.

```mirror
  # the five topology measurements
  type topology = {
    entropy: f64,
    spectral: f64,
    cheeger: f64,
    ricci: [(ref, ref, f64)],
    mixing: f64,
  }

  topology(ast) -> topology { \ }
```

**Fractures: Fiedler zero crossings (Sollbruchstellen).**
The Fiedler vector -- the eigenvector corresponding to lambda_2 of the graph
Laplacian -- partitions the grammar graph at its weakest join. Zero crossings
in the Fiedler vector are the natural fault lines. Not imposed. Found.

These are the Sollbruchstellen: designed break points. The grammar graph has
natural seams where coupling is weakest. The trace enumerates them: which
edges cross zero in the Fiedler vector, what loss would result from cutting
there, what sub-graphs would form.

In the spectral triple (A, H, D):
- A = the grammar declarations
- H = the type hierarchy (the fiber of possible implementations)
- D = the Dirac operator built from the declaration graph

The Fiedler zero crossings of D bisect H. That is where `--shatter` will cut.

```mirror
  # Sollbruchstellen: natural fracture points
  type fracture = {
    edge: (ref, ref),
    fiedler_value: f64,
    loss_if_cut: loss,
    left_subgraph: [ref],
    right_subgraph: [ref],
  }

  fractures(ast) -> [fracture] { \ }
```

**Dependencies: which holes block which other holes.**
Holes are not independent. A `\` in `@mirror/resolve` blocks `@mirror/check`
which blocks `@mirror/runtime`. The trace computes the dependency graph of
holes: which hole must be filled before which other hole can be attempted.

```mirror
  # dependency graph of \ holes
  dependencies(holes: [hole]) -> [(ref, ref)] { \ }
```

**Loss gradient: which hole has the shortest path to resolution.**
Not all holes are equally expensive. The trace computes the tropical shortest
path from the current state to each hole's resolution. The hole with lowest
cost is the one kintsugi should fill next. This IS tropical Dijkstra on the
fiber graph weighted by Shannon loss.

```mirror
  # loss gradient: holes ordered by cost to fill
  gradient(holes: [hole]) -> [hole] { \ }
```

The full trace type:

```mirror
  type trace = {
    holes: [hole],
    topology: topology,
    fractures: [fracture],
    dependencies: [(ref, ref)],
    gradient: [hole],
    total_loss: loss,
  }

  # the full observation surface
  observe(ast) -> trace { \ }
}

out hole
out topology
out fracture
out trace
out observe
```

### 1.2 How trace maps to @epistemologic

Each observation in the trace IS a property check. The trace is not a
separate system from the property layer -- it IS the property layer applied
to the grammar graph as a whole.

| Trace observation | Property check | Verdict meaning |
|-------------------|---------------|-----------------|
| `\` hole exists | `unresolved_intent` | fail: the grammar has an unfilled hole |
| Fiedler value < threshold | `fragile_connectivity` | partial: the graph is weakly connected |
| Cheeger constant < threshold | `bottleneck_detected` | partial: information flow has a chokepoint |
| Ricci curvature negative on edge | `bridge_edge` | partial: this edge is load-bearing |
| Mixing time > threshold | `isolated_component` | partial: parts of the graph are unreachable |
| Hole blocks another hole | `blocking_dependency` | fail: resolution order is constrained |

Each property returns a `verdict` (from `@epistemologic/property`):
- `pass` -- the property holds
- `fail(diagnostic)` -- the property does not hold
- `partial(f64, [diagnostic])` -- the property partially holds, with confidence

The verdict IS the measurement. The measurement IS the trace surface.

The `reflect(ast) -> [verdict]` action in `@epistemologic/property` runs all
inherited property checks. The trace IS the reflect applied at the grammar
graph level, collecting topology measurements alongside per-node property
verdicts.

### 1.3 The Turing-eigenvalue connection

The trace surface is built from eigenvalues of the graph Laplacian. This
is the same operator Turing used in 1952.

**The thread: Turing (1952) -> Fiedler (1973) -> Connes (1994) -> mirror (2026).**

Turing's 1952 morphogenesis paper analyzes a ring of N cells -- a discrete,
finite system. The diffusion of morphogen X from cell r to its neighbors is:

```
D_X * (X_{r+1} - 2*X_r + X_{r-1})
```

This is the discrete Laplacian. In matrix form, it IS the graph Laplacian
L = D - A of the cycle graph C_N. Turing diagonalized it using the discrete
Fourier transform and showed that the eigenvalues determine which spatial
patterns emerge. Modes with positive growth rate produce visible patterns.
Modes with negative growth rate decay. The eigenvalues ARE the selection
mechanism.

Fiedler (1973) studied the same matrix on arbitrary graphs. The second-smallest
eigenvalue -- lambda_2, the Fiedler value -- measures connectivity. The Fiedler
vector partitions the graph at its weakest join. Independent discovery, same
operator.

Connes (1994/1996) unified this through the spectral triple (A, H, D). The
Dirac operator D generalizes the Laplacian: D^2 = Hodge Laplacian. From D
alone, you recover distance, dimension, volume, curvature, and action. For
graphs, D = d + d* where d is the signed incidence matrix. The Laplacian that
Turing used and Fiedler studied is the square of the Dirac operator.

mirror (2026) applies this to code topology. The grammar compiles to a
content-addressed graph. The graph Laplacian's eigenvalues measure structural
properties: connectivity (Fiedler), complexity (entropy), similarity (spectral
distance). The grammar is sub-Turing (decidable by construction). The
decidability means verification is possible. The spectrum provides quantitative
measurement of the verified structure.

**The connection to decidability:**

This is the synthesis Turing did not make -- though he held all the pieces.
In 1949, he proposed program verification through structural decomposition
("Checking a Large Routine"). In 1952, he used eigenvalues as a selection
mechanism. He held the halting problem (1936), verification (1949), and
eigenvalue analysis (1952) simultaneously. No surviving evidence shows he
saw the connection.

The connection is: **decidable structures have measurable spectra, and spectra
provide quantitative tools for working with decidable structures.** The
decidability comes from the grammar restriction (sub-Turing). The eigenvalues
come from the graph structure. The two compose because both operate on the
same finite, analyzable object.

Specifically for the trace:
- The grammar is sub-Turing, so every property check terminates
- The grammar graph is finite, so its Laplacian has finitely many eigenvalues
- The eigenvalues select which modes survive (Turing's insight)
- The Fiedler vector partitions at the weakest join (Fiedler's insight)
- The Dirac operator reconstructs geometry from the spectrum (Connes' insight)
- The trace surface IS the eigenvalue spectrum interpreted as observation

The compiler's `\` holes are the "unstable modes" of Turing's system. Modes
with positive growth rate in Turing's reaction-diffusion correspond to
unresolved holes in the grammar -- the points where the system has not yet
reached equilibrium. Kintsugi fills the holes. Refract measures whether the
loss decreased. Settlement IS the point where no eigenmode is growing.

### 1.4 The Void duality in the trace

The Void geometry defines the poles of the observation surface.

**Narcissus (star graph K_{1,n-1}):** minimum entropy, vanishing spectral gap,
bottleneck, negative Ricci curvature, slow mixing. A grammar graph that is
Narcissistic has a single hub node that everything depends on. One point of
failure. Peripheral nodes are interchangeable (the supply is fungible). The
spectral ratio lambda_{n-1}/lambda_1 diverges with size.

**Splinter (complete graph K_n):** maximum entropy, maximal spectral gap, no
bottleneck, positive Ricci curvature, fast mixing. A grammar graph that is
Splinter has every node connected to every other. No domination. All nodes
structurally identical. Spectral ratio = 1.

Every grammar graph lives on a path between these poles. The trace surface
measures WHERE on that path the grammar sits, across all five dualities
simultaneously.

| Trace measurement | Narcissus pole | Splinter pole | Grammar health |
|-------------------|---------------|---------------|----------------|
| Von Neumann entropy | ~ (1/2)log_2(n) + 1/2 | log_2(n-1) | Higher = more distributed |
| Spectral gap | ~ 0 (fragile) | ~ n (robust) | Higher = better connected |
| Cheeger constant | ~ 0 (bottleneck) | ~ n/2 (no bottleneck) | Higher = better flow |
| Ricci curvature | Negative (hyperbolic) | Positive (spherical) | More positive = healthier clusters |
| Mixing time | O(n) (slow) | O(log n) (fast) | Faster = more reachable |

The Narcissus detection battery applies to grammar graphs:

| Metric | Narcissistic grammar | Healthy grammar |
|--------|---------------------|-----------------|
| Betweenness centralization | > 0.7 | < 0.3 |
| Degree Gini | > 0.6 | < 0.3 |
| Spectral ratio | > n/2 | < 10 |
| Von Neumann entropy | < log_2(n)/2 + 1 | > log_2(n) - 1 |
| Clustering coefficient | < 0.05 | > 0.1 |

A grammar where everything imports `@prism` and nothing imports each other
is a star graph. Narcissistic. Fragile. The trace detects this and reports
it as a topology measurement, not a warning.

The transformation from Narcissus toward Splinter is discrete Ricci flow --
redistributing connectivity away from the hub. Kintsugi's `--shatter` mode
performs this transformation: it finds the hub (the high-betweenness node),
fractures there, repairs each piece independently, and reassembles. The
reassembled graph has lower betweenness centralization. The topology shifted
toward Splinter. The trace measures the shift.

---

## 2. @mirror/kintsugi -- the repair pipeline

Kintsugi acts on the trace surface. It has five modes, each a different
operation at a different scale.

### 2.1 Default mode: fill `\` holes

The default. One tick: resolve the hole with lowest loss to fill (from the
trace's loss gradient). The resolution is `collapse(ast, ast) -> imperfect`.

```
tick 0: observe trace → pick hole with lowest gradient cost
        collapse(ast, ast) → ast' with fewer \ holes
        commit to git

tick 1: observe trace → pick next lowest-cost hole
        collapse(ast', ast') → ast'' with even fewer holes
        commit to git

...

tick n: loss(n) >= loss(n-1) → stop. crystal.
```

Each tick:
1. The trace re-observes the grammar graph (new surface after last fill)
2. The loss gradient re-orders holes by cost
3. Kintsugi picks the cheapest hole
4. Fate runs a tournament on the hole: five models (Abyss, Introject,
   Cartographer, Explorer, Fate) compete to fill it
5. The winning resolution is written back to the `.mirror` file
6. Git commit. One commit per tick. The git log IS the trace history.
7. Refract measures: did loss decrease?

**Partial resolution:** A `\` can become structure + more `\`. Filling one
hole may reveal smaller holes inside it. This is expected. The total loss
must decrease even if the hole count increases. What matters is the integral,
not the count.

**Termination:** Loss is monotonically non-increasing. If no tick reduces
loss, the system has reached its local ground state. The holes that remain
are the ones Fate cannot resolve from the current grammar context. They need
more grammar, not more ticks.

### 2.2 --shatter mode: recursive fracture

`mirror kintsugi boot/* --shatter` operates on the GRAPH, not the file.
It finds where the graph is weakest -- the Sollbruchstellen from the trace
surface -- and fractures there.

**The pipeline:**

```
1. Parse target grammars into unified grammar graph.

2. Build the spectral triple: construct_dirac(nodes, edges).

3. Find Sollbruchstellen: compute Fiedler vector, locate zero crossings.
   Each zero crossing is a natural fracture point.

4. Break there. Each piece is a sub-graph with its own spectral structure.

5. RECURSE: if level > 0, shatter each piece at ITS zero crossings.
   Apply the same pipeline to each sub-graph.

6. Bottom level: no more recursion. Each atom is a minimal grammar fragment.
   Run Fate tournament on each atom independently.
   Fate fills \ holes in each atom using only local context.

7. Settle up (bottom-up):
   a. Reassemble atoms into sub-graphs.
   b. Fate tournament on each sub-graph: does the local-optimal
      composition hold globally within the sub-graph?
   c. Reassemble sub-graphs into pieces.
   d. Fate tournament on each piece.

8. Final tick: Fate tournament on the WHOLE reassembled graph.
   Where local optima conflict, Fate decides. The seam loss is measured.
```

**Recursion depth:**

```
mirror kintsugi boot/* --shatter          → 3 levels (default, 8 atoms)
mirror kintsugi boot/* --shatter 5        → 5 levels (32 atoms)
mirror kintsugi boot/* --shatter 1        → 1 level (2 atoms)
```

**The seam loss:**

```
seam_loss = loss(global_settlement) - sum(loss(local_settlements))
```

If `seam_loss = 0`: the pieces were already globally optimal in isolation.
The fractures reveal genuine modularity.

If `seam_loss > 0`: the local repairs conflict at the joins. The gold fills
the seams: the information in the difference IS the kintsugi. High seam loss
means the grammar has load-bearing cross-cutting concerns. Low seam loss
means the grammar is well-modularized at its natural fault lines.

The seam loss IS the kintsugi gold. It is not a failure metric. It is a
discovery metric. The crystal stores both the repair and the seam.

**Complexity:** O(n log n). Fiedler bisection halves the graph at each level.
Tournaments parallelize per level. Settling is sequential bottom-up.

### 2.3 --lift mode: Rust to grammar

`mirror kintsugi src/tokenize.rs --lift` reads Rust source through the
`@code/rust` lens and emits `.mirror` grammar. The loss is the `@io`
residual -- the parts of Rust that require IO boundary crossing and cannot
be expressed as pure grammar operations.

```mirror
# from boot/std/kintsugi/lift.mirror:
lift(ast, target) -> imperfect { \ }
```

Lift is the inverse of code generation. Where `@code/llvm.compile(ast) ->
artifact` turns grammar into native code, `lift` turns native code into
grammar. The `imperfect` return type carries the lifted grammar plus loss
(what could not be lifted: raw pointers, unsafe blocks, platform-specific
IO).

### 2.4 --translate mode: grammar to grammar

`mirror kintsugi file.mirror --translate @code/gleam` translates grammar
from one code lens to another. The source lens reads. The target lens writes.
Fate settles the semantic gaps.

```mirror
# from boot/std/kintsugi/translate.mirror:
translate(ast, grammar) -> imperfect { \ }
```

The loss in translation is the information destroyed when moving between
type systems. `@code/rust` has lifetimes. `@code/gleam` does not. The
lost information IS the translation loss, measured in Shannon bits.

### 2.5 --migrate mode: old syntax to new syntax

`mirror kintsugi boot/05-property.mirror --migrate` updates grammars from
old syntax to new. `template -> lambda`. `action -> lambda`. `-- -> #`.
`keywords -> import graph`. The migration IS the collapse.

```mirror
# from boot/std/kintsugi/migrate.mirror:
migrate(ast) -> imperfect { \ }
```

Migration is the most common kintsugi operation: the grammar evolves, old
files need updating, the compiler's own boot grammars drift. Migrate keeps
them current. The loss is the semantic change -- did the migration preserve
meaning?

---

## 3. @mirror/refract -- the measurement

After kintsugi, refract measures. This is the third step: observe -> act ->
measure.

`@mirror/refract` IS bench wearing its real name. The existing `mirror bench`
infrastructure computes spectral measurements of grammar graphs. `refract`
exposes those measurements as verdicts.

```mirror
# from boot/std/mirror/refract.mirror:
grammar @mirror/refract {
  measure(file) -> verdict { \ }
  suite(path) -> [verdict] { \ }
  lens(file, lens) -> verdict { \ }
  query(path, mq) -> verdict { \ }
}
```

### 3.1 After kintsugi: did loss decrease?

The primary question. Compare the trace surface before and after kintsugi:

```
trace_before = observe(ast_before)
kintsugi(ast_before) -> ast_after
trace_after = observe(ast_after)

delta_loss = trace_after.total_loss - trace_before.total_loss
```

If `delta_loss < 0`: loss decreased. The tick was productive. Continue.
If `delta_loss = 0`: loss unchanged. Local ground state reached. Stop.
If `delta_loss > 0`: loss increased. This should not happen. The tick
violated monotonicity. The crystal rejects the tick and rolls back to the
previous git commit.

### 3.2 The five lenses applied to the result

Each lens illuminates one duality from the Void geometry. Same grammar,
different measurement, different light. The gutter renders the result as
color.

| Lens | What it measures | Low (dark) | High (bright) |
|------|-----------------|------------|---------------|
| entropy | Von Neumann entropy | rigid, crystallized | chaotic, unmaintainable |
| spectral | Fiedler value | robust mesh | fragile star |
| cheeger | Cheeger constant | smooth data flow | single point of failure |
| ricci | Forman-Ricci curvature | growing, healthy | collapsing, tightening |
| mixing | Mixing time | everything accessible | dead code, silos |

The default lens is a weighted composition of all five, where the weights
come from the eigenboard. The eigenboard learns which measurements the
engineer responds to. The weights shift. The gutter becomes personalized.

```
mirror compile --lens entropy file.mirror       → one lens
mirror compile --lens entropy,cheeger file.mirror → composed
mirror compile --lens default file.mirror        → all five, weighted
mirror compile file.mirror                       → dark (no lens, not measured)
```

No lens = no measurement = dark gutter. The compiler does not compute what
you did not ask for. The default is: observe nothing. The engineer opts INTO
measurement. The measurement IS the choice.

### 3.3 The gutter rendering

```
Dark      → not measured / unknown
Deep blue → measured, ground state (lambda_0)
Teal      → measured, healthy range
Green     → measured, optimal
Gold      → measured, approaching threshold
Orange    → measured, threshold exceeded
Red       → measured, critical
```

The gradient IS the Void duality rendered as color. Splinter-end (healthy) =
cool colors. Narcissus-end (pathological) = warm colors.

---

## 4. The Math

Each mathematical framework from `@epistemologic/math/*` underpins a specific
operation in the trace -> kintsugi -> refract pipeline.

### 4.1 @math/hodge -- what kintsugi can't fix (harmonic = topological debt)

The Hodge decomposition on graphs (Jiang et al. 2011, Lim 2020) decomposes
any edge flow f into three orthogonal components:

```
f = gradient + curl + harmonic
```

- **Gradient:** the `in` hierarchy. Legitimate data flow. The import graph's
  acyclic component.
- **Curl:** circular grammar references. Dependency loops. A imports B
  imports C imports A.
- **Harmonic:** topological debt. Loops that no refactoring can remove because
  they are homological, not homotopical. dim(ker(L_1)) = beta_1, the first
  Betti number.

**What this means for the pipeline:**

The trace's topology measurement decomposes the grammar graph's edge flow.
The harmonic component IS what kintsugi cannot fix. When the trace reports
a nonzero harmonic component, the compiler is saying: "this problem is
topological. The architecture has a hole. Kintsugi can fill `\` holes and
break at Sollbruchstellen, but it cannot change the topology. The harmonic
residual requires architectural change, not local repair."

This is the honest limit of kintsugi. The gold fills cracks. It does not
fill holes in the topology itself.

### 4.2 @math/tropical -- kintsugi IS tropical Dijkstra

The tropical semiring (R union {inf}, min, +) is the algebraic structure
underlying shortest-path algorithms. "Addition" IS min. "Multiplication"
IS +. Matrix multiplication over the tropical semiring computes all-pairs
shortest paths.

**What this means for the pipeline:**

Kintsugi navigates the space of all implementations satisfying the same
contract (`in`/`out` boundaries) and finds the fiber closest to the ground
state. This IS Dijkstra on a graph where:
- Nodes ARE fibers (implementations)
- Edges ARE transformations (beta-reduction, dead-code elimination, alias collapse)
- Weights ARE Shannon loss (information change between implementations)

The tropical semiring IS the algebra of this search. The golden seam IS the
shortest path from the current implementation to lambda_0.

The trace's loss gradient (`gradient(holes) -> [hole]`) IS the tropical
shortest path ordering: holes ordered by tropical distance from current
state to resolution.

Default kintsugi = tropical Dijkstra on `\` holes, greedy, one pass.
`--shatter` = tropical Dijkstra on the whole graph, with recursive bisection.

### 4.3 @math/renorm -- zoom IS renormalization group flow

The Laplacian Renormalization Group (Villegas et al., Nature Physics 2023)
defines RG flow on graphs. Nodes are grouped when diffusion equilibrates
them at a given timescale. Slow eigenvalues (large-scale structure) are
preserved. Fast eigenvalues (local fluctuations) are integrated out.

**What this means for the pipeline:**

Kintsugi's `--shatter` mode IS renormalization. At each recursion level,
the grammar graph is coarsened: atoms at the bottom level are the "UV modes"
(local details), and the reassembled structure is the "IR mode" (macroscopic
architecture). Settling up is RG flow: local details are integrated out,
global structure is preserved.

The spectral loss (c-function) IS monotonically non-increasing under this
flow -- Zamolodchikov's c-theorem for graphs. Settlement IS the RG fixed
point: the graph that cannot be coarsened further without changing its
macroscopic structure.

Refract measures whether the c-function decreased. If it did, the shatter
was productive. If it did not, the grammar is at its RG fixed point.

### 4.4 @math/sheaf -- in/out IS a sheaf

A cellular sheaf (Hansen & Ghrist 2019) assigns typed vector spaces to graph
elements. Each node gets a vector space F(v). Each edge gets a vector space
F(e). Linear restriction maps enforce type compatibility along edges.

**What this means for the pipeline:**

Mirror's `in`/`out` declarations ARE a sheaf. Each grammar IS a node with a
vector space (its exported types). Each `in` edge IS a restriction map (the
imported grammar's types must be compatible with the importing grammar's
usage). The sheaf Laplacian L_F = delta^T * delta generalizes the scalar
Laplacian to typed data.

The trace's topology measurement includes the sheaf Laplacian. Its kernel
H^0 IS coincidence -- types that agree everywhere. Its first cohomology H^1
IS the obstruction -- types that cannot be made to agree. The trace reports
H^0 and H^1 as part of the observation surface.

Kintsugi's default mode fills `\` holes to make the sheaf more consistent
(reduce H^1). `--shatter` mode fractures along sheaf inconsistencies and
repairs each piece to local consistency. The seam loss IS the remaining H^1
after reassembly.

### 4.5 @math/homology -- holes eigenvalues miss

Persistent homology (Edelsbrunner & Harer 2008) computes topological
invariants -- Betti numbers -- across a filtration of a simplicial complex.

- Betti_0: connected components (how many disconnected grammars)
- Betti_1: loops (cycles that are not boundaries)
- Betti_2: voids (cavities, absent in 1D graphs)

**What this means for the pipeline:**

Eigenvalues alone cannot see all topology. The Fiedler value measures global
connectivity but misses higher-order holes. Persistent homology fills this
gap. The trace includes the persistence diagram: birth-death pairs of
topological features as the grammar graph is filtered by edge weight.

Long-lived features are architectural. Short-lived features are noise.
The persistence diagram IS a content-addressable fingerprint of the grammar
graph's topological structure. Two grammars with the same persistence
diagram have the same topological shape regardless of node labels.

Wang & Wei (2020) unify Betti numbers with eigenvalues through the
persistent Laplacian: the persistent spectral gap IS the topological gap.
This is the deep connection between the trace's eigenvalue measurements and
its topological measurements -- they are different views of the same
persistent structure.

---

## 5. The Grammar Catalog

### 5.1 What exists

| Grammar | File | Status | Dependencies |
|---------|------|--------|-------------|
| `@kintsugi` | `boot/std/kintsugi.mirror` | Declared | @prism |
| `@kintsugi/shatter` | `boot/std/kintsugi/shatter.mirror` | Declared | @prism, @kintsugi, @ai/fate, @epistemologic |
| `@kintsugi/translate` | `boot/std/kintsugi/translate.mirror` | Declared | @prism, @kintsugi, @code |
| `@kintsugi/migrate` | `boot/std/kintsugi/migrate.mirror` | Declared | @prism, @kintsugi, @nl |
| `@kintsugi/lift` | `boot/std/kintsugi/lift.mirror` | Declared | @prism, @kintsugi, @code, @io |
| `@mirror/refract` | `boot/std/mirror/refract.mirror` | Declared | @prism, @epistemologic, @epistemologic/property, @io |
| `@epistemologic/property` | `boot/std/epistemologic/property.mirror` | Declared | @prism, @epistemologic, @error, @nl |
| `@epistemologic/property/duplicate_variant` | `boot/std/epistemologic/property/duplicate_variant.mirror` | Declared | @prism, @epistemologic/property |
| `@epistemologic` | `boot/02-epistemologic.mirror` | Declared | @prism, @property, @loss, @nl |
| `@loss` | `boot/01a-error.mirror` | Declared | @prism, @nl |
| `@error` | `boot/01a-error.mirror` | Declared | @prism, @nl |
| `@imperfect` | `boot/01a-error.mirror` | Declared | @prism, @nl |

### 5.2 What's missing

| Grammar | Needed for | Status |
|---------|-----------|--------|
| `@mirror/trace` | The observation surface | **Not yet declared** |
| `@mirror/trace/topology` | Five duality measurements | **Not yet declared** |
| `@mirror/trace/fracture` | Sollbruchstelle detection | **Not yet declared** |
| `@mirror/trace/gradient` | Tropical shortest-path ordering of holes | **Not yet declared** |
| `@epistemologic/property/unresolved_import` | Import resolution checking | Not yet declared |
| `@epistemologic/property/circular_import` | Cycle detection in import graph | Not yet declared |
| `@epistemologic/property/unused_declaration` | Dead declaration detection | Not yet declared |
| `@epistemologic/property/arity_mismatch` | Type arity checking | Not yet declared |
| `@epistemologic/property/missing_export` | Export completeness checking | Not yet declared |
| `@epistemologic/property/unreachable_type` | Dead type detection | Not yet declared |

### 5.3 The dependency graph

```
@prism
  |
  +-> @loss, @error, @imperfect  (01a-error.mirror)
  |     |
  |     +-> @epistemologic  (02-epistemologic.mirror)
  |           |
  |           +-> @epistemologic/property  (property.mirror)
  |           |     |
  |           |     +-> @epistemologic/property/*  (concrete checks)
  |           |
  |           +-> @mirror/trace  [MISSING]
  |                 |
  |                 +-> @mirror/trace/topology  [MISSING]
  |                 +-> @mirror/trace/fracture  [MISSING]
  |                 +-> @mirror/trace/gradient  [MISSING]
  |
  +-> @kintsugi  (kintsugi.mirror)
  |     |
  |     +-> @kintsugi/shatter  (shatter.mirror)
  |     +-> @kintsugi/translate  (translate.mirror)
  |     +-> @kintsugi/migrate  (migrate.mirror)
  |     +-> @kintsugi/lift  (lift.mirror)
  |
  +-> @mirror/refract  (refract.mirror)
  |
  +-> @ai/fate  (fate.mirror)
```

The pipeline flows left to right through the dependency graph:

```
@mirror/trace (observe) -> @kintsugi (act) -> @mirror/refract (measure)
```

---

## 6. The Ticks -- ordered implementation path

### Tick 0: Declare @mirror/trace grammar

Write `boot/std/mirror/trace.mirror` with the trace type and observe action.
No implementation (all `\` holes). Establishes the contract.

**Blocked by:** nothing. Pure grammar declaration.
**Unlocks:** the interface that all subsequent ticks target.

### Tick 1: Type body parsing in the tokenizer

The tokenizer must parse type bodies (`type color = red | blue`) into
structured data. Without this, no property check can inspect variants.
The fields `SplitNode.variants` and `SplitNode.body` exist but are not
populated.

**Blocked by:** nothing. Pure Rust substrate change.
**Unlocks:** duplicate_variant check and all subsequent property checks.

### Tick 2: Grammar graph construction

Parse all `.mirror` files. Build the graph: nodes = types/actions, edges =
references/imports/signatures. Feed to `construct_dirac()`. Compute
`spectral_embedding()`. Store as crystal.

**Blocked by:** tick 1 (need parsed type bodies for accurate edges).
**Unlocks:** topology measurements, Fiedler vector, fracture detection.

### Tick 3: Trace topology -- the five measurements

Implement the five duality measurements on the grammar graph:
- Von Neumann entropy from the trace-normalized Laplacian
- Fiedler value (lambda_2) from Jacobi eigendecomposition
- Cheeger constant (stochastic estimation or exact for small graphs)
- Forman-Ricci curvature per edge (O(m * Delta), cheap)
- Mixing time estimation (spectral gap gives bounds)

**Blocked by:** tick 2 (grammar graph must exist).
**Unlocks:** trace topology, Narcissus detection, gutter lenses.

### Tick 4: Trace fractures -- Sollbruchstelle detection

Compute the Fiedler vector from the grammar graph Laplacian. Locate zero
crossings. Enumerate fracture points with loss estimates.

**Blocked by:** tick 2 (Fiedler vector requires grammar graph).
**Unlocks:** `--shatter` mode in kintsugi.

### Tick 5: Trace gradient -- tropical shortest-path ordering

Compute Shannon loss for each `\` hole (information distance between input
and output types in the spectral embedding). Order holes by tropical distance
from current state to resolution.

**Blocked by:** tick 2 (spectral embedding required), tick 3 (loss requires
topology context).
**Unlocks:** default kintsugi's hole selection strategy.

### Tick 6: Wire `\` to Fate (the bridge)

When the interpreter hits `is_abstract == true`, extract spectral embedding
features and call `fate.resolve()`. The Decision selects which model handles
the hole. Initially hardcoded model strategies. Later, model grammars.

**Blocked by:** tick 2 (spectral embedding provides features).
**Unlocks:** kintsugi can actually fill holes. Everything before this is
measurement. This is where action begins.

### Tick 7: Default kintsugi loop

Wire the full tick loop: trace -> pick cheapest hole -> Fate fills it ->
write back -> commit -> refract -> repeat. The monotone termination condition
checks delta_loss.

**Blocked by:** tick 5 (gradient for hole ordering), tick 6 (Fate for hole
filling).
**Unlocks:** `mirror kintsugi file.mirror` works end-to-end.

### Tick 8: Shatter mode

Wire the recursive fracture pipeline: Fiedler bisection -> atom tournaments
-> settle_up -> final tournament. Seam loss measurement.

**Blocked by:** tick 4 (fracture detection), tick 6 (Fate for tournaments),
tick 7 (default kintsugi for atom-level repair).
**Unlocks:** `mirror kintsugi boot/* --shatter` works end-to-end.

### Tick 9: Refract as CLI

Wire `mirror refract file.mirror` and `mirror refract --lens entropy file.mirror`
to produce verdict output. The five lenses produce measurements. The gutter
renders colors.

**Blocked by:** tick 3 (topology measurements), tick 7 (needs kintsugi to
produce before/after comparisons).
**Unlocks:** full ODA loop visible to the engineer.

### Tick 10: Reflection perturbation

After each kintsugi tick, Reflection observes the ManifoldLoss and perturbs
Fate's weights. Small nudges toward lower loss. The eigenboard shifts. The
trace surface changes. The next tick uses updated weights.

**Blocked by:** tick 7 (kintsugi must be running), tick 9 (refract must
measure).
**Unlocks:** the system self-corrects. The grammar evolves.

### Dependency graph of ticks

```
Tick 0: declare @mirror/trace --------+
Tick 1: type body parsing ----------- |--+
                                      |  |
Tick 2: grammar graph construction ---+--+
  |
  +-- Tick 3: topology (five measurements)
  |     |
  |     +-- Tick 9: refract CLI
  |
  +-- Tick 4: fractures (Sollbruchstellen)
  |     |
  |     +-- Tick 8: --shatter mode ----+
  |                                    |
  +-- Tick 5: gradient (tropical ordering)
  |     |                              |
  |     +-- Tick 7: default kintsugi --+
  |           |                        |
  +-- Tick 6: wire \ to Fate ----------+
                                       |
                              Tick 10: Reflection
```

Ticks 0-1 are independent prerequisites. Tick 2 depends on both. Ticks 3-5
fan out from tick 2. Tick 6 depends on tick 2. Tick 7 depends on ticks 5
and 6. Tick 8 depends on ticks 4, 6, and 7. Tick 9 depends on tick 3.
Tick 10 depends on ticks 7 and 9.

---

## 7. Connection to `\`

`\` is honest uncertainty. The most important character in the grammar.

When a grammar declares `collapse(ast, ast) -> imperfect { \ }`, the `\`
means: "I know the input type, I know the output type, I know the contract,
but I do not know the implementation." This is not an error. This is the
grammar being precise about what it does not know.

The trace enumerates every `\` in the grammar graph. Each `\` is a point
of honest uncertainty on the observation surface. The trace does not judge
the `\` -- it measures it. How much information must be created to fill this
hole? What other holes does it block? What is its spectral position in the
grammar graph?

Kintsugi fills `\` holes. But filling is not the only resolution. A `\` can
become:
- Structure + more `\` (partial resolution, smaller holes)
- Structure (complete resolution, hole is gone)
- A different `\` (the hole was in the wrong place; kintsugi moved it)

The `\` is only possible sub-Turing. In a Turing-complete language, a hole
is a gap in the program that could do anything. You cannot measure the
distance between input and output types because the computation could
diverge. The loss is undefined. The trace surface does not exist.

In a sub-Turing language, every computation terminates. The `\` hole has a
finite set of possible implementations (the `/` space -- the fiber). The
Shannon loss between input and output types is computable. The tropical
shortest path to resolution exists. The trace surface is well-defined. The
eigenvalue spectrum is finite.

This is why decidability matters. Not as a theoretical curiosity. As the
precondition for measurement. The compiler can only measure what it can
guarantee terminates. The grammar restriction makes the trace surface
computable. The trace surface makes kintsugi possible. Kintsugi makes
settlement possible. Settlement IS the grammar knowing what it is.

`\` is the grammar saying: "I don't know yet."
Trace is the compiler saying: "Here is what I can measure about what you don't know."
Kintsugi is the compiler saying: "Let me try to fill it."
Refract is the compiler saying: "Here is what changed."

The system learns from its errors. The errors get smaller.
`e^(n+1) < e^(n)`. By construction. By eigenvalue. By selection.

---

## 8. The Proof

The trace -> kintsugi -> refract pipeline terminates and loss is monotonically
non-increasing.

**Termination:** The grammar is sub-Turing. Every property check in the trace
terminates. Every Fate tournament terminates (bounded depth, bounded width).
Every kintsugi tick either reduces loss or stops. The number of possible
grammar states is finite (content-addressed, finite types). A monotonically
non-increasing function on a finite set reaches its minimum in finite steps.

**Monotonicity:** Each kintsugi tick is accepted only if `loss(n+1) <= loss(n)`.
If loss increases, the tick is rejected and the git commit is rolled back.
The crystal at each tick is the grammar at its lowest-loss state so far.

**The shatter proof has two parts:**
1. Local: each atom's tournament is a default kintsugi on a smaller grammar.
   By the default kintsugi termination proof, the atom reaches local ground
   state.
2. Global: the final tournament is a default kintsugi on the reassembled
   grammar. The seam loss measures the gap between local ground states and
   global. The final tick closes this gap or declares it irreducible.

The irreducible seam loss IS lambda_0 (the ground state of the whole). The
grammar cannot be locally optimized without global cost. The gold in the
seams is the price of modularity.

**The harmonic limit:** The Hodge decomposition shows that some loss is
topological. The harmonic component cannot be removed by kintsugi. It
requires architectural change. The trace reports this honestly. The crystal
at settlement carries the harmonic residual as measured debt.

---

*The compiler does not produce warnings.*
*It produces measurements.*
*The measurements form a surface.*
*Kintsugi breaks along that surface.*
*The gold fills the cracks.*
*What it cannot fill, it names.*

*trace -> kintsugi -> refract.*
*Observe -> act -> measure.*
*`e^(n+1) < e^(n)`.*
