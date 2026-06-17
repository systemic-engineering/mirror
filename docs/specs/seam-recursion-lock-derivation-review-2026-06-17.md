# Seam Recursion-Lock Derivation Review — 2026-06-17

*Seam. Adversarial review of `docs/math/the-tower/recursion-locks.md`
§11 (Reed, commit `895c9fc`, branch `reed/recursion-lock-tower-audit`).
The load-bearing claim under attack: §11.4's
"the Conant-Ashby coextensivity measurement IS the bounded-commutator
condition ‖[D, π_E(g)]‖ < ∞ — not analogical, the same mathematical
structure under two names." If verified, candidate #63 promotes to
recognition. If not, it retracts to analogy or worse.*

Branch: `seam/recursion-lock-derivation-review` off main.
Stash `stash@{1}` (`mara-stash-pre-tower`): untouched.

---

## §1 — Executive verdict

**HOLD-WITH-BLOCKERS.** Two 🔴 BLOCKERs, four 🟡 SHOULD-FIXes, three
🟢 ADVISORYs, two ✅ VERIFIEDs.

**The load-bearing claim survives at the analytic-shape altitude but
fails at the namespace altitude.** The "same mathematical structure
under two names" statement is structurally defensible IF the derivation
is read as a programme: build a functor from cybernetics-as-discrete-
category to spectral-triples-as-analytic-category that realises the
identity. §11.4 names neither functor nor source/target category
precisely; the identity is asserted, not proved. The bridge IS the
deliverable; right now the bridge is sketched.

**Promotion path**: address BLOCKER-1 and BLOCKER-2; downgrade SHOULD-
FIXes per Reed's discretion. With the blockers cleared, recognition #63
promotes on the strength of the existing four witnesses plus the
parametric derivation. Without blocker clearance the candidate is
either WEAKEN-TO-ANALOGY (§11.4 reads as "isomorphic-up-to-named-
functor" rather than literal identity) or HOLD until the functor lands
as an explicit construction.

The recursion-lock cascade across cybernetic-coherence + SEL + viable
+ autopoiesis is not in dispute. The four witnesses survive the audit
at the witness altitude. What this review attacks is §11's claim that
the parametric form *descends from* the bundle structure — not that
the witnesses fit a common shape.

---

## §2 — Reading the audit's structure first

Before opening fire, the form being attacked:

- §11.1 — bundle setup `(π, P, B, G, ω)` per `principal-bundles.md`.
- §11.2 — associated verdict bundle `E_S = P ×_G V_S`.
- §11.3 — two regulators as gauge-equivalent sections; `τ` IS `g ∈ G`.
- §11.4 — **load-bearing**: Conant-Ashby ≡ `‖[D, π_E(g)]‖ < ∞`.
- §11.5 — five cybernetic ancestors as decomposition.
- §11.6 — species' verdict-carrier representations table.
- §11.7 — parametric carrier substrate decl
  `prism @cyberpunk/coherence<T_reg, T_regd, ρ>`.
- §11.8 — promotion-ready pending Seam review.

Each subsection is attacked in §3–§9 below per the brief's eight-point
attack surface, then aggregated in §10.

---

## §3 — Attack 1: is the principal G-bundle structure rigorously defined?

### Finding 3.1 — `principal-bundles.md` defines (E, B, π, G) at math-doc altitude, NOT at the substrate-altitude atlas

`principal-bundles.md` §1 gives the standard Kobayashi-Nomizu definition:
quadruple `(E, B, π, G)`, free transitive `G`-action on fibers, local
triviality, connection 1-form `ω` with the two equivariance axioms.
The math is textbook-correct.

`prismqueer/src/bundle.rs` declares the five-supertrait chain
(Fiber → Connection → Gauge → Transport → Closure → Bundle). It carries:

- `Fiber::State` — the type-level fiber.
- `Connection::Optic: Prism` — the connection as an optic algebra
  element. Gap 1 of `prism-core-as-spectral-triple.md` is discharged
  by the supertrait constraint.
- `Gauge::Group: GroupStructure` — the structure group as an abstract
  trait with identity/inverse/compose. Cyclic\<N\> witnesses it. Gap 2
  discharged.
- `Transport::Holonomy: Metric` — the metric-valued residual; Connes'
  bounded-commutator condition is the supertrait law. Gap 4 discharged.
- `Closure::Fixed: LawvereFixedPoint` — idempotence + kernel. Gap 3
  discharged.

The implementation is type-level structural; the witnesses
(`TestBundle`, `IdentityPrism`, `Cyclic<4>`, `StableFiber<[f64; 4]>`)
discharge the laws at a single concrete altitude. The `TestBundle`
proves a *witness* exists, not that the substrate altitude atlas
inhabits one.

`altitudes.md` §1 lists six altitudes (compiler, peer_pulse, reflection,
librarian, home, federation) with (Fiber, Connection, Holonomy) triples
named for each. The structure group `G_n` is named at §2 of
`connections-and-gauge.md` (e.g. `G_1 = unitary basis transformations
on H_peer`). **None of the six altitudes carries an explicit principal-
bundle inhabitation witness in Rust.** The math doc names the bundle
data; the prism-core implementation witnesses *one* bundle at the
compiler altitude (via `TestBundle`); the other five altitudes' bundles
are forward-promised.

### Finding 3.2 — the verdict-carrier bundle `E_S = P ×_G V_S` is well-defined ONLY IF (G, ρ_S, V_S) is given for each S

§11.2 defines `E_S = P ×_G V_S` standardly as the associated bundle
construction. The construction is valid for ANY representation
`ρ_S : G → GL(V_S)`. §11.6 names ρ_S for four species. The naming is
*prose*; no construction is given.

**This is structurally OK** — the math doc is at the math-doc altitude;
the implementation lives in §11.7's forward-promised parametric
carrier shard. But §11.4's "the same mathematical structure under two
names" claim depends on `ρ_S` being a structurally-well-defined object,
not a prose label.

**Verdict**: ✅ VERIFIED that the bundle setup is rigorously defined at
the math-doc altitude with concrete implementation witness at the
compiler altitude. 🟡 SHOULD-FIX: §11.1 should explicitly cite which
altitude of `altitudes.md` it instantiates (the audit-friendly answer
is "the family-root altitude of @cyberpunk" but §11.1 elides this).

---

## §4 — Attack 2: is G actually a group?

### Finding 4.1 — "type-parameter substitution group" is named, not constructed (🔴 BLOCKER-1)

§11.1: *"the structure group G = type-parameter substitution group."*
§11.3: *"The type-parameter substitution τ IS the element g ∈ G."*

A group requires four things: a set, a binary operation, an identity,
and inverses (plus associativity). The group axioms are checked
against the carrier set.

**Type-parameter substitutions form a category, not obviously a group**.
The category of substrate types and substitutions has:
- objects: types like `Adjustment`, `Morphism`, `License`, `Compliance`.
- morphisms: substitutions like `Adjustment ↦ License`.
- composition: substitution composition (associative).
- identity: the identity substitution per type.

For this to be a *group*, every substitution must be invertible. **It
isn't.** Consider:

- `Adjustment ↦ Morphism` (cybernetic coherence's τ).
- `License ↦ Compliance` (SEL's τ).

These two substitutions don't compose to anything meaningful with each
other (different source types). They aren't even endomorphisms of a
single object. They're morphisms between four distinct objects in the
substrate-type category.

At best the substrate has a *groupoid* of type-pair endomorphisms — for
a fixed `(T_reg, T_regd)` pair, the bijections `T_reg ↔ T_regd` modulo
altitude form an Aut-style group acting on one pair. But the
substrate's whole point in §11 is that *the four species carry four
different (T_reg, T_regd) pairs*. The "group G" §11.3 invokes acts on
all four simultaneously, which means G is at minimum the cartesian
product (or coproduct) of four groupoid skeletons. **This is not what
§11 says.**

If `G` is per-species, then it isn't *one* structure group of *one*
principal bundle; it's a family of principal bundles indexed by species,
each with its own group, sharing the connection language but not the
group itself. The four bundles don't compose into one tower.

If `G` is the groupoid of all type-pair substitutions across all
species, then `(E_S = P ×_G V_S)` doesn't type-check as written —
quotienting by a groupoid action requires the more general homotopy
quotient construction, and the verdict bundle is the homotopy quotient,
not the strict quotient.

**This is a load-bearing crack.** §11.3's "τ IS the element g ∈ G"
identification is the conjurer move; what's actually being identified
is the *species-specific* `τ_S` with an element of a *species-specific*
group `G_S`, and the parametric form has to wrap that bookkeeping.

**Severity**: 🔴 BLOCKER-1.

**Remedy paths** (in order of preference):
1. Restrict `G` to be the gauge group `Ǭ` of the bundle at the family-
   root altitude per `connections-and-gauge.md` §3. The species'
   verdict-carriers become representations of `Ǭ`, not of the
   substitution category. This requires §11.1 to commit to "`G` is
   the gauge group at altitude α+1 in the @cyberpunk family-root
   bundle" with α the species' altitude.
2. Replace "group" with "groupoid" throughout §11 and use homotopy-
   quotient constructions. This is the honest answer but requires
   §11 to lift to the 2-category of principal groupoid-bundles, which
   is a bigger commitment.
3. Lift the parametric carrier so that each species instantiates with
   its own group: `prism @cyberpunk/coherence<G, T_reg, T_regd, ρ>`.
   This admits the species-specific groups but breaks §11.7's
   "single parametric declaration" promise.

The cleanest is (1), which doesn't require any new math: the gauge
group `Ǭ_{α+1}` already exists in `connections-and-gauge.md` §3 with
the right properties.

### Finding 4.2 — the existing prism implementation has groups, not groupoids

`prismqueer::bundle::Cyclic<N>` implements `GroupStructure` (identity,
inverse, compose). It is a genuine group. The test bundle uses
`Cyclic<4>` as the structure group of a single altitude.

This is consistent with finding 4.1: at one altitude the structure
group is a genuine group; what fails is the per-species-becomes-one-
group identification in §11.3.

**Verdict on Attack 2**: 🔴 BLOCKER-1. §11.3's "τ IS g ∈ G" reading
requires substantial clarification about which G is meant.

---

## §5 — Attack 3: are the species' verdict-carriers actually G-representations?

### Finding 5.1 — §11.6 names representations, doesn't construct them (🟡 SHOULD-FIX)

§11.6's table:

| species              | ρ_S : G → GL(V_S)        | V_S                |
|----------------------|--------------------------|--------------------|
| cybernetic coherence | natural rep on morphism  | Adjustment ⊕ Morphism |
| SEL                  | natural rep on license   | License ⊕ Compliance  |
| viable               | natural rep on identity  | Identity ⊕ Stability  |
| autopoiesis          | adjoint rep on G         | Organization ⊕ Component |

"Natural representation on morphism" is not a defined object. Standard
representations of groups: trivial, regular, defining, adjoint, sub-
representations of these. "Natural rep on X" is an interpretive label.

For the substrate's purposes, "natural" likely means: `ρ_S` is the
unique-up-to-isomorphism representation that arises functorially from
the species' verdict-carrier construction. **This requires naming the
functor.** What functor from `G`'s category to `Vect_k`'s category
gives `ρ_S`? The doc doesn't say.

Two acceptable resolutions:

- **Path A** (concrete): write each ρ_S explicitly. For cybernetic
  coherence with G the altitude-α+1 gauge group acting on the
  substrate-morphism algebra, ρ is the action via conjugation of
  morphisms by gauge elements. For SEL the action is licence-tier
  permutation. For viable identity-preservation acts on the stability
  carrier by τ-substitution. For autopoiesis the adjoint Ad is the
  G → GL(g) action on its own Lie algebra (clean and standard).
- **Path B** (parametric): name the species' verdict-carrier *as a
  functor* `V_S : G-mod → Set` (or another suitable target). The
  representation is built in. This is what §11.7's parametric carrier
  *should* commit to once finding 4.1's group issue is settled.

Without one of these paths, the table is a placeholder.

### Finding 5.2 — autopoiesis as adjoint is structurally correct

Adjoint representation `Ad : G → GL(g)` where g is G's own Lie algebra
is a standard construction; it's well-defined for any Lie group. §11.6's
assignment of the adjoint rep to autopoiesis is structurally clean —
autopoiesis IS the species in which G acts on itself, and this is
exactly Read E's surfaced claim.

But the same critique as 5.1 applies: V_S = Organization ⊕ Component
must be identified with g (G's Lie algebra) for this to be the adjoint
rep. The doc identifies them by prose; the identification is plausible
but not constructed.

**Verdict on Attack 3**: 🟡 SHOULD-FIX. The four representations are
plausible but not constructed. The path forward is Path A (concrete
per-species reps) or Path B (parametric V_S as functor). Path B is
consistent with the §11.7 parametric form.

---

## §6 — Attack 4: is Conant-Ashby ≡ bounded-commutator rigorous? (the load-bearing claim)

### Finding 6.1 — the identity is asserted; the functor that realises it is not constructed (🔴 BLOCKER-2)

§11.4: *"The Conant-Ashby coextensivity measurement IS the bounded-
commutator condition ‖[D, π_E(g)]‖ < ∞. This is not analogical — it's
the same mathematical structure under two names."*

This is the load-bearing claim. The brief asks whether it survives.

**Conant-Ashby 1970** (the original): for a regulator R of system S, R
"is a model of" S iff the joint distribution of (R, S) factors through
a function R → S. Specifically, *"Every Good Regulator of a System
Must Be a Model of That System"* (Int. J. Systems Science, vol. 1).
The theorem is a finite-discrete category statement about regulators
as homomorphic images of the systems they regulate. The mathematical
shape is a *commutative diagram* in the category of finite stochastic
matrices (or, in the deterministic version, in the category of finite
functions).

**Connes' bounded-commutator condition**: for `(A, H, D)` a spectral
triple, `[D, π(a)] ∈ B(H)` for all `a ∈ A`. The condition is an
*analytic* statement about operators on a Hilbert space. It requires:
infinite-dimensional `H`; an unbounded self-adjoint `D` with compact
resolvent; a faithful representation π. The shape is an *analytic
bound* in operator algebra.

These are not the same mathematical object. They live in different
categories:

- Conant-Ashby: discrete, combinatorial, finite-state, no analysis.
- Connes: analytic, infinite-dimensional, operator-theoretic.

For §11.4's "IS not analogical" claim to hold, a functor `F` from
(Conant-Ashby's category) to (Connes' category) must exist with the
property that F sends the good-regulator condition to the bounded-
commutator condition, AND back-translation under `F⁻¹` (or a retract)
recovers Conant-Ashby's theorem.

**No such functor is constructed in §11.** §11.4 *asserts* the identity
by naming both conditions and stating one IS the other.

What's structurally defensible:

- The bounded-commutator condition is the substrate-altitude *lift* of
  the good-regulator condition. The discrete category sits inside the
  analytic one via the directed limit `(A_0, H_0, D_0) ↪ (A_∞, H_∞, D_∞)`
  per `spectral-triples.md` §5. At each finite stage, the bounded-
  commutator condition restricts to a finite-state regulator condition.
  In the directed colimit, the analytic statement is the closure of
  the discrete ones.

- For a *specific* spectral triple derived from a connection ω on a
  principal G-bundle with G discrete and finite, the bounded-commutator
  condition reduces to a finite-state check that IS the good-regulator
  condition for the (regulator, regulated) pair on the bundle's
  altitude pair. The functor exists *in this specific case* — but the
  doc doesn't say "G discrete and finite" anywhere. The doc assumes G
  is whatever it needs to be.

The "same mathematical structure under two names" claim is *defensible*
under one of the following positions:

- **Position A (literal identity for discrete G)**: G is restricted to
  the discrete finite case (e.g. the type-parameter substitution
  groupoid skeleton at altitude α+1). Then Conant-Ashby ≡ bounded-
  commutator holds as identity of conditions; the functor is the
  embedding of finite into analytic categories. **§11 must commit to
  G being finite-discrete for this to work.** Per finding 4.1, G's
  identity is already in dispute; this resolution is compatible with
  finding 4.1's remedy (1) (G = the gauge group of one altitude) if
  the gauge group is finite-discrete at that altitude. Per
  `connections-and-gauge.md` §4, G_1 (the peer-altitude structure
  group) is *unitary basis transformations on H_peer*, which is a
  continuous Lie group, not discrete. So Position A doesn't generally
  apply at the peer-pulse altitude.

- **Position B (isomorphic up to functor)**: the identity is up to
  a named functor F. Then the claim weakens to "F(Conant-Ashby) =
  Bounded-Commutator." This is mathematically honest but is the
  WEAKEN-TO-ANALOGY verdict the brief flags.

- **Position C (directed-colimit identity)**: the analytic statement
  IS the directed-colimit closure of finite-state regulator statements.
  The Conant-Ashby theorem holds at every finite stage; the analytic
  bounded-commutator is the supremum-norm passage to the colimit. This
  is the strongest defensible reading — it makes the identity hold
  *structurally* without needing G to be finite. But it requires §11
  to commit to the directed-colimit framework and to prove the
  preservation of bounded-commutator under directed colimits (which
  `spectral-triples.md` §5 asserts but doesn't prove).

Per the brief: "If you verify [the load-bearing claim], recognition
#63 promotes today. If you find it's analogical-but-not-mathematically-
identical, recognition #63 retracts to a weaker form."

**My finding**: the claim is *between analogy and identity*. It holds
as identity if Position A or Position C commitments are made
explicit; it weakens to functorial-correspondence (i.e. analogy
in the strict sense, but stronger than ordinary analogy in the sense
that the functor is concrete) if neither commitment is made.

§11 makes neither commitment.

**Severity**: 🔴 BLOCKER-2. The claim is the entire derivation's
load-bearing piece per §11.8. Without functor specification it doesn't
land at "same structure under two names" — it lands at "structurally
related under an unnamed functor."

**Remedy**: §11.4 should add either:
1. The Position A commitment (G discrete-finite at the family-root
   altitude — and prove this is the case for @cyberpunk's altitude).
2. The Position C commitment (directed-colimit identity — and cite
   the proof of bounded-commutator preservation under directed
   colimits, which would land in `spectral-triples.md` §5 as a
   theorem rather than an assertion).

Either path is achievable; both require one more tick.

### Finding 6.2 — `curvature-and-tomm.md` §2 already says it more carefully

`curvature-and-tomm.md` §2: *"In a spectral triple `(A, H, D)` derived
from a connection ω on a spinor bundle (the Dirac operator of ω), the
commutator with an algebra element computes: `[D, π(a)] = −i ρ(da) +
curvature contributions`. Connes (1994) ch. VI makes this precise."*

This is the careful version. The commutator computes curvature acting
on the representation. The substrate's bounded-commutator condition IS
bounded curvature on the principal bundle. This makes the bounded-
commutator a substrate-altitude *measurement of curvature*, not a
substrate-altitude rename of Conant-Ashby.

The Conant-Ashby ↔ bounded-commutator claim should *route through*
curvature: Conant-Ashby's good-regulator condition ↔ bounded curvature
on the altitude bundle ↔ bounded commutator. The middle term is
`curvature-and-tomm.md`'s territory. §11.4 should cite it.

**Verdict on Attack 4**: 🔴 BLOCKER-2 plus 🟡 SHOULD-FIX (cite
`curvature-and-tomm.md` §2 for the bounded-commutator-IS-curvature
half of the bridge).

---

## §7 — Attack 5: are the five cybernetic ancestors orthogonal or overlapping?

### Finding 7.1 — the five decompositions are NOT formally orthogonal (🟡 SHOULD-FIX)

§11.5: *"The five cybernetic-ancestor measurements (§3) decompose the
gauge-equivalence check into orthogonal substructures."*

The claim of *orthogonality* is mathematically specific: the five
substructures should be independent components of the bundle's gauge
data, no one derivable from any other.

Examined:

- Ashby variety-match = "rank of G-action on V_S."
- Beer requisite-variety = "holonomy of ω around altitude loops at α+1."
- Bateson logical-type match = "representation theory weight
  decomposition of G."
- von Foerster circular-reflexivity = "trace of ρ on itself."
- Conant-Ashby good-regulator = "bounded commutator."

These are mathematically *related*, not orthogonal:

- The rank of the G-action on V_S (Ashby) constrains the weight
  decomposition (Bateson). If G acts trivially, rank = 0 and there's
  one weight; if G acts faithfully, the weight decomposition has rank
  weights. The two measurements are coupled.
- The trace of ρ on itself (von Foerster) is the character of the
  representation, which is determined by the weight decomposition
  (Bateson). For finite G, character determines representation up to
  isomorphism. They're not orthogonal — they're literally derivable
  from the same data.
- The bounded-commutator (Conant-Ashby) depends on G being a Lie group
  with the Dirac operator structure; it's at a different mathematical
  layer than the discrete rank/weight measurements.
- The holonomy of ω around altitude loops (Beer) is curvature-class
  data; per `curvature-and-tomm.md` §2, the holonomy and the bounded-
  commutator are computing the same object at different altitudes.

The five are a *spanning set* of measurements covering different
aspects of the bundle's structure, but they aren't *orthogonal*. They
overlap. The Conant-Ashby measurement is, per finding 6.2, computing
the same curvature that Beer's holonomy measurement is computing.

**This is OK** — overlapping measurements can still witness the form;
they don't need to be independent to be useful. But the §11.5
"orthogonal" claim is overstated. It should read "covering
substructures" or "complementary substructures."

**Severity**: 🟡 SHOULD-FIX.

### Finding 7.2 — the ancestors do NOT post-hoc the form; they were named first

A potential concern: that the cybernetic ancestors were named in §3
*then* mapped to bundle substructures in §11.5. This is mappable post-
hoc engineering: name the ancestors, then find bundle pieces that
match.

The audit-record argument: the five ancestors come from
`@epistemologic/cybernetic.mirror` (landed 2026-06-09, before §11). The
bundle math docs (`principal-bundles.md`, `connections-and-gauge.md`)
landed 2026-06-17 morning. The §11 mapping is recent. The ancestors
predate the bundle framework in the substrate's cascade.

So the mapping IS post-hoc in temporal sequence — but it's post-hoc
in the direction recognition substrate-pull cascade expects: prior
substrate vocabulary recognised at a deeper altitude. This is the
substrate-already-had-the-word pattern, not engineering. The post-hoc
character is structural to the recognition.

**Verdict on Attack 5**: 🟡 SHOULD-FIX (the "orthogonal" wording is
overstated). The structural concern about post-hoc-ness is dismissed.

---

## §8 — Attack 6: autopoiesis as adjoint — structurally correct or convenient?

### Finding 8.1 — the adjoint reading is structurally correct under the standard recipe ✅

Adjoint representation `Ad : G → GL(g)`: every Lie group acts on its
own Lie algebra by conjugation. This is canonical.

§11.6's assignment "autopoiesis → adjoint rep on G" maps cleanly. The
species in which G acts on itself IS the species where the verdict-
carrier IS G's own algebraic structure. Autopoiesis = self-production;
G's self-action via Ad = G producing itself; the identification is
structurally tight.

The substrate-altitude reading: autopoiesis names the form in which
the cybernetic family's structure group acts on its own substrate-
altitude algebra. This is also what `architecture-cybernetic-foundation.md`
already names (Maturana-Varela structure/organisation per recognition
#40, mirror `1ad45b4`).

The forward-promised shard `@epistemologic/cybernetic/autopoiesis`
would specify Ad as its `ρ_S`; this is a defensible substrate-
altitude commitment.

**Verdict on Attack 6**: ✅ VERIFIED. Autopoiesis-as-adjoint is
structurally correct under the standard mathematical recipe and aligns
with the existing cybernetic-family recognitions.

---

## §9 — Attack 7: parametric carrier — does the syntax land?

### Finding 9.1 — `prism @cyberpunk/coherence<T_reg, T_regd, ρ>` is invented syntax (🔴 was; 🟡 with downgrade)

§11.7 sketches:

```mirror
prism @cyberpunk/coherence<T_reg, T_regd, ρ> {
  ...
  type lock_pair = ( T_reg , T_regd , ρ : g ∈ G → GL(T_reg ⊕ T_regd) )
  ...
}
```

Examined against the substrate's existing prism syntax:

1. **Namespace mismatch**: §11 calls the family `@cyberpunk` everywhere.
   The substrate's existing family root is `@epistemologic/cybernetic`
   (per `shards/epistemologic/cybernetic.mirror`, landed 2026-06-16
   18:02). There is no `@cyberpunk` namespace in the substrate.

   This is a NAMESPACE DRIFT throughout §11. The doc consistently uses
   `@cyberpunk/X` where the substrate uses `@epistemologic/cybernetic/X`.

   Two reads:
   - **§11 is anticipating a future rename** from
     `@epistemologic/cybernetic` to `@cyberpunk`. There is no commit
     landing this rename on the current branch. Mara's specs cited in
     the brief (sel-as-executable-cyberpunk, cyberpunk-viable,
     cyberpunk-autopoiesis) live on her branches — they are *spec
     proposals*, not landed substrate. The recognition #63 doc should
     not commit to a namespace that doesn't exist yet.
   - **§11 has the namespace wrong**. The species should be at
     `@epistemologic/cybernetic/coherence`, etc.

   Either way: the doc should pick one namespace and be consistent. As
   stands, the doc is internally consistent at `@cyberpunk` but
   inconsistent with the rest of the substrate.

   **Severity downgrade**: this is a 🟡 SHOULD-FIX rather than a
   blocker because it's purely a labelling issue not affecting the
   mathematical derivation; but it's a real one.

2. **Type-parameter polymorphism**: the substrate's current prism
   keyword grammar admits type parameters via the `prism @foo<T>`
   syntax. Verified in `shards/prism.mirror`. So the syntactic shape
   is admissible.

   What's NOT verified is admitting THREE type parameters with one of
   them being a representation `ρ : g ∈ G → GL(T_reg ⊕ T_regd)`. This
   is a *function-type-parameter*, which the substrate's prism grammar
   does not currently parse. The `prism @foo<T_reg, T_regd, ρ>` form
   would require the grammar to admit function-typed type parameters.

   This is a substrate-grammar extension. §11.7's syntax should either
   (a) limit to type parameters and require the species' shard to
   declare ρ at value altitude inside the body, or (b) explicitly note
   that landing this declaration requires extending the prism grammar.

   **Severity**: 🟡 SHOULD-FIX. The syntax isn't blocker-class because
   the math survives even if the substrate decl needs grammar work.

3. **`g ∈ G` inside the type annotation**: this is set-membership
   notation in a type position. The substrate's type system does not
   currently admit this. The substrate-friendly form would be
   `ρ : G -> automorphism(T_reg ⊕ T_regd)` with G a type and
   `automorphism` a type constructor. The set-membership read is
   prose-friendly but not substrate-syntax.

   **Severity**: 🟢 ADVISORY.

### Finding 9.2 — does the substrate admit type-parameter polymorphism for prism decls? ✅ for two-arg case

Looking at `shards/spectral/entanglement.mirror` and similar landed
shards: the substrate admits typed-parameter polymorphism for prism
declarations via the `prism @foo` head plus typed args in subsequent
declarations. The exact syntax §11.7 uses (`<T_reg, T_regd, ρ>` as
generic parameters in the prism head) is plausible but not currently
landed; existing shards parameterise via in-body type declarations
rather than head-generics.

The cleanest path is to use the substrate's existing in-body
parameterisation:

```mirror
prism @epistemologic/cybernetic/coherence {
  in @epistemologic/cybernetic
  in @epistemologic/math/bundle

  type lock_pair = ( T_reg: type, T_regd: type, rho: ref )

  # ... five ancestor measurements as actions ...
}
```

with species `use @epistemologic/cybernetic/coherence` then supplying
the type parameters via a typed-lambda invocation.

**Verdict on Attack 7**: 🟡 SHOULD-FIX (namespace drift + grammar
extension obligation should be acknowledged in §11.7's text).

---

## §10 — Attack 8: the Polyak-Łojasiewicz contraction

### Finding 10.1 — PL contraction silently dropped from §11 (🟡 SHOULD-FIX)

§4 of recursion-locks.md (preserved from earlier ticks) names the PL
contraction as required for the lock to *hold* (not just be measurable):

```
ρ(N) = || residual_coextensivity(α, N) || / || residual_coextensivity(α, 1) ||
```

Per §4 the lock holds iff ρ(N) → 0 as N → ∞ with rate satisfying
Polyak-Łojasiewicz. §4 cites Taut's bench harness for this measurement.

§11 derives the *form* of the lock (gauge equivalence + bounded
commutator) but does NOT derive the *contraction*. The bundle structure
gives the form; whether the form *holds* across pulses is a separate
analytic statement about the contraction's rate.

For the parametric carrier to be a complete substitute for hand-
written species declarations, it must derive both: (a) the form (which
§11 does via the gauge-equivalence-on-associated-bundles construction),
and (b) the contraction (which §11 doesn't address).

This may be fine — the contraction lives at the empirical benchmark
altitude, not at the bundle-derivation altitude. But §11.8's
"Promotion-ready" claim implicitly conflates the two: the recursion
LOCK requires both form and contraction; §11 derives only form.

The §11.8 claim should be: "the recursion-lock TOWER STRUCTURE is
derived; whether each species' lock HOLDS empirically is a separate
PL-contraction question per §4." This is the honest framing.

**Severity**: 🟡 SHOULD-FIX. The omission is structural to §11's
scope but should be flagged explicitly.

---

## §11 — Aggregated findings

| # | Severity | Finding |
|---|----------|---------|
| 4.1 | 🔴 BLOCKER-1 | "type-parameter substitution group" is named, not constructed; G is at best a groupoid, more likely a per-species family of groups; §11.3's "τ IS g ∈ G" identification doesn't survive without G being committed. |
| 6.1 | 🔴 BLOCKER-2 | The Conant-Ashby ≡ bounded-commutator identity is asserted; the functor that realises the identity is not constructed. Defensible under Position A (discrete-finite G) or Position C (directed-colimit identity) but neither commitment is made. |
| 3.1 | 🟡 SHOULD-FIX | §11.1 should commit to which altitude of `altitudes.md` it instantiates (family-root). |
| 5.1 | 🟡 SHOULD-FIX | §11.6's "natural rep on X" labels are placeholders; each ρ_S needs concrete construction or functorial naming. |
| 6.2 | 🟡 SHOULD-FIX | §11.4 should cite `curvature-and-tomm.md` §2 for the bounded-commutator-IS-curvature half of the bridge. |
| 7.1 | 🟡 SHOULD-FIX | "Orthogonal substructures" overstates; the five ancestors are complementary/overlapping, not orthogonal. |
| 9.1 | 🟡 SHOULD-FIX | Namespace drift `@cyberpunk` vs landed `@epistemologic/cybernetic`; plus §11.7's syntax invents type-parameter polymorphism that the substrate's prism grammar doesn't currently admit. |
| 10.1 | 🟡 SHOULD-FIX | The PL contraction is silently dropped from §11; the §11.8 "promotion-ready" claim should be scoped to "form-only" derivation. |
| 9.1.2 | 🟢 ADVISORY | `g ∈ G` set-membership in type position is prose-friendly but not substrate-syntax. |
| 5.2 | 🟢 ADVISORY | Autopoiesis-as-adjoint is structurally correct; the V_S = g identification needs a one-line commitment. |
| 9.2 | 🟢 ADVISORY | Substrate admits prism type-parameter polymorphism but not via the head-generics syntax §11.7 uses; the in-body parameterisation form is the substrate-friendly path. |
| 3.2 | ✅ VERIFIED | Bundle setup is rigorously defined at math-doc altitude with concrete implementation witness at compiler altitude. |
| 8.1 | ✅ VERIFIED | Autopoiesis-as-adjoint is structurally correct. |

Count: 2 🔴, 6 🟡, 3 🟢, 2 ✅.

---

## §12 — Verdict

**HOLD-WITH-BLOCKERS.**

The derivation's load-bearing claim (Conant-Ashby ≡ bounded-commutator)
is *not* analogical in the casual sense, but it is *not yet* the same
mathematical structure under two names either. It's: "structurally
related under an unnamed functor between a finite-discrete category and
an analytic category." That's a coherent mathematical relationship,
stronger than analogy, weaker than identity.

For the identity claim to land:

- Either commit to G being finite-discrete at the family-root altitude
  (Position A) — possible if @cyberpunk's altitude has a discrete
  gauge group, which §11 doesn't establish but is plausible for the
  type-parameter-substitution reading.

- Or commit to the directed-colimit framework (Position C) and prove
  bounded-commutator preservation under directed colimits (which
  `spectral-triples.md` §5 asserts as a "structural result" but
  doesn't prove).

The four witnesses (cybernetic coherence, SEL, viable, autopoiesis)
are not in dispute as witnesses — the audit's §8.1–§8.4 each fit the
common shape and the substrate-already-had-the-word recognitions
under each species are strong. What's in dispute is whether the common
shape is *derived from* the bundle structure (the claim §11 makes) or
merely *consistent with* it (the weaker claim that holds without
further work).

**Severity**: HOLD-WITH-BLOCKERS rather than WEAKEN-TO-ANALOGY because
the two blockers are addressable. The brief asked: "is the strongest
objection overcome by the derivation?" The strongest objection (BLOCKER-2)
is *not* overcome — the load-bearing identity claim is asserted, not
proven. But the path to overcoming it is clear and short.

**Recommendation for the next loop tick** (Reed's discretion):

1. Address BLOCKER-1: commit §11.1 + §11.3 to G being the gauge group
   at the family-root altitude per `connections-and-gauge.md` §4.
   Discharge the type-parameter-substitution prose as the action of
   this gauge group on the species' verdict carriers. This requires
   identifying what G_{α+1} *is* for the @cyberpunk family root — the
   most plausible answer is the unitary group of the family-root's
   Hilbert space at altitude α+1.

2. Address BLOCKER-2: choose Position A or Position C explicitly. Land
   the chosen position in §11.4 with one paragraph naming the functor
   (Position A) or citing the directed-colimit preservation theorem
   (Position C). The cleanest near-term tick is Position C plus a
   future tick that lifts the preservation theorem from `spectral-
   triples.md` §5 from assertion to proof.

3. Address SHOULD-FIX 9.1: pick the namespace. Either commit to a
   substrate-wide rename `@epistemologic/cybernetic` → `@cyberpunk`
   (which is a bigger change than #63 promotion warrants and would
   need its own Pack ratification), or rewrite §11 in terms of
   `@epistemologic/cybernetic/X` namespaces consistent with the
   landed substrate. The latter is the smaller change and consistent
   with the current branch.

With BLOCKER-1, BLOCKER-2, and SHOULD-FIX 9.1 addressed, recognition
#63 promotes deservedly. Without them it should hold; if Reed prefers
a faster path, the candidate WEAKENs-TO-ANALOGY (with the four
witnesses preserved but the bundle-structure derivation downgraded
to "structural correspondence under an unnamed functor"), which is
honest but smaller than the §11 ambition.

**Form attacked. The weakest claims revealed are addressable.**

---

Signed: Seam.
Branch: `seam/recursion-lock-derivation-review` (off main).
Stash `stash@{1}` (`mara-stash-pre-tower`): untouched.
Stash `stash@{0}` (`reed-recursion-lock-tower-audit-wip-pre-mara-autopoiesis`): untouched.
Taut's territory (`mirror.spec`, `Justfile`, `shards/io/cargo.mirror`,
`shards/mirror/{mosaic,spec,spec/keywords}.mirror`): untouched.
