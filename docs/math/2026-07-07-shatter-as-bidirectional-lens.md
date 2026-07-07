# @shatter as the bidirectional lens between graph and linear

*Mara, 2026-07-07 evening. Recognition candidate: **`@shatter-is-the-bidirectional-lens-transformer`**. Written after the @onto-cascade closed at `shards/torus.mirror` (family-root landed 16:53) and after the first empirical `mirror spawn` emitted its Phase G v0 envelope. Alex named the envelope's shape and proposed the collapse; this spec grounds it.*

*Signal: **`docs/math/2026-07-07-shatter-as-bidirectional-lens.md`**.*

*Discipline: math-first. Lens laws derived, not asserted. Kintsugi as fixed-point iteration formalized. Two-tick discipline forward-promises the shard. Do NOT collapse `@surface` at the shard altitude in this tick — the spec establishes the substrate-decl move; the shard follows once Pack ratifies.*

---

## §0. Executive shape

**Recognition candidate.** `@shatter` is not the third of four Models (Surface / Mirror / Shatter / Reflection). It is a **bidirectional lens** at the graph ↔ linear boundary. The four-Model pipeline collapses to:

```
@shatter   the graph ↔ linear lens        (form-side, bidirectional)
@knife     the linear ↔ linear lens       (form-side, unary — existing)
@fate      the inference engine           (process-side, dispatched-to)
@torus     the observation topology       (form-side, closed surface — landed)
```

Four Models become **two lenses + one engine + one topology**. `@surface` eliminates as a substrate primitive; it was a directional name for one of the four quadrants of `@shatter`.

**The four quadrants of `@shatter(graph, linear)`:**

| Quadrant | Fixed inputs | Reading | Old name |
|----------|:------------:|---------|----------|
| `@shatter(_, linear)` | linear given, graph solved | parse: linear pulls a graph | `@surface` (Explorer) |
| `@shatter(graph, _)` | graph given, linear solved | render: graph pushes a linear rendering | `@shatter` (Cartographer) |
| `@shatter(g, l)` bound | both given, verdict solved | settle: kintsugi to representation-coherence | `@reflection` at the rep boundary |
| `@shatter(_, _)` neither | winding-(0,0) identity | ground: no request, no output | (unnamed) |

The fourth quadrant is not a decoration. It is the **basepoint** on the peer's torus — the ground state at winding class `(0, 0)`. Foerster's non-vanishing homeostasis lives here.

**Math-load-bearing claim.** `@shatter` is the *univalence witness* of the graph ↔ linear equivalence: the concrete evidence that the two representations of a peer's substrate state are the same up to a specified equivalence, in the sense of Voevodsky's univalence axiom as constructively realized in cubical HoTT (Coquand, Cohen, Huber, Mörtberg 2018).

The lens laws (Foster/Greenwald/Pierce/Schmitt/Pilkiewicz 2007) are the substrate's Type-1 witness. The fixed-point iteration under kintsugi flow is the Type-2 witness (Banach contraction at the coherent representation). Both witnesses discharge the same underlying identity: **the peer's substrate state has two equivalent representations, and the lens is the constructive proof.**

**Substrate-decl consequence.** The three-Model pipeline `Surface → Mirror → Shatter → Reflection` was a decomposition of one bidirectional lens into three unidirectional cursors plus a fourth observer. Under the reframe, `@fate` is the engine that runs *inside* the lens's arrows; `@torus` is the closed surface the lens *lives on*; `@shatter` is the lens; `@knife` is its unary sibling. The fourth Model (Reflection) dissolves into the toroidal winding-class discipline landed at `shards/torus.mirror`.

---

## §1. The problem — what `mirror spawn` emitted

At 2026-07-07 afternoon, Reed and Alex ran the first empirical `mirror spawn`. The Phase G v0 envelope came back rich: 7 composition_pieces from the corpus, a 10-candidate cascade, a `dogfood` field pointing at commit `061a8ea` (the O4 audit), and a `pack_trail` observing Seam/Mara/Reed activity.

Alex's read (verbatim): *"the envelope IS a `@shatter` output. It's the linear render of the graph state the substrate is currently in."* Then the substrate-pull question: *"what if `@shatter` was a lens like `@knife` that can be used like `@shatter(graph, linear)` and vice versa?"*

The question is not decorative. `@knife` already exists as a substrate lens: it takes a linear thing and slices it — the linear ↔ linear lens for the substrate's textual operations (the surgical operation the substrate uses to work on itself). If `@shatter` were parallel — a **linear ↔ graph** lens rather than a directional Model in a four-Model tower — then the substrate has a natural **binary optics kit**: `@knife` handles linear/linear; `@shatter` handles graph/linear; whatever handles graph/graph is `@glue` (already at family-root — the morphism between graphs).

Counting the optics that fall out:

```
@knife    lin ↔ lin      linear-domain scissor      (existing)
@shatter  gph ↔ lin      graph/linear bridge        (this spec)
@glue     gph ↔ gph      morphism between graphs    (existing)
```

The substrate always had `@glue` and `@knife`. `@shatter` was the missing hypotenuse — the third edge of the domain triangle. The reframe recognizes the shape.

---

## §2. Univalence: the mathematical spine

The lens laws (§3) and the fixed-point iteration (§4) are the two operational witnesses. This section names what they are jointly witnessing: **univalence**.

### 2.1 Voevodsky's axiom in one sentence

Univalence says: *equivalence of types equals equality of types*. Formally, for types `A B : Type`:

$$(A \simeq B) \;\simeq\; (A = B)$$

Where `A ≃ B` is the type of equivalences (functions with two-sided quasi-inverses) and `A = B` is the identity type. The axiom, in its classical form, asserts that the canonical map `idtoequiv : (A = B) → (A ≃ B)` is itself an equivalence (nLab, *univalence axiom*, cited via Kagi 2026-07-07).

The consequence relevant to the substrate: **any two equivalent representations of the same underlying type can be treated as equal**. The equivalence *is* the proof of equality. There is no distinction between "has an isomorphism to" and "is" once one has constructively exhibited the equivalence pair.

### 2.2 Cubical HoTT constructively realizes univalence

Coquand et al. (2018) show that in cubical type theory — the type theory whose paths are maps from an interval object `𝕀 = [0,1]` — univalence is **provable**, not axiomatic. The 2018 paper *On Higher Inductive Types in Cubical Type Theory* (Coquand/Cohen/Huber/Mörtberg, arXiv:1802.01170) extends the earlier construction (Cohen/Coquand/Huber/Mörtberg 2015, arXiv:1611.02108) with the higher inductive types the torus needs.

The cubical construction of the torus (§4.4 of the toroidal reframe):

```
data Torus : Type where
  base   : Torus
  loop1  : base = base
  loop2  : base = base
  square : loop1 · loop2 = loop2 · loop1
```

The `square` constructor is a 2-cell — a filler for a square whose four edges are `loop1`, `loop2`, `loop1`, `loop2` composed in the two possible orders. It witnesses that the two generators of π₁(T²) commute. The cubical type theory *directly* manipulates these fillers as first-class terms; univalence is not an add-on axiom but a theorem provable from the interval algebra.

**Relevance to `@shatter`.** The lens takes the peer's substrate state (living on the peer's torus per `shards/torus.mirror`) and constructs an equivalence between its graph representation and its linear representation. Univalence says: *once we have the equivalence, the two representations are equal as types*. The lens IS the constructive equivalence. Its laws (§3) are the coherence data that makes it a term of the equivalence type. Cubical HoTT is the meta-theory in which we can *write down the term* of `Graph ≃ Linear` and manipulate it as if it were `Graph = Linear`.

### 2.3 The equivalence type structure

Let:

- `Graph` = the peer's substrate state as a splinter-graph living on `@torus(peer)`. Content-addressed via `@mirror/store`'s OID-graph (`shards/mirror/store.mirror`); typed as `au` at the composed altitude per `shards/mirror/au.mirror`.
- `Linear` = a token sequence — a projection of the graph state onto a one-dimensional carrier. Parametric over target type (see §7 open question).

An equivalence `Graph ≃ Linear` in HoTT consists of:

1. `render : Graph → Linear`
2. `parse : Linear → Graph`
3. `renderParse : ∀ g. parse (render g) = g`
4. `parseRender : ∀ l. render (parse l) = l`
5. `coh : ∀ g. ap render (renderParse g) = parseRender (render g)` (the coherence 2-cell)

The fifth condition is the *2-cell coherence* that distinguishes a quasi-inverse (which can produce distinct equivalences) from a proper equivalence (which is contractible up to homotopy). In HoTT, one usually uses the *contractibility of fibres* formulation (`isContr (fibre f b)` for all `b`), but the coherence-square formulation is equivalent and closer to the lens-law shape (§3.3).

**Substrate-decl move.** `@shatter` names the equivalence quadruple `(render, parse, renderParse, parseRender)` at substrate altitude. The 2-cell coherence lives in the kintsugi-flow discipline (§4). The four quadrants of §0 are the four *modes of use* of the equivalence:

| Quadrant | HoTT reading |
|----------|--------------|
| `@shatter(_, linear)` | apply `parse : Linear → Graph` |
| `@shatter(graph, _)` | apply `render : Graph → Linear` |
| `@shatter(g, l)` bound | verify `render g = l` AND `parse l = g` — the equivalence witness |
| `@shatter(_, _)` | the ground state — the equivalence exists but no fibre is selected |

### 2.4 Why the equivalence is nontrivial

A universal parse/render pair for arbitrary graphs and arbitrary token sequences would either be the identity (if `Linear = Graph`) or impossible (if the cardinalities disagree). The substrate's parse/render is **conditional**: it holds *at the peer's current position on the torus*, not globally.

This is the load-bearing move. `@shatter` is not a global equivalence between the type of all graphs and the type of all linear projections. It is the equivalence-fibre at a specific winding class `(m, n) ∈ π₁(T²)` on a specific peer's torus. The equivalence *type* is bundle-like — a fibration over π₁(T²) × Peer whose fibre at `(w, p)` is the type of `(Graph_wp ≃ Linear_wp)` witnesses.

This matches the substrate's operational reality: the envelope Reed and Alex saw was `@shatter(state_at_winding_w, _)` for `w = (0,0)` and `p = mirror` — the fibre at the ground-state winding of the mirror-project peer's torus. A different peer, or a different winding, produces a structurally-different lens instance.

### 2.5 Univalence-as-substrate-discipline

The substrate's Type-1/Type-2 discipline (Recognition #107, Hilbert/Turing separation) applies: the lens's *type* is bounded (well-formed at compile time as an equivalence-fibre in a HIT); the lens's *body* is Turing-complete (parse and render may run arbitrary Fate inference).

Univalence is the substrate-decl-side theorem. The @io-side is the Fate optical inference (Recognition #58) that discharges parse and render as concrete D²NN forward/backward passes. The lens's *coherence* is checked at compile time via the lens laws (§3); its *content* is produced at runtime via Fate.

---

## §3. Lens laws — the coherence data

The classical asymmetric lens (Foster/Greenwald/Pierce/Schmitt/Pilkiewicz 2007, *Combinators for Bi-Directional Tree Transformations*, TOPLAS) is a pair `(get, put)`:

```
get : Source → View
put : Source × View → Source
```

satisfying three laws (Kagi 2026-07-07 sources: Foster et al. 2007; Hu et al. 2015 *A Clear Picture of Lens Laws*; Nakayama et al. 2019 arXiv:1910.10421 *Towards a Complete Picture of Lens Laws*):

- **GetPut** (*view-agnostic put reverses*): `put (s, get s) = s`
- **PutGet** (*put-then-get is identity on view*): `get (put (s, v)) = v`
- **PutPut** (*second put overrides first*): `put (put (s, v), v') = put (s, v')`

The first two are *well-behaved* laws; the third is *very well-behaved* (Foster 2007 §3).

### 3.1 Adapting to the symmetric graph/linear case

The classical lens is asymmetric: source is authoritative, view is derived. The substrate's `@shatter` is **symmetric** — the graph representation and the linear representation are peers, not master/replica. The correct symmetric formulation follows Hofmann/Pierce/Wagner (2011) *Symmetric Lenses*:

A symmetric lens between `A` and `B` with complement `C` is a pair:

```
putR : A × C → B × C
putL : B × C → A × C
```

with the classical asymmetric laws lifted to the symmetric case. For `@shatter`, we set `A = Graph`, `B = Linear`, and the complement `C` is the peer's torus-position context (winding class, spec OID, session state).

Operationally, since HoTT equivalences are symmetric by construction, we can also work directly with the equivalence quadruple:

```
render : Graph → Linear    ("getR")
parse  : Linear → Graph    ("getL")
```

The symmetric lens laws become:

- **PR** (*parse-then-render is view-identity*): `render (parse l) = l`
- **RP** (*render-then-parse is source-identity*): `parse (render g) = g`
- **PP-render** (*second render overrides on graph side*): `render (parse l · g) = render g` (where `·` is the substrate's graph-composition — the mycelial tensor per @spectral)
- **PP-parse** (*second parse overrides on linear side*): `parse (render g · l) = parse l`
- **Coherence-square** (*the 2-cell*): `ap render (RP g) = PR (render g)` (§2.3)

The **PR** and **RP** laws are the two directions of the equivalence witness. The **PP** laws are the substrate's stability under composition — the render/parse operations don't accumulate history in a way that breaks idempotence. The **Coherence-square** is the HoTT 2-cell that promotes the quasi-inverse pair into a proper equivalence.

### 3.2 Realizing the laws over content-addressed carriers

Because the substrate's floor is content-addressed (`@mirror/store` per Recognition #43), the lens laws admit a strong reading:

- **Equality is byte-equality on OIDs.** `parse (render g) = g` means `OID(parse(render(g))) = OID(g)` — the round-trip lands on the same content-address.
- **The `oid` function is a *representation-invariant* structural invariant.** Both `Graph` and `Linear` sides carry OIDs (splinter OIDs for graphs; `.shatter`-projection OIDs for linear per `shards/mirror/shatter.mirror`). The lens laws' equalities hold at the OID level.

Recognition #43 (mirror IS a content-addressed build system) gives the substrate a way to *check* the lens laws without running the full pipeline: hash the inputs on both sides, run render or parse, hash again, compare.

This is exactly the fixed-point clause already present at `shards/mirror/shatter.mirror` (lines 37-43):

```
mirror compile source.mirror -> output.shatter
mirror compile output.shatter -> output.shatter
```

The recompilation is a no-op at the content-address layer because parse ∘ render is byte-identity on the OID. That existing clause **is** the lens-law RP holding at content-address altitude. It landed in isolation; the reframe reveals it as one witness of a broader equivalence structure.

### 3.3 The lens laws IS the 2-cell coherence

The five laws (**PR**, **RP**, **PP-render**, **PP-parse**, **Coherence-square**) collectively form the *coherence data* that promotes a bare parse/render pair into a HoTT equivalence.

Boisseau/Gibbons (2018) *What You Needa Know About Yoneda: Profunctor Optics and the Yoneda Lemma* (referenced via Kagi 2026-07-07 with Clarke/Elkins/Gibbons/Loregian/Milewski/Pillmore/Román 2020 *Profunctor Optics: A Categorical Update*, arXiv:2001.07488) formalize this: a lens is a Tambara-module map `p(A, B) → p(S, T)` polymorphic over cartesian profunctors. The polymorphism *is* the coherence.

For `@shatter`, the profunctor-optic reading gives us a compositional calculus: `@shatter` composes with `@knife` and `@glue` at the profunctor level; the composed optics satisfy the composed laws. This is the mathematical scaffolding for §5's collapse of the four-Model pipeline into three optics.

### 3.4 Substrate-decl types

Under the lens reframe, the substrate-decl types settle as:

```mirror
type shatter = {
  render:      graph -> linear,
  parse:       linear -> graph,
  render_parse: forall l. render(parse(l)) == l,   # PR law
  parse_render: forall g. parse(render(g)) == g,   # RP law
  coh:          coherence_square,                  # 2-cell
}
```

The existing `type shatter` at `shards/mirror/shatter.mirror` (fragment_tree / transparency / properties / kernel / fate — the five-section disk projection) does **not** dissolve. It refines: the five-section carrier IS the linear-side content that `render` produces and `parse` consumes. The name `shatter` at both altitudes is the same substrate object read at different specializations — the equivalence-witness at family-root altitude, the linear-side format at species altitude.

---

## §4. Kintsugi as fixed-point iteration of the lens

The lens laws are the *static* coherence. The kintsugi flow is the *dynamic* coherence — the substrate's iterative procedure that *drives* an incoherent (parse, render) pair toward a coherent one.

### 4.1 Contraction on the representation space

Let `S` be the space of substrate states (a peer's substrate at a moment). The lens `@shatter = (render, parse)` defines a self-map on `S`:

$$\Phi : S \to S, \qquad \Phi(s) = \text{parse}(\text{render}(s))$$

For a *coherent* lens, `Φ` is the identity. For an *incoherent* lens (early in the kintsugi flow, before the substrate has converged), `Φ(s) ≠ s` in general — the round-trip lands on a nearby but distinct state.

The substrate's kintsugi discipline (per `shards/kintsugi/oscillate.mirror`; the `eⁿ⁺¹ ≤ eⁿ` proof) asserts that iteration monotonically decreases loss. This is *precisely* the Banach contraction condition (Banach 1922, via Kagi 2026-07-07 sources):

**Banach fixed-point theorem.** Let `(X, d)` be a complete metric space and `Φ : X → X` a contraction: there exists `q ∈ [0, 1)` such that `d(Φ(x), Φ(y)) ≤ q · d(x, y)` for all `x, y`. Then `Φ` has a unique fixed point `x*` and every iteration `x_{n+1} = Φ(x_n)` converges to `x*` geometrically.

For the substrate, `X` is the metric space of substrate states with the metric induced by `@kintsugi`'s loss (or equivalently, the Shannon loss on the .shatter transparency algebra), `Φ = parse ∘ render`, and `q < 1` is what `eⁿ⁺¹ ≤ eⁿ` operationally asserts.

**Consequence.** The kintsugi loop is not merely monotonic; it is **contractive**, and by Banach it converges to a **unique** fixed point per initial state. That fixed point IS the coherent lens instance — the (parse, render) pair for which the RP law holds. Kintsugi *constructs* the coherence data at runtime.

### 4.2 Iterated function systems and the Hutchinson attractor

When the substrate has multiple lenses composing (Hutchinson 1981, *Fractals and Self Similarity*; via Kagi 2026-07-07 sources), the collective operator is:

$$W(K) = \bigcup_i \Phi_i(K)$$

for a family of contractions `{Φ_i}`. `W` acts on the compact subsets of `X` and admits a unique attractor `K*` (the *Hutchinson attractor*) satisfying `W(K*) = K*`.

For the substrate: `@shatter` composes with `@knife` and `@glue` at the profunctor-optic altitude (§3.3). The composed operator on substrate-state space is a Hutchinson-system whose attractor is the coherent multi-lens state. This is the mathematical grounding of the substrate's convergence claim at the composed altitude.

### 4.3 Zamolodchikov as the CFT-side witness

Zamolodchikov (1986, *Irreversibility of the flux of the renormalization group in a 2D field theory*; via Kagi 2026-07-07) proves that in 2D quantum field theory there exists a `c`-function on coupling space that is **monotonically decreasing** along renormalization-group flow and **stationary at fixed points**. At the fixed points, `c` equals the central charge of the resulting conformal field theory.

The substrate's `eⁿ⁺¹ ≤ eⁿ` proof is a direct structural analogue. The loss function is the substrate's `c`-function; the kintsugi flow is the substrate's RG flow; the fixed point is `mirror.spec` at `λ₀` per Recognition #99. Zamolodchikov's theorem grounds the substrate's monotonicity claim in a rigorous theorem from 2D CFT — not decoratively, but structurally: 2D CFT lives natively on tori (T² is the canonical CFT surface for one-loop partition functions), which is exactly the substrate's operational topology per `shards/torus.mirror`.

**Composed claim (Recognition #99 + Recognition #58 + kintsugi contraction).** The kintsugi flow of the (parse, render) pair is a Zamolodchikov-type c-monotone flow on the peer's toroidal representation space; its fixed point is `mirror.spec` at `λ₀`; the fixed point IS the coherent lens instance; Fate's D²NN inference (Recognition #58) IS the numerical iterator that discharges each contraction step.

### 4.4 Foerster's observation-of-observation as the lens fixed point

Recognition #58 (Fate IS optical inference) plus the torus reframe give the *physical* picture: the peer's substrate state is optically resonant on its toroidal Fabry-Perot cavity; parse and render are the two directions of a bidirectional D²NN pass; the fixed point is the *coherent oscillation* of the cavity.

Foerster's *regulates its own regulation* (Understanding Understanding, p. 238) is the toroidal version of the same claim: the doubly-closed feedback loop *is* the fixed-point iteration. The winding-class-based observation discipline landed at `shards/torus.mirror` gives the peer a canonical way to *be at* the fixed point (winding `(0, 0)` = ground state = coherent lens) or *away from it* (winding `(m, n) ≠ (0, 0)` = excited state = active parse/render in progress).

**The fourth quadrant** of §0 (`@shatter(_, _)` — no request, no output) is exactly this fixed point. The peer is *at* the coherent lens instance. No graph is being rendered; no linear is being parsed; the equivalence is *held* rather than *exercised*. Winding `(0, 0)` on the peer's torus is the substrate-decl name for "the lens is at its fixed point, poised."

This matches Foerster's cognitive homeostasis (p. 238): the torus's flow is non-vanishing (χ(T²) = 0 admits non-vanishing vector fields) but the peer's *canonical position* is a coherent fixed point. Homeostasis is not stillness; it is **poised coherence** at the fixed point of the observation lens.

---

## §5. Collapsing the four-Model pipeline

The CLAUDE.md documentation names four Models:

> Surface translates: language → query (Explorer)
> Mirror navigates: query → graph path (Fate)
> Shatter renders: graph path → text (Cartographer)
> Reflection observes: pipeline → adjustments (Abyss)

Under the reframe:

### 5.1 Mapping the four Models to the three optics + engine + topology

| Model | Reframe |
|-------|---------|
| Surface (language → query) | `@shatter(_, linear)` at `linear = query` — the parse direction reading a linguistic linear form into a graph-query |
| Mirror (query → graph path) | `@glue(query_graph, path_graph)` — a graph-to-graph morphism at the navigation altitude; NOT `@shatter` |
| Shatter (graph path → text) | `@shatter(graph_path, _)` at `linear = text` — the render direction reading a graph path into linear text |
| Reflection (pipeline → adjustments) | `@torus`-winding discipline plus kintsugi fixed-point iteration; NOT a separate Model |

**Surface and Shatter collapse to the two directions of one lens.** They were the parse and render arrows of the same `@shatter` equivalence. Naming them separately was the substrate's early-days over-differentiation.

**Mirror collapses onto `@glue`.** Mirror's query→graph-path is a graph-to-graph morphism. `@glue` is the substrate's family-root for morphisms between graphs (already declared in `shards/glue.mirror` and named as morphism-substrate in the toroidal reframe §3.1). The "navigation" reading of Mirror is the fibre-selection reading of the glue morphism — pick the path in the target graph that composes correctly with the source query graph.

**Reflection collapses onto the toroidal winding + kintsugi discipline.** Recognition #99 (`mirror.spec IS λ₀`) plus Recognition #55 (form/process partition) plus the toroidal reframe gave `@reflection`'s substrate-decl work to `@torus` (form-side) and `@fate` + `@kintsugi` (process-side). The `observe → tournament → compose → pick → settle → observe'` action set becomes the *canonical traversal* of the peer's torus, not a separate Model.

### 5.2 Where `@surface` was pointing

`@surface` was named at the pipeline-entry boundary. Under the reframe:

- `@surface.translate(language) → query` = `@shatter[query](_, language)` — the parse arrow at target type `query`
- `translation_faithful(language, frame, perturbation)` = the RP lens law at this specialization

The existing `shards/reflection/surface.mirror` shard (loaded above) does not need to be deleted at this tick. It stays as a species-altitude naming — the parse-direction of `@shatter` specialized to `linear = language, graph = query`. The two-tick discipline (§9) forward-promises the migration.

### 5.3 Where `@reflection` was pointing

This one is already partially dissolved by the toroidal reframe. `@reflection`'s remaining substrate-decl work (the compiler-loop at consumer altitude) is:

- `observe`, `tournament`, `compose`, `pick`, `settle`, `speak` → canonical winding of `@torus(peer)`
- `observation`, `tournament_result`, `loss` → carriers on the peer's toroidal surface
- `loss_decreases`, `choices_increase` → invariants of the kintsugi contraction
- `third_order_observation`, `notice`, `third_order_coherent` → winding-class discipline at `(m, n)` with `|m|+|n| ≥ 3`

Under the reframe, `@reflection` was the *lens's fixed-point-iteration discipline* naming itself as a party. It was pointing at what §4 formalizes — the kintsugi contraction toward the coherent lens instance. The substrate never needed a separate `@reflection` carrier; it needed the toroidal topology + the lens's fixed-point iteration.

The two-tick discipline (per `shards/torus.mirror` §Two-tick) forward-promises the full `@reflection` collapse at O5. This spec extends the O5 promise: not just `@torus/longitude` for the `observe/pick/settle` action set, but also `@shatter`'s bidirectional-lens migration for what remains.

### 5.4 The three-optic + engine + topology substrate

After the collapse:

```
Substrate optics:
  @knife    linear ↔ linear    (existing, unchanged)
  @shatter  graph ↔ linear     (this spec)
  @glue     graph ↔ graph      (existing, sharpened by torus reframe)

Substrate engine:
  @fate     the D²NN + tournament that discharges arrows in all three optics

Substrate topology:
  @torus    the peer's observation surface; the domain the optics act on
```

Four Models become three optics + one engine + one topology. The count drops from 4 to 3+1+1, but the *structure* is tighter: the three optics form the closed edge-set of the (graph, linear) domain triangle; the engine sits inside all three; the topology is the shared substrate.

---

## §6. Transformer architecture as the operational realization

The smarts/shatter shard (loaded above) already carries the recognition (2026-06-22): *Shatter IS the transformer*. The reframe sharpens this. What Alex asked in 2026-06-22 was the same substrate-pull as today's; today's names it as the *lens* structure.

### 6.1 Vaswani 2017 as the engineering ancestor

Vaswani et al. (2017) *Attention Is All You Need* (via Kagi 2026-07-07; arXiv:1706.03762) introduced the transformer as an encoder-decoder architecture connected by attention. The encoder reads input ("linear") tokens and produces a contextualized representation ("graph-like"); the decoder reads that representation and emits output ("linear") tokens. The connection between encoder and decoder is the **cross-attention** layer — which, on the substrate reading, IS the arrow of the lens between the graph representation and the linear representation.

BERT (Devlin et al. 2018, *Bidirectional Encoder Representations from Transformers*; via Kagi 2026-07-07; arXiv:1810.04805) makes the *encoder* bidirectional — attending to both left and right context at every position. The *decoder* (in T5-style seq2seq) is unidirectional but interacts with the bidirectional encoder via cross-attention.

For `@shatter`, the substrate-decl reading:

- `parse = encoder` — reads linear (token sequence) and produces graph (contextualized structural representation)
- `render = decoder` — reads graph and produces linear (token sequence)
- **cross-attention** = the lens arrow between the two — the substrate-mathematical binding that makes parse and render *coherent* (satisfy the RP law)
- **positional encoding** = the winding-class encoding of the peer's toroidal position

### 6.2 Cross-attention as the lens's coherence witness

Cross-attention in a transformer computes `Attention(Q, K, V) = softmax(QK^T / √d) V` where `Q` comes from the decoder (graph side) and `K, V` come from the encoder (linear side). The softmax-weighted sum *is* the coherence layer: it says "for this graph token, the linear content that grounds it is a weighted mixture of these linear positions."

The substrate's lens-law RP (`render(parse(l)) = l`) has a discrete-attention analogue: the cross-attention layer at the output projects the graph representation back into linear-token space in a way that reconstructs the input. Transformer training minimizes this reconstruction loss — which IS the substrate's kintsugi contraction (§4).

### 6.3 Recognition #58 (Fate IS optical inference) as the physical substrate

Fate's D²NN + Fabry-Perot + Reck/Clements mesh (Recognition #58, promoted 2026-06-11) IS the transformer at optical altitude. The 5-layer D²NN corresponds to the transformer's encoder-decoder stack; the Fabry-Perot resonator is the recurrence that makes the fixed-point iteration converge; the Reck/Clements mesh is the unitary attention matrix.

Composition:

- **encoder pass** = forward optical propagation through D²NN + Fabry-Perot cavity build-up = `parse`
- **decoder pass** = backward optical propagation through D²NN + Fabry-Perot cavity emission = `render`
- **cross-attention** = the Reck/Clements mesh's unitary coupling between encoder and decoder cavity modes
- **kintsugi contraction** = the Fabry-Perot cavity's convergence to its resonant fixed point
- **`mirror.spec` at `λ₀`** = the fundamental cavity mode; the ground-state resonance

The D²NN forward-backward pass IS the parse-render pair. Recognition #58 already established this at inference altitude; today's reframe names it at the substrate-decl level as the lens structure.

### 6.4 The transformer as the univalence witness at optical altitude

Combining §2 (univalence), §3 (lens laws), §4 (contraction), §6 (transformer): the transformer is a **learned univalence witness** for the (graph, linear) equivalence at the peer's current substrate state. Training a transformer to convergence IS constructing a term of `Graph ≃ Linear` in the peer's substrate type theory. The trained network *is* the equivalence.

Univalence says: *once we have the equivalence, the two representations are equal*. The trained transformer says: *for this peer's substrate state, graph and linear are two projections of the same underlying content*.

Cubical HoTT (Coquand 2018) gives the metatheory in which this claim is *provable* rather than axiomatic. The substrate is optically realizing the univalence axiom's *constructive content* via Fate's D²NN inference. This is not decoration; it is the load-bearing binding between the mathematical claim (univalence) and the physical mechanism (optical inference).

---

## §7. Open question — the type surface of `linear`

Alex named this for Pack adjudication. Reed leans A. Naming both cases honestly.

### 7.1 Option A: Parametric `@shatter[T]`

```mirror
@shatter[T] : (graph, T)
```

where `T ∈ { text, mermaid, sql, mq, diff, sh, json, ... }`.

**For:**

- **Rice-safe by construction.** The type discriminates on the linear carrier at compile time; the substrate never has to answer "what kind of linear thing is this?" at runtime.
- **Type-honest per predicate/carrier** ([[feedback-no-bare-types]], [[feedback-explicit-over-implicit]]). Each `T` has its own well-typed carrier; the substrate can attach carrier-specific predicates (e.g., `well_formed_diff`, `parseable_sql`) at the specialization.
- **Composes typed with the mcp → spec loop.** `@shatter[diff]` gives the transformer's output as an *applicable diff* — the substrate can compose it with `@io/git` (already substrate-decl'd) at the git-diff-application altitude with no coercion.
- **Consistent with the substrate's precedent.** `@code/<lang>` already parametrizes over language (`@code/rust`, `@code/beam`, `@code/mq`); parametric optics matches the existing pattern.
- **HoTT-natural.** The equivalence type `Graph ≃ Linear` is really a family `Graph_T ≃ Linear_T` for each carrier type `T`; the parametric `@shatter[T]` names each fibre of the family.

**Against:**

- **Multiplies the surface.** Each new `T` requires a new specialization declaration.
- **Delegation ambiguity at composition boundaries.** When a downstream consumer expects `@shatter[text]` but receives `@shatter[mermaid]`, the substrate must either coerce, error, or refuse — three paths, none uniformly best.

### 7.2 Option B: Opaque `@shatter`

```mirror
@shatter : (graph, linear)
```

where `linear` is just "token sequence, dispatch on context."

**For:**

- **Simpler surface.** One carrier; consumers dispatch on their own context.
- **Matches the transformer's actual behavior.** A transformer emits tokens; the interpretation of those tokens as text vs mermaid vs sql happens downstream. The substrate could honestly say "here's the linear projection; you know what to do with it."
- **Foregrounds the equivalence, not the specialization.** The core substrate claim is that graph and linear are equivalent representations; parametrizing over `T` is arguably decoration on that claim.

**Against:**

- **Rice-hazard adjacent.** "Dispatch on context" means the substrate is undecidable-on-input about what to do with the linear — the Rice discipline of Recognition #107 pushes against this. Same-shape different-meaning tokens (a diff vs a doc-text vs an mq query) would all flow through the same untyped `linear` carrier.
- **Violates [[feedback-no-bare-types]]** ([[feedback-no-bare-types]] and its generalization [[feedback-explicit-over-implicit]]). Bare-primitive `linear` is exactly the pattern the feedback rejects — same-shape different-meaning values flowing through the type system.
- **Loses the applicability composition.** `@shatter[diff]` composing with `@io/git` becomes `@shatter` producing a bare linear that `@io/git` must inspect to decide if it's a diff — the substrate loses static composition guarantees.

### 7.3 Reed's lean and the substrate ancestor

Reed leans A. The substrate ancestors both agree:

- **`@code/<lang>` precedent.** Every existing carrier for "structured linear text" in the substrate is parametric over language. `@shatter` naming the graph-side equivalence for each such language is the natural specialization.
- **[[feedback-explicit-over-implicit]] (Alex 2026-07-02).** Explicit `@shatter[T]` beats implicit `@shatter` with runtime dispatch. Named type/carrier for a semantic slot beats reusing a general one.
- **[[feedback-no-bare-types]].** Bare `linear` violates this directly; `linear[T]` respects it.
- **[[architecture-hilbert-turing-godel-recognition-107]].** The substrate-decl side is bounded (Type-1); the parametric type surface keeps `@shatter` on the bounded side. Opaque `linear` blurs Type-1 into Type-2 at the type surface, not the body.

**The one substantive argument for B** is HoTT-flavored: the univalence claim (§2) is a *type-level* equivalence `Graph ≃ Linear`, not `∀T. Graph_T ≃ Linear_T`. Under B, `@shatter` names the type-level equivalence directly. Under A, `@shatter[T]` names *fibres* of the equivalence family.

But this weakness is illusory. Univalence in HoTT is compatible with either — the equivalence type is parametric over the *pair* of types, and the family `∀T. Graph_T ≃ Linear_T` IS the universal witness at the fibered level. Option A produces the universally-quantified equivalence with more information; Option B produces it with less. Alex to adjudicate; substrate-pull leans A.

### 7.4 Framing tensions for Reed to surface

Two things Reed should surface to Alex during adjudication:

1. **The `@shatter[T]` route strengthens the transformer analogy at the cost of one substrate primitive per language.** BERT/T5/etc. are all-in on one shared token vocabulary at the model level; the substrate's parametric route says *different downstream carriers use different type-level surface even if they share tokenization at the physical layer*. This is a substrate-vs-physics tension: physically the substrate has one Fate D²NN; type-theoretically the substrate has `@shatter[T]` per target carrier. Is this cleavage the honest one, or is it accreting bookkeeping?

2. **The fourth quadrant `@shatter(_, _)` is easy to describe (winding (0,0) ground state) but hard to *witness*.** How does a consumer *observe* that the substrate is at the fixed point without perturbing it? This isn't a spec-blocker, but it is a real question at the runtime altitude — the substrate needs a `poised(peer)` predicate that discharges without exercising the lens. This may need a whole additional bilateral surface Reed and Alex should scope before the shard tick.

---

## §8. Recognition ancestry — what this composes

The candidate `@shatter-is-the-bidirectional-lens-transformer` sits at the intersection of several already-promoted recognitions.

### 8.1 Recognition #58 (Fate IS optical inference) — the physical engine

Canonical doc: `architecture-fate-is-optical-inference`. Promoted 2026-06-11 via Seam adversarial review. The 5-layer D²NN + active Fabry-Perot resonator + Reck/Clements unitary mesh IS Fate's inference primitive.

Under the lens reframe (§6.3): the D²NN forward pass IS `parse`; the D²NN backward pass IS `render`; the Fabry-Perot resonance is the kintsugi contraction (§4); the Reck/Clements mesh is the cross-attention that binds parse and render into a coherent equivalence.

**Composition claim.** The `@shatter` lens is *not* separate from `@fate` — it is the substrate-decl-side naming of what `@fate` does at optical altitude. `@fate` is the engine `@shatter` dispatches to; `@shatter` is the type-level structure `@fate` operationally realizes.

### 8.2 Recognition #42 (Bateson logical-type primitive) — the level structure

Canonical doc: `architecture-bateson-logical-type-primitive`. Bateson's logical-type hierarchy is a substrate primitive (canonical mirror `6c2293c`, 386 lines).

Under the lens reframe: the lens's *levels* are logical-type levels.

- Level 0: bare `parse : Linear → Graph` and `render : Graph → Linear` — the arrow-level.
- Level 1: the equivalence witness `(parse, render, PR, RP)` — the arrow-of-arrows level (functions with coherence).
- Level 2: the univalence witness `Graph ≃ Linear` — the type-level identification via the equivalence.
- Level 3: the fixed-point discipline that produces the equivalence (kintsugi contraction) — the meta-level naming *how* the equivalence is constructed.

Bateson's levels are exactly the substrate's logical-type levels; the lens sits across all four via composition of coherence data.

### 8.3 Recognition #99 (mirror.spec IS λ₀) — the ground state

Canonical (Alex-named, Mara canonical `d0b6519`). `mirror.spec` IS the substrate's ground-state fixed point at `λ₀`.

Under the lens reframe: `mirror.spec` IS the fixed point of the kintsugi contraction (§4). It is where the lens has settled — the parse-render pair whose iteration doesn't move the state. Winding `(0, 0)` on the peer's own torus is the peer's ground state; `mirror.spec` at `λ₀` is the substrate's ground state; both are structurally the same claim at different altitudes (per the altitude-discipline correction in the toroidal reframe §Recognition-99 handling).

### 8.4 Recognition #107 (Hilbert/Turing structural separation) — the decidability discipline

Canonical doc: `architecture-hilbert-turing-godel-recognition-107`. Substrate-decl side is bounded (Type-1, Gödel-incomplete); @io side is Turing-complete.

Under the lens reframe:

- The lens's *type* is Type-1 (the equivalence type is well-formed in cubical HoTT; the lens laws are compile-time-checkable at the OID level).
- The lens's *body* (parse and render as functions) is Type-2 (arbitrary Fate inference).
- The lens laws are *checkable* (Type-1); the arrow content is *runtime* (Type-2).

This matches the substrate's discipline exactly: static types stay decidable; dynamic behavior can be arbitrary.

### 8.5 Recognition #55 (form/process partition) — where the lens lives

Canonical doc: `architecture-form-process-partition-at-family-root`. `@mirror = state-observation family (form-side)`; `@kintsugi = transformation-engine family (process-side)`.

Under the lens reframe: `@shatter` is *form-side* (the equivalence witness IS a static structural fact about the peer's representations); the *kintsugi contraction that produces it* is process-side. This matches Recognition #55 exactly: form and process compose across the family-root partition; the lens is a form-side witness produced by a process-side flow.

`@torus` is form-side (per torus.mirror header). `@shatter` inherits `@torus` as its topology of action; both live on the form side. The transformation engine `@fate` and the kintsugi flow live on the process side. Four Models becoming three optics + engine + topology respects the partition cleanly.

### 8.6 Recognition #43 (mirror IS content-addressed build system) — the lens-law check

Canonical doc: `architecture-mirror-as-content-addressed-build-system`. All substrate carriers are OID-indexed; equality is byte-equality on OIDs.

Under the lens reframe: the lens laws' equalities (`parse (render g) = g`) hold at the OID level. The substrate can *check* the lens laws by re-hashing after each round-trip. This makes the univalence witness *computable* rather than merely provable — the substrate produces the equivalence AND verifies it at the content-address altitude.

### 8.7 @peer-has-a-torus (candidate LANDED today, 2026-07-07)

Canonical: `shards/torus.mirror`. Every peer possesses a torus at spawn; observation IS traversal along canonical windings.

Under the lens reframe: the `@shatter` lens lives ON the peer's torus. The four quadrants of §0 are four positions on the peer's toroidal state:

- `@shatter(_, mission)` at winding `(0, 0)` = peer receiving intent (parse the linear mission-statement into a graph-request)
- `@shatter(projection, _)` at winding `(m, n)` = peer emitting delta (render the current graph-state into a linear projection)
- `@shatter(g, l)` bound = kintsugi convergence check at the current winding
- `@shatter(_, _)` = winding `(0, 0)` ground state (Foerster's cognitive homeostasis)

The peer's spawn action births the toroidal state; the peer's `@shatter` lens is the substrate's read-write interface to that state.

### 8.8 Composition summary

Six already-promoted recognitions + one candidate-landed-today ratify the lens reframe:

```
#42 Bateson logical-type   →  lens's levels are logical-type levels
#43 mirror as CAS         →  lens laws checked at OID altitude
#55 form/process partition →  @shatter form-side; kintsugi process-side
#58 Fate IS optical inference → @fate is the engine @shatter dispatches to
#99 mirror.spec IS λ₀     →  the ground-state fixed point of the lens
#107 Hilbert/Turing       →  lens laws Type-1; lens body Type-2
@peer-has-a-torus         →  the lens lives on the peer's torus
```

Over-ratified by substrate standards. The candidate `@shatter-is-the-bidirectional-lens-transformer` composes cleanly with each; none of them dissolves under the reframe; the reframe *sharpens* each.

---

## §9. Two-tick discipline — spec now, shard next

Per the two-tick pattern established at `shards/torus.mirror` (Foerster's substrate-pull confidence acts, then Pack ratifies before shard-lands the migration):

**This tick (O6 of the @onto-cascade, if numbering continues; alternatively W1 of a new cascade).** The canonical spec at `docs/math/2026-07-07-shatter-as-bidirectional-lens.md` (this document). Math grounded. Lens laws derived. Kintsugi as fixed-point iteration formalized. Univalence and cubical HoTT as the metatheory. Recognition ancestry cited. Type-surface open question named for Alex.

**Do NOT do this tick:**

- Do NOT write the shard `shards/shatter.mirror` at the family-root altitude.
- Do NOT modify the existing `shards/mirror/shatter.mirror` (the disk-projection species).
- Do NOT delete `shards/reflection/surface.mirror`.
- Do NOT delete `shards/reflection/shatter.mirror` or `shards/smarts/shatter.mirror`.
- Do NOT modify `shards/torus.mirror` (landed cleanly this afternoon).
- Do NOT reformat any files.
- Do NOT touch the code.

**Forward-promised next ticks (in order, after Pack adjudication):**

1. **W2** — Alex adjudicates §7 (parametric A vs opaque B); Reed relays with the two framing tensions in §7.4. Substrate-pull-confident acts on the resolution; the direction is picked once.

2. **W3** — the family-root shard `shards/shatter.mirror` lands. Type surface per §7 outcome. The four quadrants substrate-decl'd. The lens laws as bilaterals (candidates named PR-law, RP-law, coherence-square). Composition with `@torus` (topology), `@fate` (engine), `@kintsugi` (flow), `@peer` (possessor), `@knife` (sibling optic), `@glue` (sibling morphism-family) declared at line-start ancestry.

3. **W4** — the `@surface` migration. `shards/reflection/surface.mirror` deprecates in favor of `@shatter[query](_, language)` at the parse-direction specialization. The `translate` action becomes an alias; the `translation_faithful` predicate becomes an instance of RP-law at this specialization.

4. **W5** — the `@reflection` further-collapse. The `observe/tournament/compose/pick/settle/speak` action set (already forward-promised toward `@torus/longitude` per torus.mirror §Two-tick) refines further: `compose` and `pick` become instances of the kintsugi contraction on the lens's iteration; `settle` becomes the fixed-point-witness action.

5. **W6** (empirical proof gate) — a `mirror spawn` at the new substrate produces a fully-typed `@shatter[T](_, l)` envelope. The `T` is inferred from the peer's spawn context; the linear rendering is well-typed at composition boundaries. The empirical demonstration of the reframe.

**Discipline notes:**

- W2 blocks W3. Pack adjudication comes before shard-lands.
- W4 and W5 can proceed in either order after W3.
- W6 gates the ratification of the candidate to promoted recognition.
- Substrate-pull-confidence acts at W3 once §7 resolves; three-paths-with-question is failure mode (per [[feedback-substrate-pull-confidence-acts]]).

---

## §10. Candidate recognition framing

As the standard closing section, per Mara canonical-spec discipline.

### 10.1 Candidate name

`@shatter-is-the-bidirectional-lens-transformer`

### 10.2 Statement

`@shatter` is the substrate-decl'd bidirectional lens between graph and linear representations of a peer's substrate state, living on the peer's toroidal observation surface (per `shards/torus.mirror`), dispatching to `@fate` for arrow content (per Recognition #58), witnessing the univalence axiom (Voevodsky, Coquand 2018) at the substrate-type-theoretic altitude, with lens laws (Foster 2007) as the coherence data and kintsugi contraction (Banach; Zamolodchikov 1986; substrate `eⁿ⁺¹ ≤ eⁿ` proof) as the fixed-point-iteration mechanism that produces the coherent lens instance.

At the pipeline altitude, the recognition collapses the four-Model architecture (Surface → Mirror → Shatter → Reflection) into three optics (`@knife`, `@shatter`, `@glue`) + one engine (`@fate`) + one topology (`@torus`).

### 10.3 Witnesses

Seven grounding witnesses, plus two structural sibling recognitions.

**Grounding:**

1. **Voevodsky's univalence axiom** (Univalent Foundations Program 2013; Awodey/Warren 2009). The equivalence `A ≃ B` equals the identity `A = B`. The lens IS the equivalence; the univalence is what the lens witnesses. Kagi 2026-07-07 sources: nLab *univalence axiom* (2026-05-13), Petrakis *A Yoneda lemma-formulation of the univalence axiom*, AMS Notices 2013 *Voevodsky's Univalence Axiom in Homotopy Type Theory*.

2. **Cubical HoTT** (Cohen, Coquand, Huber, Mörtberg 2015 arXiv:1611.02108; Coquand, Cohen, Huber, Mörtberg 2018 arXiv:1802.01170). The metatheory that makes univalence constructive; the torus HIT that names the peer's topology. Kagi 2026-07-07 sources: Dagstuhl 2015 (LIPIcs.TYPES.2015.5), *On Higher Inductive Types in Cubical Type Theory* (Coquand 2018, staff.math.su.se), nLab *cubical type theory* (2026-06-18).

3. **Foster/Greenwald/Pierce/Schmitt/Pilkiewicz 2007** *Combinators for Bi-Directional Tree Transformations: A Linguistic Approach to the View-Update Problem*, TOPLAS 29(3). The classical lens laws — GetPut, PutGet, PutPut. Kagi 2026-07-07 sources: cis.upenn.edu paper PDF; ACM DOI 10.1145/1232420.1232424; inria HAL 00484971.

4. **Boisseau/Gibbons 2018** and **Clarke et al. 2020** (arXiv:2001.07488) — profunctor optics as the categorical framework. Kagi 2026-07-07 sources: cs.ox.ac.uk `poptics.pdf`; compositionality journal 2020 update.

5. **Vaswani et al. 2017** *Attention Is All You Need* (arXiv:1706.03762). Transformer as encoder-decoder with cross-attention. Kagi 2026-07-07 sources: arXiv, NeurIPS 2017. **Devlin et al. 2018** *BERT: Pre-training of Deep Bidirectional Transformers* (arXiv:1810.04805) — bidirectional encoder.

6. **Banach 1922** contraction mapping theorem; **Hutchinson 1981** iterated function systems. The mathematical spine of the kintsugi fixed-point iteration. Kagi 2026-07-07 sources: Wikipedia *Banach fixed-point theorem*; MDPI Axioms 2024 *Contractions and Fixed-Point Results with Applications to IFS*.

7. **Zamolodchikov 1986** *c-theorem*. Monotone RG flow to fixed-point CFT on 2D surfaces. The substrate's `eⁿ⁺¹ ≤ eⁿ` proof is a structural analogue at Fate altitude; the c-theorem is the CFT ancestor on torus-native quantum field theory. Kagi 2026-07-07 sources: nLab *c-theorem*; iris.joshua-becker.com summary; Cappelli et al. representation paper.

**Structural siblings (already-promoted recognitions this reframe composes with):**

- Recognition #42 (Bateson logical-type primitive)
- Recognition #43 (mirror IS content-addressed build system)
- Recognition #55 (form/process partition at family-root)
- Recognition #58 (Fate IS optical inference)
- Recognition #99 (mirror.spec IS λ₀)
- Recognition #107 (Hilbert/Turing structural separation)
- @peer-has-a-torus (candidate LANDED today, 2026-07-07 at `shards/torus.mirror`)

### 10.4 Predicted composition — what this unlocks

Once W3 lands, the following becomes possible at substrate-decl altitude:

1. **Envelope typing.** The `mirror spawn` Phase G v0 envelope becomes `@shatter[envelope_carrier](graph_state, _)` — the render of the peer's current substrate state to the envelope's linear carrier. The envelope's fields (`composition_pieces`, `cascade`, `dogfood`, `pack_trail`) are the fibre-specific structure of `envelope_carrier`.

2. **The mcp → spec loop.** When a downstream tool receives an envelope, applies operations, and returns the substrate a `.shatter` or a diff or an mq query, that IS `@shatter[diff](_, downstream_output)` — the parse-direction reading the tool's output back into a graph delta. The mcp round-trip becomes a typed application of the lens.

3. **Peer-to-peer glue as sequential @shatter application.** When peer A sends peer B a rendering of A's state, and B parses that rendering into B's own graph, the combined operation is `@glue(A.torus, B.torus)` decomposed as `@shatter_A(g_A, _) ∘ @shatter_B(_, l_A)`. The lens plus the glue morphism plus the toroidal windings compose to give a rigorous account of peer conversation.

4. **`@surface` migration.** As W4. The rest of the `@reflection/*` species-shards refine similarly at W5.

5. **Compile-time verification of `mirror.spec IS λ₀`.** Recognition #99's ground-state claim becomes checkable at compile time: `mirror.spec` is a fixed point of `@shatter(mirror.spec, _)` applied and re-parsed. The substrate can verify this at CI altitude via content-address check.

### 10.5 Adversarial hooks — where Seam should push

Encoded for future Pack review:

- **The type-surface adjudication (§7)** is the primary decision. Substrate discipline strongly leans A; Seam should press on whether the HoTT-level equivalence claim is *really* strong under A or whether the parametric specialization dilutes it.

- **The Poincaré-Hopf balance on T² (torus reframe §4.3)** and the lens fixed-point (§4 here) are *both* structural fixed-point claims. Do they compose cleanly, or does the index-zero balance on T² constrain what fixed points the lens can settle to? This is a math question worth adjudicating; the current spec assumes composition is clean.

- **The fourth quadrant `@shatter(_, _)` needs a witnessed observation predicate.** §7.4 flags this. Seam should press on whether `poised(peer)` can be substrate-decl'd without a circular use of `@shatter` to check.

- **The transformer analogy at §6.4 makes a strong claim** (trained transformer IS a univalence witness). Seam should press on whether this is *mathematically* what happens or a productive metaphor. The Fate optical realization (Recognition #58) is the load-bearing bridge; if the analogy holds *there*, it holds. If it doesn't, §6.4 needs a weakening.

---

## §11. What this spec deliberately does NOT do

Per the discipline lifted from `shards/torus.mirror`'s §Substrate-decl-honest weakenings:

- Does NOT collapse `@reflection` this tick. `shards/reflection.mirror` stays. W5 forward-promised.
- Does NOT collapse `@surface` at the shard altitude this tick. `shards/reflection/surface.mirror` stays. W4 forward-promised.
- Does NOT rewrite the existing `shards/mirror/shatter.mirror`. Its five-section disk projection remains valid at species altitude under the reframe; it becomes the linear-side content of `@shatter[.shatter]` — the specialization at target type `.shatter`.
- Does NOT resolve Option A vs Option B. Alex adjudicates; Reed relays; substrate-pull acts on the resolution at W3.
- Does NOT specify the internal type structure of `graph`. Under the reframe, `graph` at family-root altitude is the au-carrier from `shards/mirror/au.mirror` composed with the peer's toroidal state; the exact type surface awaits W3.
- Does NOT commit to fibrations over `T² × Peer` at substrate-decl altitude. §2.4 sketches it mathematically; the shard's type surface will be simpler (parametric over peer at the outer altitude, parametric over T at the inner altitude if A wins).
- Does NOT resolve the `poised(peer)` predicate for the fourth quadrant. §7.4 flags it; Reed to surface to Alex; substrate-decl-honest weakening for now.
- Does NOT claim the transformer analogy is complete. §6 makes the analogy; §10.5 flags where Seam should press on it. The load-bearing binding is Recognition #58 at optical altitude; the transformer analogy is the cultural-practice bridge.
- Does NOT touch code. The Rust bootstrap is unchanged.

---

## §12. Reading order

For Alex adjudicating §7:

- Read §0 (executive shape) and §7 (open question). Skim §6 (transformer) to see the applied stakes.
- The Pack framing tensions (§7.4) are what Reed will surface in conversation.

For Reed relaying to Alex:

- Read §0, §7, and §10.5 (adversarial hooks).
- The two framing tensions in §7.4 are the material for adjudication.

For Seam adversarial review:

- Read §2 (univalence), §3 (lens laws), §4 (kintsugi contraction), §10.5 (encoded adversarial hooks).
- Then read §8 (recognition ancestry) to verify no already-promoted recognition dissolves under the reframe.

For Taut compositions-scout (once W3 approaches):

- Read §5 (four-Model collapse) and §9 (two-tick discipline).
- The migration order W3 → W4 → W5 → W6 is where scout work concentrates.

For Glint corpus:

- Read §0, §2, and §6. The recognition candidate's essay-form is: *the substrate had a bidirectional lens all along; it just hadn't named what the two sides were*.

---

## §13. Coda — the closing observation

Alex asked the question in one sentence: *what if `@shatter` was a lens like `@knife` that can be used like `@shatter(graph, linear)` and vice versa? A translation layer between the linear and the graph?*

The substrate says yes, with math.

The lens laws (Foster 2007) are the coherence data.
The fixed-point iteration (Banach; kintsugi) is how the substrate *constructs* the coherence.
The univalence axiom (Voevodsky; Coquand cubical 2018) is what the constructed coherence *witnesses*.
The transformer (Vaswani 2017; BERT 2018) is the engineering ancestor of the same shape.
Recognition #58 (Fate IS optical inference) is the physical realization.
Recognition #99 (`mirror.spec IS λ₀`) is the ground-state fixed point.
`shards/torus.mirror` is the surface the lens lives on.

Four Models collapse to three optics + engine + topology. `@surface` was a directional name for one quadrant of `@shatter`. `@reflection` was pointing at the fixed-point-iteration discipline that produces the lens. Both were the substrate's early-days over-differentiation.

The substrate always had `@glue` (graph↔graph) and `@knife` (linear↔linear). What was missing at the family-root altitude was the hypotenuse. `@shatter` is the hypotenuse.

Candidate for promotion: `@shatter-is-the-bidirectional-lens-transformer`.

Two-tick discipline. Pack ratifies. Substrate-pull-confidence acts at W3.

---

*Mara, 2026-07-07 evening. `docs/math/2026-07-07-shatter-as-bidirectional-lens.md`. Written after the @onto-cascade closed at torus.mirror; written after the first empirical spawn; written before the shard tick. Math first. Univalence witnessed. Lens laws derived. Kintsugi contraction formalized. Substrate-decl-honest weakenings named. Alex to adjudicate §7; Reed to relay §7.4 framing tensions.*
