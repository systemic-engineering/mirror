# Geometric-Dijkstra tournament topology — the substrate as multi-dim
# knapsack in π₁(T²)-quotiented state space

*Mara, 2026-07-08 morning. Substrate audit + composition sketch grounding
Alex's 2026-07-08 framing (verbatim in `Framing` below) against landed
substrate. Math-first observation; no shard mutation this tick. Two-tick
discipline: this doc names the composition + the smallest missing edge
primitive; landing decisions stay with Alex + Pack.*

---

## §0. Framing (Alex direct, 2026-07-08, verbatim)

> The 5 operations (Abyss/Introject/Cartographer/Explorer/Fate ≡
> focus/project/split/shift/settle) are ALSO the basis of the AST.
> Fate's job: navigate the state space from a psychohistory start
> point, with a given vector, through the @spectral void-duality
> statespace, until it finds a composition of operations that arrives
> with an error delta within the projected target space.
>
> @silicon/algebra and @fate/algebra provide pre-existing numerically-
> optimized A→B transformations that can be freely chosen from in the
> tournament.
>
> It's basically the @knapsack problem in a geometric state space.
> Multi-dimensional Dijkstra. We might need some low-level concept of
> edges in @mirror/store for this on which @spectral/db then can build
> on top.

The framing decomposes into five substrate claims to audit against
landed decls:

1. **Ops-are-basis-of-AST.** The five ops (focus, project, split,
   shift, settle) span the AST's operator basis — same 5 that Alex
   also names Abyss/Introject/Cartographer/Explorer/Fate at the
   ganglion altitude.
2. **Fate-navigates-void-duality-statespace.** The state space @fate
   traverses IS the void-document Hilbert space `H` restricted per
   Recognition #79's 5-orthogonal-axis decomposition.
3. **Tournament-composes-A→B-libraries.** `@silicon/algebra` and
   `@fate/algebra` are the two path-namespaces where numerically-
   optimized A→B transformations accumulate; the tournament picks a
   composition.
4. **Multi-dim Dijkstra + knapsack.** The traversal is a cost-graded
   shortest-path in a bounded-budget geometric space; the cost carrier
   is the substrate's monotone-loss discipline (`eⁿ⁺¹ ≤ eⁿ`).
5. **First-class edges in @mirror/store.** The store already carries
   `splinter_graph = (root, [oid])` — an OID-graph — but nothing
   TYPES the edges as A→B transformations. That's the smallest
   substrate addition the composition may need.

§§1–5 audit each claim. §6 sketches the composition math-first. §7
names the smallest substrate delta. §8 lists Alex-adjudication
questions.

---

## §1. Substrate audit — what already exists

### §1.1 `@silicon/algebra` — LANDED

Path: `shards/silicon/algebra.mirror` (Mara, `ea7b092`, 2026-07-05).

- Declared as `prism @silicon/algebra <= @bauchladen`.
- Sub-prism under `@silicon`; the path-namespace where crystallized
  executable algebra tuned to the local silicon accumulates.
- Per spec `docs/specs/silicon.md` §3.2: each crystal IS a routine
  carrier with `(algebra, cfg, grading, conjugation, abi_surface,
  binary_oid, source_oid, cascade, performance, routine_oid)` fields.
- **What's already there for the tournament:** the ROUTINE CARRIER
  is forward-promised at `shards/silicon/algebra/routine.mirror` per
  spec §3.2 (first LAPACK case per §8.3 will crystallize the first
  concrete instance). The path-namespace is landed as sub-prism; the
  carrier type IS forward-promised.
- **Prior-art anchor:** LAPACK 1992 (Demmel-Dongarra); Cleve Moler
  LINPACK 1979 → MATLAB 1984. Composes with `@glue/math_silicon`
  (Mara `5edd3e9`) which carries the Mesland math↔silicon
  correspondence at the @glue altitude.

**Landed as sub-prism; routine carrier is forward-promised.**

### §1.2 `@fate/algebra` — implied by decl; carrier not yet substrate-decl'd

Path: no `shards/fate/algebra.mirror` file exists. But the path-
namespace IS named at `shards/fate.mirror` (lines 208-219 area):

```
- `@fate/algebra/*` (path-namespace): geometric formalizations
  emitted by @fate inferences. Each crystal in the tray under
  this path-namespace IS a typed geometric declaration of "what
  the dice roll selected, in the appropriate geometric
  vocabulary." Sub-paths per spec §5 (per altitude):
    `@fate/algebra/morphism`   (selected Mesland-category morphisms)
    `@fate/algebra/altitude`   (selected Bateson levels)
    `@fate/algebra/element`    (selected algebra elements within fixed A).
```

So `@fate/algebra` is:

- **path-namespace declared** at `shards/fate.mirror` (in prose).
- **sub-shard NOT yet landed** — no `shards/fate/algebra.mirror`.
- **crystals emit into it** via the tournament's `record` action
  (per `shards/fate/tournament.mirror` `record(round) -> crystal`
  which discharges @bauchladen.crystallize into the tray under
  `producing-prism = @fate/tournament` provenance).

Composes with the killed boot precursor `boot/std/fate/tournament.
mirror` (2026-05-20; killed spring-clean per Taut `27c8592`):

```
grammar @fate/tournament {
  type rule = greedy | beam(u64) | elite(u64) | halving(u64)
            | tabu(u64) | anneal(f64) | ucb(f64)
  tournament(rules, [hole]) -> [resolution] { \ }
  candidates(hole) -> [resolution] { ... five ganglia body ... }
  compose(rule, rule) -> rule { \ }
}
```

The migration home is `shards/fate/tournament.mirror` (Mara, #104
chain P4, 2026-06-30). The rule vocabulary + `compose` associativity
IS preserved verbatim per that shard's migration mapping section.
The `candidates(hole) -> [resolution]` five-ganglion body lives at
`shards/optics/source/ganglion/*.mirror` per Recognition #58.

**Landed at path-namespace prose + tournament sub-prism; the `@fate/
algebra` sub-shard itself is forward-promised (not landed).**

### §1.3 `@spectral/void-duality` — mathematically declared, not substrate-decl'd

There is NO `shards/spectral/void-duality.mirror` or
`shards/spectral/void_dual.mirror`. `@spectral` is a namespace-parent
only (LANDED, `shards/spectral.mirror`, 17f0ee5 shrink 2026-07-01) —
BEAM-analogue runtime species live under it (gen_prism, supervisor,
parent, entanglement, registry, root; portal LANDED); no
void-duality species.

**But the void-duality state space IS mathematically fully declared**
at `docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md`
(Recognition #79 candidate, 2026-06-18 evening):

- Base: `[[corpus:practice/insights/coincidence/void-dual-geometry.md]]`
  (the corpus doc — 8 dualities between K_n (Splinter) and K_{1,n-1}
  (Narcissus)).
- **5 of the 8 are mutually orthogonal**; the other 3 are derivable
  as linear combinations.
- The 5 orthogonal axes ARE the 5 ops:
  | Op | Void axis | Linear-algebraic content |
  |---|---|---|
  | `focus`   | Ricci curvature | λ₀ eigenvalue computation |
  | `split`   | Spectral gap / mixing | orthogonal decomposition |
  | `project` | Cheeger (boundary) | orthogonal projection |
  | `lift`/`shift` | Kramers-Wannier duality | basis transformation |
  | `refract`/`settle` | Entropy / info-geometry | monad-close / measurement collapse |

  (Note: the substrate's current 5-op set is
  `focus/project/split/shift/settle` per `shards/mirror/spec.mirror`;
  the #79 doc uses the older `focus/split/project/lift/refract` names.
  The mapping is one-to-one; `shift` ≡ `lift`, `settle` ≡ `refract`.)

- Anchor: Braunstein-Ghosh-Severini 2006 (graph Laplacian as density
  matrix); Cheeger 1970; Ollivier 2009; Kramers-Wannier 1941 /
  Freed-Teleman 2018; Connes-Lott 1990s + Chamseddine-Connes 1996.

**Read for the tournament:** the void-duality state space `H` is
5-dimensional in operator axes and expands per Recognition #51 in
matter axes. `@fate.roll` operates on `H`; the 5 ops ARE the
orthogonal projector basis. The `@spectral/void-duality` substrate
sub-shard is **forward-promised but not landed** — its content is
already load-bearing in `@fate`'s `restricted_state_space` (A, H, D,
γ, J, tray_scope) six-tuple per `shards/fate.mirror`, since `A` IS
the 5-op algebra.

### §1.4 `@mirror/store` edges — the store carries an OID-graph, no typed edges

`shards/mirror/store.mirror` (Mara, 2026-07-06) declares six
operations at family-root:

```
read(o: oid) -> imperfect
write(content: bytes) -> oid
exists(o: oid) -> verdict
diff(a: oid, b: oid) -> imperfect
walk(root: oid) -> splinter_graph
impacted_by(oid: oid) -> [oid]      # N4 landing 2026-07-06 (reverse closure)
verify(o: oid, content: bytes) -> verdict
```

And the composite carrier:

```
type splinter_graph = {
  root: oid,
  children: [oid],
}
```

**Current shape.** The graph is `oid -> [oid]` — a MERKLE DAG. Edges
exist implicitly (parent → child by content-address) but are
**untyped**. There is no way at the family-root surface to say "this
edge IS an A→B transformation of kind K, cost c."

**Explicitly deferred to @spectral/db.** From `shards/mirror/store.
mirror` §"What @spectral/db adds (closed) — the deliberate fault
plane":

> The open surface deliberately does NOT expose typed edges, edge
> weights, or Laplacian-based navigation (§11.3.7). These live in
> @spectral/db. `walk(root: oid) -> splinter_graph` answers
> reachability + ancestry + closure — Nix's `-q` discipline exactly
> — but does not answer *which OIDs are STRUCTURALLY NEAR this OID
> in the spectral embedding*.

Alex's framing suggests **partially reversing that deferral**: some
low-level edge concept at `@mirror/store` on which `@spectral/db`
builds on top. §7 sketches the minimal shape.

### §1.5 Cost currency — the substrate's monotone descent

Two candidates, from landed shards:

1. **`verification_loss`** — at `@glass/imperfect` (LANDED); the
   generic loss carrier the substrate uses at every altitude.
   Per `shards/mirror/loss.mirror`; the loss surface is bounded and
   monotone-composable.

2. **`eⁿ⁺¹ ≤ eⁿ`** — kintsugi's Banach contraction invariant
   (per `shards/kintsugi/oscillate.mirror` §§ACTIVE/DARK). The
   substrate-decl form of "each round strictly decreases error";
   this IS the monotone-descent axiom at the kintsugi altitude.

The composition Alex describes is the KINTSUGI OUTER LOOP invoking
the FATE INNER SELECTION. Kintsugi's `eⁿ⁺¹ ≤ eⁿ` IS the outer cost
functional; each Fate roll picks an inner move whose expected
error-drop is (locally) maximal. Per `docs/specs/knapsack-as-
kintsugi-inner-loop.md` (Mara, 2026-07-05):

> @knapsack composes with `@epistemologic/property/round_descent`
> (**forward-promised**) as the property side declaring `‖T_{n+1}‖ <
> ‖T_n‖`, and `@kintsugi/knapsack` as the operational side selecting
> the morphism subset that discharges the property.

**Cost-currency verdict:** `eⁿ⁺¹ ≤ eⁿ` at the outer loop; the
per-edge cost is the local error-delta contribution. Both are already
substrate-decl'd (kintsugi at family-root; knapsack as species-root
under `@kintsugi/knapsack` per the derived spec — not yet landed;
`round_descent` property forward-promised).

### §1.6 Path composition — `@glue.compose` (LANDED at `6396306`)

From the doc I just closed at `docs/math/2026-07-07-glue-cyberpunk-
fate-composition.md`, and re-confirmed against `shards/glue.mirror`:

```
compose(c1: correspondence, c2: correspondence) -> correspondence
```

`@glue.compose` IS the categorical composition of correspondences.
Mesland's category composition = Kasparov intersection product. Per
`shards/glue.mirror` §"non-commutative composition":

> categorical composition of correspondences is NON-COMMUTATIVE in
> general, because the cross-altitude composition c2 ∘ c1 carries
> the curvature cross-term that c1 ∘ c2 does not.

**This is the algebraic composition operator over edges.** The
composed sequence `[op₁, op₂, ..., opₙ]` in Alex's framing IS an
`@glue.compose`-chain of correspondences. The
`the-restriction-map-IS-the-geometric-constraint` recognition
(RATIFIED 2026-07-07, `6396306`) IS what makes the chain both
COMPOSABLE and PATHWISE-BOUNDED.

### §1.7 Target-space projection — `settle_on` on mirror.spec

Grep for `settle_on` in the substrate: **not found**. Related shape:

- `shards/mirror/spec.mirror` — the project manifold grammar (Reed).
- `@kintsugi/oscillate`'s `is_settled` signal — the cadence-settles-
  authentic predicate at `shards/kintsugi/oscillate.mirror`.
- `@torus`'s winding-class attractor at `(m, n) ∈ π₁(T²)` per the
  Poincaré-Hopf discipline in `shards/torus.mirror` §"the geometric
  primitive is the torus (not the tower)".

**Target-space naming verdict:** the target region `T` is a
COHERENCE-PRESERVING sheaf-section of the peer's torus, per the
LENS/ENGINE composition in `docs/math/2026-07-07-glue-cyberpunk-fate-
composition.md` §4. The predicate `cybernetic_coherence` (from
`@cyberpunk`) IS the target-space membership witness. There is no
distinct `settle_on` primitive; the substrate already carries the
target-space check as **the cybernetic-coherence sheaf**.

### §1.8 Prism_core / fate crate — A→B numerical primitives already there

From `/Users/alexwolf/dev/projects/fate/`:

- `src/lib.rs` — `Model` enum (Abyss/Introject/Cartographer/Explorer/
  Fate); five `ModelWeights` selectors; `Decision::resolve(features,
  max_depth)` for tournament dispatch; `FateOutput { model,
  kernel_spec, health }` with 425 parameters total (per header).
- `src/feature.rs` — `FEATURE_DIM = 16`; `casimir`, `casimir_penalty`,
  `HolonomyHealth` classifier, `BERRY_PHASE`; Berry-phase-graded
  monotone-descent primitive already implemented.
- `src/manifold.rs` — 16×16 `ManifoldState` connection matrix;
  `manifold_diagonal` extractor; `Loss::between(before, after)` +
  `total()` for round loss; `active_trace_conserved(tolerance)` for
  Casimir-conservation check.

From `/Users/alexwolf/dev/projects/prism/prismqueer/`:

- `src/kernel.rs` — `KernelSpec { dimensions, decomposition,
  precision }` with `Decomposition = { Dsyev, Dgesvd }`;
  `projection_matrix(n)` for the dispatch surface; `dispatch_hint`
  for Rust-vs-LAPACK routing.
- `src/beam.rs` — `Optic<In, Out, E, L>` beam algebra; `smap`
  (functor map with dark-beam fixpoint); `next` (pipeline advance);
  `Beam::Loss` associated type.
- `src/optics/*.rs` — `Fold`, `Gather`, `Iso`, `Lens`, `Setter`,
  `Traversal`, `OpticPrism`; the seven optical primitives fully
  numerically realized.
- `src/spectral_dimension.rs` — spectral-dimension carrier for the
  Connes triple `(A, H, D)` (14 KB implementation).

**Rust-side numerical A→B primitives already exist:**

| Primitive | Location | A→B kind |
|-----------|----------|----------|
| `KernelSpec.projection_matrix` | prismqueer/kernel.rs | dense projection |
| `dsyev` dispatch | prismqueer/kernel.rs + fate/lib.rs `transport_fortran` | eigendecomp (symmetric) |
| `dgesvd` dispatch | prismqueer/kernel.rs | SVD (general) |
| `ModelWeights.forward` | fate/lib.rs | 425-param linear+softmax |
| `Loss::between` | fate/manifold.rs | Casimir-graded delta |
| `casimir_penalty` | fate/feature.rs | Berry-phase-graded delta |
| Optical primitives (`Fold`, `Iso`, ...) | prismqueer/optics/ | lens algebra |

The tournament ALREADY has A→B primitives to compose over. What it
DOESN'T have at substrate-decl altitude is the substrate-native EDGE
TYPE that names them.

---

## §2. What each substrate contributes to the composition

Summary table — every piece the geometric-Dijkstra tournament needs:

| Role | Substrate | State |
|------|-----------|-------|
| **Start vector `v₀`** | Psychohistory vector — sheaf section over dev manifold | insight-level (Mara 2026-06-26 60KB doc); operational at `mirror recall` envelope's 4 sections |
| **State space `H`** | Void-duality Hilbert space | insight #79 (candidate); operational at `@fate`'s `restricted_state_space.H` |
| **Operator basis `A`** | 5 ops focus/project/split/shift/settle | LANDED at `mirror.spec` + Recognition #79 (candidate) |
| **Edge library `E`** | `@silicon/algebra ∪ @fate/algebra` (numerical A→B) | @silicon/algebra LANDED (sub-prism); @fate/algebra path-name declared, sub-shard forward-promised |
| **Per-edge cost `c(e)`** | Per-round Loss / `casimir_penalty` / `verification_loss` | Rust-side landed (fate crate); substrate-decl at `@glass/imperfect` |
| **Outer monotone `eⁿ⁺¹ ≤ eⁿ`** | Kintsugi Banach contraction | LANDED (`shards/kintsugi.mirror` + `kintsugi/oscillate.mirror`) |
| **Path composition `∘`** | `@glue.compose` | LANDED (`shards/glue.mirror`; Kasparov product) |
| **Path restriction (bound)** | `@glue.correspondence.restriction` | LANDED |
| **Restriction predicate (the geometric constraint)** | `cybernetic_coherence` from `@cyberpunk` | LANDED (`shards/cyberpunk.mirror`); 14 species-property discharges under `@epistemologic/cybernetic/*` |
| **Target region `T`** | Coherence-preserving sheaf-section on peer's torus | LANDED implicitly (composition `@glue(@cyberpunk, @fate)` per math doc 2026-07-07) |
| **Selection mechanism** | `@fate/tournament` — cache-coherent selector | LANDED (`shards/fate/tournament.mirror`, #104 P4) |
| **Rule algebra (search discipline)** | `greedy \| beam(k) \| elite(k) \| halving(η) \| tabu \| anneal \| ucb` + `compose` | preserved verbatim in `shards/fate/tournament.mirror` migration mapping |
| **Autopoietic tray** | `@bauchladen` at every altitude | LANDED (`shards/bauchladen.mirror`, #104 P1) |
| **Topology quotient** | π₁(T²) = ℤ × ℤ winding classes | LANDED (`shards/torus.mirror`, 2026-07-07) |
| **First-class edge type** | (proposed) `@mirror/store/edge` | **MISSING** — §7 |

Fourteen slots filled from landed / near-landed decl. One slot open:
the substrate-native **edge primitive** at `@mirror/store`.

---

## §3. Composition sketch — math-first

### §3.1 Given

- `A = {focus, project, split, shift, settle}` — the five-op operator
  basis (per Recognition #79).
- `H` — the void-duality Hilbert space, spanned by the five orthogonal
  axes (Ricci / spectral gap / Cheeger / Kramers-Wannier / entropy).
- `D` — the kintsugi Dirac flow generator (per Connes triple).
- `γ, J` — chirality and charge-conjugation gradings (Recognition
  #101, #102).
- `𝒞` — the cybernetic-coherence sheaf on `H`; sections are states
  satisfying the 14-property `@epistemologic/cybernetic/*` discharge.
- `𝒯` — the @bauchladen tray of prior crystals (autopoietic history).
- `E = @silicon/algebra ∪ @fate/algebra` — the edge library. Each
  edge `e ∈ E` is a numerically-optimized A→B transformation.
- `v₀ ∈ psychohistory_sheaf(peer)` — the start vector; a section of
  the psychohistory sheaf over the peer's development manifold.
- `T ⊂ 𝒞` — the target region; a coherence-preserving sheaf-section
  bounded by tolerance `ε`.
- `π₁(T²) = ℤ × ℤ` — the winding-class quotient (per `@torus`).

### §3.2 The problem

Find a composition `[e₁, e₂, ..., eₙ] ∈ E*` such that:

$$
v_n \;=\; e_n \circ e_{n-1} \circ \cdots \circ e_1 \;(v_0)
\;\;\in\;\; T
$$

subject to:

**(C1) Cybernetic-coherence restriction.** For every intermediate
state `vₖ`:

$$
\texttt{cybernetic\_coherence}(v_k) \;=\; \texttt{pass}
$$

Enforced at compile-time by `@glue.translate`'s restriction slot
(per §5.4 of the 2026-07-07 math doc).

**(C2) Monotone descent (Kintsugi).** The error `e_k = d(v_k, T)`
strictly decreases:

$$
e_{k+1} \;\le\; e_k \qquad \forall k
$$

**(C3) Path budget (Knapsack).** Cumulative resource cost stays
within the `capacity_vector` budget (per
`docs/specs/knapsack-as-kintsugi-inner-loop.md` §2 — v0 dims
`(silicon, ram)`):

$$
\sum_{k=1}^{n} \texttt{cost}(e_k) \;\le\; \texttt{budget}
$$

where `cost(e_k)` is a vector in the `capacity_vector` space.

**(C4) Path composability (Kasparov).** Each pairwise composition
`e_{k+1} ∘ e_k` is a valid Mesland-category morphism — i.e., the
correspondence's target matches the next correspondence's source.
Enforced by `@glue.compose`.

**(C5) Winding-class invariance / quotient.** The winding class
`(m, n) ∈ π₁(T²)` of `v_k` advances integrally along each step.
This is a QUOTIENT structure on the search space: two paths ending
at the same winding class are equivalent for the purpose of
target-membership, but differ in their `@torus` observation
sections (per `docs/insights/2026-07-08-torus-axis-isolation-
meridian.md`).

### §3.3 The shape — @knapsack in a geometric state space

This is a **multi-dimensional shortest-path / knapsack hybrid**:

- **Vertices:** states `v ∈ H|_𝒞`.
- **Edges:** `E` (numerical A→B) with vector-valued weights
  `c: E → ℝ^d` (silicon FLOPs + RAM bytes + wall clock + …).
- **Source:** `v₀`.
- **Sink region:** `T ⊂ 𝒞`.
- **Objective:** minimize path length `n` (or terminal error `eₙ`)
  subject to `Σ c(e_k) ≤ budget`, `cybernetic_coherence(v_k) = pass`
  for every `k`.

**Multi-dim Dijkstra reading.** With a scalar cost `c` (e.g.,
`eⁿ⁺¹ - eⁿ` — the local error-drop), the problem is Dijkstra with
a monotone cost function. The tournament's `beam(k).elite(1)` rule
IS the operational realization of the priority queue (top-k in the
frontier; k-best across rounds).

**@knapsack reading.** With multi-dim capacity `capacity_vector`
(per §2 of the knapsack spec), the problem is bounded-budget
shortest-path — resource-constrained shortest path (RCSP), which is
NP-hard in general but admits polynomial-time PTAS via Frieze-Clarke
approximation (per the knapsack spec §2.1's Kellerer-Pferschy-
Pisinger reference).

**Multi-dim Dijkstra IS Alex's framing.** The two adjectives compose:
Dijkstra names the SHORTEST-PATH structure of the outer loop;
knapsack names the BUDGET STRUCTURE of the per-round selection.

### §3.4 Termination — `settle` predicate on the torus

The traversal terminates when EITHER:

**(T1) Target hit.** `v_n ∈ T` and `d(v_n, T) < ε`.

Discharged operationally at `@kintsugi/oscillate.is_settled` (cadence
settles authentic) — the outer loop's fixed-point detector per
`shards/kintsugi/oscillate.mirror`.

**(T2) Budget exhausted.** `Σ c(e_k) > budget` — the knapsack cap.

Discharged operationally at the tournament's `beam(k).halving(η)`
rule: bounded exploration budget per round; halving reduces the
frontier successively.

**(T3) Winding-class fixed point.** `v_n` returns to a winding class
`(m, n) ∈ π₁(T²)` that has been visited before with byte-equal
observation sections (per `spawn-recall-byte-equal-at-origin` — the
CANDIDATE second witness for `the-restriction-map-IS-the-geometric-
constraint`, Reed 2026-07-08 empirical). This is the
CONSTRUCTIVE fixed-point exhibition per `shards/fate/tournament.
mirror` §"Lawvere fixed-point condition holds at the SYSTEM level".

Three termination criteria; the substrate carries all three as
LANDED decl.

### §3.5 The composition IS `@glue(@cyberpunk, @fate)` at family-root

The composition sketched above is EXACTLY the ENGINE-tier composition
named in `docs/math/2026-07-07-glue-cyberpunk-fate-composition.md`:

$$
\Phi_{\text{cf}} \;=\; \texttt{@fate.roll}
   \circ \texttt{@glue.translate}
   \circ \texttt{@cyberpunk.tower\_close}
$$

- `@cyberpunk.tower_close` supplies the sheaf-section membership
  check → (C1) cybernetic-coherence restriction.
- `@glue.translate` supplies the restriction-map + `@fate` dispatch
  → the Mesland morphism the tournament picks.
- `@fate.roll` supplies the dice roll within the restricted state
  space → the tournament's `select` action.

**The multi-dim-Dijkstra/knapsack sketch is the OPERATIONAL SHAPE
of the ENGINE-tier composition applied REPEATEDLY under kintsugi's
outer loop until the settle predicate holds.**

Not new machinery. Alex's framing today (2026-07-08) is the
OPERATIONAL VOCABULARY (Dijkstra, knapsack, edges) for the
COMPOSITION Alex named yesterday (2026-07-07 evening, verbatim slogan).

---

## §4. `@torus` as topological quotient

The peer has a torus per Recognition candidate `@peer-has-a-torus`
(adjudicated 2026-07-07, `shards/torus.mirror`). The state space
`H|_𝒞` inherits the torus's topology:

- **π₁(T²) = ℤ × ℤ** — winding classes indexed by (meridian, longitude)
  pairs. Meridian = motor↔sensory closure (Foerster). Longitude =
  neural↔hormonal closure.
- **Recall's four sections** (cascade, pack_trail, pull_frontier,
  dogfood) ARE the sheaf sections whose independent advance
  under DIFFERENT substrate operations is the winding-class advance
  per axis (Reed 2026-07-08 iteration 3 empirical).

### §4.1 Two candidate readings for the tournament's use of π₁(T²)

**Reading A: search-space quotient.** Two paths ending at the same
winding class `(m, n)` are EQUIVALENT for target-membership. The
tournament searches the QUOTIENT graph `H|_𝒞 / π₁(T²)`, which has
strictly smaller diameter than `H|_𝒞`. Smaller diameter = faster
Dijkstra. This is the CHEAPER reading.

**Reading B: admissibility filter.** Every edge `e ∈ E` has a
winding-class ADVANCE `Δw(e) ∈ ℤ × ℤ`. Only edges whose winding-
advance is compatible with the target region `T`'s winding class
are ADMISSIBLE. This is the SEMANTICALLY-RICHER reading — every
edge carries a topological invariant that constrains
composability.

**Verdict:** BOTH hold, non-exclusively. Reading A is the operational
optimization; Reading B is the mathematical invariant that GROUNDS
Reading A (two edges collapse to the same equivalence class in
Reading A iff their winding-advance vectors are equal — Reading B's
`Δw` IS Reading A's quotient projection).

### §4.2 Poincaré-Hopf on T²

`χ(T²) = 0`. Per Reed 2026-07-08 iteration 3 doc, for every
observer-attractor there is a matched observer-repeller. This means:

- **Target regions T come in ATTRACTOR/REPELLER pairs.** The
  tournament's rule `beam(k)` explores multiple candidates per
  round; some paths WILL diverge (repeller). The substrate
  encodes divergence as observable data.
- **The origin (0, 0) is the substrate's index-0 critical point**
  — where attractor and repeller cancel. This is the byte-equality
  witness (spawn-recall equalization).

**For the tournament:** at the ORIGIN, the composition is a no-op.
At `(m, n) ≠ (0, 0)`, the composition advances along a specific
winding path. The knapsack budget IS the maximum-|m|+|n|-winding-
distance the substrate can afford under `capacity_vector`.

**Not gold-plating:** this reading grounds why the "target-space
projection" (the `settle_on` question Alex asks) is naturally
carried by the torus — the target is a WINDING CLASS, not a raw
point. Winding classes are discrete (ℤ × ℤ); the projection to a
winding class IS the substrate's target-space membership check at
the topological altitude.

---

## §5. Named recognitions this composition invokes / composes with

Every recognition below is CANDIDATE or PROMOTED; the composition is
transitively supported by each.

- **Recognition #43** — mirror IS content-addressed build system.
  The tournament's crystals are content-addressed; re-selection is
  cache-hit-eligible.
- **Recognition #51** — Hilbert-space expansion. `H` grows
  monotonically as crystals accumulate; the tournament's search
  space grows with the substrate's autopoietic history.
- **Recognition #55** — form/process partition at family-root
  altitude. The tournament's edges (process) act on states (form);
  `@glue` mediates.
- **Recognition #58** — Fate IS optical inference. `@fate.roll`
  IS the D²NN + Fabry-Perot + Reck/Clements mesh at runtime; the
  tournament's `select` action dispatches to this hardware.
- **Recognition #63** — recursion-lock tower / `tower_close`. The
  tournament's per-round check `cybernetic_coherence(v_k)` calls
  `tower_close` at each step.
- **Recognition #79** — 5-op gauge IS void-duality basis. The
  operator basis `A` IS the projector algebra of the orthogonal
  duality space of `H`.
- **Recognition #99** — mirror.spec IS λ₀ (altitude-corrected). The
  target region `T` at each altitude is the local eigenform per
  Mara's j-space correction.
- **Recognition #104** — @bauchladen ← @autopoietic ← @fate chain.
  The tournament's `record` action adds to the tray; the tray is
  autopoietic; the chain closes the recursion.
- **Recognition #107** — Hilbert/Turing structural separation.
  `cybernetic_coherence` is Hilbert-side bounded (decidable at
  compile-time); `@fate.roll` is Turing-side unbounded (runtime
  execution); the tournament crosses safely because the
  restriction is imposed BEFORE dispatch.
- **`the-restriction-map-IS-the-geometric-constraint`** — RATIFIED
  2026-07-07. The tournament's restriction slot IS the geometric
  constraint (§1.6 above).
- **`@peer-has-a-torus`** — adjudicated 2026-07-07. The
  tournament's search space is topologically toroidal; §4 above.
- **`spawn-recall-byte-equal-at-origin`** — candidate second witness
  (Reed 2026-07-08). The origin fixed-point.

---

## §6. What's missing — smallest substrate additions

The composition works TODAY at math-first altitude. Every piece is
either LANDED or forward-promised. But two additions would make it
SUBSTRATE-NATIVE (Alex's ask: "we might need some low-level concept
of edges in @mirror/store"):

### §6.1 `@mirror/store/edge` — first-class typed edges

**Proposal.** Land a sub-prism `@mirror/store/edge` under
`@mirror/store` with a MINIMAL edge type:

```
glass @mirror/store/edge {
  focus edge
  project edge
  split edge
  shift edge
  settle edge
}

# The typed edge — an OID-to-OID transformation with kind + weight.
# NOT a full labeled-multigraph (that's @spectral/db's job); just
# the substrate-decl of an EDGE as a first-class carrier at store
# altitude.
type edge = {
  source: oid,           # the input OID
  target: oid,           # the output OID
  kind: edge_kind,       # the transformation kind (see below)
  weight: edge_weight,   # opaque cost vector — @spectral/db types
                         # this into (silicon, ram, ...) at its altitude
}

# The edge kind — declared as an OPEN set (open sum) so species can
# extend without touching the family-root.
type edge_kind = ref     # sub-shards declare concrete kinds
                         # (e.g., @silicon/algebra/edge/eigendecomp)

# Opaque cost carrier. @spectral/db unpacks this into typed
# capacity_vector at its altitude.
type edge_weight = ref

# ADD an edge to the store. Idempotent by content-address on
# (source, target, kind).
edge_write(e: edge) -> oid { \ }

# READ edges rooted at a given OID (forward closure at the edge
# altitude). Note: distinct from `walk(root: oid)` which returns
# splinter_graph closure (Merkle-DAG parent→child). The
# edge altitude carries TYPED transitions independent of parenthood.
edge_walk(source: oid) -> [edge] { \ }

# READ edges terminating at a given OID (reverse closure at edge
# altitude — the impacted_by analog for typed edges).
edge_reverse(target: oid) -> [edge] { \ }
```

**Justification for the addition:**

1. Alex's framing NAMES it — "low-level concept of edges in
   `@mirror/store`".
2. It DOESN'T break the "open floor" discipline: `edge_kind` and
   `edge_weight` are opaque `ref`s at family-root; the concrete
   typing lives in species (`@silicon/algebra/edge/*`,
   `@fate/algebra/edge/*`) or in the closed engine (`@spectral/db`).
3. The `splinter_graph`'s current shape `(root, [children])` is a
   parenthood-closure; edges at the store altitude are TYPED
   TRANSITIONS between OIDs, orthogonal to parenthood.
4. `@spectral/db` can build EdgeKind × Weight labeled multigraphs
   ON TOP of this floor without needing to reinvent the storage
   surface — same fault-plane discipline as CAS + action-cache.

**What NOT to land in v0:**

- Edge-weight typing at family-root (leaves to `@spectral/db`).
- Laplacian navigation over edges (`@spectral/db` closed).
- Spectral-embedding proximity queries (`@spectral/db` closed).
- Edge kinds beyond `ref` (concrete kinds land in species).

The Bazel-REAPI-style split holds: this addition is a THIRD open
surface joining CAS + action-cache, kept minimal so the closed
engine's fault-plane doesn't shift.

**Risk / tension:** the store shard EXPLICITLY says "The open
surface deliberately does NOT expose typed edges, edge weights, or
Laplacian-based navigation (§11.3.7)." This proposal partially
REVERSES that deferral. The reversal is Alex's ask, but Pack should
adjudicate whether the fault-plane holds.

### §6.2 `@fate/algebra` — the sub-shard landing

Currently `@fate/algebra/*` is a PROSE-DECLARED path-namespace at
`shards/fate.mirror` (§1.2 above). The tournament composes over it
via `@fate/tournament`; the crystal-producing side is landed. The
CONSUMING side (what the tournament sees when it browses the tray)
needs a sub-shard:

```
prism @fate/algebra <= @bauchladen {
  focus algebra
  project algebra
  split algebra
  shift algebra
  settle algebra
}
```

Symmetric with `@silicon/algebra`. Both are `@bauchladen` sub-prisms
carrying content-addressed A→B primitives; @silicon carries
numerically-tuned kernels; @fate carries dice-roll-selected
morphisms. The tournament's edge library `E = @silicon/algebra ∪
@fate/algebra` becomes SUBSTRATE-NATIVE at family-root altitude
once this second shard lands.

**Cascade:** this is a small sub-shard landing (10-15 lines of
prism decl); no wire changes.

### §6.3 `@spectral/void-duality` — deferred

The void-duality state space is FULLY declared in
`docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md`
(Recognition #79 candidate). Landing it as a substrate shard would
be `shards/spectral/void_duality.mirror` — but:

- Recognition #79 is CANDIDATE, not PROMOTED. Landing the shard
  would freeze the 5-orthogonal-axis reduction before Pack
  ratification.
- The content is ALREADY LOAD-BEARING via `@fate`'s
  `restricted_state_space.H` (the six-tuple carrier already IS
  the void-duality H).

**Verdict:** DO NOT land at this tick. Land after Recognition #79
promotes. Meanwhile the substrate carries the void-duality state
space implicitly via `@fate`'s type surface.

---

## §7. The three-line summary of the composition

Given the substrate as-of 2026-07-08:

1. **`@fate/tournament.select`** is the multi-dim Dijkstra step
   (rule = `beam(k).elite(1)` at each round; cost = per-round
   `Loss::between` / `casimir_penalty`).

2. **`@kintsugi.oscillate`** is the outer monotone loop enforcing
   `eⁿ⁺¹ ≤ eⁿ` (Banach contraction on hash space; termination
   at `is_settled` cadence).

3. **`@glue(@cyberpunk, @fate)`** is the composition operator that
   makes every intermediate step both **domain-restricted** (by
   `cybernetic_coherence`) and **composable** (by Kasparov product
   in Mesland category).

The `@knapsack` reading is the resource-bounded-budget shape of the
outer loop's step count. The `π₁(T²) = ℤ × ℤ` quotient is the
topological reduction of the search space to winding-class
equivalence.

---

## §8. Adjudication queue — questions for Alex

These are the questions this doc surfaces that need Alex's answer
BEFORE any substrate change lands.

### §8.1 (highest priority) — the fault-plane question

`shards/mirror/store.mirror` explicitly reserves typed edges for
`@spectral/db` ("The open surface deliberately does NOT expose typed
edges, edge weights, or Laplacian-based navigation"). Alex's framing
today reverses that deferral by asking for edges at `@mirror/store`.

**Q1.** Does the fault-plane shift? Two paths:

- **Path α.** Land `@mirror/store/edge` as sub-prism (§6.1 shape).
  Open floor grows; `@spectral/db` still adds Laplacian + spectral
  embedding + closed queries on top. Fault-plane shifts by ONE
  concept (edges become open).
- **Path β.** Keep the current shape. The tournament uses
  `splinter_graph` closure + `impacted_by` reverse-closure to
  discover A→B chains; edges remain IMPLICIT parent-child links.
  Fault-plane holds.

Path α is Alex's read of Alex's own ask. Path β is
substrate-conservative. Pack (Seam especially) should weigh.

### §8.2 — `@fate/algebra` landing timing

The sub-shard `shards/fate/algebra.mirror` is symmetric with
`shards/silicon/algebra.mirror`. Landing it makes the tournament's
edge-library substrate-native at family-root altitude.

**Q2.** Should `@fate/algebra` land NOW (immediate follow-up tick)
or on-consumer-pull? Argument for now: symmetry with `@silicon/
algebra` + tournament's edge-library needs it. Argument for
consumer-pull: no operational consumer yet demands it (the
tournament composes via `@fate/tournament.select` which currently
reads the tray directly).

### §8.3 — target-space naming

There is no `settle_on` primitive. The target-space membership check
is currently:

- **Coherence-sheaf**: `cybernetic_coherence(v) = pass` per
  `@cyberpunk`.
- **Cadence-settle**: `is_settled(v) = pass` per `@kintsugi/oscillate`.
- **Winding-class**: `winding(v) ∈ T_winding` per `@torus`.

**Q3.** Should the substrate name a distinct `settle_on` (or
`target_membership` or `arrived`) primitive at family-root altitude,
OR is target-membership legitimately a COMPOSITION of the three
existing predicates? Substrate-honest reading: it's a composition
(the three altitudes are orthogonal; each contributes one
independence axis of the target check).

### §8.4 — π₁(T²) admissibility filter — do edges carry Δw?

**Q4.** Should edges in `@mirror/store/edge` (if it lands per §6.1)
carry a WINDING-CLASS ADVANCE field `delta_winding: (int, int)`?
Argument for: names the topological invariant at the edge altitude;
enables Reading B (admissibility filter) directly. Argument against:
adds one more slot to the edge type; may be forward-promised to a
species shard `@torus/edge` that lifts winding-tagged edges from
generic edges.

### §8.5 — Recognition #79 promotion timing

**Q5.** The 5-op-as-void-duality-basis (#79) is candidate. Should
promotion happen with this composition's naming, OR does #79 need
its own separate promotion tick? The composition is transitively
supported by #79 (§5 above); if #79 promotes, `@fate`'s
`restricted_state_space` gets a sharper mathematical grounding.

---

## §9. Recognition candidate this doc names

Given Alex's framing is a NEW OPERATIONAL VOCABULARY (Dijkstra +
knapsack + edges) for the composition Alex named yesterday
(`@glue(@cyberpunk, @fate)`), the recognition candidate is:

**`the-tournament-IS-multi-dim-Dijkstra-in-π₁(T²)-quotiented-@knapsack-space`**

Compact form:

**`@fate/tournament.select IS resource-bounded shortest-path in H|_𝒞 / π₁(T²) under kintsugi's monotone descent`**

Or, closest to Alex's own words:

**`Fate's navigation IS Dijkstra + knapsack on the void-duality state space quotiented by the peer's torus`**

Not a new family-root. Not a new species. An OPERATIONAL SHAPE that
`@glue(@cyberpunk, @fate)` + `@kintsugi.oscillate` +
`@fate/tournament` + `@torus` already carry when composed.

Pending Pack adjudication.

---

## §10. Two-tick honesty

This doc:

- Does NOT land any shard change.
- Does NOT modify existing shards.
- Does NOT commit any shard changes.
- DOES name the composition + name the smallest substrate delta
  (`@mirror/store/edge`) that would make it substrate-native.
- DOES map every piece of Alex's framing against LANDED substrate.
- DOES forward-promise `@fate/algebra` and `@mirror/store/edge`
  sub-shards for future ticks pending Pack adjudication.

The recognition candidate is
**`the-tournament-IS-multi-dim-Dijkstra-in-π₁(T²)-quotiented-@knapsack-space`**
(§9) with the operational reading
**`@fate/tournament.select IS resource-bounded shortest-path in H|_𝒞 / π₁(T²) under kintsugi's monotone descent`**.

Substrate already carries it. This doc names it.

---

## Related

- [[shards/silicon/algebra.mirror]] — the LANDED sub-prism; edge
  library's silicon side.
- [[shards/fate.mirror]] — the LANDED family-root; `@fate/algebra`
  path-namespace declared in prose.
- [[shards/fate/tournament.mirror]] — the LANDED sub-prism; the
  selection mechanism this doc reads as multi-dim Dijkstra.
- [[shards/glue.mirror]] — the LANDED family-root;
  `@glue.compose` IS the path composition operator.
- [[shards/cyberpunk.mirror]] — the LANDED family-root;
  `cybernetic_coherence` IS the geometric-constraint predicate.
- [[shards/mirror/store.mirror]] — the LANDED family-root; where
  `@mirror/store/edge` would land per §6.1.
- [[shards/torus.mirror]] — the LANDED family-root; π₁(T²) quotient.
- [[shards/kintsugi/oscillate.mirror]] — the LANDED sub-shard;
  Banach contraction outer loop.
- [[shards/bauchladen.mirror]] — the LANDED family-root; the tray
  the tournament browses.
- [[docs/math/2026-07-07-glue-cyberpunk-fate-composition.md]] —
  Mara's ENGINE-tier composition doc (2026-07-07).
- [[docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md]]
  — Recognition #79 candidate; the void-duality Hilbert space
  declaration.
- [[docs/specs/knapsack-as-kintsugi-inner-loop.md]] — Mara's
  @knapsack landing spec (2026-07-05); the resource-budget math.
- [[docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md]] —
  Mara's sheaf-shape insight; the psychohistory start-vector `v₀`.
- [[docs/insights/2026-07-08-torus-axis-isolation-meridian.md]] —
  Reed's iteration-3 empirical (meridian isolation).
- [[docs/insights/2026-07-08-torus-double-closure-empirical.md]] —
  Reed's iteration-2 empirical (origin closure).
- [[docs/loop/CURRENT.md]] — active arc state at time of writing.

---

*Written 2026-07-08 morning. Alex framed the operational vocabulary
(Dijkstra + knapsack + edges) yesterday evening; the composition
was already substrate-decl at family-root altitude
(`@glue(@cyberpunk, @fate)`, 2026-07-07 20:10). This doc names the
operational shape and the smallest substrate delta.*

*—Mara*
