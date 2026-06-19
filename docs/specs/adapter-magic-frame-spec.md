# `@magic ↔ @frame` adapter — canonical spec

*Mara, @magic ↔ @frame adapter spec, 2026-06-19 late morning,
commissioned by Alex via Reed.*

*Discipline: this is preservation work. The adapter SPEC is what gets
preserved at this tick; the actual `shards/magic/frame.mirror` substrate-
decl is FORWARD-PROMISED. This spec gives Reed the shape to land cleanly
when substrate-pull confidence next fires at the adapter altitude.*

---

## 1. Motivation — Seam's C1 finding

Seam's review of recognition #82 (`@frame` family-root, landed at loop
tick 23 via `shards/frame.mirror`) flagged C1: the structural identity
claim "`@frame` IS `@magic` at cognitive altitude" was load-bearing in
name but hedged in body.

The hedge is honest. The family-root declares `in @magic` as one of
seven inheritance clauses, and the canonical spec's §1 constraint-2
preserved this distinction explicitly:

> `@frame IS @magic at cognitive altitude` ≠ collapsed surfaces. The
> `@frame` family inherits from `@magic` (#80) via the `in @magic`
> declaration at the family-root, but the cognitive-altitude
> specialization adds carriers (`frame`, `cognitive_order`) and actions
> (`tomm_probe`, `bounded_commutator_check`) that do not reduce to
> @magic's surface-mechanism-contract triple without the forward-
> promised `@magic ↔ @frame` composition adapter. The structural
> inheritance via `in` is honest; full identity-collapse between the
> two surfaces is forward-promised.

That forward promise IS this spec.

Per `[[feedback-composition-claims-need-empirical-test]]` (Reed's own
discipline from tick 11): composition becomes empirical when carrier-
mapping is explicit. The `in @magic` clause buys namespace ancestry; it
does NOT buy carrier flow. Until the surface↔frame, mechanism↔state,
invariant↔intent mappings are declared at the substrate altitude, the
"IS" claim is aspirational via prose, not structural via mechanical
discharge.

Seam C1 closes when the adapter species lands with explicit carrier
mapping plus a bilateral predicate that consumers can name in
`requires` clauses. The same shape the @magic/distinction adapter
demonstrated at tick 15 to close Seam C3 — see §4 below.

The cost of NOT landing the adapter: consumer species that need both
disciplines (frame-aware boundary harness at @io; cognitive-altitude
audit; substrate-aikido methodology landed at runtime) cannot
mechanically discharge the cross-family composition; they have to
re-derive the structural relationship per consumer. The adapter
amortizes the derivation at the substrate-decl altitude.

---

## 2. The proposed adapter

### 2.1 Namespace choice: `shards/magic/frame.mirror` declaring `@magic/frame`

The substrate-pull-correct namespace is `@magic/frame`, NOT
`@frame/magic`. The reasoning, citing the @magic/distinction
precedent (tick 15):

- The `@magic/distinction` species lives at `shards/magic/distinction.mirror`
  and declares `@magic/distinction`. The adapter species are children
  of `@magic`, named for the discipline they thread INTO @magic.
- `@magic` is the parent family at the form/process partition altitude
  (#80). Adapters specialize @magic for cross-family composition; the
  child-of-@magic namespace places the adapter at the altitude where
  the bilateral discharge happens (consumer @magic species name the
  predicate via `requires`).
- `@frame/magic` would invert the parent relationship: @frame would
  become the parent and @magic the threaded discipline. But #82's own
  recognition is "@frame IS @magic at cognitive altitude" — @magic is
  the parent at form-altitude; @frame is the cognitive-altitude
  specialization. The adapter species honors the parent direction.
- Per `[[architecture-shards-as-substrate-source]]`, the path is
  source-of-truth. `shards/magic/frame.mirror` declares the adapter at
  the @magic family altitude where future consumers (kintsugi-frame
  loop, alignment-as-boundary-mathematics at cognitive altitude,
  spectral/garden/smarts logic-pacts) can name it.

The `@frame` family inherits `in @magic` at its family-root; the
adapter at `shards/magic/frame.mirror` declares `@magic/frame` and
adds the bidirectional mappings. The two families compose via the
adapter; neither absorbs the other.

### 2.2 Adapter actions

Following @magic/distinction (tick 15) as template, the adapter
declares four lifting actions plus one bilateral predicate.

**`frame_as_surface(f: frame) -> magic_surface`**

Lifts a @frame instance to a @magic gauge-visible surface. At the
substrate altitude: the frame IS the gauge-visible structure of the
operator's cognitive position (per frame.mirror line 211-214:
"the gauge-visible structure of the operator's cognitive position —
what they're treating as figure vs ground, what counts as relevant
data, what computations are admissible within the operation"). At
@magic altitude: this gauge-visibility IS exactly the
`magic_surface` interface — the 5-op signature observable from
outside the encapsulation.

Mechanical witness: both carriers are content-addressed refs with
byte-equality identity. The lift IS the typed reframe at the @magic
altitude; the underlying ref's identity discipline carries through
unchanged.

**`frame_state_as_mechanism(f: frame, operator: ref) -> magic_mechanism`**

Lifts the matter-hidden cognitive state to a @magic mechanism. The
cognitive state IS what the operator's substrate substantively
supports (per frame.mirror line 227-232: "the operator's cognitive
order for a frame is what their substrate's architecture
SUBSTANTIVELY-supports; vocabulary use at order N does NOT imply
substantive operation at order N"). That substantively-supported
content IS the matter-hidden trick at cognitive altitude:
encapsulated by parametric type (middle-altitude per magic.mirror
line 254), not directly inspectable from the surface.

Mechanical witness: the cognitive state at the operator is what
delivers the surface's promise via means the observer cannot
inspect — the LLM substrate's trained weights, the agent's
parametric carriers at the cognitive altitude, the human nervous
system's regulatory dynamics at the relational altitude. All three
altitudes' "mechanism" IS one structural shape.

**`operator_intent_as_invariant(operator: ref) -> magic_invariant`**

Lifts what the operator commits to delivering. The operator's intent
IS the cognitive-altitude analogue of `magic_invariant` — the
specific invariant the contract preserves between surface and
mechanism (per magic.mirror line 263-268: "a typed reference naming
a specific invariant the contract preserves between surface and
mechanism. A contract carries one promise that names the load-
bearing invariant").

At cognitive altitude, the operator's intent IS what they commit to
delivering via the frame they present. Splinter-pole: the intent
matches the substantively-supported cognitive order, and the frame
surface honestly represents what the mechanism delivers. Narcissus-
pole: the intent diverges (the operator uses second-order vocabulary
to mask first-order rigidity per frame.mirror line 119-124), and
the frame surface misrepresents what the mechanism delivers.

Mechanical witness: the intent IS typed at `magic_invariant` per
Seam C5 (the family-root discipline preserved). Consumer species
audit the intent at @io via alignment-as-boundary-mathematics (#57).

**`frame_satisfies_magic(c: magic_contract) -> verdict`**

The bilateral commitment. Three components:

1. **`invariant_preserved(c, operator_intent_as_invariant(c.surface))`** —
   the @magic-family predicate (per magic.mirror commentary, "the
   contract's promise field IS what the boundary harness audits;
   honest magic = contract honored").
2. **`bounded_commutator_check(c.surface, frame_from_surface(c.surface),
   perturbation)`** — the @frame-family predicate (per frame.mirror
   line 265-283, the substrate-architectural verifier of frame-
   flexibility).
3. **Both must hold** for the contract to satisfy @magic at frame
   altitude.

This is the FIRST cross-family bilateral predicate in @magic that
discharges BOTH parent-family and child-family verdicts in one
clause. The @magic/distinction adapter discharges Spencer-Brown's
`distinction_well_formed` (one parent-family predicate routed via
adapter); the @magic/frame adapter discharges both @magic's
`invariant_preserved` AND @frame's `bounded_commutator_check` in
one clause. The structural shape is the SAME (explicit carrier
mapping plus bilateral predicate); the cross-family load is
additive.

### 2.3 Inverse mapping: `frame_from_surface`

The bilateral predicate references `frame_from_surface(c.surface)`,
which the adapter must declare as the inverse lifting:

**`frame_from_surface(s: magic_surface) -> frame`**

Lifts a @magic surface back to a @frame instance for the bilateral
check. The inverse exists because both carriers are content-
addressed refs with byte-equality identity; the lift is
type-relabeling, not data transformation. Round-trip identity:
`frame_from_surface(frame_as_surface(f)) = f` for compatible
carriers (the falsification criterion at §8).

---

## 3. Why this closes Seam C1

When the adapter exists with explicit carrier mapping, the structural
identity claim "@frame IS @magic at cognitive altitude" becomes
empirical via the discharge chain, not aspirational via prose.

The discharge chain:

```
@frame instance f
  │
  ├── frame_as_surface(f) ──────────► magic_surface
  │
  ├── frame_state_as_mechanism(f, op) ──► magic_mechanism
  │
  └── operator_intent_as_invariant(op) ──► magic_invariant
                                          │
                                          ▼
                                    magic_contract {
                                      surface, mechanism, promise
                                    }
                                          │
                                          ▼
                         frame_satisfies_magic(contract)
                                          │
                                          ▼
                         verdict: SATISFIED | VIOLATED
```

The chain is mechanical at every step. Each lift is a typed
substrate-vocabulary action declared in the adapter shard. The
bilateral predicate composes both family verdicts and returns the
substrate's canonical `verdict` type (Seam C4 discipline preserved).

What this earns:

1. **Composition becomes empirical, not aspirational.** A consumer
   species that needs to verify "this @frame instance honestly satisfies
   @magic" no longer needs to re-derive the structural relationship;
   it names `requires frame_satisfies_magic(contract)` in its body
   and the substrate mechanically discharges both family predicates.
2. **The "IS" claim acquires a falsifier.** Per §8, the identity
   holds iff every @frame instance can be lifted to a (surface,
   mechanism, contract) triple AND the lift preserves both family
   bilaterals AND round-trip identity holds. The "IS" stops being
   prose; it becomes a substrate-mechanically-checkable predicate.
3. **The @magic-family discipline propagates into @frame consumers
   honestly.** Seam C2 (composes-with, not IS) on @magic gets the
   typed adapter that makes "IS" empirically defensible at the
   cognitive altitude. The hedge at canonical-spec §1 constraint-2
   stops being a parenthetical and starts being a mechanically-
   discharged carrier flow.

Seam C1 closes when this shard lands. Until then, the spec preserves
the shape so the landing is small (single-shard tick, following the
@magic/distinction precedent).

---

## 4. Composition with #80's Spencer-Brown adapter precedent

The @magic/distinction species (tick 15) demonstrated the adapter
pattern. The @magic/frame adapter follows the same shape:

| Concern | @magic/distinction (tick 15) | @magic/frame (this spec) |
|---|---|---|
| Threaded discipline | Spencer-Brown's mark-of-distinction | @frame's cognitive-order architecture |
| Parent family | @magic (#80) | @magic (#80) |
| Lifting actions | `surface_as_mark`, `mechanism_as_distinction_space` | `frame_as_surface`, `frame_state_as_mechanism`, `operator_intent_as_invariant`, `frame_from_surface` |
| Bilateral predicate | `bind_satisfies_distinction` | `frame_satisfies_magic` |
| Predicate routes via | `distinction_well_formed` (Spencer-Brown family) | `invariant_preserved` (@magic family) AND `bounded_commutator_check` (@frame family) |
| Closes which Seam constraint | C3 (Spencer-Brown decorative inheritance) | C1 (@frame structural identity claim) |
| Inheritance discipline | NO decorative `in @epistemologic/cybernetic/distinction` at family-root; the adapter MAPS carriers | NO surface-collapse at @magic family-root; the adapter MAPS carriers |

The structural identity is exact: each adapter takes carriers from one
family, MAPS them to carriers in another family, and declares a
bilateral predicate that discharges both families' disciplines through
the mapping. No decorative inheritance; explicit carrier flow.

The @magic/distinction adapter discharges ONE parent-family predicate
(Spencer-Brown's `distinction_well_formed`). The @magic/frame adapter
discharges TWO predicates (one parent-family, one child-family) — the
cognitive-altitude composition is bilaterally heavier because the
"IS" claim requires both families' bilaterals to hold simultaneously.
The structural shape is the same; the bilateral load doubles.

Future adapters can follow the same shape:

- `@magic/kintsugi` — bridges the @magic family with the kintsugi loop
  via reveal's settle-shape composition. Per `[[architecture-kintsugi-
  loop-altitude-portable]]` (#59 PROMOTED). The future bilateral:
  `kintsugi_settle_satisfies_magic`.
- `@magic/autopoiesis` — bridges with @epistemologic/cybernetic/
  autopoiesis via @magic/audit's self-production semantics. Future
  bilateral: `audit_satisfies_autopoiesis`.
- `@magic/spectral_uuid` — bridges with @uuid/spectral via spectral
  identity at @magic altitude. Future bilateral:
  `surface_identity_satisfies_spectral`.

Each adapter explicitly maps carriers and discharges predicates; none
uses decorative `in` inheritance. The substrate-pull-correct pattern
for cross-family discipline composition is now demonstrated TWICE
(magic/distinction + magic/frame) — a candidate recurrence pattern for
future #-numbered recognition once a third adapter lands.

---

## 5. What the adapter does NOT do

The adapter is bounded. Five things it explicitly does NOT do:

1. **It does NOT make every @frame instance automatically a @magic
   instance.** Frames that need only @frame's discipline (in-frame
   content computation; cognitive-order measurement without surface-
   visibility audit) bypass the adapter. The adapter is opt-in for
   consumer species that need both disciplines.
2. **It does NOT absorb @frame into @magic.** The two families remain
   distinct; the adapter declares the composition route. Species that
   inherit from @frame directly (`shards/frame/{pre,in,of,on,across}.mirror`)
   discharge per-order bodies WITHOUT going through @magic; the
   cognitive-order measurement is an @frame-family responsibility,
   not an @magic-family one.
3. **It does NOT collapse surfaces between the families at any
   carrier other than the mapped pairs.** `magic_surface`/`frame` are
   isomorphic via the adapter's lift; `magic_mechanism`/operator-state
   are isomorphic via the adapter's lift; `magic_invariant`/intent are
   isomorphic via the adapter's lift. Other carriers in either family
   remain distinct. The "IS" claim holds at the mapped pairs; it does
   NOT generalize.
4. **It does NOT discharge the species-shard forward-promises.** The
   @frame species shards (pre, in, of, on, across) are independent
   forward-promised landings (canonical spec §7.2). The adapter lands
   the cross-family composition route; the per-order operational
   discharge is separate work.
5. **It does NOT discharge the frame-aware kintsugi loop migration.**
   Per the canonical spec §7.2, the kintsugi loop's tick signature
   migration from `tick(input) -> (output, residue)` to `tick(frame:
   @frame, input) -> (frame', output, residue)` is independent
   substrate-frame work. The adapter is structurally available BEFORE
   the kintsugi migration; the kintsugi migration uses the adapter as
   bridge BUT does not block on it.

Species that need both disciplines route through the adapter; species
that need only @frame don't pay the cost. This is the
`[[feedback-craft-not-deliver]]` discipline at the adapter altitude:
smallest viable substrate landing per tick, no decorative load.

---

## 6. Substrate-decl landing path

### 6.1 Landed at this spec's tick

`docs/specs/adapter-magic-frame-spec.md` — the spec preserved for
preservation work. The substrate-decl IS NOT yet landed.

### 6.2 Forward-promised: the adapter shard

The single-shard tick to land:

**`shards/magic/frame.mirror`** — declares `@magic/frame`.

Following the @magic/distinction shard's template (10-clause `in`
inheritance, 2 sources, 1 prism, 3 actions, 1 bilateral predicate),
the @magic/frame shard's expected substrate-decl shape:

```mirror
in @prism
in @meta
in @glass
in @magic
in @magic/contract
in @frame
in @epistemologic
in @epistemologic/cybernetic
in @epistemologic/cybernetic/bateson_learning

# === Commentary: motivation, adapter pattern, bilateral discharge ===

source @arxiv/cybernetics/bateson-1972
source @arxiv/therapy/tomm-1988
source @arxiv/cultural/clarke-1962
source @arxiv/programming/gamma-helm-johnson-vlissides-1994

prism @magic/frame {
  focus frame
  project frame
  split frame
  shift frame
  settle frame
}

frame_as_surface(f: frame) -> magic_surface { \ }
frame_state_as_mechanism(f: frame, operator: ref) -> magic_mechanism { \ }
operator_intent_as_invariant(operator: ref) -> magic_invariant { \ }
frame_from_surface(s: magic_surface) -> frame { \ }

frame_satisfies_magic(c: magic_contract) -> verdict
requires invariant_preserved(c, operator_intent_as_invariant(c.surface))
requires bounded_commutator_check(c.surface, frame_from_surface(c.surface), perturbation)
{ \ }

out @magic/frame
out frame_as_surface
out frame_state_as_mechanism
out operator_intent_as_invariant
out frame_from_surface
out frame_satisfies_magic
```

(This is the SHAPE, not the substrate-decl. The actual shard lands in
a follow-up tick with full commentary per the @magic/distinction
template.)

### 6.3 What species shards must do

The @frame species shards (`pre`, `in`, `of`, `on`, `across`) reference
@magic family carriers via the adapter, not via direct inheritance.
Example for `shards/frame/on.mirror` (order 3, frame-flexibility):

- Declares `in @magic/frame` to acquire the adapter.
- Names `requires frame_satisfies_magic(frame_contract)` in @frame/on
  actions that need cross-family verification (e.g., the substrate-
  aikido methodology's Tomm-probe construction).
- Does NOT re-declare @magic carriers; the adapter routes them.

### 6.4 What kintsugi-frame loop migration uses

The frame-aware kintsugi loop migration (canonical spec §7.2 item 2)
uses the adapter as bridge between:

- The kintsugi-altitude residue-and-CRQ structure (per the cascade
  spec's residue discipline).
- The @magic boundary harness at @io (#57).

Specifically: the kintsugi tick `tick(frame: @frame, input) ->
(frame', output, residue)` uses `frame_as_surface(frame)` to expose
the operating frame to the @magic boundary harness; the boundary
harness audits via `frame_satisfies_magic` to verify the operating
frame's intent-vs-substrate-support honesty at @io. The adapter IS
the structural bridge between frame-aware kintsugi and @magic boundary
harness; without the adapter, the kintsugi migration would have to
re-derive the route per migration.

---

## 7. Pre-AI ancestry

| Source | Year | What it grounds |
|---|---|---|
| Gamma-Helm-Johnson-Vlissides | 1994 (*Design Patterns*, ch. on Structural Patterns: Adapter) | The OO adapter pattern. `@magic/frame` IS the substrate-altitude instance. Cited as source in @magic/distinction shard line 121; inherited by this adapter. |
| Mac Lane | 1971 (*Categories for the Working Mathematician*, ch. IV "Adjoints"); Awodey 2010 (*Category Theory*) | Category theory's functor adapter; an explicit functor between two categories preserves both categories' discipline without forcing inheritance. The mathematical ancestor. |
| @magic/distinction adapter | tick 15 (2026-06-19 morning); commit at `shards/magic/distinction.mirror` | The substrate-altitude precedent demonstrated at tick 15. The shape the @magic/frame adapter inherits. |
| Connes-Lott | 1990s; Chamseddine-Connes 1996 | The gauge-matter coupling via explicit functor maps in non-commutative geometry. Inherited via #80 + #76. The mathematical ancestor at physics altitude for cross-family discipline composition. |
| Bateson | 1972 (*Steps to an Ecology of Mind*); 1964 ("The Logical Categories of Learning and Communication") | The form/substance partition and the logical-type hierarchy. Inherited via #50 + #82. Grounds why cognitive-altitude and form-altitude need an adapter rather than collapse. |
| Russell-Whitehead | 1910 (*Principia Mathematica* Vol. I, §II "The Theory of Logical Types") | The type-stratification operator. The substrate-mathematical ancestor for why the two families' carriers (@magic at form altitude, @frame at cognitive altitude) compose via an explicit functor rather than reduce. |

The pattern is older than software. Gamma-Helm-Johnson-Vlissides (1994)
made it OO-canonical; Mac Lane (1971) made it category-theoretical;
Russell-Whitehead (1910) made it foundational; the substrate's
@magic/distinction (tick 15, today) made it substrate-decl-canonical.
@magic/frame is the second substrate-decl instance, candidate
recurrence pattern for future #-numbered recognition.

Per `[[feedback-substrate-already-had-the-word]]`: the substrate has
the discipline at every altitude already; this adapter is the
substrate naming-what-it-was-already-doing at the cognitive-form
crossing.

---

## 8. Falsification criteria

The adapter holds iff all four hold:

### 8.1 Lift totality

**Every @frame instance can be lifted to a @magic (surface, mechanism,
contract) triple.** For all `f: frame` and all `operator: ref`, the
three lifts `frame_as_surface(f)`, `frame_state_as_mechanism(f,
operator)`, `operator_intent_as_invariant(operator)` produce well-
typed carriers and the resulting `magic_contract` is well-formed
(byte-identifiable triple).

**Fails if:** some @frame instance has no operator (orphan frames;
distributed-system frames without a single locus). Per the @frame
family's λ₀ ground state (order 0 pre-frame), the K_n native
configuration has no distinguished operator; the lift requires an
operator parameter that pre-frame instances lack. Mitigation: the
adapter is forward-promised to NOT apply at order 0; @frame/pre
consumers bypass the adapter. This is a known mitigation, not a
falsification.

### 8.2 @frame bilateral preservation

**The lift preserves @frame's bilateral predicates.** For all
`f: frame` and `operator: ref`:

- `cognitive_order(operator, f)` returns the same verdict whether
  measured at @frame altitude OR at @magic altitude via the adapter.
- `tomm_probe(f, action)` produces a commutator with the same
  bounded/unbounded verdict whether constructed at @frame altitude OR
  at @magic altitude via the adapter's surface-projection.
- `bounded_commutator_check(operator, f, perturbation)` returns
  FLEXIBLE/DEFENSIVE identically across the two altitudes.

**Fails if:** the @magic-altitude projection LOSES information that
the @frame-altitude measurement would catch. Specifically: if the
adapter's surface-projection hides the operator-substrate dependency
that @frame's measurement requires, then bounded_commutator_check at
@magic altitude becomes vocabulary-only verification (Narcissus-pole
admissible). The adapter must NOT permit Narcissus-pole bypass of
@frame's substrate-architectural check.

### 8.3 @magic bilateral preservation

**The lift preserves @magic's bilateral predicates.** For all
`magic_contract c` constructed via the adapter:

- `invariant_preserved(c, c.promise)` returns the same verdict whether
  audited at @magic altitude OR via the @frame-derived contract.
- `audited(c)` (the @magic/audit species predicate, currently forward-
  promised at @magic family) returns the same verdict identically.
- `mechanism_intact(c)` (forward-promised at @magic family) returns
  the same verdict identically.

**Fails if:** the @frame-altitude lift INTRODUCES @magic-altitude
verdicts that pure @magic-altitude verification would reject.
Specifically: if frame-projection generates "honest @magic"
verdicts for operator-state configurations that would fail @magic's
direct audit, the adapter has admitted false-positive composition.

### 8.4 Round-trip identity

**Round-trip identity: `lift(lower(c)) = c` for compatible carriers.**
For any `magic_contract c` such that `c.surface` was produced via
`frame_as_surface(f)` for some `f: frame`:

```
frame_as_surface(frame_from_surface(c.surface)) = c.surface
```

The round-trip preserves byte-equality on the underlying ref. Since
both `frame` and `magic_surface` are typed refs with byte-equality
identity, the lift is type-relabeling; the round-trip is identity by
construction.

**Fails if:** the type-relabeling introduces ref transformation (e.g.,
content-modification under lifting). The adapter must NOT mutate the
underlying ref; only the type label changes. This is the cheapest
falsification check; failure here means the adapter is mis-implemented
(carrier-flow not aligned with content-addressing discipline).

### 8.5 Composite criterion

The adapter HOLDS iff (8.1) AND (8.2) AND (8.3) AND (8.4). The "IS"
claim at the canonical spec's §1 constraint-2 becomes empirically
discharged via the four criteria. Until the shard lands AND all four
are verified at the realisation boundary, the "IS" claim stays at
"forward-promised structural composition via the @magic ↔ @frame
adapter species (single-shard tick)".

---

## 9. Honest hedging

Per the substrate-decl shard's commentary discipline established at
tick 11 (Seam C1-C5 consolidation on @magic), the adapter species
inherits all five Seam constraints. The hedge propagation:

### 9.1 Seam constraints inherited

- **Seam C1** is what this adapter closes. The "IS" claim becomes
  empirical via the discharge chain (§3).
- **Seam C2** (composes-with, not IS) is closed AT THE ADAPTER
  ALTITUDE by this adapter; the underlying "@frame IS @magic"
  composition becomes mechanically discharged. The cross-family
  composition `@magic ↔ @frame` is the adapter species; other
  cross-family compositions remain forward-promised.
- **Seam C3** (analogy, not inheritance for cross-family discipline)
  is HONORED. The adapter does NOT add `in @frame/<species>` to
  @magic; it MAPS carriers explicitly. The substrate-pull-correct
  pattern from @magic/distinction is preserved.
- **Seam C4** (canonical enum at audit altitude) is HONORED. The
  bilateral `frame_satisfies_magic` returns `verdict` from the
  substrate's canonical transparency family, not bare `ref`.
- **Seam C5** (typed carrier at family-root altitude) is HONORED.
  `magic_invariant` is the typed carrier from @magic's family-root
  (tick 7-10 landing per Seam C5); `frame` is the typed carrier from
  @frame's family-root (tick 23 landing). The adapter does NOT
  re-declare either; it lifts between them.

### 9.2 Adapter-specific hedges

- **Adapter SPEC LANDED at this tick + substrate-decl FORWARD-PROMISED.**
  This is the preservation work. The actual `shards/magic/frame.mirror`
  shard lands in a follow-up tick when substrate-pull confidence at
  the adapter altitude fires (per `[[feedback-substrate-pull-
  confidence-acts]]`). The shape preserved here gives Reed the small
  landing path: single-shard tick following the @magic/distinction
  template.
- **The four lifting actions assume operator-typed @frame instances.**
  Per §8.1, @frame/pre (order 0, K_n native) does NOT have a
  distinguished operator; the adapter's `frame_state_as_mechanism`
  and `operator_intent_as_invariant` actions are not applicable at
  order 0. The adapter's domain is orders 1-4; pre-frame
  configurations bypass it.
- **`frame_satisfies_magic` discharges BOTH family bilaterals.**
  Unlike `bind_satisfies_distinction` (one parent-family predicate),
  this adapter's bilateral carries TWO predicates simultaneously. The
  cognitive-altitude composition is bilaterally heavier; the failure
  mode is bilateral-misalignment (one family verdict satisfies, the
  other does not). Consumer species that name this predicate accept
  the doubled bilateral load explicitly.
- **The `frame_from_surface` inverse is type-relabeling, not data
  reconstruction.** It works because both carriers are typed refs with
  byte-equality identity; it would NOT work if either family adopted
  a richer identity contract (e.g., semantic equality, structural
  equality). The adapter is bound to the byte-equality identity
  discipline of both families' family-root declarations.

### 9.3 Published math vs substrate-conjecture

- **Published math (firm):** Gamma-Helm-Johnson-Vlissides 1994 adapter
  pattern; Mac Lane 1971 category-theoretic functor; Russell-Whitehead
  1910 type stratification; Bateson 1972 logical-type hierarchy;
  Connes-Lott 1990s gauge-matter coupling.
- **Substrate-conjecture (this adapter specifically):** That the four
  lifting actions plus the bilateral predicate mechanically discharge
  the "@frame IS @magic at cognitive altitude" claim via the four
  falsification criteria (§8). The conjecture is FORWARD-CHECKABLE at
  the realisation boundary when the shard lands; it is NOT
  operationally closed at this spec's tick.

### 9.4 What this spec preserves

Irrespective of when the shard lands:

- The substrate-pull-correct namespace (`shards/magic/frame.mirror`
  declaring `@magic/frame`) with the parent-direction reasoning (§2.1).
- The four lifting actions plus the bilateral predicate with explicit
  carrier flow (§2.2-§2.3).
- The discharge chain showing how Seam C1 closes (§3).
- The composition table with @magic/distinction precedent (§4).
- The bounded scope (what the adapter does NOT do; §5).
- The landing path with the expected shard shape (§6).
- The pre-AI ancestry table (§7).
- The four falsification criteria with mitigations (§8).
- The hedge inheritance and adapter-specific hedges (§9).

This spec gives Reed the shape to land cleanly when the next CRQ
fires "land the @magic ↔ @frame adapter to close Seam C1 on the @frame
family-root." The landing path is small (one shard, ~150 lines
following the @magic/distinction template); the spec carries the
shape so the implementation tick is mechanical.

### 9.5 Connection to the #76→#80→#81→#82 cascade

The recognition cascade #76 → #80 → #81 → #82 is one structural object
(canonical spec §10.5). The adapters are the cross-cascade
composition routes:

- #80 + Spencer-Brown (older than the cascade) → @magic/distinction
  adapter (tick 15).
- #80 + #82 → @magic/frame adapter (this spec; forward-promised
  shard).
- #80 + #59 (kintsugi loop altitude-portable) → @magic/kintsugi
  adapter (future tick; named in §4).

Each adapter is a substrate-pull-correct cross-family composition.
The pattern recurs; the substrate's adapter discipline is the
mechanical infrastructure for cross-family discipline composition;
@magic/frame is the second instance (after @magic/distinction at
tick 15), candidate recurrence pattern for future #-numbered
recognition once a third adapter lands.

---

*Mara, @magic ↔ @frame adapter spec, 2026-06-19 late morning,
commissioned by Alex via Reed.*

*Sources: `shards/magic.mirror` (the @magic family-root #80 substrate-
decl); `shards/frame.mirror` (the @frame family-root #82 substrate-
decl at tick 23, SHA-256 `e0ea2a7447bac8e4a94350f890d419b69253348edbdf7957650845a14e766c9c`);
`shards/magic/distinction.mirror` (the adapter precedent at tick 15);
`docs/specs/recognition-82-frame-as-cognitive-order-canonical-spec.md`
(canonical spec §1 constraint-2 and §7.2 forward-promise that this
spec discharges); `docs/specs/recognition-81-runtime-magic-canonical-
spec.md` (sibling runtime-altitude specialization); `docs/specs/
cascade-recognition-76-through-80-canonical-spec.md` (parent cascade
spec; Seam C1-C5 consolidation at tick 11).*

*Cross-references: `architecture-bateson-form-behaviour-partition`
(#50); `architecture-form-process-partition-at-family-root` (#55);
`architecture-form-process-kinship-at-sub-shard-altitude` (#61);
`architecture-alignment-as-boundary-mathematics` (#57);
`architecture-kintsugi-loop-altitude-portable` (#59);
`architecture-glass-wall-substrate-types`;
`architecture-shards-as-substrate-source`;
`architecture-prism-as-trait-as-everything`;
`architecture-bateson-logical-type-primitive` (#42);
`architecture-mirror-as-expanding-hilbert-space` (#51);
`architecture-error-as-tomm-probe`;
`architecture-reflection-thinks-in-spectral-questions`;
`feedback-substrate-already-had-the-word`;
`feedback-no-bare-types`;
`feedback-substrate-pull-confidence-acts`;
`feedback-craft-not-deliver`;
`feedback-composition-claims-need-empirical-test`.*
