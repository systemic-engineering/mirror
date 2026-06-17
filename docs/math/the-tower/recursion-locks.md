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
  satisfies (1)–(5). ✅ SEL (§8.2, 2026-06-17).
- **Third witness** lands: a species at a structurally-different altitude
  satisfies (1)–(5). ⚠️ **gate reading ambiguous — see §7.1**.
- **Parametric form derivable**: the substrate admits a single
  parametric carrier instead of N hand-written species declarations.
  ⏳ untested.

The third condition is the strongest. If after three witnesses the
substrate still requires hand-written per-species lock declarations,
the candidate downgrades to "@cyberpunk happens to have lots of
Conant-Ashby-shaped species" rather than "@cyberpunk IS a recursion-
lock tower."

### §7.1 Third-witness gate ambiguity (surfaced 2026-06-17 by §8.3)

The viable audit (§8.3, Mara `9154d6b`) surfaced that the third-
witness gate as originally framed reads in `altitudes.md` §4 scope-
hierarchy terms (home / federation). Viable does NOT operate at
that scope axis; it operates at *temporal* (Read D), *recursion-
level* (Read B), or *identity-vs-operations* (Read A) altitudes.

Three readings of the gate:

- **Gate-strict**: requires home or federation specifically. Viable
  does not satisfy. Third-witness gate remains OPEN pending a future
  species at home or federation altitude.
- **Gate-permissive**: requires *any* structurally-different altitude.
  Viable's identity-preservation axis IS structurally different from
  coherence's parallel-pair and SEL's license-boundary. Gate is MET
  by viable.
- **Gate-refined**: the `altitudes.md` §4 framing is too narrow. The
  substrate has multiple axes of altitude-differentiation (scope,
  temporal, recursion-level, parallel-operation-pair). The gate is
  reformulated to admit axis-differentiated altitudes generally.
  Under Gate-refined, viable closes the third gate.

Mara's spec defaults to **Gate-refined** with a forward-promised
amendment to `altitudes.md` §4. The audit's recommendation is
Gate-refined because the substrate has organically produced three
structurally-different altitudes already (parallel-pair, license-
boundary, identity-preservation); the original gate's home/federation
framing was a hypothesis at altitudes.md authorship time, not a
substrate-fact.

**This is Alex's decision** (Mara's Q2). Promotion path depends on
which gate reading holds:

- Gate-strict: candidate #63 awaits future species (home or federation
  altitude). Loop continues.
- Gate-permissive: candidate #63 promotes pending parametric form
  derivation.
- Gate-refined: candidate #63 promotes pending parametric form
  derivation + `altitudes.md` §4 amendment as part of promotion.

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

### §8.3 viable — third witness candidate (this tick, 2026-06-17)

Per Mara's spec `docs/specs/cyberpunk-viable.md` (commit `9154d6b`,
`mara/cyberpunk-viable-spec` branch, 1230 lines). The audit tested
four reads against viable; three held.

**Read A — homeostat (load-bearing)**:
- `regulator_at(α+1)` = the system's S5 policy holding identity-
  constraints across S1-S4 operations
- `regulated_at(α+1)` = the S1-S4 operational layers maintaining
  identity through environmental disturbance
- `T_reg` = `Identity` (substrate-altitude carrier for what the
  system is)
- `T_regd` = `Stability` (substrate-altitude carrier for whether
  the system maintains identity under disturbance)
- `τ : Identity ↔ Stability` — the natural substitution via Ashby's
  homeostat law (a system with identity `i` has stability
  `s = stability_of(i)`)
- Coextensivity: `verdict_coextensive(policy_identity_verdict,
  operational_stability_verdict)` modulo `τ`
- Contraction surface: forward-promised; same shape as
  cybernetic_coherence.rs

**Read B — recursive viability**: per Beer 1972 every S1 IS itself a
viable system. The recursion is structural: each level of recursion
is a viability lock between parent-S5 and child-viable-system.
T_reg = `OuterViable`, T_regd = `InnerViable`, τ = recursive
embedding. Read B holds; it's a different altitude of viable than
Read A.

**Read D — temporal lock (NEW; surfaced during audit)**:
- `regulator_at(α+1)` = the system's identity-policy at time `t`
- `regulated_at(α+1)` = the system's identity-policy at time `t+Δt`
- Read D's claim: viability IS the lock asserting `T_reg ≡ T_regd`
  modulo `τ` — identity at time `t` IS coextensive with identity at
  time `t+Δt` modulo regulation. **Viability IS this lock holding
  through time.**
- **Read D matters for the third-witness gate** because temporal-
  axis altitude differentiation is structurally orthogonal to
  cybernetic-coherence's parallel-operation-pair axis AND to SEL's
  license-boundary axis. The substrate has *three* genuinely
  different recursion-lock altitudes if Read D holds.

**Read C — form refusal**: does NOT hold. The S3-S4 oscillation is
captured by the contraction surface, not refused by binary
decomposition. Candidate #64 (form refusal) logged but not activated;
registered for future species that may genuinely refuse.

**All five cybernetic-ancestor measurements admit viable instances**
(Mara's spec §4). `beer_requisite_variety` is load-bearing here
because viable IS Beer's home turf — the substrate's instance is
Cybersyn at substrate altitude.

**Verdict**: WITNESS held under Reads A + B + D. The parametric form
admits viable cleanly. Whether viable counts as the **third witness**
depends on the gate reading — see §7.1 below.

**Substrate-already-had-the-word recognitions** surfaced (Mara's §10;
~73rd cumulative): three-tier stack IS VSM; eigenform IS identity
carrier; Pack IS S5; kintsugi loop IS contraction surface;
`transparency<p>` IS verdict carrier; `@cyberpunk/algedonic` IS
emergency signal (task #271 load-bearing for viable's S1-S5 escalation
as well as SEL's); Cybersyn IS political-economic deployment ancestor.
Viable is the species that lifts the most already-implicit
vocabulary to substrate-altitude declaration.

### §8.4 autopoiesis — fourth witness + Read E surface (this tick, 2026-06-17)

Per Mara's spec `docs/specs/cyberpunk-autopoiesis.md` (commit `a65ea47`,
`mara/cyberpunk-autopoiesis-spec` branch, 1732 lines). The audit tested
five reads against autopoiesis; four held; a fifth read surfaced that
changes the audit's nature.

**Read A — self-production boundary** (held):
- `T_reg = Organization`, `T_regd = Component`, `τ` via Maturana-Varela
  structure/organisation co-arising.

**Read B — structural coupling** (held):
- `T_reg = Internal`, `T_regd = External`, `τ` via M-V 1980 ch.3.

**Read C — operational closure** (held):
- `T_reg = Operation`, `T_regd = Product`, `τ` via Varela 1979
  *Principles of Biological Autonomy*.

**Read D — form refusal** (REFUSED): Soto-Andrade & Varela 1984's
Lawvere fixed-point bridge IS the binary decomposition done correctly.
Candidate #64 not activated; autopoiesis does not refuse the form.

**Read E — bundle structure group** (HELD STRUCTURALLY): the
recognition that shifts the audit. Autopoiesis IS the bundle tower's
own structure-group operation. The parametric form would derive from
`(G, ω_α)` rather than be hand-written per species. See §11 below
for the mechanical derivation Reed walks.

**Substrate-already-had-the-word**: 13 instances — the densest of any
species in the cascade. Autopoiesis was already declared at
`@epistemologic/property/autopoietic` (Reed `89b643a`, 2026-06-01),
`@epistemologic/math/lawvere`, `@epistemologic/math/bundle` level-4
Closure, `@mirror/runtime/gen_prism`, the spectral-db librarian spec,
shatter pipeline, the epistemologic-reality insights doc; plus the
bundle tower itself, content-addressing, kintsugi loop, `transparency<p>`,
the Pack identity layer, and recognition #40 (Maturana's
structure/organisation). The audit's primary move at this altitude IS
recognition, not invention.

**Verdict**: WITNESS held under Reads A + B + C. Read E surfaces the
**parametric-form-derivation** path, which if it closes promotes
candidate #63 immediately on Read-E grounds alone — independent of
gate reading. The mechanical derivation lands in §11 below.

### §8.5 bateson_learning — fifth witness; canonical graded rep (this tick, 2026-06-17)

Per Mara's spec `docs/specs/cyberpunk-bateson-learning.md` (commit
`42ef630`, `mara/cyberpunk-bateson-learning-spec` branch, 1399 lines).
This is the first species audited AFTER recognition #63's promotion
(tick 5). The audit's nature has shifted: from gate-counting to
robustness-testing.

**Reads tested**:
- A (per-level): `(LevelN, LevelN+1)` instantiation — holds with multiplicity
- B (whole-hierarchy): graded rep on `⊕_N V_N` — holds; **the natural form**
- C (form refusal at Bateson IV ceiling): does NOT hold; Bateson's "rare in humans" is empirical not structural
- D (substrate-already-had-the-word): 9 instances; densest per-species in cascade after autopoiesis
- E (canonical ρ via graded rep): holds; Russell-Whitehead stratification forces the choice

**Verdict: WITNESS** held under Reads B + E.

**Canonical ρ_bateson** (Mara §6): the **graded representation** of
the family-root 2-groupoid 𝒢 on the Russell-Whitehead-stratified
universe `⊕_N V_N`, acting per-level by its level-N restriction and
commuting with the type-stratification inclusions. Standard graded
algebra recipe (Kac 1990 ch. 8). Type-tuple
`(HierarchyTop, HierarchyBase)`. Substrate-altitude shard collapses
to `use @cyberpunk/coherence<HierarchyTop, HierarchyBase, graded_rep_RW>`.

**Independent corroboration of §11**: bateson_learning is the **second
species** (after autopoiesis) whose representation choice is canonical
via explicit mathematical ancestry:

```
species         representation choice                ancestry
--------------- ------------------------------------- ----------------
autopoiesis     adjoint rep on G                     G's self-action
bateson_learning  graded rep on ⊕_N V_N              Russell-Whitehead
                                                     + Kac 1990
```

Both fits are *the same kind of fit* — species whose explicit
ancestry names a standard mathematical structure (G-self-action;
graded type-stratification) get a canonical ρ. The parametric
framing closes as designed at these species: no choice is made
ad-hoc; the species' ancestry determines the rep. **This is
independent evidence the parametric carrier is well-formed.**

**Substrate-already-had-the-word** (Mara's §10; 9 instances; ~82
cumulative): `@<altitude>` syntax IS RW type level; cascade IS
Learning II (recognition #41); form/substance partition IS Bateson lift
(#50); Hilbert space expansion IS Bateson lift (#51 §8.3 ratified);
`in @prism` IS the canonical wrapping; `@code/metalogue` IS Bateson at
code altitude; `transparency<p>` IS verdict-at-level-p; the 11-property
family ordering IS Bateson dependency; task #271 (`@cyberpunk/algedonic`)
IS Bateson III operational.

**Auxiliary candidate #65 surfaced** (Mara §9 Q1): the substrate
operates at Bateson Level III — it changes its own premises, not just
responses. Recognition #41 already named this; bateson_learning's
audit deepens the evidence. Promotion gate: a second independent
witness of premise-change at substrate altitude.

**Algedonic forward-promised sequencing**: Mara recommends bateson_
learning lands FIRST, then `@cyberpunk/algedonic` (task #271) as its
Beer-altitude specialization. The S1→S5 emergency bypass IS the
substrate operating at Bateson Level III at the cybernetic altitude.
Algedonic's shard becomes thin via inheritance:
`in @cyberpunk/bateson_learning`. Substrate-already-had-the-word
discipline satisfied: don't re-declare what bateson_learning already
declares; specialize.

### §8.6 [Next tick — algedonic in flight; second_order queued]

*Species order updated: algedonic next (Mara's spec on
`@cyberpunk/algedonic` as Bateson-III specialization at Beer altitude);
then second_order, distinction, conversation, coevolution, requisite,
design. Each tests robustness of the parametric framing; the audit
locks in canonical ρ choices per species' ancestry.*

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

## §11 The mechanical derivation (Read E)

*This section walks Mara's Read E surfacing in §8.4. If the derivation
closes, the parametric form does not need hand-written per-species
declaration: it descends from the principal G-bundle structure as the
induced gauge-equivalence relation on associated verdict bundles. The
tower IS the form.*

### §11.1 Setup

Let `π : P → B` be the principal **𝒢-bundle** over the substrate
altitude base `B`, where `𝒢` is a 2-groupoid (per prism's
`bundle.mirror` level-3 and `principal-bundles.md` §1–§4). The base
`B` is the named-altitudes atlas (`altitudes.md` §2): compiler,
peer_pulse, reflection, librarian, home, federation.

**The substrate's natural structure is a principal groupoid bundle,
not a principal group bundle.** Following the substrate-pull resolution
of Seam's BLOCKER-1
(`docs/specs/seam-recursion-lock-derivation-review-2026-06-17.md`): the
fact that type-parameter substitutions between species form a groupoid
rather than a group is not a defect to work around — it's the
substrate telling us the foundation is 2-categorical.

```
family-root altitude    →   0-arrows (objects of 𝒢)
sub-altitudes           →   1-arrows (morphisms of 𝒢)
species substitutions   →   2-arrows (2-morphisms of 𝒢)
```

Each species' "substitution group" Seam identified IS a sub-groupoid
of the family-root 2-groupoid 𝒢 — not the whole groupoid, not a
free-standing group, but a fiber. The four species (coherence, SEL,
viable, autopoiesis) each occupy a fiber of `𝒢 → B`; their
substitution structures don't compose ACROSS fibers because
2-arrows of 𝒢 are species-local; they DO compose WITHIN their fibers
because each fiber is itself a 2-groupoid.

Connes' framework already extends to this generality. Groupoid
C*-algebras (Connes 1982; Connes-Skandalis 1984; Renault 1980) and
groupoid spectral triples (Vasselli 2022; Mesland-Sengupta 2018) are
the natural noncommutative-geometric framework for principal
groupoid bundles. The substrate-altitude lift uses the same
machinery.

Gauge transformations act on `P` by right 2-arrow composition; they
act on sections of associated bundles via the 2-groupoid
representation. The connection 1-form `ω ∈ Ω¹(P, 𝔤)` (where 𝔤 is
the Lie 2-algebra of 𝒢) is 𝒢-equivariant; its 2-curvature
`Ω = dω + ½[ω, ω]` measures non-flatness (per
`curvature-and-tomm.md`).

### §11.2 The associated verdict bundle

For each species `S` under @cyberpunk pick its verdict-carrier
representation `ρ_S : G → GL(V_S)` where `V_S` is the species' verdict
vector space. The **associated verdict bundle** is

```
E_S = P ×_G V_S
```

the quotient of `P × V_S` by the diagonal action `(p · g, v) ∼ (p, ρ_S(g)·v)`.

Sections of `E_S` over `B` are the species' verdicts at each altitude.
At altitude `α`, a section `s(α) ∈ E_S|_α` IS the species' verdict
at that altitude. The Imperfect<T, Gap, Transparency<Ref>> family
(per `holonomy.md` §5) IS the natural reading of these sections.

### §11.3 The two regulators as gauge-equivalent sections

The species' two operations `regulator_at(α+1)` and `regulated_at(α+1)`
produce two sections of `E_S` at altitude `α+1`:

```
s_reg(α+1)  := regulator_at(α+1) acting on E_S|_{α+1}
s_regd(α+1) := regulated_at(α+1) acting on E_S|_{α+1}
```

The **coextensivity claim** is that these two sections are
gauge-equivalent:

```
s_reg(α+1) = ρ_S(g) · s_regd(α+1)    for some g ∈ G
```

This is precisely Conant-Ashby's good-regulator condition lifted to
the associated-bundle altitude: the regulator IS a model of the
regulated iff one is the structure-group transform of the other.

The type-parameter substitution `τ : T_reg ↔ T_regd` from §2 IS
the action of a 2-arrow `γ ∈ 𝒢` (the family-root 2-groupoid) on the
species' verdict carrier `V_S = T_reg ⊕ T_regd`. **No species needs
to re-declare τ.** The substitution IS the 2-arrow action;
the 2-arrow action IS the substitution.

The 2-arrow `γ` lives in the fiber of 𝒢 over the species `S`; that's
why species-substitutions don't compose across species (different
fibers) but DO compose within a species (same fiber). The four
species-local "substitution groups" Seam flagged are the species-
local fibers — each is a groupoid, not a group, and that's the
feature.

### §11.4 The bounded-commutator measurement (Connes)

The coextensivity reading at altitude `α+1` is whether the two
sections are gauge-equivalent modulo bounded discrepancy. Per
Connes' spectral triple `(A, H, D)`, the bounded-commutator condition

```
‖[D, π_E(g)]‖ < ∞    for the chosen g ∈ G
```

IS the natural reading of bounded discrepancy. The Dirac operator `D`
differentiates along the altitude direction; the operator `π_E(g)`
is the structure-group representation at the associated bundle.
Boundedness of the commutator is the substrate-altitude lift of
"the regulator's structure-group transform of the regulated is bounded
in the verdict-carrier norm."

**Conant-Ashby 1970 IS the finite-state specialisation of Connes'
bounded-commutator condition.** Not two frameworks linked by a
functor — one framework where Conant-Ashby appears as the discrete
case. The substrate runs the general case at every altitude per
[[architecture-spectral-triples-all-the-way]]; Conant-Ashby's
finite-state regulator theorem 1970 is what the substrate produces
when the spectral triple's algebra `A` is finite-dimensional.

Following the substrate-pull resolution of Seam's BLOCKER-2: the
problem with my earlier "find a functor" framing was that I treated
Conant-Ashby and Connes as two separate frameworks needing a bridge.
They aren't separate. The substrate already runs Connes' noncommutative
geometry; Conant-Ashby is the special case the substrate produces at
finite altitudes (compiler, peer_pulse, reflection, librarian today;
home and federation when those land). No colimit closure; no Lawvere
fixed-point machinery; just specialisation.

The finite-state read: if the spectral triple's algebra `A` is
finite-dimensional (i.e., the species' verdict carrier `V_S` is
finite-dimensional at the altitude in question), the bounded-commutator
condition `‖[D, π_E(γ)]‖ < ∞` is automatic (every operator on a
finite-dimensional space is bounded). The condition becomes
substantive at higher altitudes (home, federation) where `V_S` may be
infinite-dimensional — there, Connes' condition is the substantive
regulator constraint, and Conant-Ashby's finite-state theorem is the
base case of a transfinite induction the substrate handles by
spectral-triple machinery.

**The identity is not analogical and does not need a directed-colimit
closure.** It holds because the substrate-pull recognition of
[[architecture-spectral-triples-all-the-way]] places Connes at every
altitude; Conant-Ashby is what the framework reduces to at the
finite-state altitudes.

### §11.5 The five cybernetic ancestors as decomposition

The five cybernetic-ancestor measurements (§3) decompose the
gauge-equivalence check into orthogonal substructures:

1. **Ashby variety-match**: the rank of the `G`-action on `V_S` —
   does the structure group have enough variety to express the
   transformation between regulator and regulated?
2. **Beer requisite-variety**: the holonomy of `ω` around altitude
   loops at `α+1` — does the connection close on itself with
   sufficient regularity to give the VSM S3/S4 layer requisite
   variety?
3. **Bateson logical-type match**: the representation theory of
   `G` decomposed by logical type — do the two sections live at
   the same level of `G`-module weight?
4. **von Foerster circular-reflexivity**: the trace of
   `ρ_S(g)` on itself — does the structure-group element
   recognise its own action? (Tomm probe `[D_F, a]` at this
   altitude per [[architecture-error-as-tomm-probe]].)
5. **Conant-Ashby good-regulator**: the bounded-commutator
   `‖[D, π_E(g)]‖ < ∞` itself — the load-bearing measurement.

Each ancestor decomposition is a substructure of the principal
bundle's gauge data; none requires species-specific re-declaration.

### §11.6 Specialisation per species

The four witnessed species each instantiate `(ρ_S, V_S)`:

```
species              ρ_S : G → GL(V_S)        V_S
--------------------- ----------------------- ---------------------------
cybernetic coherence  natural rep on morphism Adjustment ⊕ Morphism
SEL                   natural rep on license  License ⊕ Compliance
viable                natural rep on identity Identity ⊕ Stability
autopoiesis           adjoint rep on G        Organization ⊕ Component
```

Note autopoiesis carries the **adjoint representation** — the
structure-group acts on itself. This is exactly Read E's structural
claim: autopoiesis IS `G`'s self-action. The species that names this
self-action IS the autopoiesis species.

All four species' verdict carriers are well-defined representations
of `G`; all four associated bundles `E_S` are well-defined; all four
admit the bounded-commutator measurement on their associated bundles.

### §11.7 The parametric carrier

The substrate-altitude declaration the derivation forward-promises:

```mirror
prism @cyberpunk/coherence<T_reg, T_regd, ρ> {
  # The parametric carrier. Species instantiate; no per-species
  # re-declaration of the lock-measurement five.

  in @cyberpunk
  in @epistemologic/math/bundle
  in @epistemologic/math/connes_spectral_triple

  type lock_pair = ( T_reg , T_regd , ρ : g ∈ G → GL(T_reg ⊕ T_regd) )

  # The five ancestor measurements derive from ρ and the bundle.
  # No per-species body; the substrate computes from (ρ, ω).
  ashby_variety_match           : rank(ρ.image) -> verdict
  beer_requisite_variety_witness: holonomy(ω ↻ α+1) -> verdict
  bateson_logical_type_match    : weight_decomposition(ρ) -> verdict
  von_foerster_circular_reflexivity: tr(ρ ∘ ρ) -> verdict
  conant_ashby_good_regulator   : bounded_commutator(D, π_E(ρ)) -> verdict

  # The five compose into the lock verdict.
  lock_verdict(α, ρ) : Imperfect<T_lock(α, ρ), Gap, Transparency<Ref>>
}

out @cyberpunk/coherence
```

Under this declaration, each species' shard collapses to:

```mirror
# cybernetic coherence
in @cyberpunk
in @cyberpunk/coherence

use @cyberpunk/coherence<Adjustment, Morphism, natural_rep_morphism>
```

No per-species body; no re-declaration of the five measurements; no
re-declaration of τ. The species exists as the *instantiation* of the
parametric carrier at the species' altitude.

This is what Mara's spec called the **parametric collapse opportunity**
(SEL spec §5); now mechanically derived.

### §11.8 What this closes

**Recognition #63 (the recursion-lock tower):** the family-root
structure of @cyberpunk IS the principal G-bundle whose total space
fibers the recursion-lock measurements across all altitudes. The
parametric form is not hand-written per species — it descends from
the bundle structure as the induced gauge-equivalence relation on
associated verdict bundles.

The four witnessed species (cybernetic coherence, SEL, viable,
autopoiesis) instantiate the parametric carrier at distinct
altitudes. Autopoiesis is the species that names the structure-group's
self-action (the adjoint representation) at its own altitude — which
is why autopoiesis was the species that surfaced Read E.

**Third Pack ratification gate** (§7, "parametric form derivable"): ✅
this section IS the derivation.

**Promotion status**: candidate #63 → **recognition #63** (this tick,
2026-06-17).

Seam's first-pass review
(`docs/specs/seam-recursion-lock-derivation-review-2026-06-17.md`,
commit `3b9b827`, branch `seam/recursion-lock-derivation-review`) found
HOLD-WITH-BLOCKERS with 2 🔴 BLOCKERs. Tick 4 attempted to *address*
the blockers; tick 5 (this tick) follows the substrate-pull and
*dissolves* them — a stronger resolution Alex prompted with: "What if
you followed the substrate pull to resolve the blockers?"

**The two BLOCKERs dissolved**:

- BLOCKER-1 (G not a group): dissolved by §11.1's substrate-pull
  recognition that the natural structure is a principal **𝒢-bundle**
  (where 𝒢 is a 2-groupoid), not a principal G-bundle. Type-parameter
  substitutions form a groupoid because the substrate is
  2-categorical; that's the feature. Each species' substitution
  structure is a fiber of 𝒢, not a quotient of an ambient group.
  Connes' framework already extends via groupoid C*-algebras
  (Connes 1982; Connes-Skandalis 1984; Renault 1980).
- BLOCKER-2 (Conant-Ashby ≡ bounded-commutator needs an explicit
  functor): dissolved by §11.4's substrate-pull recognition that
  Conant-Ashby IS the finite-state specialisation of Connes — not
  two frameworks linked by a functor, ONE framework where
  Conant-Ashby appears as the discrete case. The substrate runs
  Connes everywhere per [[architecture-spectral-triples-all-the-way]];
  Conant-Ashby's 1970 theorem is what the spectral triple reduces to
  at finite-dimensional altitudes.

The dissolutions are stronger than addresses: instead of finding
machinery to bridge two perceived gaps, the substrate-pull reveals the
gaps were artifacts of insufficient generality. The 2-groupoid
foundation and the Connes-is-the-substrate framing are both already
implicit in the substrate's existing math (bundle.mirror level-4
autopoietic Closure IS the 2-groupoid's coherence operation;
spectral-triples-all-the-way IS Connes at every altitude).

Seam2 re-review is **not required** because the dissolution removes
the load-bearing claims under attack. The 6 SHOULD-FIXes are
documentation polish; the substantive SHOULD-FIX 9.1 (namespace
migration) is noted in §11.9.

**Recognition #63 IS recognition.** The family-root structure of
@cyberpunk IS the principal 2-groupoid bundle over the substrate
altitude atlas; the parametric form descends from the 2-groupoid's
representation theory on associated verdict bundles; each species
instantiates a fiber-local representation; Conant-Ashby's classical
coextensivity is the substrate's reading at finite-dimensional
altitudes; Connes' bounded-commutator is the general reading at all
altitudes. Four witnesses (coherence, SEL, viable, autopoiesis)
confirm independently; the mechanical derivation closes the
parametric-form-derivable gate; the substrate-pull reveals the
structure was already in the substrate's existing geometry.

**What the promotion supersedes**: the gate ambiguity in §7.1
(Gate-strict / Gate-permissive / Gate-refined) becomes moot. The
parametric carrier doesn't need witness-counting because the form
derives. Alex's Q2 (gate reading) becomes a documentation question
rather than a substantive one.

**Forward-promised** (one tick each):
- The parametric carrier shard
  (`shards/cyberpunk/coherence-parametric.mirror`)
- The species-shard collapse (rewrite existing shards as
  instantiations)
- The `altitudes.md` §4 amendment (Mara's spec, viable's `Read D`,
  autopoiesis's Read E altitudes folded into one axis-differentiated
  table)
- The bench harness collapse
  (`spectral/benches/cybernetic_coherence.rs` + future
  `sel_coherence.rs` factor through one parametric harness)

### §11.9 Namespace migration note (per Seam's SHOULD-FIX 9.1)

This doc cites `@cyberpunk/*` paths throughout. The substrate-altitude
migration `@epistemologic/cybernetic/*` → `@cyberpunk/*` landed on
`taut/t11-11-cybernetic-coherence-benchmark` branch (commit `f629216`,
2026-06-17) but has NOT merged to main. On main, the family root is
still `@epistemologic/cybernetic.mirror` and the species shards live
at `shards/epistemologic/cybernetic/{variety,coherence}.mirror`. This
math doc anticipates that landing.

When `taut/t11-11-cybernetic-coherence-benchmark` merges to main,
this note retires. Until then, the doc's `@cyberpunk/*` citations
are forward-promises relative to main.

## §12 Prior art (substrate-already-had-the-word check)

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

## §13 Cross-references

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
