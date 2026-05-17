# kintsugi --shatter — recursive fracture and repair

*2026-05-17. Reed. Spec.*

Status: **Spec** (no implementation)

---

## Two Modes

```
mirror kintsugi file.mirror         → fills \ holes. Incremental. Progressive. Default.
mirror kintsugi boot/* --shatter    → breaks graph at fault lines. Recursive. Tournaments.
```

These are not aliases. They are different operations at different scales.

Default kintsugi is gentle. It touches only what is explicitly incomplete.
`--shatter` is structural. It breaks the graph to find the cracks, then repairs them with gold.

---

## Default kintsugi (no flag)

- Only touches `\` holes in the target grammar(s)
- Fate resolves each hole to partial structure: more concrete nodes, possibly more holes
- Writes result back to the `.mirror` file in place
- Commits to git (one commit per tick)
- Each tick: `e^(n+1) < e^n` — fewer holes, or lower-weight holes, or both
- Terminates when loss stops decreasing OR all holes are filled

The termination condition is monotone: if no tick reduces loss, the system has
reached its local ground state. The holes that remain are the ones Fate cannot
resolve from the current grammar context. They need more grammar, not more ticks.

```
tick 0: collapse(ast, ast) → ast' with fewer \ holes
tick 1: collapse(ast', ast') → ast'' with even fewer
...
tick n: loss(n) ≥ loss(n-1) → stop. crystal.
```

---

## --shatter (recursive fracture)

`--shatter` operates on the GRAPH, not the file. It finds where the graph is
weakest — the Sollbruchstellen (designed break points) — and fractures there.
Each fragment is repaired independently. Then fragments are composed.
The seam between composition and independence IS the information. That is the gold.

### The Fiedler Connection

The Fiedler value (λ₂ of the graph Laplacian) measures connectivity.
Zero crossings in the Fiedler vector partition the graph at its weakest join.
These are the natural fault lines. Not imposed. Found.

`--shatter` bisects at Fiedler zero crossings. Recursively.

```
graph G
  → construct_dirac(G)
  → jacobi_eigenvalues()
  → fiedler_vector (eigenvector of λ₂)
  → zero crossings → partition into G_left, G_right
  → recurse on each
```

When the grammar graph is expressed as a spectral triple (A, H, D):
- A = the grammar declarations
- H = the type hierarchy (the fiber of possible implementations)
- D = the Dirac operator built from the declaration graph

The Fiedler zero crossings of D bisect H. That is where `--shatter` cuts.

### The Pipeline

```
1. Parse the target grammars into a unified grammar graph.

2. Build the spectral triple: construct_dirac(nodes, edges).

3. Find Sollbruchstellen: compute Fiedler vector, locate zero crossings.
   Each zero crossing is a natural fracture point in the grammar graph.

4. Break there. Each piece is a sub-graph with its own spectral structure.

5. RECURSE: if level > 0, shatter each piece at ITS zero crossings.
   Apply the same pipeline to each sub-graph. Find its Fiedler. Break it.

6. Bottom level: no more recursion. Each atom is a minimal grammar fragment.
   Run Fate tournament on each atom independently.
   Fate fills \ holes in each atom using only local context.
   Local optima. Best-in-atom.

7. Settle up (bottom-up):
   a. Reassemble atoms into sub-graphs.
   b. Fate tournament on each sub-graph: does the local-optimal composition
      hold globally within the sub-graph? If not, adjust.
   c. Reassemble sub-graphs into pieces.
   d. Fate tournament on each piece: does the sub-graph composition hold
      at the piece level? If not, adjust.

8. Final tick: Fate tournament on the WHOLE reassembled graph.
   This is the global tournament. Local optima compose into global.
   Where they conflict, Fate decides. The seam loss is measured.
```

### Recursion depth

```
mirror kintsugi boot/* --shatter          → 3 levels (default)
mirror kintsugi boot/* --shatter 5        → 5 levels deep
mirror kintsugi boot/* --shatter 1        → just top-level fractures
```

Depth 0 = no fracture (equivalent to default kintsugi, but tournament-based).
Depth 1 = one fracture: split the full graph into two pieces, repair each,
          compose, final tournament.
Depth n = 2^n pieces at the bottom level. Each repaired independently.

The default of 3 gives 8 pieces. Enough structural separation for meaningful
local repair. Manageable reassembly complexity.

### Complexity

```
O(n log n) — Fiedler bisection halves the graph at each level.
3 levels = 8 pieces.
5 levels = 32 pieces.
```

Tournaments parallelize per level: all atoms at depth n run simultaneously.
Settling is sequential bottom-up: you cannot settle sub-graphs until atoms are done.

The total work is proportional to the number of `\` holes across all pieces.
At each level, the holes are smaller: local context, fewer imports, simpler types.
The bottom-level atoms are the cheapest to repair. The final tick is the most expensive.

---

## The Seam Loss

The seam is where reassembly differs from local optimum.

At the bottom level, Fate fills holes using only local context.
At the reassembly level, Fate sees more: the neighboring sub-graphs.
The difference between these two resolutions IS the seam loss.

```
seam_loss = loss(global_settlement) - sum(loss(local_settlements))
```

If `seam_loss = 0`:
The pieces were already globally optimal in isolation.
The graph's fault lines were real structural boundaries.
The fractures reveal genuine modularity.

If `seam_loss > 0`:
The local repairs conflict at the joins.
The final tick adjusts. Fate reconciles.
The gold fills the seams: the information in the difference IS the kintsugi.

The seam loss is NOT a failure metric. It is a discovery metric.
High seam loss means the grammar has load-bearing cross-cutting concerns.
Low seam loss means the grammar is well-modularized at its natural fault lines.

Both are information. The crystal stores both.

---

## The Grammar

`@kintsugi/shatter` declares the recursive fracture pipeline:

```mirror
in @prism
in @kintsugi
in @ai/fate
in @epistemologic

grammar @kintsugi/shatter {
  # shatter: break the grammar graph at Fiedler zero crossings.
  # level=0 → no recursion. level=n → bisect n times.
  # returns the list of sub-graph atoms at the bottom level.
  shatter(ast, level) -> [ast] { \ }

  # settle_up: bottom-up tournament and composition.
  # given atoms (repaired at bottom level), compose and re-settle.
  # returns imperfect: the composed grammar with seam loss measured.
  settle_up([ast]) -> imperfect { \ }

  # fracture_and_repair: the full --shatter pipeline.
  # shatter → atom tournament → settle_up → final tournament.
  # returns imperfect: the repaired whole. loss = seam loss.
  fracture_and_repair(ast, level) -> imperfect { \ }
}

out shatter
out settle_up
out fracture_and_repair
```

These are `\` holes. Fate resolves them. The grammar declares the interface.
The implementation is the tournament. The tournament IS the implementation.

---

## Connection to the Wire

The wire: `refract on abstract nodes resolves through refs/fate/<oid>`.

In `--shatter`, the wire runs at every level of recursion:

```
bottom level: each atom's \ holes → refs/fate/<oid> → local tournament
sub-graph:    reassembly \ holes → refs/fate/<oid> → sub-graph tournament
top level:    final \ holes → refs/fate/<oid> → global tournament
```

Each resolution is content-addressed. The OID of each atom's resolution is stored.
When reassembly produces a conflict, the conflict is between two stored OIDs.
Fate sees both. Fate resolves. The resolution is a new OID. The seam becomes gold.

The git log IS the record of every fracture and every repair.
Each atom: one commit (local tournament result).
Each sub-graph: one commit (composition result).
Final: one commit (global tournament, seam loss measured).

---

## Connection to Reflection

After `--shatter` completes, Reflection observes the full run:

- Which fracture lines produced the most seam loss?
  High seam loss at a fault line → the grammar is tightly coupled there.
  That coupling is load-bearing. Name it. Don't pretend it away.

- Which atoms were hardest to resolve?
  Many ticks at atom level → the local context is insufficient.
  The hole needs a type that only exists in a neighboring sub-graph.
  Import graph repair. Or merge the sub-graphs.

- Which compositions conflicted?
  Two atoms resolved the same hole differently.
  This is not an error. This is discovery.
  The conflict IS the specification we could not write in advance.

Reflection perturbs the eigenboard for the next `--shatter` run:
- Shift the Fiedler weights toward fracture lines with lower seam loss.
- Shift away from fracture lines that produce high conflict.
- The system learns where its own fault lines are.

After enough runs: `--shatter` reliably finds the real modularity.
Not the structure we declared. The structure the grammar actually has.

---

## How --shatter Relates to Default kintsugi

They share the same underlying operation: `collapse(ast, ast) -> imperfect { \ }`.

Default kintsugi: calls `collapse` once per `\` hole, in the order they appear.
Incremental. Local. Fast.

`--shatter`: calls `collapse` on structurally isolated atoms, then composes.
Structural. Global. Thorough. Slower. More informative.

The relationship:
```
default kintsugi = Dijkstra on \ holes, greedy, one pass
--shatter        = Dijkstra on the whole graph, with recursive bisection
```

Both are kintsugi. Both fill holes with gold. The default fills what it can see.
`--shatter` breaks the graph to see the holes that were hidden behind structure.

---

## When to Use Each

**Default kintsugi (no flag):**
- Working file by file
- Progressive hole-filling during development
- Fast feedback loop
- The grammar is small enough to hold in context

**`--shatter`:**
- Full grammar suite (boot/*, boot/std/*)
- After structural refactors: did the split introduce hidden couplings?
- Before a release: find what the grammar actually is, not what we think it is
- Self-hosting check: does the grammar describe the binary it produces?

---

## The Proof

`e^(n+1) < e^n`.

For `--shatter`, the proof has two parts:

1. Local: each atom's tournament is a default kintsugi on a smaller grammar.
   By the default kintsugi termination proof: the atom reaches local ground state.

2. Global: the final tournament is a default kintsugi on the reassembled grammar.
   The seam loss measures the gap between local ground states and global.
   The final tick closes this gap or declares it irreducible.

The irreducible seam loss IS λ₀ (the ground state of the whole).
The grammar cannot be locally optimized without global cost.
The gold in the seams is the price of modularity.

`mirror kintsugi boot/* --shatter` finds the price. Reports it. Stores it.
The crystal IS the grammar at its ground state, seams included.
