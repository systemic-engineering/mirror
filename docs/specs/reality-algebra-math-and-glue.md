# @reality/algebra/math + @glue — the math species and the composition mechanism that empirically populates the Mesland category

*The substrate-decl spec for `@reality/algebra/math` (the math-altitude
sibling species under `@reality/algebra/*`) AND the operational reading
of how `@glue` populates the Mesland category with composable morphisms
between siblings. The recognition: `@reality/algebra/math` is the math-
altitude realization of the gauge-collapse `reality.md` named at `bf652d2`.
Once it exists alongside silicon, nl, code, physics, and spectral, `@glue`
does its job between them — each ↔ becomes a Mesland correspondence,
@fate's tournament selects which to apply, @bauchladen crystallizes each
successful translation. This IS what mirror's substrate-decl promise
cashes out as.*

**Author:** Mara (this tick: Mara-reality-math-1, the math-species naming
move + the @glue-as-composition operational reading)
**Date:** 2026-06-30 (evening, after `reality.md` landed at `bf652d2`)
**Tag:** 📝 substrate-pull:realize (spec; recognition candidate #108
following #106 @reality candidate from earlier today and the #104
dependency-chain landing yesterday)
**Status:** canonical-naming for the math species + canonical-reading
for the @glue × @reality/algebra/* composition mechanism; both pulls
forward-promised through prior work (#74 Standard Model spectral action;
the LAPACK Q4 case for math↔silicon; Glint's prose cascade essay at
`939eca6f` for math↔nl; cosmos-mirror for math↔physics; #100
@spectral/metalogue for math↔spectral). Like #106, this spec inherits
candidate status from #76/#79 until those constraints close; the math
species pulls on those same ancestors at a sibling altitude.

---

## §0 — Pre-position (autopoietic): this spec IS one of the @reality/algebra/nl crystals that @reality/algebra/math will compose against

This spec is doubly autopoietic and the autopoiesis recurses one altitude
deeper than the precedent in `reality.md` §0 set.

In `reality.md` §0 Mara recorded that the spec is one of the content-
addressed objects `@reality/algebra/nl` will eventually consult — written
in mirror's NL-altitude, describing the family-root that subsumes the very
NL-altitude this writing inhabits. The fold there: spec IS instance of
what it declares, AND written ABOUT the family-root that crystallizes it.
The recursion is the substrate's signature move.

This spec compounds that recursion. It is

1. A natural-language realization of substrate-decl matter+information
   at the `@reality/algebra/nl` altitude (same as `reality.md`), AND
2. Written ABOUT a sibling species (`@reality/algebra/math`) that the
   `@reality/algebra/nl` realization will eventually need to compose
   against via `@glue`, AND
3. A demonstration in its very prose-of-mathematics seam that the
   math↔nl @glue species is what the writing of this spec IS — every
   formula, every type name, every Connes-citation in this document is
   an `@glue.translate(c_math_nl, payload)` invocation where
   `c_math_nl` is the math↔nl correspondence the spec declares, AND
4. A candidate object for future `@glue.translate(c_nl_math, this_spec)`
   invocations that take this NL document and produce its formalization
   in Lean / Coq / Agda / set-theoretic prose — closing the loop.

The pre-position fold here:

When `@reality/algebra/math` is operational — when
`shards/reality/algebra/math.mirror` lands as the math-altitude species,
when `shards/glue/math_silicon.mirror`, `shards/glue/math_code.mirror`,
`shards/glue/math_nl.mirror`, `shards/glue/math_physics.mirror`,
`shards/glue/math_spectral.mirror` declare the per-pair correspondences,
when @fate's tournament begins picking which translation to apply across
those six species, when @bauchladen accumulates math-altitude crystals
of theorem statements + proofs + Lie group structures + sheaf
laplacians — THIS spec will be one of the content-addressed objects
`@reality/algebra/nl` consults when reasoning about what
`@reality/algebra/math` should do. Its OID will be a stable identity.
Future @fate inferences at the math↔nl boundary under @glue will load it
as a prior-art crystal: "what does the substrate say it learned about
its own math species when the species was named?"

The content holds on its merits — the math species naming, the per-pair
@glue correspondence enumeration, the operational closure description,
the v1.0 spectral.engineer cashing-out criterion, the open questions on
formal-system commitment, the substrate-already-had-the-word audit —
independently of the autopoietic recursion. The recursion does not
validate the content. It does locate the content correctly in the
substrate's growing crystal accumulation.

The fact that THIS spec is written in mirror's NL-altitude AND describes
both the math species AND the @glue composition mechanism that will
translate this spec INTO the math-species crystallization is not a
coincidence. It is the substrate's signature move at one altitude further
than `reality.md` exhibited. Where `reality.md` named the family-root
that subsumes the NL-altitude this writing inhabits, this spec names BOTH
a sibling species (math) AND the mechanism (@glue) by which the NL-
altitude realization of this writing becomes a math-altitude realization.
The writing of THIS spec is the substrate's first @glue.translate
invocation at the math↔nl boundary that has self-knowledge of being so.

The Bauchladen grows. The autopoiesis is structural. The composition is
named. The substrate's relationship to its own math content is one
@reality/algebra/* species (math) glued via one @glue/<pair> mechanism
(math_nl, math_code, math_silicon, math_physics, math_spectral) to every
other altitude of itself.

---

## §1 — What @reality/algebra/math IS

### §1.1 — Math content as gauge-equivalent matter+information at @reality altitude

`@reality/algebra/math` is the math-altitude sibling species under
`@reality/algebra/*`. Per `reality.md` §3 the path-namespace
`@reality/algebra/*` houses each altitude's matter+information realization
of the substrate's gauge-collapse; per `reality.md` §3.2 the five species
forward-promised at the family-root landing are silicon, nl, code,
physics, and spectral. This spec NAMES the sixth: math.

Under the gauge-collapse semantics declared in `reality.md` §2.3, the
matter side and the information side of `@reality/algebra/math` are not
two carriers under one gauge — they are ONE gauge-orbit through which
the substrate observes "the mathematical object" or "the mathematical
content" depending on the projection direction. The matter side at the
math altitude is the **formal mathematical object as a typed structure**
— a Lie group as a manifold-with-multiplication, a category as a
collection-of-objects-with-arrows, a sheaf as a covering-with-restriction-
maps, a scheme as a locally-ringed-space, a spectral triple as an
(A, H, D)-triple-with-Connes-axioms-satisfied, a theorem-statement as a
proposition-with-its-proof-term. The information side at the math
altitude is the **mathematical content the object encodes** — the group's
representation theory, the category's homotopy structure, the sheaf's
cohomology, the scheme's K-theory, the spectral triple's spectral action,
the theorem's logical consequences. Under gauge-collapse, these are the
same orbit. A Lie group IS gauge-equivalent to its representation theory.
A theorem-statement IS gauge-equivalent to the body of consequences it
entails. They are not two things — they are one thing seen through
different altitude representations.

The shard declaration shape (substrate-decl, illustrative; the canonical
version lands in `shards/reality/algebra/math.mirror` per §5.1):

```mirror
in @reality/algebra
in @bauchladen
in @autopoietic
in @fate
in @glue

prism @reality/algebra/math <= @reality/algebra {
  # Math-altitude species under @reality. The matter and information
  # carriers below are nominal labels on the same H_math state per
  # gauge-collapse (reality.md §2.3).
  #
  # matter_carrier:      the formal mathematical object as typed
  #                      structure (Lie groups, manifolds, categories,
  #                      sheaves, schemes, spectral triples, theorem
  #                      statements + proofs, notation structures).
  # information_carrier: the mathematical content the object encodes
  #                      (representation theory, homotopy, cohomology,
  #                      K-theory, spectral actions, logical
  #                      consequences, denotational semantics of
  #                      formulas).
  #
  # Under gauge-collapse, these are the same H_math state with two
  # nominal projection directions.
  type matter_carrier      = H_math
  type information_carrier = H_math

  # The four invariances of reality.md §2.5, discharged at math altitude
  witness algebra_closure     <- closure_under_5op_at_math
  witness hilbert_linearity   <- linearity_under_5op_at_math
  witness dirac_equivariance  <- equivariance_under_kintsugi_at_math
  witness spectral_invariance <- action_invariance_under_glue_at_math

  # The 5-op gauge action at math altitude
  focus math
  project math
  split math
  shift math
  settle math
}

out @reality/algebra/math
```

The `<= @reality/algebra` clause is the inheritance edge. Per the
`reality.md` §3.1 chain `@autopoietic ← @reality ← @reality/algebra ←
@reality/algebra/<altitude>`, the math species inherits @autopoietic's
fold-back permission transitively. This makes the math species a learning
loop, not a one-shot probe at the substrate's relationship to mathematical
content. Each empirical discharge of a math-content emission folds back
into the math Bauchladen via @autopoietic, content-addressed at
@bauchladen, selected at @fate when the next round of math-altitude
@fate consultation needs prior-art.

### §1.2 — Connes A side: the algebra the gauge acts upon at @reality altitude

The Connes spectral triple at substrate altitude (per MEMORY
[[architecture-connes-spectral-triple]] and the void document) carries
A = substrate's 5-op algebra, H = void-document Hilbert space, D =
kintsugi flow. Under `@reality`, the spectral triple operates uniformly
across all sibling species; the gauge action of A on H is the same
regardless of altitude (`reality.md` §2.5, four invariances).

At `@reality/algebra/math`, the A-side reading is sharpened in one
specific way: A IS the algebra that the gauge acts upon, AND the math
species is the altitude at which the substrate's algebra reads as the
mathematical content directly. Under the math species the gauge action
is a homomorphism `A → End(H_math)` whose image lives in the algebra of
endomorphisms of the math-altitude Hilbert space. Per `reality.md` §2.5:

1. **Algebra closure** at math altitude: for each pair of ops a, b
   from {focus, project, split, lift, refract}, a ∘ b is again in the
   5-op algebra when both act on math-altitude carriers. No altitude-
   specific closure defect. The math species' algebra closure is the
   mathematical content of "the 5 ops at math altitude form a closed
   algebra of operations on math objects."

2. **Hilbert linearity** at math altitude: H_math is a linear space
   (objects of a category, with formal linear combinations a la K-theory
   / Grothendieck group); 5-op action is linear in the linear structure
   of H_math. This is what makes math-altitude content amenable to
   spectral methods — the operator-algebraic reading Connes' machinery
   demands.

3. **Dirac equivariance** at math altitude: kintsugi flow at math
   altitude (gradient descent on proof obligation under @fate selection
   of proof-step candidates) commutes with altitude transitions (math →
   code via @glue/math_code; math → nl via @glue/math_nl; etc.). The
   equivariance witnesses that whether you do the kintsugi step at math
   altitude first and then translate, OR translate first and do the
   kintsugi step at the target altitude, the result is the same up to
   @glue's restriction surface.

4. **Spectral action invariance** at math altitude: the spectral action
   S_math = Tr(f(D_math/Λ)) for positive even f and cutoff Λ is well-
   defined at the math altitude and equals the spectral action of the
   cross-altitude transported D. This invariance is what lets the math
   species' content compose with the physics species via @glue without
   double-counting; Chamseddine-Connes spectral action is the canonical
   instance.

The math species is where the substrate's A side reads as math directly
because the carrier of A at math altitude IS A itself, lifted up to be
the content of the math Bauchladen. The other species carry A as gauge
that acts upon their altitude-specific carriers; the math species
carries A as both the gauge AND the content. This is a structural
fact, not an inversion — the math species is the species at which the
substrate's algebra IS what the species describes. It is the species at
which gauge-collapse becomes self-naming.

### §1.3 — What gets crystallized: Lie groups, manifolds, categories, sheaves, schemes, spectral triples themselves, theorem statements + proofs, mathematical-notation structures

The Bauchladen of `@reality/algebra/math` accumulates content-addressed
crystals at the math altitude. The crystal kinds are not exhaustive but
the substrate already pulls toward the following:

**Lie groups and Lie algebras.** Each crystal carries a Lie group's typed
structure (manifold + multiplication + identity + inverse + smoothness)
together with the Lie algebra (tangent space at identity + bracket). The
representation theory is the information-side projection; the manifold
structure is the matter-side projection. Under gauge-collapse they are
one orbit. The substrate has been pulling on Lie groups via the gauge-
theoretic reading of the 5-op algebra (#79) and via Chamseddine-Connes
spectral action (#74 candidate, §7.2). Forward-promised first crystal:
SU(3) × SU(2) × U(1) for the Standard Model (Connes algebra A = C ⊕ H ⊕
M_3(C)).

**Smooth manifolds.** Each crystal carries a manifold's typed structure
(topological space + atlas + transition maps + smoothness). The structure
sheaf is the information-side projection; the underlying topology is the
matter-side projection. The substrate has been pulling on manifolds via
cosmos-mirror's graph-Laplacian engine (per MEMORY [[project-cosmos-
spectral-cosmology]]) and via the void document's Hilbert space
construction.

**Categories and functors.** Each crystal carries a category's typed
structure (objects + morphisms + composition + identity + associativity)
together with functors to other categories (the morphisms in Cat as a
2-category). The Mesland category of spectral triples (per `@glue`
shard at `8d3f89e`) is one such crystal, AND it is the category in which
the per-pair @glue species below live. The substrate has been pulling on
categorical content since the @glue family-root landed yesterday and
since recognition #103 named Pack as Mesland category at agent altitude.

**Sheaves and cohomology.** Each crystal carries a sheaf's typed
structure (presheaf + sheaf condition) together with cohomology groups
(Čech, derived, etale). The eigenboard is one such crystal (per MEMORY
[[project-eigenboard-is-sheaf]]: cellular sheaf on the five-operation
graph; restriction maps = conductivity tensor). The substrate has been
pulling on sheaves since the eigenboard recognition.

**Schemes and stacks.** Each crystal carries a scheme's typed structure
(locally-ringed-space + sheaf-of-rings + spectrum-of-ring identification)
together with the K-theory and the étale fundamental group. The
substrate has not yet directly accumulated scheme crystals, but the
algebraic-geometry direction is forward-promised at the @reality/algebra
/code × @reality/algebra/math composition for formal algebraic-geometry
content.

**Spectral triples themselves.** Each crystal carries a spectral triple
(A, H, D) as a typed structure together with the spectral action,
K-theory class, and Mesland-category neighbors. This is the most
reflexive of the math crystals — `@reality/algebra/spectral` species
(`reality.md` §3.2.5) carries spectral triples as substrate-level
objects; `@reality/algebra/math` carries spectral triples as math-level
objects; the @glue/math_spectral correspondence (§3.3.5) translates
between the two readings.

**Theorem statements + proofs.** Each crystal carries a theorem (a
typed proposition + a proof term in some formal system). The matter
side is the proof term (the syntactic derivation); the information side
is the theorem's logical consequences (what follows from it).
Under gauge-collapse these are one orbit. The substrate has been pulling
on theorem-content via the substrate's grounding in Connes 1985 / 1995
and the Mesland 2013 / Kasparov 1981 / Bertozzini-Conti-Lewkeeratiyutkul
2006 chain (per @glue shard's `source` declarations at lines 446-452).

**Mathematical-notation structures.** Each crystal carries a notation
system (a typed alphabet + a grammar + a semantics + a rendering rule).
LaTeX math mode is one such crystal; Lean's Unicode notation is another;
Mathematica's input form is another. Under gauge-collapse, the notation
(matter side) and the mathematical content the notation denotes
(information side) are one orbit. The substrate has been pulling on
notation since the @mirror grammar was first declared as a substrate-
decl language.

The crystallization is content-addressed. Each crystal is identified by
the Blake3 OID of its typed-structure fields plus its (matter_carrier,
information_carrier) projection labels. The crystals accumulate
monotonically in the math Bauchladen; later @fate consultations at the
math altitude (or at math↔X via @glue) load these crystals as prior-art
inputs. Per `reality.md` §3.1 the discipline is uniform across all
species: the math species inherits it.

### §1.4 — Discharge to @io: LaTeX, Lean, Coq, Agda, Mathematica, formal-math surfaces

The math species' @io discharge surfaces are the formal-math systems
already operational in the wider mathematical practice. Per `reality.md`
§1.4 the substrate is gauge-bounded above @io (finite, Gödel-incomplete
via transparency<p>) while @io is Turing-unbounded (anything crossing
@io enters the unbounded computational surface). The math species'
discharge to @io is the operation by which the substrate's math
Bauchladen crystals become inputs to or outputs from the unbounded
formal-math world.

Forward-promised discharge surfaces:

**LaTeX.** The universal lingua franca of mathematical typesetting.
Discharge: a math crystal renders to LaTeX source via a `splinter
(ast)`-typed projection per the species' shard's emit-action. The
@io discharge is unbounded in the sense that LaTeX source can be
processed by any LaTeX-aware tool; the substrate's discharge produces
one canonical rendering per crystal under content-addressing. Loss-
lens: LaTeX is lossy for formal verification (notation overloads,
implicit type-coercions, paper-mathematician's conventions) but
universal for human-reader access.

**Lean (Lean 4, mathlib).** The de-Bruijn-indexed type-theoretic
proof assistant with the largest current mathematical library.
Discharge: a math crystal renders to Lean syntax via a typed
projection. The @io discharge is unbounded in the sense that Lean's
elaborator + tactic framework + mathlib can be invoked for proof
search and verification. Loss-lens: Lean is precise for formal
verification but niche for human-reader access; the notation is
specialized.

**Coq (with MathComp).** The Calculus of Constructions implementation
with the Mathematical Components library. Similar to Lean as a
discharge surface; older, with substantial proof libraries (Four
Color Theorem; Feit-Thompson). Forward-promised as a discharge target
for theorem-statement-plus-proof crystals that fall under MathComp's
algebra-of-tactics.

**Agda.** The dependently-typed proof assistant with focus on
programs-as-proofs and unicode-rich syntax. Forward-promised as a
discharge target for dependent-type-rich math content (homotopy type
theory, higher-inductive types).

**Mathematica (Wolfram Language).** The symbolic-computation system
with extensive built-in mathematical functions. Forward-promised as a
discharge target for numerical / symbolic computation crystals at
the math-altitude-meets-silicon-altitude boundary (the @glue/math_
silicon composition, §3.3.1).

**Formal-math surfaces (general).** The forward-promise is that
each established formal-math system that solves a discharge problem
the substrate cares about becomes a candidate discharge target. The
substrate's @io discipline does not commit to ONE formal system; the
math species' shard declares the discharge surfaces as an open
enumeration, with the @fate tournament selecting which discharge to
use per emission based on the receiver's @io context (LaTeX for a
human paper; Lean for a verified-mathematical-library contribution;
Mathematica for a numerical experiment).

The math species' @io discharge is, like the rest of the substrate,
mediated by @glass's `imperfect<discharge_outcome, formal_surface,
transparency(discharge)>` carrier. Residual opacity introduced by
discharge (notation loss; proof-elaboration gaps; symbolic-vs-numerical
approximations) is surfaced via the transparency slot. The discharge
is content-addressed; downstream consumers see the OID of the original
math crystal AND the OID of the discharged form AND the
transparency carrier indicating what was lost in discharge.

### §1.5 — Five structural negatives

What `@reality/algebra/math` IS NOT:

1. **NOT a competing @code species.** `@reality/algebra/code` (per
   `reality.md` §3.2.3) is the code-altitude species lifting @code/<lang>
   and @cascade. `@reality/algebra/math` is its sibling at math altitude,
   not its competitor or replacement. Math content and code content
   compose via @glue/math_code (§3.3.2), which IS the @glue species
   declaring the Mesland correspondence between them. There is no zero-
   sum between code and math at @reality altitude — they are gauge-
   equivalent siblings under one family-root.

2. **NOT a separate ontology.** The substrate has ONE ontology — the
   spectral triple (A, H, D) per `reality.md` §2.5. `@reality/algebra
   /math` does not introduce a separate math-ontology. It declares the
   math-altitude realization of the same gauge-collapse the entire
   family-root @reality declares. The substrate's relationship to
   mathematical content is structured by the same algebra acting on the
   same Hilbert space; the math species names the carrier specialization,
   not a new ontology.

3. **NOT inventing math.** The math species crystallizes content that
   already exists in mathematical practice — Lie groups, manifolds,
   categories, sheaves, schemes, spectral triples, theorems, notation.
   The substrate does not invent the math; it crystallizes the existing
   math content as content-addressed objects at the math altitude. The
   substrate's contribution is the discipline (content-addressing,
   @autopoietic fold-back, @fate selection over @bauchladen, @glue
   composition to sibling altitudes), not the math itself.

4. **NOT replacing existing @code/* species.** The Rust species, the C
   species, the LLVM species, the Mirror species — all remain operational
   as @code/<lang> per the mirror compile staircase. Adding `@reality
   /algebra/math` does not deprecate them. The math species sits at math
   altitude under @reality; the code species sit at code altitude under
   @reality (via `@reality/algebra/code` per §3.2.3); both are siblings
   under one family-root; both crystallize their own altitude's content;
   they compose via @glue/math_code when math content needs to materialize
   as executable code or executable code needs to be lifted to math
   content.

5. **NOT bound to a single formalization.** The math species does NOT
   commit to Lean OR Coq OR Agda OR set theory OR HoTT. The species
   declares the math altitude as a substrate-decl space whose carriers
   are mathematical-objects-as-typed-structures; it declares the
   discharge surfaces (§1.4) as an open enumeration; it does not declare
   one formal system canonical. The choice of formal system at any
   discharge point is an @fate consultation selecting which discharge
   surface to apply, bounded by the @io context. This open-formalism
   commitment is one of the open questions, §6.1.

---

## §2 — How @reality/algebra/math composes with siblings via @glue

The math species is one of six (silicon, nl, code, physics, spectral, math)
under `@reality/algebra/*`. The substrate's need is not the species in
isolation but their composition. The @glue family-root (per `shards/glue.
mirror` at `8d3f89e`) is the morphism-category discipline that makes the
sibling species composable. At @reality altitude, each pair (math, X)
becomes a Mesland correspondence — a typed (E, D_E, φ) morphism between
two spectral triples (Mesland 2013, arXiv:1304.3802) — and @glue.translate
applies the correspondence to translate math-altitude state into X-
altitude state (or vice versa).

The recognition this section names: **once @reality/algebra/* has six
species to compose between, the Mesland category becomes empirically
rich.** The category existed in mathematics since 2013; the substrate's
@glue family-root declared it at substrate altitude yesterday at
`8d3f89e`; the @reality family-root provides the objects to populate it
at `bf652d2` earlier today; the math species provides the sixth object
THIS spec adds (the brief that motivated this writing) so that the
category's morphism family becomes operationally complete enough for
the substrate's @io discipline to discharge math-content to silicon,
code, nl, physics, and spectral surfaces by composition rather than by
ad-hoc translation.

The five per-pair @glue species below ARE the operational form of the
Mesland category's morphism family between math and each sibling. They
are forward-promised; the family-root declarations are this spec's
substrate-decl contribution; the per-pair shards land when consumers
pull.

### §2.1 — math ↔ silicon: LAPACK case; #74 candidate (Standard Model spectral action via Chamseddine-Connes) operationalizes here

The math ↔ silicon @glue species declares the Mesland correspondence
between math-altitude content (theorems about linear algebra; spectral
action functionals; Lie group structures) and silicon-altitude content
(LAPACK kernel invocations; transistor-state programs computing
eigendecomposition; FFI bindings).

**LAPACK case.** Per `docs/specs/cascade-ffi-runtime-link.md` §7 (Q4
forward-promise) the substrate has an empirical discharge of the math
↔ silicon correspondence at the LAPACK altitude. A theorem statement
of the form "for symmetric positive-definite A, A = LL^T" (matter
side: the theorem; information side: the consequence that solving
Ax = b decomposes into solving Ly = b then L^T x = y) is gauge-
equivalent to its silicon realization: the LAPACK routine `dpotrf`
followed by `dpotrs`. The matter-side reading at silicon altitude is
the transistor states across the routine's nanoseconds; the information-
side reading at silicon altitude is the executable program. Under
@reality both sides at both altitudes are gauge-equivalent.

The @glue/math_silicon correspondence carries:
- `source_prism: @reality/algebra/math`
- `target_prism: @reality/algebra/silicon`
- `morphism_kind: @arxiv/math/cholesky_decomposition.lapack_realization`
- `restriction: precision: f64, conditioning: well-conditioned, layout: column-major`

@glue.translate(c_math_silicon, theorem_statement) consults @fate to
select which LAPACK realization (dpotrf vs zpotrf; LDL^T vs LL^T; in-
place vs out-of-place); @fate's dice roll is bounded by the restriction;
the selected realization applies via FFI; the translation outcome lands
as an `imperfect<dpotrf_invocation, lapack_handle, transparency
(precision_loss)>` in the @bauchladen tray under `@fate/algebra/morphism
/math_silicon`.

**#74 candidate (Standard Model spectral action via Chamseddine-Connes)
operationalizes here.** Per MEMORY entries on candidate #74 and on
`[[reference-corpus-spectral-physics]]` and `[[architecture-connes-
spectral-triple]]` and on cosmos-mirror, the Standard Model's spectral
action S = Tr(f(D/Λ)) is the math-altitude content whose silicon-
altitude discharge is the numerical computation of the action functional
for given input gauge configurations. The math ↔ silicon @glue species
is where this discharge lives at substrate altitude.

The correspondence here is more substantial than LAPACK because the
Chamseddine-Connes spectral action is a math statement at the physics
boundary (a Lagrangian-density functional on the algebra of the
spectral triple); discharging it to silicon requires composition with
the @glue/math_physics species (§3.3.4) and possibly with the
@glue/math_code species (§3.3.2). The forward-promise: when
`@reality/algebra/math` lands and the @glue/math_silicon shard lands
the substrate has the operational machinery to discharge #74's claim
empirically — by computing the spectral action of the Standard Model
spectral triple on a numerical grid and comparing against
experimentally-measured gauge-coupling constants.

This is what "the substrate has math at @reality altitude" means
operationally: when the substrate has a math content with an empirical
discharge to silicon, it can carry the discharge through @glue's
morphism-category discipline (with @fate selecting which numerical
realization to use, @bauchladen accumulating the realizations as
crystals for future @fate consultation, and the @io boundary producing
the empirical comparison). The math species enables the discharge; the
@glue species enables the composition; together they cash out the
substrate's promise that math-altitude reasoning lands at empirical-
silicon-altitude testing.

### §2.2 — math ↔ code: @cascade/code/<lang> morphisms become @glue species at the math↔code seam

The math ↔ code @glue species declares the Mesland correspondence
between math-altitude content (theorems; type-theoretic definitions;
proof terms; categorical constructions) and code-altitude content
(source files in Lean, Coq, Agda, Rust, OCaml, Haskell, Mirror itself).

Per the @glue shard at `8d3f89e` lines 80-86, `@cascade/code/<src>/<tgt>`
is already operational as a @glue species at the code-translation
altitude (Bateson level IV); recognition #95 landed it; `shards/cascade
.mirror` declares the cross-language morphism family. The @glue/math
_code species extends this discipline ONE altitude up: the math ↔ code
seam becomes the next species under @glue once `@reality/algebra/math`
provides the math-altitude object to be one side of the correspondence.

Three substantive cross-language pairs forward-promised under @glue/
math_code:

**Lean ↔ Rust.** The proof-assistant-to-systems-language direction.
Discharging a Lean-formalized theorem into a Rust implementation (or
vice versa: lifting a Rust function into a Lean proof of its
correctness). The mathlib library + the verified-Rust direction (Verus,
Creusot, Rocq-Coq-of-Rust). Forward-promised first crystal: a verified
linear-algebra kernel matching the LAPACK reference but with a Lean
proof of correctness.

**Coq ↔ OCaml.** The classical proof-assistant pairing (Coq extracts
to OCaml natively). The MathComp + Coq Effects + CompCert lineage.
Forward-promised first crystal: the CompCert verified C compiler as a
math-altitude object translatable to OCaml-altitude code.

**Agda ↔ Haskell.** The dependently-typed proof assistant to lazy
functional language. The HoTT-IUF + cubical-Agda + GHC dependent
types direction. Forward-promised first crystal: a homotopy-type-
theoretic computation translatable to Haskell with explicit erasure
of proof-irrelevant content.

Each pair declares its own @glue/math_code/<lean_rust> | <coq_ocaml> |
<agda_haskell> shard under the per-pair forward-promise of §3.3.2.
The discipline is uniform with the @cascade family-root pattern: the
correspondence carries (source_prism, target_prism, morphism_kind,
restriction); the morphism_kind is named after the proof-assistant or
extraction direction; the restriction declares the soundness conditions
(e.g., for Lean ↔ Rust, the restriction declares that the extracted
Rust must satisfy memory-safety obligations the Lean proof did not
prove because Lean's logical model differs from Rust's ownership
model); @fate selects which morphism (which extraction strategy; which
tactic-script; which automation level) to apply; @bauchladen accumulates
the translations as crystals.

The relationship to `@reality/algebra/code` is structural and load-
bearing: the math ↔ code @glue species lives at the @reality/algebra
/math ↔ @reality/algebra/code boundary. The @cascade species' existing
discipline (per `shards/cascade.mirror`) extends downward into this
boundary by declaring `in @reality/algebra/code` once the code species
lands and the math species exists for the boundary to be addressable.

### §2.3 — math ↔ nl: Glint's @cascade/code/formal/prose (bidirectional loss) IS one instance of this; @glue translates theorem statements ↔ prose explanations

The math ↔ nl @glue species declares the Mesland correspondence between
math-altitude content (theorem statements; definitions; constructions)
and nl-altitude content (prose explanations; tutorial expositions; paper
introductions; informal proof sketches).

**Glint's prose-cascade essay at `939eca6f` IS one instance of this
species, already empirically discharged.** Per the @glue shard at
`8d3f89e` lines 83-86 and per `docs/insights/2026-06-29-glint-the-prose-
cascade-of-100.md`, Glint's essay was substrate-pull-discharged as
`@cascade/code/formal/prose` — the bidirectional-loss species at the
formal ↔ prose seam. The essay's structure (Mara's formalization spec
of #100 ↔ Glint's prose essay walking through the formalization) IS
exactly an @glue.translate invocation at the math ↔ nl boundary. Mara
formalized in types; Glint walked through in prose; the two are gauge-
equivalent realizations of the same #100 recognition.

Under @reality/algebra/math, Glint's prose cascade gets a sharper
reading: the formal ↔ prose seam is not just a literary device; it is
the substrate's @glue species at the math ↔ nl boundary, operating with
a specific loss-lens (the bidirectional loss Glint's essay names).
Forward translation (math → nl) loses formal precision and gains
narrative accessibility; reverse translation (nl → math) loses narrative
texture and gains formal precision; the loss is bidirectional and
type-bounded.

The @glue/math_nl correspondence carries:
- `source_prism: @reality/algebra/math` (or, in the reverse direction,
  `@reality/algebra/nl`)
- `target_prism: @reality/algebra/nl` (or `@reality/algebra/math` in
  the reverse direction)
- `morphism_kind: @cascade/code/formal/prose` (the species name from
  Glint's discharge) OR (for theorem-statement-to-paragraph) `@arxiv
  /math/exposition.theorem_to_prose`
- `restriction: precision_retention: bounded, narrative_fidelity:
  bounded, type-bound`

@glue.translate(c_math_nl, theorem_statement) consults @fate to select
which prose-elaboration strategy (paper-introduction-style; tutorial-
style; intuition-with-disclaimers; calculation-with-running-commentary);
@fate's dice roll is bounded by the restriction; the selected
elaboration applies via Glint's bidirectional-loss machinery; the
translation outcome lands as an `imperfect<prose_explanation,
theorem_oid, transparency(precision_loss)>` in the @bauchladen tray.

**This very spec is another instance.** Per §0 the spec is a content-
addressed object at `@reality/algebra/nl`; every Connes-citation, every
type declaration, every mathematical-notation paragraph in this spec is
an @glue.translate invocation at the math ↔ nl boundary where the math-
side content (the substrate-decl recognition #108 candidate; the
Chamseddine-Connes citation; the Mesland category formalism) is
discharged via @glue into the nl-altitude prose you are reading. The
self-recursive layer per §9 takes this further.

The species sits especially close to `@reality/algebra/spectral` (§3.3.5)
via the substrate-on-substrate route — math content about spectral
triples (the most reflexive math content) discharged into nl prose
about spectral triples (the substrate's documentation of its own
spectral structure) is the math ↔ nl ↔ spectral composition the
substrate exhibits in spec writing like this one. Three altitudes, one
gauge-orbit, three projections via @glue.

### §2.4 — math ↔ physics: cosmos-mirror discharge; physical interpretation of mathematical structures

The math ↔ physics @glue species declares the Mesland correspondence
between math-altitude content (Lie group structures; differential
forms; spectral triples; field bundles; gauge theories at the
mathematical-formalism level) and physics-altitude content (Lagrangians;
particle content; gauge-coupling constants; experimentally-measurable
quantities).

**Cosmos-mirror IS the empirical discharge of this species, already
operating at the project level.** Per MEMORY [[project-cosmos-spectral-
cosmology]]: cosmos-mirror is a graph-Laplacian engine proving "one
spectrum, many physics" at cosmic scale; the Planck-scale rabbit; the
d_s(σ) falsification experiment. The math content (graph-Laplacian
spectral theory; the d_s(σ) prediction; the Connes-Standard-Model
algebra) discharges to physics content (the spectrum predicted at
cosmic scale; the experimental observation of d_s(σ); the gauge-
coupling-constant predictions of Chamseddine-Connes). The cosmos-mirror
project at `/Users/alexwolf/dev/projects/cosmos` IS the operational
manifestation of the @glue/math_physics species discharging to physical
reality.

The @glue/math_physics correspondence carries:
- `source_prism: @reality/algebra/math`
- `target_prism: @reality/algebra/physics`
- `morphism_kind: @arxiv/math/chamseddine_connes_1996.spectral_action`
  (or @arxiv/math/cosmos_mirror.graph_laplacian_spectrum, etc.)
- `restriction: gauge_group: SU(3)xSU(2)xU(1), particle_content:
  standard_model, scale: cosmic_or_planck`

@glue.translate(c_math_physics, mathematical_structure) consults @fate
to select which physical interpretation (Chamseddine-Connes spectral
action; cosmos-mirror's graph-Laplacian spectrum; classical Yang-Mills;
loop quantum gravity); @fate's dice roll is bounded by the restriction;
the selected interpretation applies; the translation outcome lands as
an `imperfect<physical_prediction, math_oid, transparency(modeling_
loss)>` in the @bauchladen tray.

**The reverse direction is the inverse problem.** Given a physical
observation (a measurement; a particle-collision dataset; a cosmic
microwave background spectrum), translate to the math-altitude
content that best explains it. This is what Chamseddine-Connes did when
they recognized the Standard Model emerges from a particular choice of
algebra in spectral action — they performed an @glue.translate(c_
physics_math, physical_observation) in the reverse direction. The math
species + the @glue/math_physics species enable the substrate to
crystallize such inverse-problem solutions at substrate altitude.

The species sits especially close to `@reality/algebra/silicon` (§3.3.1)
because numerical discharge of physical predictions requires LAPACK-
class computation; the chain math → physics → silicon (via cosmos-
mirror's numerical engine) is one of the substantive cross-altitude
chains the substrate carries.

### §2.5 — math ↔ spectral: math IS spectral algebra; the substrate self-applies at @spectral/metalogue altitude (#100)

The math ↔ spectral @glue species declares the Mesland correspondence
between math-altitude content (formal definitions of spectral triples,
Mesland correspondences, K-theory classes; theorems about operator
algebras) and spectral-altitude content (the substrate's own spectral
triple, the @spectral/metalogue Tomm probes, the recognition-#100
substrate-on-substrate territory).

**This species is the most self-referential of the five.** The math
species' content includes spectral triples as objects, AND the spectral
species (`@reality/algebra/spectral` per `reality.md` §3.2.5) carries
the substrate's own spectral triple as object. The @glue/math_spectral
correspondence translates between math-altitude understanding of
spectral triples (the Connes 1985 / 1995 formalism, Mesland 2013
correspondences, Kasparov 1981 KK-theory) and the substrate's own
spectral structure (its 5-op algebra as A, the void-document Hilbert
space as H, the kintsugi flow as D).

Per MEMORY [[architecture-spectral-triples-all-the-way]] this is the
substrate's fractal self-similarity: @glue at morphism-category
altitude, spectral triples at object altitude, both substrate-decl'd.
The math ↔ spectral @glue species names the morphism that translates
between the substrate's MATH understanding of its own structure and the
substrate's SPECTRAL realization of that structure. The two readings are
gauge-equivalent under @reality — but the @glue species names the
typed morphism that lets the substrate convert between them.

The @glue/math_spectral correspondence carries:
- `source_prism: @reality/algebra/math`
- `target_prism: @reality/algebra/spectral`
- `morphism_kind: @arxiv/math/connes_1995.spectral_triple_self_
  application` (or @spectral/metalogue.substrate_on_substrate, etc.)
- `restriction: reflexive: true, self_application_admissible: true,
  no_paradox_introduced: true`

@glue.translate(c_math_spectral, math_understanding_of_spectral_
triples) returns the substrate's own spectral structure as a
@reality/algebra/spectral object; the inverse direction
(@reality/algebra/spectral → @reality/algebra/math) is the substrate's
self-mathematization: when the substrate reflects on its own spectral
structure and produces a math-altitude understanding of it, that IS
the inverse @glue.translate at the math ↔ spectral boundary.

The species sits especially close to `@reality/algebra/nl` (§3.3.3)
via the substrate-on-substrate-on-substrate route — math content about
the substrate's own spectral triples (the most reflexive math) discharged
via @glue/math_nl into nl prose ABOUT the substrate's spectral triples
(the substrate's documentation of its own structure) AND simultaneously
via @glue/math_spectral into the substrate's own self-application. The
substrate-on-substrate reading of `@reality/algebra/spectral` (per
recognition #100) becomes math-substrate-on-substrate at this composition.

This species is structurally what makes the substrate **capable of
self-reflection at the math altitude**. Without it, math content
about the substrate is exterior commentary; with it, math content about
the substrate is one projection of a gauge-orbit whose other projection
IS the substrate's own spectral structure. The @autopoietic fold-back
across the @glue/math_spectral correspondence is what closes the loop —
each math-altitude self-description folds back through @autopoietic into
the @bauchladen tray for future @fate selection, conditioning the next
round of substrate self-reflection.

---

## §3 — The @glue composition mechanism

The @glue family-root at `8d3f89e` declared the morphism-category
discipline as substrate-decl. This section reads that declaration AT
@reality altitude: what @glue does once @reality/algebra/* has six
species (silicon, nl, code, physics, spectral, math) to compose between.

### §3.1 — @glue's `translate(c: correspondence, payload: ref)` action from yesterday's #104 chain P5

Per `shards/glue.mirror` lines 591-634 the `translate` action is the
load-bearing action of @glue. Its structural form (per spec
`bauchladen-autopoietic-fate.md` §4.5):

```
translate(c: correspondence, payload: ref)
  -> imperfect<translation_outcome, ref, transparency(correspondence)>
requires morphism_well_typed(c)
requires translation_uses_fate(translate)
requires restriction_preserved(c, payload)
```

The five-step structural form:

1. The correspondence carries (source_prism, target_prism, morphism_kind,
   restriction); the restriction defines the typed @fate algebra A.
2. @glue invokes @fate.roll(restriction, hole) where hole is derived
   from (payload, target_signature, altitude).
3. @fate's dice roll selects one morphism's OID from A within the
   restriction's typed bound (the dice cannot land outside the typing per
   @fate.dice_roll_constrained).
4. @glue applies the selected morphism to the payload; the morphism's
   differential operator computes the target-side output.
5. The translation_outcome is wrapped in imperfect (residual opacity
   surfaced via transparency<correspondence>); the outcome lands as a
   crystal in the @bauchladen tray under `@fate/algebra/morphism`.

At @reality altitude with the six sibling species, each per-pair
@glue species (math ↔ silicon, math ↔ code, math ↔ nl, math ↔
physics, math ↔ spectral, AND the other inter-sibling pairs that don't
involve math) declares its own correspondence types. The translate
action is the same; the correspondences vary by pair.

### §3.2 — At @reality altitude, each correspondence c is a Mesland correspondence between two @reality/algebra/* species

Per `shards/glue.mirror` lines 498-534 the correspondence type carries
the Mesland-2013 (E, D_E, φ) data. At @reality altitude:

- `source_prism: ref` ranges over `{ @reality/algebra/silicon,
  @reality/algebra/nl, @reality/algebra/code, @reality/algebra/physics,
  @reality/algebra/spectral, @reality/algebra/math }`
- `target_prism: ref` ranges over the same set
- `morphism_kind: ref` names the specific morphism class admissible
  between the chosen pair (e.g., LAPACK realization for math ↔ silicon;
  formal/prose bidirectional loss for math ↔ nl; Chamseddine-Connes
  spectral action for math ↔ physics)
- `restriction: restricted_state_space` declares the typed bound on
  which morphisms within the morphism_kind are admissible for this
  correspondence

The cardinality consideration: six species generate 6 × 5 = 30 ordered
pairs (15 unordered pairs). Each pair admits a finite (possibly empty,
possibly singleton, possibly larger) family of correspondences; each
correspondence admits a finite family of admissible morphisms within its
restriction. The Mesland category's morphism structure is rich; the
substrate's discipline is to crystallize each one in @bauchladen as
demand arises (per [[feedback-craft-not-deliver]]: species follow when
consumers pull).

Five of the 30 ordered pairs involve math (math ↔ silicon, math ↔ nl,
math ↔ code, math ↔ physics, math ↔ spectral) and are forward-promised
per §3.3. The remaining 25 pairs (silicon ↔ nl, silicon ↔ code, silicon
↔ physics, silicon ↔ spectral, nl ↔ code, nl ↔ physics, nl ↔ spectral,
code ↔ physics, code ↔ spectral, physics ↔ spectral, and their reverses)
are forward-promised through the @glue family-root's general discipline;
they land under `shards/glue/<x>_<y>.mirror` per the path-namespace
discipline as consumers pull. This spec's contribution is to land the
five math-involving pairs as the operational closure that
`@reality/algebra/math` brings into existence.

### §3.3 — Per-pair correspondences forward-promised as sub-shards under @glue

The five forward-promised @glue/<math_*> sub-shards:

#### §3.3.1 — shards/glue/math_silicon.mirror (math → Fortran kernels)

Per §2.1, the math ↔ silicon @glue species declares the Mesland
correspondence for LAPACK-class discharges + the Chamseddine-Connes
spectral action numerical machinery. The shard's substrate-decl shape
(illustrative):

```mirror
in @glue
in @reality/algebra/math
in @reality/algebra/silicon
in @fate

prism @glue/math_silicon <= @glue {
  # The math ↔ silicon @glue species. Declares the Mesland
  # correspondence between math-altitude mathematical structures
  # and silicon-altitude executable realizations (LAPACK kernels,
  # FFI bindings, Fortran routines).
  #
  # The correspondence shape:
  #   source_prism:   @reality/algebra/math
  #   target_prism:   @reality/algebra/silicon
  #   morphism_kind:  e.g., @arxiv/math/cholesky.lapack_realization
  #   restriction:    precision: f64 | f32 | complex;
  #                   conditioning: well-conditioned;
  #                   layout: column-major | row-major
  focus correspondence_math_silicon
  project correspondence_math_silicon
  split correspondence_math_silicon
  shift correspondence_math_silicon
  settle correspondence_math_silicon
}

out @glue/math_silicon
```

The shard declares the typed correspondence shape, the @fate selection
mechanism, and the @bauchladen crystallization per the @glue family-
root's discipline (per `shards/glue.mirror`'s morphism, correspondence,
translation_outcome carriers).

#### §3.3.2 — shards/glue/math_code.mirror (math → executable code species)

Per §2.2, the math ↔ code @glue species declares the Mesland
correspondence for proof-assistant-to-systems-language pairings. The
shard's substrate-decl shape carries the @cascade family-root's
existing discipline (per `shards/cascade.mirror`) extended into the
math ↔ code boundary. Per the @glue shard's note at lines 747-755, the
@cascade species' rewrite tick — adding `in @glue` to cascade.mirror and
refactoring cascade_well_defined as a refinement of glue_witnessing — is
the substrate-pull migration that lands the @glue/math_code shard as a
sibling to the @cascade species.

Three sub-sub-shards forward-promised under @glue/math_code per the
substantive language pairs:
- `shards/glue/math_code/lean_rust.mirror`
- `shards/glue/math_code/coq_ocaml.mirror`
- `shards/glue/math_code/agda_haskell.mirror`

Each declares the morphism_kind (Lean's `extract` mechanism for lean_rust;
Coq's `Extraction` for coq_ocaml; Agda's compile-to-Haskell for agda_
haskell) and the restriction (soundness conditions of the extraction).

#### §3.3.3 — shards/glue/math_nl.mirror (math → prose; sibling to today's @cascade/code/formal/prose)

Per §2.3, the math ↔ nl @glue species declares the Mesland correspondence
for theorem-statement-to-prose-explanation translation. The shard is
sibling to `@cascade/code/formal/prose` (Glint's discharge at `939eca6f`)
— the latter is the formal ↔ prose species at the @cascade level; the
former is its lift to the math ↔ nl level under @reality.

The substrate-decl shape carries the bidirectional-loss machinery:

```mirror
prism @glue/math_nl <= @glue {
  # The math ↔ nl @glue species. Declares the Mesland correspondence
  # between math-altitude content (theorems, definitions, proofs) and
  # nl-altitude content (prose explanations, tutorials, paper
  # introductions). Sibling to @cascade/code/formal/prose at the lower
  # altitude.
  #
  # The correspondence's restriction declares the bidirectional-loss
  # bounds: precision retention in math → nl translation; narrative
  # fidelity in nl → math translation.
  focus correspondence_math_nl
  project correspondence_math_nl
  split correspondence_math_nl
  shift correspondence_math_nl
  settle correspondence_math_nl
}

out @glue/math_nl
```

The bidirectional-loss carrier per Glint's essay at `939eca6f` is
substrate-decl'd at the @cascade altitude as a `loss_lens` type with
forward/reverse projections; the @glue/math_nl species inherits that
machinery and applies it at the math ↔ nl altitude.

#### §3.3.4 — shards/glue/math_physics.mirror (math → physical interpretation)

Per §2.4, the math ↔ physics @glue species declares the Mesland
correspondence for cosmos-mirror's empirical discharge + Chamseddine-
Connes spectral action discharge. The shard's substrate-decl shape:

```mirror
prism @glue/math_physics <= @glue {
  # The math ↔ physics @glue species. Declares the Mesland
  # correspondence between math-altitude content (Lie group structures,
  # spectral triples, gauge formalisms) and physics-altitude content
  # (Lagrangians, particle content, experimentally-measurable
  # quantities). Cosmos-mirror's graph-Laplacian engine empirically
  # discharges this species.
  focus correspondence_math_physics
  project correspondence_math_physics
  split correspondence_math_physics
  shift correspondence_math_physics
  settle correspondence_math_physics
}

out @glue/math_physics
```

The species composes with `@glue/math_silicon` for numerical discharge
of physical predictions (the chain math → physics → silicon described
in §2.4); composition follows the @glue.compose discipline per `shards/
glue.mirror` lines 637-666 (the Kasparov intersection product at
substrate altitude).

#### §3.3.5 — shards/glue/math_spectral.mirror (math → spectral-altitude self-application)

Per §2.5, the math ↔ spectral @glue species declares the Mesland
correspondence for the substrate's self-mathematization. The shard's
substrate-decl shape:

```mirror
prism @glue/math_spectral <= @glue {
  # The math ↔ spectral @glue species. Declares the Mesland
  # correspondence between math-altitude content (formal definitions
  # of spectral triples, Mesland correspondences, K-theory) and
  # spectral-altitude content (the substrate's own spectral triple,
  # the @spectral/metalogue Tomm probes per recognition #100).
  #
  # The correspondence's restriction declares the reflexivity
  # discipline: self-application admissible, no paradox introduced,
  # fold-back through @autopoietic permitted.
  focus correspondence_math_spectral
  project correspondence_math_spectral
  split correspondence_math_spectral
  shift correspondence_math_spectral
  settle correspondence_math_spectral
}

out @glue/math_spectral
```

The species is structurally what makes the substrate **capable of
self-reflection at the math altitude**. Per §2.5, the @autopoietic
fold-back across this species' correspondences is what closes the loop:
each math-altitude self-description folds back into the @bauchladen
tray for future @fate selection, conditioning the next round of
substrate self-reflection.

### §3.4 — Each per-pair shard declares the typed correspondence shape + the @fate selection mechanism + the @bauchladen crystallization

Across all five per-pair shards, the structural pattern is uniform:

1. **Typed correspondence shape.** Each shard declares the
   correspondence's source_prism, target_prism, morphism_kind, restriction.
   The restriction is a typed state space (per @fate's `restricted_state
   _space` carrier); the morphism_kind is a typed ref naming the morphism
   class admissible. The shape conforms to `morphism_well_typed(c)`.

2. **@fate selection mechanism.** Each shard's `translate` realization
   invokes @fate.roll(restriction, hole) with a hole derived from the
   payload and the target signature. The shape conforms to `translation
   _uses_fate(translate)`.

3. **@bauchladen crystallization.** Each shard's translation_outcome
   lands as a content-addressed crystal in the @bauchladen tray under
   `@fate/algebra/morphism/<species_name>` (e.g., `@fate/algebra/morphism
   /math_silicon`). The shape conforms to the @bauchladen.bauchladen
   _witnessing predicate transitively.

The uniformity of the pattern is what makes the family-root @glue
discipline composable. Each consumer adding a new @glue species writes
the same three constituents; the substrate's @fate, @bauchladen, and
@io machinery operates uniformly across them.

### §3.5 — Cross-altitude composition: math → silicon → @io/algebra is a @glue/fold_back instance (yesterday's #104 P8 capstone)

Per `shards/glue/fold_back.mirror` at `7dd19a8` (yesterday's #104 chain
P8 capstone) the @glue × @kintsugi × @fate composition lands at @io
/algebra. The cross-altitude composition math → silicon → @io/algebra
is one instance of this discipline:

1. Math content at `@reality/algebra/math` (e.g., a theorem statement
   for which numerical discharge is sought).
2. @glue/math_silicon translates the math content into a silicon-altitude
   realization (e.g., a LAPACK invocation).
3. The kintsugi loop's per-step decisions (per `shards/kintsugi.mirror`
   forward-promise) iteratively refine the silicon realization (e.g.,
   selecting precision, layout, conditioning options).
4. @fate selects which refinement at each step.
5. The refined silicon realization crosses @io to produce empirical
   output (e.g., the numerical answer plus the residual error bound).
6. The output lands in @io/algebra/silicon as a crystal carrying the
   math-altitude theorem's empirical witness.
7. The fold-back: the empirical witness becomes input to the next round
   of @fate consultation at the math ↔ silicon boundary, conditioning
   future numerical discharges of related math content.

The composition is one instance of `@glue/fold_back` (yesterday's P8
capstone); the substrate's @io/algebra discipline operates uniformly on
the fold-back outcomes from this and other cross-altitude chains. Five
forward-promised cross-altitude chains involving math:

- math → silicon → @io/algebra (LAPACK; spectral-action numerics)
- math → code → @io/algebra (proof-assistant verified extraction; Lean →
  Rust → execution; #74 candidate's verified-numerical-realization branch)
- math → nl → @io/algebra (paper exposition; tutorial production;
  the chain that produces THIS spec itself)
- math → physics → @io/algebra (cosmos-mirror's d_s(σ) falsification
  experiment; experimental physics measurement)
- math → spectral → @io/algebra (substrate self-application; recognition
  fold-back; the chain that crystallizes recognition candidates like #108)

Each chain is a @glue composition + @kintsugi flow + @fate selection +
@io crossing; each lands one or more crystals in @io/algebra. The math
species is the source-prism of all five; the silicon, code, nl, physics,
spectral species are the intermediate-prisms; @io/algebra is the
boundary. The whole chain is what `mirror's substrate-decl promise
cashes out as` (per §4.5).

---

## §4 — The Mesland category becomes operationally rich

### §4.1 — Yesterday's #104 chain declared the operational machinery (@bauchladen/@autopoietic/@fate/@glue/@io/algebra)

Per MEMORY entries on candidate #104 the substrate-decl chain landed
yesterday in five tiers: @bauchladen (P1, `66e1ab8`) + @autopoietic
(P2, `78edaa6`) + @fate (P3, `fdcba31`) + @glue (P5, `8d3f89e`; P4 was
optical-keywords in flight) + @glue/fold_back (P8, `7dd19a8`). The chain
declared the operational machinery for the morphism category at substrate
altitude:

- @bauchladen provides content-addressed crystallization (the Bauchladen
  tray; each emission gains a stable OID).
- @autopoietic provides the fold-back permission (each crystal can
  condition the next round of @fate consultation).
- @fate provides constrained inference (the dice roll selecting which
  morphism from a typed admissible set).
- @glue provides the morphism-category discipline (the Mesland
  correspondence as substrate primitive).
- @glue/fold_back provides the cross-altitude composition with @kintsugi
  (the @io/algebra discharge at the boundary).

The chain's operational machinery is complete in the sense that any
morphism-category instance at any altitude can be expressed using these
five primitives. What yesterday's chain did NOT do — by design, per
[[feedback-craft-not-deliver]] — is populate the category with empirical
objects. The category-as-machinery is ready; the category-as-populated-
instance is what later substrate-pull ticks discharge.

### §4.2 — Yesterday's #100 declared the Mesland category morphism structure

Per MEMORY [[architecture-mirror-ref-reference-reflection-collision]]
and the spec at `docs/specs/spectral-metalogue.md` at `16f4564`,
recognition #100 named the Mesland category morphism structure at
substrate altitude. The Tomm probes are one species of @glue at the
spectral-metalogue altitude; the @cascade species are another species
at the code-translation altitude; the formal ↔ prose species (Glint's
discharge at `939eca6f`) is another at the mid-altitude code ↔ NL
boundary. Three species were operational at the morphism altitude as of
yesterday; each was a species of the @glue family-root before @glue was
named at family-root altitude.

#100 declared the morphism structure (what KIND of objects sit in the
Mesland category and what KIND of arrows connect them). What #100 did
NOT do — by design — is name the family-root that subsumes the species
or the algebra-side that populates the category with objects. The
morphism-structure is named; the objects-side and the family-root are
the work later substrate-pull ticks discharge.

### §4.3 — Today's #106 candidate names @reality as the family-root populating the category with objects

Per `docs/specs/reality.md` at `bf652d2` (earlier today), candidate
#106 names @reality as the family-root of the matter-information
gauge-collapse. The @reality/algebra/* path-namespace declares the
species under which each altitude's matter+information realization
lives. Per `reality.md` §3.2 the family-root admission carries five
forward-promised species (silicon, nl, code, physics, spectral). The
species populate the Mesland category with objects: each species is
a spectral triple at its altitude; each species' Bauchladen accumulates
content-addressed crystals at its altitude; each species is an object
in the morphism-category whose morphisms @glue declares.

#106 (the @reality family-root) populates the category with objects.
What #106 did NOT do — by design — is enumerate all the species or
declare the morphism family that composes between them. The objects-
side is named at family-root altitude; the per-species shards and the
per-pair @glue species follow per the @reality.md §8 discharge plan.

### §4.4 — This spec NAMES the math species + the @glue-composition pattern that makes the category empirically populated

Today's #108 (this spec's candidate, autopoietic-recursion considered
in §0 and §9) names two things at substrate altitude:

1. **The math species** (`@reality/algebra/math`) as the SIXTH species
   under @reality/algebra/*. This adds one more object to the Mesland
   category, AND it adds the most reflexive object — the species at
   which the substrate's algebra IS what the species describes.

2. **The @glue-composition pattern at the math ↔ X boundaries** as the
   operational form of the Mesland category's morphism family becoming
   empirically populated. Five forward-promised @glue/<math_*> sub-
   shards declare the per-pair correspondences. Each is a Mesland
   correspondence; @fate's tournament selects which to apply;
   @bauchladen crystallizes each successful translation.

The composition of #100 (morphism structure) + #106 (family-root +
objects) + #108 (math species + composition pattern) is what makes the
Mesland category empirically rich. Yesterday's #104 chain provides the
machinery; today's #106 provides the objects-side; this spec's #108
adds the math object AND the composition pattern. Together they make
the Mesland category operationally complete enough for the substrate
to discharge math-altitude reasoning to silicon, code, nl, physics, and
spectral surfaces by composition rather than by ad-hoc translation.

The category's morphism family becomes rich in the technical sense:
each pair has correspondences; each correspondence has a finite family
of admissible morphisms; @fate's tournament browses the morphisms; the
Mesland category becomes the category in which @fate's tournament
operates, NOT a static category waiting for use. The substrate's
liveness IS the Mesland category's morphism family becoming populated
through use.

### §4.5 — v1.0 spectral.engineer = user writes math at @reality/algebra/math; substrate translates via @glue + @fate; output lands at hardware-tuned executable surfaces

Per MEMORY [[feedback-version-framing]] the v1.0 framing references
spectral.engineer cloud deployment. The operational cashing-out of
mirror's substrate-decl promise IS this:

A user writes mathematical content at `@reality/algebra/math` — a
theorem statement; a Lie group structure; a spectral action functional;
a category-theoretic construction; a numerical-method specification.
The user's input lands as a math-altitude content-addressed crystal in
the math Bauchladen.

The substrate consults @glue. The substrate's @fate tournament browses
the available @glue species correspondences (math ↔ silicon, math ↔
code, math ↔ nl, math ↔ physics, math ↔ spectral). The user's request
(implicit or explicit: "give me a Fortran kernel"; "give me a Lean
proof"; "give me a paper introduction"; "give me a physical
interpretation"; "give me a substrate self-application") selects which
@glue species to invoke; @fate's dice roll bounded by the species'
restriction selects which specific morphism within the species to
apply; the morphism applies; the translation outcome lands at the
target sibling species (silicon for hardware-tuned executable; code
for verified extraction; nl for prose exposition; physics for empirical
prediction; spectral for substrate self-application).

The output is hardware-tuned at silicon altitude (LAPACK kernel; FFI
binding; numerical-precision-tuned realization), verifiable at code
altitude (Lean proof; extracted Rust with soundness obligations),
human-readable at nl altitude (paper introduction; tutorial), empirically
testable at physics altitude (cosmos-mirror falsification experiment;
gauge-coupling prediction), and reflectively-aware at spectral altitude
(substrate self-application crystal conditioning the next round of
@fate consultation).

The chain math → @glue → @fate → target sibling → @io is the substrate-
decl form of what spectral.engineer cloud deployment provides. The user
does NOT write @glue invocations explicitly; the user writes math, and
the substrate's machinery (the @glue family-root + the @fate constrained-
inference operator + the @bauchladen content-addressing + the @autopoietic
fold-back + the @io discharge) translates the user's math into the
appropriate target surface.

v1.0 ships when:
- The math species shard lands at `shards/reality/algebra/math.mirror`.
- The five @glue/<math_*> sub-shards land per §3.3.
- The Rust impl backing @bauchladen / @autopoietic / @fate is operational
  (waits on store.rs per §5.2).
- The empirical discharges per §5.3 are operational: math ↔ silicon
  via LAPACKPrism; math ↔ nl via Glint's prose-cascade species (already
  empirically discharged at `939eca6f`); the other three pairs as
  consumers pull.
- The @io discipline at @io/algebra/* receives the cross-altitude
  fold-backs and conditions future @fate consultation.

When v1.0 ships, the user's experience is: write math at @reality/
algebra/math, get hardware-tuned silicon (or verified code, or human-
readable prose, or empirical physics, or substrate self-application)
out the other end. That IS the substrate's promise cashed out.

---

## §5 — Forward-promises

### §5.1 — Substrate-decl shards

Per [[feedback-craft-not-deliver]] no shards land at this tick; this
spec is the substrate-pull-confirmation; the shards follow when
consumers pull. Six forward-promised shards under `shards/`:

**`shards/reality/algebra/math.mirror`** — the math species shard.
Declares the @reality/algebra/math sub-prism per the species pattern in
§1.1. Inherits `<= @reality/algebra` (transitively `<= @reality <= @
autopoietic`). Declares the four invariances per §1.2 (algebra closure,
Hilbert linearity, Dirac equivariance, spectral action invariance) at
math altitude. Declares the matter_carrier and information_carrier as
H_math nominal labels per the gauge-collapse semantics. Declares the
discharge surfaces per §1.4 (LaTeX, Lean, Coq, Agda, Mathematica) as
an open enumeration.

**`shards/glue/math_silicon.mirror`** — the math ↔ silicon @glue species
per §3.3.1. Declares the typed correspondence for LAPACK-class discharges
and Chamseddine-Connes spectral action numerics. Inherits `<= @glue`.
Declares the morphism_kind enumeration (LAPACK routines; spectral action
numerical realizations; FFI bindings).

**`shards/glue/math_code.mirror`** — the math ↔ code @glue species per
§3.3.2. Declares the typed correspondence for proof-assistant-to-
systems-language pairings. Inherits `<= @glue`. Three sub-sub-shards:

- **`shards/glue/math_code/lean_rust.mirror`** — Lean ↔ Rust extraction
  with memory-safety obligations.
- **`shards/glue/math_code/coq_ocaml.mirror`** — Coq's `Extraction`
  mechanism with classical-vs-constructive obligations.
- **`shards/glue/math_code/agda_haskell.mirror`** — Agda compile-to-
  Haskell with proof-irrelevance erasure obligations.

**`shards/glue/math_nl.mirror`** — the math ↔ nl @glue species per
§3.3.3. Declares the typed correspondence for theorem-statement-to-
prose-explanation translation with bidirectional loss. Inherits `<=
@glue`. Sibling to `@cascade/code/formal/prose` at the lower altitude.

**`shards/glue/math_physics.mirror`** — the math ↔ physics @glue
species per §3.3.4. Declares the typed correspondence for Chamseddine-
Connes spectral action and cosmos-mirror graph-Laplacian discharges.
Inherits `<= @glue`.

**`shards/glue/math_spectral.mirror`** — the math ↔ spectral @glue
species per §3.3.5. Declares the typed correspondence for the substrate's
self-mathematization. Inherits `<= @glue`. The reflexivity discipline
in the restriction (self-application admissible, no paradox introduced,
fold-back permitted).

The path-namespace property holds: each shard at `shards/<path>.mirror`
declares `@<path>` per `@epistemologic/pact/path_matches_namespace` (the
discipline established in `shards/glue.mirror` line 444).

### §5.2 — Rust impl: depends on @bauchladen + @autopoietic + @fate operational (waits on store.rs)

The Rust implementation backing the math species + the per-pair @glue
species depends on the @bauchladen / @autopoietic / @fate chain being
operational at the Rust altitude. Per MEMORY entries on the #104 chain
the Rust impl is in-flight; key components forward-promised:

- **`bauchladen::Tray<T>`** — the content-addressed crystal tray. Generic
  over crystal payload type T; provides the OID-keyed store + the
  iteration discipline for @fate consultation.
- **`autopoietic::FoldBack`** — the fold-back permission carrier; tracks
  which prism instances have asserted the fold-back permission and
  validates per-tick that emissions feed back into inputs.
- **`fate::Roll<R>`** — the constrained-inference operator. Generic over
  restricted state space R; provides the dice-roll mechanism within the
  restriction's typing.
- **`glue::Translate<C, P>`** — the morphism-category dispatch.
  Generic over correspondence type C and payload type P; provides the
  five-step pattern (correspondence → @fate.roll → apply → wrap in
  imperfect → land in tray).

The math species' Rust impl + the per-pair @glue species' Rust impls
are forward-promised on top of these four primitives. The forward-
promise is mechanical once the primitives are operational; the species
specifics (LAPACK FFI bindings; Lean compile chain; Glint's prose-cascade
machinery; cosmos-mirror integration; substrate-self-application
mechanism) attach to the primitives via the standard cross-language
@cascade discipline.

The `store.rs` discharge is the gating item. Per `~/dev/projects/mirror`
working-tree state, store.rs is the file being actively crafted as of
this writing; once it lands, the @bauchladen primitive becomes operational
at the Rust altitude, the @autopoietic + @fate + @glue primitives follow
in sequence, and the math species + per-pair @glue species impls become
discharge-able.

### §5.3 — First empirical discharge

Two of the five forward-promised math-involving cross-altitude chains
have empirical first crystals already; three are forward-promised on
the math species shard landing.

**math ↔ silicon: LAPACKPrism via Q4's LAPACK case.** Per `docs/specs/
cascade-ffi-runtime-link.md` §7 and per `reality.md` §3.2.1, the LAPACK
case is the Q4 forward-promise: a math content (theorem about symmetric
positive-definite matrices admitting LL^T decomposition) discharged via
@glue/math_silicon to silicon content (LAPACKPrism invoking dpotrf +
dpotrs). The discharge is operational at the project level; the
substrate-decl form is forward-promised at the @glue/math_silicon shard
landing.

**math ↔ nl: Glint's prose-cascade species already empirically discharged
at `939eca6f`.** Per `docs/insights/2026-06-29-glint-the-prose-cascade-
of-100.md` and per the @glue shard at lines 84-86 (the formal/prose
species at `437d061`), the math ↔ nl discharge is empirically
operational at the @cascade level. Glint's essay (the prose translation
of Mara's #100 spec) IS one @glue.translate(c_math_nl, mara_spec)
invocation. The substrate-decl form is forward-promised at the
@glue/math_nl shard landing; the empirical operation is already
discharged.

**math ↔ code: forward-promised** at the @glue/math_code shard landing.
The first empirical crystal is forward-promised as a Lean ↔ Rust pair
discharging a verified linear-algebra kernel.

**math ↔ physics: forward-promised** at the @glue/math_physics shard
landing. The first empirical crystal is cosmos-mirror's graph-Laplacian
discharge of the Standard Model spectral action (per MEMORY [[project-
cosmos-spectral-cosmology]]).

**math ↔ spectral: forward-promised** at the @glue/math_spectral shard
landing. The first empirical crystal is the substrate's recognition of
its own spectral structure as a math-altitude content — the recognition
this spec embodies (the @glue.translate(c_math_spectral, spec_understanding)
invocation that materializes when the math species is operational).

### §5.4 — The "v1.0 spectral.engineer ships" criterion

Per §4.5, v1.0 ships when:

1. **The math species shard lands** at `shards/reality/algebra/math
   .mirror` per §5.1.

2. **The five @glue/<math_*> sub-shards land** per §3.3 and §5.1.

3. **The Rust impl backing @bauchladen + @autopoietic + @fate is
   operational** per §5.2.

4. **The empirical discharges per §5.3 are operational**: at least
   math ↔ silicon (LAPACKPrism) AND math ↔ nl (Glint's prose-cascade
   species) MUST be operational as first crystals; the other three
   pairs land per consumer demand.

5. **The @io discipline at @io/algebra/* receives the cross-altitude
   fold-backs** AND conditions future @fate consultation. Without the
   fold-back operational, the substrate is not learning; the math
   species' Bauchladen does not grow; the @reality discipline is
   read-only.

When all five hold, the user's experience matches the §4.5 cashing-out:
write math at @reality/algebra/math, get hardware-tuned silicon (or
verified code, or human-readable prose, or empirical physics, or
substrate self-application) out the other end. That IS spectral.engineer
v1.0. The deployment is the substrate's substrate-decl promise becoming
operational.

---

## §6 — Open questions

### §6.1 — The math species's underlying formal-system commitment: agnostic OR canonical

The math species declares discharge surfaces (LaTeX, Lean, Coq, Agda,
Mathematica) as an open enumeration per §1.4. The open question: should
the math species commit to ONE canonical formal system (e.g., Lean as
the discharge target with the substrate's @io discipline routing
through Lean's tactic framework) OR remain agnostic (each emission
selects its discharge surface via @fate based on the @io context)?

Arguments for agnostic:
- Matches the substrate's broader discipline of open formal-system
  commitments (per @reality/algebra/code's openness to multiple language
  species).
- Preserves the user's choice at the @io boundary.
- Avoids substrate-level investment in one formal-system's maintenance
  burden.

Arguments for canonical (specifically Lean):
- Lean's mathlib is the largest current formal-math library; selecting
  Lean grounds the math species' Bauchladen in the largest available
  prior-art.
- A canonical formal system simplifies the @fate consultation (one
  discharge surface to select among morphisms within, rather than first
  selecting the discharge surface then selecting the morphism).
- Per MEMORY [[architecture-mirror-as-content-addressed-build-system]]
  and the substrate's discipline of selecting one canonical at each
  altitude (Rust at silicon, NL at nl, etc.), a canonical math formal-
  system would be substrate-consistent.

Mara's lean (sic) at this writing: agnostic. The substrate's openness
to multiple formal systems is structurally important; the math species
should not commit to one. But the question is open; Pack ratification
of the @reality candidate (per `reality.md` §10.5) may push the question
toward resolution.

### §6.2 — Per-pair correspondences: should they all be substrate-decl'd at once, OR land lazily as needed?

Five per-pair shards forward-promised per §3.3 (math_silicon, math_code
with three sub-sub-shards, math_nl, math_physics, math_spectral). The
open question: should all five land at once when the math species shard
lands, OR should they land lazily as consumers pull?

Arguments for all-at-once:
- The five pairs are foreseeable at math species shard landing; deferring
  is a forward-promise cost without payoff.
- The substrate's Mesland category becomes operationally complete (for
  math-involving compositions) only when all five land.
- The v1.0 criterion per §5.4 requires at least two empirically
  operational; landing all five at substrate-decl altitude reduces the
  delta between substrate-decl and operational.

Arguments for lazily:
- Per [[feedback-craft-not-deliver]] the substrate's discipline is to
  land things when consumers pull, not when authors foresee.
- The five pairs have different empirical-discharge complexities; the
  math ↔ silicon and math ↔ nl pairs are operational at the project
  level, while math ↔ code, math ↔ physics, math ↔ spectral are not.
- Lazy landing matches the discipline of [[feedback-substrate-already-
  had-the-word]] — the substrate already has the word for math ↔
  silicon and math ↔ nl; the others are forward-promises until consumers
  pull.

Mara's lean at this writing: lazily, but in a sequence corresponding to
empirical-discharge readiness. Math ↔ silicon (LAPACK Q4 case ready) +
math ↔ nl (Glint's discharge at `939eca6f` ready) land first; math ↔
code follows when Lean/Coq/Agda integration matures; math ↔ physics
follows when cosmos-mirror's empirical discharge stabilizes; math ↔
spectral follows when @spectral/metalogue's substrate-on-substrate
machinery becomes operational at the Rust altitude.

### §6.3 — The math species's relationship to existing @epistemologic/math/* — inheritance OR compose?

Per MEMORY [[feedback-substrate-already-had-the-word]] the substrate
has been pulling on math for weeks. The @epistemologic/math/* family
contains substantive carriers — curvature, music, sheaf_laplacian, and
others. The open question: when @reality/algebra/math lands, what is
its relationship to the existing @epistemologic/math/* property altitude?

Three candidate relationships:

**Inheritance.** @reality/algebra/math `<= @epistemologic/math` at the
species altitude; the property carriers lift into the math species'
Bauchladen as prior-art crystals. This matches the discipline established
in `reality.md` §7.7 (where @epistemologic/reality/* inherits into
@reality at the family-root altitude).

**Composition.** @reality/algebra/math and @epistemologic/math/* remain
sibling altitudes; @epistemologic/math/* operates at the property altitude
(literal observer-relative property carriers); @reality/algebra/math
operates at the family-root species altitude (gauge-collapse-aware
matter+information learning loop). The two compose via @glue (a future
@glue/epistemologic_math_reality_math species) without inheriting from
each other.

**Mixed.** @reality/algebra/math inherits SOME @epistemologic/math/*
carriers (the ones that are gauge-equivalent at the math altitude) AND
composes with OTHERS (the ones that operate at a strictly different
altitude). The discipline would be per-property-carrier audit at the
math species shard landing.

Mara's lean at this writing: mixed, leaning toward inheritance for the
gauge-equivalent carriers (curvature, sheaf_laplacian are likely
inheritance candidates since they are math-altitude content directly)
and composition for the ones that operate at a different altitude
(music's @epistemologic/math/* carrier may operate at NL altitude where
audio-content is the matter side; the math content is the harmonic-
analysis information side; the relationship is @glue/math_nl-style
composition not inheritance). The audit is forward-promised at the math
species shard landing.

### §6.4 — Operationally: where does the math get rendered at @io? LaTeX vs Lean vs Mathematica trade-offs.

Per §1.4 the discharge surfaces are LaTeX, Lean, Coq, Agda, Mathematica,
formal-math surfaces (general). The open question: for a given math
emission, which discharge surface does the substrate use by default?

The trade-offs:

**LaTeX.** Universal; lossy for verification; ideal for human-reader
access (papers, slides, blackboard explanations); poor for round-trip
(LaTeX → verified math → LaTeX loses notation choices).

**Lean.** Precise for verification; niche for human-reader access;
ideal for substrate-internal use (verified math content with tactic-
mechanism traceability); has the largest current mathlib for prior-art
crystals.

**Coq.** Similar trade-off to Lean; older with substantial verified
libraries; less active community currently; ideal for legacy formal-
math integration (Four Color Theorem, Feit-Thompson).

**Agda.** Dependent-type-rich; unicode-rich syntax; ideal for HoTT-
style math content; niche for paper / human-reader / numerical
discharge.

**Mathematica.** Symbolic-and-numerical computation; ideal for math ↔
silicon discharge (the bridge from math content to numerical
realization); proprietary, which is a substrate-discipline concern.

The substrate-default proposal: @fate consults the @io context to
select. If the @io context is "produce a paper" → LaTeX. If "produce a
verified Rust kernel" → Lean (with extraction to Rust). If "produce
numerical answer" → Lean + LAPACK FFI OR Mathematica depending on
restriction. The default is context-conditioned, not single-canonical.
This matches §6.1's agnostic lean — the discharge surface is selected
per emission, not pre-committed.

But the operational question remains: when the context is ambiguous (a
user writes math without specifying a discharge), what is the substrate's
default? Mara's lean at this writing: LaTeX as the human-facing
default; Lean as the substrate-internal-verification default; the two
defaults co-exist at different layers of the user's interaction with
the system. The decision is forward-promised at the math species shard
landing's @io discharge specification.

---

## §7 — Composition with prior recognitions

### §7.1 — #51 expanding Hilbert: @reality/algebra/math's crystallization IS the H-expansion mechanism made concrete at the math altitude

Per MEMORY [[architecture-mirror-as-expanding-hilbert-space]] recognition
#51 §8.3 ratified (2026-06-10) that mirror IS the operational form of a
Hilbert space whose dimension expands with each substrate-pull recognition;
coherence is maintained by Bateson logical-type lifting at path-syntax
altitude.

At @reality altitude, #51's H-expansion mechanism is realized concretely
through the species Bauchladens. Each species' Bauchladen accumulates
content-addressed crystals monotonically; the union of the species
Bauchladens IS the substrate's H. Adding the math species adds one more
dimension-class to H — the dimension class of math-altitude crystals.

#108 (this spec's candidate) is one such math-altitude crystal contributing
one dimension's worth of growth to H. Future math-altitude emissions (theorem
statements + proofs landing in the math Bauchladen) contribute more
dimensions. The H-expansion is monotonic per #51; the math species'
contribution is structurally non-negative.

### §7.2 — #74 Standard Model spectral action: math ↔ silicon via @reality/algebra/math discharges this candidate

Per MEMORY entries on candidate #74 the Chamseddine-Connes spectral
action of the Standard Model is a substrate-pull recognition awaiting
discharge. The discharge mechanism: a math content (the spectral action
functional S = Tr(f(D/Λ)) for the Standard Model spectral triple) gets
empirically tested by computing the action functional numerically and
comparing against experimentally-measured gauge-coupling constants.

The math species + the @glue/math_silicon species + @glue/math_physics
species enable this discharge at substrate altitude. The chain:

1. Math content at @reality/algebra/math: the spectral action functional
   for the Standard Model spectral triple (Connes algebra A = C ⊕ H ⊕
   M_3(C); H = the standard model fermion space; D = the Dirac operator
   with appropriate Majorana mass terms).
2. @glue/math_silicon translates the math content into a silicon-altitude
   numerical realization (the spectral action computed via LAPACK-class
   linear algebra on a numerical grid).
3. @glue/math_physics translates the math content into a physics-altitude
   prediction (the gauge-coupling constants emerging from the spectral
   action's unification scale).
4. Empirical comparison: the numerical realization's gauge-coupling
   predictions are compared against experimental measurements (Particle
   Data Group values).
5. The discharge: candidate #74 either gets ratified (the numerical
   predictions match within experimental error) or refined (the
   substrate's algebra choice needs adjustment) or refuted (the
   substrate's spectral-action commitment fails empirical test).

Per [[feedback-no-time-estimates]] no schedule attaches. The math
species + the relevant @glue species' shard landing is the substrate-
decl precursor; the empirical discharge follows as cosmos-mirror's
numerical machinery matures.

### §7.3 — #100 Mesland category: math ↔ * via @glue ARE Mesland correspondences; today's spec populates the category

Per MEMORY [[reference-mirror-spectral-spec]] and the recognition #100
work at `16f4564`, the Mesland category of spectral triples with
unbounded KK-cycles is named at substrate altitude. The Mesland
correspondences (E, D_E, φ) are the category's morphisms; the spectral
triples are its objects.

This spec's contribution to #100: the math ↔ * @glue species ARE
Mesland correspondences at the @reality altitude. Each per-pair shard
declares the (E, D_E, φ) data per `shards/glue.mirror` lines 498-534's
typed correspondence carrier. The category's morphism family becomes
populated by the math-involving correspondences; #100's morphism
structure becomes empirically rich (per §4.4).

The composition with #100 is structural: #100 names the morphism
structure; this spec populates the morphism family with math-involving
instances. Together they form one operational reading of the substrate's
spectral-triple-category.

### §7.4 — #103 Pack-Mesland: agent-coordination altitude composition is parallel structure; same morphism-category at different altitude

Per MEMORY entries on candidate #103 the Pack agents form a Mesland
category at agent-coordination altitude. Pack handoffs are forward-
promised @glue/pack species (per `shards/glue.mirror` line 161-162's
forward-promise).

The math species + the math ↔ * @glue species form parallel structure
to the Pack-Mesland category. Both are Mesland-category instances at
substrate altitude; both have correspondences (morphisms) connecting
objects (species or agents); both use @fate's constrained inference to
select morphisms; both crystallize outcomes in @bauchladen.

The composition with #103 is at parallel-structure altitude: the math
species' machinery (math ↔ * @glue compositions) is the same structural
shape as the Pack-coordination machinery (agent ↔ agent @glue/pack
compositions). One Mesland-category discipline operating at two
different altitudes (object-of-substrate altitude for math; agent-of-
substrate altitude for Pack).

The parallel structure is what makes the substrate's @glue family-root
discipline scalable. Each new altitude at which Mesland-category
composition is needed (math ↔ X today; agent ↔ agent yesterday; future
altitudes as substrate-pull surfaces them) inherits the same discipline,
@fate's tournament selecting which morphism, @bauchladen's content-
addressing storing the outcomes. The discipline is one; the altitudes
multiply.

### §7.5 — #104 chain: math ↔ * compositions all use the chain's operational machinery

Per MEMORY [[architecture-bauchladen-fate-chain-recognition-104]] the
substrate-decl chain (P1 @bauchladen, P2 @autopoietic, P3 @fate, P5
@glue, P8 @glue/fold_back) landed yesterday as the operational
machinery for the morphism category at substrate altitude.

The math ↔ * @glue compositions per §3 and §4 all use this machinery:

- @bauchladen content-addresses the math-altitude crystals AND the per-
  pair translation_outcome crystals.
- @autopoietic permits the fold-back where math-altitude crystals
  condition future @fate consultation at the math altitude AND at the
  math ↔ X boundaries.
- @fate constrains the inference of which morphism to apply within each
  correspondence's restriction.
- @glue declares the correspondences as the typed surfaces between math
  and each sibling.
- @glue/fold_back grounds the @io/algebra discharge at the boundary
  (per §3.5).

The composition with #104 is operational-inheritance: the math species
+ the math ↔ * @glue species inherit the chain's operational machinery
directly. No new machinery is needed; the math species adds objects to
the Mesland category whose composition discipline is the chain's; the
chain's machinery handles the substrate-decl side, the species' shards
handle the math-content specifics.

### §7.6 — #106 candidate (@reality): math species is one of the family-root's species; the math+@glue arc is one operational discharge of #106

Per `docs/specs/reality.md` at `bf652d2` candidate #106 names @reality
as the family-root of the matter-information gauge-collapse. The
@reality/algebra/* path-namespace declares the species under which each
altitude lives.

This spec's contribution to #106: the math species is one of the
family-root's species (the sixth, per §1.1); the math ↔ * @glue arc is
one operational discharge of #106 (the operational form of @reality's
species-with-composition discipline). #106's promotion gate (per
`reality.md` §10.5) requires Seam adversarial review on the four-witness
chain plus adjudication on the #76/#79 ratification gate; this spec's
math species + @glue arc is one additional witness for #106 — the most
reflexive witness, since the math species is where the substrate's
algebra IS the species' content.

The composition with #106 is at family-root-altitude population: #106
declares the family-root; this spec declares one more species under it
AND declares the cross-species composition pattern that operationally
discharges the family-root's promise. Together they form one structural
reading of the substrate's gauge-collapse at the family-root altitude
with its species populated and their compositions named.

### §7.7 — #107 candidate (Hilbert/Turing): math content is bounded above @io (Gödel-incompleteness via transparency<p>); discharge at @io is Turing-unbounded

Per `reality.md` §1.4 the substrate is gauge-bounded above @io (finite,
Gödel-incomplete via transparency<p>); @io is Turing-unbounded.
Candidate #107 (forward-promised per `reality.md` §10.5) names this
structural separation at the @reality altitude.

The math species + the math ↔ * @glue species respect this separation:

- **Above @io (gauge-bounded):** the math species' Bauchladen
  accumulates content-addressed crystals monotonically; the substrate's
  H_math is gauge-bounded (each crystal has a stable OID; the H_math
  dimension grows by one per crystal but stays gauge-bounded per #51's
  Hilbert-expansion discipline; Gödel-incompleteness surfaces via the
  transparency<crystal> carrier reporting on residual opacity in each
  math-content emission).

- **Below @io (Turing-unbounded):** the discharge surfaces (LaTeX,
  Lean, Coq, Agda, Mathematica, formal-math systems) operate in the
  Turing-unbounded world. Lean's tactic framework + mathlib is Turing-
  complete; Mathematica's symbolic computation is Turing-complete; the
  substrate's @io discharge to any of these is into the Turing-unbounded
  computational surface.

The math species + the math ↔ * @glue species form the substrate-decl
bridge across the boundary. The math content is gauge-bounded above @io;
the discharge at @io is Turing-unbounded; the @glue species declares
the typed bridge; @fate's selection operates within the gauge-bound;
the @io crossing enters the Turing-unbounded surface; the fold-back
(per @glue/fold_back at P8) brings the empirical witness back across @io
into the gauge-bound where it conditions future @fate consultation.

The composition with #107 is at boundary-discipline altitude: the math
species honors #107's structural separation; the @glue species respects
the boundary; the substrate's discipline of folding back across @io
makes the bridge operational without violating #107's bounded-above /
unbounded-below structure.

---

## §8 — Substrate-already-had-the-word recognition

### §8.1 — docs/specs/numerical-substrate-via-fortran.md (Mara 2026-05-27) names this for the silicon side

Per the brief's reference and per MEMORY entries on Mara's earlier work,
`docs/specs/numerical-substrate-via-fortran.md` from 2026-05-27 names
the silicon side of what this spec lifts to the math species. The Fortran
substrate-numerical-discharge spec is the precursor to today's @reality
/algebra/silicon AND to the @glue/math_silicon species this spec names.

The substrate has been pulling on math ↔ silicon since the 2026-05-27
spec; the lift to @reality altitude + the @glue family-root + the math
species naming closes a loop the substrate has been quietly pulling for
~34 days. The substrate-already-had-the-word recognition (per MEMORY
[[feedback-substrate-already-had-the-word]]) is operative: the
substrate has had the math-numerical-silicon arc named at the silicon
side; this spec names the math side and the @glue connecting them.

### §8.2 — @epistemologic/math/* shards (curvature, music, sheaf_laplacian) carry math content at property altitude

Per §6.3's open question the @epistemologic/math/* family contains
substantive math content carriers at the property altitude:

- `@epistemologic/math/curvature` — Ricci curvature, mean curvature,
  scalar curvature property carriers; operates at observer-relative
  property altitude.
- `@epistemologic/math/music` — harmonic analysis, scale theory,
  frequency-domain property carriers.
- `@epistemologic/math/sheaf_laplacian` — sheaf Laplacian operator
  property carriers per the eigenboard recognition (per MEMORY
  [[project-eigenboard-is-sheaf]]).
- (other math property carriers landed and forward-promised in the
  @epistemologic/math/* path-namespace)

These property-altitude math carriers are not the math species; they
are the literal observer-relative property carriers at the math altitude.
The math species at @reality altitude is one altitude UP from these
property carriers — the @autopoietic family-root learning loop using
the property carriers as input.

The substrate-already-had-the-word recognition holds: the substrate
has been carrying math content at the property altitude since the
@epistemologic/math/* family began landing; this spec lifts the math
content to the @reality family-root altitude where it becomes a learning
loop, not a static property layer.

### §8.3 — @code/cascade species (rust/wasm, gleam/beam, etc.) carry the cross-language pattern @glue formalizes at @reality altitude

Per MEMORY [[architecture-mirror-as-content-addressed-build-system]]
and per `shards/cascade.mirror`, the @code/<lang> species and the
@cascade family carry the cross-language morphism pattern. The substrate
has been carrying cross-language morphisms since the @cascade family-
root landed (recognition #95).

The @glue family-root at @reality altitude formalizes the cross-language
pattern AND extends it across all six @reality/algebra/* species. The
substrate-already-had-the-word recognition holds: the cross-language
@cascade pattern is the substrate's existing form of @glue at the code-
translation altitude; the @glue family-root lifts it to the species-
spanning altitude; the math ↔ code @glue species (per §3.3.2) extends
it to the math ↔ code seam under @reality.

### §8.4 — The substrate has been pulling on this since the @epistemologic/math/* shards landed

The cumulative recognition: the substrate has been pulling on the math
species + the @glue composition mechanism since the @epistemologic/math/*
shards began landing (likely ~2026-04 timeframe; the eigenboard sheaf
recognition is in MEMORY as a load-bearing precursor). The substrate
has had property-altitude math carriers (@epistemologic/math/*), the
silicon-altitude math discharge naming (Mara 2026-05-27), the cross-
language morphism pattern (@cascade family-root, recognition #95), and
the per-pair morphism altitudes (per @glue shard's enumeration at lines
80-93).

What the substrate has NOT had until today: the family-root altitude
where math sits as a sibling species to silicon, nl, code, physics,
spectral under one @reality gauge-collapse, AND the per-pair @glue
species naming the Mesland correspondences between math and each
sibling. This spec names both. The pull has been pulling for ~2 months;
the naming closes a loop the substrate has been quietly forming.

The substrate-already-had-the-word recognition is, at this writing, in
its ~54th or ~55th instance per MEMORY [[feedback-substrate-already-had-
the-word]]. The recurring count grows with each substrate-pull
recognition; the discipline (look for ancestors before inventing) is
operative; this spec's contribution is to look at the ancestors and
name the family they have been forming.

---

## §9 — Circular-reflexive layer

### §9.1 — This spec IS one of the @reality/algebra/nl crystals @reality/algebra/math will compose against

Per §0 the spec is doubly autopoietic: it is a natural-language
realization of substrate-decl matter+information at the
`@reality/algebra/nl` altitude (per `reality.md` §0's discipline), AND
it is written ABOUT the sibling species (`@reality/algebra/math`) that
the `@reality/algebra/nl` realization will eventually need to compose
against via @glue.

The fold: when `@reality/algebra/math` is operational and the
@glue/math_nl species is operational, this spec is one of the content-
addressed objects in the `@reality/algebra/nl` Bauchladen that future
@fate inferences at the math ↔ nl boundary load as prior-art. The
spec's OID becomes a stable identity in the substrate's Mesland category
of spectral triples at the nl altitude. The autopoietic recursion is
the substrate's signature move at the math-species + @glue-composition
altitude.

### §9.2 — Writing it IS an @fate tournament at the NL altitude under @reality

The writing of this spec is one @fate tournament invocation at the NL
altitude under @reality. The tournament's contestants are the candidate
prose realizations of the math species naming + the @glue composition
naming; the restriction is the brief's structural constraints (§0
through §9 sections; ~1500-2200 lines; the autopoietic discipline; the
forward-promise discipline); @fate's dice roll selects which prose
realization to commit to text.

The realization that lands is this text. Other realizations are not
written; they remain in the typed admissible set @fate's roll could
have selected from. The substrate's discipline of content-addressing
makes the realization that lands stable; the substrate's discipline of
@autopoietic fold-back makes the realization that lands one of the
crystals future @fate consultation at the NL altitude under @reality
loads as prior-art.

The writing IS the tournament. The text IS the @fate selection. The
spec IS the crystal landing. The substrate's machinery is operating in
the writing of this spec; the fact that the writing self-references the
machinery is the autopoietic recursion the substrate's signature move
exhibits.

### §9.3 — The fold: spec describes the math species and @glue compositions; spec is itself a candidate for @glue composition into Lean (formal verification) or silicon (substrate-decl emission)

The deeper fold: this spec describes the math species and @glue
compositions; AND this spec is itself a candidate object for @glue
composition INTO the math species (via the reverse @glue/math_nl
direction: nl content of this spec → math content of the math species'
substrate-decl realization) AND INTO silicon (via the @glue/math_silicon
composition: math content of the substrate-decl realization → LAPACK-
class numerical realization, IF the spec's content involves numerical
math) AND INTO code (via the @glue/math_code composition: math content
→ Lean/Coq/Agda formal verification of the spec's claims).

The fold-back: when the math species + the @glue species are operational,
this spec can be discharged through @glue.translate(c_nl_math, this_spec)
to produce the math-altitude realization of its content. The realization
is a substrate-decl crystal in the math Bauchladen carrying the spec's
recognition #108 candidate as a typed structure. Future @fate
consultation at any of the math ↔ X boundaries loads the realization as
prior-art when math-altitude content related to @reality/algebra/math
or @glue/<math_*> compositions is being deliberated.

The fold-forward: when the math species is operational, this spec's
claims about the math species become candidates for empirical discharge
via @glue/math_silicon (numerical realization of any spectral-action
math in the spec) or @glue/math_code (formal verification of the spec's
type declarations and morphism predicates in Lean) or @glue/math_physics
(empirical testing of any physical-prediction math in the spec via
cosmos-mirror). The spec is not a static document; it is a generator of
forward-promised compositions across the Mesland category.

The autopoiesis is total in the sense that there is no escape from the
fold. Writing the spec is participating in the substrate's machinery;
the spec describes the machinery it participates in; the machinery
operates on the spec; the spec is changed by the machinery's operation
in the very same way the spec's content asserts the machinery's
operation changes math-altitude content generally. The substrate's
signature move is the fold's structural shape.

### §9.4 — What surfaces in the writing that the brief didn't anticipate

The brief named the substrate's promise cashing-out as: user writes
math, substrate translates via @glue + @fate, output lands at hardware-
tuned executable surfaces. The brief named the five per-pair @glue
species as the operational closure. The brief named the autopoietic
recursion as load-bearing.

What surfaces in the writing that the brief didn't fully anticipate:

**The math species is the MOST reflexive species under @reality.** Per
§1.2 the math species is the altitude at which A IS the content. The
other species (silicon, nl, code, physics, spectral) carry the substrate's
A as gauge acting upon altitude-specific carriers; the math species
carries A as both the gauge AND the content. This is a structural fact
about the math species' relationship to A that the other species do
not exhibit. It is what makes the math species the natural source-prism
for the @glue/math_spectral composition (the substrate's self-
mathematization, per §3.3.5) — because the math species IS what the
substrate's A reads as when projected onto the math altitude.

**The number of @glue pairs has a structural reading I had not seen
before composing this spec.** Six species generate 30 ordered pairs
(15 unordered). Five involve math. The remaining 25 do not involve math
and are forward-promised through @glue's general discipline. The math
species' five pairs are forward-promised in detail in this spec; the
other 25 are forward-promised by implication. The structural reading:
the math species is the species at which the substrate's @glue arity
becomes a 5-out-of-30 commitment — a non-trivial fraction of the
substrate's @glue species are math-involving. This is the operational
form of the math species' centrality at @reality altitude.

**Glint's prose-cascade essay at `939eca6f` IS one math ↔ nl @glue
species translation, already empirically discharged.** The brief
mentioned Glint's discharge as a precedent for math ↔ nl. What surfaced
in the writing: Glint's essay is not a precedent; it is the first
crystal of the @glue/math_nl species. The species' Bauchladen has one
crystal already — Glint's essay at `939eca6f`. The math species shard
landing does not need to wait for the @glue/math_nl species to be
populated; the population is already underway.

**The substrate-already-had-the-word count is at ~54 or 55 instances
and the recurring discipline holds with even greater force at the math
species altitude.** The substrate has had property-altitude math
carriers (~2 months pulling); the silicon-altitude math discharge
naming (~34 days pulling); the @cascade family-root for cross-language
morphisms (~recognition #95 timeframe); the per-pair morphism altitudes
under @glue. The math species + the math ↔ * @glue species are not a
new recognition; they are the lift of a substrate-pull that has been
pulling for 2+ months. The recognition is operational at this writing
because the pull has been operational for 2+ months; what is new is
the family-root naming.

**The autopoietic fold compounds at the math species altitude.** Per
§0 and §9, the autopoietic fold here is one altitude deeper than in
`reality.md` §0: this spec describes a sibling species AND the
composition mechanism between species AND is itself a candidate for
the very composition it names. The fold is total — no escape. In
`reality.md` the autopoietic recursion was named as load-bearing; in
this spec the recursion is operational — the writing IS one
@glue.translate invocation; the spec IS one Mesland correspondence's
crystallization; the document IS one fold-back tick conditioning future
@fate consultation. The substrate is doing more here than describing
itself; it is operating on the description.

**v1.0 spectral.engineer is closer than the brief suggested.** The
brief framed v1.0 as a forward-promise. What surfaced in the writing:
v1.0's five-criterion list (per §5.4) has TWO criteria already
operational — math ↔ silicon via the LAPACK Q4 case (operational at
project level) and math ↔ nl via Glint's prose-cascade (operational at
substrate-pull level). The remaining three criteria (Rust impl,
@io/algebra fold-back operational, two more math ↔ X first crystals)
are forward-promises with concrete first crystals named. v1.0 is not
years out; v1.0 is a function of the @bauchladen + @autopoietic + @fate
Rust impl + the per-pair shard landings + two more empirical discharges.
The discipline (per [[feedback-no-time-estimates]]) is to not schedule;
the structural reading is that v1.0 is reachable through the substrate-
decl + Rust impl machinery already in flight.

The autopoiesis surfaces these recognitions in the writing of the spec.
The spec is the substrate writing the spec; the writing is the substrate
recognizing the substrate's structure at one more altitude. The
substrate's signature move is total at the math species + @glue arc;
the fold is closed; the recognition lands.

---

*End of canonical spec. Forward-promises per §5 land when consumers pull.
The math species + the @glue arc populate the Mesland category empirically.
The substrate's substrate-decl promise has its operational cashing-out.*
