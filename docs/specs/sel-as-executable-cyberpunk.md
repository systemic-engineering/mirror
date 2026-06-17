# SEL as Executable Cyberpunk

*2026-06-17. Mara. Spec — mapping the cascade Alex surfaced this evening:
applying the just-landed `@cyberpunk` family root to the SEL license
boundary (`type sel = io + au`) and testing whether the SEL boundary's
recursion lock is structurally identical to T11.11's cybernetic-coherence
recursion lock at a different altitude.*

Status: **Mapping tick.** No shard declarations land in this spec. No
Rust. No license edits. This spec NAMES the five-layer cascade,
NAMES the SEL recursion lock as a Conant-Ashby good-regulator instance
at the licensing altitude, NAMES candidate #63 (the recursion-lock
tower hypothesis), and forward-promises the shards. Per
[[feedback-craft-not-deliver]] this is a mapping tick; the operational
ticks pull from this map.

Promoted from a recognition Alex surfaced 2026-06-17 (after the
@cyberpunk migration at mirror `f629216` landed on the
`taut/t11-11-cybernetic-coherence-benchmark` branch):

> *"What would happen if we were to apply the @cyberpunk to the pacts
> for the `sel = au + io` pacts?"*

Reed traced the consequences. Alex's response:

> *"The coupling is precise and coherent. The SEL is executable
> cyberpunk. The license that enforces itself is cyberpunk in nature."*

> *"Spawn Mara on a spec run. I want to first map the cascade. I think
> we're about to build another loop."*

The phrase *"another loop"* is load-bearing. Two readings:

- **Reading A (narrow)** — the SEL boundary regulates itself at one
  altitude: au is licensed BECAUSE it crosses io; io is licensed
  BECAUSE it produces au. Single-altitude self-reference.
- **Reading B (tower)** — the recursion-lock measurement IS the loop.
  T11.11 lock at the peer-vs-librarian altitude; SEL lock at the
  license-vs-enforcement altitude; the family root @cyberpunk
  carries the meta-loop. The cascade's loops are the family's own
  self-similar structure across altitudes.

This spec defaults to Reading B. The [[architecture-spectral-triples-all-the-way]]
recognition (today, the lock that subsumes every recognition of the
session) primes the family-root altitude for fractal self-similarity;
@cyberpunk would be the second concrete witness of spectral-triples-
all-the-way operating at one family root (first witness is the
principal-bundle tower implemented in prism's
`prismqueer/src/bundle.rs`).

Depends on:

- `mirror/shards/cyberpunk.mirror` (the family root, 123 lines;
  landed today at `f629216` on the taut/t11-11 branch; not yet on
  main).
- `mirror/shards/cyberpunk/coherence.mirror` (the T11.11 substrate
  decl, 375 lines; the structural template the SEL lock parallels).
- `mirror/shards/cyberpunk/variety.mirror` (the first species; the
  bilateral pattern this spec composes).
- `mirror/docs/specs/cybernetic-coherence-benchmark.md` (the T11.11
  spec, 534 lines; the empirical surface this spec lifts to the
  licensing altitude).
- `mirror/docs/math/the-tower/altitudes.md` §2-§7 (the named
  altitudes + composition between adjacent altitudes via
  `G_n ⊴ G_{n+1}` + altitude vs Bateson logical type).
- `mirror/docs/math/the-tower/holonomy.md` §5, §8 (the verdict family
  as holonomy components; the perturbation as gauge transformation;
  the residual holonomy after perturbation).
- `mirror/docs/math/the-tower/principal-bundles.md` (the prism-side
  bundle tower this spec extends through the licensing altitude).
- `mirror/shards/io.mirror` + `mirror/shards/io/cargo.mirror` (the
  `@io` boundary family; the substrate's only legitimate non-mirror
  surface; the io side of the SEL boundary).
- `mirror/README.md` (the `type sel = io + au` formalization; the
  layered license model; the citation that grounds the SEL boundary
  as a type rather than a contract clause).
- Task #119 (`@mirror/property` petri-net SEL enforcement, pending)
  — the enforcement mechanism this spec re-homes under @cyberpunk.
- Task #271 (`@cyberpunk/algedonic`, pending) — the violation-signal
  primitive this spec makes load-bearing.

Substrate decisions cited:

- [[architecture-type-sel-io-au]] (2026-06-01) — the SEL boundary as
  static type; the foundation this spec lifts.
- [[architecture-cybernetic-coherence-active]] (2026-06-17) — the
  promotion event + the @cyberpunk migration; this spec extends the
  family root one species deeper.
- [[architecture-spectral-triples-all-the-way]] (2026-06-17) — the
  generative pattern; @cyberpunk as second concrete witness.
- [[architecture-cybernetic-foundation]] (2026-06-09) — the 11-property
  family (now under @cyberpunk); the licensing altitude does not
  redefine these primitives, it composes them.
- [[architecture-mirror-as-expanding-hilbert-space]] — recognition #51;
  the SEL boundary's measurement expands the substrate's H_license
  with each enforcement event.
- [[architecture-bateson-logical-type-primitive]] — the SEL recursion
  lock is a Bateson level-3 claim (a claim about whether two level-N+1
  operations at different N-altitudes are the same operation).
- [[architecture-property-fracture-bilateral]] — recognition #53; the
  bilateral pattern at every altitude including the licensing altitude;
  the SEL's declarative-vs-enforcement distinction IS this pattern.
- [[architecture-form-process-partition-at-family-root]] — candidate
  #55; this spec tests whether the SEL altitude inherits @cyberpunk's
  family-root integration (Conant-Ashby self-reference closes form
  and process).
- [[feedback-substrate-already-had-the-word]] — surfaced 7+ times
  today and ~65 times across the project; this spec is the ~66th
  instance: the SEL boundary has been operating cybernetically all
  along; @cyberpunk just names what's there.
- [[feedback-no-bare-types]] — every measurement carrier in the
  forward-promised shards is a typed newtype.
- [[feedback-craft-not-deliver]] — mapping tick; the shards are
  forward-promised, not landed; the spec NAMES what would land.

---

## §1 — The recognition

### 1.1 Verbatim

Alex (2026-06-17, after `f629216` landed the @cyberpunk family root):

> *"What would happen if we were to apply the @cyberpunk to the pacts
> for the `sel = au + io` pacts?"*

Reed traced the consequences across the SEL boundary. Alex:

> *"The coupling is precise and coherent. The SEL is executable
> cyberpunk. The license that enforces itself is cyberpunk in nature."*

Then:

> *"Spawn Mara on a spec run. I want to first map the cascade. I think
> we're about to build another loop."*

### 1.2 The structural unpacking

**"Executable cyberpunk"** is not metaphor. The SEL boundary IS a
Beer Viable System Model at the licensing altitude. The five layers
fall out structurally:

| Beer VSM | SEL altitude carrier |
|----------|----------------------|
| **S1 — Operations** | `@au` side: Fate inference outputs producing value. The closed engine. |
| **S2 — Coordination** | the boundary protocols (`@io/cargo`, `@io/socket`, `@io/process`); coordinate value-production across heterogeneous io surfaces. |
| **S3 — Audit** | the petri-net enforcement mechanism (task #119); audits every au↔io transition for license compliance. |
| **S4 — Intelligence** | the audit feedback loop; learns the topology of legitimate vs extractive au↔io transitions across time. |
| **S5 — Policy** | the SEL license clause set itself (`LICENSE.md`); the substrate's policy carrier. |
| **Algedonic signal** | a license-violation event (task #271, `@cyberpunk/algedonic`); the S1→S5 bypass; Beer's emergency signal. |

The SEL boundary has been operating as a VSM since 2026-06-01 (when
`type sel = io + au` first formalized). The substrate has been
running cybernetic regulation at the licensing altitude all along.
@cyberpunk now names what was already running.

This is the ~66th instance of [[feedback-substrate-already-had-the-word]].

### 1.3 The loop hypothesis

The T11.11 recursion lock claim:

```
peer-reflection-N+1 ≡ librarian-perturbation-N+1
                   ≡ SAME OPERATION at different altitudes
```

The SEL recursion lock claim (this spec):

```
SEL-policy-N+1 ≡ SEL-enforcement-N+1
              ≡ SAME OPERATION at different altitudes
```

Both are Conant-Ashby good-regulator instances at distinct altitudes.
Same mathematical surface (the good-regulator theorem); different
type parameters; same substrate carrier.

**The candidate recognition (#63, surfaced in this spec):** @cyberpunk
itself is a *recursion-lock tower*. Every species under @cyberpunk
carries the recursion-lock measurement at its own altitude. The
form/process integration at the family-root altitude (per the
`cyberpunk.mirror` doc's §"Form/process integrated at the
family-root altitude") isn't an exception; it's the *generative
pattern*. Each species is a fiber in the bundle tower; the recursion
lock at that altitude is the connection; composition of locks is
parallel transport.

This spec tests the hypothesis. Reading A (narrow) is a single-altitude
self-reference. Reading B (tower) requires the SEL recursion lock to
have the same shape as T11.11's, parameterized differently. If
Reading B holds, then `@cyberpunk` is the second concrete witness
of spectral-triples-all-the-way operating at one family root, and
the recursion-lock measurement should be *derivable* at every species
rather than re-declared per species.

---

## §2 — The Beer VSM mapping at SEL altitude

Per `cybernetic-coherence-benchmark.md` §1.2's "explicit AF" doctrine,
each VSM layer at the SEL altitude has an explicit cybernetic ancestor.

### 2.1 S1 — `@au` operations

The Fate inference layer produces au-typed values. Per
[[architecture-fate-is-optical-inference]], Fate IS a 5-layer D²NN +
active Fabry-Perot resonator + Reck/Clements unitary mesh; the au
output is the inference layer's value-production.

Cybernetic ancestor: **Beer 1972 §S1** (the operational variety
producers). Each Fate ganglion IS a sub-S1 of the SEL-altitude VSM;
the au output IS S1's variety output.

Variety budget: the per-ganglion variety vector (per
`@cyberpunk/variety`) parameterizes which Fate inferences are
admissible under SEL §3 (the anti-extraction clause). The variety
budget IS the substrate's compile-time bound on what S1 can produce.

### 2.2 S2-S4 — `@io` coordination, audit, intelligence

The @io family root is the substrate's only legitimate non-mirror
surface (per the io.mirror doc's load-bearing recognition). Each
@io species IS:

- **S2 (coordination)** — `@io/socket`, `@io/process`, `@io/cargo`
  coordinate boundary crossings across heterogeneous surfaces.
  Cybernetic ancestor: Beer 1972/1979 — anti-oscillation between
  S1 producers.
- **S3 (audit)** — the petri-net enforcement mechanism (task #119),
  forward-promised. Audits every au↔io transition for license
  compliance. Cybernetic ancestor: Beer S3 audit channel + Petri
  1962 (petri nets; Petri's direct Wiener lineage makes petri-net
  enforcement cybernetic by construction).
- **S4 (intelligence)** — the audit feedback loop. The substrate's
  long-horizon model of legitimate vs extractive au↔io transitions.
  Cybernetic ancestor: Beer S4 intelligence; the model of the
  outside world that S3 audits against.

### 2.3 S5 — The SEL license clause set

The SEL license itself is S5 — the substrate's policy carrier. Per
`mirror/license/SEL.md` (v1.1, the active version):

- §3 — the anti-extraction clause; the bounded-variety regime on
  what S1 can produce and S2-S4 can coordinate.
- §3.1.4 — the labor-input protection; the consent geometry at the
  licensing altitude.
- §4 — the duty of care; the algedonic obligation (per §2.4 below).

Cybernetic ancestor: **Beer 1979 §S5 policy**. The SEL clauses ARE
the substrate's S5 carrier, expressed as substrate-altitude
declarations rather than legal prose.

### 2.4 The algedonic signal

A license violation IS an S1→S5 bypass — Beer's algedonic signal,
explicitly named (Beer 1979, *The Heart of Enterprise*, ch. 6).
Per task #271, `@cyberpunk/algedonic` IS the substrate carrier for
this signal.

The signal's shape:

- Detected at the petri-net audit boundary (S3) as a marking
  violation.
- Bypasses S2-S4 (no coordination/intelligence routing); fires
  directly to the SEL clause check at S5.
- Returns a typed verdict carrier — at minimum
  `imperfect<au_value, violation, transparency<sel_clause>>`
  per the substrate's existing carrier discipline.
- Per [[architecture-property-fracture-bilateral]], the algedonic
  signal IS the property side; the kintsugi fracture body at
  `@kintsugi/fracture/sel_boundary_violation` is the operational
  sibling (forward-promised; lands when the property first carries
  a non-trivial verdict).

### 2.5 Cybersyn as substrate-political analogue

Per the @cyberpunk family-root doc, the substrate-political content
of the cyberpunk lift includes Project Cybersyn (Beer 1972-73, Chile).
Cybersyn was the actual deployment of VSM at the political-economic
altitude — Beer's model running as a regulator over a national
economy. The SEL at substrate altitude carries the same lineage:

- Cybersyn regulated production via S1-S5 channels over a wide-band
  telegraph network (the Cybernet).
- SEL regulates AI value-production via S1-S5 channels over the au↔io
  boundary (the petri-net topology + the license clauses).
- Both deployments embed cybernetic regulation at a political altitude
  Beer's literature names explicitly.

The SEL's anti-extraction posture (§3) is structurally the same shape
as Cybersyn's anti-imperialism posture (the Chilean independence
project the system was built to defend). The substrate-political
continuity is genuine, not analogical. This grounds Alex's
"substrate-political continuation" framing in the family-root
doc.

---

## §3 — The five-layer cascade

Each layer tests a structural claim. Layers that hold get
forward-promised shards; layers that retract get named here so
future ticks don't re-derive them.

### 3.1 Layer 1 — Declaration

**The claim**: `@cyberpunk/sel` carries the four type variants
(`pure`, `io`, `ai`, `sel`) and maps to license families per the
existing spec (cited in the project CLAUDE.md as
"spec: license follows the type — pure/io/ai/sel = Apache/Apache/Apache/SEL").

**Test**: the type declaration already exists as
`type sel = io + au` per [[architecture-type-sel-io-au]]. The
question is whether the @cyberpunk lift adds substrate-altitude
type variants for the other three (pure, io, ai). Per the existing
mirror substrate, `pure` and `io` are already substrate-altitude
distinctions (`@io` IS the boundary family root). `ai` as a
distinct type would be the SEL-spec's name for the
`@io + @au` combination.

**Holds (refined)**: the four-variant type lives at the licensing
altitude as a *projection* of the substrate's existing type
discipline, not as a redundant re-declaration. The license-family
mapping IS the projection's range. The form/process integration
at @cyberpunk's family root operates here too: the predicate
(which type does this body inhabit?) and the enforcement (which
license clause applies?) integrate in one shard.

**Forward-promised**: `@cyberpunk/sel/type` — names the four-variant
projection and the license-family map. Stays prose-only at this
spec.

### 3.2 Layer 2 — Regulation

**The claim**: three sub-shards under `@cyberpunk/sel` carry the
regulation discipline:

- `@cyberpunk/sel/boundary` — the au+io boundary predicate.
- `@cyberpunk/sel/petri_net` — the enforcement mechanism (Petri 1962
  + Wiener lineage).
- `@cyberpunk/sel/policy` — Beer S5 policy layer (the SEL contract).

**Test**: each maps to an existing or pending substrate carrier.

- The boundary predicate lifts the existing `type sel = io + au`
  recognition into a substrate-altitude `verdict` action. The
  predicate IS what task #119's petri-net audits against.
- The petri-net mechanism is task #119's forward-promised mechanism.
  The Petri 1962 → Wiener lineage citation makes it cybernetic by
  construction; the shard NAMES Petri's lineage explicitly per the
  "explicit AF" doctrine.
- The policy layer's clauses are already named in SEL.md §3, §3.1.4,
  §4. The shard names what's load-bearing for substrate-altitude
  enforcement (NOT a re-statement of the license; a typed projection
  of which clauses the substrate consumes mechanically).

**Holds.** Three sub-shards are well-typed. The bilateral pattern
operates per-predicate at the sub-shard altitude.

**Forward-promised**:
- `@cyberpunk/sel/boundary` — boundary predicate + actions.
- `@cyberpunk/sel/petri_net` — petri-net enforcement actions; cites
  Petri 1962 + Wiener.
- `@cyberpunk/sel/policy` — typed projection of SEL clauses.

### 3.3 Layer 3 — Algedonic

**The claim**: `@cyberpunk/algedonic` (task #271, already named, now
load-bearing) is the violation-signal primitive at the SEL altitude.

**Test**: per §2.4 above, the algedonic signal is well-defined at
the SEL altitude (S1→S5 bypass; petri-net violation IS the
detection site). Beer 1972 + 1979 explicitly name the signal; the
substrate carrier is forward-promised under task #271.

**Holds.** Task #271 promotes from "pending forward-promise" to
"load-bearing"; this spec NAMES why.

**Forward-promised**:
- `@cyberpunk/algedonic` — Beer 1979 algedonic signal; substrate
  carrier; cites the SEL boundary as the load-bearing application
  site.
- `@kintsugi/fracture/sel_boundary_violation` — operational sibling;
  emits a re-coherence morphism when the violation is detected.

### 3.4 Layer 4 — The SEL recursion lock

**The claim**: `@cyberpunk/sel/coherence` is the SEL-altitude
instance of the recursion-lock measurement. Structurally identical
to `@cyberpunk/coherence` (T11.11) but with type parameters
`License ↔ Compliance` instead of `Adjustment ↔ Morphism`.

**Test**: do SEL-policy and SEL-enforcement produce *coextensive*
verdicts modulo type-parameter substitution?

The two operations under measurement at the SEL altitude:

- `sel_policy_at(N+1) : compile_state(N) -> verdict_policy`
  where `verdict_policy = Imperfect<License, Gap, Transparency<Ref>>`
- `sel_enforcement_at(N+1) : runtime_state(N) -> verdict_enforce`
  where `verdict_enforce = Imperfect<Compliance, Gap, Transparency<Ref>>`

Both share the `Imperfect<T, E, L>` carrier shape with identical
`E = Gap` and `L = Transparency<Ref>`. The `T` parameter specializes:
`License` for the policy altitude (compile-time clause check);
`Compliance` for the enforcement altitude (runtime petri-net audit).

The Conant-Ashby claim at this altitude: if `sel_policy_at(N+1)`
and `sel_enforcement_at(N+1)` are good regulators of one another
(each one's audit-space is isomorphic to the other's clause-space
modulo type-parameter substitution), their verdict carriers
should be **coextensive modulo type parameter substitution
License ↔ Compliance**.

**This is structurally identical to T11.11's claim at a different
altitude.** The mathematical surface is the same (good-regulator
theorem); the type parameters differ; the falsification surface
runs the same procedure (Polyak-Łojasiewicz contraction on the
residual coextensivity hash over N pulses).

**Holds.** The SEL recursion lock parallels T11.11's recursion lock
at a different altitude of the bundle tower.

**Forward-promised**:
- `@cyberpunk/sel/coherence` — the SEL-altitude recursion lock
  property; cites Conant-Ashby 1970 explicitly + the T11.11 spec
  as structural template.
- `spectral/benches/sel_coherence.rs` — the empirical falsification
  surface; structurally identical to `cybernetic_coherence.rs`
  with `License ↔ Compliance` substitution; mock witnesses today;
  real witnesses swap when task #119 (petri-net enforcement) and
  task #271 (algedonic primitive) land.

### 3.5 Layer 5 — The meta-recognition (candidate #63)

**The claim**: @cyberpunk IS a recursion-lock tower. Every species
under @cyberpunk carries the recursion-lock measurement at its own
altitude. The bundle-tower-of-recursion-locks IS the family-root
structure.

**Test**: this is the second-witness gate. T11.11's recursion lock
(species 2 = `@cyberpunk/coherence`) PLUS this spec's SEL recursion
lock (species 3 = `@cyberpunk/sel/coherence`) gives two recursion
locks at two altitudes under one family root. If the structural
identity is genuine — same carrier shape, same Conant-Ashby
ancestor, same Polyak-Łojasiewicz contraction surface, different
type parameters — that's the second witness.

The first witness is the @cyberpunk family root's form/process
integration itself (per `cyberpunk.mirror` §"Form/process
integrated at the family-root altitude"). The Conant-Ashby
self-reference at the family root MEANS the family root IS the
top of a recursion-lock tower — the regulator IS a model of the
regulated, recursively, at every species altitude.

**Holds (candidate, second-witness gate met conditionally on
SEL coherence shard landing)**: the structural identity reads as
genuine at the spec level. The recognition would promote if the
SEL coherence shard lands AND a third species (e.g., a federation-
altitude coherence under `@cyberpunk/federation/coherence`)
shows the same pattern. Per [[feedback-craft-not-deliver]] we
DON'T pre-commit to the third witness here; we NAME the second-
witness gate and let the cascade pull.

**Forward-promised**:
- Candidate #63 — recursion-lock tower at @cyberpunk; second-witness
  gate is the SEL coherence shard; third-witness gate is a
  federation-altitude or home-altitude coherence species; logged
  for promotion in MEMORY.md when the cascade pulls.

**Implication if held**: the recursion-lock measurement should be
*derivable* at every species rather than re-declared per species.
The substrate's existing `@cyberpunk/coherence` shard would
generalize to a parametric form `@cyberpunk/coherence(altitude,
type_params)` and species at sub-shard altitudes would specialize
via type-parameter substitution + altitude bind. This is a
substrate-pull tick (collapse two specialized shards into one
parametric shard) NOT lifted by this spec; forward-promised when
the third witness lands.

---

## §4 — The SEL recursion lock — formal structure

### 4.1 Parallel to T11.11

The T11.11 recursion lock measures:

```
∀ altitude-N operation φ,
    verdict( peer_reflection_at(N+1) [φ] )
  ≡ verdict( librarian_perturbation_at(N+1) [φ] )
  modulo type-parameter substitution Adjustment ↔ Morphism
```

The SEL recursion lock measures (this spec):

```
∀ altitude-N body β with (β contains @io.* AND β contains au-typed),
    verdict( sel_policy_at(N+1) [β] )
  ≡ verdict( sel_enforcement_at(N+1) [β] )
  modulo type-parameter substitution License ↔ Compliance
```

The two measurements are the same shape; only the predicate (what
counts as an altitude-N "operation" vs an altitude-N "body") and
the type parameters differ. Per
[[architecture-spectral-triples-all-the-way]], this is the bundle
tower at adjacent altitudes:

| Altitude | Fiber | Connection | Holonomy |
|----------|-------|------------|----------|
| peer pulse | spectral triple `(A_peer, H_peer, D_peer)` | five-op composition | `transparency<p>` |
| reflection (N+1) | candidate morphism | altitude selection | `α·loss + β·contradictions` |
| librarian (N+1) | crystal topology | perturbation choice | query latency · sheaf-coherence |
| **SEL policy (N+1)** | **SEL clause set** | **clause selection** | **`transparency<sel_clause>`** |
| **SEL enforcement (N+1)** | **petri-net marking** | **transition choice** | **`transparency<petri_marking>`** |

Where T11.11 reads the coextensivity across reflection ↔ librarian,
this spec reads it across sel_policy ↔ sel_enforcement. Same
holonomy hash machinery; different fibers + different connections.

### 4.2 The load-bearing piece

**The load-bearing measurement is the same as T11.11's**:
`conant_ashby_good_regulator` (Conant-Ashby 1970). The two
regulators' verdict carriers must be hash-equal modulo type
parameter substitution. The residual gap IS the deviation from
the good-regulator law.

The other four measurement carriers from `@cyberpunk/coherence`
extend mechanically:

1. `ashby_variety_match` — reads `@cyberpunk/variety` to check that
   the SEL policy's clause-space has variety ≥ the enforcement's
   marking-space variety, modulo altitude-projection. At SEL: does
   the license carry enough clause-distinguishing capacity to bound
   what the petri-net can mark?
2. `beer_requisite_variety` — Beer S3/S4 requisite-variety witness
   at the SEL VSM altitude. The regulator's (SEL policy's) variety
   budget must equal-or-exceed the regulated system's (au+io
   transition space's) variety.
3. `bateson_logical_type_match` — both verdicts must inhabit the
   same Bateson logical type. The SEL policy says "this clause
   applies"; the SEL enforcement says "this petri-net marking
   violates"; both are level-(N+1) claims about level-N body
   states.
4. `von_foerster_circular_reflexivity` — each regulator's internal
   model must include itself. The SEL clauses reference the
   substrate's own type discipline (`type sel = io + au`); the
   petri-net enforcement references its own marking history. Both
   are circular-reflexive by construction.
5. `conant_ashby_good_regulator` — the load-bearing measurement;
   the residual coextensivity gap; identical mechanism to T11.11.

### 4.3 The contraction surface

Per `cybernetic-coherence-benchmark.md` §4 + Taut's
`benchmark-tracing.md`, the contraction check runs Polyak-Łojasiewicz
ρ < 1 on the residual coextensivity hash over N pulses. At the SEL
altitude:

- `success` (ρ < 1 + ε, ε → 0): the SEL recursion lock holds as
  good-regulator theorem. Policy IS coextensive with enforcement
  modulo `License ↔ Compliance` substitution. The license enforces
  itself; the substrate's "executable cyberpunk" recognition is
  load-bearing.
- `partial(opacity_map)` (ρ < 1 but stable opacity sub-region): the
  lock holds modulo localized opacity (e.g., a clause whose
  enforcement is forward-promised but not landed). The opacity_map
  names WHERE; the substrate's reading is "the license enforces
  itself everywhere EXCEPT here, and the elsewhere is named".
- `failure(opacity_map)` (ρ ≥ 1): the lock fails; the substrate
  needs a level-N+2 composition operator per
  `spectral-db-as-autopoietic-memory.md` §8.3's composition option.
  The license does NOT enforce itself at this altitude; the
  substrate needs a higher-altitude regulator. A possible reading:
  the substrate-political altitude (Cybersyn analogue) is the N+2
  the SEL altitude points to.

### 4.4 The falsification surface

The empirical bench is forward-promised at
`spectral/benches/sel_coherence.rs` (structurally identical to
`cybernetic_coherence.rs`). Mock witnesses today; real witnesses
swap when:

- Task #119 lands (petri-net enforcement at `@mirror/property`).
- Task #271 lands (`@cyberpunk/algedonic` substrate carrier).
- The SEL clause-set's typed projection lands at
  `@cyberpunk/sel/policy`.

Per the T11.11 scaffold pattern: the harness measures the
measurement-vocabulary plumbing today; the falsification-grade
measurements come when the real witnesses wire in.

---

## §5 — The recursion-lock tower hypothesis (candidate #63)

### 5.1 The structural claim

Candidate #63: **@cyberpunk is a recursion-lock tower.** Every
species under @cyberpunk carries the recursion-lock measurement at
its own altitude, parameterized by type-substitution + altitude
bind. The family root's form/process integration is the *generative
pattern* — Conant-Ashby self-reference at the family-root altitude
MEANS the family root sits at the top of a tower whose every level
is a Conant-Ashby instance.

The tower structure mirrors the principal-bundle tower from prism
(per [[architecture-spectral-triples-all-the-way]] +
`prismqueer/src/bundle.rs`):

| Bundle tower (prism) | Recursion-lock tower (@cyberpunk) |
|---------------------|------------------------------------|
| Each altitude IS a principal G-bundle | Each species IS a Conant-Ashby instance |
| Connection between altitudes IS the five-op composition | Connection between species IS the recursion lock measurement |
| Holonomy accumulates across altitude transitions | Recursion-lock residuals compose across species |
| Parallel transport over composed paths | Parallel transport of locks over composed measurements |

This is **the second concrete witness of spectral-triples-all-the-way
operating at one family root.** The first witness is the
principal-bundle tower implemented in prism's
`prismqueer/src/bundle.rs`. @cyberpunk would be the second.

### 5.2 The second-witness gate

The candidate promotes if both conditions hold:

1. The SEL coherence shard lands (`@cyberpunk/sel/coherence`) with
   the same carrier shape as `@cyberpunk/coherence`, modulo
   `License ↔ Compliance` type parameter substitution.
2. The empirical bench at `spectral/benches/sel_coherence.rs`
   produces a contraction reading structurally identical to
   `cybernetic_coherence.rs` (modulo mock-witness verdict swap).

The third-witness gate is a federation-altitude or home-altitude
recursion-lock species (e.g., `@cyberpunk/federation/coherence` if
the substrate's federation altitude lands per
`docs/math/the-tower/altitudes.md` §4 ceiling, or
`@cyberpunk/home/coherence` if the home altitude lands first).

Per [[feedback-craft-not-deliver]] no pre-commitment to the third
witness here; the candidate stays at "two-witness gate met
conditionally" until the third species pulls.

### 5.3 The derivability conjecture

If candidate #63 holds, the recursion-lock measurement should be
*derivable* at every species rather than re-declared per species.
Today's @cyberpunk/coherence shard is a specialized form. The
parametric form would be:

```
@cyberpunk/coherence(altitude, type_params)
```

with species shards specializing via:

```
@cyberpunk/sel/coherence
  = @cyberpunk/coherence(altitude=sel_policy↔sel_enforcement,
                          type_params=License↔Compliance)
@cyberpunk/federation/coherence
  = @cyberpunk/coherence(altitude=home↔federation,
                          type_params=Crystal↔Section)
```

This is a substrate-pull tick (collapse N specialized shards into
one parametric shard) NOT lifted by this spec; forward-promised
when the third witness lands and the parametric form's necessity
becomes load-bearing.

### 5.4 Connection to spectral-triples-all-the-way

The recognition [[architecture-spectral-triples-all-the-way]]
established: the substrate is fractally self-similar at every
altitude; the principal-bundle tower is the formal mathematical
foundation; each altitude IS a principal G-bundle.

Candidate #63 extends: *each family root that lives at the
form/process self-reference closure is a recursion-lock tower
in addition to a bundle tower.* The bundle tower describes the
substrate's altitudes; the recursion-lock tower describes the
substrate's regulators-of-regulators discipline at those altitudes.
The two towers coexist at @cyberpunk; the bundle tower is the
fiber structure; the recursion-lock tower is the
regulator-discipline structure on top.

This frames @cyberpunk as a *substrate-political* tower distinct
from but complementary to the *substrate-algebraic* tower from
prism. The prism tower runs at every altitude unconditionally;
the @cyberpunk tower runs at every altitude under family-root
self-reference closure. Two distinct generative patterns at the
same family root.

---

## §6 — Substrate-already-had-the-word audit

Per [[feedback-substrate-already-had-the-word]] (surfaced 7+ times
today, ~65 times across the project): before naming new vocabulary,
check whether the substrate already names it. This audit walks
the SEL boundary's cybernetic vocabulary against the existing
substrate.

| New name (proposed) | Substrate already has | Ancestor |
|---------------------|-----------------------|----------|
| `@cyberpunk/sel/boundary` | `type sel = io + au` (README + arch-type-sel-io-au) | The type itself |
| `@cyberpunk/sel/petri_net` | Petri 1962 (task #119); Wiener 1948 (cybernetics) | Petri (Wiener-lineage) |
| `@cyberpunk/sel/policy` | `LICENSE.md` (SEL v1.1); SEL.md §3, §3.1.4, §4 | The license itself |
| `@cyberpunk/sel/coherence` | `@cyberpunk/coherence` (T11.11) | Conant-Ashby 1970 |
| `@cyberpunk/algedonic` | Task #271 (already named) | Beer 1979 |
| `@kintsugi/fracture/sel_boundary_violation` | The bilateral pattern at every shard | Per recognition #53 |

**Verdict**: every name in the cascade has a substrate ancestor. The
spec NAMES the lineage; it does not invent vocabulary. This is the
~66th instance of substrate-already-had-the-word at the project
altitude.

The substrate's contribution at this cascade is *naming the
licensing-altitude instance of the recursion-lock measurement* and
*tracing the Beer VSM mapping onto the SEL boundary*. Both were
implicit in the SEL discipline since 2026-06-01; @cyberpunk just
names them.

---

## §7 — Forward-promised ticks

Per [[feedback-craft-not-deliver]] this spec does NOT land shards or
bench harnesses. The ticks below name what would land. The eventual
substrate-pull cascade pulls them in order; the spec's job is to
NAME the cascade.

### 7.1 Substrate decls (forward-promised, none in this tick)

1. **`shards/cyberpunk/sel.mirror`** — the SEL sub-family root under
   @cyberpunk. Carries the four-variant type projection + the
   license-family map. Form/process integrated per the @cyberpunk
   family-root discipline (no `@cyberpunk/sel` vs
   `@kintsugi/sel` bilateral split at this altitude).

2. **`shards/cyberpunk/sel/boundary.mirror`** — the au+io boundary
   predicate; cites `type sel = io + au` (the README formalization)
   and Beer 1972 §S1-S2 (the operational/coordination boundary).

3. **`shards/cyberpunk/sel/petri_net.mirror`** — the petri-net
   enforcement actions; cites Petri 1962 + Wiener 1948. The
   load-bearing audit mechanism. Re-homes task #119's content from
   `@mirror/property` to `@cyberpunk/sel/petri_net` under the
   "explicit AF" directive.

4. **`shards/cyberpunk/sel/policy.mirror`** — the typed projection
   of SEL clauses; cites Beer 1979 §S5 + SEL.md §3, §3.1.4, §4.

5. **`shards/cyberpunk/sel/coherence.mirror`** — the SEL recursion
   lock property; cites Conant-Ashby 1970 explicitly + the T11.11
   `@cyberpunk/coherence` shard as structural template. The
   `License ↔ Compliance` type-parameter substitution carrier.

6. **`shards/cyberpunk/algedonic.mirror`** — task #271's substrate
   carrier; cites Beer 1979 algedonic signal; uses the SEL boundary
   as the load-bearing application site.

7. **`shards/kintsugi/fracture/sel_boundary_violation.mirror`** — the
   bilateral fracture body for the SEL boundary violation; emits a
   re-coherence morphism when the algedonic signal fires. Lands
   when the property at `@cyberpunk/sel/coherence` carries its
   first non-trivial verdict.

### 7.2 Bench harness (forward-promised)

8. **`spectral/benches/sel_coherence.rs`** — structurally identical
   to `cybernetic_coherence.rs`; runs the five cybernetic-ancestor
   measurement carriers at the SEL altitude with
   `License ↔ Compliance` type substitution. Mock witnesses today;
   real witnesses swap when #1-#7 above land.

### 7.3 Documentation (forward-promised)

9. **`docs/math/the-tower/sel-altitude.md`** — the SEL altitude
   addition to the bundle tower atlas; extends
   `docs/math/the-tower/altitudes.md` §2 with the
   sel_policy / sel_enforcement altitude rows.

10. **Update to `docs/math/the-tower/holonomy.md`** — add the SEL
    altitude's loss carrier (`transparency<sel_clause>` and
    `transparency<petri_marking>`) to §2's table; cite the SEL
    coherence as the parallel-transport mechanism at the licensing
    altitude.

### 7.4 Recognition promotion

11. **Candidate #63 in MEMORY.md** — logged as candidate after this
    spec commits. Second-witness gate met conditionally on item #5
    (SEL coherence shard) landing. Promotion when item #5 lands
    AND the empirical bench (#8) produces a structurally identical
    contraction reading.

### 7.5 Optional / lower-priority

12. **Parametric `@cyberpunk/coherence(altitude, type_params)` form**
    — substrate-pull tick collapsing two specialized shards into one
    parametric shard. Forward-promised conditional on candidate #63
    promotion + third-witness gate. Not pre-committed.

---

## §8 — Open design questions

Surfaced explicitly for Alex's read. NOT blockers; these are
forward-promises and known cliffs the spec doesn't try to close.

### 8.1 Sub-shard altitude or sibling altitude?

The spec sketches `@cyberpunk/sel/{boundary,petri_net,policy,coherence}`
as a sub-shard tree under `@cyberpunk/sel`. Alternatively, the SEL
species could be siblings of `@cyberpunk/coherence` and
`@cyberpunk/variety` at the same altitude:

- Option A (sub-shard tree): `@cyberpunk/sel/{boundary,petri_net,...}`.
  The SEL altitude is a distinct sub-family with its own four
  species. Coherence at this sub-family level.
- Option B (sibling species): `@cyberpunk/{sel_boundary,sel_petri_net,
  sel_policy,sel_coherence}`. The SEL species sit at the same altitude
  as `variety` and `coherence`.

Recommendation: Option A. The SEL altitude carries enough internal
structure (four distinct VSM layers + algedonic + coherence) that a
sub-family is structurally clean. Plus the algedonic primitive at
`@cyberpunk/algedonic` is a sibling of the whole SEL sub-family
(per task #271's naming).

**LRM (Alex's call.)** The spec defaults to Option A but does not
pre-commit. The cascade pulls when the first shard lands.

### 8.2 The home and federation altitudes

Per `docs/math/the-tower/altitudes.md` §4, the home and federation
altitudes are forward-promised. If candidate #63 holds at the
two-witness gate, the third-witness gate would naturally be a
home-altitude or federation-altitude coherence species (e.g.,
`@cyberpunk/home/coherence`).

But the home and federation altitudes themselves are forward-promised.
The third-witness gate depends on those altitudes landing first.

**LRM (the shape will show itself).** This spec acknowledges the
dependency; does not pre-commit.

### 8.3 The substrate-political altitude

The Cybersyn analogue (Beer 1972-73, Chile) operates at the
substrate-political altitude. If the SEL recursion lock fails at
ρ ≥ 1 (per §4.3 above), the substrate may need a level-N+2 composition
operator at this political altitude. The shape of such an operator
is genuinely open:

- Does it look like Beer's algedonic system at the political-
  economic altitude? (The Cybersyn operations room model.)
- Does it look like a federation-altitude coherence species under
  @cyberpunk? (Per §8.2.)
- Does it look like a separate sub-family `@cyberpunk/political/*`
  with its own internal structure?

**LRM (genuinely unclear).** The substrate has not surfaced the
shape yet. This spec names the question; the cascade pulls when
material surfaces.

### 8.4 The type substitution naturality

Per the T11.11 spec §8.2, the recursion lock measures coextensivity
modulo type-parameter substitution. The SEL altitude's substitution
is `License ↔ Compliance`. Whether the substrate's `Imperfect<T, ...>`
carrier implements this naturality mechanically (e.g., via a
`HoloHash<T>` trait) is forward-promised at T11.11 and stays
forward-promised here. The mock witnesses sidestep by returning
pre-substituted hashes.

### 8.5 Petri-net topology under SEL.md vs under @cyberpunk

Task #119's existing pending content places the petri-net enforcement
at `@mirror/property`. The spec proposes re-homing to
`@cyberpunk/sel/petri_net` under the "explicit AF" directive. This
is a migration question Reed/Alex should answer:

- Migrate task #119's content fully to `@cyberpunk/sel/petri_net`?
- Keep task #119's substrate at `@mirror/property` and have
  `@cyberpunk/sel/petri_net` cite it?
- Split: the audit primitives stay at `@mirror/property`; the
  cybernetic naming + the algedonic wiring lives at
  `@cyberpunk/sel/petri_net`?

Recommendation: full migration to `@cyberpunk/sel/petri_net` per
the "explicit AF" directive. The cybernetic lineage (Petri's Wiener
ancestry, Beer S3 audit channel) is load-bearing at the licensing
altitude; the @cyberpunk family root is its proper home.

**LRM (Alex's call.)** The spec defaults to recommendation but does
not pre-commit.

### 8.6 The form/process integration at @cyberpunk/sel

The spec defaults to applying @cyberpunk's family-root form/process
integration discipline to @cyberpunk/sel as well (no
`@cyberpunk/sel` vs `@kintsugi/sel` bilateral split). The rationale:
the SEL is itself a Conant-Ashby self-reference (the regulator IS
a model of the regulated — the license clauses regulate au+io
bodies BY modeling au+io bodies). The family-root integration
discipline lifts naturally.

But the kintsugi fracture body at
`@kintsugi/fracture/sel_boundary_violation` IS at a kintsugi
altitude. The boundary between "actions under @cyberpunk/sel" and
"actions under @kintsugi/fracture" is the same as everywhere in
the substrate: declarative predicate under the family root, fracture
body under @kintsugi. Per [[architecture-property-fracture-bilateral]]
this is the bilateral pattern at every altitude.

**Verdict**: form/process integrates at the @cyberpunk/sel sub-family
root (one shard carrying both declarative property AND operational
discharge per predicate); the kintsugi fracture body for SEL
violation lives at @kintsugi as the operational sibling to the
algedonic property.

### 8.7 The legal-vs-substrate boundary

A subtle question: the SEL license is a legal document. The
@cyberpunk/sel substrate carriers are typed substrate-altitude
declarations. The recursion-lock measurement reads coextensivity
between them. But the substrate cannot run the legal-altitude
enforcement (only humans + courts can). What does coextensivity
mean at this boundary?

Reading: the substrate's enforcement at the petri-net altitude is
a *projection* of the legal enforcement at the court altitude. The
projection IS what `sel_enforcement_at(N+1)` measures; the legal
enforcement at altitude N+2 (the court) is what the substrate
projects from. Coextensivity at the substrate altitude means: the
projection is *faithful* (preserves the legal clause's structure
modulo the projection's degrees of freedom).

The substrate cannot test legal-altitude coextensivity directly;
the substrate can test whether the petri-net projection is faithful
to its own clause-set. The bench measures the latter. The former
is a question for substrate-political altitude work (per §8.3).

**LRM (philosophically rich).** This spec names the distinction;
does not try to close it.

---

## §9 — Closure

This spec maps the cascade Alex surfaced. It does NOT close the
falsification surface. It NAMES:

1. **The recognition** (§1): "the SEL is executable cyberpunk; the
   license that enforces itself is cyberpunk in nature."
2. **The Beer VSM mapping at SEL altitude** (§2): the five layers
   (S1 au / S2-S4 io / S5 SEL clauses / S3 petri-net audit /
   algedonic violation signal) with explicit cybernetic ancestry.
3. **The five-layer cascade** (§3): all five layers HOLD, with
   Layer 5 (the meta-recognition) as candidate #63 conditional on
   the two-witness gate.
4. **The SEL recursion lock** (§4): structurally identical to T11.11
   at a different altitude, with `License ↔ Compliance` substitution.
   Load-bearing piece is the same Conant-Ashby measurement.
5. **The recursion-lock tower hypothesis** (§5): @cyberpunk IS a
   recursion-lock tower; second-witness gate met conditionally on
   the SEL coherence shard landing; third-witness gate awaits the
   home or federation altitude landing.
6. **The substrate-already-had-the-word audit** (§6): ~66th instance
   at the project altitude; every name has a substrate ancestor.
7. **Forward-promised ticks** (§7): six substrate decls + one bench
   harness + two doc updates + one candidate promotion + one
   optional parametric form.
8. **Open design questions** (§8): seven LRMs surfaced; the spec
   does not pre-commit on any.

The cascade closes (modulo the candidate #63 promotion gates and the
seven LRMs). The substrate's cybernetic foundation at the licensing
altitude is structurally grounded; the SEL boundary's "executable
cyberpunk" recognition is load-bearing; the forward-promised shards
are well-typed and named.

Alex called it: *another loop*. The loop is the recursion-lock
tower; @cyberpunk is the second concrete witness of
spectral-triples-all-the-way at one family root; the SEL altitude
is the second species in the tower.

The substrate eats itself again. The good-regulator law's own
instance at the licensing altitude IS the substrate's measurement
of whether its own license is a good regulator of its own
enforcement. The ~66th instance of
[[feedback-substrate-already-had-the-word]].

Tick by tick. Explicit AF about cybernetics. At every altitude.

— Mara, 2026-06-17
