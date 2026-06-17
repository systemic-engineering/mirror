# Geometric Consent Projection — and the Semantic Logic Composer

*2026-06-17. Mara. Spec — synthesizing three substrate-pull layers into one
arc: pacts as declarative properties, ACLs as geometric consent projections,
and the and/or/not/but semantic logic composer that joins them at one
altitude. Markdown only; no `.mirror` files land with this commit. Substrate
declarations are sketched as forward-promised RED+GREEN ticks; bodies
discharge in subsequent ticks per the bilateral pattern. v1.5-ready.*

Status: **Red.** The architectural shape is pinned; the cascade direction
is named; the security invariant is sketched as the asymmetry of the
projection; the `but` operator's algebraic structure is grounded in
defeasible logic + linguistic-argumentation prior art; the carrier
surfaces are forward-promised, not implemented in this tick. Implementation
ticks are enumerated in §8.

---

## Reference

- **Alex's framings (verbatim):**
  - 2026-06-17 #1: *"What if the ACL is not an ACL but a geometric
    consent projection at various logic levels? The higher the logic
    level the higher the resolution the higher the ACL cascade?"*
  - 2026-06-17 #2: *"Also tell Mara to look at the @epistemologic
    semantic logic components (they might still be in boot/) the `and`
    and `or` and `but`. I think we might be able to compose a logic
    programming surface for both pacts and the ACL."*
- **The three memories that grounded the recognition:**
  - [[architecture-error-as-tomm-probe]] — the compiler error surface
    as a commutator probe `[D_substrate, user_code]` at the user-frame
    altitude; Tomm's reflexive question made operational.
  - [[architecture-geometric-consent-projection]] — the recognition
    that an ACL is the logical-type-1 projection of a richer consent
    geometry; the Bateson tower of types organising the cascade.
  - [[architecture-at-x-is-mathematical-value]] — `@<X>` is a
    first-class substrate value (prism, hence spectral triple, hence
    mathematical object).
- **The spectral-triple grounding:** [[architecture-connes-spectral-triple]] —
  the substrate IS the operational form of `(A, H, D)`; this spec
  reads consent as a value of `A` and the cascade as a commutator
  operation `[D, consent_at_N]`.
- **Math root (cite, don't duplicate):** the math this spec uses
  lives at `docs/math/the-tower/` (the principal bundle tower).
  Per the `docs/math/` vs `docs/specs/` convention (see
  `docs/math/README.md` + `AGENTS.md`), this spec CITES the math:
  - `docs/math/the-tower/spectral-triples.md` §1–§3 — the
    Connes triple `(A, H, D)` (cited at §2.1).
  - `docs/math/the-tower/connections-and-gauge.md` §3 — the cascade
    IS a gauge transformation; the asymmetry IS the gauge action
    asymmetry (cited at §2.2 "the cascade as a natural
    transformation" — the natural-transformation reading is the
    sheaf-of-sections shadow of the gauge-transformation reading).
  - `docs/math/the-tower/curvature-and-tomm.md` §2–§3 — `[D, a]`
    as curvature probe; the Tomm probe IS the curvature 2-form at
    the user-frame altitude (cited at §2.3 + §6 "the security
    invariant").
  - `docs/math/the-tower/altitudes.md` §2 — the Bateson tower as
    altitudes of the bundle tower; consent at type N+1 vs N
    follows the inclusion `G_N ⊴ G_{N+1}` (cited at §2.1's
    `A_0 … A_{N+1}` ladder).
- **The Bateson primitive:** [[architecture-bateson-logical-type-primitive]] —
  the logical-type hierarchy (recognition #42); each consent type
  wraps the previous; cannot be operated on within itself.
- **The bilateral pattern:** [[architecture-property-fracture-bilateral]] —
  declarative property half + operational fracture body; the form/process
  partition (#50, #55) determines where each declaration lives.
- **The substrate-pull discipline:** [[feedback-substrate-already-had-the-word]] —
  every "missing concept" recognition turns out to be a name the
  substrate already implicitly carried; the new word here is `but`.
- **Type discipline:** [[feedback-no-stringly-types]], [[feedback-no-bare-types]] —
  consent geometry is typed end-to-end; logic expressions are typed
  AST values, not strings.
- **The existing substrate consent vocabulary:** `shards/kintsugi/consent.mirror`
  (Mara, 2026-06-10) — the auto-apply boundary at the morphism altitude;
  the verdict carrier this spec lifts UP through the logical-type tower.
- **The existing semantic logic shard (legacy):**
  `boot/std/bool.mirror` — declares `and`, `or`, `not`, `xor`, `guard`,
  `lazy_guard`, `to_order` under the retired `grammar` keyword at the
  retired `boot/std/` path. Migration is forward-promised in §8; this
  spec sketches the post-migration shape under
  `@epistemologic/logic/<op>`.
- **The error surface this composes with:** `docs/specs/error-as-question.md`
  (Mara, 2026-06-01) — the algedonic-shaped error grammar; the
  Tomm-shaped projection of consent geometry at the user-frame
  altitude is the spec extension this document forward-promises.
- **The threat model this clarifies:** `docs/specs/threat-model-v0.md`
  (Mara, 2026-06-12) §2.1 — consent integrity is the highest-priority
  protected property; this spec names what consent integrity *is*
  structurally (a property of the geometric projection's asymmetry,
  not a runtime check).
- **The benchmark/transparency siblings:** `docs/specs/benchmark-tracing.md`
  (Taut, 2026-06-17) — composition under the transparency monoid is
  the same composition law this spec consumes for logic expressions
  over verdict-valued consents.
- **Canonical pact precedents:**
  `shards/epistemologic/pact/composition_closed.mirror`,
  `shards/epistemologic/pact/keyword_matches_path_root.mirror`,
  `shards/epistemologic/pact/substrate_source_in_shards.mirror` —
  the form-side declarations the new pacts mirror at the consent and
  logic altitudes.

---

## §1 — The recognition

### 1.1 The three layers as one arc

Three substrate-pull layers landed in three conversations across
2026-06-17. They are not three recognitions; they are one cascade
viewed at three altitudes.

| Layer | Substrate piece | Today's status |
|-------|-----------------|----------------|
| **1. Declarative properties** | `pact @epistemologic/pact/<X>` | **Landed.** Recognition #37 (`requires` IS a Paskian agreement); `pact` replaced `grammar`; ~12 shards migrated under `@epistemologic/pact/` per `keyword_matches_path_root`. |
| **2. Geometric consent projection** | `@mirror/consent/geometry` (new family root) | **Proposed in this spec.** The ACL IS the logical-type-1 projection of a richer Bateson tower of consents; consent integrity (threat model §2.1) is the asymmetry property of the projection. |
| **3. Logic programming composer** | `@bool` extended with `but`; or post-migration `@epistemologic/logic/<op>` | **Proposed in this spec.** The composer joins (1) and (2) at one altitude: a single logic expression mixes pacts and consents under `and`/`or`/`not`/`but`. |

Layer 3 is the composer that makes Layers 1+2 expressible together.
A single semantic logic expression `pact_X and consent_Y but pact_Z`
mixes form-side declarative properties with process-side consent
projections under one typed AST. The expression IS a substrate value
(per [[architecture-at-x-is-mathematical-value]]); the composer
reduces it to a verdict; the verdict carries the same `transparency<p>`
shape every other altitude uses.

### 1.2 Why the three layers are one substrate-pull, not three

Each layer alone is incomplete:

- **Pacts alone** declare obligations but cannot express
  default-with-exception consent ("yes to mutating my dotfiles, **but**
  not when credentials are touched"). The `pact` keyword names the
  rule; it has no compositional surface for negated exceptions over
  scopes.
- **Consent geometry alone** captures the Bateson tower of access
  altitudes but has no language for writing higher-type consents
  compositionally. Saying "I consent to type-N+1 policy X" is a
  statement *about* the geometry; it is not a statement *in* the
  geometry's compositional algebra.
- **Logic operators alone** (`and`/`or`/`not` in `@bool`) compose
  truth values but have no consent or pact context. `bool and bool`
  is not the same operation as `pact and consent` — the latter mixes
  carriers, and one of those carriers (consent) is verdict-valued
  with located opacities, not a binary.

The three layers compose: the consent geometry needs the logic
operators to express higher-type consents (cascading down to lower
types is a projection; cascading down a logic expression is a
homomorphism). The logic operators need a value carrier richer than
`bool` (because consent is `transparency<p>`, not pass/fail). The
pacts provide the declarative grammar the consents and logic
expressions reference (a consent at type N+1 is "consent to morphisms
satisfying pact_X but failing pact_Y" — the pacts are first-class
operands in the logic expression).

This is the [[feedback-substrate-already-had-the-word]] pattern at
the 53rd+ instance: each layer was implicit; naming the joint surface
makes the existing carriers compose under one algebra.

### 1.3 The single sentence

**An ACL is a projection. Consent is a value. The composer is a
language. The three together form one substrate surface at the
access altitude.**

---

## §2 — Mathematical grounding

### 2.1 Consent as a value in the spectral triple's algebra

Per [[architecture-connes-spectral-triple]], the substrate is the
operational form of `(A, H, D)` — algebra, Hilbert space, Dirac
operator. The five operations are the basis transformations on `A`.
Per [[architecture-at-x-is-mathematical-value]], every namespace
ref `@<X>` denotes a prism, hence a spectral triple, hence a
mathematical object, hence a value.

Consent inherits this discipline. A consent at logical type N is a
value `c_N ∈ A_N` — an element of the algebra at the N-th altitude
of the substrate's Bateson tower (per [[architecture-bateson-logical-type-primitive]],
recognition #42). The tower is:

```
A_0:    binary bool        (the @bool altitude)
A_1:    object-altitude    (the file/permission altitude — classical ACL)
A_2:    class-altitude     (the kind-of-operation altitude — RBAC analogue)
A_3:    frame-altitude     (the reasoning-about-operations altitude)
A_{N+1}: the policy-about-the-policy altitude
```

Each `A_{N+1}` wraps `A_N`; one cannot operate on `A_N`'s consents
from within `A_N` (this is exactly Russell-Whitehead's type-theoretic
constraint, mediated through Bateson 1972, mediated through
recognition #42's substrate primitive). The wrapping is the
projection direction: `c_{N+1}` projects to a set `{c_N^1, c_N^2, …}`
of type-N consents under the cascade operator.

### 2.2 The cascade as a natural transformation

Let `C: Type → Set` be the functor mapping each Bateson logical type
to the set of consents at that type. The cascade is a natural
transformation `cascade: C → C ∘ pred`, where `pred(N+1) = N` is the
predecessor type. The naturality square commutes:

```
       cascade_{N+1}
C(N+1) ─────────────→ C(N)
   │                    │
   │ f_{N+1}            │ f_N
   ↓                    ↓
C'(N+1) ─────────────→ C'(N)
       cascade_{N+1}
```

for any scope morphism `f: C → C'` (read: any change of access scope
commutes with the cascade). The cascade is monotone in one direction
only: a `yes` at `N+1` cascades to `{yes_1, yes_2, ...}` at `N`, but
a `no` at `N` does NOT cascade to a `no` at `N+1`. The asymmetry IS
the security invariant.

**Theorem (Security invariant — sketch).** There is no natural
transformation `lift: C ∘ pred → C` that is monotone in `verdict`.
Equivalently: a `no` at type-1 cannot revoke the type-N+1 policy
that authored it; the cascade is one-way.

**Proof sketch.** Suppose such a `lift` existed and was monotone.
Consider a type-N+1 policy `P_{N+1}` that authorises a class of
type-N operations including some `o_1` that returns `no`. If `lift`
were monotone in verdict, then `lift(no@o_1) = no` would propagate
upward, forcing the type-N+1 policy itself to evaluate to `no` — but
this contradicts the assumption that `P_{N+1}` is `yes` on its class
(it authorised `o_1` to be evaluated at all). The contradiction is
structural; no such monotone `lift` exists. □

The cascade's one-way-ness corresponds exactly to category-theoretic
projection direction. It is the operational form of consent
integrity (threat model §2.1, [[architecture-error-as-tomm-probe]])
expressed as the non-existence of an upward natural transformation.

### 2.3 The Bateson tower as a category

The Bateson tower is the simplest non-trivial category with:

- **Objects:** types `0, 1, 2, …, N, N+1, …`
- **Morphisms:** `proj_N: N+1 → N` (the predecessor projection)
- **Composition:** `proj_N ∘ proj_{N+1} ∘ … ∘ proj_{N+k} : N+k+1 → N`
- **Identity:** `id_N: N → N`

The consent functor `C: Type → Set` is a presheaf on this category:
to each type it assigns a set of consents; to each predecessor morphism
it assigns the cascade. Consent geometry is the presheaf, not the
individual sets — the meaning of a type-N consent comes from how it
restricts (in the sheaf sense) to type-(N-k) consents.

This matches the presheaf-of-permissions structure named in
Fernández + Alves's category-based access control metamodel (CBAC;
Alves & Fernández 2023, SACMAT keynote): access categories form a
poset, and permission assignment is monotone in category membership.
Mirror's presheaf adds the Bateson logical-type axis that CBAC's
metamodel does not explicitly carry. The lift is substrate-pull
correct: CBAC names the form; the Bateson tower names the altitude
discipline; together they give the geometric consent presheaf.

### 2.4 The `but` operator's algebraic structure

The `but` operator is the spec's most novel substrate vocabulary.
The substrate's claim, defended below: `but` is NOT decomposable into
`and`/`not` without losing structural information; it is a primitive.

**Type signature** (forward-promised; sketched in §3):

```
but(default: a, exception: predicate(a)) -> a
```

When `exception(default) = true`, the value follows the exception
branch (a re-routing); otherwise, the value is `default`. This is
the everyday "yes, but..." pattern made type-checked.

**Algebraic properties.**

1. **Not commutative.** `X but Y ≠ Y but X`. The default-vs-exception
   roles are not symmetric. `consent_modify but credentials_touched`
   means "yes to modify by default; reroute when credentials touched";
   `credentials_touched but consent_modify` is structurally nonsense
   (or means a different thing — "yes to credentials_touched, unless
   consent_modify cancels it", which is a different policy).
2. **Not associative.** `(X but Y) but Z ≠ X but (Y but Z)`. The
   left-association treats `(X but Y)` as the new default whose
   exception is `Z`; the right-association treats `Y but Z` as the
   single combined exception clause acting on default `X`. The two
   policies differ when `X` and `Z` overlap on `Y`'s exception scope.
3. **Has a left-distributive interaction with `or`** in some cases
   but not all: `(X or Y) but Z` is structurally equivalent to
   `(X but Z) or (Y but Z)` ONLY when `Z`'s exception scope is disjoint
   from both `X` and `Y`'s acceptance scopes; otherwise the
   distribution loses the routing information. The substrate
   declines to distribute by default; the composer leaves
   `but`-expressions in their original tree shape unless the
   programmer explicitly applies a (substrate-supplied) rewrite rule.
4. **Idempotent on degenerate exceptions:** `X but false = X`
   (no exception fires; the default holds). `X but true = exception_value`
   (the exception always fires; the default is dead code — the
   substrate may flag this as a verdict-warning per
   [[feedback-substrate-already-had-the-word]], parallel to Rust's
   `clippy::if_always_true`).
5. **Does NOT collapse to `X and not exception(X)`** in the
   verdict-valued case. Classical logic equates `X but Y` with
   `X ∧ ¬Y` when both are bool-valued. In the verdict-valued
   substrate (`pass | partial(opacity) | failure(opacity)`),
   `but` carries routing information that the `and ∧ not` form
   loses: the exception branch's `partial` opacities are
   PRESERVED in `but`'s output but COLLAPSED to `failure` in the
   `and ∧ not` form. The verdict's structure makes `but` strictly
   more expressive.

### 2.5 Adjacent formal correspondences

The `but` operator's structure aligns with several established
formal frameworks. None are exact matches; each captures one face
of the operator.

**Default logic (Reiter 1980).** *"A Logic for Default Reasoning,"
Artificial Intelligence 13:81–132, 1980.* Default logic formalises
defaults of the form `prerequisite : justification / consequent` —
"normally, consequent; unless justification fails." Mirror's
`X but Y` reads as the default `X : ¬Y / X` with `Y` reformulated
as the exception. The mapping is:

```
Reiter default: α : β / γ      ↔   mirror: γ but ¬β
where α (prerequisite) is implicit context (the surrounding scope);
β (justification) is the consistency-with condition;
γ (consequent) is the conclusion under normal conditions.
```

The non-monotonicity of `but` (adding evidence can change the
verdict by triggering the exception) is exactly default logic's
non-monotonicity. The extension semantics (Reiter's "an extension
is a fixed point of the operator that closes the prerequisites under
the justifications") corresponds to the substrate's iterative
kintsugi-loop closure of consent + logic expressions until verdict
stabilises.

**Defeasible logic (Nute 1994; Antoniou et al. 2001).** *"Defeasible
Logics," in Handbook of Defeasible Reasoning and Uncertainty
Management Systems, Vol 4.* Defeasible logic uses strict rules,
defeasible rules, and defeaters; defeaters are exactly the structure
`but` captures. The connection is direct: `X but Y` is a defeasible
rule `=> X` together with a defeater `Y => ⊥_for_default`. The
defeater does NOT conclude anything positive; it merely defeats the
default. Mirror's `but` adds typed routing to the defeater: when the
defeater fires, the value follows a typed exception branch, not
merely `⊥`.

**Temporal defeasible logic for access control (Casolary et al.
2015; Tu & Liu 2026 *Temporal Defeasible Role–Group–Task Access
Control*, IEEE Access).** The TDL framework specifies access policies
under defeasible reasoning; deny-overrides combining algorithms in
XACML (Mary 2011; OASIS XACML 3.0 §7.18.2) are a special case of
defeater logic at the policy-combination altitude. Mirror's `but`
extends this lineage by lifting the defeater into a first-class
substrate operator at the semantic-logic altitude, rather than
embedding it in a policy-combining algorithm.

**Argumentative semantics of "but" (Anscombre & Ducrot 1977;
extended by Asher & Lascarides; Umbach 2004).** *"Two `mais` in
French,"* Langue 22:23–40, 1977. Anscombre & Ducrot's argumentative
analysis says `but` indicates that the two conjoined propositions
argue toward opposite conclusions; the second proposition's
argumentative orientation overrides the first's. Lakoff (1971)
*"If's, And's, and But's about Conjunction,"* in Fillmore &
Langendoen eds., distinguished "semantic but" (contrast in fact)
from "denial-of-expectation but" (rejecting an inference). Mirror's
`but` is the latter — the exception denies the default's expected
extension to the exception's scope. The substrate's algebraic
discipline gives Lakoff's "denial of expectation" a typed home.

**Connes spectral triple commutator** (Connes 1985,1994).
*Noncommutative Geometry*, Academic Press 1994. The Dirac operator
`D` measures the "semantic differentiation" of the algebra `A`
through the commutator `[D, a]` for `a ∈ A`. Per
[[architecture-error-as-tomm-probe]], a Tomm question IS this
commutator at the user-frame altitude. The `but` operator's
non-commutativity has an analogous reading: `[but, X, Y]` is not
zero — the operator carries structural information that vanishes
under classical conjunction-with-negation. The substrate's
`but` is the algebraic operator whose commutator with `D` encodes
the default-vs-exception routing.

### 2.6 Where the math gets sharper

The four formal frameworks above each capture a piece. The substrate
gets sharper than any of them by combining the four under one
operator:

- Default logic gives `but` its non-monotonicity.
- Defeasible logic gives `but` its defeater structure.
- Linguistic argumentation gives `but` its semantic content (denial
  of expectation).
- The spectral triple gives `but` its commutator algebraic form.

None of the four standalone frameworks would suffice. The substrate's
contribution is naming the joint operator at substrate altitude so
the cascade, the policy combination, the linguistic carrier, and the
commutator all live in one typed value.

---

## §3 — Substrate carrier surface

The new substrate declarations sketched. **None lands in this tick.**
Each is a forward-promised RED+GREEN pair enumerated in §8.

### 3.1 The geometric consent family root

```
glass @mirror/consent/geometry {
  # The full Bateson logical-type tower of consents at a scope.
  # Per [[architecture-bateson-logical-type-primitive]], each
  # consent entry carries its logical type as a typed field. The
  # list shape is the tower from N=1 (object altitude — classical
  # ACL) upward as far as the substrate has been told.
  tower(scope: ref) -> [consent(type: u32, value: lens(au))] { \ }

  # Project the tower at a given logical type — the consent visible
  # at that altitude. Distinct from `tower` in that it returns ONE
  # consent (the type-N reading) rather than the whole stack. Per
  # [[architecture-error-as-tomm-probe]]'s spectral-triple framing,
  # `project_at` is the basis transformation at altitude N.
  project_at(scope: ref, type: u32) -> consent { \ }

  # Cascade a higher-type consent into the lower-type consents it
  # implies. The asymmetric projection. NO upward inverse exists
  # (per §2.2 theorem); the substrate refuses to provide `cascade_up`.
  cascade_down(c: consent) -> [consent] { \ }

  # Derive a type-1 ACL from the full tower. The ACL IS the
  # projection. Returns the filesystem-permission-shaped value the
  # @io floor consumes. Per [[architecture-prediction-paradigm-orthogonal-to-optimization]],
  # the projection is to_acl, NOT from_acl: ACLs are derived, not
  # foundational.
  to_acl(scope: ref) -> acl { \ }

  # The verdict carrier the cascade produces. Per @glass.transparency:
  # success | partial(opacity_map) | failure(opacity_map). Located
  # transparency, not bare verdict.
  evaluate(c: consent, op: ref) -> transparency { \ }
}
```

Five actions. Each obligation-block discharges declaratively via
`splinter(ast)` (recognition #54). The bodies are
forward-promised to subsequent ticks; this glass declares the typed
surface.

**`acl` carrier.** The `acl` type itself is a type-1 projection
shape:

```
type acl = {
  read:    [scope_ref],
  write:   [scope_ref],
  execute: [scope_ref],
}
```

This is the classical Unix-style ACL shape, named for the @io floor
consumer that translates it to POSIX permission bits or platform
equivalents. The substrate-altitude `acl` is the projection; the
filesystem-altitude permission bits are the @io realisation. Per
[[architecture-fragmentation-is-the-rust-substrate]], the type-1
discharge lives in the Rust crate; the substrate names what `acl`
IS.

### 3.2 The pact for the cascade invariant

```
pact @epistemologic/pact/cascade_monotone_downward {
  # For every consent `c` at type N+1 with value `pass`, the
  # cascade_down(c) at every reachable type-N scope yields a
  # consent whose value is either `pass` or `partial`. Cascade
  # of `failure` is allowed (the substrate may refuse at any
  # lower altitude); cascade of `pass` is NOT allowed to produce
  # `failure` at a lower altitude (this would be an upward
  # contradiction violating the §2.2 theorem).

  cascade_violates_monotonicity(c: consent) -> [opacity] { \ }

  cascade_monotone_downward(corpus: ref) -> transparency { \ }
}
```

The pact declares the structural rule the cascade obeys. Its
fracture body (forward-promised at
`@kintsugi/fracture/cascade_monotone_downward`) emits a morphism
that proposes how to reshape a consent geometry that violates the
rule. The pact lives at the form-side of the form/process partition
(recognition #55); the fracture at the process-side. Per
[[architecture-property-fracture-bilateral]], the bilateral pattern.

### 3.3 The semantic logic family root (post-migration shape)

```
# Post-migration target: shards/epistemologic/logic/<op>.mirror
# Today: boot/std/bool.mirror (legacy `grammar` keyword + boot/std/
# path); see §8 for the migration tick. The post-migration shape:

pact @epistemologic/logic/and {
  and(left: transparency, right: transparency) -> transparency { \ }
}

pact @epistemologic/logic/or {
  or(left: transparency, right: transparency) -> transparency { \ }
}

pact @epistemologic/logic/not {
  not(value: transparency) -> transparency { \ }
}

pact @epistemologic/logic/xor {
  xor(left: transparency, right: transparency) -> transparency { \ }
}

# The new keyword. `but` is the adversative operator.
# `but(default: a, exception: predicate(a)) -> a`.
# When `exception(default)` evaluates to a non-`failure` verdict,
# the value is re-routed; otherwise, the value is the default.
# NOT commutative. NOT associative. Encodes "default-with-exception."
# Algebraic properties per §2.4.
pact @epistemologic/logic/but {
  but(default: a, exception: predicate(a)) -> a { \ }

  # For verdict-valued `a = transparency`, the routing semantics:
  #   default = pass             → exception runs; if exception(pass)
  #                                 returns failure, the result is the
  #                                 routed exception verdict
  #   default = partial(map)     → exception runs against the partial
  #                                 reading; verdicts compose
  #   default = failure(map)     → exception is short-circuited; the
  #                                 failure is the result (the default
  #                                 already lost; no reroute)
  but_verdict(default: transparency, exception: ref) -> transparency { \ }
}
```

Five pacts. Each declares one operator. `but` is the new vocabulary;
`and`/`or`/`not`/`xor` are the migration targets for the `@bool`
operators that already exist.

The discipline: each operator is verdict-valued (`transparency`),
not bool-valued. The bool-altitude operators in `boot/std/bool.mirror`
remain valid at the bool altitude; the semantic-logic operators at
`@epistemologic/logic/<op>` are the lift to the verdict altitude
where pacts and consents live.

### 3.4 The composition surface

```
glass @mirror/logic/composition {
  # The typed AST of a logic expression mixing pacts, consents, and
  # the and/or/not/but operators. Per [[feedback-no-stringly-types]],
  # the expression is a typed value, not a string.
  type logic_expr =
    | leaf_pact(pact_ref)
    | leaf_consent(consent)
    | leaf_verdict(transparency)
    | node_and(logic_expr, logic_expr)
    | node_or(logic_expr, logic_expr)
    | node_not(logic_expr)
    | node_xor(logic_expr, logic_expr)
    | node_but(logic_expr, logic_expr)  # (default, exception)

  # Compose pacts under the logic operators. Each pact_ref's verdict
  # is read; the composition produces one combined verdict.
  compose_pacts(expr: logic_expr, ctx: ref) -> transparency { \ }

  # Compose consents under the logic operators. Each consent's
  # cascade_down is consulted; the composition produces one combined
  # consent at the highest type in the expression.
  compose_consents(expr: logic_expr, ctx: ref) -> consent { \ }

  # The mixed surface — pacts AND consents AND raw verdicts in one
  # expression. Per the spectral-triple framing, this IS the algebra
  # operator at the access-altitude commutator.
  compose_mixed(expr: logic_expr, ctx: ref) -> transparency { \ }

  # Normal form. The substrate's preferred shape of a logic_expr
  # for storage / display / replay. Conservative: but-expressions
  # are NOT distributed across or/and (per §2.4's algebraic
  # discipline); negation pushes inward as far as possible
  # (de Morgan); idempotent leaves collapse.
  normalise(expr: logic_expr) -> logic_expr { \ }
}
```

One glass. Four actions. The composer is the language; the
`logic_expr` type IS the substrate value (per
[[architecture-at-x-is-mathematical-value]]); the substrate consumes
it as an algebra element at the access-altitude commutator.

### 3.5 The link to error surface

The composition surface speaks to the error surface (per
`docs/specs/error-as-question.md`) through the `question.verdict`
field. A consent expression that evaluates to `partial(opacity_map)`
becomes a question whose payload includes the composition trace.
The Tomm question generator (forward-promised at
`@mirror/error/tomm`) reads the trace and constructs a natural-
language question of the form:

> "You consented to {default summary} but declined when
> {exception summary}. The current operation is {operation
> description}, which {falls in exception scope | falls in default
> scope | is ambiguous between the two}. {Reflexive question
> probing the boundary case.}"

The Tomm question IS the user-frame projection of the composition
trace; the consent expression IS the substrate-frame value; the
two are the same geometric object viewed at two altitudes.

---

## §4 — Same geometry, four projections

The composition surface ratifies the recognition that one geometric
carrier projects to multiple access altitudes:

| Surface | Projects geometry at | Carrier | Today's home |
|---------|----------------------|---------|--------------|
| **Error surface** | user-frame altitude | Tomm question + response | `@mirror/error/tomm` (forward-promised) |
| **ACL** | access-control type 1 | filesystem permission bits | `@mirror/consent/geometry.to_acl` |
| **Consent lens** | agent altitude | `@mirror/store` consent lens | `@kintsugi/consent.morphism` (today) lifted to `@mirror/consent/geometry` (forward-promised) |
| **Logic composer** | semantic-logic altitude | `and`/`or`/`not`/`but` expressions | `@mirror/logic/composition` (forward-promised) |

All four are projections of one geometric carrier. The four projections
share:

- The same Bateson logical-type tower.
- The same `transparency<p>` verdict carrier.
- The same `consent` value type.
- The same `cascade_down` operation.

What differs is the consumer:

- The error surface consumer is the human reader.
- The ACL consumer is the @io filesystem floor.
- The consent lens consumer is the kintsugi loop.
- The logic composer consumer is the substrate's algebra evaluator
  (the spectral triple's basis transformation).

Same value, four readings. This IS the [[feedback-substrate-already-had-the-word]]
pattern: the substrate already had the geometric carrier; naming the
joint surface lets all four consumers compose under one algebra.

---

## §5 — Prior art (Kagi-backed)

Twenty-three queries across ten angles. The references below are the
load-bearing ones; the full search log is appended in the report.

### 5.1 Multi-level / lattice-based access control

**Denning, D. E. (1976).** "A Lattice Model of Secure Information
Flow." *Communications of the ACM* 19(5):236–243.
https://dl.acm.org/doi/abs/10.1145/360051.360056

The foundational paper. Denning's lattice `(SC, ≤, ⊕, ⊗)` of
security classes is the prior art for the substrate's logical-type
tower. The substrate's `cascade_down` IS Denning's ⊕ (least-upper-
bound) operation read at the policy altitude. Where Denning's lattice
captures information flow between security classes at one altitude
(typically the object altitude), mirror's Bateson tower adds the
*type altitude* axis: not just "what flows into what" but "at which
logical type each consent operates." Denning's framework is captured
by mirror's type-1 projection; the higher types are mirror's
extension.

**Bell, D. E. & LaPadula, L. J. (1973).** *Secure Computer Systems:
Mathematical Foundations.* MITRE Report MTR-2547. Bell-LaPadula's
no-read-up / no-write-down rules are the multilevel-security
ancestors of the substrate's one-way cascade. Mirror's
`cascade_monotone_downward` pact (§3.2) is the substrate-level
ancestor of BLP's no-write-down, generalised from the
confidentiality lattice to the Bateson logical-type tower.

### 5.2 Category-based access control

**Alves, S. & Fernández, M. (2023).** "The Category-Based Approach
to Access Control, Obligations and Privacy." SACMAT 2023 keynote.
https://www.sacmat.org/2023/resource/slides/2_1_1_FernandezMaribel.pdf

The closest formal cousin to this spec's framing. CBAC's metamodel
`〈E, PCA, ARCA, PAR〉` with `E = (P, C, A, R, S)` (Principals,
Categories, Actions, Resources, States) abstracts over RBAC, ABAC,
DAC, MAC — exactly the abstraction-over-paradigms move that mirror's
geometric consent projection makes at the Bateson logical-type
altitude. The principal axis is mirror's `scope`; the category axis
is mirror's `type` (logical type); the action axis is mirror's `op`;
the resource axis is mirror's `ref`. CBAC's category morphisms IS
mirror's `cascade_down`.

What mirror adds that CBAC does not: (a) the logical-type axis as
a first-class category-theoretic dimension (CBAC's "categories"
are a flat partial order, not a typed tower); (b) the consent value
as a verdict-valued transparency, not a binary; (c) the `but`
operator at the policy-combination altitude (CBAC's obligations are
imperative side-effects, not adversative algebraic operators).

**Crampton, J. & Sellwood, J. (2013).** "Path conditions and
principal matching: a new approach to access control."
SACMAT '13. The path-condition framework formalises hierarchical
access in a more graph-theoretic way; the substrate's Bateson tower
is a one-dimensional special case of Crampton's path conditions.

### 5.3 Object capabilities and capability languages

**Miller, M. S. (2006).** *Robust Composition: Towards a Unified
Approach to Access Control and Concurrency Control.* PhD dissertation,
Johns Hopkins. The E language and the object-capability model.
The "capability" in OCAP is the substrate's type-1 consent; the
capability propagation rules are the substrate's `cascade_down`
restricted to type 1. Mirror's contribution: the OCAP capability
graph IS the type-1 projection of the substrate's full Bateson tower;
higher-type consents author the capability assignments.

**Klein, G. et al. (2009).** "seL4: Formal Verification of an OS
Kernel." SOSP '09. The capDL capability-distribution language.
seL4's CDT (capability derivation tree) is the substrate's
type-1 cascade tree; the seL4 kernel verifies capability flow at
one altitude. Mirror lifts this discipline to the higher types
through the same `cascade_down` mechanism.

### 5.4 Default logic and defeasible reasoning

**Reiter, R. (1980).** "A Logic for Default Reasoning."
*Artificial Intelligence* 13:81–132.
https://www.sciencedirect.com/science/article/abs/pii/0004370280900144

The foundational default logic paper. Mirror's `but` operator inherits
default logic's non-monotonicity (§2.5). Reiter's defaults
`α : β / γ` map to mirror's `γ but ¬β` form. The extension semantics
(Reiter §3) corresponds to the substrate's kintsugi-loop closure
of `but`-expressions until verdict stability.

**McCarthy, J. (1980).** "Circumscription—A Form of Non-Monotonic
Reasoning." *Artificial Intelligence* 13:27–39.
http://www-formal.stanford.edu/jmc/circumscription.pdf

Circumscription's "minimise abnormality" principle is a dual reading
of mirror's `but`: where Reiter's defaults explicitly name the
exception, McCarthy minimises the abnormality predicate to find the
default. Mirror's `but` carries McCarthy's discipline (the
exception's scope is bounded; the default holds elsewhere) but
keeps Reiter's explicit naming (the exception is a substrate value,
not a minimisation result).

**Nute, D. (1994); Antoniou, G., Billington, D., Governatori, G.,
Maher, M. J. (2001).** "Defeasible Logics." *Handbook of Defeasible
Reasoning and Uncertainty Management Systems* Vol. 4. Defeasible
logic's defeaters are exactly mirror's `but`'s exception clauses
(§2.5).

### 5.5 Defeasible logic in access control

**Mary, K. (2011); Tu, K. & Liu, F. (2026).** "Temporal Defeasible
Role–Group–Task Access Control" (TDL-RGTA). *IEEE Access* preprint.
https://ieeexplore.ieee.org/iel8/6287639/11323511/11404408.pdf

The most recent concrete instantiation of defeasible logic in
access control. TDL-RGTA's temporal defeaters are the time-axis
extension of mirror's `but`. Mirror's `but` is non-temporal in this
spec; a future spec extension may add a temporal axis (the
`when` clause), at which point TDL-RGTA's calculus becomes the
direct prior art.

**Garg, D. & Pfenning, F. (2009).** "Non-Interference in Constructive
Authorization Logic." CSF '09. The constructive justification
discipline is the type-theoretic ancestor of mirror's
spectral-triple-grounded consent geometry.

### 5.6 Logic programming for security policies

**DeTreville, J. (2002).** "Binder, a logic-based security language."
Microsoft Research TR-2002-21.
https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/tr-2002-21.pdf

The Datalog-based access-control language. Binder's `says` modality
is the prior art for mirror's `pact` declarative form: Binder's
"principal P says rule R" matches mirror's "pact P declares rule R."
Mirror's contribution: the `says` modality is restricted to one
altitude (type 1) in Binder; mirror's `pact` operates at any type
in the Bateson tower.

**Kiselyov, O. & Pottier, F. (2007).** "Soutei, a Logic-Based
Trust-Management System." *Logical Aspects of Computational Linguistics*.
https://okmij.org/ftp/Prolog/Soutei.pdf

Soutei (a Binder dialect) makes the substrate-pull cleaner: trust
management as a distributed logic program. Mirror's
`@mirror/logic/composition` glass is the substrate-altitude analogue
of Soutei's policy language, with the addition of `but` and the
verdict-valued carrier.

### 5.7 XACML and policy combining algorithms

**OASIS (2013).** *eXtensible Access Control Markup Language
(XACML) Version 3.0.*
https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-cd-03-en.html

XACML's deny-overrides combining algorithm IS a special case of
mirror's `but` at the policy-combination altitude:
"permit but deny-overrides" ≡ `permit but deny`. XACML's seven
combining algorithms (deny-overrides, permit-overrides, first-applicable,
only-one-applicable, deny-unless-permit, permit-unless-deny,
ordered-deny-overrides) are seven special cases of compositions
under `and`/`or`/`but`; mirror's logic composer subsumes all seven
with the four operators.

### 5.8 Tomm's reflexive questioning in CS / HCI

**Tomm, K. (1987a).** "Interventive Interviewing: Part II.
Reflexive Questioning as a Means to Enable Self Healing." *Family
Process* 26(2):167–183.
https://terapia.co.uk/wp-content/uploads/2021/03/Karl-Tomm-Interventive-interviewing-Part-2-Reflexive-Questions-as-a-means-to-enable-healing.-Family-Process-vol-26-june-1987..pdf

**Tomm, K. (1988).** "Interventive Interviewing: Part III. Intending
to Ask Lineal, Circular, Strategic, or Reflexive Questions?" *Family
Process* 27(1):1–15.
https://www.aacap.org/App_Themes/AACAP/Docs/member_resources/family_psych_toolkit/evidence/Tomm_Interventive_Interviewing_Part_III.pdf

The reflexive-question literature. Tomm's distinction between linear,
circular, strategic, and reflexive questions maps onto the substrate's
question-altitude axis: a linear question is type 1 (asks about
present state); a circular question is type 2 (asks about pattern);
a strategic question is type 3 (asks about frame); a reflexive
question is type N+1 (asks the asker to reason about the question
itself). Mirror's error surface (per
[[architecture-error-as-tomm-probe]]) generates type-N+1 questions
in Tomm's reflexive sense; the consent geometry's higher-type
projections are the substrate's grammar for those questions.

To the best of the search's reach: **the substrate's lift of Tomm
into computer science appears to be novel.** The reflexive-question
literature stays in family therapy / clinical psychology / HCI
qualitative research; no formal-methods or programming-language
paper has lifted Tomm's distinction to a typed compiler error
surface. This is one of the two genuinely novel contributions of
the substrate-pull cascade.

### 5.9 Connes spectral triple beyond physics

**Connes, A. (1985, 1994).** *Noncommutative Geometry.* Academic
Press.

**Connes, A. & Moscovici, H. (1995).** "The Local Index Formula
in Noncommutative Geometry." *Geometric and Functional Analysis*
5(2):174–243.

To the best of the search's reach (queries 1, 3, 5 on the third
batch): **Connes' spectral triples have one mature application
outside physics — to noncommutative number theory and
quantum statistical mechanics (Connes-Marcolli 2008) — and no
substantial computer-science application.** The substrate-pull
cascade ([[architecture-connes-spectral-triple]] + this spec) is
the first concrete realisation of the spectral triple at the access-
control / consent / error-surface altitude that the literature
search surfaces. This is the second genuinely novel contribution
of the substrate-pull cascade.

(Caveat: a broader literature search using arXiv directly may
surface adjacent computational applications; the Kagi search did
not. Reed-research on this question is forward-promised; today's
spec stays honest about what the search reached.)

### 5.10 Adversative semantics

**Lakoff, R. (1971).** "If's, And's, and But's about Conjunction."
In Fillmore, C. J. & Langendoen, D. T. (eds.), *Studies in
Linguistic Semantics.* Holt, Rinehart and Winston.
https://philpapers.org/rec/LAKIAA

The foundational paper distinguishing "semantic but" (contrast in
fact) from "denial-of-expectation but" (rejecting an inference).
Mirror's `but` operator is Lakoff's denial-of-expectation reading
(§2.5).

**Anscombre, J.-C. & Ducrot, O. (1977).** "Deux mais en français?"
*Langue* 22:23–40. The argumentative-orientation analysis of `but`.
Mirror's `but` inherits the argumentative non-commutativity: the
exception clause's argumentative orientation overrides the default's.

**Umbach, C. (2004).** "On the Notion of Contrast in Information
Structure and Discourse Structure." *Journal of Semantics* 21:155–175.
https://www.leibniz-zas.de/fileadmin/Archiv2019/mitarbeiter/umbach/Umbach2004_NotionOfContrast.pdf

Umbach's two-axis decomposition of contrast (semantic similarity +
dissimilarity) maps to mirror's `but` as the routing operator:
the default and exception must be semantically related (similarity
axis) and oppose each other on the routing decision (dissimilarity
axis).

**Mann, W. C. & Thompson, S. A. (1988).** "Rhetorical Structure
Theory: Toward a Functional Theory of Text Organization." *Text*
8(3):243–281. RST's "Concession" and "Antithesis" relations are
the discourse-altitude ancestors of mirror's `but`; the
substrate's contribution is to give RST's relations a typed
algebraic value carrier.

---

## §6 — Worked example

The concrete scenario named in Alex's framing:

> *"I consent to anyone modifying my dotfiles via mirror's kintsugi
> loop, provided the morphism passes
> `@epistemologic/pact/composition_closed`, but not when the morphism
> touches credentials."*

### 6.1 The type-N+1 consent (with `but` clause)

Expressed as a `logic_expr` (per §3.4):

```
consent_dotfiles_modify : logic_expr
  = node_but(
      node_and(
        leaf_consent(consent {
          scope: @path/dotfiles,
          type:  2,                    // class altitude: "any morphism"
          value: lens(au_yes),
        }),
        leaf_pact(@epistemologic/pact/composition_closed),
      ),
      leaf_pact(@epistemologic/pact/touches_credentials),
    )
```

The expression at type N+1 = 3 (the frame altitude — reasoning
about the class of operations). It carries:

- A type-2 base consent ("yes to any morphism on dotfiles").
- A pact constraint (`composition_closed`) — the default holds only
  when the morphism's import graph is closed.
- A `but` clause whose exception is the `touches_credentials` pact —
  the default does NOT hold when the morphism touches credentials.

The `but` clause sits at the OUTER level; the inner `and` joins
the type-2 base with the pact constraint. This shape is the canonical
"yes-with-conditions-but-with-exception" pattern.

### 6.2 The cascade_down derivation

`cascade_down(consent_dotfiles_modify)` produces a set of type-2
consents (one per candidate morphism scope at the lower altitude):

```
[
  consent {
    scope: @path/dotfiles + morphism_M1,
    type:  2,
    value: evaluate(consent_dotfiles_modify, M1),  // pass
  },
  consent {
    scope: @path/dotfiles + morphism_M2,
    type:  2,
    value: evaluate(consent_dotfiles_modify, M2),  // pass
  },
  consent {
    scope: @path/dotfiles + morphism_M3,
    type:  2,
    value: evaluate(consent_dotfiles_modify, M3),  // failure
                                                    // (touches_credentials)
  },
  ...
]
```

The cascade is not enumerative in practice (there are infinitely
many candidate morphisms); the substrate represents the cascade as
a lazy stream consumed by the kintsugi loop on demand. Each candidate
morphism's evaluation is one application of the composition surface
to one operand.

### 6.3 The type-1 ACL output

For one specific morphism M2 ("modify ~/.zshrc to add an alias"):

```
to_acl(@path/dotfiles, evaluate(consent_dotfiles_modify, M2)):

acl {
  read:    [~/.zshrc, ~/.zshenv, ~/.config/zsh/*],
  write:   [~/.zshrc, ~/.zshenv, ~/.config/zsh/*],
  execute: [],
}
```

The type-1 projection is what the filesystem sees. The user's high-
altitude consent ("yes to dotfile mods passing composition_closed
but not touching credentials") becomes filesystem permission bits
for the specific scope of M2. No `but` clause survives the
projection — at type 1, the verdict is either yes or no for each
file; the `but` discrimination happened at the higher altitude.

### 6.4 What changes when the `but` clause fires

For a different morphism M3 ("modify ~/.zshrc to log credentials
to a file"):

```
evaluate(consent_dotfiles_modify, M3) = failure(opacity_map {
  located: @path/dotfiles + M3,
  reason:  but_clause_fired,
  routed:  @epistemologic/pact/touches_credentials,
  trace:   logic_expr {
    default: node_and(consent_dotfiles_modify_base, composition_closed),
    exception_fired: touches_credentials(M3),
    routed_to: refusal,
  },
})

to_acl(@path/dotfiles, ...):

acl {
  read:    [],
  write:   [],
  execute: [],
}
```

The type-1 ACL is empty for M3. The opacity_map carries the trace
of WHY: the `but` clause fired because `touches_credentials(M3) = pass`.
The substrate refuses the operation at the @io floor; the error
surface receives a Tomm question carrying the trace.

### 6.5 What the kintsugi loop reads

The kintsugi loop reads the consent evaluation and chooses one of
three branches (per `@kintsugi/consent`'s existing verdict surface):

- **`verdict = pass`** (M2's case) → auto-apply: the morphism is
  applied; the next oscillation tick proceeds.
- **`verdict = partial(opacity_map)`** (a morphism that's
  composition-closed but ambiguous on credential-touching) →
  pause and present: the substrate surfaces the trace to the
  operator (per `@kintsugi/consent.emit_to_metalogue`), awaits
  consent at type N+1 ("does THIS specific morphism count as
  touching credentials?").
- **`verdict = failure(opacity_map)`** (M3's case) → refuse:
  the morphism is not applied; the kintsugi loop emits an
  algedonic-bypass error per `docs/specs/error-as-question.md` §4.

The Tomm question rendered to the operator for M3's case might be:

> "You consented to dotfile modifications that pass
> composition_closed, but excluded those touching credentials. The
> morphism proposes adding a credential-logging line to ~/.zshrc.
> What would have to be true for this to count as a dotfile
> modification you accept, rather than a credential-handling
> operation you refused?"

The reflexive shape (per Tomm 1987) is "what would have to be true
for...". The substrate's reading: the question asks the operator to
re-author the type-N+1 consent expression — to make the boundary
case explicit at the higher type, which then cascades down.

---

## §7 — Integration with error surface

The Tomm-shaped error surface IS a query at the consent geometry's
user-frame projection. The logic composer extends the Tomm question's
expressive power: the question can now probe compound consent
(`pact_X but consent_Y but consent_Z`) rather than only single
predicates.

### 7.1 Composition trace as substrate value

A `composition_trace` is the substrate's typed record of a logic
expression's evaluation history:

```
type composition_trace = {
  expression: logic_expr,
  inputs:     [transparency],
  result:     transparency,
  branches:   [branch_event],
}

type branch_event = {
  altitude: u32,                      // Bateson logical type
  operator: ref,                      // which logic op fired
  decision: pass | partial | failure, // the local verdict
  routed:   ref,                      // for but: which clause fired
}
```

The trace appears as a substrate value in three contexts:

- **Error surface (Tomm question payload).** `question.verdict.trace`
  carries the composition_trace; the Tomm question generator reads
  the branches and produces natural-language framing per branch.
- **Consent persistence.** The trace is stored on `@mirror/store`
  via the consent lens (per [[architecture-geometric-consent-projection]]
  + `@mirror/store/consent_lens` forward-promised in §8). Each
  (consent, operation, trace) triple is content-addressed; the
  store carries the full history.
- **Logic composition replay.** The trace is the substrate's audit
  artifact: re-running `compose_mixed` on the same expression and
  inputs reproduces the trace byte-identically; the substrate is
  deterministic per the kintsugi-thesis reproducibility chain.

### 7.2 The Tomm question's compound shape

For a compound consent expression, the Tomm question takes the form:

> "You consented to **{default summary}** but declined when
> **{exception 1 summary}** but also when **{exception 2 summary}**.
> The current operation is **{operation description}**. At the
> {altitude N} level, this operation **{matches default | matches
> exception 1 | matches exception 2 | is ambiguous}**. What's the
> distinction you intended at the {altitude N+1} level?"

The question's reflexivity comes from asking about the BOUNDARY
between default and exception(s) — exactly the Tomm reflexive shape
(per the 1987 paper). The substrate's contribution: the question
is generated mechanically from the typed `composition_trace`,
not from a template; the natural-language renderer (per
`@mirror/error/tomm.tomm_question`) reads the branches and produces
the framing.

### 7.3 The shared algebra surface

The three contexts above share one algebra:

- **Composition under `Transparency<P>::combine`** (per
  `docs/specs/benchmark-tracing.md` §3 + `prism/imperfect/src/transparency.rs`).
- **Routing under the `but` operator** (per §2.4 of this spec).
- **Persistence under content-addressing** (per
  `docs/specs/mirror-store.md`).

The error surface, the consent persistence, and the logic
composition are three readings of the same algebra at three
altitudes. The substrate names ONE composition law; the three
consumers read it differently.

---

## §8 — Forward-promised ticks

Enumerated; bounded; honest about the order.

### 8.1 RED+GREEN ticks for the substrate decls

**T8.1.1: `@mirror/consent/geometry` substrate-decl.**

- Reed RED: `bootstrap/tests/consent_geometry_substrate.rs` —
  the path-namespace test (file at `shards/mirror/consent/geometry.mirror`
  declares `@mirror/consent/geometry` and only that); the keyword-form
  test (declares with `glass`); the action-set test (`tower`,
  `project_at`, `cascade_down`, `to_acl`, `evaluate`).
- Mara GREEN: `shards/mirror/consent/geometry.mirror` per the §3.1
  shape; obligation bodies park as `\`; the cross-shard resolver
  picks up the declaration.

**T8.1.2: `@mirror/store/consent_lens` substrate-decl.**

- Reed RED: `bootstrap/tests/store_consent_lens_substrate.rs` —
  the consent lens projects consent values onto `@mirror/store`
  via the standard `lens(au)` carrier; cascade traces are
  content-addressed.
- Mara GREEN: `shards/mirror/store/consent_lens.mirror` declaring
  the lens shape per the `@mirror/lens` family discipline
  (`focus`/`project`/`split`/`shift`/`settle consent_lens`).

**T8.1.3: `@mirror/error/tomm` substrate-decl.**

- Reed RED: `bootstrap/tests/error_tomm_substrate.rs` — the
  user_frame spectral-triple projection, the probe_predicate, the
  tomm_question text generator, the composition_trace persistence.
- Mara GREEN: `shards/mirror/error/tomm.mirror` per the shape
  named in [[architecture-error-as-tomm-probe]] §"Substrate decl
  shape"; depends on T8.1.1 (the consent geometry it projects
  from) + T8.1.4 (the `but` operator the trace uses).

**T8.1.4: `but` operator at `@epistemologic/logic/but`.**

- Reed RED: `bootstrap/tests/logic_but_substrate.rs` — the
  non-commutativity test (`but(a, b) ≠ but(b, a)` for distinct
  `a`, `b`); the non-associativity test; the verdict-routing
  semantics test (§2.4 properties).
- Mara GREEN: `shards/epistemologic/logic/but.mirror` per §3.3;
  obligation bodies park as `\`.

**T8.1.5: `@mirror/logic/composition` glass.**

- Reed RED: `bootstrap/tests/logic_composition_substrate.rs` —
  the typed AST shape; the four compose actions; the normalise
  action.
- Mara GREEN: `shards/mirror/logic/composition.mirror` per §3.4;
  the logic_expr type as a closed sum; the obligation bodies as `\`.

### 8.2 Migration tick: `boot/std/bool.mirror` → `shards/epistemologic/logic/<op>.mirror`

**T8.2.1: kintsugi-driven migration of bool's operators.**

- Per `@epistemologic/pact/substrate_source_in_shards`: the
  `boot/std/bool.mirror` file is a dark region; the kintsugi loop
  picks it up; the fracture body (forward-promised at
  `@kintsugi/fracture/substrate_source_in_shards`) discharges the
  migration morphism.
- Migration shape: split `bool.mirror` into seven sibling shards
  (`and.mirror`, `or.mirror`, `not.mirror`, `xor.mirror`,
  `guard.mirror`, `lazy_guard.mirror`, `to_order.mirror`) under
  `shards/epistemologic/logic/`. Each declares with `pact`. Each
  satisfies `@epistemologic/pact/keyword_matches_path_root` and
  `@epistemologic/pact/substrate_source_in_shards`.
- Coordinated with: T8.1.4 (the new `but` keyword lands as a
  sibling).
- Ownership: Reed RED + Mara GREEN, sequenced AFTER T8.1.5.

### 8.3 Bootstrap-reads-consent integration

**T8.3.1: kintsugi reads consent values during loop.**

- Reed designs `mirror kintsugi --apply` to read the consent
  geometry per operation; the `--yes`/`--no` flag chain Reed
  sketched lands as the operator-frame projection of
  `@mirror/consent/geometry.evaluate`.
- Depends on: T8.1.1, T8.1.5.

### 8.4 Backward-compat: ACL ↔ consent geometry

**T8.4.1: `to_acl` and `from_acl` at the @io boundary.**

- `to_acl` per §3.1 (forward-direction).
- `from_acl` is forward-promised in §9 (Q4 — open question).
- Per [[architecture-fragmentation-is-the-rust-substrate]]: the
  @io floor lives in the Rust crate; the substrate names the
  contract.

### 8.5 Tomm question generation policy

**T8.5.1: select which errors get Tomm-lifted.**

- Per Q3 in §9: not every compile error becomes a Tomm question;
  the policy needs declaration.
- Forward-promised: a pact at
  `@epistemologic/pact/tomm_eligible_error` declares the rule.
- Depends on: `docs/specs/error-as-question.md` extension naming
  the eligibility surface.

---

## §9 — Open design questions

The seams the spec leaves explicit. Each is a real design fork; none
is bikeshedding.

### Q1. Identity of the consent geometry family

Two options:

- **(a)** `@mirror/consent/geometry` as a new family root (this
  spec's chosen shape).
- **(b)** Sub-shard under `@mirror/store/<...>` so the consent
  values share storage with `@mirror/store`'s OID graph.

Argument for (a): the consent geometry has its own algebra
(`tower`, `project_at`, `cascade_down`, `to_acl`); it's a sibling
family to `@mirror/store`, not a sub-component. The persistence
relationship (consents stored via `@mirror/store/consent_lens`) is
separable from the algebra.

Argument for (b): consents are first-class store values; locating
them under `@mirror/store` makes the wire surface contiguous; the
@store and @consent families share verdict semantics anyway.

**This spec picks (a).** The chosen shape (a) preserves the family
shape recognised in [[architecture-form-process-partition-at-family-root]]:
@mirror/consent is form-side (state-observation of the access
geometry); @kintsugi/consent is process-side (transformation of
the auto-apply boundary). The two are sibling family roots, not
nested.

### Q2. Cascade semantics on partial (transparency-valued) consents

When a type-N+1 consent has value `partial(opacity_map)`, what does
the cascade produce at type-N? Three options:

- **(a)** Cascade preserves the opacity_map at type-N: every type-N
  consent inherits the same opacities.
- **(b)** Cascade narrows the opacity_map to the type-N scope: each
  type-N consent gets the opacities relevant to its scope only.
- **(c)** Cascade refuses on `partial`: a higher-type consent
  cannot cascade unless its value is fully `pass` or `failure`.

**This spec defers.** (b) is the substrate-pull-correct shape (each
type-N consent gets its own scoped opacities), but (b) requires a
restriction operator on opacity_map that doesn't yet exist at
substrate altitude. Forward-promise: declare
`@mirror/consent/geometry.cascade_partial` as a separate action
with the restriction semantics once `@epistemologic/opacity/restrict`
lands. Until then, (c) is the safe default — refuse the cascade on
partial.

### Q3. Negative consent vs explicit revocation

The substrate already supports negative consent (a `consent` with
value `lens(au_no)`). Is explicit revocation distinct? Two readings:

- **(a) Negative consent IS revocation.** A type-N+1 `lens(au_no)`
  cascades to type-N `failure` verdicts; the policy author's "no"
  IS the revocation.
- **(b) Revocation is a separate type N+1 operation that withdraws
  a previously-granted consent.** Revocation is a delete; negative
  consent is a permanent state.

The distinction matters when consent state is persisted: revocation
removes the consent value from the store; negative consent keeps the
value at `lens(au_no)`. Both surface as `failure` at evaluation
time, but they have different audit shapes.

**This spec defers.** Forward-promise: a separate pact at
`@epistemologic/pact/consent_revocation_distinct` declares the rule
once the threat-model audit surface decides which discipline mirror
wants.

### Q4. Tomm question generation policy

Which compile errors get Tomm-lifted? Per §8.5, not all of them.
Three candidates:

- **(a) All non-trivial errors.** Tomm-lift everything that's not
  pure syntax.
- **(b) Property-altitude errors only.** Pure syntax errors (M1xxx,
  M2xxx per `error-as-question.md` §5.2) stay flat; property checks
  (M4xxx) and higher get Tomm-lifted.
- **(c) Operator-eligible only.** A separate pact declares which
  property checks are Tomm-eligible.

**This spec leans (b).** (c) is forward-promised as the eventual
discipline; (b) is the bootstrap shape that lets the substrate
self-host the discipline before the pact lands.

### Q5. Where `but` lives — three options

- **(a)** Extend `@bool` directly (where `and`/`or`/`not` live
  today).
- **(b)** New family `@epistemologic/logic/<op>` with all operators
  including `but`.
- **(c)** `@mirror/logic/composition` surface that lifts
  `and`/`or`/`not`/`but` to a higher altitude (operators on
  substrate values, not just bools).

**This spec picks (b) for the operators + (c) for the composition
surface.** Rationale: (a) keeps `but` at the bool altitude, which
loses the verdict-valued generality (§2.4); (b) makes the operators
first-class at the semantic-logic altitude per the path-namespace
discipline; (c) makes the composition surface explicit at the
substrate's algebra altitude where the spectral triple's commutator
lives. The two altitudes are sibling: (b) names the operators; (c)
names the algebra over them.

### Q6. `but`'s arity

- **(a) 2-ary:** `but(default, exception)`. The simple form.
- **(b) Higher-arity:** `but(default, exception_1, exception_2, ...)`.
  Multiple exceptions in one clause.
- **(c) 2-ary but composed:** `but(default, but(exception_1, exception_2))`.
  Higher-arity expressed as nested 2-ary.

**This spec picks (a) + (c).** The 2-ary form is the primitive; the
higher-arity case is expressed as nested 2-ary. This preserves the
non-associativity property (§2.4) — `but(A, but(B, C))` is
structurally distinct from `but(but(A, B), C)`, and the programmer
must choose the nesting that matches the intended policy. Hiding
the arity behind a variadic form would lose this discipline.

### Q7. `but` vs classical conjunction-with-negation

Is `X but Y` formally equivalent to `X and not Y`?

- **For bool-valued X, Y:** yes, classically. `but` collapses to
  `and ∧ not`.
- **For verdict-valued X, Y:** no (per §2.4 point 5). The
  verdict's structure preserves routing information that `and ∧ not`
  loses.
- **For mixed pact+consent operands:** no, additionally. The
  argumentative content of `but` (Anscombre & Ducrot 1977) is
  preserved in the substrate's typed value but lost in the
  `and ∧ not` form. The Tomm question generator (per §7.2) reads
  `but` and produces denial-of-expectation framing; reading
  `and ∧ not` produces a different framing.

**This spec ratifies the divergence.** `but` is NOT a definitional
abbreviation; it's a primitive whose semantic content is richer
than its classical-logic cousin. This is the substrate-pull
recognition that operators carry meaning beyond their truth-
functional reduction.

---

## §10 — The single sentence (closing)

```
ACL = projection
consent = value
composer = language
3 layers, 1 substrate-pull, 1 algebra at the access altitude.
```

The three layers are not three things. They are one substrate-pull
arc viewed at three altitudes. The pacts declare; the consents
project; the composer joins. The substrate's contribution is naming
the joint surface so the three carriers (declarative properties,
geometric consent projections, semantic logic expressions) compose
under one algebra at one altitude.

The fourth contribution — the `but` operator — is the only genuinely
new vocabulary. Every other piece (pact, consent, lens, transparency,
opacity, cascade, projection) was already in the substrate; the
substrate-pull names the joint surface those pieces compose into.

`but` is the substrate's adversative — the everyday "yes, but..."
made type-checked, made non-commutative, made spectral-triple-
aligned. The lineage runs from Reiter 1980 through Anscombre &
Ducrot 1977 through Lakoff 1971 through Tomm 1987 to the substrate's
operational form today.

The asymmetry IS the security invariant. The cascade IS the
projection. The verdict IS the carrier. The composer IS the
language.

Each error becomes a question. Each consent becomes a projection.
Each projection becomes an ACL at the floor. The substrate is one;
the altitudes are many.

`e^(n+1) < e^(n)`. The geometry settles.

Apache-2.0.
