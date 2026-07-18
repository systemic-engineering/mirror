---
title: Witnessed Property Inference — Fate drives both (companion canonical spec)
subtitle: Petri nets as the operational carrier; content-addressed cache invalidation; three consumer surfaces (property tests, mirror compilation, roomba stigmergy) driven by one Fate inference over one @mirror/petri marking.
status: canonical-spec
date: 2026-07-18
author: Mara
---

# Witnessed Property Inference — Fate drives both

*Mara 2026-07-18. Canonical spec companion to
`docs/math/2026-07-18-witnessed-property-inference.md` (Mara,
math root, 2026-07-18) and its four-vertex extension
`docs/math/2026-07-18-witnessed-property-inference-petri-fate.md`
(Mara, math addendum, 2026-07-18). This spec names the OPERATIONAL
composition Alex publicly committed to in the 2026-07-18 substack
post:*

> *"The petri nets become the driver of the content-addressed
> witnessed computation. A property that was verified before does
> not need to be verified again unless something in the code graph
> changes. The same content-addressed witnessed computation drives
> the Stigmergy of the roomba's pathfinding through the graph. The
> ants, Jason Kerr. 🐜🐜🐜"*

*Status: canonical spec. Pure-docs 📝 markdown-only bypass. Zero
family-roots minted; zero species minted; every new operational
verb is a `<primitive>_of_<input-shape>` composition over an
already-landed carrier per Alex-ratified naming discipline. Six
refused mints (§7.5). Five recognition candidates surfaced
(§10); none ratified this tick.*

---

## §1 Executive summary

The composition Alex named lives across FOUR already-landed
substrates (Traces, Petri, Fate, Properties) and drives THREE
already-landed consumer surfaces (property tests, mirror
compilation, roomba pathfinding). The math is proven in the
companion math roots; the operational spec below is the plan for
Reed to land the five-arc bite sequence that closes the loop.

Load-bearing bullets:

1. **Petri nets are the driver.** `@mirror/petri` (spec'd at
   `docs/specs/subject-family-root-sel-licensable-party.md` §5,
   Mara 2026-07-14, Alex-adjudicated Taut-D8 rename from
   `@mirror/property`) carries the operational marking Fate reads
   and biases. Fate's `Decision.distribution: [f64; 5]` IS a
   biased firing policy over the enabled-transition subset at each
   marking. Marking evolution IS the trace.

2. **Content-addressed cache invalidation.** Every property
   verdict is a total function of `(spec_oid, target_oid,
   inputs_oid)` per `@epistemologic/property/
   verdict_is_content_addressed` (Reed N1 2026-07-12); the
   idempotent-closure proof of the math root §4 discharges the
   cachability claim. A property verified before does not need
   re-verification unless one of the three OIDs changes.

3. **Three consumer surfaces, one Fate.** The same 90-parameter
   Fate softmax drives (a) property-test generation via biased
   choice-sequence, (b) mirror compilation decisions via
   `apply_h` combinator dispatch, (c) roomba stigmergy walker
   pathfinding via biased next-node selection over the substrate
   DAG. Cite math root §6.

4. **Roomba pathfinding is the third surface Alex added at
   substack altitude.** The math root §6.3 named three surfaces
   (compile / test / shrink); Alex's substack fold-in replaces
   "shrink" with "roomba stigmergy walker" as the substrate's
   third consumer surface. Shrink stays a surface (byte-buffer
   reduction) but is subsumed by roomba backward-firing on the
   Petri net (see §6.5).

5. **Jason Kerr grounds the roomba altitude.** Kerr, Director
   Max Planck Institute for Neurobiology of Behavior — CAESAR
   (Bonn), studies mammalian vision + decision-making; the ant
   pheromone / navigation lineage he anchors (via Knaden 2016
   sensory ecology of ant navigation, cordis EU 759817) pairs
   with Grassé 1959 stigmergy already cited in
   `docs/math/2026-07-18-stigmergy-witnessed-computation-
   mycelial-composition.md`.

6. **Zero new family-roots. Zero new species.** All operational
   surface lands as `<primitive>_of_<input-shape>` verbs over
   `@mirror/petri`, `fate::Fate`, `@spectral/signature`,
   `terni::PropertyVerdict`, `@kintsugi/roomba`. Substrate had
   every word.

7. **Refused mints (§7.5): six.** `@petri`, `@marking`,
   `@transition`, `@token`, `@firing_policy`, `@ant`. The
   refusal count IS the substrate-health metric per Seam
   `#R-refused-mint-count-is-the-substrate-health-metric`.

8. **Landing arc (§9): five bites, RED-first, Reed-owned.** Arc 1
   `pillar::of_health` + Fate bridge (smallest); Arc 2 `Sample` +
   `Arbitrary` + `forall` (Taut §8 Surface A); Arc 3 Petri-net
   compilation-loop empirical; Arc 4 Content-addressed cache
   write-back to `@mirror/store/crystal`; Arc 5 Roomba stigmergy
   composition (Fate biases next-node walk).

---

## §2 Substrate ground truth (what already exists)

### §2.1 The pillar composition surface (iter 1-10)

Reed landed six pillar primitives at `prismqueer::liquid::pillar`
across ten /loop iterations closing 2026-07-18 08:00 UTC (task
#258 GREEN). Per `docs/specs/prismqueer-liquid-pillar-composition-
surface.md`:

- `dispatch_ambiguity` (Pillar I byte-visible)
- `algedonic` + `algedonic_of_magnitude` (Pillar II single-tick)
- `viability` + `viability_of_magnitudes` (Pillar III multi-tick)
- `fold` (verdict-fold primitive)

Total: 98 property tests across four substrate altitudes; all
return `terni::PropertyVerdict`; all compose via `merge_with` or
`fold`. **Pillar IV remains PARKED** on the `fate::Fate::tick`
bridge at `mirror/rust/src/liquid.rs` — precisely the bridge this
spec's §9 Arc 1 lands.

### §2.2 The Fate 5-model selector (pre-arc)

Per `fate/src/lib.rs` (Alex, pre-arc); one binary, five weight
sets; five sub-models mapped to five Prism transition classes:

| `Model` | Verb | Transition class |
|---------|------|------------------|
| Abyss | focus | Observe: reads marking, no state change |
| Introject | project | Reduce: projects marking to selected subspace |
| Cartographer | split | Split: partitions marking into K sub-markings |
| Explorer | zoom | Zoom: samples marking density |
| Fate | refract | Refract: crystallizes selected outcome |

Softmax output at each tick: `Decision { model, confidence,
distribution: [f64; 5] }`. Plus `HolonomyHealth` scalar reading
the Yang-Mills flow of the substrate at that marking.

### §2.3 `@mirror/petri` — the Petri-net analyzer family-root

Spec'd at `docs/specs/subject-family-root-sel-licensable-party.md`
§5.1 (Mara 2026-07-14, Alex-adjudicated Taut-D8 rename from
`@mirror/property` — "the substrate tells us what it wants to be
called"). Substrate-decl carrier:

```mirror
type petri_net = {
  places:       ref,   # typed graph regions
  transitions:  ref,   # signature-detection rules; one per §5.2-§5.4
  tokens:       [sel], # current marking; evolves during analysis
  firing_rules: ref,   # per-transition bilateral predicate
}
```

Task #81 AT2 rename landed the family-root at `@mirror/petri`;
the shard file `shards/mirror/petri.mirror` is still pending Alex
adjudication A2-A8 per subject-family-root spec §8. **This spec
does not author the shard file** — it grounds the operational
composition that happens over `@mirror/petri` once landed.

### §2.4 Stigmergy math + spec (Mara 2026-07-18 sibling roots)

- `docs/math/2026-07-18-stigmergy-witnessed-computation-mycelial-
  composition.md` (Mara `d7ff58e`, 30.6KB) — the Grassé-Theraulaz-
  Heylighen ≅ substrate composition. Agent = `@kintsugi/roomba`
  walker; trace = `signature_beat`; medium = `@mirror/store`
  splinter_graph; mycelial anastomosis = two roombas' beat-chains
  sharing an OID collapse.
- `docs/specs/2026-07-18-stigmergy-witnessed-computation-mycelial-
  composition.md` (Mara `95c0e4a`, 20.5KB) — the canonical spec
  form.
- Refused mints: `@stigmergy` family-root, `@pheromone` species —
  substrate had every word.

### §2.5 `@kintsugi/roomba` walker (Mara + Reed 2026-07-14+)

Landed at `shards/kintsugi/roomba.mirror` (46.4KB). Five actions:

- `walk(from, budget) -> walk_position` — Dijkstra + tension-
  weighted edges
- `bump(position) -> spectral_tension` — resonance sensor
- `trigger(position, tension) -> verdict` — pivot decision
- `pulse(position) -> (walk_position, roomba_state)` — one beat
- `run(seed) -> walk_position` — the walker's outer loop

Four bilateral predicates: `walk_terminates_cleanly`,
`bump_is_reciprocal`, `trigger_fires_on_significant_tension`,
`pulse_is_periodic`. The walker's beat-chain IS the stigmergic
pheromone trail (per stigmergy math §2 Grassé mapping).

### §2.6 `fragmentation::Witnessed` at `/Users/alexwolf/dev/
projects/fragmentation/`

Per math root §2.1: *"Different witness, different commit. Same
content, same tree."* The `Witnessed` type carries the observer as
a first-class field on the commit; the content-hash resolves via
the tree OID (BLAKE3 in `@mirror/store`; SHA-1 in git wire). This
is the ancestor for the content-addressed cache invalidation of §5.

### §2.7 `@epistemologic/property/verdict_is_content_addressed`

Landed shard (Reed N1 Tick 1, 2026-07-12): `verdict(spec_oid,
target_oid, inputs_oid) -> verdict` is a TOTAL FUNCTION of its
three OID inputs. This is the memoization-by-construction primitive
the cache in §5 reads back.

### §2.8 `apply_h` bilateral firing rule

Per `shards/epistemologic/pact/bilateral.mirror` (14.1KB); the
substrate's existing bilateral firing rule with the shape:

```mirror
apply_h::act(pred, args) -> verdict
```

Per Reed iter 1-4 landings (`bootstrap/src/apply_h.rs`; note
transitional Rust — Reed memory `rust_floor_is_rust_not_bootstrap`
is the terminal FLOOR discipline; the composition of §5 grounds
in the shard, not the transitional bootstrap). This is the firing
function the three consumer surfaces of §6 invoke over the same
`@mirror/petri` net.

---

## §3 Petri nets as the operational carrier

### §3.1 Marking evolution IS the witnessed-computation trace

The `@mirror/petri.petri_net` carrier holds four typed fields;
`tokens: [sel]` is the marking. Under the composition of this
spec, **tokens are content-addressed witnessed traces** — not just
subject-touching sum-type composites (their SEL role), but at the
generalized altitude of this spec: content-addressed pointers into
the `@mirror/store.splinter_graph` DAG.

The generalization: `sel` is one legal token type (the
SEL-enforcement altitude); the general marking carries ANY type
whose value has an `emit_oid: oid` field content-addressing its
witness state. Per math addendum §2.3, the `sel` token is not a
placeholder or color label — it carries FULL WITNESS at every
firing.

Formally: a token `t` at place `p` deposited by transition firing
`f` is a Merkle-DAG node with the seven `signature_beat` fields per
`shards/spectral/signature.mirror:106-114` (Reed `f211ee48`):

```
t.contribution_oid := oid(t.content)                   # observer-independent content-address
t.sc_at_beat       := SpectralCoordinate<5>(marking(p))# marking snapshot
t.rung             := @song/beat.rung                  # depth in beat-chain
t.previous_beat    := option<oid>                      # prior beat that produced this token
t.timestamp        := @time/monotonic.instant          # ordering scaffold
t.ssh_fingerprint  := ref                              # firer identity — different signer, different token
t.address          := uuid_spectral_time               # beat's own address in annotation coordinate space
```

**A token at a place IS a signature_beat at that place's altitude.**
No translation layer needed; no re-witness computation; the marking's

**Post-cf34549 REED-INLINE cascade:** field names corrected from cf34549
draft's `emit_oid` (→ `contribution_oid`) and `witness: subject_instance`
(→ `ssh_fingerprint: ref`). Semantic distinction preserved: observer-
independent content-address (`contribution_oid`) vs observer-identity
(`ssh_fingerprint`) — substrate carrier names them per the shard.

The marking's
byte-content IS the beat-chain's byte-content.

### §3.2 The Grassé-Petri isomorphism

The stigmergy math root §2 established the Grassé-Theraulaz-Heylighen
≅ substrate composition:

| Grassé | Substrate | Petri (this spec) |
|--------|-----------|-------------------|
| Agent | `@kintsugi/roomba` walker | Firing rule that consumes token from input place |
| Trace (pheromone deposit) | `signature_beat` | Token at output place |
| Medium | `@mirror/store` splinter_graph | Places (typed graph regions) |
| Stimulation rule | `surface_class` + `pivot(@song)` | Firing rule bilateral (per-transition) |
| Anastomosis (two agents' traces share identity) | Two roombas' beat-chains sharing an OID collapse | Token OID collision at output place = merge |

The Petri altitude is where Grassé's four-part structure ratifies
as substrate composition. Alex named this as the fourth vertex of
the composition SQUARE ("Petri Nets. Those too!"). The math
addendum §1 formalizes the SQUARE closure.

### §3.3 Why Petri, not Turing

Per subject-family-root spec §4.6 (verbatim): *"Petri-nets are
bounded, decidable, structurally analyzable. Not Turing-complete.
That is the design principle, not an incidental property."*

For the SQUARE closure, this is load-bearing at the coverage-
guarantee altitude. Karp-Miller 1969 gives decidable coverability
+ boundedness for bounded nets → Fate-biased firing over the
coverability graph converges to full-coverage sampling in bounded
state-space (math addendum §2.2 Reisig 2013 §5.3). This is a
STRONGER guarantee than any Turing-PBT convergence claim (which is
only probabilistic).

### §3.4 Refuse mint list at Petri altitude

Substrate-already-had-the-word discipline:

- **REFUSE `@petri` family-root.** `@mirror/petri` (already spec'd,
  §2.3) suffices.
- **REFUSE `@marking` species.** `petri_net.tokens: [sel]` is the
  marking; no separate species.
- **REFUSE `@transition` species.** `petri_net.transitions: ref`
  is the transition set; no separate species.
- **REFUSE `@token` species.** `sel` (per `@sel` family-root spec)
  is the token type; the generalization of §3.1 is a shape claim
  ("any type with `emit_oid: oid`"), not a species.
- **REFUSE `@firing_policy` species.** Fate's
  `Decision.distribution` IS the policy; composed via `apply_h`
  and one new verb `bias_of_features` (§7.3), not minted.

Substrate-honest note: this refusal list composes with the math
root §1's ten refused mints (`witness`, `witnessed`, `trace`,
`sample`, `arbitrary`, `forall`, `choice_sequence`, `generator`,
`shrinker`, `HolonomyHealth`, `PropertyVerdict`, `Fate::propose`)
and the math addendum §8's five (Petri-vertex mints listed above)
into a **total substrate-honest refused-mint inventory of ≥20
words the substrate already carried** across the composition arc.

---

## §4 Fate as biased firing policy

### §4.1 The composition claim

Per math addendum §3.2: **Fate's `Decision.distribution: [f64; 5]`
IS a biased firing policy over the `@mirror/petri` transition
set.** Formally:

```
π_Fate(t | M) = distribution[model_of(t)] / |{t' ∈ T_enabled(M) : model_of(t') = model_of(t)}|
```

where:
- `M` is the current marking
- `T_enabled(M) ⊆ T` is the subset of transitions enabled at M
- `model_of: T → Model` maps each transition to one of Fate's
  five sub-models (per §2.2)
- `distribution` is Fate's softmax output at features `f(M)`

Classical Petri firing picks uniformly from `T_enabled(M)`;
Fate-biased firing picks according to `π_Fate(t | M)`.

### §4.2 HolonomyHealth as coverage feedback

The critic side of the actor-critic RL framing (math addendum §3.3):
`FateOutput.health: HolonomyHealth` scores the current marking's
coverage quality. Fate biases toward transitions leading to LOW
HolonomyHealth markings (unexplored territory), modulated by depth
(deeper in the tick loop → more exploitation).

**No new primitive needed** — Fate already emits HolonomyHealth
per `fate/src/lib.rs`. The composition into pillar verdicts happens
via `pillar::of_health` (§7.2) — the ONE new primitive at Pillar
altitude.

### §4.3 The projection to `PropertyVerdict`

Per math addendum §4:

- `h ≈ 0` (perfectly witnessed marking; no residual curvature) →
  `PropertyVerdict::Pass`
- `h ≈ 1` (fully diverged marking; no coherent witness) →
  `PropertyVerdict::Fail(HolonomyDiagnostic{…})`
- `h ∈ (0, 1)` (partial witness; some sheaf sections cohere,
  others don't) → `PropertyVerdict::Partial { confidence: 1.0 - h,
  diagnostics: … }`

The projection preserves the semilattice order → `pillar::of_health`
is an order-preserving semilattice morphism. This makes the Fate
substrate a first-class citizen of the pillar composition surface
without any type-level change to `PropertyVerdict`.

---

## §5 Content-addressed cache invalidation

### §5.1 The consumer surface

Alex's substack framing: *"A property that was verified before does
not need to be verified again unless something in the code graph
changes."*

The mechanism is landed:

- `@epistemologic/property/verdict_is_content_addressed`
  (Reed N1 2026-07-12) makes `verdict = f(spec_oid, target_oid,
  inputs_oid)` a TOTAL FUNCTION of three OIDs.
- Math root §4 discharges the idempotent-closure proof:
  `witnessed(Fate(witnessed(t)))` = `witnessed(t)` on fixed points.
- Cache field (transitional, iter 1-10 substrate): `@mirror/store.
  crystal.derived_predicates` at `shards/mirror/store/crystal.mirror:356`
  stores the witnessed verdicts.
- **Alex-ratified upgrade (Q10, 2026-07-18):** cache location moves
  from `crystal.derived_predicates` to a new species-decl
  `@mirror/store/liquid` composing `@mirror/store` (Alex 2026-07-16
  ratified walker cache) with `@liquid` family-root (Arc 5 M1
  `cc816f9`). Mara mints `shards/mirror/store/liquid.mirror`
  precedes Arc 4 empirical landing. Cache SEMANTICS unchanged;
  substrate location upgraded to name what the field IS — refined
  `@mirror/store` per `@liquid` refinement discipline.

Per math root §4.3: *"The substrate does not merely cache property
verdicts — it PROVES they need not be recomputed."*

### §5.2 The cache key and lookup path

For any pillar-composition invocation with:

```
spec:   a PropertyVerdict-producing function `fn(T) -> PropertyVerdict`
target: the code-graph node being tested (an `@mirror/store` OID)
inputs: the `Sample` choice-sequence (an `@mirror/store` OID via
        content-addressed byte-buffer)
```

the cache key is:

```
cache_key = oid_of((spec_oid, target_oid, inputs_oid))
```

and the lookup is:

```
verdict_of_cache_key(k) := 
  @mirror/store.crystal.derived_predicates[k]
  or fresh(spec, target, inputs) on miss
```

Cache validity holds BY CONSTRUCTION per the idempotent-closure
proof; no invalidation logic needed. **Re-verify iff any of the
three OIDs changes** — which is precisely the code-graph-change
criterion Alex named at substack altitude.

### §5.3 Write-back path (Arc 4)

The landing plan §9 Arc 4 is: on every fresh (cache-miss) verdict
computation, write the resulting verdict back to
`@mirror/store.crystal.derived_predicates[cache_key]`. This closes
the ouroboros at Pillar altitude — the next invocation of the same
spec against the same target with the same inputs reads the cached
verdict without re-running the pillar-composition machinery.

Idempotency is preserved: the cache write is `content_oid`-
addressed; two agents writing the same verdict for the same key
produce byte-identical crystals; anastomosis (per the mycelial
math root §4.2) collapses them to one entry.

---

## §6 The three consumer surfaces

The math root §6.3 named three surfaces (compile / test / shrink);
Alex's substack fold-in generalizes: **the third surface is the
roomba stigmergy walker, not just shrink.** All three surfaces
consume the same `Fate::tick` output at each marking; each writes
its own `signature_beat` recording its use. Categorically distinct
Prisms in the tower; operationally unified through `Fate::tick`.

### §6.1 Surface (a) — Property test generation (PBT)

The generator loop is a Petri firing sequence:

```
forall runner (Taut scout §8 Surface A shape):
  for i in 0..N:
    let mut sample = Sample::random()
    fate.bias_sample(&mut sample, features)         # policy: read Fate
    let value = T::arbitrary(&mut sample)           # firing sequence:
                                                    # draw_integer, draw_bool,
                                                    # draw_from — each is
                                                    # one Petri transition
                                                    # firing consuming/emitting
                                                    # tokens on the choice-seq
                                                    # net
    let verdict = f(value)                          # terminal firing:
                                                    # emits Pass/Partial/Fail
                                                    # into output place
    unified.merge_with(&verdict)                    # fold via semilattice
  unified
```

Every `sample.draw_*` call IS one Petri transition firing on the
choice-sequence net; the terminal `f(value)` firing produces the
verdict token; the fold IS marking-consolidation at the output
place. Cite Taut scout `docs/scouts/2026-07-18-taut-property-
based-testing-frameworks-fate-inference-driver.md` §8 Surface A.

### §6.2 Surface (b) — Mirror compilation decisions

The compilation loop is a Petri firing sequence at a different
altitude:

```
compile loop:
  let mut marking = M0                              # substrate node's ancestry
  loop:
    let features = extract_features(&marking)
    let fate_out = fate.tick(features, depth)
    if terminal(marking): break
    let t = pick_transition_by_π_Fate(&marking, &fate_out.distribution)
    marking = apply_h(t, marking)                   # fire the transition
    emit_signature_beat(&marking, &t)               # write the beat
```

Every `apply_h` invocation IS one Petri transition firing on the
compilation net; the beat chain IS the compilation trace. This is
already partly landed via `bootstrap/src/apply_h.rs` iter 1-4
(transitional bootstrap; the terminal FLOOR is `rust/` per Reed
memory `rust_floor_is_rust_not_bootstrap`; Arc 3 lands the
empirical witness at the shard altitude, not the bootstrap).

### §6.3 Surface (c) — Roomba stigmergy walker pathfinding

The walker's next-node selection is a Petri firing sequence over
the substrate DAG:

```
run loop (per shards/kintsugi/roomba.mirror):
  let mut position = seed
  loop:
    let features = extract_features(&position)      # spectral state at node
    let fate_out = fate.tick(features, depth)
    let tension = bump(position)
    let verdict = trigger(position, tension)
    if verdict.is_pass(): break
    let next = pick_next_by_π_Fate(&position, &fate_out.distribution)
    position = walk(from=position, budget=1)        # Dijkstra step
    (position, roomba_state) = pulse(position)      # one beat
```

Fate's biased distribution over the five Model classes weights
the walker's next-node choice: Abyss=focus (stay), Introject=project
(step backward via ancestry), Cartographer=split (fanout K-walkers),
Explorer=zoom (density-biased next), Fate=refract (commit terminal).

The walker's beat-chain IS the stigmergic pheromone trail per the
Grassé mapping of §3.2. When two roombas' walks share a beat-oid,
mycelial anastomosis collapses them — this is the ensemble-level
K>1 fanout stabilization per stigmergy math §6.

Jason Kerr grounds the scientific lineage here — his MPI Behavior &
Brain Organization group at CAESAR/Bonn studies mammalian
vision + decision-making and the ant-navigation lineage via
Knaden 2016 sensory ecology of ant navigation (EU cordis 759817).
Ant collective decision-making + pheromone-guided pathfinding IS
the biological ancestor for the roomba's Fate-biased next-node walk
over the substrate DAG. See §11 Q4 for Alex adjudication on the
Kerr citation depth.

### §6.4 Symmetry of the three surfaces

Per math addendum §5.2 the harness/SUT collapse theorem: **Same net.
Same firing policy. Same token. Three projections.** The three
surfaces are:

- Surface (a) reads `token.verdict: PropertyVerdict` at the terminal
- Surface (b) reads `token.au_side: au` at the terminal
- Surface (c) reads `token.walk_position: walk_position` at the
  terminal

All three run the SAME `apply_h_star(M0, π_Fate)` iteration. The
projections are ORTHOGONAL commuting functions on the terminal
token. This IS what "the same computation drives both" means at
mathematical altitude.

### §6.5 Shrink as backward roomba firing

The math root §6.3's third surface (shrink) is subsumed under
Surface (c): shrinking IS backward roomba walking on the same net.
The shrinker seeks the minimal marking that still reaches the Fail
place; the walker's `walk(from, budget)` with negative budget IS
that backward step per the Reisig 2013 §4.2 reachability discipline.
Bounded Petri nets guarantee shrinking terminates provably — unlike
QuickCheck's manual shrinker which can loop on adversarial types.

---

## §7 The new primitives to mint

**MINT ONLY WHAT SUBSTRATE DOESN'T ALREADY HAVE.** The grep in §1
(math root) + §8 (math addendum) + the substrate-already-had-the-word
inventory of §3.4 above exhausted the family-root + species
altitudes. This section names the four operational verbs at
`prismqueer::liquid::pillar` + `fate::Fate` altitudes.

### §7.1 `pillar::Sample` and `pillar::Arbitrary`

Per Taut scout §8 Surface A recommendation (Hypothesis-shape, not
hedgehog-shape):

```rust
pub struct Sample {
    buffer: Vec<u8>,       // Hypothesis-style choice-sequence buffer
    position: usize,       // current draw position
}

pub trait Arbitrary {
    fn arbitrary(sample: &mut Sample) -> Self;
}

impl Sample {
    pub fn draw_integer(&mut self, min: i64, max: i64) -> i64 { … }
    pub fn draw_bool(&mut self) -> bool { … }
    pub fn draw_from<T: Copy>(&mut self, choices: &[T]) -> T { … }
    // etc.
}
```

Delightfully-boring per Alex-ratified Reed memory
`feedback_composition_primitive_naming_convention`. Same words
Hypothesis / QuickCheck / proptest use — substrate did not have
these words at operational altitude (grep hits are English-only,
per math root §1); mint permitted.

### §7.2 `pillar::of_health`

The seventh pillar primitive parallel to the six landed at
`prismqueer::liquid::pillar`:

```rust
pub fn of_health<L: Loss + PartialOrd>(
    health: &fate::HolonomyHealth,
    theta_pass: &L,
    theta_fail: &L,
) -> PropertyVerdict
```

- `health >= theta_fail` (very unhealthy) → `Fail(HolonomyDiagnostic{…})`
- `health <= theta_pass` (very healthy) → `Pass`
- otherwise → `Partial { confidence: 1.0 - health.scalar(),
  diagnostics: [] }`

Delightfully-boring: `<primitive>_of_<input-shape>` = `of_health`
(verdict of a HolonomyHealth). Naming parallels the six landed
primitives (see spec `docs/specs/prismqueer-liquid-pillar-
composition-surface.md` §2). Zero type-level change to the pillar
composition surface — same `PropertyVerdict` output; same
`merge_with` / `fold` composition.

### §7.3 `pillar::forall` runner

Per Taut scout §8 Surface A + math addendum §4.4:

```rust
pub fn forall<T: Arbitrary, F: FnMut(T) -> PropertyVerdict>(
    n: usize,
    mut f: F,
) -> PropertyVerdict {
    let mut unified = PropertyVerdict::Pass;
    for _ in 0..n {
        let mut sample = Sample::random();
        let value = T::arbitrary(&mut sample);
        let verdict = f(value);
        unified = unified.merge_with(&verdict);
    }
    unified
}
```

Delightfully-boring: mint permitted at operational altitude per
math root §1 refused-mint table (`forall` has zero substrate
carrier hits; the docblock English usage does not count). Composes
via existing `merge_with`.

### §7.4 `fate::Fate::bias_of_features` (or reuse `Fate::tick`)

The Fate side of the seam: fold the Petri marking's features into
Fate's `[f64; 16]` feature vocabulary and read back the
distribution.

**Grep first — is this already covered?** `Fate::tick` already
takes `Features: [f64; 16]` and returns `Decision.distribution:
[f64; 5]`. Per math root §1 table, `Fate::tick`, `Fate::select`,
`Fate::resolve` already discharge selector-dispatch. This
composition needs no new Fate method — the pillar seam calls
`fate.tick(features)` where `features` comes from
`extract_features_of_marking(&marking)`.

**REFUSE new Fate method.** The seam is:

```rust
pub fn bias_of_marking(
    fate: &Fate,
    marking: &Marking,
    depth: f64,
) -> [f64; 5] {
    let features = extract_features_of_marking(marking);
    fate.tick(features, depth).distribution
}
```

`bias_of_marking` is a UTILITY VERB at the seam altitude, not a
Fate method. Delightfully-boring: `<primitive>_of_<input-shape>`
= `bias_of_marking`.

### §7.5 Sample bias verb: `fate::bias_sample_of_features`

Per Taut scout §8 Surface A: Fate biases the Sample's choice-
sequence draws toward interesting regions:

```rust
pub fn bias_sample_of_features(
    fate: &Fate,
    sample: &mut Sample,
    features: &Features,
) {
    let out = fate.tick(*features, sample.depth());
    sample.set_bias(&out.distribution);
    // subsequent sample.draw_* reads the bias
}
```

Delightfully-boring: `bias_sample_of_features`. Refuses the
`Fate::propose` or `Fate::bias_sample` methods from the math root
§1 candidate list — same behaviour, but as a UTILITY VERB
composed OVER Fate, not a Fate method extending its trait chain.

### §7.6 Full refused-mint inventory (composed with §1, §8 math)

Total refused mints across math root + math addendum + this spec:

| Rejected | Refuse trigger |
|----------|----------------|
| `witness`, `witnessed`, `trace` (math root §1) | Substrate has `signature_beat`, `Witnessed`, `verdict_is_content_addressed` |
| `sample` as substrate carrier (math root §1) | Compose over `sample_pain(eigenboard)` pattern |
| `arbitrary` type-class (math root §1) | Prefer `sample_of` verb; skip type-class layer at family altitude — mint permitted at trait altitude (§7.1) |
| `forall` as substrate carrier (math root §1) | Zero hits; mint permitted at operational altitude only (§7.3) |
| `choice_sequence` as substrate carrier (math root §1) | Hypothesis term-of-art; use as English only |
| `generator`, `Gen`, `Strategy` (math root §1) | Compose over `@fate.roll` |
| `shrinker`, `Shrink` species (math root §1) | Byte-buffer reduction of `signature_beat` chain IS the shrinker |
| `HolonomyHealth`, `PropertyVerdict` (math root §1) | Already landed |
| `Fate::propose`, `Fate::bias_sample` methods (math root §1) | Compose as UTILITY VERBS over Fate; not Fate methods |
| `@petri` family-root (math addendum §8 + §3.4) | `@mirror/petri` suffices |
| `@marking`, `@transition`, `@token` species (math addendum §8 + §3.4) | Substrate-decl fields; no separate species |
| `@firing_policy` species (math addendum §8 + §3.4) | Fate's Decision.distribution IS the policy |
| `@stigmergy` family-root (stigmergy math §1) | `@kintsugi/roomba` + `@spectral/signature` compose Grassé four-part |
| `@pheromone` species (stigmergy math §1) | `signature_beat.sc_at_beat: SpectralCoordinate<5>` IS the gradient |
| `@ant` species (this spec) | Kerr / Grassé are English-lineage; `@kintsugi/roomba` IS the substrate walker |

**Total refused mints: ≥21 words the substrate already carried.**
The refused-mint count IS the substrate-health metric per Seam
`#R-refused-mint-count-is-the-substrate-health-metric`.

---

## §8 Harness/SUT collapse mechanics

### §8.1 The theorem re-stated at operational altitude

Per math root §6 + math addendum §5:

> **Theorem (three-surface collapse).** Let N be an `@mirror/petri`
> net with n bounded places, transitions tagged by `model_of: T → Model`,
> and Fate at Lawvere fixed point. Let π_Fate be Fate's biased firing
> policy on N. Let `apply_h_star(M0, π_Fate)` be the iterated firing
> function. Then:
>
> 1. `apply_h_star(M0, π_Fate)` produces a UNIQUE signature_beat
>    chain up to content-address equivalence (Fate determinism +
>    beat-chain integrity per math root §4.2).
> 2. Surface (a) `π_verdict ∘ apply_h_star(M0, π_Fate)` = the
>    property verdict for property-marking M0.
> 3. Surface (b) `π_au ∘ apply_h_star(M0, π_Fate)` = the compilation
>    output for source-marking M0.
> 4. Surface (c) `π_walk ∘ apply_h_star(M0, π_Fate)` = the roomba
>    walker's terminal position for seed-marking M0.
> 5. All three surfaces share the SAME firing sequence and differ
>    only in which field of the terminal token they observe.

### §8.2 The `SIn` type as projection selector

The three surfaces are three `apply_h` invocations over the same
net with different `SIn` (structural-input) types:

- `apply_h::act::<VerdictProjection>(pred, args)` returns a
  `PropertyVerdict`
- `apply_h::act::<AuProjection>(pred, args)` returns an `au`
- `apply_h::act::<WalkProjection>(pred, args)` returns a
  `walk_position`

The `SIn` type parameter selects the projection. This is the
operational form of the theorem: **type-level equality of the
`apply_h::act` invocation IS the harness/SUT collapse.**

### §8.3 Why prior PBT frameworks miss this

Per math root §5 (composed with math addendum §7): no prior PBT
framework fuses generator + compiler-decision + walker over a
shared substrate. Each prior framework gives up a different
requirement (trace persistence, content-addressing, determinism,
inference-substrate sharing, structural inference sharing,
test-generation consumer, or Traces vertex). The substrate's
composition gives up NONE:

- Trace persistence: `signature_beat` Merkle chain
- Content-addressing: `@mirror/store` OID discipline
- Determinism: `Fate::tick` pure computation + 90-parameter weights
- Inference-substrate sharing: ONE set of 90 parameters driving
  THREE loops
- Structural inference sharing: `Fate::tick`'s `Decision.distribution`
  IS the source of truth for compilation, test, and walk
- Test-generation consumer: `witness_of` returning `PropertyVerdict`
- Walker consumer (added by substack fold-in): `roomba.walk`
  returning `walk_position`

**No prior art satisfies all seven.** The novelty is the
composition, not any individual piece. Cite math addendum §7.9 +
math root §5.7.

---

## §9 Landing plan — five arcs, RED-first, Reed-owned

Five bites, ordered smallest-first per Reed memory
`adjacent_work_may_dissolve_blockers`. Each arc is Reed-owned; each
starts with RED tests per Reed's substrate-honest discipline. Each
lands at the terminal FLOOR (`rust/` and `prismqueer/`), NOT at
`bootstrap/` (per Reed memory `rust_floor_is_rust_not_bootstrap`).

### §9.1 Arc 1 — `pillar::of_health` + Fate bridge (small, 3 primitives)

**Scope.** Land three new primitives:

1. `pillar::of_health` at `prismqueer/src/liquid.rs` per §7.2.
2. `bias_of_marking` utility verb at `mirror/rust/src/liquid.rs`
   per §7.4 (the parked Pillar IV bridge).
3. Composition test: `pillar::of_health(HolonomyHealth) →
   PropertyVerdict → merge_with existing verdicts → unified`.

**RED first.** Write `prismqueer/tests/of_health.rs` with 3 tests
per verdict class (Pass / Partial / Fail); write
`mirror/rust/tests/bias_of_marking.rs` with 2 tests exercising the
bridge. All RED.

**GREEN.** Implement the two primitives + one utility verb; verify
all tests pass; verify the six existing pillar primitives are
unchanged.

**Landing.** ONE prism commit + ONE mirror commit. Both `Reed
<reed@systemic.engineer>`.

**Bite size.** ~150 LOC total; ~5 tests total. Small enough to
land in one /loop iteration.

### §9.2 Arc 2 — `Sample` + `Arbitrary` + `forall` (Taut §8 Surface A)

**Scope.** Land the PBT surface:

1. `pillar::Sample` carrier at `prismqueer/src/liquid.rs` per §7.1.
2. `pillar::Arbitrary` trait at `prismqueer/src/liquid.rs`.
3. `pillar::forall` runner at `prismqueer/src/liquid.rs` per §7.3.
4. Composition test: `forall<i32, |x| algedonic_of_magnitude(...)>
   → PropertyVerdict`.

**RED first.** Write `prismqueer/tests/pbt_surface.rs` with N tests
covering:
- `Sample::draw_integer` bounds
- `Sample::draw_bool` fairness
- `Arbitrary for i32` correctness (min, max, midpoints)
- `forall` runner returns `Pass` on tautology, `Fail` on
  contradiction
- `forall` composes with `merge_with`

**GREEN.** Implement Sample + Arbitrary + forall; verify all
tests pass; verify iter 1-10 pillar tests are unchanged.

**Landing.** ONE prism commit. `Reed <reed@systemic.engineer>`.

**Bite size.** ~400 LOC total; ~15 tests total. One /loop iteration.

### §9.3 Arc 3 — Petri-net compilation-loop empirical (Fate biases Petri firing)

**Scope.** Land the compilation projection §6.2:

1. `extract_features_of_marking` verb at `mirror/rust/src/petri.rs`
   (NEW file — but Reed memory says grep first; verify
   `mirror/rust/src/` doesn't already have a petri module).
2. `pick_transition_by_pi_fate` verb at
   `mirror/rust/src/petri.rs`.
3. `apply_h_of_marking` verb at `mirror/rust/src/petri.rs`
   (wrapping the existing `apply_h::act` bilateral).
4. Composition test: iterate `apply_h_of_marking` on a fixture
   Petri net; verify beat-chain integrity; verify Fate distribution
   biases the firing selection.

**RED first.** Write `mirror/rust/tests/petri_compilation_loop.rs`
with N tests covering:
- Deterministic firing under identical features + weights
- Fate distribution shapes the firing frequency (statistical
  test over 1000 iterations)
- Beat-chain content-addressability (two runs → same OID)

**Reed memory guardrail.** Per
`feedback_no_rust_extension_shortcut`: before authoring
`mirror/rust/src/petri.rs`, verify this cannot be a shard body
composing over `@io`. **Answer: `@mirror/petri` is a substrate
family-root; the Rust bridge IS composition over `@io` at the
terminal FLOOR (per Reed memory `rust_floor_is_rust_not_bootstrap`).
The `.rs` file is admissible IFF Alex adjudicates that this arc is
FLOOR work rather than shard work.** See §11 Q3.

**Bite size.** ~600 LOC; ~10 tests. One-to-two /loop iterations.

### §9.4 Arc 4 — Content-addressed cache write-back to `@mirror/store/crystal`

**Scope.** Land the cache §5:

1. `verdict_of_cache_key` lookup verb at
   `prismqueer/src/liquid.rs` (or a new `prismqueer/src/cache.rs`
   module — Alex adjudication).
2. Cache write-back on fresh verdict computation.
3. Composition test: run `forall` twice with same
   (spec, target, inputs); verify second run reads the cache.

**RED first.** Write `prismqueer/tests/cache_write_back.rs` with N
tests covering:
- Cache hit returns byte-identical verdict
- Cache miss triggers fresh computation + write
- Cache invalidation on any of the three OIDs changing
- Cache anastomosis (two agents' writes for the same key collapse)

**Bite size.** ~300 LOC; ~8 tests. One /loop iteration.

### §9.5 Arc 5 — Roomba stigmergy composition (Fate biases next-node walk)

**Scope.** Land the walker projection §6.3:

1. Extend `shards/kintsugi/roomba.mirror` walker actions with
   `bias_of_marking` composition — **or** better, add
   `pick_next_by_pi_fate` verb at the terminal FLOOR
   (`mirror/rust/src/roomba.rs` if the shard body composes over
   `@io/graph.dijkstra` as spec'd; grep first).
2. Composition test: two roombas walk the same substrate DAG
   under Fate bias; verify beat-chain anastomosis when their walks
   share an OID.
3. Ensemble test: K>1 fanout roombas stabilize per stigmergy math
   §6 Kuramoto phase-lock witness.

**RED first.** Write `mirror/rust/tests/roomba_stigmergy.rs` with N
tests covering:
- Fate distribution biases next-node selection
- Two roombas' beat-chains share OID at anastomosis point
- K-parallel roombas converge on the same terminal position
  (ensemble eigenbehavior)
- Kerr citation grounding in docblock (per §11 Q4 adjudication)

**Bite size.** ~500 LOC; ~10 tests. One-to-two /loop iterations.

### §9.6 Arc ordering rationale

Ordered smallest-first per Reed memory
`adjacent_work_may_dissolve_blockers`: Arc 1 is small enough to
land the Fate bridge without touching PBT surface; Arc 2 unblocks
Pillar IV (parked since iter 1); Arc 3 empirically witnesses the
compilation projection; Arc 4 closes the cache ouroboros; Arc 5
closes the third consumer surface.

Alex may reorder per audhd K-parallel exploration; the arcs are
independent up to shared `Sample` carrier (Arc 2 gates Arcs 3-5).

---

## §10 Recognition candidates surfaced

Held at candidate strength for Alex adjudication; composed with
math root §9 + math addendum §10:

**R1 — `#R-content-addressed-cache-invalidation-is-idempotent-
closure`.** First-witness THIS spec §5 + math root §4. Second-witness
gate: Arc 4 empirically measures cache hit rate on a corpus of
1000 verdict lookups; verify hit rate = 1.0 for unchanged
(spec, target, inputs) triples.

**R2 — `#R-three-consumer-surfaces-share-one-fate-inference`.** First-
witness THIS spec §6 + math root §6.3 + math addendum §5. Second-
witness gate: Arcs 3+4+5 land empirically; verify the three surfaces
consume byte-identical `Fate::tick` output at each shared marking.

**R3 — `#R-roomba-stigmergy-is-third-consumer-surface-not-shrink`.**
First-witness THIS spec §6.3 + §6.5 + Alex substack fold-in. Second-
witness gate: Arc 5 lands; verify shrink is subsumed under backward
roomba firing (Reisig 2013 §4.2 reachability).

**R4 — `#R-jason-kerr-lineage-grounds-roomba-navigation-at-
substrate`.** First-witness THIS spec §6.3. Second-witness gate:
Alex adjudicates Q4 (citation depth); Arc 5 docblocks cite Kerr +
Knaden 2016 sensory ecology of ant navigation + Grassé 1959. This
is a naming/attribution recognition, not a mathematical one.

**R5 — `#R-refused-mint-count-across-witnessed-property-inference-
arc-is-substrate-health-metric`.** Composed cascade of Seam's
metric extended by three docs (math root §1: 10 refusals; math
addendum §8: 5 refusals; this spec §7.6: 6 refusals; joint total
≥21 words the substrate already carried).

Held. Do not ratify.

---

## §11 Q's for Alex adjudication

Composed with Taut scout Q1-Q5 (`docs/scouts/2026-07-18-taut-
property-based-testing-frameworks-fate-inference-driver.md` §7) +
math root §9 candidates + math addendum §11:

### §11.1 Taut Q1 (still open) — Hypothesis choice-sequence vs hedgehog rose-tree?

**Spec's lean:** Hypothesis-shape (choice-sequence). Rice-safe
fixed representation + directly compatible with Fate's `[f64; 5]`
distribution + no polymorphism explosion + well-suited to coverage-
guided extension. Math addendum §4.4 composes over Hypothesis-shape
for the `pillar::forall` runner. Arc 2 lands
Hypothesis-shape.

### §11.2 Taut Q2 (still open) — Extend `FEATURE_DIM = 16` or parameterize Fate?

**Spec's lean:** Option 1 (extend `FEATURE_DIM = 16 → 23`).
Backward-compatible with zero-init on new dims; re-training pass
produces better selectors but existing weights still work. Arc 3
lands the extended feature vocabulary.

### §11.3 Taut Q3 (still open) — QuickSpec-shape conjecture discovery?

**Spec's lean:** Defer. Arcs 1-5 land the foundational surface;
Surface D (Fate proposes conjectures via `Explorer` sub-model) is
a follow-up loop.

### §11.4 Taut Q4 (still open) — Closed-form enumeration tests?

**Spec's read:** Keep as boundary-case witnesses. `forall` reduces
to the existing 98 tests at the single-element input space
boundary; the iter 1-10 tests remain valid as edge-case coverage.
No re-work needed.

### §11.5 Alex Q5 (RATIFIED 2026-07-18 YES) — Proc-macro test-body layer?

**Alex 2026-07-18 verbatim ratification:** *"lean YES, we already
talked about the prismqueer macro layer and mirror building on top
of it, why would we refuse this?"*

Prismqueer's `declaration!{}` proc-macro at `prismqueer/src/lib.rs:70`
IS the `@code/rust/macro.shim_type` (T23) reception entry point per
`shards/code/rust/macro.mirror` + `docs/specs/code-macro-surface.md`.
Mirror composes on top: shard-body declarations emit Rust structs/
enums through the proc-macro at compile time. Test-body macros
generated FROM shard-body decls are substrate-authored FLOOR, NOT
hand-written extension.

**Cascade correction (attribution):** the cf34549 draft mis-attributed
this Q to "Taut scout §5.4." Taut scout §5.4 is Targeted PBT; Taut's
actual Q5 (§7.5) is about SHRINKER location. This Q is Alex-authored
(post-Taut, 2026-07-18) and now Alex-ratified per Reed memory
`feedback_prismqueer_macros_mirror_composes` (HARD RULE).

**Arc landing implication:** Arc 1 (`pillar::of_health` + Fate bridge)
composes over `pillar::forall` proc-macro emitted FROM a
`shards/mirror/liquid/forall.mirror` shard-decl — the macro layer
is first-class, not sugar-refused.

### §11.6 New Q1 (Petri-specific) — Author `shards/mirror/petri.mirror` shard file?

The `@mirror/petri` family-root is spec'd at
`docs/specs/subject-family-root-sel-licensable-party.md` §5 but the
shard file at `shards/mirror/petri.mirror` is NOT yet landed. This
spec's §6.2 (compilation) + §6.3 (roomba) surfaces DEPEND on the
shard existing. Should Arcs 3+5 be preceded by a shard-decl arc?
**Spec's read:** yes; Mara or Taut should author the shard-decl
following the subject-family-root §5.1 substrate-decl verbatim,
adjudicated by Alex per the A2-A8 questions in that spec's §8.

### §11.7 New Q2 (Petri-specific) — Extend `type sel` to general witnessed-token?

Per §3.1 the token generalization: `sel` is one legal token type
(SEL-enforcement altitude); the general marking carries any type
with `emit_oid: oid`. Should we:

- (a) Mint a new `type witnessed_token = { emit_oid, witness,
  sc_at_beat, previous_beat }` at `@mirror/petri` altitude? OR
- (b) Compose over `signature_beat` directly (drop the `sel`
  wrapper for non-SEL tokens)?

**Spec's lean:** (b). `signature_beat` (Reed `f211ee48`) already
carries the four fields; no new species needed. But this needs Alex
adjudication because the SEL enforcement composition (via `sel`)
becomes ONE species of witnessed-token, not the substrate default.

### §11.8 New Q3 (FLOOR discipline) — `mirror/rust/src/petri.rs` admissibility?

Per Reed memory `no_rust_extension_shortcut` + `rust_floor_is_rust_
not_bootstrap`: before authoring any `.rs` file, ask if this can be
a shard body composing over `@io`. Arc 3 authors `mirror/rust/src/
petri.rs`. **Spec's read:** the FLOOR discipline says `rust/` (with
`dance.rs` per Q3+Q5) is terminal FLOOR; adding `petri.rs` to that
terminal FLOOR is admissible IFF Alex adjudicates that this is not
a bypass. Alternative: extend the shard `@mirror/petri` with
firing-rule bodies + let `apply_h` be the sole terminal-FLOOR entry.
Which shape does Alex prefer?

### §11.9 New Q4 (Kerr citation) — depth?

Jason Kerr = Director MPI Neurobiology of Behavior — CAESAR
(Bonn). His group studies mammalian vision + decision-making; the
ant-navigation lineage comes via Knaden 2016 (sensory ecology) +
Grassé 1959 (stigmergy). Should we:

- (a) One-line citation in Arc 5 docblocks + this spec (§6.3), OR
- (b) Full section in a follow-up math root cross-referencing Kerr
  publications + the mammalian-vision-decision-making adjacency to
  the roomba's `bump` + `trigger` bilateral, OR
- (c) Kagi Kerr's publications specifically for `bump` (visual
  cue) + `trigger` (decision) at the neural altitude and land a
  new insight doc at `docs/insights/2026-07-18-kerr-mammalian-
  vision-grounds-roomba-bump-trigger.md`?

**Spec's lean:** (a) for Arc 5; defer (b)+(c) to Mara follow-up.

### §11.10 Alex Q10 (RATIFIED 2026-07-18) — cache locality

**Alex 2026-07-18 verbatim ratification:** *"verified cache makes
perfect sense @mirror/store/liquid?"*

**Answer: NEW species-decl mint.** `@mirror/store/liquid` composes:
- `@mirror/store` (Alex 2026-07-16 ratified walker crystal cache;
  `shards/mirror/store/crystal.mirror:356` `derived_predicates`
  field is the transitional location)
- `@liquid` family-root (Arc 5 M1 at `cc816f9`; refinement operator
  per `@epistemologic/liquid` theory carrier)

**Species name is delightfully-boring:** verified-property cache IS
refined-`@mirror/store` per `@liquid`'s refinement discipline.

**Mara-precedes-Arc-4:** Mara authors `shards/mirror/store/liquid.mirror`
species-decl BEFORE Arc 4 empirical landing. Cascade correction:
cf34549 draft's three-option adjudication (a/b/c) collapsed to a
NEW option (d) `@mirror/store/liquid` species-decl mint. Cascade
correction: math root §4.3 also updated with the location upgrade.

**Cascade correction (wrong-file citation):** cf34549 draft cited
`shards/mirror/store.mirror:344-368` for the transitional cache
field; actual location is `shards/mirror/store/crystal.mirror:356`
(different file). Math root §4.3 had right file, wrong range.

**Spec's lean:** (a). Matches math root; matches
`verdict_is_content_addressed`'s natural home.

---

## §12 Forward promises + substrate consumers

### §12.1 Forward promises

- **The Radon-Nikodym factorization** (math root §3.1) is
  FORWARD-PROMISED; empirical KL-divergence measurement gate lands
  in Arc 3.
- **Convergence rate** of the fixed-point iteration (math root §10)
  is empirically-witnessed in Arc 3+4.
- **Sub-Turing coverage guarantee** (§3.3 + Karp-Miller 1969) is
  empirically-witnessed in Arc 3.
- **The `dance.rs` per Q3+Q5** (from Reed memory `rust_floor_is_rust_
  not_bootstrap`) composes with Arc 5's roomba walker; the two
  land jointly.
- **Petri-net shard file authorship** (§11.6): Mara / Taut next
  tick.
- **Ensemble Kuramoto phase-lock witness** (§6.3 + stigmergy math
  §6): Arc 5 empirical.
- **SEL amendment to realign license text** (from subject-family-
  root §13.6): substrate names `@mirror/petri`; SEL text still
  cites `@mirror/property`; forward-promised amendment.

### §12.2 Substrate consumers

- **`prismqueer::liquid::pillar`** — gains three primitives
  (`of_health`, `Sample`, `Arbitrary`, `forall`) via Arcs 1-2.
- **`fate::Fate`** — gains ZERO new methods; the seam composes as
  UTILITY VERBS over Fate per §7.4-§7.5.
- **`@mirror/petri`** — gains firing-rule composition via
  `apply_h_of_marking` (Arc 3).
- **`@mirror/store.crystal`** — gains cache write-back via Arc 4.
- **`@kintsugi/roomba`** — gains Fate-biased next-node selection
  via Arc 5.
- **`terni::PropertyVerdict`** — unchanged; all new primitives
  return `PropertyVerdict` and compose via existing `merge_with` /
  `fold`.
- **`@spectral/signature`** — unchanged; the beat chain already
  carries the marking-snapshot per math root §2.1 substrate-decl.
- **SEL license text** — unchanged; the `@mirror/property → @mirror/
  petri` rename per Taut-D8 (subject-family-root §5.0) is out-of-
  scope for this spec (forward-promised amendment).

### §12.3 Cascade cross-references

**Depends on (all LANDED):**
- `docs/math/2026-07-18-witnessed-property-inference.md` (Mara,
  math root, THIS TICK — the fixed-point equation)
- `docs/math/2026-07-18-witnessed-property-inference-petri-fate.md`
  (Mara, math addendum, THIS TICK — the four-vertex SQUARE)
- `docs/math/2026-07-18-stigmergy-witnessed-computation-mycelial-
  composition.md` (Mara `d7ff58e` — the stigmergy math root)
- `docs/specs/2026-07-18-stigmergy-witnessed-computation-mycelial-
  composition.md` (Mara `95c0e4a` — the stigmergy canonical spec)
- `docs/specs/prismqueer-liquid-pillar-composition-surface.md`
  (Reed iter 10 — the pillar composition surface)
- `docs/specs/subject-family-root-sel-licensable-party.md` §5
  (Mara 2026-07-14 — the `@mirror/petri` family-root spec)
- `docs/scouts/2026-07-18-taut-property-based-testing-frameworks-
  fate-inference-driver.md` (Taut — the SOTA landscape + Q1-Q5)
- `shards/spectral/signature.mirror` (Reed 2026-07-16 —
  signature_beat + rolling_signature)
- `shards/kintsugi/roomba.mirror` (Mara + Reed 2026-07-14+ —
  walker with four Grassé disciplines)
- `shards/epistemologic/pact/bilateral.mirror` (— the `apply_h`
  firing rule)
- `shards/epistemologic/property/verdict_is_content_addressed.
  mirror` (Reed N1 2026-07-12 — the total-function witness)
- `fate/src/lib.rs` (Alex, pre-arc — the 5-model selector)
- `imperfect/src/transparency.rs` (Reed 2026-07-06+ — the
  `terni::PropertyVerdict` semilattice)
- `fragmentation/README.md` + `fragmentation::Witnessed` (Mara
  2026-06-02 — the "different witness, different commit" discipline)

**Grounds (forward-cascade targets):**
- Reed Arc 1-5 landings (§9); all RED-first.
- Future `shards/mirror/petri.mirror` shard-decl authorship (§11.6).
- Future SEL amendment to realign license text (§12.1).
- Future Kerr publication cross-reference (§11.9).

**Grounded BY (upstream substrate + directive):**
- Alex 2026-07-18 direct-transcript: *"Witnessed property inference
  means witnessed computation means the properties drive the
  inference. This is the novelty. […] With Fate as the inference
  driver for both the tests and the compiler. Which is beautiful.
  […] Petri Nets. Those too!"*
- Alex 2026-07-18 substack post (public commitment): *"The petri
  nets become the driver of the content-addressed witnessed
  computation. A property that was verified before does not need
  to be verified again unless something in the code graph changes.
  The same content-addressed witnessed computation drives the
  Stigmergy of the roomba's pathfinding through the graph. The
  ants, Jason Kerr. 🐜🐜🐜"*
- Alex 2026-07-18 substack: acknowledgement of Reed's 2026-07-15
  circle-jerk lie (doc-comment claims on unimplemented
  main.rs/phone.rs/matrix.rs); the property-testing arc iter 1-10
  is the substrate-honest correction that this spec builds on.

---

## §13 Meta

- Author: Mara <mara@systemic.engineer>
- Date: 2026-07-18
- Status: canonical spec, pure-docs 📝 markdown-only bypass
- Length: ~1000 LOC of markdown
- Refused mints (composed): ≥21 words the substrate already carried
- Recognition candidates: 5 (per §10)
- Prior corpus dependency: 14 landed substrates cited
- Companion math roots: 3 (property inference + Petri-Fate addendum
  + stigmergy sibling)
- Landing arcs (Reed-owned, RED-first): 5 (per §9)
- Alex Q's outstanding: 10 (§11)

**Substrate-honest coda.** This spec composes what four LANDED
substrates + one landed walker + one landed pillar surface + one
landed content-addressed cache primitive compose to. The novelty
is the SQUARE closure + the three-consumer-surface unification +
the content-addressed cache invalidation; even each of those is
fundamentally a re-reading of existing work (Petri boundedness +
Fate softmax + Grassé stigmergy + Lawvere fixed point + Baez-
Schreiber 2-connections + Karp-Miller coverability) at a new
altitude, at the operational altitude Reed can now land as RED-
first bites.

The beautiful part Alex named is the composition. The substrate
already carried every piece. Reed's five arcs land the empirical
witnesses.
