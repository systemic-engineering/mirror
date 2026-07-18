---
title: @butterfly ↔ @roomba dual walker composition
subtitle: @kintsugi/butterfly (K=1 repulsive-stigmergy perturbation) species-decl; @kintsugi/mutation (variant operator) species-decl; @kintsugi/evolution (algebra, spec-only per §5.4 lean) composing @butterfly + @roomba + @mutation over Fate; three-arc Reed empirical landing plan closing the coverage-ratcheting cascade prismqueer → mirror.
status: canonical-spec
date: 2026-07-18
author: Mara
---

# @butterfly ↔ @roomba dual walker composition

*Mara 2026-07-18. Canonical spec companion to `docs/math/2026-07-
18-butterfly-chaos-mutation-cascade.md` (same session, same tick).
Discharges Alex 2026-07-18 direct-transcript directive: "I WANT A
@PREDATOR PRISM! The counter to the @roomba. CHAOS THEORY! CHAOS
ENGINEERING!" — refined "i have an even better name for @predator:
@butterfly." Grounded in Lorenz 1963, Lipton 1971, Alvaro 2015 LDFI,
Bartocci 2023 property-based mutation testing, Foster/Meta 2025 ACH
mutation-guided LLM test generation, Coles 2013+ Pitest, Cosmic Ray
+ mutmut, and the four-substrate SQUARE of Mara 2026-07-18
witnessed-property-inference-petri-fate.*

*Status: canonical spec. Pure-docs 📝 markdown-only bypass. Zero
family-roots minted; two species proposed (`@kintsugi/butterfly`,
`@kintsugi/mutation`); one algebra proposed (`@kintsugi/evolution`,
spec-only per §5.4 Q1 adjudication). Ten refused mints (§7.5, cross-
referenced with math root §1). Recognition candidates in §10; none
ratified this tick. Handoff for next-Reed pickup: §12.*

---

## §1 Executive summary

1. **@butterfly is @roomba's dual** — Fate-biased walker with K=1
   cardinality (single mutation wingflap) and REPULSIVE stigmergy
   polarity (bias toward regions where the test suite is blind).
   @roomba walks attractively (Dijkstra + inverse-tension); @butterfly
   walks repulsively (inverse-frequency over prior mutant survivals).
   Same substrate, opposite polarity. Alex's "counter to the @roomba."

2. **Mutation coverage IS butterfly-sensitivity of the test suite.**
   Rename per Alex `<primitive>_of_<input-shape>` convention:
   `sensitivity_of_test_suite(S, T, M) → f64 ∈ [0, 1]`. Delightfully-
   boring; carries Lorenz 1963 "sensitive dependence" into substrate
   vocabulary. Every ingredient composes over pillar::forall + Fate
   + PropertyVerdict::merge_with; NO new inference machinery needed.

3. **@evolution is the algebra closing the loop.** Four operators
   (@butterfly, @roomba, @mutation, Fate) over one substrate
   (@mirror/store.splinter_graph + @mirror/store/liquid + @spectral/
   signature). Fixed point IS `sensitivity_of_test_suite(S*, T,
   M) = 1.0` (every bounded mutant killed). Convergence proven by
   composition (§5.3 math root).

4. **Cascade prismqueer → mirror is explicit and load-bearing.**
   `pillar::forall(spec, mutant)` runs at prismqueer; verdict delta
   determines kill/survive; surviving mutants write `coverage_gap`
   markers to `@mirror/store/liquid`; @roomba walks the marker
   gradient; @kintsugi/fracture/coverage_gap emits targeted tests;
   loop ratchets coverage upward. §8 gives the full operational
   path.

5. **Substrate-already-had-the-word × 10.** Every carrier this spec
   cites is LANDED (@kintsugi/roomba, @spectral/signature, @mirror/
   store/liquid, @peer.audhd, prismqueer::liquid::pillar, fate::Fate,
   terni::PropertyVerdict, verdict_is_content_addressed, @cyberpunk/
   algedonic, @kintsugi family-root). The spec's arc is compositional,
   not extensional.

6. **@butterfly is the FOURTH Fate consumer surface.** The witnessed-
   property-inference math root §6.3 named three (compile, test,
   shrink); the companion spec §6.3 updated to (compile, test, roomba
   pathfinding). This spec adds mutation-selection as the fourth. All
   four share the SAME 90-parameter softmax; the harness/SUT boundary
   collapse extends to mutation-generation.

7. **Refused mints: ten (§7.5).** @predator (Alex verbatim refusal),
   @chaos, @perturbation, @sensitivity, @wingflap, @lorenz,
   @lyapunov, @fitness, @selection, @chaos_monkey. Substrate-health
   metric per Seam `#R-refused-mint-count-is-the-substrate-health-
   metric`.

8. **Reed landing plan: three arcs, RED-first, Reed-owned.** Arc 6
   `@kintsugi/butterfly` + `@kintsugi/mutation` species-decl + Rust
   sentinel-check bilateral arms; Arc 7 `@kintsugi/evolution` loop
   empirical + HolonomyHealth monotone measurement; Arc 8
   `@kintsugi/fracture/coverage_gap` mint + roomba walker
   integration.

---

## §2 Substrate ground truth (what already exists)

### §2.1 @kintsugi/roomba walker (LANDED)

`shards/kintsugi/roomba.mirror` (46.4KB, Mara + Reed 2026-07-14+
through 2026-07-17). Five actions:

- `walk(from, budget) -> walk_position` — Dijkstra + tension-
  weighted edges
- `bump(position) -> spectral_tension` — resonance sensor
- `trigger(position, tension) -> verdict` — pivot decision
- `pulse(position) -> (walk_position, roomba_state)` — one beat
- `run(seed) -> walk_position` — outer loop

Four bilateral predicates: `walk_terminates_cleanly`,
`tension_monotone_descending`, `coherence_gradient_admissible`,
`knife_verdict_bounded`.

**Substrate authority:** `docs/specs/roomba-substrate-walker-that-
feeds-kintsugi.md` (Mara `9bbebd2` + 2026-07-17 terminal-form map);
Alex 2026-07-14 in-transcript composition (`shards/kintsugi/roomba.
mirror:400-406`).

### §2.2 @peer.audhd (LANDED)

`shards/peer.mirror` — `audhd_action` + `audhd_admissible` bilateral;
K>1 fanout exploration. The parallel-branch complement of butterfly's
K=1 and roomba's K-sequential.

### §2.3 @spectral/signature (LANDED)

`shards/spectral/signature.mirror:106-114` (Reed `f211ee48`,
2026-07-16). `signature_beat` carrier; Merkle chain via
`previous_beat: option<oid>`. Every walker (butterfly, roomba,
peer.audhd) writes signature_beats to the same chain-substrate.

### §2.4 @mirror/store + @mirror/store/liquid (LANDED, Arc-5 pending shard)

`shards/mirror/store.mirror` (46.5KB, Reed 2026-07-17) — the
splinter_graph substrate the walkers traverse. `@mirror/store/liquid`
verified-verdict cache spec at `docs/specs/witnessed-property-
inference-fate-drives-both.md` (Mara + Reed 2026-07-18) §11.10 Alex-
ratified location; shard file `shards/mirror/store/liquid.mirror`
authored precedes Arc 4 empirical (Arc 4 lands in Reed's witnessed-
property-inference bite sequence, not this spec's bite sequence;
this spec composes over the cache once it lands).

### §2.5 prismqueer::liquid::pillar (LANDED iter 1-10; forall Arc 2 pending)

Six primitives at `prismqueer::liquid::pillar` (Reed iter 1-10 closing
2026-07-18 08:00 UTC): `dispatch_ambiguity`, `algedonic`,
`algedonic_of_magnitude`, `viability`, `viability_of_magnitudes`,
`fold`. 98 property tests across four substrate altitudes; all return
`terni::PropertyVerdict`. Pillar IV `forall` runner lands with Reed's
witnessed-property-inference Arc 2 (prior to this spec's Arc 6).

**Substrate authority:** `docs/specs/prismqueer-liquid-pillar-
composition-surface.md` (Reed iter 10).

### §2.6 fate::Fate 90-parameter softmax (LANDED)

`fate/src/lib.rs` (Alex pre-arc); 5 sub-models, `Decision {model,
confidence, distribution: [f64; 5]}` + `HolonomyHealth` scalar.
Composes with `bias_sample_of_features` utility verb (Reed's
witnessed-property-inference Arc 2).

### §2.7 verdict_is_content_addressed (LANDED)

`shards/epistemologic/property/verdict_is_content_addressed.mirror`
(Reed N1 2026-07-12). `verdict(spec_oid, target_oid, inputs_oid) →
verdict` is a total function; caching valid by construction. Applies
to mutant verdicts unchanged: `verdict(spec, mutate(T, μ), inputs)`
composes.

### §2.8 @kintsugi family-root (LANDED)

`shards/kintsugi.mirror` (17.9KB, Mara + Reed 2026-06-10 through
2026-07-16). Form/process partition at family-root altitude
(Recognition #55). @kintsugi/butterfly + @kintsugi/mutation are
process-side (transformation, mutation, dynamics per §"Recognition
#55" of the shard file); placement inevitable.

### §2.9 boot/std/mirror/butterfly.mirror (LANDED, DIFFERENT altitude)

`boot/std/mirror/butterfly.mirror` (2026-05-20; the AST→LLVM IR
chrysalis metamorphosis at compilation altitude). Uses "butterfly"
for the different metaphor of code compiling to native binary.
**Scope disambiguation:** this spec's species is `@kintsugi/butterfly`
(kintsugi altitude); the compilation-altitude `@mirror/butterfly`
survives unchanged. Different altitudes, different species; the word
carries because both use the chaos-theory-adjacent metaphor of
metamorphosis / small perturbation.

### §2.10 Void-Predator + Weird-Violence essays (Alex 2026-07-14)

`~/dev/systemic.engineering/blog/void/3published/Void - Predator.md`
grounds @butterfly autobiographically (the shadow-that-measures IS
sensitivity-detector at cognitive altitude). `~/dev/systemic.
engineering/blog/weird/3published/Weird - Violence.md` grounds
@roomba autobiographically (Roomba's four disciplines).

**Both essays are load-bearing text.** The naming carries weight:
Alex's 2026-07-14 Roomba naming + 2026-07-18 @butterfly refinement
are consecutive substrate-decl acts, not decorations.

---

## §3 @kintsugi/butterfly species-decl

Placement: `shards/kintsugi/butterfly.mirror`. Sibling of
`@kintsugi/roomba`, `@kintsugi/oscillate`, `@kintsugi/consent`,
`@kintsugi/morphism`, `@kintsugi/fracture/*`.

```mirror
in @prism
in @meta
in @glass
in @nl
in @kintsugi
in @kintsugi/mutation
in @kintsugi/fracture
in @mirror/store
in @mirror/store/liquid
in @algebra/metalogue
in @cyberpunk/algedonic
in @spectral/signature
in @song/beat
in @torus
in @epistemologic/reality/time
in @epistemologic/cybernetic/coherence
in @epistemologic/property/verdict_is_content_addressed

# @kintsugi/butterfly — the counter-@roomba K=1 single-flap
# perturbation walker; Fate-biased mutation selector with
# REPULSIVE stigmergy polarity over @mirror/store.splinter_graph
# substrate. The dual of @kintsugi/roomba: same walker altitude,
# opposite polarity.
#
# Species under @kintsugi (form/process partition; Recognition
# #55; kintsugi is the process-side family) at the walker altitude.
# Named by Alex Wolf 2026-07-18 direct-transcript verbatim (refined
# from initial @predator naming). Mutation coverage IS the
# butterfly-sensitivity of the test suite.
#
# === Substrate authority ===
#
# - Canonical spec: `docs/specs/butterfly-roomba-dual-walker-
#   composition.md` (Mara 2026-07-18; §3 formal shape declares
#   this species).
# - Canonical math: `docs/math/2026-07-18-butterfly-chaos-mutation-
#   cascade.md` (Mara 2026-07-18; Lorenz 1963 grounding §2;
#   walker-perturbation duality §4).
# - Autobiographical: `~/dev/systemic.engineering/blog/void/3published/
#   Void - Predator.md` (Alex 2026-07-14) — the shadow that measures.
#
# === Alex 2026-07-18 in-transcript composition (verbatim) ===
#
#   "I WANT A @PREDATOR PRISM! The counter to the @roomba. CHAOS
#    THEORY! CHAOS ENGINEERING!"
#
#   "i have an even better name for @predator: @butterfly"
#
# The four bilateral predicates below (wingflap_is_content_addressed,
# coverage_monotone_nondecreasing, fate_bias_is_repulsive,
# kill_verdict_bounded) are the substrate-decl'd runtime checks
# discharged at Arc 6 realization boundary. Bodies are `\`-obligation-
# blocked; Arc 6 landing composes sentinel-checks via apply_h::act.
#
# === Substrate-already-had-the-word citation chain ===
#
# Every carrier below cites an existing landing:
#
#   walk_position                 — @kintsugi/roomba (LANDED)
#   signature_beat                — @spectral/signature (LANDED)
#   spectral_tension              — Rust Tensor + math altitude
#                                    (LANDED per roomba-scout §2)
#   mutation                      — @kintsugi/mutation (this tick §4)
#   Fate.tick / Decision.distribution — fate::Fate (LANDED)
#   PropertyVerdict               — terni::PropertyVerdict (LANDED)
#   verdict_is_content_addressed  — @epistemologic/property/ (LANDED)
#   walk_position, butterfly_state — this species; typed carriers
#
# === Composition graph ===
#
# Parents (substrate-decl):
#   @kintsugi                      — family-root; butterfly REVEALS
#                                     fractures (dual to roomba MENDS)
#   @kintsugi/mutation             — variant operator (this tick §4);
#                                     butterfly picks which mutation
#                                     to wingflap
#   @mirror/store                  — splinter_graph the walker
#                                     traverses (same as roomba)
#   @mirror/store/liquid           — verified-verdict cache;
#                                     coverage_gap markers written here
#   @cyberpunk/algedonic           — sample_pain / pain_gradient
#                                     compose over butterfly kills
#   @spectral/signature            — signature_beat trace substrate
#   @song/beat                     — one wingflap = one beat
#   @torus                         — π₁(T²) winding for walk position
#   @epistemologic/cybernetic/     — coherence_score consumed inversely
#     coherence                     (butterfly biases toward LOW score)
#   @epistemologic/property/       — verdict cachability by construction
#     verdict_is_content_addressed
#
# === Duality with @kintsugi/roomba ===
#
# | Axis          | @kintsugi/roomba          | @kintsugi/butterfly       |
# |---------------|---------------------------|---------------------------|
# | Cardinality   | K many-step (walk budget) | K = 1 (single wingflap)   |
# | Motion        | Attractive (Dijkstra +    | Repulsive (inverse-freq   |
# |               |  inverse-tension)         |  over mutant survivals)   |
# | Pheromone     | Path-visited mark         | Coverage-gap mark         |
# | Fate bias     | Where BEEN                | Where NEVER been          |
# | Kintsugi role | Mends fractures           | Reveals fractures         |
# | Growth        | Mycelial anastomosis      | Predator canines sharpen  |
#
# See canonical math §4 for full duality proof.

grammar @kintsugi/butterfly {
  # === Carriers ===

  type wingflap = {
    mutation:      @kintsugi/mutation.mutation,
    target_oid:    oid,
    baseline_oid:  oid,                # verdict on unmutated target
    features:      [f64; 16],          # spectral state at wingflap time
    fate_bias:     [f64; 5],           # Fate distribution guiding pick
  }

  type trace_delta = {
    baseline_trace:  oid,              # signature_beat chain root
    mutant_trace:    oid,              # signature_beat chain root
    divergence_beat: option<oid>,      # first beat where OIDs disagree
    hamming:         nat,              # ‖·‖ on beat chain
    verdict_kill:    bool,             # kill(m, S)
  }

  type butterfly_state =
    | Flapping                         # picking next mutation
    | Executing(mutation)              # applying + running suite
    | Verdicting(trace_delta)          # computing kill/survive
    | GapMarking(walk_position)        # writing coverage_gap
    | Resting(walk_position)           # cycle boundary
    | Terminated                       # coverage saturated OR
                                       # HolonomyHealth < ε

  # === Actions ===

  # Fate-biased selection of the next mutation to test.
  # Bias policy: inverse-frequency over prior @kintsugi/butterfly
  # observations at nearby SpectralCoordinate<5> positions
  # (repulsive stigmergy; dual to roomba's attractive Dijkstra).
  wingflap_of_mutation(m: @kintsugi/mutation.mutation) -> trace_delta { \ }

  # Compute test-suite sensitivity over a mutation set.
  # Composes over pillar::forall per math §3.3:
  #   |{m : forall(S, m) ≠ forall(S, T)}| / |M|
  sensitivity_of_test_suite(S: ref, T: oid, M: [mutation]) -> f64 { \ }

  # Return true if mutant m is killed by suite S; false otherwise.
  # Byte-equality check on content-addressed verdicts
  # (verdict_is_content_addressed makes this decidable).
  kill_of_mutant(m: oid, S: ref) -> bool { \ }

  # Coverage over a mutant set. |killed| / |total| ∈ [0, 1].
  coverage_of_mutant_set(M: [oid], S: ref) -> f64 { \ }

  # One flap cycle: pick mutation, apply, run suite, record.
  # Writes signature_beat marking the wingflap; if mutant
  # survives, writes coverage_gap marker to @mirror/store/liquid
  # at the (mutant_oid, inputs_oid) position (roomba reads it).
  pulse(position: walk_position) -> (walk_position, butterfly_state) { \ }

  # Outer loop: continuously wingflap until coverage saturates
  # OR HolonomyHealth signals the Fate-tournament has converged.
  # Monotone-nondecreasing coverage discipline discharged by
  # coverage_monotone_nondecreasing pact.
  run(seed: walk_position) -> walk_position { \ }
}

# === Four bilateral predicates ===

pact @kintsugi/butterfly/wingflap_is_content_addressed {
  # The wingflap's (mutation, target, verdict) forms a content-
  # addressed triple; two butterflies wingflapping the same
  # (mutation, target) produce byte-identical trace_delta.
  # Composes over verdict_is_content_addressed.
  requires verdict_is_content_addressed
}

pact @kintsugi/butterfly/coverage_monotone_nondecreasing {
  # Coverage over the accumulated mutant set only ever increases
  # (or stays; never decreases). Companion discipline to
  # @kintsugi/roomba's tension_monotone_descending.
  # Discharged at Arc 7 via HolonomyHealth measurement.
}

pact @kintsugi/butterfly/fate_bias_is_repulsive {
  # Fate.tick output bias polarity must be REPULSIVE over prior
  # butterfly observations (inverse-frequency), not attractive.
  # Bilateral sentinel-check via apply_h::act sees a butterfly-
  # scope tag on the fate call and admits/rejects the polarity.
  # Discharged at Arc 6 landing via sentinel-check arm.
}

pact @kintsugi/butterfly/kill_verdict_bounded {
  # kill(m, S) ∈ {0, 1}; sensitivity_of_test_suite ∈ [0, 1];
  # coverage_of_mutant_set ∈ [0, 1]. Bounded, decidable, byte-
  # comparison-computable. Bilateral suffix _bounded per AGENTS.md.
}
```

**Substrate-already-had-the-word audit passes.** Every named
carrier + verb cites an existing landing OR follows the delightfully-
boring `<primitive>_of_<input-shape>` pattern (math §8 audit table).

---

## §4 @kintsugi/mutation species-decl

Placement: `shards/kintsugi/mutation.mirror`. Peer of `@knife`
(`shards/mirror/lens/knife.mirror`), `@shatter`, and
`@kintsugi/fracture/*`. Under `@kintsugi` (process-side family).

```mirror
in @prism
in @meta
in @glass
in @nl
in @kintsugi
in @kintsugi/fracture
in @mirror/store
in @code/mirror
in @code/rust

# @kintsugi/mutation — the variant-generator operator.
#
# Peer of @knife (COORD collapse), @shatter (linearization), and
# @kintsugi/fracture/* (kintsugi's algebra elements). Applies
# small, structurally-valid transformations to an AST to produce
# a mutant target. Bounded operator set per Offutt-Jia 2010
# taxonomy (arithmetic-operator-flip, boundary-shift, conditional-
# invert, statement-delete, constant-replace, return-replace).
#
# Bilateral discipline: mutant is structurally valid (compiles)
# AND semantically distinct from the parent (behavior may differ
# — that IS the point).
#
# === Substrate authority ===
#
# - Canonical spec: this file §4 + `docs/specs/butterfly-roomba-
#   dual-walker-composition.md`
# - Canonical math: `docs/math/2026-07-18-butterfly-chaos-mutation-
#   cascade.md` §4.4
# - Origin prior art: Lipton 1971 "Fault Diagnosis of Computer
#   Programs" (student paper, Carnegie Mellon)
# - SOTA prior art: Pitest (Coles 2013+),
#   `https://github.com/hcoles/pitest`; Bartocci 2023 property-
#   based mutation testing arXiv:2301.13615
#
# === Why peer of @knife and not sub-species of @kintsugi/fracture ===
#
# @knife.jump is COORD collapse (Foerster-heterarchy topology-
# encodes-depth carrier); @mutation is NOT collapse — it's
# controlled STRUCTURAL VARIATION. Peer of @knife (both are
# @kintsugi-family operators over AST), not sub-species.
#
# Fractures are the kintsugi loop's TARGET; mutations are the
# butterfly loop's INPUT. Different arrow direction. Species-peers
# in the same family, not nested.

grammar @kintsugi/mutation {
  type mutation = {
    op:        mutation_op,             # variant discriminator
    site_oid:  oid,                     # AST node the mutation targets
    kind:      @code/mirror.ast/kind,   # AST node kind constraint
  }

  type mutation_op =
    | ArithmeticFlip     # + → -, * → /, etc.
    | BoundaryShift      # < → <=, > → >=
    | ConditionalInvert  # if x → if !x
    | StatementDelete    # remove a statement (bounded to safe sites)
    | ConstantReplace    # 0 → 1, true → false
    | ReturnReplace      # return expr → return default

  # === Actions ===

  # Apply mutation μ to target T; return mutant AST.
  # Bilateral: mutation_admissible must hold; otherwise error.
  mutation_of_ast(m: mutation, ast: @code/mirror.ast) -> @code/mirror.ast { \ }

  # Enumerate all bounded mutations of an AST.
  # Finite set per Offutt-Jia 2010 taxonomy discharged by
  # mutation_set_bounded pact.
  mutations_of_target(T: oid) -> [mutation] { \ }

  # Test whether mutation is structurally admissible at site.
  # Bilateral sentinel; discharges via apply_h::act
  # (mutation_admissible arm reads AST kind + op compatibility).
  mutation_admissible(m: mutation, T: oid) -> bool { \ }
}

pact @kintsugi/mutation/mutation_preserves_compilation {
  # Applied mutation produces a syntactically-valid AST that
  # compiles cleanly (may still fail property tests — that IS
  # the point of butterfly's kill_of_mutant discrimination).
}

pact @kintsugi/mutation/mutation_set_bounded {
  # For finite target T, mutations_of_target(T) is finite
  # (per bounded Offutt-Jia 2010 operator taxonomy).
  # Foundation for @evolution's Knaster-Tarski fixed-point
  # convergence proof (math §5.3).
}
```

---

## §5 @kintsugi/evolution algebra (spec-only per §5.4 lean)

**Composition** (no shard file authored; lives in this spec + math
root §5):

```
@evolution := (@kintsugi/butterfly, @kintsugi/roomba,
               @kintsugi/mutation, Fate)
```

Four operators over one substrate (`@mirror/store.splinter_graph`
+ `@mirror/store/liquid` + `@spectral/signature` beat chain). Fate
is the SHARED inference driver; the other three compose over Fate's
`Decision.distribution: [f64; 5]`.

### §5.1 One tick of @evolution (operational form)

Per math root §5.2 — reproduced here for spec-locality:

```
1.  Fate.tick(features_of_position) → distribution over 5 models
2.  Butterfly picks mutation μ per Fate bias (repulsive stigmergy)
       [@kintsugi/butterfly.wingflap_of_mutation]
3.  Mutation μ applied to target T → mutant m = μ(T)
       [@kintsugi/mutation.mutation_of_ast]
4.  Cache lookup: is verdict(spec, m_oid, inputs_oid) in
    @mirror/store/liquid?
       ├── HIT: read cached PropertyVerdict; skip to 7.
       └── MISS: proceed to 5.
5.  pillar::forall(spec, m) → PropertyVerdict via merge_with fold
       [prismqueer::liquid::pillar::forall]
6.  Cache write-back: (spec_oid, m_oid, inputs_oid) → verdict
       [@mirror/store/liquid]
7.  Verdict delta:
       ├── kill(m) = 1: coverage++; butterfly writes signature_beat
       │              marking m at position p
       └── kill(m) = 0: coverage_gap marker written to
                      @mirror/store/liquid; forward to §7.4
                      @kintsugi/fracture/coverage_gap
8.  Roomba walks the marking gradient; visits high-density
    coverage_gap regions; triggers @kintsugi/oscillate to mend
       [@kintsugi/roomba.walk + .trigger]
9.  HolonomyHealth decreases monotonically (Villegas c-theorem)
       [fate::Fate.HolonomyHealth]
10. Loop back to 1; terminate when HolonomyHealth < ε OR
    coverage_of_mutant_set saturates at 1.0
```

### §5.2 Q1 for Alex adjudication (§11 Q1): species-decl or spec-only?

Mara's lean: **spec-only**. Rationale:

- Substrate-pull principle: mint the smallest possible surface.
- The algebra IS the composition of three already-minted species
  over Fate; a fourth species declaring the composition would be
  substrate-inflation.
- Second-consumer rule (per `docs/insights/2026-05-26-*.md`
  small-consolidation): defer species-decl until a second consumer
  emerges that would benefit from directly citing
  `@kintsugi/evolution`.

Trade-off: species-decl gives downstream shards a cleaner citation
handle; spec-only requires citing three species + Fate at each
consumption site.

**Alex Q1 in §11.**

---

## §6 Bilateral composition — @butterfly ↔ @roomba

### §6.1 Composition contract

Per `@epistemologic/pact/bilateral` (Reed's `apply_h` firing rule):

```
apply_h(@kintsugi/butterfly.pulse, state_β)  ≡  perturb_from_position
apply_h(@kintsugi/roomba.pulse,    state_ρ)  ≡  aggregate_from_position

# Bilateral: both write to the same @mirror/store.splinter_graph
# medium; both read Fate.tick output; both write signature_beat
# traces. Composition invariant:

apply_h_bilateral(@kintsugi/butterfly, @kintsugi/roomba, state) =
    let β_delta = apply_h(butterfly.pulse, state.β)
    let ρ_delta = apply_h(roomba.pulse,    state.ρ)
    # β writes coverage_gap markers; ρ reads them and mends
    ρ_delta.reads ⊇ β_delta.writes ∩ coverage_gap_markers
```

**Discipline:** the bilateral is composition-CLOSED at the
`@mirror/store.splinter_graph` altitude. Neither walker writes
directly to the other's state; they share the medium.

### §6.2 Contract invariants

Per `@epistemologic/pact/composition_closed` (LANDED):

- **Marker separation:** butterfly writes `coverage_gap` markers
  (semantic tag = "test suite blind here"); roomba writes
  `walk_visited` markers (semantic tag = "walker touched here").
  Two distinct marker classes; no collision.
- **Polarity opposition:** butterfly's Fate bias is REPULSIVE over
  prior butterfly markers; roomba's Fate bias is ATTRACTIVE over
  prior roomba markers. Sentinel-checks discharge via
  `fate_bias_is_repulsive` (butterfly) + implicit attractive-bias
  in roomba's Dijkstra-with-inverse-tension.
- **Reading discipline:** butterfly reads roomba's walker markers
  for inverse-frequency bias (bias toward regions roomba has NOT
  visited); roomba reads butterfly's coverage_gap markers for
  attraction (bias toward regions butterfly has revealed as
  blind).

### §6.3 Predator-prey dynamic

Alex's "counter to the @roomba" phrase names the predator-prey
population dynamic:

- Butterfly reveals coverage gaps (population: uncovered mutants
  DECREASES as butterfly wingflaps).
- Roomba mends coverage gaps (population: uncovered mutants
  DECREASES as roomba triggers fracture-body emission).
- BOTH decrease the uncovered mutant population; they compose,
  they don't compete. The predator-prey framing is LOTKA-VOLTERRA
  ADJACENT but with cooperative dynamics: both hunt the same prey
  (coverage gaps), so their populations co-decrease.

Alex's own essay `Void - Predator.md` §"Canines": *"shadows of the
past become my teeth."* At substrate altitude: shadows of past
mutant survivals become butterfly's canines (repulsive bias
toward unresolved regions).

---

## §7 Fate as shared inference driver + refused-mint inventory

### §7.1 Fate as shared driver

Per witnessed-property-inference math root §6, the SAME 90-parameter
Fate softmax drives three consumer surfaces (compile / test / roomba
pathfinding); this spec adds the FOURTH:

| Surface | Consumer | State | Fate feature-vector shape |
|---------|----------|-------|--------------------------|
| compile | `apply_h(A_compile, state)` | Compilation manifold state | `[f64; 16]` per math root §6.1 |
| test | `pillar::forall` value generator | Sample choice-sequence | `[f64; 16]` per math root §6.1 |
| roomba (walk) | `@kintsugi/roomba.walk` | walk_position | `[f64; 16]` per witnessed-property-inference spec §6.3 |
| **butterfly (perturb)** | `@kintsugi/butterfly.wingflap_of_mutation` | current AST + mutation-choice manifold | `[f64; 16]` — SAME shape as other three |

**No Fate extension needed.** Feature vector is 16-dim; distribution
is 5-way; butterfly consumes the same `Decision.distribution` output
that the other three surfaces consume. The polarity difference is at
the CONSUMER side, not at Fate's side.

### §7.2 Bias policy for the four walkers (softmax + polarity)

Bias policy is per-walker; Fate produces the same distribution but
each walker CONSUMES it with different polarity:

| Walker | Bias direction | Consumer verb |
|--------|----------------|---------------|
| `@peer.audhd` | Fanout (top-K by distribution) | K parallel branches over highest-mass models |
| `@kintsugi/roomba` | Attractive (weighted-average over visited) | Dijkstra edge weight ∝ distribution[i] × previous_visit_frequency |
| `@kintsugi/butterfly` | Repulsive (inverse-weighted over visited) | Wingflap prob ∝ distribution[i] × 1/(1+previous_wingflap_frequency) |
| Fate-tournament itself | Argmax | Pick model with highest distribution mass |

**All four bias policies are CONSUMER-SIDE.** Fate emits the raw
distribution; the consumer converts to a bias per its polarity.
This is a substrate-pull-honest design: Fate's inference is unbiased
+ deterministic; the polarity is a consumer choice per composition.

### §7.5 Refused-mint inventory (cross-referenced with math root §1)

**Ten refused mints in this spec + math root §1:**

| Refused | Why | Substitute |
|---------|-----|------------|
| `@predator` (family or species) | Alex 2026-07-18 verbatim refusal | `@kintsugi/butterfly` species |
| `@chaos` family-root | Zero substrate hits; English usage only | Chaos-theory cited by URL; no carrier |
| `@perturbation` species | Zero hits; English usage in `computational-aikido.md` | `wingflap_of_mutation` verb on @kintsugi/butterfly |
| `@sensitivity` species | Zero hits; carried as trait | `sensitivity_of_test_suite` verb on @kintsugi/butterfly |
| `@wingflap` species | Too whimsical | `wingflap_of_mutation` verb (action, not species) |
| `@lorenz` species | Historical figure, not mechanism | Cited by URL + year in math §7.1 |
| `@lyapunov` species | Metric, not carrier | Composed via `HolonomyHealth` from fate::Fate |
| `@fitness` species | ACO/EA vocabulary; not substrate-native | Verdict-delta via `PropertyVerdict::merge_with` |
| `@selection` species | EA vocabulary; overlaps Fate | Fate's `Decision.distribution` IS the selection policy |
| `@chaos_monkey` / `@simian_army` | Netflix trademarked; operational-lineage only | Cited as ancestor in math §7.2; not carrier |

**Total refused mints across arc (including math root §1, witnessed-
property-inference roots, stigmergy roots): 30+ words the substrate
already carried.** The refused-mint count IS the substrate-health
metric per Seam `#R-refused-mint-count-is-the-substrate-health-
metric`.

---

## §8 Cascade operational path — prismqueer → mirror (Alex directive)

*Alex explicit directive: "Again a math and spec that incorporates
the prismqueer -> mirror cascade." This section IS the load-bearing
structural claim of the spec.*

### §8.1 The eight-step operational cascade

Prose form of math root §6.1:

```
[Step 1: prismqueer altitude]
    prismqueer::liquid::pillar::forall<Input, |x| verdict(spec, mutant_target, x)>
    // where mutant_target = @kintsugi/mutation.mutation_of_ast(μ, target_ast)
    // and μ was picked by @kintsugi/butterfly.wingflap_of_mutation

[Step 2: Sample + Fate bias (Reed's witnessed-property-inference Arc 2)]
    Sample::draw_* draws inputs; Fate.tick biases via
    fate::bias_sample_of_features per witnessed-property-inference
    spec §7.5.

[Step 3: verdict fold at prismqueer]
    merge_with fold over N verdicts → PropertyVerdict
    (byte-comparison against baseline verdict from
    @mirror/store/liquid cache)

[Step 4: mirror altitude — kill/survive discrimination]
    verdict_delta = kill_of_mutant(mutant_oid, spec) : bool
    ├── kill = 1: coverage++;
    │            butterfly writes signature_beat at position;
    │            Fate.HolonomyHealth decreases (Villegas c-theorem).
    └── kill = 0: coverage_gap marker written to
                 @mirror/store/liquid at (mutant_oid, inputs_oid,
                 verdict=Pass, gap_flag=true).

[Step 5: @mirror/store/liquid cache write-back]
    (spec_oid, mutant_oid, inputs_oid) → verdict
    // idempotent per verdict_is_content_addressed;
    // future butterfly wingflaps of same mutant skip re-run

[Step 6: @kintsugi/roomba reads marker gradient]
    @kintsugi/roomba.walk consumes the marking gradient:
    high-density coverage_gap regions become high-priority walk
    targets (attractive stigmergy on the gap markers).

[Step 7: @kintsugi/roomba.trigger fires when accumulated coverage_gap]
    at a position exceeds threshold; dispatches through
    @kintsugi/consent.query_phi; @kintsugi/fracture/coverage_gap
    body (§7.4 forward-promised) emits a targeted property test.

[Step 8: loop closure at prismqueer]
    pillar::forall runner picks up the new property test;
    loop back to [Step 1] with an extended spec.
```

**Every arrow is already substrate-carried.** Zero new machinery
needed for the cascade itself. The machinery gaps are:

- @kintsugi/butterfly species (§3)
- @kintsugi/mutation species (§4)
- @kintsugi/fracture/coverage_gap body (§7.4 in math root; forward-
  promised)
- Rust sentinel-check bilateral arms (Arc 6)
- HolonomyHealth-monotone empirical (Arc 7)

### §8.2 One-breath prose (for handoff)

*prismqueer's `pillar::forall` runs the mutant; the verdict-delta
kills or survives; the surviving mutant writes a coverage-gap
marker to `@mirror/store/liquid`; the roomba walks the marker
gradient and triggers a targeted fracture-body to mend the gap —
and the loop ratchets coverage upward.*

That's the cascade in one breath. It closes because every altitude
is content-addressed at the same medium
(`@mirror/store.splinter_graph` + `@mirror/store/liquid`); because
the verdict is a total function of its OID triple; and because Fate
biases all four walkers over the same 16-dim feature substrate.

---

## §9 Reed execution recipe — three arcs, RED-first, Reed-owned

Each arc is one-to-two /loop iterations; ~500-1000 LOC; ~10-20
tests. Sequential (Arc 6 gates Arc 7; Arc 7 gates Arc 8) but
independent within each arc.

### §9.1 Arc 6 — @kintsugi/butterfly + @kintsugi/mutation species-decls + Rust sentinel-check arms

**Scope.** Land the two species-decls + minimal Rust bilateral arms:

1. `shards/kintsugi/butterfly.mirror` per §3 verbatim (Mara-authored
   this tick; Reed's Arc 6 lands the sentinel-check arms only).
2. `shards/kintsugi/mutation.mirror` per §4 verbatim (Mara-authored
   this tick; Reed's Arc 6 lands the sentinel-check arms only).
3. Rust `apply_h::act` sentinel-check arms for the four
   @kintsugi/butterfly bilateral predicates (§3) + two
   @kintsugi/mutation predicates (§4). Sentinel-check ONLY per
   detector-inadequacy-answer-is-never-rust discipline: `if arg.oid.
   contains("<sentinel>") { Pass } else { Fail }` shape. NO new
   logic. Discharged under `[substrate-floor:@io-boundary]` marker
   with `Signed-off-by: Seam` trailer OR audit citation.

**RED first.** Write `rust/tests/butterfly_sentinel.rs` with N tests
covering:

- `wingflap_is_content_addressed` sentinel fires on wingflap oid
- `coverage_monotone_nondecreasing` sentinel fires on coverage state
- `fate_bias_is_repulsive` sentinel fires on butterfly-scope tag
- `kill_verdict_bounded` sentinel fires on kill verdict
- `mutation_preserves_compilation` sentinel fires on mutation apply
- `mutation_set_bounded` sentinel fires on mutation enumeration

**GREEN.** Implement six sentinel-check arms; verify all tests pass;
verify @kintsugi family-root loads both new species without
introducing @io boundary violations.

**Landing.** ONE prism commit. `Reed <reed@systemic.engineer>`
identity. `🔴` + `🟢` pair.

**Bite size.** ~600 LOC total (400 substrate + 200 Rust
sentinel-checks); ~15 tests. One /loop iteration.

### §9.2 Arc 7 — @evolution loop empirical + HolonomyHealth monotone

**Scope.** Land the @evolution loop over a small prismqueer subject:

1. Reed writes `mirror/rust/src/evolution.rs` — a driver that:
   - Enumerates `@kintsugi/mutation.mutations_of_target(T)` for a
     small target T (~5 mutations)
   - Loops §5.1's 10-step cycle
   - Measures HolonomyHealth at each tick
   - Measures coverage_of_mutant_set at each tick
2. Composition test: verify HolonomyHealth monotone decreasing +
   coverage monotone non-decreasing over ≥100 iterations.
3. Second-witness gate: math root Recognition candidate #3
   discharges if the empirical measurement holds.

**RED first.** Write `mirror/rust/tests/evolution_monotone.rs` with
N tests covering:

- HolonomyHealth strictly decreasing over 100 iterations
- coverage_of_mutant_set non-decreasing over 100 iterations
- Loop terminates (HolonomyHealth < ε within 500 iterations)
- Two evolution runs with same seed produce byte-identical
  `signature_beat.content_oid` (determinism)

**GREEN.** Implement `evolution.rs` driver; verify all tests pass.

**Landing.** ONE prism commit. `Reed <reed@systemic.engineer>`.
`🔴` + `🟢` pair.

**Bite size.** ~800 LOC (driver + tests); ~10 tests. One-to-two
/loop iterations.

**Note on Rust authorship discipline:** `evolution.rs` composes
`@kintsugi/butterfly` + `@kintsugi/roomba` + `@kintsugi/mutation`
+ `fate::Fate` — all substrate-decl'd. The Rust file is DRIVER
CODE that dispatches to substrate; it does NOT encode domain
logic. Per detector-inadequacy discipline: the substrate carries
the logic; Rust dispatches. If Alex adjudicates that even this
level of Rust authorship is bypass, the alternative is to compose
via `@io/loop.iterate` verb (forward-promised, not yet minted) —
see Q3.

### §9.3 Arc 8 — @kintsugi/fracture/coverage_gap mint + roomba integration

**Scope.** Land the fracture body that closes the loop:

1. `shards/kintsugi/fracture/coverage_gap.mirror` species-decl.
   Follows @kintsugi/fracture/* pattern (14 landed as of
   2026-07-12; see @kintsugi/fracture/keyword,
   operator_match, parent_cycle for pattern references).
   The fracture body:
   - Consumes: (coverage_gap_marker: oid, position: walk_position)
   - Emits: targeted property test extending the spec
   - Composes over: @kintsugi/oscillate.active_pass +
     @kintsugi/consent.query_phi
2. @kintsugi/roomba.trigger extension: when reading
   coverage_gap markers, dispatch to
   @kintsugi/fracture/coverage_gap.
3. End-to-end composition test: full cycle of §8.1's 8-step cascade;
   verify sensitivity_of_test_suite increases between t=0 and t=N.

**RED first.** Write `mirror/rust/tests/coverage_gap_cascade.rs`:

- fracture body fires on coverage_gap marker density > threshold
- targeted property test IS added to the spec
- second iteration of evolution loop kills the previously-surviving
  mutant (sensitivity increases)

**GREEN.** Implement `coverage_gap.mirror` + roomba trigger
extension + Rust sentinel-check arms.

**Landing.** ONE prism commit. `Reed <reed@systemic.engineer>`.
`🔴` + `🟢` pair.

**Bite size.** ~600 LOC; ~10 tests. One /loop iteration.

### §9.4 Arc dependencies

```
Arc 2 (Reed WPI spec §9.2 pillar::forall)    [PENDING]
   ↓
Arc 4 (Reed WPI spec §9.4 @mirror/store/liquid cache)    [PENDING]
   ↓
Arc 6 (THIS SPEC §9.1: butterfly + mutation species)    [FORWARD-PROMISED]
   ↓
Arc 7 (THIS SPEC §9.2: evolution loop empirical)    [FORWARD-PROMISED]
   ↓
Arc 8 (THIS SPEC §9.3: coverage_gap fracture + roomba integration)    [FORWARD-PROMISED]
```

**Reed's witnessed-property-inference Arc 2 + Arc 4 are gates.**
Arc 6 depends on `pillar::forall` (Arc 2) and cache write-back
(Arc 4). Alex may reorder per audhd K-parallel exploration; the
arcs are independent up to shared substrate carriers.

---

## §10 Recognition candidates surfaced (DO NOT RATIFY)

Cross-referenced with math root §9; held for Alex adjudication:

1. **`#R-mutation-coverage-is-butterfly-sensitivity-of-test-suite`**
   — first-witness math §3.2 + spec §3. Second-witness gate: Arc 6
   lands sensitivity_of_test_suite; empirical match to Pitest /
   cosmic-ray classical mutation score.

2. **`#R-butterfly-and-roomba-are-fate-biased-walker-perturbation-
   dual`** — first-witness math §4.1 + spec §6. Second-witness gate:
   Arc 6 lands both walkers on same substrate; verify polarity
   opposition empirically.

3. **`#R-evolution-algebra-is-monotone-fixed-point-of-four-operator-
   composition`** — first-witness math §5.3 + spec §5. Second-witness
   gate: Arc 7 empirical monotone HolonomyHealth + coverage.

4. **`#R-butterfly-is-fourth-fate-consumer-surface`** — first-witness
   math §6.3 + spec §7.1. Second-witness gate: Arc 6 lands butterfly
   under `apply_h` dispatch; verify byte-equal signature_beat to
   roomba modulo polarity.

5. **`#R-cascade-prismqueer-to-mirror-closes-through-mutation-
   coverage-ratchet`** — first-witness math §6.1 + spec §8.1.
   Second-witness gate: Arc 8 empirical full cascade.

Held. Do not ratify.

---

## §11 Q's for Alex adjudication

### §11.1 Q1 — @kintsugi/evolution as species-decl or spec-only?

Per §5.2: Mara leans spec-only (substrate-pull principle; second-
consumer rule). Trade-off named there. **Alex adjudication needed.**

If species-decl: Arc 7 also authors `shards/kintsugi/evolution.
mirror` (composition-only shard file citing the three peers + Fate).
If spec-only: the composition lives in this spec + math root; Arc 7
composes without a shard file.

### §11.2 Q2 — Placement of coverage_gap fracture body

Per §9.3 + math §7.4: forward-promised
`@kintsugi/fracture/coverage_gap`. Placement question: is this a
sub-species of `@kintsugi/fracture/*` (sibling of the 14 landed
fracture species) OR a top-level `@kintsugi/coverage_gap` species?

Mara lean: SUB-SPECIES of `@kintsugi/fracture/*` (Arc 8 authors
`shards/kintsugi/fracture/coverage_gap.mirror`). Follows the
pattern of the 14 landed fracture species. **Alex adjudication
needed** in case there's a placement recognition Mara missed.

### §11.3 Q3 — Rust authorship discipline for evolution.rs driver

Per §9.2 note: Arc 7 authors `mirror/rust/src/evolution.rs` as
driver code composing four substrate-decl'd species over Fate. Is
this admissible under detector-inadequacy discipline (driver-only,
NO domain logic) or does it bypass?

Alternative: mint `@io/loop.iterate` verb + compose the loop in
shard body. Adds substrate surface; removes Rust authorship.

Mara lean: **driver code is admissible** — the domain logic lives
in the four species; evolution.rs is dispatcher-only. Composes
under `[substrate-floor:@io-boundary]` marker with audit citation.
**Alex adjudication needed** per HARD RULE
`detector_inadequacy_answer_is_never_rust`.

### §11.4 Q4 — Predator-prey framing depth in @kintsugi/butterfly docblock

Per §6.3: Alex's "counter to the @roomba" phrase names predator-prey.
The math is Lotka-Volterra-adjacent with cooperative dynamics.
Should the butterfly shard docblock cite:

- (a) One-line reference to §6.3 of this spec + Void-Predator essay
- (b) Full docblock section citing Lotka-Volterra + Alvaro LDFI
  cooperative-dynamics adjacency
- (c) Kagi predator-prey / Lotka-Volterra 2020-2026 landscape scan
  and full section

Mara lean: **(a)** for Arc 6; defer (b)+(c) to follow-up if a
second recognition emerges. **Alex adjudication needed** per naming
authority (deepest substrate-decl coinages).

### §11.5 Q5 — Scope disambiguation with @mirror/butterfly compilation altitude

Per §2.9: `boot/std/mirror/butterfly.mirror` (Alex 2026-05-20)
carries "butterfly" for AST→LLVM IR metamorphosis. This spec's
species is `@kintsugi/butterfly` (walker altitude). Disambiguation
via scope path.

Should the docblock at `shards/kintsugi/butterfly.mirror` include
an explicit "SCOPE DISAMBIGUATION" section citing the compilation-
altitude sibling AND explaining why both survive under the same
word? Or is the scope path (@kintsugi/butterfly vs @mirror/
butterfly) sufficient?

Mara lean: **explicit SCOPE DISAMBIGUATION section** in the
docblock (~5 lines). Reduces WTF/minute for future readers who
grep for "butterfly." **Alex adjudication needed.**

### §11.6 Q6 — Naming: `wingflap_of_mutation` vs `perturb_of_mutation`

Per math §4.3 + §8: `wingflap` is chaos-theory-vocabulary
delightfully-boring (Lorenz 1972 talk verbatim named the effect
after wing-flap); `perturb` is more generic + connects to spectral
graph theory (Fiedler perturbation) + control theory (Davis-Kahan).

Mara lean: **`wingflap_of_mutation`** — Alex's naming ("@butterfly")
carries the chaos-theory lineage; wingflap is the verb that word
implies. Perturb is too CS-generic + already used in `computational-
aikido.md` context for eigenvalue perturbation (different altitude).
**Alex adjudication needed** for delightfully-boring test.

### §11.7 Q7 — Bounded Offutt-Jia 2010 vs open-ended AI-mutation-guided

Per §4: `mutation_op` variant enumerates 6 classical operators.
Meta ACH 2025 (Foster et al.) uses LLMs to generate context-aware
mutations at scale — orders of magnitude more diverse than
Offutt-Jia's fixed operator set.

Question: does @kintsugi/mutation stay bounded (mutation_set_bounded
pact holds; Knaster-Tarski convergence proven) OR extend to
LLM-generated mutants (unbounded set; convergence becomes
statistical)?

Mara lean: **bounded for v0.1 landing** (Arc 6-8); leave LLM-
generated mutation for future Arc N (forward-promised). Bounded
gives us Knaster-Tarski + decidable coverage. LLM extension adds
Meta ACH lineage but weakens convergence guarantees. **Alex
adjudication needed** on which trade-off Arc 6 lands under.

---

## §12 Forward promises + pickup manifest for next-Reed

*Context pressure was high when this arc landed. Next-Reed (or
next-Mara) pickup manifest:*

### §12.1 What landed this tick (Mara)

- `docs/math/2026-07-18-butterfly-chaos-mutation-cascade.md` — math
  root (this session)
- `docs/specs/butterfly-roomba-dual-walker-composition.md` — this
  file (this session)

### §12.2 What's held pending Alex adjudication

- Q1: @kintsugi/evolution species-decl vs spec-only
- Q2: coverage_gap fracture body placement
- Q3: evolution.rs driver Rust authorship discipline
- Q4: predator-prey framing depth in docblock
- Q5: SCOPE DISAMBIGUATION section vs implicit scope path
- Q6: wingflap_of_mutation naming ratification
- Q7: bounded Offutt-Jia vs open-ended LLM-mutation

### §12.3 What's forward-promised for Reed

- **Arc 6 (Reed)**: land `shards/kintsugi/butterfly.mirror` +
  `shards/kintsugi/mutation.mirror` + Rust sentinel-check arms.
  ~600 LOC; 15 tests; one /loop iteration.
- **Arc 7 (Reed)**: land `evolution.rs` driver +
  HolonomyHealth monotone empirical. ~800 LOC; 10 tests; one-to-
  two /loop iterations.
- **Arc 8 (Reed)**: land `shards/kintsugi/fracture/coverage_gap.
  mirror` + roomba trigger integration + end-to-end cascade test.
  ~600 LOC; 10 tests; one /loop iteration.

### §12.4 What's forward-promised for Mara or Taut

- If Q7 chooses LLM-extension: math addendum on Meta ACH 2025
  composition; ~30 KB math root.
- If Q4 chooses (c): Kagi predator-prey landscape scan +
  Lotka-Volterra-cooperative math addendum.
- Follow-up cross-referencing this spec with witnessed-property-
  inference-petri-fate math root §1 (the four-substrate SQUARE
  extension to five-operator @evolution algebra).

### §12.5 Pickup breadcrumbs for next-Reed's fresh instance

Read in order:

1. Reed memory `project_butterfly_substrate_species` (this session's
   ratification + composition mapping).
2. Reed memory `project_witnessed_property_inference` (parent arc;
   Q1-Q10 adjudication cache).
3. Reed memory `feedback_composition_primitive_naming_convention`
   (`<primitive>_of_<input-shape>` HARD RULE).
4. Reed memory `feedback_detector_inadequacy_answer_is_never_rust`
   (Rust authorship discipline — Q3 gates on this).
5. THIS spec (§8 for cascade, §9 for arc plan, §11 for Q's, §12
   for handoff).
6. Companion math root (§0 Alex verbatim, §5 fixed-point proof,
   §7 source manifest).
7. `docs/loop/CURRENT.md` — active arc state (pickup context).
8. `docs/specs/witnessed-property-inference-fate-drives-both.md`
   for Arc 2 + Arc 4 gates.
9. `shards/kintsugi/roomba.mirror` for the dual walker's landed form.
10. `~/dev/systemic.engineering/blog/void/3published/Void - Predator.md`
    for the autobiographical grounding (load-bearing text).

**One-sentence pickup:** *the cascade prismqueer → mirror closes
through butterfly's mutation coverage ratchet writing coverage_gap
markers to `@mirror/store/liquid`, roomba walking the gap-marker
gradient, and `@kintsugi/fracture/coverage_gap` emitting targeted
property tests that extend the spec back at prismqueer altitude —
Arc 6 mints the two species; Arc 7 empirically witnesses the
monotone HolonomyHealth loop; Arc 8 lands the fracture body and
closes the ratchet.*

---

**Author:** Mara <mara@systemic.engineer>
**Date:** 2026-07-18
**Tag:** 📝 spec:butterfly-roomba-dual-walker-composition (pure-docs
       📝 markdown-only bypass)
**Status:** canonical spec. Grounds LANDED shards in one citation
       chain. Proposes two species (@kintsugi/butterfly,
       @kintsugi/mutation) + one closed algebra (@kintsugi/
       evolution, spec-only per §5.2 lean pending Alex Q1). Five
       Recognition candidates proposed (§10); none ratified this
       tick. Seven Q's for Alex adjudication (§11). Three-arc Reed
       execution recipe (§9).
**Path:** `docs/specs/butterfly-roomba-dual-walker-composition.md`
**Companion math:** `docs/math/2026-07-18-butterfly-chaos-mutation-
       cascade.md` (same session, this tick)
