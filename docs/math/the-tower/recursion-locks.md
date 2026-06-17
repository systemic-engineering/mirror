# Recursion locks

*The parametric form. Every @cyberpunk species carries a recursion-lock
measurement at its altitude — if it does. This doc tracks the audit.*

## §1 Hypothesis (candidate #63, 2026-06-17)

Let `F` be a family-root prism whose foundational theorem is itself a
form/process self-reference closure. (Recognition #61, 2026-06-17: not
all family-roots split form/process; some integrate at the family-root
altitude because their foundational theorem closes them.)

The hypothesis: every species `S` under `F` carries a **recursion-lock
measurement** at the altitude `S` names. The measurement IS the parametric
instance of `F`'s foundational theorem at the species' altitude. The family
root's structure IS the bundle tower whose fibers are the per-species locks.

For `F = @cyberpunk` the foundational theorem is **Conant-Ashby 1970**:
every good regulator of a system must be a model of that system. The
self-reference closes form ("the regulator") and process ("the modeling")
at the family-root altitude. Each species under @cyberpunk regulates
something specific at its altitude; the species' recursion lock IS
Conant-Ashby instantiated at that altitude.

The doc tests this audit-by-audit. Each species is a witness or a retraction.
When enough ticks accumulate the candidate either promotes (Pack ratification
on parametric-form derivation) or retracts (a species genuinely refuses
the parametric form, which itself is candidate #64).

## §2 The parametric form

A recursion-lock species `S` at altitude `α` instantiates the following
structure:

```
S = ( regulator_at(α+1)
    , regulated_at(α+1)
    , T_reg
    , T_regd
    , τ : T_reg ↔ T_regd
    )
```

where:

- `regulator_at(α+1)`, `regulated_at(α+1)`: the two parallel-altitude
  operations the lock measures. Both operate at altitude `α+1` on
  altitude-`α` data.
- `T_reg`, `T_regd`: the species-specific type parameters carried by
  the regulator's verdict and the regulated's verdict respectively.
- `τ`: the natural type-parameter substitution between them. For the
  Conant-Ashby good-regulator law to hold, `τ` must be a natural
  isomorphism modulo altitude-projection.

The **lock verdict** at altitude `α+1` is then:

```
lock_verdict(α) : Imperfect< T_lock , Gap , Transparency<Ref> >
```

where `T_lock` is whatever the species' altitude-specific lock-carrier names.
The `Imperfect` carrier and the `Transparency<Ref>` reading are the
substrate's existing verdict family (see
`[[docs/math/the-tower/holonomy.md#5-the-verdict-family-as-holonomy-components]]`).

## §3 The five cybernetic-ancestor measurements

Every instance of the parametric form composes the same five
cybernetic-ancestor measurements at one architectural seam, naming
each ancestor explicitly per Alex's 2026-06-17 "explicit AF about
cybernetics" directive ([[architecture-cybernetic-coherence-active]]):

1. `ashby_variety_match` (Ashby 1956 §11/7) — do `regulator_at(α+1)` and
   `regulated_at(α+1)` carry compatible variety budgets at altitude α?
2. `beer_requisite_variety` (Beer 1972, 1979) — does the regulator hold
   requisite variety at the VSM S3/S4 altitude?
3. `bateson_logical_type_match` (Bateson 1972; Russell-Whitehead 1910-1913)
   — do the two verdicts live at the same logical-type level?
4. `von_foerster_circular_reflexivity` (von Foerster 1981) — does each
   regulator's internal model include itself? (The Tomm probe `[D_F, a]`
   at altitude `α+1` IS the circular-reflexive measurement; see
   `[[architecture-error-as-tomm-probe]]`.)
5. `conant_ashby_good_regulator` (Conant-Ashby 1970) — **the load-bearing
   measurement**: the coextensivity hash-residual between
   `regulator_at(α+1)`'s verdict and `regulated_at(α+1)`'s verdict
   modulo `τ`.

A species fits the parametric form iff its lock decomposes into these
five measurements with species-specific type parameters. A species
refuses the form iff one or more of these decompositions fails at its
altitude.

## §4 The Polyak-Łojasiewicz contraction

For the lock to **hold** (rather than just be measurable), the residual
coextensivity across `N` pulses must contract:

```
ρ(N) = || residual_coextensivity(α, N) || / || residual_coextensivity(α, 1) ||
```

The lock holds iff `ρ(N) → 0` as `N → ∞` with rate satisfying
Polyak-Łojasiewicz. See Taut's `docs/specs/benchmark-tracing.md` §2.3
for the substrate's existing PL-contraction reading. T11.11 lifted the
bench harness for measuring this empirically at the cybernetic-coherence
altitude.

Lock-not-holding is itself substrate-data: failure verdict
`failure(opacity_map)` per `holonomy.md` §5 records WHERE the lock
fails. A species can have a measurable lock that doesn't hold; the
audit asks about the form, not the verdict.

## §5 The bundle-tower connection

The principal bundle tower from prism (see
`docs/math/the-tower/principal-bundles.md`) gives the natural geometric
home for the recursion-lock tower:

- **Fiber at altitude α**: the species' recursion-lock measurement
  apparatus (the five cybernetic ancestors + the Polyak-Łojasiewicz
  contraction reading).
- **Structure group `G`**: the type-parameter substitution group; τ
  is the gauge transformation between fibers at different altitudes.
- **Connection ω**: the natural mapping between adjacent altitudes' lock
  measurements (composition between adjacent altitudes per
  `altitudes.md` §5).
- **Holonomy around a closed altitude-loop**: the accumulated residual
  coextensivity around a cycle of altitudes returning to its start.
  Per `holonomy.md` §8, the librarian's perturbation IS gauge
  transformation; its quality metric IS the residual holonomy after
  perturbation. For recursion locks: the librarian's quality metric IS
  the residual coextensivity after the lock measures.

If candidate #63 promotes, the tower geometry says the family-root
structure of @cyberpunk is exactly the principal G-bundle whose total
space fibers the recursion-lock measurements across all altitudes.

## §6 Test surface (what a species must show)

A species `S` audited for membership in the tower must show:

1. **Two parallel-altitude operations** named at altitude `α+1`,
   operating on altitude-`α` data, with a substrate-architectural
   regulator/regulated relation between them.
2. **Two type-parameter carriers** `T_reg`, `T_regd` carried by the
   two verdicts respectively.
3. **A natural substitution** `τ : T_reg ↔ T_regd` whose naturality is
   substrate-evident at the altitude.
4. **A coextensivity reading**: a way to compare the two verdicts
   modulo `τ`. The species names how `τ` is computed; the
   coextensivity is the hash-residual after substitution.
5. **A contraction surface**: a bench-or-equivalent that exhibits the
   residual decaying (or not) across pulses; PL-contraction expected.

A species fails the test iff any of (1)–(5) cannot be named at the
species' altitude. Failure of (1) or (2) is the form refusal. Failure
of (4) or (5) is empirical (lock measurable but not holding). Failure
of (3) is the candidate #64 territory — the parametric form needs
weakening.

## §7 Pack ratification gates

Per the Pack's ratification discipline candidate #63 promotes when:

- **Second witness** lands: a species other than the template (T11.11)
  satisfies (1)–(5).
- **Third witness** lands: a species at a structurally-different altitude
  (home or federation per `altitudes.md` §4) satisfies (1)–(5).
- **Parametric form derivable**: the substrate admits a single
  parametric carrier instead of N hand-written species declarations.

The third condition is the strongest. If after three witnesses the
substrate still requires hand-written per-species lock declarations,
the candidate downgrades to "@cyberpunk happens to have lots of
Conant-Ashby-shaped species" rather than "@cyberpunk IS a recursion-
lock tower."

## §8 Species audits

Running tally. Each tick of `/loop @cyberpunk recursion-lock tower audit`
appends one species. Witness, retraction, or parametric-form refusal.

### §8.1 cybernetic coherence — first witness (T11.11, 2026-06-17)

The template. By construction the parametric form fits (the form was
extracted from this species). Verdict: **first witness**.

- `regulator_at(α+1)` = `measure_peer_reflection(altitude, op)`
- `regulated_at(α+1)` = `measure_librarian_perturbation(altitude, op)`
- `T_reg` = `Adjustment`
- `T_regd` = `Morphism`
- `τ` = morphism-shape preservation modulo altitude-specific
  newtyping
- Coextensivity: `verdict_coextensive(peer_verdict, librarian_verdict)`
  hashes the shape modulo `τ`
- Contraction surface: `recursion_lock_residual(altitude, pulse_count)`
  in `cybernetic_coherence.rs` bench scaffold (Taut 2026-06-17,
  spectral `047c8bd`)

Substrate-altitude declaration: `shards/cyberpunk/coherence.mirror`
(ccc227d, mirror taut/t11-11-cybernetic-coherence-benchmark branch).
Spec home: `docs/specs/cybernetic-coherence-benchmark.md`. Empirical
verdict pending real-witness swap once T11.10 + `@peer/cogito` land.

### §8.2 SEL coherence — second witness (this tick, 2026-06-17)

For the SEL (Source-available, Eventually-Libre) license boundary,
the parametric form applies as follows:

- `regulator_at(α+1)` = the SEL policy at altitude `α+1` (Beer's S5
  policy lifted to substrate altitude per Mara's mapping in
  `sel-as-executable-cyberpunk.md`)
- `regulated_at(α+1)` = the SEL enforcement at altitude `α+1` (the
  petri-net audit at Beer's S3 layer; per task #119)
- `T_reg` = `License` (the per-shard license-tier carrier; pure / io /
  ai / sel)
- `T_regd` = `Compliance` (the per-flow compliance verdict; whether the
  au+io boundary crossing observed the declared license)
- `τ` = the natural substitution between license-tier and compliance-
  verdict: a flow tagged `license = sel` produces a compliance verdict
  whose shape is determined by SEL clauses; the same flow tagged
  `license = apache` produces a compliance verdict whose shape is
  determined by Apache clauses. Naturality is substrate-evident
  because the license-as-type discipline ([[architecture-type-sel-io-au]])
  computes `τ` at compile time.
- Coextensivity reading: `verdict_coextensive(policy_verdict,
  enforcement_verdict)` hashes the compliance shape modulo `τ`.
- Contraction surface: forward-promised
  `spectral/benches/sel_coherence.rs` (Mara's spec §4; same shape as
  cybernetic_coherence.rs).

All five cybernetic-ancestor measurements admit SEL instances:

- `ashby_variety_match` — do policy and enforcement carry compatible
  variety budgets across the four license tiers (pure/io/ai/sel)?
- `beer_requisite_variety` — the canonical VSM read; SEL boundary IS
  Beer's actual deployment lifted to substrate altitude (cf. Cybersyn).
- `bateson_logical_type_match` — are License (a type) and Compliance (a
  verdict about a flow against a type) at the right Russell-Whitehead
  level? (Avoids "the license-of-the-license" paradoxes.)
- `von_foerster_circular_reflexivity` — the SEL license applies to its
  own enforcement mechanism. The petri-net audit's source code IS
  itself licensed under SEL. Circular reflexivity by construction.
- `conant_ashby_good_regulator` — the load-bearing piece. The SEL
  enforcement (petri-net) must BE a model of the SEL license (policy);
  the coextensivity verdict measures this directly.

**Verdict: second witness held.** The parametric form admits SEL
coherence cleanly. Forward-promised shard
`shards/cyberpunk/sel/coherence.mirror` (Mara's spec §3 layer 4); the
lock declaration would be a structural copy of `cyberpunk/coherence.mirror`
with type parameters `License ↔ Compliance` substituted for
`Adjustment ↔ Morphism`.

This is what Mara's spec called the "parametric collapse opportunity"
(§5): if both T11.11 and SEL fit the same parametric form, the
substrate should admit a single `@cyberpunk/coherence(T_reg, T_regd, τ)`
declaration that both species instantiate. The substrate-already-had-
the-word recognition would close at the parametric carrier.

### §8.3 [Next tick — next species under audit]

*This section appended by subsequent /loop ticks. Species order:
viable, bateson_learning, second_order, autopoiesis, distinction,
conversation, coevolution, requisite, design, algedonic. Order may
reorder per substrate-pull on what surfaces.*

## §9 What promotion or retraction closes

**Promotion path** (candidate #63 → recognition):

- Three witnesses (two landed, one at different-altitude species)
- Parametric form derivable: a single substrate-altitude declaration
  `prism @cyberpunk/coherence(T_reg, T_regd, τ)` that species instantiate
  rather than re-declare
- Math doc §2–§5 lifts from "the form" to "the theorem"; §8 sections
  rewrite as theorem corollaries
- Bundle tower at `principal-bundles.md` §7 gains a Recursion-Lock
  Tower cross-reference; @cyberpunk gets a dedicated section in
  `the-tower/README.md`

**Retraction path** (candidate #63 retracts to candidate #64):

- Two species refuse the parametric form at (1) or (2)
- The candidate downgrades: @cyberpunk happens to have many
  Conant-Ashby-shaped species rather than BEING a recursion-lock tower
- Candidate #64 surfaces: "the form/process integration at family-root
  altitude (recognition #61) is *necessary but not sufficient* for the
  tower structure"
- Math doc preserved as audit-record; §2 reframes as "the form some
  species satisfy"

**Weakening path** (candidate #63 refines mid-audit):

- A species satisfies the form with a *weakened* type-parameter
  substitution (one that's not natural isomorphism modulo altitude)
- The parametric carrier still derives but with a wider type-parameter
  signature
- Audit continues; #63 promotes to the weaker form; candidate #64 names
  the weakening

## §10 Prior art (substrate-already-had-the-word check)

- **Conant-Ashby 1970** — the foundational theorem. The recursion-lock
  parametric form IS this theorem applied at the substrate altitude.
- **Connes' spectral triple `(A, H, D)`** — the bounded-commutator
  condition `[D, a]` for `a ∈ A` is what the recursion lock measures
  at altitude `α+1` per [[architecture-error-as-tomm-probe]]. The
  Tomm probe at substrate altitude IS the recursion-lock measurement.
- **Maturana-Varela 1972/1980** — autopoiesis is the regulator-as-
  self-model. The recursion-lock theorem at the autopoietic altitude
  is what this audit's §8.x for `autopoiesis` will test.
- **von Foerster 1981 second-order observation** — the cybernetics of
  cybernetics. The recursion-lock tower hypothesis IS this discipline
  applied to the substrate's own family-root structure.
- **Beer's Viable System Model** — S3 (audit) and S5 (policy) at every
  recursion level. Each recursion level of a VSM has its own
  S3-versus-S5 lock; the recursion-lock tower at @cyberpunk would BE
  the VSM at substrate altitude.

## §11 Cross-references

- `[[docs/math/the-tower/principal-bundles.md]]` — the bundle geometry
  this tower instantiates
- `[[docs/math/the-tower/altitudes.md]]` §2–§5 — the named altitudes
  and composition between them
- `[[docs/math/the-tower/holonomy.md]]` §5, §8 — the verdict family as
  holonomy components; librarian perturbation as gauge transformation
- `[[docs/specs/cybernetic-coherence-benchmark.md]]` (T11.11 spec) —
  the template species
- `[[docs/specs/sel-as-executable-cyberpunk.md]]` (Mara's spec, 2026-06-17)
  — the second-witness mapping
- `[[architecture-cybernetic-coherence-active]]` — the promotion event
  for the @cyberpunk migration + candidate #61 (form/process partition
  non-universal across roots) which is the prerequisite to #63
- `[[architecture-spectral-triples-all-the-way]]` — the fractal
  self-similarity recognition; the recursion-lock tower would be the
  second concrete instance at one family root
- `[[architecture-cybernetic-foundation]]` — the 11-property family
  canon (now @cyberpunk); the species this audit walks
