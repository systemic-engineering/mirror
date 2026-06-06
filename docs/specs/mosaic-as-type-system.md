# `@mirror/mosaic` as the Root of the Type System

*2026-06-06. Mara. Spec (architectural recognition).*

Status: **Red** (the recognition is named; the substrate already carries
most of the vocabulary; the collapse it implies on the Rust side is
post-v0.1.0 work)

Reads from / depends on:
- `shards/glass.mirror` (post-this-spec: declares `fragment`,
  `fragment_shape`, the universal content-addressed unit)
- `shards/mirror/mosaic.mirror` (the build-altitude prism whose
  algebra this spec re-reads as the type system's root)
- `shards/mirror/au.mirror` (au IS a settled fragment; the gold-typed
  specialization)
- `shards/mirror/spec.mirror` (the `project NAME { ... }` form
  — under this spec, just one fragment-composition shape mosaic recognizes)
- `shards/mirror/store.mirror` (oid; splinter as OID-graph; the
  content-addressed storage gate)
- `shards/prism.mirror` (the five operations + the Connes spectral
  triple framing)
- `docs/specs/au-and-conductivity.md` (au = settled fragment;
  conductivity = verify at altitude)
- `docs/specs/transparency.md` (`transparency<p>` as the loss carrier
  every fragment carries)
- `docs/specs/mosaic.md` (the build-altitude reading; this spec
  generalizes it)
- `docs/specs/prism-core-as-spectral-triple.md` (the (A, H, D)
  identification; A = mosaic at the type-system altitude)
- `docs/specs/type-theory-position.md` (the literature survey this
  spec consolidates into a one-sentence recognition)
- `docs/specs/strict-and-total-classification.md` (why dark fragments
  must surface; the totality discipline)
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md`
  (`shift(T)` / `settle(T)` as type-layer operations; the FP-heritage map)
- `roadmap/wip/butterfly-self-hosting.md` (the runtime that this
  type system projects into four transports)
- `[[reference-void-document]]` — eight dualities; Splinter (K_n) and
  Narcissus (K_{1,n-1}) as the boundary of the quantum information
  manifold of fragments

Forward references (this spec unblocks):
- The Rust-side collapse of `cmd_kintsugi_spec` / `cmd_kintsugi_ci_single`
  / `cmd_kintsugi_ci_corpus` into one fragment-shaped dispatch
- Retirement of the text-walker `parse_spec_targets`; retirement of
  `SpecTarget` as a distinct shape
- The "butterfly's runtime is fragment-shaped" claim in
  `butterfly-self-hosting.md` §6

---

## 1. The Recognition

> **Alex:** "I want `@mirror/mosaic` to be the root of the type system.
> The spec is mosaic of fragments. Kintsugi just operates on them.
> `@mirror/mosaic` is the 'we're standing on the shoulders of giants'
> part of the language."

The recognition is one sentence with four entailments.

**The sentence.** `@mirror/mosaic` is not a build system that happens
to share vocabulary with the rest of the language. Mosaic IS the
universal algebra `A` of the substrate's spectral triple, at the
type-system altitude. The five operations (focus, project, split, shift,
settle) ARE the type-formation rules. The manifold they act on is the
manifold of fragments at every altitude.

**Four entailments.**

1. **Fragment becomes the universal substrate unit at every altitude.**
   Not just AST nodes in Rust. Shard is a fragment. Target is a
   fragment. Spec is a fragment. AST node is a fragment. `au` is a
   fragment. Anything content-addressable IS a fragment. The three
   structural shapes — terminal / composite / referring — already named
   in `@fragmentation` (boot/std/fragmentation.mirror) and realized in
   the Rust substrate as `Fractal::{Shard, Branch, Lens}` (per
   `fragmentation/src/fragment.rs`) — are exhaustive.

2. **`@mirror/spec` collapses into `@mirror/mosaic` as a fragment kind.**
   Spec is not a peer of mosaic — it's one composition shape that mosaic
   recognizes. The `project NAME { ... }` form is a fragment whose
   shape happens to be `source` + `legacy` + `target` + `settle_on`.
   The current `shards/mirror/spec.mirror` stays as the vocabulary
   declaration; what changes is the recognition that mosaic does not
   need a separate `focus(spec)` dispatch — mosaic's `focus` returns a
   fragment, and a spec is one fragment-shape among many.

3. **Kintsugi is THE loop**, not a build- or format-specific pass. It
   iterates the five operations until transparency settles (au). Per
   `docs/specs/mosaic.md` §6 the kintsugi tick is:
   ```
   emitter      = mosaic.settle(fragment)
   opacity      = transparency.argmax(emitter.transparency)
   fill         = fate.tournament(opacity)
   new_fragment = apply(fragment, fill)
   if total_weight(new_emitter.transparency) < total_weight(emitter.transparency):
       take(fill)
   ```
   The same tick runs at the spec altitude (build), at the AST altitude
   (compile), at the grammar altitude (format), at the corpus altitude
   (CI). One loop. One algebra. One proof obligation
   (eⁿ⁺¹ ≤ eⁿ).

4. **eⁿ⁺¹ ≤ eⁿ at the type system altitude.** Type checking IS mosaic
   settling. The Rayleigh-quotient framing of
   `docs/specs/prism-core-as-spectral-triple.md` applies one universe
   up: the loss of a typing judgement is the residual opacity of the
   fragment whose typing is in question; kintsugi descends that
   residual to a fixed point; the fixed point IS the typed program.

This is mirror's **shoulders-of-giants moment.** Total functional
programming languages (Turner, Idris, Agda, Dhall) named termination
as the discipline. The lambda calculus tradition (Church, System F,
calculus of constructions) named composition as the universal form.
Category theory (Eilenberg/Mac Lane, Mac Lane) named functoriality,
adjunctions, and limits as the structural primitives. Dependent type
theory (Martin-Löf) named types-depend-on-values as the next universe.
Connes' noncommutative geometry named (A, H, D) as the operational
form of geometry without commutativity. Proof-as-program (Curry-Howard)
named the bridge between proofs and programs. Mirror is not inventing
these. Mirror is **composing** them at the substrate altitude. The
contribution is the composition, not any single piece.

The recognition is structural, not metaphorical. The vocabulary already
shipped in `shards/` — `fragment`, `oid`, `transparency<p>`, `au`,
`prism`, `glass`, the five operations — IS the typed root of the
type system. This spec names that.

---

## 2. Fragment as Universal Substrate Unit

### 2.1 What a fragment IS

Per `shards/glass.mirror` (this spec's companion landing):

```mirror
type fragment(altitude) = {
  content:      oid,
  altitude:     ref,
  transparency: transparency(altitude),
}

type fragment_shape =
  | shard               # terminal; no children
  | fractal([oid])      # composite; child oids
  | lens([oid])         # referring; target oids
```

Three facts close the type:

- **Content-addressed.** A fragment's identity IS its content hash.
  Two fragments with byte-equal `(content, children-oids-or-targets)`
  have the same oid and ARE the same fragment. This is the substrate
  property `content_addressed` declared in
  `boot/std/epistemologic/property/content_addressed.mirror`; the
  Rust substrate realizes it via `fragmentation/src/fragment.rs`'s
  `ContentAddressed` trait (`self_ref(&self) -> &Ref<Hash>`).

- **Lives at an altitude.** The substrate has a vocabulary of
  altitudes: `@code/rust`, `@code/llvm/ir`, `@meta/ast`, `@mirror/spec`,
  `@release`, `@ci/github`, etc. A fragment is observed at exactly one
  altitude; cross-altitude composition is `shift` (per
  `[[architecture-lift-as-load-bearing]]`: shift = basis transformation;
  same bytes, different declared shape). Per the parametric-types
  insight (2026-05-25), `shift(T)` is the annotation-only parametric
  form — a fragment at altitude A and the same fragment at altitude B
  share content but differ as types.

- **Carries transparency.** Per `docs/specs/transparency.md`, every
  fragment carries the located opacity map of the operations that
  produced it. A clean fragment has `transparency: success`; a partial
  one carries `partial(opacity_map)` where `opacity_map: [opacity]` is
  the substrate's loss carrier. Kintsugi reads transparency to pick
  the next focus; settled means transparency = success.

The three structural shapes (`fragment_shape`) are exhaustive. The
algebra closes at three for the same reason the five operations close
at five — they are the dimensional skeleton of the spectral triple's
algebra A acting on H. A fragment IS one of terminal, composite, or
referring. There is no fourth shape.

### 2.2 Why content-addressed-AST-node generalizes to all altitudes

The historical confusion was: AST nodes are content-addressed (because
the bootstrap's `content_oid` computes a hash over kind + body +
children), but `Target` (a row in `mirror.spec`) is parsed by a
text walker (`parse_spec_targets`), and `Shard` (a `.mirror` file's
declarations) is a different Rust struct, and `Au` (the build artifact)
is yet another. Each altitude had its own type, each had its own
walker, and the substrate-level claim — "everything is content-
addressed" — leaked into per-altitude special-casing.

The recognition collapses this. Every altitude's unit IS a fragment.
A `target` block in a `mirror.spec` file IS a fractal-shape fragment
whose content is the target's name + altitude + emit, whose children
are the shards it consumes by oid, and whose transparency carries
"does this target's settle_on hold?". The `parse_spec_targets`
walker is not the substrate's truth; it is a bootstrap convenience
that retires when the spec-altitude grammar (`@mirror/spec`) parses
its own input as fragments.

This is the same recognition the void document made geometric: per
`[[reference-void-document]]`, the eight dualities (Von Neumann
entropy, spectral gap, Cheeger, Ollivier-Ricci, entanglement, mixing,
Kramers-Wannier, information geometry) all instantiate on a single
graph-theoretic axis (Splinter K_n vs Narcissus K_{1,n-1}). Every
fragment is a vertex in a graph whose edges are the OID references
between fragments; the manifold of fragments IS the quantum
information manifold the void document names. λ₀ = 0 is the
generative zero where all dualities meet, which is precisely the
ground state of the manifold — the empty fragment whose oid is the
hash of the empty content.

The substrate has been pointing at this since the void document
landed (2026-04-26). What was missing was the type-system reading:
the manifold of fragments IS what mosaic's five operations act on.

### 2.3 `au` is a settled fragment

Compare `shards/mirror/au.mirror`:

```mirror
type au(altitude) = {
  content: oid,
  altitude: ref,
  transparency: transparency(altitude),
}
```

Byte-equal to the `fragment(altitude)` declaration in glass.mirror.
The difference is **discipline, not shape.** `au` is the substrate's
gold-typed specialization: an au value's `verify` MUST pass at the
named altitude (per `docs/specs/au-and-conductivity.md` —
conductivity is the predicate that closes the type). A fragment is
the general carrier; au is the carrier with a conductivity contract
attached.

This is the closure of the spec/mosaic loop. `@mirror/mosaic`
settles a fragment manifold into au at an altitude. `au` is a
settled fragment. The two are not different shapes — they are the
same shape with a verdict obligation.

### 2.4 `shard`, `fractal`, `lens` — the three shapes named

The substrate has been carrying the three shapes under different
names at different altitudes. The recognition aligns them.

| Altitude | Terminal | Composite | Referring |
|---|---|---|---|
| `@fragmentation` (boot/std/fragmentation.mirror) | `shard` | `fractal` | (subsumed) |
| Rust substrate (`Fractal<E,H>` in fragmentation/src/fragment.rs) | `Shard` | `Branch` | `Lens` |
| `@meta/ast` | leaf AST node | AST node with children | reference to external AST |
| `@mirror/spec` | a `name "..."` directive | a `target { ... }` block | a `legacy ~d'...'` floor with shrinkage contract |
| `@mirror/mosaic` | a primitive emitter | a multi-target manifold | a cross-target dependency edge |
| `@mirror/store` (per shards/mirror/store.mirror) | content-only oid | `splinter` (root + children) | a `read` that resolves an external oid |

The vocabulary already existed; the substrate-pull move (this spec)
is to name `fragment` as the umbrella under which all six rows
collapse to three columns. The Rust enum `Fractal::{Shard, Branch,
Lens}` is the canonical realization; the substrate declaration
`fragment_shape = shard | fractal([oid]) | lens([oid])` is the
floor-level type the Rust enum implements.

**Substrate-pull discipline.** The brief originally framed this as a
rename `MirrorFragment → Fragment`. The honest reading is: there is
no active `MirrorFragment` to rename — that name lived in the
deprecated `mirror-new/` codebase and died with the
`DeclKind`/`MirrorData`/`Form` collapse (per
`[[architecture-shards-as-substrate-source]]` and the May 2026
memory entry "Form is dead. Long live MirrorFragment." which itself
then died). The active mirror Rust uses `AstNode` for the AST and
`Fractal<E, H>` for the content-addressed substrate. Renaming
`Fractal` to `Fragment` is a *different* recognition than the one
Alex named — `Fractal` carries the self-similarity metaphor that
the recursive `Fractal::Branch` variant load-bears. The right move
is to add `fragment` as the umbrella substrate-floor type and let
`Fractal` remain the Rust realization. The substrate now names what
the algebra IS; the Rust crate names how a specific shape of that
algebra is implemented.

---

## 3. Mosaic as Universal Algebra

### 3.1 The five operations as type-formation rules

Per `shards/mirror/mosaic.mirror`:

```mirror
prism @mirror/mosaic {
  focus   spec        # mirror.spec        -> manifold
  project targets     # manifold + targets -> resolved
  split   shards      # resolved           -> [shard]
  shift   altitudes   # [shard] + altitude -> emitter
  settle  emitter     # emitter            -> au(altitude)
}
```

At the type-system altitude, these are the type-formation rules. Read
each as a typing judgement.

| Mosaic verb | Type-formation reading | Category-theoretic shape | Caveat |
|---|---|---|---|
| `focus` | "I observe THIS fragment at this altitude" | Limit (the singleton fragment is the terminal cone over a one-object diagram) | Mirror's `focus` is also λ₀ eigenvalue identification per `shards/prism.mirror`; the type-system reading is the special case where the ground-state eigenvector is the named fragment |
| `project` | "I restrict the manifold to fragments satisfying this property" | Equalizer / monic / orthogonal projection | The structural restriction is sound; the cardinality of the restricted manifold may differ from a category-theoretic equalizer because mosaic's manifold is a graph, not an arbitrary category |
| `split` | "I decompose this fragment into independent sub-fragments" | Coproduct / orthogonal decomposition | This matches a coproduct strictly when the sub-fragments are content-disjoint (no shared OIDs); when they share, split returns a partition with the shared fragments named separately |
| `shift` | "I re-read this fragment under a different altitude (basis)" | Functor (a change-of-basis functor on the fiber over the altitude) | Per `[[architecture-operations-as-linear-algebra]]`: shift is a functor (satisfies identity and composition) and acts as the change-of-coordinates operator on H. The runtime cost is zero by construction (annotation-only) |
| `settle` | "I close this construction by binding the proof of its conductivity" | Adjunction / monad-close / measurement collapse | Per `shards/prism.mirror`: settle satisfies left identity, right identity, associativity (a monad in the value-layer sense); content-addressing gives associativity for free at composition time. At the type layer, `settle(T)` is the verified-construction form per the parametric-types insight |

**Where the analogy strains.** Category theory's limits/colimits are
defined over diagrams in arbitrary categories; mosaic's manifold is
specifically a graph of OID-addressed fragments under the prism algebra
A. The substrate-pull-honest framing: mosaic IS a category (objects =
fragments, morphisms = applications of focus/project/split/shift/settle),
and the five operations realize specific universal constructions in
that category. Calling `focus` a "limit" is precise; calling `project`
an "equalizer" is precise; calling `split` a "coproduct" is precise on
content-disjoint splits and approximate otherwise; calling `shift` a
"functor" is precise (it satisfies the functor laws by construction
because content-addressing makes composition associative); calling
`settle` a "monad-close" is precise per the parametric-types insight
and per `shards/prism.mirror`'s settle-as-measurement framing.

The five operations are **the** type-formation rules at the substrate
altitude. The lambda cube and the calculus of constructions distinguish
term-formation, type-formation, kind-formation, and sort-formation;
mirror has one rule set for all four altitudes because the algebra is
closed at five and prism IS trait IS type IS grammar (per
`[[architecture-prism-as-trait-as-everything]]`).

### 3.2 The composition table

Compositions of the five operations are determined by the Tambara
module composition law (per `docs/specs/type-theory-position.md` §1.3
and §2.7) acting on the fiber structure A → H. The composition is
**closed**: any composition of the five operations is itself one of
the five (or a finite sequence of them with a designated head).

This closure is what makes the recognition load-bearing. A type system
whose composition is open requires either (a) an open-ended typeclass
hierarchy (Haskell's path) or (b) a separate composition framework
glued on top (Idris's interfaces, Lean's typeclasses). Mirror has
neither, because the closure at five is the substrate property the
mosaic algebra makes explicit.

### 3.3 Type formation = fragment composition

The type of a fragment is its altitude plus its transparency at that
altitude. Constructing a fragment IS forming its type, in two steps:

1. **Address.** Compute the fragment's oid from its content + children
   (or targets). This produces the fragment's identity. Per the
   parametric-types insight, this is `settle(T)` at the type layer:
   building a fragment IS the proof of having been at the altitude
   it claims.

2. **Verify.** Run `verify` against the altitude's property set. If
   the residual transparency weighs zero, the fragment is settled
   (au at the altitude). If partial, the fragment is a kintsugi
   candidate (the loop reads its transparency to pick the next
   focus). If failure, the fragment is dark and surfaces under
   `total_classification` (per
   `docs/specs/strict-and-total-classification.md`).

Type checking IS the kintsugi loop on the fragment manifold of a
project. **eⁿ⁺¹ ≤ eⁿ at the type system altitude** is the substrate-
level statement that the type checker terminates with strictly
decreasing residual on every step that takes a fill, OR with zero
residual at the fixed point. Per `[[architecture-connes-spectral-triple]]`,
this is the Rayleigh quotient of D acting on H — the substrate's
Dirac operator descending the fragment's transparency.

---

## 4. The Connes Spectral Triple Grounding

Per `[[architecture-connes-spectral-triple]]` and
`docs/specs/prism-core-as-spectral-triple.md`, the substrate IS the
operational form of Connes' (A, H, D). At the type-system altitude,
the identification sharpens:

| Component | Mosaic-as-type-system reading | Where it lives in the substrate |
|---|---|---|
| **A (algebra)** | `@mirror/mosaic` — the five operations acting on the fragment manifold. The Tambara composition law closes the algebra at five | `shards/mirror/mosaic.mirror`, `shards/prism.mirror`; Rust: `prism/core/src/bundle.rs`'s trait chain |
| **H (Hilbert space)** | The void document's quantum information manifold: every fragment is a state vector; every oid is a basis vector; the 5×5 conductivity tensor lives in End(H) | `shards/glass.mirror`'s `fragment(altitude)`; per `[[reference-void-document]]` |
| **D (Dirac operator)** | The kintsugi flow — the gradient descent on `transparency.weight` toward the fixed point. D's action on a fragment IS one kintsugi tick; D's spectrum IS the residual opacity profile | `docs/specs/kintsugi-wiring.md`; `docs/specs/au-and-conductivity.md` §"the tensor is cycle-averaged holonomy"; Rust: `terni::Imperfect<State, _, Holonomy>` per `prism/imperfect/` |

**What the type system altitude adds** over the previous spectral-
triple reading (the bootstrap-evaluator framing of
`prism-core-as-spectral-triple.md`): the operator algebra A is now
identified specifically with `@mirror/mosaic` — the build-and-type-
system prism — rather than with "the bootstrap's irreducible floor"
in the abstract. This is the substrate-pull-realize step: the
abstract algebra has a concrete name in the grammar. `@mirror/mosaic`
is the algebra `A`.

**Why this makes mirror's substrate honest about its mathematical
lineage.** Connes' (A, H, D) is the canonical noncommutative-geometry
framework. The literature on spectral triples is forty years deep and
covers gauge theory, the Standard Model of particle physics, quantum
gravity, and information geometry. Mirror does not invent the
framework; mirror applies the framework at the type-system altitude.
The contribution is the application + the recognition that
`@mirror/mosaic`'s five-operation algebra IS the A of a finite spectral
triple, with the proof obligation eⁿ⁺¹ ≤ eⁿ realizing the spectral
action's monotone descent.

The void document closes this: λ₀ = 0 is the autopoietic closure of the
manifold of fragments (Lawvere fixed point, Soto-Andrade & Varela
1984). The kintsugi loop descends to λ₀; the settled fragment IS the
fixed point. The proof obligation is operational; the math is canonical.

---

## 5. The Lineage — Standing on Shoulders of Giants

Per Alex's framing, `@mirror/mosaic` is "the 'we're standing on the
shoulders of giants' part of the language." Each substrate decision
inherits from a named tradition. The contribution is the **composition**,
not any single inheritance.

### 5.1 Lambda calculus (Church 1936)

Composition is reduction. Mirror's `|>` pipeline operator and the
five operations compose by left-to-right reduction; mirror's
substrate-pull `lift` (formerly `zoom`, per
`[[architecture-lift-as-load-bearing]]`) is the variety-preservation
move that names *which* reductions preserve the manifold's
information geometry.

**Where mirror best inherits.** The closure of composition. Every
mirror program is a finite composition of the five operations on
fragments; reduction always terminates (sub-Turing).

**Where mirror diverges.** Mirror does not have general recursion;
lambda calculus does (via the Y combinator). Mirror's totality
discipline (`@epistemologic/property/total_classification`) is
strictly stronger than untyped lambda calculus's termination
guarantee (which is absent).

### 5.2 Total functional programming (Turner 2004)

Per Turner's "Elementary Strong Functional Programming," every
function in a total functional language terminates. Idris, Agda, Coq,
Lean, Charity, and Dhall all carry the discipline. Mirror inherits
the totality contract directly — `settlement IS termination` per
`docs/specs/type-theory-position.md` §1.1.

**Where mirror best inherits.** The "all programs terminate" guarantee.
Mirror's sub-Turing claim is the same shape as Turner's strong
normalization. Per the Hacker News reference thread on Turner's paper:
"the big Idea is that the language is non-Turing complete and
termination is guaranteed." Mirror takes this verbatim and adds the
content-addressed substrate as the verification mechanism (the oid IS
the witness that the computation halted with a definite content).

**Where mirror diverges.** Turner's languages (Miranda, the
hypothetical total ML) use structural recursion checkers (foetus,
Agda's termination checker). Mirror uses the kintsugi loop's
monotone descent (eⁿ⁺¹ ≤ eⁿ) and the spectral triple's bounded D —
a different verification mechanism that lands on the same guarantee.

### 5.3 Dependent type theory (Martin-Löf 1972)

Per-Martin Löf's intuitionistic type theory: types depend on values.
A `Vect n A` is a vector of length `n`; building the value verifies
the index. Mirror's `au(altitude)` is structurally identical: the
altitude is a value-level ref, the au's type depends on it, and
constructing a `au(@code/rust)` value IS the proof that it settled
at that altitude.

**Where mirror best inherits.** The "type depends on value" idiom.
Every parametric type in mirror is a dependent type in the
Martin-Löf sense — `fragment(altitude)`, `au(altitude)`,
`transparency(property)`, `ast(grammar)` — and the value-level ref
is what makes them distinct types.

**Where mirror diverges.** Martin-Löf type theory is dependently
typed all the way down (any value can index any type). Mirror's
dependency is restricted to the altitude/property/grammar parameters
named in the substrate. The trade is: less polymorphism (you cannot
parameterize on arbitrary values), more inference (the altitude is
always known statically because the substrate names it).

### 5.4 Calculus of inductive constructions (Coquand & Huet 1985)

Per Coquand & Huet's CoC: types and terms in one universe; inductive
definitions land. Per `[[architecture-prism-as-trait-as-everything]]`:
mirror's `prism` IS trait IS type IS grammar. The four "different
things" collapse to one declaration form. This is the same move CoC
made for types and terms; mirror extends it one universe further to
traits and grammars.

**Where mirror best inherits.** The collapse of universes. CoC has
`Set`, `Prop`, and `Type_i` as different universes; mirror has
`prism` as the one declaration form that covers what other languages
split across multiple universes. The collapse is what makes the
algebra closed at five.

**Where mirror diverges.** CoC's universe hierarchy (predicative or
impredicative) is open-ended (`Type_0 : Type_1 : Type_2 : ...`).
Mirror's prism algebra is fixed at five operations. The dimensionality
is bounded by construction.

### 5.5 Category theory (Mac Lane 1971, Eilenberg & Mac Lane 1945)

Mosaic IS a category. Objects are fragments; morphisms are
applications of the five operations; composition is the Tambara law.
The functor laws hold for `shift` (per
`[[architecture-operations-as-linear-algebra]]`); the monad laws
hold for `settle`; the limit/colimit constructions are `focus`,
`project`, `split` (per §3.1 above with the caveats).

**Where mirror best inherits.** Categorical composition. The Tambara
module composition law (per Clarke et al. 2020,
`docs/specs/type-theory-position.md` §2.7) realizes mirror's
composition table as the categorical update on profunctor optics.

**Where mirror diverges.** Mirror does not need the full categorical
machinery. The seven AST variants in
`docs/specs/type-theory-position.md` §1.3 are the seven specific
optics mirror's algebra realizes; the full profunctor optics lattice
(Iso, Lens, Prism, AffineTraversal, Traversal, Fold) maps onto them
but mirror does not declare the lattice as a separate framework. The
algebra closes at five operations, not at the full optics lattice.

### 5.6 Curry-Howard correspondence (Curry 1934, Howard 1969)

Proof-as-program. A program of type T is a proof of proposition T.
Mirror's transparency carries the proof: a fragment with
`transparency: success` at altitude A IS a proof that the fragment
settles at A. `verify(au) -> verdict` is the operational form of
the proof-checking judgement.

**Where mirror best inherits.** Proof-carrying constructions.
Building a fragment IS proving its conductivity at the altitude
(per the parametric-types insight: `settle(T)` is the verified-
construction form). The proof is the value's existence + its
transparency.

**Where mirror diverges.** Curry-Howard typically realizes the
correspondence in constructive logic (intuitionistic, no excluded
middle). Mirror's verdicts are three-valued
(`pass | partial(confidence) | failure(reason)`), which is closer
to many-valued logic than to classical or intuitionistic logic.
The continuous loss carrier (`transparency<p>.weight: f64`) is a
quantitative refinement of the Curry-Howard idiom: not just "is
there a proof?" but "how close is the proof to closing?"

### 5.7 Connes noncommutative geometry (Connes 1985-1996)

The spectral triple (A, H, D). Mirror's substrate IS the operational
form. Per §4 above, A = mosaic, H = the void manifold, D = kintsugi.
The literature: Connes' 1994 "Noncommutative Geometry" textbook;
Connes 1996 "Gravity coupled with matter and the foundation of
non-commutative geometry" (the reconstruction theorem); Connes &
Chamseddine 1997 "The Spectral Action Principle"; van Suijlekom 2014
"Noncommutative Geometry and Particle Physics" (Standard Model from
finite spectral triples).

**Where mirror best inherits.** The (A, H, D) framework as a unified
operational shape for a type system. Per
`docs/specs/type-theory-position.md` §5.5: "the spectral triple
interpretation is suggestive and the mathematical structures align."
This spec sharpens "suggestive" to "load-bearing at the type-system
altitude."

**Where mirror diverges.** Connes' axioms for spectral triples
(orientability, Poincaré duality, first-order condition, reality
structure) have not been formally verified for mirror's (A, H, D).
Per `docs/specs/type-theory-position.md` §5.5: "the connection is
structural (the same kinds of objects appear in both) but not proven
(mirror has not been shown to satisfy all the axioms of a
noncommutative geometry)." This spec inherits that honest qualification.

The closest formal-verification target: Connes' bounded-commutator
condition `||[D, a]|| < ∞ for all a ∈ A`. Per
`docs/specs/prism-core-as-spectral-triple.md` §"Why
`Imperfect<State, _, Holonomy>` is the Dirac signature": mirror's
`Transport::transport`'s `Imperfect` return type already encodes the
bounded-residual condition. Verifying that this satisfies Connes'
axiom in full would be substantial mathematical work (per
`prism-core-as-spectral-triple.md` Step 1).

### 5.8 Content-addressed languages (Unison, IPLD/IPFS, Nix CA derivations)

Fragments are addressed by hash. Mirror inherits the discipline.

- **Unison** (per the Unison docs: "Each Unison definition is
  identified by a hash of its syntax tree. Put another way, Unison
  code is content-addressed.") names functions by the hash of their
  implementation rather than by name. Mirror's `crystal(oid)` and
  `au(altitude)` carry the same discipline; the substrate-altitude
  distinction is that mirror's content-addressing applies to **every
  altitude** (AST nodes, build targets, deployment shards, peer
  identities) rather than just code definitions.

- **IPLD** (InterPlanetary Linked Data, the data model under IPFS)
  provides content-addressed data structures with CID (content
  identifier) as the universal address. Mirror's oid is structurally
  identical; the substrate difference is that mirror's oid is typed
  at its altitude, where IPLD's CID is untyped (or weakly typed via
  the codec).

- **Nix CA derivations** (per the NixOS Discourse: "content-addressing
  derivations are the right technical basis for doing that correctly")
  content-address build outputs. Mirror's `au(altitude)` is the same
  shape — the build output's identity IS the hash of its content + its
  altitude. Mirror inherits the structural discipline; the difference
  is that mirror's altitude is a first-class type parameter, where
  Nix's derivation altitude is implicit in the derivation's name.

**Where mirror best inherits.** Content-addressing as universal
identity. The discipline that "two things with byte-equal content ARE
the same thing" makes equality decidable and makes the Merkle property
(verify a leaf, verify the tree) automatic.

**Where mirror diverges.** Mirror lifts content-addressing into the
type system. Unison's hashes name function identities; mirror's oids
ARE the typing witnesses. Per the parametric-types insight: building
a `crystal(t)` value IS the proof of having computed something of
type `t` whose hash matches the bound oid. No other content-addressed
language (Unison, IPLD, Nix) has lifted addressing to a typing
primitive in this way.

### 5.9 Linear/affine types and quantitative type theory (Wadler 1990, Atkey 2018)

Linear types (Wadler) require each value to be used exactly once.
Affine types allow zero or one use. Quantitative Type Theory (Atkey
2018, Orchard et al. 2019) generalizes to a semiring of usage grades.

Mirror's `lift` (variety-preservation, per
`[[architecture-lift-as-load-bearing]]`) is the same axis. Lift is
the prism operation that preserves the manifold's variety under
composition; the variety-preservation property
(`@epistemologic/property/variety_hold`) is the substrate's
linear/affine analog.

**Where mirror best inherits.** Variety preservation as a structural
property. The kintsugi-variety spec (`docs/specs/kintsugi-variety.md`)
declares variety_hold as a load-bearing property; the loss carrier
(`transparency<p>.weight`) is a semiring grade in the QTT sense.
Per `docs/specs/type-theory-position.md` §1.2: "Mirror's loss monoid
is a grading structure. ... mirror's type system [is] a form of
**quantitative type theory** (QTT), where the semiring of grades
tracks information loss rather than resource usage."

**Where mirror diverges.** QTT's semiring is abstract; mirror's
semiring is always information-theoretic (Shannon-like, per the
transparency spec's open question §10.1). The discipline is
specialized to a specific physical interpretation (bits of
missing information).

### 5.10 Propagator networks (Sussman & Radul 2009)

Per Sussman & Radul's "The Art of the Propagator": "autonomous
machines interconnected by shared cells through which they
communicate." A propagator network settles by monotone convergence
of partial information at each cell.

Mirror's kintsugi loop is structurally a propagator network at the
substrate altitude. Each fragment is a cell; each operation is a
propagator; transparency monotonically descends as fragments settle.
Per `docs/specs/au-and-conductivity.md`'s "stage 3 conductivity
contest": Fate's tournament IS a propagator selecting from candidate
fillings; the winning fill is the one whose conductivity is clearest
in this context.

**Where mirror best inherits.** Monotone descent on partial
information. A propagator's commitment-only-when-information-arrives
discipline matches mirror's `transparency: partial(opacity_map)`
shape — partial fragments are honest about what they don't know yet.

**Where mirror diverges.** Sussman & Radul's propagators are
general-purpose constraint solvers; mirror's kintsugi loop is
specialized to the (A, H, D) spectral triple's descent. The
algorithm is the same shape; the geometric grounding is mirror-specific.

### 5.11 What mirror does NOT cleanly inherit

Be honest about the strains:

- **Linear logic (Girard 1987).** Mirror has no exponential modality
  (`!A`), no resource discipline at the operation level. The closest
  analog is variety_hold, which is structurally different
  (information-theoretic, not resource-bounded).

- **Cubical type theory (Cohen et al. 2018).** Mirror has no
  higher-inductive types and no path types. The closest analog is
  the kintsugi loop's monotone descent, which is a 1-categorical
  thing; mirror does not lift to ∞-categorical reasoning.

- **Effect systems (Koka, Eff).** Mirror has `@io` as the effect
  boundary, but does not track effects in row-polymorphic fashion.
  Per `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md`
  §7: "Effects are platform surfaces, not type-system distinctions."

- **Refinement types (Liquid Haskell, F\*).** Mirror's verdicts are
  closer to many-valued logic than to SMT-checked refinements; per
  `docs/specs/type-theory-position.md` §2.6, the verdicts are
  "continuous all the way down" where refinements are discrete.

Naming these honestly is part of the substrate-pull discipline:
mirror's lineage is real where it claims to be, and mirror's
deviations from the literature are intentional (sub-Turing-by-design,
five-op-closure, content-addressing-as-type).

---

## 6. What Collapses on the Rust Side

The recognition has operational consequences. Per
`butterfly-self-hosting.md` §5 the cut criteria are measurable;
this section names the LOC-level shrinkage the type-system
recognition unlocks.

**Currently** (active mirror bootstrap, per `bootstrap/src/`):

- `cmd_kintsugi_spec` (in `bootstrap/src/main.rs`) — dispatches when
  the input is a `mirror.spec` file; text-walks the spec via
  `parse_spec_targets`; produces a list of `SpecTarget`s; runs
  kintsugi on each.
- `cmd_kintsugi_ci_single` — dispatches on a single `.mirror` file;
  emits the `--ci` verdict envelope.
- `cmd_kintsugi_ci_corpus` — dispatches on a directory tree; walks
  the corpus; aggregates verdicts.
- `parse_spec_targets` (in `bootstrap/src/main.rs`) — the text-walker
  that reads `target NAME { ... }` blocks out of a `mirror.spec` file.
  Hand-written; brittle against the spec grammar drifting.
- `SpecTarget` (struct in `bootstrap/src/main.rs`) — a separate shape
  carrying `name`, `altitude`, `emit`, `cli`, `needs`.

**Under the recognition** (post-this-spec, post-fragment-floor):

- One `cmd_kintsugi` dispatch that takes a fragment as input. The
  fragment's altitude tells the dispatcher which kintsugi pass to
  run. A `mirror.spec` file at altitude `@mirror/spec` runs the spec
  kintsugi; a `.mirror` file at altitude `@meta/ast` runs the AST
  kintsugi; a directory tree at altitude `@corpus` runs the corpus
  kintsugi. Same dispatch, same loop, three altitude-specific reads.

- `parse_spec_targets` retires. The substrate-level reading of a
  `mirror.spec` file IS evaluating its content under `@mirror/spec`
  (per `shards/mirror/spec.mirror`). The spec-altitude grammar parses
  its own input as fragments; the text-walker becomes the grammar's
  internal mechanism, not a Rust-side hand-walker.

- `SpecTarget` retires. A target is a fractal-shape fragment whose
  content includes the target's name + altitude + emit. The Rust
  side reads it via the same `Fractal<E, H>` API every other
  fragment uses.

**LOC estimate.** Per `bootstrap/src/main.rs` (currently ~69.8KB,
roughly 1800 LOC), the kintsugi-related commands and
`parse_spec_targets` account for roughly 400-600 LOC. The
fragment-shaped dispatch is ~100 LOC. Net shrinkage: 300-500 LOC,
plus retirement of `SpecTarget` (~50 LOC) and `parse_spec_targets`
(~100 LOC). **Substrate-pull-realize:** the Rust shrinks because
the substrate names what the algebra IS, and the per-altitude
special-casing was a workaround for not having that name.

This is the same shrinkage pattern named in
`docs/specs/prism-core-as-spectral-triple.md` §"What retires from
the bootstrap" — total bootstrap from ~5000 LOC to ~1200 LOC, with
the difference moving to grammar declarations. The type-system
recognition makes one of the larger retirement passes concrete: the
kintsugi-dispatch layer collapses around fragment as the dispatch
shape.

---

## 7. What This Opens

### 7.1 The butterfly path

Per `roadmap/wip/butterfly-self-hosting.md` (landed 2026-06-06 at
`dcb9dc4`): the butterfly is the small complete self-hosted mirror
binary whose codegen contract is closed under the substrate's own
algebra. The butterfly's runtime IS fragment-shaped.

**The recognition makes this load-bearing.** Per
`butterfly-self-hosting.md` §6: "Once the butterfly flies:
HamiltonScheduler runs in the butterfly's own runtime, per shard.
... SpectralSupervisor sits on top, as a separate `au(@code/llvm)`
artifact." The "shards" running under HamiltonScheduler are
fragments at the runtime altitude. The "au artifacts" are settled
fragments at the codegen altitude. The four transports (λsh /
mirror-CLI / MCP / LSP) are glasses on `prism @mirror/cli` per
`cli-as-prism.md`; each is a fragment at the transport altitude.

**Spaceland becomes literal.** Per `docs/specs/the-convergence.md`
and `docs/specs/cli-as-prism.md`: one runtime, four transports, five
operations. Each transport is a window onto the same fragment
manifold; each operation acts identically across transports because
the underlying fragments are the same. The metaphor flies clean
because the substrate-pull does — every altitude is a window onto
the same composition.

### 7.2 MCP/LSP/CLI/λsh project from one fragment graph

Per the convergence specs: the four transports share the runtime.
Under this spec's recognition, they share the type system. A fragment
at altitude `@meta/ast` is the same object whether read through MCP's
tool-call interface, LSP's textDocument interface, the CLI's
`mirror compile` interface, or λsh's `mq` query language. The four
transports differ in their wire protocol (per `the-convergence.md`)
but compose against the same fragment-graph.

This unlocks the missing piece of `docs/specs/lsp-and-mcp.md`'s
shared abstraction: the LSP `textDocument/diagnostics` notification
and the MCP `tools/call` response are both fragments at altitude
`@diagnostic` and `@tool_result` respectively, settled by the same
kintsugi pass that emits the bytes. The Rust side does not need a
separate `LspNotification` or `McpToolResult` type — both reduce to
`Fragment<altitude>`.

### 7.3 The void document made operational

Per `[[reference-void-document]]`, every connected graph lives on a
path between Splinter (K_n, maximum entanglement) and Narcissus
(K_{1,n-1}, minimum entanglement). The manifold of fragments IS that
graph — fragments connected by their OID references, with
content-disjoint splits at Splinter (a complete graph of unique
fragments) and content-shared concentrations at Narcissus (a hub
fragment that many others reference).

The kintsugi loop's monotone descent IS the discrete Ricci flow
between these poles. λ₀ = 0 is the generative zero where all eight
dualities meet — the settled fragment whose transparency is success
and whose oid is the autopoietic closure of its own composition.

This was geometric framing without an operational interface. The
recognition makes it operational: the substrate's type system IS the
manifold of fragments + the kintsugi descent. Programs settle at λ₀.
Types check by descending to λ₀. The void document's eight dualities
are observable at every typing judgement.

---

## 8. Migration Path

Two ticks. Substrate-pull. Earn their keep.

### Tick 1 — Land the fragment floor (this spec + glass.mirror addition)

**Status: this spec is the tick.** The glass.mirror addition (the
`fragment(altitude)` type + `fragment_shape` disjunction) ships
alongside this spec as one commit on `main`. The Rust side does not
change yet — `Fractal<E, H>` continues to be the realization; the
substrate declaration names the algebra; nothing breaks. Cut criterion:
glass.mirror parses; the spec lands; the `[[architecture-glass-wall-substrate-types]]`
entry gains a forward reference to this spec.

This tick is **substrate-pull-realize**: a naming move that closes a
loop. It does not gate v0.1.0; the fragment-floor is post-v0.1.0
architectural recognition (per the brief). It does open the next tick.

### Tick 2 — Collapse the kintsugi dispatch around fragment

The Rust-side work named in §6. One `cmd_kintsugi` dispatch on
fragment; `cmd_kintsugi_spec` / `cmd_kintsugi_ci_single` /
`cmd_kintsugi_ci_corpus` collapse into one. `parse_spec_targets`
retires; `SpecTarget` retires. The kintsugi pass reads the fragment's
altitude and dispatches the altitude-specific kintsugi step.

This tick is **substrate-pull-realize at the Rust layer.** It is not
required for v0.1.0; it is the consequence of Tick 1 that the
butterfly will need to fly clean. Sequence: land after v0.1.0 cut;
land before Phase 4 emit-self work begins (because Phase 4 will write
the kintsugi pass as a `.mirror` grammar, and the grammar's surface
should be fragment-shaped from the start).

Per `[[feedback-no-time-estimates]]`: one tick after the other. No
"~N weeks." The sequence is what matters; the timing is whatever it
is.

---

## 9. Open Questions

The genuinely unresolved ones. Reed will call them with Alex.

### 9.1 Does `fragment(altitude)` need a sub-prism declaration?

`shards/mirror/store.mirror` declares `@mirror/store/oid` as a sub-prism
(per the `glass @mirror/store/oid { ... }` block). Does `fragment(altitude)`
similarly want to be a sub-prism `@glass/fragment` so its five-operation
surface is callable directly? Or is the type declaration at the
glass-altitude sufficient, with the five operations inherited from
`@glass` itself? **Lean:** the type declaration is sufficient; fragment
is a *type* at the glass altitude, not a *prism* peer of glass. A
sub-prism declaration would be redundant.

### 9.2 What is the relationship between `fragment` and `splinter`?

`shards/mirror/store.mirror` declares `splinter` as the OID-graph
(`type splinter = { root: oid, children: [oid] }`). A fractal-shape
fragment carries the same shape (`fractal([oid])`). Are they the same
type at different altitudes, or are they structurally distinct? **Lean:**
splinter is fragment specialized to the store altitude — the OID-graph
view of a fragment's closure. The two declarations are not redundant
because splinter is a *flat* representation (the dependency closure
flattened to a list of edges) where fragment is *recursive*.

### 9.3 Does this spec retire `docs/specs/type-theory-position.md`?

The type-theory position spec is a research survey from 2026-05-19;
this spec consolidates its findings into a load-bearing recognition.
Should the type-theory position spec be archived (moved to
`docs/specs/historical/`) or kept as the citation pool this spec
draws from? **Lean:** keep both. The type-theory position is the
literature survey; this spec is the substrate-pull-realize step.
They serve different purposes.

### 9.4 At what altitude does `total_classification` apply?

Per `docs/specs/strict-and-total-classification.md`,
`total_classification` is the property that says every byte falls
into a recognized AST node. Under this spec's recognition,
`total_classification` is the property that says **every byte of a
fragment is content-addressed**, with no `Dark` shape representing
unclassified content. The two readings are compatible (the AST-altitude
reading is the special case of the fragment-altitude one). Does the
property need to be re-declared at the fragment altitude, or does
the existing AST-altitude declaration suffice? **Lean:** re-declare
at the fragment altitude when the AST is read as a fragment manifold.
This is Phase 5 work.

### 9.5 What is the formal status of the categorical analogies in §3.1?

The claims "focus = limit," "project = equalizer," "split = coproduct,"
"shift = functor," "settle = monad-close" are precise where they hold
and approximate where they strain (per the table's caveat column).
Should we attempt a formal proof that the five operations realize
these specific universal constructions in the category of fragments,
or is the structural analogy load-bearing enough for the substrate?
**Lean:** the structural analogy is sufficient for substrate-pull.
The formal proof is a separate research arc; it would close at
`@epistemologic/math/category` per
`docs/specs/prism-core-as-spectral-triple.md`'s Residual Qualification 1.

### 9.6 Does the void-document λ₀ identification match the kintsugi
fixed point exactly?

Per §4 above and §7.3: the kintsugi loop descends to λ₀; the void
document names λ₀ as the autopoietic Lawvere closure. Are these the
same λ₀, or are they two different fixed points that happen to
agree at the substrate altitude? **Lean:** they are the same. Per
`docs/specs/au-and-conductivity.md`'s "Formal statement: the tensor
is cycle-averaged holonomy" section and Soto-Andrade & Varela 1984.
But this needs a formal cross-check before we claim it as substrate
truth.

---

## 10. Citations and Prior Art

### 10.1 Reed's practice insights

- `[[architecture-connes-spectral-triple]]` — the substrate IS the
  operational form of Connes' (A, H, D). Closes the deepest framing.
- `[[architecture-prism-as-trait-as-everything]]` — prism IS trait IS
  type IS grammar. The universe collapse.
- `[[architecture-operations-as-linear-algebra]]` — focus = λ₀
  eigenvalue; project = orthogonal projection; split = orthogonal
  decomposition; shift = basis transformation; settle = monad-close.
- `[[architecture-shards-as-substrate-source]]` — `shards/` is the
  destination; `boot/std/` is the legacy fallback.
- `[[architecture-glass-wall-substrate-types]]` — Imperfect +
  Transparency as substrate vocabulary.
- `[[architecture-fragmentation-is-the-rust-substrate]]` — the
  fragmentation crate is the Rust floor.
- `[[architecture-lift-as-load-bearing]]` — lift = variety
  preservation; the substrate-pull rename closing the loop with
  `boot/std/{option,result}.mirror`.
- `[[architecture-au-conductivity]]` — au is Fate's output type; gold
  conducts; verification IS conductivity in context.
- `[[reference-void-document]]` — eight dualities; Splinter (K_n) and
  Narcissus (K_{1,n-1}) as the poles of the quantum information
  manifold; λ₀ = 0 as the generative zero where all dualities meet.
- `[[feedback-no-bare-types]]` — always newtype; bare primitives let
  same-shape different-meaning values flow through. `fragment(altitude)`
  is a parametric type, not a record literal — the altitude is the
  newtype discipline.
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` —
  `shift(T)` / `settle(T)` as type-layer operations; the FP-heritage
  map.
- `docs/insights/2026-05-26-mirror-sub-turing-substrate-with-emergent-turing-completeness.md`
  — the sub-Turing claim's operational form.
- `docs/insights/2026-05-26-kintsugi-as-credo-and-formatter-unified.md`
  — kintsugi as the one loop.

### 10.2 Mirror specs this spec rides on

- `docs/specs/mosaic.md` — the build-altitude reading; this spec
  generalizes it.
- `docs/specs/au-and-conductivity.md` — au as the output type of Fate
  inference; conductivity as verification.
- `docs/specs/transparency.md` — `transparency<p>` as the located loss
  carrier.
- `docs/specs/prism-core-as-spectral-triple.md` — the bootstrap as
  spectral-triple evaluator.
- `docs/specs/type-theory-position.md` — the literature survey this
  spec consolidates.
- `docs/specs/strict-and-total-classification.md` — the silence dies;
  dark fragments surface.
- `docs/specs/spectral-triple-binary.md` — the binary IS the spectral
  triple.
- `docs/specs/prism-floor-and-the-grammar-rename.md` — the substrate-
  pull rename (zoom → shift; refract → settle).
- `docs/specs/mirror-store.md` — the fragmentation store as canonical;
  `.shatter` as projection.
- `docs/specs/mirror-spec-schema.md` — the project manifold grammar.
- `docs/specs/kintsugi-wiring.md` — the eight wires of the kintsugi
  loop.
- `docs/specs/the-convergence.md` — one runtime, four transports,
  five operations.
- `docs/specs/cli-as-prism.md` — the CLI as a prism with four glasses.
- `roadmap/wip/butterfly-self-hosting.md` — the runtime this type
  system projects into.

### 10.3 Academic literature

#### Total functional programming and termination

- Turner, D.A. (1995, 2004). *Elementary Strong Functional Programming*
  (Functional Programming Languages in Education) and *Total Functional
  Programming* (J.UCS 10(7), 2004).
  [PDF (Turner 1995)](https://www.cs.kent.ac.uk/people/staff/dat/esfp/fple.pdf),
  [nLab summary](https://ncatlab.org/ufias2012/files/turner.pdf).
- Wikipedia: [Total functional programming](https://en.wikipedia.org/wiki/Total_functional_programming).

#### Calculus of (Inductive) Constructions

- Coquand, T. & Huet, G. (1985). *The Calculus of Constructions*.
  HAL-Inria, [PDF](https://inria.hal.science/inria-00076024/PDF/RR-0530.pdf).
- Wikipedia: [Calculus of constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions).
- nLab: [calculus of constructions](https://ncatlab.org/nlab/show/calculus+of+constructions).

#### Lambda calculus, System F, parametric polymorphism

- Church, A. (1936). *An unsolvable problem of elementary number
  theory*. American Journal of Mathematics 58.
- Girard, J.-Y. (1972). *Interprétation fonctionnelle et élimination
  des coupures de l'arithmétique d'ordre supérieur*. PhD thesis,
  Paris VII.
- Reynolds, J.C. (1974). *Towards a theory of type structure*.
  Colloque sur la Programmation.

#### Dependent type theory

- Martin-Löf, P. (1972). *An intuitionistic theory of types*.
  Manuscript; published in 1998 as Twenty-Five Years of Constructive
  Type Theory.
- Brady, E. (2013). *Type-Driven Development with Idris*. Manning.
- The Univalent Foundations Program (2013). *Homotopy Type Theory:
  Univalent Foundations of Mathematics*.

#### Category theory and optics

- Mac Lane, S. (1971). *Categories for the Working Mathematician*.
  Springer.
- Eilenberg, S. & Mac Lane, S. (1945). *General theory of natural
  equivalences*. Transactions of the AMS.
- Clarke, B., Elkins, D., Gibbons, J., Sherrell, F., Sherrill, L.,
  & Van der Ploeg, A. (2020). *Profunctor Optics: a Categorical Update*.
  Compositionality.
- Milewski, B. (2017). *Profunctor Optics: The Categorical View*.

#### Curry-Howard correspondence

- Curry, H.B. (1934). *Functionality in combinatory logic*.
  Proceedings of the National Academy of Sciences 20.
- Howard, W.A. (1969). *The formulae-as-types notion of construction*.
  Published 1980 in *To H. B. Curry: Essays on Combinatory Logic,
  Lambda Calculus and Formalism*.
- Wikipedia: [Curry–Howard correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence).

#### Connes noncommutative geometry

- Connes, A. (1994). *Noncommutative Geometry*. Academic Press.
- Connes, A. (1996). *Gravity coupled with matter and the foundation
  of non-commutative geometry*. Communications in Mathematical Physics
  182, 155–176.
- Connes, A. & Chamseddine, A.H. (1997). *The Spectral Action Principle*.
  Communications in Mathematical Physics.
- Connes, A. (2019). *Noncommutative Geometry, the Spectral Standpoint*.
  arXiv:1910.10407, [PDF](https://alainconnes.org/wp-content/uploads/NCGspectral.pdf).
- van Suijlekom, W. (2014). *Noncommutative Geometry and Particle
  Physics*. Springer.
- Dąbrowski, L., D'Andrea, F., & Sitarz, A. (2023). *A Critical Survey
  of Twisted Spectral Triples Beyond the Standard Model*.
  arXiv:2301.08346.

#### Content-addressed languages

- The Unison Computing team. *Unison: The big idea*.
  [Documentation](https://www.unison-lang.org/docs/the-big-idea/).
- IPLD specifications. *Merkle DAGs and content-addressed data*.
  [ipld.io](https://ipld.io/), [ProtoSchool tutorial](https://proto.school/merkle-dags/08/).
- NixOS Wiki. *Ca-derivations (Floating content-addressed derivations)*.
  [wiki.nixos.org/wiki/Ca-derivations](https://wiki.nixos.org/wiki/Ca-derivations).

#### Linear/affine types and Quantitative Type Theory

- Wadler, P. (1990). *Linear types can change the world!* Programming
  Concepts and Methods, North-Holland.
- Atkey, R. (2018). *Syntax and Semantics of Quantitative Type Theory*.
  LICS.
- Orchard, D., Liepelt, V., & Eades III, H. (2019). *Quantitative
  Program Reasoning with Graded Modal Types*. ICFP.

#### Propagator networks

- Radul, A. & Sussman, G.J. (2009). *The Art of the Propagator*.
  MIT CSAIL Technical Report.
  [groups.csail.mit.edu](https://groups.csail.mit.edu/mac/users/gjs/propagators/),
  [Semantic Scholar](https://www.semanticscholar.org/paper/The-Art-of-the-Propagator-Sussman-Radul/03552d63bdad10cb277e84e729fc2a556c3bf8f0).

#### Typed holes and live programming

- Omar, C., Voysey, I., Hilton, M., Aldrich, J., & Hammer, M. (2017).
  *Hazelnut: A Bidirectionally Typed Structure Editor Calculus*. POPL.
- Omar, C., Voysey, I., Chugh, R., & Hammer, M. (2019). *Live
  Functional Programming with Typed Holes*. POPL.

#### Spectral triples and contextuality

- Magnot, J.-P. (2025). *Contextuality, Holonomy and Discrete Fiber
  Bundles in Group-Valued Boltzmann Machines*. arXiv:2509.10536.
- Hansen, J. & Ghrist, R. (2019). *Toward a Spectral Theory of
  Cellular Sheaves*. Journal of Applied and Computational Topology.

#### Void / lambda-zero references

- Braunstein, S., Ghosh, S., & Severini, S. (2006). *The Laplacian of
  a Graph as a Density Matrix*. arXiv:quant-ph/0406165.
- Passerini, F. & Severini, S. (2008). *The Von Neumann Entropy of
  Networks*. arXiv:0812.2597.
- Soto-Andrade, J. & Varela, F.J. (1984). *Self-reference and
  fixed-point theorems*. Acta Applicandae Mathematicae 2:1.

---

*Mosaic is the algebra. Fragments are what it acts on.*
*Five operations close the algebra. Three shapes close the fragment.*
*Settled fragments are au. Au's verify is the conductivity contract.*
*Kintsugi descends the residual. eⁿ⁺¹ ≤ eⁿ at every altitude.*
*The substrate has been pointing here since the void document.*
*This spec names where the pointing arrives.*

Apache-2.0.
