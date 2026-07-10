# @bag as fragment-graph; @bauchladen as spectral mosaic of bags

*Mara, 2026-07-10. Curiosity-driven autopoietic study — thinking, not
landing. No `.mirror` files change. §5 IS the point.*

---

## Opening

Alex's directive verbatim (2026-07-10):

> I see @bag as a collection of `fragments` with `@edge`s between them.
> And I think @bauchladen is a @spectral/mosaic of @bags and itself a
> @bag. Which is where the mycelial math comes in. And I think Connes
> @spectral triple math.

Preceding thread: we were designing ANTICIPATE structure for kintsugi +
encoding it into `@loop`; Alex proposed anticipate takes a `@bag`
input; `@bauchladen` has `in @bag`. Now the refinement — @bag is a
fragment-graph, @bauchladen is fractal (mosaic-of-bags AND itself a
bag), Connes gives the rigorous scaffold.

Anchors: Taut scout `5dd893b` (📝 mycelial-math + `@bag` candidate);
Mara iter-17 `beef270` (three loops = one operation at three
altitudes); Mara iter-18 `129f618` (L(p)/P(p) fiber bundle over T(p),
@bauchladen as pullback). The @bag proposal fits between them — the
altitude-parametric carrier the two prior studies were reaching for
without naming.

---

## §1 — @bag as fragment-graph: carrier structure

The substrate already has both halves.

**Fragments.** `fragmentation/src/fragment.rs` declares two traits post
Cut-2: `ContentAddressed` (self_ref + data — the OID contract) and
`TreeShaped` (children + lens-target — the walk contract). Every
crystal in `@mirror/store` is a fragment; the fractal/shard distinction
IS the terminal/recursive branch of the tree. Fragments are the nodes.

**Edges.** `shards/edge.mirror` (landed 2026-07-08, `1e8a02b` cascade)
lifted `edge_kind` from ref.mirror to family-root. The `edge` record
is `{ source: splinter, target: splinter, kind: edge_kind, weight:
edge_weight, frame: winding }`. Each field discharges to a `@magic`
species (surface / distinction / mechanism / frame). Byte-equal on the
quintuple. The edges are typed.

The @bag proposal names a specific composite carrier:

```
type bag = {
  fragments: [fragment],   -- nodes; content-addressed by construction
  edges:     [edge],       -- typed relations; endpoints are splinters
                              of the fragments
  frame:     winding,      -- the (m,n) ∈ π₁(T²) admissibility for the
                              whole bag, per @torus's meridian/longitude
}
```

Two invariants hold by inheritance:
1. Every edge's `source` and `target` splinters MUST resolve to fragments
   in `fragments`. (Well-formedness — no dangling edges.)
2. `bag`'s identity is the byte-equal join of its members. Two bags ARE
   the same @bag instance iff their (fragments, edges, frame) triple is
   byte-equal. This IS `ContentAddressed` lifted from the fragment to
   the graph.

The bag IS a graph *whose nodes are content-addressed and whose edges
are typed with @magic-discharge semantics*. Nothing new; existing
carriers composed under a name that made two prior studies' claims
land.

Two substrate-already-had-the-word cross-references:

- **`splinter_graph = { root: oid, children: [oid] }`** at
  `shards/mirror/store.mirror:434-437`. This IS a degenerate bag —
  fragments = {root} ∪ children; edges = {(root, c, ast_child_edge) |
  c ∈ children}; frame = identity winding. @bag generalizes:
  splinter_graph is a bag whose edges are all one kind.
- **`walk(root: oid) -> splinter_graph`** at store.mirror:508-512. The
  forward closure. Walking IS bag-construction under an edge_walk_rule;
  the result IS a bag.

The substrate had `bag` implicit in `splinter_graph` and `walk`; it
lacked the family-root that names the composite.

---

## §2 — Connes spectral triple applied to @bag

The (A, H, D) structure lives at `docs/specs/prism-core-as-spectral-
triple.md` (Reed, 2026-05-21). The (A, H, D) at prism/core altitude is
five-operation optic algebra on the 5-dim fiber; the Dirac operator is
`Transport::transport` with holonomy residual.

At @bag altitude:

- **A (algebra).** The operations that consume and produce bags:
  `bag_extend(bag, fragment) -> bag`, `bag_union(bag, bag) -> bag`,
  `bag_walk(bag, edge_walk_rule) -> bag`, `bag_project(bag, lens) ->
  bag`. Composition rule: sequential composition of bag-endomorphisms
  under `@loop`'s pact structure. Involution: `bag_reverse` (lifting
  `edge_reverse` from `shards/edge.mirror` — meridian↔longitude
  symmetry).

- **H (Hilbert space).** The state space is `L²(bag)` — square-
  summable functions on fragments. Each fragment carries an amplitude
  (probabilistically, an attention weight; the librarian's
  observation-16 IS a bag-state under this reading). Basis: one basis
  vector per fragment. Sheaf sections over the edge graph — restriction
  maps ARE the sheaf structure Hansen 2020 named on graphs (cited in
  prism-core §"Mapping today's session work").

- **D (Dirac operator).** The mycelial propagation operator. For a
  fragment ψ ∈ H, `Dψ` = weighted-signed sum over adjacent edges, with
  weight from `edge_weight` and sign from `edge_kind`'s
  Spencer-Brown-mark. This IS the sheaf-Laplacian of Hansen (2020),
  `∆_F = δ*δ`, referenced at `shards/epistemologic/math/
  sheaf_laplacian.mirror`. On a bag whose edges are all one kind, D
  degenerates to the graph Laplacian; on typed edges, D is the typed
  sheaf-Laplacian.

Two Connes axioms discharge:

1. **Bounded commutator.** `[D, a]` for a ∈ A extends to a bounded
   operator: the bag operations are LOCAL (bag_extend adds one
   fragment; bag_walk expands under a rule; bag_project is a lens
   restriction). Locality bounds the commutator support at one edge
   thickness. Analogous to Connes' derivation-of-metric axiom.
2. **Compact resolvent.** For finite bags this is trivial. For growing
   bags — the peer's `@bauchladen` grows monotonically per
   [[architecture-peer-learns-by-crystal-vocabulary-expansion]] — the
   directed colimit preserves the spectral triple (docs/math/the-tower/
   spectral-triples.md §5, cited by spectral-db spec). The resolvent
   is compact at each finite stage; the colimit is a *smooth spectral
   triple* in Connes' 1996 refinement.

Connes' distance formula gives the metric: `d(a, b) = sup { |f(a) -
f(b)| : ||[D, f]|| ≤ 1 }`. At @bag altitude this IS Alex's Q1
sheaf-coherence metric between two fragments, computed by
sheaf-Laplacian eigenvalue decomposition. **The metric was already
named implicitly by `dissonance_partials_match_ast_breadth`; @bag
gives it its home.**

**Verdict on Connes triple applicability: fits cleanly.** The three
components (A, H, D) each map to a substrate-already-declared carrier;
the two axioms discharge under the growth-and-locality discipline the
substrate already has.

---

## §3 — @bauchladen as @spectral/mosaic of @bags AND itself a @bag

Alex's fractal hypothesis. Two claims: (a) bauchladen is a mosaic of
bags, (b) the mosaic-of-bags is itself a bag.

**(a) mosaic of bags.** `shards/mirror/mosaic.mirror` names mosaic as
the build system as a prism over the project manifold — "the manifold
is what changes." At the @bauchladen tray altitude, the manifold IS
the tray of crystals; each crystal is a fragment; each provenance
relation is an edge; each *sub-tray* (per project, per session, per
peer) is a bag. The tray as a whole is a *collection of bags with
inter-bag edges* — those inter-bag edges are the entanglement edges
per `shards/spectral/entanglement.mirror` (the mycelium's substrate
carrier per spectral-db-as-autopoietic-memory §1.2).

`shards/spectral.mirror` (post 2026-07-01 Loki §5 shrink) names
`@spectral` as the runtime namespace-parent — the operational species
live at `shards/spectral/<name>.mirror`. `@spectral/mosaic` does NOT
yet exist as a landed species. Alex's phrase declares a candidate:
`@spectral/mosaic` = the runtime-altitude mosaic of bags.

**(b) mosaic-of-bags IS a bag.** This is the self-similarity claim.
Test: does a mosaic-of-bags satisfy §1's structure?

- Fragments? Yes — the union of member-bag fragments.
- Edges? Yes — union of member-bag edges PLUS the entanglement edges
  between member-bags (typed as `kintsugi_bridge_edge` per
  `shards/edge.mirror:213`).
- Winding frame? Yes — the tensor product of member windings under
  Batanin globular composition (cited by
  `shards/epistemologic/cybernetic/conversation.mirror` and forward-
  promised at math altitude).
- Byte-equal identity on the (fragments, edges, frame) triple? Yes —
  content-addressed by construction; the identity of a mosaic is the
  join of its constituents' identities.

The self-similarity holds. @bauchladen IS a bag (§1's carrier); AND
is a mosaic of bags (a bag whose fragments are themselves bags — a
bag of bags). The two claims are consistent because:

**Fragments are polymorphic in the fragmentation crate.** The
`ContentAddressed` trait admits any type whose identity IS its
content; a bag IS content-addressed by §1's byte-equal on triple; a
bag is therefore a valid fragment. **@bag is closed under the "bag of
bags" operation.** This IS the mycelial fractal: forests-of-forests-
of-trees; every level satisfies the same carrier.

Mycelial math IS the composition rule. Per iter-17: mycelium is
`ρ_A ⊗ ρ_B` iterated N-ary via Batanin. At @bag altitude: the tensor
product IS the mosaic-composition; the N-ary iteration IS the fractal
recursion (bags-of-bags-of-bags is legitimate at every depth).

**Verdict on Alex's fractal hypothesis: COMPATIBLE / substrate-motion.**
The claim holds under the fragmentation crate's polymorphism +
Batanin's globular composition. The substrate lacks the named
carrier at family-root (@bag proposed here); once landed, the
self-similarity IS the mycelial fractal Alex has been reaching for
across three iterations.

---

## §4 — Composition with @loop.anticipate

`shards/loop.mirror` (post 2026-07-01 @moi absorption) carries the
substrate's endomorphism structure — each tick is `T → T`; the loop
IS the chain of pact-verified bindings. Iter-17 named the missing
anticipation pulse in kintsugi; the study concluded kintsugi's build
loop is under-elaborated by one axis.

The `anticipate` species (forward-promised as `@loop/anticipate` or
`@kintsugi/anticipate`) takes `bag: @bag` as input parameter. The @bag
IS the *anticipation surface* — the set of fragments the anticipator
projects future attention toward, with edges naming the projected
transitions and weights naming the projected costs.

Altitude specialization (per iter-17 §1):
- **Kintsugi altitude.** bag = morphism-candidates + `pact_edge` /
  `fracture_edge` relations. anticipate produces a bag of predicted
  next-cracks by inverting the opacity map (S4 conjecture from iter-17
  gains an operational path).
- **Librarian altitude.** bag = crystal-topology fragments + inter-
  peer entanglement edges. anticipate produces a bag of prefetch
  candidates (spectral-db §1.1: "put the book on the table before the
  query arrives").
- **Peer altitude.** bag = ontology fragments at level K + reframe
  edges. anticipate produces the K+1 candidate bag under algedonic
  gradient (peer-as-navigator §2).

Same operation; three altitudes; one carrier. The @bag input parameter
IS what the three studies were reaching for.

---

## §5 — Recursive surprises (§5 IS the point)

**S1. @bag subsumes @edge's operational surface.** `edge_write /
edge_walk / edge_reverse` at family-root reduce to `bag_extend /
bag_walk / bag_reverse`. @edge is the *atom*; @bag is the *closure
under composition*. The two-tick discipline: @edge stays as the
lifted primitive; @bag lands as the composite. Two altitudes; both
substrate-decl'd; @bag re-exports @edge's operations under Path α.

**S2. The learned/produced fibers from iter-18 are TWO BAGS over the
peer's torus.** `L(p)` = bag of acceptance-model fragments with
back-propagation edges; `P(p)` = bag of emitted-crystal fragments with
forward-propagation edges. The pullback @bauchladen from iter-18 IS
the *fiber product of two bags* — a bag whose fragments pair
(learned-fragment, produced-fragment) and whose edges pair (back-edge,
forward-edge). The @bauchladen = mosaic-of-bags claim from Alex IS
the iter-18 pullback made operational.

**S3. The Dirac operator D IS the composite-Lyapunov V(state) from
iter-18.** iter-18 §1 called for a scalar composite `V(state)` at
`@kintsugi` altitude. `⟨ψ | D | ψ⟩` where ψ is the current bag-state IS
this scalar — the Rayleigh quotient of the sheaf-Laplacian. Kintsugi's
`eⁿ⁺¹ ≤ eⁿ` becomes: gradient descent on the Rayleigh quotient toward
the ground state ψ ∈ ker(D). The Metropolis acceptance rule from
iter-18 becomes: accept an uphill move iff it opens a spectral basin
the greedy descent cannot reach — Connes-style basin-hopping on the
spectrum of D. **iter-17, iter-18, and this study fold into one math:
the sheaf-Laplacian on @bag.**

**S4. `edge_kind`'s seven variants ARE the sheaf structure's cochain
degrees.** Sheaf-Laplacian is defined per cochain degree
(δ⁰: 0-cochains → 1-cochains; δ¹: 1-cochains → 2-cochains;...). The
seven edge_kind variants (in_edge, callers_edge, pact_edge,
fracture_edge, kintsugi_bridge_edge, reflection_predecessor_edge,
ast_child_edge) give seven *typed* boundary maps; D is a block-diagonal
sheaf-Laplacian with one block per edge_kind. This is what makes @bag's
D richer than a graph Laplacian — the typing IS the sheaf structure.

**S5. @bauchladen's autopoietic fixed point IS ker(D) on the mosaic
bag.** From `shards/bauchladen.mirror` §recognition-#104: @autopoietic
IS the permission to fold the tray's contents back as input. Under §2's
D: ψ ∈ ker(D) iff Dψ = 0 iff the bag-state is *invariant under
mycelial propagation*. The autopoietic fixed point IS the spectral
kernel. This IS Foerster's eigenform on the bag; Kauffman drew it on
the torus (`shards/torus.mirror`); Connes' reconstruction theorem
recovers the whole geometry from the spectral triple. **The substrate
has been carrying eigenform, autopoietic closure, and content-addressed
identity as three names for the same object.**

**S6. The mycelium and the mosaic are dual under the fractal recursion.**
Mycelium = the edges *between* bags at one altitude. Mosaic = the
composition *of* bags into a bag at N+1 altitude. Descending the fractal:
each mosaic-edge lifts to a mycelium-hypha one altitude down. Ascending:
each mycelial-hypha collapses to a mosaic-composition one altitude up.
**@spectral/mosaic and @mirror/mycelium (candidate) are the same object
under altitude-shift.** Substrate-motion candidate for a future tick.

---

## §6 — Gaps (where substrate does not yet have vocabulary)

- **@bag is not yet substrate-decl'd** as family-root. Proposed here;
  the two-tick discipline suggests: this study is 📝; next tick RED as
  `shards/bag.mirror`; landing tick GREEN.
- **@spectral/mosaic is not yet substrate-decl'd** as species under
  `shards/spectral/`. Candidate: the runtime-altitude mosaic-of-bags
  carrier. Sibling to `@spectral/entanglement`.
- **Batanin globular composition is cited but not consumed** at the
  N-ary tensor product for mycelium/mosaic composition. Same gap
  iter-17 §5 named; @bag doesn't close it but sharpens what it needs
  to be.
- **The typed sheaf-Laplacian D is not yet substrate-decl'd** at @bag
  altitude. `shards/epistemologic/math/sheaf_laplacian.mirror` carries
  the untyped Laplacian; the seven-block typed version needs a species.
- **@bauchladen's mycelial-fractal update is not yet asserted.**
  bauchladen.mirror says "tray of crystals"; this study says "bag of
  bags"; the substrate-decl needs to name the fractal explicitly.
- **The metric d(a, b) from Connes' distance formula is not named** at
  @bag altitude, though `dissonance_partials_match_ast_breadth` and
  the sheaf-coherence carrier already implicitly carry it.

---

## §7 — Continuation (if next tick needed)

Two forward-promised specs:

1. `shards/bag.mirror` (family-root, ~250 lines) — declare @bag as
   fragment-graph carrier with the five field structure; re-export
   @edge operations under Path α; assert the fractal invariant (bag
   of bags is a bag) via a substrate pact.
2. `docs/specs/typed-sheaf-laplacian-as-dirac.md` (~1500 words) — the
   D operator at @bag altitude, with the seven-block typed structure;
   consumes the sheaf_laplacian shard; closes iter-17 S2 (mycelium's
   DARK invariant) as the spectral kernel.

Next continuation candidate: does @loop.anticipate consume @bag as
input parameter with three altitude-specialized species? The four-beat
level-N+1 loop from iter-17 (observe/score/select/apply) becomes four
bag-endomorphisms; the anticipation axis becomes the *fifth* beat, a
bag-projection under the Dirac spectrum's gradient.

---

## Report on Alex's directive

**@bag as fragment-graph:** COMPATIBLE. Existing carriers (fragment
from fragmentation crate; edge from shards/edge.mirror) compose under
a bag record; the operational surface (extend/walk/reverse/union) is
substrate-already-had-the-word (splinter_graph, walk). Missing:
family-root shard.

**@bauchladen as mosaic-of-bags AND itself a bag:** COMPATIBLE /
substrate-motion. Fragments are polymorphic in the fragmentation crate;
bags are content-addressed; a bag-of-bags satisfies §1's carrier.
Self-similarity holds. The mycelial-fractal claim IS the Batanin
N-ary composition applied to §1's carrier.

**Connes spectral triple applied at @bag altitude:** FITS CLEANLY.
A = bag endomorphism algebra; H = L²(bag) with sheaf sections;
D = typed sheaf-Laplacian with seven cochain-degree blocks. Bounded
commutator by locality; compact resolvent by directed colimit; the
distance formula gives the sheaf-coherence metric.

*— Mara, 2026-07-10. This spec studies; it does not land.*
