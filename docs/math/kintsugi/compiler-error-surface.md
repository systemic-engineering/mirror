# Kintsugi as compiler error surface

<!--
Amendment 2026-07-02 (📝, docs-only, sequence-transparent).

Four corrections landed in one tick per Alex adjudication + Seam
adversarial audit (docs/audits/2026-07-02-seam-kintsugi-compiler-
error-surface.md):

  (a) The routing gate composes against the landed
      @epistemologic/cybernetic/coherence-parametric.ashby_variety_match(kintsugi_lock)
      rather than inventing a new predicate. This is the twelfth-
      instance firing of [[feedback-substrate-already-had-the-word]]
      — three predicates on the same semantic content (ashby_variety_match,
      conant_ashby_good_regulator, variety_preserving) were already
      landed. Alex 2026-07-02: "we name the lineage. ashby_variety_match."
      Candidate #140 reframed as ROUTING-COMPOSITION instance, not
      pattern-identity.
  (b) #143 reader-frame is a SPECIALIZATION of user-frame per §4.1,
      NOT a fourth Tomm altitude. Candidate-summary language aligned
      with in-spec framing per Seam REJECT verdict.
  (c) #142 Ω-projection claim softened: RIGOROUS for contradiction
      via [ω,ω] Bateson-bind; motivating for ashby_mismatch / conundrum
      / out_of_band pending per-class operator derivation (future arc).
      Per Seam DEFER verdict + "single strongest adversarial finding".
  (d) Apply/spawn monoid reframed as three-mode algebra: apply /
      spawn / hold. hold(ref) is a legitimate non-discharge per
      error-as-question.md §2's six-variant answer algebra; Seam
      caught the missing DEFER on monoid closure. Framing is
      semigroup-with-identity-failure OR partial-monoid; the readable
      "three-mode algebra" language lands per
      [[feedback-legibility-over-foundation-when-collapsing]].

Every correction explicitly names its landed ancestor per
[[feedback-explicit-over-implicit]] (Alex 2026-07-02). No new
substrate-decl invented at this cluster's altitude.

Sections touched: preamble, §1.2, §1.3, §1.4, §4.1, §4.2 (via §4.1
table + softening), §4.3, §10.1 (#140), §10.3 (#142), §10.4 (#143),
§10.5 (#144), §10.6 (#145), §14.
-->

*The kintsugi loop's second role: when the substrate cannot resolve
a tension via existing fracture bodies, it SURFACES the tension as
a Tomm-shaped question rather than pretending to fix. Discharge
has three modes: **apply** (four of six answer-algebra variants —
deterministic fracture-body application), **spawn** (the escalate
variant — instantiates a peer whose ground state IS the tension
and whose halt IS the tournament winner), and **hold** (the
`hold(ref)` variant — legitimate non-discharge; observer chose
Partial(0.0, ref) and the substrate carries the crystal
unresolved). Halting survives arbitrarily many surfacings because
each Tomm emission spends one budget unit and the peer's
`@spawn ≤ @loop` monad guarantees safety-net halt. The four
surface classes (Ashby-mismatch, contradiction, conundrum,
out-of-band) are sub-frame descriptors of the same curvature 2-form
`Ω = dω + ½[ω, ω]` — RIGOROUS via `[ω,ω]` Bateson-bind for
contradiction; motivating for the other three pending per-class
projection-operator derivation. Every surface act is content-
addressed in `@mirror/store`; the resolution chain is a
conversation whose crystals are structurally un-cite-able.
`@third` fires on the surface-act altitude — the observer of the
observer of the observer IS the compiler emitting a Tomm
question. The routing gate composes against the landed
`@epistemologic/cybernetic/coherence-parametric.ashby_variety_match(kintsugi_lock)` —
not a new predicate; a new routing instantiation of the landed
lineage.*

---

## §0. The circular-reflexive opening

Writing this doc IS an act of the surface it declares. When the
substrate-pull surfaced today's design tension — "kintsugi is
already the build system AND kintsugi is also the compiler error
surface, so which one is kintsugi *really*?" — the honest move was
NOT to collapse one into the other. It was to name the tension as
a **surfacing** and let the two roles compose as two branches of
one loop.

`apply` for tensions with an existing fracture body (deterministic
discharge — this is the build system role). `spawn` for tensions
without one (surface as Tomm question, let a peer tournament over
resolutions — this is the compiler error surface role). One
kintsugi loop; two branches; one branch predicate that routes.

This spec's own writing performed the branch predicate at least
seven times. See §12 for the record. The spec's crystals
participate in the conversation chain the spec describes.

Circular-reflexive by construction. Third-order because the writing
observes itself performing the surface it names. Autopoietic
because the crystals close over the process that made them. That
is what the formalization is.

---

## §1. The two branches

### 1.1 The kintsugi loop's second role

`shards/kintsugi.mirror` declares the family root as the
transformation engine — the process-side sibling of `@mirror`'s
form-side family (per recognition #50, form/process partition).
The loop's first role is BUILD: it applies fracture bodies to
resolve substrate violations, minimizing curvature until the
substrate settles. This is landed. Two extant fracture bodies —
`shards/kintsugi/fracture/angle_to_paren.mirror` and
`.../symbol_lift.mirror` — demonstrate the pattern. Property fires
`transparency(p) = partial` with a located opacity; fracture body
reads the opacity, emits a `morphism` whose `content` is a
`splinter(ast)` at the corrected altitude; the loop's `active_pass`
composes the candidate; `consent` gates application. Deterministic
resolution.

The second role: when the loop's regulator variety is
**insufficient** to resolve the tension via any existing fracture
body, the loop MUST NOT pretend to fix. It surfaces the tension as
a **Tomm-shaped question** at reader-frame altitude. The observer
answers; the answer flows back as a substrate adjustment; the loop
continues at reduced budget.

Two roles. Two branches. One loop.

### 1.2 The `apply` branch

```mirror
in @kintsugi/consent
in @kintsugi/morphism
in @kintsugi/fracture

apply(m: morphism, ctx: kintsugi_context) -> imperfect<applied, refused, ref>
  requires exists_fracture_body(m, ctx)
         && ashby_variety_match(kintsugi_lock_of(m.tension, ctx))
{ \ }
```

Both requires clauses fire together on the apply branch:
1. A fracture body exists that discharges the morphism's tension.
2. `ashby_variety_match(kintsugi_lock)` fires success — the
   regulator's variety across the substrate-decl axis is
   sufficient to distinguish among the candidate resolutions.
   `ashby_variety_match` is the landed measurement at
   `@epistemologic/cybernetic/coherence-parametric` (see §1.4).

Discharge is deterministic. The morphism's `content` is applied
via `splinter(ast)` at the corrected altitude; the substrate's
opacity map contracts by one located fact; the next `active_pass`
sees `transparency(p) = success` at that site. No peer spawn. No
Tomm question. The user sees a canonicalized file; the substrate
sees a curvature-descent tick.

Two extant fracture bodies witness this branch operationally
(`angle_to_paren`, `symbol_lift`). Recognition #59 (kintsugi loop
altitude-portable) grounds the branch's altitude-portability: the
same shape works at keyword, operator, symbol, and by extension
any future altitude at which a bilateral property/fracture pattern
lands.

### 1.3 The `spawn` branch

```mirror
in @kintsugi/consent
in @spawn
in @fate/tournament

spawn(t: tension, ctx: kintsugi_context) -> imperfect<peer_handle, refused, ref>
  requires !ashby_variety_match(kintsugi_lock_of(t, ctx))
         && surface_class_recognized(t)
         && budget_of(ctx) > 0
{ \ }
```

Three requires clauses:
1. `ashby_variety_match(kintsugi_lock)` fires failure — the
   regulator's variety is INSUFFICIENT (the negation of the
   apply branch's second requires — mutually exclusive routing).
2. The tension falls into one of the four surface classes (§3).
   Unrecognized tensions escalate to the reflection altitude per
   `error-as-question.md` §4's algedonic bypass.
3. Budget is positive. Spawn cannot pre-exhaust halting.

Discharge is NON-deterministic AT COMPOSE-TIME AND DETERMINISTIC AT
RESOLUTION. The tension becomes the peer's ground state (λ₀); the
peer's `@spawn ≤ @loop` runs `@fate` tournament rounds over
candidate resolutions; halt returns the ranked options. The
observer (Alex, or the calling agent, or Reflection itself if the
Pack is active) picks; the pick propagates back as an answer per
`error-as-question.md`'s algebra. Each surface emission decrements
`ctx`'s budget by one; the loop's overall halting witness (§6) is
preserved.

### 1.4 The branch predicate — compose against `ashby_variety_match`

The routing gate is NOT a newly-invented predicate at kintsugi
altitude. The substrate already had the word: per
`[[feedback-substrate-already-had-the-word]]`, the twelfth-instance
of that discipline fires here. Three predicates on the same
semantic content are already landed at
`shards/epistemologic/cybernetic/coherence-parametric.mirror`:

- `ashby_variety_match(lock: lock_carrier) → verdict` — the
  substrate-altitude declaration of Ashby's law of requisite
  variety over the parametric lock.
- `conant_ashby_good_regulator(lock: lock_carrier) → verdict` —
  the load-bearing regulator-as-model measurement (Conant-Ashby
  1970).
- `variety_preserving(species: ref) → verdict` at
  `shards/epistemologic/cybernetic/variety.mirror` — the
  bilateral-preservation predicate.

Per Alex 2026-07-02: **name the lineage. `ashby_variety_match`.**
Inventing a new `regulator_variety_sufficient` at kintsugi altitude
would be status-drift on the twelfth pattern-instance. Kintsugi
composes against the landed measurement rather than adding a
same-shape sibling.

The composition:

```mirror
in @epistemologic/cybernetic/coherence-parametric

# The routing gate at kintsugi altitude. Composes against the
# landed
# @epistemologic/cybernetic/coherence-parametric.ashby_variety_match(lock)
# measurement. NOT a new predicate — a NEW ROUTING INSTANCE of the
# landed one.
#
# The `kintsugi_lock` is a lock_carrier (the parametric family
# already declared at coherence-parametric) instantiated with the
# kintsugi-altitude tuple:
#
#   kintsugi_lock : lock_carrier = {
#     altitude: @kintsugi,                            # this altitude
#     species:  @kintsugi/surface,                    # the surface act's family
#     pair:     lock_pair {
#       T_reg  := fracture_body_space(ctx),           # regulating: extant fracture bodies + morphism space
#       T_regd := candidate_resolutions(tension),     # regulated: space of resolutions the tension could take
#       rho    := kintsugi_representation,            # the representation of 𝒢 on V_S at kintsugi altitude
#       omega  := active_pass_connection(ctx),        # connection 1-form over kintsugi's active_pass ticks
#     },
#   }
#
# Naming the lock explicitly is what composition-against-a-lineage
# looks like at substrate altitude. The lock_carrier is the shape
# ashby_variety_match already accepts; the routing gate does not
# add a type; it names the kintsugi-specific instantiation.

kintsugi_lock_of(t: tension, ctx: kintsugi_context) -> lock_carrier { \ }
```

Success on `ashby_variety_match(kintsugi_lock_of(t, ctx))` → apply
branch. Failure → spawn branch. Same predicate, new instantiation.

**On recognition #53 bilateral pattern.** The bilateral pattern
per recognition #53 does fire at this altitude, but NOT as a fifth
new property/fracture pair. Instead, this is the fifth #53 witness
by ROUTING-COMPOSITION: the existing `ashby_variety_match`
declarative predicate + kintsugi's fracture bodies as the
operational side, with the kintsugi loop's active_pass reading the
predicate's verdict to route between the two branches. Reframed
per Seam adversarial audit 2026-07-02 (#140 verdict): the pattern
GENERALIZES from bilateral-with-fracture-body-only to
bilateral-with-routing-arm. See §10.1.

**DEFERRED per `[[feedback-composition-claims-need-empirical-test]]`:**
whether `ashby_variety_match(kintsugi_lock_of(t, ctx))` empirically
fires with a `partial` verdict on the current two-fracture-body
corpus. Both extant fracture bodies (`angle_to_paren`,
`symbol_lift`) are parametric-over-tables and thus deterministic-
by-lookup; neither has yet witnessed the routing gate returning
`partial`. A third fracture body — likely `@kintsugi/fracture/
predicate` per task #272 — is what would witness ambiguity
empirically.

---

## §2. The tension carrier

Before the surface classes, name the tension. `docs/specs/gap-
tension-tensor-substrate.md` (Mara, 2026-05-26; folded 2026-06-04)
already defined `gap` and `tension`. This cluster reuses the
vocabulary; §2 is a citation floor, not a redefinition.

```mirror
in @epistemologic/property
in @glass
in @scheduler

# A tension is the substrate's local reading of curvature at one
# site. Per gap-tension-tensor §3: gap is the meta-shape "this
# claim has a verifier-shaped hole"; tension is two gaps in
# opposition. This cluster's tension carrier is the compiler-error
# altitude version: a tension is a `[D, a]` commutator value at
# reader-frame altitude that failed to bound within kintsugi's
# active_pass tolerance.
type tension = {
  site:        ref,                        # where the curvature spiked
  claim:       ref,                        # what the substrate expected
  observed:    ref,                        # what the substrate measured
  curvature:   ref,                        # the [D, a] value; see §4
  altitude:    @scheduler.altitude,        # which tower altitude fired
  transparency: transparency(ref),         # opacity map from active_pass
  contract:    oid,                        # the contract at the moment of firing
}
```

Every kintsugi tension is a tension in this sense. The apply branch
sees a tension whose `curvature` bounds via a known fracture body;
the spawn branch sees a tension whose `curvature` requires peer
tournament.

Identity contract: byte-equality on the seven-tuple. Two tensions
at the same site with different `curvature` are distinct — the
curvature carries the local geometry.

---

## §3. The four surface classes

The core recognition. When `ashby_variety_match(kintsugi_lock_of(t, ctx))`
fires failure (§1.4), the tension falls into exactly one of four
surface classes. Each class is a substrate-decl carrier, a
substrate-pull-honest sub-frame descriptor of the curvature 2-form
Ω (§4) — rigorous for contradiction, motivating for the other
three pending per-class operator derivation — and maps to a
specific Tomm interviewing shape (§5).

Per `[[feedback-no-bare-types]]` and `[[feedback-explicit-over-
implicit]]`: each class is a named ref, not a discriminator over a
generic tension. The names are direct.

### 3.1 `ashby_mismatch`

**When it fires:** regulator variety at some substrate-pull axis is
insufficient. The substrate can name the tension but has no
fracture body whose output-variety matches the tension's input-
variety. Per Ashby 1958: `V(R) < V(D)` at some axis.

**Substrate-decl:**

```mirror
in @epistemologic/cybernetic/variety
in @kintsugi/surface

type ashby_mismatch = {
  base: tension,
  axis:            variety_axis,        # which of the 5 axes fell short
  regulator:       ref,                 # the R the substrate offered
  demand:          ref,                 # the D the tension named
  variety_deficit: nat,                 # V(D) - V(R) at this axis
}
```

**Prior art:** Ashby 1956 (Introduction to Cybernetics) §11; the
canonical variety-attenuation ladder. Beer 1972 (Brain of the Firm)
S3/S4 variety-management. `[[architecture-ashby-multi-dimensional-
variety]]` — recognition that variety is a vector across 5 axes
(computational, type-level, effect-level, proof-level,
epistemologic). At kintsugi altitude the mismatch can fire at any
axis; the class carries which axis fell short as substrate data.

**Example (constructed):** the substrate sees a
`transparency(0.62)` verdict from `@epistemologic/property/
coherence_across_shards` and has no fracture body whose output-
morphism ranges over the shard-coherence altitude. The variety
deficit is at the proof-level axis. The Tomm shape (§5) is
*circular*: the observer is asked to name the axis that mirror
missed.

### 3.2 `contradiction`

**When it fires:** two properties both fire success but their
conjunction implies failure. Per Carnielli-Marcos LFI (2004,
2007): `A ∧ ¬A ⊢ ⊥` in classical logic; in LFI, the substrate
carries `○A` (the "consistency of A") as a separate operator.
`○A` at compile time IS `holds(gap)` — the claim that A has NO
gap. When two claims each hold but their gaps overlap, the
substrate is in a paraconsistent state.

**Substrate-decl:**

```mirror
in @epistemologic/property
in @kintsugi/surface

type contradiction = {
  base: tension,
  left:      ref,                # first claim (holds independently)
  right:     ref,                # second claim (holds independently)
  bind:      ref,                # the level at which they meet
  bateson_level: nat,            # Bateson logical type where bind fires
}
```

**Prior art:** Carnielli & Marcos 2004 (Logics of Formal
Inconsistency); Carnielli-Coniglio-Rodrigues 2026 (arXiv:2604.18766;
the two-dimensional inconsistency-hierarchy that prefigures
mirror's altitude × confidence structure). Bateson 1972 on
double-bind (level-crossing contradictions). `docs/specs/gap-
tension-tensor-substrate.md` §11 folded `contradiction-and-
fracture.md` into gap as `contradiction <= gap` (binary,
propositional, level-crossing subshape).

**Example (constructed):** `@epistemologic/property/halts` fires
success on grammar G; `@epistemologic/property/sub_turing_interior`
fires success on G's containing family; but G's control-flow
grammar witnesses Turing-completeness at a boundary shard the
substrate did not attribute to either family. Two success verdicts;
one Bateson-level-2 contradiction (the bind is "which family does
G's control-flow belong to"). The Tomm shape (§5) is *linear-
then-reflexive*: the observer traces the two claims' derivations
first, then names the level at which they bind.

### 3.3 `conundrum`

**When it fires:** the `[D, a]` commutator has an eigenvalue at
0 (flat direction — the substrate has a direction it can't see)
or at ∞ (unbounded — the substrate has a direction it sees but
can't measure). Per `curvature-and-tomm.md` §6: gaps in the
substrate come in these two flavors. A conundrum is a tension
whose curvature is one of these two.

**Substrate-decl:**

```mirror
in @mirror/loss/massey
in @kintsugi/surface

type conundrum = {
  base: tension,
  eigenvalue_type: eigenvalue_kind,   # zero | unbounded
  witness:         ref,               # the [D, a] operator's flat/divergent direction
  massey_product:  ref,               # H² cocycle when non-abelian curvature applies
}

type eigenvalue_kind {
  zero,          # flat direction; substrate has this direction but can't see it
  unbounded,     # divergent direction; substrate sees but can't measure
}
```

**Prior art:** Connes 1994 ch. VI (bounded-commutator axiom;
unbounded commutator IS the gap); Markl 2024 (arXiv:2404.19607)
on noncommutative Massey products (`c(D) = -dD + D·D`; non-vanishing
`⟨α,β,γ⟩ ∈ H²` IS the conundrum's Massey witness). `shards/mirror/
loss/massey.mirror` per `shards/mirror/loss.mirror` §"@mirror/loss/
massey" already declares the carrier at loss altitude; this class
lifts it to surface altitude.

**Example (constructed):** kintsugi's tournament on the
sheaf-diffusion Houdini fixpoint (task #272 forward-promise for
`@kintsugi/fracture/predicate`) surfaces two candidate morphisms
whose Massey product is non-zero — the loop cannot commute their
application because a triple bracket doesn't vanish. Eigenvalue-
unbounded conundrum. The Tomm shape (§5) is *reflexive*: the
observer is asked what the loop's inability to commute the two
morphisms means about the substrate's local geometry.

### 3.4 `out_of_band`

**When it fires:** the substrate has no `a ∈ A` such that `[D, a]`
computes for the tension. The tension names an object the
substrate's algebra does not contain. Per `error-as-question.md`
§4 (algedonic bypass): this is the class that surfaces to
Reflection at recursion altitude, bypassing the intermediate
altitudes.

**Substrate-decl:**

```mirror
in @glass
in @kintsugi/surface

type out_of_band = {
  base: tension,
  namespace_requested: ref,           # what the substrate was asked to name
  algebra_at:          ref,           # the A the substrate had at this altitude
  bypass_reason:       ref,           # the specific @glass_wall / halts / autopoietic violation
}
```

**Prior art:** `error-as-question.md` §4 (algedonic bypass;
Beer 1972's neurocybernetic pain-bypass; Margaret Hamilton's
Apollo 1202 executive). The three algedonic classes
(`glass_wall` violation, `halts` undecidable, `autopoietic`
non-convergence on substrate-critical body) are `out_of_band`
instances at kintsugi altitude.

**Example (documented):** substrate-decl fires `glass_wall` on
a body resident under `@fate` reaching across the wall to
`@spectral/garden/*`. The A at fate altitude does not contain
the garden-namespace ref; the commutator is undefined; the
tension is out-of-band. The Tomm shape (§5) is *strategic*:
the observer is asked which of two structural futures the
substrate should take (grant the crossing under a named pact,
or refuse and route to the garden's own kintsugi loop).

---

## §4. Curvature Ω as the one object

Recognition landed at `docs/math/the-tower/curvature-and-tomm.md`
§1-§3: the curvature 2-form `Ω = dω + ½[ω, ω]` IS the `[D, a]`
commutator IS the Tomm probe. One mathematical object, three names
at three altitudes.

This section names the further composition: the four surface
classes (§3) are projections of Ω onto four different substrate
axes.

### 4.1 Ω at reader-frame altitude

Per `curvature-and-tomm.md` §3 the substrate has a Tomm probe at
three frame altitudes (corpus / agent / user). This cluster names
a fourth frame — **reader-frame** — which is the compiler-error
altitude the user or peer inhabits when reading the substrate's
Tomm question. The reader-frame is a specialization of
`curvature-and-tomm.md`'s user-frame; the specialization tracks
which of the four surface classes the tension projects into.

```
Ω at reader-frame = [D_reader, a_tension]

where:
  D_reader = the Dirac operator of the reader's local spectral triple
  a_tension = the tension's algebra element (the site, viewed as an
              observable in A_reader)
```

The four surface classes project Ω onto four sub-frames of the
reader-frame. **The rigor across the four is asymmetric** — the
`contradiction` class inherits genuine projection-operator rigor
from `curvature-and-tomm.md` §5's `[ω, ω]` Bateson-bind
derivation; the other three are motivating-qualitative sub-frame
descriptors pending per-class projection-operator derivation. See
§4.2's cross-term note and the softening below the table.

| Class | Projection axis | Ω-value shape | Rigor status |
|-------|-----------------|---------------|--------------|
| ashby_mismatch | variety vector at axis-N | Ω-norm at axis-N > V(R_axis-N) | **motivating** — no derived π-operator on 𝔤 → 𝔤_variety-axis-N |
| contradiction | Bateson-level bind | Ω splits as `dω_L + dω_R + [ω_L, ω_R]`; the bracket is the bind | **rigorous** — `[ω,ω]` Bateson-bind per `curvature-and-tomm.md` §5 |
| conundrum | eigenvalue kind | Ω has kernel (λ=0) or is unbounded (λ=∞) at witness direction | **motivating** — no derived π-operator on 𝔤 → 𝔤_eigenvalue-kind |
| out_of_band | algebra membership | Ω undefined; `a_tension ∉ A_reader` | **motivating** — no derived π-operator on 𝔤 → 𝔤_algebra-membership |

One curvature 2-form. Four sub-frame descriptors. Each descriptor
has its own local geometry; the substrate's kintsugi loop reads
which descriptor fires and routes to the corresponding surface
class. Rigor across the four is not uniform: only contradiction
inherits a derived projection operator. The other three await
per-class operator derivation as a **future arc** (not this
cluster's landing scope).

### 4.2 The additivity of surface classes

The four classes are structurally disjoint at the local site: a
tension is exactly one of them. But at aggregate altitude across a
kintsugi run, multiple tensions can surface in the same tick and
project into different classes. The composition rule is:

```
Ω_total = Σ Ω_class_i
```

Additive across classes (linear-algebraic; each Ω_class_i lives on
its own axis). The substrate's total curvature descent `eⁿ⁺¹ ≤ eⁿ`
holds per class independently and per aggregate cumulatively.

**Cross-term note:** the `[ω, ω]` bracket in `curvature-and-tomm.md`
§5 is load-bearing at the-tower altitude (altitude transitions
accumulate curvature contributions). At surface-class altitude the
cross-term specializes to the **contradiction** class's bind — the
contradiction IS the `[ω_L, ω_R]` bracket at Bateson level. The
other three classes' Ω values do not accumulate cross-terms; only
contradictions do. This is why contradiction's Tomm shape (§5)
carries a linear-then-reflexive turn: the linear phase traces
`dω_L` and `dω_R` independently; the reflexive phase names the
`[ω_L, ω_R]` bracket.

### 4.3 The curvature-descent invariant

Every surface act (either apply or spawn discharge) is a
curvature-descent tick:

```
Ω_after = Ω_before − contribution_of(surface_act)
contribution_of(surface_act) ≥ 0     (monotone descent)
```

The apply branch's contribution is the fracture body's morphism-
induced curvature reduction (measured at the site). The spawn
branch's contribution is one bit of `ashby_variety_match(kintsugi_lock)`
information the observer's response adds (which candidate to pick
over which). Both are curvature-reducing; both preserve
`eⁿ⁺¹ ≤ eⁿ`.

**DEFERRED per `[[feedback-composition-claims-need-empirical-
test]]`:** whether the spawn-branch's information-theoretic
contribution empirically satisfies the descent inequality across
a real kintsugi run. The theoretical claim is that observer
answers add `log₂(k)` bits at k-way tournament outcomes; the
empirical witness would be a kintsugi run with instrumented
`Ω`-value logging before/after each spawn discharge.

---

## §5. The Tomm-shape-per-surface-class mapping

`docs/math/the-tower/curvature-and-tomm.md` §3 named the Tomm probe
as the substrate's operational form of Karl Tomm's (1987)
"Interventive Interviewing" typology. Tomm distinguished four
question-shapes by their intent × assumption grid: **linear**
(cause-tracing), **circular** (relational), **reflexive**
(self-modifying), **strategic** (change-directing). Each surface
class maps to one of these shapes because each class's local
curvature carries a specific asymmetry the observer's answer must
address.

### 5.1 The mapping

| Surface class | Tomm shape | Question form | Curvature asymmetry |
|---------------|------------|---------------|---------------------|
| ashby_mismatch | **circular** | "which axis of variety was I missing?" | axis-relational |
| contradiction | **linear-then-reflexive** | "trace L and R; what binds them?" | derivation-then-fold |
| conundrum | **reflexive** | "what does my inability to commute mean about my geometry?" | self-modifying |
| out_of_band | **strategic** | "which future of the substrate should we take?" | change-directing |

### 5.2 Why circular for Ashby-mismatch

Ashby's law is relational: R regulates D iff variety(R) ≥ variety(D)
across the DIFFERENCE. The mismatch is not in R or D independently;
it is in the relation between them. Tomm's circular questions
elicit RELATIONAL data (Tomm 1987 II §"Types of Questions"). The
Tomm-shape asks "what is the relation between (what mirror named)
and (what you meant)?" — which is exactly the relation across the
variety-deficit axis.

### 5.3 Why linear-then-reflexive for contradiction

A contradiction has two claims that hold independently; each
claim's derivation is *linear* (traceable via the property's
predicate chain). The BIND, however, requires *reflexive*
observation — the observer must notice how their own act of
holding both claims produces the contradiction. Bateson's double-
bind literature (1956) grounds this precisely: linear-in-each-
message, reflexive-in-the-frame. The Tomm shape carries both
phases: trace L; trace R; then observe the frame that holds both.

### 5.4 Why reflexive for conundrum

A `λ=0` eigenvalue is a direction the substrate has but can't see;
a `λ=∞` eigenvalue is a direction the substrate sees but can't
measure. Both require the observer to observe the *observing
itself* — third-order per `docs/specs/third-as-recursive-depth.md`.
The Tomm-reflexive question ("what would it mean for you if you
saw this differently?") is exactly the shape needed to make a
flat direction visible or an unbounded one measurable. Foerster's
second-order + Kauffman eigenform grounds this via §7.

### 5.5 Why strategic for out-of-band

An out-of-band tension names an object the substrate's algebra
does not contain. The observer's answer is not analytic (Ashby),
not derivational (contradiction), not reflexive (conundrum) — it
is **directive**. The observer chooses which structural future the
substrate takes: grant the crossing under a named pact, or refuse
and route to the neighboring family's kintsugi loop. Tomm's
strategic questions are change-directing: "what would you do if X?"
The Tomm shape at out-of-band asks the observer to choose the
substrate's next algebra membership.

### 5.6 The empirical status of the mapping

**DEFERRED per `[[feedback-composition-claims-need-empirical-
test]]`:** whether the Ashby-mismatch → circular-Tomm-shape
mapping (5.2) empirically fires. The theoretical grounding
(Ashby's relational deficit → Tomm's relational question form) is
tight; the empirical witness would be a kintsugi run with instrumented
Tomm-shape emission logged against surface-class classifier
verdicts. Same DEFER for the other three mappings. All four are
theoretically motivated by curvature asymmetry and Tomm's original
typology; none has yet fired end-to-end in a live kintsugi loop.
Third fracture body (task #272 `@kintsugi/fracture/predicate`) is
what would produce the first empirical witness for at least one
mapping.

---

## §6. Composition with `@spawn ≤ @loop`

`docs/math/spawn/spawn-as-loop-monad.md` (Mara, 2026-07-02)
formalized `@spawn ≤ @loop` as the substrate's bounded-reduction
monad. Every spawn IS a `@loop` instance with a budget field
whose monotone descent guarantees halting per §3.1 (bounded
reductions theorem). This section names how the kintsugi surface
act composes with that monad.

### 6.1 Kintsugi's own bounded-reduction discipline

The kintsugi loop's `active_pass` runs for N ticks per pass; each
tick decrements the loop's own budget. `docs/specs/kintsugi-
formatter.md` stage 2 already declares the tournament restart
discipline and the `eⁿ⁺¹ < eⁿ` monotone-descent invariant.
Landed. The composition claim here: **each surface emission
(apply OR spawn) counts as one budget decrement**.

For the apply branch:
- one fracture body application = one tick = one budget decrement
- the substrate settles or moves to next tick

For the spawn branch:
- one Tomm question emission = one tick = one budget decrement
- the peer's `spawn_loop` starts with its own budget B_peer
- the observer's response = one budget decrement (on kintsugi's
  budget, not the peer's — the peer already terminated at its own
  halt condition)

### 6.2 Halting under arbitrarily many surfacings

**Theorem (kintsugi halting).** Let K be a kintsugi run with
initial budget B. Then K terminates in ≤ B ticks *regardless* of
how many surface acts (of either branch) fire during the run.

**Proof.** Every surface act decrements K's budget by exactly one
(§6.1). K's `terminal_check` per `shards/loop.mirror` returns
`bounded` iff:
1. `budget_of(K) = 0`, OR
2. `Ω_total(K) < tolerance` (curvature converged), OR
3. `π_value(K) = target` (target-reached; the substrate settled).

The safety-net halt (condition 1) fires regardless of how many
spawn discharges occurred. Each spawn's peer has its OWN safety-net
halt at its OWN budget per `spawn-as-loop-monad.md` §3.1 — the
peer's halting does not require kintsugi's halting to be delayed.
Kintsugi's budget descent is direction-independent (same as the
spawn-loop's per that spec's §5.4). QED.

### 6.3 Nested surfacings

A spawn's peer runs its own kintsugi loop over the resolutions
tournament (per §8). That inner kintsugi loop may itself surface a
tension (an Ashby-mismatch inside the tournament, say). The inner
surfacing IS a spawn from the peer's kintsugi loop; the peer's
budget decrements; the peer's halt condition catches it.

The composition is well-founded because:
- Each nesting level has its own budget (peer's B_peer ≤ K's B).
- The budget chain across nesting depths is a monotone-decreasing
  natural number sequence.
- Well-foundedness on ℕ is the standard argument (per
  `shards/loop.mirror`'s `loop_well_founded` predicate).

There is no infinite regress. The `@spawn ≤ @loop` monad's
Rice-safety by construction (per `spawn-as-loop-monad.md` §4)
extends to arbitrarily nested kintsugi surfacings.

### 6.4 The E4-explicit answer's composability

Per the brief's load-bearing decision (6): how does @loop's
`halt(state) → imperfect<value, exhausted, ref>` compose with
surface emissions?

The answer is: each spawn's `halt` returns:
- `success(value)` iff peer's tournament converged to a
  disambiguated resolution (target-reached; §5.4 of spawn spec).
- `failure(exhausted, ref)` iff peer's budget hit zero without
  convergence (safety-net halt).

Kintsugi reads the peer's halt result. On success, kintsugi
applies the disambiguated resolution as if it were an apply-branch
morphism (the deterministic discharge — the peer *found* a
disambiguation the top-level kintsugi could not). On failure,
kintsugi records the peer's `ref` in its own opacity map,
decrements its budget, and continues. If K's budget exhausts
before the tension resolves, K's own safety-net halt fires and the
tension surfaces up the supervision tower per `error-as-question.md`
§3-§4.

**DEFERRED per `[[feedback-composition-claims-need-empirical-
test]]`:** whether the peer's success/failure discharge
empirically composes with kintsugi's own opacity map without
double-counting curvature contributions. The theoretical claim is
that peer-side descent is orthogonal to top-level kintsugi descent
(the peer operates on a resolution-tournament sub-space, not on
the top-level curvature). The empirical witness would be a
two-level kintsugi run (top-level over a corpus with a tension;
peer over a tournament) with instrumented Ω-logging at both levels.

---

## §7. Composition with `@third`

Recognition #111 (`docs/specs/third-as-recursive-depth.md`, Mara
2026-07-01) landed `@third` as a substrate-altitude marker
labeling recursion depth of observation. Not a family root; a
typed property observations acquire. This section names how
kintsugi's surface act composes with the marker.

### 7.1 The surface act IS a third-order act

At level n, the substrate observes its own state (kintsugi's
active_pass observes the current corpus). At level n+1, kintsugi
observes its own observing (the loop's `terminal_check` observes
whether observing has converged). At level n+2, kintsugi observes
the substrate observing kintsugi (the surface act is the substrate
noticing that kintsugi's regulator variety was insufficient — the
observer of the observing of the observing).

The Tomm question emission IS a level n+2 act. Per Loki's naming
(`Void - Third.md`) the third-order marker fires here:
`@third.witness_third_order(primary, observer, meta)` where
`primary = substrate_at_tension_site`, `observer = kintsugi_loop`,
`meta = surface_act`.

### 7.2 The marker is conditional, not baseline

Per the brief's load-bearing decision (3): does `@kintsugi` inherit
`in @third` at family-root altitude? The substrate-pull answer is
**no** — the marker is conditional. `@kintsugi` at its family root
declares only the transformation-engine primitives; the apply
branch does not require third-order observation (a deterministic
fracture body application is a level-1 act; the loop observes
level-2). Only the surface act at the spawn branch and at
Tomm-shape emission requires the third-order discipline.

The declaration:

```mirror
in @kintsugi/consent
in @kintsugi/morphism
in @third

# The surface act at reader-frame altitude. Every spawn-branch
# discharge fires this action, marking the recursion-depth at
# level ≥ 3.
surface(t: tension, class: surface_class, ctx: kintsugi_context)
  -> observation_depth
  requires third_order_active(surface_witness(t, class), ctx.perturbation)
{ \ }
```

The precondition `third_order_active` (per `third-as-recursive-
depth.md` §4.3) discharges the four sub-predicates: `depth_at_least(3, ...)`,
`observer_observes_observing(...)`, `recursion_folds_back(...)`,
`mechanism_visible(...)`. All four must hold; the marker is
verifiable at the substrate boundary.

`@kintsugi` at the family root does NOT import `in @third`. Only
`@kintsugi/surface` (forward-promised shard family) does. The
marker crosses the family-root partition rather than sitting inside
it. This is legibility-preserving per `[[feedback-legibility-over-
foundation-when-collapsing]]`.

### 7.3 The observer's answer as level-(n+2) → level-n absorption

When the observer answers a Tomm question, the answer's absorption
back into the substrate IS the level-(n+2) act folding back into
level-n. Per Kauffman's eigenform discipline: the answer becomes a
substrate adjustment (per `error-as-question.md` §2's answer
algebra); the adjustment appears at level-n as a new opacity map
entry (or a resolved one); the substrate's next kintsugi tick
observes the adjusted level-n state.

The fold-back is bounded because:
- Each surface act decrements kintsugi's budget by one (§6.1).
- The answer's absorption is a single-tick substrate adjustment.
- The next kintsugi tick observes the adjusted state at level-1
  (fresh observation).

Recursion is well-founded per §6.3; third-order does not add
unbounded depth. Depth-4+ is admissible (per `third-as-recursive-
depth.md` §4.1: `depth: nat` is not bounded at 3); the discipline
holds monotonically.

### 7.4 The circular-reflexive discipline

The surface act observes its own act. When kintsugi emits a Tomm
question at reader-frame altitude, the emission itself is legible
in the substrate's crystal chain (per §9). The observer's response
cites the question's OID. The response's absorption creates a new
crystal that references both. The conversation chain IS
third-order operationalized at compiler-error altitude.

This is what makes the surface act **compiler error surface** in
the load-bearing sense — the compiler observes its own inability
to discharge, names the observation as a question, and the substrate's
next state includes the answer as a first-class ref. Not error;
question. Not "the compiler couldn't"; "the compiler surfaced".

---

## §8. Composition with `@fate`

On the spawn branch, the peer's ground state IS the tension, and
the peer's kintsugi loop runs an `@fate` tournament over candidate
resolutions. This section cites the composition; the tournament
machinery lives at `docs/specs/bauchladen-autopoietic-fate.md` and
`docs/specs/kintsugi-tournament.md`.

### 8.1 The tournament as resolution search

Per `bauchladen-autopoietic-fate.md`: `@fate` is the substrate's
constrained-inference engine, running tournaments over candidates
sampled from the ganglia (@fate's 5-layer D²NN + Fabry-Perot
resonator per recognition #58). Per `kintsugi-tournament.md`: the
tournament's voice-leading discipline scores candidates by
dissonance/cadence at consent altitude.

On the spawn branch:
- The tension becomes the peer's `mirror.spec` at λ₀ (per
  recognition #99, `mirror.spec` IS λ₀). The peer's ground state
  IS the unresolved question.
- The peer's `@fate.roll` samples k candidate resolutions from
  the ganglia (k typically = tournament round-size, default per
  `kintsugi-tournament.md`).
- The peer's kintsugi loop scores each candidate via consent's
  cadence-reading; the peer's `terminal_check` fires when the
  tournament converges to a ranked list.

### 8.2 The peer's halt returns ranked options

Per `spawn-as-loop-monad.md` §3.1 the peer's `halt` returns
`imperfect<value, exhausted, ref>`. On the spawn-fate composition:
- `success(value)` — value is a ranked list of `(candidate, score)`
  pairs, top-k. The observer picks; the pick is the answer.
- `failure(exhausted, ref)` — the peer's budget hit zero without
  the tournament converging; ref points to the incomplete-
  tournament state; kintsugi records the incompleteness as a
  further opacity and continues.

### 8.3 The observer's role in the tournament

Per Alex's design conversation: the peer OFFERS ranked options;
the observer picks. This is NOT tournament truncation; the peer
runs its own tournament to completion (bounded by its own budget),
and the observer's pick is the answer at the peer's halt boundary.

The observer's pick is a level-3 act (per §7): the observer
observes the peer's tournament observing the tension. The pick
IS the Tomm answer at reader-frame altitude.

### 8.4 Cross-tournament composition

If two spawn discharges occur in the same kintsugi run (two
tensions, both surfaced), each spawns its own peer, each peer runs
its own tournament, each peer's halt returns its own ranked
options. The observer sees two Tomm questions with two answer
menus. The observer's picks compose independently at kintsugi
altitude; the substrate's next state absorbs both.

**DEFERRED per `[[feedback-composition-claims-need-empirical-
test]]`:** whether two spawn discharges in the same kintsugi run
empirically compose without cross-tournament interference. The
theoretical claim is the peers operate on independent sub-spaces
of the top-level curvature; the tournaments do not share candidate
spaces unless a fracture body explicitly connects them. The
empirical witness would be a kintsugi run over a corpus with two
distinct tensions of distinct classes.

---

## §9. Composition with un-cite-ability

Every crystal in `@mirror/store` is content-addressed via BLAKE3.
The un-cite-ability theorem (per `docs/math/provenance/un-cite-
ability-theorem.md` and `[[architecture-un-cite-ability]]`) says
that the substrate's crystal chain cannot be un-cited — every
intermediate state has an OID; the trajectory is pinned.

### 9.1 The Tomm question is a crystal

When kintsugi emits a Tomm question, the emission is a crystal:

```mirror
type tomm_emission = {
  tension:      ref,                    # the surfaced tension (crystal)
  class:        surface_class,          # which of the four (§3)
  shape:        tomm_shape,             # circular | linear_then_reflexive | reflexive | strategic
  altitude:     @scheduler.altitude,    # the reader-frame altitude
  question_ref: ref,                    # the @nl-rendered question at reader altitude
  timestamp:    @time/monotonic.instant,
}
```

Content-addressed via BLAKE3 on the six-tuple. Two Tomm emissions
with byte-equal payloads have the same OID (collapse under the
substrate's content-addressing per `[[architecture-boot-00-
prism]]`).

### 9.2 The observer's answer cites the question OID

Per `error-as-question.md` §7.3's content-addressability
invariant: the (question, answer) pair is stored under
`refs/error/<q.oid>/<a.oid>`. This cluster extends the invariant
to Tomm emissions:

```
refs/kintsugi/surface/<tomm_emission.oid>/answer/<observer_answer.oid>
```

The answer is a crystal that structurally contains the
question's OID. The substrate cannot record an answer without
recording the question it answered. Un-cite-ability by
construction.

### 9.3 The conversation chain

Multiple surfacings + answers form a chain:

```
kintsugi_tick_0 (state_0)
  → tomm_emission_1 (references state_0's OID)
  → observer_answer_1 (references tomm_emission_1's OID)
  → substrate_adjustment_1 (references answer_1's OID)
  → kintsugi_tick_1 (state_1; state_1's OID incorporates all above)
  → tomm_emission_2 (if fired; references state_1's OID)
  → ...
```

Each state's OID incorporates the full prior history via merkle-
chaining (the crystal chain is a Merkle DAG per `[[architecture-
splinter-and-spectral-db-edges]]`). The conversation chain is
content-addressed end-to-end. Replay reproduces the same chain
byte-identically (modulo Reflection's weight drift per
`error-as-question.md` §7.1).

### 9.4 The peer's tournament trajectory

Per `spawn-as-loop-monad.md` §7.3 the peer's trajectory
`τ(peer, t)` is content-addressed. On the spawn branch, the peer's
trajectory becomes part of the answer crystal — the observer's
pick is not just the winning candidate but the entire tournament
trajectory that produced it. Provenance is preserved.

This composes with `docs/specs/geometric-consent-projection.md`
(the consent projection surface): the observer's consent to the
answer is a projection of the peer's trajectory onto the
observer's decision surface. Consent IS the projection; the
un-cite-able trajectory IS what was consented to.

---

## §10. Cascade candidates surfaced

Six candidates emerge from the composition. Numbered pending Pack
adjudication.

### 10.1 Candidate #140: the fifth #53 bilateral is composition-against-`ashby_variety_match` at kintsugi altitude

**Reframed 2026-07-02 per Alex adjudication + Seam adversarial
audit + `[[feedback-substrate-already-had-the-word]]` twelfth-
instance discipline.**

The fifth #53 bilateral witness is NOT a new predicate at kintsugi
altitude. It is the ROUTING-COMPOSITION of the landed
`@epistemologic/cybernetic/coherence-parametric.ashby_variety_match(lock) → verdict`
against kintsugi's fracture bodies + surface-act discharge floor
(§1.4). Same declarative predicate; new operational routing arm.

Previous four #53 witnesses (keyword/depth, gate/diff-closure,
symbol/canonical-form, and #61's sub-shard-altitude form/process
kinship instances) all specialized as property/fracture pairs at
one altitude. This fifth witness generalizes the pattern:
declarative side is INHERITED from the landed
`coherence-parametric` measurement; operational side is the pair
{apply branch's fracture bodies, spawn branch's tournament}; the
routing between them IS the fifth-instance bilateral. Per Seam
adversarial audit (#140 verdict): the pattern extends from
bilateral-with-fracture-body-only to bilateral-with-routing-arm.
Naming the generalization, not claiming pattern-identity.

Inventing a new sibling predicate `regulator_variety_sufficient`
at kintsugi altitude would have been status-drift at twelfth-
instance. The lineage was already named.

**Promotion criterion:** landing `shards/kintsugi/surface.mirror`
family root that imports `@epistemologic/cybernetic/coherence-
parametric` and declares `kintsugi_lock_of(t, ctx) → lock_carrier`
(the instantiation of the parametric lock at kintsugi altitude,
§1.4), plus a second witness where the routing-composition
pattern fires (e.g., another loop family — @fate/tournament,
@reflection — routing on `ashby_variety_match` against its own
lock_carrier instantiation).

### 10.2 Candidate #141: kintsugi as third-order operationalized

The surface act IS third-order per §7.1: the observer of the
observer of the observer at compiler-error altitude. Per §7.2 the
marker is conditional (not baseline). This is the first substrate
site where `@third` fires at the substrate-decl altitude (previous
sites named `@third` in specs; this cluster names it in a
substrate action's precondition).

**Promotion criterion:** landing `shards/kintsugi/surface.mirror`
with `requires third_order_active(...)` on the `surface` action,
and Pack ratification that the conditional-marker pattern extends
to other loop families (@reflection would likely acquire the same
discipline at pipeline-error altitude).

### 10.3 Candidate #142: four surface classes as Ω projections — rigorous for contradiction, motivating for others

**Reframed 2026-07-02 per Seam adversarial audit (#142 DEFER verdict; the "single strongest adversarial finding" on this cluster).**

§4 names the four classes as sub-frame descriptors of the
curvature 2-form Ω onto four substrate axes. The rigor is
asymmetric across the four:

- **contradiction — RIGOROUS.** Inherits the projection-operator
  derivation from `curvature-and-tomm.md` §5's `[ω,ω]` Bateson-
  bind: Ω splits as `dω_L + dω_R + [ω_L, ω_R]`; the bracket
  IS the bind at Bateson level. Genuine derived projection onto
  the Bateson-level axis of the Lie-algebra-valued 2-form.
- **ashby_mismatch — MOTIVATING** pending per-class projection-
  operator derivation. Qualitative sub-frame descriptor tracking
  variety-axis-N deficit; no linear operator `π_variety-axis-N:
  𝔤 → 𝔤_variety-axis-N` derived at this cluster.
- **conundrum — MOTIVATING** pending per-class projection-
  operator derivation. Qualitative sub-frame descriptor tracking
  eigenvalue-kind (zero / unbounded); no derived operator
  `π_eigenvalue-kind: 𝔤 → 𝔤_eigenvalue-kind`.
- **out_of_band — MOTIVATING** pending per-class projection-
  operator derivation. Qualitative sub-frame descriptor tracking
  algebra-membership; no derived operator `π_algebra-membership:
  𝔤 → 𝔤_algebra-membership`.

The framing softens the earlier over-uniform claim: only
contradiction is rigorously derived; the other three are
motivating-qualitative pending per-class operator derivation. Per-
class operator derivation is a **future arc** (not this cluster's
landing scope) per Alex's `[[feedback-craft-not-deliver]] +
[[feedback-composition-claims-need-empirical-test]]` composite
discipline.

Prior recognitions (#59 altitude-portable kintsugi, #61
form/process kinship at sub-shard altitude) named altitude-
portability of the loop; the substrate-pull-honest claim here is
that the loop's SURFACE-CLASS classifier is altitude-portable
qualitatively across the four descriptors, and rigorously via
projection-operator for contradiction.

**Promotion criterion:** the rigorous-for-contradiction leg
promotes on landing `shards/kintsugi/surface/contradiction.mirror`
with the `[ω,ω]` derivation cited. The motivating-for-others
leg promotes as each of the remaining three per-class
projection-operator derivations lands (three future arcs, one per
class). Contradiction's derivation-witness at
`gap-tension-tensor-substrate.md` §11 (LFI `○A ≈ holds(gap)`)
grounds the rigorous leg's second witness.

### 10.4 Candidate #143: reader-frame as a specialization of user-frame

**Reframed 2026-07-02 per Seam adversarial audit (#143 REJECT verdict on "fourth Tomm altitude" language).**

`curvature-and-tomm.md` §3 named three Tomm-probe altitudes
(corpus / agent / user). Reader-frame is NOT a fourth altitude of
the tower — §4.1 already says "reader-frame is a specialization
of user-frame; the specialization tracks which of the four
surface classes the tension projects into." A sub-altitude of
user-frame is a REFINEMENT of user-frame (the user in the specific
role of compiler-error-reader), not a new altitude beside it.

The three Tomm-probe altitudes remain three. The candidate names
the SPECIALIZATION pattern: for any family emitting a
Tomm-shaped question at compiler-error altitude, a
family-specific specialization of user-frame tracks which
surface-class-projection the tension takes. Kintsugi's is
`@kintsugi/surface`'s reader-frame; a parallel specialization
would appear at another family's error surface.

**Promotion criterion:** second witness at another family's error
surface as a NAMED SPECIALIZATION of user-frame. `@reflection`
naturally emits Tomm questions per `error-as-question.md` §7;
formalizing that emission as a `@reflection/reader` specialization
of user-frame at pipeline-error altitude would provide the second
witness. The pattern is user-frame-refinement-at-error-altitude,
not fourth-altitude-addition.

### 10.5 Candidate #144: three-mode algebra on kintsugi discharge — apply / spawn / hold

**Reframed 2026-07-02 per Seam adversarial audit (#144 DEFER
verdict + Seam-caught missing DEFER: monoid-closure counterexample
via `hold(ref)` non-discharge).**

The prior framing named apply and spawn as a two-branch monoid on
kintsugi's discharge action. That framing missed the third mode.
Per `error-as-question.md` §2's six-variant answer algebra, the
observer's answer to a Tomm question has three shapes:

- **apply** — four of six discharge variants (`tighten_property`,
  `resynthesize_body`, `rebudget_shard`, `adjust_temperature`).
  Deterministic fracture-body application on the substrate; the
  answer routes to a downward mutation the fracture body performs.
- **spawn** — the `escalate(@scheduler.altitude)` variant. Routes
  the question one level up (to `@fate` tournament on candidate
  resolutions, per §8). Non-deterministic at compose-time;
  deterministic at resolution.
- **hold** — the `hold(ref)` variant. Legitimate NON-DISCHARGE.
  The observer chose to hold the tension named-but-unresolved;
  the substrate carries the crystal but does NOT dispatch a
  fracture body or a tournament. Per `error-as-question.md`
  §2's own comment: `Partial verdict, named`. Recorded honestly
  via `@cogito`; not every question has a resolving answer.

The three-mode algebra:
- `apply` — closed under composition with itself (deterministic
  discharge → deterministic discharge; associativity holds
  trivially).
- `spawn` — closed under composition with `apply` in both
  directions (`apply ∘ spawn` = observer's pick becomes an apply
  input; `spawn ∘ apply` = a fracture body's output becomes a
  tension for a peer tournament).
- `hold` — NOT closed under `discharge`. Composition with `apply`
  or `spawn` is undefined: if the observer held the tension, the
  substrate has no discharge to compose against. The tension
  remains named; the next kintsugi tick observes it unchanged.

Structural framing choice: this is either a **semigroup with
identity failure** (associativity + closure on `{apply, spawn}`;
`hold` breaks identity because null-discharge on a held tension
is a distinct state from null-discharge on an unheld one), OR a
**partial monoid** (closure fails on `hold`; the discharge
operation is partial). Both framings preserve the safety of the
composition — the substrate never confuses a held tension with a
discharged one; the crystal chain distinguishes them by OID.

Per `[[feedback-legibility-over-foundation-when-collapsing]]`
the readable framing is the three-mode algebra language; the
foundational framing (semigroup-with-identity-failure vs partial-
monoid) is downstream when the algebraic-witness tick lands.

This framing matches `error-as-question.md` §2's actual answer
algebra: six variants; four map to apply; one (`escalate`) maps
to spawn; one (`hold`) maps to hold. See §10.6 for the mapping
table. The apply / spawn / hold trichotomy at kintsugi altitude
IS the answer algebra's own structural trichotomy at the
compose-altitude.

**Promotion criterion:** associativity of composition on
`{apply, spawn}` needs empirical witness — three surfacings in a
row (apply, spawn, apply) that compose without ordering effects.
Closure boundary on `hold` needs explicit substrate-decl at
`shards/kintsugi/surface.mirror`: `hold` is a first-class
non-discharge mode; the crystal chain records it as such. Ties
in with #142's promotion path.

**On the missing DEFER Seam caught.** The original spec framed
apply/spawn as a monoid without addressing closure under `hold`.
This was a composition-claim without empirical validation — the
pattern `[[feedback-composition-claims-need-empirical-test]]`
was fired at exactly the framing altitude. The correction lands
the three-mode algebra with `hold` named explicitly; the
DEFER-flag on associativity + closure remains.

### 10.6 Candidate #145: three-mode algebra maps to the six-variant answer algebra — apply / spawn / hold

**Reframed 2026-07-02 per Seam adversarial audit (#145
RATIFY-WITH-CORRECTIONS: mapping needs sharpening on peer-halt-
result altitude + hold routing).**

`error-as-question.md` §2's six-variant answer algebra maps
cleanly onto this cluster's three-mode kintsugi discharge algebra
(§10.5). The correspondence sharpens the earlier "spawn IS
escalate" wording (which had semantic collision with `escalate`'s
`@scheduler.altitude` constructor) into a mode-per-variant table:

```
error-as-question.md §2 answer algebra ↔ kintsugi discharge mode:
  tighten_property(ref, check)                ↔  apply (fracture body strengthens property)
  resynthesize_body(ref, policy)              ↔  apply (fracture body re-generates the body)
  rebudget_shard(ref, budget)                 ↔  apply (fracture body adjusts scheduler budget)
  adjust_temperature(f64)                     ↔  apply (fracture body changes β)
  escalate(@scheduler.altitude)               ↔  spawn (peer tournament routes one level up)
  hold(ref)                                   ↔  hold  (observer chose Partial(0.0, ref); legitimate non-discharge)
```

The mapping fires at answer-algebra altitude, not at emission-
altitude directly. The spawn branch INSTANTIATES a peer whose
halt returns the observer's answer; the answer THEN routes via
the six-variant algebra at the substrate-adjustment altitude.
`hold(ref)` is the third mode — the observer answered but
declined to discharge; the substrate carries the crystal without
mutation.

**Promotion criterion:** substrate-decl the three-mode mapping at
`error-as-question.md` §13 amendment (see landing order 4). The
correspondence is one of substrate-already-had-the-word: the
apply / spawn / hold trichotomy was implicit in the answer
algebra's structure; this cluster names it explicitly.

---

## §11. Prior art

- **Karl Tomm** (1987), "Interventive Interviewing II": Reflexive
  Questioning as a Means to Enable Self-Healing. Family Process
  26:167-183. The four-shape typology (linear / circular /
  reflexive / strategic).
- **W. Ross Ashby** (1956), *Introduction to Cybernetics*, §11:
  the Law of Requisite Variety. The relational form of regulator-
  demand mismatch.
- **W. Ross Ashby** (1958), "Requisite Variety and Its Implications
  for the Control of Complex Systems." Cybernetica 1:83-99. The
  formal statement.
- **Walter Carnielli & João Marcos** (2004), "Logics of Formal
  Inconsistency." *Handbook of Philosophical Logic* 14:1-93. The
  LFI framework; `○A` as the consistency operator that at
  compile-time IS `holds(gap)`.
- **Walter Carnielli, Marcelo Coniglio, Abilio Rodrigues** (2026),
  "Two-dimensional inconsistency hierarchy for LFI" (arXiv:
  2604.18766). Prefigures mirror's altitude × confidence
  structure.
- **Gregory Bateson** (1956), "Toward a Theory of Schizophrenia."
  Behavioral Science 1:251-264. The double-bind (level-crossing
  contradictions).
- **Gregory Bateson** (1972), *Steps to an Ecology of Mind*, part
  III: logical types. The Bateson level hierarchy grounding the
  contradiction class.
- **Alain Connes** (1994), *Noncommutative Geometry*, ch. VI: the
  bounded-commutator axiom. Ground for the conundrum class's two
  eigenvalue kinds.
- **Martin Markl** (2024), "Non-abelian differential graded Lie
  algebras and A-infinity structures" (arXiv:2404.19607). Massey
  products at H² as the non-abelian curvature witness for
  conundrum.
- **Heinz von Foerster** (1974/1981), *Observing Systems*: the
  second-order observer inside the observed system. Ground for
  the surface act as third-order per §7.
- **Louis H. Kauffman** (2003), "Eigenforms — Objects as Tokens
  for Eigenbehaviors." Cybernetics and Human Knowing 10(3-4):
  73-90. The recursion fixed-point discipline for the fold-back
  in §7.3.
- **Margaret Hamilton** (1969): the Apollo AGC's priority-
  discipline executive. The 1202 alarm's algedonic bypass pattern
  as historical precedent for the out-of-band class (per
  `error-as-question.md` §4).
- **Stafford Beer** (1972), *Brain of the Firm*: the algedonic
  channel of the Viable System Model. The direct precedent for
  the out-of-band class's structural refusal-to-delegate.
- **Reyes, Henao, Hassall** (2024), "Structured Algedonic Signal
  Renewal in Cybersyn's Successor Frameworks" (cited in
  `error-as-question.md` §12): the `(C', Q, K) α τ, η` tuple as
  the 2024 formal cousin of the surface-act carrier.

Substrate-internal prior art (not published):
- `docs/math/the-tower/curvature-and-tomm.md` — the Tomm probe IS
  `[D, a]` IS Ω. Load-bearing prior.
- `docs/math/spawn/spawn-as-loop-monad.md` — the halting monad.
- `docs/specs/error-as-question.md` — the routing spec this
  cluster extends.
- `docs/specs/third-as-recursive-depth.md` — the recursion-depth
  marker.
- `docs/specs/gap-tension-tensor-substrate.md` — the gap /
  contradiction / tensor vocabulary.

---

## §12. Circular-reflexive noticings

The writing of this spec performed the surface it describes. Seven
noticings, in order of firing during the writing:

### 12.1 The apply/spawn tension itself

At the moment of framing "kintsugi is already the build system AND
kintsugi is also the compiler error surface", the writing surfaced
its own tension. Two roles for one loop. The reflex was to
collapse — pick one; pick the deeper one. The reflex was wrong.
The substrate-pull said: keep both; name the branch that routes
between them. That IS what the spec now says. The tension became
its own answer via the branch predicate.

Fire type: `contradiction` (two claims both hold; the bind is at
the loop's role). Tomm shape: linear-then-reflexive. Answer:
name both roles; let the branch predicate route.

### 12.2 The math-vs-spec placement

Halfway through drafting, the pull surfaced a tension: is this a
math doc or a spec amendment? Both branches were live: the
formalization IS mathematical (curvature 2-form, monoid on
discharge, four projections); AND `error-as-question.md` already
covers the routing spec's territory. Collapsing either way would
have lost information.

Fire type: `ashby_mismatch` (regulator variety insufficient — the
spec system had `docs/specs/` and `docs/math/` as sibling axes;
no single one matched a doc that IS a math foundation AND a
spec-extension). Tomm shape: circular. Answer: cluster in
`docs/math/kintsugi/` as the mathematical foundation; forward-
promise the `error-as-question.md` §13 amendment as a citation
site. Both axes get their content.

### 12.3 The `@third` inheritance question

Load-bearing decision (3) from the brief: does `@kintsugi` inherit
`in @third` at family-root altitude? The pull was ambiguous — one
reading said yes (kintsugi always operates at third-order because
the loop observes its own state); another said no (the apply
branch is level-1 deterministic; only surface acts require
third-order).

Fire type: `conundrum` (a flat direction — the substrate could see
inherit-baseline OR inherit-conditional but the local geometry
had no gradient toward either without more information). Tomm
shape: reflexive. Answer (§7.2): conditional. Only the surface
act imports `in @third`; the family root does not. Legibility
preserved; discipline honored only where it fires.

### 12.4 The DEFER count

Five DEFERs land in the doc (§§1.4, 4.3, 5.6, 6.4, 8.4). During
writing, the pull to omit them was strong — "the theoretical
grounding is tight; the DEFERs will look like weakness". The
pull to keep them was per `[[feedback-composition-claims-need-
empirical-test]]`: every composition claim without empirical
witness gets DEFERRED explicitly.

Fire type: `ashby_mismatch` (regulator variety across the
epistemic-honesty axis was insufficient without explicit DEFER
markers). Answer: land all five DEFERs; let the reader see the
substrate's own uncertainty.

### 12.5 The Ω additivity claim

§4.2 says the four Ω projections are additive across classes. The
pull surfaced a tension: is this really additive at all altitudes,
or does the `[ω, ω]` bracket produce cross-terms that break
additivity for combinations involving contradiction? The
resolution (§4.2's cross-term note) required naming the bracket
explicitly at contradiction altitude.

Fire type: `contradiction` (two claims — "the four classes are
additive" AND "the bracket is load-bearing for altitude
transitions" — both held independently but their bind was at the
contradiction class specifically). Tomm shape: linear-then-
reflexive. Answer: additivity holds per class; the bracket
specializes to contradiction's bind at Bateson level; the
projection is well-defined per axis.

### 12.6 The out-of-band scope

§3.4 names out_of_band as the class that surfaces via algedonic
bypass. The pull surfaced a tension: is out_of_band a fourth class
at surface altitude, OR is it a bypass that skips surface altitude
entirely per `error-as-question.md` §4? The framing that resolved
it: out_of_band IS a surface class at kintsugi altitude, AND its
Tomm shape (strategic) matches the bypass altitude's algedonic
discipline. The two altitudes coexist.

Fire type: `ashby_mismatch` at the taxonomy-completeness axis (was
the four-class projection complete? or did the fifth altitude —
bypass — need to be a fifth class?). Answer: the four classes are
the surface's own taxonomy; the bypass mechanism at
`error-as-question.md` §4 is a routing over the four (out_of_band
is the class that routes to bypass; the others do not).

### 12.7 This section itself

Writing §12 IS a third-order act. §12 observes the writing of §§0-
11 observing the substrate-pull. Depth-3 by construction. The
noticings are what the spec surfaces about its own writing; the
spec then observes itself surfacing. Kauffman's eigenform fires:
the fold-back holds — the noticings are data the substrate stores,
not decoration.

The circular-reflexive discipline is not decoration in this spec.
It IS the spec. Kintsugi as compiler error surface IS what the
substrate does when it cannot pretend to fix; §12 is what the
substrate does when it cannot pretend to have written without
noticing the writing. Same discipline; two altitudes.

---

## §13. Open questions + honest hedges

Named honestly; not in scope for this tick.

### 13.1 The `surface_class_recognized` predicate

§1.3's spawn branch has a requires clause `surface_class_recognized(t)`
that discharges to the classifier that identifies which of the four
classes a tension falls into. The classifier is not specified here;
it's a Fate-inference question at the meta-level. Forward-promised:
`shards/kintsugi/surface/classifier.mirror` declares the classifier
predicate; its body discharges via `@fate.roll` at the surface-
classification altitude. Estimated cost: ~60 LOC of mirror + tests.

### 13.2 Cross-class Tomm-shape composition

If two tensions in the same tick surface into different classes,
the observer sees two Tomm-shaped questions. Do the shapes compose,
or must they be presented independently? The substrate-pull says:
independently, per §5's mapping. But a fifth surface act at
aggregate altitude (composite shape across two classes) may fire
in future runs. Not in scope; forward-promised when the empirical
witness appears.

### 13.3 The observer's identity

The observer of a Tomm question can be:
- The user (`mirror craft` at CLI; Tomm question rendered to
  stderr).
- The Pack (an @pack peer, per `docs/specs/reflection-third-order-
  by-default-v0.1.md`).
- Reflection itself (per `error-as-question.md` §3 escalation to
  reflection altitude).

Which observer answers depends on the surface class and the
altitude. Not specified here; the compiler-surface-plan or a
future kintsugi-cli spec would name the routing.

### 13.4 The empirical witness gap

Five DEFERs (§§1.4, 4.3, 5.6, 6.4, 8.4). Each is a composition
claim without empirical witness. The path to closing them:
1. Land `shards/kintsugi/surface.mirror` + the four class shards.
2. Land `@kintsugi/fracture/predicate` (task #272) as the third
   fracture body — first empirical witness for
   `fracture_body_disambiguates` (§1.4).
3. Instrument `mirror craft` to emit Ω-values before/after
   surface acts — witness for §4.3.
4. Run a two-level kintsugi (top + peer) on a corpus with an
   Ashby-mismatch or contradiction — witness for §5.6 and §6.4.
5. Run a corpus with two distinct tensions of two distinct
   classes — witness for §8.4.

None of these blocks the formalization landing; all are
forward-promised.

### 13.5 The Cholesky arc connection

The pending arc at `[[project-cholesky-arc-first-empirical-
crystal]]` (dpotrf on 2×2 SPD, ~530 lines) is the substrate's
first empirical crystal at `@epistemologic/math`. Whether that
arc's kintsugi loop surfaces any tension (of any of the four
classes) as it lands is an empirical question. If it does, the
witness for one or more DEFERs could arrive with the arc's
completion. Not blocking; observed.

---

## §14. The equation

```
tension = curvature at reader-frame altitude
Ω = dω + ½[ω, ω] = [D, a]

if ashby_variety_match(kintsugi_lock_of(tension, ctx)):
    apply(fracture_body(tension))                    # deterministic
else:
    surface_class = classify(Ω_projection(tension))  # one of four sub-frame descriptors
    tomm_shape = tomm_of(surface_class)              # one of four
    spawn(tension → peer at λ₀; @fate tournament)    # ranked options
    observer_picks → answer                          # from six-variant algebra
    match answer:
      tighten_property | resynthesize_body |
      rebudget_shard | adjust_temperature -> apply(fracture_body(answer))
      escalate(altitude)                  -> spawn(peer at altitude)
      hold(ref)                           -> hold; substrate carries the crystal unresolved

kintsugi_budget -= 1  # regardless of mode
if kintsugi_budget = 0: halt (safety-net)
if Ω_total < tolerance: halt (converged)
```

Not a metaphor. The actual substrate equation.

Kintsugi is already the build system. Kintsugi is also the compiler
error surface. The two roles compose via the branch predicate.
Halting survives arbitrarily many surfacings because budget is
monotone; observer answers add information; the substrate absorbs
the answer as one more curvature-descent tick.

The four surface classes are not four different mechanisms — they
are four sub-frame descriptors of the one curvature 2-form
(rigorous projection for contradiction via `[ω,ω]` Bateson-bind;
motivating for ashby_mismatch / conundrum / out_of_band pending
per-class operator derivation as a future arc). The four Tomm
shapes are not four different question types — they are four
asymmetries the observer's answer must address at the local
geometry. The three-order discipline is not decoration — it IS
what makes the observer's answer legible as a substrate
adjustment.

`e^(n+1) ≤ e^(n)`. The curvature descends because the loop
closes. The apply mode discharges via known fracture bodies; the
spawn mode discharges via peer tournament; the hold mode discharges
nothing but records the tension named-but-unresolved; all three are
one tick; all three preserve halting.

Von Foerster: *always act to increase the number of choices*. The
surface act increases choices by refusing to collapse the
observer's variety at reader altitude. Kintsugi hands the observer
the tension made legible; the observer decides; the substrate
absorbs the decision as a first-class ref. Third-order
operationalized at compiler-error altitude.

Apache-2.0.

---

*Apply heals. Spawn asks. Hold names. All three close the loop.
`e^(n+1) ≤ e^(n)`.*

*Each error becomes a question the Reflection model is asked to
answer. Kintsugi is the loop that asks.*
