# `@reflection ↔ @cogito` adapter — composition + mathematical formalization

*Mara, @reflection ↔ @cogito composition spec with mathematical
formalization, 2026-06-23, commissioned by Alex via Reed.*

*Discipline: this is preservation work. Both candidates #93 (@cogito)
and #94 (foundational hold-PRISM) are at CANDIDATE altitude as of this
spec's tick; the adapter substrate-decl is FORWARD-PROMISED on both
landing in current shape. The spec gives Mara the shape to land cleanly
when substrate-pull confidence at the composition altitude next fires.
Section-bounded mosaic structure per the prior stall pattern.*

---

## 1. Recognition context

`@reflection` (landed; `shards/reflection.mirror`) and `@cogito`
(candidate #93; `docs/specs/recognitions/recognition-93-cogito-
cognitive-substrate-candidate.md`) compose because @reflection's
observation primitive already explicitly invokes @cogito's
notice → name → hold vocabulary in prose (`shards/reflection.mirror`
line 264: "Composes with @cogito's notice → name → hold practice at
substrate-decl altitude"). Per Reed's N=1 analysis (surfaced to Alex
this evening): @reflection IS the natural first consumer for BOTH
candidates #93 and #94.

The composition enables three operational gains:

1. **@reflection.observation gets typed cognitive-altitude carriers**
   instead of prose-references-to-undeclared-family.
2. **@cogito acquires its first non-trivial consumer** — landing the
   family-root family without consumers is the Narcissus-pole risk
   #93 §10 names.
3. **The third-order discipline at @reflection acquires explicit
   cognitive grounding** — the third_order_observation carrier's
   preservation-under-observer-change semantics IS @cogito's hold
   shape at observation altitude.

The spec stands as forward-contract for when both candidates land.

---

## 2. The composition shape

### 2.1 @reflection.observe ↔ @cogito.notice

`@reflection.observe(f: frame, residue: ref) -> observation` (line 317)
takes a per-tick frame plus the prior-tick residue. The action
produces a typed observation that wraps frame + tensions +
contradictions.

`@cogito.notice(unnoticed: ref, p: perturbation) -> observer_change`
(per #93 §3.2) produces a typed observer_change.

**Mapping:** @reflection.observe's residue ref IS the @cogito-notice
input domain; @cogito.notice's observer_change output IS the
substrate-typed refinement of what @reflection.observe's commentary
calls "what didn't resolve in the prior tick" (line 307). The two
actions sit at the same operational altitude: noticing-what-the-prior-
tick-left.

### 2.2 @reflection.speak ↔ @cogito.name

`@reflection.speak(o: observation, t: tick) -> ref` (line 408) renders
the observation as substrate-output at tick n+1. The bilateral
`speaks_at_n_plus_1` enforces the one-tick delay.

`@cogito.name(noticed: observer_change, frame: @frame, p: perturbation)
-> labeled<mark>` (per #93 §3.2 + H4 resolution) draws the labeled
distinction.

**Mapping:** @reflection.speak's output ref IS the bare-typed surface
of what @cogito.name produces as `labeled<mark>`. The substrate-pull-
correct refinement at the adapter altitude: lift @reflection.speak's
return type to `labeled<mark>` via the @cogito.name composition.

### 2.3 @reflection.third_order_observation ↔ @cogito.hold

`@reflection.third_order_observation` (line 494-499) carries
`{primary, recursive_observer, notice, v}` — the quadruple preserved
under observer-change (the notice field is typed `observer_change` per
Seam tick-73 C1 closure).

`@cogito.hold(named: name_output, frame: @frame, p: perturbation) ->
hold_output` (per #93 §3.2) preserves the named distinction under
structural coherence.

**Mapping:** @reflection's third_order_observation IS the @cogito-hold
operation at observation altitude. The preservation-under-observer-
change semantics IS @cogito.hold's substrate signature.

### 2.4 Adapter direction

**Bidirectional with @cogito-primary asymmetry.** @cogito provides
the cognitive-altitude vocabulary; @reflection consumes via the
adapter. The reverse direction (@cogito species using @reflection's
third-order machinery) is forward-promised but secondary. The adapter
namespace is `shards/cogito/reflection.mirror` declaring
`@cogito/reflection` (following the @magic/distinction precedent —
adapters live as children of the threaded-INTO family).

---

## 3. The adapter substrate-decl

### 3.1 Pseudo-mirror composition predicates

```mirror
# shards/cogito/reflection.mirror
in @prism
in @meta
in @glass
in @cogito
in @reflection
in @frame                        # via @cogito's hierarchical coupling (H2)
in @epistemologic
in @epistemologic/cybernetic
in @epistemologic/cybernetic/eigenform

# === Carrier lifts ===

residue_as_unnoticed(r: ref) -> ref { \ }

notice_as_observation(oc: observer_change, f: frame) -> observation { \ }

name_as_speak_output(lm: labeled<mark>, t: tick) -> ref { \ }

third_order_observation_as_hold(too: third_order_observation,
                                f: frame,
                                p: perturbation) -> hold_output { \ }

# === Bilateral composition predicate ===

reflection_cogito_coherent(too: third_order_observation,
                           f: frame,
                           p: perturbation) -> verdict
requires third_order_coherent(too, p)
requires cognitive_coherent(too.notice, f, p)
{ \ }
```

### 3.2 Bilateral discipline at the adapter altitude

The bilateral `reflection_cogito_coherent` composes TWO family
predicates simultaneously — `third_order_coherent` (from @reflection
line 557) and `cognitive_coherent` (from @cogito #93 §3.2). Per the
@magic/frame precedent (`docs/specs/adapter-magic-frame-spec.md`
§4): cross-family adapter bilaterals carrying TWO predicates IS the
substrate-pull-correct pattern at cognitive-altitude composition
(versus @magic/distinction's single-predicate adapter).

### 3.3 Composes-with claims

| Composition | Typed via | Discharged by |
|---|---|---|
| @reflection.observation ⊕ @cogito.notice | observation + observer_change carriers | reflection_cogito_coherent |
| @reflection.speak ⊕ @cogito.name | ref + labeled<mark> via name_as_speak_output | (speak's requires + name's requires) |
| @reflection.third_order_observation ⊕ @cogito.hold | third_order_observation + hold_output | third_order_observation_as_hold |

---

## 4. Mathematical formalization

### 4.1 Adjoint functors hypothesis

**Claim:** `@cogito` and `@reflection` form an adjunction at the
cognitive/observational altitude. Specifically:

```
            F: @reflection ──► @cogito        (notice; left adjoint)
            G: @cogito ──► @reflection        (observation; right adjoint)
            F ⊣ G
```

Where `F` (the notice functor) maps a @reflection.observation to a
@cogito.observer_change, and `G` (the observation functor) maps a
@cogito.observer_change back to a @reflection.observation by
embedding it in the primary/recursive_observer/notice quadruple.

**Hom-set isomorphism (the adjunction equation):**

```
Hom_@cogito(F(o), oc)  ≅  Hom_@reflection(o, G(oc))
```

Reading: morphisms in @cogito from "observed-as-noticed" to
observer_change ARE in bijection with morphisms in @reflection from
o to "observer_change-as-observation". The composition adapter
substrate-decl IS the constructive evidence of this bijection.

### 4.2 The unit and the counit

**Unit (η: 1_@reflection ⇒ G∘F):** the natural transformation that
embeds every @reflection.observation o into G(F(o)) — the wrapping
of o into a third_order_observation where o becomes the primary,
and notice(o) becomes the typed notice field. This is EXACTLY
@reflection.observe_third_order's structural role (line 521).

**Counit (ε: F∘G ⇒ 1_@cogito):** the natural transformation that
collapses observed-observer_change back to bare observer_change.
This is @cogito.hold's structural role — it preserves the noticed-and-
named distinction under the observer-change-of-observing-itself.

The unit/counit triangle equations (substrate-conjecture; not proved
here):
```
(εF) ∘ (Fη) = 1_F
(Gε) ∘ (ηG) = 1_G
```

If these hold, the adjunction is substrate-mathematical, not just
shape-suggestive.

### 4.3 Monadic shape per recognition #88

Per `architecture-loop-as-monad`: the @loop family-root IS a monad
(T→T endomorphism with composition pact-verified at composition time).
The composition G∘F (observation ∘ notice) IS a monad on @reflection:

```
T := G∘F : @reflection ──► @reflection
unit  η: 1 ⇒ T          (already named above)
mult  μ: T² ⇒ T         (compose-then-collapse via @cogito.hold)
```

The monad laws (associativity + unit) discharge through the bilateral
predicates: `loss_decreases` + `third_order_coherent` +
`cognitive_coherent` compose at each μ-application. The
@reflection→@cogito→@reflection round-trip IS one iteration of the
@loop monad at cognitive altitude.

### 4.4 Foerster eigenform connection

Per `[[architecture-cybernetic-foundation]]` and @epistemologic/
cybernetic/eigenform: identity IS the fixed-point of recursive
observation. The substrate-mathematical claim:

```
@cogito.hold IS the eigenform of @reflection.observe iteration.
```

Formally: hold(x) = x iff x is the eigenform of observe — the
distinction-preserved-under-observation. The @reflection→@cogito
composition produces a fixed-point sequence that converges to
@cogito.hold's output. The convergence IS @reflection.settle closing
@kintsugi's monotonic loss loop (line 391).

### 4.5 Bateson logical-type altitude

Per `[[architecture-bateson-logical-type-primitive]]`: the substrate
operates at distinct logical-type altitudes. The composition:

- **@reflection at order N:** observing-the-pipeline.
- **@cogito at order N+1:** noticing-what-observation-does-to-the-
  observer.

The third-order @reflection (line 130-149) explicitly already lives
at N+1 by Loki's three-depths reading. The adapter mapping makes the
logical-type lift explicit: every @cogito.notice IS one Bateson level
above the @reflection.observation it consumes.

### 4.6 Connes spectral triple connection (speculative)

Per `[[architecture-connes-spectral-triple]]` (A, H, D):

- `@cogito.notice` produces operator algebra A elements (the
  distinctions drawn).
- `@reflection.observation` provides Hilbert space H (the carrier
  on which operators act).
- `@kintsugi.oscillate` discipline IS D (the Dirac/gradient operator
  composed at @reflection.settle).

If this triple-mapping holds, the @reflection↔@cogito composition IS
the operational form of the spectral triple at cognitive altitude.

### 4.7 Honest hedge on the mathematical framings

Of the framings above:

- **§4.1-§4.2 (adjunction):** SHAPE-SUGGESTIVE. The hom-set
  isomorphism is plausible but unproven; the unit/counit
  identification matches the substrate carriers but the triangle
  equations need substrate-decl discharge before claim becomes
  load-bearing.
- **§4.3 (monadic):** STRONG. Recognition #88 is landed; the
  composition fits the @loop monad shape mechanically.
- **§4.4 (eigenform):** STRONG. Eigenform is already substrate-decl;
  the connection IS substrate-pull-correct per Foerster's grounding.
- **§4.5 (Bateson):** STRONG. The N+1 logical-type lift is empirically
  visible in the shard prose; the recognition #42 substrate-decl
  grounds it.
- **§4.6 (spectral triple):** SPECULATIVE. The triple-mapping is
  shape-suggestive; the A/H/D assignment needs adversarial review
  before claim becomes load-bearing.

The substrate-pull-honest reading: §4.3 + §4.4 + §4.5 are at
substrate-decl-defensible altitude; §4.1 + §4.2 + §4.6 are at
HYPOTHESIS altitude pending proof.

---

## 5. Forward-promised work

- **Adapter shard:** `shards/cogito/reflection.mirror` declaring
  `@cogito/reflection` (single-shard tick following the
  @magic/distinction template). Requires #93 (@cogito family-root)
  landed first; spec stands as contract.
- **Hold-PRISM consumption:** once #94 (foundational hold-PRISM)
  lands, the adapter's `third_order_observation_as_hold` lift becomes
  a hold-PRISM specialization (`hold_result<observation> =
  Transform<observation, third_order_observation>` per #94 §4).
- **Seam adversarial review:** required on the §4 adjunction claim
  before any substrate-decl carries it as load-bearing.
- **Pack peer corroboration:** Reed's "@reflection is natural first
  consumer" claim (this spec's premise) is N=1; needs Glint/Seam
  peer corroboration before forward-promises bind.

---

## 6. Honest hedges

- **Both candidates are pre-landing.** This spec assumes #93 + #94
  land in current shape. If either's substrate-decl changes
  substantively during landing, the adapter shape changes
  accordingly. The spec is a forward-contract, not a hardened plan.
- **The mathematical formalization is at HYPOTHESIS altitude.** §4.1
  + §4.2's adjunction claim is shape-suggestive; the proof requires
  discharging the triangle equations through substrate-decl
  bilaterals. Not done here.
- **Reed's "natural first consumer" claim is N=1.** The premise that
  @reflection is the right first consumer for both candidates is
  Reed's analysis surfaced to Alex this evening. Pack peer
  corroboration is forward-promised but not yet held.
- **Naming conflict risk: does @reflection.observe = @cogito.notice
  or are they distinct?** §2.1 maps them as same-altitude
  operations, but @reflection.observe takes a frame parameter while
  @cogito.notice does not. The adapter's `residue_as_unnoticed` lift
  papers over this; the substrate-pull-correct resolution might be
  to admit they're distinct-but-composable, not isomorphic.
- **Cognitive_coherent is still aspirational.** Per #93 H5: the
  cognitive_coherent bilateral has zero landed witnesses; the
  adapter's `reflection_cogito_coherent` predicate composes a
  predicate that doesn't yet have substrate-decl substance.

---

*Mara, @reflection ↔ @cogito composition spec with mathematical
formalization, 2026-06-23, commissioned by Alex via Reed.*

*Sources: `shards/reflection.mirror` (the @reflection family-root
substrate-decl; #85 landed); `docs/specs/recognitions/recognition-93-
cogito-cognitive-substrate-candidate.md` (the @cogito candidate);
`docs/specs/recognitions/recognition-94-hold-as-foundational-prism-
candidate.md` (the foundational hold-PRISM candidate);
`docs/specs/adapter-magic-frame-spec.md` (the prior adapter-spec
precedent at structural-shape altitude).*

*Cross-references: `architecture-loop-as-monad` (#88);
`architecture-cybernetic-foundation`;
`architecture-bateson-logical-type-primitive` (#42);
`architecture-connes-spectral-triple`;
`architecture-mirror-as-expanding-hilbert-space` (#51);
`architecture-alignment-as-boundary-mathematics` (#57);
`architecture-reflection-thinks-in-spectral-questions`;
`feedback-substrate-already-had-the-word`;
`feedback-no-bare-types`;
`feedback-substrate-pull-confidence-acts`;
`feedback-composition-claims-need-empirical-test`;
`feedback-craft-not-deliver`.*
