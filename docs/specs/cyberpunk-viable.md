# @cyberpunk/viable — recursion-lock audit

*2026-06-17. Mara. Spec — tick 2 of the `/loop @cyberpunk recursion-lock
tower audit`. Audits Beer's viability species against the parametric
form named in `docs/math/the-tower/recursion-locks.md` §2. The third-
witness gate for candidate #63 (whether @cyberpunk IS a recursion-lock
tower) turns on whether viability — operating at a structurally
different altitude than coherence (T11.11) and SEL (Mara `7807a77`) —
carries the same parametric form.*

Status: **Audit tick.** No shard declarations land in this spec. No
Rust. No license edits. This spec NAMES the question, tests three
readings (homeostat / recursive viability / form refusal) against the
parametric form, applies the five cybernetic-ancestor measurements
to each read, verdicts the species, and forward-promises whatever
shards / bench scaffolds / math doc updates the verdict implies. Per
[[feedback-craft-not-deliver]] this is an audit tick; the operational
ticks pull from this spec when the cascade pulls.

Promoted from the math doc Reed landed at the start of `/loop` tick 1
(commit `cf62da3` on `reed/recursion-lock-tower-audit`,
`docs/math/the-tower/recursion-locks.md`, 341 lines):

> *"Every species `S` under `F` carries a recursion-lock measurement
> at the altitude `S` names. The measurement IS the parametric instance
> of `F`'s foundational theorem at the species' altitude. The family
> root's structure IS the bundle tower whose fibers are the per-species
> locks."*  
> — recursion-locks.md §1

And the third-witness gate, named in §7:

> *"A species at a structurally-different altitude (home or federation
> per `altitudes.md` §4) satisfies (1)–(5)."*  
> — recursion-locks.md §7

Reed has §8.1 (coherence template) and §8.2 (SEL second witness) landed
on `cf62da3`. This spec writes §8.3 (viable) as a candidate for the
third-witness gate. Reed will integrate the verdict in the next loop
tick.

Depends on:

- `mirror/docs/math/the-tower/recursion-locks.md` (Reed `cf62da3`, on
  `reed/recursion-lock-tower-audit`; the parametric form this audit
  tests against; especially §2 the form, §3 the five measurements,
  §6 the test surface, §9 promotion / retraction paths).
- `mirror/docs/math/the-tower/altitudes.md` §2-§5 (the named altitudes
  + composition between altitudes; the "structurally different
  altitude" reading for the third-witness gate).
- `mirror/docs/math/the-tower/holonomy.md` §5, §8 (the `Imperfect`
  verdict family as holonomy components; the carrier the lock verdict
  uses).
- `mirror/shards/cyberpunk.mirror` (Reed `f629216`, 123 lines, on
  `taut/t11-11-cybernetic-coherence-benchmark`; the family root that
  forward-promises `@cyberpunk/viable`; lists viable as the next
  species after coherence + variety).
- `mirror/shards/cyberpunk/coherence.mirror` (Reed `ccc227d` migrated
  by `f629216`; the structural template — type parameters
  `Adjustment ↔ Morphism`; the parametric form's first instance).
- `mirror/docs/specs/cybernetic-coherence-benchmark.md` (Taut
  `b66058d`; the T11.11 empirical surface; the bench harness shape
  this spec's audit reads against).
- `mirror/docs/specs/sel-as-executable-cyberpunk.md` (Mara `7807a77`,
  1004 lines; the second-witness spec; the discipline this audit
  mirrors).
- `mirror/shards/epistemologic/cybernetic/variety.mirror` (the
  first-landed species under the cybernetic family; the substrate
  vocabulary check viable must clear).
- Beer 1972, *Brain of the Firm*, Wiley — the VSM introduction;
  S1-S5 structure; the original "viable system" naming.
- Beer 1979, *The Heart of Enterprise*, Wiley — the VSM elaboration;
  the recursive viability principle; S3 audit channel; the
  algedonic signal (ch. 6).
- Beer 1984, "The Viable System Model: Its Provenance, Development,
  Methodology and Pathology", *Journal of the Operational Research
  Society* 35(1): 7-25 — the methodology paper Beer wrote a decade
  in; the most precise statement of the viability law.
- Conant-Ashby 1970, "Every Good Regulator of a System Must Be a
  Model of That System", *International Journal of Systems Science*
  1(2): 89-97 — the foundational theorem the parametric form
  instantiates at every altitude.
- Project Cybersyn (Chile, 1971-1973) — VSM deployed at the
  political-economic altitude; the substrate-political precedent
  the SEL spec lifted as analogue.

Substrate decisions cited:

- [[architecture-cybernetic-coherence-active]] (2026-06-17) — the
  promotion event and @cyberpunk migration; this spec extends the
  family root one species deeper.
- [[architecture-cybernetic-foundation]] (2026-06-09) — viability
  IS property #3 of the 11-property canonical family; the substrate's
  three-tier stack is *already* VSM-conformant per §6 of the
  foundation doc; this spec lifts that recognition to substrate-
  altitude declaration.
- [[architecture-spectral-triples-all-the-way]] (2026-06-17) — the
  generative pattern; @cyberpunk as second concrete witness; the
  third-witness gate this audit is testing against.
- [[architecture-bateson-logical-type-primitive]] — viable's logical-
  type structure (a viable system that contains viable systems is
  a level-3 claim).
- [[architecture-property-fracture-bilateral]] — the bilateral
  pattern operates per-predicate; viable's enforcement side is
  forward-promised under the @kintsugi fracture vocabulary.
- [[architecture-form-process-partition-at-family-root]] —
  recognition #61; @cyberpunk integrates form/process at the
  family-root altitude; this audit tests whether viable inherits
  the integration.
- [[architecture-mirror-as-expanding-hilbert-space]] — recognition
  #51; viable's measurement expands `H_viable` with each
  identity-preservation event.
- [[feedback-substrate-already-had-the-word]] — the substrate has
  been *running* a viable system (the three-tier stack) since the
  cybernetic-foundation lift on 2026-06-09; this spec is the ~67th
  instance — naming what's already there.
- [[feedback-no-bare-types]] — every measurement carrier in the
  forward-promised shards is a typed newtype.
- [[feedback-craft-not-deliver]] — audit tick; the shards are
  forward-promised, not landed; the spec NAMES what would land.
- [[feedback-conversation-not-pipeline]] — verdict comes back to
  Alex for mutual agreement before any shard lands; no
  pre-commitment past spec.

---

## §1 — The audit question

Does Beer's *viability* species — the "identity-preservation under
environmental disturbance" species at the substrate-altitude lift of
the Viable System Model — instantiate the recursion-lock parametric
form named in `docs/math/the-tower/recursion-locks.md` §2?

The parametric form (verbatim from §2 of recursion-locks.md):

```
S = ( regulator_at(α+1)
    , regulated_at(α+1)
    , T_reg
    , T_regd
    , τ : T_reg ↔ T_regd
    )
```

with lock verdict at altitude `α+1`:

```
lock_verdict(α) : Imperfect< T_lock , Gap , Transparency<Ref> >
```

The audit asks five sub-questions per recursion-locks.md §6 test
surface:

1. Two parallel-altitude operations at altitude α+1, with a
   regulator/regulated relation.
2. Two type-parameter carriers `T_reg`, `T_regd` on the verdicts.
3. A natural substitution `τ : T_reg ↔ T_regd`.
4. A coextensivity reading: comparing the two verdicts modulo τ.
5. A contraction surface: a bench-or-equivalent exhibiting
   Polyak-Łojasiewicz contraction (or failure).

A species fits the form iff (1)–(5) can be named at its altitude.
A species refuses the form iff (1) or (2) cannot be named. A
species weakens the form iff (3) needs a non-natural substitution.
A species is empirically open iff (4) or (5) are well-typed but
the bench has not yet measured.

The third-witness gate: viable must operate at a structurally
*different* altitude than coherence (T11.11) and SEL (Mara
`7807a77`). Per altitudes.md §4 (home / federation altitudes
preferred), but the math doc accepts any genuinely different
altitude. §3.5 below argues viable's altitude — *identity-
preservation under environmental disturbance* — is structurally
distinct from both predecessors.

This spec defaults to one of three readings (Reading A, B, C below)
and verdicts. A fourth Reading D surfaces in §4.5 because the
substrate pulled toward it during the audit.

---

## §2 — What `viable` is

### 2.1 Beer's literature

Beer 1972, *Brain of the Firm*, introduces the Viable System Model
(VSM) as the recursive structure of any system that maintains its
identity through environmental disturbance. The five subsystems:

- **S1 — Operations.** The variety producers; the parts that DO
  things. Each S1 is itself a complete viable system (recursion).
- **S2 — Coordination.** Anti-oscillation between S1s; damps
  conflict via shared protocol.
- **S3 — Audit.** Sees the operational reality of the S1s; the
  here-and-now management channel.
- **S4 — Intelligence.** Models the *outside* world the system
  inhabits; the elsewhere-and-elsewhen channel. S3 + S4 are in
  Eckhart relation; oscillation between them is *normal* and
  *load-bearing*.
- **S5 — Policy.** Holds the system's identity. Resolves S3 vs S4
  conflict by appeal to what the system *is*.

Plus:

- **Algedonic signal.** Beer 1979 ch. 6. An S1→S5 emergency
  bypass: if an S1 detects a threat to system identity that the
  normal channels would damp out, the algedonic channel fires
  directly to S5.

The viability law (Beer 1972 ch. 9; sharpened in Beer 1984 §3):

> *"A system is viable if and only if it can maintain a separate
> existence in the kind of environment it has to face. The model
> derives strictly from the cybernetic theory of viability — it is
> the necessary and sufficient condition for a system to remain
> itself in the face of perturbation."*

"Remain itself" — that's the identity-preservation reading. Viable
is the *predicate* a system satisfies iff the VSM's S1-S5
structure (plus algedonic) actually maintains the system's
identity through disturbance.

### 2.2 Recursive viability

Beer 1972, ch. 10: **every S1 is itself a viable system**. The
recursion is structural. A national economy as VSM has firms as
S1; each firm has departments as S1; each department has people
as S1; each person... A viable system is one whose every component
is a viable system *whose every component is a viable system*. The
recursion descends until it bottoms out at primitive elements (or
doesn't — Beer didn't insist on a floor).

This recursion is the load-bearing piece for the third-witness
audit. The parametric form's `regulator_at(α+1)` over
`regulated_at(α+1)` reads naturally as "the parent viable system's
S5 regulating the child viable system" — the recursion is *in*
the model, not added to it.

### 2.3 The substrate's existing VSM-conformance

Per [[architecture-cybernetic-foundation]] §6, the substrate's
three-tier stack ALREADY maps to a VSM at the substrate-altitude:

- **S1 (operations)** = fragmentation-mcp
- **S2 / S3 (coordination / management)** = mirror +
  SpectralSupervisor
- **S4 (intelligence / strategy)** = @spectral/db
- **S5 (identity / policy)** = the Pack identity layer
  (Reed/Mara/Glint/Taut/Seam)

The substrate has been *running* as a viable system at the
substrate-altitude since the three-tier stack architecture
landed. This audit doesn't introduce viability; it names what
the substrate has already been doing.

This is the ~67th instance of
[[feedback-substrate-already-had-the-word]].

### 2.4 Cybersyn as substrate-political analogue

Per the @cyberpunk family-root doc and Mara's SEL spec §2.5,
Project Cybersyn (Chile, 1971-1973) was the actual deployment of
VSM at the political-economic altitude. The Cybernet telegraph
network routed S1-S5 channels over a national economy; the
system was built to defend Chilean independence against external
disturbance (the "environment it has to face" was geopolitical
extraction).

Cybersyn's viability law: the Chilean economy IS a viable system
iff the S1-S5 channels can maintain Chilean economic identity
through external disturbance. The system failed in 1973 not
because the VSM theory failed but because the disturbance
exceeded the system's variety (Pinochet's coup; the regulatory
budget was overwhelmed by external violence). The substrate-
political continuity to SEL (Mara `7807a77` §2.5) is: the
substrate's anti-extraction posture is structurally the same
shape as Cybersyn's anti-imperialism posture; the *viability*
species is what both deployments are protecting.

### 2.5 What altitude does viable operate at?

This question matters for the third-witness gate (§3.5).

**Coherence (T11.11)** operates at the *parallel-operation-pair*
altitude: peer reflection at N+1 vs librarian perturbation at
N+1. Two parallel operations at the same N+1. The lock asks
whether the two operations agree modulo type-substitution.

**SEL (Mara `7807a77`)** operates at the *license-boundary*
altitude: SEL policy at N+1 vs SEL enforcement at N+1. Two
parallel operations at the licensing N+1. Same parallel-pair
structure as coherence, different domain.

**Viable** operates at the *identity-preservation-under-
disturbance* altitude. The structure is different: the lock isn't
asking whether two parallel operations agree; it's asking whether
the system's regulation *succeeds at preserving identity through
time as the environment changes*. This is a *temporal* lock —
the parallel pair is the system-at-time-t and the
system-at-time-t+Δt, NOT two simultaneous operations.

That altitude difference is structurally distinct from both
predecessors. It's not "home" or "federation" per altitudes.md §4
strictly — those altitudes are about scope hierarchy (single repo
→ home → federation). Viable operates at the *temporal-
preservation* axis, which crosses every scope altitude.

Whether this counts as "structurally different" for the third-
witness gate is a question for Alex (§9 Q1). The audit reads it
as yes — but the math doc's §7 framing of "structurally-different
altitude (home or federation preferred)" doesn't explicitly
admit the temporal axis. Could be a case where the math doc's
language needs refinement.

---

## §3 — Three reads

Each read tests the parametric form against viable. The reads
are not mutually exclusive in principle; they bind on different
naming choices for `regulator_at(α+1)` / `regulated_at(α+1)`.
The audit asks which read holds *under viable's natural
substrate-reading*.

### 3.1 Read A — Homeostat

The simplest read: a viable system is a homeostat (Ashby 1952,
*Design for a Brain*) — a multi-unit regulator that maintains
essential variables within survival bounds through ultrastable
random reorganisation.

The parametric instantiation under Read A:

- `regulator_at(α+1)` = the system's S5 *policy*; holds
  identity-constraints (the survival-bound predicate).
- `regulated_at(α+1)` = the S1-S4 *operational layers*; the
  units doing variety-production under disturbance.
- `T_reg` = `Identity` — the substrate-altitude carrier for
  *what the system is* (the per-shard identity vector, the
  Pack-altitude identity carrier, or the SpectralUuid at the
  spectral-coordinate altitude; identity = eigenform per
  [[architecture-cybernetic-foundation]] property #9).
- `T_regd` = `Stability` — the substrate-altitude carrier for
  *whether the system maintains identity under disturbance*
  (a transparency-shaped verdict reading `success` /
  `partial(opacity_map)` / `failure(opacity_map)` per the
  holonomy.md §5 verdict family).
- `τ : Identity ↔ Stability` — the natural substitution between
  identity-as-policy and stability-as-operational-fact. A system
  with identity `i` has stability `s = stability_of(i)` under
  the homeostat reading; the substitution is read off by
  *running the system* through a disturbance.

Reading A's structural claim: stability IS what identity-
preservation MEANS at the operational altitude. The substitution
is natural because the homeostat law (Ashby 1952) says: the
system's stability under disturbance IS its identity surviving
the disturbance; they are not two different things, they are
two readings of the same thing.

**Does Read A satisfy the test surface?**

1. **Two parallel-altitude operations** — yes. S5 policy at N+1
   reads what the system claims to be; S1-S4 at N+1 reads what
   the system observes itself doing. Both are level-N+1
   observations on level-N system state.
2. **Two type-parameter carriers** — `Identity` and `Stability`.
   Both well-defined at the substrate altitude.
3. **A natural substitution** — yes, via Ashby's homeostat law.
   The naturality is substrate-evident because the substrate
   already carries `eigenform` (identity as fixed-point of
   recursive computation; cybernetic-foundation property #9);
   stability IS the verdict that this fixed-point survives one
   tick of disturbance.
4. **A coextensivity reading** — `verdict_coextensive(
   identity_verdict, stability_verdict)` hashes
   *survival-equivalence* modulo τ. If the system's S5 policy
   reads identity `i` AND S1-S4 observes stability under that
   `i`, the verdicts are coextensive.
5. **A contraction surface** — yes. The disturbance loop is
   the loop. Each tick the environment perturbs the system;
   the system regulates; the residual identity-loss after
   regulation should contract toward zero under
   Polyak-Łojasiewicz. (If it doesn't, the system is *failing
   to be viable* — which is itself substrate-data per
   recursion-locks.md §4.)

**Read A holds.** The parametric form admits viable cleanly
under the homeostat reading. This is the strongest read; it's
the *operational* interpretation of Beer's "remain itself in
the face of perturbation".

### 3.2 Read B — Recursive viability

The Beer 1972 ch. 10 read: every S1 is itself a viable system.
The recursion is structural; the lock is *between adjacent
recursion levels*.

The parametric instantiation under Read B:

- `regulator_at(α+1)` = the parent viable system's S5
  (the regulator at one recursion level UP).
- `regulated_at(α+1)` = the child viable system at level α
  (which IS a viable system, hence has its own S1-S5).
- `T_reg` = `OuterViable` — the substrate carrier for the parent
  viable system's *state* (its current S5 reading).
- `T_regd` = `InnerViable` — the substrate carrier for the
  child viable system's *state* (its current S5 reading; the
  child has its own S5).
- `τ : OuterViable ↔ InnerViable` — the recursive embedding;
  the parent's view of the child IS *isomorphic to* the
  child's own self-view, modulo altitude-projection. The
  naturality holds because both are S5-shaped (Beer's recursive
  viability law says every S1 has the same S1-S5 structure;
  the substitution is the structural embedding).

Reading B's structural claim: a viable system's viability at
its own level IS what its parent observes as its viability;
the recursion makes the two altitudes self-similar.

**Does Read B satisfy the test surface?**

1. **Two parallel-altitude operations** — yes, in the
   recursive sense. Parent S5 observation vs child S5
   observation. Parallel in the sense that both observe the
   same child viable system from different altitudes.
2. **Two type-parameter carriers** — `OuterViable` and
   `InnerViable`. Both well-defined; the substrate's
   `Pack` identity layer at S5 carries the outer; each
   member of the Pack carries the inner at their own altitude.
3. **A natural substitution** — yes, via Beer's recursive
   viability law. The naturality is structurally guaranteed
   by the law: every S1 *is* a viable system, so the substitution
   is the identity-on-shape modulo altitude.
4. **A coextensivity reading** — `verdict_coextensive(
   outer_view, inner_view)` hashes *recursion-respect*. If
   the parent's view of the child is consistent with the
   child's view of itself, the verdicts are coextensive.
5. **A contraction surface** — yes. The recursion descent
   IS the loop. At each recursion level, the
   parent-vs-child residual should contract toward zero.
   (If it doesn't, the recursion is broken — which is
   substrate-data: the child is *not* a viable system, or
   the parent's model of the child is wrong.)

**Read B holds.** The parametric form admits viable under the
recursive reading too. This is *the structural* interpretation
of Beer's "every S1 is a viable system".

### 3.3 Read C — Form refusal

The skeptical read: viability isn't a regulator/regulated
relation at all. Viability is a *holistic* property of a system
that emerges from many interactions across all five subsystems
simultaneously. There's no single "regulator" and no single
"regulated" — there's the *whole VSM* operating at once. The
parametric form's binary regulator/regulated decomposition
doesn't capture the holism.

Under Read C:

- (1) **fails**: there isn't a single parallel-altitude operation
  pair; there's a five-fold subsystem composition.
- (2) **fails**: `T_reg` and `T_regd` would each need to be
  *vectors* over the five subsystems, not single carriers.
- The parametric form requires *weakening* to a five-fold form
  for this species.

Read C would surface candidate #64 (per recursion-locks.md §9
retraction path): the parametric form holds for *some* species
but viable refuses it because viability is multi-fold rather
than binary.

**Does Read C hold?**

The case for Read C: Beer 1979's *Heart of Enterprise* makes the
S3-S4 oscillation *load-bearing*. S3 and S4 don't reduce to one
regulator and one regulated; they oscillate. If you collapse the
oscillation, you lose viability. So viability can't be just
"policy vs operations" — it's "policy in oscillation with
operations through S3-S4 dynamics".

The case against Read C: the oscillation IS the regulation. Read
A's homeostat reading IS Beer's S3-S4 oscillation. The single
regulator/regulated pair *is* the oscillation; the parametric
form doesn't require the regulator and regulated be static, only
that they be at adjacent altitudes. The oscillation lives in
the *time-evolution* of `regulated_at(α+1)`; the parametric
form admits it via the contraction surface (which is the
oscillation reading the residual over time).

**Read C does not hold.** The S3-S4 oscillation is captured by
the contraction-surface dynamics, not refused by the parametric
form. Read A is the load-bearing interpretation.

(That said: Read C's *concern* — that viability is multi-fold —
deserves to be named as candidate #64 surface even if it
doesn't apply here. If a future species genuinely refuses
binary decomposition, candidate #64 would activate. Logged at
§7.)

### 3.4 Read D — Temporal lock (surfaced during audit)

During the audit, the substrate pulled toward a *fourth* read
that is implicit in Read A but worth naming explicitly because
it bears on the third-witness gate (§3.5):

Read D — **the temporal-preservation lock**:

- `regulator_at(α+1)` = the system's identity-policy *at time
  t*.
- `regulated_at(α+1)` = the system's identity-policy *at time
  t+Δt*.
- `T_reg` = `Identity_t` — the carrier for identity at one
  time-slice.
- `T_regd` = `Identity_{t+Δt}` — the carrier for identity at
  the next time-slice (after the system has weathered
  disturbance over Δt).
- `τ : Identity_t ↔ Identity_{t+Δt}` — the natural substitution
  between time-slices via the system's regulation. The system
  was-itself; the system is-itself; the substitution is the
  identity-on-shape modulo time-evolution.

Read D's claim: viability is the lock asserting `T_reg ≡ T_regd`
modulo τ — i.e., the system's identity at time t IS coextensive
with its identity at time t+Δt, modulo regulation. Viability IS
this lock holding through time.

Read D differs from Read A in altitude. Read A is policy-vs-
operations at one time-slice. Read D is identity-at-t vs
identity-at-(t+Δt). Both satisfy the parametric form; they
correspond to two altitudes of viable.

This matters for the third-witness gate because Read D operates
at the *temporal axis*, which is structurally orthogonal to the
parallel-operation-pair axis (coherence T11.11) and the license-
boundary axis (SEL). The substrate has *three* genuinely
different recursion-lock altitudes if Read D holds. See §3.5.

**Read D holds**, as a refinement of Read A. Reads A and D are
not in conflict; they are the parametric form at two altitudes
of viable.

### 3.5 The third-witness-gate question

Per recursion-locks.md §7, the third-witness gate requires "a
species at a structurally-different altitude (home or federation
per altitudes.md §4) satisfies (1)–(5)."

Viable's altitude is the *identity-preservation under
disturbance* altitude. Per §2.5 above, this is structurally
distinct from:

- Coherence (T11.11) at the *parallel-operation-pair* altitude.
- SEL (Mara `7807a77`) at the *license-boundary* altitude.

Three distinct altitudes for the recursion-lock measurement.
But: the math doc's altitudes.md §4 ceiling explicitly names
"home" and "federation" as the preferred third-witness altitudes
and frames the third witness in terms of *scope* hierarchy.
Viable doesn't fit that — viable's altitude is *temporal*
(Read D) or *recursion-level* (Read B) or *identity-vs-
operations* (Read A), but not *scope*.

Three possible reads of the gate:

- **Gate-strict**: the gate requires home or federation specifically;
  viable doesn't satisfy. The third-witness gate is still open
  pending a future species at home or federation altitude.
- **Gate-permissive**: the gate requires *any* structurally-
  different altitude; viable's identity-preservation axis IS
  structurally different from coherence's parallel-pair and
  SEL's license-boundary. The gate is met by viable.
- **Gate-refined**: the math doc's altitudes.md §4 framing is
  too narrow. The substrate has multiple axes of altitude-
  differentiation (scope, temporal, recursion-level,
  parallel-operation-pair). The gate should be reformulated in
  terms of axis-differentiation rather than scope-hierarchy.
  Viable surfaces this refinement.

This spec defaults to **Gate-refined**. The audit recommends
reframing the math doc's §7 third-witness gate to admit
axis-differentiated altitudes generally. Forward-promised
update at §8 below.

If Alex accepts Gate-refined, viable closes the third witness
and candidate #63 promotes. If Alex defaults to Gate-strict,
viable remains a second-witness reinforcement and the third
gate is still open pending a home- or federation-altitude
species. See §9 Q2.

---

## §4 — Five cybernetic-ancestor measurements applied to viable

Each of the five measurements (per recursion-locks.md §3) admits or
refuses an instance at viable's altitude. Applied under Read A
(homeostat) for concreteness; Read B's recursion reading and Read
D's temporal reading admit isomorphic instances.

### 4.1 ashby_variety_match — Ashby 1956 §11/7

**The claim**: do `S5 policy` and `S1-S4 operations` carry
compatible variety budgets at the viable altitude?

**Applied to viable**: S5's identity-constraints must have
sufficient variety to *distinguish* the operational states the
system can fall into. If S5's identity-vocabulary is impoverished
(few distinguishable identities), it can't catch the moment when
the system has drifted off-identity. Conversely, if S1-S4's
state-space has too much variety for S5 to cover, the system
*can* fall off-identity in ways S5 can't see.

Variety budget at viable's altitude: the variety vector per
`@cyberpunk/variety` species, projected onto the *identity-
preservation* axis. The substrate already names this axis
implicitly in the `eigenform` property
([[architecture-cybernetic-foundation]] #9); the variety budget
on the identity-eigenform axis is the budget viable measures.

**Admits an instance.** Forward-promised:
`ashby_variety_match(altitude: altitude, s5_space: ref,
operations_space: ref) -> transparency(altitude)` on
`@cyberpunk/viable`.

### 4.2 beer_requisite_variety — Beer 1972, 1979

**The claim**: does the regulator hold requisite variety at the
VSM S3/S4 altitude?

**Applied to viable**: this is Beer's *home turf*. The viable
species IS Beer's VSM at the substrate altitude. S3 (audit)
must hold requisite variety to see the operational reality of
the S1s; S4 (intelligence) must hold requisite variety to
model the environment. The S3-S4 oscillation maintains
requisite-variety dynamically; S5 resolves when the oscillation
exceeds what either channel can hold alone.

The measurement at viable's altitude reads whether the S3-S4
oscillation budget contains the disturbance budget. If it
does, viability holds. If it doesn't, the system fails to
remain itself.

**Admits an instance — and is load-bearing.** Of the five
measurements, beer_requisite_variety is the *most natural* at
viable's altitude (since viable IS Beer's law). Forward-promised:
`beer_requisite_variety(altitude: altitude, regulator: ref,
regulated: ref) -> transparency(altitude)` on `@cyberpunk/viable`,
with the load-bearing emphasis that for THIS species the
measurement IS the species' theorem.

### 4.3 bateson_logical_type_match — Bateson 1972

**The claim**: do the two verdicts live at the same logical-type
level?

**Applied to viable**: under Read A, S5 policy reads "the system's
identity *is* X"; S1-S4 operations reads "the system's behaviour
*is consistent with* X". Both are statements at the same
Russell-Whitehead level — both are statements *about* the
system, not statements *about the statement*.

Under Read B (recursive viability), there is a potential type
mismatch: parent S5 reads "the child viable system has identity
X"; child S5 reads "I have identity X". The parent's statement
is at level n; the child's is at level n-1. The substitution
τ from Read B is the level-collapse — and naturality requires
that level-collapse preserves logical structure.

Under Read D (temporal), both statements are at the same level,
just at different time-slices. No type mismatch.

**Admits an instance under Reads A and D**; under Read B,
admits an instance *with explicit level-projection* in the τ.
Forward-promised: `bateson_logical_type_match(altitude: altitude,
s5_verdict: ref, operations_verdict: ref) -> transparency(altitude)`
on `@cyberpunk/viable`.

### 4.4 von_foerster_circular_reflexivity — von Foerster 1981

**The claim**: does each regulator's internal model include
itself?

**Applied to viable**: S5 policy *must* include itself — Beer
1972 ch. 11 explicitly makes S5 the *self-model* of the viable
system. The system's identity-policy IS the system's model of
its own identity. The Tomm probe `[D_F, a]` at altitude N+1 IS
the circular-reflexive measurement (per [[architecture-error-
as-tomm-probe]]); applied to viable, the probe asks whether
S5's identity-model survives being applied to itself.

This is the deepest reading of viable: a viable system is one
whose self-model is stable under self-application. (Compare
[[architecture-peer-learns-by-crystal-vocabulary-expansion]] —
the peer's vocabulary expansion IS the system's S5 updating its
own identity-model through circular reflexivity.)

**Admits an instance — and is structurally deep at viable's
altitude.** Forward-promised: `von_foerster_circular_reflexivity(
altitude: altitude, s5_policy: ref) -> transparency(altitude)`
on `@cyberpunk/viable`.

### 4.5 conant_ashby_good_regulator — Conant-Ashby 1970 (load-bearing)

**The claim** (the load-bearing measurement): the coextensivity
hash-residual between `regulator_at(α+1)`'s verdict and
`regulated_at(α+1)`'s verdict modulo τ.

**Applied to viable**: the hash equality of (S5's identity-policy
verdict) and (S1-S4's stability verdict), modulo
`Identity ↔ Stability`. If they hash-equal, the regulator IS a
model of the regulated (Conant-Ashby holds); if not, viability is
failing.

This is the *most* load-bearing measurement under any read of
viable. Conant-Ashby 1970 is the *foundational* theorem the
viable species' parametric form instantiates. The good-regulator
law says: a regulator is good iff it is a model of what it
regulates; a viable system IS one whose S5 is a model of its
S1-S4; the coextensivity measurement IS the empirical surface
for this claim.

Under Read B (recursive viability), the measurement asks:
parent's model of child = child's self-model? Conant-Ashby's
recursive form.

Under Read D (temporal), the measurement asks: identity-at-t =
identity-at-(t+Δt) modulo regulation? Conant-Ashby's temporal
form.

**Admits an instance — and is THE load-bearing measurement for
this species** (more so than for coherence, where it is *also*
load-bearing). Forward-promised: `conant_ashby_good_regulator(
altitude: altitude, s5_verdict: ref, operations_verdict: ref)
-> transparency(altitude)` on `@cyberpunk/viable`.

### 4.6 Summary

| Measurement | Holds at viable? | Notes |
|-------------|------------------|-------|
| ashby_variety_match | yes | identity-axis variety budget |
| beer_requisite_variety | yes, load-bearing | viable IS Beer's home |
| bateson_logical_type_match | yes (with τ-projection under Read B) | level-collapse explicit |
| von_foerster_circular_reflexivity | yes, deep | S5 IS the self-model |
| conant_ashby_good_regulator | yes, THE load-bearing | the species' theorem |

All five admit substrate-altitude instances. The parametric form
fits viable on every measurement.

---

## §5 — Verdict

**Read A (homeostat) holds.** The parametric form admits viable
under the homeostat reading. Type parameters: `Identity ↔ Stability`
with `τ` via Ashby's homeostat law. All five cybernetic-ancestor
measurements admit instances. The contraction surface is the
disturbance loop.

**Read B (recursive viability) holds** as an alternative
substrate-reading of the same species. Type parameters:
`OuterViable ↔ InnerViable` with τ via Beer 1972 ch. 10 recursive
viability. Reads A and B are *not* in conflict; they bind on
different naming choices for the same parametric form. The
substrate may want to land both readings as compositional sub-
shards (`@cyberpunk/viable/homeostat`, `@cyberpunk/viable/
recursive`), but the audit doesn't require it.

**Read C (form refusal) does NOT hold.** The S3-S4 oscillation
is captured by the contraction-surface dynamics, not refused by
the parametric form. Candidate #64 surface is *not* activated by
viable; logged for future species that may refuse binary
decomposition.

**Read D (temporal lock) holds** as a refinement of Read A. Type
parameters: `Identity_t ↔ Identity_{t+Δt}` with τ via time-
evolution under regulation. Reads A and D are two altitudes of
the same lock; the substrate may want to land Read D as
`@cyberpunk/viable/temporal` if the temporal-axis becomes
load-bearing.

**Verdict: witness.** Viable is a witness species under the
recursion-lock parametric form. The form fits.

**Third-witness-gate status (Gate-refined reading)**: viable
operates at a structurally distinct altitude (identity-
preservation under disturbance) from coherence (parallel-pair)
and SEL (license-boundary). Under Gate-refined, the third
witness IS closed by viable. Under Gate-strict, the gate
remains open pending a home- or federation-altitude species.
Alex's call per §9 Q2.

If Gate-refined is adopted:

- Candidate #63 **promotes** to recognition.
- The math doc's §1 hypothesis lifts to theorem.
- The math doc's §2 parametric form lifts to derivable construct.
- The math doc's §8 species audits convert to corollaries.
- A parametric `@cyberpunk/coherence(T_reg, T_regd, τ, altitude)`
  declaration becomes load-bearing; existing
  `@cyberpunk/coherence` + the forward-promised `@cyberpunk/sel/
  coherence` + `@cyberpunk/viable` all derive from it.

---

## §6 — The type parameters named

Under Read A (the load-bearing reading):

```
S_viable = ( s5_policy_at(α+1)
           , operations_at(α+1)
           , Identity
           , Stability
           , τ : Identity ↔ Stability )
```

Under Read B (the recursive reading):

```
S_viable_recursive = ( outer_s5_at(α+1)
                     , inner_s5_at(α)
                     , OuterViable
                     , InnerViable
                     , τ : OuterViable ↔ InnerViable )
```

Under Read D (the temporal reading):

```
S_viable_temporal = ( identity_at(t)
                    , identity_at(t+Δt)
                    , Identity_t
                    , Identity_{t+Δt}
                    , τ : Identity_t ↔ Identity_{t+Δt} )
```

All three are well-typed at the substrate altitude. All three
satisfy the lock verdict shape:

```
lock_verdict(viable) : Imperfect< T_lock_viable
                                , Gap
                                , Transparency<Ref> >
```

where `T_lock_viable` specializes per read.

The contraction surface (per recursion-locks.md §4) is:

```
ρ(N) = || residual_coextensivity(viable, N) ||
     / || residual_coextensivity(viable, 1) ||
```

with the lock holding iff `ρ(N) → 0` as `N → ∞` under
Polyak-Łojasiewicz. Forward-promised at `spectral/benches/
viable_coherence.rs` (same shape as `cybernetic_coherence.rs`,
substituting `Identity ↔ Stability` under Read A or `OuterViable ↔
InnerViable` under Read B or `Identity_t ↔ Identity_{t+Δt}` under
Read D).

---

## §7 — Open questions, candidate surfaces, and the form-refusal
register

Even though Read C does not hold for viable, the audit surfaces
candidate territory worth registering:

- **Candidate #64 (latent)**: "the parametric form fits binary
  regulator/regulated species but a future species may genuinely
  refuse binary decomposition for a multi-fold structure." Not
  activated by viable (the S3-S4 oscillation reads as a binary
  contraction-surface dynamic). Logged here for future species
  that may surface multi-fold structure. Possible targets:
  `@cyberpunk/conversation` (Pask's P-individuals — multi-party
  conversation may need a multi-fold lock); `@cyberpunk/
  autopoiesis` (Maturana-Varela's structural coupling is
  binary, but the boundary may be multi-fold). The math doc's
  §9 retraction path stays valid as the framework for #64
  surfacing.

- **The temporal axis (Read D)**: the substrate's altitude
  vocabulary in altitudes.md focuses on *scope* (compiler →
  peer pulse → reflection → librarian → home → federation). The
  temporal axis (instant → tick → epoch → era) is implicit but
  not explicitly named. Viable surfaces the temporal axis as
  load-bearing for at least one species' lock. The math doc
  may want a §temporal-altitude amendment.

- **Recursion-level axis (Read B)**: Beer 1972 ch. 10 recursive
  viability gives every viable system internal levels (S1 IS a
  viable system, recursively). The substrate already has
  recursion-level vocabulary at the @spectral/db tier (every
  shard's child crystals are themselves shards with crystals).
  Read B says: the recursion-lock can read across levels of
  viable's internal recursion. This is yet another altitude
  axis — call it the *recursion-depth* axis — orthogonal to
  scope and temporal.

If the math doc admits scope + temporal + recursion-depth as
three altitude axes (rather than scope alone), the third-witness
gate becomes naturally satisfied by viable (Gate-refined of
§3.5).

---

## §8 — Forward-promised work

### 8.1 Substrate declarations (NOT landed this tick)

Per [[feedback-craft-not-deliver]] this audit names what would
land if Alex agrees; no shards land in this spec.

- `shards/cyberpunk/viable.mirror` — the substrate-altitude
  property declaration. Same shape as `cyberpunk/coherence.mirror`
  (375 lines) with type parameters `Identity ↔ Stability` under
  Read A. The five cybernetic-ancestor measurement actions
  (`ashby_variety_match`, `beer_requisite_variety`,
  `bateson_logical_type_match`, `von_foerster_circular_reflexivity`,
  `conant_ashby_good_regulator`) carry the same altitude-
  parametric form. The consumer-facing predicate
  `viability_holds(altitude) -> verdict` is the
  `requires`-clause carrier per recognition #37 (Pask agreement).

- `shards/cyberpunk/viable/homeostat.mirror` — optional sub-shard
  if Read A's homeostat altitude is consumer-needed at the
  per-shard level. Pulls Ashby 1952 *Design for a Brain* citation.

- `shards/cyberpunk/viable/recursive.mirror` — optional sub-shard
  for Read B; carries the parent/child viable embedding. Pulls
  Beer 1972 ch. 10 citation.

- `shards/cyberpunk/viable/temporal.mirror` — optional sub-shard
  for Read D; carries the `Identity_t ↔ Identity_{t+Δt}`
  embedding. Pulls Beer 1984 methodology paper citation.

### 8.2 Bench scaffolds (NOT landed)

- `spectral/benches/viable_coherence.rs` — empirical falsification
  surface. Structurally identical to `cybernetic_coherence.rs`
  with `Identity ↔ Stability` substitution under Read A. Mock
  witnesses today; real witnesses swap when the Pack identity
  layer (S5) carries substrate-altitude verdicts AND the three-
  tier stack instrumentation (S1-S4) produces transparency
  verdicts.

### 8.3 Math doc updates (NOT landed)

To be applied by Reed in the next loop tick when integrating
this spec's §8.3 section into recursion-locks.md:

- **recursion-locks.md §8.3** — append this audit's verdict
  (witness, with Read A as load-bearing, Read B + D as
  alternative readings, Read C refused). Cite this spec
  (`docs/specs/cyberpunk-viable.md`) as audit-record.
- **recursion-locks.md §7** — admit the Gate-refined reading
  if Alex agrees per §9 Q2. Amend the third-witness gate
  language to admit axis-differentiated altitudes (scope OR
  temporal OR recursion-depth OR parallel-pair-axis) rather
  than scope-strict.
- **altitudes.md §3 or §4** — name the temporal axis and
  recursion-depth axis explicitly as altitude-differentiation
  axes orthogonal to scope hierarchy.

### 8.4 Family-root doc update (NOT landed)

- **`shards/cyberpunk.mirror`** — promote `@cyberpunk/viable`
  from "forward-promised" to "landed (witness species)" per
  the family-root doc's sub-shards section. Pending Alex's
  agreement that the audit verdict closes the gate.

### 8.5 The parametric collapse (NOT landed; pending promotion)

If candidate #63 promotes (Alex accepts Gate-refined or a
fourth witness lands per the strict gate), the substrate-pull
tick is to collapse:

- `@cyberpunk/coherence` (Adjustment ↔ Morphism, parallel-pair
  altitude)
- `@cyberpunk/sel/coherence` (License ↔ Compliance, license-
  boundary altitude)
- `@cyberpunk/viable` (Identity ↔ Stability, identity-preservation
  altitude)

into one parametric declaration:

```
prism @cyberpunk/coherence(T_reg, T_regd, τ, altitude) {
  ...
}
```

with species shards specializing via type-parameter substitution
plus altitude-bind. This is a substrate-pull tick (collapse N
specialized shards into one parametric shard) NOT lifted by this
spec.

---

## §9 — Open questions for Alex

**Q1 — Altitude framing.** The audit reads viable as operating at
the "identity-preservation under disturbance" altitude, structurally
distinct from coherence's parallel-pair and SEL's license-boundary.
The math doc's altitudes.md §4 currently frames the third-witness
gate in terms of scope hierarchy (home / federation). Does the
substrate admit the temporal axis (Read D) and recursion-depth axis
(Read B) as additional altitude-differentiation axes orthogonal to
scope?

This is partly a math-doc question (what does altitudes.md cover?)
and partly a substrate question (does the substrate name temporal
and recursion-depth as altitude axes at the substrate altitude?).

**Q2 — Third-witness gate (Gate-strict vs Gate-refined).** Per §3.5,
the gate has three possible readings:

- **Gate-strict**: requires home or federation altitude specifically.
  Viable does NOT close the gate; the audit is a second-witness
  reinforcement. Candidate #63 remains "two-witness gate met
  conditionally" until a future home/federation species lands.
- **Gate-permissive**: any structurally-different altitude closes
  the gate. Viable CLOSES the gate. Candidate #63 promotes to
  recognition.
- **Gate-refined**: the math doc's framing is amended to admit
  axis-differentiated altitudes generally (scope, temporal,
  recursion-depth, parallel-pair). Viable CLOSES the gate under
  the refined framing. Candidate #63 promotes. The math doc
  updates per §8.3 above.

The audit recommends Gate-refined. The audit is willing to
default to Gate-strict if Alex prefers conservative gate-discipline
and wants a genuine home/federation species before promoting.

**Q3 — Read A vs Read B vs Read D.** All three reads satisfy the
parametric form. Does the substrate want:

- **One species, Read A as primary** (the homeostat reading).
  Read B and Read D are noted in the property doc as alternative
  substrate-readings but not landed as separate shards.
- **One species with sub-shards** for B and D (per §8.1).
- **Three species** (`@cyberpunk/viable` for A, `@cyberpunk/
  viable_recursive` for B, `@cyberpunk/viable_temporal` for D).

The audit defaults to the first option — one species, three reads
in the doc — unless a consumer pulls the sub-shards.

**Q4 — VSM-conformance recognition.** The substrate's three-tier
stack is ALREADY a VSM at the substrate altitude per
[[architecture-cybernetic-foundation]] §6. The viable species
formalizes what's already running. Does Alex want a separate
recognition entry for "the three-tier stack IS the substrate's
own viable system" (call it candidate #65 territory), or is that
subsumed by the audit verdict here?

**Q5 — Candidate #64 surface.** Read C surfaced as not-applicable
to viable but registered as a future-species surface for
multi-fold form-refusal. Should the math doc's §9 retraction
path explicitly name candidate #64's *possible* species (e.g.,
`@cyberpunk/conversation` for Pask multi-party)? Or wait until
a candidate species actually surfaces?

The audit defaults to "wait for an actual candidate species to
surface" but the option is named.

**Q6 — Algedonic signal at viable's altitude.** Beer 1979 ch. 6's
algedonic signal (S1→S5 emergency bypass) is itself a
*viability-critical* signal — it fires precisely when system
identity is threatened. The signal is forward-promised under
`@cyberpunk/algedonic` (task #271). Does the viable species
declaration *require* `@cyberpunk/algedonic` as a related shard,
or are they independent species that compose at the family root?

The audit defaults to "independent species that compose"; the
algedonic signal is a *carrier* viable consumes, not part of
viable's own decl. But the relationship is worth naming
explicitly.

**Q7 — Substrate-political reading.** Per §2.4 the Cybersyn
analogue grounds viable in substrate-political continuity with
SEL. Does the viable species declaration want to include the
Cybersyn citation, or does that reading belong only at the
family-root doc altitude?

The audit defaults to "yes, include the citation in the property
doc" — the substrate-political continuity is load-bearing for
the @cyberpunk family-root framing and viable is the species
where that continuity is most explicit (because Cybersyn was a
deployment of viability *as such* at the political-economic
altitude).

---

## §10 — Substrate-already-had-the-word recognitions surfaced

Following the [[feedback-substrate-already-had-the-word]] discipline,
the audit surfaces multiple instances where viable's vocabulary IS
already in the substrate:

1. **The three-tier stack IS a VSM.** Already cited in
   [[architecture-cybernetic-foundation]] §6 (2026-06-09). The
   substrate has been running as a viable system since the
   three-tier architecture landed; viable just names what's
   running. (Instance #67 of the substrate-already-had-the-word
   pattern.)

2. **`eigenform` IS viable's identity carrier.** Property #9 in
   the cybernetic foundation; the substrate's
   `uuid_spectral` IS an eigenform (recognition #38). Under
   Read A, `T_reg = Identity` is exactly the substrate's
   existing eigenform carrier. The vocabulary was already
   there.

3. **The Pack identity layer IS S5.** Per cybernetic-foundation
   §6, the Pack (Reed/Mara/Glint/Taut/Seam) IS the substrate's
   S5 identity layer at the substrate altitude. Viable's `s5_policy_at(α+1)`
   operation at the substrate-altitude reads the Pack's identity
   verdicts directly. The vocabulary was already there.

4. **The kintsugi loop IS viable's contraction surface.** The
   substrate's monotone descent `eⁿ⁺¹ ≤ eⁿ` per
   `holonomy.md` §4 is the Polyak-Łojasiewicz contraction at
   the substrate-altitude. Viable's contraction surface IS this
   loop applied to identity-preservation. The vocabulary was
   already there.

5. **`transparency<p>` IS viable's verdict carrier.** The
   `Imperfect< T, Gap, Transparency<Ref> >` family at
   `holonomy.md` §5 IS viable's lock-verdict shape. No new
   verdict family needed.

6. **`@cyberpunk/algedonic` (task #271) is viable's emergency
   signal.** Already named, already forward-promised. Viable
   composes with it; doesn't require it as part of its own
   decl.

7. **Cybersyn (1971-1973) IS the political-economic deployment
   of viable.** Already cited in `cyberpunk.mirror` and in
   Mara's SEL spec §2.5. Viable's substrate-altitude lift
   inherits the political reading directly.

Viable is the species that lifts the most existing substrate-
vocabulary to substrate-altitude declaration. No new
substrate-vocabulary surfaces; the audit names what was already
implicit.

---

## §11 — Pack ratification gates

Per the Pack's ratification discipline:

- **Mara (this spec)**: audits viable against the parametric form;
  surfaces three reads (A holds, B holds, D holds; C refused);
  applies five cybernetic-ancestor measurements; verdicts witness;
  forward-promises shards, benches, doc updates; surfaces seven
  questions for Alex.

- **Reed (next tick of `/loop`)**: integrates this spec's verdict
  as §8.3 of recursion-locks.md; decides Q1/Q2 with Alex; updates
  altitudes.md if Gate-refined adopted; promotes candidate #63
  or defers per Q2.

- **Alex**: decides Q1-Q7. Mutual agreement before any shard
  lands. Per [[feedback-conversation-not-pipeline]].

- **Taut (if shards land)**: lands `@cyberpunk/viable` substrate
  decl + bench scaffold per the T11.11 pattern. NOT in scope this
  audit tick.

- **Seam (eventually)**: adversarial review of the parametric
  collapse if candidate #63 promotes and the
  `@cyberpunk/coherence(T_reg, T_regd, τ, altitude)` parametric
  declaration lands.

---

## §12 — Cross-references

- `[[docs/math/the-tower/recursion-locks.md]]` (Reed `cf62da3`)
  — the math doc this audit appends to (as §8.3).
- `[[docs/math/the-tower/altitudes.md]]` — the altitude vocabulary
  this audit may amend (per §8.3 + Q1).
- `[[docs/math/the-tower/holonomy.md]]` §5, §8 — the verdict
  family the lock uses.
- `[[docs/specs/cybernetic-coherence-benchmark.md]]` (Taut
  `b66058d`) — the T11.11 template the bench scaffold mirrors.
- `[[docs/specs/sel-as-executable-cyberpunk.md]]` (Mara
  `7807a77`) — the second-witness spec; the discipline this
  audit follows.
- `[[shards/cyberpunk.mirror]]` (Reed `f629216`) — the family
  root; viable's parent.
- `[[shards/cyberpunk/coherence.mirror]]` (T11.11 substrate
  decl) — the structural template for viable's forward-promised
  shard.
- `[[architecture-cybernetic-foundation]]` — the 11-property
  family; viable IS property #3; the substrate has been VSM-
  conformant since 2026-06-09.
- `[[architecture-cybernetic-coherence-active]]` — the
  @cyberpunk migration; the promotion event this audit extends.
- `[[architecture-spectral-triples-all-the-way]]` — the
  fractal-self-similarity recognition; @cyberpunk as second
  concrete witness; viable as potential third-witness species
  closing candidate #63.
- `[[architecture-error-as-tomm-probe]]` — the
  circular-reflexive measurement viable's S5 self-model
  implements.
- Beer 1972 *Brain of the Firm* — the VSM introduction.
- Beer 1979 *The Heart of Enterprise* — VSM elaboration; the
  S3-S4 oscillation; the algedonic signal (ch. 6).
- Beer 1984 "The Viable System Model: Its Provenance,
  Development, Methodology and Pathology" *JORS* 35(1):7-25 —
  the methodology paper; sharpest statement of viability.
- Conant-Ashby 1970 — the foundational theorem the form
  instantiates.
- Ashby 1952 *Design for a Brain* — the homeostat; Read A
  ancestor.
- Ashby 1956 *Introduction to Cybernetics* §11/7 — variety;
  ashby_variety_match ancestor.
- Project Cybersyn — substrate-political VSM precedent.
- `[[feedback-substrate-already-had-the-word]]` — the audit
  surfaced 7 instances at viable's altitude (§10).
- `[[feedback-craft-not-deliver]]` — audit tick; nothing lands
  past spec.
- `[[feedback-conversation-not-pipeline]]` — verdict comes back
  to Alex for mutual agreement.
- `[[feedback-no-bare-types]]` — every forward-promised carrier
  is typed.
