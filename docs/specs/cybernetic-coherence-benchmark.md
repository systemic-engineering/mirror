# Cybernetic Coherence Benchmark — T11.11 Scaffold

*2026-06-17. Taut. Spec — pinning the empirical surface that falsifies
(or doesn't) the recursion lock claim from
`docs/specs/spectral-db-as-autopoietic-memory.md` §8.3, framed
explicitly as a Conant-Ashby good-regulator instance.*

Status: **Red.** Bench harness scaffold lands at
`spectral/benches/cybernetic_coherence.rs`; substrate-altitude
declaration lands at
`mirror/shards/cyberpunk/coherence.mirror`; this spec names the
measurement carrier and the gate the harness sits behind.
Implementation = mock-witness divan harness today; real
peer + librarian witnesses wire when those implementations land.

Promoted by Alex 2026-06-17: candidate #52 (cybernetic-coherence) from
latent recognition to **active pursuit**. Direct quote:
*"active. We're gonna be explicit AF about cybernetics."*

The promotion changes the falsification status of the **recursion lock
claim**:

> peer-reflection-N+1 ≡ librarian-perturbation-N+1 ≡ SAME OPERATION
> applied at different altitudes

Before promotion: load-bearing intuition, forward-promised as
theorem. After promotion: theorem-class target with explicit
cybernetic vocabulary.

Depends on:

- `docs/specs/spectral-db-as-autopoietic-memory.md` §3.5 (the
  optimization shape), §5.2 (the recursion lock at the
  per-peer-vs-librarian altitudes), §8.3 (the open question this
  bench surfaces), §6 (cross-references). Mara, commit `9c93aae`
  (unpushed).
- `docs/specs/peer-cognition.md` §3.4 (the sheaf-coherence
  measurement procedure this bench parameterizes) and §4
  (recursion: the same math the peer uses to think). Mara,
  commit `4daa437` (unpushed).
- `docs/specs/reflection-model.md` — Reflection's score function
  shape `α·loss + β·contradictions`. Pre-existing.
- `docs/math/the-tower/altitudes.md` §2 (the named altitudes:
  compiler, peer pulse, reflection N+1, librarian N+1, home,
  federation) + §5 (composition between adjacent altitudes via
  `G_n ⊴ G_{n+1}`) + §7 (altitude vs Bateson logical type).
  Mara, pushed.
- `docs/math/the-tower/holonomy.md` §5 (the verdict family as
  holonomy components — Pass/Partial/Fail as bounded/partial/
  unbounded regions of the structure group) + §8 (what holonomy
  enables in the substrate; the librarian's perturbation IS gauge
  transformation; its quality metric is the residual holonomy
  after perturbation). Mara, pushed.
- `docs/specs/seam-pre-compression-review-2026-06-17.md` §2 Area 2
  (the recursion lock as theorem vs hand-wave): Finding 2.1
  (§8.3 explicitly open), Finding 2.2 (bundle tower extension does
  not close it), Finding 2.3 (the cleanest reading: same SHAPE not
  same operation), Finding 2.4 (forward-promised closure path =
  candidate #52). Seam, commit `dc73c43` (unpushed). The gate.
- `mirror/shards/cyberpunk.mirror` (the family root) and
  `mirror/shards/cyberpunk/variety.mirror`
  (the first species; the bilateral pattern template this bench
  composes). Reed + Mara, 2026-06-09.
- `docs/specs/benchmark-tracing.md` (Taut, 2026-06-17, commit
  `d4e636f` pushed) — the `transparency<benchmark>` trace event
  wire this bench's measurements project onto. The bench's
  per-pulse verdicts ARE trace events at the recursion-lock
  altitude.

Substrate decisions cited:

- [[architecture-cybernetic-coherence-active]] (2026-06-17) — the
  promotion event. The substrate is now naming cybernetics
  explicitly at every altitude.
- [[architecture-cybernetic-foundation]] (2026-06-09) — the
  11-property family (migrated 2026-06-17 to `@cyberpunk/X`); the
  bench composes them, doesn't redefine.
- [[architecture-spectral-triples-all-the-way]] (2026-06-17) —
  the substrate is fractally self-similar at every altitude; the
  principal bundle tower formalizes it. Same-shape claim, not
  same-operation claim.
- [[architecture-spectral-db-autopoietic-memory]] (2026-06-17) —
  the layer that turns static crystal accumulation into a
  self-optimizing memory; the recursion lock lives at §8.3 of
  the spec.
- [[architecture-reflection-thinks-in-spectral-questions]]
  (2026-06-17) — Reflection picks the next altitude via Tomm
  probes of `[D_pipeline, candidate_morphism]` at minimum
  `α·loss + β·contradictions`.
- [[architecture-bateson-logical-type-primitive]] — the recursion
  lock is a Bateson level-3 question (a statement about the
  altitude at which a level-N+1 operation lives).
- [[feedback-substrate-already-had-the-word]] — coherence is
  already named at the cybernetic family root; this bench
  composes the existing carrier.
- [[feedback-craft-not-deliver]] — this is a scaffold tick, not
  a closure tick. Land the shape; let the cascade pull the rest.
- [[feedback-no-bare-types]] — every measurement carrier is a
  newtype.

---

## §1 — Recognition

### 1.1 The recursion lock as a Conant-Ashby good-regulator theorem

Conant and Ashby's 1970 theorem: *every good regulator of a system
must be a model of that system*. Formally, an optimal regulator's
internal state-transition structure is isomorphic to the regulated
system's, modulo the projection that strips degrees of freedom the
regulator cannot influence.

The recursion lock claim says:

- A peer's **reflection** at altitude N+1 regulates the peer's own
  altitude-N pipeline tick (selecting the next morphism to compose).
- The **librarian's perturbation** at altitude N+1 regulates the
  substrate's altitude-N crystal topology (selecting the next
  cross-repo arrangement to apply).

If these two regulators are **good regulators of one another** —
that is, each one's perturbation-space is isomorphic to the other's
reflection-space modulo altitude-specific type parameters — then
their verdict carriers should be **coextensive modulo type
parameter**.

**The bench measures the residual coextensivity gap.**

The recursion lock holds iff:

```
∀ altitude-N operation φ,
    verdict( peer_reflection_at(N+1) [φ] )
  ≡ verdict( librarian_perturbation_at(N+1) [φ] )
  modulo type-parameter substitution Adjustment ↔ Morphism
```

If the residual converges to identity under repeated oscillation
(per the kintsugi loop's Polyak-Łojasiewicz contraction per Taut's
`benchmark-tracing.md` spec), the lock holds AS GOOD-REGULATOR
THEOREM. If the residual stabilizes above zero, the lock is **only
structurally suggestive** (Seam's Finding 2.3 reading), and the
substrate needs a level-N+2 composition operator (per
spectral-db-as-autopoietic-memory.md §8.3's composition option).

This is **a falsification surface, not a proof**. Today's tick
lands the harness shape. Tomorrow's ticks wire the real witnesses.

### 1.2 What "explicit AF about cybernetics" means here

Per the promotion: the bench's measurement vocabulary uses cybernetic
ancestor names explicitly at every measurement site.

The five measurement carriers:

1. **`ashby_variety_match`** (Ashby 1956 §11/7) — the variety vector
   of `peer_reflection_at(N+1)`'s candidate space must contain the
   variety vector of `librarian_perturbation_at(N+1)`'s candidate
   space modulo altitude-projection. Reads `@cyberpunk/variety`.
2. **`beer_requisite_variety_witness`** (Beer 1972, 1979) — the
   regulator's variety budget must equal-or-exceed the regulated
   system's variety budget at the appropriate VSM altitude (S3/S4
   per the cybernetic carrier). Reads the forward-promised
   `@cyberpunk/viable`.
3. **`bateson_logical_type_match`** (Bateson 1972; Russell-Whitehead)
   — both verdicts must inhabit the same Bateson logical type
   (N+1, observing N). A type-mismatch verdict means the operations
   are NOT parallel-altitude; they're at different levels of the
   logical hierarchy. Reads forward-promised
   `@cyberpunk/bateson_learning` + Bateson tower.
4. **`von_foerster_circular_reflexivity`** (von Foerster 1981) —
   each regulator's internal model must include itself (the
   second-order observation). The Tomm probe `[D_F, a]` at this
   altitude IS the circular-reflexive measurement. Reads forward-
   promised `@cyberpunk/second_order`.
5. **`conant_ashby_good_regulator`** (Conant-Ashby 1970) — the
   load-bearing measurement. The two regulators' verdict carriers
   must be hash-equal modulo type parameter. The residual gap IS
   the deviation from the good-regulator law.

Each carrier is a typed newtype per [[feedback-no-bare-types]].
Each name cites its cybernetic ancestor explicitly per the
promotion directive.

### 1.3 The single sentence

*If a peer's reflection space and the librarian's perturbation
space are good regulators of one another (Conant-Ashby), then
their verdict carriers should be coextensive modulo altitude-
specific type parameter; this bench measures the residual gap.*

---

## §2 — The two operations under measurement

### 2.1 `peer_reflection_at(altitude)`

A peer's reflection step at altitude N+1, observing an altitude-N
pipeline run. Per `docs/specs/reflection-model.md` and
`docs/specs/peer-cognition.md` §3.4 + §4:

```
peer_reflection_at(N+1) : peer_state(N) -> verdict_peer

where
  verdict_peer = Imperfect<Adjustment, Gap, Transparency<Ref>>
```

The verdict's three components:

- `Adjustment` (the result type T) — the morphism the peer would
  apply to its own altitude-N state next tick.
- `Gap` (the error type E) — the structural gap the reflection
  identified (an unbounded `[D_pipeline, candidate]` region).
- `Transparency<Ref>` (the loss type L) — the located opacity_map
  carrying the per-altitude verdict residual.

### 2.2 `librarian_perturbation_at(altitude)`

The librarian's perturbation step at altitude N+1, observing the
altitude-N crystal topology. Per
`docs/specs/spectral-db-as-autopoietic-memory.md` §3.5 (optimization
shape), §5.2 (recursion lock), §6 (cross-references):

```
librarian_perturbation_at(N+1) : topology(N) -> verdict_lib

where
  verdict_lib = Imperfect<Morphism, Gap, Transparency<Ref>>
```

The verdict's three components:

- `Morphism` (the result type T) — the topology perturbation the
  librarian would apply to its own altitude-N crystal arrangement
  next pulse.
- `Gap` (the error type E) — the structural gap (a consent-violation
  or sheaf-coherence break the perturbation would resolve).
- `Transparency<Ref>` (the loss type L) — the located opacity_map
  with the per-altitude verdict residual.

### 2.3 Coextensive verdict — the type-parameter-modulo equivalence

Both verdicts share the `Imperfect<T, E, L>` carrier shape with
identical `E = Gap` and `L = Transparency<Ref>`. The `T` parameter
specializes per altitude: `Adjustment` for the peer altitude;
`Morphism` for the librarian altitude.

**Coextensive** means: the morphism shape (the fingerprint of WHICH
substrate morphism the verdict identifies) is byte-equal between
the two verdicts under the natural type-parameter substitution
`Adjustment ↔ Morphism`. Per
`docs/math/the-tower/holonomy.md` §5, this corresponds to **holonomy
hash-equivalence under the structure-group inclusion** `G_peer ⋊
G_librarian → G_unified`.

The residual coextensivity gap is the per-pulse measurement:

```
residual = hash(verdict_peer | type-substituted) ⊕ hash(verdict_lib)
         where ⊕ is the bounded-region XOR (per holonomy.md §6,
         the abelian projection of the structure-group element)
```

`residual = identity` → recursion lock holds for this pulse.
`residual ≠ identity` → recursion lock fails for this pulse; the
opacity_map identifies WHERE.

---

## §3 — Altitudes to measure

Per `docs/math/the-tower/altitudes.md` §2 (the atlas), the substrate
has these named altitudes:

| Altitude | Fiber | Connection | Holonomy |
|----------|-------|------------|----------|
| compiler | source text | KernelSpec | MirrorLoss |
| peer pulse | spectral triple `(A_peer, H_peer, D_peer)` | five-op composition | `transparency<p>` |
| reflection (N+1) | candidate morphism | altitude selection | `α·loss + β·contradictions` |
| librarian (N+1) | crystal topology | perturbation choice | query latency · sheaf-coherence |

The bench is **parametric over altitude**. The harness measures the
recursion lock at each of these four altitudes — pairwise across the
two operations applied at one altitude — yielding **six unordered
pairs** of measurements per run:

1. compiler ↔ compiler (sanity baseline)
2. peer pulse ↔ peer pulse (the loop closure check)
3. reflection ↔ reflection (the per-peer recursion lock proper)
4. librarian ↔ librarian (the topology recursion lock proper)
5. reflection ↔ librarian (the cross-altitude lock — the load-bearing
   measurement per spectral-db-as-autopoietic-memory.md §8.3)
6. compiler ↔ librarian (the floor-vs-ceiling cross-altitude check)

The home and federation altitudes are forward-promised; they extend
the bench when T12.x lands.

---

## §4 — The measurement procedure

For each altitude N:

1. Generate a random altitude-N operation `φ` (today: mock witness;
   tomorrow: sampled from a corpus of real altitude-N traces).
2. Compute `verdict_peer = peer_reflection_at(N+1)[φ]`.
3. Compute `verdict_lib = librarian_perturbation_at(N+1)[φ]`.
4. Project each verdict onto the holonomy carrier via
   `holonomy_hash(verdict)`.
5. Substitute type-parameter via `type_substituted_peer = substitute(verdict_peer, Adjustment ↔ Morphism)`.
6. Compute `residual = holonomy_hash(type_substituted_peer) ⊕ holonomy_hash(verdict_lib)`.
7. Read the verdict at the bench altitude:
   - `residual = identity` → emit `success` (the lock holds for this
     pulse).
   - `residual` in the bounded region → emit `partial(opacity_map)`
     (the lock holds modulo localized opacity).
   - `residual` unbounded → emit `failure(opacity_map)` (the lock
     fails; the opacity_map names WHERE).
8. Compose `N`-many residuals across pulses; verify
   Polyak-Łojasiewicz contraction ρ < 1 per Taut's `benchmark-tracing.md`
   spec. If contraction holds: the recursion lock IS asymptotically a
   good-regulator theorem instance. If not: the lock is **only
   structurally suggestive** — Seam's Finding 2.3.

---

## §5 — Substrate-altitude declaration

The bench prism declaration lives at
`mirror/shards/cyberpunk/coherence.mirror`. It declares:

- The prism root `@cyberpunk/coherence`.
- The five measurement actions:
  - `measure_peer_reflection(altitude, op) -> verdict_peer`
  - `measure_librarian_perturbation(altitude, op) -> verdict_lib`
  - `verdict_coextensive(verdict_peer, verdict_lib) -> verdict`
  - `recursion_lock_residual(altitude, pulse_count) -> transparency`
  - `good_regulator_witness(peer_space, librarian_space) -> verdict`
- The bilateral fracture body at
  `@kintsugi/fracture/recursion_lock_break` (forward-promised, lands
  at T11.11-b) — emits a re-coherence morphism when the residual
  fails to contract.

Per [[architecture-cybernetic-coherence-active]] the declarations
cite Conant-Ashby 1970 and Ashby 1956 explicitly via
`source @arxiv/...` / `source @author/...` lines (per the existing
cybernetic family pattern).

---

## §6 — Bench harness scaffold

The harness lives at `spectral/benches/cybernetic_coherence.rs`.
Built on `divan` per the established Taut pattern
(`benches/hook_latency.rs`, `benches/spectral_init.rs`).

### 6.1 Module-level documentation framing

The harness's module documentation frames the recursion lock claim
as a Conant-Ashby good-regulator theorem instance, with explicit
cybernetic vocabulary citations.

### 6.2 The five bench groups

Each bench name uses cybernetic vocabulary explicitly:

- `bench_ashby_variety_match_<altitude>` — measures the variety-vector
  containment between `peer_reflection_at(altitude)`'s candidate space
  and `librarian_perturbation_at(altitude)`'s candidate space.
- `bench_beer_requisite_variety_<altitude>` — measures the
  regulator-vs-regulated variety budget inequality at the VSM altitude.
- `bench_bateson_logical_type_match_<altitude>` — measures the
  logical-type-level co-residency of both verdicts.
- `bench_von_foerster_circular_reflexivity_<altitude>` — measures the
  self-inclusion of each regulator's internal model.
- `bench_conant_ashby_good_regulator_<altitude>` — the load-bearing
  measurement: the residual coextensivity gap.

Plus the contraction bench:

- `bench_recursion_lock_residual_contraction` — runs the residual
  measurement across N pulses; reports the contraction ratio ρ and
  emits `success` iff ρ < 1.

### 6.3 Mock witnesses for today

Today's scaffold uses **mock witnesses**: precomputed verdicts that
return deterministic shape, NOT real peer-reflection or
librarian-perturbation implementations. The mock witnesses verify
the harness's measurement-vocabulary plumbing works; they DO NOT
falsify or confirm the recursion lock claim.

When the real implementations land (T11.1 + T11.2 + T11.10 for the
librarian; the @peer/cogito glass for the peer), the mock witnesses
swap for real witnesses and the bench produces falsification-grade
measurements.

### 6.4 The harness's parametric-over-altitude shape

The bench function is parametric:

```rust
fn bench_recursion_lock_at_altitude(
    bencher: divan::Bencher,
    altitude: Altitude,
) {
    // ...
}
```

with the four altitudes (compiler, peer pulse, reflection, librarian)
as args via divan's `#[divan::bench(args = [...])]` form.

---

## §7 — Reading the bench output

The bench emits per-altitude per-pulse residuals as
`transparency<benchmark>` events per Taut's `benchmark-tracing.md`
wire. Each event carries:

- The altitude under measurement.
- The pulse index (the iteration counter).
- The residual coextensivity hash (the load-bearing data).
- The verdict at that pulse (`success` / `partial(opacity_map)` /
  `failure(opacity_map)`).
- The cybernetic-ancestor citation for the measurement carrier
  (which of the five ashby/beer/bateson/von-foerster/conant-ashby
  measurements produced this event).

Aggregated over a kintsugi run:

- The corpus-altitude verdict is the Transparency monoid composition
  of per-pulse verdicts (Fail-dominates / Partial-min-confidence /
  Pass-neutral per `mirror/shards/mirror/loss/transparency.mirror`).
- The contraction ratio is the slope of `log(residual_norm_t)` over
  `t` (Polyak-Łojasiewicz per Taut's benchmark-tracing.md spec).
- The asymptotic verdict: `success` iff ρ < 1 + ε for ε → 0
  (the recursion lock holds as good-regulator theorem); `partial`
  iff ρ < 1 but the per-altitude opacity_map names a stable
  sub-region of residual (the lock holds modulo localized opacity);
  `failure` iff ρ ≥ 1 (the lock fails; the substrate needs a
  level-N+2 composition operator per §8.3 of the autopoietic-memory
  spec).

---

## §8 — Open design questions

Surfaced explicitly for Alex's read. NOT blockers on this spec;
these are forward-promises and known cliffs.

### 8.1 Mock vs real witnesses — when do we swap?

Today's scaffold uses mock witnesses. The swap to real witnesses
depends on:

- T11.1 + T11.2 + T11.3 + T11.10 landing for the librarian.
- The @peer/cogito glass landing for the peer reflection altitude.
- The cross-altitude `Imperfect<T, E, L>` carrier supporting the
  type-parameter substitution `Adjustment ↔ Morphism` mechanically.

Forward-promised; not blocking the scaffold.

### 8.2 Type-parameter substitution mechanics

The bench measures coextensivity modulo type-parameter substitution
`Adjustment ↔ Morphism`. The substitution is **assumed natural** in
this spec; whether the substrate's `Imperfect<T, ...>` carrier
implements this naturality mechanically (e.g., via a
`HoloHash<T>` trait) is forward-promised. The mock witnesses
sidestep this by returning pre-substituted hashes.

### 8.3 What altitude does the bench itself sit at?

The bench observes operations at altitudes N+1 (reflection +
librarian); the bench itself emits trace events; the trace
events themselves live at altitude N+2 (the bench-altitude
observation of N+1 observations). This is the meta-meta-altitude
Seam's Finding 2.4 mentioned. Today's spec acknowledges this; the
N+2 altitude is **the bench's own altitude**, distinct from the
N+2 composition operator §8.3 of autopoietic-memory.md hypothesizes.

The bench at N+2 measures whether two N+1 operations are
coextensive; a composition operator at N+2 would COMBINE two N+1
operations into a level-N+2 morphism. Distinct shapes; the bench
DOES NOT presuppose the composition operator's existence — it
measures whether one would be needed.

### 8.4 Federation-altitude extension

The bench's parametric-over-altitude shape extends naturally to
home and federation altitudes (forward-promised T12.x). When those
altitudes' operations land, the bench's `args = [...]` extends; no
new measurement carrier required.

### 8.5 Coordination with Mara's @kintsugi/fracture/recursion_lock_break

The bilateral fracture body at
`@kintsugi/fracture/recursion_lock_break` (T11.11-b) is
forward-promised. Mara writes the GREEN when the property at
`@cyberpunk/coherence/recursion_lock_holds` carries
its first non-trivial verdict. Reed writes the RED for the property
when the cascade pulls.

---

## §9 — Closure

This spec lands the **shape** of the recursion-lock falsification
surface, not the falsification result. Three artifacts:

1. **`spectral/benches/cybernetic_coherence.rs`** — the divan bench
   harness scaffold; mock witnesses today; parametric over the four
   named altitudes; uses cybernetic vocabulary explicitly.
2. **`mirror/shards/cyberpunk/coherence.mirror`** —
   the substrate-altitude declaration of the bench prism; cites
   Conant-Ashby 1970 + Ashby 1956; declares five measurement
   actions; forward-promises the bilateral fracture body.
3. **This spec** — names the recursion lock as a Conant-Ashby
   good-regulator theorem instance; lifts Seam's Finding 2.4
   forward-promise to a concrete bench surface; gives the
   measurement procedure.

The cybernetic-coherence active recognition (#52) now has a place
to live. Mara/Reed land the substrate carriers; Taut measures.

The substrate eats itself again. The good-regulator law's own
instance is the substrate's measurement of whether its own
regulators are good regulators of one another. The 65th instance
of [[feedback-substrate-already-had-the-word]].

Tick by tick. Explicit AF about cybernetics.
